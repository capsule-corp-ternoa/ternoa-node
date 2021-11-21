use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create_user() -> Weight;
    fn set_vchatname() -> Weight;
    fn set_username() -> Weight;
}

impl WeightInfo for () {
    fn create_user() -> Weight {
        (25_000_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Altvr Users (r:1 w:1)
    fn set_username() -> Weight {
        (30_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Altvr Users (r:1 w:1)
    fn set_vchatname() -> Weight {
        (32_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
