use crate::{
    constants::currency::{CENTS, MILLICENTS},
    Balances, Event, Runtime, Staking, TiimeAccountStore,
};
use frame_support::{
    parameter_types,
    traits::{Currency, OnUnbalanced},
    weights::IdentityFee,
};
use pallet_transaction_payment::{CurrencyAdapter, Multiplier, TargetedFeeAdjustment};
use sp_runtime::{FixedPointNumber, Perquintill};
use ternoa_primitives::{AccountId, Balance};

pub type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

/// Used so that we can support the `OnUnbalanced` trait and handle transaction fees. In this
/// case we wire all of them to the block author.
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_nonzero_unbalanced(amount: NegativeImbalance) {
        // rewards are stored in the staking's reward pool for the current era,
        // they are then dispatched to the validators being rewarded based on
        // how much work they have performed.
        Staking::on_unbalanced(amount);
    }
}

parameter_types! {
    pub const TransactionByteFee: Balance = 10 * MILLICENTS;
    pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
    pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
    pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl pallet_transaction_payment::Config for Runtime {
    type OnChargeTransaction = CurrencyAdapter<Balances, DealWithFees>;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate =
        TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 5 * CENTS;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Pallet<Runtime>;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}

impl ternoa_account_store::Config for Runtime {
    type AccountData = pallet_balances::AccountData<Balance>;
}

impl pallet_balances::Config<pallet_balances::Instance1> for Runtime {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = TiimeAccountStore;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
}
