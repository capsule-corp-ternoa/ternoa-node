use crate::constants::currency::UNIT;
use crate::{
    Balances, Call, Capsules, Event, Nfts, OriginCaller, Runtime, Scheduler, TiimeBalances,
    Treasury,
};
use frame_support::{parameter_types, PalletId};
use ternoa_primitives::Balance;

parameter_types! {
    pub const EnclaveFee: Balance = 500_000 * UNIT;
    pub const MaxStringLength: u16 = 1000;
    pub const MinStringLength: u16 = 1;
    pub const ClusterSize: u32 = 8;
    pub const MaxUrlLength: u32 = 1000;
    pub const CapsulePalletId: PalletId = PalletId(*b"tcapsule");
    pub const MinNameLength : u8 = 1;
    pub const MaxNameLength : u8 = 20;
}

impl ternoa_nfts::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = Treasury;
    type MaxStringLength = MaxStringLength;
    type MinStringLength = MinStringLength;
    type CapsulesTrait = Capsules;
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
    type CapsulesTrait = Capsules;
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

impl ternoa_capsules::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type NFTSTrait = Nfts;
    type MinStringLength = MinStringLength;
    type MaxStringLength = MaxStringLength;
    type PalletId = CapsulePalletId;
}

impl ternoa_associated_accounts::Config for Runtime {
    type Event = Event;
    type WeightInfo = ();
    type MinNameLength = MinNameLength;
    type MaxNameLength = MaxNameLength;
}
