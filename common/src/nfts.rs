//! Common NFT specific traits

use frame_support::dispatch::DispatchResult;
use sp_runtime::DispatchError;
use sp_std::result;

/// Implemented by a pallet that supports the creation and transfer of NFTs.
pub trait NFTs {
    type AccountId;

    // How details related to an NFT are represented.
    type NFTDetails;

    /// How NFTs are represented internally.
    type NFTId;

    /// Create a new NFT with the specified details and return its ID or an error.
    fn create(
        owner: &Self::AccountId,
        details: Self::NFTDetails,
    ) -> result::Result<Self::NFTId, DispatchError>;

    /// Change the details related to an NFT.
    fn mutate<F: FnOnce(&Self::AccountId, &mut Self::NFTDetails) -> DispatchResult>(
        id: Self::NFTId,
        f: F,
    ) -> DispatchResult;

    /// Change the owner of an NFT.
    fn set_owner(id: Self::NFTId, owner: &Self::AccountId) -> DispatchResult;

    /// Return the details related to an NFT.
    fn details(id: Self::NFTId) -> Self::NFTDetails;

    /// Return the owner of an NFT.
    fn owner(id: Self::NFTId) -> Self::AccountId;

    /// Mark an NFT as "sealed", this will make any future mutation fail.
    fn seal(id: Self::NFTId) -> DispatchResult;

    /// Check wether an NFT is sealed.
    fn sealed(id: Self::NFTId) -> bool;
}

/// Implemented by a pallet where it is possible to lock NFTs.
pub trait LockableNFTs {}
