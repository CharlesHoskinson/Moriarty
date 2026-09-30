import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

let codec;
try { codec = await import('./codec.mjs'); } catch (error) {
  if (error.code !== 'ERR_MODULE_NOT_FOUND') throw error;
}
const vectors = JSON.parse(readFileSync(new URL('./vectors.json', import.meta.url)));
const fixture = (index = 0) => {
  const value = structuredClone(vectors.vectors[index].inputs);
  value.package.files = value.package.filesHex.map((hex, i) => ({role: i + 1, bytes: Buffer.from(hex, 'hex')}));
  delete value.package.filesHex;
  return value;
};

test('codec implements the proposed closed purpose 1/3/4 API', () => {
  assert.ok(codec, 'the scoped codec API must exist');
  for (const name of ['id', 'encodeImage', 'produceImages', 'compareContent']) assert.equal(typeof codec[name], 'function');
});

for (const [index, vector] of vectors.vectors.entries()) {
  test(`${vector.name}: payload, envelope and digest equal frozen Python vectors`, () => {
    const input = fixture(index);
    const images = codec.produceImages(input.definition, input.package, input.terms);
    for (const name of ['source', 'core', 'policy']) {
      assert.equal(images[name].payload.toString('hex'), vector.images[name].payloadHex);
      assert.equal(images[name].preimage.toString('hex'), vector.images[name].preimageHex);
      assert.equal(images[name].digest, vector.images[name].sha256);
      assert.equal(images[name].payload.length, vector.images[name].payloadBytes);
    }
    assert.deepEqual(images.policyValue, vector.policyInput);
  });
}

const sourceRejection = (label, mutate) => test(label, () => {
  const { definition } = fixture(); mutate(definition);
  assert.throws(() => codec.encodeImage(1, definition), codec.ImageCodecError);
});
sourceRejection('extra fields cannot enter a Source image', x => { x.sourceHash = '00'.repeat(32); });
sourceRejection('agreement and action nominal sorts stay distinct', x => { x.agreementInstanceId.sort = 'action'; });
sourceRejection('reserved Source IDs reject', x => { x.domain.value = 'domain'; });
sourceRejection('wire-only identifier punctuation rejects', x => { x.domain.value = 'Preview:a'; });
sourceRejection('initial digits reject', x => { x.domain.value = '1Preview'; });
sourceRejection('non-ASCII IDs reject', x => { x.domain.value = 'Pr\u00e9view'; });
sourceRejection('ID length over 64 rejects', x => { x.domain.value = 'A'.repeat(65); });
sourceRejection('scale canonical leading zeros reject', x => { x.scale = '00'; });
sourceRejection('scale 19 rejects', x => { x.scale = '19'; });
sourceRejection('numeric coercion rejects', x => { x.scale = 0; });
sourceRejection('old Source version rejects', x => { x.sourceVersion = 5; });
sourceRejection('old Source profile rejects', x => { x.profile = 'moriarty-financial-agreement-source/5'; });
sourceRejection('selector and signed constructor disagreement rejects', x => { x.operationKind = 'Repay'; });
sourceRejection('closed records reject getters', x => { Object.defineProperty(x, 'domain', {get() { throw Error('must not run'); }, enumerable: true}); });
sourceRejection('closed records reject symbols', x => { x[Symbol('hidden')] = 1; });

const policyRejection = (label, mutate) => test(label, () => {
  const policy = structuredClone(vectors.vectors[0].policyInput); mutate(policy);
  assert.throws(() => codec.encodeImage(4, policy), codec.ImageCodecError);
});
policyRejection('policy rejects empty keyRef', x => { x.keyRef = ''; });
policyRejection('policy rejects lone high surrogate', x => { x.keyRef = '\ud800'; });
policyRejection('policy rejects lone low surrogate', x => { x.keyRef = '\udfff'; });
policyRejection('decoded NUL control rejects', x => { x.keyRef = JSON.parse('"\\u0000"'); });
policyRejection('decoded DEL control rejects', x => { x.keyRef = '\x7f'; });
policyRejection('UTF-8 byte cap applies to multibyte keyRef', x => { x.keyRef = '\u00e9'.repeat(513); });
policyRejection('UTF-16 keyRef cap rejects before encoding', x => { x.keyRef = 'k'.repeat(1025); });
policyRejection('round 2^64 rejects instead of truncating', x => { x.validUntil = '18446744073709551616'; });
policyRejection('round leading zeros reject', x => { x.validFrom = '01'; });
policyRejection('reversed validity interval rejects', x => { x.validFrom = '2'; x.validUntil = '1'; });
policyRejection('nominal 2^127 rejects', x => { x.grossCap = '170141183460469231731687303715884105728'; });
policyRejection('negative nominal rejects', x => { x.feeCap = '-1'; });
policyRejection('unbounded integer text rejects', x => { x.grossCap = '1'.repeat(10000); });
policyRejection('float nominal rejects', x => { x.netFloor = '1.0'; });
policyRejection('uppercase hash text rejects', x => { x.sourceHash = 'AA'.repeat(32); });
policyRejection('short hash text rejects', x => { x.coreHash = 'ab'; });
policyRejection('nominal action/core interchange rejects', x => { x.actionId.sort = 'coreProgram'; });
policyRejection('accepted failure variant rejects', x => { x.failureRelation = 'accepted_failure'; });
policyRejection('nonempty retention rejects', x => { x.retainedDuties = []; });
policyRejection('operation variant disagreement rejects', x => { x.operation = structuredClone(vectors.vectors[1].policyInput.operation); });
policyRejection('arbitrary dynamic policy fields reject', x => { x.operation.creditor = {sort: 'account', value: 'Bob'}; });

test('repayment requires the explicit allocation and identity conversion constants', () => {
  for (const key of ['allocation', 'conversion']) {
    const value = structuredClone(vectors.vectors[1].policyInput);
    value.operation[key] = 'other';
    assert.throws(() => codec.encodeImage(4, value), codec.ImageCodecError);
  }
});

test('keyRef 1024 ASCII bytes and 512 two-byte scalars admit exactly', () => {
  for (const keyRef of ['k'.repeat(1024), '\u00e9'.repeat(512)]) {
    const value = structuredClone(vectors.vectors[0].policyInput); value.keyRef = keyRef;
    assert.ok(codec.encodeImage(4, value).payload.length < 4096);
  }
});

test('keyRef preserves Unicode composition, whitespace, case and literal escape content', () => {
  const hashes = ['\u00e9', 'e\u0301', ' key ', 'key', 'Key', '\\u0000'].map(keyRef => {
    const value = structuredClone(vectors.vectors[0].policyInput); value.keyRef = keyRef;
    return codec.encodeImage(4, value).digest;
  });
  assert.equal(new Set(hashes).size, hashes.length);
});

test('equal nominal text is admitted separately in agreement/domain roles', () => {
  const { definition } = fixture();
  definition.agreementInstanceId = codec.id('agreement', 'Same');
  definition.domain = codec.id('domain', 'Same');
  assert.ok(codec.encodeImage(1, definition));
});

const coreRejection = (label, mutate) => test(label, () => {
  const { package: value } = fixture(); mutate(value);
  assert.throws(() => codec.encodeImage(3, value), codec.ImageCodecError);
});
coreRejection('file count must be exactly three', x => { x.fileCount = 2; });
coreRejection('file role order must be 1/2/3', x => { x.files.reverse(); });
coreRejection('extra package files reject', x => { x.files.push({role: 4, bytes: Buffer.alloc(0)}); });
coreRejection('file over 1MiB rejects', x => { x.files[0].bytes = Buffer.alloc(1048577); });
coreRejection('file path cannot replace explicit bytes', x => { x.files[0].bytes = './front.ts'; });
coreRejection('package metadata is outside the image', x => { x.files[0].path = './front.ts'; });
coreRejection('stale package entry labels reject', x => { x.lowerEntry = 'renamed'; });
coreRejection('wrong Core profile rejects', x => { x.coreProfile = 'moriarty-core/4'; });
coreRejection('wrong intent schema rejects', x => { x.intentSchema = 'moriarty-intent/4'; });
coreRejection('shared file backing memory rejects', x => { x.files[0].bytes = new Uint8Array(new SharedArrayBuffer(4)); });

test('all three files at exactly 1MiB admit below total 4MiB cap', () => {
  const { package: value } = fixture();
  value.files = [1, 2, 3].map(role => ({role, bytes: Buffer.alloc(1048576, role)}));
  assert.ok(codec.encodeImage(3, value).payload.length < 4194304);
});

test('every file byte, line ending, BOM and unused branch changes the package digest', () => {
  const { package: value } = fixture(); const original = codec.encodeImage(3, value).digest;
  for (const index of [0, 1, 2]) {
    const changed = fixture().package;
    changed.files[index].bytes = Buffer.concat([changed.files[index].bytes, Buffer.from('// unused repayment branch\r\n\ufeff')]);
    assert.notEqual(codec.encodeImage(3, changed).digest, original);
  }
  value.files[0].bytes = Buffer.from(value.files[0].bytes.toString().replace('\r\n', '\n'));
  assert.notEqual(codec.encodeImage(3, value).digest, original);
});

test('producer recomputes links and rejects incoherent selected package identity', () => {
  const input = fixture();
  input.package.coreProgramId = codec.id('coreProgram', 'RepayAccrualFirst');
  input.package.operationKind = 'Repay';
  assert.throws(() => codec.produceImages(input.definition, input.package, input.terms), codec.ImageCodecError);
});

test('content comparator states its content-only scope for equal and unequal claims', () => {
  const { definition } = fixture(); const digest = vectors.vectors[0].images.source.sha256;
  assert.deepEqual(codec.compareContent(1, definition, digest), {matches: true, digest, claimedDigest: digest,
    scope: 'content-only', authenticated: false});
  assert.equal(codec.compareContent(1, definition, '00'.repeat(32)).matches, false);
  assert.throws(() => codec.compareContent(1, definition, digest.toUpperCase()), codec.ImageCodecError);
});

test('no purpose 2, unknown purpose or implicit suite search is available', () => {
  for (const purpose of [0, 2, 5, '1']) assert.throws(() => codec.encodeImage(purpose, fixture().definition), codec.ImageCodecError);
});

test('object enumeration order does not determine bytes', () => {
  const { definition } = fixture();
  const reverse = Object.fromEntries(Object.entries(definition).reverse());
  assert.deepEqual(codec.encodeImage(1, reverse).preimage, codec.encodeImage(1, definition).preimage);
});

test('policy amount and zero-fee recipient have independent content effects', () => {
  const original = structuredClone(vectors.vectors[0].policyInput);
  const digest = codec.encodeImage(4, original).digest;
  for (const edit of [x => {x.operation.amount = '12';}, x => {x.operation.feeRecipient.value = 'Other';}]) {
    const value = structuredClone(original); edit(value); assert.notEqual(codec.encodeImage(4, value).digest, digest);
  }
});

test('actual current package bytes match independently frozen transfer/repayment digests', () => {
  for (const expected of vectors.actualPackages) {
    const value = fixture(expected.operationKind === 'Transfer' ? 0 : 1).package;
    value.files = vectors.actualModuleInputs.map(input => ({role: input.role,
      bytes: readFileSync(new URL(`../../../src/successor/${input.name}`, import.meta.url))}));
    const result = codec.encodeImage(3, value);
    assert.equal(result.payload.length, expected.payloadBytes);
    assert.equal(result.digest, expected.sha256);
  }
});

test('standalone policy body identity differs without a consumer context assertion', () => {
  const value = structuredClone(vectors.vectors[0].policyInput);
  value.coreProgramId = codec.id('coreProgram', 'RepayAccrualFirst');
  const encoded = codec.encodeImage(4, value);
  assert.notEqual(encoded.digest, vectors.vectors[0].images.policy.sha256);
  assert.equal(codec.compareContent(4, value, encoded.digest).authenticated, false);
});

test('standalone producer keyRef domain error has no consumer anchor', () => {
  const input = fixture(); input.terms.keyRef = '\x1f';
  assert.throws(() => codec.produceImages(input.definition, input.package, input.terms), error => {
    assert.equal(error.code, 'W_D2F_DOMAIN_UNSUPPORTED');
    assert.equal(error.classification, 'BindingRejected');
    assert.equal(error.field, 'policy.keyRef');
    assert.equal(error.inputPath, 'intent.keyRef');
    assert.equal(error.sourcePath, 'intent.keyRef');
    assert.equal(error.comparisonTag, null);
    assert.equal(error.factPath, null);
    assert.equal(error.publishedPost, null);
    assert.equal(error.publishedEffects, null);
    return true;
  });
});

test('maximum 64-byte IDs and scale 18 encode without repair', () => {
  const {definition} = fixture();
  definition.domain = codec.id('domain', 'A'.repeat(64)); definition.scale = '18';
  const encoded = codec.encodeImage(1, definition);
  assert.ok(encoded.payload.length < 4096);
  assert.ok(encoded.payload.includes(Buffer.from('A'.repeat(64), 'ascii')));
});

test('file array extra properties and accessors reject without executing accessors', () => {
  const input = fixture().package; input.files.extra = true;
  assert.throws(() => codec.encodeImage(3, input), codec.ImageCodecError);
  const next = fixture().package;
  Object.defineProperty(next.files, '0', {get() { throw new Error('must not execute'); }});
  assert.throws(() => codec.encodeImage(3, next), codec.ImageCodecError);
});

test('package uses exactly the passed Uint8Array subview bytes', () => {
  const value = fixture().package;
  const view = new Uint8Array([99, 0, 255, 239, 187, 191, 99]).subarray(1, 6);
  value.files[1].bytes = view;
  assert.equal(codec.encodeImage(3, value).digest, vectors.vectors[0].images.core.sha256);
});

test('decoded astral scalar length observes UTF-8 cap simultaneously', () => {
  const value = structuredClone(vectors.vectors[0].policyInput); value.keyRef = '\ud83d\ude00'.repeat(256);
  assert.ok(codec.encodeImage(4, value));
  value.keyRef += '\ud83d\ude00';
  assert.throws(() => codec.encodeImage(4, value), codec.ImageCodecError);
});

test('cross-purpose values cannot be coerced into the other closed records', () => {
  const input = fixture(); const policy = structuredClone(vectors.vectors[0].policyInput);
  for (const [purpose, value] of [[1, input.package], [3, input.definition], [4, input.definition], [1, policy]])
    assert.throws(() => codec.encodeImage(purpose, value), codec.ImageCodecError);
});

test('returned buffers do not alias caller package bytes', () => {
  const value = fixture().package; const image = codec.encodeImage(3, value);
  const expected = Buffer.from(image.preimage); value.files[0].bytes.fill(0);
  assert.deepEqual(image.preimage, expected);
});

test('shadowed byte lengths cannot bypass the exact file cap', () => {
  const input = fixture().package;
  const bytes = new Uint8Array(1048577);
  Object.defineProperty(bytes, 'byteLength', {value: 0});
  input.files[0].bytes = bytes;
  assert.throws(() => codec.encodeImage(3, input), codec.ImageCodecError);
});

test('byte buffer shadow properties do not change the intrinsic view content', () => {
  const input = fixture().package;
  const bytes = new Uint8Array([0, 255, 239, 187, 191]);
  Object.defineProperties(bytes, {length: {value: 0}, byteOffset: {value: 12345}, buffer: {get() { throw Error('must not execute'); }}});
  input.files[1].bytes = bytes;
  assert.equal(codec.encodeImage(3, input).digest, vectors.vectors[0].images.core.sha256);
});

test('Proxy selected ID get cannot diverge between constructor check and encoding', () => {
  const input = fixture(); let reads = 0;
  input.definition.selectedActionId = new Proxy(input.definition.selectedActionId, {
    get(target, key, receiver) {
      if (key === 'value') return ++reads <= 5 ? 'TransferLiteralFee' : 'RepayAccrualFirst';
      return Reflect.get(target, key, receiver);
    },
  });
  const encoded = codec.encodeImage(1, input.definition);
  assert.equal(encoded.digest, vectors.vectors[0].images.source.sha256);
  assert.equal(reads, 0, 'descriptor snapshot never reads caller ID properties');
});

test('Proxy definition asset get cannot change between Source and policy construction', () => {
  const input = fixture(); let reads = 0;
  input.definition = new Proxy(input.definition, {
    get(target, key, receiver) {
      if (key === 'asset') return ++reads === 1 ? target.asset : {sort: 'asset', value: 'Other'};
      return Reflect.get(target, key, receiver);
    },
  });
  const encoded = codec.produceImages(input.definition, input.package, input.terms);
  assert.equal(encoded.source.digest, vectors.vectors[0].images.source.sha256);
  assert.equal(encoded.policy.digest, vectors.vectors[0].images.policy.sha256);
  assert.equal(encoded.policyValue.asset.value, 'A');
  assert.equal(reads, 0, 'cross-phase construction uses the same owned snapshot');
});

test('Proxy ID cannot change to non-ASCII after admission checks', () => {
  const input = fixture(); let reads = 0;
  input.definition.domain = new Proxy(input.definition.domain, {
    get(target, key, receiver) {
      if (key === 'value') return ++reads <= 4 ? 'Preview' : '\u00e9';
      return Reflect.get(target, key, receiver);
    },
  });
  const encoded = codec.encodeImage(1, input.definition);
  assert.equal(encoded.digest, vectors.vectors[0].images.source.sha256);
  assert.equal(reads, 0);
});

test('Proxy descriptor values determine admission and bytes, never divergent get values', () => {
  const input = fixture(); let reads = 0; let captures = 0;
  input.definition.asset = new Proxy(input.definition.asset, {
    getOwnPropertyDescriptor(target, key) { if (key === 'value') captures++; return Reflect.getOwnPropertyDescriptor(target, key); },
    get(target, key, receiver) { reads++; return key === 'value' ? 'Other' : Reflect.get(target, key, receiver); },
  });
  const encoded = codec.produceImages(input.definition, input.package, input.terms);
  assert.equal(encoded.source.digest, vectors.vectors[0].images.source.sha256);
  assert.equal(encoded.policy.digest, vectors.vectors[0].images.policy.sha256);
  assert.equal(captures, 1, 'nested original object descriptors captured exactly once');
  assert.equal(reads, 0);
});

function descriptorOnlyGraph(value, captures = new WeakMap(), counts = []) {
  if (value === null || typeof value !== 'object' || value instanceof Uint8Array) return value;
  if (captures.has(value)) return captures.get(value);
  const target = Array.isArray(value) ? [] : {};
  const descriptorCounts = new Map();
  const proxy = new Proxy(target, {
    get() { throw new Error('caller property get must never run'); },
    getOwnPropertyDescriptor(inner, key) {
      descriptorCounts.set(key, (descriptorCounts.get(key) ?? 0) + 1);
      return Reflect.getOwnPropertyDescriptor(inner, key);
    },
  });
  captures.set(value, proxy); counts.push(descriptorCounts);
  for (const [key, child] of Object.entries(value)) target[key] = descriptorOnlyGraph(child, captures, counts);
  return proxy;
}

for (const [index, vector] of vectors.vectors.entries()) {
  test(`${vector.name}: complete Proxy graph preserves all independent byte vectors`, () => {
    const input = fixture(index); const counts = [];
    const memo = new WeakMap();
    const definition = descriptorOnlyGraph(input.definition, memo, counts);
    const core = descriptorOnlyGraph(input.package, memo, counts);
    const terms = descriptorOnlyGraph(input.terms, memo, counts);
    const images = codec.produceImages(definition, core, terms);
    for (const name of ['source', 'core', 'policy']) {
      assert.equal(images[name].payload.toString('hex'), vector.images[name].payloadHex);
      assert.equal(images[name].preimage.toString('hex'), vector.images[name].preimageHex);
      assert.equal(images[name].digest, vector.images[name].sha256);
    }
    for (const objectCounts of counts) for (const count of objectCounts.values())
      assert.equal(count, 1, 'each caller descriptor is captured exactly once');
  });
}

test('invalid ID descriptor cannot be hidden behind a valid Proxy get', () => {
  const input = fixture(); let reads = 0;
  input.definition.asset = new Proxy({sort: 'asset', value: '\u00e9'}, {
    get(target, key, receiver) { reads++; return key === 'value' ? 'A' : Reflect.get(target, key, receiver); },
  });
  assert.throws(() => codec.encodeImage(1, input.definition), codec.ImageCodecError);
  assert.equal(reads, 0);
});

test('changing ID descriptors are captured once across constructor check and encoding', () => {
  const input = fixture(); let captures = 0;
  input.definition.selectedActionId = new Proxy(input.definition.selectedActionId, {
    getOwnPropertyDescriptor(target, key) {
      const descriptor = Reflect.getOwnPropertyDescriptor(target, key);
      if (key === 'value') descriptor.value = ++captures === 1 ? 'TransferLiteralFee' : 'RepayAccrualFirst';
      return descriptor;
    },
  });
  assert.equal(codec.encodeImage(1, input.definition).digest, vectors.vectors[0].images.source.sha256);
  assert.equal(captures, 1);
});

test('later descriptor traps cannot change captured Source or signer terms', () => {
  const input = fixture(); let triggered = 0;
  input.terms.operation = new Proxy(input.terms.operation, {
    getOwnPropertyDescriptor(target, key) {
      triggered++;
      input.definition.asset.value = 'Other';
      input.definition.domain.value = 'OtherDomain';
      input.terms.signer.value = 'OtherSigner';
      return Reflect.getOwnPropertyDescriptor(target, key);
    },
  });
  const images = codec.produceImages(input.definition, input.package, input.terms);
  assert.ok(triggered > 0);
  assert.equal(images.source.digest, vectors.vectors[0].images.source.sha256);
  assert.equal(images.policy.digest, vectors.vectors[0].images.policy.sha256);
});

test('returned policy has immutable nested owned records and no caller aliases', () => {
  const input = fixture(); const images = codec.produceImages(input.definition, input.package, input.terms);
  input.definition.asset.value = 'Other'; input.terms.operation.amount = '12';
  input.terms.operation.recipient.value = 'OtherRecipient'; input.terms.keyRef = 'otherKey';
  assert.equal(codec.encodeImage(4, images.policyValue).digest, vectors.vectors[0].images.policy.sha256);
  for (const value of [images.policyValue, images.policyValue.asset, images.policyValue.operation,
    images.policyValue.operation.owner, images.policyValue.operation.recipient]) assert.equal(Object.isFrozen(value), true);
  assert.throws(() => { images.policyValue.operation.amount = '12'; }, TypeError);
});

test('shared nested ID aliases are captured once and retained as owned aliases', () => {
  const policy = structuredClone(vectors.vectors[0].policyInput); let captures = 0;
  const shared = new Proxy(policy.operation.owner, {
    getOwnPropertyDescriptor(target, key) { if (key === 'value') captures++; return Reflect.getOwnPropertyDescriptor(target, key); },
    get() { throw Error('must not read alias properties'); },
  });
  policy.operation.owner = shared; policy.operation.recipient = shared;
  const expected = structuredClone(vectors.vectors[0].policyInput);
  expected.operation.recipient.value = expected.operation.owner.value;
  assert.equal(codec.encodeImage(4, policy).digest, codec.encodeImage(4, expected).digest);
  assert.equal(captures, 1);
});

test('Proxy file array and record scalar gets cannot rewrite roles or contents', () => {
  const input = fixture(); let gets = 0;
  input.package.files[0] = new Proxy(input.package.files[0], {
    get() { gets++; throw Error('must use captured role and bytes'); },
  });
  input.package.files = new Proxy(input.package.files, {
    get() { gets++; throw Error('must use captured length and indexes'); },
  });
  assert.equal(codec.encodeImage(3, input.package).digest, vectors.vectors[0].images.core.sha256);
  assert.equal(gets, 0);
});

test('opaque Proxy typed byte wrappers reject through intrinsic slot admission', () => {
  const input = fixture(); input.package.files[0].bytes = new Proxy(input.package.files[0].bytes, {});
  assert.throws(() => codec.encodeImage(3, input.package), codec.ImageCodecError);
});

test('shared byte storage cannot be admitted as a closed operation record', () => {
  const input = fixture(); input.terms.operation = input.package.files[0].bytes;
  assert.throws(() => codec.produceImages(input.definition, input.package, input.terms), codec.ImageCodecError);
});
