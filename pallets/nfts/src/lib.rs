#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, ensure, weights::Weight, Parameter,
};
use frame_system::ensure_signed;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_runtime::{
    traits::{CheckedAdd, MaybeSerializeDeserialize, Member, StaticLookup},
    DispatchError, DispatchResult, RuntimeDebug,
};
use sp_std::result;
use ternoa_common::traits::{LockableNFTs, NFTs};

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod default_weights;
#[cfg(test)]
mod tests;

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

pub trait WeightInfo {
    fn create() -> Weight;
    fn mutate() -> Weight;
    fn seal() -> Weight;
    fn transfer() -> Weight;
}

pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// How NFTs are represented
    type NFTId: Parameter + Default + CheckedAdd + Copy + Member + From<u8>;
    /// How NFT details are represented
    type NFTDetails: Parameter + Member + MaybeSerializeDeserialize + Default;

    type WeightInfo: WeightInfo;
}

decl_storage! {
    trait Store for Module<T: Trait> as NFTs {
        /// The number of NFTs managed by this pallet
        pub Total get(fn total): T::NFTId;
        /// Data related to NFTs.
        pub Data get(fn data): map hasher(blake2_128_concat) T::NFTId => NFTData<T::AccountId, T::NFTDetails>;
    }
    add_extra_genesis {
        config(nfts): Vec<(T::AccountId, T::NFTDetails)>;
        // ^^ begin, length, amount liquid at genesis
        build(|config: &GenesisConfig<T>| {
            &config.nfts
                .clone()
                .into_iter()
                .for_each(|(account, details)| drop(<Module<T> as NFTs>::create(&account, details)));
        });
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
        /// An NFT has been locked, preventing transfers until it is unlocked.
        /// \[nft id\]
        Locked(NFTId),
        /// A locked NFT has been unlocked. \[nft id\]
        Unlocked(NFTId),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
        /// We do not have any NFT id left, a runtime upgrade is necessary.
        NFTIdOverflow,
        /// This function can only be called by the owner of the nft.
        NotOwner,
        /// NFT is sealed and no longer accepts mutations.
        Sealed,
        /// NFT is locked and thus its owner cannot be changed until it
        /// is unlocked.
        Locked,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        fn deposit_event() = default;

        /// Create a new NFT with the provided details. An ID will be auto
        /// generated and logged as an event, The caller of this function
        /// will become the owner of the new NFT.
        #[weight = T::WeightInfo::create()]
        fn create(origin, details: T::NFTDetails) {
            let who = ensure_signed(origin)?;
            let _id = <Self as NFTs>::create(&who, details)?;
        }

        /// Update the details included in an NFT. Must be called by the owner of
        /// the NFT and while the NFT is not sealed.
        #[weight = T::WeightInfo::mutate()]
        fn mutate(origin, id: T::NFTId, details: T::NFTDetails) {
            let who = ensure_signed(origin)?;
            <Self as NFTs>::mutate(id, |owner, dets| -> DispatchResult {
                ensure!(owner == &who, Error::<T>::NotOwner);

                *dets = details;

                Ok(())
            })?;
        }

        /// Transfer an NFT from an account to another one. Must be called by the
        /// actual owner of the NFT.
        #[weight = T::WeightInfo::transfer()]
        fn transfer(origin, id: T::NFTId, to: <T::Lookup as StaticLookup>::Source) {
            let who = ensure_signed(origin)?;
            let to_unlookup = T::Lookup::lookup(to)?;
            let mut data = Data::<T>::get(id);

            ensure!(data.owner == who, Error::<T>::NotOwner);
            ensure!(!data.locked, Error::<T>::Locked);

            data.owner = to_unlookup.clone();
            Data::<T>::insert(id, data);

            Self::deposit_event(RawEvent::Transfer(id, who, to_unlookup));
        }

        /// Mark an NFT as sealed, thus disabling further details modifications (but
        /// not preventing future transfers). Must be called by the owner of the NFT.
        #[weight = T::WeightInfo::seal()]
        fn seal(origin, id: T::NFTId) {
            let who = ensure_signed(origin)?;
            let mut data = Data::<T>::get(id);

            ensure!(!data.sealed, Error::<T>::Sealed);
            ensure!(data.owner == who, Error::<T>::NotOwner);

            data.sealed = true;
            Data::<T>::insert(id, data);

            Self::deposit_event(RawEvent::Sealed(id));
        }
    }
}

impl<T: Trait> NFTs for Module<T> {
    type AccountId = T::AccountId;
    type NFTDetails = T::NFTDetails;
    type NFTId = T::NFTId;

    fn create(
        owner: &Self::AccountId,
        details: Self::NFTDetails,
    ) -> result::Result<Self::NFTId, DispatchError> {
        let id = Total::<T>::get();
        Total::<T>::put(id.checked_add(&1.into()).ok_or(Error::<T>::NFTIdOverflow)?);
        Data::<T>::insert(
            id,
            NFTData {
                owner: owner.clone(),
                details,
                sealed: false,
                locked: false,
            },
        );

        Self::deposit_event(RawEvent::Created(id, owner.clone()));
        Ok(id)
    }

    fn mutate<F: FnOnce(&Self::AccountId, &mut Self::NFTDetails) -> DispatchResult>(
        id: Self::NFTId,
        f: F,
    ) -> DispatchResult {
        let mut data = Data::<T>::get(id);
        let mut details = data.details;

        ensure!(!data.sealed, Error::<T>::Sealed);
        f(&data.owner, &mut details)?;

        data.details = details;
        Data::<T>::insert(id, data);

        Self::deposit_event(RawEvent::Mutated(id));
        Ok(())
    }

    fn set_owner(id: Self::NFTId, owner: &Self::AccountId) -> DispatchResult {
        Data::<T>::try_mutate(id, |data| -> DispatchResult {
            ensure!(!data.locked, Error::<T>::Locked);
            (*data).owner = owner.clone();
            Ok(())
        })?;

        Ok(())
    }

    fn details(id: Self::NFTId) -> Self::NFTDetails {
        Data::<T>::get(id).details
    }

    fn owner(id: Self::NFTId) -> Self::AccountId {
        Data::<T>::get(id).owner
    }

    fn seal(id: Self::NFTId) -> DispatchResult {
        Data::<T>::mutate(id, |d| (*d).sealed = true);
        Self::deposit_event(RawEvent::Sealed(id));
        Ok(())
    }

    fn sealed(id: Self::NFTId) -> bool {
        Data::<T>::get(id).sealed
    }
}

impl<T: Trait> LockableNFTs for Module<T> {
    type AccountId = T::AccountId;
    type NFTId = T::NFTId;

    fn lock(id: Self::NFTId) -> DispatchResult {
        Data::<T>::try_mutate(id, |d| -> DispatchResult {
            ensure!(!d.locked, Error::<T>::Locked);
            (*d).locked = true;
            Ok(())
        })
    }

    fn unlock(id: Self::NFTId) {
        Data::<T>::mutate(id, |d| (*d).locked = false);
    }

    fn locked(id: Self::NFTId) -> bool {
        Data::<T>::get(id).locked
    }
}
