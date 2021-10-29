use crate::constants::currency::UNIT;
use crate::{
    Balances, Call, Event, Nfts, OriginCaller, Runtime, Scheduler, TiimeBalances, Treasury,
};
use frame_support::parameter_types;
use ternoa_primitives::Balance;

parameter_types! {
    pub const EnclaveFee: Balance = 500_000 * UNIT;
    pub const MaxStringLength: u16 = 1000;
    pub const MinStringLength: u16 = 1;
    pub const ClusterSize: u32 = 8;
    pub const MaxUrlLength: u32 = 1000;
}

impl ternoa_nfts::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = Treasury;
    type MaxStringLength = MaxStringLength;
    type MinStringLength = MinStringLength;
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
    type FeesCollector = Treasury;
    type MaxStringLength = MaxStringLength;
    type MinStringLength = MinStringLength;
}

impl ternoa_sgx::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type EnclaveFee = EnclaveFee;
    type FeesCollector = Treasury;
    type ClusterSize = ClusterSize;
    type MaxUrlLength = MaxUrlLength;
}
