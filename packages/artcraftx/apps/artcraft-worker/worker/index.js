import { initSync, catalog, model_config, inspect_bytes, utility, decode_base64,
  image_info, image_thumbnail, transform_image, normalize_flux_mask, image_to_png, webp_to_png } from '../pkg/artcraft_worker.js';
// A statically imported WebAssembly.Module works under workerd's code-generation
// restrictions. No browser globals, file fetches, eval, or WASI shim.
import wasm from './artcraft_worker_bg.wasm';
initSync({ module: wasm });

const unavailable = {
  generation: 'Native router/provider clients depend on native TLS, sessions and desktop integration; no provider execution is implemented.',
  filesystem: 'Worker requests carry bytes; native paths, filesystem traversal and file dialogs are unavailable.',
  database: 'Native SQLx/SQLite task and local-file storage is unavailable. No D1/storage parity is claimed.',
  browser: 'Tauri/WebView, cookie harvesting, Cloudflare/DataDome mitigation and live Statsig fingerprinting require a real browser.',
  video: 'Native OpenH264 decoding and animated WebP thumbnail encoding are unavailable in this build.',
  hardware: 'Desktop, GPU, hardware and OS integration are unavailable.'
};
const operations = ['resize','resize_exact','resize_filter','resize_to_fill','thumbnail','thumbnail_exact','crop',
  'rotate90','rotate180','rotate270','flip_horizontal','flip_vertical','grayscale','invert','blur','fast_blur',
  'unsharpen','filter3x3','contrast','brighten','hue_rotate','color'];
const json = (body, status = 200, headers = {}) => Response.json(body, { status, headers });
const rustJson = value => new Response(value, { headers: { 'content-type': 'application/json' } });
const binary = bytes => {
  const meta = JSON.parse(inspect_bytes(bytes));
  return new Response(bytes, { headers: { 'content-type': meta.mime, 'x-content-type-options': 'nosniff', 'x-artcraft-blake3': meta.blake3 } });
};

export default {
  async fetch(request) {
    const url = new URL(request.url);
    const path = url.pathname;
    try {
      if (path === '/' || path === '/v1/capabilities') {
        if (request.method !== 'GET') return json({ error: 'method_not_allowed' }, 405, { Allow: 'GET' });
        return json({ runtime: 'wasm-bindgen', modalities: ['image','video','mesh','splat','audio'],
          image_operations: operations, unavailable, limits: 'No application byte, dimension, operation or item-count caps. Host resource and codec constraints apply.',
          endpoints: ['/v1/catalog','/v1/models/:modality/:model?provider=...', '/v1/inspect', '/v1/utilities',
            '/v1/base64/decode','/v1/images/info','/v1/images/png','/v1/images/webp-to-png','/v1/images/flux-mask',
            '/v1/images/thumbnail?dimension=...', '/v1/images/transform'] });
      }
      if (path === '/v1/catalog' || path.startsWith('/v1/models/')) {
        if (request.method !== 'GET') return json({ error: 'method_not_allowed' }, 405, { Allow: 'GET' });
        if (path === '/v1/catalog') return rustJson(catalog());
        const parts = path.split('/');
        if (parts.length !== 5) return json({ error: 'not_found' }, 404);
        return rustJson(model_config(decodeURIComponent(parts[3]), decodeURIComponent(parts[4]), url.searchParams.get('provider') ?? undefined));
      }
      const component = path.split('/')[2];
      if (Object.hasOwn(unavailable, component)) return json({ error: 'unavailable', component, reason: unavailable[component] }, 501);
      const routes = {
        '/v1/inspect': async () => rustJson(inspect_bytes(new Uint8Array(await request.arrayBuffer()))),
        '/v1/utilities': async () => rustJson(utility(await request.text())),
        '/v1/base64/decode': async () => binary(decode_base64(await request.text())),
        '/v1/images/thumbnail': async () => {
          const value = url.searchParams.get('dimension');
          const dimension = value === null ? 512 : Number(value);
          if (!Number.isInteger(dimension) || dimension < 1 || dimension > 0xffffffff) {
            return json({ error: 'invalid_request', message: 'dimension must be a positive u32' }, 400);
          }
          return binary(image_thumbnail(new Uint8Array(await request.arrayBuffer()), dimension));
        },
        '/v1/images/info': async () => rustJson(image_info(new Uint8Array(await request.arrayBuffer()))),
        '/v1/images/png': async () => binary(image_to_png(new Uint8Array(await request.arrayBuffer()))),
        '/v1/images/webp-to-png': async () => binary(webp_to_png(new Uint8Array(await request.arrayBuffer()))),
        '/v1/images/flux-mask': async () => binary(normalize_flux_mask(new Uint8Array(await request.arrayBuffer()))),
        '/v1/images/transform': async () => {
          // Multipart keeps binary images binary and options out of constrained URL/header fields.
          const form = await request.formData();
          const image = form.get('image');
          const options = form.get('options');
          if (!image || typeof image.arrayBuffer !== 'function' || typeof options !== 'string') {
            return json({ error: 'invalid_request', message: 'multipart form requires image (file) and options (JSON string)' }, 400);
          }
          return binary(transform_image(new Uint8Array(await image.arrayBuffer()), options));
        }
      };
      if (!Object.hasOwn(routes, path)) return json({ error: 'not_found' }, 404);
      if (request.method !== 'POST') return json({ error: 'method_not_allowed' }, 405, { Allow: 'POST' });
      return await routes[path]();
    } catch (error) {
      // Runtime traps are server failures, not successful fake outputs or invalid input.
      if (error instanceof WebAssembly.RuntimeError) return json({ error: 'wasm_runtime_failure', message: String(error) }, 500);
      return json({ error: 'invalid_request', message: String(error) }, 400);
    }
  }
};
