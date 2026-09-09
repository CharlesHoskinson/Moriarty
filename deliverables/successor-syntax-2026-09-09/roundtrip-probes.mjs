import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
const root = process.argv[2];
const { parseSuccessorSource: parse } = await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/frontend.ts`));
const { formatSuccessorSource: format } = await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/format.ts`));
const clean = v => Array.isArray(v) ? v.map(clean) : v && typeof v === 'object' ? Object.fromEntries(Object.entries(v).filter(([k])=>k!=='span').map(([k,x])=>[k,clean(x)])) : v;
let seed=918273;const rnd=n=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed%n;};
function expr(d){if(!d)return ['x','1','true','"é😀"','"\\u0041"'][rnd(5)];switch(rnd(7)){case 0:return `(${expr(d-1)} + ${expr(d-1)})`;case 1:return `(${expr(d-1)} - ${expr(d-1)})`;case 2:return `(${expr(d-1)} * ${expr(d-1)})`;case 3:return `(${expr(d-1)} ${['==','!=','<=','>'][rnd(4)]} ${expr(d-1)})`;case 4:return `(not ${expr(d-1)})`;case 5:return `(${expr(d-1)} ${['and','or'][rnd(2)]} ${expr(d-1)})`;default:return `f(${expr(d-1)},${expr(d-1)}).value`;}}
for(let i=0;i<500;i++){const source=`profile "moriarty-successor-syntax/0";agreement A {const result:T=${expr(5)};}`;const a=parse(source);const b=format(source);assert.deepEqual(clean(parse(b)),clean(a),`case ${i}`);assert.equal(format(b),b,`idempotence ${i}`);}
console.log(JSON.stringify({scope:'Independent deterministic formatter AST round-trip samples; syntactic expressions need not typecheck',seed:918273,cases:500,pass:true}));
