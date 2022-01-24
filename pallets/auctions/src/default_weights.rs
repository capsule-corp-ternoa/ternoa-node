use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create_auction() -> Weight;
    fn cancel_auction() -> Weight;
    fn add_bid() -> Weight;
    fn remove_bid() -> Weight;
    fn increase_bid() -> Weight;
    fn buy_it_now() -> Weight;
    fn complete_auction() -> Weight;
    fn claim_bid() -> Weight;
}

/// Weight functions for `ternoa_auctions`.
impl WeightInfo for () {
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: Auctions Auctions (r:0 w:1)
    fn create_auction() -> Weight {
        (30_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: Nfts Data (r:1 w:1)
    fn cancel_auction() -> Weight {
        (23_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn add_bid() -> Weight {
        (51_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn remove_bid() -> Weight {
        (45_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn increase_bid() -> Weight {
        (40_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: System Account (r:2 w:2)
    // Storage: Nfts Data (r:1 w:1)
    fn buy_it_now() -> Weight {
        (64_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(4 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: System Account (r:2 w:2)
    // Storage: Nfts Data (r:1 w:1)
    fn complete_auction() -> Weight {
        (74_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(4 as Weight))
    }
    // Storage: Auctions Auctions (r:1 w:1)
    // Storage: System Account (r:2 w:2)
    fn claim_bid() -> Weight {
        (47_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
}
