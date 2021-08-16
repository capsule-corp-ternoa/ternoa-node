use codec::{Decode, Encode};
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::{BalanceCaps, BalanceTiime, Config};
use sp_runtime::RuntimeDebug;

/// Structure that stores both NFT currencies at the same time.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTCurrencyCombined<T: Config> {
    pub caps: BalanceCaps<T>,
    pub tiime: BalanceTiime<T>,
}

impl<T: Config> NFTCurrencyCombined<T> {
    pub fn new(caps: BalanceCaps<T>, tiime: BalanceTiime<T>) -> Self {
        Self { caps, tiime }
    }
}

#[cfg(feature = "std")]
impl<T: Config> std::fmt::Debug for NFTCurrencyCombined<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "NFTCurrencyCombined {{caps: {:?}, tiime: {:?}}}",
            self.caps, self.tiime
        )
    }
}

/// Currency combination that can be used to set a price of an NFT.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
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
        match &self {
            NFTCurrency::CAPS(x) => write!(f, "CAPS({:?})", x),
            NFTCurrency::TIIME(x) => write!(f, "TIIME({:?})", x),
            NFTCurrency::COMBINED(x) => write!(f, "COMBINED({:?})", x),
        }
    }
}

/// Currency ID
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

pub type MarketplaceId = u32;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct SaleInformation<T: Config> {
    pub account_id: T::AccountId,
    pub price: NFTCurrency<T>,
    pub marketplace_id: MarketplaceId,
}

impl<T: Config> Default for SaleInformation<T> {
    fn default() -> Self {
        Self {
            account_id: Default::default(),
            price: Default::default(),
            marketplace_id: Default::default(),
        }
    }
}

impl<T: Config> SaleInformation<T> {
    pub fn new(
        account_id: T::AccountId,
        price: NFTCurrency<T>,
        marketplace_id: MarketplaceId,
    ) -> SaleInformation<T> {
        Self {
            account_id,
            price,
            marketplace_id,
        }
    }
}
