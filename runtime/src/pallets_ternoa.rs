use crate::{Call, Capsules, Event, OriginCaller, Runtime, Scheduler};
use ternoa_primitives::{AccountId, Hash};

impl ternoa_capsules::Trait for Runtime {
    type Event = Event;
    type WeightInfo = ();
}

impl ternoa_timed_escrow::Trait for Runtime {
    type Event = Event;
    type Capsules = Capsules;
    type CapsuleData = ternoa_capsules::CapsuleData<AccountId, Hash>;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
    type WeightInfo = ();
}
