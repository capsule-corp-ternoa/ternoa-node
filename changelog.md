# 0.2.1 Release
This is the second release for the Ternoa Testnet.
Two new modules have been added :
 - capsule for creating capsules from an NFT
 - associated-accounts for linking third party accounts to main account
Also a marketplaces can now have a link to a website and also a link for the marketplace's logo

# 🚀 New Features
 - Add a link to a marketplace website
 - Link third party accounts with your main account

# Added
- new pallet: capsules - enables you to create a capsule out of a NFT
- new pallet: associated-accounts - enables you to link third party accounts with your main account
- new capsules extrinsic: create - enables you to create a capsule
- new capsules extrinsic: create_from_nft - enables you turn an nft to a capsule
- new capsules extrinsic: remove - Converts back a capsule into nft
- new capsules extrinsic: add_funds - enables you to add additional funds to a capsule
- new capsules extrinsic: set_ipfs_reference - enables you to change a capsule's ipfs reference.
- new capsules extrinsic: set_capsule_mint_fee - enables you to set a capsule mint fee.
- new associated-accounts extrinsic: set_altvr_username - enables you set an altvr username
- new marketplace extrinsic: set_uri - Allows you to store the link to your marketpklace
- new marketplace extrinsic: set_logo_uri - Allows you to store the link of your marketpklace logo
- new marketplace extrinsic: add_account_to_disallow_list - enables you add an account to the disallow_list to buy from public marketplaces
- new marketplace extrinsic: remove_account_from_disallow_list - enables you remove an account from the disallow_list

# Changed
- spec_41.json
- uri and logo_uri properties added to marketplace information  **(marketplace)**

# 0.2.XX-dev
## Specification 41 - 2021-10-29 TODO
# 🚀 New Features
- NFT series can now be in two states, drafted and completed. 
- If series id is not specified when an NFT is created, the blockchain will generate and use a random series id. #
# Added
- set_commission_fee marketplace extrinsic
- set_ipfs_reference nfts extrinsic
- associated-accounts pallet
# Changed
- Series Id is now a string type instead of a number.
- NFT Mint Fee and Marketplace Mint Fee are not changeable.
- Created types folder

# 0.2.0 Release
This is the initial release for the Ternoa Testnet.
Most notably, the marketplace was extended to allow anyone to create their own
marketplace. The marketplace can either be Public or Private and the owner can
set a commission fee so that he can get a cut of every nft sold on his
marketplace. 

# 🚀 New Features
- Marketplaces can be created by users if they have enough funds. 
- Gitbook with tutorial on how to use the Ternoa blockchain client.
# ⛓️ Updates
- Updated to substrate version 0.4.0-dev #5be50ac14b23147c6f120745c2205a86a2675169

# 0.1.XX-dev
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