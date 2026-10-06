import wasm from '../pkg/lightcraft_worker_bg.wasm';
import { initSync, LightCraft, metadata } from '../pkg/lightcraft_worker.js';
initSync({ module: wasm });

const types = { png: 'image/png', jpeg: 'image/jpeg', jpg: 'image/jpeg', tiff: 'image/tiff', tif: 'image/tiff', webp: 'image/webp', dng: 'image/dng' };
const decode64 = text => {
  if (typeof text !== 'string') throw new Error('base64 must be a string');
  return Uint8Array.from(atob(text), c => c.charCodeAt(0));
};
function encode64(bytes) {
  let text = '';
  for (let i = 0; i < bytes.length; i += 32768) text += String.fromCharCode(...bytes.subarray(i, i + 32768));
  return btoa(text);
}
function object(value, name) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error(`${name} must be an object`);
  return value;
}
function array(value, name) {
  if (!Array.isArray(value)) throw new Error(`${name} must be an array`);
  return value;
}
function photoId(value = 0) {
  if (typeof value === 'number' && (!Number.isSafeInteger(value) || value < 0)) throw new Error('photo id must be a nonnegative safe integer or decimal string');
  if (typeof value !== 'number' && (typeof value !== 'string' || !/^\d+$/.test(value))) throw new Error('invalid photo id');
  const id = BigInt(value);
  if (id > 18446744073709551615n) throw new Error('photo id exceeds engine u64 representation');
  return id;
}
function attachment(engine, output = {}) {
  object(output, 'output');
  const { id, ...options } = output;
  const bytes = engine.export(photoId(id), JSON.stringify(options));
  const info = JSON.parse(engine.export_info());
  return { bytes, info: { ...info, sidecars: info.sidecars.map(([extension, data]) => ({ extension, base64: encode64(Uint8Array.from(data)) })) } };
}

export default {
  async fetch(request) {
    const url = new URL(request.url);
    try {
      if (url.pathname === '/commands') {
        if (request.method !== 'GET') return new Response('Method not allowed', { status: 405, headers: { allow: 'GET' } });
        const engine = new LightCraft('', false);
        try { return new Response(engine.commands(), { headers: { 'content-type': 'application/json' } }); }
        finally { engine.free(); }
      }
      if (!['/run', '/render', '/convert', '/metadata', '/merge'].includes(url.pathname)) return new Response('Not found', { status: 404 });
      if (request.method !== 'POST') return new Response('Method not allowed', { status: 405, headers: { allow: 'POST' } });
      if (url.pathname === '/metadata') return new Response(metadata(new Uint8Array(await request.arrayBuffer())), { headers: { 'content-type': 'application/json' } });
      if (url.pathname === '/convert') {
        const engine = new LightCraft('', false);
        try {
          const imported = JSON.parse(engine.import(url.searchParams.get('name') || 'input', new Uint8Array(await request.arrayBuffer()), '{}'));
          if (!imported.imported?.length) throw new Error(JSON.stringify(imported));
          const output = object(JSON.parse(url.searchParams.get('options') || '{}'), 'options');
          output.format = url.searchParams.get('format') || output.format || 'png';
          const { bytes } = attachment(engine, output);
          return new Response(bytes, { headers: { 'content-type': types[output.format] || 'application/octet-stream' } });
        } finally { engine.free(); }
      }
      const job = object(await request.json(), 'job');
      const state = job.state === undefined ? { version: 1, snapshot: '', originals: [] } : object(job.state, 'state');
      if (state.version !== 1 || typeof state.snapshot !== 'string') throw new Error('unsupported state; expected version 1 and snapshot string');
      const engine = new LightCraft(state.snapshot, job.demo === true);
      try {
        const originals = new Map();
        for (const original of array(state.originals, 'state.originals')) {
          const path = engine.upload(original.name, decode64(original.base64));
          originals.set(path, original);
        }
        const imported = [];
        for (const input of array(job.inputs ?? [], 'inputs')) {
          object(input, 'input');
          if (typeof input.name !== 'string') throw new Error('input.name must be a string');
          const bytes = decode64(input.base64);
          const path = engine.upload(input.name, bytes);
          originals.set(path, { name: input.name, base64: input.base64 });
          const result = input.import === false ? null : JSON.parse(engine.import(input.name, bytes, JSON.stringify(input.options ?? {})));
          imported.push({ path, result });
        }
        const results = [];
        for (const command of array(job.commands ?? [], 'commands')) {
          object(command, 'command');
          if (typeof command.id !== 'string') throw new Error('command.id must be a string');
          results.push(JSON.parse(engine.execute(command.id, JSON.stringify(command.params ?? {}))));
        }
        if (url.pathname === '/merge') {
          const merge = object(job.merge, 'merge');
          if (!['merge.hdr', 'merge.panorama', 'merge.hdrPanorama'].includes(merge.id)) throw new Error('unknown merge operation');
          const bytes = engine.merge(merge.id, JSON.stringify(merge.params ?? {}));
          return new Response(bytes, { headers: { 'content-type': 'image/dng' } });
        }
        if (url.pathname === '/render') {
          const output = job.output ?? { format: 'png' };
          const { bytes } = attachment(engine, output);
          return new Response(bytes, { headers: { 'content-type': types[output.format || 'jpeg'] || 'application/octet-stream' } });
        }
        const outputs = array(job.outputs ?? [], 'outputs').map(output => {
          const { bytes, info } = attachment(engine, output);
          return { ...info, base64: encode64(bytes) };
        });
        const nextState = { version: 1, snapshot: engine.snapshot(), originals: [...originals.values()] };
        return Response.json({ imported, results, outputs, state: nextState });
      } finally { engine.free(); }
    } catch (error) {
      const message = String(error?.message || error);
      return Response.json({ error: message }, { status: message.startsWith('unavailable:') ? 501 : 400 });
    }
  },
};
