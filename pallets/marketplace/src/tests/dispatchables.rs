use super::mock::*;
use crate::tests::mock;
use crate::{
    Error, MarketplaceIdGenerator, MarketplaceType, Marketplaces, NFTCurrency, NFTCurrencyCombined,
    NFTCurrencyId, NFTsForSale, SaleInformation,
};
use frame_support::instances::Instance1;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

const NFT_ID_1: u32 = 0;
const NFT_ID_2: u32 = 1;
const NFT_ID_3: u32 = 2;
const NFT_ID_4: u32 = 3;
const NFT_ID_5: u32 = 4;
const NFT_ID_6: u32 = 5;
const NFT_ID_7: u32 = 6;
const NFT_ID_8: u32 = 7;
const CAPS_ID: NFTCurrencyId = NFTCurrencyId::Caps;
const TIIME_ID: NFTCurrencyId = NFTCurrencyId::Tiime;

type MPT = MarketplaceType;

#[test]
fn list_register_price() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let caps = NFTCurrency::Caps(50);
            let sale = SaleInformation::new(ALICE, caps, 0);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps, None));
            assert_eq!(Marketplace::nft_for_sale(0), sale);
        })
}

#[test]
fn buy_transfer_funds_to_owner() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let caps = NFTCurrency::Caps(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps, None));
            assert_ok!(Marketplace::buy(bob, NFT_ID_1, CAPS_ID));

            assert_eq!(Balances::free_balance(ALICE), 150);
            assert_eq!(Balances::free_balance(BOB), 50);
        })
}

#[test]
fn buy_change_owner() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let caps = NFTCurrency::Caps(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps, None));
            assert_ok!(Marketplace::buy(bob, NFT_ID_1, CAPS_ID));
            assert_eq!(NFTs::data(0).owner, BOB);
        })
}

#[test]
fn buy_unlock_nft() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let caps = NFTCurrency::Caps(50);

            assert_ok!(Marketplace::list(alice, NFT_ID_1, caps, None));
            assert_ok!(Marketplace::buy(bob, NFT_ID_1, CAPS_ID));
            assert_eq!(NFTs::data(NFT_ID_1).locked, false);
        })
}

#[test]
fn unlist_unlocks_nft() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let caps = NFTCurrency::Caps(50);

            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, caps, None));
            assert_ok!(Marketplace::unlist(alice, NFT_ID_1));
            assert_eq!(NFTs::data(0).locked, false);
        })
}

#[test]
fn unlist_remove_from_for_sale() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let caps = NFTCurrency::Caps(50);

            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, caps, None));
            assert_ok!(Marketplace::unlist(alice, NFT_ID_1));
            assert_eq!(NFTsForSale::<Test>::contains_key(NFT_ID_1), false);
        })
}

#[test]
fn bought_nft_is_not_listed_anymore() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 1)])
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let seller: mock::Origin = RawOrigin::Signed(ALICE).into();
            let buyer: mock::Origin = RawOrigin::Signed(BOB).into();

            assert_ok!(Marketplace::list(
                seller,
                NFT_ID_1,
                NFTCurrency::Caps(100),
                None
            ));
            assert_ok!(Marketplace::buy(buyer.clone(), NFT_ID_1, CAPS_ID));
            assert_noop!(
                Marketplace::buy(buyer, NFT_ID_1, CAPS_ID),
                Error::<Test>::NftNotForSale,
            );
        })
}

#[test]
fn list_nft() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 5), (BOB, 1)])
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .tiime(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let caps = NFTCurrency::Caps(10);
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Alice should NOT be able to list an nft that she does not own.
            assert_noop!(
                Marketplace::list(alice.clone(), 100, caps, None),
                Error::<Test>::NotNftOwner,
            );

            // Alice should be able list her own nft.
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, caps, None));

            // Alice should NOT be able to list her own nft again.
            assert_noop!(
                Marketplace::list(alice.clone(), NFT_ID_1, caps, None),
                ternoa_nfts::Error::<Test>::Locked,
            );

            // Alice should be able to list her second nft with Tiime.
            let tiime = NFTCurrency::Tiime(10);
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_2, tiime, None));

            // Alice should be able to list her third nft with Combined currency.
            let combined = NFTCurrency::Combined(NFTCurrencyCombined::new(10, 10));
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_3, combined, None));

            // Alice should NOT be able to list nfts on a user-marketplace that does not exist.
            assert_noop!(
                Marketplace::list(alice.clone(), NFT_ID_4, caps, Some(20)),
                Error::<Test>::UnknownMarketplace,
            );

            // Alice should be able to list nfts on user-marketplaces.
            assert_ok!(Marketplace::create(bob.clone(), MPT::Public, 0, "A".into()));
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_4, caps, Some(1)));

            // Alice should be able to list nfts on private user-marketplaces with access.
            let ok = Marketplace::create(bob.clone(), MPT::Private, 0, "A".into());
            assert_ok!(ok);
            let ok = Marketplace::add_account_to_allow_list(bob.clone(), 2, ALICE);
            assert_ok!(ok);
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_5, caps, Some(2)));

            // Bob should NOT be able to list nfts on private user-marketplaces without access.
            let ok = Marketplace::list(bob.clone(), NFT_ID_6, caps, Some(2));
            assert_noop!(ok, Error::<Test>::NotAllowed);
        })
}

#[test]
fn unlist_nft() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 3)])
        .caps(vec![(ALICE, 100), (BOB, 100)])
        .tiime(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let nft_price = NFTCurrency::Caps(10);
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // Bob should NOT be able to unlist an nft that he does not own.
            assert_noop!(
                Marketplace::unlist(bob.clone(), NFT_ID_1),
                Error::<Test>::NotNftOwner,
            );

            // Alice should NOT be able to unlist an nft that she owns but it's not listed.
            assert_noop!(
                Marketplace::unlist(alice.clone(), NFT_ID_1),
                Error::<Test>::NftNotForSale,
            );

            // Alice should be able to unlist her own listed nft.
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, nft_price, None));
            assert_ok!(Marketplace::unlist(alice.clone(), NFT_ID_1,));
        })
}

#[test]
fn buy_nft() {
    ExtBuilder::default()
        .nfts(vec![(ALICE, 8)])
        .caps(vec![(ALICE, 100), (BOB, 100), (DAVE, 1000)])
        .tiime(vec![(ALICE, 100), (BOB, 100)])
        .build()
        .execute_with(|| {
            let caps = NFTCurrency::Caps(200);
            let tiime = NFTCurrency::Tiime(200);
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();
            let dave: mock::Origin = RawOrigin::Signed(DAVE).into();

            // Bob should NOT be able to buy nfts that are not listed.
            assert_noop!(
                Marketplace::buy(bob.clone(), NFT_ID_1, CAPS_ID),
                Error::<Test>::NftNotForSale,
            );

            // Alice should NOT be able to buy her own listed nfts.
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_1, caps, None));
            assert_noop!(
                Marketplace::buy(alice.clone(), NFT_ID_1, CAPS_ID),
                Error::<Test>::NftAlreadyOwned,
            );

            // Bob should NOT be able to buy nfts (that are listed with caps) with tiime. And Vice versa.
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_2, tiime, None));
            assert_noop!(
                Marketplace::buy(bob.clone(), NFT_ID_1, TIIME_ID),
                Error::<Test>::WrongCurrencyUsed,
            );
            assert_noop!(
                Marketplace::buy(bob.clone(), NFT_ID_2, CAPS_ID),
                Error::<Test>::WrongCurrencyUsed,
            );

            // Bob should NOT be able to buy nfts that are too expensive.
            assert_noop!(
                Marketplace::buy(bob.clone(), NFT_ID_1, CAPS_ID),
                pallet_balances::Error::<Test>::InsufficientBalance,
            );

            assert_noop!(
                Marketplace::buy(bob.clone(), NFT_ID_2, TIIME_ID),
                pallet_balances::Error::<Test, Instance1>::InsufficientBalance,
            );

            // Bob should be able to buy nfts that are listed with either caps or time.
            let caps = NFTCurrency::Caps(10);
            let time = NFTCurrency::Tiime(10);
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_3, caps, None));
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_4, time, None));
            assert_ok!(Marketplace::buy(bob.clone(), NFT_ID_3, CAPS_ID));
            assert_ok!(Marketplace::buy(bob.clone(), NFT_ID_4, TIIME_ID));

            // Bob should be able to buy nfts (that are listed with combined currency) with either caps or tiime.
            let combined = NFTCurrency::Combined(NFTCurrencyCombined::new(10, 10));
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_5, combined, None));
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_6, combined, None));
            assert_ok!(Marketplace::buy(bob.clone(), NFT_ID_5, CAPS_ID));
            assert_ok!(Marketplace::buy(bob.clone(), NFT_ID_6, TIIME_ID));

            // Dave should be able to buy nfts that are listed in private markets without being in the allow list.
            let commission_fee = 5;
            assert_ok!(Marketplace::create(
                bob.clone(),
                MPT::Private,
                commission_fee,
                "A".into()
            ));
            assert_ok!(Marketplace::add_account_to_allow_list(
                bob.clone(),
                1,
                ALICE
            ));
            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_7, caps, Some(1)));
            assert_ok!(Marketplace::buy(dave.clone(), NFT_ID_7, CAPS_ID));
            assert_ok!(Marketplace::add_account_to_allow_list(bob.clone(), 1, DAVE));

            // Dave should be able to buy nfts from private markets if he is in the allow list.
            // He also needs to pay the commission fee.
            let dave_balance = Balances::free_balance(DAVE);
            let alice_balance = Balances::free_balance(ALICE);
            let bob_balance = Balances::free_balance(BOB);

            let price = caps.caps().unwrap();
            let commission = price / 100 * commission_fee as u64;

            let expected_dave_balance = dave_balance - price;
            let expected_alice_balance = alice_balance + (price - commission);
            let expected_bob_balance = bob_balance + commission;

            assert_ok!(Marketplace::list(alice.clone(), NFT_ID_8, caps, Some(1)));
            assert_ok!(Marketplace::buy(dave.clone(), NFT_ID_8, CAPS_ID));
            assert_eq!(Balances::free_balance(DAVE), expected_dave_balance);
            assert_eq!(Balances::free_balance(ALICE), expected_alice_balance);
            assert_eq!(Balances::free_balance(BOB), expected_bob_balance);
        })
}

#[test]
fn create() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
            let bob: mock::Origin = RawOrigin::Signed(BOB).into();

            // The default marketplace has the ID 0.
            assert_eq!(MarketplaceIdGenerator::<Test>::get(), 0);

            // Alice should be able to create a marketplace if she has enough tokens.
            assert_ok!(Marketplace::create(
                alice.clone(),
                MPT::Public,
                0,
                "A".into()
            ));
            assert_eq!(MarketplaceIdGenerator::<Test>::get(), 1);
            assert_eq!(Marketplaces::<Test>::get(1).unwrap().owner, ALICE);

            // Bob should NOT be able to create a marketplace since he doesn't have enough tokens.
            assert_noop!(
                Marketplace::create(bob.clone(), MPT::Public, 0, "A".into()),
                pallet_balances::Error::<Test>::InsufficientBalance,
            );
        })
}

#[test]
fn change_marketplace_owner() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100), (DAVE, 100)])
        .marketplace(vec![
            (ALICE, MPT::Public, 0, "A".into()),
            (DAVE, MPT::Public, 0, "A".into()),
        ])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Alice should be able to give her marketplace to someone else.
            assert_ok!(Marketplace::change_owner(alice.clone(), 1, BOB));

            // Alice should NOT be able to give a marketplace not owned by her to someone else.
            let ok = Marketplace::change_owner(alice.clone(), 2, BOB);
            assert_noop!(ok, Error::<Test>::NotMarketplaceOwner);

            // Alice should NOT be able to give a non existing marketplace to someone else.
            let ok = Marketplace::change_owner(alice.clone(), 3, BOB);
            assert_noop!(ok, Error::<Test>::UnknownMarketplace);
        })
}

#[test]
fn add_account() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1_000), (DAVE, 1_000)])
        .marketplace(vec![
            (ALICE, MPT::Private, 0, "".into()),
            (ALICE, MPT::Public, 0, "".into()),
            (BOB, MPT::Public, 0, "".into()),
        ])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Alice should be able to add Dave to her private marketplace.
            let ok = Marketplace::add_account_to_allow_list(alice.clone(), 1, DAVE);
            assert_ok!(ok);

            // Alice should NOT be able to add Dave to her public marketplace.
            let ok = Marketplace::add_account_to_allow_list(alice.clone(), 2, DAVE);
            assert_noop!(ok, Error::<Test>::UnsupportedMarketplace);

            // Alice should NOT be able to add Dave someones else marketplace.
            let ok = Marketplace::add_account_to_allow_list(alice.clone(), 3, DAVE);
            assert_noop!(ok, Error::<Test>::NotMarketplaceOwner);
        })
}

#[test]
fn remove_account() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 1_000), (DAVE, 1_000)])
        .marketplace(vec![
            (ALICE, MPT::Private, 0, "".into()),
            (ALICE, MPT::Public, 0, "".into()),
            (BOB, MPT::Private, 0, "".into()),
        ])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Alice should be able to remove Dave from her private marketplace.
            let ok = Marketplace::add_account_to_allow_list(alice.clone(), 1, DAVE);
            assert_ok!(ok);
            let ok = Marketplace::remove_account_from_allow_list(alice.clone(), 1, DAVE);
            assert_ok!(ok);

            // Alice should NOT be able to remove Dave if dave is not on the allow list.
            let ok = Marketplace::remove_account_from_allow_list(alice.clone(), 1, DAVE);
            assert_noop!(ok, Error::<Test>::AccountNotFound);

            // Alice should NOT be able to remove Dave from a marketplace that she does not own.
            let ok = Marketplace::remove_account_from_allow_list(alice.clone(), 3, DAVE);
            assert_noop!(ok, Error::<Test>::NotMarketplaceOwner);

            // Alice should NOT be able to remove Dave from a marketplace that does not use the allow list.
            let ok = Marketplace::remove_account_from_allow_list(alice.clone(), 2, DAVE);
            assert_noop!(ok, Error::<Test>::UnsupportedMarketplace);
        })
}

#[test]
fn change_marketplace_type() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100), (DAVE, 100)])
        .marketplace(vec![
            (ALICE, MPT::Public, 0, "".into()),
            (DAVE, MPT::Public, 0, "".into()),
        ])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Alice should be able to change her marketplace.
            let ok = Marketplace::change_market_type(alice.clone(), 1, MPT::Private);
            assert_ok!(ok);

            // Alice should NOT be able to change the type of a marketplace not owned by her.
            let ok = Marketplace::change_market_type(alice.clone(), 2, MPT::Private);
            assert_noop!(ok, Error::<Test>::NotMarketplaceOwner);

            // Alice should NOT be able to change the type of a marketplace that does not exist.
            let ok = Marketplace::change_market_type(alice.clone(), 3, MPT::Private);
            assert_noop!(ok, Error::<Test>::UnknownMarketplace);
        })
}

#[test]
fn set_name() {
    ExtBuilder::default()
        .caps(vec![(ALICE, 100)])
        .marketplace(vec![
            (ALICE, MPT::Public, 0, "".into()),
            (DAVE, MPT::Public, 0, "".into()),
        ])
        .build()
        .execute_with(|| {
            let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

            // Alice should be able to change her marketplace name.
            let ok = Marketplace::set_name(alice.clone(), 1, "Dance, boogie wonderland".into());
            assert_ok!(ok);

            // Alice should NOT be able to change the name if the name has less the X characters.
            let ok = Marketplace::set_name(alice.clone(), 1, "".into());
            assert_noop!(ok, Error::<Test>::TooShortName);

            // Alice should NOT be able to change the name if the name has more the Y characters.
            let ok = Marketplace::set_name(
                alice.clone(),
                1,
                "I find romance when I start to dance in boogie wonderland".into(),
            );
            assert_noop!(ok, Error::<Test>::TooLongName);

            // Alice should NOT be able to change the name of a marketplace not owned by her.
            let ok = Marketplace::set_name(alice.clone(), 2, "Dance, boogie wonderland".into());
            assert_noop!(ok, Error::<Test>::NotMarketplaceOwner);

            // Alice should NOT be able to change the type of a marketplace that does not exist.
            let ok = Marketplace::set_name(alice.clone(), 3, "Dance, boogie wonderland".into());
            assert_noop!(ok, Error::<Test>::UnknownMarketplace);
        })
}
