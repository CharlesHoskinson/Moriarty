// Recompute observations; expected inputs are read-only and hash checked.
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import assert from 'node:assert/strict';
import { encodeEffects, effectCommitment, compareEffectCommitment } from './codec.mjs';
import { encodeAuthorization } from '../wire/codec.mjs';
import { prepareMil4S0 } from '../../../src/successor/mil4-s0-core-v5.ts';

const here=new URL('./',import.meta.url);
const root=new URL('../../../../../',here);
const read=name=>readFileSync(new URL(name,here));
const digest=bytes=>createHash('sha256').update(bytes).digest('hex');
const freeze=JSON.parse(read('freeze-receipt.json'));
for(const [name,expected] of Object.entries(freeze.expectedInputs)) assert.equal(digest(read(name)),expected,name);
for(const [name,expected] of Object.entries(freeze.readOnlyDependencies)) assert.equal(digest(readFileSync(new URL(name,root))),expected,name);
const fixtures=JSON.parse(read('fixtures.json'));
const outcome={phase:'TerminalSuccess',retainedEffects:[],retainedDuties:[]};
const positive=fixtures.positive.map(v=>{
  const bytes=encodeEffects(v.prepared);
  assert.equal(bytes.toString('hex'),v.expected.wireHex);
  assert.equal(effectCommitment(v.prepared),v.expected.commitment);
  const wire=encodeAuthorization(v.authorization);
  const equality=compareEffectCommitment(wire,v.prepared);
  assert.equal(equality.status,'CommitmentEqualUnqualified');
  const core=prepareMil4S0(v.state,v.coreIntent,v.prepared.effects,v.prepared.successor,outcome,
    {state:v.state,intent:v.coreIntent,round:v.state.round,expectedSuccessor:v.prepared.successor,requestedOutcome:outcome});
  assert.equal(core.status,'PreparedUnqualified');
  assert.deepEqual(core.effects,v.prepared.effects);
  assert.deepEqual(core.candidatePost,v.expectedPost);
  // Binary export uses the pre-implementation frozen bytes, not codec output.
  writeFileSync(new URL(`${v.id}.bin`,here),Buffer.from(v.expected.wireHex,'hex'));
  return {id:v.id,length:bytes.length,commitment:v.expected.commitment,equality,
    coreStatus:core.status,exactEffectsAndPost:true};
});
const hostile=fixtures.hostile.map(h=>{
  const base=fixtures.positive.find(v=>v.id===h.base);
  let observed;
  try { observed=compareEffectCommitment(encodeAuthorization(base.authorization),h.prepared); }
  catch(e){observed={status:'CodecRejected',code:e.code};}
  assert.equal(observed.code,h.expectedCode);
  if(h.expected) assert.equal(effectCommitment(h.prepared),h.expected.commitment);
  return {id:h.id,base:h.base,observed,commitment:h.expected?.commitment??null};
});
const tap=read('test-output.tap').toString('utf8');
const testCounts=Object.fromEntries(['tests','pass','fail','skipped'].map(k=>{
  const match=tap.match(new RegExp(`^# ${k} (\\d+)$`,'m'));
  assert.ok(match,`TAP ${k}`); return [k,Number(match[1])];
}));
assert.equal(testCounts.fail,0);
assert.equal(testCounts.tests,testCounts.pass);
const scopes=['SPEC.md','PLAN.md','fixtures.json','reference-vectors.py','codec.mjs','codec.test.mjs','record-results.mjs'];
const result={status:'W-D2E finite experiment observed; all acceptance gates open',
  observedAt:new Date().toISOString(),node:process.version,freezeReceiptSha256:digest(read('freeze-receipt.json')),
  frozenInputsUnchanged:true,readOnlyDependenciesUnchanged:true,
  command:'node --test --test-reporter=tap codec.test.mjs',testCounts,
  testOutputSha256:digest(read('test-output.tap')),
  red:{command:'node --test codec.test.mjs',exitCode:1,reason:'Expected ERR_MODULE_NOT_FOUND before codec existed',outputSha256:digest(read('red-output.tap'))},
  scopeSha256:Object.fromEntries(scopes.map(n=>[n,digest(read(n))])),
  positive,hostile,
  limits:['Same-author independent Python construction; no independent reviewer verdict',
    'Only equality: authorization signature, semantic preparation, state/head authentication and consume are external',
    'Successor head must be selected independently of authorization digest; codec cannot prove producer dependency',
    'No effect decoder, native proof, wallet interoperability, ledger settlement or normative W-D2 freeze',
    'S0 finite bounds only; no claim of cross-layer correspondence or SHA256 circuit feasibility']};
writeFileSync(new URL('results.json',here),JSON.stringify(result,null,2)+'\n');
console.log(JSON.stringify({testCounts,positive:positive.length,hostile:hostile.length,
  frozenInputsUnchanged:true,readOnlyDependenciesUnchanged:true}));
