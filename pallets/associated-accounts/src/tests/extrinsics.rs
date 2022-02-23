use super::mock::*;
use crate::tests::mock;
use crate::{Account, Error, Event as AccountEvent, SupportedAccount, SupportedAccounts, Users};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;

fn origin(account: u64) -> mock::Origin {
	RawOrigin::Signed(account).into()
}

fn root() -> mock::Origin {
	RawOrigin::Root.into()
}

mod set_account {
	use super::*;

	#[test]
	fn set_account() {
		ExtBuilder::new_build().execute_with(|| {
			let service_name: Vec<u8> = SERVICE_NAME.into();
			let acc = Account::new(service_name.clone(), "Marko".into());

			let ok = AAccounts::set_account(origin(ALICE), acc.key.clone(), acc.value.clone());
			assert_ok!(ok);

			// Storage
			assert_eq!(Users::<Test>::get(ALICE), Some(vec![acc.clone()]));

			// Events
			let event = AccountEvent::UserAccountAdded {
				user: ALICE,
				account_key: acc.key,
				account_value: acc.value,
			};
			let event = Event::AAccounts(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn updating_existing_account() {
		ExtBuilder::new_build().execute_with(|| {
			let service_name: Vec<u8> = SERVICE_NAME.into();
			let acc = Account::new(service_name.clone(), vec![10]);

			let ok = AAccounts::set_account(origin(ALICE), acc.key.clone(), acc.value.clone());
			assert_ok!(ok);

			let mut new_acc = acc.clone();
			new_acc.value = vec![20];

			assert_ne!(acc.value, new_acc.value);
			let ok =
				AAccounts::set_account(origin(ALICE), new_acc.key.clone(), new_acc.value.clone());
			assert_ok!(ok);

			// Storage
			assert_eq!(Users::<Test>::get(ALICE), Some(vec![new_acc.clone()]));

			// Events
			let event = AccountEvent::UserAccountAdded {
				user: ALICE,
				account_key: new_acc.key,
				account_value: new_acc.value,
			};
			let event = Event::AAccounts(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}

	#[test]
	fn account_is_not_supported() {
		ExtBuilder::new_build().execute_with(|| {
			let service_name: Vec<u8> = INVALID_SERVICE_NAME.into();

			let ok = AAccounts::set_account(origin(ALICE), service_name, vec![20]);
			assert_noop!(ok, Error::<Test>::UnknownAccountKey);
		})
	}

	#[test]
	fn value_is_too_short() {
		ExtBuilder::new_build().execute_with(|| {
			let supports = SupportedAccounts::<Test>::get();
			let supp = supports[0].clone();

			let value: Vec<u8> = vec![];
			assert!(value.len() < supp.min_length as usize);

			let ok = AAccounts::set_account(origin(ALICE), supp.key, value);
			assert_noop!(ok, Error::<Test>::ValueIsTooShort);
		})
	}

	#[test]
	fn value_is_too_long() {
		ExtBuilder::new_build().execute_with(|| {
			let supports = SupportedAccounts::<Test>::get();
			let supp = supports[0].clone();

			let value: Vec<u8> = "Lorem ipsum dolor sit amet".into();
			assert!(value.len() > supp.max_length as usize);

			let ok = AAccounts::set_account(origin(ALICE), supp.key, value);
			assert_noop!(ok, Error::<Test>::ValueIsTooLong);
		})
	}
}

mod add_new_supported_account {
	use super::*;

	#[test]
	fn add_new_supported_account() {
		ExtBuilder::new_build().execute_with(|| {
			let mut supports = SupportedAccounts::<Test>::get();
			let supp = supports.last().unwrap().clone();

			let ok = AAccounts::remove_supported_account(root(), supp.key.clone());
			assert_ok!(ok);

			// Storage
			supports.pop();

			assert_eq!(SupportedAccounts::<Test>::get(), supports);

			// Events
			let event = AccountEvent::SupportedAccountRemoved { key: supp.key };
			let event = Event::AAccounts(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}
}

mod remove_supported_account {
	use super::*;

	#[test]
	fn add_new_supported_account() {
		ExtBuilder::new_build().execute_with(|| {
			let supp = SupportedAccount::new(vec![65], 1, 10, true);

			let ok = AAccounts::add_new_supported_account(
				root(),
				supp.key.clone(),
				supp.min_length,
				supp.max_length,
				supp.initial_set_fee,
			);
			assert_ok!(ok);

			// Storage
			let supports = SupportedAccounts::<Test>::get();
			assert_eq!(supports.last().unwrap().clone(), supp);

			// Events
			let event = AccountEvent::SupportedAccountAdded {
				key: supp.key,
				min_length: supp.min_length,
				max_length: supp.max_length,
				initial_set_fee: supp.initial_set_fee,
			};
			let event = Event::AAccounts(event);
			assert_eq!(System::events().last().unwrap().event, event);
		})
	}
}
