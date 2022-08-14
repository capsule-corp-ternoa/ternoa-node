# This is the first stage. Here we install all the dependencies that we need in order to build the Ternoa binary.
FROM ubuntu:22.10 as builder

ADD . ./chain
WORKDIR "/chain"

# This installs all dependencies that we need (besides Rust).
RUN apt update -y && apt upgrade -y  && \
    apt install build-essential git clang curl libssl-dev llvm libudev-dev make protobuf-compiler -y

# This installs Rust and updates Rust to the right version.
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rust_install.sh && chmod u+x rust_install.sh && ./rust_install.sh -y && \
    . $HOME/.cargo/env && rustup update && rustup show

# This builds the binary.
RUN $HOME/.cargo/bin/cargo build --locked --release

# This is the second stage. Here we copy the built Ternoa binary to a new minimal image.
FROM ubuntu:22.10 as runner

COPY --from=builder /chain/target/release/ternoa /usr/local/bin

# Taken from Parity. TODO Checkout what it does.
RUN useradd -m -u 1000 -U -s /bin/sh -d /ternoa ternoa && \
    mkdir -p /data /ternoa/.local/share && \
    chown -R ternoa:ternoa /data && \
    ln -s /data /ternoa/.local/share/ternoa && \
    # unclutter and minimize the attack surface
    rm -rf /usr/bin /usr/sbin && \
    # check if executable works in this container
    /usr/local/bin/ternoa --version

USER ternoa

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/ternoa"]
CMD ["--chain", "alphanet-dev", "--alice", "--tmp", "--name", "MyDockerNode"]