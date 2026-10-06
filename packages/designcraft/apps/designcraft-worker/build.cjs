// Generate the required Workers module glue; HTTP and engine behavior are Rust.
const { buildSync } = require('esbuild');
const { mkdirSync, writeFileSync, copyFileSync } = require('node:fs');
const { join } = require('node:path');
const root = __dirname;
const result = buildSync({ entryPoints: [join(root, 'pkg/designcraft_worker.js')], bundle: true, write: false, format: 'iife', globalName: 'bindings', platform: 'browser', target: 'es2022', logLevel: 'silent' });
mkdirSync(join(root, 'dist'), { recursive: true });
// A separate instance/JS closure per request isolates upstream global font/color caches,
// releases linear memory after each response, and lets later requests survive a WASM trap.
writeFileSync(join(root, 'dist/worker.mjs'), `import module from './designcraft_worker_bg.wasm';
function createBindings() {\n${result.outputFiles[0].text}\nreturn bindings;\n}
export default { async fetch(request) {
  const api = createBindings();
  try {
    api.initSync({ module });
    return await api.fetch(request);
  } catch (error) {
    return Response.json({error: String(error)}, {status: 500});
  }
}};
`);
copyFileSync(join(root, 'pkg/designcraft_worker_bg.wasm'), join(root, 'dist/designcraft_worker_bg.wasm'));
