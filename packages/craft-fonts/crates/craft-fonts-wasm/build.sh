#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
cargo build --locked --manifest-path Cargo.toml --target wasm32-unknown-unknown --release
root=$(cargo metadata --format-version 1 --no-deps | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')
"${WASM_BINDGEN:-wasm-bindgen}" "$root/wasm32-unknown-unknown/release/craft_fonts_wasm.wasm" --target web --out-dir pkg
mkdir -p assets
cp -R ../../fonts assets/
cp ../../ATTRIBUTION.md ../../LICENSE-APACHE ../../LICENSE-MIT assets/
