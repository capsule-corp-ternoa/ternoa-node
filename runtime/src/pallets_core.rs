use crate::{Call, Event, Origin, PalletInfo, Runtime, VERSION};
use frame_support::{
    parameter_types,
    weights::{
        constants::{BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_PER_SECOND},
        Weight,
    },
};
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentityLookup, Saturating},
    Perbill,
};
use sp_version::RuntimeVersion;
use static_assertions::const_assert;
use ternoa_primitives::{AccountId, Balance, BlockNumber, Hash, Index};

const AVERAGE_ON_INITIALIZE_WEIGHT: Perbill = Perbill::from_percent(10);
parameter_types! {
    pub const BlockHashCount: BlockNumber = 2400;
    /// We allow for 2 seconds of compute with a 6 second average block time.
    pub const MaximumBlockWeight: Weight = 2 * WEIGHT_PER_SECOND;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
    /// Assume 10% of weight for average on_initialize calls.
    pub MaximumExtrinsicWeight: Weight = AvailableBlockRatio::get().saturating_sub(AVERAGE_ON_INITIALIZE_WEIGHT)
            * MaximumBlockWeight::get();
    pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
    pub const Version: RuntimeVersion = VERSION;
}

const_assert!(
    AvailableBlockRatio::get().deconstruct() >= AVERAGE_ON_INITIALIZE_WEIGHT.deconstruct()
);

impl frame_system::Trait for Runtime {
    type BaseCallFilter = ();
    type Origin = Origin;
    type Call = Call;
    type Index = Index;
    type BlockNumber = BlockNumber;
    type Hash = Hash;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<AccountId>;
    type Header = generic::Header<BlockNumber, BlakeTwo256>;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type DbWeight = RocksDbWeight;
    type BlockExecutionWeight = BlockExecutionWeight;
    type ExtrinsicBaseWeight = ExtrinsicBaseWeight;
    type MaximumExtrinsicWeight = MaximumExtrinsicWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = Version;
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
}
