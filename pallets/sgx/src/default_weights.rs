use frame_support::weights::{constants::RocksDbWeight as DbWeight, Weight};

pub trait WeightInfo {
    fn register_enclave() -> Weight;
    fn assign_enclave() -> Weight;
    fn unassign_enclave() -> Weight;
    fn update_enclave() -> Weight;
    fn change_enclave_owner() -> Weight;
    fn create_cluster() -> Weight;
    fn remove_cluster() -> Weight;
}

impl WeightInfo for () {
    // Storage: Sgx EnclaveIndex (r:1 w:1)
    // Storage: Sgx EnclaveIdGenerator (r:1 w:1)
    // Storage: Sgx EnclaveRegistry (r:0 w:1)
    fn register_enclave() -> Weight {
        (67_000_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(3 as Weight))
    }
    // Storage: Sgx EnclaveIndex (r:1 w:0)
    // Storage: Sgx ClusterIndex (r:1 w:1)
    // Storage: Sgx ClusterRegistry (r:1 w:1)
    fn assign_enclave() -> Weight {
        (34_450_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Sgx EnclaveIndex (r:1 w:0)
    // Storage: Sgx ClusterIndex (r:1 w:1)
    // Storage: Sgx ClusterRegistry (r:1 w:1)
    fn unassign_enclave() -> Weight {
        (35_380_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Sgx EnclaveIndex (r:1 w:0)
    // Storage: Sgx EnclaveRegistry (r:1 w:1)
    fn update_enclave() -> Weight {
        (28_410_000 as Weight)
            .saturating_add(DbWeight::get().reads(2 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
    // Storage: Sgx EnclaveIndex (r:2 w:2)
    // Storage: Sgx EnclaveRegistry (r:1 w:0)
    fn change_enclave_owner() -> Weight {
        (36_440_000 as Weight)
            .saturating_add(DbWeight::get().reads(3 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Sgx EnclaveIdGenerator (r:1 w:0)
    // Storage: Sgx ClusterIdGenerator (r:0 w:1)
    // Storage: Sgx ClusterRegistry (r:0 w:1)
    fn create_cluster() -> Weight {
        (21_550_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(2 as Weight))
    }
    // Storage: Sgx ClusterRegistry (r:1 w:1)
    fn remove_cluster() -> Weight {
        (24_320_000 as Weight)
            .saturating_add(DbWeight::get().reads(1 as Weight))
            .saturating_add(DbWeight::get().writes(1 as Weight))
    }
}
