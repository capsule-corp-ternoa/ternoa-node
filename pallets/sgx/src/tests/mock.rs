use crate::{self as ternoa_marketplace, Config, MarketplaceInformation, MarketplaceType};
use frame_support::instances::Instance1;
use frame_support::parameter_types;
use frame_support::traits::{Contains, GenesisBuild};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};
use ternoa_primitives::nfts::NFTDetails;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Event<T>},
        NFTs: ternoa_nfts::{Pallet, Call, Storage, Event<T>, Config<T>},
        Marketplace: ternoa_marketplace::{Pallet, Call, Event<T>},
        TiimeBalances: pallet_balances::<Instance1>::{Pallet, Call, Storage, Event<T>},
    }
);

pub struct TestBaseCallFilter;
impl Contains<Call> for TestBaseCallFilter {
    fn contains(c: &Call) -> bool {
        match *c {
            // Transfer works. Use `transfer_keep_alive` for a call that doesn't pass the filter.
            Call::Balances(pallet_balances::Call::transfer(..)) => true,
            // For benchmarking, this acts as a noop call
            Call::System(frame_system::Call::remark(..)) => true,
            // For tests
            _ => false,
        }
    }
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = TestBaseCallFilter;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = u64;
    type Call = Call;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

impl pallet_balances::Config<pallet_balances::Instance1> for Test {
    type Balance = u64;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

parameter_types! {
    pub const MintFee: u64 = 5;
    pub const MarketplaceFee: u64 = 10;
    pub const MaxNameLength: u32 = 50;
    pub const MinNameLength: u32 = 1;
}

impl ternoa_nfts::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type MintFee = MintFee;
    type FeesCollector = ();
}

impl Config for Test {
    type Event = Event;
    type CurrencyCaps = Balances;
    type CurrencyTiime = TiimeBalances;
    type NFTs = NFTs;
    type WeightInfo = ();
    type MarketplaceFee = MarketplaceFee;
    type FeesCollector = ();
    type MaxNameLength = MaxNameLength;
    type MinNameLength = MinNameLength;
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const DAVE: u64 = 3;

pub struct ExtBuilder {
    nfts: Vec<(u64, NFTDetails)>,
    series: Vec<(u64, u32)>,
    caps_endowed_accounts: Vec<(u64, u64)>,
    tiime_endowed_accounts: Vec<(u64, u64)>,
    marketplaces: Vec<(u64, MarketplaceType, u8, Vec<u8>)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            nfts: Vec::new(),
            series: Vec::new(),
            caps_endowed_accounts: Vec::new(),
            tiime_endowed_accounts: Vec::new(),
            marketplaces: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn nfts(mut self, accounts: Vec<(u64, u64)>) -> Self {
        for account in accounts {
            for _ in 0..account.1 {
                self.nfts.push((account.0, NFTDetails::default()));
            }
        }

        self
    }

    pub fn caps(mut self, accounts: Vec<(u64, u64)>) -> Self {
        for account in accounts {
            self.caps_endowed_accounts.push(account);
        }
        self
    }

    pub fn tiime(mut self, accounts: Vec<(u64, u64)>) -> Self {
        for account in accounts {
            self.tiime_endowed_accounts.push(account);
        }
        self
    }

    pub fn marketplace(mut self, markets: Vec<(u64, MarketplaceType, u8, Vec<u8>)>) -> Self {
        for market in markets {
            self.marketplaces.push(market);
        }
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.caps_endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        pallet_balances::GenesisConfig::<Test, Instance1> {
            balances: self.tiime_endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        ternoa_nfts::GenesisConfig::<Test> {
            nfts: self.nfts,
            series: self.series,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut marketplaces = vec![(
            0,
            MarketplaceInformation::new(
                MarketplaceType::Public,
                0,
                ALICE,
                Default::default(),
                "Ternoa marketplace".into(),
            ),
        )];
        let mut i = 1;
        for market in self.marketplaces {
            marketplaces.push((
                i,
                MarketplaceInformation::new(market.1, market.2, market.0, vec![], market.3),
            ));

            i += 1;
        }

        ternoa_marketplace::GenesisConfig::<Test> {
            nfts_for_sale: Default::default(),
            marketplaces: marketplaces,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    pub fn build_v6_migration(self) -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    ternoa_marketplace::GenesisConfig::<Test> {
        nfts_for_sale: Default::default(),
        marketplaces: vec![(
            0,
            MarketplaceInformation::new(
                MarketplaceType::Public,
                0,
                ALICE,
                Default::default(),
                "Ternoa Marketplace".into(),
            ),
        )],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
