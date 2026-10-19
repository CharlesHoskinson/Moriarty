/**
 * TEST-ONLY whole-module loader for compile-custody.mjs.
 *
 * Identity contract (caller pinning, not a baked-in pin):
 * This loader evaluates caller-supplied whole source TEXT. It never reads
 * source from disk and never, by itself, knows which bytes are the real
 * producer or publication. Identity is established only when the caller
 * (a) hashes the exact bytes it read (for example the raw file bytes) and
 * asserts the expected digests itself before the call, and (b) passes those
 * digests as expectedProducerSha256 / expectedPublicationSha256. When a pin
 * is supplied it must be 64 lowercase hex characters and is enforced here
 * against the UTF-8 bytes of the supplied text before any module is
 * constructed, linked, or evaluated. Domain qualification: the caller
 * hashes raw bytes, whereas this loader hashes the UTF-8 re-encoding of
 * the supplied source text; those domains coincide for valid UTF-8
 * sources (as inspected here), not for arbitrary lossy-decoded bytes.
 * No producer or publication digest is
 * baked into this helper: a later separately frozen variant (for example an
 * old producer used by a future F01 RED run) is loaded under its own frozen
 * pin supplied by that caller, not under the current GREEN digest. When a
 * pin is omitted this loader provides NO source-identity guarantee at all;
 * any non-empty string with the required exports can load.
 *
 * Linking contract, narrowed to what the code actually does.
 * Coded CustodyModuleLoaderError rejections: INVALID_SOURCE (missing or
 * empty source text), INVALID_SOURCE_PIN (malformed pin),
 * SOURCE_PIN_MISMATCH (pin does not match supplied text),
 * INVALID_DEPENDENCIES, UNKNOWN_DEPENDENCY (any supplied dependency key
 * that is not one of the 14 declared specifiers, including surplus keys),
 * RESERVED_DEPENDENCY (caller trying to supply
 * ./custody-publication.mjs, which is always linked to the evaluated
 * publicationSource), MISSING_DEPENDENCY (a declared specifier absent from
 * dependencies), INVALID_NAMESPACE, MISSING_EXPORT (a declared export name
 * absent from a supplied namespace, or a required producer export absent
 * after evaluation), INVALID_EXPORT (SUCCESS_ARTIFACTS is not the frozen
 * four-filename array below, or producer LowerError is not a
 * function/class), UNKNOWN_IMPORT (a static import specifier, or any import
 * attempted by the publication module, that is neither the reserved
 * publication specifier nor a declared dependency specifier), and
 * DYNAMIC_IMPORT_REFUSED (any dynamic import()).
 * Accepted changes (not rejections): removing a static import from the
 * producer still loads, provided the caller still supplies all 14 declared
 * namespaces (the unused namespace is simply never linked); renaming a
 * local import binding while keeping the specifier and exported name loads
 * by design. Publication validation runs before producer construction
 * and linking: a missing shouldApply/shouldPublish is coded
 * MISSING_EXPORT, and a missing/invalid SUCCESS_ARTIFACTS shape is coded
 * INVALID_EXPORT. The publication module's own link callback rejects
 * ANY requested static import as UNKNOWN_IMPORT, unlike the producer's
 * finite/reserved specifier map. Platform-dependent failures (NOT coded
 * loader errors) are only genuinely unprovided export names at later
 * platform link stages (for example an undeclared name from a declared
 * SyntheticModule such as 'node:path', or a publication name the producer
 * imports that was not pre-validated above) and syntactically invalid
 * source text, which fail in Node linking/parsing with platform
 * SyntaxError/LinkError-style errors that this loader does not wrap.
 *
 * Trust and evaluation contract: dependencies are trusted HOST code, not a
 * sandbox boundary. Validation and snapshotting read properties from the
 * caller namespaces, so getters, proxies, and prototype-chain values on
 * those namespaces execute host code during load; only the listed exported
 * names are copied (by reference) into SyntheticModules, and extra
 * properties on a supplied namespace are not exposed to the producer.
 * Evaluating the producer runs its top-level module code in the vm realm:
 * in the current producer that top level computes
 * join(ROOT, 'eutxo-custody') and assigns the caller STDLIB_TREE value, so
 * load requires a caller join that accepts those arguments, a
 * join-compatible ROOT string, and tolerates evaluation of the whole
 * producer top level; a throwing or false host value surfaces as an uncoded
 * host error and load does not succeed. This loader never calls
 * compileCustody. Node vm module isolation is JavaScript isolation only;
 * it is not a ledger VM, and it does not isolate trusted caller namespaces.
 *
 * Raw namespaces are INTENTIONAL: the returned producerNamespace and
 * publicationNamespace are the raw evaluated module namespace objects,
 * retained solely so callers and tests can make genuine export-identity
 * assertions (for example returned compileCustody ===
 * producerNamespace.compileCustody). They are not an oversight to remove.
 *
 * Requires Node's --experimental-vm-modules option at the call site.
 */
import { createHash } from 'node:crypto';
import { createContext, SourceTextModule, SyntheticModule } from 'node:vm';

export const CUSTODY_PUBLICATION_SPECIFIER = './custody-publication.mjs';

export const EXPECTED_SUCCESS_ARTIFACTS = Object.freeze([
  'plutus.json',
  'plutus.applied.json',
  'cash.script.json',
  'apply-report.json',
]);

export const CUSTODY_PRODUCER_DEPENDENCIES = Object.freeze({
  'node:crypto': Object.freeze(['createHash']),
  'node:fs': Object.freeze(['cpSync', 'existsSync', 'mkdirSync', 'readFileSync', 'rmSync', 'symlinkSync', 'writeFileSync']),
  'node:os': Object.freeze(['tmpdir']),
  'node:path': Object.freeze(['join']),
  'node:assert': Object.freeze(['deepStrictEqual']),
  '../../moriarty-language/src/successor/financial-agreement-source-v5.ts': Object.freeze(['createFinancialAgreementSourceV5']),
  '../../moriarty-language/src/codec.ts': Object.freeze(['canonicalEncode']),
  './admit-custody.mjs': Object.freeze(['admitCustody', 'fenceLifecycle']),
  './custody-oracle-bind.mjs': Object.freeze(['bindOracleSpends']),
  './custody-capacity.mjs': Object.freeze(['bindCapacity']),
  './custody-time.mjs': Object.freeze(['actionTimeGuards', 'isFixedUtxoProbe', 'isProductionProbeTime', 'substituteTimeGuards']),
  './lower-custody.mjs': Object.freeze(['lowerCustody', 'LowerError']),
  './compile-public.mjs': Object.freeze(['parseAikenCheckJson', 'ROOT', 'runAiken']),
  './apply-head.mjs': Object.freeze(['applyUnitParameters', 'CASH_VALIDATOR', 'STDLIB_TREE', 'STDLIB_TREE_SHA256', 'stdlibTreeSha256']),
});

export class CustodyModuleLoaderError extends Error {
  constructor(code, detail) {
    super(detail ? `${code}: ${detail}` : code);
    this.name = 'CustodyModuleLoaderError';
    this.code = code;
  }
}

function fail(code, detail) {
  throw new CustodyModuleLoaderError(code, detail);
}

function isRecord(value) {
  return (typeof value === 'object' && value !== null && !Array.isArray(value)) || typeof value === 'function';
}

function requireSource(name, value) {
  if (typeof value !== 'string' || value.length === 0) fail('INVALID_SOURCE', `${name} must be a non-empty whole source string supplied by the caller`);
  return value;
}

function sha256HexOfText(text) {
  return createHash('sha256').update(text, 'utf8').digest('hex');
}

function enforcePin(name, text, pin) {
  if (pin === undefined) return;
  if (typeof pin !== 'string' || !/^[0-9a-f]{64}$/.test(pin)) {
    fail('INVALID_SOURCE_PIN', `${name} pin must be 64 lowercase hex characters`);
  }
  if (sha256HexOfText(text) !== pin) {
    fail('SOURCE_PIN_MISMATCH', `${name} text does not match the caller-supplied SHA-256 pin`);
  }
}

function rejectDynamicImport(specifier) {
  fail('DYNAMIC_IMPORT_REFUSED', String(specifier));
}

function initializeImportMeta(meta, module) {
  meta.url = module.identifier;
}

function validateDependencies(dependencies) {
  if (!isRecord(dependencies) || typeof dependencies !== 'object') fail('INVALID_DEPENDENCIES', 'dependencies must be one finite object keyed by exact import specifier');
  const keys = Reflect.ownKeys(dependencies);
  for (const key of keys) {
    if (typeof key !== 'string') fail('UNKNOWN_DEPENDENCY', 'dependency keys must be exact string specifiers');
    if (key === CUSTODY_PUBLICATION_SPECIFIER) fail('RESERVED_DEPENDENCY', './custody-publication.mjs is linked only to the supplied real publication source');
    if (!Object.prototype.hasOwnProperty.call(CUSTODY_PRODUCER_DEPENDENCIES, key)) fail('UNKNOWN_DEPENDENCY', key);
  }
  for (const [specifier, exportNames] of Object.entries(CUSTODY_PRODUCER_DEPENDENCIES)) {
    if (!Object.prototype.hasOwnProperty.call(dependencies, specifier)) fail('MISSING_DEPENDENCY', specifier);
    const namespace = dependencies[specifier];
    if (!isRecord(namespace)) fail('INVALID_NAMESPACE', specifier);
    for (const exportName of exportNames) {
      if (!(exportName in namespace)) fail('MISSING_EXPORT', `${specifier}#${exportName}`);
    }
  }
}

function validatePublicationNamespace(publicationNamespace) {
  if (typeof publicationNamespace.shouldApply !== 'function' || typeof publicationNamespace.shouldPublish !== 'function') {
    fail('MISSING_EXPORT', 'custody-publication.mjs#shouldApply|shouldPublish');
  }
  const artifacts = publicationNamespace.SUCCESS_ARTIFACTS;
  const shapeOk = Array.isArray(artifacts)
    && artifacts.length === EXPECTED_SUCCESS_ARTIFACTS.length
    && EXPECTED_SUCCESS_ARTIFACTS.every((name, index) => artifacts[index] === name)
    && Object.isFrozen(artifacts);
  if (!shapeOk) {
    fail('INVALID_EXPORT', 'custody-publication.mjs#SUCCESS_ARTIFACTS must be the frozen four-filename array [plutus.json, plutus.applied.json, cash.script.json, apply-report.json]');
  }
}

/**
 * Link and evaluate whole producer and publication source texts.
 *
 * @param {{producerSource: string, publicationSource: string,
 *   dependencies: object, expectedProducerSha256?: string,
 *   expectedPublicationSha256?: string}} input
 *   producerSource and publicationSource are complete source texts from the
 *   caller. The caller establishes source identity: it must hash the exact
 *   bytes it read and assert the digests itself before this call, and should
 *   pass the same digests as the expected*Sha256 pins so this loader also
 *   enforces them before evaluation; without pins no identity is guaranteed
 *   and any separately frozen variant must be pinned by its own caller.
 *   dependencies must contain exactly the 14 non-publication specifiers in
 *   CUSTODY_PRODUCER_DEPENDENCIES, each mapped to a trusted host namespace
 *   object holding at least the listed exported names.
 * @returns {Promise<object>} Frozen accessors plus the intentionally raw
 *   producerNamespace and publicationNamespace for export-identity
 *   assertions. This evaluates top-level module code only; it never calls
 *   compileCustody.
 */
export async function loadCustodyModule({ producerSource, publicationSource, dependencies, expectedProducerSha256, expectedPublicationSha256 } = {}) {
  const producerText = requireSource('producerSource', producerSource);
  const publicationText = requireSource('publicationSource', publicationSource);
  enforcePin('producerSource', producerText, expectedProducerSha256);
  enforcePin('publicationSource', publicationText, expectedPublicationSha256);
  validateDependencies(dependencies);
  const context = createContext(Object.create(null), {
    name: 'custody-module-loader-test-context',
    codeGeneration: { strings: false, wasm: false },
  });
  const publicationModule = new SourceTextModule(publicationText, {
    identifier: 'custody-publication.mjs',
    context,
    initializeImportMeta,
    importModuleDynamically: rejectDynamicImport,
  });
  await publicationModule.link((specifier) => {
    fail('UNKNOWN_IMPORT', `custody-publication.mjs -> ${String(specifier)}`);
  });
  await publicationModule.evaluate();
  const publicationNamespace = publicationModule.namespace;
  validatePublicationNamespace(publicationNamespace);
  const dependencyModules = Object.create(null);
  for (const [specifier, exportNames] of Object.entries(CUSTODY_PRODUCER_DEPENDENCIES)) {
    const supplied = dependencies[specifier];
    const snapshot = Object.create(null);
    for (const exportName of exportNames) snapshot[exportName] = supplied[exportName];
    dependencyModules[specifier] = new SyntheticModule([...exportNames], function () {
      for (const exportName of exportNames) this.setExport(exportName, snapshot[exportName]);
    }, { identifier: specifier, context });
  }
  const producerModule = new SourceTextModule(producerText, {
    identifier: 'compile-custody.mjs',
    context,
    initializeImportMeta,
    importModuleDynamically: rejectDynamicImport,
  });
  await producerModule.link((specifier) => {
    if (specifier === CUSTODY_PUBLICATION_SPECIFIER) return publicationModule;
    if (Object.prototype.hasOwnProperty.call(dependencyModules, specifier)) return dependencyModules[specifier];
    fail('UNKNOWN_IMPORT', `compile-custody.mjs -> ${String(specifier)}`);
  });
  await producerModule.evaluate();
  const producerNamespace = producerModule.namespace;
  if (typeof producerNamespace.compileCustody !== 'function') fail('MISSING_EXPORT', 'compile-custody.mjs#compileCustody');
  if (typeof producerNamespace.createIsolatedCustodyProject !== 'function') fail('MISSING_EXPORT', 'compile-custody.mjs#createIsolatedCustodyProject');
  if (typeof producerNamespace.LowerError !== 'function') fail('INVALID_EXPORT', 'compile-custody.mjs#LowerError must be a function or class');
  return Object.freeze({
    compileCustody: producerNamespace.compileCustody,
    createIsolatedCustodyProject: producerNamespace.createIsolatedCustodyProject,
    LowerError: producerNamespace.LowerError,
    producerNamespace,
    publicationNamespace,
    publication: Object.freeze({
      SUCCESS_ARTIFACTS: publicationNamespace.SUCCESS_ARTIFACTS,
      shouldApply: publicationNamespace.shouldApply,
      shouldPublish: publicationNamespace.shouldPublish,
    }),
  });
}
