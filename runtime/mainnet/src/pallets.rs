use super::weights;
pub use common::authority::{EpochDuration, BABE_GENESIS_EPOCH_CONFIG};
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
	AuthorityDiscovery, Babe, BagsList, Balances, Call, ElectionProviderMultiPhase, Event, Grandpa,
	Historical, ImOnline, Offences, Origin, OriginCaller, PalletInfo, Preimage, Runtime, Session,
	Signature, SignedPayload, Staking, StakingRewards, System, TechnicalCommittee, Timestamp,
	TransactionPayment, Treasury, UncheckedExtrinsic, VERSION,
};

#[cfg(any(feature = "std", test))]
pub use pallet_staking::StakerStatus;

type AtLeastThirdsOfCommittee = EnsureOneOf<
	EnsureRoot<AccountId>,
	pallet_collective::EnsureProportionAtLeast<AccountId, TechnicalCollective, 2, 3>,
>;

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;
}

impl frame_system::Config for Runtime {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = common::system::RuntimeBlockWeights;
	type BlockLength = common::system::RuntimeBlockLength;
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
	type BlockHashCount = common::system::BlockHashCount;
	type Version = Version;
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	type SS58Prefix = common::system::SS58Prefix;
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
	type TransactionByteFee = common::transactions::TransactionByteFee;
	type OperationalFeeMultiplier = common::transactions::OperationalFeeMultiplier;
	type WeightToFee = IdentityFee<Balance>;
	type FeeMultiplierUpdate = common::transactions::SlowAdjustingFeeUpdate<Self>; // TODO!
}

impl pallet_balances::Config for Runtime {
	type MaxLocks = common::other::MaxLocks;
	type Balance = Balance;
	type MaxReserves = common::other::MaxReserves;
	type ReserveIdentifier = [u8; 8];
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = common::other::ExistentialDeposit;
	type AccountStore = frame_system::Pallet<Runtime>;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
}

impl pallet_timestamp::Config for Runtime {
	type Moment = Moment;
	type OnTimestampSet = Babe;
	type MinimumPeriod = common::other::TimestampMinimumPeriod;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = common::other::TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = AtLeastThirdsOfCommittee;
	type RejectOrigin = AtLeastThirdsOfCommittee;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = common::other::ProposalBond;
	type SpendPeriod = common::other::SpendPeriod;
	type Burn = common::other::Burn;
	type BurnDestination = ();
	type SpendFunds = ();
	type WeightInfo = weights::pallet_treasury::WeightInfo<Runtime>;
	type MaxApprovals = common::other::MaxApprovals;
	type ProposalBondMinimum = common::other::ProposalBondMinimum;
	type ProposalBondMaximum = common::other::ProposalBondMaximum;
}

// Babe
impl pallet_babe::Config for Runtime {
	type EpochDuration = common::authority::EpochDuration;
	type ExpectedBlockTime = common::authority::ExpectedBlockTime;
	// session module is the trigger
	type EpochChangeTrigger = common::authority::EpochChangeTrigger;
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
	type HandleEquivocation = pallet_babe::EquivocationHandler<
		Self::KeyOwnerIdentification,
		Offences,
		common::authority::ReportLongevity,
	>;
	type WeightInfo = ();
	type MaxAuthorities = common::authority::MaxAuthorities;
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
		common::authority::ReportLongevity,
	>;
	type WeightInfo = ();
	type MaxAuthorities = common::authority::MaxAuthorities;
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
	type UncleGenerations = common::authority::UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (Staking, ImOnline);
}

impl pallet_authority_discovery::Config for Runtime {
	type MaxAuthorities = common::authority::MaxAuthorities;
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
		let period = common::system::BlockHashCount::get()
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
	type UnsignedPriority = common::authority::ImOnlineUnsignedPriority;
	type WeightInfo = weights::pallet_im_online::WeightInfo<Runtime>;
	type MaxKeys = common::authority::MaxKeys;
	type MaxPeerInHeartbeats = common::authority::MaxPeerInHeartbeats;
	type MaxPeerDataEncodingSize = common::authority::MaxPeerDataEncodingSize;
}

impl pallet_offences::Config for Runtime {
	type Event = Event;
	type IdentificationTuple = pallet_session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

impl frame_election_provider_support::onchain::Config for Runtime {
	type Accuracy = common::elections::OnChainAccuracy;
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
	type SlashCancelOrigin = AtLeastThirdsOfCommittee;
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
	type MaxUnlockingChunks = frame_support::traits::ConstU32<32>;
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
	type SignedPhase = common::elections::SignedPhase;
	/// Duration of the unsigned phase. After the signed phase the unsigned phase comes where the
	/// OCWs from validators compute the election result (solution). The best score from the
	/// unsigned and signed phase is used.
	type UnsignedPhase = common::elections::UnsignedPhase;
	type SignedMaxSubmissions = common::elections::SignedMaxSubmissions;
	type SignedRewardBase = common::elections::SignedRewardBase;
	type SignedDepositBase = common::elections::SignedDepositBase;
	type SignedDepositByte = common::elections::SignedDepositByte;
	type SignedDepositWeight = ();
	type SignedMaxWeight = Self::MinerMaxWeight;
	type SlashHandler = Treasury; // burn slashes
	type RewardHandler = (); // nothing to do upon rewards
	type SolutionImprovementThreshold = common::elections::SolutionImprovementThreshold;
	type MinerMaxWeight = common::elections::MinerMaxWeight;
	type MinerMaxLength = common::elections::MinerMaxLength;
	type OffchainRepeat = common::elections::OffchainRepeat;
	type MinerTxPriority = common::elections::NposSolutionPriority;
	type DataProvider = Staking;
	type Solution = common::elections::NposCompactSolution24;
	type Fallback = common::elections::Fallback<Self>;
	type GovernanceFallback = common::elections::GovernanceFallback<Self>;
	type Solver = common::elections::Solver<Self>;
	type WeightInfo = weights::pallet_election_provider_multi_phase::WeightInfo<Runtime>;
	type ForceOrigin = AtLeastThirdsOfCommittee;
	type BenchmarkingConfig = common::elections::BenchmarkConfig;
	type MaxElectingVoters = common::elections::MaxElectingVoters;
	type MaxElectableTargets = common::elections::MaxElectableTargets;
}

// BagsList
impl pallet_bags_list::Config for Runtime {
	type Event = Event;
	type ScoreProvider = Staking;
	type WeightInfo = weights::pallet_bags_list::WeightInfo<Runtime>;
	type BagThresholds = common::other::BagThresholds;
	type Score = sp_npos_elections::VoteWeight;
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type MaxSize = common::other::PreimageMaxSize;
	type BaseDeposit = common::other::PreimageBaseDeposit;
	type ByteDeposit = common::other::PreimageByteDeposit;
}

// Technical collective
type TechnicalCollective = pallet_collective::Instance1;
impl pallet_collective::Config<TechnicalCollective> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = common::other::TechnicalMotionDuration;
	type MaxProposals = common::other::TechnicalMaxProposals;
	type MaxMembers = common::other::TechnicalMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

// Pallet Membership
impl pallet_membership::Config for Runtime {
	type Event = Event;
	type AddOrigin = AtLeastThirdsOfCommittee;
	type RemoveOrigin = AtLeastThirdsOfCommittee;
	type SwapOrigin = AtLeastThirdsOfCommittee;
	type ResetOrigin = AtLeastThirdsOfCommittee;
	type PrimeOrigin = AtLeastThirdsOfCommittee;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = common::other::TechnicalMaxMembers;
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
	type MaximumWeight = common::other::MaximumSchedulerWeight;
	type ScheduleOrigin = AtLeastThirdsOfCommittee;
	type MaxScheduledPerBlock = common::other::MaxScheduledPerBlock;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
	type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = common::other::NoPreimagePostponement;
}

// Staking rewards
impl ternoa_staking_rewards::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type PalletId = common::staking::StakingRewardsPalletId;
	type ExternalOrigin = AtLeastThirdsOfCommittee;
	type WeightInfo = weights::ternoa_staking_rewards::WeightInfo<Runtime>;
}
