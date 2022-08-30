# This is the first stage. Here we install all the dependencies that we need in order to build the Ternoa binary.
FROM ubuntu:22.04 as builder

ADD ./ ./workdir
WORKDIR "/workdir"

# This installs all dependencies that we need (besides Rust).
RUN apt update -y && \
    apt install build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler -y

# This installs Rust and updates Rust to the right version.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y && \
    . $HOME/.cargo/env && rustup update && rustup show

# Get all submodules
RUN git submodule update --init --recursive

# Create Output folder
RUN mkdir -p output

VOLUME ["/workdir"]
VOLUME ["/output"]
