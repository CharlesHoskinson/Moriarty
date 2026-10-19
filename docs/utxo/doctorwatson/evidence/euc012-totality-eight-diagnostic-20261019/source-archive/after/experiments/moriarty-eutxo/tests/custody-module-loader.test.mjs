/**
 * TEST-ONLY ordinary Node runtime API test for custody-module-loader.mjs.
 *
 * Run later, under separate review/admission, with:
 *   timeout --signal=TERM --kill-after=5s 90s node --experimental-vm-modules experiments/moriarty-eutxo/tests/custody-module-loader.test.mjs
 * (finite 90s TERM plus 5s KILL, max 1 run; see the author JSON for the
 * exact proposed command). This file has NOT been executed by its author;
 * all execution remains UNRUN and no runtime admission is claimed.
 *
 * What this test does: it reads the real whole producer and publication
 * files itself (explicit public source reads at the caller, the only disk
 * reads of source in this increment), asserts their raw byte lengths and
 * SHA-256 digests BEFORE loading, and passes those digests to the loader as
 * caller-supplied pins. It then loads the unchanged real modules through
 * the vm loader with trusted stub host namespaces and asserts genuine
 * export identity without ever calling compileCustody. Small deliberately
 * altered module texts are used ONLY as loader-API error probes in the
 * clearly labelled ALTERED PROBE sections; an altered text is never the
 * real production module and never evidences F01. No SDK, native tool,
 * Aiken, ledger, proof, compiler child process, external fixture, or new
 * dependency is used. Node vm is JavaScript isolation, not a ledger VM.
 */
import assert from 'node:assert';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { dirname } from 'node:path';

import {
  loadCustodyModule,
  CustodyModuleLoaderError,
  CUSTODY_PRODUCER_DEPENDENCIES,
} from './support/custody-module-loader.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const repoModuleRoot = join(here, '..');
const producerPath = join(repoModuleRoot, 'src', 'compile-custody.mjs');
const publicationPath = join(repoModuleRoot, 'src', 'custody-publication.mjs');

// ROOT-CUSTODY-REFRESH: after assembly, root mechanically replaces the two unique anchors below with the assembled producer byte length and SHA-256 computed from the exact post-assembly file bytes. The author does not compute or invent the new producer pin.
const EXPECTED_PRODUCER_BYTES = 32880;
const EXPECTED_PRODUCER_SHA256 = 'c7ebdd08fdb35dd0aa4bd13092021c50ba929bc473bc9817317a497253e9bc4b';
const EXPECTED_PUBLICATION_BYTES = 529;
const EXPECTED_PUBLICATION_SHA256 = 'a87a4ba2bfd2a7ee19c6e433ede509ffc32535788fbf94d2b1d3db338dba6a4a';
const EXPECTED_SUCCESS_ARTIFACTS = ['plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json'];

function sha256Hex(bytes) {
  return createHash('sha256').update(bytes).digest('hex');
}

// Caller-side whole-source pinning, asserted BEFORE any load. The loader is
// also given these digests as pins; neither hash is baked into the loader,
// so a future separately frozen variant is pinned by its own caller.
// Domain qualification: this caller hashes raw file bytes, whereas the
// loader hashes the UTF-8 re-encoding of the supplied source text; those
// domains coincide for valid UTF-8 sources (as inspected here), not for
// arbitrary lossy-decoded bytes.
const producerBytes = readFileSync(producerPath);
const publicationBytes = readFileSync(publicationPath);
assert.strictEqual(producerBytes.length, EXPECTED_PRODUCER_BYTES, 'producer byte length');
assert.strictEqual(sha256Hex(producerBytes), EXPECTED_PRODUCER_SHA256, 'producer SHA-256 pin');
assert.strictEqual(publicationBytes.length, EXPECTED_PUBLICATION_BYTES, 'publication byte length');
assert.strictEqual(sha256Hex(publicationBytes), EXPECTED_PUBLICATION_SHA256, 'publication SHA-256 pin');
const producerSource = producerBytes.toString('utf8');
const publicationSource = publicationBytes.toString('utf8');

class StubLowerError extends Error {
  constructor(code, detail) {
    super(detail ? `${code}: ${detail}` : String(code));
    this.name = 'StubLowerError';
    this.code = code;
  }
}

function makeDependencies() {
  const calls = [];
  const neverCalled = (name) => (...args) => {
    calls.push(name);
    throw new Error(`stub dependency unexpectedly called during load: ${name}`);
  };
  const dependencies = {
    'node:buffer': { Buffer },
    'node:crypto': { createHash },
    'node:fs': {
      cpSync: neverCalled('cpSync'),
      existsSync: neverCalled('existsSync'),
      mkdirSync: neverCalled('mkdirSync'),
      readFileSync: neverCalled('readFileSync'),
      rmSync: neverCalled('rmSync'),
      symlinkSync: neverCalled('symlinkSync'),
      writeFileSync: neverCalled('writeFileSync'),
    },
    'node:os': { tmpdir: neverCalled('tmpdir') },
    'node:path': { join },
    'node:assert': { deepStrictEqual: assert.deepStrictEqual },
    '../../moriarty-language/src/successor/financial-agreement-source-v5.ts': {
      createFinancialAgreementSourceV5: neverCalled('createFinancialAgreementSourceV5'),
    },
    '../../moriarty-language/src/codec.ts': { canonicalEncode: neverCalled('canonicalEncode') },
    './admit-custody.mjs': { admitCustody: neverCalled('admitCustody'), fenceLifecycle: neverCalled('fenceLifecycle') },
    './custody-oracle-bind.mjs': { bindOracleSpends: neverCalled('bindOracleSpends') },
    './custody-capacity.mjs': { bindCapacity: neverCalled('bindCapacity') },
    './custody-time.mjs': {
      actionTimeGuards: neverCalled('actionTimeGuards'),
      isFixedUtxoProbe: neverCalled('isFixedUtxoProbe'),
      isProductionProbeTime: neverCalled('isProductionProbeTime'),
      substituteTimeGuards: neverCalled('substituteTimeGuards'),
    },
    './lower-custody.mjs': { lowerCustody: neverCalled('lowerCustody'), LowerError: StubLowerError },
    './compile-public.mjs': {
      parseAikenCheckJson: neverCalled('parseAikenCheckJson'),
      ROOT: '/custody-loader-test-root',
      runAiken: neverCalled('runAiken'),
    },
    './apply-head.mjs': {
      applyUnitParameters: neverCalled('applyUnitParameters'),
      CASH_VALIDATOR: '',
      STDLIB_TREE: '/custody-loader-test-stdlib-tree',
      STDLIB_TREE_SHA256: '0'.repeat(64),
      stdlibTreeSha256: neverCalled('stdlibTreeSha256'),
    },
  };
  assert.deepStrictEqual(
    Object.keys(dependencies).sort(),
    Object.keys(CUSTODY_PRODUCER_DEPENDENCIES).sort(),
    'stub dependencies cover exactly the 15 declared specifiers',
  );
  return { dependencies, calls };
}

async function expectLoaderError(promise, code) {
  await assert.rejects(promise, (error) => {
    assert.ok(error instanceof CustodyModuleLoaderError, `expected CustodyModuleLoaderError ${code}, got ${error}`);
    assert.strictEqual(error.code, code, `expected loader error code ${code}`);
    return true;
  });
}

async function expectCodedOrMessage(promise, code) {
  await assert.rejects(promise, (error) => {
    const matches = (error && error.code === code) || String(error && error.message).includes(code);
    assert.ok(matches, `expected rejection carrying ${code}, got ${error}`);
    return true;
  });
}

// ---------------------------------------------------------------------------
// Real-module success path: unchanged, hash-pinned producer + publication.
// compileCustody is returned and identity-checked but NEVER called. The
// empty calls[] check below observes only the neverCalled counting stubs;
// stronger non-invocation of compileCustody rests on loader/test control
// flow (the loader never calls it and this test never invokes it).
// ---------------------------------------------------------------------------
{
  const { dependencies, calls } = makeDependencies();
  const loaded = await loadCustodyModule({
    producerSource,
    publicationSource,
    dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256,
    expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  });
  assert.strictEqual(typeof loaded.compileCustody, 'function', 'compileCustody is a function');
  assert.strictEqual(loaded.verifyCustodyPublication, loaded.producerNamespace.verifyCustodyPublication, 'returned verifyCustodyPublication is the evaluated producer export');
  // Genuine export identity against the intentionally retained raw namespace.
  assert.strictEqual(loaded.compileCustody, loaded.producerNamespace.compileCustody, 'returned compileCustody is the evaluated producer export');
  assert.strictEqual(loaded.createIsolatedCustodyProject, loaded.producerNamespace.createIsolatedCustodyProject, 'createIsolatedCustodyProject identity');
  assert.strictEqual(loaded.LowerError, loaded.producerNamespace.LowerError, 'LowerError namespace identity');
  assert.strictEqual(loaded.LowerError, StubLowerError, 'producer re-exports the supplied LowerError binding unchanged');
  // Genuine publication identity and import/export binding: the reserved
  // specifier was linked to this evaluated publication module, whose three
  // exports are exactly the names the real producer imports.
  assert.strictEqual(loaded.publication.shouldApply, loaded.publicationNamespace.shouldApply, 'publication shouldApply identity');
  assert.strictEqual(loaded.publication.shouldPublish, loaded.publicationNamespace.shouldPublish, 'publication shouldPublish identity');
  assert.strictEqual(loaded.publication.SUCCESS_ARTIFACTS, loaded.publicationNamespace.SUCCESS_ARTIFACTS, 'publication SUCCESS_ARTIFACTS identity');
  assert.strictEqual(loaded.publication.shouldApply({ checkOk: true, compiled: true }), true, 'shouldApply true case');
  assert.strictEqual(loaded.publication.shouldApply({ checkOk: true, compiled: false }), false, 'shouldApply false case');
  assert.strictEqual(loaded.publication.shouldPublish({ checkOk: true, compiled: true, applied: { ok: true } }), true, 'shouldPublish true case');
  assert.strictEqual(loaded.publication.shouldPublish({ checkOk: true, compiled: true, applied: null }), false, 'shouldPublish false case');
  // SUCCESS_ARTIFACTS is one export holding four filenames. Compare a
  // host-realm copy here; the cross-realm probe below explains why.
  assert.deepStrictEqual(Array.from(loaded.publication.SUCCESS_ARTIFACTS), EXPECTED_SUCCESS_ARTIFACTS, 'SUCCESS_ARTIFACTS four filenames');
  assert.ok(Object.isFrozen(loaded.publication.SUCCESS_ARTIFACTS), 'SUCCESS_ARTIFACTS is frozen');
  assert.ok(Object.isFrozen(loaded), 'returned accessor object is frozen');
  assert.ok(Object.isFrozen(loaded.publication), 'returned publication accessor is frozen');
  assert.deepStrictEqual(calls, [], 'no neverCalled counting stub was invoked during load (uncounted real join/createHash/deepStrictEqual excluded)');

  // One genuine cross-realm node:assert probe, relevant to later fixtures:
  // SUCCESS_ARTIFACTS was created in the vm realm, so its Array prototype is
  // not the host Array.prototype, host node:assert deepStrictEqual rejects
  // it against a host literal on prototype grounds, and a host-realm copy
  // (Array.from) is accepted. Later fixture owners must use such a realm
  // strategy; this probe does not solve the financial fixture world.
  const vmArtifacts = loaded.publicationNamespace.SUCCESS_ARTIFACTS;
  assert.notStrictEqual(Object.getPrototypeOf(vmArtifacts), Array.prototype, 'vm-realm array has a non-host Array prototype');
  assert.throws(
    () => assert.deepStrictEqual(vmArtifacts, EXPECTED_SUCCESS_ARTIFACTS),
    assert.AssertionError,
    'host deepStrictEqual rejects a vm-realm array against a host literal',
  );
  assert.deepStrictEqual(Array.from(vmArtifacts), EXPECTED_SUCCESS_ARTIFACTS, 'host-realm copy passes deepStrictEqual');
}

// Caller-pin behaviour on the real texts: a mutated text under the real pin,
// and a malformed pin, are both rejected before any evaluation.
{
  const { dependencies } = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource: `${producerSource}\n// caller-side mutation probe\n`,
    publicationSource,
    dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256,
    expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'SOURCE_PIN_MISMATCH');
  const fresh = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource,
    publicationSource,
    dependencies: fresh.dependencies,
    expectedProducerSha256: 'not-a-sha256',
  }), 'INVALID_SOURCE_PIN');
}

// Coded dependency-surface rejections (real sources; failure is in
// validateDependencies before any module is evaluated).
{
  const surplus = makeDependencies();
  surplus.dependencies['./surplus-probe.mjs'] = {};
  await expectLoaderError(loadCustodyModule({
    producerSource, publicationSource, dependencies: surplus.dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256, expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'UNKNOWN_DEPENDENCY');

  const reserved = makeDependencies();
  reserved.dependencies['./custody-publication.mjs'] = {};
  await expectLoaderError(loadCustodyModule({
    producerSource, publicationSource, dependencies: reserved.dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256, expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'RESERVED_DEPENDENCY');

  const missing = makeDependencies();
  delete missing.dependencies['node:os'];
  await expectLoaderError(loadCustodyModule({
    producerSource, publicationSource, dependencies: missing.dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256, expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'MISSING_DEPENDENCY');

  const missingExport = makeDependencies();
  missingExport.dependencies['node:path'] = {};
  await expectLoaderError(loadCustodyModule({
    producerSource, publicationSource, dependencies: missingExport.dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256, expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'MISSING_EXPORT');
}

// ---------------------------------------------------------------------------
// ALTERED PROBES. Each text below is a small deliberately altered module
// used solely to probe loader API error behaviour. None is the real
// production producer or publication and none evidences F01. Each probe
// pins its own altered text, exactly as a separately frozen variant would.
// ---------------------------------------------------------------------------
const pinOf = (text) => sha256Hex(Buffer.from(text, 'utf8'));

// Unknown static import specifier: coded UNKNOWN_IMPORT at link.
{
  const alteredProducer = "import { probe } from './not-a-declared-specifier.mjs';\nexport function compileCustody() { return probe; }\nexport function createIsolatedCustodyProject() { return null; }\nexport class LowerError extends Error {}\n";
  const { dependencies } = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource: alteredProducer, publicationSource, dependencies,
    expectedProducerSha256: pinOf(alteredProducer), expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'UNKNOWN_IMPORT');
}

// Dynamic import: refused with DYNAMIC_IMPORT_REFUSED.
{
  const alteredProducer = "export function compileCustody() { return null; }\nexport function createIsolatedCustodyProject() { return null; }\nexport class LowerError extends Error {}\nawait import('./dynamic-probe.mjs');\n";
  const { dependencies } = makeDependencies();
  await expectCodedOrMessage(loadCustodyModule({
    producerSource: alteredProducer, publicationSource, dependencies,
    expectedProducerSha256: pinOf(alteredProducer), expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'DYNAMIC_IMPORT_REFUSED');
}

// Undeclared export name from an allowed specifier: platform link error,
// NOT a coded CustodyModuleLoaderError. This documents the narrowed claim.
{
  const alteredProducer = "import { definitelyNotAnExport } from 'node:path';\nexport function compileCustody() { return definitelyNotAnExport; }\nexport function createIsolatedCustodyProject() { return null; }\nexport class LowerError extends Error {}\n";
  const { dependencies } = makeDependencies();
  await assert.rejects(loadCustodyModule({
    producerSource: alteredProducer, publicationSource, dependencies,
    expectedProducerSha256: pinOf(alteredProducer), expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), (error) => {
    assert.ok(!(error instanceof CustodyModuleLoaderError), `expected a platform link error, got coded loader error ${error && error.code}`);
    assert.match(String(error && error.message), /does not provide an export|SyntaxError/i);
    return true;
  });
}

// Altered producer missing compileCustody: coded MISSING_EXPORT.
{
  const alteredProducer = "export function createIsolatedCustodyProject() { return null; }\nexport class LowerError extends Error {}\n";
  const { dependencies } = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource: alteredProducer, publicationSource, dependencies,
    expectedProducerSha256: pinOf(alteredProducer), expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'MISSING_EXPORT');
}

// Altered producer with a non-function LowerError: coded INVALID_EXPORT.
{
  const alteredProducer = "export function compileCustody() { return null; }\nexport function createIsolatedCustodyProject() { return null; }\nexport const LowerError = 42;\n";
  const { dependencies } = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource: alteredProducer, publicationSource, dependencies,
    expectedProducerSha256: pinOf(alteredProducer), expectedPublicationSha256: EXPECTED_PUBLICATION_SHA256,
  }), 'INVALID_EXPORT');
}

// Altered publication with a wrong SUCCESS_ARTIFACTS shape (paired with the
// real producer): coded INVALID_EXPORT from publication validation.
{
  const alteredPublication = "export const SUCCESS_ARTIFACTS = Object.freeze(['only.json']);\nexport function shouldApply() { return true; }\nexport function shouldPublish() { return true; }\n";
  const { dependencies } = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource, publicationSource: alteredPublication, dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256, expectedPublicationSha256: pinOf(alteredPublication),
  }), 'INVALID_EXPORT');
}

// Altered publication missing shouldPublish: coded MISSING_EXPORT,
// because loader publication validation runs before producer linking.
{
  const alteredPublication = "export const SUCCESS_ARTIFACTS = Object.freeze(['plutus.json', 'plutus.applied.json', 'cash.script.json', 'apply-report.json']);\nexport function shouldApply() { return true; }\n";
  const { dependencies } = makeDependencies();
  await expectLoaderError(loadCustodyModule({
    producerSource, publicationSource: alteredPublication, dependencies,
    expectedProducerSha256: EXPECTED_PRODUCER_SHA256, expectedPublicationSha256: pinOf(alteredPublication),
  }), 'MISSING_EXPORT');
}

console.log('custody-module-loader.test.mjs: all loader API assertions passed (UNRUN at authoring time)');
