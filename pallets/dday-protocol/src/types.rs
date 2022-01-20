#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use scale_info::TypeInfo;
use ternoa_primitives::BlockNumber;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct TransmissionData<AccountId> {
    pub recipient: AccountId,
    pub delivery_date: BlockNumber,
}

impl<AccountId> TransmissionData<AccountId> {
    pub fn new(recipient: AccountId, delivery_date: BlockNumber) -> TransmissionData<AccountId> {
        TransmissionData {
            recipient, delivery_date
        }
    }
}