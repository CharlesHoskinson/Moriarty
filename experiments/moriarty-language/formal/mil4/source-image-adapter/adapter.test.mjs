import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { parseSource6, Source6Error } from '../../../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../../../src/successor/mil4-s0-source-v6.ts';
import { encodeImage, ImageCodecError } from '../hash-image-codec/codec.mjs';
import { transfer, repay } from './fixtures.mjs';

const api = await import('./adapter.mjs').catch(() => ({}));
const vectors = JSON.parse(readFileSync(new URL('./vectors.json', import.meta.url), 'utf8'));
const expectedDefinition = (kind) => ({ sourceVersion: 6,
  profile: 'moriarty-financial-agreement-source/6', wireProfile: 1,
  agreementInstanceId: {sort: 'agreement', value: 'AgreementA'},
  domain: {sort: 'domain', value: 'D'}, asset: {sort: 'asset', value: 'A'}, scale: '0',
  selectedActionId: {sort: 'action', value: kind === 'Transfer' ? 'TransferLiteralFee' : 'RepayAccrualFirst'},
  operationKind: kind });

test('source-text-only API exists without arbitrary AST projection entry', () => {
  assert.equal(typeof api.projectSource6Definition, 'function');
  assert.equal(typeof api.encodeSource6Definition, 'function');
  assert.equal(typeof api.compareSource6DefinitionContent, 'function');
  assert.deepEqual(Object.keys(api).sort(), ['SOURCE_IMAGE_SUITE', 'SourceImageAdapterError',
    'compareSource6DefinitionContent', 'encodeSource6Definition', 'projectSource6Definition'].sort());
});
for (const [kind, fixture] of [['Transfer', transfer], ['Repay', repay]]) {
  test(`${kind}: real parser and private AST projection retain exact closed fields`, () => {
    const source = fixture();
    const ast = parseSource6(source);
    assert.equal(ast.intent.signedAction.kind, kind);
    assert.equal(prepareSource6S0Unqualified(source).status, 'PreparedUnqualified');
    const definition = api.projectSource6Definition(source);
    assert.deepEqual(definition, expectedDefinition(kind));
    assert.equal(definition.agreementInstanceId.value, ast.programId);
    assert.equal(definition.selectedActionId.value, ast.selected.actionId);
    assert.equal(definition.operationKind, ast.intent.signedAction.kind);
    assert.equal(Object.isFrozen(definition), true);
    for (const name of ['agreementInstanceId', 'domain', 'asset', 'selectedActionId'])
      assert.equal(Object.isFrozen(definition[name]), true);
    assert.throws(() => { definition.asset.value = 'Forged'; }, TypeError);
  });
  test(`${kind}: independent complete expected payload, preimage and digest`, () => {
    const vector = vectors.find(v => v.expectedFields.operationKind === kind);
    const image = api.encodeSource6Definition(fixture());
    assert.equal(image.purpose, 1);
    assert.equal(image.payload.toString('hex'), vector.payloadHex);
    assert.equal(image.preimage.toString('hex'), vector.preimageHex);
    assert.equal(image.digest, vector.sha256);
    assert.deepEqual(image, encodeImage(1, expectedDefinition(kind)));
  });
  for (const [field, value] of [['agreement', 'OtherAgreement'], ['domain', 'OtherDomain'],
    ['asset', 'OtherAsset'], ['scale', '18']]) {
    test(`${kind}: ${field} is committed`, () => {
      assert.notEqual(api.encodeSource6Definition(fixture({[field]: value})).digest,
        api.encodeSource6Definition(fixture()).digest);
    });
  }
  for (const [name, changed] of [
    ['comments and whitespace', s => '// leading\n' + s.replaceAll(';', '; /* inert */\r\n') + '\n// trailing'],
    ['source claim', s => s.replace('claim-source', 'unverified-changed-source-claim')],
    ['policy claim', s => s.replace('claim-policy', 'unverified-changed-policy-claim')],
    ['decoded claim escape', s => s.replace('claim-source', 'claim-\\u0073ource')],
    ['intent key and nonce', s => s.replace('key1', 'other-key').replaceAll('n1', 'n2')],
    ['authenticated counters and round', s => s.replace('round 1;', 'round 2;').replace('work_remaining 10;', 'work_remaining 9;')],
    ['submitted effects', s => s.replace('debit Owner 11;', 'debit Owner 12;').replace('debit Payer 30;', 'debit Payer 31;')],
  ]) {
    test(`${kind}: ${name} is excluded after successful formation`, () => {
      const original = fixture(), changedSource = changed(original);
      assert.notEqual(changedSource, original);
      parseSource6(changedSource);
      assert.deepEqual(api.encodeSource6Definition(changedSource), api.encodeSource6Definition(original));
    });
  }
}

test('suite metadata explicitly identifies version and wire-profile constants', () => {
  assert.deepEqual(api.SOURCE_IMAGE_SUITE, {sourceVersion: 6, wireProfile: 1,
    profile: 'moriarty-financial-agreement-source/6', purpose: 1});
  assert.equal(Object.isFrozen(api.SOURCE_IMAGE_SUITE), true);
  assert.equal(Object.hasOwn(parseSource6(transfer()), 'wireProfile'), false);
});
test('selector and operation kind change together across valid builtins', () => {
  assert.notEqual(api.encodeSource6Definition(transfer()).digest, api.encodeSource6Definition(repay()).digest);
});
test('zero-fee transfer is formed and prepared with the same selected-definition image', () => {
  const source = transfer({fee: '0'});
  assert.equal(prepareSource6S0Unqualified(source).status, 'PreparedUnqualified');
  assert.deepEqual(api.encodeSource6Definition(source), api.encodeSource6Definition(transfer()));
});
test('full repayment is formed and prepared with the same selected-definition image', () => {
  const source = repay({amount: '1010', postPrincipal: '0', postOutstanding: '0', postStatus: 'settled'});
  assert.equal(prepareSource6S0Unqualified(source).status, 'PreparedUnqualified');
  assert.deepEqual(api.encodeSource6Definition(source), api.encodeSource6Definition(repay()));
});
test('stage/effect rejection is outside successful source formation', () => {
  const source = transfer().replace('debit Owner 11;', 'debit Owner 12;');
  assert.equal(prepareSource6S0Unqualified(source).status, 'CoreRejected');
  assert.deepEqual(api.encodeSource6Definition(source), api.encodeSource6Definition(transfer()));
});

for (const [name, change, code] of [
  ['old version', s => s.replace('source/6', 'source/5'), 'SOURCE6_VERSION'],
  ['escaped profile', s => s.replace('source/6', 'source/\\u0036'), 'SOURCE6_VERSION'],
  ['unknown selector', s => s.replace('TransferLiteralFee', 'Arbitrary'), 'SOURCE6_PROFILE_UNSUPPORTED'],
  ['wrong supported selector', s => s.replace('TransferLiteralFee', 'RepayAccrualFirst'), 'SOURCE6_PROFILE_UNSUPPORTED'],
  ['duplicate field', s => s.replace('domain D;', 'domain D; domain Other;'), 'SOURCE6_SHAPE'],
  ['extra wire profile field', s => s.replace('domain D;', 'domain D; wire_profile 1;'), 'SOURCE6_SHAPE'],
  ['noncanonical integer', s => s.replace('scale 0;', 'scale 00;'), 'INVALID_INTEGER'],
  ['scale bound', s => s.replace('scale 0;', 'scale 19;'), 'SOURCE6_RANGE'],
  ['non-ASCII identifier', s => s.replace('AgreementA', 'AgreementÁ'), 'NON_ASCII_IDENTIFIER'],
  ['wrong authenticated owner', s => s.replace('balance Owner', 'balance Other'), 'SOURCE6_CELL_SHAPE'],
  ['trailing source', s => s + ' extra', 'SOURCE6_SHAPE'],
  ['unterminated comment', s => s + ' /*', 'UNTERMINATED_COMMENT'],
  ['source size bound', s => s + ' '.repeat(65537), 'SOURCE_BOUND'],
  ['lone surrogate', s => s + '\ud800', 'INVALID_SURROGATE'],
]) {
  test(`malformed ${name} rejects before projection and encoding`, () => {
    for (const fn of [api.projectSource6Definition, api.encodeSource6Definition])
      assert.throws(() => fn(change(transfer())), e => e instanceof Source6Error && e.code === code);
  });
}
test('repayment inconsistent obligation rejects formation', () => {
  assert.throws(() => api.encodeSource6Definition(repay({outstanding: '1009'})),
    e => e instanceof Source6Error && e.code === 'SOURCE6_RANGE');
});
test('caller objects and Proxy/coercion inputs are not parsed AST authority', () => {
  let gets = 0;
  const proxy = new Proxy(parseSource6(transfer()), {get() { gets++; throw Error('must not read'); }});
  for (const input of [proxy, parseSource6(transfer()), new String(transfer()), null, 6, undefined,
    {[Symbol.toPrimitive]() { gets++; return transfer(); }}]) {
    assert.throws(() => api.encodeSource6Definition(input),
      e => e instanceof api.SourceImageAdapterError && e.code === 'SOURCE_TEXT_REQUIRED');
  }
  assert.equal(gets, 0);
});
test('content comparison never authenticates or fills embedded claims', () => {
  const source = transfer(), digest = api.encodeSource6Definition(source).digest;
  const comparison = api.compareSource6DefinitionContent(source, digest);
  assert.deepEqual(comparison, {matches: true, digest, claimedDigest: digest,
    scope: 'content-only', authenticated: false});
  assert.equal(api.compareSource6DefinitionContent(source, '0'.repeat(64)).matches, false);
  assert.equal(parseSource6(source).selected.sourceHash, 'claim-source');
  assert.throws(() => api.compareSource6DefinitionContent(source, 'not-a-hash'), ImageCodecError);
});
test('malformed Source takes precedence over malformed comparison claim', () => {
  assert.throws(() => api.compareSource6DefinitionContent('garbage', 'not-a-hash'), Source6Error);
});
