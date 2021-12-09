use frame_support::dispatch::{DispatchErrorWithPostInfo, DispatchResult};
use ternoa_primitives::nfts::{NFTId, NFTSeriesId};
use ternoa_primitives::TernoaString;

pub trait NFTs {
    type AccountId;

    /// Change the owner of an NFT.
    fn set_owner(id: NFTId, owner: &Self::AccountId) -> DispatchResult;

    /// Return the owner of an NFT.
    fn owner(id: NFTId) -> Option<Self::AccountId>;

    /// Is series completed(locked)
    fn is_series_completed(id: NFTId) -> Option<bool>;

    /// Create NFT
    fn create_nft(
        owner: Self::AccountId,
        ipfs_reference: TernoaString,
        series_id: Option<NFTSeriesId>,
    ) -> Result<NFTId, DispatchErrorWithPostInfo>;

    /// Lock series WARNING: Only for benchmark purposes!
    fn benchmark_lock_series(series_id: NFTSeriesId);
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

pub trait CapsulesTrait {
    fn is_capsulized(nft_id: NFTId) -> bool;
}
