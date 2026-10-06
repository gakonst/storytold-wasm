# lightcraft WASM

Photo catalogues, development, metadata, presets and image export.

From the repository root:

```sh
npm run build -- lightcraft
npm test -- lightcraft
npm run bundle -- lightcraft
npm run dev -- lightcraft
npm run deploy -- lightcraft
```

`GET /commands`; `POST /run` with `{inputs:[{name,base64}], commands:[{id,params}], outputs:[{format}], state?}`; `POST /convert?format=png` with encoded image bytes. `/run` returns results, exported bytes and a reusable session state. Native device, disk and watching capabilities return explicit errors.

See the adapter tests in `apps/lightcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
