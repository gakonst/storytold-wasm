import assert from 'node:assert/strict';
import { build } from 'esbuild';
import { Miniflare } from 'miniflare';
import { mkdir, copyFile, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
const root = fileURLToPath(new URL('..', import.meta.url));
const dist = path.join(root, 'dist');
await mkdir(dist, { recursive: true });
await build({ entryPoints: [path.join(root, 'src/worker.js')], outfile: path.join(dist, 'worker.js'), bundle: true, format: 'esm', platform: 'browser', external: ['*.wasm'] });
await copyFile(path.join(root, 'generated/filmcraft_worker_bg.wasm'), path.join(dist, 'filmcraft_worker_bg.wasm'));
// esbuild preserves the relative import from generated/; resolve that observed path explicitly.
const code = (await readFile(path.join(dist, 'worker.js'), 'utf8')).replace('../generated/filmcraft_worker_bg.wasm', './filmcraft_worker_bg.wasm');
const mf = new Miniflare({ modules: [
  { type: 'ESModule', path: path.join(dist, 'worker.js'), contents: code },
  { type: 'CompiledWasm', path: path.join(dist, 'filmcraft_worker_bg.wasm') },
], compatibilityDate: '2026-02-10', modulesRoot: dist });
async function request(body, endpoint = '/run', status = 200) {
  const response = await mf.dispatchFetch(`http://filmcraft.test${endpoint}`, { method: 'POST', body: JSON.stringify(body), headers: { 'content-type': 'application/json' } });
  if (response.status !== status) throw new Error(`HTTP ${response.status}, expected ${status}: ${await response.text()}`);
  return response;
}
const cmd = (command, params = {}) => ({ command, params });
const ref = $ref => ({ $ref });
try {
  const registryResponse = await mf.dispatchFetch('http://filmcraft.test/commands');
  assert.equal(registryResponse.status, 200, await registryResponse.clone().text());
  const registry = await registryResponse.json();
  assert(registry.length > 400);
  console.log(`workerd: full registry (${registry.length} commands)`);
  const commands = [
    cmd('file.newSequence', { name: 'Worker edit', width: 64, height: 48, fps: 24, video: 1, audio: 1 }),
    cmd('file.newColorMatte', { color: '#f03020', seconds: 1 }),
    cmd('source.open', { item: ref('1.item') }),
    cmd('source.overwrite'),
    cmd('sequence.inspect'),
  ];
  const result = await (await request({ commands })).json();
  console.log(JSON.stringify(result.results));
  assert(result.project.includes('Worker edit'));
  const png = new Uint8Array(await (await request({ commands }, '/render')).arrayBuffer());
  assert.deepEqual([...png.subarray(0, 8)], [137,80,78,71,13,10,26,10]);
  assert.equal(new DataView(png.buffer).getUint32(16), 64);
  assert.equal(new DataView(png.buffer).getUint32(20), 48);
  console.log('workerd: create, assemble, inspect, CPU render PNG');
} finally { await mf.dispose(); }
