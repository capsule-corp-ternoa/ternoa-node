use crate::{
	self as ternoa_auctions,
	types::{AuctionData, BidderList},
	Config,
};
use frame_support::{
	parameter_types,
	traits::{ConstU32, Contains, GenesisBuild, OnFinalize, OnInitialize},
	PalletId,
};
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};
use ternoa_primitives::{
	marketplace::{MarketplaceInformation, MarketplaceType},
	nfts::{NFTData, NFTSeriesDetails},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHARLIE: u64 = 3;
pub const DAVE: u64 = 4;
pub const EVE: u64 = 5;
pub type BlockNumber = u64;

pub const MIN_AUCTION_DURATION: u64 = 100;
pub const MAX_AUCTION_DURATION: u64 = 1000;
pub const MAX_AUCTION_DELAY: u64 = 50;
pub const AUCTION_GRACE_PERIOD: u64 = 5;
pub const AUCTION_ENDING_PERIOD: u64 = 10;

pub const ALICE_NFT_ID: u32 = 1;
pub const ALICE_SERIES_ID: u8 = 1;
pub const ALICE_MARKET_ID: u32 = 1;

pub const BOB_NFT_ID: u32 = 10;
pub const BOB_SERIES_ID: u8 = 10;
pub const INVALID_NFT_ID: u32 = 404;
pub const MARKETPLACE_COMMISSION_FEE: u8 = 10;
pub const BID_HISTORY_SIZE: u16 = 3;

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

pub enum AuctionState {
	Before,
	InProgress,
	Extended,
}

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
	type MaxConsumers = ConstU32<16>;
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
	type WeightInfo = ternoa_nfts::weights::TernoaWeight<Test>;
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
	pub const MinAuctionDuration: BlockNumber = MIN_AUCTION_DURATION;
	pub const MaxAuctionDuration: BlockNumber = MAX_AUCTION_DURATION;
	pub const MaxAuctionDelay: BlockNumber = MAX_AUCTION_DELAY;
	pub const AuctionGracePeriod: BlockNumber = AUCTION_GRACE_PERIOD;
	pub const AuctionEndingPeriod: BlockNumber = AUCTION_ENDING_PERIOD;
	pub const AuctionsPalletId: PalletId = PalletId(*b"tauction");
}

impl Config for Test {
	type Event = Event;
	type Currency = Balances;
	type NFTHandler = NFTs;
	type MarketplaceHandler = Marketplace;
	type MaxAuctionDelay = MaxAuctionDelay;
	type MaxAuctionDuration = MaxAuctionDuration;
	type MinAuctionDuration = MinAuctionDuration;
	type AuctionGracePeriod = AuctionGracePeriod;
	type AuctionEndingPeriod = AuctionEndingPeriod;
	type PalletId = AuctionsPalletId;
	type WeightInfo = ();
}

pub struct ExtBuilder {
	balances: Vec<(u64, u128)>,
	state: Option<AuctionState>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		ExtBuilder { balances: Vec::new(), state: None }
	}
}

impl ExtBuilder {
	pub fn new(balances: Vec<(u64, u128)>, state: Option<AuctionState>) -> Self {
		ExtBuilder { balances, state }
	}

	pub fn new_build(
		balances: Vec<(u64, u128)>,
		state: Option<AuctionState>,
	) -> sp_io::TestExternalities {
		Self::new(balances, state).build()
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

		pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.unwrap();

		Self::build_nfts(&mut t);
		Self::build_market(&mut t);
		Self::build_auction(&mut t, self.state);

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}

	fn build_nfts(t: &mut sp_runtime::Storage) {
		let alice_nft = NFTData::new_default(ALICE, vec![10], vec![ALICE_SERIES_ID]);
		let bob_nft = NFTData::new_default(BOB, vec![10], vec![BOB_SERIES_ID]);

		let alice_series = NFTSeriesDetails::new(ALICE, false);
		let bob_series = NFTSeriesDetails::new(ALICE, false);

		let nfts = vec![(ALICE_NFT_ID, alice_nft), (BOB_NFT_ID, bob_nft)];
		let series = vec![(vec![ALICE_SERIES_ID], alice_series), (vec![BOB_SERIES_ID], bob_series)];

		ternoa_nfts::GenesisConfig::<Test> { nfts, series, nft_mint_fee: 5 }
			.assimilate_storage(t)
			.unwrap();
	}

	fn build_market(t: &mut sp_runtime::Storage) {
		let alice_market = MarketplaceInformation::new(
			MarketplaceType::Public,
			MARKETPLACE_COMMISSION_FEE,
			ALICE,
			vec![],
			vec![],
			vec![10],
			None,
			None,
			None,
		);
		let marketplaces = vec![(ALICE_MARKET_ID, alice_market)];

		ternoa_marketplace::GenesisConfig::<Test> {
			nfts_for_sale: vec![],
			marketplaces,
			marketplace_mint_fee: 15,
		}
		.assimilate_storage(t)
		.unwrap();
	}

	fn build_auction(t: &mut sp_runtime::Storage, state: Option<AuctionState>) {
		pub const NFT_PRICE: u128 = 100;
		pub const NFT_BUY_PRICE: Option<u128> = Some(200);

		let mut auctions = vec![];
		if let Some(state) = state {
			let (start, end, extended) = match state {
				AuctionState::Before => (2, 2 + MAX_AUCTION_DURATION, false),
				AuctionState::InProgress => (1, 1 + MAX_AUCTION_DURATION, false),
				AuctionState::Extended => (1, 1 + MAX_AUCTION_DURATION, true),
			};

			let alice_data = AuctionData {
				creator: ALICE,
				start_block: start,
				end_block: end,
				start_price: NFT_PRICE,
				buy_it_price: NFT_BUY_PRICE.clone(),
				bidders: BidderList::new(BID_HISTORY_SIZE),
				marketplace_id: ALICE_MARKET_ID,
				is_extended: extended,
			};

			let bob_data = AuctionData {
				creator: BOB,
				start_block: start,
				end_block: end,
				start_price: NFT_PRICE,
				buy_it_price: NFT_BUY_PRICE.clone(),
				bidders: BidderList::new(BID_HISTORY_SIZE),
				marketplace_id: ALICE_MARKET_ID,
				is_extended: extended,
			};

			auctions = vec![(ALICE_NFT_ID, alice_data), (BOB_NFT_ID, bob_data)];
		}
		ternoa_auctions::GenesisConfig::<Test> { auctions, bid_history_size: BID_HISTORY_SIZE }
			.assimilate_storage(t)
			.unwrap();
	}
}

#[allow(dead_code)]
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	ternoa_auctions::GenesisConfig::<Test> {
		auctions: Default::default(),
		bid_history_size: BID_HISTORY_SIZE,
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
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
