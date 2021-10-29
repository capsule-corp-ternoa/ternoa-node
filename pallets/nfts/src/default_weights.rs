use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create() -> Weight;
    fn transfer() -> Weight;
    fn burn() -> Weight;
    fn finish_series() -> Weight;
}

impl WeightInfo for () {
    // Storage: Nfts NftIdGenerator (r:1 w:1)
    // Storage: Nfts Data (r:0 w:1)
    fn create() -> Weight {
        (61_660_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    fn transfer() -> Weight {
        (25_190_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Nfts Series (r:1 w:1)
    fn burn() -> Weight {
        (36_081_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Nfts Series (r:1 w:1)
    fn finish_series() -> Weight {
        (36_081_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
}
