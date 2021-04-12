# Ternoa Blockchain

Welcome to the Ternoa Blockchain repo which hosts the code used to build and run a Ternoa node.
Ternoa supports the transfer of arbitrary data to your descendants, friends and loved ones even after your death or disappearence or a given timed period in a **non custodial**, cryptographically enforced manner.

## Developement

### Dependencies
The Ternoa node is built on [Parity Substrate](https://www.substrate.io) a framework to build robust, next generation blockchain solutions. In order to compile the node you will need to setup a few things on your machine (or use docker, see below):
1. You need to install a few packages and libraries:
   - On mac, you will need to make sure `openssl` and `cmake` are installed, with [brew](https://brew.sh) you can do a simple `brew openssl cmake`.
   - On linux you need to install a few more packages, for debian and derivatives use `sudo apt install cmake pkg-config libssl-dev git build-essential clang libclang-dev curl`.
2. You need to install rust, the best way to do so is via [rustup](https://rustup.rs). We use a specific toolchain version which the rust tools will autodetect and fetch when needed via the [`rust-toolchain` file](./rust-toolchain).
3. You need to configure rust to support Web Assembly compilation, we provide a script to do so, just run it via `./scripts/init.sh`.

### Building
The fastest way to build the node is to run `cargo build` at the root of the repo, however this will build it with debugging symbols and some optimizations disabled. If you'd prefer a non debugging and optimized build you can run `cargo build --release`.

### Running
After those steps you can use either of `cargo run`, `cargo run --release`, `./target/debug/ternoa` or `./target/release/ternoa` to start a node. We provide examples later in this document.

### Unit tests
We provide a battery of unit tests which you can run via `cargo test --all --all-features` (omitting `--all-features` would not run a few tests related to benchmarking).

### Documentation
You can generate the rust developer documentation via `cargo doc --open`.

### With docker 🐳
Just run `docker build -t netickfr/ternoa-chain .` and then use `docker run --rm -p 9944:9944 -it ternoa/chain` to build and use a containerized node (built with all the optimizations).

Alternatively, you can fetch a prebuilt, bleeding edge container on the [Docker Hub](https://hub.docker.com/r/netickfr/ternoa-chain).

This is how you'd start a local node with docker: `docker run --rm -p 9944:9944 -p 9933:9933 -it netickfr/ternoa-chain --tmp --dev --ws-external --rpc-cors all`.

### Usage and testing
First, you need to run a node:
1. Run an ephemeral, development node: `cargo run --release -- --dev --tmp`.
2. Connect to the [Polkadot Web UI](https://polkadot.js.org/apps/?rpc=ws%3A%2F%2F127.0.0.1%3A9944#/explorer), navigate to `Settings`, `Development` and copy paste in the content of the [`types.json` file](./types.json). Click `Apply` and reload.

> **What are you seeing**: we are running a chain configured for development and testing. There some preprovisioned test accounts and one authority with some staked coins creating the blocks.

Now, let's create a capsule:
1. Go to `Developers` and then `Extrinsics`.
2. Use the `Alice` account as prefilled, replace `system` with `capsules`.
3. Choose a few parameters:
   - `offchain_uri` is a ascii encoded string, you can fill it with garbage for testing, we use `0xdeadbeef`.
   - `pk_hash` is a hash of the public key used to encode the capsule data, since it is not relevant to the pallet logic itself leave it at `0x0000000000000000000000000000000000000000000000000000000000000000`.
   - keep `creator` and `owner` set to `Alice`, the pallet will enforce this.
   - set `locked` to `false`, this will be used later when sending capsules.
4. Click `Apply` and submit the transaction.
5. You can now go to `Network` and `Explorer` and see that an event `capsules.CapsuleCreated` was triggered. Your capsule has ID `1`.
6. You can also view at any time the state of the capsule by doing the following:
   1. Go to `Developers` and then `Chain State`.
   2. Select `capsules` and `metadata`.
   3. Give it an ID, in our case it is `1`.
   4. Click the big round `+` button and see the data pop up.

> The `capsules` pallet features two more functions for transfering and updating a capsule, play around with them!

Now that we have created a capsules let's escrow it to our descendants:
1. Go back to the extrinsics tab but select `timedEscrow` and `create` this time.
2. Set the parameters to `1`, `Bob` and the current block number + 10.
3. Submit the transaction.
4. An event `timedEscrow.TransferScheduled` is triggered in the network tab.
5. Ten blocks later or so you should see the events `timedEscrow.TransferCompleted` and `capsules.CapsuleTransferred` indicating that the capsule has a new owner. This should be reflected in the chain state as well.

> the `timedEscrow` pallet will also let you cancel a transfer if it didn't happen yet.

## Notes
- Staking slashes and remainders are burned, you may want to send them to a treasury later.
- There are no collectives, treasuries or democracy pallets.
- We kept the runtime slim and did not include less core pallets such as `recovery`, `utility` or `vesting`.
- In local and dev chains we have configured `Alice` to have access to the `sudo` pallet which we added to test runtime migrations and upgrades.
- we do not charge for creating a capsule, this may be wanted or not in the future for tokeneconomics purposes.
- weights were computed on an Apple M1 macbook, once validators starts coming in and that we have some reference hardware we will want to recompute those.