use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn create() -> Weight;
    fn transfer() -> Weight;
    fn burn() -> Weight;
    fn finish_series() -> Weight;
    fn set_nft_mint_fee() -> Weight;
    fn set_ipfs_reference() -> Weight;
}

impl WeightInfo for () {
    // Storage: Nfts NftMintFee (r:1 w:0)
    // Storage: System Account (r:1 w:1)
    // Storage: Nfts NftIdGenerator (r:1 w:1)
    // Storage: Nfts SeriesIdGenerator (r:1 w:1)
    // Storage: Nfts Series (r:1 w:1)
    // Storage: Nfts Data (r:0 w:1)
    fn create() -> Weight {
        (78_531_000 as Weight)
            .saturating_add(DbWeight::get().reads(5 as Weight))
            .saturating_add(DbWeight::get().writes(5 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Nfts Series (r:1 w:0)
    // Storage: Capsules Capsules (r:1 w:0)
    fn transfer() -> Weight {
        (34_711_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Capsules Capsules (r:1 w:0)
    fn burn() -> Weight {
        (29_880_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Nfts Series (r:1 w:1)
    fn finish_series() -> Weight {
        (24_500_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Nfts NftMintFee (r:0 w:1)
    fn set_nft_mint_fee() -> Weight {
        (17_980_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Nfts Data (r:1 w:1)
    // Storage: Nfts Series (r:1 w:0)
    fn set_ipfs_reference() -> Weight {
        (30_661_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
