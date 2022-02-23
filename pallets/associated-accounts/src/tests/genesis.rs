use super::mock::*;
use crate::types::{Account, SupportedAccount};
use crate::GenesisConfig;
use frame_support::traits::GenesisBuild;

#[test]
fn genesis() {
	let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

	let users = vec![
		(ALICE, vec![Account::new(vec![10], vec![10]), Account::new(vec![20], vec![10])]),
		(BOB, vec![Account::new(vec![10], vec![30])]),
	];

	let supported_accounts = vec![
		SupportedAccount::new(vec![10], 1, 10, true),
		SupportedAccount::new(vec![20], 1, 10, true),
	];

	GenesisConfig::<Test> { users: users.clone(), supported_accounts: supported_accounts.clone() }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		for user in users {
			assert_eq!(AAccounts::users(user.0), Some(user.1));
		}

		assert_eq!(AAccounts::supported_accounts(), supported_accounts);
	});
}
