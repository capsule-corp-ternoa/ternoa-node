#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use ternoa_primitives::TextFormat;

pub type MarketplaceId = u32;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct SaleInformation<AccountId, Balance>
where
    Balance: Clone + Default,
{
    pub account_id: AccountId,
    pub price: Balance,
    pub marketplace_id: MarketplaceId,
}

impl<AccountId, Balance> Default for SaleInformation<AccountId, Balance>
where
    AccountId: Clone + Default,
    Balance: Clone + Default,
{
    fn default() -> Self {
        Self {
            account_id: Default::default(),
            price: Default::default(),
            marketplace_id: Default::default(),
        }
    }
}

impl<AccountId, Balance> SaleInformation<AccountId, Balance>
where
    Balance: Clone + Default,
{
    pub fn new(
        account_id: AccountId,
        price: Balance,
        marketplace_id: MarketplaceId,
    ) -> SaleInformation<AccountId, Balance> {
        Self {
            account_id,
            price,
            marketplace_id,
        }
    }
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum MarketplaceType {
    Public,
    Private,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct MarketplaceInformation<AccountId> {
    pub kind: MarketplaceType,
    pub commission_fee: u8,
    pub owner: AccountId,
    pub allow_list: Vec<AccountId>,
    pub disallow_list: Vec<AccountId>,
    pub name: TextFormat,
    pub uri: Option<TextFormat>,
    pub logo_uri: Option<TextFormat>,
    pub description: Option<TextFormat>,
}

impl<AccountId> MarketplaceInformation<AccountId> {
    pub fn new(
        kind: MarketplaceType,
        commission_fee: u8,
        owner: AccountId,
        allow_list: Vec<AccountId>,
        disallow_list: Vec<AccountId>,
        name: TextFormat,
        uri: Option<TextFormat>,
        logo_uri: Option<TextFormat>,
        description: Option<TextFormat>,
    ) -> MarketplaceInformation<AccountId> {
        Self {
            kind,
            commission_fee,
            owner,
            allow_list,
            disallow_list,
            name,
            uri,
            logo_uri,
            description,
        }
    }
}
