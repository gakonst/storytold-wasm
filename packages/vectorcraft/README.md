# vectorcraft WASM

Vector document commands, geometry, rendering and byte import/export.

From the repository root:

```sh
npm run build -- vectorcraft
npm test -- vectorcraft
npm run bundle -- vectorcraft
npm run dev -- vectorcraft
npm run deploy -- vectorcraft
```

`GET /commands`, `/formats`, `/health`; `POST /run` with `{input?:{name,dataBase64}, steps:[{command,params}], output?:{format}}`; `POST /convert` and `/mcp`. Results contain the upstream command output and exported files. The headless session uses memory-backed file services.

See the adapter tests in `apps/vectorcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
