import wasm from './effectcraft_worker_bg.wasm';
import { bindings } from './dist/bindings.js';

const capabilities = {
  engine: 'EffectCraft', runtime: 'wasm32 / Cloudflare Workers',
  commands: 'Full upstream registry via command.list and command.describe; no allowlist',
  files: 'Request-local byte store. Supply files again with the returned project on the next request.',
  render: 'CPU compositor; PNG frames and upstream Render Queue encoders to memory',
  plugins: 'Portable wasmi interpreter', expressions: 'Boa', scripting: 'Boa; globals persist within one request only',
  unavailable: ['GPU/WebGPU', 'desktop UI and dialogs', 'host filesystem and directory watching',
    'native audio/video hardware', 'native sockets/processes', 'disk cache',
    'installed ML models'],
  limits: 'No adapter byte/dimension/count caps. Upstream validation and actual runtime resources apply.',
};

function object(value, label) {
  if (!value || typeof value !== 'object' || Array.isArray(value)) throw new Error(`${label} must be an object`);
  return value;
}
function decode(base64) {
  if (typeof base64 !== 'string') throw new Error('file.base64 must be a string');
  return Uint8Array.from(atob(base64), c => c.charCodeAt(0));
}
function encode(bytes) {
  // Chunk the conversion to avoid JavaScript argument/stack limits; no data-size cap.
  let text = '';
  for (let offset = 0; offset < bytes.length; offset += 8192) text += String.fromCharCode(...bytes.subarray(offset, offset + 8192));
  return btoa(text);
}
function errorMessage(error) { return error instanceof Error ? error.message : String(error); }
const json = (body, status = 200) => Response.json(body, { status });

export default {
  async fetch(request) {
    const url = new URL(request.url);
    if (request.method === 'GET' && url.pathname === '/capabilities') return json(capabilities);
    if (!['/run', '/render'].includes(url.pathname)) return json({ error: 'Use GET /capabilities, POST /run or POST /render' }, 404);
    if (request.method !== 'POST') return new Response(null, { status: 405, headers: { Allow: 'POST' } });
    let engine;
    try {
      const body = object(await request.json(), 'request');
      for (const key of Object.keys(body)) {
        if (!['project', 'files', 'commands', 'render'].includes(key)) throw new Error(`Unknown request field: ${key}`);
      }
      const commands = body.commands ?? [];
      const files = body.files ?? [];
      if (!Array.isArray(commands) || !Array.isArray(files)) throw new Error('commands and files must be arrays');
      const api = bindings();
      api.initSync({ module: wasm });
      engine = new api.Engine();
      for (const file of files) {
        object(file, 'file');
        if (typeof file.path !== 'string') throw new Error('file.path must be a string');
        engine.put_file(file.path, decode(file.base64));
      }
      if (body.project !== undefined) engine.load_project(typeof body.project === 'string' ? body.project : JSON.stringify(body.project));
      const results = [];
      let failed = false;
      for (const step of commands) {
        object(step, 'command');
        if (typeof step.command !== 'string') throw new Error('command.command must be a string');
        try {
          results.push({ ok: true, result: JSON.parse(engine.execute(step.command, JSON.stringify(step.params ?? {}))) });
        } catch (error) {
          results.push({ ok: false, error: errorMessage(error) });
          failed = true;
          // Return recoverable state and stop at the first failed command; prior edits remain.
          break;
        }
      }
      if (url.pathname === '/render' && !failed) {
        return new Response(engine.render_png(JSON.stringify(body.render ?? {})), { headers: { 'Content-Type': 'image/png' } });
      }
      const output = { ok: !failed, results, project: JSON.parse(engine.project()), state: JSON.parse(engine.state()),
        files: JSON.parse(engine.files()).map(file => ({ ...file, base64: encode(engine.get_file(file.path)) })) };
      if (body.render && !failed) output.render = { mime: 'image/png', base64: encode(engine.render_png(JSON.stringify(body.render))) };
      return json(output, failed ? 422 : 200);
    } catch (error) {
      return json({ ok: false, error: errorMessage(error) }, error instanceof WebAssembly.RuntimeError ? 500 : 400);
    } finally { engine?.free(); }
  },
};
