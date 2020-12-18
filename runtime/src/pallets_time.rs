use crate::{constants::time::SLOT_DURATION, Babe, Runtime};
use frame_support::parameter_types;
use ternoa_primitives::Moment;

parameter_types! {
    pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Trait for Runtime {
    type Moment = Moment;
    type OnTimestampSet = Babe;
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}
