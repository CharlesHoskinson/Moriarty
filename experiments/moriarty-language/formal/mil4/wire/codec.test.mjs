import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';

const { fixtures } = JSON.parse(await readFile(new URL('./fixtures.json', import.meta.url)));
let codec;
try { codec = await import('./codec.mjs'); }
catch (error) { if (error.code !== 'ERR_MODULE_NOT_FOUND') throw error; codec = {}; }
const encode = (...args) => { assert.equal(typeof codec.encodeAuthorization, 'function', 'candidate encoder is not implemented'); return codec.encodeAuthorization(...args); };
const decode = (...args) => { assert.equal(typeof codec.decodeAuthorization, 'function', 'candidate decoder is not implemented'); return codec.decodeAuthorization(...args); };
const digest = (...args) => { assert.equal(typeof codec.authorizationDigest, 'function', 'candidate digest is not implemented'); return codec.authorizationDigest(...args); };
const transfer = fixtures[0];
const repayment = fixtures[1];
const fresh = () => structuredClone(transfer.authorization);
const wire = () => Buffer.from(transfer.expected.wireHex, 'hex');
const at = (name, part = 'value') => transfer.expected.fieldOffsets[name][part];
const rejects = (fn, code) => assert.throws(fn, error => error.code === code, `expected ${code}`);
const U64 = (2n ** 64n - 1n).toString();
const S = (2n ** 127n - 1n).toString();

for (const fixture of fixtures) {
  test(`${fixture.id}: encoder matches independently assembled exact bytes`, () => assert.equal(encode(fixture.authorization).toString('hex'), fixture.expected.wireHex));
  test(`${fixture.id}: decoder matches independently specified typed value`, () => assert.deepEqual(decode(Buffer.from(fixture.expected.wireHex, 'hex')), fixture.authorization));
  test(`${fixture.id}: canonical round trip and independent digest`, () => {
    assert.deepEqual(decode(encode(fixture.authorization)), fixture.authorization);
    assert.equal(createHash('sha256').update(Buffer.from(fixture.expected.wireHex, 'hex')).digest('hex'), fixture.expected.digestHex);
    assert.equal(digest(fixture.authorization), fixture.expected.digestHex);
  });
  test(`${fixture.id}: nested operation key order cannot change signed bytes`, () => {
    const a = structuredClone(fixture.authorization);
    a.operation = Object.fromEntries(Object.entries(a.operation).reverse());
    assert.equal(encode(a).toString('hex'), fixture.expected.wireHex);
  });
}
test('fixtures have distinct provisional replay identities', () => {
  const replay = a => JSON.stringify([a.domain, a.signer, a.nonce]);
  assert.equal(new Set(fixtures.map(fixture => replay(fixture.authorization))).size, fixtures.length);
});
test('object insertion order cannot change signed bytes', () => assert.deepEqual(encode(Object.fromEntries(Object.entries(fresh()).reverse())), encode(fresh())));
test('explicit zero fee preserves its bound fee recipient', () => { const a = fresh(); a.operation.fee = '0'; assert.deepEqual(decode(encode(a)), a); });
test('maximum 64-byte nominal ID is admitted', () => { const a = fresh(); a.agreementId = 'a'.repeat(64); assert.deepEqual(decode(encode(a)), a); });
test('maximum numeric widths and scale are admitted', () => {
  const a = fresh(); a.validFrom = U64; a.validUntil = U64; a.grossCap = S; a.feeCap = S; a.netFloor = S; a.operation.amount = S; a.operation.fee = S; a.scale = 38;
  assert.deepEqual(decode(encode(a)), a); // formation only; semantic amount/cap relationships are external
});
for (const [name, mutate, code] of [
  ['unknown root property', a => { a.extra = true; }, 'SHAPE'],
  ['missing explicit empty', a => { delete a.observations; }, 'SHAPE'],
  ['unknown operation property', a => { a.operation.extra = 0; }, 'SHAPE'],
  ['wrong version', a => { a.schemaVersion = 'moriarty-intent/4'; }, 'LITERAL'],
  ['unknown profile', a => { a.profile = 'future'; }, 'LITERAL'],
  ['unknown scheme', a => { a.keyScheme = 'ecdsa_secp256k1_sha256'; }, 'LITERAL'],
  ['wrong source version', a => { a.sourceVersion = 5; }, 'LITERAL'],
  ['wrong core version', a => { a.coreVersion = 4; }, 'LITERAL'],
  ['unknown operation', a => { a.operation.kind = 'mint'; }, 'OPERATION_TAG'],
  ['ID cap+1', a => { a.domain = 'a'.repeat(65); }, 'LENGTH'],
  ['non-ASCII identifier', a => { a.signer = 'alïce'; }, 'ID'],
  ['empty identifier', a => { a.asset = ''; }, 'ID'],
  ['upper-case hex', a => { a.preHead = 'AA'.repeat(32); }, 'HEX'],
  ['wrong hash length', a => { a.effectCommitment = '88'.repeat(33); }, 'HEX'],
  ['wrong key length', a => { a.signerKey = '11'.repeat(31); }, 'HEX'],
  ['leading-zero decimal', a => { a.grossCap = '0110'; }, 'INTEGER'],
  ['numeric JS amount', a => { a.operation.amount = 100; }, 'INTEGER'],
  ['negative decimal', a => { a.feeCap = '-1'; }, 'INTEGER'],
  ['nominal cap+1', a => { a.grossCap = (2n ** 127n).toString(); }, 'RANGE'],
  ['u64 cap+1', a => { a.validUntil = (2n ** 64n).toString(); }, 'RANGE'],
  ['scale cap+1', a => { a.scale = 39; }, 'RANGE'],
  ['zero operation amount', a => { a.operation.amount = '0'; }, 'RANGE'],
  ['reversed validity', a => { a.validFrom = '201'; }, 'VALIDITY'],
  ['nonempty failure branch', a => { a.retainedEffects = ['fee']; }, 'EMPTY'],
  ['nonempty supply field', a => { a.supplyChanges = ['mint']; }, 'EMPTY'],
  ['nonempty observation field', a => { a.observations = ['price']; }, 'EMPTY'],
  ['nonempty disclosure field', a => { a.disclosures = ['secret']; }, 'EMPTY'],
  ['nonempty duty field', a => { a.retainedDuties = ['refund']; }, 'EMPTY'],
  ['delegation', a => { a.delegation = 'delegated'; }, 'LITERAL'],
  ['recovery', a => { a.recovery = 'recover'; }, 'LITERAL'],
  ['unselected failure policy', a => { a.failurePolicy = 'retain-fee'; }, 'LITERAL'],
]) test(`typed rejection: ${name}`, () => { const a = fresh(); mutate(a); rejects(() => encode(a), code); });
test('inherited required fields are rejected', () => { const a = fresh(); delete a.domain; Object.setPrototypeOf(a, { domain: 'midnight:preview' }); rejects(() => encode(a), 'SHAPE'); });
for (const field of ['domain', 'operation.owner']) {
  test(`huge identifier ${field} is length-rejected before alphabet validation`, () => {
    for (const text of ['a'.repeat(1024 * 1024), ' '.repeat(1024 * 1024)]) {
      const a = fresh();
      if (field === 'domain') a.domain = text; else a.operation.owner = text;
      rejects(() => encode(a), 'LENGTH');
    }
  });
}
for (const field of ['validUntil', 'grossCap', 'operation.amount']) {
  test(`huge decimal ${field} is range-rejected before canonicality validation`, () => {
    for (const text of ['9'.repeat(1024 * 1024), 'x'.repeat(1024 * 1024)]) {
      const a = fresh();
      if (field === 'operation.amount') a.operation.amount = text; else a[field] = text;
      rejects(() => encode(a), 'RANGE');
    }
  });
}
test('repayment allocation and conversion are closed', () => {
  for (const field of ['allocation', 'conversion']) { const a = structuredClone(fixtures[1].authorization); a.operation[field] = 'other'; rejects(() => encode(a), 'LITERAL'); }
});
for (const [name, mutate, code] of [
  ['unknown field tag', b => { b[at('domain', 'tag')] = 255; return b; }, 'FIELD_TAG'],
  ['duplicated field tag', b => { b[at('stageId', 'tag')] = 3; return b; }, 'FIELD_TAG'],
  ['wrong header', b => { b[0] = 0; return b; }, 'HEADER'],
  ['unknown profile byte', b => { b[at('profile')] = 2; return b; }, 'LITERAL'],
  ['unknown operation tag', b => { b[at('operation')] = 255; return b; }, 'OPERATION_TAG'],
  ['unknown scheme byte', b => { b[at('keyScheme')] = 2; return b; }, 'LITERAL'],
  ['nominal high bit', b => { b[at('grossCap')] = 128; return b; }, 'RANGE'],
  ['nonzero deferred count', b => { b.writeUInt16BE(1, at('observations')); return b; }, 'EMPTY'],
  ['ID cap+1 bytes', b => { b.writeUInt16BE(65, at('domain')); return b; }, 'LENGTH'],
  ['empty ID bytes', b => { b.writeUInt16BE(0, at('domain')); return b; }, 'ID'],
  ['non-ASCII ID bytes', b => { b[at('domain') + 2] = 255; return b; }, 'ID'],
  ['integer extra padding', b => Buffer.concat([b.subarray(0, at('grossCap')), Buffer.from([0]), b.subarray(at('grossCap'))]), 'FIELD_TAG'],
  ['trailing bytes', b => Buffer.concat([b, Buffer.from([0])]), 'TRAILING'],
  ['record cap+1', () => Buffer.alloc(4097), 'LENGTH'],
]) test(`wire rejection: ${name}`, () => rejects(() => decode(mutate(wire())), code));
for (const fixture of fixtures) {
  test(`every truncation of a valid ${fixture.id} reports TRUNCATED`, () => {
    const b = Buffer.from(fixture.expected.wireHex, 'hex');
    for (let n = 0; n < b.length; n++) rejects(() => decode(b.subarray(0, n)), 'TRUNCATED');
  });
  test(`${fixture.id}: reversed wire validity reports VALIDITY`, () => {
    const b = Buffer.from(fixture.expected.wireHex, 'hex');
    b.writeBigUInt64BE(201n, fixture.expected.fieldOffsets.validFrom.value);
    rejects(() => decode(b), 'VALIDITY');
  });
}
for (const [name, mutate, code] of [
  ['unknown kind', (b, at) => { b[at('operation')] = 3; }, 'OPERATION_TAG'],
  ['obligation length cap+1', (b, at) => { b.writeUInt16BE(65, at('operation.obligationId')); }, 'LENGTH'],
  ['empty payer', (b, at) => { b.writeUInt16BE(0, at('operation.payer')); }, 'ID'],
  ['non-ASCII debtor', (b, at) => { b[at('operation.debtor') + 2] = 255; }, 'ID'],
  ['invalid creditor alphabet', (b, at) => { b[at('operation.creditor') + 2] = 32; }, 'ID'],
  ['amount high bit', (b, at) => { b[at('operation.amount')] = 128; }, 'RANGE'],
  ['zero amount', (b, at) => { b.fill(0, at('operation.amount'), at('operation.amount') + 16); }, 'RANGE'],
  ['unknown allocation', b => { b[b.length - 2] = 2; }, 'LITERAL'],
  ['unknown conversion', b => { b[b.length - 1] = 2; }, 'LITERAL'],
]) test(`repayment wire rejection: ${name}`, () => {
  const b = Buffer.from(repayment.expected.wireHex, 'hex');
  const offset = name => repayment.expected.fieldOffsets[name].value;
  mutate(b, offset); rejects(() => decode(b), code);
});
test('decoder rejects ordinary arrays as bytes', () => rejects(() => decode([...wire()]), 'SHAPE'));
test('decoder uses actual size for a huge view with shadowed byteLength', () => {
  const bytes = new Uint8Array(1024 * 1024); bytes.set(wire());
  Object.defineProperty(bytes, 'byteLength', { value: 0 });
  rejects(() => decode(bytes), 'LENGTH');
});
test('decoder does not read shadowed view metadata getters', () => {
  const bytes = new Uint8Array(wire());
  for (const name of ['byteLength', 'length', 'buffer', 'byteOffset']) {
    Object.defineProperty(bytes, name, { get() { throw new Error(`untrusted ${name} read`); } });
  }
  assert.deepEqual(decode(bytes), transfer.authorization);
});
test('decoder ignores a shorter shadowed view length', () => {
  const bytes = new Uint8Array(wire());
  Object.defineProperty(bytes, 'length', { value: 1 });
  assert.deepEqual(decode(bytes), transfer.authorization);
});
test('decoder rejects a Proxy over a byte view with SHAPE', () => rejects(() => decode(new Proxy(new Uint8Array(wire()), {})), 'SHAPE'));
test('decoder rejects a forged typed-array prototype with SHAPE', () => rejects(() => decode(Object.create(Uint8Array.prototype)), 'SHAPE'));
test('decoder rejects a detached view with SHAPE', () => {
  const bytes = new Uint8Array(wire());
  structuredClone(bytes.buffer, { transfer: [bytes.buffer] });
  rejects(() => decode(bytes), 'SHAPE');
});
test('decoder preserves a byte subview with nonzero offset', () => {
  const expected = wire(); const padded = new Uint8Array(expected.length + 16);
  padded.set(expected, 7);
  assert.deepEqual(decode(padded.subarray(7, 7 + expected.length)), transfer.authorization);
});
test('decoder rejects other typed-array element types with SHAPE', () => rejects(() => decode(new Uint16Array(270)), 'SHAPE'));
test('root Proxy cannot hide reversed validity through changing property reads', () => {
  const a = fresh(); a.validFrom = '201'; let reads = 0;
  const proxy = new Proxy(a, { get(target, name) { if (name === 'validFrom') return ++reads === 1 ? '201' : '100'; return Reflect.get(target, name); } });
  rejects(() => encode(proxy), 'VALIDITY');
});
test('operation Proxy cannot hide zero amount through changing property reads', () => {
  const a = fresh(); a.operation.amount = '0'; let reads = 0;
  a.operation = new Proxy(a.operation, { get(target, name) { if (name === 'amount') return ++reads === 1 ? '0' : '1'; return Reflect.get(target, name); } });
  rejects(() => encode(a), 'RANGE');
});
test('root and nested descriptor snapshots encode without property get traps', () => {
  for (const fixture of fixtures) {
    const a = structuredClone(fixture.authorization); const counts = new Map();
    const handler = field => ({
      get() { throw new Error('property get trap must not run'); },
      getOwnPropertyDescriptor(target, name) {
        const key = `${field}.${name}`; counts.set(key, (counts.get(key) ?? 0) + 1);
        return Reflect.getOwnPropertyDescriptor(target, name);
      },
    });
    a.operation = new Proxy(a.operation, handler('operation'));
    const encoded = encode(new Proxy(a, handler('authorization')));
    assert.equal(encoded.toString('hex'), fixture.expected.wireHex);
    assert.deepEqual(decode(encoded), fixture.authorization);
    assert.equal(counts.size, Object.keys(fixture.authorization).length + Object.keys(fixture.authorization.operation).length);
    for (const count of counts.values()) assert.equal(count, 1, 'each descriptor must be read once');
  }
});
test('operation kind descriptor is snapshotted once', () => {
  const a = fresh(); let reads = 0;
  a.operation = new Proxy(a.operation, {
    getOwnPropertyDescriptor(target, name) {
      const descriptor = Reflect.getOwnPropertyDescriptor(target, name);
      if (name === 'kind' && ++reads > 1) descriptor.value = 'repayment';
      return descriptor;
    },
  });
  const encoded = encode(a);
  assert.equal(encoded.toString('hex'), transfer.expected.wireHex);
  assert.deepEqual(decode(encoded), transfer.authorization);
  assert.equal(reads, 1);
});
for (const [name, mutate] of [
  ['recipient', a => { a.operation.recipient = 'eve'; }],
  ['fee recipient', a => { a.operation.feeRecipient = 'other'; }],
  ['fee cap one unit', a => { a.feeCap = '6'; }],
  ['domain', a => { a.domain = 'midnight:preprod'; }],
  ['source identity', a => { a.sourceHash = '01'.repeat(32); }],
  ['Core identity', a => { a.coreHash = '02'.repeat(32); }],
  ['head', a => { a.preHead = '03'.repeat(32); }],
  ['predecessor', a => { a.predecessor = '04'.repeat(32); }],
  ['replay nonce', a => { a.nonce = '05'.repeat(32); }],
  ['validity', a => { a.validUntil = '201'; }],
  ['signer key', a => { a.signerKey = '06'.repeat(32); }],
  ['effect commitment', a => { a.effectCommitment = '07'.repeat(32); }],
  ['signer identity', a => { a.signer = 'other'; }],
  ['asset', a => { a.asset = 'asset:EUR'; }],
  ['scale', a => { a.scale = 3; }],
  ['policy hash', a => { a.policyHash = '08'.repeat(32); }],
  ['gross cap', a => { a.grossCap = '111'; }],
  ['net floor', a => { a.netFloor = '101'; }],
  ['amount', a => { a.operation.amount = '101'; }],
  ['fee', a => { a.operation.fee = '6'; }],
  ['owner', a => { a.operation.owner = 'other'; }],
  ['program id', a => { a.coreProgramId = 'other-program'; }],
]) test(`content binding discriminator: ${name}`, () => { const a = fresh(); mutate(a); assert.notEqual(digest(a), transfer.expected.digestHex); });
test('repayment creditor substitution changes digest', () => { const a = structuredClone(fixtures[1].authorization); a.operation.creditor = 'eve'; assert.notEqual(digest(a), fixtures[1].expected.digestHex); });
