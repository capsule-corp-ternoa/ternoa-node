use crate::{
    pallets_core::RuntimeBlockWeights, pallets_governance::TechnicalCollectiveMembers, Runtime,
};
use frame_support::{
    debug,
    storage::unhashed,
    traits::{InitializeMembers, OnRuntimeUpgrade},
    weights::Weight,
    StorageHasher, Twox128,
};
use sp_std::vec::Vec;
use ternoa_primitives::AccountId;

pub struct SudoToTechComm;
impl OnRuntimeUpgrade for SudoToTechComm {
    fn on_runtime_upgrade() -> Weight {
        debug::RuntimeLogger::init();
        debug::print!("🕊️ Starting sudo migration...");

        let mut sudo_key_storage_key = Vec::new();
        sudo_key_storage_key.extend_from_slice(&Twox128::hash(b"Sudo"));
        sudo_key_storage_key.extend_from_slice(&Twox128::hash(b"Key"));

        let mut members_storage_key = Vec::new();
        members_storage_key.extend_from_slice(&Twox128::hash(b"Instance0Membership"));
        members_storage_key.extend_from_slice(&Twox128::hash(b"Members"));

        if let Some(sudo_key) = unhashed::get::<AccountId>(&sudo_key_storage_key) {
            // Configure the tech membership with the old sudo key
            let mut members = Vec::new();
            members.push(sudo_key.clone());
            unhashed::put(&members_storage_key, &members);

            // Let the tech committee pallet know about the membership changes
            <<Runtime as pallet_membership::Config<TechnicalCollectiveMembers>>::MembershipInitialized as InitializeMembers<AccountId>>::initialize_members(&members);
        }

        // Clean sudo storage
        unhashed::kill(&sudo_key_storage_key);

        debug::print!("🕊️ Sudo migration done");

        // Keep things simple, take a full block to execute when the migration is deployed
        RuntimeBlockWeights::get().max_block
    }
}
