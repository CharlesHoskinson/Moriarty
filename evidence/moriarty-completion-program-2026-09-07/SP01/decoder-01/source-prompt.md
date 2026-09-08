You are the user-selected Grok4.6 high implementation worker for Moriarty. Make the concrete Edit/Write calls immediately. Parent completed design review and execution admission. Do not begin another planning/permission cycle. Root runs tests and independent GPT6 review. No shell commands, tests, Git writes, provider/subagent calls, dependencies, native proofs, wallets, or web.

OBJECTIVE: Implement the reviewed decoder correction. This is source-only phase two after parent-verified RED.
FILES: Work only in /home/charl/Moriarty/.worktrees/sp01-grok-high. Own experiments/moriarty-language/src/evaluate.ts and src/node-shims.d.ts plus FOREMAN_REPORT.md/.json. The tests are frozen. Never edit the test file.
INTERFACES: Preserve decodeEvaluation(string|Uint8Array):EvaluationInput and its throwing RuntimeError API. Reuse registeredBounds, canonicalDecode and admitEvaluation. Here is the complete existing function to replace:
```ts
export function decodeEvaluation(bytes:string|Uint8Array):R.EvaluationInput{
 need((typeof bytes==='string'?new TextEncoder().encode(bytes).length:bytes.length)<=65536,'INPUT_BOUNDS',9);
 try{return canonicalDecode(bytes,inputSchema) as R.EvaluationInput;}catch(e){if(e instanceof RuntimeError)throw e;throw new RuntimeError('INPUT_SCHEMA',9);}
}
```
Replace it with:
```ts
export function decodeEvaluation(bytes:string|Uint8Array):R.EvaluationInput{
 try{
  const registered=registeredBounds().bounds;
  const limits=registered as typeof registered & {
   evaluationEncoding:EncodingLimits;signingEnvelope:EncodingLimits
  };
  const maximum=limits.evaluationEncoding.utf8Bytes;
  let admitted:string|Uint8Array;
  if(typeof bytes==='string'){
   // Primitive strings are immutable. Check code units before scanning, then
   // count scalar UTF-8 bytes without allocating an encoded caller string.
   need(bytes.length<=maximum,'INPUT_BOUNDS',9);
   need(scalarText(bytes),'INPUT_SCHEMA',9);
   let length=0;
   for(let j=0;j<bytes.length;j++){
    const c=bytes.charCodeAt(j);
    if(c<0x80)length++;
    else if(c<0x800)length+=2;
    else if(c>=0xd800&&c<=0xdbff){length+=4;j++;}
    else length+=3;
    need(length<=maximum,'INPUT_BOUNDS',9);
   }
   admitted=bytes;
  }else{
   // Native brand tests reject proxies and impostors without property access.
   // Never read caller .length, .buffer, .constructor or Symbol.iterator.
   need(!nodeTypes.isProxy(bytes)&&nodeTypes.isUint8Array(bytes),'INPUT_SCHEMA',9);
   const length=evaluationByteLength.call(bytes) as number;
   need(length<=maximum,'INPUT_BOUNDS',9);
   const snapshot=new Uint8Array(length);
   // The native TypedArray-to-TypedArray copy uses internal slots. Its target
   // capacity is fixed before copying, even for shared/resizable source views.
   // Detached/out-of-bounds/native-copy failures normalize in the catch below.
   evaluationByteSet.call(snapshot,bytes);
   admitted=snapshot;
  }
  const decoded=canonicalDecode(admitted) as R.EvaluationInput;
  return admitEvaluation(decoded,limits);
 }catch(e){
  if(e instanceof RuntimeError)throw e;
  throw new RuntimeError('INPUT_SCHEMA',9);
 }
}
```
After the existing `const UNKNOWN={startByte:'0',endByte:'0'};` insert:
```ts
const evaluationByteLength=Object.getOwnPropertyDescriptor(
 Object.getPrototypeOf(Uint8Array.prototype),'byteLength'
)!.get!;
const evaluationByteSet=Uint8Array.prototype.set;
```
Replace the existing last declaration `declare module 'node:util' { export const types:{isProxy(value:unknown):boolean}; }` with:
```ts
declare module 'node:util' {
  export const types:{isProxy(value:unknown):boolean;isUint8Array(value:unknown):value is Uint8Array};
}
```
CONSTRAINTS: All required imports already exist. Do not refactor unrelated functions or change registered bounds. Trusted intrinsics and registry are assumed; the copy creates private parsed bytes but promises no shared-writer atomicity. Do not claim semantic, signature, proof or ledger validity merely from decoder admission.
VERIFICATION: Parent runs frozen focused regressions, complete semantics tests, typecheck and complete language suite including existing Compact --skip-zk lowering. Parent then freezes the source and requests fresh GPT6 result review. Update FOREMAN_REPORT.md/.json with actual changes, no self-run tests, and deviations. Finish within 1190 seconds.

Parent verified RED: 11 focused tests, 9 substantive decoder failures and 2 passing controls. No import/fixture failure. Frozen test SHA256 3eac34c2bdb2fd818991d39dfe815ba5ab8884e84bb9210f2ae5d7b5eacadcef. Do not edit tests.
