use crate::{Balances, Event, Runtime};
use frame_support::parameter_types;
use ternoa_primitives::Moment;

parameter_types! {
    pub const MomentsPerDay: Moment = 86_400_000; // [ms/d]
    pub const MaxSilenceTime: u64 = 172_800_000; // 48h
}

impl pallet_teerex::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type MomentsPerDay = MomentsPerDay;
    type WeightInfo = ();
    type MaxSilenceTime = MaxSilenceTime;
}
