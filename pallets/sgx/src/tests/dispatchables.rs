use super::mock;
use super::mock::*;
use crate::{
	Cluster, ClusterId, ClusterIdGenerator, ClusterIndex, ClusterRegistry, Enclave, EnclaveId,
	EnclaveIdGenerator, EnclaveIndex, EnclaveRegistry, Error,
};
use frame_support::{assert_noop, assert_ok};
use frame_system::RawOrigin;
use pallet_balances::Error as BalanceError;
use sp_runtime::traits::BadOrigin;
use ternoa_primitives::TextFormat;

#[test]
fn register_enclave() {
	ExtBuilder::default()
		.tokens(vec![(ALICE, 100), (BOB, 0), (DAVE, 10)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let dave: mock::Origin = RawOrigin::Signed(DAVE).into();

			assert_eq!(EnclaveIndex::<Test>::iter().count(), 0);
			assert_eq!(EnclaveRegistry::<Test>::iter().count(), 0);
			assert_eq!(EnclaveIdGenerator::<Test>::get(), 0);
			let uri: TextFormat = vec![1];

			// Alice should be able to create an enclave if she has enough tokens.
			assert_ok!(Sgx::register_enclave(alice.clone(), uri.clone()));
			assert_eq!(Balances::free_balance(ALICE), 95);

			let enclave = Enclave::new(uri.clone());
			let enclave_id: EnclaveId = 0;
			assert!(EnclaveRegistry::<Test>::contains_key(enclave_id));
			assert_eq!(EnclaveRegistry::<Test>::get(enclave_id), Some(enclave));
			assert!(EnclaveIndex::<Test>::contains_key(ALICE));
			assert_eq!(EnclaveIndex::<Test>::get(ALICE).unwrap(), enclave_id);
			assert_eq!(EnclaveIdGenerator::<Test>::get(), 1);

			// Alice should NOT be able to create an enclave if she already has one.
			let ok = Sgx::register_enclave(alice, vec![1]);
			assert_noop!(ok, Error::<Test>::PublicKeyAlreadyTiedToACluster);

			// Bob should NOT be able to create an enclave if the doesn't have enough tokens.
			let ok = Sgx::register_enclave(bob, vec![1]);
			assert_noop!(ok, BalanceError::<Test>::InsufficientBalance);

			// Dave should NOT be able to create an enclave if the uri is too short.
			let ok = Sgx::register_enclave(dave.clone(), vec![]);
			assert_noop!(ok, Error::<Test>::UriTooShort);

			// Dave should NOT be able to create an enclave if the uri is too long.
			let ok = Sgx::register_enclave(dave, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
			assert_noop!(ok, Error::<Test>::UriTooLong);
		})
}

#[test]
fn assign_enclave() {
	ExtBuilder::default()
		.tokens(vec![(ALICE, 10), (BOB, 10), (DAVE, 10)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let dave: mock::Origin = RawOrigin::Signed(DAVE).into();

			let cluster_id: ClusterId = 0;
			let enclave_id: EnclaveId = 0;
			assert_ok!(Sgx::create_cluster(RawOrigin::Root.into()));
			assert_ok!(Sgx::register_enclave(alice.clone(), vec![1]));

			// Alice should be able to assign her enclave to a cluster.
			assert_ok!(Sgx::assign_enclave(alice.clone(), cluster_id));
			let cluster = ClusterRegistry::<Test>::get(cluster_id).unwrap();
			assert_eq!(cluster.enclaves, vec![enclave_id]);
			assert_eq!(ClusterIndex::<Test>::get(enclave_id), Some(cluster_id));

			// Alice should NOT be able to assign her enclave if it is already assigned.
			let ok = Sgx::assign_enclave(alice, cluster_id);
			assert_noop!(ok, Error::<Test>::EnclaveAlreadyAssigned);

			// Bob should NOT be able to assign his enclave to an non existing cluster.
			assert_ok!(Sgx::register_enclave(bob.clone(), vec![1]));
			let ok = Sgx::assign_enclave(bob.clone(), 1);
			assert_noop!(ok, Error::<Test>::UnknownClusterId);

			// Dave should NOT be able to register his enclave if the cluster is already full.
			assert_ok!(Sgx::assign_enclave(bob, cluster_id));
			assert_ok!(Sgx::register_enclave(dave.clone(), vec![1]));
			let ok = Sgx::assign_enclave(dave, 0);
			assert_noop!(ok, Error::<Test>::ClusterIsAlreadyFull);
		})
}

#[test]
fn unassign_enclave() {
	ExtBuilder::default()
		.tokens(vec![(ALICE, 10), (BOB, 10)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();

			let cluster_id: ClusterId = 0;
			let enclave_id: EnclaveId = 0;
			assert_ok!(Sgx::create_cluster(RawOrigin::Root.into()));
			assert_ok!(Sgx::register_enclave(alice.clone(), vec![1]));
			assert_ok!(Sgx::assign_enclave(alice.clone(), cluster_id));
			let cluster = ClusterRegistry::<Test>::get(cluster_id).unwrap();
			assert_eq!(cluster.enclaves, vec![enclave_id]);
			assert_eq!(ClusterIndex::<Test>::get(enclave_id), Some(cluster_id));

			// Alice should be able to unassign her enclave from a cluster.
			assert_ok!(Sgx::unassign_enclave(alice.clone()));
			let cluster = ClusterRegistry::<Test>::get(cluster_id).unwrap();
			let empty: Vec<EnclaveId> = Default::default();
			assert_eq!(cluster.enclaves, empty);
			assert_eq!(ClusterIndex::<Test>::get(enclave_id), None);

			// Alice should NOT be able to unassign her enclave if the enclave is already unassigned.
			let ok = Sgx::unassign_enclave(alice.clone());
			assert_noop!(ok, Error::<Test>::EnclaveNotAssigned);

			// Bob should NOT be able to unassign his enclave if he does not have one
			let ok = Sgx::unassign_enclave(bob.clone());
			assert_noop!(ok, Error::<Test>::NotEnclaveOwner);
		})
}

#[test]
fn update_enclave() {
	ExtBuilder::default()
		.tokens(vec![(ALICE, 10), (BOB, 10)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();

			assert_ok!(Sgx::register_enclave(alice.clone(), vec![1]));
			let enclave_id: EnclaveId = 0;

			// Alice should be able to update her enclave.
			let uri: TextFormat = vec![0, 1];
			let enclave = Enclave::new(uri.clone());
			assert_ok!(Sgx::update_enclave(alice.clone(), uri.clone()));
			assert_eq!(EnclaveRegistry::<Test>::get(enclave_id), Some(enclave));

			// Dave should NOT be able to update an enclave if the uri is too short.
			let ok = Sgx::update_enclave(alice.clone(), vec![]);
			assert_noop!(ok, Error::<Test>::UriTooShort);

			// Dave should NOT be able to update an enclave if the uri is too long.
			let ok = Sgx::update_enclave(alice.clone(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
			assert_noop!(ok, Error::<Test>::UriTooLong);

			// Bob should NOT be able to update his enclave if he doesn't have one.
			let ok = Sgx::update_enclave(bob.clone(), uri.clone());
			assert_noop!(ok, Error::<Test>::NotEnclaveOwner);
		})
}

#[test]
fn change_enclave_owner() {
	ExtBuilder::default()
		.tokens(vec![(ALICE, 10), (BOB, 10)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

			assert_ok!(Sgx::register_enclave(alice.clone(), vec![1]));
			let enclave_id: EnclaveId = 0;

			// Alice should be able to change owner of his enclave.
			assert_ok!(Sgx::change_enclave_owner(alice.clone(), BOB));
			assert_eq!(EnclaveIndex::<Test>::get(BOB), Some(enclave_id));

			// Alice should NOT be able to change the owner if she doesn't own an enclave.
			let ok = Sgx::change_enclave_owner(alice.clone(), BOB);
			assert_noop!(ok, Error::<Test>::NotEnclaveOwner);

			// Alice should NOT be able to change the owner if the new owner already has an enclave.
			assert_ok!(Sgx::register_enclave(alice.clone(), vec![1]));
			let ok = Sgx::change_enclave_owner(alice.clone(), BOB);
			assert_noop!(ok, Error::<Test>::PublicKeyAlreadyTiedToACluster);
		})
}

#[test]
fn create_cluster() {
	ExtBuilder::default().build().execute_with(|| {
		let alice: mock::Origin = RawOrigin::Signed(ALICE).into();

		assert_eq!(ClusterIndex::<Test>::iter().count(), 0);
		assert_eq!(ClusterRegistry::<Test>::iter().count(), 0);
		assert_eq!(ClusterIdGenerator::<Test>::get(), 0);
		assert_eq!(EnclaveIdGenerator::<Test>::get(), 0);
		let cluster_id: ClusterId = 0;
		let cluster = Cluster::new(Default::default());

		// Sudo should be able to create clusters.
		assert_ok!(Sgx::create_cluster(RawOrigin::Root.into()));
		assert_eq!(ClusterIndex::<Test>::iter().count(), 0);
		assert_eq!(ClusterRegistry::<Test>::get(cluster_id), Some(cluster));
		assert_eq!(ClusterIdGenerator::<Test>::get(), 1);
		assert_eq!(EnclaveIdGenerator::<Test>::get(), 0);

		// Alice should NOT be able to create a cluster.
		let ok = Sgx::create_cluster(alice.clone());
		assert_noop!(ok, BadOrigin);
	})
}

#[test]
fn remove_cluster() {
	ExtBuilder::default()
		.tokens(vec![(ALICE, 10), (BOB, 10)])
		.build()
		.execute_with(|| {
			let alice: mock::Origin = RawOrigin::Signed(ALICE).into();
			let bob: mock::Origin = RawOrigin::Signed(BOB).into();
			let uri: TextFormat = vec![1];
			let cluster_id: ClusterId = 0;
			let cluster = Cluster::new(vec![0, 1]);

			assert_ok!(Sgx::create_cluster(RawOrigin::Root.into()));
			assert_ok!(Sgx::register_enclave(alice.clone(), uri.clone()));
			assert_ok!(Sgx::register_enclave(bob.clone(), uri.clone()));
			assert_ok!(Sgx::assign_enclave(alice.clone(), cluster_id));
			assert_ok!(Sgx::assign_enclave(bob.clone(), cluster_id));

			assert_eq!(ClusterIndex::<Test>::iter().count(), 2);
			assert_eq!(ClusterIndex::<Test>::get(0), Some(0));
			assert_eq!(ClusterIndex::<Test>::get(1), Some(0));
			assert_eq!(ClusterRegistry::<Test>::iter().count(), 1);
			assert_eq!(ClusterRegistry::<Test>::get(0), Some(cluster));
			assert_eq!(ClusterIdGenerator::<Test>::get(), 1);

			// Sudo should be remove an existing cluster
			assert_ok!(Sgx::remove_cluster(RawOrigin::Root.into(), cluster_id));
			assert_eq!(ClusterIndex::<Test>::iter().count(), 0);
			assert_eq!(ClusterRegistry::<Test>::iter().count(), 0);

			// Sudo should NOT be able to remove an non-existing cluster
			let ok = Sgx::remove_cluster(RawOrigin::Root.into(), 10);
			assert_noop!(ok, Error::<Test>::UnknownClusterId);

			// Alice should NOT be able to remove a cluster.
			let ok = Sgx::remove_cluster(alice.clone(), 1);
			assert_noop!(ok, BadOrigin);
		})
}
