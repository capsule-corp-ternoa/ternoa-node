use crate::{self as ternoa_dday_protocol, Config};
use frame_support::{parameter_types, PalletId};
use frame_support::traits::{GenesisBuild, Contains, OnFinalize, OnInitialize};
use frame_system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

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

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        NFTs: ternoa_nfts::{Pallet, Call, Config<T>, Storage, Event<T>},
        Capsules: ternoa_capsules::{Pallet, Call, Config<T>, Storage, Event<T>},
		DdayProtocol: ternoa_dday_protocol::{Pallet, Call, Config, Storage, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
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
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = ();
    type MinIpfsLen = MinIpfsLen;
    type MaxIpfsLen = MaxIpfsLen;
}

impl ternoa_capsules::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type NFTTrait = NFTs;
    type PalletId = CapsulePalletId;
    type MinIpfsLen = MinIpfsLen;
    type MaxIpfsLen = MaxIpfsLen;
}

impl Config for Test {
    type Event = Event;
    type NFTs = NFTs;
    type Capsules = Capsules;
    type WeightInfo = ();
}

pub struct ExtBuilder {
    endowed_accounts: Vec<(u64, u128)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            endowed_accounts: Vec::new(),
        }
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
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        ternoa_nfts::GenesisConfig::<Test> {
            nfts: Default::default(),
            series: Default::default(),
            nft_mint_fee: 10,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        ternoa_capsules::GenesisConfig::<Test> {
            capsule_mint_fee: 1000,
            ..Default::default()
        }
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
    use ternoa_primitives::{TextFormat, nfts::NFTId};

    pub fn create_encapsulated_nft (
        origin: Origin,
        nft_ipfs_reference: TextFormat,
        capsule_ipfs_reference: TextFormat,
    ) -> NFTId {
        assert_ok!(Capsules::create(origin, nft_ipfs_reference, capsule_ipfs_reference, None));
        return NFTs::nft_id_generator() - 1;
    }

    pub fn run_to_block(n: u64) {
        while System::block_number() < n {
            if System::block_number() > 1 {
                DdayProtocol::on_finalize(System::block_number());
                System::on_finalize(System::block_number());
            }
            System::set_block_number(System::block_number() + 1);
            System::on_initialize(System::block_number());
            DdayProtocol::on_initialize(System::block_number());
        }
    }
}