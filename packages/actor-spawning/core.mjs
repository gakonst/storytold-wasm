let wasm;
export function initialize(module) { wasm = new WebAssembly.Instance(module).exports; }
export async function load(url = new URL('./output/core.wasm', import.meta.url)) {
  initialize(await WebAssembly.compile(await (await fetch(url)).arrayBuffer()));
}
function ready() { if (!wasm) throw new Error('Call load() or initialize(module) first'); }
function number(value) { if (typeof value !== 'number' || !Number.isFinite(value)) throw new Error('Expected finite number'); return value; }
function vector(value, size=3) { if (!Array.isArray(value) || value.length !== size) throw new Error(`Expected ${size} coordinates`); return value.map(number); }

export function location({position,offset=[0,0,0],forward=[0,0,0],distance=0}) {
 ready();position=vector(position);offset=vector(offset);forward=vector(forward);number(distance);
 return position.map((p,i)=>number(wasm.location(p,offset[i],forward[i],distance)));
}
export function randomLocation({position,forward=[0,0,0],ranges,forwardRange=[0,0],random}) {
 ready(); if(!Array.isArray(ranges)||ranges.length!==3) throw new Error('Three offset ranges required');
 const bounds=[...ranges,forwardRange].map(r=>{r=vector(r,2);if(r[0]>r[1])throw new Error('Reversed range');return r;});
 if(random===undefined) random=Array.from(crypto.getRandomValues(new Uint32Array(4)),v=>v/0xffffffff);
 random=vector(random,4);if(random.some(x=>x<0||x>1))throw new Error('Random values must be in [0,1]');
 const values=bounds.map((b,i)=>number(wasm.between(...b,random[i])));
 return location({position,forward,offset:values.slice(0,3),distance:values[3]});
}
