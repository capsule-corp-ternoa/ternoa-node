use crate::{Balances, Event, Runtime};
use frame_support::parameter_types;
use ternoa_primitives::Moment;

parameter_types! {
    pub const MomentsPerDay: Moment = 86_400_000; // [ms/d]
}

impl pallet_substratee_registry::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type MomentsPerDay = MomentsPerDay;
}
