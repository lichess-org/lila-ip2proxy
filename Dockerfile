# syntax=docker/dockerfile:1
# Based on https://depot.dev/docs/container-builds/optimal-dockerfiles/rust-dockerfile

FROM rust:1 AS build

RUN cargo install cargo-chef --locked

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src

RUN cargo chef prepare --recipe-path recipe.json

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo chef cook --release --recipe-path recipe.json

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    cargo build --release --bin lila-ip2proxy

FROM debian:trixie-slim AS download

ARG IP2PROXY_FILE=PX2BIN

RUN apt-get update && apt-get install -y --no-install-recommends \
    wget unzip ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY update-ip2proxy.sh /usr/local/bin/update-ip2proxy.sh
RUN chmod +x /usr/local/bin/update-ip2proxy.sh

RUN --mount=type=secret,id=IP2PROXY_TOKEN \
    mkdir -p /data && \
    LILA_IP2PROXY_DATA_DIR=/data \
    LILA_IP2PROXY_UPDATE_TOKEN="$(cat /run/secrets/IP2PROXY_TOKEN)" \
    LILA_IP2PROXY_UPDATE_FILE="$IP2PROXY_FILE" \
    /usr/local/bin/update-ip2proxy.sh

FROM debian:trixie-slim AS runtime

RUN groupadd -g 1001 lichess && \
    useradd -u 1001 -g lichess -m -d /home/lichess -s /bin/bash lichess

COPY --from=build --chown=lichess:lichess /app/target/release/lila-ip2proxy /usr/local/bin/lila-ip2proxy
COPY --from=download --chown=lichess:lichess /data /data

USER lichess

ENV LILA_IP2PROXY_DB=/data/IP2PROXY-IP-PROXYTYPE-COUNTRY.BIN

ENTRYPOINT ["/usr/local/bin/lila-ip2proxy"]
