use crate::{self as ternoa_marketplace, Config};
use frame_support::instances::Instance1;
use frame_support::parameter_types;
use frame_support::traits::{Contains, GenesisBuild};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use ternoa_primitives::nfts::NFTDetails;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        NFTs: ternoa_nfts::{Pallet, Call, Storage, Event<T>, Config<T>},
        Marketplace: ternoa_marketplace::{Pallet, Call, Event<T>},
        TiimeBalances: pallet_balances::<Instance1>::{Pallet, Call, Storage, Event<T>},
    }
);

pub struct TestBaseCallFilter;
impl Contains<Call> for TestBaseCallFilter {
    fn contains(c: &Call) -> bool {
        match *c {
            // Transfer works. Use `transfer_keep_alive` for a call that doesn't pass the filter.
            Call::Balances(pallet_balances::Call::transfer(..)) => true,
            // For benchmarking, this acts as a noop call
            Call::System(frame_system::Call::remark(..)) => true,
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

impl pallet_balances::Config<pallet_balances::Instance1> for Test {
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
    pub const MintFee: u64 = 0;
    pub const MarketplaceFee: u64 = 10;
}
impl ternoa_nfts::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type MintFee = MintFee;
    type FeesCollector = ();
}

impl Config for Test {
    type Event = Event;
    type CurrencyCaps = Balances;
    type CurrencyTiime = TiimeBalances;
    type NFTs = NFTs;
    type WeightInfo = ();
    type MarketplaceFee = MarketplaceFee;
    type FeesCollector = ();
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

pub struct ExtBuilder {
    nfts: Vec<(u64, NFTDetails)>,
    series: Vec<(u64, u32)>,
    caps_endowed_accounts: Vec<(u64, u64)>,
    tiime_endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            nfts: Vec::new(),
            series: Vec::new(),
            caps_endowed_accounts: Vec::new(),
            tiime_endowed_accounts: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn one_nft_for_alice(mut self) -> Self {
        self.nfts.push((ALICE, NFTDetails::default()));
        self
    }

    pub fn three_nfts_for_alice(mut self) -> Self {
        self.nfts.push((ALICE, NFTDetails::default()));
        self.nfts.push((ALICE, NFTDetails::default()));
        self.nfts.push((ALICE, NFTDetails::default()));
        self
    }

    pub fn n_nfts_for_alice(mut self, n: u32) -> Self {
        for _ in 0..n {
            self.nfts.push((ALICE, NFTDetails::default()));
        }

        self
    }

    pub fn one_hundred_caps_for_alice(mut self) -> Self {
        self.caps_endowed_accounts.push((ALICE, 100));
        self
    }

    pub fn one_hundred_caps_for_alice_n_bob(mut self) -> Self {
        self.caps_endowed_accounts.push((ALICE, 100));
        self.caps_endowed_accounts.push((BOB, 100));
        self
    }

    pub fn one_hundred_tiime_for_alice_n_bob(mut self) -> Self {
        self.tiime_endowed_accounts.push((ALICE, 100));
        self.tiime_endowed_accounts.push((BOB, 100));
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.caps_endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_balances::GenesisConfig::<Test, Instance1> {
            balances: self.tiime_endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        ternoa_nfts::GenesisConfig::<Test> {
            nfts: self.nfts,
            series: self.series,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
