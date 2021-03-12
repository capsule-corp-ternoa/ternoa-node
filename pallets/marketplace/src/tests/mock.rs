use crate::{Module, Trait};
use codec::{Decode, Encode};
use frame_support::{impl_outer_origin, parameter_types, weights::Weight};
use serde::{Deserialize, Serialize};
use sp_core::H256;
use sp_runtime::{
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Perbill,
};

impl_outer_origin! {
    pub enum Origin for Test  where system = frame_system {}
}

#[derive(Clone, Eq, PartialEq)]
pub struct Test;

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const MaximumBlockWeight: Weight = 1024;
    pub const MaximumBlockLength: u32 = 2 * 1024;
    pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
}

impl frame_system::Trait for Test {
    type Origin = Origin;
    type Call = ();
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = ();
    type BlockHashCount = BlockHashCount;
    type MaximumBlockWeight = MaximumBlockWeight;
    type MaximumBlockLength = MaximumBlockLength;
    type AvailableBlockRatio = AvailableBlockRatio;
    type Version = ();
    type PalletInfo = ();
    type AccountData = pallet_balances::AccountData<u64>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BlockExecutionWeight = ();
    type ExtrinsicBaseWeight = ();
    type MaximumExtrinsicWeight = MaximumBlockWeight;
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 0;
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Trait for Test {
    type Balance = u64;
    type DustRemoval = ();
    type Event = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<Test>;
    type WeightInfo = ();
    type MaxLocks = MaxLocks;
}

impl ternoa_nfts::Trait for Test {
    type Event = ();
    type NFTId = u8;
    type NFTDetails = MockNFTDetails;
    type WeightInfo = ();
}

impl Trait for Test {
    type Event = ();
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
pub type NFTs = ternoa_nfts::Module<Test>;
pub type Balances = pallet_balances::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Marketplace = Module<Test>;

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

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
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
