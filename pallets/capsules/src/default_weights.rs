use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create() -> Weight;
    fn create_from_nft() -> Weight;
    fn remove() -> Weight;
    fn add_funds() -> Weight;
    fn set_ipfs_reference() -> Weight;
    fn set_capsule_mint_fee() -> Weight;
}

impl WeightInfo for () {
    // TODO!
    fn create() -> Weight {
        (100_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // TODO!
    fn create_from_nft() -> Weight {
        (100_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // TODO!
    fn remove() -> Weight {
        (100_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // TODO!
    fn add_funds() -> Weight {
        (100_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // TODO!
    fn set_ipfs_reference() -> Weight {
        (100_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // TODO!
    fn set_capsule_mint_fee() -> Weight {
        (100_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
}
