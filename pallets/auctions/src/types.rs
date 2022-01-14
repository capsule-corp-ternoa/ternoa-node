#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use ternoa_primitives::marketplace::MarketplaceId;

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
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
    pub bidders: SortedBidderList<AccountId, BalanceCaps>,
    pub marketplace_id: MarketplaceId,
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// wrapper type to store sorted list of all bids
/// The wrapper exists to ensure a queue implementation of sorted bids
pub struct SortedBidderList<AccountId, BalanceCaps>(pub Vec<(AccountId, BalanceCaps)>);

impl<AccountId, BalanceCaps> SortedBidderList<AccountId, BalanceCaps>
where
    AccountId: std::cmp::Ord + Clone,
    BalanceCaps: std::cmp::PartialOrd,
{
    pub const MAX_COUNT: usize = 10;

    /// Create a new empty bidders list
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Insert a new bid to the list
    ///
    /// The function ensures that the new bid is always higher than the current highest bid
    /// If the insert action will cause an overflow, the lowest bid is removed and returned
    /// If value lower than current highest bid is passed, the function will panic
    pub fn insert_new_bid(
        &mut self,
        account_id: AccountId,
        value: BalanceCaps,
    ) -> Option<(AccountId, BalanceCaps)> {
        // ensure the new bid is larger than current highest bid
        if let Some(current_highest_bid) = self.get_current_highest_bid() {
            if current_highest_bid.1 >= value {
                // this panic should never happen since the extrinsic already checks if value > current_highest_bid
                panic!("cannot accept a lower bid!");
            }
        }
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
    pub fn get_current_highest_bid(&self) -> Option<&(AccountId, BalanceCaps)> {
        self.0.last()
    }

    /// Get current lowest bid in list
    pub fn get_current_lowest_bid(&self) -> Option<&(AccountId, BalanceCaps)> {
        self.0.first()
    }

    /// Remove the lowest bid in list
    pub fn remove_lowest_bid(&mut self) -> (AccountId, BalanceCaps) {
        self.0.remove(0)
    }

    /// Remove a specific bid from list
    pub fn remove_bid(&mut self, account_id: &AccountId) {
        match self.0.binary_search_by_key(account_id, |(a, _)| a.clone()) {
            Ok(index) => {
                self.0.remove(index);
            }
            Err(_) => (), // since the value does not exist, ignore
        };
    }

    /// Find a specific bid from list
    pub fn find_bid(&self, account_id: &AccountId) -> Option<&(AccountId, BalanceCaps)> {
        match self.0.binary_search_by_key(account_id, |(a, _)| a.clone()) {
            Ok(index) => self.0.get(index),
            Err(_) => None,
        }
    }
}

#[test]
fn test_sorted_bid_works() {
    type MockBalance = u32;
    type MockAccount = u32;
    // create a new list
    let mut bidders_list: SortedBidderList<MockAccount, MockBalance> = SortedBidderList::new();

    // insert to list works
    bidders_list.insert_new_bid(1u32, 2u32);
    assert_eq!(bidders_list, SortedBidderList([(1u32, 2u32)].to_vec()));

    bidders_list.insert_new_bid(2u32, 3u32);
    assert_eq!(
        bidders_list,
        SortedBidderList([(1u32, 2u32), (2u32, 3u32)].to_vec())
    );

    // get highest bid works
    assert_eq!(bidders_list.get_current_highest_bid(), Some(&(2u32, 3u32)));

    // get lowest bid works
    assert_eq!(bidders_list.get_current_lowest_bid(), Some(&(1u32, 2u32)));

    // insert max bids
    for n in 4..12 {
        bidders_list.insert_new_bid(n, n + 1);
    }

    // ensure the insertion has worked correctly
    assert_eq!(
        bidders_list,
        SortedBidderList(
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
        SortedBidderList(
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
    assert_eq!(bidders_list.find_bid(&5), Some(&(5, 6)));
    assert_eq!(bidders_list.find_bid(&11), Some(&(11, 12)));
    assert_eq!(bidders_list.find_bid(&7), Some(&(7, 8)));

    // ensure remove_bid works
    bidders_list.remove_bid(&5);
    assert_eq!(
        bidders_list,
        SortedBidderList(
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
    bidders_list.remove_bid(&11);
    assert_eq!(
        bidders_list,
        SortedBidderList(
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
}

#[test]
#[should_panic(expected = "cannot accept a lower bid!")]
fn test_sorted_bid_insert_fails_for_lower_bid() {
    type MockBalance = u32;
    type MockAccount = u32;
    // create a new list
    let mut bidders_list: SortedBidderList<MockAccount, MockBalance> = SortedBidderList::new();

    // insert to list works
    bidders_list.insert_new_bid(1u32, 2u32);
    bidders_list.insert_new_bid(1u32, 2u32);
}
