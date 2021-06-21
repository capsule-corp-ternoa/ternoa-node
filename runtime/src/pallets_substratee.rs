use crate::{
    constants::time::{DAYS, MILLISECS_PER_BLOCK},
    Balances, Event, Runtime,
};
use frame_support::parameter_types;

parameter_types! {
    pub const MomentsPerDay: u64 = MILLISECS_PER_BLOCK * DAYS; // [ms/d]
}

impl pallet_substratee_registry::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type MomentsPerDay = MomentsPerDay;
}
