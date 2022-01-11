#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use ternoa_primitives::marketplace::MarketplaceId;
//use sp_std::iter::Map;

#[derive(Encode, Decode, Clone, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Structure to store Auction data
pub struct AuctionData<AccountId, BlockNumber, BalanceCaps>
where
    AccountId: Clone + Default,
    BalanceCaps: Clone + Default,
{
    pub creator: AccountId,
    pub start_block: BlockNumber,
    pub end_block: BlockNumber,
    pub start_price: BalanceCaps,
    pub buy_it_price: Option<BalanceCaps>,
    //pub bidders: Map<AccountId, BalanceCaps>, // TODO : Does this have to be stored on chain?
    pub top_bidder: Option<(AccountId, BalanceCaps)>,
    pub marketplace_id: MarketplaceId,
}
