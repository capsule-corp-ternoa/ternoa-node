#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure, weights::Weight};
use frame_system::ensure_signed;
use sp_runtime::{traits::StaticLookup, DispatchError, DispatchResult};
use ternoa_common::traits::{CapsuleCreationEnabled, CapsuleTransferEnabled};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;
mod types;

pub use types::{CapsuleData, CapsuleID};

pub trait WeightInfo {
    fn create() -> Weight;
    fn mutate() -> Weight;
    fn transfer() -> Weight;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// Weight values for this pallet
    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Trait> as Capsules {
        /// Total number of capsules created, also used to create capsule IDs.
        pub Total get(fn total): CapsuleID;
        /// Metadata associated to all capsules.
        pub Metadata get(fn metadata): map hasher(blake2_128_concat) CapsuleID => CapsuleData<T::AccountId, T::Hash>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
        Hash = <T as frame_system::Trait>::Hash,
    {
        /// A capsule was created. \[id, creator, data\]
        CapsuleCreated(CapsuleID, AccountId, CapsuleData<AccountId, Hash>),
        /// A capsule has been transferred to a new address. \[id, new owner\]
        CapsuleTransferred(CapsuleID, AccountId),
        /// A capsule's data was updated. \[id, new data\]
        CapsuleUpdated(CapsuleID, CapsuleData<AccountId, Hash>),
        /// A capsule was locked. \[id\]
        CapsuleLocked(CapsuleID),
        /// A capsule was unlocked. \[id\]
        CapsuleUnlocked(CapsuleID),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// This function is reserved to the owner of a capsule.
        NotCapsuleOwner,
        /// Creating a new capsule would produce a number of overflow.
        /// A runtime upgrade is necessary to re-enable capsule creations.
        OutOfCapsuleIDs,
        /// The metadata passed to the function is malformed, verify the
        /// `creator` and `owner` values.
        MalformedMetadata,
        /// Capsule has been locked for transfers, it needs to be unlocked
        /// first. Typically, this would happen because another pallet
        /// locked the capsule.
        CapsuleLocked,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create a new capsule with the given metadata. An event will be triggered with
        /// the capsule id. Make sure that the `owner` field in the `data` is set to the
        /// correct account.
        #[weight = T::WeightInfo::create()]
        pub fn create(origin, data: CapsuleData<T::AccountId, T::Hash>) {
            let who = ensure_signed(origin)?;
            let capsule_id = <Self as CapsuleCreationEnabled>::create(&who, data.clone())?;
            Self::deposit_event(RawEvent::CapsuleCreated(capsule_id, who, data));
        }

        /// Transfer a capsule to another account. This would mutate the `owner` value of
        /// the metadata.
        #[weight = T::WeightInfo::transfer()]
        fn transfer(origin, to: <T::Lookup as StaticLookup>::Source, capsule_id: CapsuleID) {
            let who = ensure_signed(origin)?;
            let to_unlookup = T::Lookup::lookup(to)?;
            Self::transfer_from(who, to_unlookup, capsule_id)?;
        }

        /// Modify a capsule's attached data. Make sure `owner` and `creator` are not modified.
        #[weight = T::WeightInfo::mutate()]
        fn mutate(origin, capsule_id: CapsuleID, data: CapsuleData<T::AccountId, T::Hash>) {
            let who = ensure_signed(origin)?;
            let capsule = Self::metadata(capsule_id);
            ensure!(capsule.owner == who, Error::<T>::NotCapsuleOwner);
            ensure!(capsule.owner == data.owner, Error::<T>::MalformedMetadata);
            ensure!(capsule.creator == data.creator, Error::<T>::MalformedMetadata);
            ensure!(capsule.locked == data.locked, Error::<T>::MalformedMetadata);

            Metadata::<T>::insert(capsule_id, data.clone());

            Self::deposit_event(RawEvent::CapsuleUpdated(capsule_id, data));
        }
    }
}

impl<T: Trait> CapsuleTransferEnabled for Module<T> {
    type AccountId = T::AccountId;
    type CapsuleID = CapsuleID;

    fn transfer_from(
        from: Self::AccountId,
        to: Self::AccountId,
        capsule_id: Self::CapsuleID,
    ) -> DispatchResult {
        let mut capsule = Self::metadata(capsule_id);
        ensure!(!capsule.locked, Error::<T>::CapsuleLocked);
        ensure!(capsule.owner == from, Error::<T>::NotCapsuleOwner);
        capsule.owner = to.clone();
        Metadata::<T>::insert(capsule_id, capsule);

        Self::deposit_event(RawEvent::CapsuleTransferred(capsule_id, to));
        Ok(())
    }

    fn lock(capsule_id: Self::CapsuleID) -> DispatchResult {
        Metadata::<T>::mutate(capsule_id, |capsule| capsule.locked = true);
        Self::deposit_event(RawEvent::CapsuleLocked(capsule_id));
        Ok(())
    }

    fn unlock(capsule_id: Self::CapsuleID) -> DispatchResult {
        Metadata::<T>::mutate(capsule_id, |capsule| capsule.locked = false);
        Self::deposit_event(RawEvent::CapsuleUnlocked(capsule_id));
        Ok(())
    }

    fn is_locked(capsule_id: Self::CapsuleID) -> bool {
        Self::metadata(capsule_id).locked
    }

    fn is_owner(maybe_owner: Self::AccountId, capsule_id: Self::CapsuleID) -> bool {
        Self::metadata(capsule_id).owner == maybe_owner
    }
}

impl<T: Trait> CapsuleCreationEnabled for Module<T> {
    type AccountId = T::AccountId;
    type CapsuleID = CapsuleID;
    type CapsuleData = CapsuleData<T::AccountId, T::Hash>;

    fn create(
        owner: &Self::AccountId,
        data: Self::CapsuleData,
    ) -> Result<Self::CapsuleID, DispatchError> {
        ensure!(&data.creator == owner, Error::<T>::MalformedMetadata);
        ensure!(&data.owner == owner, Error::<T>::MalformedMetadata);
        ensure!(data.locked == false, Error::<T>::MalformedMetadata);

        let capsule_id = Self::total()
            .checked_add(1)
            .ok_or(Error::<T>::OutOfCapsuleIDs)?;
        Metadata::<T>::insert(capsule_id, data.clone());
        Total::put(capsule_id);

        Ok(capsule_id)
    }
}
