use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create_altvr() -> Weight;
    fn update_vchatname() -> Weight;
    fn update_username() -> Weight;
}

impl WeightInfo for () {
    fn create_altvr() -> Weight {
        (25_000_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Altvr Altvrs (r:1 w:1)
    fn update_username() -> Weight {
        (30_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Altvr Altvrs (r:1 w:1)
    fn update_vchatname() -> Weight {
        (32_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
