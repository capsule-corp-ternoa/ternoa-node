#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod tests;

mod default_weights;
mod types;

use default_weights::WeightInfo;
use frame_support::pallet_prelude::DispatchResult;
use frame_support::transactional;
use frame_system::ensure_signed;
use sp_runtime::DispatchError;
use ternoa_common::traits::{CapsulesTrait, NFTTrait};
use types::TransmissionData;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
    use ternoa_common::traits::{CapsulesTrait, NFTTrait};
    use ternoa_primitives::{nfts::NFTId, BlockNumber};

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Pallet managing NFTs.
        type NFTs: NFTTrait<AccountId = Self::AccountId>;
        /// Capsules Pallet
        type Capsules: CapsulesTrait<AccountId = Self::AccountId>;
        /// Weight values for this pallet
        type WeightInfo: WeightInfo;
    }

    #[pallet::storage]
    #[pallet::getter(fn transmissions)]
    pub type Transmissions<T: Config> =
        StorageMap<_, Blake2_128Concat, NFTId, TransmissionData<T::AccountId>, OptionQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig {}

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {}
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {}
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Transmission has been store
        TransmissionStored {
            nft_id: NFTId,
            delivery_date: BlockNumber,
            recipient: T::AccountId,
        },
        /// Transmission has been completed
        TransmissionCompleted { nft_id: NFTId },
        /// Transmission has been cancelled
        TransmissionCancelled { nft_id: NFTId },
    }

    #[pallet::error]
    pub enum Error<T> {
        /// This function is reserved to the owner of a capsule.
        NotCapsuleOwner,
        /// Unknown Capsule
        UnknownCapsule,
        /// Is already stored in transmissions
        AlreadyInTransmission,
        /// Should be stored in transmissions
        NotInTransmission,
        /// Should not be listed for sale
        IsListedForSale,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            for data in Transmissions::<T>::iter() {
                let (nft_id, transmission_data) = data;
                if n >= transmission_data.delivery_date.into() {
                    if Self::transmits_capsule(nft_id, transmission_data.recipient).is_ok() {
                        Self::deposit_event(Event::TransmissionCompleted { nft_id });
                    } else {
                        return 0;
                    }
                }
            }
            1
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Add a new capsule Transmission into storage
        #[pallet::weight(T::WeightInfo::dday_transmission())]
        #[transactional]
        pub fn dday_transmission(
            origin: OriginFor<T>,
            nft_id: NFTId,
            recipient: T::AccountId,
            delivery_date: BlockNumber,
        ) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            let capsule = T::Capsules::get_capsule(nft_id).ok_or(Error::<T>::UnknownCapsule)?;
            ensure!(account_id == capsule.owner, Error::<T>::NotCapsuleOwner);

            ensure!(
                T::NFTs::is_listed_for_sale(nft_id) == Some(false),
                Error::<T>::IsListedForSale
            );
            ensure!(
                T::NFTs::is_in_transmission(nft_id) == Some(false),
                Error::<T>::AlreadyInTransmission
            );

            T::NFTs::set_in_transmission(nft_id, true)?;
            Transmissions::<T>::insert(nft_id, TransmissionData::new(recipient.clone(), delivery_date));
            Self::deposit_event(Event::TransmissionStored {
                nft_id,
                delivery_date,
                recipient,
            });
            Ok(())
        }

        /// Cancel capsule Transmission
        #[pallet::weight(T::WeightInfo::cancel())]
        pub fn cancel(origin: OriginFor<T>, nft_id: NFTId) -> DispatchResult {
            let account_id = ensure_signed(origin)?;
            let capsule = T::Capsules::get_capsule(nft_id).ok_or(Error::<T>::UnknownCapsule)?;
            ensure!(account_id == capsule.owner, Error::<T>::NotCapsuleOwner);

            ensure!(
                T::NFTs::is_in_transmission(nft_id) == Some(true),
                Error::<T>::NotInTransmission
            );

            Transmissions::<T>::remove(nft_id);
            T::NFTs::set_in_transmission(nft_id, false)?;
            Self::deposit_event(Event::TransmissionCancelled { nft_id });
            Ok(())
        }
    }
}

impl<T: Config> Pallet<T> {
    #[transactional]
    fn transmits_capsule(
        nft_id: u32,
        recipient: T::AccountId,
    ) -> Result<(), DispatchError> {
        T::Capsules::set_owner(nft_id, &recipient)?;
        T::NFTs::set_owner(nft_id, &recipient)?;
        T::NFTs::set_in_transmission(nft_id, false)?;
        Transmissions::<T>::remove(nft_id);
        Ok(())
    }
}
