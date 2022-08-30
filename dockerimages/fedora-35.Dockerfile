# This is the first stage. Here we install all the dependencies that we need in order to build the Ternoa binary.
FROM fedora:35 as builder

ADD . ./workdir
WORKDIR "/workdir"

# This installs all dependencies that we need (besides Rust).
RUN dnf update -y && \
    dnf install git clang curl make protobuf-compiler -y

# This installs Rust and updates Rust to the right version.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y && \
    . $HOME/.cargo/env && rustup update && rustup show

# Get all submodules
RUN git submodule update --init --recursive

# This builds the binary.
RUN $HOME/.cargo/bin/cargo build --locked --release

# Create output folder
RUN mkdir -p output

VOLUME ["/output"]

CMD cp ./target/release/ternoa /output