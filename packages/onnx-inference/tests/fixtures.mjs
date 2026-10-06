// Minimal protobuf wire encoder: fixtures are real ONNX ModelProto messages.
const cat=(...x)=>Buffer.concat(x.map(x=>Buffer.from(x)));
const varint=n=>{const b=[];do{b.push((n&127)|(n>127?128:0));n=Math.floor(n/128);}while(n);return Buffer.from(b);};
const num=(field,n)=>cat(varint(field*8),varint(n));
const blob=(field,b)=>cat(varint(field*8+2),varint(Buffer.byteLength(b)),Buffer.from(b));
export function model(op, names=['x','b'], tensors={}, outputShape=[1]) {
  const info=(name,shape)=>cat(blob(1,name),blob(2,blob(1,cat(num(1,1),blob(2,cat(...shape.map(d=>blob(1,num(1,d)))))))));
  const node=cat(...names.map(n=>blob(1,n)),blob(2,'y'),blob(4,op));
  const graph=cat(blob(1,node),blob(2,'test'),...names.map(n=>blob(11,info(n,tensors[n]?.shape || [1]))),blob(12,info('y',outputShape)));
  return cat(num(1,8),blob(7,graph),blob(8,num(2,13)));
}
export function safetensors(tensors) {
  const header={};const data=[];let offset=0;
  for(const [name,{shape,values}] of Object.entries(tensors)) {
    const b=Buffer.alloc(values.length*4);values.forEach((v,i)=>b.writeFloatLE(v,i*4));
    header[name]={dtype:'F32',shape,data_offsets:[offset,offset+b.length]};offset+=b.length;data.push(b);
  }
  let h=JSON.stringify(header);h+=' '.repeat((8-Buffer.byteLength(h)%8)%8);
  const length=Buffer.alloc(8);length.writeBigUInt64LE(BigInt(Buffer.byteLength(h)));
  return cat(length,Buffer.from(h),...data);
}
export function decode(bytes) {
  const b=Buffer.from(bytes);const n=Number(b.readBigUInt64LE());const h=JSON.parse(b.subarray(8,8+n));
  return Object.fromEntries(Object.entries(h).map(([name,t])=>[name,{...t,values:Array.from({length:(t.data_offsets[1]-t.data_offsets[0])/4},(_,i)=>b.readFloatLE(8+n+t.data_offsets[0]+i*4))}]));
}
export function envelope(m,inputs) {const n=Buffer.alloc(4);n.writeUInt32LE(m.length);return cat(n,m,inputs);}
