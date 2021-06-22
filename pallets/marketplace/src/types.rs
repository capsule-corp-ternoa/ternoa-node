use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::{BalanceCaps, BalanceTiime, Config};

/// TODO!
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTCurrencyCombined<T: Config> {
    pub caps: BalanceCaps<T>,
    pub tiime: BalanceTiime<T>,
}

/// TODO!
#[derive(Encode, Decode, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum NFTCurrency<T: Config> {
    CAPS(BalanceCaps<T>),
    TIIME(BalanceTiime<T>),
    COMBINED(NFTCurrencyCombined<T>),
}

impl<T: Config> NFTCurrency<T> {
    pub fn caps(&self) -> Option<BalanceCaps<T>> {
        match self {
            NFTCurrency::CAPS(x) => Some(x.clone()),
            NFTCurrency::TIIME(_) => None,
            NFTCurrency::COMBINED(x) => Some(x.caps),
        }
    }

    pub fn tiime(&self) -> Option<BalanceTiime<T>> {
        match self {
            NFTCurrency::CAPS(_) => None,
            NFTCurrency::TIIME(x) => Some(x.clone()),
            NFTCurrency::COMBINED(x) => Some(x.tiime),
        }
    }
}

impl<T: Config> Default for NFTCurrency<T> {
    fn default() -> Self {
        Self::CAPS(BalanceCaps::<T>::default())
    }
}

#[cfg(feature = "std")]
impl<T: Config> std::fmt::Debug for NFTCurrency<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// TODO!
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
