import assert from 'node:assert/strict';
import { fileURLToPath } from 'node:url';
import { createRequire } from 'node:module';
import { inflateSync } from 'node:zlib';
const require = createRequire(import.meta.url);
const { Miniflare } = require('miniflare');
const mf = new Miniflare({
  modules: true,
  scriptPath: fileURLToPath(new URL('../dist/worker.js', import.meta.url)),
  modulesRules: [{ type: 'CompiledWasm', include: ['**/*.wasm'], fallthrough: true }],
  compatibilityDate: '2026-02-10',
});
const cmd = (command, params = {}) => ({ command, params });
async function post(body, endpoint = '/run', status = 200) {
  const response = await mf.dispatchFetch(`http://localhost${endpoint}`, { method: 'POST', body: JSON.stringify(body) });
  if (response.status !== status) throw new Error(`${response.status}: ${await response.text()}`);
  if (endpoint === '/render' && status === 200) return Buffer.from(await response.arrayBuffer());
  return response.json();
}
// Decode PNG without an image dependency, checking every pixel via the PNG row filters.
function png(bytes) {
  assert.deepEqual([...bytes.subarray(0, 8)], [137, 80, 78, 71, 13, 10, 26, 10]);
  const width = bytes.readUInt32BE(16), height = bytes.readUInt32BE(20);
  assert.equal(bytes[24], 8); assert.equal(bytes[25], 6);
  const data = [];
  for (let p = 8; p < bytes.length;) {
    const n = bytes.readUInt32BE(p), kind = bytes.toString('ascii', p + 4, p + 8);
    if (kind === 'IDAT') data.push(bytes.subarray(p + 8, p + 8 + n));
    p += n + 12;
  }
  const raw = inflateSync(Buffer.concat(data)), stride = width * 4;
  const rgba = Buffer.alloc(stride * height);
  const paeth = (a, b, c) => {
    const p = a + b - c, pa = Math.abs(p - a), pb = Math.abs(p - b), pc = Math.abs(p - c);
    return pa <= pb && pa <= pc ? a : pb <= pc ? b : c;
  };
  for (let y = 0; y < height; y++) {
    const filter = raw[y * (stride + 1)];
    for (let x = 0; x < stride; x++) {
      const i = y * stride + x, a = x >= 4 ? rgba[i - 4] : 0, b = y ? rgba[i - stride] : 0, c = y && x >= 4 ? rgba[i - stride - 4] : 0;
      rgba[i] = raw[y * (stride + 1) + x + 1] + [0, a, b, Math.floor((a + b) / 2), paeth(a, b, c)][filter];
    }
  }
  return { width, height, rgba };
}
try {
  const caps = await (await mf.dispatchFetch('http://localhost/capabilities')).json();
  assert.equal(caps.engine, 'EffectCraft');
  const registry = await post({ commands: [cmd('command.list'), cmd('effect.list')] });
  const commands = registry.results[0].result;
  console.log('registry shape', JSON.stringify(commands).slice(0, 250));
  assert.ok(JSON.stringify(commands).includes('renderQueue.render'));
  assert.ok(JSON.stringify(commands).includes('mask.new'));
  console.log('PASS full command and effect registries');

  const edited = await post({ commands: [
    cmd('comp.new', { name: 'Journey', width: 64, height: 48, duration: 1, fps: 24 }),
    cmd('layer.newSolid', { name: 'Red', color: '#ff0000' }),
    cmd('layer.rename', { name: 'Changed' }), cmd('edit.undo'), cmd('edit.redo'),
    cmd('file.saveAs', { path: '/journey.ecproj' }), cmd('project.summary'),
  ], render: {} });
  assert.equal(edited.ok, true);
  assert.ok(JSON.stringify(edited.project).includes('Changed'));
  const frame = png(Buffer.from(edited.render.base64, 'base64'));
  assert.equal(frame.width, 64); assert.equal(frame.height, 48);
  assert.deepEqual([...frame.rgba.subarray(0, 4)], [255, 0, 0, 255]);
  const save = edited.files.find(f => f.path === '/journey.ecproj');
  assert.ok(save);
  const reopened = await post({ files: [save], commands: [cmd('file.open', { path: save.path })], render: {} });
  assert.deepEqual(png(Buffer.from(reopened.render.base64, 'base64')).rgba, frame.rgba);
  console.log('PASS edit/undo/redo/save/upload/reopen and pixel-identical CPU render');

  const animated = await post({ project: edited.project, commands: [
    cmd('layer.select', { layers: ['Changed'] }),
    cmd('prop.addKey', { path: 'transform/opacity', time: 0, value: 0 }),
    cmd('prop.addKey', { path: 'transform/opacity', time: 1, value: 100 }),
    cmd('effect.apply', { effect: 'Gaussian Blur' }),
  ] });
  const half = png(await post({ project: animated.project, render: { time: 0.5, transparent: true } }, '/render'));
  assert.ok(half.rgba[3] >= 126 && half.rgba[3] <= 129);
  const expr = await post({ project: edited.project, commands: [
    cmd('prop.setExpression', { layer: 'Changed', path: 'transform/opacity', expression: 'time * 100' }),
  ], render: { time: 0.25, transparent: true } });
  const quarter = png(Buffer.from(expr.render.base64, 'base64'));
  assert.ok(quarter.rgba[3] >= 63 && quarter.rgba[3] <= 65);
  console.log('PASS animated properties, Gaussian Blur and Boa expression evaluation');

  const scripted = await post({ commands: [cmd('script.run', { code: "var c = app.project.items.addComp('Scripted', 80, 60, 1, 1, 24); c.layers.addSolid([0,1,0], 'Green', 80, 60, 1);" })], render: {} });
  assert.deepEqual([...png(Buffer.from(scripted.render.base64, 'base64')).rgba.subarray(0, 4)], [0, 255, 0, 255]);
  console.log('PASS Boa scripting creates and renders composition');

  const imageImport = await post({ files: [{ path: '/red.png', base64: edited.render.base64 }], commands: [
    cmd('comp.new', { width: 64, height: 48 }), cmd('file.import', { paths: ['/red.png'] }),
    cmd('layer.addItem', { item: 'red.png' }),
  ], render: {} });
  assert.deepEqual(png(Buffer.from(imageImport.render.base64, 'base64')).rgba, frame.rgba);
  console.log('PASS byte-backed PNG media probing, import and decoding');

  const lottie = await post({ project: edited.project, commands: [cmd('file.exportLottie', { path: '/animation.json' })] });
  const animation = lottie.files.find(f => f.path === '/animation.json');
  assert.ok(animation);
  const imported = await post({ files: [animation], commands: [cmd('file.importLottie', { path: animation.path })], render: {} });
  assert.equal(png(Buffer.from(imported.render.base64, 'base64')).width, 64);
  console.log('PASS Lottie export/import/render round trip');

  const exported = await post({ project: edited.project, commands: [
    cmd('renderQueue.add', { format: 'gif', output: '/movie.gif', timeSpan: 'custom', start: 0, end: 0.125 }),
    cmd('renderQueue.render', { wait: true }),
  ] });
  const gif = exported.files.find(f => f.path === '/movie.gif');
  assert.ok(gif, JSON.stringify(exported.results));
  assert.equal(Buffer.from(gif.base64, 'base64').toString('ascii', 0, 6), 'GIF89a');
  console.log('PASS Render Queue encodes multi-frame GIF into memory');

  const failure = await post({ commands: [cmd('comp.new', { name: 'Keep' }), cmd('not.a.command')] }, '/run', 422);
  assert.ok(JSON.stringify(failure.project).includes('Keep'));
  assert.match(failure.results[1].error, /unknown/i);
  await post({ commands: [cmd('file.open', { path: '/native/absent.ecproj' })] }, '/run', 422);
  const ui = await post({ commands: [cmd('app.quit')] }, '/run', 422);
  assert.match(ui.results[0].error, /unavailable/);
  const invalid = await mf.dispatchFetch('http://localhost/run', { method: 'POST', body: '{' });
  assert.equal(invalid.status, 400);
  console.log('PASS malformed/unknown/native/UI errors and recoverable project');

  const wide = png(await post({ commands: [cmd('comp.new', { width: 4097, height: 2 })] }, '/render'));
  assert.equal(wide.width, 4097);
  const many = await post({ commands: Array.from({ length: 300 }, () => cmd('project.summary')) });
  assert.equal(many.results.length, 300);
  const fresh = await post({});
  assert.equal(Object.keys(fresh.project.items).length, 0);
  assert.equal(fresh.files.length, 0);
  console.log('PASS no artificial dimension/command-count caps; requests are isolated');
} finally { await mf.dispose(); }
