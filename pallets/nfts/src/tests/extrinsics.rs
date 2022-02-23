use super::mock::*;
use crate::tests::mock;
use crate::{Error, Event as NFTsEvent, NFTData, NFTSeriesDetails};
use frame_support::error::BadOrigin;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::NFTTrait;

fn origin(account: u64) -> mock::Origin {
	RawOrigin::Signed(account).into()
}

/* fn root() -> mock::Origin {
	RawOrigin::Root.into()
} */

mod transfer {
	use super::*;

	#[test]
	fn cannot_transfer_lent_nfts() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();
			assert_ok!(NFTs::set_viewer(nft_id, Some(BOB)));

			let ok = NFTs::transfer(origin(ALICE), nft_id, BOB);
			assert_noop!(ok, Error::<Test>::CannotTransferLentNFTs);
		})
	}
}

mod burn {
	use super::*;

	#[test]
	fn cannot_burn_lent_nfts() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();
			assert_ok!(NFTs::set_viewer(nft_id, Some(BOB)));

			let ok = NFTs::burn(origin(ALICE), nft_id);
			assert_noop!(ok, Error::<Test>::CannotBurnLentNFTs);
		})
	}
}

mod lend {
	use super::*;

	#[test]
	fn lend() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();
			let mut nft = NFTs::data(nft_id).unwrap();
			let viewer = Some(BOB);

			assert_ok!(NFTs::lend(origin(ALICE), nft_id, viewer.clone()));

			// Storage
			nft.viewer = viewer.clone();
			assert_eq!(NFTs::data(nft_id), Some(nft));

			// Event
			let event = NFTsEvent::NFTLent { nft_id, viewer };
			let event = Event::NFTs(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn nft_not_found() {
		ExtBuilder::new_build(vec![]).execute_with(|| {
			let ok = NFTs::lend(origin(ALICE), INVALID_NFT_ID, None);
			assert_noop!(ok, Error::<Test>::NFTNotFound);
		})
	}

	#[test]
	fn not_the_nft_owner() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();

			let ok = NFTs::lend(origin(BOB), nft_id, None);
			assert_noop!(ok, Error::<Test>::NotTheNFTOwner);
		})
	}

	#[test]
	fn cannot_lend_nfts_listed_for_sale() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();
			assert_ok!(NFTs::set_listed_for_sale(nft_id, true));

			let ok = NFTs::lend(origin(ALICE), nft_id, None);
			assert_noop!(ok, Error::<Test>::CannotLendNFTsListedForSale);
		})
	}

	#[test]
	fn cannot_lend_capsules() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();
			assert_ok!(NFTs::set_converted_to_capsule(nft_id, true));

			let ok = NFTs::lend(origin(ALICE), nft_id, None);
			assert_noop!(ok, Error::<Test>::CannotLendCapsules);
		})
	}

	#[test]
	fn cannot_lend_nfts_in_transmission() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();
			assert_ok!(NFTs::set_in_transmission(nft_id, true));

			let ok = NFTs::lend(origin(ALICE), nft_id, None);
			assert_noop!(ok, Error::<Test>::CannotLendNFTsInTransmission);
		})
	}

	#[test]
	fn cannot_lend_nfts_to_yourself() {
		ExtBuilder::new_build(vec![(ALICE, 100)]).execute_with(|| {
			let nft_id = NFTs::create_nft(ALICE, vec![0], None).unwrap();

			let ok = NFTs::lend(origin(ALICE), nft_id, Some(ALICE));
			assert_noop!(ok, Error::<Test>::CannotLendNFTsToYourself);
		})
	}
}

#[test]
fn create_happy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 1000), (BOB, 1), (CHAD, 100)])
		.build()
		.execute_with(|| {
			// Initial state
			assert_eq!(NFTs::nft_id_generator(), 0);
			assert_eq!(NFTs::series_id_generator(), 0);

			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

			// Happy path NFT with series
			let series = NFTSeriesDetails::new(ALICE, true);
			let data = NFTData::new_default(ALICE, vec![1], vec![50]);
			let alice_balance = Balances::free_balance(ALICE);

			let ok = NFTs::create(
				alice.clone(),
				data.ipfs_reference.clone(),
				Some(data.series_id.clone()),
			);
			assert_ok!(ok);

			assert_eq!(NFTs::nft_id_generator(), 1);
			assert_eq!(NFTs::series(&data.series_id), Some(series));
			assert_eq!(NFTs::data(0), Some(data));
			assert_eq!(NFTs::series_id_generator(), 0);
			assert_eq!(Balances::free_balance(ALICE), alice_balance - NFTs::nft_mint_fee());

			// Happy path NFT without series
			let data = NFTData::new_default(ALICE, vec![0], vec![48]);
			let series = NFTSeriesDetails::new(ALICE, true);

			let ok = NFTs::create(alice.clone(), vec![0], None);
			assert_ok!(ok);

			assert_eq!(NFTs::series(&data.series_id), Some(series));
			assert_eq!(NFTs::data(1), Some(data));
			assert_eq!(NFTs::series_id_generator(), 1);
		})
}

#[test]
fn create_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 1), (BOB, 100), (CHAD, 100)])
		.build()
		.execute_with(|| {
			// Initial state
			assert_eq!(NFTs::nft_id_generator(), 0);
			assert_eq!(NFTs::series_id_generator(), 0);

			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();

			// Unhappy too short name
			let ok = NFTs::create(alice.clone(), vec![], None);
			assert_noop!(ok, Error::<Test>::IPFSReferenceIsTooShort);

			// Unhappy too long name
			let ok = NFTs::create(alice.clone(), vec![1, 2, 3, 4, 5, 6], None);
			assert_noop!(ok, Error::<Test>::IPFSReferenceIsTooLong);

			// Unhappy not enough caps to mint an NFT
			let ok = NFTs::create(alice.clone(), vec![1], None);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);

			// Unhappy not the owner of series
			let series_id = Some(vec![50]);
			<NFTs as NFTTrait>::create_nft(CHAD, vec![50], series_id.clone()).unwrap();

			let ok = NFTs::create(bob.clone(), vec![1], series_id);
			assert_noop!(ok, Error::<Test>::NotTheSeriesOwner);
			assert_eq!(Balances::free_balance(BOB), 100);

			// Unhappy cannot create nfts with complete (locked) series
			let series_id = Some(vec![51]);
			<NFTs as NFTTrait>::create_nft(BOB, vec![50], series_id.clone()).unwrap();
			NFTs::finish_series(bob.clone(), series_id.clone().unwrap()).unwrap();

			let ok = NFTs::create(bob.clone(), vec![1], series_id.clone());
			assert_noop!(ok, Error::<Test>::CannotCreateNFTsWithCompletedSeries);
		})
}

#[test]
fn transfer_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 1000)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		// Happy path transfer
		let series_id = vec![2];
		let nft_id =
			<NFTs as NFTTrait>::create_nft(ALICE, vec![1], Some(series_id.clone())).unwrap();
		NFTs::finish_series(alice.clone(), series_id).unwrap();
		let nft = NFTs::data(nft_id).unwrap();
		assert_eq!(nft.owner, ALICE);
		assert_eq!(nft.creator, ALICE);

		assert_ok!(NFTs::transfer(alice.clone(), nft_id, BOB));
		let nft = NFTs::data(nft_id).unwrap();
		assert_eq!(nft.owner, BOB);
		assert_eq!(nft.creator, ALICE);
	})
}

#[test]
fn transfer_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 100), (BOB, 100)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

			// Unhappy unknown NFT
			let ok = NFTs::transfer(alice.clone(), 1001, BOB);
			assert_noop!(ok, Error::<Test>::NFTNotFound);

			// Unhappy draft(open) series
			let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
			let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
			assert_noop!(ok, Error::<Test>::CannotTransferNFTsInUncompletedSeries);

			// Unhappy NFT is listed for sale
			let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
			<NFTs as NFTTrait>::set_listed_for_sale(nft_id, true).unwrap();

			let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
			assert_noop!(ok, Error::<Test>::CannotTransferNFTsListedForSale);

			// Unhappy NFT is converted to a capsule
			let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
			<NFTs as NFTTrait>::set_converted_to_capsule(nft_id, true).unwrap();

			let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
			assert_noop!(ok, Error::<Test>::CannotTransferCapsules);

			// Unhappy NFT is in transmission
			let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![0], None).unwrap();
			<NFTs as NFTTrait>::set_in_transmission(nft_id, true).unwrap();

			let ok = NFTs::transfer(alice.clone(), nft_id, BOB);
			assert_noop!(ok, Error::<Test>::CannotTransferNFTsInTransmission);
		})
}

#[test]
fn burn_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 1000)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		// Happy path transfer
		let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![1], Some(vec![2])).unwrap();
		assert_eq!(NFTs::data(nft_id).is_some(), true);

		assert_ok!(NFTs::burn(alice.clone(), nft_id));
		assert_eq!(NFTs::data(nft_id).is_some(), false);
	})
}

#[test]
fn burn_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 100), (BOB, 100)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

			// Unhappy unknown NFT
			let ok = NFTs::burn(alice.clone(), 10001);
			assert_noop!(ok, Error::<Test>::NFTNotFound);

			// Unhappy not the owner
			let nft_id = <NFTs as NFTTrait>::create_nft(BOB, vec![1], Some(vec![3])).unwrap();
			let ok = NFTs::burn(alice.clone(), nft_id);
			assert_noop!(ok, Error::<Test>::NotTheNFTOwner);

			// Unhappy listed for sale
			let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![1], Some(vec![2])).unwrap();
			<NFTs as NFTTrait>::set_listed_for_sale(nft_id, true).unwrap();

			let ok = NFTs::burn(alice.clone(), nft_id);
			assert_noop!(ok, Error::<Test>::CannotBurnNFTsListedForSale);

			// Unhappy converted to capsule
			let nft_id = <NFTs as NFTTrait>::create_nft(ALICE, vec![1], Some(vec![2])).unwrap();
			<NFTs as NFTTrait>::set_converted_to_capsule(nft_id, true).unwrap();

			let ok = NFTs::burn(alice.clone(), nft_id);
			assert_noop!(ok, Error::<Test>::CannotBurnCapsules);
		})
}

#[test]
fn finish_series_happy() {
	ExtBuilder::default().caps(vec![(ALICE, 1000)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		// Happy path transfer
		let series_id = vec![50];
		<NFTs as NFTTrait>::create_nft(ALICE, vec![1], Some(series_id.clone())).unwrap();
		assert_eq!(NFTs::series(&series_id).unwrap().draft, true);

		assert_ok!(NFTs::finish_series(alice.clone(), series_id.clone()));
		assert_eq!(NFTs::series(&series_id).unwrap().draft, false);
	})
}

#[test]
fn finish_series_unhappy() {
	ExtBuilder::default()
		.caps(vec![(ALICE, 100), (BOB, 100)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

			// Unhappy series id not found
			let ok = NFTs::finish_series(alice.clone(), vec![123]);
			assert_noop!(ok, Error::<Test>::SeriesNotFound);

			// Unhappy not series owner
			let series_id = vec![3];
			<NFTs as NFTTrait>::create_nft(BOB, vec![1], Some(series_id.clone())).unwrap();
			let ok = NFTs::finish_series(alice.clone(), series_id);
			assert_noop!(ok, Error::<Test>::NotTheSeriesOwner);
		})
}

#[test]
fn set_nft_mint_fee_happy() {
	ExtBuilder::default().build().execute_with(|| {
		// Happy path
		let old_mint_fee = NFTs::nft_mint_fee();
		let new_mint_fee = 654u64;
		assert_eq!(NFTs::nft_mint_fee(), old_mint_fee);

		let ok = NFTs::set_nft_mint_fee(mock::Origin::root(), new_mint_fee);
		assert_ok!(ok);
		assert_eq!(NFTs::nft_mint_fee(), new_mint_fee);
	})
}

#[test]
fn set_nft_mint_fee_unhappy() {
	ExtBuilder::default().caps(vec![(ALICE, 10000)]).build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		// Unhappy non root user tries to modify the mint fee
		let ok = NFTs::set_nft_mint_fee(alice.clone(), 654);
		assert_noop!(ok, BadOrigin);
	})
}
