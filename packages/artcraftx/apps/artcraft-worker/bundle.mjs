import { build } from 'esbuild';
import { mkdir, copyFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
process.chdir(fileURLToPath(new URL('.', import.meta.url)));
await mkdir('dist', { recursive: true });
await build({ entryPoints: ['worker/index.js'], outfile: 'dist/worker.js', bundle: true,
  format: 'esm', platform: 'browser', target: 'es2022', external: ['*.wasm'] });
await copyFile('pkg/artcraft_worker_bg.wasm', 'dist/artcraft_worker_bg.wasm');
