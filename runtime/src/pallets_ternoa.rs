use crate::{Call, Event, Nfts, OriginCaller, Runtime, Scheduler};
use ternoa_primitives::NFTDetails;

impl ternoa_nfts::Trait for Runtime {
    type Event = Event;
    type NFTId = u32;
    type NFTDetails = NFTDetails;
    type WeightInfo = ();
}

impl ternoa_timed_escrow::Trait for Runtime {
    type Event = Event;
    type NFTs = Nfts;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
    type WeightInfo = ();
}
