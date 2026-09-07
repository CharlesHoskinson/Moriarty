import assert from 'node:assert/strict';
import { pathToFileURL } from 'node:url';
import test from 'node:test';
const generated = process.env.MORIARTY_COMPACT_ARTIFACT;
assert.ok(generated, 'MORIARTY_COMPACT_ARTIFACT must identify the compiled contract/index.js');
const {pureCircuits: c} = await import(pathToFileURL(generated).href);
const B = 1n << 64n, MAX = (1n << 128n)-1n;
const hints = (a,b) => ({aLo:a%B,aHi:a/B,bLo:b%B,bHi:b/B,lo:(a%B)*(b%B)%B,carry:(a%B)*(b%B)/B});
const mul = (a,b) => c.checkedMul(a,b,hints(a,b));
const div = (a,b,q=a/b,r=a%b,h=hints(b,q)) => c.checkedDiv(a,b,q,r,h);
test('128-bit boundary and checked arithmetic', () => {
  assert.equal(c.checkedAdd(MAX-1n,1n),MAX);
  assert.throws(()=>c.checkedAdd(MAX,1n));
  assert.equal(c.checkedSub(MAX,MAX),0n);
  assert.throws(()=>c.checkedSub(0n,1n));
  for (const [a,b] of [[0n,MAX],[1n,MAX],[B-1n,B+1n],[B, B-1n],[123456789n,987654321n]]) assert.equal(mul(a,b),a*b);
  for (const [a,b] of [[MAX,2n],[B,B],[B+1n,B],[MAX,MAX]]) assert.throws(()=>mul(a,b));
});
test('division and financial values are calculated from dynamic inputs', () => {
  for(const [n,d] of [[MAX,MAX],[MAX,1n],[0n,MAX],[MAX,B+1n],[1240000000000n,36500n],[19940000000000n,1009970000n]]) assert.equal(div(n,d),n/d);
  assert.equal(div(mul(mul(5000000000n,8n),31n),mul(100n,365n)),33972602n);
  assert.equal(div(mul(mul(10000n,997n),2000000n),c.checkedAdd(mul(1000000n,1000n),mul(10000n,997n))),19743n);
  assert.throws(()=>div(5n,0n,0n,5n,hints(0n,0n)));
});
test('every multiplication witness limb is constrained', () => {
  const a=B+123n,b=12345n,valid=hints(a,b);
  for(const key of Object.keys(valid)) assert.throws(()=>c.checkedMul(a,b,{...valid,[key]:valid[key]+1n}), key);
  assert.throws(()=>c.checkedMul(a,b,{...valid,aLo:B}));
});
test('forged division witnesses cannot change floor or wrap field arithmetic', () => {
  assert.throws(()=>div(99n,10n,8n,19n));
  assert.throws(()=>div(99n,10n,10n,0n));
  assert.throws(()=>div(0n,MAX,MAX,0n));
  const q=MAX/(B+1n),r=MAX%(B+1n),h=hints(B+1n,q);
  assert.throws(()=>div(MAX,B+1n,q,r+1n,h));
  for(const key of Object.keys(h)) assert.throws(()=>div(MAX,B+1n,q,r,{...h,[key]:h[key]+1n}),key);
});
test('deterministic boundary grid agrees with independent BigInt oracle', () => {
  const xs=[0n,1n,2n,B-1n,B,B+1n,MAX/2n,MAX-1n,MAX];
  for(const a of xs) for(const b of xs) {
    if(a*b<=MAX) assert.equal(mul(a,b),a*b); else assert.throws(()=>mul(a,b));
    if(b>0n) assert.equal(div(a,b),a/b);
  }
});
test('generated ledger circuits use checked helpers before writing', async () => {
  const harness = process.env.MORIARTY_COMPACT_HARNESS;
  assert.ok(harness, 'compiled arithmetic harness path required');
  const h = await import(pathToFileURL(harness).href);
  const runtime = await import(new URL('../node_modules/@midnight-ntwrk/compact-runtime/dist/index.js',pathToFileURL(harness)).href);
  const contract = new h.Contract({});
  const zero = '00'.repeat(32);
  const initial = contract.initialState(runtime.createConstructorContext({},zero));
  const context = () => runtime.createCircuitContext(runtime.dummyContractAddress(),zero,initial.currentContractState,{});
  const positive = contract.circuits.multiply(context(),B+1n,12345n,hints(B+1n,12345n));
  assert.equal(h.ledger(positive.context.currentQueryContext.state).output,(B+1n)*12345n);
  assert.throws(()=>contract.circuits.multiply(context(),MAX,2n,hints(MAX,2n)));
  const divided = contract.circuits.divide(context(),99n,10n,9n,9n,hints(10n,9n));
  assert.equal(h.ledger(divided.context.currentQueryContext.state).output,9n);
  assert.throws(()=>contract.circuits.divide(context(),99n,10n,8n,19n,hints(10n,8n)));
  assert.equal(h.ledger(initial.currentContractState.data).output,0n);
});
