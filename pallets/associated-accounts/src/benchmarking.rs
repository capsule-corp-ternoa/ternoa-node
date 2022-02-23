#![cfg(feature = "runtime-benchmarks")]

use crate::{Account, Call, Config, Pallet, SupportedAccount, SupportedAccounts, Users};
use frame_benchmarking::{account, benchmarks};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::prelude::*;

use crate::Pallet as AAcounts;

benchmarks! {
	set_account {
		let alice: T::AccountId = account("ALICE", 0, 0);

		// Add supported account
		let supp = SupportedAccount::new(vec![20], 1, 20, true);
		assert_ok!(AAcounts::<T>::add_new_supported_account(RawOrigin::Root.into(), supp.key.clone(), supp.min_length, supp.max_length, supp.initial_set_fee));

		let acc = Account {key: supp.key.clone(), value: vec![50]};
	}: _(RawOrigin::Signed(alice.clone()), acc.key.clone(), acc.value.clone())
	verify {
		assert_eq!(Users::<T>::get(&alice), Some(vec![acc]));
	}

	add_new_supported_account {
		let supp = SupportedAccount::new(vec![20], 1, 20, true);
	}: _(RawOrigin::Root, supp.key.clone(), supp.min_length, supp.max_length, supp.initial_set_fee)
	verify {
		assert_eq!(SupportedAccounts::<T>::get(), vec![supp]);
	}

	remove_supported_account {
		// Add supported account
		let supp = SupportedAccount::new(vec![20], 1, 20, true);
		assert_ok!(AAcounts::<T>::add_new_supported_account(RawOrigin::Root.into(), supp.key.clone(), supp.min_length, supp.max_length, supp.initial_set_fee));

	}: _(RawOrigin::Root, supp.key)
	verify {
		assert_eq!(SupportedAccounts::<T>::get(), vec![]);
	}
}
