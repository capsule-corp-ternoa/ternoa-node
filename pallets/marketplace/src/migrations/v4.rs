use super::Config;
use crate::NFTsForSale;
use crate::{BalanceCaps, BalanceTiime, NFTCurrency, SaleInformation};
use frame_support::traits::Get;
use frame_support::weights::Weight;

pub fn migrate<T: Config>() -> Weight {
    /*     NFTsForSale::<T>::translate::<(T::AccountId, NFTCurrency<T>), _>(
        |_key, (account_id, price)| {
            return Some(SaleInformation::<
                T::AccountId,
                BalanceCaps<T>,
                BalanceTiime<T>,
            >::new(account_id, price, 0));
        },
    );

    T::BlockWeights::get().max_block */

    T::BlockWeights::get().max_block
}
