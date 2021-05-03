//! Common NFT specific traits

use frame_support::{dispatch::DispatchResult, Parameter};
use sp_runtime::DispatchError;
use sp_std::result;

/// Implemented by a pallet that supports the creation and transfer of NFTs.
pub trait NFTs {
    type AccountId;

    // How details related to an NFT are represented.
    type NFTDetails: Default;

    /// How NFTs are represented internally.
    type NFTId: Parameter + Copy;

    type NFTSeriesId: Parameter + Copy + Default;

    /// Create a new NFT with the specified details and return its ID or an error.
    fn create(
        owner: &Self::AccountId,
        details: Self::NFTDetails,
        series_id: Self::NFTSeriesId,
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

    /// Remove an NFT from the storage.
    fn burn(id: Self::NFTId) -> DispatchResult;

    /// TODO!
    fn series_id(id: Self::NFTId) -> Option<Self::NFTSeriesId>;
}

/// Implemented by a pallet where it is possible to lock NFTs.
pub trait LockableNFTs {
    type AccountId;

    /// How NFTs are represented internally.
    type NFTId: Parameter + Copy;

    /// Mark an NFT as locked thus preventing further owner changes or transfers.
    /// Note that this doesn't mark the token as sealed and thus it could still has
    /// its metadata changed by its actual owner.
    fn lock(id: Self::NFTId) -> DispatchResult;

    /// Unlock a locked NFT.
    fn unlock(id: Self::NFTId);

    /// Return the lock status of an NFT.
    fn locked(id: Self::NFTId) -> bool;
}
