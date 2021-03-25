use crate::{self as ternoa_marketplace, Config};
use codec::{Decode, Encode};
use frame_support::parameter_types;
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Module, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Module, Call, Storage, Event<T>},
        NFTs: ternoa_nfts::{Module, Call, Storage, Event<T>, Config<T>},
        Marketplace: ternoa_marketplace::{Module, Call, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
    type BaseCallFilter = ();
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
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Config for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

impl ternoa_nfts::Config for Test {
    type Event = Event;
    type NFTId = u8;
    type NFTDetails = MockNFTDetails;
    type WeightInfo = ();
}

impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    type NFTs = NFTs;
    type WeightInfo = ();
}
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MockNFTDetails {
    Empty,
    WithU8(u8),
}
impl Default for MockNFTDetails {
    fn default() -> Self {
        Self::Empty
    }
}

// Do not use the `0` account id since this would be the default value
// for our account id. This would mess with some tests.
pub const ALICE: u64 = 1;
pub const BOB: u64 = 2;

pub struct ExtBuilder {
    nfts: Vec<(u64, MockNFTDetails)>,
    endowed_accounts: Vec<(u64, u64)>,
}

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder {
            nfts: Vec::new(),
            endowed_accounts: Vec::new(),
        }
    }
}

impl ExtBuilder {
    pub fn one_nft_for_alice(mut self) -> Self {
        self.nfts.push((ALICE, Default::default()));
        self
    }

    pub fn one_hundred_for_alice_n_bob(mut self) -> Self {
        self.endowed_accounts.push((ALICE, 100));
        self.endowed_accounts.push((BOB, 100));
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

        ternoa_nfts::GenesisConfig::<Test> { nfts: self.nfts }
            .assimilate_storage(&mut t)
            .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}
