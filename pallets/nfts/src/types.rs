use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// How the NFT series id is encoded.
pub type NFTSeriesId = u32;

/// TODO!
#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Protocol {
    Safe = 1,
    DDay = 2,
    Consent = 3,
    Death = 4,
    Countdown = 5,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::Safe
    }
}

/// Data related to NFTs on the Ternoa Chain.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTDetails {
    /// ASCII encoded URI to fetch additional metadata
    pub offchain_uri: Vec<u8>,
    /// TODO!
    pub is_capsule: bool,
    /// TODO!
    pub series_id: NFTSeriesId,
    /// TODO!
    pub protocol: Option<Protocol>,
}

impl NFTDetails {
    pub fn new(
        offchain_uri: Vec<u8>,
        is_capsule: bool,
        series_id: NFTSeriesId,
        protocol: Option<Protocol>,
    ) -> Self {
        Self {
            offchain_uri,
            series_id,
            is_capsule,
            protocol,
        }
    }

    pub fn unique_series(&self) -> bool {
        self.series_id != NFTSeriesId::default()
    }
}

/// Data related to an NFT, such as who is its owner.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTData<AccountId, NFTDetails> {
    pub owner: AccountId,
    pub details: NFTDetails,
    /// Set to true to prevent further modifications to the details struct
    pub sealed: bool,
    /// Set to true to prevent changes to the owner variable
    pub locked: bool,
}

impl<AccountId, NFTDetails> NFTData<AccountId, NFTDetails> {
    pub fn new(owner: AccountId, details: NFTDetails, sealed: bool, locked: bool) -> Self {
        Self {
            owner,
            details,
            sealed,
            locked,
        }
    }
}

/// Data related to an NFT Series.
#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTSeriesDetails<AccountId, NFTId> {
    // Series owner.
    pub owner: AccountId,
    // NFTs that are part of the same series.
    pub nfts: Vec<NFTId>,
}

impl<AccountId, NFTId> NFTSeriesDetails<AccountId, NFTId> {
    pub fn new(owner: AccountId, nfts: Vec<NFTId>) -> Self {
        Self { owner, nfts }
    }
}
