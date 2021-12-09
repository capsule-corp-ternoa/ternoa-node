# 🚀 0.2.2 Release
Replaced faulty ternoa::String type with TernoaString


# 🚀 0.2.1 Release
This is the second release for the Ternoa Testnet.
This release incorporates two large features, a couple of small improvements and a large list of fixes.
Most notably, you can now convert your NFT to a Capsule and associate your third-party accounts with
your public address.

## NFTs pallet
The NFTs data has been streamlined and it now holds only the bare minimum information that it needs.
This is how it looks now:

```rust
struct NFTData {
    owner: AccountId,
    ipfs_reference: TernoaString,
    series_id: NFTSeriesId,
    locked: bool,
}
```

The way how series work has been changed. A series can now be in two state: draft or non-draft state.
Series that is in draft state allows adding NFTs to it but those NFTs cannot be listed for sale nor transferred.
Series that is in non-draft state doesn't allow adding new NFTs to it but those NFTs can now be sold and transferred.
Another change is that the Series Id is not anymore a number and now it's a string. In case the user doesn't provide
a series id when he is creating an NFT the blockchain will generate one for him.
This is how the series data looks now:

```rust
    struct NFTSeriesDetails {
        owner: AccountId,
        draft: bool, 
    }
```

NFT mint fee is not any more static and it can be changed through government proposals.

List of all changes:
- NFTData structure has been changed
- NFTSeriesDetails structure has been changed
- SeriesId is now a string
- Added min and max characters constraints for ipfs_reference values
- NFTs cannot be burned if they are converted to capsules
- NFTs cannot be transferred if their series is not in non-draft state
- Constant MintFee was removed ❌
- Extrinsic create changed its interface 💡 
- Extrinsic finish_series was added ✨
- Extrinsic set_nft_mint_fee was added ✨
- Extrinsic set_ipfs_reference was added ✨
- Extrinsic seal was removed ❌
- Extrinsic mutate was removed ❌
- Event SeriesFinished, NftMintFeeChanged and IpfsReferenceChanged were added ✨
- Event SeriesTransfer was removed ❌
- Error SeriesIsInDraft, SeriesIsCompleted, SeriesNotFound, InvalidNFTId, TooShortIpfsReference, TooLongIpfsReference and NFTIsCapsulized were added ✨
- Storage data now returns None instead of a default value on non-existing keys 💡
- Storage SeriesIdGenerator was added ✨
- Storage NftMintFee was added ✨
- Genesis nft_mint_fee was added ✨
- Trait NFTs was moved and streamlined 💡 

## Marketplace pallet
Marketplace data has been extended and now contains link to the marketplace website and a link to the location of the marketplace logo. Just like the `allow_list` is used for enabling only certain users to list their nfts, we have added `disallow_list` to block certain users to list their nfts. `allow_list` can only be used by private marketplaces while `disallow_list` can only be used by public marketplaces.

This is how the new marketplace data looks like:
```rust
struct MarketplaceInformation {
    kind: MarketplaceType,
    commission_fee: u8,
    owner: AccountId,
    allow_list: Vec<AccountId>,
    disallow_list: Vec<AccountId>,
    name: TernoaString,
    uri: Option<URI>,
    logo_uri: Option<URI>,
}
```

Marketplace mint fee is not any more static and it can be changed through government proposals.

List of all changes:
- MarketplaceInformation structure has been changed
- NFTs cannot be listed if they are convert to capsules
- Added min and max characters constraints for marketplace name
- Constant MarketplaceFee was removed ❌
- Extrinsic create changed its interface 💡
- Extrinsic change_owner was renamed to set_owner 💡
- Extrinsic change_market_type was renamed to set_market_type 💡
- Extrinsic add_account_to_disallow_list was added ✨
- Extrinsic remove_account_from_disallow_list was added ✨
- Extrinsic set_marketplace_mint_fee was added ✨
- Extrinsic set_commission_fee was added ✨
- Extrinsic set_uri was added ✨
- Extrinsic set_logo_uri was added ✨
- Event AccountAddedToMarketplace was renamed to AccountAddedToAllowList 💡
- Event AccountRemovedFromMarketplace was renamed to AccountRemovedFromAllowList 💡
- Event MarketplaceMintFeeChanged, MarketplaceCommissionFeeChanged, MarketplaceUriUpdated, MarketplaceLogoUriUpdated, AccountAddedToDisallowList and AccountRemovedFromDisallowList were added ✨
- Error TooShortName was renamed to TooShortMarketplaceName 💡
- Error TooLongName was renamed to TooLongMarketplaceName 💡
- Error SeriesNotCompleted, TooLongMarketplaceUri, TooShortMarketplaceUri, TooLongMarketplaceLogoUri, TooShortMarketplaceLogoUri, NFTIsCapsulized were added ✨
- Storage NFTsForSale now returns None instead of a default value on non-existing keys 💡
- Storage MarketplaceMintFee was added ✨
- Genesis marketplace_mint_fee was added ✨

## Capsules pallet
Brand new pallet that handles capsule like data for NFTs. When capsules are created no separate entity is crated and instead we use existing NFTs and add capsule specific data to it. Also, the caller needs to freeze 1000 Caps when he creates a capsule and he can add later additional funds if he wants. The frozen funds are send to the pallets address and there is a ledger that keeps track on how much each user has frozen Caps. Those frozen caps will in the future be used to pay fees for storing data in third party solutions. The user can remove the capsule data from an NFT and in that case he will get all the remaning frozen Caps back.

The Capsule pallet offer the following features:
-  Extrinsic create -> Creates an NFT and then it capsulizes it
-  Extrinsic create_from_nft -> Capsulizes an existing NFT
-  Extrinsic remove -> Removes capsule data from an NFT and returns the staked caps
-  Extrinsic add_funds -> Adds additional funds to stake
-  Extrinsic set_ipfs_reference -> Changes the capsule ipfs reference
-  Extrinsic set_capsule_mint_fee -> Sets the capsule mint fee
-  Event CapsuleIpfsReferenceChanged, CapsuleFundsAdded, CapsuleRemoved, CapsuleCreated, CapsuleMintFeeChanged, CapsuleDeposit
-  Error ArithmeticError, NotOwner, TooShortIpfsReference, TooLongIpfsReference, CapsuleAlreadyExists, InternalError, NftLocked
-  Storage CapsuleMintFee, Capsules, Ledgers
-  Genesis capsule_mint_fee, capsules, ledgers
-  Trait CapsulesTrait

## Associated-Accounts pallet
Brand new pallet that handles third party accounts. Currently, it only handles storing AltVR usernames but in the future many more services will be stored and mapped to user accounts.

The Associated-Accounts pallet offer the following features:
-  Extrinsic set_altvr_username -> Sets AltVR username
-  Event AltVRUsernameChanged
-  Error TooShortUsername, TooLongUsername
-  Storage AltVRUsers
-  Genesis altvr_users

## Other changes
- We have completely rewritten how we do our tests, storage migration and benchmarking
- We moved our types file to a separate folder where we now track multiple types files each associated with a sigle chain specification version

# 🚀 0.2.0 Release
This is the initial release for the Ternoa Testnet.
Most notably, the marketplace was extended to allow anyone to create their own
marketplace. The marketplace can either be Public or Private and the owner can
set a commission fee so that he can get a cut of every nft sold on his
marketplace. 

# New Features
- Marketplaces can be created by users if they have enough funds. 
- Gitbook with tutorial on how to use the Ternoa blockchain client.
# ⛓️ Updates
- Updated to substrate version 0.4.0-dev #5be50ac14b23147c6f120745c2205a86a2675169

# 🚀 0.1.XX-dev
## Specification 40 - 2021-10-03
### Added
- Sgx to the chain genesis
- Added validator-net, staging-net, test-net and main-net chain specification
### Changed
- You don't need to be anymore on the allow list to buy from private marketplaces
### Removed
- Removed chaos, dev-remote and staging chain specification

## Specification 39 - 2021-09-27
### Changed
- types.json
- Changed Create NFT Event. It it also contains the Offchain URI

## Specification 38 - 2021-09-20
### Added
- Created and added pallet `ternoa-sgx`
### Changed
- types.json
- ternoa-nfts weights
### Removed
- Removed pallet `pallet-substratee-registry`

## Specification 37 - 2021-09-11
Skipped

## Specification 36 - 2021-09-10
### Added
- Marketplaces now have names (Storage migration needed)
- `set_name` extrinsic to marketplace pallet
### Changed
- types.json