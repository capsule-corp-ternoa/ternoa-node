// Copyright 2022 Capsule Corp (France) SAS.
// This file is part of Ternoa.

// Ternoa is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Ternoa is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Ternoa.  If not, see <http://www.gnu.org/licenses/>.

use super::weights;
use common::{
	election_provider_multi_phase::BetterUnsignedThreshold,
	staking::{BondingDuration, SessionsPerEra},
	transaction_payment::TransactionByteFee,
	BlockHashCount, BlockLength,
};
use frame_election_provider_support::{SequentialPhragmen, Weight};
use frame_support::{
	parameter_types,
	traits::{ConstU32, EitherOfDiverse, KeyOwnerProofSystem, U128CurrencyToVote},
	weights::{constants::RocksDbWeight, ConstantMultiplier, IdentityFee},
};
use frame_system::EnsureRoot;
use pallet_grandpa::AuthorityId as GrandpaId;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use pallet_transaction_payment::CurrencyAdapter;
use parity_scale_codec::Encode;
use sp_core::crypto::KeyTypeId;
use sp_runtime::{
	generic::{self, Era},
	impl_opaque_keys,
	traits::{AccountIdLookup, BlakeTwo256, OpaqueKeys, StaticLookup},
	Perbill, SaturatedConversion,
};
use sp_std::vec::Vec;
use sp_version::RuntimeVersion;
use static_assertions::const_assert;
use ternoa_core_primitives::{AccountId, Balance, BlockNumber, Hash, Index, Moment};
use ternoa_runtime_common as common;

use crate::{
	constants::time::EPOCH_DURATION_IN_SLOTS, AuthorityDiscovery, Babe, BagsList, Balances,
	BlockWeights, Call, Council, ElectionProviderMultiPhase, Event, Grandpa, Historical, ImOnline,
	OffchainSolutionLengthLimit, OffchainSolutionWeightLimit, Offences, Origin, OriginCaller,
	PalletInfo, Preimage, Runtime, Scheduler, Session, Signature, SignedPayload, Staking,
	StakingRewards, System, TechnicalCommittee, Timestamp, TransactionPayment, Treasury,
	UncheckedExtrinsic, NFT, VERSION,
};

pub use common::babe::BABE_GENESIS_EPOCH_CONFIG;

#[cfg(any(feature = "std", test))]
pub use pallet_staking::StakerStatus;

type RootOrAtLeastHalfOfCommittee = EitherOfDiverse<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>,
>;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = BlockWeights;
	type BlockLength = BlockLength;
	type DbWeight = RocksDbWeight;
	type Origin = Origin;
	type Call = Call;
	type Index = Index;
	type BlockNumber = BlockNumber;
	type Hash = Hash;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId;
	type Lookup = AccountIdLookup<AccountId, ()>;
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	type SS58Prefix = common::SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

// Utility
impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>; // TODO Weights
}

impl pallet_transaction_payment::Config for Runtime {
	type Event = Event;
	type OnChargeTransaction = CurrencyAdapter<Balances, StakingRewards>;
	type OperationalFeeMultiplier = common::transaction_payment::OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = common::SlowAdjustingFeeUpdate<Self>;
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = common::balances::MaxLocks;
	type Balance = Balance;
	type MaxReserves = common::balances::MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = common::balances::ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type MinimumPeriod = common::timestamp::TimestampMinimumPeriod;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = common::treasury::PalletId;
	type Currency = Balances;
	type ApproveOrigin = RootOrAtLeastHalfOfCommittee;
	type RejectOrigin = RootOrAtLeastHalfOfCommittee;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = common::treasury::ProposalBond;
	type ProposalBondMinimum = common::treasury::ProposalBondMinimum;
	type ProposalBondMaximum = common::treasury::ProposalBondMaximum;
	type SpendPeriod = common::treasury::SpendPeriod;
	type Burn = common::treasury::Burn;
	type BurnDestination = ();
	type MaxApprovals = common::treasury::MaxApprovals;
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
	type SpendFunds = ();
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
}

parameter_types! {
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS as u64;
	pub const ReportLongevity: u64 =
		BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}

// Babe
impl pallet_babe::Config for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = common::babe::ExpectedBlockTime;
	// session module is the trigger
	type EpochChangeTrigger = common::babe::EpochChangeTrigger;
	type DisabledValidators = Session;
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
		pallet_babe::EquivocationHandler<Self::KeyOwnerIdentification, Offences, ReportLongevity>;
	type WeightInfo = ();
	type MaxAuthorities = common::shared::MaxAuthorities;
}

// Grandpa
impl pallet_grandpa::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type KeyOwnerProofSystem = Historical;
	type KeyOwnerProof =
		<Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(KeyTypeId, GrandpaId)>>::Proof;
	type KeyOwnerIdentification = <Self::KeyOwnerProofSystem as KeyOwnerProofSystem<(
		KeyTypeId,
		GrandpaId,
	)>>::IdentificationTuple;
	type HandleEquivocation = pallet_grandpa::EquivocationHandler<
		Self::KeyOwnerIdentification,
		Offences,
		ReportLongevity,
	>;
	type WeightInfo = ();
	type MaxAuthorities = common::shared::MaxAuthorities;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub grandpa: Grandpa,
		pub babe: Babe,
		pub im_online: ImOnline,
		pub authority_discovery: AuthorityDiscovery,
	}
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	type ShouldEndSession = Babe;
	type NextSessionRotation = Babe;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
}

impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = common::authorship::UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (Staking, ImOnline);
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = common::shared::MaxAuthorities;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Runtime
where
	Call: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: Call,
		public: <Signature as sp_runtime::traits::Verify>::Signer,
		account: AccountId,
		nonce: Index,
	) -> Option<(Call, <UncheckedExtrinsic as sp_runtime::traits::Extrinsic>::SignaturePayload)> {
		let tip = 0;
		// take the biggest period possible.
		let period =
			BlockHashCount::get().checked_next_power_of_two().map(|c| c / 2).unwrap_or(2) as u64;
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
				sp_tracing::warn!("Unable to create signed payload: {:?}", e);
			})
			.ok()?;
		let signature = raw_payload.using_encoded(|payload| C::sign(payload, public))?;
		let address = <Runtime as frame_system::Config>::Lookup::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature.into(), extra)))
	}
}

impl frame_system::offchain::SigningTypes for Runtime {
	type Public = <Signature as sp_runtime::traits::Verify>::Signer;
	type Signature = Signature;
}

impl<C> frame_system::offchain::SendTransactionTypes<C> for Runtime
where
	Call: From<C>,
{
	type Extrinsic = UncheckedExtrinsic;
	type OverarchingCall = Call;
}

impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type Event = Event;
	type NextSessionRotation = Babe;
	type ValidatorSet = Historical;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = common::imonline::ImOnlineUnsignedPriority;
	type WeightInfo = weights::pallet_im_online::WeightInfo<Runtime>;
	type MaxKeys = common::imonline::MaxKeys;
	type MaxPeerInHeartbeats = common::imonline::MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = common::imonline::MaxPeerDataEncodingSize;
}

impl pallet_offences::Config for Runtime {
	type Event = Event;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

pub struct OnChainSeqPhragmen;
impl frame_election_provider_support::onchain::Config for OnChainSeqPhragmen {
	type System = Runtime;
	type Solver = SequentialPhragmen<AccountId, common::election_provider_support::OnChainAccuracy>;
	type DataProvider = Staking;
	type WeightInfo = frame_election_provider_support::weights::SubstrateWeight<Runtime>; // TODO Weights
}

impl pallet_staking::Config for Runtime {
	type MaxNominations = common::staking::MaxNominations;
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type ElectionProvider = ElectionProviderMultiPhase;
	type GenesisElectionProvider =
		frame_election_provider_support::onchain::UnboundedExecution<OnChainSeqPhragmen>;
	type RewardRemainder = Treasury;
	type Event = Event;
	type Slash = Treasury; // send the slashed funds to the treasury.
	type Reward = (); // rewards are minted from the void
	type SessionsPerEra = common::staking::SessionsPerEra;
	type BondingDuration = common::staking::BondingDuration;
	type SlashDeferDuration = common::staking::SlashDeferDuration;
	/// A super-majority of the council can cancel the slash.
	type SlashCancelOrigin = RootOrAtLeastHalfOfCommittee;
	type SessionInterface = Self;
	type EraPayout = StakingRewards;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = common::staking::MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = common::staking::OffendingValidatorsThreshold;
	// Alternatively, use pallet_staking::UseNominatorsMap<Runtime> to just use the nominators map.
	// Note that the aforementioned does not scale to a very large number of nominators.
	type VoterList = BagsList;
	type MaxUnlockingChunks = common::staking::MaxUnlockingChunks;
	type BenchmarkingConfig = common::staking::StakingBenchmarkingConfig;
	type OnStakerSlash = (); // TODO To see NominationPools
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
}

impl pallet_election_provider_multi_phase::MinerConfig for Runtime {
	type AccountId = AccountId;
	type MaxLength = OffchainSolutionLengthLimit;
	type MaxWeight = OffchainSolutionWeightLimit;
	type Solution = common::election_provider_multi_phase::NposCompactSolution24;
	type MaxVotesPerVoter = <
		<Self as pallet_election_provider_multi_phase::Config>::DataProvider
		as
		frame_election_provider_support::ElectionDataProvider
	>::MaxVotesPerVoter;

	// The unsigned submissions have to respect the weight of the submit_unsigned call, thus their
	// weight estimate function is wired to this call's weight.
	fn solution_weight(v: u32, t: u32, a: u32, d: u32) -> Weight {
		<
			<Self as pallet_election_provider_multi_phase::Config>::WeightInfo
			as
			pallet_election_provider_multi_phase::WeightInfo
		>::submit_unsigned(v, t, a, d)
	}
}

parameter_types! {
	// phase durations. 1/4 of the last session for each.
	pub const SignedPhase: u32 = EPOCH_DURATION_IN_SLOTS / 4;
	pub const UnsignedPhase: u32 = EPOCH_DURATION_IN_SLOTS / 4;
	pub OffchainRepeat: BlockNumber = UnsignedPhase::get() / 8;
}

impl pallet_election_provider_multi_phase::Config for Runtime {
	type Event = Event;
	/// What Currency to use to reward or slash miners.
	type Currency = Balances;
	/// Something that can predict the fee of a call. Used to sensibly distribute rewards.
	type EstimateCallFee = TransactionPayment;
	/// Duration of the unsigned phase. After the signed phase the unsigned phase comes where the
	/// OCWs from validators compute the election result (solution). The best score from the
	/// unsigned and signed phase is used.
	type UnsignedPhase = UnsignedPhase;
	type SignedMaxSubmissions = common::election_provider_multi_phase::SignedMaxSubmissions;
	type SignedMaxRefunds = common::election_provider_multi_phase::SignedMaxRefunds;
	type SignedRewardBase = common::election_provider_multi_phase::SignedRewardBase;
	type SignedDepositBase = common::election_provider_multi_phase::SignedDepositBase;
	type SignedDepositByte = common::election_provider_multi_phase::SignedDepositByte;
	type SignedDepositWeight = ();
	type SignedMaxWeight =
		<Self::MinerConfig as pallet_election_provider_multi_phase::MinerConfig>::MaxWeight;
	type MinerConfig = Self;
	type SlashHandler = Treasury; // burn slashes
	type RewardHandler = (); // nothing to do upon rewards
	/// Duration of the signed phase. In the Signed phase miners (or any account) can compute the
	/// (solution) result of the election. If they did it correctly they will be rewarded. If they
	/// wanted to cheat the system they will be slashed. This Signed phase happens before then
	/// Unsigned one.
	type SignedPhase = SignedPhase;
	type BetterUnsignedThreshold = BetterUnsignedThreshold;
	type BetterSignedThreshold = ();
	type OffchainRepeat = OffchainRepeat;
	type MinerTxPriority = common::election_provider_multi_phase::NposSolutionPriority;
	type DataProvider = Staking;
	type Fallback = frame_election_provider_support::onchain::UnboundedExecution<OnChainSeqPhragmen>;
	type GovernanceFallback =
		frame_election_provider_support::onchain::UnboundedExecution<OnChainSeqPhragmen>;
	type Solver = common::election_provider_multi_phase::Solver<Self>;
	type BenchmarkingConfig = common::election_provider_multi_phase::BenchmarkConfig;
	type ForceOrigin = RootOrAtLeastHalfOfCommittee;
	type WeightInfo = weights::pallet_election_provider_multi_phase::WeightInfo<Runtime>;
	type MaxElectingVoters = common::election_provider_multi_phase::MaxElectingVoters;
	type MaxElectableTargets = common::election_provider_multi_phase::MaxElectableTargets;
}

// BagsList
impl pallet_bags_list::Config for Runtime {
	type Event = Event;
	type ScoreProvider = Staking;
	type WeightInfo = weights::pallet_bags_list::WeightInfo<Runtime>;
	type BagThresholds = common::bags_list::BagThresholds;
	type Score = sp_npos_elections::VoteWeight;
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = RootOrAtLeastHalfOfCommittee;
	type MaxSize = common::preimage::PreimageMaxSize;
	type BaseDeposit = common::preimage::PreimageBaseDeposit;
	type ByteDeposit = common::preimage::PreimageByteDeposit;
}

// Technical collective
type TechnicalCollective = pallet_collective::Instance1;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = common::technical_collective::TechnicalMotionDuration;
	type MaxProposals = common::technical_collective::TechnicalMaxProposals;
	type MaxMembers = common::technical_collective::TechnicalMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

// Pallet Membership
impl pallet_membership::Config for Runtime {
	type Event = Event;
	type AddOrigin = RootOrAtLeastHalfOfCommittee;
	type RemoveOrigin = RootOrAtLeastHalfOfCommittee;
	type SwapOrigin = RootOrAtLeastHalfOfCommittee;
	type ResetOrigin = RootOrAtLeastHalfOfCommittee;
	type PrimeOrigin = RootOrAtLeastHalfOfCommittee;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = common::technical_collective::TechnicalMaxMembers;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

// Pallet Membership
impl ternoa_mandate::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type ExternalOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>;
}

// Scheduler
parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
	BlockWeights::get().max_block;
	pub const MaxScheduledPerBlock: u32 = 50;
	pub const NoPreimagePostponement: Option<u32> = Some(10);
}

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = RootOrAtLeastHalfOfCommittee;
	type MaxScheduledPerBlock = MaxScheduledPerBlock;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
	type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = NoPreimagePostponement;
}

// Staking rewards
impl ternoa_staking_rewards::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type PalletId = common::staking_rewards::PalletId;
	type ExternalOrigin = RootOrAtLeastHalfOfCommittee;
	type WeightInfo = weights::ternoa_staking_rewards::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ProposalLifetime: BlockNumber = 1000;
	pub const InitialBridgeFee: Balance = 10_000_000_000_000_000_000;
}

impl ternoa_bridge::Config for Runtime {
	type Event = Event;
	type WeightInfo = weights::ternoa_bridge::WeightInfo<Runtime>;
	type Currency = Balances;
	type FeesCollector = Treasury;
	type ExternalOrigin = RootOrAtLeastHalfOfCommittee;
	type ChainId = common::bridge::ChainId;
	type PalletId = common::bridge::PalletId;
	type ProposalLifetime = ProposalLifetime;
	type RelayerVoteThreshold = common::bridge::RelayerVoteThreshold;
	type RelayerCountLimit = common::bridge::RelayerCountLimit;
	type InitialBridgeFee = InitialBridgeFee;
}

// Council
type CouncilCollective = pallet_collective::Instance2;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = common::council::CouncilMotionDuration;
	type MaxProposals = common::council::CouncilMaxProposals;
	type MaxMembers = common::council::CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}
// Make sure that there are no more than MaxMembers members elected via phragmen.
const_assert!(
	common::phragmen_election::PhragmenDesiredMembers::get() <=
		common::council::CouncilMaxMembers::get()
);

// Elections Phragmen
impl pallet_elections_phragmen::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type ChangeMembers = Council;
	type InitializeMembers = Council;
	type CurrencyToVote = frame_support::traits::U128CurrencyToVote;
	type CandidacyBond = common::phragmen_election::PhragmenCandidacyBond;
	type VotingBondBase = common::phragmen_election::PhragmenVotingBondBase;
	type VotingBondFactor = common::phragmen_election::PhragmenVotingBondFactor;
	type LoserCandidate = Treasury;
	type KickedMember = Treasury;
	type DesiredMembers = common::phragmen_election::PhragmenDesiredMembers;
	type DesiredRunnersUp = common::phragmen_election::PhragmenDesiredRunnersUp;
	type TermDuration = common::phragmen_election::PhragmenTermDuration;
	type PalletId = common::phragmen_election::PhragmenElectionPalletId;
	type WeightInfo = weights::pallet_elections_phragmen::WeightInfo<Runtime>;
	type MaxCandidates = common::phragmen_election::MaxCandidates;
	type MaxVoters = common::phragmen_election::MaxVoters;
}

// Democracy
impl pallet_democracy::Config for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = common::democracy::EnactmentPeriod;
	type VoteLockingPeriod = common::democracy::VoteLockingPeriod;
	type LaunchPeriod = common::democracy::LaunchPeriod;
	type VotingPeriod = common::democracy::VotingPeriod;
	type MinimumDeposit = common::democracy::MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
	/// A majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 2>;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 1, 1>;
	/// Two thirds of the technical committee can have an `ExternalMajority/ExternalDefault` vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>;
	type InstantOrigin =
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>;
	type InstantAllowed = common::democracy::InstantAllowed;
	type FastTrackVotingPeriod = common::democracy::FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 2, 3>,
	>;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// To cancel a proposal before it has been passed, the technical committee must be 1/2 or
	// Root must agree.
	type CancelProposalOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 1, 2>,
	>;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cooloff period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCollective>;
	type CooloffPeriod = common::democracy::CooloffPeriod;
	type PreimageByteDeposit = common::preimage::PreimageByteDeposit;
	type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilCollective>;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = common::democracy::MaxVotes;
	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
	type MaxProposals = common::democracy::MaxProposals;
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = common::multisig::DepositBase;
	type DepositFactor = common::multisig::DepositFactor;
	type MaxSignatories = common::multisig::MaxSignatories;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

impl pallet_identity::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BasicDeposit = common::identity::BasicDeposit;
	type FieldDeposit = common::identity::FieldDeposit;
	type SubAccountDeposit = common::identity::SubAccountDeposit;
	type MaxSubAccounts = common::identity::MaxSubAccounts;
	type MaxAdditionalFields = common::identity::MaxAdditionalFields;
	type MaxRegistrars = common::identity::MaxRegistrars;
	type Slashed = Treasury;
	type ForceOrigin = RootOrAtLeastHalfOfCommittee;
	type RegistrarOrigin = RootOrAtLeastHalfOfCommittee;
	type WeightInfo = weights::pallet_identity::WeightInfo<Runtime>;
}

parameter_types! {
	pub const InitialMintFee: Balance = 10_000_000_000_000_000_000;
	pub const NFTOffchainDataLimit: u32 = 150;
	pub const CollectionOffchainDataLimit: u32 = 150;
	pub const CollectionSizeLimit: u32 = 1_000_000;
}

impl ternoa_nft::Config for Runtime {
	type Event = Event;
	type WeightInfo = weights::ternoa_nft::WeightInfo<Runtime>;
	type Currency = Balances;
	type FeesCollector = Treasury;
	type InitialMintFee = InitialMintFee;
	type NFTOffchainDataLimit = NFTOffchainDataLimit;
	type CollectionOffchainDataLimit = CollectionOffchainDataLimit;
	type CollectionSizeLimit = CollectionSizeLimit;
}

parameter_types! {
	pub const MarketplaceInitialMintFee: Balance = 1_000_000_000_000_000_000_000;
	pub const OffchainDataLimit: u32 = 150;
	pub const AccountSizeLimit: u32 = 100_000;
}

impl ternoa_marketplace::Config for Runtime {
	type Event = Event;
	type WeightInfo = weights::ternoa_marketplace::WeightInfo<Runtime>;
	type Currency = Balances;
	type FeesCollector = Treasury;
	type NFTExt = NFT;
	type InitialMintFee = MarketplaceInitialMintFee;
	type OffchainDataLimit = OffchainDataLimit;
	type AccountSizeLimit = AccountSizeLimit;
}

impl pallet_assets::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Currency = Balances;
	type ForceOrigin = RootOrAtLeastHalfOfCommittee;
	type AssetId = u32;
	type AssetDeposit = common::assets::AssetDeposit;
	type AssetAccountDeposit = common::assets::AssetAccountDeposit;
	type MetadataDepositBase = common::assets::MetadataDepositBase;
	type MetadataDepositPerByte = common::assets::MetadataDepositPerByte;
	type ApprovalDeposit = common::assets::ApprovalDeposit;
	type StringLimit = common::assets::StringLimit;
	type Freezer = ();
	type Extra = ();
	type WeightInfo = weights::pallet_assets::WeightInfo<Runtime>;
}
