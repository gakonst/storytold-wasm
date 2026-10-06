import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import assert from 'node:assert/strict';
import {initSync,catalog,font_bytes} from '../pkg-embed/craft_fonts_wasm.js';
initSync({module:await readFile(new URL('../pkg-embed/craft_fonts_wasm_bg.wasm',import.meta.url))});
const fonts=JSON.parse(catalog(''));
for(let i=0;i<fonts.length;i++) assert.equal(createHash('sha256').update(font_bytes(i)).digest('hex'),fonts[i].sha256);
assert.throws(()=>font_bytes(fonts.length));
console.log(JSON.stringify({embedded_fonts_verified:fonts.length}));
