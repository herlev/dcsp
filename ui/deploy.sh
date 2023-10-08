#!/usr/bin/env bash

# rustup target add x86_64-unknown-linux-musl
set -euox pipefail

HOST="$1"

cargo build --target x86_64-unknown-linux-musl --release
ssh "$HOST" rm ui
scp ../target/x86_64-unknown-linux-musl/release/ui "$HOST":
ssh -t "$HOST" doas rc-service docker-compose-ui restart