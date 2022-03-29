// Copyright 2021 Centrifuge Foundation (centrifuge.io).
// This file is part of Centrifuge chain project.

// Centrifuge is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Centrifuge is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

//! # Example pallet's unit test cases.

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

use super::*;
use crate::mock::{
    helpers::*, Balances, ChainBridge, Example, NativeTokenId, Origin, ProposalLifetime,
    TestExternalitiesBuilder, ENDOWED_BALANCE, RELAYER_A, RELAYER_B, RELAYER_C,
    TEST_RELAYER_VOTE_THRESHOLD,
};

use frame_support::assert_ok;

// ----------------------------------------------------------------------------
// Test cases
// ----------------------------------------------------------------------------

#[test]
fn transfer_native() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            let origin = Origin::signed(RELAYER_A);
            let dest_chain = 0;
            let resource_id = NativeTokenId::get();
            let amount: u64 = 100;
            let recipient = vec![99];

            assert_ok!(ChainBridge::whitelist_chain(
                Origin::root(),
                dest_chain.clone()
            ));

            let origin_balance_before = Balances::free_balance(RELAYER_A);
            let total_issuance_before = Balances::total_issuance();

            assert_ok!(Example::transfer_native(
                origin.clone(),
                amount.clone(),
                recipient.clone(),
                dest_chain,
            ));

            assert_eq!(
                Balances::free_balance(RELAYER_A),
                origin_balance_before - amount
            );
            assert_eq!(Balances::total_issuance(), total_issuance_before - amount);

            expect_event(chainbridge::Event::FungibleTransfer(
                dest_chain,
                1,
                resource_id,
                amount.into(),
                recipient,
            ));
        })
}

#[test]
fn transfer() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            let amount = 10;
            let relayer_a_balance_before = Balances::free_balance(RELAYER_A);
            let total_issuance_before = Balances::total_issuance();

            assert_ok!(Example::transfer(
                Origin::signed(ChainBridge::account_id()),
                RELAYER_A,
                amount,
            ));

            assert_eq!(
                Balances::free_balance(RELAYER_A),
                relayer_a_balance_before + 10
            );
            assert_eq!(Balances::total_issuance(), total_issuance_before + amount);

            assert_events(vec![mock::Event::Balances(
                pallet_balances::Event::Deposit {
                    who: RELAYER_A,
                    amount,
                },
            )]);
        })
}

#[test]
fn create_sucessful_transfer_proposal() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            let prop_id = 1;
            let src_id = 1;
            let r_id = chainbridge::derive_resource_id(src_id, b"transfer");
            let resource = b"Example.transfer".to_vec();
            let proposal = make_transfer_proposal(RELAYER_A, 10);

            assert_ok!(ChainBridge::set_threshold(
                Origin::root(),
                TEST_RELAYER_VOTE_THRESHOLD
            ));
            assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_A));
            assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_B));
            assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_C));
            assert_ok!(ChainBridge::whitelist_chain(Origin::root(), src_id));
            assert_ok!(ChainBridge::set_resource(Origin::root(), r_id, resource));

            // Create proposal (& vote)
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_A),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = chainbridge::types::ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: chainbridge::types::ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            // Second relayer votes against
            assert_ok!(ChainBridge::reject_proposal(
                Origin::signed(RELAYER_B),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = chainbridge::types::ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![RELAYER_B],
                status: chainbridge::types::ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            let total_issuance_before = Balances::total_issuance();

            // Third relayer votes in favour
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_C),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = chainbridge::types::ProposalVotes {
                votes_for: vec![RELAYER_A, RELAYER_C],
                votes_against: vec![RELAYER_B],
                status: chainbridge::types::ProposalStatus::Approved,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            assert_eq!(Balances::free_balance(RELAYER_A), ENDOWED_BALANCE + 10);
            assert_eq!(Balances::total_issuance(), total_issuance_before + 10);

            assert_events(vec![
                mock::Event::ChainBridge(chainbridge::Event::VoteFor(src_id, prop_id, RELAYER_A)),
                mock::Event::ChainBridge(chainbridge::Event::VoteAgainst(
                    src_id, prop_id, RELAYER_B,
                )),
                mock::Event::ChainBridge(chainbridge::Event::VoteFor(src_id, prop_id, RELAYER_C)),
                mock::Event::ChainBridge(chainbridge::Event::ProposalApproved(src_id, prop_id)),
                mock::Event::Balances(pallet_balances::Event::Deposit {
                    who: RELAYER_A,
                    amount: 10,
                }),
                mock::Event::ChainBridge(chainbridge::Event::ProposalSucceeded(src_id, prop_id)),
            ]);
        })
}
