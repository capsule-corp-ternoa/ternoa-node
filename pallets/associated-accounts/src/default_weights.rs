use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn set_altvr_username() -> Weight;
}

impl WeightInfo for () {
    // Storage: AssociatedAccounts AltVRUsers (r:0 w:1)
    fn set_altvr_username() -> Weight {
        (19_730_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
