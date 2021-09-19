use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn register_enclave() -> Weight;
    fn assign_enclave() -> Weight;
    fn unassign_enclave() -> Weight;
    fn update_enclave() -> Weight;
    fn change_enclave_owner() -> Weight;
    fn create_cluster() -> Weight;
    fn remove_cluster() -> Weight;
}

impl WeightInfo for () {
    // Storage: Marketplace NFTsForSale (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: System Account (r:2 w:2)
    // Storage: Nfts Data (r:1 w:1)
    fn register_enclave() -> Weight {
        (79_311_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(4 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:1 w:0)
    // Storage: Marketplace NFTsForSale (r:0 w:1)
    fn assign_enclave() -> Weight {
        (34_030_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Marketplace NFTsForSale (r:1 w:1)
    fn unassign_enclave() -> Weight {
        (31_800_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Marketplace MarketplaceIdGenerator (r:1 w:1)
    // Storage: Marketplace Marketplaces (r:0 w:1)
    fn update_enclave() -> Weight {
        (60_830_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Marketplace Marketplaces (r:1 w:1)
    fn change_enclave_owner() -> Weight {
        (24_440_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Marketplace Marketplaces (r:1 w:1)
    fn create_cluster() -> Weight {
        (24_810_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Marketplace Marketplaces (r:1 w:1)
    fn remove_cluster() -> Weight {
        (24_050_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
