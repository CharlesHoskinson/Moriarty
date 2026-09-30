import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { encodeEffects, effectCommitment, compareEffectCommitment } from './codec.mjs';
import { encodeAuthorization, decodeAuthorization, authorizationDigest } from '../wire/codec.mjs';
import { prepareMil4S0 } from '../../../src/successor/mil4-s0-core-v5.ts';

const fixtures = JSON.parse(readFileSync(new URL('./fixtures.json', import.meta.url)));
const clone = value => structuredClone(value);
const outcome = { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [] };
for (const vector of fixtures.positive) {
  test(`frozen exact bytes, SHA256, field26: ${vector.id}`, () => {
    const before = JSON.stringify(vector);
    const bytes = encodeEffects(vector.prepared);
    assert.equal(bytes.length, vector.expected.length);
    assert.equal(bytes.toString('hex'), vector.expected.wireHex);
    assert.equal(effectCommitment(vector.prepared), vector.expected.commitment);
    assert.equal(createHash('sha256').update(bytes).digest('hex'), vector.expected.commitment);
    const wire = encodeAuthorization(vector.authorization);
    assert.equal(decodeAuthorization(wire).effectCommitment, vector.expected.commitment);
    assert.deepEqual(compareEffectCommitment(wire, vector.prepared), {
      status: 'CommitmentEqualUnqualified', commitment: vector.expected.commitment,
    });
    assert.equal(JSON.stringify(vector), before);
  });
  test(`read-only Core/5 literal effects and post: ${vector.id}`, () => {
    const before = JSON.stringify(vector);
    const result = prepareMil4S0(vector.state, vector.coreIntent, vector.prepared.effects,
      vector.prepared.successor, outcome, { state: vector.state, intent: vector.coreIntent,
        round: vector.state.round, expectedSuccessor: vector.prepared.successor,
        requestedOutcome: outcome });
    assert.equal(result.status, 'PreparedUnqualified');
    assert.deepEqual(result.effects, vector.prepared.effects);
    assert.deepEqual(result.candidatePost, vector.expectedPost);
    assert.deepEqual(result.requiredPremises, vector.prepared.requiredPremises);
    assert.equal(JSON.stringify(vector), before);
  });
}
for (const hostile of fixtures.hostile) {
  test(`frozen hostile: ${hostile.id}`, () => {
    const base = fixtures.positive.find(x => x.id === hostile.base);
    const wire = encodeAuthorization(base.authorization);
    if (hostile.expectedCode === 'LITERAL') {
      assert.throws(() => compareEffectCommitment(wire, hostile.prepared), { code: 'LITERAL' });
      return;
    }
    assert.equal(encodeEffects(hostile.prepared).toString('hex'), hostile.expected.wireHex);
    assert.equal(effectCommitment(hostile.prepared), hostile.expected.commitment);
    assert.notEqual(hostile.expected.commitment, base.expected.commitment);
    assert.deepEqual(compareEffectCommitment(wire, hostile.prepared), {
      status: 'Rejected', code: hostile.expectedCode,
    });
  });
}

const malformed = [
 ['digest circularity field','SHAPE',p => { p.authorizationDigest = '11'.repeat(32); }],
 ['commitment circularity field','SHAPE',p => { p.effectCommitment = '11'.repeat(32); }],
 ['signature circularity field','SHAPE',p => { p.signature = '11'.repeat(64); }],
 ['numeric amount','INTEGER',p => { p.effects[0].amount = 11; }],
 ['noncanonical decimal','INTEGER',p => { p.effects[0].amount = '011'; }],
 ['nominal cap plus one','RANGE',p => { p.effects[0].amount = (1n<<127n).toString(); }],
 ['work cap plus one','RANGE',p => { p.consumption.workSpentAfter = (1n<<128n).toString(); }],
 ['round cap plus one','RANGE',p => { p.round = (1n<<64n).toString(); }],
 ['oversized integer text','RANGE',p => { p.round = 'x'.repeat(100000); }],
 ['uppercase hash','HEX',p => { p.successor = 'AA'.repeat(32); }],
 ['invalid id','ID',p => { p.domain = 'D space'; }],
 ['oversized id','LENGTH',p => { p.domain = 'x'.repeat(100000); }],
 ['zero-prefixed replay nonce','REPLAY',p => { p.effects[4].key = '["D","O","0x77"]'; }],
 ['replay whitespace','REPLAY',p => { p.effects[4].key = '[ "D", "O", "'+'77'.repeat(32)+'" ]'; }],
 ['non-array replay','REPLAY',p => { p.effects[4].key = '{"key":"D"}'; }],
 ['oversized replay text','REPLAY',p => { p.effects[4].key = 'x'.repeat(100000); }],
 ['excessive lines','LENGTH',p => { p.effects.push(clone(p.effects[0])); }],
 ['excessive balances','LENGTH',p => { p.footprint.balances.push(clone(p.footprint.balances[0])); }],
 ['excessive replay history','LENGTH',p => { p.consumption.replayBefore = Array(17).fill(p.effects[4].key); }],
 ['record byte cap','LENGTH',p => { const key=JSON.stringify(['D'.repeat(64),'O'.repeat(64),'77'.repeat(32)]); p.consumption.replayBefore=Array(16).fill(key); p.consumption.replayAfter=Array(16).fill(key); }],
 ['unknown kind','VARIANT',p => { p.effects[0].kind = 'Mint'; }],
 ['unknown status','VARIANT',p => { p.operationKind = 'netted'; }],
 ['missing field','SHAPE',p => { delete p.preHead; }],
 ['nested extra field','SHAPE',p => { p.effects[0].note = 'ignored'; }],
 ['inherited field','SHAPE',p => { const head=p.preHead; delete p.preHead; Object.setPrototypeOf(p,{preHead:head}); }],
 ['accessor field','SHAPE',p => { Object.defineProperty(p,'round',{enumerable:true,get(){throw Error('getter read');}}); }],
 ['line accessor field','SHAPE',p => { Object.defineProperty(p.effects[0],'amount',{enumerable:true,get(){throw Error('getter read');}}); }],
 ['nonenumerable field','SHAPE',p => { Object.defineProperty(p,'round',{enumerable:false,value:'100'}); }],
 ['array hole','SHAPE',p => { delete p.effects[1]; }],
 ['array extra property','SHAPE',p => { p.effects.extra = 1; }],
 ['scale cap plus one','RANGE',p => { p.scale = 39; }],
 ['negative zero scale','RANGE',p => { p.scale = -0; }],
 ['wrong core','LITERAL',p => { p.core = 'moriarty-core/4'; }],
];
for (const [name, code, mutate] of malformed) {
  test(`schema rejection: ${name}`, () => {
    const p = clone(fixtures.positive[0].prepared);
    mutate(p);
    assert.throws(() => encodeEffects(p), { code });
  });
}

test('property insertion order is irrelevant; effect order is committed', () => {
  function reverseKeys(x) {
    if (Array.isArray(x)) return x.map(reverseKeys);
    if (x && typeof x === 'object') return Object.fromEntries(Object.entries(x).reverse().map(([k,v])=>[k,reverseKeys(v)]));
    return x;
  }
  assert.equal(encodeEffects(reverseKeys(fixtures.positive[0].prepared)).toString('hex'),
    fixtures.positive[0].expected.wireHex);
});
test('field26 is compared after canonical wire decoding', () => {
  const vector = fixtures.positive[0];
  const authorization = { ...vector.authorization, effectCommitment: 'ff'.repeat(32) };
  assert.deepEqual(compareEffectCommitment(encodeAuthorization(authorization), vector.prepared),
    { status: 'Rejected', code: 'EFFECT_COMMITMENT_MISMATCH' });
  assert.notEqual(authorizationDigest(authorization), authorizationDigest(vector.authorization));
  assert.throws(() => compareEffectCommitment(Buffer.concat([encodeAuthorization(vector.authorization),Buffer.from([0])]),vector.prepared), { code:'TRAILING' });
  assert.throws(() => compareEffectCommitment(vector.authorization,vector.prepared), { code:'SHAPE' });
});
test('hostile re-commitment obtains only equality, demonstrating the semantic limit', () => {
  const hostile = fixtures.hostile[0];
  const base = fixtures.positive.find(x => x.id === hostile.base);
  const authorization = { ...base.authorization, effectCommitment: hostile.expected.commitment };
  assert.equal(compareEffectCommitment(encodeAuthorization(authorization),hostile.prepared).status,
    'CommitmentEqualUnqualified');
  const result = prepareMil4S0(base.state,base.coreIntent,hostile.prepared.effects,
    base.prepared.successor,outcome);
  assert.equal(result.status,'Rejected');
  assert.equal(result.code,'S0_EFFECT_MISMATCH');
});
