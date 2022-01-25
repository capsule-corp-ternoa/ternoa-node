#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;
use ternoa_primitives::marketplace::MarketplaceId;

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// Structure to store Auction data
pub struct AuctionData<AccountId, BlockNumber, Balance>
where
    AccountId: Clone + Default,
    Balance: Clone + Default,
{
    /// the owner of the nft that has listed the item on auction
    pub creator: AccountId,
    /// `BlockNumber` at which the auction will accept bids
    pub start_block: BlockNumber,
    /// `BlockNumber` at which the auction will no longer accept bids
    pub end_block: BlockNumber,
    /// floor `Balance` for creating a bid
    pub start_price: Balance,
    /// Optional price at which the auction is stopped and item can be bought
    pub buy_it_price: Option<Balance>,
    /// List of last `MAX_COUNT` bids
    pub bidders: BidderList<AccountId, Balance>,
    /// the marketplace where the auction has been listed
    pub marketplace_id: MarketplaceId,
    /// the current state of the auction [Pending, InProcess, Extended, Completed]
    pub state: AuctionState,
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// enum to store the current state of an auction
pub enum AuctionState {
    /// The auction has been created but not yet started
    Pending,
    /// The auction has started and is in process
    InProcess,
    /// The auction has been extended past the original end_block
    Extended,
    /// The auction has been completed, the nft has been assigned to highest bidder
    Completed,
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// wrapper type to store sorted list of all bids
/// The wrapper exists to ensure a queue implementation of sorted bids
pub struct BidderList<AccountId, Balance>(pub Vec<(AccountId, Balance)>);

impl<AccountId, Balance> BidderList<AccountId, Balance>
where
    AccountId: sp_std::cmp::Ord + Clone,
    Balance: sp_std::cmp::PartialOrd,
{
    pub const MAX_COUNT: usize = 10;

    /// Create a new empty bidders list
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Insert a new bid to the list
    pub fn insert_new_bid(
        &mut self,
        account_id: AccountId,
        value: Balance,
    ) -> Option<(AccountId, Balance)> {
        // If list is at max capacity, remove lowest bid
        match self.0.len() {
            Self::MAX_COUNT => {
                let removed_bid = self.0.remove(0);
                self.0.push((account_id, value));
                // return removed bid
                Some(removed_bid)
            }
            _ => {
                self.0.push((account_id, value));
                None
            }
        }
    }

    /// Get length of bidders list
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Get current highest bid in list
    pub fn get_highest_bid(&self) -> Option<&(AccountId, Balance)> {
        self.0.last()
    }

    /// Get current lowest bid in list
    pub fn get_lowest_bid(&self) -> Option<&(AccountId, Balance)> {
        self.0.first()
    }

    /// Remove the lowest bid in list
    pub fn remove_lowest_bid(&mut self) -> (AccountId, Balance) {
        self.0.remove(0)
    }

    /// Remove the highest bid in list
    pub fn remove_highest_bid(&mut self) -> Option<(AccountId, Balance)> {
        match self.0.len() {
            0 => None,
            n => Some(self.0.remove(n - 1)),
        }
    }

    /// Remove a specific bid from `account_id` from list if it exists
    pub fn remove_bid(&mut self, account_id: AccountId) -> Option<(AccountId, Balance)> {
        match self.0.iter().position(|x| x.0 == account_id) {
            Some(index) => Some(self.0.remove(index)),
            None => None,
        }
    }

    /// Return the bid of `account_id` if it exists
    pub fn find_bid(&self, account_id: AccountId) -> Option<&(AccountId, Balance)> {
        // this is not optimal since we traverse the entire link, but we cannot use binary search here
        // since the list is not sorted by accountId but rather by bid value, this should not drastically affect performance
        // as long as MAX_COUNT remains small.
        self.0.iter().find(|&x| x.0 == account_id)
    }
}

#[test]
fn test_sorted_bid_works() {
    type MockBalance = u32;
    type MockAccount = u32;
    // create a new list
    let mut bidders_list: BidderList<MockAccount, MockBalance> = BidderList::new();

    // insert to list works
    bidders_list.insert_new_bid(1u32, 2u32);
    assert_eq!(bidders_list, BidderList([(1u32, 2u32)].to_vec()));

    bidders_list.insert_new_bid(2u32, 3u32);
    assert_eq!(
        bidders_list,
        BidderList([(1u32, 2u32), (2u32, 3u32)].to_vec())
    );

    // get highest bid works
    assert_eq!(bidders_list.get_highest_bid(), Some(&(2u32, 3u32)));

    // get lowest bid works
    assert_eq!(bidders_list.get_lowest_bid(), Some(&(1u32, 2u32)));

    // insert max bids
    for n in 4..12 {
        bidders_list.insert_new_bid(n, n + 1);
    }

    // ensure the insertion has worked correctly
    assert_eq!(
        bidders_list,
        BidderList(
            [
                (1, 2),
                (2, 3),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 8),
                (8, 9),
                (9, 10),
                (10, 11),
                (11, 12)
            ]
            .to_vec()
        )
    );

    // inserting the new bid should replace the lowest bid
    let lowest_bid = bidders_list.insert_new_bid(1u32, 102u32);
    assert_eq!(lowest_bid, Some((1, 2)));

    // ensure the insertion has worked correctly
    assert_eq!(
        bidders_list,
        BidderList(
            [
                (2, 3),
                (4, 5),
                (5, 6),
                (6, 7),
                (7, 8),
                (8, 9),
                (9, 10),
                (10, 11),
                (11, 12),
                (1, 102)
            ]
            .to_vec()
        )
    );

    // ensure find_bid works
    assert_eq!(bidders_list.find_bid(5), Some(&(5, 6)));
    assert_eq!(bidders_list.find_bid(11), Some(&(11, 12)));
    assert_eq!(bidders_list.find_bid(7), Some(&(7, 8)));
    assert_eq!(bidders_list.find_bid(2021), None);

    // ensure remove_bid works
    assert_eq!(bidders_list.remove_bid(5), Some((5, 6)));
    assert_eq!(
        bidders_list,
        BidderList(
            [
                (2, 3),
                (4, 5),
                (6, 7),
                (7, 8),
                (8, 9),
                (9, 10),
                (10, 11),
                (11, 12),
                (1, 102)
            ]
            .to_vec()
        )
    );

    // ensure remove_bid works
    assert_eq!(bidders_list.remove_bid(11), Some((11, 12)));
    assert_eq!(
        bidders_list,
        BidderList(
            [
                (2, 3),
                (4, 5),
                (6, 7),
                (7, 8),
                (8, 9),
                (9, 10),
                (10, 11),
                (1, 102)
            ]
            .to_vec()
        )
    );
    assert_eq!(bidders_list.remove_bid(2022), None);
    // ensure remove_bid works till empty
    assert_eq!(bidders_list.remove_bid(2), Some((2, 3)));
    assert_eq!(bidders_list.remove_bid(4), Some((4, 5)));
    assert_eq!(bidders_list.remove_bid(6), Some((6, 7)));
    assert_eq!(bidders_list.remove_bid(7), Some((7, 8)));
    assert_eq!(bidders_list.remove_bid(8), Some((8, 9)));
    assert_eq!(bidders_list.remove_bid(9), Some((9, 10)));
    assert_eq!(bidders_list.remove_bid(10), Some((10, 11)));
    assert_eq!(bidders_list.remove_bid(1), Some((1, 102)));
    assert_eq!(bidders_list, BidderList([].to_vec()));

    // insert max bids
    for n in 4..12 {
        bidders_list.insert_new_bid(n, n + 1);
    }

    assert_eq!(bidders_list.remove_highest_bid(), Some((11, 12)));
    assert_eq!(bidders_list.remove_highest_bid(), Some((10, 11)));
}
