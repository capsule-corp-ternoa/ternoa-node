use crate::{self as ternoa_auctions, Config};
use frame_support::traits::{Contains, GenesisBuild, OnFinalize, OnInitialize};
use frame_support::{parameter_types, PalletId};
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use ternoa_primitives::{
    marketplace::{MarketplaceId, MarketplaceType},
    nfts::{NFTData, NFTSeriesDetails},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;
pub const TREASURY: u64 = 2021;
pub type BlockNumber = u64;

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
        Auctions: ternoa_auctions::{Pallet, Call, Event<T>}
    }
);

pub struct TestBaseCallFilter;
impl Contains<Call> for TestBaseCallFilter {
    fn contains(c: &Call) -> bool {
        match *c {
            // Transfer works. Use `transfer_keep_alive` for a call that doesn't pass the filter.
            Call::Balances(pallet_balances::Call::transfer { .. }) => true,
            // For benchmarking, this acts as a noop call
            Call::System(frame_system::Call::remark { .. }) => true,
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
    type BlockNumber = BlockNumber;
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
    type AccountData = pallet_balances::AccountData<u128>;
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
    type Balance = u128;
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
    pub const MinUriLen: u16 = 1;
    pub const MaxUriLen: u16 = 5;
    pub const MinIpfsLen: u16 = 1;
    pub const MaxIpfsLen: u16 = 5;
    pub const MinDescriptionLen: u16 = 1;
    pub const MaxDescriptionLen: u16 = 500;
    pub const MinNameLen: u16 = 1;
    pub const MaxNameLen: u16 = 5;
}

impl ternoa_nfts::Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = ();
    type MinIpfsLen = MinIpfsLen;
    type MaxIpfsLen = MaxIpfsLen;
}

impl ternoa_marketplace::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type NFTs = NFTs;
    type WeightInfo = ();
    type FeesCollector = ();
    type MinNameLen = MinNameLen;
    type MaxNameLen = MaxNameLen;
    type MinUriLen = MinUriLen;
    type MaxUriLen = MaxUriLen;
    type MinDescriptionLen = MinDescriptionLen;
    type MaxDescriptionLen = MaxDescriptionLen;
}

parameter_types! {
    // all calculations assume blocktime of 6secs
    // min auction duration of 24 hours (24*60*60)/6
    pub const MinAuctionDuration: BlockNumber = 14400;
    // min auction buffer of 1 hour (1*60*60)/6
    pub const MinAuctionBuffer: BlockNumber = 600;
    // max auction duration of 30 days (30*24*60*60)/6
    pub const MaxAuctionDuration: BlockNumber = 432000;
    // auction grace period of 10min (10*60)/6
    pub const AuctionGracePeriod: BlockNumber = 100;
    // auction ending period of 1hr (1*60*60)/6
    pub const AuctionEndingPeriod: BlockNumber = 600;
    pub const AuctionsPalletId: PalletId = PalletId(*b"py/enauc");
}

impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type NFTHandler = NFTs;
    type MarketplaceHandler = Marketplace;
    type MinAuctionBuffer = MinAuctionBuffer;
    type MaxAuctionDuration = MaxAuctionDuration;
    type MinAuctionDuration = MinAuctionDuration;
    type AuctionGracePeriod = AuctionGracePeriod;
    type AuctionEndingPeriod = AuctionEndingPeriod;
    type PalletId = AuctionsPalletId;
    type WeightInfo = ();
}

pub struct ExtBuilder {
    nfts: Vec<(u32, NFTData<u64>)>,
    series: Vec<(Vec<u8>, NFTSeriesDetails<u64>)>,
    caps_endowed_accounts: Vec<(u64, u128)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            nfts: Vec::new(),
            series: Vec::new(),
            caps_endowed_accounts: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn caps(mut self, accounts: Vec<(u64, u128)>) -> Self {
        for account in accounts {
            self.caps_endowed_accounts.push(account);
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

        ternoa_nfts::GenesisConfig::<Test> {
            nfts: self.nfts,
            series: self.series,
            nft_mint_fee: 10,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

#[allow(dead_code)]
pub mod help {
    use super::*;
    use frame_support::assert_ok;
    use ternoa_primitives::nfts::{NFTId, NFTSeriesId};
    use ternoa_primitives::TextFormat;

    pub fn create_nft(
        owner: Origin,
        ipfs_reference: TextFormat,
        series_id: Option<NFTSeriesId>,
    ) -> NFTId {
        assert_ok!(NFTs::create(owner, ipfs_reference, series_id));
        return NFTs::nft_id_generator() - 1;
    }

    pub fn create_mkp(
        owner: Origin,
        kind: MarketplaceType,
        fee: u8,
        name: TextFormat,
        list: Vec<u64>,
    ) -> MarketplaceId {
        assert_ok!(Marketplace::create(
            owner.clone(),
            kind,
            fee,
            name,
            None,
            None,
            None,
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
}

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    t.into()
}

pub fn run_to_block(n: u64) {
    while System::block_number() < n {
        Auctions::on_finalize(System::block_number());
        Balances::on_finalize(System::block_number());
        System::on_finalize(System::block_number());
        System::set_block_number(System::block_number() + 1);
        System::on_initialize(System::block_number());
        Balances::on_initialize(System::block_number());
        Auctions::on_initialize(System::block_number());
    }
}
