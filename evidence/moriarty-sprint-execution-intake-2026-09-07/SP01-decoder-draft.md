# SP01 Evaluation Decoder Correction Implementation Plan — Draft v2

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. This draft does not dispatch workers or authorize execution before admission.

**Goal:** Make `decodeEvaluation` return only an inert, closed-schema evaluation input admitted under the exact registered atomic profile, while rejecting hostile carriers and excessive input before unbounded conversion or copying.

**Architecture:** Bound primitive strings before conversion; snapshot genuine byte views through captured TypedArray intrinsics without reading caller properties. Decode canonical JSON from the immutable string or private byte copy, then reuse `admitEvaluation` for the evaluation graph and signing-statement limits. Preserve the throwing `EvaluationInput` API and existing semantic execution boundaries.

**Tech Stack:** Node.js 24, TypeScript ES2022/NodeNext, native `node:util` type predicates, existing canonical codec and `node:test`. No package dependency or new execution backend.

## Global Constraints

- **Status: draft preparation only; not admitted for execution, not an executable or dispatched packet, not implemented, not run, not audited, and not an SP01 or MC01 acceptance result.**
- Parent reports the runtime goal is `usageLimited`, Fable quota is unavailable, and native `create_goal` refused creation because an unfinished usage-limited goal exists. This packet neither changes those states, creates a replacement loop, nor substitutes another reviewer. Refresh these observations at execution admission; do not infer live state from this document.
- Preserve `export function decodeEvaluation(bytes:string|Uint8Array):R.EvaluationInput` and synchronous throwing behavior. Runtime admission failures throw an error with `code` and numeric `stage: 9`; they do not return `Rejected` or a union.
- Exact profile: `moriarty-bounded-atomic/1`. Resolve limits only through `registeredBounds().bounds`. No decoder bounds argument, configurable registry, test registry, or production profile change.
- Registered domain-separated bounds hash: `ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c`; registered source byte length: `8861`.
- Both `evaluationEncoding` and `signingEnvelope` currently have: `utf8Bytes=65536`, `decodedDepthRootZero=16`, `decodedNodes=8192`, `keysPerRecord=64`, `arrayLength=128`, `textUtf8Bytes=4096`, `textJavascriptCodeUnits=4096`.
- A 4096-character lowercase even-length hexadecimal signature is within the generic text bound; 4098 characters is outside it. These lengths describe encoded hexadecimal characters, not cryptographic signature verification or 4096 signature bytes.
- Reject proxies, non-byte impostors, detached buffers, and views whose intrinsic byte length exceeds the limit without invoking caller accessors or conversion methods. Accept genuine `Uint8Array` and `Buffer` views even when incidental properties shadow `length`, byte metadata, `constructor`, or iteration; ignore those properties without invoking them. This is deliberately not a ban on all decorations.
- Decoded JSON graph admission remains strict: only permitted inert records/arrays and schema members, with profile encoding limits and signing-statement limits. Byte-carrier decoration tolerance does not relax graph admission.
- Preserve the already implemented atomic snapshot, source-map, authority, and financial semantics. Decode admission does not authenticate signatures, check ledger consumption, or replace `evaluate`/`validate` semantic checks.
- The full language suite may run its existing bounded `compact compile --skip-zk` lowering regressions, once and within the admitted suite allocation. No new Compact proof build, proving-key generation, native proving campaign, public network action, wallet operation, unbounded experiment, source-profile extension, or canonical vault write belongs to this packet. Merely preparing this draft authorizes none of those test executions.
- Required current result audits remain exact Fable 5.1 medium and fresh GPT-6 Astra under repository rules. Consequential design/resource decisions use the separately authorized Fable/GPT-6/Grok 4.6 majority process. Failed or missing providers do not vote and cannot satisfy result audits.

## Inspected baseline and ownership

Repository observed at commit `5de8228be2fc051f6381c50a0271c02afb9872bf`; the working tree contains unrelated parent changes. Preserve them. These file digests identify inspected bytes, not a clean or approved candidate:

| File relative to `/home/charl/Moriarty` | Inspected raw SHA-256 | Ownership |
| --- | --- | --- |
| `experiments/moriarty-language/src/evaluate.ts` | `4df6d0841f206f5bd18dbc716e2243ffbdc6c013caa2a9583c05113dfac9f2fa` | Modify decoder and add two captured intrinsics only |
| `experiments/moriarty-language/src/node-shims.d.ts` | `508e8a692e5de94bf0cc357f7969d3b8bf55c78d458d83d32f517e3cdec2517e` | Add `isUint8Array` declaration |
| `experiments/moriarty-language/tests/semantics.test.mjs` | `5e214ddbadeda7b8abc03ac42a6f8b61de5b779d8764c6338b44efcf65466158` | Add imports and decoder regressions |
| `experiments/moriarty-language/src/registered-bounds.ts` | `e58f3afcd62c8db0bc4de31101048d457d4946f860856f61b9a71b7fb6b31370` | Read only |
| `experiments/moriarty-language/src/codec.ts` | `8575d98e630fed5b6e68b23c574aec450fc2bbce1b83b057ce6154e43610b7e8` | Read only |
| `experiments/moriarty-language/spec/bounds.json` | `b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664` | Read only; raw SHA-256 differs from the domain-separated registry hash |

The current decoder reads caller `.length` for non-string inputs, allocates a UTF-8 encoding before rejecting oversized strings, passes caller bytes directly to `canonicalDecode`, and invokes only `inputSchema`. Existing `admitEvaluation` already freezes an inert graph, enforces `evaluationEncoding`, invokes `inputSchema`, and checks the authority statement against `signingEnvelope`; use it rather than duplicating those checks.

Before admitted edits, compare these digests and reread changed owned files. Rebase the draft on material drift and retain the previous baseline. Do not discard parent work or reset the tree to the inspected commit.

## Isolated-worktree binding required before dispatch

The original `/home/charl/Moriarty` paths identify inspected source provenance. They are not permission to modify the parent's working tree. Before any behavioral edit, RED invocation, or implementation-agent dispatch, the parent must use `superpowers:using-git-worktrees` to prepare an isolated checkout and attach a reviewed execution binding containing all of the following:

- The canonical absolute worktree path, branch, full base commit, and initial clean status. Preserve approved planning and any parent changes needed by the candidate through an explicit baseline; do not silently choose old HEAD or copy a dirty tree without recording its contents.
- The exact packet digest and the admitted three-file ownership set mapped into that checkout. Recheck every baseline digest above against that tree; any mismatch requires a recorded disposition and updated candidate, not an implicit waiver.
- A concrete command working directory equal to that worktree root, plus absolute paths to its evidence destination and allowed temporary-output directory. All relative commands below run only from this bound root. Every absolute implementation path in this draft is remapped by replacing the inspected repository prefix with the bound worktree prefix.
- Node, TypeScript and Compact identities and versions; the resolved read-only `MORIARTY_RUNTIME_NODE_MODULES` dependency path and dependency provenance used by existing lowering tests. The current lowering test has a historical worktree fallback; do not silently inherit that location without explicitly recording and validating the dependency binding.
- The actual resource-enforcement wrapper, process-tree memory and timeout behavior, temporary/retained storage limits, and resource-admission record. Include child Compact processes under the enforcement boundary; a Node heap flag alone does not constrain them.
- The existing goal/runtime eligibility, applicable decision record, and current audit requirements. No alternate goal or automatic restart is implied by writing this binding.

No isolated worktree has been created or bound by this revision. No concrete execution binding, working memory enforcer, or admitted resource record is supplied here. Their absence means this remains a reviewable preparation draft, not an executable packet. The parent must incorporate the binding into a reviewed successor before dispatch; publishing approved sprint planning does not admit this pending implementation packet.

## Resource proposal requiring recorded admission

This is one CPU-only decoder correction cycle. Proposed ceilings: one RED invocation (60 seconds), one focused GREEN invocation (60 seconds), one entire semantics-file invocation (90 seconds), one TypeScript check (120 seconds), one full language test-suite invocation (180 seconds), and one baseline/candidate inspection/recording allowance (90 seconds): **600 aggregate process-wall seconds**. Each Node test/check process receives a 512 MiB V8 old-space ceiling; this is not a total RSS guarantee. Proposed outer worker/container cap: **1 GiB RSS**, one active execution command, and **20 MiB retained local logs/artifacts**. The parent must supply and validate an actual outer resource enforcer for the complete process tree or revise the proposal honestly before execution; `--max-old-space-size` alone does not enforce RSS. The 1 GiB RSS limit is a proposed, unadmitted requirement, not an installed or measured guarantee. Bind separate enforced temporary-output limits for the existing lowering compilation artifacts before dispatch; the 20 MiB figure covers retained logs/artifacts only.

No retries are implicit. A timeout, memory stop, unexpected RED cause, or failed GREEN stops this cycle. Retain outputs and consumed resources, diagnose without another execution, then record a bounded successor amendment before rerunning. No native/resource campaign is inherited from the historical exhausted k17 run. Review resources and provider quotas are separate; unavailable required review keeps acceptance pending.

The commands below are proposed command bodies, each to run from the isolated worktree root recorded by the required execution binding, inside its admitted isolation/resource wrapper. The path `/home/charl/Moriarty` remains only the inspected repository location; no command here is authorized against that shared working tree. Node flags and per-command timeouts complement that wrapper. Parent campaign recording must retain command, environment identity, timestamp, exit status, stdout/stderr, process-wall/RSS observations and candidate digests. Log truncation or missing exit status is a coverage failure, not passing evidence.

## Task 1: Close the decoder admission boundary

**Files:**
- Modify: `/home/charl/Moriarty/experiments/moriarty-language/tests/semantics.test.mjs`
- Modify: `/home/charl/Moriarty/experiments/moriarty-language/src/evaluate.ts`
- Modify: `/home/charl/Moriarty/experiments/moriarty-language/src/node-shims.d.ts`

**Interfaces:**
- Consumes existing `canonicalDecode(bytes, validate?)`, `registeredBounds(): {bytes: Uint8Array; bounds: Bounds}`, `admitEvaluation(input, limits): R.EvaluationInput`, `need`, and `RuntimeError`.
- Produces the unchanged public `decodeEvaluation(string | Uint8Array): R.EvaluationInput`; admitted return values are now the deeply frozen snapshot from `admitEvaluation`.
- No new public export, callback, bounds override, dependency, or acceptance claim.

- [ ] **Step 1: Add these exact failing regression tests.**

Extend the existing evaluation import to include `decodeEvaluation`, then add the codec and registry imports:

```js
import {createSimulator, evaluate, derive, hash, sealState, decodeEvaluation} from '../src/evaluate.ts';
import {canonicalEncode} from '../src/codec.ts';
import {registeredBounds} from '../src/registered-bounds.ts';
```

Append the following code to `semantics.test.mjs`. It reuses that file's existing `setup`, `input`, and `amount` helpers. Every new test name begins `SP01 decoder`, so the focused command runs precisely this packet. Keep these synchronous tests serial because two tests temporarily instrument host codec methods and restore them in `finally`.

```js
function decoderFixture() { return input(setup('loan'),'accrue',[]); }
function decoderReject(carrier,code='INPUT_SCHEMA') {
  assert.throws(()=>decodeEvaluation(carrier),error=>{
    assert.equal(error.code,code);
    assert.equal(error.stage,9);
    return true;
  });
}
function decoderNodeCount(v) {
  if(Array.isArray(v))return 1+v.reduce((sum,x)=>sum+decoderNodeCount(x),0);
  if(v!==null&&typeof v==='object')return 1+Object.values(v).reduce((sum,x)=>sum+decoderNodeCount(x),0);
  return 1;
}
function decoderNodeTree(nodes) {
  if(nodes===1)return true;
  const children=[];
  for(let remaining=nodes-1;remaining>0;){
    const count=Math.min(remaining,129);
    children.push(count===1?true:Array(count-1).fill(true));
    remaining-=count;
  }
  assert.ok(children.length<=128);
  return children;
}
function decoderWireAtBytes(target) {
  const i=decoderFixture();
  i.state.body.obligations=Array.from({length:64},()=>({
    amount:amount(1,'USD_micro'),creditor:'',debtor:'',denomination:'',dueId:'',status:'Outstanding'
  }));
  let remaining=target-Buffer.byteLength(canonicalEncode(i),'utf8');
  assert.ok(remaining>=0);
  for(const obligation of i.state.body.obligations){
    for(const field of ['creditor','debtor','denomination','dueId']){
      const count=Math.min(remaining,256);
      obligation[field]='x'.repeat(count);
      remaining-=count;
    }
  }
  assert.equal(remaining,0,'the closed-schema fixture has sufficient padding capacity');
  const wire=canonicalEncode(i);
  assert.equal(Buffer.byteLength(wire,'utf8'),target);
  return {i,wire};
}

test('SP01 decoder accepts canonical string, Buffer and Uint8Array as frozen inputs',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  for(const carrier of [wire,Buffer.from(wire),new TextEncoder().encode(wire)]){
    const decoded=decodeEvaluation(carrier);
    assert.deepEqual(decoded,i);
    assert.ok(Object.isFrozen(decoded));
    assert.ok(Object.isFrozen(decoded.authority.statement));
    assert.ok(Object.isFrozen(decoded.action.arguments[0].value));
    assert.throws(()=>{decoded.action.name='settle';},TypeError);
  }
});

test('SP01 decoder ignores hostile shadow properties on genuine byte views',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  for(const bytes of [Buffer.from(wire),new TextEncoder().encode(wire)]){
    let calls=0;
    for(const key of ['length','byteLength','byteOffset','buffer','constructor',Symbol.iterator]){
      Object.defineProperty(bytes,key,{configurable:true,get(){calls++;throw Error('caller getter');}});
    }
    assert.deepEqual(decodeEvaluation(bytes),i);
    assert.equal(calls,0);
  }
});

test('SP01 decoder rejects proxies and non-byte impostors without caller code',()=>{
  let calls=0;
  const trap=()=>{calls++;throw Error('caller trap');};
  const traps={get:trap,getPrototypeOf:trap,ownKeys:trap,getOwnPropertyDescriptor:trap};
  const getterObject={get length(){return trap();},get byteLength(){return trap();},[Symbol.iterator]:trap};
  const revoked=Proxy.revocable(new Uint8Array(0),traps);revoked.revoke();
  for(const carrier of [
    getterObject,new Proxy(new Uint8Array(0),traps),revoked.proxy,
    new Proxy({},traps),Object.create(Uint8Array.prototype),
    new Uint8ClampedArray(0),new Uint16Array(0),new DataView(new ArrayBuffer(0)),
    new ArrayBuffer(0),[],null,undefined,42,true,new String('{}')
  ]){
    decoderReject(carrier);
    assert.equal(calls,0);
  }
});

test('SP01 decoder rejects detached and oversized byte views by intrinsic size',()=>{
  const detached=new Uint8Array(8);
  structuredClone(detached.buffer,{transfer:[detached.buffer]});
  decoderReject(detached);
  const oversized=new Uint8Array(65537);let calls=0;
  Object.defineProperty(oversized,'length',{get(){calls++;return 0;}});
  decoderReject(oversized,'INPUT_BOUNDS');
  assert.equal(calls,0);
  decoderReject(new Uint8Array(0));
});

test('SP01 decoder snapshots precisely the byte view before downstream decoding',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  const storage=Buffer.from('!'+wire+'!');
  const bytes=new Uint8Array(storage.buffer,storage.byteOffset+1,Buffer.byteLength(wire));
  registeredBounds();
  const original=TextDecoder.prototype.decode;let observed=0;
  TextDecoder.prototype.decode=function(snapshot,options){
    observed++;
    assert.notEqual(snapshot,bytes);
    assert.notEqual(snapshot.buffer,bytes.buffer);
    assert.equal(snapshot.byteLength,Buffer.byteLength(wire));
    bytes.fill(0);
    return original.call(this,snapshot,options);
  };
  try{assert.deepEqual(decodeEvaluation(bytes),i);}finally{TextDecoder.prototype.decode=original;}
  assert.equal(observed,1);
});

test('SP01 decoder bounds primitive strings before any UTF8 encoding allocation',()=>{
  registeredBounds();
  const ascii='x'.repeat(65537),multibyte='\u0800'.repeat(21846),supplementary='\u{1f600}'.repeat(16385);
  assert.equal(multibyte.length,21846);
  assert.equal(Buffer.byteLength(multibyte),65538);
  assert.equal(supplementary.length,32770);
  assert.equal(Buffer.byteLength(supplementary),65540);
  const encode=TextEncoder.prototype.encode,encodeInto=TextEncoder.prototype.encodeInto;
  let conversions=0;
  TextEncoder.prototype.encode=function(value){
    if(value===ascii||value===multibyte||value===supplementary){conversions++;throw Error('unexpected input encode');}
    return encode.call(this,value);
  };
  TextEncoder.prototype.encodeInto=function(value,destination){
    if(value===ascii||value===multibyte||value===supplementary){conversions++;throw Error('unexpected input encodeInto');}
    return encodeInto.call(this,value,destination);
  };
  try{
    decoderReject(ascii,'INPUT_BOUNDS');
    decoderReject(multibyte,'INPUT_BOUNDS');
    decoderReject(supplementary,'INPUT_BOUNDS');
  }finally{
    TextEncoder.prototype.encode=encode;
    TextEncoder.prototype.encodeInto=encodeInto;
  }
  assert.equal(conversions,0);
});

test('SP01 decoder accepts 4096 hex signature characters and rejects 4098',()=>{
  const i=decoderFixture();i.authority.signature.bytes='ab'.repeat(2048);
  assert.deepEqual(decodeEvaluation(canonicalEncode(i)),i);
  i.authority.signature.bytes+='ab';
  assert.equal(i.authority.signature.bytes.length,4098);
  decoderReject(canonicalEncode(i),'INPUT_BOUNDS');
  for(const hex of ['f','zz','AB']){
    i.authority.signature.bytes=hex;
    decoderReject(canonicalEncode(i));
  }
});

test('SP01 decoder applies array length 128 and 129 to a schema-valid statement',()=>{
  const i=decoderFixture();i.authority.statement.permittedRecipients=Array(128).fill('party');
  assert.deepEqual(decodeEvaluation(canonicalEncode(i)),i);
  i.authority.statement.permittedRecipients.push('party');
  decoderReject(canonicalEncode(i),'INPUT_BOUNDS');
});

test('SP01 decoder admits exact 65536 input bytes and rejects the next byte',()=>{
  const accepted=decoderWireAtBytes(65536),rejected=decoderWireAtBytes(65537);
  for(const carrier of [accepted.wire,Buffer.from(accepted.wire),new TextEncoder().encode(accepted.wire)]){
    assert.deepEqual(decodeEvaluation(carrier),accepted.i);
  }
  for(const carrier of [rejected.wire,Buffer.from(rejected.wire),new TextEncoder().encode(rejected.wire)]){
    decoderReject(carrier,'INPUT_BOUNDS');
  }
});

test('SP01 decoder enforces depth, node, record-key and UTF8 text admission before schema',()=>{
  // The at-limit variants deliberately have one unknown outer member: their
  // expected INPUT_SCHEMA proves only generic encoding admission, not validity.
  for(const [depth,code] of [[16,'INPUT_SCHEMA'],[17,'INPUT_BOUNDS']]){
    const i=decoderFixture();let nested=true;
    for(let n=1;n<depth;n++)nested=[nested];
    i.extra=nested;decoderReject(canonicalEncode(i),code);
  }
  for(const [nodes,code] of [[8192,'INPUT_SCHEMA'],[8193,'INPUT_BOUNDS']]){
    const i=decoderFixture(),base=decoderNodeCount(i);
    i.extra=decoderNodeTree(nodes-base);
    assert.equal(decoderNodeCount(i),nodes);
    const wire=canonicalEncode(i);assert.ok(Buffer.byteLength(wire)<=65536);
    decoderReject(wire,code);
  }
  for(const [keys,code] of [[64,'INPUT_SCHEMA'],[65,'INPUT_BOUNDS']]){
    const i=decoderFixture();i.extra=Object.fromEntries(Array.from({length:keys},(_,n)=>['k'+n,true]));
    decoderReject(canonicalEncode(i),code);
  }
  for(const [count,code] of [[2048,'INPUT_SCHEMA'],[2049,'INPUT_BOUNDS']]){
    const i=decoderFixture();i.extra='é'.repeat(count);
    assert.equal(Buffer.byteLength(i.extra),count*2);
    decoderReject(canonicalEncode(i),code);
  }
});

test('SP01 decoder rejects noncanonical UTF8, JSON, schema and Unicode inputs',()=>{
  const i=decoderFixture(),wire=canonicalEncode(i);
  const duplicate=wire.slice(0,-1)+',"schemaVersion":"moriarty-evaluation/1"}';
  for(const carrier of [
    ' '+wire,wire+'\n',duplicate,'{', 'null', '42', '[]',
    new Uint8Array([0xc3,0x28]),new Uint8Array([0xef,0xbb,0xbf,...Buffer.from(wire)]),
    wire.replace('"nonce0"','"\ud800"'),wire.replace('"nonce0"','"\\ud800"')
  ])decoderReject(carrier);
  const wrong=structuredClone(i);wrong.extra=true;decoderReject(canonicalEncode(wrong));
  const profile=structuredClone(i);profile.program.profile='moriarty-bounded-atomic/2';decoderReject(canonicalEncode(profile));
  const unicode=structuredClone(i);unicode.authority.statement.nonce='é'.repeat(128);
  assert.deepEqual(decodeEvaluation(canonicalEncode(unicode)),unicode);
  unicode.authority.statement.nonce+='é';decoderReject(canonicalEncode(unicode));
  unicode.authority.statement.nonce='\u{1f600}'.repeat(64);
  assert.deepEqual(decodeEvaluation(canonicalEncode(unicode)),unicode);
  unicode.authority.statement.nonce+='\u{1f600}';decoderReject(canonicalEncode(unicode));
});
```

`registeredBounds()` re-encodes its cached trusted text on each call. The conversion instrumentation therefore rejects conversion of the hostile input argument specifically, while permitting fixed-size trusted registry work.

- [ ] **Step 2: Run the focused tests against the unmodified decoder to retain RED.**

```sh
timeout --signal=TERM --kill-after=5s 60s node --max-old-space-size=512 --test --test-concurrency=1 --test-name-pattern='^SP01 decoder' experiments/moriarty-language/tests/semantics.test.mjs
```

Expected: nonzero test exit with substantive failures including 4098 signature admission, 129 recipients, hostile getter/proxy handling, missing frozen result, and missing private byte snapshot. Several existing canonical rejection predicates should already pass. The detached test may already pass through codec normalization; that is preserved behavior. An import or syntax failure, unsupported Node version, missing fixtures, or fixture boundary-calculation failure does not establish RED. Do not implement until the actual failures match decoder behavior.

- [ ] **Step 3: Add the exact minimal runtime and type changes.**

Replace the final `node:util` shim declaration with:

```ts
declare module 'node:util' {
  export const types:{isProxy(value:unknown):boolean;isUint8Array(value:unknown):value is Uint8Array};
}
```

In `evaluate.ts`, directly after `const UNKNOWN`, add:

```ts
const evaluationByteLength=Object.getOwnPropertyDescriptor(
 Object.getPrototypeOf(Uint8Array.prototype),'byteLength'
)!.get!;
const evaluationByteSet=Uint8Array.prototype.set;
```

Replace the complete existing `decodeEvaluation` function with:

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

The local bounds intersection supplies the runtime encoding fields omitted from the current public `Bounds` TypeScript declaration; its only source is the hash-pinned registry. It changes no runtime profile and avoids broadening ownership into a global type refactor.

The scalar preflight permits supplementary Unicode characters and counts each valid surrogate pair as four UTF-8 bytes. `scalarText` rejects lone surrogates before the counting loop relies on pair structure. All caller-string scanning is bounded by 65536 code units. The byte copy target is at most 65536 bytes; native copy exceptions do not trigger retries or resized allocations. A shared source changing during copying does not provide a coherent multi-byte transaction snapshot; only the resulting private byte sequence is parsed and admitted. This packet claims no concurrent-writer atomicity.

`canonicalDecode` still checks UTF-8, JSON and canonical roundtrip, with its existing defensive encoder ceiling. Raw parsing/roundtrip occurs under the serialized byte cap before `admitEvaluation` applies decoded graph limits. Thus malformed JSON can still fail `INPUT_SCHEMA` before a more specific graph-limit classification; the tests for limit diagnostics use canonical payloads that reach graph admission.

- [ ] **Step 4: Run focused GREEN with the same command, once.**

```sh
timeout --signal=TERM --kill-after=5s 60s node --max-old-space-size=512 --test --test-concurrency=1 --test-name-pattern='^SP01 decoder' experiments/moriarty-language/tests/semantics.test.mjs
```

Expected: all `SP01 decoder` tests pass, including tests known to fail at RED. Record real output and counts; the draft predicts no count from a run it has not performed.

- [ ] **Step 5: Run the relevant regression file and required language checks within the admitted budget.**

Run these commands sequentially and inspect every exit status:

```sh
timeout --signal=TERM --kill-after=5s 90s node --max-old-space-size=512 --test --test-concurrency=1 experiments/moriarty-language/tests/semantics.test.mjs
```

```sh
env NODE_OPTIONS=--max-old-space-size=512 timeout --signal=TERM --kill-after=5s 120s npm --prefix experiments/moriarty-language run typecheck
```

```sh
timeout --signal=TERM --kill-after=5s 180s node --max-old-space-size=512 --test --test-concurrency=1 experiments/moriarty-language/tests/*.test.mjs
```

Expected: zero exits and no failed tests; existing atomic input rejection, authority, bounded arithmetic, financial transitions, source-map and registered-bounds tests stay passing. The full suite includes existing bounded Compact lowering compilation with `--skip-zk`; those regressions are expressly within this proposed validation scope. No new proof build or native proving campaign is added. A pre-existing unrelated failure still remains an explicit failed campaign check; do not hide it or claim full-suite GREEN.

Read-only inspection of `experiments/moriarty-language/tests/lowering.test.mjs` found `compileArtifact` invokes `compact compile --skip-zk --compact-path ...` via `spawnSync` with a 90000 ms child timeout. It writes temporary lowering artifacts and imports the generated JavaScript for comparisons. The existing full-suite 180-second outer allocation still applies and is not automatically expanded by the child timeout. Any child timeout, aggregate cap breach, missing compiler/runtime dependency, or exhausted storage stops the cycle for a recorded resource amendment. No such compilation or validation was run while preparing this revision.

- [ ] **Step 6: Inspect scope and freeze candidate evidence for independent result audits.**

```sh
git diff --check
```

```sh
git diff -- experiments/moriarty-language/src/evaluate.ts experiments/moriarty-language/src/node-shims.d.ts experiments/moriarty-language/tests/semantics.test.mjs
```

```sh
sha256sum experiments/moriarty-language/src/evaluate.ts experiments/moriarty-language/src/node-shims.d.ts experiments/moriarty-language/tests/semantics.test.mjs experiments/moriarty-language/src/registered-bounds.ts experiments/moriarty-language/src/codec.ts experiments/moriarty-language/spec/bounds.json
```

Read-only dependencies and the bounds file must still have their inspected digests unless a separately reviewed concurrent change has been explicitly incorporated. Supply the precise diff, baseline/candidate digests, RED/GREEN results, resource receipts and scope statement to the required current result auditors. Missing Fable quota means pending audit; no synthetic identity or alternative reviewer completes that gate. A majority resource decision is not a result audit.

- [ ] **Step 7: Record the scoped implementation checkpoint only after the parent-controlled integration gate.**

This draft does not authorize an immediate commit, PR, publish or promotion. At the parent-controlled integration step, stage only these three owned files if their complete diffs belong to this packet:

```sh
git add -- experiments/moriarty-language/src/evaluate.ts experiments/moriarty-language/src/node-shims.d.ts experiments/moriarty-language/tests/semantics.test.mjs
```

```sh
git commit -m "fix: bound and snapshot evaluation decoding"
```

If concurrent changes share any owned file, use parent-reviewed patch staging rather than staging that whole file. A commit is a recovery checkpoint and does not itself approve SP01, MC01, ledger acceptance, financial conformance, or any native proof claim.

## Independent expected predicates and limits of the tests

| Predicate | Independent expected observation | Scope limit |
| --- | --- | --- |
| Throwing API | Reject helper sees an exception with requested code and stage 9 | Does not return a rejection object or authenticate anything |
| Branded carriers | String, Buffer and Uint8Array decode equal to a separately built expected fixture | No support promised for plain arrays, DataView, Uint8ClampedArray or boxed strings |
| No caller execution | Hostile proxy/getter counters remain exactly zero | Intrinsics and the runtime realm are trusted; arbitrary global intrinsic replacement before module import is outside scope |
| Inert bytes | Decoder receives a distinct fixed-size buffer containing exactly the requested view; mutating caller bytes before downstream decode has no effect | The instrumentation runs after the snapshot; it does not prove atomic copying against concurrent shared-memory writes |
| Allocation admission | 65537 code-unit and 65538-byte arguments reject without encoding those arguments; intrinsic view length caps private byte allocation | Caller-owned input allocation is not decoder allocation; JSON parsing remains bounded by the raw byte cap before tighter graph admission |
| Signature text | 4096 hex characters accepted by decode; 4098 rejected with INPUT_BOUNDS | No cryptographic validity or signature algorithm size inferred |
| Complete wire bytes | 65536 accepted, 65537 rejected, independently measured by Buffer.byteLength | Boundary fixture is decoder-schema-valid only; altered obligations do not become semantically authenticated |
| Graph metrics | 128/129 recipient array boundary; 16/17 depth; 8192/8193 nodes; 64/65 keys; 4096/4098 UTF8 text bytes | Unknown-member at-limit controls correctly finish as INPUT_SCHEMA, not successful inputs |
| Canonical/UTF8/schema | Whitespace, duplicate root key, BOM, malformed UTF8, lone surrogates, wrong profile and unknown fields reject | Existing generic codec behavior is retained; this does not revise its format |
| Signing-envelope admission | Decoder calls existing `admitEvaluation`; reviewer inspects its statement-specific `enc` invocation and exact registered metrics | With identical current generic metric ceilings, a statement subtree cannot exceed a signing metric while the whole evaluation passes that same metric. No fabricated independently reachable signing-only counterexample or custom bounds override is used |

For valid Unicode scalars, UTF-8 bytes are at least UTF-16 code units, and these two registered text ceilings are equal. The signature test covers both ceilings; the multibyte case demonstrates the stricter byte constraint. It is not evidence that an otherwise byte-admitted string can independently violate the equal code-unit ceiling.

The node-count helper is separate from production `measureEncoding`/`checkEncoding` and uses the metric definition directly. The byte-boundary expected size uses Node's independent byte-count operation. Fixture construction uses the canonical serializer to provide valid wire syntax; it is not presented as an independent implementation of the canonical codec.

## Draft self-review and handoff

- The packet owns one decoder correction, three files, one RED/GREEN cycle, and bounded follow-on checks.
- Public signature, exact registry, financial semantics and throwing behavior are preserved.
- Every proposed implementation edit and test has concrete code; commands are specified but unperformed.
- Test instrumentation permits only trusted registry encoding while detecting premature hostile-input encoding; full-suite Node flags precede the file arguments.
- The stronger all-decoration ban was explicitly rejected in favor of intrinsic byte-carrier admission; decoded JSON graph admission remains strict.
- Isolated-worktree binding, enforced process-tree RSS/storage limits, resource admission, runtime allowance and required current audits remain pending under the parent's live decision process. Existing bounded `--skip-zk` lowering checks are part of the proposed full suite; no test, repository edit, worktree creation, build or proving activity was performed while preparing this revision.
