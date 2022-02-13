use crate::{self as ternoa_timed_escrow, Config};
use frame_benchmarking::account;
use frame_support::traits::{ConstU32, Contains, EqualPrivilegeOnly, GenesisBuild};
use frame_support::{parameter_types, weights::Weight};
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use sp_runtime::{testing::Header, Perbill};
use ternoa_primitives::nfts::{NFTData, NFTSeriesDetails};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system,
        Balances: pallet_balances,
        Scheduler: pallet_scheduler,
        NFTs: ternoa_nfts,
        TimedEscrow: ternoa_timed_escrow,
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
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}
impl pallet_balances::Config for Test {
    type Balance = u64;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * BlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Test {
    type Event = Event;
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<u64>;
    type MaxScheduledPerBlock = ();
    type WeightInfo = ();
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
}

parameter_types! {
    pub const MinIpfsLen: u16 = 1;
    pub const MaxIpfsLen: u16 = 5;
}

impl ternoa_nfts::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = ();
    type MinIpfsLen = MinIpfsLen;
    type MaxIpfsLen = MaxIpfsLen;
}

impl Config for Test {
    type Event = Event;
    type NFTs = NFTs;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
    type WeightInfo = ();
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

pub struct ExtBuilder {
    nfts: Vec<(u32, NFTData<u64>)>,
    endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            nfts: Vec::new(),
            endowed_accounts: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        ternoa_nfts::GenesisConfig::<Test> {
            nfts: self.nfts,
            series: Vec::new(),
            nft_mint_fee: 10,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    pub fn caps(mut self, accounts: Vec<(u64, u64)>) -> Self {
        for account in accounts {
            self.endowed_accounts.push(account);
        }
        self
    }
}

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    let alice = account("ALICE", 0, 0);
    let bob = account("BOB", 0, 0);
    let nft_data = NFTData::new(alice, ALICE, vec![0], vec![50], false, false, false);
    let series_data = NFTSeriesDetails::new(alice, false);

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(alice, 10000), (bob, 10000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    ternoa_nfts::GenesisConfig::<Test> {
        nfts: vec![(100, nft_data)],
        series: vec![(vec![50], series_data)],
        nft_mint_fee: 10,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
