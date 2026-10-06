#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
cargo build --locked --manifest-path Cargo.toml --target wasm32-unknown-unknown --release
root=$(cargo metadata --format-version 1 --no-deps --manifest-path Cargo.toml | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
"${WASM_BINDGEN:-wasm-bindgen}" "$root/wasm32-unknown-unknown/release/photocraft_worker.wasm" --target web --out-dir pkg
