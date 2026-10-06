import { initSync, HeadlessSession } from '../generated/filmcraft_worker.js';
import wasm from '../generated/filmcraft_worker_bg.wasm';

function bytesFromBase64(value) {
  if (typeof value !== 'string') throw new Error('file base64 must be a string');
  return Uint8Array.from(atob(value), c => c.charCodeAt(0));
}
function base64FromBytes(bytes) {
  // Chunking avoids JS's argument-stack limit; it does not limit file size.
  let text = '';
  for (let i = 0; i < bytes.length; i += 16384) {
    text += String.fromCharCode(...bytes.subarray(i, i + 16384));
  }
  return btoa(text);
}
function object(value, name) {
  if (value === null || typeof value !== 'object' || Array.isArray(value)) throw new Error(`${name} must be an object`);
  return value;
}
function resolve(value, results) {
  if (Array.isArray(value)) return value.map(v => resolve(v, results));
  if (value && typeof value === 'object') {
    if (Object.keys(value).length === 1 && typeof value.$ref === 'string') {
      const parts = value.$ref.split('.');
      let current = results;
      for (const key of parts) {
        if (current === null || typeof current !== 'object' || !Object.hasOwn(current, key)) throw new Error(`unresolved result reference: ${value.$ref}`);
        current = current[key];
      }
      return current;
    }
    return Object.fromEntries(Object.entries(value).map(([k, v]) => [k, resolve(v, results)]));
  }
  return value;
}
function snapshot(session, results) {
  return {
    results,
    project: new TextDecoder().decode(session.project()),
    files: JSON.parse(session.files()).map(file => ({ ...file, base64: base64FromBytes(session.read_file(file.path)) })),
  };
}

export default {
  async fetch(request) {
    const url = new URL(request.url);
    if (request.method === 'GET' && url.pathname === '/') {
      return Response.json({
        engine: 'FilmCraft', runtime: 'wasm32-unknown-unknown',
        endpoints: ['GET /commands', 'POST /run', 'POST /render', 'POST /file?path=...'],
        persistence: 'request-local; return project and files to continue in another request; undo history is request-local',
        compute: 'CPU; no DOM, GPU, hardware codecs, microphone, native filesystem, or background threads',
        limits: 'No adapter byte, dimension, command-count caps; engine format validation and runtime resource limits apply',
      });
    }
    if (!((request.method === 'GET' && url.pathname === '/commands') ||
          (request.method === 'POST' && ['/run', '/render', '/file'].includes(url.pathname)))) {
      return Response.json({ error: 'unknown endpoint or method' }, { status: 404 });
    }
    let body;
    try {
      body = request.method === 'GET' ? {} : object(await request.json(), 'request');
      if (body.files !== undefined && !Array.isArray(body.files)) throw new Error('files must be an array');
      if (body.commands !== undefined && !Array.isArray(body.commands)) throw new Error('commands must be an array');
    } catch (error) { return Response.json({ error: String(error) }, { status: 400 }); }

    // Initialization and all WASM calls run inside fetch, where Workers permit clocks.
    // There are no awaits while a session is live: requests cannot interleave engine calls.
    let session;
    const results = [];
    try {
      initSync({ module: wasm });
      session = new HeadlessSession();
      if (url.pathname === '/commands') return Response.json(JSON.parse(session.execute('command.list', '{}')));
      for (const file of body.files ?? []) {
        object(file, 'file');
        if (typeof file.path !== 'string') throw new Error('file path must be a string');
        session.put_file(file.path, bytesFromBase64(file.base64));
      }
      if (body.project !== undefined) {
        if (typeof body.project !== 'string') throw new Error('project must be a .fcproj JSON string (preserves exact integer ticks)');
        session.put_file('/__request_project.fcproj', new TextEncoder().encode(body.project));
        session.execute('file.open', '{"path":"/__request_project.fcproj"}');
      }
      for (const entry of body.commands ?? []) {
        object(entry, 'command');
        if (typeof entry.command !== 'string') throw new Error('command must be a string');
        const params = resolve(entry.params ?? {}, results);
        results.push(JSON.parse(session.execute(entry.command, JSON.stringify(params))));
        // All input files are already resident; exports run cooperatively until complete.
        // No arbitrary step ceiling or elapsed-time deadline is imposed.
        while (session.pump()) { /* advance the real upstream export queue */ }
      }
      if (url.pathname === '/render') {
        const scale = body.scale ?? 1;
        if (typeof scale !== 'number' || !Number.isFinite(scale) || scale <= 0) throw new Error('scale must be finite and positive');
        return new Response(session.render_png(scale), { headers: { 'content-type': 'image/png' } });
      }
      if (url.pathname === '/file') {
        const path = url.searchParams.get('path');
        if (!path) throw new Error('path query parameter required');
        return new Response(session.read_file(path), { headers: { 'content-type': 'application/octet-stream' } });
      }
      return Response.json(snapshot(session, results));
    } catch (error) {
      // Preserve completed commands and file outputs after ordinary engine errors.
      // Traps cannot unwind Rust on wasm32; report them distinctly, never as successful work.
      if (error instanceof WebAssembly.RuntimeError) return Response.json({ error: String(error), kind: 'wasm-trap', results }, { status: 500 });
      let partial;
      try { if (session) partial = snapshot(session, results); } catch { /* original error wins */ }
      return Response.json({ ...partial, results, error: String(error), failedCommand: results.length }, { status: 422 });
    } finally { session?.free(); }
  },
};
