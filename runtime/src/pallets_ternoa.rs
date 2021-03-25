use crate::{Balances, Call, Event, Nfts, OriginCaller, Runtime, Scheduler};
use ternoa_primitives::{NFTDetails, NFTId};

impl ternoa_nfts::Config for Runtime {
    type Event = Event;
    type NFTId = NFTId;
    type NFTDetails = NFTDetails;
    type WeightInfo = ();
}

impl ternoa_timed_escrow::Config for Runtime {
    type Event = Event;
    type NFTs = Nfts;
    type Scheduler = Scheduler;
    type PalletsOrigin = OriginCaller;
    type PalletsCall = Call;
    type WeightInfo = ();
}

impl ternoa_marketplace::Config for Runtime {
    type Event = Event;
    type Currency = Balances;
    type NFTs = Nfts;
    type WeightInfo = ();
}
