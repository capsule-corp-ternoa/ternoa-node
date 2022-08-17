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

Welcome to the Ternoa Blockchain repo which hosts the code used to build and run the Ternoa node.
Ternoa supports the transfer of arbitrary data to your descendants, friends and loved ones even after your death or disappearence or a given timed period in a **non custodial**, cryptographically enforced manner.

</br>

Table of Contents:

- [Build](#build)
  - [Build Locally](#build-locally)
  - [Build With Podman](#build-with-podman)
- [Run](#run)
  - [Run Locally](#run-locally)
  - [Run With Podman](#run-with-podman)
  - [Run With Provided Binary](#run-with-provided-binary)
- [Running Benchmarks](#running-benchmarks)
- [Running Unit Tests](#running-unit-tests)
- [Generating Reference Documentation](#generating-reference-documentation)
- [Running With Podman Tips](#running-with-podman-tips)
  - [Permanent Storage](#permanent-storage)
  - [Run The Container And Access Its Shell](#run-the-container-and-access-its-shell)
  - [Create A Detached Instance And Access Its Shell](#create-a-detached-instance-and-access-its-shell)
- [Useful tools](#useful-tools)

# Build
All the examples in this document assume that you use a Ubuntu like system. If that's not the case, you need to modify the code so that it works for your system.

## Build Locally
```bash
  # Downloads the package lists and "updates" them.
  sudo apt update -y
  # Installing all dependencies (but not Rust).
  sudo apt install build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler -y
  # Installing Rust.
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  # Starting a new bash environment so we have access to cargo and rust commands.
  exec bash
  # Updating Rust to latest versions and installing the right Rust version.
  rustup update && rustup show
  # Building the Ternoa Binary.
  cargo build --locked --release
  # Checking if everything is OK. 
  ./target/release/ternoa -V
```

## Build With Podman
```bash
  # Downloads the package lists and "updates" them.
  sudo apt update -y
  # Installing podman.
  sudo apt install podman
  # Building the image using podman and the already available Dockerfile.
  podman build -t ternoaimage .
  # Checking if everything is OK.
  podman images | grep ternoaimage
```

# Run
Node flag explanation:
- `--chain alphanet-dev`: There are a couple of chain configurations that we provide and each configuration have a drastic impact on how the chain behaves and what features it has. For testing purposes it's best to stick with alphanet-dev configuration.
- `--alice`: This sets a couple of flags for us. It sets the `--validator` flag so that the client is running in a validator mode, it makes Alice a validator and it inserts Alice's keys into the local keystore.
- `--tmp`: Makes is so that the blockchain data is stored in a temporary location. Usually this data is deleted on reboot.
- `--name MyLocalNode`: Sets the name of the name. This should be something unique.
- `--rpc-external`: Listen to all RPC interfaces. This should be used only during testing.
- `--ws-external`: Listen to all Websocket interfaces. This should be used only during testing.
- `--rpc-cors all`: Specifies browser Origins allowed to access the HTTP && WS RPC servers. This should be used only during testing.
- `--telemetry-url "wss://telemetry.polkadot.io/submit/ 0"`: Tells the node to send node telemetry data to `telemetry.polkadot.io`.

Podman flag explanation:
- `-p 127.0.0.1:9944:9944`: Maps host `127.0.0.1:9944` address:port to container `9944` port. This is the Websocket traffic port.
- `-p 127.0.0.1:9933:9933`: Maps host `127.0.0.1:9933` address:port to container `9933` port. This is the RPC traffic port.
- `-p 127.0.0.1:30333:30333`: Maps host `127.0.0.1:30333` address:port to container `30333` port. This is the P2P port.

## Run Locally
```bash
  # Make sure that you have built a binary from the "Build Locally" step.
  ./target/release/ternoa --chain alphanet-dev --alice --tmp --name MyLocalNode --rpc-external --ws-external --rpc-cors all --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

## Run With Podman
```bash
  # Make sure that you have built a image from the "Build With Podman" step.
  podman run -p 127.0.0.1:9944:9944 -p 127.0.0.1:9933:9933 -p 127.0.0.1:30333:30333 ternoaimage
```

## Run With Provided Binary
Depending on what binary you downloaded certain features might not be available in to use. To get the latest features get the latest binary. In this example the oldest binary is being used.
```bash
  # Getting the binary from github.
  wget https://github.com/capsule-corp-ternoa/chain/releases/download/v1.0.0/ternoa
  # Makes the binary executable
  chmod u+x ternoa
  # Runs the chain
  ./ternoa --chain alphanet-dev --alice --tmp --name MyLocalNode --rpc-external --ws-external --rpc-cors all --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

# Running Benchmarks
```bash
  # Building the Ternoa Binary.
  cargo build --locked --release --features runtime-benchmarks
  # Run the benchmarks for the balances pallet.
  ./target/release/ternoa benchmark pallet --chain alphanet-dev --steps=50 --repeat=20 --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./weights/ --pallet=pallet_balances
```

# Running Unit Tests
```bash
  # It's important to not omit the "--all-features" flag otherwise not all test will run.
  cargo test --all --all-features
```

# Generating Reference Documentation
```bash
  # While compiling it might display some warning that can be safely ignored.
  cargo doc --open
```

# Running With Podman Tips
In the next examples some useful Podman commands will be shown. It's important to note that most flags have been omitted in order to make the examples more concise. Before running anything make sure that the image was built from the the "Build With Podman" step.

## Permanent Storage
```bash
  # This folder will be used to stored ternoa node and chain data.
  mkdir ternoa-data 
  # Flag -v tells the host machine to map the physical "./ternoa-data" path with the virtual container one "/data".
  podman run -v ./ternoa-data:/data ternoaimage
```

## Run The Container And Access Its Shell
```bash
  # The default entry point is running the Ternoa binary. Here we manually force a new entry point which will allow use to directly land into a bash shell. 
  podman run -it --entrypoint=bash tchain
```

## Create A Detached Instance And Access Its Shell
```bash
  # Flag "-d" runs the container in detached mode. 
  podman run -d tchain
  # Access its shell.
  podman exec -itl bash
```


# Useful tools
[Substrate JS utilities](https://www.shawntabrizi.com/substrate-js-utilities/)

[Subwasm](https://github.com/chevdor/subwasm)

[Querying Substrate Storage via RPC](https://www.shawntabrizi.com/substrate/querying-substrate-storage-via-rpc/)