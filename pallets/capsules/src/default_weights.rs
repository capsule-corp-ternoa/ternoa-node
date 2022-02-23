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
	// Storage: Capsules CapsuleMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Nfts NftMintFee (r:1 w:0)
	// Storage: Nfts NftIdGenerator (r:1 w:1)
	// Storage: Nfts SeriesIdGenerator (r:1 w:1)
	// Storage: Nfts Series (r:1 w:1)
	// Storage: Capsules Ledgers (r:1 w:1)
	// Storage: Capsules Capsules (r:0 w:1)
	// Storage: Nfts Data (r:0 w:1)
	fn create() -> Weight {
		(241_761_000 as Weight)
			.saturating_add(DbWeight::get().reads(8 as Weight))
			.saturating_add(DbWeight::get().writes(8 as Weight))
	}
	// Storage: Nfts Data (r:1 w:0)
	// Storage: Capsules Capsules (r:1 w:1)
	// Storage: Capsules CapsuleMintFee (r:1 w:0)
	// Storage: System Account (r:2 w:2)
	// Storage: Capsules Ledgers (r:1 w:1)
	fn create_from_nft() -> Weight {
		(86_590_000 as Weight)
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	// Storage: Capsules Ledgers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	// Storage: Capsules Capsules (r:1 w:1)
	fn remove() -> Weight {
		(101_271_000 as Weight)
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(4 as Weight))
	}
	// Storage: Capsules Ledgers (r:1 w:1)
	// Storage: System Account (r:2 w:2)
	fn add_funds() -> Weight {
		(79_300_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	// Storage: Capsules Capsules (r:1 w:1)
	fn set_ipfs_reference() -> Weight {
		(27_960_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	// Storage: Capsules CapsuleMintFee (r:0 w:1)
	fn set_capsule_mint_fee() -> Weight {
		(19_951_000 as Weight).saturating_add(DbWeight::get().writes(1 as Weight))
	}
}
