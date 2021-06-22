use crate::{self as ternoa_nfts, Config, NegativeImbalanceOf};
use frame_support::{parameter_types, traits::Currency};
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
        NFTs: ternoa_nfts::{Pallet, Call, Storage, Event<T>, Config<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
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
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 50;
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const MintFee: u64 = 10;
}
impl Config for Test {
    type Event = Event;
    type NFTId = u32;
    type WeightInfo = ();
    type Currency = Balances;
    type MintFee = MintFee;
    type FeesCollector = MockFeeCollector;
}

pub struct MockFeeCollector;
impl frame_support::traits::OnUnbalanced<NegativeImbalanceOf<Test>> for MockFeeCollector {
    fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&COLLECTOR, amount);
    }
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHAD: u64 = 3;
pub const COLLECTOR: u64 = 99;

pub struct ExtBuilder {
    endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            endowed_accounts: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn one_hundred_for_everyone(mut self) -> Self {
        self.endowed_accounts.push((ALICE, 100));
        self.endowed_accounts.push((BOB, 100));
        self.endowed_accounts.push((CHAD, 100));
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

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
