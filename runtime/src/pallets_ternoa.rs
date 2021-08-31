use crate::constants::currency::UNIT;
use crate::{
    Balances, Call, Event, Nfts, OriginCaller, Runtime, Scheduler, TiimeBalances, Treasury,
};
use frame_support::parameter_types;
use ternoa_primitives::Balance;

parameter_types! {
    pub const MintFee: Balance = 10 * UNIT;
    pub const MarketplaceFee: Balance = 10_000 * UNIT;
}

impl ternoa_nfts::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type MintFee = MintFee;
    type FeesCollector = Treasury;
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
    type CurrencyCaps = Balances;
    type CurrencyTiime = TiimeBalances;
    type NFTs = Nfts;
    type WeightInfo = ();
    type MarketplaceFee = MarketplaceFee;
    type FeesCollector = Treasury;
}
