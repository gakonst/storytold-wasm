import { build } from 'esbuild';
import { fileURLToPath } from 'node:url';
process.chdir(fileURLToPath(new URL('.', import.meta.url)));
await build({ absWorkingDir: fileURLToPath(new URL('.', import.meta.url)), entryPoints: ['worker/index.js'], outfile: 'dist/worker.js', bundle: true, format: 'esm', platform: 'browser', target: 'es2022', external: ['*.wasm'] });
