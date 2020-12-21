//! Pallets related to consensus, block production and authorship, session keys
//! and staking.

use crate::{
    constants::time::{EPOCH_DURATION_IN_BLOCKS, EPOCH_DURATION_IN_SLOTS, MILLISECS_PER_BLOCK},
    pallets_core::{BlockHashCount, MaximumBlockWeight},
    AuthorityDiscovery, Babe, Balances, Call, Event, Grandpa, Historical, ImOnline, Offences,
    Runtime, Session, Signature, SignedPayload, Staking, System, Timestamp, UncheckedExtrinsic,
};
use codec::Encode;
use frame_support::{debug, parameter_types, traits::KeyOwnerProofSystem, weights::Weight};
use frame_system::EnsureRoot;
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
#[cfg(any(feature = "std", test))]
pub use pallet_staking::StakerStatus;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
    curve::PiecewiseLinear,
    generic::Era,
    impl_opaque_keys,
    traits::{self as sp_runtime_traits, Convert, OpaqueKeys, StaticLookup},
    Perbill, SaturatedConversion,
};
use sp_std::prelude::*; // `impl_opaque_keys` need `Vec`
use sp_transaction_pool::TransactionPriority;
use ternoa_primitives::{AccountId, Balance, BlockNumber, Index, Moment};

parameter_types! {
    pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
    pub const ExpectedBlockTime: Moment = MILLISECS_PER_BLOCK;
}

impl pallet_babe::Trait for Runtime {
    type EpochDuration = EpochDuration;
    type ExpectedBlockTime = ExpectedBlockTime;
    type EpochChangeTrigger = pallet_babe::ExternalTrigger;

    type KeyOwnerProofSystem = Historical;

    type KeyOwnerProof = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        pallet_babe::AuthorityId,
    )>>::IdentificationTuple;

    type HandleEquivocation =
        pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences>;

    type WeightInfo = ();
}

impl pallet_grandpa::Trait for Runtime {
    type Event = Event;
    type Call = Call;

    type KeyOwnerProofSystem = Historical;

    type KeyOwnerProof =
        <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;

    type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
        KeyTypeId,
        GrandpaId,
    )>>::IdentificationTuple;

    type HandleEquivocation =
        pallet_grandpa::EquivocationHandler<Self::KeyOwnerIdentification, Offences>;

    type WeightInfo = ();
}

impl_opaque_keys! {
    pub struct SessionKeys {
        pub grandpa: Grandpa,
        pub babe: Babe,
        pub im_online: ImOnline,
        pub authority_discovery: AuthorityDiscovery,
    }
}

parameter_types! {
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

impl pallet_session::Trait for Runtime {
    type Event = Event;
    type ValidatorId = <Self as frame_system::Trait>::AccountId;
    type ValidatorIdOf = pallet_staking::StashOf<Self>;
    type ShouldEndSession = Babe;
    type NextSessionRotation = Babe;
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
    type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
    type Keys = SessionKeys;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type WeightInfo = ();
}

impl pallet_session::historical::Trait for Runtime {
    type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

parameter_types! {
    pub const UncleGenerations: BlockNumber = 5;
}

impl pallet_authorship::Trait for Runtime {
    type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = (Staking, ImOnline);
}

impl pallet_authority_discovery::Trait for Runtime {}

parameter_types! {
    pub const SessionDuration: BlockNumber = EPOCH_DURATION_IN_SLOTS as _;
    pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();
    /// We prioritize im-online heartbeats over election solution submission.
    pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
    Call: From<LocalCall>,
{
    fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
        call: Call,
        public: <Signature as sp_runtime_traits::Verify>::Signer,
        account: AccountId,
        nonce: Index,
    ) -> Option<(
        Call,
        <UncheckedExtrinsic as sp_runtime_traits::Extrinsic>::SignaturePayload,
    )> {
        let tip = 0;
        // take the biggest period possible.
        let period = BlockHashCount::get()
            .checked_next_power_of_two()
            .map(|c| c / 2)
            .unwrap_or(2) as u64;
        let current_block = System::block_number()
            .saturated_into::<u64>()
            // The `System::block_number` is initialized with `n+1`,
            // so the actual block number is `n`.
            .saturating_sub(1);
        let era = Era::mortal(period, current_block);
        let extra = (
            frame_system::CheckSpecVersion::<Runtime>::new(),
            frame_system::CheckTxVersion::<Runtime>::new(),
            frame_system::CheckGenesis::<Runtime>::new(),
            frame_system::CheckEra::<Runtime>::from(era),
            frame_system::CheckNonce::<Runtime>::from(nonce),
            frame_system::CheckWeight::<Runtime>::new(),
            pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
        );
        let raw_payload = SignedPayload::new(call, extra)
            .map_err(|e| {
                debug::warn!("Unable to create signed payload: {:?}", e);
            })
            .ok()?;
        let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
        let address = <Runtime as frame_system::Trait>::Lookup::unlookup(account);
        let (call, extra, _) = raw_payload.deconstruct();
        Some((call, (address, signature.into(), extra)))
    }
}

impl frame_system::offchain::SigningTypes for Runtime {
    type Public = <Signature as sp_runtime_traits::Verify>::Signer;
    type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
    Call: From<C>,
{
    type Extrinsic = UncheckedExtrinsic;
    type OverarchingCall = Call;
}

impl pallet_im_online::Trait for Runtime {
    type AuthorityId = ImOnlineId;
    type Event = Event;
    type SessionDuration = SessionDuration;
    type ReportUnresponsiveness = Offences;
    type UnsignedPriority = ImOnlineUnsignedPriority;
    type WeightInfo = ();
}

parameter_types! {
    pub OffencesWeightSoftLimit: Weight = Perbill::from_percent(60) * MaximumBlockWeight::get();
}

impl pallet_offences::Trait for Runtime {
    type Event = Event;
    type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
    type OnOffenceHandler = Staking;
    type WeightSoftLimit = OffencesWeightSoftLimit;
}

pallet_staking_reward_curve::build! {
    const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_025_000,
        max_inflation: 0_100_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 40,
        test_precision: 0_005_000,
    );
}

/// Struct that handles the conversion of Balance -> `u64`. This is used for staking's election
/// calculation.
pub struct CurrencyToVoteHandler;

impl CurrencyToVoteHandler {
    fn factor() -> Balance {
        (Balances::total_issuance() / u64::max_value() as Balance).max(1)
    }
}

impl Convert<Balance, u64> for CurrencyToVoteHandler {
    fn convert(x: Balance) -> u64 {
        (x / Self::factor()) as u64
    }
}

impl Convert<u128, Balance> for CurrencyToVoteHandler {
    fn convert(x: u128) -> Balance {
        x * Self::factor()
    }
}

parameter_types! {
    pub const SessionsPerEra: sp_staking::SessionIndex = 6;
    pub const BondingDuration: pallet_staking::EraIndex = 24 * 28;
    pub const SlashDeferDuration: pallet_staking::EraIndex = 24 * 7; // 1/4 the bonding duration.
    pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
    pub const MaxNominatorRewardedPerValidator: u32 = 64;
    pub const ElectionLookahead: BlockNumber = EPOCH_DURATION_IN_BLOCKS / 4;
    pub const MaxIterations: u32 = 10;
    // 0.05%. The higher the value, the more strict solution acceptance becomes.
    pub MinSolutionScoreBump: Perbill = Perbill::from_rational_approximation(5u32, 10_000);
}

impl pallet_staking::Trait for Runtime {
    type Currency = Balances;
    type UnixTime = Timestamp;
    type CurrencyToVote = CurrencyToVoteHandler;
    type Event = Event;
    // Slashed funds and remainders are burned.
    type RewardRemainder = ();
    type Slash = ();
    type Reward = (); // rewards are minted from the void
    type SessionsPerEra = SessionsPerEra;
    type BondingDuration = BondingDuration;
    type SlashDeferDuration = SlashDeferDuration;
    type SlashCancelOrigin = EnsureRoot<AccountId>;
    type SessionInterface = Self;
    type RewardCurve = RewardCurve;
    type NextNewSession = Session;
    type ElectionLookahead = ElectionLookahead;
    type Call = Call;
    type MaxIterations = MaxIterations;
    type MinSolutionScoreBump = MinSolutionScoreBump;
    type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
    type UnsignedPriority = StakingUnsignedPriority;
    type WeightInfo = ();
}
