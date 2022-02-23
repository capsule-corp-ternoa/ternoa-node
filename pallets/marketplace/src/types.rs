#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use ternoa_primitives::marketplace::MarketplaceId;

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
		Self { account_id, price, marketplace_id }
	}
}
