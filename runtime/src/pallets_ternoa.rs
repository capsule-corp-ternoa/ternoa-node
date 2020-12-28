use crate::{Call, Capsules, Event, OriginCaller, Runtime, Scheduler};

impl ternoa_capsules::Trait for Runtime {
    type Event = Event;
}

impl ternoa_timed_escrow::Trait for Runtime {
    type Event = Event;
    type Capsules = Capsules;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
}
