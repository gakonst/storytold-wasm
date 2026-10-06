import {Miniflare} from 'miniflare';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {fileURLToPath} from 'node:url';
import {inflateSync} from 'node:zlib';

const root = fileURLToPath(new URL('../', import.meta.url));
const mf = new Miniflare({modules: true, scriptPath: root + 'dist/index.js', modulesRules: [{type: 'CompiledWasm', include: ['**/*.wasm'], fallthrough: true}], compatibilityDate: '2026-02-10'});
const c = (name, args = {}) => ({name, arguments: args});
const v = (job, i) => job.results.at(i)[0].value;
const bytes = (job, path) => Buffer.from(job.files.find(f => f.path === path).base64, 'base64');
const upload = (path, data) => ({path, base64: Buffer.from(data).toString('base64')});
const completed = [];
async function run(calls, files = [], status = 200) {
  const response = await mf.dispatchFetch('https://test/run', {method: 'POST', body: JSON.stringify({calls, files})});
  const body = await response.json();
  assert.equal(response.status, status, JSON.stringify(body));
  return body;
}
function png(data) {
  const b = Buffer.from(data);
  assert.equal(b.subarray(1, 4).toString(), 'PNG');
  const width = b.readUInt32BE(16), height = b.readUInt32BE(20);
  const chunks = [];
  for (let i = 8; i < b.length;) {
    const n = b.readUInt32BE(i);
    if (b.subarray(i + 4, i + 8).toString() === 'IDAT') chunks.push(b.subarray(i + 8, i + 8 + n));
    i += 12 + n;
  }
  const raw = inflateSync(Buffer.concat(chunks));
  assert(raw.length > width * height); // Real compressed pixel data, not just a PNG header.
  return {width, height, raw};
}
async function journey(name, fn) {
  await fn();
  completed.push(name);
  console.log('PASS', name);
}
try {
  const catalogue = await (await mf.dispatchFetch('https://test/tools')).json();
  // Compare with the actual upstream table, so a hidden HTTP whitelist fails this test.
  const source = await readFile(root + '../../crates/automation/src/tools.rs', 'utf8');
  const upstream = [...source.matchAll(/\bt\(\s*"([a-z_]+)"/g)].map(m => m[1]).sort();
  assert.deepEqual(catalogue.map(t => t.name).sort(), upstream);
  assert(catalogue.every(t => t.input_schema && t.availability));
  console.log('Full upstream catalogue:', catalogue.length);
  let pdf;
  await journey('create, edit, text, bookmarks, comments, undo/redo, save/reopen', async () => {
    const job = await run([
      c('doc_create', {from: 'text', text: 'Worker journey hello world'}),
      c('doc_set_info', {doc: 1, key: 'Title', value: 'Worker test'}),
      c('bookmark_add', {doc: 1, page: 1, title: 'Start'}),
      c('comment_add', {doc: 1, page: 1, type: 'note', at: [20, 20], contents: 'Review in Workers'}),
      c('page_rotate', {doc: 1, degrees: 90}), c('edit_undo', {doc: 1}), c('edit_redo', {doc: 1}), c('edit_undo', {doc: 1}),
      c('text_extract', {doc: 1}), c('doc_save', {doc: 1, path: 'edited.pdf'}),
      c('doc_set_info', {doc: 1, key: 'Subject', value: 'incremental'}), c('doc_save', {doc: 1}),
      c('doc_open', {path: 'edited.pdf'}), c('doc_info', {doc: 2}),
    ]);
    assert.match(JSON.stringify(v(job, 8)), /hello world/);
    assert.equal(v(job, 11).incremental, true);
    assert.equal(v(job, 13).title, 'Worker test');
    assert.equal(v(job, 13).outline[0].title, 'Start');
    assert.equal(v(job, 13).annotations[0].contents, 'Review in Workers');
    pdf = bytes(job, '/edited.pdf');
    assert.equal(pdf.subarray(0, 5).toString(), '%PDF-');
  });
  await journey('raw PDF HTTP rendering with real pixels', async () => {
    const response = await mf.dispatchFetch('https://test/render?page=1&dpi=72', {method: 'POST', body: pdf});
    assert.equal(response.status, 200, await response.clone().text());
    assert.equal(response.headers.get('content-type'), 'image/png');
    const result = png(await response.arrayBuffer());
    assert.equal(result.width, 612); assert.equal(result.height, 792);
    assert(result.raw.some(x => x !== 0 && x !== 255));
  });
  await journey('memory files combine, insert, extract, split, print-ready PDF', async () => {
    const job = await run([
      c('doc_combine', {paths: ['a.pdf', 'b.pdf'], out: 'combined.pdf', open: true}),
      c('page_insert_file', {doc: 1, path: 'a.pdf', at: 2}),
      c('page_extract', {doc: 1, pages: [2], out: 'extracted.pdf', open: false}),
      c('doc_split', {doc: 1, every: 1, out_dir: 'parts'}),
      c('doc_print', {doc: 1, path: 'print.pdf', layout: 'multiple', per_sheet: 2}),
      c('doc_list'),
    ], [upload('a.pdf', pdf), upload('b.pdf', pdf)]);
    assert.equal(v(job, -1).documents[0].pages, 3);
    assert.equal(job.files.filter(f => f.path.startsWith('/parts/')).length, 3);
    assert.equal(bytes(job, '/print.pdf').subarray(0, 5).toString(), '%PDF-');
  });
  await journey('form authoring, Boa JavaScript, data export/import, flatten', async () => {
    const job = await run([
      c('doc_create', {from: 'blank'}),
      c('form_add_field', {doc: 1, page: 1, type: 'text', rect: [30, 30, 200, 60], name: 'Name'}),
      c('form_fill', {doc: 1, values: {Name: 'Before'}}),
      c('js_run', {doc: 1, script: 'this.getField("Name").value = "Worker Ada"; console.println("ran");'}),
      c('form_fields', {doc: 1}),
      c('doc_export_data', {doc: 1, path: 'form.xfdf', what: 'fields'}),
      c('form_reset', {doc: 1}), c('doc_import_data', {doc: 1, path: 'form.xfdf'}),
      c('form_fields', {doc: 1}), c('doc_flatten', {doc: 1}), c('form_fields', {doc: 1}),
    ]);
    assert.match(JSON.stringify(v(job, 4)), /Worker Ada/);
    assert.match(bytes(job, '/form.xfdf').toString(), /Worker Ada/);
    assert.match(JSON.stringify(v(job, 8)), /Worker Ada/);
    assert.equal(v(job, -1).fields.length, 0);
  });
  await journey('AES-256 password save, rejected password, owner reopen', async () => {
    const encrypted = await run([
      c('doc_open', {path: 'input.pdf'}),
      c('doc_protect', {doc: 1, open_password: 'reader', permissions_password: 'owner-password', compatibility: 'aes-256'}),
      c('doc_save', {doc: 1, path: 'encrypted.pdf'}),
    ], [upload('input.pdf', pdf)]);
    const input = upload('encrypted.pdf', bytes(encrypted, '/encrypted.pdf'));
    const bad = await run([c('doc_open', {path: 'encrypted.pdf', password: 'wrong'})], [input], 400);
    assert.match(bad.error, /password/i);
    const opened = await run([c('doc_open', {path: 'encrypted.pdf', password: 'owner-password'}), c('text_extract', {doc: 1})], [input]);
    assert.match(JSON.stringify(v(opened, -1)), /hello world/);
  });
  await journey('P-256 identity creation, PKCS12, PAdES signing and trust verification', async () => {
    const signed = await run([
      c('doc_open', {path: 'input.pdf'}),
      c('sign_id_create', {name: 'Worker Test', key: 'p256', password: 'secret123', path: 'id.p12'}),
      c('sign_document', {doc: 1, id: 'id.p12', password: 'secret123', out: 'signed.pdf', reason: 'Runtime test'}),
      c('sign_trust', {paths: ['id.p12'], password: 'secret123'}), c('sign_list', {doc: 1}),
    ], [upload('input.pdf', pdf)]);
    assert.match(JSON.stringify(v(signed, -1)), /"status":"valid"/);
    assert(bytes(signed, '/signed.pdf').length > pdf.length);
  });
  await journey('redaction removes searchable text after save/reopen', async () => {
    const job = await run([
      c('doc_create', {from: 'text', text: 'Public text SECRETWORD private'}),
      c('redact_mark', {doc: 1, find: 'SECRETWORD'}), c('redact_apply', {doc: 1}),
      c('doc_save', {doc: 1, path: 'redacted.pdf'}), c('doc_open', {path: 'redacted.pdf'}), c('text_extract', {doc: 2}),
    ]);
    assert(!JSON.stringify(v(job, -1)).includes('SECRETWORD'));
    assert.match(JSON.stringify(v(job, -1)), /Public/);
  });
  await journey('optimization, accessibility, preflight, Office export and compare', async () => {
    const job = await run([
      c('doc_open', {path: 'input.pdf'}),
      c('doc_optimize', {doc: 1, path: 'optimized.pdf'}), c('accessibility_check', {doc: 1}),
      c('accessibility_report', {doc: 1, path: 'report.html'}), c('pdfa_verify', {doc: 1}),
      c('doc_export_office', {doc: 1, path: 'export.docx'}),
      c('doc_create', {from: 'text', text: 'Worker journey changed world'}), c('doc_compare', {doc: 2, other: 1}),
      c('doc_compare_report', {doc: 2, other: 1, path: 'compare.pdf'}),
    ], [upload('input.pdf', pdf)]);
    assert.equal(bytes(job, '/export.docx').subarray(0, 2).toString(), 'PK');
    assert.match(bytes(job, '/report.html').toString(), /html/i);
    assert.equal(bytes(job, '/compare.pdf').subarray(0, 5).toString(), '%PDF-');
    assert(v(job, 7).changes.length > 0);
  });
  await journey('no adapter dimension, upload byte, or call count caps', async () => {
    const big = Buffer.alloc(2 * 1024 * 1024 + 1, 65);
    const job = await run([
      c('doc_create', {from: 'blank', width: 1025, height: 3}),
      ...Array.from({length: 130}, () => c('doc_list')),
      c('page_render', {doc: 1, page: 1, dpi: 72}),
    ], [upload('large.bin', big)]);
    assert.equal(job.results.length, 132);
    assert.equal(png(Buffer.from(job.results.at(-1)[0].base64, 'base64')).width, 1025);
    assert.deepEqual(bytes(job, '/large.bin'), big);
  });
  await journey('hardware errors, schema errors, corrupt input, request isolation and recovery', async () => {
    for (const name of ['printers', 'sign_keychain_ids', 'ocr_recognize']) {
      const job = await run([c(name, name === 'ocr_recognize' ? {doc: 1} : {})], [], 400);
      assert.match(job.error, /unavailable/i);
    }
    assert.match((await run([c('not_a_tool')], [], 400)).error, /unknown tool/);
    assert.match((await run([c('doc_create', {from: 'blank', typo: 1})], [], 400)).error, /typo/);
    await run([c('doc_open', {path: 'bad.pdf'})], [upload('bad.pdf', 'not a PDF')], 400);
    await run([c('doc_open', {path: '/etc/passwd'})], [], 400);
    await run([], [upload('../escape', 'x')], 400);
    const fail = await run([c('doc_create', {from: 'blank'}), c('doc_save', {doc: 1, path: 'preserved.pdf'}), c('not_a_tool')], [], 400);
    assert.equal(bytes(fail, '/preserved.pdf').subarray(0, 5).toString(), '%PDF-');
    assert.equal(v(await run([c('doc_list')]), 0).documents.length, 0);
    const isolated = await Promise.all(Array.from({length: 3}, (_, i) => run([c('doc_create', {from: 'blank', pages: i + 1}), c('doc_list')])));
    isolated.forEach((job, i) => assert.equal(v(job, 1).documents[0].pages, i + 1));
    assert.equal((await mf.dispatchFetch('https://test/run')).status, 405);
    assert.equal((await mf.dispatchFetch('https://test/missing')).status, 404);
    assert.equal((await mf.dispatchFetch('https://test/run', {method: 'POST', body: '{'})).status, 400);
    assert.equal(v(await run([c('doc_create', {from: 'blank'})]), 0).pages, 1);
  });
  console.log(JSON.stringify({ok: true, runtime: 'Miniflare/workerd', tools: catalogue.length, journeys: completed}));
} finally {
  await mf.dispose();
}
