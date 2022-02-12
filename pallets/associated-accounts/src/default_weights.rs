use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn set_account() -> Weight;
    fn add_new_supported_account() -> Weight;
    fn remove_supported_account() -> Weight;
}

/// Weight functions for `ternoa_associated_accounts`.
impl WeightInfo for () {
    // Storage: AssociatedAccounts SupportedAccounts (r:1 w:0)
    // Storage: AssociatedAccounts Users (r:1 w:1)
    fn set_account() -> Weight {
        (16_170_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: AssociatedAccounts SupportedAccounts (r:1 w:1)
    fn add_new_supported_account() -> Weight {
        (12_540_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: AssociatedAccounts SupportedAccounts (r:1 w:1)
    fn remove_supported_account() -> Weight {
        (12_521_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
