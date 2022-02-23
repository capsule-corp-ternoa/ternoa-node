use crate::{self as ternoa_capsules, Config};
use frame_support::{
	parameter_types,
	traits::{ConstU32, Contains, GenesisBuild},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
		TernoaNFTs: ternoa_nfts::{Pallet, Call, Storage, Event<T>, Config<T>},
		TernoaCapsules: ternoa_capsules::{Pallet, Call, Storage, Event<T>, Config<T>},
	}
);

pub struct TestBaseCallFilter;
impl Contains<Call> for TestBaseCallFilter {
	fn contains(c: &Call) -> bool {
		match *c {
			// Transfer works. Use `transfer_keep_alive` for a call that doesn't pass the filter.
			Call::Balances(pallet_balances::Call::transfer { .. }) => true,
			// For benchmarking, this acts as a noop call
			Call::System(frame_system::Call::remark { .. }) => true,
			// For tests
			_ => false,
		}
	}
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
	type BaseCallFilter = TestBaseCallFilter;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Index = u64;
	type BlockNumber = u64;
	type Call = Call;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 1;
	pub const MaxLocks: u32 = 50;
	pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
	type MaxLocks = MaxLocks;
	type MaxReserves = MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const MinIpfsLen: u16 = 1;
	pub const MaxIpfsLen: u16 = 5;
	pub const CapsulePalletId: PalletId = PalletId(*b"mockcaps");
}

impl ternoa_nfts::Config for Test {
	type Event = Event;
	type WeightInfo = ternoa_nfts::weights::TernoaWeight<Test>;
	type Currency = Balances;
	type FeesCollector = ();
	type MinIpfsLen = MinIpfsLen;
	type MaxIpfsLen = MaxIpfsLen;
}

impl Config for Test {
	type Event = Event;
	type WeightInfo = ();
	type Currency = Balances;
	type NFTTrait = TernoaNFTs;
	type PalletId = CapsulePalletId;
	type MinIpfsLen = MinIpfsLen;
	type MaxIpfsLen = MaxIpfsLen;
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

pub struct ExtBuilder {
	endowed_accounts: Vec<(u64, u128)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder { endowed_accounts: Vec::new() }
	}
}

impl ExtBuilder {
	pub fn caps(mut self, accounts: Vec<(u64, u128)>) -> Self {
		for account in accounts {
			self.endowed_accounts.push(account);
		}
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> { balances: self.endowed_accounts }
			.assimilate_storage(&mut t)
			.unwrap();

		ternoa_nfts::GenesisConfig::<Test> {
			nfts: Default::default(),
			series: Default::default(),
			nft_mint_fee: 10,
		}
		.assimilate_storage(&mut t)
		.unwrap();

		ternoa_capsules::GenesisConfig::<Test> { capsule_mint_fee: 1000, ..Default::default() }
			.assimilate_storage(&mut t)
			.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub mod help {
	use super::*;
	use frame_support::assert_ok;
	use ternoa_primitives::{
		nfts::{NFTId, NFTSeriesId},
		TextFormat,
	};

	pub fn create_capsule_fast(owner: Origin) -> NFTId {
		let nft_id = create_nft(owner.clone(), vec![50], None);
		assert_ok!(TernoaCapsules::create_from_nft(owner, nft_id, vec![60]));
		nft_id
	}

	pub fn create_nft_fast(owner: Origin) -> NFTId {
		create_nft(owner, vec![50], None)
	}

	pub fn create_nft(
		owner: Origin,
		ipfs_reference: TextFormat,
		series_id: Option<NFTSeriesId>,
	) -> NFTId {
		assert_ok!(TernoaNFTs::create(owner, ipfs_reference, series_id));
		TernoaNFTs::nft_id_generator() - 1
	}
}

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
	let t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
	t.into()
}
