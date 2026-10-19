/**
 * EUC-012 F02-F13 failure-isolation and publication regressions through the
 * REAL exported compileCustody loaded whole by custody-module-loader.mjs.
 * Controlled fixture dependencies exercise producer control flow only. They
 * are not financial-evaluator, Aiken, SDK, stdlib-tree, native, UPLC, proof,
 * or ledger conformance evidence. The positive case uses a controlled apply
 * adapter that persists the same three apply artifacts as the real adapter;
 * it does not claim a real Aiken application or native correspondence.
 */
import test from 'node:test';
import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadCustodyModule } from './support/custody-module-loader.mjs';
import { createCustodyFixtureWorld } from './support/custody-fixture-world.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const root = join(here, '..');
const producerPath = join(root, 'src', 'compile-custody.mjs');
const publicationPath = join(root, 'src', 'custody-publication.mjs');
const PUBLICATION = { bytes: 529, sha256: 'a87a4ba2bfd2a7ee19c6e433ede509ffc32535788fbf94d2b1d3db338dba6a4a' };
const FIX = join(here, 'fixtures', 'f01', 'designers', '02-escrow-time');
const FILES = {
  designer: { path: join(FIX, 'escrow-custody-v5.mori'), bytes: 2907, sha256: '00c9893ee5d2cbf41adb619bbe2a4b37e4e3f4de64889aa4567bbfe0174ec957' },
  seed: { path: join(FIX, 'financial-seed-custody-v5.json'), bytes: 587, sha256: 'e31a3a7a808c1cecf36768be52cd93d279617d80de7d6f075c3dbb8dd0d471a6' },
  residual: { path: join(FIX, 'financial-residual-custody-v5.json'), bytes: 1208, sha256: '5ab8453e9d22651777600c37984e60553499acc65680fc0ec909c03ce0b8ebd6' },
  cases: { path: join(FIX, 'cases.json'), bytes: 53983, sha256: 'b3e74aba4d06402679421f42fb4bb87ea2bf7ed813390ac0faeab3ceeee76cf3' },
  expectedRelease: { path: join(FIX, 'E13-release-real-custody.expected.json'), bytes: 1173, sha256: '02efee11c65eb6f684523479ef6e8a39d0498b2be79bc5611426e39cee4404e8' },
  expectedRefund: { path: join(FIX, 'E11-refund-real-custody.expected.json'), bytes: 1172, sha256: '6218ba8da1bb67fb660cb486b8376bb774688217a6549c2e5acaf6d8dc159149' },
  oracle: { path: join(here, 'fixtures', 'f01', 't1-independent-oracles-19', 'RESULTS.json'), bytes: 166136, sha256: '3ebe82c927e5693b5714a7133b7a1bcb79df76d6bcd5f84529c1a1db5c9521d8' },
};
const SUCCESS_NAMES = ['plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json'];
const CASH_HASH = 'c1'.repeat(28);
const HEAD_HASH = 'd2'.repeat(28);
const APPLIED_HASH = 'e3'.repeat(28);
function sha256Hex(bytes) { return createHash('sha256').update(bytes).digest('hex'); }
function readPinned(spec, label) {
  const bytes = readFileSync(spec.path);
  assert.strictEqual(bytes.length, spec.bytes, `${label} byte length`);
  assert.strictEqual(sha256Hex(bytes), spec.sha256, `${label} SHA-256 pin`);
  return bytes.toString('utf8');
}
// ROOT-CUSTODY-REFRESH: root mechanically replaces the two unique EXPECTED_PRODUCER_* anchors below with the assembled producer byte length and SHA-256 computed from exact post-assembly bytes, exactly like the loader-API and F01 current anchors. Until that refresh this test is intentionally not runnable as GREEN; the author does not derive the pin from the file read here and does not invent a hash.
const EXPECTED_PRODUCER_BYTES = 31818;
const EXPECTED_PRODUCER_SHA256 = '5a3b6837a5b0f7315415e028f2c2fdf38947dbf252b893e3aeebcb2e90922831';
const producerBytes = readFileSync(producerPath);
assert.strictEqual(producerBytes.length, EXPECTED_PRODUCER_BYTES, 'producer byte length (external root-refreshed pin)');
assert.strictEqual(sha256Hex(producerBytes), EXPECTED_PRODUCER_SHA256, 'producer SHA-256 pin (external root-refreshed pin)');
const producerSource = producerBytes.toString('utf8');
assert.strictEqual(Buffer.byteLength(producerSource, 'utf8'), EXPECTED_PRODUCER_BYTES, 'producer must be valid UTF-8 without lossy replacement');
const producerPin = EXPECTED_PRODUCER_SHA256;
const publicationBytes = readFileSync(publicationPath);
assert.strictEqual(publicationBytes.length, PUBLICATION.bytes);
assert.strictEqual(sha256Hex(publicationBytes), PUBLICATION.sha256);
const publicationSource = publicationBytes.toString('utf8');
const texts = {};
for (const [key, spec] of Object.entries(FILES)) texts[key] = readPinned(spec, key);
function unitManifest() {
  return { schema: 'moriarty-t1-custody-manifest/1', nativePins: false, unitFixture: true, production: false, domainNetwork: 'yaci-isolated-t1', domainDeployment: 't1-custody-revision03', threadPolicyId: 'e5'.repeat(28), cashPolicyId: 'b2'.repeat(28), lenderVkh: 'c3'.repeat(28), borrowerVkh: 'd4'.repeat(28), scriptHash: 'e5'.repeat(28), threadAssetNameHex: '48656164', cashAssetNameHex: '43617368', quantum: '1' };
}
async function setup(mutator = null) {
  const world = createCustodyFixtureWorld({ designerSource: texts.designer, seedText: texts.seed, casesText: texts.cases, expectedReleaseText: texts.expectedRelease, expectedRefundText: texts.expectedRefund, residualText: texts.residual, oracleText: texts.oracle });
  const dependencies = world.dependencies;
  if (mutator) mutator(world, dependencies);
  const loaded = await loadCustodyModule({ producerSource, publicationSource, dependencies, expectedProducerSha256: producerPin, expectedPublicationSha256: PUBLICATION.sha256 });
  assert.strictEqual(loaded.compileCustody, loaded.producerNamespace.compileCustody);
  assert.strictEqual(loaded.verifyCustodyPublication, loaded.producerNamespace.verifyCustodyPublication);
  return { world, loaded };
}
function compile(loaded, world, outDir, aikenRun) {
  world.fs.mkdirSync(outDir, { recursive: true });
  for (const name of SUCCESS_NAMES) world.fs.writeFileSync(join(outDir, name), 'STALE\n');
  return loaded.compileCustody({ sourcePath: '/world/designer/escrow-custody-v5.mori', seedPath: '/world/designer/financial-seed-custody-v5.json', casesPath: '/world/designer/cases.json', expectedReleasePath: '/world/designer/E13-release-real-custody.expected.json', expectedRefundPath: '/world/designer/E11-refund-real-custody.expected.json', residualPath: '/world/designer/financial-residual-custody-v5.json', manifest: unitManifest(), outDir, oracleResultsPath: '/world/oracle/RESULTS.json', aikenRun });
}
function passingCheck(world) {
  return (args, cwd) => {
    if (args[0] !== 'check') throw new Error('passingCheck only handles check');
    const validator = world.fs.readFileSync(join(cwd, 'validators/financial_head.ak'), 'utf8');
    const titles = [...validator.matchAll(/^test ([A-Za-z0-9_]+)\(/gm)].map((match) => match[1]);
    const tests = titles.map((title) => ({ title, status: 'success', kind: 'unit' }));
    return { status: 0, stdout: JSON.stringify({ summary: { fake: true }, modules: [{ name: 'financial_head', test: tests }] }), stderr: '' };
  };
}
function assertNoSuccess(world, outDir, result) {
  assert.notEqual(result?.ok, true);
  assert.equal(result?.blueprintPath ?? null, null);
  for (const name of SUCCESS_NAMES) assert.equal(world.fs.existsSync(join(outDir, name)), false, `${name} survived failure`);
  assert.equal(world.fs.existsSync(join(outDir, 'publication.json')), false, 'publication.json survived failure');
  if (world.fs.existsSync(join(outDir, 'admission.json'))) assert.equal(JSON.parse(world.fs.readFileSync(join(outDir, 'admission.json'), 'utf8')).ok, false);
}
test('EUC-012 F05/F04 an unparseable blueprint returns a normalised coded refusal', async () => {
  const { world, loaded } = await setup();
  const check = passingCheck(world);
  const result = compile(loaded, world, '/world/out-parse', (args, cwd) => args[0] === 'check' ? check(args, cwd) : (world.fs.writeFileSync(join(cwd, 'plutus.json'), '{not json\n'), { status: 0, stdout: '', stderr: '' }));
  assertNoSuccess(world, '/world/out-parse', result);
  assert.equal(result.code, 'COMPILE_EXCEPTION');
  assert.match(String(result.detail), /JSON|Unexpected|not json/i);
});
test('EUC-012 F04 a non-object throw from the injected Aiken runner is guarded', async () => {
  const { world, loaded } = await setup();
  const result = compile(loaded, world, '/world/out-throw', () => { throw 'forced-string-failure'; });
  assertNoSuccess(world, '/world/out-throw', result);
  assert.equal(result.code, 'COMPILE_EXCEPTION');
  assert.match(String(result.detail), /forced-string-failure/);
});
test('EUC-012 F04 a cross-realm error object keeps its code and bare message', async () => {
  const { world, loaded } = await setup();
  const hostile = { name: 'Error', code: 'EIO', message: 'forced-object-failure' };
  const result = compile(loaded, world, '/world/out-object-throw', () => { throw hostile; });
  assertNoSuccess(world, '/world/out-object-throw', result);
  assert.equal(result.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(result.detail), 'EIO: forced-object-failure');
  const nullResult = compile(loaded, world, '/world/out-null-throw', () => { throw null; });
  assertNoSuccess(world, '/world/out-null-throw', nullResult);
  assert.equal(nullResult.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(nullResult.detail), 'null');
});
test('EUC-012 F04 hostile throwing getters and a Proxy do not escape compileCustody', async () => {
  const { world, loaded } = await setup();
  const codeHostile = { message: 'hostile-code-message' };
  Object.defineProperty(codeHostile, 'code', { get() { throw new Error('code-getter-boom'); } });
  const codeResult = compile(loaded, world, '/world/out-hostile-code', () => { throw codeHostile; });
  assertNoSuccess(world, '/world/out-hostile-code', codeResult);
  assert.equal(codeResult.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(codeResult.detail), 'hostile-code-message');
  const messageHostile = { code: 'EHOSTILE', name: 'Error', get message() { throw new Error('message-getter-boom'); }, toString() { return 'Error: hostile-message-fallback'; } };
  const messageResult = compile(loaded, world, '/world/out-hostile-message', () => { throw messageHostile; });
  assertNoSuccess(world, '/world/out-hostile-message', messageResult);
  assert.equal(messageResult.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(messageResult.detail), 'EHOSTILE: hostile-message-fallback');
  const nameHostile = { code: 'ENAME', message: '', toString() { return 'Error: name-fallback'; } };
  Object.defineProperty(nameHostile, 'name', { get() { throw new Error('name-getter-boom'); } });
  const nameResult = compile(loaded, world, '/world/out-hostile-name', () => { throw nameHostile; });
  assertNoSuccess(world, '/world/out-hostile-name', nameResult);
  assert.equal(nameResult.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(nameResult.detail), 'ENAME: Error: name-fallback');
  const proxyHostile = new Proxy({}, { get() { throw new Error('proxy-get-boom'); } });
  const proxyResult = compile(loaded, world, '/world/out-hostile-proxy', () => { throw proxyHostile; });
  assertNoSuccess(world, '/world/out-hostile-proxy', proxyResult);
  assert.equal(proxyResult.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(proxyResult.detail), 'unknown error');
});
test('EUC-012 F02/F03/F06 a failure after blueprint copy removes success artifacts and admission', async () => {
  const { world, loaded } = await setup((w, dependencies) => {
    const writeFileSync = (path, data) => { if (String(path).endsWith('publication.json')) { const error = new Error('forced-publication-write'); error.code = 'EACCES'; throw error; } return w.fs.writeFileSync(path, data); };
    dependencies['node:fs'] = { cpSync: w.fs.cpSync, existsSync: w.fs.existsSync, mkdirSync: w.fs.mkdirSync, readFileSync: w.fs.readFileSync, rmSync: w.fs.rmSync, symlinkSync: w.fs.symlinkSync, writeFileSync };
    dependencies['./apply-head.mjs'] = { ...dependencies['./apply-head.mjs'], applyUnitParameters: ({ blueprintPath, persistDir }) => { const blueprint = JSON.parse(w.fs.readFileSync(blueprintPath, 'utf8')); if (!Array.isArray(blueprint.validators) || blueprint.validators.length === 0) return { ok: false, code: 'AIKEN_CHECK', detail: 'blueprint titles', ledgerExecution: false, nativePins: false }; const appliedBp = { validators: [{ title: 'financial_head.mint', compiledCode: 'abcd', hash: APPLIED_HASH }, { title: 'cash.mint', compiledCode: '1234', hash: CASH_HASH }] }; w.fs.writeFileSync(join(persistDir, 'plutus.applied.json'), JSON.stringify(appliedBp)); w.fs.writeFileSync(join(persistDir, 'cash.script.json'), JSON.stringify({ type: 'PlutusScriptV3', description: 'controlled cash', cborHex: '1234' })); w.fs.writeFileSync(join(persistDir, 'apply-report.json'), JSON.stringify({ ok: true, appliedHash: APPLIED_HASH, cashHash: CASH_HASH, unappliedHash: HEAD_HASH }) + '\n'); return { ok: true, appliedHash: APPLIED_HASH, cashHash: CASH_HASH, unappliedHash: HEAD_HASH, cashCanMint: false, genesis: 'controlled-genesis', instanceId: 'f01-fixture-instance', literals: {}, ledgerExecution: false, nativePins: false }; } };
  });
  const check = passingCheck(world);
  const blueprint = { validators: [{ title: 'cash.mint', compiledCode: '1234', hash: CASH_HASH }, { title: 'financial_head.mint', compiledCode: 'ef01', hash: HEAD_HASH }, { title: 'financial_head.spend', compiledCode: 'ef01', hash: HEAD_HASH }, { title: 'financial_head.else', compiledCode: 'ef01', hash: HEAD_HASH }] };
  const result = compile(loaded, world, '/world/out-postpublish', (args, cwd) => args[0] === 'check' ? check(args, cwd) : (world.fs.writeFileSync(join(cwd, 'plutus.json'), JSON.stringify(blueprint)), { status: 0, stdout: '', stderr: '' }));
  assertNoSuccess(world, '/world/out-postpublish', result);
  assert.equal(result.code, 'COMPILE_EXCEPTION');
  assert.strictEqual(String(result.detail), 'EACCES: forced-publication-write');
  assert.equal(world.fs.existsSync('/world/out-postpublish/admission.json'), false);
});
test('EUC-012 F08 removal failure on a stale artifact returns STALE_ARTIFACT (memory-fs fault seam, not real-FS undeletability)', async () => {
  const { world, loaded } = await setup((w, dependencies) => {
    const rmSync = (path, options) => { if (String(path).endsWith('/plutus.json')) { const error = new Error('forced-stale-removal'); error.code = 'EACCES'; throw error; } return w.fs.rmSync(path, options); };
    dependencies['node:fs'] = { cpSync: w.fs.cpSync, existsSync: w.fs.existsSync, mkdirSync: w.fs.mkdirSync, readFileSync: w.fs.readFileSync, rmSync, symlinkSync: w.fs.symlinkSync, writeFileSync: w.fs.writeFileSync };
  });
  const result = compile(loaded, world, '/world/out-stale', () => { throw new Error('aiken must not run'); });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'STALE_ARTIFACT');
  assert.equal(result.blueprintPath, null);
  assert.equal(world.fs.existsSync('/world/out-stale/plutus.json'), true, 'fault seam leaves the single-representation stale file in place');
});
test('EUC-012 F07 isolated project setup failure does not leak its temporary directory', async () => {
  const { world, loaded } = await setup((w, dependencies) => { dependencies['./apply-head.mjs'] = { ...dependencies['./apply-head.mjs'], STDLIB_TREE_SHA256: 'f'.repeat(64) }; });
  const before = [...world.fs.dirs].filter((path) => path.startsWith('/fixture-tmp/')).sort();
  assert.throws(() => loaded.createIsolatedCustodyProject(), /STDLIB_PIN_MISMATCH/);
  const after = [...world.fs.dirs].filter((path) => path.startsWith('/fixture-tmp/')).sort();
  assert.deepEqual(after, before);
});
test('EUC-012 positive controlled publication has six-artifact manifest correspondence and stale refusal', async () => {
  const { world, loaded } = await setup((w, dependencies) => {
    dependencies['./apply-head.mjs'] = { ...dependencies['./apply-head.mjs'], applyUnitParameters: ({ blueprintPath, persistDir }) => { const blueprint = JSON.parse(w.fs.readFileSync(blueprintPath, 'utf8')); if (!Array.isArray(blueprint.validators) || blueprint.validators.length === 0) return { ok: false, code: 'AIKEN_CHECK', detail: 'blueprint titles', ledgerExecution: false, nativePins: false }; const appliedBp = { validators: [{ title: 'financial_head.mint', compiledCode: 'abcd', hash: APPLIED_HASH }, { title: 'financial_head.spend', compiledCode: 'abcd', hash: APPLIED_HASH }, { title: 'financial_head.else', compiledCode: 'abcd', hash: APPLIED_HASH }, { title: 'cash.mint', compiledCode: '1234', hash: CASH_HASH }] }; w.fs.writeFileSync(join(persistDir, 'plutus.applied.json'), JSON.stringify(appliedBp)); w.fs.writeFileSync(join(persistDir, 'cash.script.json'), JSON.stringify({ type: 'PlutusScriptV3', description: 'controlled cash', cborHex: '1234' })); w.fs.writeFileSync(join(persistDir, 'apply-report.json'), JSON.stringify({ ok: true, appliedHash: APPLIED_HASH, cashHash: CASH_HASH, unappliedHash: HEAD_HASH, cashCanMint: false }) + '\n'); return { ok: true, appliedHash: APPLIED_HASH, cashHash: CASH_HASH, unappliedHash: HEAD_HASH, cashCanMint: false, genesis: 'controlled-genesis', instanceId: 'f01-fixture-instance', literals: {}, ledgerExecution: false, nativePins: false }; } };
  });
  const check = passingCheck(world);
  const blueprint = { validators: [{ title: 'cash.mint', compiledCode: '1234', hash: CASH_HASH }, { title: 'financial_head.mint', compiledCode: 'ef01', hash: HEAD_HASH }, { title: 'financial_head.spend', compiledCode: 'ef01', hash: HEAD_HASH }, { title: 'financial_head.else', compiledCode: 'ef01', hash: HEAD_HASH }] };
  const outDir = '/world/out-positive';
  const result = compile(loaded, world, outDir, (args, cwd) => args[0] === 'check' ? check(args, cwd) : (world.fs.writeFileSync(join(cwd, 'plutus.json'), JSON.stringify(blueprint)), { status: 0, stdout: '', stderr: '' }));
  assert.equal(result.ok, true, `${result.code || ''} ${result.detail || ''}`);
  assert.equal(result.blueprintPath, join(outDir, 'plutus.json'));
  assert.equal(result.publicationPath, join(outDir, 'publication.json'));
  for (const name of ['custody.ak', ...SUCCESS_NAMES, 'admission.json', 'publication.json']) assert.equal(world.fs.existsSync(join(outDir, name)), true, `${name} missing`);
  const manifest = JSON.parse(world.fs.readFileSync(join(outDir, 'publication.json'), 'utf8'));
  assert.deepEqual(manifest.artifacts.map((item) => item.name), ['custody.ak', 'plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json', 'admission.json']);
  for (const artifact of manifest.artifacts) {
    const bytes = Buffer.from(world.fs.readFileSync(join(outDir, artifact.name), 'utf8'), 'utf8');
    assert.strictEqual(artifact.bytes, bytes.length, artifact.name);
    assert.strictEqual(artifact.sha256, sha256Hex(bytes), artifact.name);
  }
  assert.strictEqual(manifest.blueprint.appliedHash, APPLIED_HASH);
  assert.strictEqual(manifest.blueprint.cashHash, CASH_HASH);
  const verified = loaded.verifyCustodyPublication({ outDir });
  assert.equal(verified.ok, true);
  assert.equal(verified.code, 'PUBLICATION_VERIFIED');
  const plutusBefore = world.fs.readFileSync(join(outDir, 'plutus.json'), 'utf8');
  world.fs.writeFileSync(join(outDir, 'plutus.json'), JSON.stringify({ validators: [] }));
  const stale = loaded.verifyCustodyPublication({ outDir });
  assert.equal(stale.ok, false);
  assert.equal(stale.code, 'STALE_MANIFEST');
  assert.equal(stale.blueprintPath, null);
  world.fs.writeFileSync(join(outDir, 'plutus.json'), plutusBefore);
  assert.equal(loaded.verifyCustodyPublication({ outDir }).ok, true, 'restored publication verifies again');
  world.fs.writeFileSync(join(outDir, 'cash.script.json'), JSON.stringify({ type: 'PlutusScriptV3', description: 'tampered', cborHex: '9999' }));
  const cashStale = loaded.verifyCustodyPublication({ outDir });
  assert.equal(cashStale.ok, false);
  assert.equal(cashStale.blueprintPath, null);
});
