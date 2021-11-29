use crate::{self as ternoa_nfts, Config, NegativeImbalanceOf};
use frame_support::parameter_types;
use frame_support::traits::{Contains, Currency, GenesisBuild};
use sp_core::H256;
use sp_runtime::testing::Header;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Config<T>, Storage, Event<T>},
        NFTs: ternoa_nfts::{Pallet, Call, Storage, Event<T>, Config<T>},
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
    pub const ExistentialDeposit: u64 = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}
impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

impl ternoa_mock::Config for Test {
    type Event = Event;
}

parameter_types! {
    pub const MaxStringLength: u16 = 5;
    pub const MinStringLength: u16 = 1;
}

impl Config for Test {
    type Event = Event;
    type WeightInfo = ();
    type Currency = Balances;
    type FeesCollector = MockFeeCollector;
    type MaxStringLength = MaxStringLength;
    type MinStringLength = MinStringLength;
    type CapsulesTrait = TernoaMock;
}

pub struct MockFeeCollector;
impl frame_support::traits::OnUnbalanced<NegativeImbalanceOf<Test>> for MockFeeCollector {
    fn on_nonzero_unbalanced(amount: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&COLLECTOR, amount);
    }
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;
pub const CHAD: u64 = 3;
pub const COLLECTOR: u64 = 99;

pub const NFT_MINT_FEE: u64 = 10;

pub struct ExtBuilder {
    endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            endowed_accounts: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn caps(mut self, accounts: Vec<(u64, u64)>) -> Self {
        for account in accounts {
            self.endowed_accounts.push(account);
        }
        self
    }

    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Test>()
            .unwrap();

        pallet_balances::GenesisConfig::<Test> {
            balances: self.endowed_accounts,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        ternoa_nfts::GenesisConfig::<Test> {
            nfts: Default::default(),
            series: Default::default(),
            nft_mint_fee: NFT_MINT_FEE,
        }
        .assimilate_storage(&mut t)
        .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub mod help {
    use super::*;
    use crate::traits::LockableNFTs;
    use frame_support::assert_ok;
    use ternoa_primitives::nfts::{NFTId, NFTSeriesId};
    use ternoa_primitives::ternoa;

    pub fn create(
        owner: Origin,
        ipfs_reference: ternoa::String,
        series_id: Option<NFTSeriesId>,
    ) -> NFTId {
        assert_ok!(NFTs::create(owner, ipfs_reference, series_id));
        return NFTs::nft_id_generator() - 1;
    }

    pub fn finish_series(owner: Origin, series_id: NFTSeriesId) {
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
    let t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    t.into()
}
