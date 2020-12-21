use crate::{Call, Event, Runtime};

impl pallet_sudo::Trait for Runtime {
    type Event = Event;
    type Call = Call;
}
