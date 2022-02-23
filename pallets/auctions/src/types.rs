#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use ternoa_primitives::marketplace::MarketplaceId;
use ternoa_primitives::nfts::NFTId;

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Structure to store Auction data
pub struct AuctionData<AccountId, BlockNumber, Balance>
where
	AccountId: Clone,
	Balance: Clone + Default,
{
	/// The owner of the nft that has listed the item on auction
	pub creator: AccountId,
	/// `BlockNumber` at which the auction will accept bids
	pub start_block: BlockNumber,
	/// `BlockNumber` at which the auction will no longer accept bids
	pub end_block: BlockNumber,
	/// Floor `Balance` for creating a bid
	pub start_price: Balance,
	/// Optional price at which the auction is stopped and item can be bought
	pub buy_it_price: Option<Balance>,
	/// List of bidders
	pub bidders: BidderList<AccountId, Balance>,
	/// The marketplace where the auction has been listed
	pub marketplace_id: MarketplaceId,
	/// Is the auction going beyond the original end_block
	pub is_extended: bool,
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// wrapper type to store sorted list of all bids
/// The wrapper exists to ensure a queue implementation of sorted bids
pub struct BidderList<AccountId, Balance> {
	pub list: Vec<(AccountId, Balance)>,
	pub max_size: u16,
}

impl<AccountId, Balance> BidderList<AccountId, Balance>
where
	AccountId: sp_std::cmp::Ord + Clone,
	Balance: sp_std::cmp::PartialOrd,
{
	/// Create a new empty bidders list
	pub fn new(max_size: u16) -> Self {
		Self { list: Vec::new(), max_size }
	}

	/// Insert a new bid to the list
	pub fn insert_new_bid(
		&mut self,
		account_id: AccountId,
		value: Balance,
	) -> Option<(AccountId, Balance)> {
		// If list is at max capacity, remove lowest bid
		if self.list.len() >= self.max_size as usize {
			let removed_bid = self.list.remove(0);
			self.list.push((account_id, value));
			// return removed bid
			Some(removed_bid)
		} else {
			self.list.push((account_id, value));
			None
		}
	}

	/// Get length of bidders list
	pub fn len(&self) -> usize {
		self.list.len()
	}

	/// Get current highest bid in list
	pub fn get_highest_bid(&self) -> Option<&(AccountId, Balance)> {
		self.list.last()
	}

	/// Get current lowest bid in list
	pub fn get_lowest_bid(&self) -> Option<&(AccountId, Balance)> {
		self.list.first()
	}

	/// Remove the lowest bid in list
	pub fn remove_lowest_bid(&mut self) -> (AccountId, Balance) {
		self.list.remove(0)
	}

	/// Remove the highest bid in list
	pub fn remove_highest_bid(&mut self) -> Option<(AccountId, Balance)> {
		match self.list.len() {
			0 => None,
			n => Some(self.list.remove(n - 1)),
		}
	}

	/// Remove a specific bid from `account_id` from list if it exists
	pub fn remove_bid(&mut self, account_id: AccountId) -> Option<(AccountId, Balance)> {
		match self.list.iter().position(|x| x.0 == account_id) {
			Some(index) => Some(self.list.remove(index)),
			None => None,
		}
	}

	/// Return the bid of `account_id` if it exists
	pub fn find_bid(&self, account_id: AccountId) -> Option<&(AccountId, Balance)> {
		// this is not optimal since we traverse the entire link, but we cannot use binary search here
		// since the list is not sorted by accountId but rather by bid value, this should not drastically affect performance
		// as long as max_size remains small.
		self.list.iter().find(|&x| x.0 == account_id)
	}
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo, Default)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// wrapper type to store sorted list of all bids
/// The wrapper exists to ensure a queue implementation of sorted bids
pub struct DeadlineList<BlockNumber>(pub Vec<(NFTId, BlockNumber)>);

impl<BlockNumber> DeadlineList<BlockNumber>
where
	BlockNumber: sp_std::cmp::PartialOrd,
{
	pub fn insert(&mut self, nft_id: NFTId, block_number: BlockNumber) {
		let index = self.0.iter().position(|x| x.1 > block_number);
		let index = index.unwrap_or_else(|| self.0.len());

		self.0.insert(index, (nft_id, block_number));
	}

	pub fn remove(&mut self, nft_id: NFTId) -> bool {
		let index = self.0.iter().position(|x| x.0 == nft_id);
		if let Some(index) = index {
			self.0.remove(index);
			true
		} else {
			false
		}
	}

	pub fn update(&mut self, nft_id: NFTId, block_number: BlockNumber) -> bool {
		let removed = self.remove(nft_id);
		if removed {
			self.insert(nft_id, block_number);
			true
		} else {
			false
		}
	}

	pub fn next(&self, block_number: BlockNumber) -> Option<NFTId> {
		let front = self.0.get(0)?;
		if front.1 <= block_number {
			Some(front.0)
		} else {
			None
		}
	}
}
