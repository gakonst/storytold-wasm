# actor-spawning WASM

Actor location and random placement arithmetic. POST /location and /randomLocation. This does not create actors inside Unreal Engine.

From the repository root, run `npm run build -w @storytold-wasm/actor-spawning` then `npm test -w @storytold-wasm/actor-spawning`. Deploy with `npm run deploy -w @storytold-wasm/actor-spawning`. Rust compiles a freestanding WASM module; this package does not need wasm-bindgen. `core.mjs` also exposes `initialize(module)` and `load(url)` for non-Worker hosts. No adapter request-size or count limits are added.

The HTTP tests run the real Worker in workerd and check numerical values, invalid inputs, recovery and protocol round trips where applicable. Upstream origin and revision are recorded in UPSTREAM.json.
