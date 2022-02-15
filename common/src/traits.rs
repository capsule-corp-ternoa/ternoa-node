use frame_support::dispatch::{DispatchErrorWithPostInfo, DispatchResult};
use ternoa_primitives::marketplace::{MarketplaceId, MarketplaceInformation, MarketplaceType};
use ternoa_primitives::nfts::{NFTData, NFTId, NFTSeriesId};
use ternoa_primitives::TextFormat;

pub trait NFTTrait {
    type AccountId: Clone;

    /// Change the owner of an NFT.
    fn set_owner(id: NFTId, owner: &Self::AccountId) -> DispatchResult;

    /// Return the owner of an NFT.
    fn owner(id: NFTId) -> Option<Self::AccountId>;

    /// Is series completed(locked)
    fn is_nft_in_completed_series(id: NFTId) -> Option<bool>;

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

    /// Set a series to be either completed or not-completed.
    fn set_series_completion(series_id: &NFTSeriesId, value: bool) -> DispatchResult;

    /// Set the NFT viewer to a value.
    fn set_viewer(id: NFTId, value: Option<Self::AccountId>) -> DispatchResult;
}

/// Trait that implements basic functionalities related to Ternoa Marketplace
/// TODO: Expand trait with more useful functions
pub trait MarketplaceTrait<AccountId> {
    /// Return if an account is permitted to list on given marketplace
    fn is_allowed_to_list(marketplace_id: MarketplaceId, account_id: AccountId) -> DispatchResult;

    /// Return marketplace
    fn get_marketplace(marketplace_id: MarketplaceId) -> Option<MarketplaceInformation<AccountId>>;

    /// create a new marketplace
    fn create(
        origin: AccountId,
        kind: MarketplaceType,
        commission_fee: u8,
        name: TextFormat,
        uri: Option<TextFormat>,
        logo_uri: Option<TextFormat>,
        description: Option<TextFormat>,
    ) -> Result<MarketplaceId, DispatchErrorWithPostInfo>;
}
