FROM ubuntu as builder

LABEL maintainer="eliott@nuclei.studio"

ARG PROFILE=release
ARG BINARY_NAME=ternoa
ARG PACKAGE_NAME=ternoa

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -y \
    clang \
    cmake \
    curl \
    git \
    libssl-dev \
    pkg-config

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y && \
    export PATH=$PATH:$HOME/.cargo/bin && \
    ./scripts/init.sh && \
    cargo build -p ${PACKAGE_NAME} --${PROFILE} && \
    cp target/${PROFILE}/${BINARY_NAME} /node

# ===== SECOND STAGE ======

FROM ubuntu

COPY --from=builder /node /usr/local/bin

RUN useradd --create-home runner

USER runner
EXPOSE 30333 9933 9944

ENTRYPOINT ["node"]