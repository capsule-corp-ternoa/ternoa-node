#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::Config;
use codec::{Decode, Encode};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

/// Structure that stores both NFT currencies at the same time.
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct NFTCurrencyCombined<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    pub caps: BalanceCaps,
    pub tiime: BalanceTiime,
}

impl<BalanceCaps, BalanceTiime> NFTCurrencyCombined<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    pub fn new(caps: BalanceCaps, tiime: BalanceTiime) -> Self {
        Self { caps, tiime }
    }
}

#[cfg(feature = "std")]
impl<BalanceCaps, BalanceTiime> std::fmt::Debug for NFTCurrencyCombined<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default + std::fmt::Debug,
    BalanceTiime: Clone + Default + std::fmt::Debug,
{
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
pub enum NFTCurrency<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    CAPS(BalanceCaps),
    TIIME(BalanceTiime),
    COMBINED(NFTCurrencyCombined<BalanceCaps, BalanceTiime>),
}

impl<BalanceCaps, BalanceTiime> NFTCurrency<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default + Copy,
    BalanceTiime: Clone + Default + Copy,
{
    pub fn caps(&self) -> Option<BalanceCaps> {
        match self {
            NFTCurrency::CAPS(x) => Some(x.clone()),
            NFTCurrency::TIIME(_) => None,
            NFTCurrency::COMBINED(x) => Some(x.caps),
        }
    }

    pub fn tiime(&self) -> Option<BalanceTiime> {
        match self {
            NFTCurrency::CAPS(_) => None,
            NFTCurrency::TIIME(x) => Some(x.clone()),
            NFTCurrency::COMBINED(x) => Some(x.tiime),
        }
    }
}

impl<BalanceCaps, BalanceTiime> Default for NFTCurrency<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    fn default() -> Self {
        Self::CAPS(BalanceCaps::default())
    }
}

#[cfg(feature = "std")]
impl<BalanceCaps, BalanceTiime> std::fmt::Debug for NFTCurrency<BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default + std::fmt::Debug,
    BalanceTiime: Clone + Default + std::fmt::Debug,
{
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
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SaleInformation<AccountId, BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    pub account_id: AccountId,
    pub price: NFTCurrency<BalanceCaps, BalanceTiime>,
    pub marketplace_id: MarketplaceId,
}

impl<AccountId, BalanceCaps, BalanceTiime> Default
    for SaleInformation<AccountId, BalanceCaps, BalanceTiime>
where
    AccountId: Clone + Default,
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    fn default() -> Self {
        Self {
            account_id: Default::default(),
            price: Default::default(),
            marketplace_id: Default::default(),
        }
    }
}

impl<AccountId, BalanceCaps, BalanceTiime> SaleInformation<AccountId, BalanceCaps, BalanceTiime>
where
    BalanceCaps: Clone + Default,
    BalanceTiime: Clone + Default,
{
    pub fn new(
        account_id: AccountId,
        price: NFTCurrency<BalanceCaps, BalanceTiime>,
        marketplace_id: MarketplaceId,
    ) -> SaleInformation<AccountId, BalanceCaps, BalanceTiime> {
        Self {
            account_id,
            price,
            marketplace_id,
        }
    }
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MarketplaceType {
    Public,
    Private,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct MarketplaceInformation<T: Config> {
    pub kind: MarketplaceType,
    pub commission_fee: u8,
    pub owner: T::AccountId,
    pub allow_list: Vec<T::AccountId>,
}

impl<T: Config> MarketplaceInformation<T> {
    pub fn new(
        kind: MarketplaceType,
        commission_fee: u8,
        owner: T::AccountId,
        allow_list: Vec<T::AccountId>,
    ) -> MarketplaceInformation<T> {
        Self {
            kind,
            commission_fee,
            owner,
            allow_list,
        }
    }
}
