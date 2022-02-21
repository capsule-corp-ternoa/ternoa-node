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

//! Unit test cases for the Substrate/Ethereum chains bridging pallet.

// ----------------------------------------------------------------------------
// Module imports and re-exports
// ----------------------------------------------------------------------------

use super::*;
use crate::{
    self as pallet_chainbridge,
    constants::DEFAULT_RELAYER_VOTE_THRESHOLD,
    mock::{
        helpers::*, Balances, Call, ChainBridge, Event, MockRuntime, Origin, ProposalLifetime,
        System, SystemCall, TestExternalitiesBuilder, ENDOWED_BALANCE, RELAYER_A, RELAYER_B,
        RELAYER_C, TEST_RELAYER_VOTE_THRESHOLD,
    },
    types::{ProposalStatus, ProposalVotes},
};

use frame_support::{assert_noop, assert_ok};

// ----------------------------------------------------------------------------
// Test cases
// ----------------------------------------------------------------------------

#[test]
fn derive_ids() {
    let chain = 1;
    let id = [
        0x21, 0x60, 0x5f, 0x71, 0x84, 0x5f, 0x37, 0x2a, 0x9e, 0xd8, 0x42, 0x53, 0xd2, 0xd0, 0x24,
        0xb7, 0xb1, 0x09, 0x99, 0xf4,
    ];
    let r_id = derive_resource_id(chain, &id);
    let expected = [
        0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x21, 0x60, 0x5f, 0x71, 0x84, 0x5f,
        0x37, 0x2a, 0x9e, 0xd8, 0x42, 0x53, 0xd2, 0xd0, 0x24, 0xb7, 0xb1, 0x09, 0x99, 0xf4, chain,
    ];
    assert_eq!(r_id, expected);
}

#[test]
fn complete_proposal_approved() {
    let mut prop = ProposalVotes {
        votes_for: vec![1, 2],
        votes_against: vec![3],
        status: ProposalStatus::Initiated,
        expiry: ProposalLifetime::get(),
    };

    prop.try_to_complete(2, 3);
    assert_eq!(prop.status, ProposalStatus::Approved);
}

#[test]
fn complete_proposal_rejected() {
    let mut prop = ProposalVotes {
        votes_for: vec![1],
        votes_against: vec![2, 3],
        status: ProposalStatus::Initiated,
        expiry: ProposalLifetime::get(),
    };

    prop.try_to_complete(2, 3);
    assert_eq!(prop.status, ProposalStatus::Rejected);
}

#[test]
fn complete_proposal_bad_threshold() {
    let mut prop = ProposalVotes {
        votes_for: vec![1, 2],
        votes_against: vec![],
        status: ProposalStatus::Initiated,
        expiry: ProposalLifetime::get(),
    };

    prop.try_to_complete(3, 2);
    assert_eq!(prop.status, ProposalStatus::Initiated);

    let mut prop = ProposalVotes {
        votes_for: vec![],
        votes_against: vec![1, 2],
        status: ProposalStatus::Initiated,
        expiry: ProposalLifetime::get(),
    };

    prop.try_to_complete(3, 2);
    assert_eq!(prop.status, ProposalStatus::Initiated);
}

#[test]
fn setup_resources() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            let id: ResourceId = [1; 32];
            let method = "Pallet.do_something".as_bytes().to_vec();
            let method2 = "Pallet.do_somethingElse".as_bytes().to_vec();

            assert_ok!(ChainBridge::set_resource(
                Origin::root(),
                id,
                method.clone()
            ));
            assert_eq!(ChainBridge::get_resources(id), Some(method));

            assert_ok!(ChainBridge::set_resource(
                Origin::root(),
                id,
                method2.clone()
            ));
            assert_eq!(ChainBridge::get_resources(id), Some(method2));

            assert_ok!(ChainBridge::remove_resource(Origin::root(), id));
            assert_eq!(ChainBridge::get_resources(id), None);
        })
}

#[test]
fn whitelist_chain() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            assert!(!ChainBridge::chain_whitelisted(0));

            assert_ok!(ChainBridge::whitelist_chain(Origin::root(), 0));
            assert_noop!(
                ChainBridge::whitelist_chain(
                    Origin::root(),
                    <MockRuntime as pallet_chainbridge::Config>::ChainId::get()
                ),
                Error::<MockRuntime>::InvalidChainId
            );

            assert_events(vec![Event::ChainBridge(crate::Event::ChainWhitelisted(0))]);
        })
}

#[test]
fn set_get_threshold() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            assert_eq!(ChainBridge::get_threshold(), DEFAULT_RELAYER_VOTE_THRESHOLD);

            assert_ok!(ChainBridge::set_threshold(
                Origin::root(),
                TEST_RELAYER_VOTE_THRESHOLD
            ));
            assert_eq!(ChainBridge::get_threshold(), TEST_RELAYER_VOTE_THRESHOLD);

            assert_ok!(ChainBridge::set_threshold(Origin::root(), 5));
            assert_eq!(ChainBridge::get_threshold(), 5);

            assert_events(vec![
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerThresholdChanged(
                    TEST_RELAYER_VOTE_THRESHOLD,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerThresholdChanged(5)),
            ]);
        })
}

#[test]
fn asset_transfer_success() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            let dest_id = 2;
            let to = vec![2];
            let resource_id = [1; 32];
            let amount = 100;

            assert_ok!(ChainBridge::set_threshold(
                Origin::root(),
                TEST_RELAYER_VOTE_THRESHOLD
            ));

            assert_ok!(ChainBridge::whitelist_chain(
                Origin::root(),
                dest_id.clone()
            ));
            assert_ok!(ChainBridge::transfer_fungible(
                dest_id.clone(),
                resource_id.clone(),
                to.clone(),
                amount.into()
            ));

            assert_events(vec![
                Event::ChainBridge(pallet::Event::<MockRuntime>::ChainWhitelisted(
                    dest_id.clone(),
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::FungibleTransfer(
                    dest_id.clone(),
                    1,
                    resource_id.clone(),
                    amount.into(),
                    to.clone(),
                )),
            ]);
        })
}

#[test]
fn asset_transfer_invalid_chain() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            let chain_id = 2;
            let bad_dest_id = 3;
            let resource_id = [4; 32];

            assert_ok!(ChainBridge::whitelist_chain(
                Origin::root(),
                chain_id.clone()
            ));
            assert_events(vec![Event::ChainBridge(
                pallet::Event::<MockRuntime>::ChainWhitelisted(chain_id.clone()),
            )]);

            assert_noop!(
                ChainBridge::transfer_fungible(
                    bad_dest_id,
                    resource_id.clone(),
                    vec![],
                    U256::zero()
                ),
                Error::<MockRuntime>::ChainNotWhitelisted
            );
        })
}

#[test]
fn add_remove_relayer() {
    TestExternalitiesBuilder::default()
        .build()
        .execute_with(|| {
            assert_ok!(ChainBridge::set_threshold(
                Origin::root(),
                TEST_RELAYER_VOTE_THRESHOLD,
            ));
            assert_eq!(ChainBridge::get_relayer_count(), 0);

            assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_A));
            assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_B));
            assert_ok!(ChainBridge::add_relayer(Origin::root(), RELAYER_C));
            assert_eq!(ChainBridge::get_relayer_count(), 3);

            // Already exists
            assert_noop!(
                ChainBridge::add_relayer(Origin::root(), RELAYER_A),
                Error::<MockRuntime>::RelayerAlreadyExists
            );

            // Confirm removal
            assert_ok!(ChainBridge::remove_relayer(Origin::root(), RELAYER_B));
            assert_eq!(ChainBridge::get_relayer_count(), 2);
            assert_noop!(
                ChainBridge::remove_relayer(Origin::root(), RELAYER_B),
                Error::<MockRuntime>::RelayerInvalid
            );
            assert_eq!(ChainBridge::get_relayer_count(), 2);
            assert_events(vec![
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerAdded(RELAYER_A)),
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerAdded(RELAYER_B)),
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerAdded(RELAYER_C)),
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerRemoved(RELAYER_B)),
            ]);
        })
}

#[test]
fn create_successful_remark_proposal() {
    let src_id: ChainId = 1;
    let r_id = derive_resource_id(src_id, b"remark");

    TestExternalitiesBuilder::default()
        .build_with(src_id, r_id, b"System.remark".to_vec())
        .execute_with(|| {
            let prop_id = 1;

            // Create a dummy system remark proposal
            let proposal = Call::System(SystemCall::remark { remark: vec![10] });

            // Create proposal (& vote)
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_A),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));

            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();

            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Initiated,
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

            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![RELAYER_B],
                status: ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            // Third relayer votes in favour
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_C),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));

            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A, RELAYER_C],
                votes_against: vec![RELAYER_B],
                status: ProposalStatus::Approved,
                expiry: ProposalLifetime::get() + 1,
            };

            assert_eq!(prop, expected);

            assert_events(vec![
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteFor(
                    src_id, prop_id, RELAYER_A,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteAgainst(
                    src_id, prop_id, RELAYER_B,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteFor(
                    src_id, prop_id, RELAYER_C,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::ProposalApproved(
                    src_id, prop_id,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::ProposalSucceeded(
                    src_id, prop_id,
                )),
            ]);
        })
}

#[test]
fn create_unsuccessful_transfer_proposal() {
    let src_id = 1;
    let r_id = derive_resource_id(src_id, b"transfer");

    TestExternalitiesBuilder::default()
        .build_with(src_id, r_id, b"System.remark".to_vec())
        .execute_with(|| {
            let prop_id = 1;

            // Create a dummy system remark proposal
            let proposal = Call::System(SystemCall::remark { remark: vec![11] });

            // Create proposal (& vote)
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_A),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Initiated,
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
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![RELAYER_B],
                status: ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            // Third relayer votes against
            assert_ok!(ChainBridge::reject_proposal(
                Origin::signed(RELAYER_C),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![RELAYER_B, RELAYER_C],
                status: ProposalStatus::Rejected,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            assert_eq!(Balances::free_balance(RELAYER_B), 0);
            assert_eq!(
                Balances::free_balance(ChainBridge::account_id()),
                ENDOWED_BALANCE
            );

            assert_events(vec![
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteFor(
                    src_id, prop_id, RELAYER_A,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteAgainst(
                    src_id, prop_id, RELAYER_B,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteAgainst(
                    src_id, prop_id, RELAYER_C,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::ProposalRejected(
                    src_id, prop_id,
                )),
            ]);
        })
}

#[test]
fn execute_after_threshold_change() {
    let src_id = 1;
    let r_id = derive_resource_id(src_id, b"transfer");

    TestExternalitiesBuilder::default()
        .build_with(src_id, r_id, b"System.remark".to_vec())
        .execute_with(|| {
            let prop_id = 1;

            // Create a dummy system remark proposal
            let proposal = Call::System(SystemCall::remark { remark: vec![11] });

            // Create proposal (& vote)
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_A),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            // Change threshold
            assert_ok!(ChainBridge::set_threshold(Origin::root(), 1));

            // Attempt to execute
            assert_ok!(ChainBridge::eval_vote_state(
                Origin::signed(RELAYER_A),
                prop_id,
                src_id,
                Box::new(proposal.clone())
            ));

            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Approved,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            assert_eq!(Balances::free_balance(RELAYER_B), 0);
            assert_eq!(
                Balances::free_balance(ChainBridge::account_id()),
                ENDOWED_BALANCE
            );

            assert_events(vec![
                Event::ChainBridge(pallet::Event::<MockRuntime>::VoteFor(
                    src_id, prop_id, RELAYER_A,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::RelayerThresholdChanged(1)),
                Event::ChainBridge(pallet::Event::<MockRuntime>::ProposalApproved(
                    src_id, prop_id,
                )),
                Event::ChainBridge(pallet::Event::<MockRuntime>::ProposalSucceeded(
                    src_id, prop_id,
                )),
            ]);
        })
}

#[test]
fn proposal_expires() {
    let src_id = 1;
    let r_id = derive_resource_id(src_id, b"remark");

    TestExternalitiesBuilder::default()
        .build_with(src_id, r_id, b"System.remark".to_vec())
        .execute_with(|| {
            let prop_id = 1;

            // Create a dummy system remark proposal
            let proposal = Call::System(SystemCall::remark { remark: vec![10] });

            // Create proposal (& vote)
            assert_ok!(ChainBridge::acknowledge_proposal(
                Origin::signed(RELAYER_A),
                prop_id,
                src_id,
                r_id,
                Box::new(proposal.clone())
            ));
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            // Increment enough blocks such that now == expiry
            System::set_block_number(ProposalLifetime::get() + 1);

            // Attempt to submit a vote should fail
            assert_noop!(
                ChainBridge::reject_proposal(
                    Origin::signed(RELAYER_B),
                    prop_id,
                    src_id,
                    r_id,
                    Box::new(proposal.clone())
                ),
                Error::<MockRuntime>::ProposalExpired
            );

            // Proposal state should remain unchanged
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            // eval_vote_state should have no effect
            assert_noop!(
                ChainBridge::eval_vote_state(
                    Origin::signed(RELAYER_C),
                    prop_id,
                    src_id,
                    Box::new(proposal.clone())
                ),
                Error::<MockRuntime>::ProposalExpired
            );
            let prop = ChainBridge::get_votes(src_id, (prop_id.clone(), proposal.clone())).unwrap();
            let expected = ProposalVotes {
                votes_for: vec![RELAYER_A],
                votes_against: vec![],
                status: ProposalStatus::Initiated,
                expiry: ProposalLifetime::get() + 1,
            };
            assert_eq!(prop, expected);

            assert_events(vec![mock::Event::ChainBridge(
                pallet::Event::<MockRuntime>::VoteFor(src_id, prop_id, RELAYER_A),
            )]);
        })
}
