#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub struct WeightInfo<T>(PhantomData<T>);

impl<T: frame_system::Config> ternoa_tee::WeightInfo for WeightInfo<T> {
    fn register_enclave() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn assign_enclave() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn unassign_enclave() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn update_enclave() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn change_enclave_owner() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn create_cluster() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn remove_cluster() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn register_enclave_provider() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn register_provider_keys() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

    fn register_enclave_operator() -> Weight {
        Weight::from_ref_time(10_000_000 as u64)
    }

}