use super::mock::*;
use crate::{
	types::{AuctionData, BidderList, DeadlineList},
	GenesisConfig,
};
use frame_support::traits::GenesisBuild;

#[test]
fn genesis() {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let bid_history_size = 100;
	let auction = AuctionData {
		creator: ALICE,
		start_block: 10,
		end_block: 20,
		start_price: 10,
		buy_it_price: Some(20),
		bidders: BidderList::new(bid_history_size),
		marketplace_id: ALICE_MARKET_ID,
		is_extended: false,
	};

	let deadlines = DeadlineList(vec![(ALICE_NFT_ID, auction.end_block)]);
	let auctions = vec![(ALICE_NFT_ID, auction)];

	GenesisConfig::<Test> { auctions: auctions.clone(), bid_history_size }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		for auction in auctions {
			assert_eq!(Auctions::auctions(auction.0), Some(auction.1));
		}
		assert_eq!(Auctions::bid_history_size(), bid_history_size);
		assert_eq!(Auctions::deadlines(), deadlines);
	});
}
