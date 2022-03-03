FROM ubuntu as builder

LABEL org.opencontainers.image.source http://github.com/capsule-corp-ternoa/chain
LABEL org.opencontainers.image.authors ["eliott@nuclei.studio"]

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config

ADD . .

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    ./scripts/init.sh && \
    cargo build -p ternoa --release && \
    cp target/release/ternoa /ternoa

# ===== SECOND STAGE ======

FROM ubuntu

# curl is used when injecting validator keys
RUN apt-get update && apt-get install -y curl
COPY --from=builder /ternoa /usr/local/bin/ternoa

RUN useradd --create-home runner

USER runner
EXPOSE 30333 9933 9944

ENTRYPOINT ["ternoa"]