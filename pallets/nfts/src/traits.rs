//! Common NFT specific traits

// use frame_support::dispatch::DispatchErrorWithPostInfo;
// use frame_support::dispatch::DispatchResult;
// use ternoa_primitives::nfts::{NFTId, NFTSeriesId, NFTString};

use frame_support::dispatch::DispatchResult;
use ternoa_primitives::nfts::NFTId;

/// Implemented by a pallet that supports the creation and transfer of NFTs.
pub trait NFTs {
    type AccountId;

    /// Change the owner of an NFT.
    fn set_owner(id: NFTId, owner: &Self::AccountId) -> DispatchResult;

    /// Return the owner of an NFT.
    fn owner(id: NFTId) -> Option<Self::AccountId>;

    /// Is series completed(locked)
    fn is_series_completed(id: NFTId) -> Option<bool>;

    /*     fn create_nft(
        owner: Self::AccountId,
        ipfs_reference: NFTString,
        series_id: Option<NFTSeriesId>,
    ) -> Result<NFTId, DispatchErrorWithPostInfo>; */
}

/// Implemented by a pallet where it is possible to lock NFTs.
pub trait LockableNFTs {
    type AccountId;

    /// Mark an NFT as locked thus preventing further owner changes or transfers.
    /// Note that this doesn't mark the token as sealed and thus it could still has
    /// its metadata changed by its actual owner.
    fn lock(id: NFTId) -> DispatchResult;

    /// Unlock a locked NFT.
    fn unlock(id: NFTId) -> bool;

    /// Return the lock status of an NFT.
    fn locked(id: NFTId) -> Option<bool>;
}
