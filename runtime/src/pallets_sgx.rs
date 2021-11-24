use crate::{Event, Runtime};
use frame_support::parameter_types;

/// A timestamp: milliseconds since the unix epoch.
pub type Moment = u64;

parameter_types! {
    pub const MomentsPerDay: Moment = 86_400_000; // [ms/d]
    pub const MaxSilenceTime: Moment = 172_800_000; // 48h
}

impl pallet_teerex::Config for Runtime {
    type Event = Event;
    type Currency = pallet_balances::Pallet<Runtime>;
    type MomentsPerDay = MomentsPerDay;
    type MaxSilenceTime = MaxSilenceTime;
    type WeightInfo = pallet_teerex::weights::IntegriteeWeight<Runtime>;
}
