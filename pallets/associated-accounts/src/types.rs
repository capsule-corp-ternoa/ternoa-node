#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use ternoa_primitives::TextFormat;

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SupportedAccount {
    pub key: TextFormat,
    pub min_length: u16,
    pub max_length: u16,
    pub initial_set_fee: bool,
}

impl SupportedAccount {
    pub fn new(key: TextFormat, min_length: u16, max_length: u16, initial_set_fee: bool) -> Self {
        Self {
            key,
            min_length,
            max_length,
            initial_set_fee,
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Account {
    pub key: TextFormat,
    pub value: TextFormat,
}

impl Account {
    pub fn new(key: TextFormat, value: TextFormat) -> Self {
        Self { key, value }
    }
}
