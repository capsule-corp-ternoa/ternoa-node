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
  - [Environment Setup](#environment-setup)
  - [Dependencies & Building & Running](#dependencies--building--running)
  - [Unit tests](#unit-tests)
  - [Benchmarks](#benchmarks)
  - [Documentation](#documentation)
  - [With docker 🐳](#with-docker-)
- [Usage and testing](#usage-and-testing)
- [Contributing](#contributing)
- [Notes](#notes)


# Development

## Environment Setup

✔️ For Ubuntu

 From a terminal run the following command
```
sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev
```

✔️ For  MacOS
if Homebrew is installed run :

```
brew update && brew install openssl
```
If there is not homebrew

```
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install.sh)"
```
and after 

```
brew install openssl
```

✔️ Install Rust and the Rust toolchain

To install and configure Rust manually:

1) Install rustup by running the following command:

```
curl https://sh.rustup.rs -sSf | sh
```
2) Configure your current shell to reload your **PATH** environment variable so that it includes the Cargo **bin** directory by running the following command:
```
source ~/.cargo/env
```

3) Configure the Rust toolchain to default to the latest stable version by running the following commands:

```
rustup default stable
rustup update
```
4) Add the nightly release and the nightly WebAssembly (wasm) targets by running the following commands:

```
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

5) Verify your installation by running the following commands:

```
rustc --version
rustup show
```

```
The previous steps walked you through the installation and configuration of Rust and the Rust toolchain so that you could see the full process for yourself.

It is also possible to automate the steps using a script.
If you want to try installing and configuring Rust using a script, see the [`getsubstrate`](https://getsubstrate.io) automation script.
```


## Dependencies & Building & Running
Check out the official [Ternoa blockchain Client](https://ternoa-2.gitbook.io/ternoa-blockchain-client-guide) docs :)

## Unit tests
We provide a battery of unit tests which you can run via `cargo test --all --all-features` (omitting `--all-features` would not run a few tests related to benchmarking).

## Benchmarks
In the **scripts** folder we can find benchmark files for each pallet. Benchmark files are named with the following pattern **run_xxx_benchmark.sh** where **xxx** is a tha nameof the pallet.
To run one benchmark file for example **run_associated_accounts_benchmark.sh**, just type in the terminal
```
./scripts/run_associated_accounts_benchmark.sh
```
An the end of the benchmarks execution we can see 
a **rust** file generated at the root of the project with the pallet name for example **ternoa_associated_accounts.rs** wich will the results of the benchmarks. 

## Documentation
You can generate the rust developer documentation via `cargo doc --open`.

## With docker 🐳
**This might be outdated**

Just run `docker build -t netickfr/ternoa-chain .` and then use `docker run --rm -p 9944:9944 -it ternoa/chain` to build and use a containerized node (built with all the optimizations).

Alternatively, you can fetch a prebuilt, bleeding edge container on the [Docker Hub](https://hub.docker.com/r/netickfr/ternoa-chain).

This is how you'd start a local node with docker: `docker run --rm -p 9944:9944 -p 9933:9933 -it netickfr/ternoa-chain --tmp --dev --ws-external --rpc-cors all`.

# Usage and testing
Check out the official [Ternoa blockchain Client](https://ternoa-2.gitbook.io/ternoa-blockchain-client-guide) docs :)

# Contributing

A guideline about contributing to Ternoa chain can be found in the [`CONTRIBUTING.md`](CONTRIBUTING.md) file.

# Notes
- Staking slashes and remainders are burned, you may want to send them to a treasury later.
- We kept the runtime slim and did not include less core pallets such as `recovery`, `utility` or `vesting`.
- weights were computed on an Apple M1 macbook, once validators starts coming in and that we have some reference hardware we will want to recompute those.

# Useful external tools

[Substrate JS utilities](https://www.shawntabrizi.com/substrate-js-utilities/)

[Subwasm](https://github.com/chevdor/subwasm)

# Useful external documentation

[Querying Substrate Storage via RPC](https://www.shawntabrizi.com/substrate/querying-substrate-storage-via-rpc/)