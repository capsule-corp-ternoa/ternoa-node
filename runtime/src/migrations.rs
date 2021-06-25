use crate::pallets_core::RuntimeBlockWeights;
use frame_support::{storage::migration::move_pallet, traits::OnRuntimeUpgrade, weights::Weight};

pub(crate) const LOG_TARGET: &'static str = "migration";

pub struct Instances;
impl OnRuntimeUpgrade for Instances {
    fn on_runtime_upgrade() -> Weight {
        sp_tracing::debug!(target: LOG_TARGET, "🕊️ Starting instances migration...");

        move_pallet(
            b"TechnicalCommitteeInstance0",
            b"TechnicalCommitteeDefaultInstance",
        );
        move_pallet(
            b"TechnicalMembershipInstance0",
            b"TechnicalMembershipDefaultInstance",
        );

        sp_tracing::debug!(target: LOG_TARGET, "🕊️ Instances migration done");

        // Keep things simple, take a full block to execute when the migration is deployed
        RuntimeBlockWeights::get().max_block
    }
}
