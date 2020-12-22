use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::Vec;

pub type CapsuleID = u32;
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct CapsuleData<AccountId, Hash> {
    /// ASCII encoded URI to fetch additional metadata
    pub offchain_uri: Vec<u8>,
    /// Hash of the public key that was used to create the capsule.
    /// Used by validators and storage nodes to know which shard is
    /// linked to which capsule without having to fetch the offcahin
    /// document.
    pub pk_hash: Hash,
    /// Address of who created the token, should not be updated over time.
    pub creator: AccountId,
    /// Address of the owner of the token, can be updated by transferring
    /// it to a new owner.
    pub owner: AccountId,
    /// Wether this capsule is locked by another pallet or not.
    pub locked: bool,
}
