# designcraft WASM

Page layout, typography, document commands and rendering.

From the repository root:

```sh
npm run build -- designcraft
npm test -- designcraft
npm run bundle -- designcraft
npm run dev -- designcraft
npm run deploy -- designcraft
```

`GET /commands`, `/health`; `POST /run` with `{steps:[{command,params}], input?:"base64 document", output?:{format,page,scale,params}}`. Output formats include JSON reports, native documents, PNG/JPEG and upstream PDF/IDML/EPUB/HTML export. A command failure returns the completed-step report with HTTP 422.

See the adapter tests in `apps/designcraft-worker` for executable examples and exact verified paths.

Only the required source dependency closure and assets are included. No adapter upload-size, image-dimension or command-count caps are added. Upstream validation, operator/format support and actual runtime resources remain applicable. Registry discovery is not complete application conformance. Native UI, hardware and host filesystem/process capabilities are not emulated.

`UPSTREAM.json` records provenance. Original source and asset licences remain in this package; the root licence does not change their terms.
