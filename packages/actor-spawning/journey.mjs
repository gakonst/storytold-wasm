export async function journey(call,a) {
 a.deepEqual(await call('location',{position:[1,2,3],offset:[4,5,6],forward:[0,1,0],distance:10}),[5,17,9]);
 a.deepEqual(await call('randomLocation',{position:[0,0,0],ranges:[[-4000,4000],[-4000,4000],[3000,10000]],random:[0,1,0.5,0]}),[-4000,4000,6500]);
 await call('location',{position:[1,2]},400);await call('randomLocation',{position:[0,0,0],ranges:[[2,1],[0,0],[0,0]]},400);
 a.deepEqual(await call('location',{position:[0,0,0]}),[0,0,0]);
}
