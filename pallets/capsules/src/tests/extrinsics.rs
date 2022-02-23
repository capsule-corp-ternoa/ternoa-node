use super::mock::*;
use crate::{tests::mock, CapsuleData, Error};
use frame_support::{assert_noop, assert_ok, error::BadOrigin};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;

#[test]
fn create_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		// Initial state
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let ipfs_reference = vec![60];
		let nft_id = 0;
		let data = CapsuleData::new(ALICE, ipfs_reference.clone());
		let ledger = vec![(nft_id, TernoaCapsules::capsule_mint_fee())];
		assert_eq!(TernoaCapsules::capsules(&nft_id), None);
		assert_eq!(TernoaCapsules::ledgers(&ALICE), None);

		// Happy path
		let ok = TernoaCapsules::create(alice.clone(), vec![50], ipfs_reference, None);
		assert_ok!(ok);
		assert_eq!(TernoaCapsules::capsules(&nft_id), Some(data));
		assert_eq!(TernoaCapsules::ledgers(&ALICE), Some(ledger));
	})
}

#[test]
fn create_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 10000), (BOB, 101)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();

			// Unhappy too short ipfs reference
			let ok = TernoaCapsules::create(bob.clone(), vec![], vec![], None);
			assert_noop!(ok, Error::<Test>::TooShortIpfsReference);

			// Unhappy too longs ipfs reference
			let long = vec![1, 2, 3, 4, 5, 6, 7];
			let ok = TernoaCapsules::create(bob.clone(), vec![], long, None);
			assert_noop!(ok, Error::<Test>::TooLongIpfsReference);

			// Unhappy not enough caps to reserve a capsule
			let ok = TernoaCapsules::create(bob.clone(), vec![], vec![1], None);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);

			// Unhappy nft creation failed
			let ok = TernoaCapsules::create(alice.clone(), vec![], vec![1], None);
			assert_noop!(ok, ternoa_nfts::Error::<Test>::IPFSReferenceIsTooShort);
		})
}

#[test]
fn create_caps_transfer() {
	ExtBuilder::default().caps(vec![(ALICE, 10001)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let capsule_fee = TernoaCapsules::capsule_mint_fee();
		let nft_fee = TernoaNFTs::nft_mint_fee();
		let balance = Balances::free_balance(ALICE);
		let pallet_id = TernoaCapsules::account_id();
		assert_ne!(capsule_fee, 0);
		assert_ne!(nft_fee, 0);
		assert_eq!(Balances::free_balance(pallet_id), 0);

		// Funds are transferred
		let ok = TernoaCapsules::create(alice.clone(), vec![50], vec![25], None);
		assert_ok!(ok);
		assert_eq!(Balances::free_balance(ALICE), balance - capsule_fee - nft_fee);
		assert_eq!(Balances::free_balance(pallet_id), capsule_fee);
	})
}

#[test]
fn create_transactional() {
	ExtBuilder::default().caps(vec![(ALICE, 1002)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let balance = Balances::free_balance(ALICE);
		let capsule_fee = TernoaCapsules::capsule_mint_fee();
		let nft_fee = TernoaNFTs::nft_mint_fee();
		let pallet_id = TernoaCapsules::account_id();

		// Lets make sure that Alice has enough to reserve but not to reserve and mint and NFT
		assert!(balance > capsule_fee);
		assert!(balance < (capsule_fee + nft_fee));

		// Trigger an error
		let ok = TernoaCapsules::create(alice.clone(), vec![], vec![1], None);
		assert_noop!(ok, ternoa_nfts::Error::<Test>::IPFSReferenceIsTooShort);

		// She should not have lost any caps
		assert_eq!(Balances::free_balance(ALICE), balance);
		assert_eq!(Balances::free_balance(pallet_id), 0);
	})
}

#[test]
fn create_from_nft_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		// Initial state
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let nft_id = help::create_nft_fast(alice.clone());
		let ipfs_reference = vec![60];
		assert_eq!(TernoaCapsules::capsules(&nft_id), None);
		assert_eq!(TernoaCapsules::ledgers(&ALICE), None);

		// Happy path
		let data = CapsuleData::new(ALICE, ipfs_reference.clone());
		let ledger = vec![(nft_id, TernoaCapsules::capsule_mint_fee())];

		let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, ipfs_reference);
		assert_ok!(ok);
		assert_eq!(TernoaCapsules::capsules(&nft_id), Some(data));
		assert_eq!(TernoaCapsules::ledgers(&ALICE), Some(ledger));
	})
}

#[test]
fn create_from_nft_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 10000), (BOB, 101)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();

			// Unhappy too short ipfs reference
			let nft_id = help::create_nft_fast(alice.clone());
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![]);
			assert_noop!(ok, Error::<Test>::TooShortIpfsReference);

			// Unhappy too longs ipfs reference
			let long = vec![1, 2, 3, 4, 5, 6, 7];
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, long);
			assert_noop!(ok, Error::<Test>::TooLongIpfsReference);

			// Unhappy not nft owner
			let nft_id = help::create_nft_fast(bob.clone());
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![25]);
			assert_noop!(ok, Error::<Test>::NotOwner);

			// Unhappy nft is listed for sale
			let nft_id = help::create_nft_fast(alice.clone());
			<TernoaNFTs as NFTTrait>::set_listed_for_sale(nft_id, true).unwrap();
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![25]);
			assert_noop!(ok, Error::<Test>::ListedForSale);

			// Unhappy nft is in transmission
			let nft_id = help::create_nft_fast(alice.clone());
			<TernoaNFTs as NFTTrait>::set_in_transmission(nft_id, true).unwrap();
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![25]);
			assert_noop!(ok, Error::<Test>::InTransmission);

			// Unhappy nft is already a capsule
			let nft_id = help::create_nft_fast(alice.clone());
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![25]);
			assert_ok!(ok);
			let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![30]);
			assert_noop!(ok, Error::<Test>::CapsuleAlreadyExists);

			// Unhappy not enough caps to reserve a capsule
			let nft_id = help::create_nft_fast(bob.clone());
			let ok = TernoaCapsules::create_from_nft(bob.clone(), nft_id, vec![30]);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);
		})
}

#[test]
fn create_from_nft_caps_transfer() {
	ExtBuilder::default().caps(vec![(ALICE, 10001)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let capsule_fee = TernoaCapsules::capsule_mint_fee();
		let pallet_id = TernoaCapsules::account_id();
		assert_ne!(capsule_fee, 0);
		assert_eq!(Balances::free_balance(pallet_id), 0);

		// Funds are transferred
		let nft_id = help::create_nft_fast(alice.clone());
		let balance = Balances::free_balance(ALICE);
		let ok = TernoaCapsules::create_from_nft(alice.clone(), nft_id, vec![50]);
		assert_ok!(ok);
		assert_eq!(Balances::free_balance(ALICE), balance - capsule_fee);
		assert_eq!(Balances::free_balance(pallet_id), capsule_fee);
	})
}

#[test]
fn remove_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		// Initial state
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let nft_id_1 = help::create_capsule_fast(alice.clone());
		let nft_id_2 = help::create_capsule_fast(alice.clone());
		let ledger = vec![(nft_id_2, TernoaCapsules::capsule_mint_fee())];

		// Happy path delete one nft id associated with that owner
		assert_ok!(TernoaCapsules::remove(alice.clone(), nft_id_1));
		assert_eq!(TernoaCapsules::capsules(&nft_id_1), None);
		assert_eq!(TernoaCapsules::ledgers(&ALICE), Some(ledger));

		// Happy path delete last nft id associated with that owner
		assert_ok!(TernoaCapsules::remove(alice.clone(), nft_id_2));
		assert_eq!(TernoaCapsules::capsules(&nft_id_2), None);
		assert_eq!(TernoaCapsules::ledgers(&ALICE), None);
	})
}

#[test]
fn remove_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 10000), (BOB, 10000)])
		.build()
		.execute_with(|| {
			// Initial state
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let pallet_id = TernoaCapsules::account_id();
			let bob_nft_id = help::create_capsule_fast(bob.clone());
			let alice_nft_id = help::create_capsule_fast(alice.clone());

			// Unhappy not owner
			let ok = TernoaCapsules::remove(alice.clone(), bob_nft_id);
			assert_noop!(ok, Error::<Test>::NotOwner);

			// Unhappy Pallet doesn't have enough caps (this should never happen)
			let ok = Balances::set_balance(Origin::root(), pallet_id, 0, 0);
			assert_ok!(ok);
			assert_eq!(Balances::free_balance(pallet_id), 0);
			let ok = TernoaCapsules::remove(alice.clone(), alice_nft_id);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);
		})
}

#[test]
fn remove_caps_transfer() {
	ExtBuilder::default().caps(vec![(ALICE, 10001)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		let nft_id = help::create_capsule_fast(alice.clone());
		let fee = TernoaCapsules::ledgers(ALICE).unwrap()[0].1;
		let pallet_id = TernoaCapsules::account_id();

		let pallet_balance = Balances::free_balance(pallet_id);
		let alice_balance = Balances::free_balance(ALICE);

		// Funds are transferred
		assert_ok!(TernoaCapsules::remove(alice.clone(), nft_id));
		assert_eq!(Balances::free_balance(ALICE), alice_balance + fee);
		assert_eq!(Balances::free_balance(pallet_id), pallet_balance - fee);
	})
}

#[test]
fn add_funds_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		// Initial state
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let nft_id = help::create_capsule_fast(alice.clone());
		let fee = TernoaCapsules::capsule_mint_fee();
		let ledger = vec![(nft_id, fee)];
		assert_eq!(TernoaCapsules::ledgers(&ALICE), Some(ledger));

		// Happy path
		let add = 55;
		let ledger = vec![(nft_id, fee + add)];
		assert_ok!(TernoaCapsules::add_funds(alice.clone(), nft_id, add));
		assert_eq!(TernoaCapsules::ledgers(&ALICE), Some(ledger));
	})
}

#[test]
fn add_funds_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 10000), (BOB, 10000)])
		.build()
		.execute_with(|| {
			// Initial state
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let bob_nft_id = help::create_capsule_fast(bob.clone());
			let alice_nft_id = help::create_capsule_fast(alice.clone());
			let add = 10000000;

			// Unhappy not owner
			let ok = TernoaCapsules::add_funds(alice.clone(), bob_nft_id, add);
			assert_noop!(ok, Error::<Test>::NotOwner);

			// Unhappy caller doesn't have enough caps
			let ok = TernoaCapsules::add_funds(alice.clone(), alice_nft_id, add);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);
		})
}

#[test]
fn add_funds_caps_transfer() {
	ExtBuilder::default().caps(vec![(ALICE, 10001)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		let nft_id = help::create_capsule_fast(alice.clone());
		let pallet_id = TernoaCapsules::account_id();

		let alice_balance = Balances::free_balance(ALICE);
		let pallet_balance = Balances::free_balance(pallet_id);

		// Funds are transferred
		let add = 1010;
		assert_ok!(TernoaCapsules::add_funds(alice.clone(), nft_id, add));
		assert_eq!(Balances::free_balance(ALICE), alice_balance - add);
		assert_eq!(Balances::free_balance(pallet_id), pallet_balance + add);
	})
}

#[test]
fn set_ipfs_reference_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		// Initial state
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
		let nft_id = help::create_capsule_fast(alice.clone());
		let data = TernoaCapsules::capsules(nft_id).unwrap();
		let old_reference = data.ipfs_reference.clone();
		let new_reference = vec![67];
		assert_ne!(old_reference, new_reference);

		// Happy path
		let ok = TernoaCapsules::set_ipfs_reference(alice.clone(), nft_id, new_reference.clone());
		assert_ok!(ok);
		assert_eq!(TernoaCapsules::capsules(nft_id).unwrap().ipfs_reference, new_reference);
	})
}

#[test]
fn set_ipfs_reference_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 10000), (BOB, 10000)])
		.build()
		.execute_with(|| {
			// Initial state
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let nft_id = help::create_capsule_fast(alice.clone());

			// Unhappy too short ipfs reference
			let ok = TernoaCapsules::set_ipfs_reference(alice.clone(), nft_id, vec![]);
			assert_noop!(ok, Error::<Test>::TooShortIpfsReference);

			// Unhappy too longs ipfs reference
			let long = vec![1, 2, 3, 4, 5, 6, 7];
			let ok = TernoaCapsules::set_ipfs_reference(alice.clone(), nft_id, long);
			assert_noop!(ok, Error::<Test>::TooLongIpfsReference);

			// Unhappy not nft owner
			let bob_nft_id = help::create_capsule_fast(bob.clone());
			let ok = TernoaCapsules::set_ipfs_reference(alice.clone(), bob_nft_id, vec![1]);
			assert_noop!(ok, Error::<Test>::NotOwner);
		})
}

#[test]
fn set_capsule_mint_fee_happy() {
	ExtBuilder::default().build().execute_with(|| {
		// Happy path
		let old_mint_fee = TernoaCapsules::capsule_mint_fee();
		let new_mint_fee = 654u128;
		assert_eq!(TernoaCapsules::capsule_mint_fee(), old_mint_fee);

		let ok = TernoaCapsules::set_capsule_mint_fee(mock::Origin::root(), new_mint_fee);
		assert_ok!(ok);
		assert_eq!(TernoaCapsules::capsule_mint_fee(), new_mint_fee);
	})
}

#[test]
fn set_capsule_mint_fee_unhappy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		// Unhappy non root user tries to modify the mint fee
		let ok = TernoaCapsules::set_capsule_mint_fee(alice.clone(), 654);
		assert_noop!(ok, BadOrigin);
	})
}
