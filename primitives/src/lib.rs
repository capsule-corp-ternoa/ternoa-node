//! Low level primitives for the runtime and node.

#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentifyAccount, Verify},
    MultiSignature, OpaqueExtrinsic,
};

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

pub mod nfts {
    #[cfg(feature = "std")]
    use serde::{Deserialize, Serialize};

    use codec::{Decode, Encode};
    use sp_runtime::RuntimeDebug;
    use sp_std::vec::Vec;

    /// How NFT IDs are encoded.
    pub type NFTId = u32;

    /// How NFT IDs are encoded. In the JSON Types this should be "Text" and not "Vec<8>".
    pub type NFTSeriesId = Vec<u8>;

    // String type
    pub type NFTString = Vec<u8>;

    /// Data related to an NFT, such as who is its owner.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
    #[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
    pub struct NFTData<AccountId> {
        // NFT owner
        pub owner: AccountId,
        // IPFS reference
        pub ipfs_reference: NFTString,
        // Series ID
        pub series_id: NFTSeriesId,
        // Is Locked
        pub locked: bool,
    }

    impl<AccountId> NFTData<AccountId> {
        pub fn new(
            owner: AccountId,
            ipfs_reference: NFTString,
            series_id: NFTSeriesId,
            locked: bool,
        ) -> Self {
            Self {
                owner,
                ipfs_reference,
                series_id,
                locked,
            }
        }
    }

    /// Data related to an NFT Series.
    #[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
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
