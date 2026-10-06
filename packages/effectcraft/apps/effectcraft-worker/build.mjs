import { execFileSync } from 'node:child_process';
import { mkdirSync, writeFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
const { build } = require('esbuild');
const root = fileURLToPath(new URL('../../', import.meta.url));
const app = path.join(root, 'apps/effectcraft-worker');
const target = path.resolve(root, process.env.CARGO_TARGET_DIR || 'target/worker');
const bindgen = process.env.WASM_BINDGEN || 'wasm-bindgen';
const version = execFileSync(bindgen, ['--version'], { encoding: 'utf8' }).trim();
if (version !== 'wasm-bindgen 0.2.129') throw new Error(`Expected wasm-bindgen 0.2.129; got ${version}`);
const env = { ...process.env, CARGO_BUILD_JOBS: '1', CARGO_TARGET_DIR: target };
execFileSync('cargo', ['build', ...(process.env.CARGO_OFFLINE === '1' ? ['--offline'] : []), '--release', '--target', 'wasm32-unknown-unknown', '-p', 'effectcraft-worker'], { cwd: root, env, stdio: 'inherit' });
const out = path.join(app, 'dist');
mkdirSync(out, { recursive: true });
execFileSync(bindgen, [path.join(target, 'wasm32-unknown-unknown/release/effectcraft_worker.wasm'), '--target', 'web', '--out-dir', out], { stdio: 'inherit' });
// A fresh glue closure AND WASM instance per request isolates Rust globals (Boa contexts,
// font/codec caches, registries) between users. The compiled Module is shared, memory isn't.
const glue = await build({ entryPoints: [path.join(out, 'effectcraft_worker.js')], bundle: true, format: 'cjs', platform: 'neutral', write: false, target: 'es2022', logLevel: 'error' });
writeFileSync(path.join(out, 'bindings.js'), `export function bindings() {\nconst module = { exports: {} }; const exports = module.exports;\n${glue.outputFiles[0].text}\nreturn module.exports;\n}\n`);
await build({ entryPoints: [path.join(app, 'worker.js')], outfile: path.join(out, 'worker.js'), bundle: true, format: 'esm', platform: 'neutral', target: 'es2022', external: ['*.wasm'], logLevel: 'info' });
