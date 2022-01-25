use frame_support::dispatch::{
    DispatchErrorWithPostInfo, DispatchResult, DispatchResultWithPostInfo,
};
use ternoa_primitives::marketplace::{MarketplaceCommission, MarketplaceId, MarketplaceType};
use ternoa_primitives::nfts::{NFTData, NFTId, NFTSeriesId};
use ternoa_primitives::TextFormat;

pub trait NFTTrait {
    type AccountId;

    /// Change the owner of an NFT.
    fn set_owner(id: NFTId, owner: &Self::AccountId) -> DispatchResult;

    /// Return the owner of an NFT.
    fn owner(id: NFTId) -> Option<Self::AccountId>;

    /// Is series completed(locked)
    fn is_series_completed(id: NFTId) -> Option<bool>;

    /// Create NFT
    fn create_nft(
        owner: Self::AccountId,
        ipfs_reference: TextFormat,
        series_id: Option<NFTSeriesId>,
    ) -> Result<NFTId, DispatchErrorWithPostInfo>;

    /// Get NFT data
    fn get_nft(id: NFTId) -> Option<NFTData<Self::AccountId>>;

    /// Lock series WARNING: Only for benchmark purposes!
    fn benchmark_lock_series(series_id: NFTSeriesId);

    /// TODO!
    fn set_listed_for_sale(id: NFTId, value: bool) -> DispatchResult;

    /// TODO!
    fn is_listed_for_sale(id: NFTId) -> Option<bool>;

    /// TODO!
    fn set_in_transmission(id: NFTId, value: bool) -> DispatchResult;

    /// TODO!
    fn is_in_transmission(id: NFTId) -> Option<bool>;

    /// TODO!
    fn set_converted_to_capsule(id: NFTId, value: bool) -> DispatchResult;

    /// TODO!
    fn is_converted_to_capsule(id: NFTId) -> Option<bool>;
}

/// Trait that implements basic functionalities related to Ternoa Marketplace
/// TODO: Expand trait with more useful functions
pub trait MarketplaceTrait<AccountId> {
    /// Return if an account is permitted to list on given marketplace
    fn is_allowed_to_list(marketplace_id: MarketplaceId, account_id: AccountId) -> DispatchResult;

    /// Return the commission charged by a given marketplace
    fn get_marketplace_info(
        marketplace_id: MarketplaceId,
    ) -> Option<(AccountId, MarketplaceCommission)>;

    /// create a new marketplace
    fn create(
        origin: AccountId,
        kind: MarketplaceType,
        commission_fee: u8,
        name: TextFormat,
        uri: Option<TextFormat>,
        logo_uri: Option<TextFormat>,
        description: Option<TextFormat>,
    ) -> DispatchResultWithPostInfo;
}
