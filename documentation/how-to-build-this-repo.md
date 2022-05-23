In this guide you will find the instructions on how to build the Ternoa Blockchain client using a Ubuntu like system. For other linux distribution or operating system, check out the official substrate [guide](https://docs.substrate.io/v3/getting-started/installation/) 

```bash
# We need to install all the dependencies.
$ sudo apt-get update
$ sudo apt-get install -y git clang curl libssl-dev llvm libudev-dev make git
# Install cargo and rust.
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ source ~/.cargo/env
# Clone Ternoa Project and cd into the folder.
$ git clone https://github.com/capsule-corp-ternoa/chain.git && cd chain
# If you want to build a specific version of the node, you can switch
# to either latest-mainnet or latest-alphanet branch or use a specific tag.
# git checkout latest-mainnet # or
# git checkout v1.0.0

# run rustup show. This will install all the rust toolchains that we need.
$ rustup show
# Compile the code. This can take from 5 minutes to an hour.
$ cargo build –-release
# Run the binary
$ ./target/release/ternoa --version
```