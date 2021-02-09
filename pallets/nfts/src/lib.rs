#![cfg_attr(not(feature = "std"), no_std)]

use codec::Encode;
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure,
    traits::{
        schedule::{DispatchTime, Named as ScheduleNamed},
        LockIdentifier,
    },
    weights::Weight,
    Parameter,
};
use frame_system::{ensure_root, ensure_signed, offchain::Account, RawOrigin};
use sp_runtime::{traits::Dispatchable, traits::StaticLookup, DispatchResult};
use ternoa_common::traits::{LockableNFTs, NFTs};

#[cfg(test)]
mod tests;

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// How NFTs are represented
    type NFTId;
    /// How NFT details are represented
    type NFTDetails;
}

decl_storage! {
    trait Store for Module<T: Trait> as NFTs {
        //
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        NFTId = <T as Trait>::NFTId,
    {
        /// A new NFT was created. \[nft id, owner\]
        Created(NFTId, AccountId),
        /// An NFT was transferred to someone else. \[nft id, old owner, new owner\]
        Transfer(NFTId, AccountId, AccountId),
        /// An NFT was updated by its owner. \[nft id\]
        Mutated(NFTId),
        /// An NFT was sealed, preventing any new mutations. \[nft id\]
        Sealed(NFTId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {}
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        // create(origin, category: CategoryId, details: NFTDetails)
        // mutate(origin, id: NFTId, details: NFTDetails)
        // transfer(origin, id: NFTId, who: T::AccountId)
        // seal(origin, id: NFTId)
    }
}
