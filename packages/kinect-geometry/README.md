# kinect-geometry WASM

Depth-frame point cloud and mesh conversion. POST /pointCloud and /mesh. Input is aligned int16 XYZ in millimetres and BGRA pixels; device capture and Kinect calibration require a native producer.

From the repository root, run `npm run build -w @storytold-wasm/kinect-geometry` then `npm test -w @storytold-wasm/kinect-geometry`. Deploy with `npm run deploy -w @storytold-wasm/kinect-geometry`. Rust compiles a freestanding WASM module; this package does not need wasm-bindgen. `core.mjs` also exposes `initialize(module)` and `load(url)` for non-Worker hosts. No adapter request-size or count limits are added.

The HTTP tests run the real Worker in workerd and check numerical values, invalid inputs, recovery and protocol round trips where applicable. Upstream origin and revision are recorded in UPSTREAM.json.
