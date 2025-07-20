#!/usr/bin/env bash

set -exo pipefail

export BASE_URL="http://127.0.0.1:8080"

# Requires nightly toolchain, rust-src component, and wasm32-unknown-unkown target
# rustup update nightly && rustup default nightly
# rustup component add rust-src
# rustup target add wasm32-unknown-unknown

cp ./.cargo/config.toml ./.cargo/config.toml.backup
cp ./.cargo/config_wasm.toml ./.cargo/config.toml
trap "mv ./.cargo/config.toml.backup ./.cargo/config.toml" EXIT

trunk serve index.html --release --features=dev-panel,dynamically-load-game-data --watch .gitignore --no-autoreload
