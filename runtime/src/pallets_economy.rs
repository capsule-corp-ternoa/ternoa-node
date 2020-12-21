use crate::{
    constants::currency::{MILLICENTS, UNIT},
    pallets_core::Author,
    Balances, Event, Runtime,
};
use frame_support::{
    parameter_types,
    traits::{Currency, Imbalance, OnUnbalanced},
    weights::IdentityFee,
};
use pallet_transaction_payment::{Multiplier, TargetedFeeAdjustment};
use sp_runtime::{FixedPointNumber, Perquintill};
use ternoa_primitives::{AccountId, Balance};

pub type NegativeImbalance = <Balances as Currency<AccountId>>::NegativeImbalance;

/// Used so that we can support the `OnUnbalanced` trait and handle transaction fees. In this
/// case we wire all of them to the block author.
pub struct DealWithFees;
impl OnUnbalanced<NegativeImbalance> for DealWithFees {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance>) {
        if let Some(mut fees) = fees_then_tips.next() {
            if let Some(tips) = fees_then_tips.next() {
                // Think of it as an addition
                tips.merge_into(&mut fees);
            }
            Author::on_unbalanced(fees);
        }
    }
}

parameter_types! {
    pub const TransactionByteFee: Balance = 10 * MILLICENTS;
    pub const TargetBlockFullness: Perquintill = Perquintill::from_percent(25);
    pub AdjustmentVariable: Multiplier = Multiplier::saturating_from_rational(1, 100_000);
    pub MinimumMultiplier: Multiplier = Multiplier::saturating_from_rational(1, 1_000_000_000u128);
}

impl pallet_transaction_payment::Trait for Runtime {
    type Currency = Balances;
    type OnTransactionPayment = DealWithFees;
    type TransactionByteFee = TransactionByteFee;
    type WeightToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate =
        TargetedFeeAdjustment<Self, TargetBlockFullness, AdjustmentVariable, MinimumMultiplier>;
}

parameter_types! {
    pub const ExistentialDeposit: Balance = 1 * UNIT;
    // For weight estimation, we assume that the most locks on an individual account will be 50.
    // This number may need to be adjusted in the future if this assumption no longer holds true.
    pub const MaxLocks: u32 = 50;
}

impl pallet_balances::Trait for Runtime {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type DustRemoval = ();
    type Event = Event;
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = frame_system::Module<Runtime>;
    type WeightInfo = ();
}
