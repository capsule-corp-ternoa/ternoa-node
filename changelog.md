# 0.2.XX-dev
## Specification 41 - 2021-10-29 TODO
# 🚀 New Features
- NFT series can now be in two states, drafted and completed. 
- If series id is not specified when an NFT is created, the blockchain will generate and use a random series id. 
# Changed
- Series Id is now a string type instead of a number.

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