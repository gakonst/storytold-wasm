import assert from 'node:assert/strict';
import { Miniflare } from 'miniflare';
import { deflateSync, inflateSync } from 'node:zlib';
import { fileURLToPath } from 'node:url';
import { readFileSync } from 'node:fs';
process.chdir(fileURLToPath(new URL('..', import.meta.url)));
function crc(bytes) { let c = 0xffffffff; for (const b of bytes) { c ^= b; for (let i = 0; i < 8; i++) c = (c >>> 1) ^ ((c & 1) ? 0xedb88320 : 0); } return (c ^ 0xffffffff) >>> 0; }
function chunk(type, data) { const t = Buffer.from(type), size = Buffer.alloc(4), sum = Buffer.alloc(4); size.writeUInt32BE(data.length); sum.writeUInt32BE(crc(Buffer.concat([t, data]))); return Buffer.concat([size, t, data, sum]); }
function png(w, h, bias = 0) {
  const header = Buffer.alloc(13); header.writeUInt32BE(w); header.writeUInt32BE(h, 4); header[8] = 8; header[9] = 2;
  const pixels = Buffer.alloc((w * 3 + 1) * h);
  for (let y = 0; y < h; y++) for (let x = 0; x < w; x++) { const i = y * (w * 3 + 1) + 1 + x * 3; pixels[i] = (x * 3 + bias) % 256; pixels[i + 1] = (y * 5 + bias) % 256; pixels[i + 2] = 80 + bias; }
  return Buffer.concat([Buffer.from('89504e470d0a1a0a', 'hex'), chunk('IHDR', header), chunk('IDAT', deflateSync(pixels)), chunk('IEND', Buffer.alloc(0))]);
}
function pngInfo(bytes) { assert.equal(bytes.subarray(0, 8).toString('hex'), '89504e470d0a1a0a'); return { width: bytes.readUInt32BE(16), height: bytes.readUInt32BE(20), depth: bytes[24] }; }
const mf = new Miniflare({ modules: true, scriptPath: 'dist/worker.js', compatibilityDate: '2026-02-10', modulesRules: [{ type: 'CompiledWasm', include: ['**/*.wasm'], fallthrough: true }] });
const input = (name, bytes) => ({ name, base64: bytes.toString('base64') });
const cmd = (id, params = {}) => ({ id, params });
let checks = 0;
async function request(path, body, status = 200) {
  const r = await mf.dispatchFetch(`http://lightcraft.test${path}`, { method: 'POST', headers: { 'content-type': 'application/json' }, body: JSON.stringify(body) });
  assert.equal(r.status, status, `${path}: ${r.status === status ? '' : await r.text()}`); checks++; return r;
}
async function run(body, status = 200) { const r = await request('/run', body, status); return r.json(); }
try {
  const commands = await (await mf.dispatchFetch('http://lightcraft.test/commands')).json();
  assert.ok(commands.length > 150, `full registry, got ${commands.length}`);
  assert.ok(commands.some(c => c.id === 'mask.add'));
  console.log(`Registry: ${commands.length} commands, ${commands.filter(c => c.disabled_reason?.includes('unavailable')).length} explicit native denials`);
  const bytes = png(96, 64);
  const first = await run({ inputs: [input('gradient.png', bytes)], commands: [cmd('photo.rate', { rating: 4 }), cmd('photo.setMeta', { title: 'Worker journey', addKeywords: ['edge', 'rust'] }), cmd('develop.set', { control: 'light.exposure', value: 0.7 }), cmd('album.create', { name: 'Worker', addSelected: true }), cmd('photo.inspect'), cmd('catalog.stats')], outputs: [{ format: 'png' }, { format: 'tiff', bitDepth: 16 }, { format: 'original' }] });
  assert.equal(first.imported[0].result.imported.length, 1);
  assert.equal(first.results.at(-1).photos, 1);
  assert.deepEqual(pngInfo(Buffer.from(first.outputs[0].base64, 'base64')), { width: 96, height: 64, depth: 8 });
  assert.equal(Buffer.from(first.outputs[2].base64, 'base64').compare(bytes), 0);
  assert.ok(first.outputs[2].sidecars.length > 0);
  const restored = await run({ state: first.state, commands: [cmd('catalog.query'), cmd('develop.get'), cmd('albums.list'), cmd('photo.allMetadata')], outputs: [{ format: 'png' }] });
  assert.equal(restored.results[0].photos[0].rating, 4);
  assert.equal(restored.results[1].light.exposure, 0.7);
  assert.equal(restored.outputs[0].base64, first.outputs[0].base64, 'render survives complete session recreation');
  const edits = await run({ state: first.state, commands: [cmd('develop.set', { control: 'light.exposure', value: 2 }), cmd('edit.undo'), cmd('develop.get'), cmd('edit.redo'), cmd('develop.get'), cmd('preset.create', { name: 'Worker look' }), cmd('version.create', { name: 'At edge' }), cmd('crop.set', { rect: [0, 0, 0.5, 1] })], outputs: [{ format: 'png', bitDepth: 16 }] });
  assert.equal(edits.results[2].light.exposure, 0.7);
  assert.equal(edits.results[4].light.exposure, 2);
  assert.equal(pngInfo(Buffer.from(edits.outputs[0].base64, 'base64')).depth, 16);
  assert.ok(edits.outputs[0].width < 96);
  for (const format of ['jpeg', 'png', 'tiff', 'webp']) {
    const r = await mf.dispatchFetch(`http://lightcraft.test/convert?format=${format}`, { method: 'POST', body: bytes });
    assert.equal(r.status, 200, await r.clone().text());
    const encoded = Buffer.from(await r.arrayBuffer());
    const back = await mf.dispatchFetch('http://lightcraft.test/convert?format=png', { method: 'POST', body: encoded });
    assert.equal(back.status, 200, await back.clone().text());
    assert.equal(pngInfo(Buffer.from(await back.arrayBuffer())).width, 96); checks += 2;
  }
  await run({ commands: [cmd('library.devices')] }, 501);
  await run({ inputs: [input('bad.png', Buffer.from('not an image'))], commands: [cmd('develop.auto')] }, 400);
  await run({ commands: [cmd('no.such.command')] }, 400);
  await run({ state: first.state, commands: [cmd('library.import', { paths: ['missing.png'], mode: 'copy' })] }, 501);
  await run({ state: first.state, outputs: [{ format: 'imaginary' }] }, 400);
  await run({ state: first.state, commands: [cmd('library.xmpPreferences', { autoWrite: true })] }, 501);
  // No dimension or command-count HTTP caps. Wide but inexpensive image, 600 real commands.
  const wide = await run({ inputs: [input('wide.png', png(10001, 1))], commands: Array.from({ length: 600 }, () => cmd('catalog.stats')), outputs: [{ format: 'png' }] });
  assert.equal(wide.results.length, 600);
  assert.equal(wide.outputs[0].width, 10001);
  // A request owns its session: no state leaks from previous or concurrent requests.
  const isolated = await Promise.all([run({ commands: [cmd('catalog.stats')] }), run({ state: first.state, commands: [cmd('catalog.stats')] })]);
  assert.equal(isolated[0].results[0].photos, 0); assert.equal(isolated[1].results[0].photos, 1);
  console.log(`PASS ${checks} actual workerd HTTP checks: imports, edits, undo/redo, metadata, albums, presets, versions, crop, state restoration, byte exports, codec round trips, errors, broad dimensions/counts, isolation`);
} finally { await mf.dispose(); }
