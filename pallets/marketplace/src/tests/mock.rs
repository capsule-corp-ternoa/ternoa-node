use crate::{self as ternoa_marketplace, Config, MarketplaceInformation, MarketplaceType};
use frame_benchmarking::account;
use frame_support::instances::Instance1;
use frame_support::parameter_types;
use frame_support::traits::{Contains, GenesisBuild};
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use ternoa_primitives::nfts::{NFTData, NFTSeriesDetails};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const DAVE: u64 = 3;

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
        TiimeAccountStore: ternoa_account_store::{Pallet, Storage},
        TernoaMock: ternoa_mock::{Pallet, Call, Storage, Event<T>, Config},
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
    type AccountStore = TiimeAccountStore;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

parameter_types! {
    pub const MaxStringLength: u16 = 5;
    pub const MinStringLength: u16 = 1;
}

impl ternoa_nfts::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = ();
    type MaxStringLength = MaxStringLength;
    type MinStringLength = MinStringLength;
    type CapsulesTrait = TernoaMock;
}

impl ternoa_account_store::Config for Test {
    type AccountData = pallet_balances::AccountData<u64>;
}

impl ternoa_mock::Config for Test {
    type Event = Event;
}

impl Config for Test {
    type Event = Event;
    type CurrencyCaps = Balances;
    type CurrencyTiime = TiimeBalances;
    type NFTs = NFTs;
    type WeightInfo = ();
    type FeesCollector = ();
    type MaxStringLength = MaxStringLength;
    type MinStringLength = MinStringLength;
    type CapsulesTrait = TernoaMock;
}

pub struct ExtBuilder {
    nfts: Vec<(u32, NFTData<u64>)>,
    series: Vec<(Vec<u8>, NFTSeriesDetails<u64>)>,
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
            nft_mint_fee: 10,
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
                vec![],
                "Ternoa marketplace".into(),
                None,
                None,
            ),
        )];
        let mut i = 1;
        for market in self.marketplaces {
            marketplaces.push((
                i,
                MarketplaceInformation::new(
                    market.1,
                    market.2,
                    market.0,
                    vec![],
                    vec![],
                    market.3,
                    None,
                    None,
                ),
            ));

            i += 1;
        }

        ternoa_marketplace::GenesisConfig::<Test> {
            nfts_for_sale: Default::default(),
            marketplaces: marketplaces,
            marketplace_mint_fee: 250,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }

    /*     pub fn build_v6_migration(self) -> sp_io::TestExternalities {
        let t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    } */
}

pub mod help {
    use crate::MarketplaceId;

    use super::*;
    use frame_support::assert_ok;
    use ternoa_common::traits::LockableNFTs;
    use ternoa_primitives::nfts::{NFTId, NFTSeriesId};
    use ternoa_primitives::ternoa;

    pub fn create_nft(
        owner: Origin,
        ipfs_reference: ternoa::String,
        series_id: Option<NFTSeriesId>,
    ) -> NFTId {
        assert_ok!(NFTs::create(owner, ipfs_reference, series_id));
        return NFTs::nft_id_generator() - 1;
    }

    pub fn create_nft_and_lock_series(
        owner: Origin,
        ipfs_reference: ternoa::String,
        series_id: NFTSeriesId,
    ) -> NFTId {
        let nft_id = help::create_nft(owner.clone(), ipfs_reference, Some(series_id.clone()));
        help::finish_series(owner.clone(), series_id.clone());

        nft_id
    }

    pub fn create_mkp(
        owner: Origin,
        kind: MarketplaceType,
        fee: u8,
        name: ternoa::String,
        list: Vec<u64>,
    ) -> MarketplaceId {
        assert_ok!(Marketplace::create(
            owner.clone(),
            kind,
            fee,
            name,
            None,
            None
        ));
        let mkp_id = Marketplace::marketplace_id_generator();

        for acc in list {
            match kind {
                MarketplaceType::Private => {
                    let ok = Marketplace::add_account_to_allow_list(owner.clone(), mkp_id, acc);
                    assert_ok!(ok);
                }
                MarketplaceType::Public => {
                    let ok = Marketplace::add_account_to_disallow_list(owner.clone(), mkp_id, acc);
                    assert_ok!(ok);
                }
            }
        }

        return Marketplace::marketplace_id_generator();
    }

    pub fn finish_series(owner: Origin, series_id: Vec<u8>) {
        assert_ok!(NFTs::finish_series(owner, series_id));
    }

    pub fn lock(nft_id: NFTId) {
        assert_ok!(NFTs::lock(nft_id));
    }

    pub fn capsulize(val: bool) {
        TernoaMock::set_is_capsulized(val);
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
                Default::default(),
                "Ternoa Marketplace".into(),
                None,
                None,
            ),
        )],
        marketplace_mint_fee: 255,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let alice = account("ALICE", 0, 0);
    let bob = account("BOB", 0, 0);
    let nft_data = NFTData::new(alice, vec![0], vec![50], false);
    let series_data = NFTSeriesDetails::new(alice, false);

    pallet_balances::GenesisConfig::<Test> {
        balances: vec![(alice, 10000), (bob, 10000)],
    }
    .assimilate_storage(&mut t)
    .unwrap();

    ternoa_nfts::GenesisConfig::<Test> {
        nfts: vec![(100, nft_data)],
        series: vec![(vec![50], series_data)],
        nft_mint_fee: 10,
    }
    .assimilate_storage(&mut t)
    .unwrap();

    t.into()
}
