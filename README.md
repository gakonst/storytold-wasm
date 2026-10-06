# Storytold WASM

Headless WebAssembly libraries and Cloudflare Worker adapters extracted from [Storytold](https://github.com/storytold). This repository keeps the required Rust dependency closures, assets, HTTP adapters and tests. Desktop applications, editor UIs, device drivers, training pipelines and deployment infrastructure are excluded.

There are no adapter-imposed upload-size, image-dimension or command-count caps, and engine commands are not restricted to an artificial allowlist. Upstream input validation and the host's actual resources still apply.

## Build a Worker

Prerequisites: Node 22+, Python 3.11+, Rust/rustup and the `wasm32-unknown-unknown` target. The root toolchain file pins the compiler. `npm run setup` installs wasm-bindgen 0.2.129 inside this checkout without replacing a global installation; alternatively set `WASM_BINDGEN` to an existing matching executable.

```sh
git clone https://github.com/gakonst/storytold-wasm.git
cd storytold-wasm
npm ci
npm run setup
npm run build -- photocraft
npm test -- photocraft
npm run bundle -- photocraft    # Wrangler deployment dry run
npm run dev -- photocraft       # Local Worker
npm run deploy -- photocraft    # Your configured Cloudflare account
```

Use another component ID below to select its Worker. `npm run build`, `npm test` and `npm run bundle` without an ID process the whole collection sequentially. `npm run check` validates that extracted source dependencies resolve within this repository. Each component is also an npm workspace with its own Wrangler configuration and package scripts.

## Components

| ID | Portable functionality |
| --- | --- |
| [photocraft](packages/photocraft/README.md) | Raster editing, color, composition, painting, PSD, image codecs and byte import/export |
| [lightcraft](packages/lightcraft/README.md) | Photo catalogues, development, metadata, presets and image export |
| [vectorcraft](packages/vectorcraft/README.md) | Vector document commands, geometry, rendering and byte import/export |
| [designcraft](packages/designcraft/README.md) | Page layout, typography, document commands and rendering |
| [filmcraft](packages/filmcraft/README.md) | Timeline editing, CPU frame rendering, audio and portable media codecs |
| [effectcraft](packages/effectcraft/README.md) | Composition, effects, expressions and CPU rendering |
| [printcraft](packages/printcraft/README.md) | PDF editing, rendering, forms, encryption, signing and document export |
| [craft-fonts](packages/craft-fonts/README.md) | Font catalogue, manifest parser and licensed font assets |
| [artcraftx](packages/artcraftx/README.md) | Image conversion, thumbnails, hashes, base64, model metadata and data utilities |
| [cloud-worker](packages/cloud-worker/README.md) | ComfyUI graph construction and frame-length calculation |
| [onnx-inference](packages/onnx-inference/README.md) | CPU ONNX evaluation through the vendored Candle implementation |
| [point-generator](packages/point-generator/README.md) | Point generation and binary point encoding/decoding |
| [actor-spawning](packages/actor-spawning/README.md) | Actor location and randomized placement arithmetic |
| [kinect-geometry](packages/kinect-geometry/README.md) | Recorded depth-frame point-cloud and mesh conversion |

These are portable components, not complete replacements for all 36 upstream repositories. Command discovery exposes the upstream registry; that is not proof that every command or format is fully implemented or tested. Package READMEs describe the HTTP APIs and supported scope; executable integration tests demonstrate the verified paths.

The `cloud-worker` package constructs ComfyUI graphs. Video generation still requires a ComfyUI inference host. `actor-spawning` implements placement arithmetic; it does not create Unreal actors. `kinect-geometry` converts supplied frames; it does not capture from a Kinect. ArtCraftX's provider login and generation clients are excluded. ONNX evaluation uses the vendored CPU evaluator and its supported operators; no model weights or GPU runtime are included.

## Runtime and integration

Workers use statically imported compiled WASM modules. Byte-oriented adapters replace native filesystem access where supported, and engine state belongs to a request or an explicitly exported document. Native hardware, host processes, desktop dialogs and background threads are not emulated.

The generated wasm-bindgen modules can also run in other compatible JavaScript/WASM hosts. Direct Rust dependencies are vendored here so consumers can build smaller modules around individual crates. The three geometry/protocol packages use freestanding Rust exports and `core.mjs` without wasm-bindgen. EffectCraft depends on selected FilmCraft crates in the adjacent package; keep the repository layout intact.

Large engines need a Workers plan that accepts their actual compressed upload size, memory, CPU and startup requirements. `npm run bundle -- <id>` reports bundle size; a successful local test or dry run does not verify production quota or workload capacity. No live Cloudflare deployment is performed by CI. See [Cloudflare's current limits](https://developers.cloudflare.com/workers/platform/limits/) when selecting a host. The same modules can be embedded in a host with more resources.

The HTTP adapters are integration examples; apply your application's authentication and storage design when exposing them. Fonts are served with Workers Static Assets by default, while a Rust `embed` feature supports standalone font-byte consumers.

## Validation and provenance

Tests execute the compiled WASM in actual workerd through Miniflare. They assert bytes, pixels, document round trips, numerical results and error recovery rather than mocking the engines. The graph tests compare against the original Python functions. CI builds and tests each component independently from a checkout.

Every extracted package records its source in `UPSTREAM.json`; original licences and asset attribution are retained. New orchestration code is MIT-licensed. Upstream code and assets retain their existing terms: the root licence does not relicense third-party material. See [NOTICE.md](NOTICE.md).
