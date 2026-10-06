#!/bin/sh
set -eu
cd "$(dirname "$0")"
mkdir -p output
rustc --edition 2021 --crate-type cdylib --target wasm32-unknown-unknown -C opt-level=3 -C panic=abort src/lib.rs -o output/core.wasm
