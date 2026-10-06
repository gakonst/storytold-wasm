# point-generator WASM

Point generation and the compatible little-endian XYZ/RGBA binary layout. POST /generate, /encode and /decode. TCP transport is supplied by the embedding host.

From the repository root, run `npm run build -w @storytold-wasm/point-generator` then `npm test -w @storytold-wasm/point-generator`. Deploy with `npm run deploy -w @storytold-wasm/point-generator`. Rust compiles a freestanding WASM module; this package does not need wasm-bindgen. `core.mjs` also exposes `initialize(module)` and `load(url)` for non-Worker hosts. No adapter request-size or count limits are added.

The HTTP tests run the real Worker in workerd and check numerical values, invalid inputs, recovery and protocol round trips where applicable. Upstream origin and revision are recorded in UPSTREAM.json.
