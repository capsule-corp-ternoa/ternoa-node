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
use common::staking::{BondingDuration, SessionsPerEra};
use frame_support::{
	parameter_types,
	traits::{ConstU32, EnsureOneOf, KeyOwnerProofSystem, U128CurrencyToVote},
	weights::{constants::RocksDbWeight, IdentityFee},
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
	SaturatedConversion,
};
use sp_std::vec::Vec;
use sp_version::RuntimeVersion;
use ternoa_core_primitives::{AccountId, Balance, BlockNumber, Hash, Index, Moment};
use ternoa_runtime_common as common;

use crate::{
	constants::time::EPOCH_DURATION_IN_SLOTS, AuthorityDiscovery, Babe, BagsList, Balances, Call,
	ElectionProviderMultiPhase, Event, Grandpa, Historical, ImOnline, Offences, Origin,
	OriginCaller, PalletInfo, Preimage, Runtime, Session, Signature, SignedPayload, Staking,
	StakingRewards, System, TechnicalCommittee, Timestamp, TransactionPayment, Treasury,
	UncheckedExtrinsic, VERSION,
};

pub use common::babe::BABE_GENESIS_EPOCH_CONFIG;

#[cfg(any(feature = "std", test))]
pub use pallet_staking::StakerStatus;

type AtLeastTwoThirdsOfCommittee = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>,
>;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = common::frame_system::RuntimeBlockWeights;
	type BlockLength = common::frame_system::RuntimeBlockLength;
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
	type BlockHashCount = common::frame_system::BlockHashCount;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	type SS58Prefix = common::frame_system::SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

// Utility
impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = weights::pallet_utility::WeightInfo<Runtime>;
}

impl pallet_transaction_payment::Config for Runtime {
	type OnChargeTransaction = CurrencyAdapter<Balances, StakingRewards>;
	type TransactionByteFee = common::transaction_payment::TransactionByteFee;
	type OperationalFeeMultiplier = common::transaction_payment::OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = common::transaction_payment::SlowAdjustingFeeUpdate<Self>; // TODO!
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
	type ApproveOrigin = AtLeastTwoThirdsOfCommittee;
	type RejectOrigin = AtLeastTwoThirdsOfCommittee;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = common::treasury::ProposalBond;
	type SpendPeriod = common::treasury::SpendPeriod;
	type Burn = common::treasury::Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
	type MaxApprovals = common::treasury::MaxApprovals;
	type ProposalBondMinimum = common::treasury::ProposalBondMinimum;
	type ProposalBondMaximum = common::treasury::ProposalBondMaximum;
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
		let period = common::frame_system::BlockHashCount::get()
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

impl frame_election_provider_support::onchain::Config for Runtime {
	type Accuracy = common::election_provider_support::OnChainAccuracy;
	type DataProvider = Staking;
}

impl pallet_staking::Config for Runtime {
	type MaxNominations = common::staking::MaxNominations;
	type Currency = Balances;
	type UnixTime = Timestamp;
	type CurrencyToVote = U128CurrencyToVote;
	type RewardRemainder = Treasury;
	type Event = Event;
	type Slash = Treasury; // send the slashed funds to the treasury.
	type Reward = (); // rewards are minted from the void
	type SessionsPerEra = common::staking::SessionsPerEra;
	type BondingDuration = common::staking::BondingDuration;
	type SlashDeferDuration = common::staking::SlashDeferDuration;
	/// A super-majority of the council can cancel the slash.
	type SlashCancelOrigin = AtLeastTwoThirdsOfCommittee;
	type SessionInterface = Self;
	type EraPayout = StakingRewards;
	type NextNewSession = Session;
	type MaxNominatorRewardedPerValidator = common::staking::MaxNominatorRewardedPerValidator;
	type OffendingValidatorsThreshold = common::staking::OffendingValidatorsThreshold;
	type ElectionProvider = ElectionProviderMultiPhase;
	type GenesisElectionProvider = common::staking::GenesisElectionProvider<Self>;
	// Alternatively, use pallet_staking::UseNominatorsMap<Runtime> to just use the nominators map.
	// Note that the aforementioned does not scale to a very large number of nominators.
	type VoterList = BagsList;
	type BenchmarkingConfig = common::staking::StakingBenchmarkingConfig;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
	type MaxUnlockingChunks = common::staking::MaxUnlockingChunks;
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
	/// Duration of the signed phase. In the Signed phase miners (or any account) can compute the
	/// (solution) result of the election. If they did it correctly they will be rewarded. If they
	/// wanted to cheat the system they will be slashed. This Signed phase happens before then
	/// Unsigned one.
	type SignedPhase = SignedPhase;
	/// Duration of the unsigned phase. After the signed phase the unsigned phase comes where the
	/// OCWs from validators compute the election result (solution). The best score from the
	/// unsigned and signed phase is used.
	type UnsignedPhase = UnsignedPhase;
	type SignedMaxSubmissions = common::election_provider_multi_phase::SignedMaxSubmissions;
	type SignedRewardBase = common::election_provider_multi_phase::SignedRewardBase;
	type SignedDepositBase = common::election_provider_multi_phase::SignedDepositBase;
	type SignedDepositByte = common::election_provider_multi_phase::SignedDepositByte;
	type SignedDepositWeight = ();
	type SignedMaxWeight = Self::MinerMaxWeight;
	type SlashHandler = Treasury; // burn slashes
	type RewardHandler = (); // nothing to do upon rewards
	type SolutionImprovementThreshold =
		common::election_provider_multi_phase::SolutionImprovementThreshold;
	type MinerMaxWeight = common::election_provider_multi_phase::MinerMaxWeight;
	type MinerMaxLength = common::election_provider_multi_phase::MinerMaxLength;
	type OffchainRepeat = OffchainRepeat;
	type MinerTxPriority = common::election_provider_multi_phase::NposSolutionPriority;
	type DataProvider = Staking;
	type Solution = common::election_provider_multi_phase::NposCompactSolution24;
	type Fallback = common::election_provider_multi_phase::Fallback<Self>;
	type GovernanceFallback = common::election_provider_multi_phase::GovernanceFallback<Self>;
	type Solver = common::election_provider_multi_phase::Solver<Self>;
	type WeightInfo = weights::pallet_election_provider_multi_phase::WeightInfo<Runtime>;
	type ForceOrigin = AtLeastTwoThirdsOfCommittee;
	type BenchmarkingConfig = common::election_provider_multi_phase::BenchmarkConfig;
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
	type ManagerOrigin = EnsureRoot<AccountId>;
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
	type AddOrigin = AtLeastTwoThirdsOfCommittee;
	type RemoveOrigin = AtLeastTwoThirdsOfCommittee;
	type SwapOrigin = AtLeastTwoThirdsOfCommittee;
	type ResetOrigin = AtLeastTwoThirdsOfCommittee;
	type PrimeOrigin = AtLeastTwoThirdsOfCommittee;
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
		pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>;
}

// Scheduler
impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = common::scheduler::MaximumSchedulerWeight;
	type ScheduleOrigin = AtLeastTwoThirdsOfCommittee;
	type MaxScheduledPerBlock = common::scheduler::MaxScheduledPerBlock;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
	type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = common::scheduler::NoPreimagePostponement;
}

// Staking rewards
impl ternoa_staking_rewards::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type PalletId = common::staking_rewards::PalletId;
	type ExternalOrigin = AtLeastTwoThirdsOfCommittee;
	type WeightInfo = weights::ternoa_staking_rewards::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ProposalLifetime: BlockNumber = 100800;
	pub const InitialBridgeFee: Balance = 100_000_000_000_000_000_000_000;
}

impl ternoa_bridge::Config for Runtime {
	type Event = Event;
	type WeightInfo = weights::ternoa_bridge::WeightInfo<Runtime>;
	type Currency = Balances;
	type FeesCollector = Treasury;
	type ExternalOrigin = AtLeastTwoThirdsOfCommittee;
	type ChainId = common::bridge::ChainId;
	type PalletId = common::bridge::PalletId;
	type ProposalLifetime = ProposalLifetime;
	type RelayerVoteThreshold = common::bridge::RelayerVoteThreshold;
	type RelayerCountLimit = common::bridge::RelayerCountLimit;
	type InitialBridgeFee = InitialBridgeFee;
}
