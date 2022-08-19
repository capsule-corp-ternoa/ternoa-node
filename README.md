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
All the examples in this document assume that you use a Ubuntu like system. If that's not the case, you need to change the commands so that it works for your system.

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
  # Get submodules
  git submodule update --init --recursive
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
  podman build -t tchain .
  # Checking if everything is OK.
  podman images | grep tchain
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
  podman run -p 127.0.0.1:9944:9944 -p 127.0.0.1:9933:9933 -p 127.0.0.1:30333:30333 tchain
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

If no command arguments are given by default it will try run the Ternoa Node with default parameters. To cancel this add `bash` at the end of the command. Example: `podman run tsdk bash`;

### Remove Container After Exit
A container that was run and it's job has been finished or the user has exited will not automatically be removed instead it will enter the Exit state.
To make sure that the container is deleted and removed after it's being used the flag `--rm` should be used.

```bash
  # The --rm flag removes the container after usage.
  podman run --rm tchain
  # Stop the container
  [ctrl+c]
  # Check if any container is running or stopped. 
  podman ps -a
```

## Persistent Storage
There are two virtual endpoints that can be mapped to real ones.

### /data
Container uses a local folder to store the chain data. This means that every time a new container is created the chain will start from block 0. To avoid this the container volume `/data` needs to mapped to a directory on the host machine. With this mapping done all the chain data will be stored on the host and it can be used with multiple containers.

```bash
  # This folder will be used to stored ternoa node and chain data.
  mkdir ternoa-data 
  # Flag -v tells the host machine to map the physical "./ternoa-data" path with the virtual container one "/data".
  podman run -v ./ternoa-data:/data tchain
```

### /workdir
Container uses a local copy of the repo in order to compile and run the Ternoa Binary. This means that if code changes are made inside the container that they will not propagate and they will be lost. To change this the virtual container volume `/workdir` needs to be mapped to a directory on the host machine that contains the chain repo. With the mapping done any change in the mapped directory will be visible to the container.

This can be useful if you want to develop your own chain without installing all the dependencies for it. For the workflow check the [Create A Development Environment](#create-a-development-environment) segment.
```bash
  # This folder will be used to stored ternoa node and chain data.
  mkdir ternoa-data 
  # Flag -v tells the host machine to map the physical "./ternoa-data" path with the virtual container one "/data".
  podman run -v ./.:/workdir tchain
```


## Run The Container And Access Its Shell
The predefined operation/command of the container when run is to run the Ternoa Node with the alphanet-dev configuration. To execute a different operation additional commands can be passed at the end of the run command. Example: passing `bash` will run the bash shell session instead the default operation.

```bash
  # If no command arguments are given this will try to run the Ternoa Node with default parameters.
  # By passing "bash" we make sure that we run a bash shell session once the container starts.
  podman run -it tchain bash
```

## Create A Detached Instance And Access Its Shell
```bash
  # Flag "-d" runs the container in detached mode. 
  podman run -d tchain
  # Access its shell.
  podman exec -itl bash
```

### Create A Development Environment
The dockerfile is made in a way that it can be used to develope new applications with it.
Example of a typical workflows:
- The host installs git, clones the repo and install a code editor like VS Code.
- The host runs the container in a interactive mode with /workdir pointing to a workdir on host machine (can be your own project or chain repo).
- The host writes code via a code editor and uses the terminal (which is connected to the container) to run the `cargo build` and `cargo check` commands.
- With that setup all the changes are done locally on the host machine while the container is only used to compile and run the chain.

```bash
  mkdir ternoa-data 
  # Flag "--name" is used to name the container.
  podman run -it --name my_chain_env -v ./ternoa-data:/data -v ./.:/workdir tchain bash
  # Do some activity and the exit the container
  [root@d4ad8ec11655:/workdir] nano -V
  [root@d4ad8ec11655:/workdir] apt install nano
  [root@d4ad8ec11655:/workdir] exit

  # Return to the same container
  podman start my_chain_env
  podman exec -it my_chain_env /bin/bash
  [root@d4ad8ec11655:/workdir] nano -V
```


# Useful tools
[Substrate JS utilities](https://www.shawntabrizi.com/substrate-js-utilities/)

[Subwasm](https://github.com/chevdor/subwasm)

[Querying Substrate Storage via RPC](https://www.shawntabrizi.com/substrate/querying-substrate-storage-via-rpc/)