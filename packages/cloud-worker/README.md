# cloud-worker WASM

ComfyUI graph construction and frame-length calculation.

From the repository root:

```sh
npm run build -- cloud-worker
npm test -- cloud-worker
npm run bundle -- cloud-worker
npm run dev -- cloud-worker
npm run deploy -- cloud-worker
```

`POST /graph` accepts the original benchmark graph parameters (`task`, `model_file`, `prompt`, `width`, `height`, `length`, `steps`, `seed`) plus optional sampler, scheduler and reference parameters. Tasks are `t2v`, `i2v`, `ref2v`. An `images` array supplies explicit reference names. `POST /snap-length` accepts `{seconds}`. The response is a ComfyUI API graph or frame count; no remote generation is initiated.

See the adapter tests in `.` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
