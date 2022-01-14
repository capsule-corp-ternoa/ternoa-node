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
    //pub bidders: SortedBidderList<AccountId, BalanceCaps>,
    pub top_bidder: Option<(AccountId, BalanceCaps)>,
    pub marketplace_id: MarketplaceId,
}

#[derive(Encode, Decode, Clone, PartialEq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
/// wrapper type to store sorted list of all bids
pub struct SortedBidderList<AccountId, BalanceCaps>(pub Vec<(AccountId, BalanceCaps)>);

impl<AccountId, BalanceCaps: std::cmp::PartialOrd> SortedBidderList<AccountId, BalanceCaps> {
    pub const MAX_COUNT: usize = 10;

    /// Create a new empty bidders list
    fn new() -> Self {
        Self(Vec::new())
    }

    /// Insert a new bid to the list
    ///
    /// The function ensures that the new bid is always higher than the current highest bid
    /// If the insert action will cause an overflow, the lowest bid is removed and returned
    /// If value lower than current highest bid is passed, the function will panic, but this panic should
    /// ideally never be triggered since the extrinsic always checks for this.
    fn insert_new_bid(
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
        /// If list is at max capacity, remove lowest bid
        match self.0.len() {
            Self::MAX_COUNT => {
                let removed_bid = self.0.remove(0);
                self.0.push((account_id, value));
                /// return removed bid
                Some(removed_bid)
            }
            _ => {
                self.0.push((account_id, value));
                None
            }
        }
    }

    /// Get length of bidders list
    fn len(&mut self) -> usize {
        self.0.len()
    }

    /// Get current highest bid in list
    fn get_current_highest_bid(&mut self) -> Option<&(AccountId, BalanceCaps)> {
        self.0.last()
    }

    /// Get current lowest bid in list
    fn get_current_lowest_bid(&mut self) -> Option<&(AccountId, BalanceCaps)> {
        self.0.first()
    }

    /// Remove the lowest bid in list
    fn remove_lowest_bid(&mut self) -> (AccountId, BalanceCaps) {
        self.0.remove(0)
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

    bidders_list.insert_new_bid(1u32, 3u32);
    assert_eq!(bidders_list, SortedBidderList([(1u32, 2u32), (1u32, 3u32)].to_vec()));

    // get highest bid works
    assert_eq!(bidders_list.get_current_highest_bid(), Some(&(1u32, 3u32)));

    // get lowest bid works
    assert_eq!(bidders_list.get_current_lowest_bid(), Some(&(1u32, 2u32)));

    // insert max bids
    for n in 4..12 {
        bidders_list.insert_new_bid(1u32, n);
    }

    // ensure the insertion has worked correctly
    assert_eq!(
        bidders_list,
        SortedBidderList(
            [
                (1, 2),
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (1, 9),
                (1, 10),
                (1, 11)
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
                (1, 3),
                (1, 4),
                (1, 5),
                (1, 6),
                (1, 7),
                (1, 8),
                (1, 9),
                (1, 10),
                (1, 11),
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
