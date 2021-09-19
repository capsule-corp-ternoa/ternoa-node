use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create() -> Weight;
    fn create_with_series() -> Weight;
    fn mutate() -> Weight;
    fn seal() -> Weight;
    fn transfer() -> Weight;
    fn burn() -> Weight;
    fn transfer_series() -> Weight;
}

impl WeightInfo for () {
    // Storage: Nfts NftIdGenerator (r:1 w:1)
    // Storage: Nfts Data (r:0 w:1)
    fn create() -> Weight {
        (61_660_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Nfts Series (r:1 w:1)
    // Storage: Nfts NftIdGenerator (r:1 w:1)
    // Storage: Nfts Data (r:0 w:1)
    fn create_with_series() -> Weight {
        (67_841_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    fn mutate() -> Weight {
        (24_590_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    fn seal() -> Weight {
        (24_581_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
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
    // Storage: Nfts Series (r:1 w:1)
    fn transfer_series() -> Weight {
        (23_830_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
