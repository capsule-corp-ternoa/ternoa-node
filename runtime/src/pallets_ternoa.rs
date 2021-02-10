use crate::{Call, NFTs, Event, OriginCaller, Runtime, Scheduler};
use ternoa_primitives::{AccountId, Hash, NFTDetails};

impl ternoa_nfts::Trait for Runtime {
    type Event = Event;
    type NFTId = u32;
    type NFTDetails = NFTDetails;
    type WeightInfo = ();
}

impl ternoa_timed_escrow::Trait for Runtime {
    type Event = Event;
    type NFTs = NFTs;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
    type WeightInfo = ();
}
