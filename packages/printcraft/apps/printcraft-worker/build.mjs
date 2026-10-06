import {build} from 'esbuild';
import {copyFile, mkdir} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
const root = fileURLToPath(new URL('.', import.meta.url));
await mkdir(root + 'dist', {recursive: true});
await build({entryPoints: [root + 'worker/index.js'], outfile: root + 'dist/index.js', bundle: true, format: 'esm', platform: 'neutral', target: 'es2022', plugins: [{name: 'worker-wasm', setup(build) {
  build.onResolve({filter: /\.wasm$/}, () => ({path: './printcraft_worker_bg.wasm', external: true}));
}}]});
await copyFile(root + 'pkg/printcraft_worker_bg.wasm', root + 'dist/printcraft_worker_bg.wasm');
