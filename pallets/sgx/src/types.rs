#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use ternoa_primitives::TextFormat;

pub type EnclaveId = u32;
pub type ClusterId = u32;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Enclave {
	pub api_uri: TextFormat,
}

impl Enclave {
	pub fn new(api_uri: TextFormat) -> Self {
		Self { api_uri }
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Cluster {
	pub enclaves: Vec<EnclaveId>,
}

impl Cluster {
	pub fn new(enclaves: Vec<EnclaveId>) -> Self {
		Self { enclaves }
	}
}
