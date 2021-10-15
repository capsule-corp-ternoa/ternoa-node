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
  - [Dependencies & Building & Running](#dependencies--building--running)
  - [Unit tests](#unit-tests)
  - [Documentation](#documentation)
  - [With docker 🐳](#with-docker-)
- [Usage and testing](#usage-and-testing)
- [Contributing](#contributing)
- [Notes](#notes)


# Development

## Dependencies & Building & Running
Check out the official [Ternoa blockchain Client](https://ternoa-2.gitbook.io/ternoa-blockchain-client-guide) docs :)

## Unit tests
We provide a battery of unit tests which you can run via `cargo test --all --all-features` (omitting `--all-features` would not run a few tests related to benchmarking).

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