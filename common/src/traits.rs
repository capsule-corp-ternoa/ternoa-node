//! Shared traits.

use frame_support::Parameter;
use sp_runtime::DispatchResult;

/// Implemented by a pallet that supports the transfers of capsules.
pub trait CapsuleTransferEnabled {
    /// Underlying runtime's account ids.
    type AccountId;
    /// How capsules are represented in the underlying pallet.
    type CapsuleID: Parameter + Copy;

    /// Transfer a capsule `capsule_id` from `from` to `to`. This should
    /// perform a sanity check to make sure that `from` is still the owner
    /// of the capsule.
    fn transfer_from(
        from: Self::AccountId,
        to: Self::AccountId,
        capsule_id: Self::CapsuleID,
    ) -> DispatchResult;

    /// Prevent a capsule from being transferred in the future.
    fn lock(capsule_id: Self::CapsuleID) -> DispatchResult;

    /// Unlock a capsule for transfers.
    fn unlock(capsule_id: Self::CapsuleID) -> DispatchResult;

    /// Returns true if `maybe_owner` owns `capsule_id`.
    fn is_owner(maybe_owner: Self::AccountId, capsule_id: Self::CapsuleID) -> bool;
}
