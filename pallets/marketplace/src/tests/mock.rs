use crate::{Module, Trait};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

use frame_support::assert_ok;
#[derive(Clone, Eq, PartialEq)]
pub struct Test;

impl Trait for Test {
    type Event = ();
    type Currency = Currency;
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
pub type Currency = ternoa_nfts::Module<Test>;
pub type Scheduler = pallet_scheduler::Module<Test>;
pub type System = frame_system::Module<Test>;
pub type Marketplace = Module<Test>;

pub fn new_test_ext() -> sp_io::TestExternalities {
    frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap()
        .into()
}

pub fn create_one_capsule() {
    assert_ok!(<NFTs as ternoa_common::traits::NFTs>::create(&ALICE, ()));
}
