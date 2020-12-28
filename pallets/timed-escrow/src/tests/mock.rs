use crate::{Module, Trait};
use frame_support::{
    assert_ok, impl_outer_dispatch, impl_outer_origin, parameter_types, weights::Weight,
};
use frame_system::{EnsureRoot, RawOrigin};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};
use ternoa_capsules::CapsuleData;

impl_outer_origin! {
    pub enum Origin for Test  where system = frame_system {}
}

impl_outer_dispatch! {
    pub enum Call for Test where origin: Origin {
        frame_system::System,
        crate::TimedEscrow,
    }
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
impl ternoa_capsules::Trait for Test {
    type Event = ();
    type WeightInfo = ();
}
impl Trait for Test {
    type Event = ();
    type Capsules = Capsules;
    type CapsuleData = ternoa_capsules::CapsuleData<u64, H256>;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
    type WeightInfo = ();
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub type Capsules = ternoa_capsules::Module<Test>;
pub type Scheduler = pallet_scheduler::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type TimedEscrow = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn create_one_capsule() {
    assert_ok!(Capsules::create(
        RawOrigin::Signed(ALICE).into(),
        CapsuleData {
            owner: ALICE,
            creator: ALICE,
            ..Default::default()
        }
    ));
}
