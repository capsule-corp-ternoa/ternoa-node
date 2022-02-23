use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
	fn create_auction() -> Weight;
	fn cancel_auction() -> Weight;
	fn end_auction() -> Weight;
	fn add_bid() -> Weight;
	fn remove_bid() -> Weight;
	fn buy_it_now() -> Weight;
	fn complete_auction() -> Weight;
	fn claim() -> Weight;
}

/// Weight functions for `ternoa_auctions`.
impl WeightInfo for () {
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Nfts Series (r:1 w:0)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: Auctions BidHistorySize (r:1 w:0)
	// Storage: Auctions Deadlines (r:1 w:1)
	// Storage: Auctions Auctions (r:0 w:1)
	fn create_auction() -> Weight {
		(39_280_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	// Storage: Auctions Auctions (r:1 w:1)
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Auctions Deadlines (r:1 w:1)
	fn cancel_auction() -> Weight {
		(27_890_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	// Storage: Auctions Auctions (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Auctions Deadlines (r:1 w:1)
	// Storage: Auctions Claims (r:1 w:1)
	fn end_auction() -> Weight {
		(80_181_000 as Weight)
			.saturating_add(DbWeight::get().reads(8 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
	// Storage: Auctions Auctions (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn add_bid() -> Weight {
		(51_450_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	// Storage: Auctions Auctions (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn remove_bid() -> Weight {
		(44_811_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	// Storage: Auctions Auctions (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Auctions Deadlines (r:1 w:1)
	fn buy_it_now() -> Weight {
		(76_360_000 as Weight)
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().writes(6 as Weight))
	}
	// Storage: Auctions Auctions (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: System Account (r:3 w:3)
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Auctions Deadlines (r:1 w:1)
	// Storage: Auctions Claims (r:1 w:1)
	fn complete_auction() -> Weight {
		(76_161_000 as Weight)
			.saturating_add(DbWeight::get().reads(8 as Weight))
			.saturating_add(DbWeight::get().writes(7 as Weight))
	}
	// Storage: Auctions Claims (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn claim() -> Weight {
		(45_170_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
}
