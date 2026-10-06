const { test, before, after } = require('node:test');
const assert = require('node:assert/strict');
const { readFileSync } = require('node:fs');
const path = require('node:path');
const { spawnSync } = require('node:child_process');
const { Miniflare } = require('miniflare');
let mf;
const root = path.resolve(__dirname, '..');
before(async () => {
  mf = new Miniflare({ modules: true, scriptPath: path.join(root, 'dist/worker.mjs'), compatibilityDate: '2026-02-10', modulesRules: [{ type: 'CompiledWasm', include: ['**/*.wasm'], fallthrough: true }] });
  await mf.ready;
});
after(async () => { if (mf) await mf.dispose(); });
const step = (command, params = {}) => ({command, params});
const basic = [step('file.new', {title: 'Worker publication', width: 240, height: 200, pages: 2}), step('frame.create', {rect: [20, 20, 220, 140], content: 'text', text: 'Hello Worker — café', caret: false})];
async function run(steps = basic, output, input) {
  return mf.dispatchFetch('http://localhost/run', {method: 'POST', headers: {'content-type': 'application/json'}, body: JSON.stringify({steps, output, input})});
}
async function report(steps) {
  const res = await run(steps);
  const body = await res.json();
  assert.equal(res.status, 200, JSON.stringify(body));
  assert.equal(body.completed, steps.length);
  return body.results;
}
function zipEntries(bytes) {
  const result = spawnSync('python3', ['-c', 'import zipfile,io,sys,json,base64; z=zipfile.ZipFile(io.BytesIO(sys.stdin.buffer.read())); print(json.dumps({n:base64.b64encode(z.read(n)).decode() for n in z.namelist()}))'], {input: bytes, maxBuffer: 32 * 1024 * 1024});
  assert.equal(result.status, 0, result.stderr.toString());
  return Object.fromEntries(Object.entries(JSON.parse(result.stdout)).map(([k,v]) => [k, Buffer.from(v, 'base64')]));
}

test('real workerd health, full command discovery and HTTP errors', async () => {
  assert.equal((await (await mf.dispatchFetch('http://localhost/health')).json()).runtime, 'wasm32');
  const commands = await (await mf.dispatchFetch('http://localhost/commands')).json();
  assert(commands.length > 200);
  for (const id of ['file.place', 'file.exportPdf', 'script.run', 'table.create', 'xml.loadDtd', 'font.loadBytes', 'color.loadProfileBytes', 'file.print']) assert(commands.some(c => c.id === id), id);
  console.log(`Upstream registry: ${commands.length} commands`);
  assert.equal((await mf.dispatchFetch('http://localhost/missing')).status, 404);
  assert.equal((await mf.dispatchFetch('http://localhost/run')).status, 405);
  assert.equal((await mf.dispatchFetch('http://localhost/run', {method:'POST', body:'{'})).status, 400);
});

test('text composition, styles, result references, undo/redo, queries', async () => {
  const out = await report([...basic,
    step('text.select', {story: '$1.story', anchor: 5, focus: 5}),
    step('text.insert', {text: ' real'}),
    step('edit.undo'), step('story.get', {story: '$1.story'}),
    step('edit.redo'), step('story.get', {story: '$1.story'}),
    step('style.paragraph.create', {name: 'Worker heading', chars: {size: 18}}),
    step('document.inspect'), step('preflight.run'), step('font.list'),
  ]);
  assert.equal(out[5].text, 'Hello Worker — café');
  assert.equal(out[7].text, 'Hello real Worker — café');
  assert(out[9].stories[0].lines > 0, 'real shaping/composition');
});

test('native ZIP persistence and IDML round trips retain editable stories and assets', async () => {
  const svg = Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20"><rect width="20" height="20" fill="red"/></svg>').toString('base64');
  const steps = [...basic, step('edit.deselectAll'), step('file.place', {base64: svg, name:'red.svg', x: 10, y: 150, width:20})];
  for (const format of ['designcraft', 'idml']) {
    const res = await run(steps, {format});
    assert.equal(res.status, 200, awaitIfError(res));
    const bytes = Buffer.from(await res.arrayBuffer());
    const zip = zipEntries(bytes);
    assert(zip.mimetype);
    assert(Object.keys(zip).some(k => format === 'designcraft' ? k.startsWith('assets/') : k.includes('Stories/')));
    const reopened = await run([step('document.inspect')], undefined, bytes.toString('base64'));
    assert.equal(reopened.status, 200);
    const doc = (await reopened.json()).results[0];
    assert.equal(doc.stories[0].preview, 'Hello Worker — café');
    assert(doc.spreads.flatMap(s => s.items).some(i => i.asset));
  }
});
function awaitIfError(res) { return `HTTP ${res.status}`; }

test('CPU PNG and JPEG, real selectable PDF, EPUB and HTML bytes', async () => {
  for (const format of ['png', 'jpeg', 'pdf', 'epub', 'html']) {
    const res = await run(basic, {format});
    assert.equal(res.status, 200, awaitIfError(res));
    const bytes = Buffer.from(await res.arrayBuffer());
    if (format === 'png') {
      assert.equal(bytes.subarray(1,4).toString(), 'PNG');
      assert.equal(bytes.readUInt32BE(16), 240);
      assert.equal(bytes.readUInt32BE(20), 200);
      const {inflateSync} = require('node:zlib');
      const idat = [];
      for (let pos = 8; pos < bytes.length;) {
        const len = bytes.readUInt32BE(pos);
        if(bytes.subarray(pos+4,pos+8).toString() === 'IDAT') idat.push(bytes.subarray(pos+8,pos+8+len));
        pos += len + 12;
      }
      const raw = inflateSync(Buffer.concat(idat));
      assert(new Set(raw).size > 20, 'rendered glyph coverage, not blank placeholder');
    } else if (format === 'jpeg') assert.equal(bytes.readUInt16BE(0), 0xffd8);
    else if (format === 'pdf') {
      assert(bytes.subarray(0,5).equals(Buffer.from('%PDF-')));
      assert(bytes.includes(Buffer.from('/ToUnicode')), 'selectable text mapping');
      assert(bytes.includes(Buffer.from('/Font')));
    } else if (format === 'epub') {
      const zip = zipEntries(bytes);
      assert.equal(zip.mimetype.toString(), 'application/epub+zip');
      assert(Object.values(zip).some(v => v.includes(Buffer.from('Hello Worker'))));
    } else assert(bytes.includes(Buffer.from('Hello Worker')));
  }
});

test('filesystem, print, UI and nested-script requests cannot pretend success', async () => {
  const rejected = [
    step('file.save', {path:'/tmp/fake.designcraft'}), step('file.saveACopy', {path:'/tmp/fake'}),
    ...['file.exportPdf','file.exportIdml','file.exportEpub','file.exportHtml','file.exportXml','snippet.export'].map(c => step(c,{path:'/tmp/fake'})),
    step('file.print', {dryRun:true}), step('file.printers'), step('file.place'),
    step('file.place',{path:'/tmp/missing.png'}), step('file.package',{dir:'/tmp/fake'}),
    step('library.new',{path:'/tmp/fake'}), step('book.new',{path:'/tmp/fake'}),
    step('script.run',{text:'file.exportHtml {"path":"/tmp/fake"}'}),
  ];
  for (const command of rejected) {
    const res = await run([...basic, command]);
    const body = await res.json();
    assert.equal(res.status, 422, `${command.command}: ${JSON.stringify(body)}`);
    assert.match(body.error, /unavailable/);
  }
  assert.equal((await run()).status, 200, 'subsequent requests still work');
});

test('no artificial command-count or request byte caps; explicit invalid inputs', async () => {
  const many = Array.from({length: 1100}, () => step('file.presets'));
  assert.equal((await run(many)).status, 200);
  const large = 'x'.repeat(2 * 1024 * 1024);
  const res = await run([step('file.new', {title: large}), step('document.list')]);
  assert.equal(res.status, 200);
  assert.equal((await res.json()).results[1][0].title.length, large.length);
  for(const scale of [0, -1, 'bad']) assert.equal((await run(basic, {format:'png', scale})).status, 400);
  assert.equal((await run(basic, {format:'png', page:999})).status, 400);
  assert.equal((await run([], undefined, Buffer.from('garbage').toString('base64'))).status, 400);
  const failed = await run([...basic, step('unknown.command'), step('file.new')]);
  assert.equal(failed.status, 422);
  assert.equal((await failed.json()).completed, 2);
});

test('requests have isolated documents and global color settings', async () => {
  const changed = await report([step('color.settings', {bpc: false})]);
  assert.equal(changed[0].bpc, false);
  const clean = await report([step('document.list'), step('color.settings')]);
  assert.deepEqual(clean[0], []);
  assert.equal(clean[1].bpc, true);
  const responses = await Promise.all(Array.from({length:4}, (_,i) => run([step('file.new',{title:`doc-${i}`}), step('document.list')])));
  for(let i=0;i<responses.length;i++) assert.equal((await responses[i].json()).results[1][0].title, `doc-${i}`);
});
