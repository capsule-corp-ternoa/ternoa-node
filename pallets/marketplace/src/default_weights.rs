use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
	fn list() -> Weight;
	fn unlist() -> Weight;
	fn buy() -> Weight;
	fn create() -> Weight;
	fn add_account_to_allow_list() -> Weight;
	fn remove_account_from_allow_list() -> Weight;
	fn set_owner() -> Weight;
	fn set_market_type() -> Weight;
	fn set_name() -> Weight;
	fn set_marketplace_mint_fee() -> Weight;
	fn set_commission_fee() -> Weight;
	fn set_uri() -> Weight;
	fn set_logo_uri() -> Weight;
	fn add_account_to_disallow_list() -> Weight;
	fn remove_account_from_disallow_list() -> Weight;
}

impl WeightInfo for () {
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Nfts Series (r:1 w:0)
	// Storage: Capsules Capsules (r:1 w:0)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: Marketplace NFTsForSale (r:0 w:1)
	fn list() -> Weight {
		(51_580_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	// Storage: Nfts Data (r:1 w:1)
	// Storage: Marketplace NFTsForSale (r:1 w:1)
	fn unlist() -> Weight {
		(34_760_000 as Weight)
			.saturating_add(DbWeight::get().reads(2 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	// Storage: Marketplace NFTsForSale (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:1 w:0)
	// Storage: Nfts Data (r:1 w:1)
	fn buy() -> Weight {
		(43_881_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	// Storage: Marketplace MarketplaceMintFee (r:1 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Marketplace MarketplaceIdGenerator (r:1 w:1)
	// Storage: Marketplace Marketplaces (r:0 w:1)
	fn create() -> Weight {
		(66_831_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn add_account_to_allow_list() -> Weight {
		(27_150_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn remove_account_from_allow_list() -> Weight {
		(25_770_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_owner() -> Weight {
		(26_170_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_market_type() -> Weight {
		(25_990_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_name() -> Weight {
		(26_481_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace MarketplaceMintFee (r:0 w:1)
	fn set_marketplace_mint_fee() -> Weight {
		(19_041_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_commission_fee() -> Weight {
		(25_770_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_uri() -> Weight {
		(26_270_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn set_logo_uri() -> Weight {
		(26_501_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn add_account_to_disallow_list() -> Weight {
		(26_810_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Marketplace Marketplaces (r:1 w:1)
	fn remove_account_from_disallow_list() -> Weight {
		(25_470_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
