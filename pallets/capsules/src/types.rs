#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use ternoa_primitives::nfts::NFTId;

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use ternoa_primitives::TextFormat;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CapsuleData<AccountId>
where
    AccountId: Clone,
{
    pub owner: AccountId,
    pub ipfs_reference: TextFormat,
}

impl<AccountId> CapsuleData<AccountId>
where
    AccountId: Clone,
{
    pub fn new(owner: AccountId, ipfs_reference: TextFormat) -> CapsuleData<AccountId> {
        Self {
            owner,
            ipfs_reference,
        }
    }
}

pub type CapsuleLedger<Balance> = Vec<(NFTId, Balance)>;
