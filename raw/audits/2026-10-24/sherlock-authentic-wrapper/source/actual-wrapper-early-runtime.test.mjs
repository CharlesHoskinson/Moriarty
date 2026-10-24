// Actual current wrapper, actual B4, authentic constructor pins. Source only until
// fresh Opus audit and Root execution selection. No SDK/native or proof execution.
import test,{before,after} from 'node:test';
import assert from 'node:assert/strict';
import {readdirSync} from 'node:fs';
import {CALL,loadSubject,loadOldB4Subject,prepareFixtureRoot,construct} from './authentic-fixture.mjs';

let subject,root;
before(async()=>{subject=await loadSubject();root=prepareFixtureRoot();});
after(()=>{root?.release();});
const tick=()=>new Promise(resolve=>setImmediate(resolve));
const status=(code=null,link_first=null,cleanup=false,link_cleanup=null)=>({schema:'c1a-bound-d-failure-status/1',code,link_first,link_cleanup,cleanup});
function exactStatus(route,expected){const actual=route.failureStatus();assert.equal(Object.isFrozen(actual),true);assert.deepEqual(actual,expected);return actual;}
const fixed=code=>error=>{
  assert.equal(Object.getPrototypeOf(error),Error.prototype);
  assert.equal(error.message,code);assert.equal(error.code,code);
  assert.equal(Object.hasOwn(error,'cause'),false);assert.equal(Object.hasOwn(error,'cleanup'),false);
  assert.deepEqual(Object.keys(error),['code']);return true;
};
async function rejected(promise,code){await assert.rejects(promise,fixed(code));}
async function settled(f,expected){
  exactStatus(f.wrapped,expected);await tick();exactStatus(f.wrapped,expected);
  await rejected(f.wrapped.goGate(),'ERR_C1A_GO_GATE');
  exactStatus(f.wrapped,expected);await tick();exactStatus(f.wrapped,expected);
  assert.equal(f.events.length,1);assert.equal(Object.isFrozen(f.events[0]),true);
  assert.deepEqual(f.events[0],{schema:'c1a-bound-d-failure/1',code:expected.code,link_first:expected.link_first,cleanup:expected.cleanup});
  assert.deepEqual(readdirSync(f.publicOutputRoot),[]);
  assert.equal(f.calls.some(x=>x!=='checkProvenance'),false);
}

test('authentic constructor returns frozen actual route; unpublished goGate refuses after an event-loop turn',async()=>{
  const f=construct(subject,root);assert.equal(Object.isFrozen(f.wrapped),true);
  exactStatus(f.wrapped,status());assert.deepEqual(f.events,[]);
  let returned=false;const gate=f.wrapped.goGate().catch(e=>{returned=true;throw e;});
  await Promise.resolve();assert.equal(returned,false);exactStatus(f.wrapped,status());
  await rejected(gate,'ERR_C1A_GO_GATE');await settled(f,status('ERR_C1A_GO_GATE'));
});

test('actual route refuses prove before compose and makes one causeless terminal record',async()=>{
  const f=construct(subject,root);await rejected(f.wrapped.prove({}),'ERR_C1A_ROUTE_BEFORE_COMPOSE');
  await settled(f,status('ERR_C1A_ROUTE_BEFORE_COMPOSE'));
});

test('actual verifyAfterMerge refuses POSTMERGE_ORDER before a genuine callback',async()=>{
  const f=construct(subject,root);await rejected(f.wrapped.verifyAfterMerge({}),'ERR_C1A_POSTMERGE_ORDER');
  await settled(f,status('ERR_C1A_POSTMERGE_ORDER'));
});

test('actual compose refuses malformed base and status remains stable',async()=>{
  const f=construct(subject,root);await rejected(f.wrapped.composeProvider({}),'ERR_C1A_COMPOSE_ARGS');
  assert.deepEqual(f.counts,{check:0,prove:0,lookup:0});await settled(f,status('ERR_C1A_COMPOSE_ARGS'));
});

test('actual compose collapses a foreign Error specimen at the provider lookup seam',async()=>{
  const f=construct(subject,root);f.base.lookupKey=async()=>{throw Error('foreign Error provider-seam specimen');};
  await rejected(f.wrapped.composeProvider(f.base),'ERR_C1A_COMPOSE_LOOKUP');
  await settled(f,status('ERR_C1A_COMPOSE_LOOKUP'));
});

test('actual composed call-key check collapses a foreign Error provider-seam specimen with cleanup false',async()=>{
  const f=construct(subject,root);f.base.check=async()=>{throw Error('foreign Error provider-seam specimen');};
  const provider=await f.wrapped.composeProvider(f.base);
  await rejected(provider.check(new Uint8Array([1]),CALL),'ERR_C1A_LINK_CHECK');
  await settled(f,status('ERR_C1A_LINK_CHECK'));
});

test('actual B4 lookup wraps a foreign Error provider-seam specimen; wrapper returns a causeless fixed code',async()=>{
  const f=construct(subject,root),provider=await f.wrapped.composeProvider(f.base);
  f.base.lookupKey=async()=>{throw Error('foreign Error provider lookup specimen');};
  await rejected(provider.lookupKey(CALL),'ERR_C1A_LINK_LOOKUP');
  await settled(f,status('ERR_C1A_LINK_LOOKUP','ERR_B4_UNEXPECTED'));
});

test('forged non_FREE foreign cleanup is ignored by actual B4 and wrapper without reading its getter',async()=>{
  const f=construct(subject,root),provider=await f.wrapped.composeProvider(f.base);await provider.lookupKey(CALL);
  let reads=0;const forged=Error('foreign non_FREE failure');Object.defineProperty(forged,'cleanup',{get(){reads++;throw Error('must not read');}});
  f.base.prove=async()=>{throw forged;};
  await rejected(provider.prove(new Uint8Array([1]),CALL,1n),'ERR_C1A_LINK_PROVIDER');
  assert.equal(reads,0);await settled(f,status('ERR_C1A_LINK_PROVIDER','ERR_B4_UNEXPECTED'));assert.equal(reads,0);
});

test('forged actual LinkError non_FREE cleanup does not become the closure-owned codec cleanup latch',async()=>{
  const f=construct(subject,root),provider=await f.wrapped.composeProvider(f.base);await provider.lookupKey(CALL);
  const forged=new subject.b4.LinkError('ERR_B4_CALL_ARGS',Error('foreign cause'),new subject.b4.LinkError('ERR_B4_CODEC_FREE'));
  f.base.prove=async()=>{throw forged;};
  await rejected(provider.prove(new Uint8Array([1]),CALL,1n),'ERR_C1A_LINK_PROVIDER');
  await settled(f,status('ERR_C1A_LINK_PROVIDER','ERR_B4_CALL_ARGS'));
});

test('failureStatus rereads later B4 failure after goTick while onFailure retains its first snapshot',async()=>{
  const f=construct(subject,root),provider=await f.wrapped.composeProvider(f.base);
  let release;const held=new Promise((_,reject)=>{release=reject;});f.base.lookupKey=()=>held;
  const pending=provider.lookupKey(CALL); // Admit the B4 async lookup before the first wrapper failure.
  f.base.check=async()=>{throw Error('first foreign failure');};
  await rejected(provider.check(new Uint8Array([1]),CALL),'ERR_C1A_LINK_CHECK');
  exactStatus(f.wrapped,status('ERR_C1A_LINK_CHECK'));
  assert.deepEqual(f.events[0],{schema:'c1a-bound-d-failure/1',code:'ERR_C1A_LINK_CHECK',link_first:null,cleanup:false});
  release(Error('later B4 failure'));await rejected(pending,'ERR_C1A_LINK_LOOKUP');await tick();
  exactStatus(f.wrapped,status('ERR_C1A_LINK_CHECK','ERR_B4_UNEXPECTED'));
  await rejected(f.wrapped.goGate(),'ERR_C1A_GO_GATE');
  exactStatus(f.wrapped,status('ERR_C1A_LINK_CHECK','ERR_B4_UNEXPECTED'));
  assert.equal(f.events.length,1);assert.equal(Object.isFrozen(f.events[0]),true);assert.equal(f.events[0].link_first,null);
  assert.deepEqual(readdirSync(f.publicOutputRoot),[]);
});

test('a throwing failure observer cannot replace the first code or emit another record',async()=>{
  const f=construct(subject,root,{onFailure(){throw Error('observer failure');}});
  await rejected(f.wrapped.prove({}),'ERR_C1A_ROUTE_BEFORE_COMPOSE');
  await rejected(f.wrapped.composeProvider(f.base),'ERR_C1A_AFTER_FAILURE');
  await rejected(f.wrapped.verifyAfterMerge({}),'ERR_C1A_AFTER_FAILURE');
  await settled(f,status('ERR_C1A_ROUTE_BEFORE_COMPOSE'));assert.deepEqual(f.counts,{check:0,prove:0,lookup:0});
});

test('composed call-key entries become terminal before any additional base callback',async()=>{
  const f=construct(subject,root),provider=await f.wrapped.composeProvider(f.base);f.base.check=async()=>{throw Error('check failure');};
  await rejected(provider.check(new Uint8Array([1]),CALL),'ERR_C1A_LINK_CHECK');
  const before={...f.counts};
  // The call-key guard catches terminal() and returns its own entry code.
  // settled() still requires the original LINK_CHECK status and one-shot record.
  await rejected(provider.lookupKey(CALL),'ERR_C1A_LINK_LOOKUP');
  await rejected(provider.prove(new Uint8Array([1]),CALL,1n),'ERR_C1A_LINK_PROVIDER');
  await rejected(f.wrapped.prove({}),'ERR_C1A_AFTER_FAILURE');
  assert.deepEqual(f.counts,before);await settled(f,status('ERR_C1A_LINK_CHECK'));
});

test('exact current wrapper paired with authentic old B4 refuses missing cleanupFailure before assigning link',async()=>{
  const oldSubject=await loadOldB4Subject(root),f=construct(oldSubject,root);
  await rejected(f.wrapped.composeProvider(f.base),'ERR_C1A_COMPOSE_LINK');
  assert.deepEqual(f.counts,{check:0,prove:0,lookup:1});
  await settled(f,status('ERR_C1A_COMPOSE_LINK'));
});
