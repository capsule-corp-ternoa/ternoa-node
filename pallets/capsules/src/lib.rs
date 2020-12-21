use frame_support::{decl_error, decl_event, decl_module, decl_storage, ensure};
use frame_system::ensure_signed;
use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod tests;
mod types;

use types::{CapsuleData, CapsuleID};

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
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
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create a new capsule with the given metadata. An event will be triggered with
        /// the capsule id. Make sure that the `owner` field in the `data` is set to the
        /// correct account.
        #[weight = 0]
        fn create(origin, data: CapsuleData<T::AccountId, T::Hash>) {
            let who = ensure_signed(origin)?;
            ensure!(data.creator == who, Error::<T>::MalformedMetadata);
            ensure!(data.owner == who, Error::<T>::MalformedMetadata);

            let capsule_id = Self::total().checked_add(1).ok_or(Error::<T>::OutOfCapsuleIDs)?;
            Metadata::<T>::insert(capsule_id, data.clone());
            Total::put(capsule_id);

            Self::deposit_event(RawEvent::CapsuleCreated(capsule_id, who, data));
        }

        /// Transfer a capsule to another account. This would mutate the `owner` value of
        /// the metadata.
        #[weight = 0]
        fn transfer(origin, to: <T::Lookup as StaticLookup>::Source, capsule_id: CapsuleID) {
            let who = ensure_signed(origin)?;
            let mut capsule = Self::metadata(capsule_id);
            ensure!(capsule.owner == who, Error::<T>::NotCapsuleOwner);

            let to_unlookup = T::Lookup::lookup(to)?;
            capsule.owner = to_unlookup.clone();
            Metadata::<T>::insert(capsule_id, capsule);

            Self::deposit_event(RawEvent::CapsuleTransferred(capsule_id, to_unlookup));
        }

        /// Modify a capsule's attached data. Make sure `owner` and `creator` are not modified.
        #[weight = 0]
        fn mutate(origin, capsule_id: CapsuleID, data: CapsuleData<T::AccountId, T::Hash>) {
            let who = ensure_signed(origin)?;
            let capsule = Self::metadata(capsule_id);
            ensure!(capsule.owner == who, Error::<T>::NotCapsuleOwner);
            ensure!(capsule.owner == data.owner, Error::<T>::MalformedMetadata);
            ensure!(capsule.creator == data.creator, Error::<T>::MalformedMetadata);

            Metadata::<T>::insert(capsule_id, data.clone());

            Self::deposit_event(RawEvent::CapsuleUpdated(capsule_id, data));
        }
    }
}
