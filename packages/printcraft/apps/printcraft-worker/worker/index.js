import wasm from '../pkg/printcraft_worker_bg.wasm';
import {initSync, PrintCraft, tools} from '../pkg/printcraft_worker.js';

initSync({module: wasm});
const decode = value => {
  if (typeof value !== 'string') throw new Error('base64 must be a string');
  return Uint8Array.from(atob(value), c => c.charCodeAt(0));
};
function encode(bytes) {
  let binary = '';
  // Chunking only avoids JavaScript's function argument limit; it does not limit the file.
  for (let i = 0; i < bytes.length; i += 32768) binary += String.fromCharCode(...bytes.subarray(i, i + 32768));
  return btoa(binary);
}
const object = value => value !== null && typeof value === 'object' && !Array.isArray(value);
const message = error => String(error?.message ?? error);
function files(engine) {
  return JSON.parse(engine.files()).map(path => ({path, base64: encode(engine.file(path))}));
}

export default {
  async fetch(request) {
    const url = new URL(request.url);
    if (!['/tools', '/run', '/render'].includes(url.pathname)) return new Response('Not found', {status: 404});
    const method = url.pathname === '/tools' ? 'GET' : 'POST';
    if (request.method !== method) return new Response('Method not allowed', {status: 405, headers: {allow: method}});
    if (url.pathname === '/tools') return new Response(tools(), {headers: {'content-type': 'application/json'}});
    let engine;
    const results = [];
    try {
      engine = new PrintCraft();
      if (url.pathname === '/render') {
        engine.put_file('input.pdf', new Uint8Array(await request.arrayBuffer()));
        const opened = JSON.parse(engine.call('doc_open', JSON.stringify({path: 'input.pdf', ...(url.searchParams.has('password') ? {password: url.searchParams.get('password')} : {})})));
        const result = JSON.parse(engine.call('page_render', JSON.stringify({doc: opened[0].value.doc, page: Number(url.searchParams.get('page') ?? 1), dpi: Number(url.searchParams.get('dpi') ?? 96)})))[0];
        return new Response(decode(result.base64), {headers: {'content-type': 'image/png', 'x-image-width': String(result.width), 'x-image-height': String(result.height)}});
      }
      const job = await request.json();
      if (!object(job)) throw new Error('job must be an object');
      if (job.files !== undefined && !Array.isArray(job.files)) throw new Error('files must be an array');
      if (job.calls !== undefined && !Array.isArray(job.calls)) throw new Error('calls must be an array');
      for (const file of job.files ?? []) {
        if (!object(file) || typeof file.path !== 'string') throw new Error('each file needs a path and base64');
        engine.put_file(file.path, decode(file.base64));
      }
      for (const call of job.calls ?? []) {
        if (!object(call) || typeof call.name !== 'string') throw new Error('each call needs a tool name');
        results.push(JSON.parse(engine.call(call.name, JSON.stringify(call.arguments ?? {}))));
      }
      if (job.download !== undefined) {
        if (typeof job.download !== 'string') throw new Error('download must be a virtual file path');
        return new Response(engine.file(job.download), {headers: {'content-type': 'application/octet-stream'}});
      }
      return Response.json({results, files: files(engine)});
    } catch (error) {
      // Return successful preceding results and files so a failed later call loses no saved output.
      return Response.json({error: message(error), results, files: engine ? files(engine) : []}, {status: 400});
    } finally {
      engine?.free();
    }
  }
};
