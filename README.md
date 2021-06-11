<h1 align="center">
    Ternoa Blockchain 🚀
</h1>


<h3 align="center">
  <a href="https://www.ternoa.com/">Website</a>
  <span> · </span>
  <a href="https://github.com/capsule-corp-ternoa/white-paper/blob/main/white-paper-en.md">White paper</a>
  <span> · </span>
  <a href="https://medium.com/ternoa">Blog</a>
  <span> · </span>
  <a href="https://twitter.com/ternoa_">Twitter</a>
  <span> · </span>
  <a href="https://discord.gg/cNZTGtGJNR">Discord</a>
</h3>

Welcome to the Ternoa Blockchain repo which hosts the code used to build and run a Ternoa node.
Ternoa supports the transfer of arbitrary data to your descendants, friends and loved ones even after your death or disappearence or a given timed period in a **non custodial**, cryptographically enforced manner.

</br>

Table of Contents:

- [Development](#development)
  - [Dependencies](#dependencies)
  - [Building](#building)
  - [Running](#running)
  - [Unit tests](#unit-tests)
  - [Documentation](#documentation)
  - [With docker 🐳](#with-docker-)
- [Usage and testing](#usage-and-testing)
  - [Setting up the node](#setting-up-the-node)
  - [Creating a capsule](#creating-a-capsule)
  - [Escrow a capsule](#escrow-a-capsule)
- [Contributing](#contributing)
- [Notes](#notes)


# Development

## Dependencies
The Ternoa node is built on [Parity Substrate](https://www.substrate.io) a framework to build robust, next generation blockchain solutions. In order to compile the node you will need to setup a few things on your machine (or use docker, see below):
1. You need to install a few packages and libraries:
   - On mac, you will need to make sure `openssl` and `cmake` are installed, with [brew](https://brew.sh) you can do a simple `brew openssl cmake`.
   - On linux you need to install a few more packages, for debian and derivatives use `sudo apt install cmake pkg-config libssl-dev git build-essential clang libclang-dev curl`.
2. You need to install rust, the best way to do so is via [rustup](https://rustup.rs).
3. You need to configure rust to support Web Assembly compilation, we provide a script to do so, just run it via `./scripts/init.sh`.

## Building
The fastest way to build the node is to run `cargo build` at the root of the repo, however this will build it with debugging symbols and some optimizations disabled. If you'd prefer a non debugging and optimized build you can run `cargo build --release`.

## Running
After those steps you can use either of `cargo run`, `cargo run --release`, `./target/debug/ternoa` or `./target/release/ternoa` to start a node. We provide examples later in this document.

## Unit tests
We provide a battery of unit tests which you can run via `cargo test --all --all-features` (omitting `--all-features` would not run a few tests related to benchmarking).

## Documentation
You can generate the rust developer documentation via `cargo doc --open`.

## With docker 🐳
Just run `docker build -t netickfr/ternoa-chain .` and then use `docker run --rm -p 9944:9944 -it ternoa/chain` to build and use a containerized node (built with all the optimizations).

Alternatively, you can fetch a prebuilt, bleeding edge container on the [Docker Hub](https://hub.docker.com/r/netickfr/ternoa-chain).

This is how you'd start a local node with docker: `docker run --rm -p 9944:9944 -p 9933:9933 -it netickfr/ternoa-chain --tmp --dev --ws-external --rpc-cors all`.

# Usage and testing
## Setting up the node
First, you need to run a node:
1. Run an ephemeral, development node: `cargo run --release -- --dev --tmp`.
2. Connect to the [Polkadot Web UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), navigate to `Settings`, `Development` and copy paste in the content of the [`types.json` file](./types.json). Click `Apply` and reload.

> **What are you seeing**: we are running a chain configured for development and testing. There some preprovisioned test accounts and one authority with some staked coins creating the blocks.

## Creating a capsule
Now, let's create a capsule:
1. From the navigation menu click on `Developer` and then `Extrinsics`.
2. Use the `Alice` account as prefilled, replace `system` with `nfts` and replace the default filled `burn()` method with `create()`
3. Choose a few parameters:
   - `offchain_uri`: Ascii encoded URI that contains additional metadata. For testing purpose this can be set to `0x00` .
   - `series_id`: Series id allows users to group their nfts into a collection. The default value is sufficient for this example.
   - `is_capsule`: If set to `Yes`, the system will treat the nft as a capsule. Set this to `Yes`.
4. Click `Apply` and `Sing and Submit` the transaction.
5. You can now go to `Network` and `Explorer` and see that an event `nfts.Created` was triggered. Your capsule has ID `0`.
6. You can also view at any time the state of the capsule by doing the following:
   1. Go to `Developers` and then `Chain State`.
   2. Select `nfts` and `data(NFTId): NFTData`.
   3. Give it an ID, in our case it is `0`.
   4. Click the big round `+` button and see the data pop up.

> The `nfts` pallet features a few more methods for manipulating with capsules, play around with them!

## Escrow a capsule
Now that we have created a capsules let's escrow it to our descendants:
1. Go back to the extrinsics tab but select `timedEscrow` and `create` this time.
2. Set `nft_id` to `0`, set parameter `to` to `BOB` and parameter `at` to current block number + 10;
3. Set the parameters to `1`, `Bob` and the current block number + 10.
4. Submit the transaction.
5. An event `timedEscrow.TransferScheduled` is triggered in the network tab.
6. Ten blocks later or so you should see the events `timedEscrow.TransferCompleted` indicating that the capsule has a new owner. This should be reflected in the chain state as well.

> the `timedEscrow` pallet will also let you cancel a transfer if it didn't happen yet.

# Contributing

A guideline about contributing to Ternoa chain can be found in the [`CONTRIBUTING.md`](CONTRIBUTING.md) file.

# Notes
- Staking slashes and remainders are burned, you may want to send them to a treasury later.
- We kept the runtime slim and did not include less core pallets such as `recovery`, `utility` or `vesting`.
- weights were computed on an Apple M1 macbook, once validators starts coming in and that we have some reference hardware we will want to recompute those.