use crate::{Module, Trait};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use sp_core::H256;

use frame_system::EnsureRoot;

use frame_support::{
    assert_ok, impl_outer_origin, parameter_types, weights::Weight,
    Currency,
};


impl_outer_origin! {
    pub enum Origin for Test  where system = frame_system {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Trait for Test {
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}
parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) * MaximumBlockWeight::get();
}

impl pallet_scheduler::Trait for Test {
    type Event = ();
    type Origin = Origin;
    type PalletsOrigin = OriginCaller;
    type Call = Call;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<u64>;
    type MaxScheduledPerBlock = ();
    type WeightInfo = ();
}

impl ternoa_nfts::Trait for Test {
    type Event = ();
    type NFTId = u8;
    type NFTDetails = ();
    type WeightInfo = ();
}

impl Trait for Test {
    type Event = ();
    type Currency = ();
    type NFTs = NFTs;
    type WeightInfo = ();
}
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MockNFTDetails {
    Empty,
    WithU8(u8),
}
impl Default for MockNFTDetails {
    fn default() -> Self {
        Self::Empty
    }
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub type NFTs = ternoa_nfts::Module<Test>;
pub type Currency = ternoa_nfts::Module<Test>;
pub type Scheduler = pallet_scheduler::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Marketplace = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn create_one_capsule() {
    assert_ok!(<NFTs as ternoa_common::traits::NFTs>::create(&ALICE, ()));
}
