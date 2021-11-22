#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use ternoa_primitives::ternoa;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct AltvrUser {
    pub username: ternoa::String,
    pub vchatname: ternoa::String,
}

impl AltvrUser {
    pub fn new(username: ternoa::String, vchatname: ternoa::String) -> AltvrUser {
        Self {
            username,
            vchatname,
        }
    }
}
