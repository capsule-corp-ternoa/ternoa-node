use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

impl crate::WeightInfo for () {
    // Storage: Marketplace NFTsForSale (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: System Account (r:2 w:2)
    // Storage: Nfts Data (r:1 w:1)
    fn buy() -> Weight {
        (79_311_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(5 as Weight))
            .saturating_add(T::DbWeight::get().writes(4 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: Marketplace NFTsForSale (r:0 w:1)
    fn list() -> Weight {
        (34_030_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Marketplace NFTsForSale (r:1 w:1)
    fn unlist() -> Weight {
        (31_800_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(2 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Marketplace MarketplaceIdGenerator (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:0 w:1)
    fn create() -> Weight {
        (60_830_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(2 as Weight))
    }
    // Storage: Marketplace Marketplaces (r:1 w:1)
    fn add_account_to_allow_list() -> Weight {
        (24_440_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Marketplace Marketplaces (r:1 w:1)
    fn remove_account_from_allow_list() -> Weight {
        (24_810_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
    // Storage: Marketplace Marketplaces (r:1 w:1)
    fn change_owner() -> Weight {
        (24_050_000 as Weight)
            .saturating_add(T::DbWeight::get().reads(1 as Weight))
            .saturating_add(T::DbWeight::get().writes(1 as Weight))
    }
}
