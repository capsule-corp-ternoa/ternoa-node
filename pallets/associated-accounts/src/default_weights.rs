use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn set_altvr_username() -> Weight;
}

impl WeightInfo for () {
    // Storage: Altvr Users (r:1 w:1)
    fn set_altvr_username() -> Weight {
        (30_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
