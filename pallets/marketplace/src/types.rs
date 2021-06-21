use std::fmt::Debug;

use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTCurrencyCombined {
    pub caps: u128,
    pub tiime: u128,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NFTCurrency {
    CAPS(u128),
    TIIME(u128),
    COMBINED(NFTCurrencyCombined),
}

impl Default for NFTCurrency {
    fn default() -> Self {
        Self::CAPS(0)
    }
}

impl NFTCurrency {
    pub fn caps(&self) -> Option<u128> {
        match self {
            NFTCurrency::CAPS(x) => Some(x.clone()),
            NFTCurrency::TIIME(_) => None,
            NFTCurrency::COMBINED(x) => Some(x.caps),
        }
    }

    pub fn tiime(&self) -> Option<u128> {
        match self {
            NFTCurrency::CAPS(_) => None,
            NFTCurrency::TIIME(x) => Some(x.clone()),
            NFTCurrency::COMBINED(x) => Some(x.tiime),
        }
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NFTCurrencyId {
    CAPS,
    TIIME,
}

impl Default for NFTCurrencyId {
    fn default() -> Self {
        Self::CAPS
    }
}
