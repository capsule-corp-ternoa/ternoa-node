use super::mock::AuctionState::{Before, Extended, InProgress};
use super::mock::*;
use crate::tests::mock;
use crate::types::{AuctionData, BidderList, DeadlineList};
use crate::{Auctions as AuctionsStorage, Claims, Deadlines, Error, Event as AuctionEvent};
use frame_support::error::BadOrigin;
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use ternoa_common::traits::{MarketplaceTrait, NFTTrait};
use ternoa_marketplace::Error as MarketError;

fn origin(account: u64) -> mock::Origin {
	RawOrigin::Signed(account).into()
}

fn root() -> mock::Origin {
	RawOrigin::Root.into()
}

pub mod create_auction {
	pub use super::*;

	#[test]
	fn create_auction() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let start_block = 10;
			let auction = AuctionData {
				creator: ALICE,
				start_block,
				end_block: start_block + MIN_AUCTION_DURATION,
				start_price: 300,
				buy_it_price: Some(400),
				bidders: BidderList::new(BID_HISTORY_SIZE),
				marketplace_id: market_id,
				is_extended: false,
			};

			let deadline = DeadlineList(vec![(nft_id, auction.end_block)]);

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				auction.marketplace_id,
				auction.start_block,
				auction.end_block,
				auction.start_price,
				auction.buy_it_price.clone(),
			);
			assert_ok!(ok);

			// Storage
			assert_eq!(NFTs::is_listed_for_sale(nft_id), Some(true));
			assert_eq!(AuctionsStorage::<Test>::iter().count(), 1);
			assert_eq!(Claims::<Test>::iter().count(), 0);

			assert_eq!(AuctionsStorage::<Test>::get(nft_id).unwrap(), auction);
			assert_eq!(Deadlines::<Test>::get(), deadline);

			// Events
			let event = AuctionEvent::AuctionCreated {
				nft_id,
				creator: auction.creator,
				start_block: auction.start_block,
				end_block: auction.end_block,
				buy_it_price: auction.buy_it_price,
				marketplace_id: auction.marketplace_id,
				start_price: auction.start_price,
			};
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn auction_cannot_start_in_the_past() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let current_block = System::block_number();
			let start_block = current_block - 1;
			assert!(start_block < current_block);

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				start_block,
				1000,
				100,
				Some(200),
			);
			assert_noop!(ok, Error::<Test>::AuctionCannotStartInThePast);
		})
	}

	#[test]
	fn auction_cannot_end_before_it_has_started() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let start_block = System::block_number();
			let end_block = start_block - 1;

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				start_block,
				end_block,
				100,
				Some(200),
			);
			assert_noop!(ok, Error::<Test>::AuctionCannotEndBeforeItHasStarted);
		})
	}

	#[test]
	fn auction_duration_is_too_long() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let start_block = System::block_number();
			let end_block = start_block + MAX_AUCTION_DURATION + 1;

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				start_block,
				end_block,
				100,
				Some(200),
			);
			assert_noop!(ok, Error::<Test>::AuctionDurationIsTooLong);
		})
	}

	#[test]
	fn auction_duration_is_too_short() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let start_block = System::block_number();
			let end_block = start_block + MIN_AUCTION_DURATION - 1;

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				start_block,
				end_block,
				100,
				Some(200),
			);
			assert_noop!(ok, Error::<Test>::AuctionDurationIsTooShort);
		})
	}

	#[test]
	fn auction_start_is_too_far_away() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let start_block = System::block_number() + MAX_AUCTION_DELAY + 1;
			let end_block = start_block + MIN_AUCTION_DURATION;

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				start_block,
				end_block,
				100,
				Some(200),
			);
			assert_noop!(ok, Error::<Test>::AuctionStartIsTooFarAway);
		})
	}

	#[test]
	fn buy_it_price_cannot_be_lower_or_equal_than_start_price() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);
			let start_price = 100;

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				start_price,
				Some(start_price),
			);
			assert_noop!(ok, Error::<Test>::BuyItPriceCannotBeLowerOrEqualThanStartPrice);
		})
	}

	#[test]
	fn nft_does_not_exist() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (INVALID_NFT_ID, ALICE_MARKET_ID);

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::NFTDoesNotExist);
		})
	}
	#[test]
	fn cannot_auction_not_owned_nfts() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (BOB_NFT_ID, ALICE_MARKET_ID);

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::CannotAuctionNotOwnedNFTs);
		})
	}

	#[test]
	fn cannot_auction_nfts_listed_for_sale() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);
			assert_ok!(NFTs::set_listed_for_sale(nft_id, true));

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::CannotAuctionNFTsListedForSale);
		})
	}

	#[test]
	fn cannot_auction_nfts_in_transmission() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);
			assert_ok!(NFTs::set_in_transmission(nft_id, true));

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::CannotAuctionNFTsInTransmission);
		})
	}

	#[test]
	fn cannot_auction_capsules() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);
			assert_ok!(NFTs::set_converted_to_capsule(ALICE_NFT_ID, true));

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::CannotAuctionCapsules);
		})
	}

	#[test]
	fn cannot_auction_nfts_in_uncompleted_series() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);
			assert_ok!(NFTs::set_series_completion(&vec![ALICE_SERIES_ID], false));

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::CannotAuctionNFTsInUncompletedSeries);
		})
	}

	#[test]
	fn not_allowed_to_list() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let alice: mock::Origin = origin(ALICE);
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);

			let ok = Marketplace::add_account_to_disallow_list(alice.clone(), market_id, ALICE);
			assert_ok!(ok);

			let ok = Auctions::create_auction(
				alice.clone(),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, MarketError::<Test>::NotAllowedToList);
		})
	}

	#[test]
	fn cannot_auction_lent_nfts() {
		ExtBuilder::new_build(vec![], None).execute_with(|| {
			let (nft_id, market_id) = (ALICE_NFT_ID, ALICE_MARKET_ID);
			assert_ok!(NFTs::set_viewer(nft_id, Some(BOB)));

			let ok = Auctions::create_auction(
				origin(ALICE),
				nft_id,
				market_id,
				System::block_number(),
				System::block_number() + MIN_AUCTION_DURATION,
				100,
				Some(101),
			);
			assert_noop!(ok, Error::<Test>::CannotAuctionLentNFTs);
		})
	}
}

pub mod cancel_auction {
	pub use super::*;

	#[test]
	fn cancel_auction() {
		ExtBuilder::new_build(vec![], Some(Before)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction_count = AuctionsStorage::<Test>::iter().count();
			let mut deadlines = Deadlines::<Test>::get();

			assert_ok!(Auctions::cancel_auction(origin(ALICE), nft_id));

			// NFT
			let nft = NFTs::get_nft(nft_id).unwrap();
			assert_eq!(nft.listed_for_sale, false);
			assert_eq!(nft.owner, ALICE);

			// Storage
			deadlines.remove(nft_id);

			assert_eq!(NFTs::is_listed_for_sale(nft_id), Some(false));
			assert_eq!(AuctionsStorage::<Test>::iter().count(), auction_count - 1);
			assert_eq!(Claims::<Test>::iter().count(), 0);

			assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);
			assert_eq!(Deadlines::<Test>::get(), deadlines);

			// Check Events
			let event = AuctionEvent::AuctionCancelled { nft_id };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn auction_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(Before)).execute_with(|| {
			let ok = Auctions::cancel_auction(origin(ALICE), INVALID_NFT_ID);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotExist);
		})
	}

	#[test]
	fn not_the_auction_creator() {
		ExtBuilder::new_build(vec![], Some(Before)).execute_with(|| {
			let ok = Auctions::cancel_auction(origin(BOB), ALICE_NFT_ID);
			assert_noop!(ok, Error::<Test>::NotTheAuctionCreator);
		})
	}

	#[test]
	fn cannot_cancel_auction_in_progress() {
		ExtBuilder::new_build(vec![], Some(Before)).execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			run_to_block(auction.start_block);

			let ok = Auctions::cancel_auction(alice, nft_id);
			assert_noop!(ok, Error::<Test>::CannotCancelAuctionInProgress);
		})
	}
}

pub mod end_auction {
	pub use super::*;

	#[test]
	fn end_auction() {
		ExtBuilder::new_build(vec![(CHARLIE, 1000), (DAVE, 1000)], Some(Extended)).execute_with(
			|| {
				let alice_balance = Balances::free_balance(ALICE);
				let bob_balance = Balances::free_balance(BOB);
				let charlie_balance = Balances::free_balance(CHARLIE);
				let dave_balance = Balances::free_balance(CHARLIE);

				let (nft_id, market_id) = (BOB_NFT_ID, ALICE_MARKET_ID);
				let auction_count = AuctionsStorage::<Test>::iter().count();
				let mut deadlines = Deadlines::<Test>::get();
				let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
				let market = Marketplace::get_marketplace(market_id).unwrap();
				let market_fee = market.commission_fee;
				assert!(market_fee > 0);

				let charlie_bid = auction.start_price + 10;
				let dave_bid = charlie_bid + 10;

				assert_ok!(Auctions::add_bid(origin(CHARLIE), nft_id, charlie_bid));
				assert_ok!(Auctions::add_bid(origin(DAVE), nft_id, dave_bid));
				let pallet_balance = Balances::free_balance(Auctions::account_id());
				assert_eq!(pallet_balance, charlie_bid + dave_bid);

				assert_ok!(Auctions::end_auction(origin(BOB), nft_id));

				// Balance
				let alice_new_balance = Balances::free_balance(ALICE);
				let bob_new_balance = Balances::free_balance(BOB);
				let charlie_new_balance = Balances::free_balance(CHARLIE);
				let dave_new_balance = Balances::free_balance(DAVE);
				let pallet_new_balance = Balances::free_balance(Auctions::account_id());

				let market_owner_cut: u128 = dave_bid.saturating_mul(market_fee.into()) / 100u128;
				let artist_cut: u128 = dave_bid.saturating_sub(market_owner_cut.into());

				assert_ne!(market_owner_cut, artist_cut);
				assert_ne!(market_owner_cut, 0);
				assert_ne!(artist_cut, 0);

				assert_eq!(alice_new_balance, alice_balance + market_owner_cut);
				assert_eq!(bob_new_balance, bob_balance + artist_cut);
				assert_eq!(charlie_new_balance, charlie_balance - charlie_bid);
				assert_eq!(dave_new_balance, dave_balance - dave_bid);
				assert_eq!(pallet_new_balance, charlie_bid);

				// NFT
				let nft = NFTs::get_nft(nft_id).unwrap();
				assert_eq!(nft.listed_for_sale, false);
				assert_eq!(nft.owner, DAVE);

				// Storage
				deadlines.remove(nft_id);

				assert_eq!(AuctionsStorage::<Test>::iter().count(), auction_count - 1);
				assert_eq!(Claims::<Test>::iter().count(), 1);

				assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);
				assert_eq!(Deadlines::<Test>::get(), deadlines);
				assert_eq!(Claims::<Test>::get(CHARLIE), Some(charlie_bid));

				// Check Events
				let event = AuctionEvent::AuctionCompleted {
					nft_id,
					new_owner: Some(DAVE),
					amount: Some(dave_bid),
				};
				let event = Event::Auctions(event);
				assert_eq!(System::events().last().unwrap().event, event);
			},
		)
	}

	#[test]
	fn auction_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(Extended)).execute_with(|| {
			let ok = Auctions::end_auction(origin(ALICE), INVALID_NFT_ID);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotExist);
		})
	}

	#[test]
	fn not_the_auction_creator() {
		ExtBuilder::new_build(vec![], Some(Extended)).execute_with(|| {
			let ok = Auctions::end_auction(origin(BOB), ALICE_NFT_ID);
			assert_noop!(ok, Error::<Test>::NotTheAuctionCreator);
		})
	}

	#[test]
	fn cannot_end_auction_that_was_not_extended() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let nft_id = ALICE_NFT_ID;

			let ok = Auctions::end_auction(alice, nft_id);
			assert_noop!(ok, Error::<Test>::CannotEndAuctionThatWasNotExtended);
		})
	}
}

pub mod add_bid {
	pub use super::*;

	#[test]
	fn add_bid() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let bob_balance = Balances::free_balance(BOB);
			let nft_id = ALICE_NFT_ID;
			let mut auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let bid = auction.start_price + 10;
			assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bid));

			// Balance
			let bob_new_balance = Balances::free_balance(BOB);
			let pallet_new_balance = Balances::free_balance(Auctions::account_id());
			assert_eq!(bob_new_balance, bob_balance - bid);
			assert_eq!(pallet_new_balance, bid);

			// Storage
			auction.bidders.list = vec![(BOB, bid)];

			assert_eq!(Claims::<Test>::iter().count(), 0);
			assert_eq!(AuctionsStorage::<Test>::get(nft_id), Some(auction));

			// Check Events
			let event = AuctionEvent::BidAdded { nft_id, bidder: BOB, amount: bid };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn add_bid_above_max_bidder_history_size() {
		ExtBuilder::new_build(
			vec![(BOB, 1000), (CHARLIE, 1000), (DAVE, 1000), (EVE, 1000)],
			Some(InProgress),
		)
		.execute_with(|| {
			let eve_balance = Balances::free_balance(EVE);
			let nft_id = ALICE_NFT_ID;
			let mut auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let bob_bid = auction.start_price + 1;
			let charlie_bid = bob_bid + 1;
			let dave_bid = charlie_bid + 1;
			let eve_bid = dave_bid + 1;
			let mut accounts =
				vec![(BOB, bob_bid), (CHARLIE, charlie_bid), (DAVE, dave_bid), (EVE, eve_bid)];
			assert_eq!(accounts.len(), (BID_HISTORY_SIZE + 1) as usize);

			for bidder in &accounts {
				assert_ok!(Auctions::add_bid(origin(bidder.0), nft_id, bidder.1));
			}

			// Balance
			let eve_new_balance = Balances::free_balance(EVE);
			let pallet_new_balance = Balances::free_balance(Auctions::account_id());
			assert_eq!(eve_new_balance, eve_balance - eve_bid);
			assert_eq!(pallet_new_balance, bob_bid + charlie_bid + dave_bid + eve_bid);

			// Storage
			accounts.remove(0);
			auction.bidders.list = accounts;

			assert_eq!(Claims::<Test>::iter().count(), 1);
			assert_eq!(Claims::<Test>::get(BOB), Some(bob_bid));
			assert_eq!(AuctionsStorage::<Test>::get(nft_id), Some(auction));

			// Check Events
			let event = AuctionEvent::BidAdded { nft_id, bidder: EVE, amount: eve_bid };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn add_bid_increase_auction_duration() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let bob_balance = Balances::free_balance(BOB);
			let nft_id = ALICE_NFT_ID;
			let mut auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
			let mut deadlines = Deadlines::<Test>::get();

			let grace_period = AUCTION_GRACE_PERIOD;
			let remaining_blocks = 3;
			let target_block = auction.end_block - remaining_blocks;
			let new_end_block = auction.end_block + (grace_period - remaining_blocks);

			run_to_block(target_block);

			let bid = auction.start_price + 10;
			assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bid));

			// Balance
			let bob_new_balance = Balances::free_balance(BOB);
			assert_eq!(bob_new_balance, bob_balance - bid);

			// Storage
			auction.bidders.insert_new_bid(BOB, bid);
			auction.end_block = new_end_block;
			auction.is_extended = true;
			deadlines.update(nft_id, new_end_block);

			assert_eq!(AuctionsStorage::<Test>::get(nft_id), Some(auction));
			assert_eq!(Deadlines::<Test>::get(), deadlines);

			// Check Events
			let event = AuctionEvent::BidAdded { nft_id, bidder: BOB, amount: bid };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn add_bid_and_replace_current() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let bob_balance = Balances::free_balance(BOB);
			let mut auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let old_bid = auction.start_price + 10;
			let new_bid = old_bid + 10;
			assert_ok!(Auctions::add_bid(origin(BOB), nft_id, old_bid));
			assert_ok!(Auctions::add_bid(origin(BOB), nft_id, new_bid));

			// Balance
			let bob_new_balance = Balances::free_balance(BOB);
			assert_eq!(bob_new_balance, bob_balance - new_bid);

			// Storage
			auction.bidders.list = vec![(BOB, new_bid)];

			assert_eq!(AuctionsStorage::<Test>::get(nft_id), Some(auction));

			// Check Events
			let event = AuctionEvent::BidAdded { nft_id, bidder: BOB, amount: new_bid };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn auction_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::add_bid(origin(ALICE), INVALID_NFT_ID, 1);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotExist);
		})
	}

	#[test]
	fn cannot_add_bid_to_your_own_auctions() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::add_bid(origin(ALICE), ALICE_NFT_ID, 1);
			assert_noop!(ok, Error::<Test>::CannotAddBidToYourOwnAuctions);
		})
	}

	#[test]
	fn auction_not_started() {
		ExtBuilder::new_build(vec![], Some(Before)).execute_with(|| {
			let ok = Auctions::add_bid(origin(BOB), ALICE_NFT_ID, 1);
			assert_noop!(ok, Error::<Test>::AuctionNotStarted);
		})
	}

	#[test]
	fn cannot_bid_less_than_the_highest_bid() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let bob_bid = auction.start_price + 1;
			assert_ok!(Auctions::add_bid(origin(BOB), ALICE_NFT_ID, bob_bid));

			let ok = Auctions::add_bid(origin(DAVE), nft_id, bob_bid);
			assert_noop!(ok, Error::<Test>::CannotBidLessThanTheHighestBid);
		})
	}

	#[test]
	fn cannot_bid_less_than_the_starting_price() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let ok = Auctions::add_bid(origin(BOB), nft_id, auction.start_price - 1);
			assert_noop!(ok, Error::<Test>::CannotBidLessThanTheStartingPrice);
		})
	}

	#[test]
	fn not_enough_funds() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let balance = Balances::free_balance(BOB);
			let bid = balance + 1;
			assert!(bid > auction.start_price);

			let ok = Auctions::add_bid(origin(BOB), nft_id, bid);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);
		})
	}

	#[test]
	fn not_enough_funds_to_replace() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let bid = Balances::free_balance(BOB);
			assert!(bid > auction.start_price);

			assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bid));

			let ok = Auctions::add_bid(origin(BOB), nft_id, bid + 10);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);
		})
	}
}

pub mod remove_bid {
	pub use super::*;

	#[test]
	fn remove_bid() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let bob_balance = Balances::free_balance(BOB);
			let nft_id = ALICE_NFT_ID;
			let mut auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

			let bid = auction.start_price + 10;
			assert_ok!(Auctions::add_bid(bob.clone(), nft_id, bid));
			assert_ok!(Auctions::remove_bid(bob.clone(), nft_id));

			// Balance
			let bob_new_balance = Balances::free_balance(BOB);
			let pallet_new_balance = Balances::free_balance(Auctions::account_id());
			assert_eq!(bob_new_balance, bob_balance);
			assert_eq!(pallet_new_balance, 0);

			// Storage
			auction.bidders.list = vec![];

			assert_eq!(Claims::<Test>::iter().count(), 0);
			assert_eq!(AuctionsStorage::<Test>::get(nft_id), Some(auction));

			// Check Events
			let event = AuctionEvent::BidRemoved { nft_id, bidder: BOB, amount: bid };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn auction_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::remove_bid(origin(ALICE), INVALID_NFT_ID);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotExist);
		})
	}

	#[test]
	fn cannot_remove_bid_at_the_end_of_auction() {
		ExtBuilder::new_build(vec![(BOB, 1000)], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
			let auction_end_period = AUCTION_ENDING_PERIOD;
			let target_block = auction.end_block - auction_end_period;

			let bid = auction.start_price + 1;
			assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bid));

			run_to_block(target_block);

			let ok = Auctions::remove_bid(origin(BOB), nft_id);
			assert_noop!(ok, Error::<Test>::CannotRemoveBidAtTheEndOfAuction);
		})
	}

	#[test]
	fn bid_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::remove_bid(origin(BOB), ALICE_NFT_ID);
			assert_noop!(ok, Error::<Test>::BidDoesNotExist);
		})
	}
}

pub mod buy_it_now {
	pub use super::*;

	#[test]
	fn buy_it_now() {
		ExtBuilder::new_build(vec![(CHARLIE, 1000)], Some(InProgress)).execute_with(|| {
			let alice_balance = Balances::free_balance(ALICE);
			let bob_balance = Balances::free_balance(BOB);
			let charlie_balance = Balances::free_balance(CHARLIE);
			let nft_id = BOB_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
			let market = Marketplace::get_marketplace(ALICE_MARKET_ID).unwrap();
			let market_fee = market.commission_fee;

			let price = auction.buy_it_price.clone().unwrap();
			assert_ok!(Auctions::buy_it_now(origin(CHARLIE), nft_id));

			// Balance
			let alice_new_balance = Balances::free_balance(ALICE);
			let bob_new_balance = Balances::free_balance(BOB);
			let charlie_new_balance = Balances::free_balance(CHARLIE);
			let pallet_new_balance = Balances::free_balance(Auctions::account_id());

			let market_owner_cut: u128 = price.saturating_mul(market_fee.into()) / 100u128;
			let artist_cut: u128 = price.saturating_sub(market_owner_cut.into());

			assert_eq!(alice_new_balance, alice_balance + market_owner_cut);
			assert_eq!(bob_new_balance, bob_balance + artist_cut);
			assert_eq!(charlie_new_balance, charlie_balance - price);
			assert_eq!(pallet_new_balance, 0);

			// NFT
			let nft = NFTs::get_nft(nft_id).unwrap();
			assert_eq!(nft.listed_for_sale, false);
			assert_eq!(nft.owner, CHARLIE);

			// Storage
			assert_eq!(Claims::<Test>::iter().count(), 0);
			assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);

			// Check Events
			let event = AuctionEvent::AuctionCompleted {
				nft_id,
				new_owner: Some(CHARLIE),
				amount: Some(price),
			};
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn buy_it_now_with_existing_bids() {
		ExtBuilder::new_build(vec![(BOB, 1000), (CHARLIE, 1000)], Some(InProgress)).execute_with(
			|| {
				let bob_balance = Balances::free_balance(BOB);
				let charlie_balance = Balances::free_balance(CHARLIE);
				let nft_id = ALICE_NFT_ID;
				let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

				let bob_bid = auction.start_price + 1;
				assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bob_bid));

				let price = auction.buy_it_price.clone().unwrap();
				assert_ok!(Auctions::buy_it_now(origin(CHARLIE), nft_id));

				// Balance
				let bob_new_balance = Balances::free_balance(BOB);
				let charlie_new_balance = Balances::free_balance(CHARLIE);
				let pallet_new_balance = Balances::free_balance(Auctions::account_id());

				assert_eq!(bob_new_balance, bob_balance - bob_bid);
				assert_eq!(charlie_new_balance, charlie_balance - price);
				assert_eq!(pallet_new_balance, bob_bid);

				// NFT
				let nft = NFTs::get_nft(nft_id).unwrap();
				assert_eq!(nft.listed_for_sale, false);
				assert_eq!(nft.owner, CHARLIE);

				// Storage
				assert_eq!(Claims::<Test>::iter().count(), 1);
				assert_eq!(Claims::<Test>::get(BOB), Some(bob_bid));
				assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);

				// Check Events
				let event = AuctionEvent::AuctionCompleted {
					nft_id,
					new_owner: Some(CHARLIE),
					amount: Some(price),
				};
				let event = Event::Auctions(event);
				assert_eq!(System::events().last().unwrap().event, event);
			},
		)
	}

	#[test]
	fn auction_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::buy_it_now(origin(BOB), INVALID_NFT_ID);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotExist);
		})
	}

	#[test]
	fn auction_does_not_support_buy_it_now() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			AuctionsStorage::<Test>::mutate(nft_id, |x| {
				let x = x.as_mut().unwrap();
				x.buy_it_price = None;
			});

			let ok = Auctions::buy_it_now(origin(BOB), nft_id);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotSupportBuyItNow);
		})
	}

	#[test]
	fn auction_not_started() {
		ExtBuilder::new_build(vec![], Some(Before)).execute_with(|| {
			let ok = Auctions::buy_it_now(origin(BOB), ALICE_NFT_ID);
			assert_noop!(ok, Error::<Test>::AuctionNotStarted);
		})
	}

	#[test]
	fn cannot_buy_it_when_a_bid_is_higher_than_buy_it_price() {
		ExtBuilder::new_build(vec![(BOB, 1000), (CHARLIE, 1000)], Some(InProgress)).execute_with(
			|| {
				let nft_id = ALICE_NFT_ID;
				let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

				let price = auction.buy_it_price.unwrap();
				assert_ok!(Auctions::add_bid(origin(CHARLIE), nft_id, price));

				let ok = Auctions::buy_it_now(origin(BOB), nft_id);
				assert_noop!(ok, Error::<Test>::CannotBuyItWhenABidIsHigherThanBuyItPrice);
			},
		)
	}
}

pub mod complete_auction {
	pub use super::*;

	#[test]
	fn complete_auction_without_bid() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let nft_id = ALICE_NFT_ID;
			let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
			let mut deadlines = Deadlines::<Test>::get();

			assert_ok!(Auctions::complete_auction(root(), nft_id));

			// NFT
			let nft = NFTs::get_nft(nft_id).unwrap();
			assert_eq!(nft.listed_for_sale, false);
			assert_eq!(nft.owner, auction.creator);

			// Storage
			deadlines.remove(nft_id);

			assert_eq!(Claims::<Test>::iter().count(), 0);
			assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);
			assert_eq!(Deadlines::<Test>::get(), deadlines);

			// Event
			let event = AuctionEvent::AuctionCompleted { nft_id, new_owner: None, amount: None };
			let event = Event::Auctions(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn complete_auction_with_one_bid() {
		ExtBuilder::new_build(vec![(BOB, 1000), (CHARLIE, 1000)], Some(InProgress)).execute_with(
			|| {
				let nft_id = BOB_NFT_ID;
				let alice_balance = Balances::free_balance(ALICE);
				let bob_balance = Balances::free_balance(BOB);
				let charlie_balance = Balances::free_balance(CHARLIE);
				let market = Marketplace::get_marketplace(ALICE_MARKET_ID).unwrap();
				let market_fee = market.commission_fee;
				let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
				let mut deadlines = Deadlines::<Test>::get();
				let bid = auction.start_price + 1;
				assert_ok!(Auctions::add_bid(origin(CHARLIE), nft_id, bid));
				assert_ok!(Auctions::complete_auction(root(), nft_id));

				// Balance
				let alice_new_balance = Balances::free_balance(ALICE);
				let bob_new_balance = Balances::free_balance(BOB);
				let charlie_new_balance = Balances::free_balance(CHARLIE);
				let pallet_new_balance = Balances::free_balance(Auctions::account_id());

				let market_owner_cut: u128 = bid.saturating_mul(market_fee.into()) / 100u128;
				let artist_cut: u128 = bid.saturating_sub(market_owner_cut.into());

				assert_eq!(alice_new_balance, alice_balance + market_owner_cut);
				assert_eq!(bob_new_balance, bob_balance + artist_cut);
				assert_eq!(charlie_new_balance, charlie_balance - bid);
				assert_eq!(pallet_new_balance, 0);

				// NFT
				let nft = NFTs::get_nft(nft_id).unwrap();
				assert_eq!(nft.listed_for_sale, false);
				assert_eq!(nft.owner, CHARLIE);

				// Storage
				deadlines.remove(nft_id);

				assert_eq!(Claims::<Test>::iter().count(), 0);
				assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);
				assert_eq!(Deadlines::<Test>::get(), deadlines);

				// Event
				let event = AuctionEvent::AuctionCompleted {
					nft_id,
					new_owner: Some(CHARLIE),
					amount: Some(bid),
				};
				let event = Event::Auctions(event);
				assert_eq!(System::events().last().unwrap().event, event);
			},
		)
	}

	#[test]
	fn complete_auction_with_two_bids() {
		ExtBuilder::new_build(vec![(BOB, 1000), (CHARLIE, 1000)], Some(InProgress)).execute_with(
			|| {
				let nft_id = ALICE_NFT_ID;
				let alice_balance = Balances::free_balance(ALICE);
				let bob_balance = Balances::free_balance(BOB);
				let charlie_balance = Balances::free_balance(CHARLIE);
				let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();
				let mut deadlines = Deadlines::<Test>::get();

				let bob_bid = auction.start_price + 1;
				let charlie_bid = bob_bid + 1;
				assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bob_bid));
				assert_ok!(Auctions::add_bid(origin(CHARLIE), nft_id, charlie_bid));
				assert_ok!(Auctions::complete_auction(root(), nft_id));

				// Balance
				let alice_new_balance = Balances::free_balance(ALICE);
				let bob_new_balance = Balances::free_balance(BOB);
				let charlie_new_balance = Balances::free_balance(CHARLIE);
				let pallet_new_balance = Balances::free_balance(Auctions::account_id());

				assert_eq!(alice_new_balance, alice_balance + charlie_bid);
				assert_eq!(bob_new_balance, bob_balance - bob_bid);
				assert_eq!(charlie_new_balance, charlie_balance - charlie_bid);
				assert_eq!(pallet_new_balance, bob_bid);

				// NFT
				let nft = NFTs::get_nft(nft_id).unwrap();
				assert_eq!(nft.listed_for_sale, false);
				assert_eq!(nft.owner, CHARLIE);

				// Storage
				deadlines.remove(nft_id);

				assert_eq!(Claims::<Test>::iter().count(), 1);
				assert_eq!(Claims::<Test>::get(BOB), Some(bob_bid));
				assert_eq!(AuctionsStorage::<Test>::get(nft_id), None);
				assert_eq!(Deadlines::<Test>::get(), deadlines);

				// Event
				let event = AuctionEvent::AuctionCompleted {
					nft_id,
					new_owner: Some(CHARLIE),
					amount: Some(charlie_bid),
				};
				let event = Event::Auctions(event);
				assert_eq!(System::events().last().unwrap().event, event);
			},
		)
	}

	#[test]
	fn bad_origin() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::complete_auction(origin(ALICE), ALICE_NFT_ID);
			assert_noop!(ok, BadOrigin);
		})
	}

	#[test]
	fn auction_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::complete_auction(root(), INVALID_NFT_ID);
			assert_noop!(ok, Error::<Test>::AuctionDoesNotExist);
		})
	}
}

pub mod claim {
	pub use super::*;

	#[test]
	fn claim() {
		ExtBuilder::new_build(vec![(BOB, 1000), (CHARLIE, 1000)], Some(InProgress)).execute_with(
			|| {
				let nft_id = ALICE_NFT_ID;
				let bob_balance = Balances::free_balance(BOB);
				let pallet_balance = Balances::free_balance(Auctions::account_id());
				let auction = AuctionsStorage::<Test>::get(nft_id).unwrap();

				let bob_bid = auction.start_price + 1;
				let charlie_bid = bob_bid + 1;
				assert_ok!(Auctions::add_bid(origin(BOB), nft_id, bob_bid));
				assert_ok!(Auctions::add_bid(origin(CHARLIE), nft_id, charlie_bid));
				assert_ok!(Auctions::complete_auction(root(), nft_id));

				let claim = Claims::<Test>::get(BOB).unwrap();
				assert_ok!(Auctions::claim(origin(BOB)));

				// Balance
				let bob_new_balance = Balances::free_balance(BOB);

				assert_eq!(bob_new_balance, bob_balance);
				assert_eq!(pallet_balance, 0);
				assert_eq!(claim, bob_bid);

				// Storage
				assert_eq!(Claims::<Test>::iter().count(), 0);
				assert_eq!(Claims::<Test>::get(BOB), None);
				// Event
				let event = AuctionEvent::BalanceClaimed { account: BOB, amount: claim };
				let event = Event::Auctions(event);
				assert_eq!(System::events().last().unwrap().event, event);
			},
		)
	}

	#[test]
	fn claim_does_not_exist() {
		ExtBuilder::new_build(vec![], Some(InProgress)).execute_with(|| {
			let ok = Auctions::claim(origin(BOB));
			assert_noop!(ok, Error::<Test>::ClaimDoesNotExist);
		})
	}
}
