#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    #[pallet::event]
    pub enum Event<T: Config> {
        TestEvent,
    }

    #[pallet::error]
    pub enum Error<T> {
        TestError,
    }

    #[pallet::storage]
    #[pallet::getter(fn is_capsulized)]
    pub type IsCapsulized<T: Config> = StorageValue<_, bool, ValueQuery>;

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
}

impl<T: Config> Pallet<T> {
    pub fn set_is_capsulized(val: bool) {
        IsCapsulized::<T>::put(val);
    }
}

impl<T: Config> ternoa_common::traits::CapsulesTrait for Pallet<T> {
    fn is_capsulized(_nft_id: ternoa_primitives::nfts::NFTId) -> bool {
        IsCapsulized::<T>::get()
    }
}
