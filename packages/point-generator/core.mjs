let wasm;
export function initialize(module) { wasm = new WebAssembly.Instance(module).exports; }
export async function load(url = new URL('./output/core.wasm', import.meta.url)) {
  initialize(await WebAssembly.compile(await (await fetch(url)).arrayBuffer()));
}
function ready() { if (!wasm) throw new Error('Call load() or initialize(module) first'); }
function number(value) { if (typeof value !== 'number' || !Number.isFinite(value)) throw new Error('Expected finite number'); return value; }
function vector(value, size=3) { if (!Array.isArray(value) || value.length !== size) throw new Error(`Expected ${size} coordinates`); return value.map(number); }

export function encode({points}) {
  if (!Array.isArray(points)) throw new Error('Expected points');
  const bytes=new Uint8Array(points.length*32), view=new DataView(bytes.buffer);
  points.forEach((p,i)=> {
    const xyz=vector(p.xyz), rgba=vector(p.rgba ?? [1,1,1,1],4);
    [...xyz,0,...rgba].forEach((v,j)=>{ if (!Number.isFinite(Math.fround(v))) throw new Error('Float32 overflow'); view.setFloat32(i*32+j*4,v,true); });
  });
  return Array.from(bytes);
}
export function decode({bytes}) {
  if (!Array.isArray(bytes) || bytes.some(v=>!Number.isInteger(v)||v<0||v>255) || bytes.length%32) throw new Error('Expected complete 32-byte point records');
  const v=new DataView(Uint8Array.from(bytes).buffer), points=[];
  for(let i=0;i<bytes.length;i+=32) {
    const fields=Array.from({length:8},(_,j)=>number(v.getFloat32(i+j*4,true)));
    points.push({xyz:fields.slice(0,3),rgba:fields.slice(4)});
  }
  return points;
}
export function generate({count,min=-1000,max=1000,seconds=0,seed=1}) {
  ready(); number(min);number(max);
  if (!Number.isSafeInteger(count)||count<0||min>max||!Number.isFinite(Math.fround(max-min))) throw new Error('Invalid count or range');
  if(!Number.isInteger(seed)||seed<1||seed>0xffffffff||!Number.isSafeInteger(seconds)||seconds<0) throw new Error('Invalid seed or seconds');
  let state=seed;
  const rgb=wasm.color(seconds%10), rgba=[(rgb>>>16)/255,((rgb>>>8)&255)/255,(rgb&255)/255,1];
  const points=Array.from({length:count},()=>({xyz:Array.from({length:3},()=>{state=wasm.random_next(state)>>>0;return wasm.coordinate(state,min,max);}),rgba}));
  return {points,bytes:encode({points}),seed:state};
}
