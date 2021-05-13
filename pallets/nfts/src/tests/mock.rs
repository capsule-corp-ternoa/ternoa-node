use crate::{self as ternoa_nfts, Config};
use frame_support::parameter_types;
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
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        NFTs: ternoa_nfts::{Module, Call, Storage, Event<T>, Config<T>},
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
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

impl Config for Test {
    type Event = Event;
    type NFTId = u32;
    type WeightInfo = ();
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHAD: u64 = 3;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}
