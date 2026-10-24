// PRE_PIN_REFUSAL_ONLY: real wrapSealedCallBoundDRoute and real freezeCallContext/isCallContext from the subject root, no mocks.
// Every wrapper case refuses before pin-path canonicalization, input reads, hashing or pinned-fixture processing.
// No positive construction, callback, compose, provider, prove, merge, verify or goGate. Synthetic public in-memory shapes only.
import test from 'node:test';
import assert from 'node:assert/strict';
import {wrapSealedCallBoundDRoute} from './bound-d-call-phase.mjs';
import {freezeCallContext,isCallContext} from './sealed-call-policy.mjs';

const ADDRESS_IN='0x'+'AB'.repeat(32),ADDRESS='ab'.repeat(32),DEPLOY='d'.repeat(64),VERIFIER='a'.repeat(64),TEMPLATE='c'.repeat(64);
const freshContext=()=>freezeCallContext({network:'stagenet',address:ADDRESS_IN,entryPoint:'acceptValidity',deploymentRecordSha256:DEPLOY});
// Unbranded objects carrying the same public fields as a factory context.
const copiesOf=ctx=>[{...ctx},Object.freeze({...ctx}),structuredClone(ctx),JSON.parse(JSON.stringify(ctx)),Object.create(ctx)];
// One fresh synthetic fixture per case. It passes every guard up to the capture-public stage; each case breaks one member.
// capture_public_expected is absent, so the furthest reachable stage refuses in JSON.parse before publicIdentity or any hash.
// Runtime functions exist only for shape: each records its name and throws, and every case asserts none was called.
function fixture({context=freshContext()}={}) {
  const calls=[],stub=name=>function(){calls.push(name);throw Error('synthetic stub invoked');};
  const route={prove:stub('route.prove'),batchNo:0n,stmt:7n,instanceId:5n};
  const config={verifier_sha256:VERIFIER,handkey_sha256:'b'.repeat(64),template_sha256:TEMPLATE,expected_dom_stmt:'01'+'00'.repeat(31),srs20_bound_path:'/synthetic-public-never-read/srs20'};
  const submissionIntent={schema:'actual-sherlock-call-intent/2',network:'stagenet',address:ADDRESS,action:'sherlock/acceptValidity',physical_calls:1,pre_state_sha256:'e'.repeat(64),deployment_record_sha256:DEPLOY,instance_id:'05'+'00'.repeat(31),batch_no:'00'.repeat(32),tagged_verifier_sha256:VERIFIER,template_sha256:TEMPLATE,prove_after_final_balance:false,balancing_scope:['dust']};
  const ledger={Proof:{deserialize:stub('ledger.Proof.deserialize')},ContractCall:stub('ledger.ContractCall')};
  const runtimePorts={callKey:'sherlock/acceptValidity',parseTemplate:stub('parseTemplate'),checkCallPis:stub('checkCallPis'),checkProvenance:stub('checkProvenance')};
  return {calls,stub,route,options:{config,submissionIntent,callContext:context,ledger,deadline:Date.now()+3600000,onFailure:stub('onFailure'),runtimePorts}};
}
// Exact constructor refusal: a fresh plain Error with the fixed message, no cause and no own enumerable members.
const refusedWith=message=>e=>{
  assert.equal(Object.getPrototypeOf(e),Error.prototype);
  assert.equal(e.message,message);
  assert.equal(Object.hasOwn(e,'cause'),false);
  assert.deepEqual(Object.keys(e),[]);
  return true;
};
function refuses(f,message) {
  assert.throws(()=>wrapSealedCallBoundDRoute(f.route,f.options),refusedWith(message));
  assert.deepEqual(f.calls,[]);
}
const assertionWith=message=>e=>e instanceof assert.AssertionError&&e.message===message;

test('import and brand: factory context is frozen, normalized and branded; same-field copies are not',()=>{
  assert.equal(typeof wrapSealedCallBoundDRoute,'function');
  assert.equal(typeof freezeCallContext,'function');
  assert.equal(typeof isCallContext,'function');
  const ctx=freshContext(),again=freshContext();
  assert.equal(Object.isFrozen(ctx),true);
  assert.equal(JSON.stringify(ctx),JSON.stringify({schema:'sherlock-sealed-call-context/1',network:'stagenet',address:ADDRESS,entry_point:'acceptValidity',deployment_record_sha256:DEPLOY}));
  assert.equal(isCallContext(ctx),true);
  assert.equal(isCallContext(again),true);
  assert.notEqual(ctx,again);
  for(const copy of copiesOf(ctx))assert.equal(isCallContext(copy),false);
  assert.equal(isCallContext(null),false);
});

test('context factory refusal: non-hex deployment digest -> ERR_CALL_CONTEXT_DEPLOYMENT',()=>{
  assert.throws(()=>freezeCallContext({network:'stagenet',address:ADDRESS_IN,entryPoint:'acceptValidity',deploymentRecordSha256:'D'.repeat(64)}),assertionWith('ERR_CALL_CONTEXT_DEPLOYMENT'));
});

test('context factory refusal: odd-length address -> ERR_CALL_ADDRESS_REPRESENTATION',()=>{
  assert.throws(()=>freezeCallContext({network:'stagenet',address:'0xabc',entryPoint:'acceptValidity',deploymentRecordSha256:DEPLOY}),assertionWith('ERR_CALL_ADDRESS_REPRESENTATION'));
});

// The snapshot refusal shares its message with the options guard; it is told apart by preceding the route guard,
// which would otherwise refuse this route with actual bound-D route required.
test('snapshot refusal: uncloneable config refused before the route guard -> bound-D wrapper options refused',()=>{
  const f=fixture();
  f.route.prove=undefined;
  f.options.config.uncloneable=f.stub('config.uncloneable');
  refuses(f,'bound-D wrapper options refused');
});

test('route guard: missing prove -> actual bound-D route required',()=>{
  const f=fixture();
  delete f.route.prove;
  refuses(f,'actual bound-D route required');
});

test('route guard: batchNo 1n -> actual bound-D route required',()=>{
  const f=fixture();
  f.route.batchNo=1n;
  refuses(f,'actual bound-D route required');
});

test('options guard: forged context copies refused while the factory original reaches capture public expected required',()=>{
  const ctx=freshContext();
  for(const forged of copiesOf(ctx))refuses(fixture({context:forged}),'bound-D wrapper options refused');
  refuses(fixture({context:ctx}),'capture public expected required');
  assert.equal(isCallContext(ctx),true);
});

test('options guard: expired deadline -> bound-D wrapper options refused',()=>{
  const f=fixture();
  f.options.deadline=Date.now()-1;
  refuses(f,'bound-D wrapper options refused');
});

test('intent guard: wrong schema -> actual submission intent required',()=>{
  const f=fixture();
  f.options.submissionIntent.schema='actual-sherlock-call-intent/1';
  refuses(f,'actual submission intent required');
});

test('intent guard: negative route instanceId -> field element',()=>{
  const f=fixture();
  f.route.instanceId=-1n;
  refuses(f,'field element');
});

test('context guard: unnormalized intent address -> submission intent differs from adopted call context',()=>{
  const f=fixture();
  f.options.submissionIntent.address=ADDRESS_IN;
  refuses(f,'submission intent differs from adopted call context');
});

test('context guard: differing deployment digest -> submission intent differs from adopted call context',()=>{
  const f=fixture();
  f.options.submissionIntent.deployment_record_sha256='f'.repeat(64);
  refuses(f,'submission intent differs from adopted call context');
});
