//! Low level primitives for the runtime and node.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};
use sp_std::vec::Vec;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Type used for expressing timestamp.
pub type Moment = u64;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Header type.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type.
pub type Block = generic::Block<Header, OpaqueExtrinsic>;

/// Text format.
pub type TextFormat = Vec<u8>;

/// The type for looking up accounts. We don't expect more than 4 billion of them.
pub type AccountIndex = u32;

pub mod marketplace {
    use super::*;
    /// The type of marketplace Id
    pub type MarketplaceId = u32;
    /// Type of marketplace commission
    pub type MarketplaceCommission = u8;

    #[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub enum MarketplaceType {
        Public,
        Private,
    }
}

pub mod nfts {
    use super::*;

    /// How NFT IDs are encoded.
    pub type NFTId = u32;

    /// How NFT IDs are encoded. In the JSON Types this should be "Text" and not "Vec<8>".
    pub type NFTSeriesId = Vec<u8>;

    /// Data related to an NFT, such as who is its owner.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTData<AccountId> {
        // NFT owner
        pub owner: AccountId,
        // NFT creator
        pub creator: AccountId,
        // IPFS reference
        pub ipfs_reference: TextFormat,
        // Series ID
        pub series_id: NFTSeriesId,
        // Is listed for sale
        pub listed_for_sale: bool,
        // Is being transmitted
        pub in_transmission: bool,
        // Is NFT converted to capsule
        pub converted_to_capsule: bool,
    }

    impl<AccountId> NFTData<AccountId> {
        pub fn new(
            owner: AccountId,
            creator: AccountId,
            ipfs_reference: TextFormat,
            series_id: NFTSeriesId,
            listed_for_sale: bool,
            in_transmission: bool,
            converted_to_capsule: bool,
        ) -> Self {
            Self {
                owner,
                creator,
                ipfs_reference,
                series_id,
                listed_for_sale,
                in_transmission,
                converted_to_capsule,
            }
        }
    }

    /// Data related to an NFT Series.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTSeriesDetails<AccountId> {
        pub owner: AccountId, // Series Owner
        pub draft: bool, // If Yes, the owner can add new nfts to that series but cannot list that nft for sale
    }

    impl<AccountId> NFTSeriesDetails<AccountId> {
        pub fn new(owner: AccountId, draft: bool) -> Self {
            Self { owner, draft }
        }
    }
}
