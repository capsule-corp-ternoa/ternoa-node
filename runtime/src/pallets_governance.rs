use crate::{constants::time::DAYS, Call, Event, Origin, Runtime, TechnicalCommittee};
use frame_support::parameter_types;
use sp_core::u32_trait::{_1, _2};
use ternoa_primitives::{AccountId, BlockNumber};

// Shared parameters with all collectives / committees
parameter_types! {
    pub const MotionDuration: BlockNumber = 2 * DAYS;
    pub const MaxProposals: u32 = 100;
    pub const MaxMembers: u32 = 50;
}

// --- Technical committee
pub type TechnicalCollective = pallet_collective::DefaultInstance;
pub type MoreThanHalfOfTheTechnicalCollective =
    pallet_collective::EnsureProportionMoreThan<_1, _2, AccountId, TechnicalCollective>;

impl pallet_collective::Config<TechnicalCollective> for Runtime {
    type Origin = Origin;
    type Proposal = Call;
    type Event = Event;
    type MotionDuration = MotionDuration;
    type MaxProposals = MaxProposals;
    type WeightInfo = ();
    type MaxMembers = MaxMembers;
    type DefaultVote = pallet_collective::PrimeDefaultVote;
}

impl pallet_membership::Config for Runtime {
    type Event = Event;
    type AddOrigin = MoreThanHalfOfTheTechnicalCollective;
    type RemoveOrigin = MoreThanHalfOfTheTechnicalCollective;
    type SwapOrigin = MoreThanHalfOfTheTechnicalCollective;
    type ResetOrigin = MoreThanHalfOfTheTechnicalCollective;
    type PrimeOrigin = MoreThanHalfOfTheTechnicalCollective;
    type MembershipInitialized = TechnicalCommittee;
    type MembershipChanged = TechnicalCommittee;
    type MaxMembers = MaxMembers;
    type WeightInfo = ();
}

impl pallet_mandate::Config for Runtime {
    type Event = Event;
    type Call = Call;
    type ExternalOrigin = MoreThanHalfOfTheTechnicalCollective;
}
