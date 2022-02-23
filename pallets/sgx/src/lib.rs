#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod tests;

mod default_weights;
mod types;

use frame_support::dispatch::DispatchResultWithPostInfo;
pub use pallet::*;
pub use types::*;

use default_weights::WeightInfo;
use frame_support::traits::StorageVersion;

/// The current storage version.
const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_support::traits::ExistenceRequirement::KeepAlive;
	use frame_support::traits::{Currency, OnUnbalanced, WithdrawReasons};
	use frame_support::transactional;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::StaticLookup;
	use ternoa_common::helpers::check_bounds;
	use ternoa_primitives::TextFormat;

	pub type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
	pub(crate) type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::NegativeImbalance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Weight values for this pallet
		type WeightInfo: WeightInfo;

		/// Currency used to bill minting fees
		type Currency: Currency<Self::AccountId>;

		/// Host much does it cost to mint enclave (extra fee on top of the tx fees)
		#[pallet::constant]
		type EnclaveFee: Get<BalanceOf<Self>>;

		/// What we do with additional fees
		type FeesCollector: OnUnbalanced<NegativeImbalanceOf<Self>>;

		/// Size of a cluster
		#[pallet::constant]
		type ClusterSize: Get<u32>;

		/// Min Uri len
		#[pallet::constant]
		type MinUriLen: Get<u16>;

		/// Max Uri len
		#[pallet::constant]
		type MaxUriLen: Get<u16>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		//
		// Enclave
		//
		#[pallet::weight(T::WeightInfo::register_enclave())]
		#[transactional]
		pub fn register_enclave(
			origin: OriginFor<T>,
			api_uri: TextFormat,
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;

			check_bounds(
				api_uri.len(),
				(T::MinUriLen::get(), Error::<T>::UriTooShort),
				(T::MaxUriLen::get(), Error::<T>::UriTooLong),
			)?;

			ensure!(
				!EnclaveIndex::<T>::contains_key(&account),
				Error::<T>::PublicKeyAlreadyTiedToACluster
			);
			let (enclave_id, new_id) = Self::new_enclave_id()?;
			// Needs to have enough money
			let imbalance = T::Currency::withdraw(
				&account,
				T::EnclaveFee::get(),
				WithdrawReasons::FEE,
				KeepAlive,
			)?;
			T::FeesCollector::on_unbalanced(imbalance);

			let enclave = Enclave::new(api_uri.clone());

			EnclaveIndex::<T>::insert(account.clone(), enclave_id);
			EnclaveRegistry::<T>::insert(enclave_id, enclave);
			EnclaveIdGenerator::<T>::put(new_id);

			Self::deposit_event(Event::AddedEnclave { account, api_uri, enclave_id });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::assign_enclave())]
		pub fn assign_enclave(
			origin: OriginFor<T>,
			cluster_id: ClusterId,
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			let enclave_id = EnclaveIndex::<T>::get(&account).ok_or(Error::<T>::NotEnclaveOwner)?;

			ensure!(
				!ClusterIndex::<T>::contains_key(enclave_id),
				Error::<T>::EnclaveAlreadyAssigned,
			);

			ClusterRegistry::<T>::mutate(cluster_id, |cluster_opt| {
				if let Some(cluster) = cluster_opt {
					if cluster.enclaves.len() >= T::ClusterSize::get() as usize {
						return Err(Error::<T>::ClusterIsAlreadyFull);
					}

					cluster.enclaves.push(enclave_id);
					ClusterIndex::<T>::insert(enclave_id, cluster_id);

					Ok(())
				} else {
					Err(Error::<T>::UnknownClusterId)
				}
			})?;

			Self::deposit_event(Event::AssignedEnclave { enclave_id, cluster_id });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::unassign_enclave())]
		pub fn unassign_enclave(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			let enclave_id = EnclaveIndex::<T>::get(&account).ok_or(Error::<T>::NotEnclaveOwner)?;
			let cluster_id =
				ClusterIndex::<T>::get(enclave_id).ok_or(Error::<T>::EnclaveNotAssigned)?;

			ClusterRegistry::<T>::mutate(cluster_id, |cluster_opt| {
				if let Some(cluster) = cluster_opt {
					let index = cluster
						.enclaves
						.iter()
						.position(|x| *x == enclave_id)
						.ok_or(Error::<T>::InternalLogicalError)?;
					cluster.enclaves.remove(index);
					ClusterIndex::<T>::remove(enclave_id);
					Ok(())
				} else {
					Err(Error::<T>::UnknownClusterId)
				}
			})?;

			Self::deposit_event(Event::UnAssignedEnclave { enclave_id });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::update_enclave())]
		pub fn update_enclave(
			origin: OriginFor<T>,
			api_uri: TextFormat,
		) -> DispatchResultWithPostInfo {
			let account = ensure_signed(origin)?;
			let enclave_id = EnclaveIndex::<T>::get(&account).ok_or(Error::<T>::NotEnclaveOwner)?;

			check_bounds(
				api_uri.len(),
				(T::MinUriLen::get(), Error::<T>::UriTooShort),
				(T::MaxUriLen::get(), Error::<T>::UriTooLong),
			)?;

			EnclaveRegistry::<T>::mutate(enclave_id, |enclave| -> DispatchResult {
				let enclave = enclave.as_mut().ok_or(Error::<T>::UnknownEnclaveId)?;
				enclave.api_uri = api_uri.clone();

				Ok(())
			})?;

			Self::deposit_event(Event::UpdatedEnclave { enclave_id, api_uri });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::change_enclave_owner())]
		pub fn change_enclave_owner(
			origin: OriginFor<T>,
			new_owner: <T::Lookup as StaticLookup>::Source,
		) -> DispatchResultWithPostInfo {
			let old_owner = ensure_signed(origin)?;
			let new_owner = T::Lookup::lookup(new_owner)?;

			let enclave_id =
				EnclaveIndex::<T>::get(old_owner.clone()).ok_or(Error::<T>::NotEnclaveOwner)?;

			ensure!(
				!EnclaveIndex::<T>::contains_key(&new_owner),
				Error::<T>::PublicKeyAlreadyTiedToACluster
			);

			ensure!(EnclaveRegistry::<T>::contains_key(enclave_id), Error::<T>::UnknownEnclaveId);

			EnclaveIndex::<T>::remove(old_owner);
			EnclaveIndex::<T>::insert(new_owner.clone(), enclave_id);

			Self::deposit_event(Event::NewEnclaveOwner { enclave_id, owner: new_owner });
			Ok(().into())
		}

		//
		// Cluster
		//
		#[pallet::weight(T::WeightInfo::create_cluster())]
		pub fn create_cluster(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let id = ClusterIdGenerator::<T>::get();
			let new_id = id.checked_add(1).ok_or(Error::<T>::ClusterIdOverflow)?;
			let cluster = Cluster::new(Default::default());

			ClusterRegistry::<T>::insert(id, cluster);
			ClusterIdGenerator::<T>::put(new_id);

			Self::deposit_event(Event::AddedCluster { cluster_id: id });
			Ok(().into())
		}

		#[pallet::weight(T::WeightInfo::remove_cluster())]
		pub fn remove_cluster(
			origin: OriginFor<T>,
			cluster_id: ClusterId,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			ClusterRegistry::<T>::mutate(cluster_id, |cluster_opt| {
				if let Some(cluster) = cluster_opt {
					for enclave_id in &cluster.enclaves {
						if ClusterIndex::<T>::take(enclave_id).is_none() {
							return Err(Error::<T>::InternalLogicalError);
						}
					}
					Ok(())
				} else {
					Err(Error::<T>::UnknownClusterId)
				}
			})?;
			ClusterRegistry::<T>::take(cluster_id);

			Self::deposit_event(Event::RemovedCluster { cluster_id });
			Ok(().into())
		}
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		// Enclave
		AddedEnclave { account: T::AccountId, api_uri: TextFormat, enclave_id: EnclaveId },
		AssignedEnclave { enclave_id: EnclaveId, cluster_id: ClusterId },
		UnAssignedEnclave { enclave_id: EnclaveId },
		UpdatedEnclave { enclave_id: EnclaveId, api_uri: TextFormat },
		NewEnclaveOwner { enclave_id: EnclaveId, owner: T::AccountId },
		// Cluster
		AddedCluster { cluster_id: ClusterId },
		RemovedCluster { cluster_id: ClusterId },
	}

	#[pallet::error]
	pub enum Error<T> {
		UnknownEnclaveId,
		UnknownClusterId,
		NotEnclaveOwner,
		PublicKeyAlreadyTiedToACluster,
		UriTooShort,
		UriTooLong,
		EnclaveIdOverflow,
		ClusterIdOverflow,
		ClusterIsAlreadyFull,
		EnclaveAlreadyAssigned,
		EnclaveNotAssigned,
		CannotAssignToSameCluster,
		InternalLogicalError,
	}

	//
	// Enclave
	//
	#[pallet::storage]
	#[pallet::getter(fn enclave_registry)]
	pub type EnclaveRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, EnclaveId, Enclave, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn enclave_id_generator)]
	pub type EnclaveIdGenerator<T: Config> = StorageValue<_, EnclaveId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn enclave_index)]
	pub type EnclaveIndex<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, EnclaveId, OptionQuery>;

	//
	// Cluster
	//
	#[pallet::storage]
	#[pallet::getter(fn cluster_registry)]
	pub type ClusterRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, ClusterId, Cluster, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cluster_id_generator)]
	pub type ClusterIdGenerator<T: Config> = StorageValue<_, ClusterId, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn cluster_index)]
	pub type ClusterIndex<T: Config> =
		StorageMap<_, Blake2_128Concat, EnclaveId, ClusterId, OptionQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub enclaves: Vec<(T::AccountId, EnclaveId, TextFormat)>,
		pub clusters: Vec<(ClusterId, Vec<EnclaveId>)>,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { enclaves: Default::default(), clusters: Default::default() }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			let enclaves = self.enclaves.clone();
			if let Some(enclave) = enclaves.last() {
				EnclaveIdGenerator::<T>::put(enclave.1 + 1);
			}

			for enclave in enclaves {
				EnclaveIndex::<T>::insert(enclave.0, enclave.1);
				EnclaveRegistry::<T>::insert(enclave.1, Enclave { api_uri: enclave.2 });
			}

			let clusters = self.clusters.clone();
			if let Some(cluster) = clusters.last() {
				ClusterIdGenerator::<T>::put(cluster.0 + 1);
			}

			for cluster in clusters {
				for enclave_id in cluster.1.iter() {
					ClusterIndex::<T>::insert(*enclave_id, cluster.0);
				}
				ClusterRegistry::<T>::insert(cluster.0, Cluster::new(cluster.1));
			}
		}
	}
}

impl<T: Config> Pallet<T> {
	pub fn new_enclave_id() -> Result<(EnclaveId, EnclaveId), Error<T>> {
		let id = EnclaveIdGenerator::<T>::get();
		let new_id = id.checked_add(1).ok_or(Error::<T>::EnclaveIdOverflow)?;

		Ok((id, new_id))
	}
}
