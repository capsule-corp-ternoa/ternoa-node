#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

pub type EnclaveId = u32;
pub type ClusterId = u32;
pub type Url = Vec<u8>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Enclave {
    pub api_url: Url,
}

impl Enclave {
    pub fn new(api_url: Url) -> Self {
        Self { api_url }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct Cluster {
    pub enclaves: Vec<EnclaveId>,
}

impl Cluster {
    pub fn new(enclaves: Vec<EnclaveId>) -> Self {
        Self { enclaves }
    }
}
