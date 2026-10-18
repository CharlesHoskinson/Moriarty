# PREPARED ONLY: Q1 adopted source1 owner input

HOLD. No provider call is authorized by this file. Root must inspect this changed preparation and establish terminal-origin transport; do not invoke the stopped legacy loop or its recursive collector. Sol prepared source custody/instructions only, not the security patch/test/judgment body. That body belongs to designated Opus5.5/4.8 HIGH or the explicitly authorized Gemini4ArgonHigh HIGH outage fallback. Latest supplied Opus failure:11:40:29, rejected rate limit, no observed recovery. Unknown actual model/effort must remain unknown.

## Exact task — verbatim selected increment

Gate blueprint publication/advertised path and parameter application on checkOk && compiled.

This is only source1. Preserve source2/source3 as recorded in TARGET-AND-RETAINED-INCREMENTS.json; do not implement them in this delivery. Do not change guards, Q2–Q6, source/validator/proof semantics, native dispatch or resource/permission state. Original EUC-012/013 and generated production end-to-end remain open.

## Minimal owner output

Return one object matching OWNER-OUTPUT-SCHEMA.json: sourceBeforeSha256, unifiedPatch, isolationTestSource, limits. unifiedPatch is the actual unified diff for this selected source increment. isolationTestSource is complete meaningful isolation-test source authored by the designated owner, with its file path identified in the patch or a source comment. Give explicit limits including unrun status/missing premises; no broad prose, filesRead report, self-approval, acceptance verdict or tool-read attestation. If a necessary premise is missing, empty patch/test strings with exact limits may report nondelivery; do not fabricate source or self-approve. A later root integration and applicable independent review remain separate. Do not execute/import Node/Aiken/SDK/compiler/proof/wallet/ledger or use network, Git, private captures, shared writes or subagents.

Before-source SHA256: 572f4ae5a767cadd47080eceb09ef56e920f507618a9fdacba27695851f41680. Exact source-main association E4D e4d7a2b5709caa18a94f5256d03a0302364b8235 is root-supplied; transport byte-compared the public filesystem copy with the old Q1 before-source and found it identical. No Git command established a new commit assertion. The entire actual producer and existing control excerpts needed for this finite output are inline; do not depend on unavailable file tools. Additional underlying source is physically complete, not claimed read. Record any necessary unread dependency in limits instead of pretending coverage.

The old39,410-byte primary ALREADY contained this whole producer and old a2 ALREADY selected source1. The actual method change is the focused source/control input and required patch/test SOURCE output instead of broad reason/changedPaths/testPredicates narration; no claim that re-embedding alone or smaller size fixes provider delivery. Both420s124/zero-whole originals remain STOPPED/immutable.

## Original EUC-012 context (unchanged excerpt)

### Requirement: EUC-012 Artifact publication and failure isolation
WHEN compilation succeeds, the compiler SHALL publish the generated Aiken, checked blueprint, UPLC bytes, Cardano credential, source-file digest, and dependency manifest from one isolated output directory. IF admission, Aiken check, Aiken build, required tests, or blueprint parsing fails, THEN the compiler SHALL return nonzero and leave no success blueprint. IF artifacts or dependencies drift after verification, THEN consumption SHALL refuse the stale manifest.

#### Scenario: Positive Publish a checked result
- **WHEN** check, build, and required named tests succeed on pinned files
- **THEN** the blueprint references exactly the compiled source manifest

#### Scenario: Negative Refuse stale success
- **WHEN** a prior plutus.json exists and the next build fails
- **THEN** the old blueprint cannot be returned as the new compilation result

## Complete actual producer: 413 lines / 19587 bytes

The next fenced block is the full file, not a selected span. Its complete bytes are also current-source/experiments/moriarty-eutxo/src/compile-custody.mjs.

```javascript
/**
 * Compile the T1 custody validator in an isolated Aiken project.
 * Does not write eutxo-public and does not submit a transaction.
 */
import { createHash } from 'node:crypto';
import { cpSync, existsSync, mkdirSync, readFileSync, rmSync, symlinkSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { deepStrictEqual } from 'node:assert';
import { createFinancialAgreementSourceV5 } from '../../moriarty-language/src/successor/financial-agreement-source-v5.ts';
import { canonicalEncode } from '../../moriarty-language/src/codec.ts';
import { admitCustody, fenceLifecycle } from './admit-custody.mjs';
import { bindOracleSpends } from './custody-oracle-bind.mjs';
import { bindCapacity } from './custody-capacity.mjs';
import { actionTimeGuards, isFixedUtxoProbe, isProductionProbeTime, substituteTimeGuards } from './custody-time.mjs';
import { lowerCustody, LowerError } from './lower-custody.mjs';
import { parseAikenCheckJson, ROOT, runAiken } from './compile-public.mjs';
import { applyUnitParameters, CASH_VALIDATOR, STDLIB_TREE, STDLIB_TREE_SHA256, stdlibTreeSha256 } from './apply-head.mjs';

const DEFAULT_ORACLE = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-independent-oracles-19/RESULTS.json';

const PROJECT = join(ROOT, 'eutxo-custody');
const PACKAGES = STDLIB_TREE;

function refuse(code, detail) {
  return { ok: false, code, detail, ledgerExecution: false, nativePins: false, blueprintPath: null };
}

function pick(cases, id) {
  const found = cases.cases.find((item) => item.id === id);
  if (!found) return null;
  return found;
}

function envelope(spec, inputLife) {
  const ordinary = JSON.parse(spec.ordinaryInputCanonicalJSON);
  return {
    args: ordinary.Args,
    pre: ordinary.Pre,
    inputLife,
  };
}

function postHead(financial, ordinary, transportRevision, transportRemaining) {
  return {
    transportRevision,
    transportRemaining,
    locked: ordinary.locked,
    revision: ordinary.revision,
    life: financial,
  };
}

function withTimeWindow(canonical, lower, upper) {
  const value = JSON.parse(canonical);
  value.Args.lower = lower;
  value.Args.upper = upper;
  return canonicalEncode(value);
}

function applySyntheticTestWindows(source, syntheticTime) {
  const releaseLower = syntheticTime.bounds.releaseLower;
  const partition = syntheticTime.bounds.partition;
  const refundInclusive = syntheticTime.bounds.refundInclusive;
  const releaseUpper = (BigInt(partition) - 1n).toString(10);
  const widerEnd = (BigInt(refundInclusive) + 2n).toString(10);
  const frozenDecode = 'actor == "Lender" && recipient == "Lender" && transfer_id == "Release1" && lower == 1000 && upper == 1099';
  const frozenProgram = 'decoded.program_hash == program_hash';
  const pairs = [
    ['lower: 1000, upper: 1099', `lower: ${releaseLower}, upper: ${releaseUpper}`],
    [', 1000, 1100)', `, ${releaseLower}, ${partition})`],
    ['finite_range(1000, 1100)', `finite_range(${releaseLower}, ${partition})`],
    ['finite_range(1100, 1201)', `finite_range(${partition}, ${widerEnd})`],
    ['Finite(1000)', `Finite(${releaseLower})`],
    ['Original guards stay 1000/1100/1200. This file is not a calibrated sibling and not a node spend.', syntheticTime.productionArtifact === true
      ? 'Measured guard literals are a calibrated sibling. This file is not a node spend.'
      : 'Synthetic guard literals are UNIT/SYNTHETIC. This file is not a production artifact and not a node spend.'],
  ];
  if (!source.includes(frozenDecode) || !source.includes(frozenProgram)) return null;
  let text = source;
  for (const [from, to] of pairs) {
    if (!text.includes(from)) return null;
    text = text.split(from).join(to);
  }
  if (!text.includes(frozenDecode) || !text.includes(frozenProgram) || !text.includes('first_period_start: 1000')) return null;
  return `${text}\ntest sibling_frozen_program_hash_differs() {\n  expect Some(data) = deserialise(e13_pre)\n  expect decoded: HeadDatum = data\n  decoded.program_hash != program_hash && decoded.instance_id != instance_digest(script_hash, cash_policy)\n}\n`;
}

function loadFixtures({ sourceText, seedText, casesPath, expectedReleasePath, expectedRefundPath, residualPath, syntheticTime = null }) {
  const cases = JSON.parse(readFileSync(casesPath, 'utf8'));
  const seed = JSON.parse(seedText);
  const releaseCase = pick(cases, 'E13-release-real-custody');
  const refundCase = pick(cases, 'E11-refund-real-custody');
  const residualCase = pick(cases, 'E12-cancel-preserves-duty');
  if (!releaseCase || !refundCase || !residualCase) return refuse('ORACLE_DIVERGENCE', 'missing designer case');
  const v5 = createFinancialAgreementSourceV5();
  const releaseUpper = syntheticTime ? (BigInt(syntheticTime.bounds.partition) - 1n).toString(10) : null;
  const releaseInput = syntheticTime
    ? withTimeWindow(releaseCase.ordinaryInputCanonicalJSON, syntheticTime.bounds.releaseLower, releaseUpper)
    : releaseCase.ordinaryInputCanonicalJSON;
  const refundInput = syntheticTime
    ? withTimeWindow(refundCase.ordinaryInputCanonicalJSON, syntheticTime.bounds.partition, syntheticTime.bounds.refundInclusive)
    : refundCase.ordinaryInputCanonicalJSON;
  const cancelInput = canonicalEncode({
    Args: {
      actor: 'Borrower',
      lower: syntheticTime ? syntheticTime.bounds.releaseLower : '1050',
      recipient: 'Borrower',
      transferId: 'Cancel1',
      upper: syntheticTime ? releaseUpper : '1051',
    },
    Obs: {},
    Pre: { locked: '1', revision: '0' },
    workInitial: '1024',
  });
  const releaseEval = v5.evaluate(sourceText, 'release', releaseInput, seedText);
  const refundEval = v5.evaluate(sourceText, 'refund', refundInput, seedText);
  const cancelEval = v5.evaluate(sourceText, 'cancel', cancelInput, seedText);
  const residualInput = syntheticTime
    ? withTimeWindow(residualCase.ordinaryInputCanonicalJSON, syntheticTime.bounds.releaseLower, releaseUpper)
    : residualCase.ordinaryInputCanonicalJSON;
  const residualEval = v5.evaluate(sourceText, 'cancel', residualInput, readFileSync(residualPath, 'utf8'));
  const expectedRelease = JSON.parse(readFileSync(expectedReleasePath, 'utf8'));
  const expectedRefund = JSON.parse(readFileSync(expectedRefundPath, 'utf8'));
  try {
    deepStrictEqual(releaseEval.financialPost, expectedRelease.financialPost);
    deepStrictEqual(releaseEval.post, expectedRelease.post);
    deepStrictEqual(releaseEval.effects, expectedRelease.effects);
    deepStrictEqual(releaseEval.status, expectedRelease.status);
    deepStrictEqual(refundEval.financialPost, expectedRefund.financialPost);
    deepStrictEqual(refundEval.post, expectedRefund.post);
    deepStrictEqual(refundEval.effects, expectedRefund.effects);
    deepStrictEqual(cancelEval.financialPost.balances, expectedRefund.financialPost.balances);
    deepStrictEqual(cancelEval.financialPost.obligations, []);
    deepStrictEqual(cancelEval.financialPost.usedTransferIds, ['Cancel1']);
    deepStrictEqual(cancelEval.post, { locked: '0', revision: '1' });
    if (cancelEval.financialPost.work.remaining !== '975' || cancelEval.financialPost.work.spent !== '49') {
      return refuse('ORACLE_DIVERGENCE', 'cancel work');
    }
    if (fenceLifecycle(residualEval.financialPost).code !== 'NONEMPTY_OBLIGATIONS') return refuse('ORACLE_DIVERGENCE', 'E12 fence');
    if (residualEval.financialPost.obligations[0].outstanding !== '40') return refuse('ORACLE_DIVERGENCE', 'E12 outstanding');
  } catch (error) {
    return refuse('ORACLE_DIVERGENCE', error.message);
  }
  const release = { ...envelope({ ordinaryInputCanonicalJSON: releaseInput }, seed), effect: releaseEval.effects[0], life: releaseEval.financialPost, postHead: postHead(releaseEval.financialPost, releaseEval.post, '1', '0') };
  const refund = { ...envelope({ ordinaryInputCanonicalJSON: refundInput }, seed), effect: refundEval.effects[0], life: refundEval.financialPost, postHead: postHead(refundEval.financialPost, refundEval.post, '1', '0') };
  const cancel = {
    args: {
      actor: 'Borrower',
      lower: syntheticTime ? syntheticTime.bounds.releaseLower : '1050',
      recipient: 'Borrower',
      transferId: 'Cancel1',
      upper: syntheticTime ? releaseUpper : '1051',
    },
    pre: { locked: '1', revision: '0' },
    inputLife: seed,
    effect: cancelEval.effects[0],
    life: cancelEval.financialPost,
    postHead: postHead(cancelEval.financialPost, cancelEval.post, '1', '0'),
  };
  return {
    ok: true,
    fixtures: {
      seedHead: { transportRevision: '0', transportRemaining: '1', locked: '1', revision: '0', life: seed },
      release,
      refund,
      cancel,
    },
  };
}

export function createIsolatedCustodyProject() {
  const dir = join(tmpdir(), `moriarty-custody-${createHash('sha256').update(String(Date.now() + Math.random())).digest('hex').slice(0, 12)}`);
  mkdirSync(join(dir, 'validators'), { recursive: true });
  cpSync(join(PROJECT, 'aiken.toml'), join(dir, 'aiken.toml'));
  cpSync(join(PROJECT, 'aiken.lock'), join(dir, 'aiken.lock'));
  mkdirSync(join(dir, 'build'), { recursive: true });
  const pin = stdlibTreeSha256(PACKAGES);
  if (!pin.ok || pin.sha256 !== STDLIB_TREE_SHA256 || pin.files !== 65) throw new Error('STDLIB_PIN_MISMATCH');
  cpSync(PACKAGES, join(dir, 'build/packages'), { recursive: true });
  return dir;
}

function clearArtifacts(outDir) {
  for (const name of ['plutus.json', 'admission.json']) {
    const path = join(outDir, name);
    if (existsSync(path)) rmSync(path);
    if (existsSync(path)) return refuse('STALE_ARTIFACT', name);
  }
  return null;
}

function writeAdmission(outDir, payload, resultOk) {
  const body = { ...payload, ok: resultOk === true, ledgerExecution: false, nativePins: false };
  const path = join(outDir, 'admission.json');
  writeFileSync(path, JSON.stringify(body, null, 2) + '\n');
  const readback = JSON.parse(readFileSync(path, 'utf8'));
  if (readback.ok !== body.ok) return refuse('ADMISSION_READBACK', String(readback.ok));
  return null;
}

export function compileCustody({ sourcePath, seedPath, casesPath, expectedReleasePath, expectedRefundPath, residualPath, manifest, outDir, oracleResultsPath = DEFAULT_ORACLE, aikenRun = runAiken, syntheticTime = null }) {
  mkdirSync(outDir, { recursive: true });
  const stale = clearArtifacts(outDir);
  if (stale) return stale;
  const designerSource = readFileSync(sourcePath, 'utf8');
  let sourceText = designerSource;
  const production = isProductionProbeTime(syntheticTime);
  if (syntheticTime) {
    const unit = syntheticTime.evidenceClass === 'UNIT/SYNTHETIC' && syntheticTime.productionArtifact === false;
    if (!production && !unit) {
      return refuse('UNIT_PARITY_REFUSED', 'synthetic time is not production');
    }
    if (production) {
      if (typeof syntheticTime.genesisSha256 !== 'string' || !/^[0-9a-f]{64}$/.test(syntheticTime.genesisSha256)) {
        return refuse('GENESIS_BINDING_MISMATCH', 'production genesis');
      }
      if (isFixedUtxoProbe(syntheticTime.queriedSeed)) {
        return refuse('FIXED_UTXO_PROBE', 'production seed');
      }
    }
    const substituted = substituteTimeGuards(designerSource, syntheticTime.bounds, { production });
    if (!substituted.ok) return refuse(substituted.code, 'guard substitution');
    sourceText = substituted.text;
    writeFileSync(join(outDir, 'sibling.mori'), sourceText);
    writeFileSync(join(outDir, 'SIBLING.json'), JSON.stringify({
      evidenceClass: substituted.evidenceClass,
      productionArtifact: substituted.productionArtifact === true,
      nativeProbe: false,
      ledgerExecution: false,
      nativePins: false,
      sourcePathUntouched: sourcePath,
      genesisSha256: production ? syntheticTime.genesisSha256 : null,
    }, null, 2) + '\n');
  }
  const seedText = readFileSync(seedPath, 'utf8');
  const admitManifestBody = production
    ? {
      ...manifest,
      production: true,
      unitFixture: false,
      evidenceClass: 'node-genesis-v3-probe',
      probeExecuted: true,
      genesisSha256: syntheticTime.genesisSha256,
      queriedSeed: syntheticTime.queriedSeed,
      productionAdmission: syntheticTime.productionAdmission,
    }
    : manifest;
  const admitted = admitCustody({ sourceText, seedText, manifest: admitManifestBody });
  if (!admitted.ok) {
    const drift = writeAdmission(outDir, admitted, false);
    if (drift) return drift;
    return { ...admitted, ok: false, blueprintPath: null };
  }
  const loaded = loadFixtures({ sourceText, seedText, casesPath, expectedReleasePath, expectedRefundPath, residualPath, syntheticTime });
  if (!loaded.ok) {
    writeFileSync(join(outDir, 'oracle.json'), JSON.stringify(loaded, null, 2) + '\n');
    const drift = writeAdmission(outDir, loaded, false);
    if (drift) return drift;
    return { ...loaded, ok: false, blueprintPath: null };
  }
  const capacity = bindCapacity({ sourceText, seed: JSON.parse(seedText), manifest, timeBinding: syntheticTime });
  if (!capacity.ok) {
    const drift = writeAdmission(outDir, capacity, false);
    if (drift) return drift;
    return { ...capacity, blueprintPath: null };
  }
  loaded.fixtures.capacity = capacity;
  let lowered;
  try {
    lowered = lowerCustody({ admitted, fixtures: loaded.fixtures, syntheticTime });
    if (syntheticTime) {
      const rewritten = applySyntheticTestWindows(lowered.validator, syntheticTime);
      if (!rewritten) {
        const failure = refuse('GUARD_SHAPE', 'synthetic test windows were not found');
        writeFileSync(join(outDir, 'lower.json'), JSON.stringify(failure, null, 2) + '\n');
        return failure;
      }
      if (rewritten.includes('first_period_start: 1000') === false) {
        return refuse('GUARD_SHAPE', 'non-time literal was removed');
      }
      lowered = { ...lowered, validator: rewritten };
    }
  } catch (error) {
    const failure = refuse(error.code || 'EXPRESSION_UNSUPPORTED', error.message);
    writeFileSync(join(outDir, 'lower.json'), JSON.stringify(failure, null, 2) + '\n');
    const drift = writeAdmission(outDir, failure, false);
    if (drift) return drift;
    return failure;
  }
  if (syntheticTime) {
    const releaseGuard = actionTimeGuards(sourceText, 'release');
    const refundGuard = actionTimeGuards(sourceText, 'refund');
    const cancelGuard = actionTimeGuards(sourceText, 'cancel');
    const bounds = syntheticTime.bounds || {};
    const aligned = releaseGuard.ok && refundGuard.ok && cancelGuard.ok
      && releaseGuard.lower === bounds.releaseLower
      && releaseGuard.exclusiveUpper === bounds.partition
      && refundGuard.lower === bounds.partition
      && refundGuard.upper === bounds.refundInclusive
      && cancelGuard.lower === releaseGuard.lower
      && cancelGuard.exclusiveUpper === releaseGuard.exclusiveUpper;
    if (!aligned) {
      const failure = refuse('ORACLE_DIVERGENCE', 'time binding does not match the substituted action guards');
      const drift = writeAdmission(outDir, failure, false);
      if (drift) return drift;
      return failure;
    }
  }
  let bound;
  try {
    bound = bindOracleSpends(JSON.parse(readFileSync(oracleResultsPath, 'utf8')), syntheticTime);
  } catch (error) {
    const failure = refuse(error.code || 'ORACLE_DIVERGENCE', error.message);
    const drift = writeAdmission(outDir, failure, false);
    if (drift) return drift;
    return failure;
  }
  const validator = `${lowered.validator}\n${bound.source}\n`;
  const expectedTests = [...lowered.expectedTests, ...bound.expectedTests];
  if (syntheticTime) expectedTests.push({ title: 'sibling_frozen_program_hash_differs', status: 'success' });
  const validatorPath = join(outDir, 'custody.ak');
  writeFileSync(validatorPath, validator);
  const isolated = createIsolatedCustodyProject();
  const published = join(outDir, 'plutus.json');
  try {
    writeFileSync(join(isolated, 'validators/financial_head.ak'), validator);
    writeFileSync(join(isolated, 'validators/cash.ak'), CASH_VALIDATOR);
    const check = aikenRun(['check'], isolated);
    writeFileSync(join(outDir, 'aiken-check.stdout'), check.stdout || '');
    writeFileSync(join(outDir, 'aiken-check.stderr'), check.stderr || '');
    const parsed = parseAikenCheckJson(check.stdout, check.stderr);
    const missing = [];
    for (const exp of expectedTests) {
      const found = (parsed.tests || []).find((item) => item.title === exp.title || item.title.endsWith(exp.title));
      if (!found) missing.push(exp.title);
      else if (found.status !== 'success') missing.push(`${exp.title}:${found.status}`);
    }
    const checkOk = check.status === 0 && parsed.ok && missing.length === 0;
    const build = aikenRun(['build', '--uplc'], isolated);
    writeFileSync(join(outDir, 'aiken-build.stdout'), build.stdout || '');
    writeFileSync(join(outDir, 'aiken-build.stderr'), build.stderr || '');
    const blueprintSrc = join(isolated, 'plutus.json');
    const compiled = build.status === 0 && existsSync(blueprintSrc);
    let applied = null;
    if (compiled) {
      cpSync(blueprintSrc, published);
      const queried = production ? syntheticTime.queriedSeed : null;
      applied = applyUnitParameters({
        blueprintPath: published,
        isolatedDir: isolated,
        hashes: admitted.hashes,
        manifest: admitted.manifest,
        seedTxId: queried?.txId || '88'.repeat(32),
        seedIndex: Number.isInteger(queried?.index) ? queried.index : 0,
        persistDir: outDir,
      });
    }
    const ok = checkOk && compiled && applied?.ok === true;
    const checkStatus = check.status === 0 && !checkOk ? 1 : check.status;
    const admission = {
      code: ok ? undefined : (!checkOk ? 'AIKEN_CHECK' : (applied && applied.ok === false ? applied.code : 'AIKEN_BUILD')),
      layer: 'source-uplc-unit',
      hashes: admitted.hashes,
      instanceId: admitted.instanceId,
      applied: applied && applied.ok ? {
        appliedHash: applied.appliedHash,
        cashHash: applied.cashHash,
        cashCanMint: false,
        genesis: applied.genesis,
        instanceId: applied.instanceId,
        literals: applied.literals,
        nativePins: false,
      } : undefined,
      applyDetail: applied && applied.ok === false ? applied.detail : undefined,
      actions: admitted.actions,
      missingTests: missing,
      checkStatus,
      buildStatus: build.status,
      oracleBound: bound.bound.length,
      oracleSkipped: bound.skipped,
    };
    const drift = writeAdmission(outDir, admission, ok);
    if (drift) return drift;
    return {
      ok,
      code: ok ? undefined : (!checkOk ? 'AIKEN_CHECK' : (applied && applied.ok === false ? applied.code : 'AIKEN_BUILD')),
      detail: checkOk ? null : missing.join(','),
      ledgerExecution: false,
      nativePins: false,
      productionArtifact: production === true,
      nativeProbe: false,
      layer: production ? 'calibrated-sibling-offline' : 'source-uplc-unit',
      hashes: admitted.hashes,
      instanceId: admitted.instanceId,
      applied,
      validatorPath,
      missingTests: missing,
      checkStatus,
      buildStatus: build.status,
      blueprintPath: compiled ? published : null,
      check: { status: checkStatus, stdout: check.stdout, stderr: check.stderr, tests: parsed.tests, missing },
      build: { status: build.status, stdout: build.stdout, stderr: build.stderr },
      oracleBound: bound.bound,
      oracleSkipped: bound.skipped,
      outDir,
    };
  } finally {
    rmSync(isolated, { recursive: true, force: true });
  }
}

export { LowerError };
```

## Existing source/control anchor public-cli

experiments/moriarty-eutxo/src/cli.mjs:299–331; full file SHA256 5f88ec51a97ea835d85ed03c62153ff6dc221d460bab6b58f2cb34c84abb55c0. Existing compile-custody CLI invokes this producer and displays result.blueprintPath.

```javascript
if (command === 'compile-custody') {
  const task = '/home/charl/moriarty-eutxo-validation-2026-10-12';
  const designer = join(task, 'designers/02-escrow-time');
  const manifestPath = arg('--manifest', join(ROOT, 'examples/custody-unit.manifest.json'));
  const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
  const source = arg('--source', join(designer, 'escrow-custody-v5.mori'));
  const seed = arg('--seed', join(designer, 'financial-seed-custody-v5.json'));
  const out = takeOut(join(task, 't1-native-adapters-57/checks/cli-custody'));
  const result = compileCustody({
    sourcePath: source,
    seedPath: seed,
    casesPath: arg('--cases', join(designer, 'cases.json')),
    expectedReleasePath: arg('--expected-release', join(designer, 'E13-release-real-custody.expected.json')),
    expectedRefundPath: arg('--expected-refund', join(designer, 'E11-refund-real-custody.expected.json')),
    residualPath: arg('--residual', join(designer, 'financial-residual-custody-v5.json')),
    manifest,
    outDir: out,
  });
  process.stdout.write(JSON.stringify({
    ok: result.ok,
    code: result.code,
    ledgerExecution: false,
    nativePins: false,
    layer: result.layer || 'source-uplc-unit',
    hashes: result.hashes,
    instanceId: result.instanceId,
    checkStatus: result.check?.status,
    buildStatus: result.build?.status,
    blueprint: result.blueprintPath,
    validator: result.validatorPath,
    missing: result.check?.missing,
  }, null, 2) + '\n');
  process.exit(result.ok ? 0 : 1);
```

## Existing source/control anchor sibling-caller

experiments/moriarty-eutxo/src/native-caller.mjs:266–318; full file SHA256 3c621003f9d4c5e00e9b5efafd21ee058aebb2d8886569b199670561b1c9f469. Existing production sibling caller invokes this producer, reads result.ok and returns blueprint identity/path fields.

```javascript
    const compiled = compileCustody({
      sourcePath,
      seedPath: join(DESIGNER, 'financial-seed-custody-v5.json'),
      casesPath: join(DESIGNER, 'cases.json'),
      expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
      expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
      residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
      manifest,
      outDir,
      syntheticTime: {
        evidenceClass: 'node-genesis-v3-probe',
        productionArtifact: true,
        unitFixture: false,
        probeExecuted: true,
        genesisSha256: production.genesisSha256,
        queriedSeed: seed,
        bounds,
        productionAdmission: production,
      },
    });
    const programHash = compiled.hashes?.programHash || null;
    if (!compiled.ok || !programHash || programHash === previousProgramHash || compiled.hashes?.sourceHash === sourceHash) {
      return closed(compiled.code || 'SIBLING_COMPILE_REJECTED', {
        siblingEmitted: false,
        productionAdmitted: false,
        calibrated: false,
        nativeProbe: false,
        detail: compiled.detail || null,
      });
    }
    return {
      ok: true,
      code: 'SIBLING_COMPILED',
      siblingEmitted: true,
      calibrated: true,
      productionAdmitted: true,
      productionArtifact: true,
      nativeProbe: true,
      spawned: true,
      compilerInvoked: true,
      cacheHit: false,
      programHash,
      boundsHash: compiled.hashes.boundsHash,
      sourceHash: compiled.hashes.sourceHash,
      schemaHash: compiled.hashes.schemaHash,
      seedHash: compiled.hashes.seedHash,
      blueprintSha256: compiled.blueprintPath ? fileHash(compiled.blueprintPath) : null,
      blueprintPath: compiled.blueprintPath,
      sourcePath: join(outDir, 'sibling.mori'),
      applied: compiled.applied,
      productionAdmission: production,
      zeroTime: production.zeroTime,
      zeroSlot: production.zeroSlot,
```

## Existing source/control anchor source-cache-caller

experiments/moriarty-eutxo/src/native-caller.mjs:1298–1326; full file SHA256 3c621003f9d4c5e00e9b5efafd21ee058aebb2d8886569b199670561b1c9f469. Existing source compiler caller checks result.ok/path then copies blueprint into source cache.

```javascript
  const { compileCustody } = await import('./compile-custody.mjs');
  const compileRoot = evidenceRoot || REPAIR_ROOT;
  const outDir = join(compileRoot, 'checks/source-compile', sha256);
  const compiled = compileCustody({
    sourcePath,
    seedPath,
    casesPath: join(DESIGNER, 'cases.json'),
    expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
    expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
    residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
    manifest,
    outDir,
  });
  if (!compiled.ok || !compiled.blueprintPath || !existsSync(compiled.blueprintPath)) {
    return closed(compiled.code || 'SOURCE_COMPILE_FAILED', {
      compiled: false,
      sha256,
      admitted: true,
      path: sourcePath,
      checkStatus: compiled.check?.status ?? null,
      buildStatus: compiled.build?.status ?? null,
      missing: compiled.check?.missing || compiled.missingTests || null,
    });
  }
  const cacheDir = evidenceRoot ? join(evidenceRoot, 'checks/source-cache') : SOURCE_CACHE;
  mkdirSync(cacheDir, { recursive: true });
  const blueprintPath = join(cacheDir, `${sha256}.plutus.json`);
  cpSync(compiled.blueprintPath, blueprintPath);
  const blueprintSha256 = fileHash(blueprintPath);
```

## Existing source/control anchor existing-check-status-control

experiments/moriarty-eutxo/tests/custody.test.mjs:549–577; full file SHA256 48e7400eff7ea431816a16793499e5947c50fe01e3991d1f9e6a4bd0f683f3c9. Existing unrun test control invokes compileCustody with an aikenRun callback. This excerpt is supplied unchanged; no test adequacy or new security expectation is supplied by Sol.

```javascript
test('aiken status 7 does not publish admission ok true or a stale blueprint', () => {
  const outDir = join(CHECKS, 'aiken-exit-7');
  rmSync(outDir, { recursive: true, force: true });
  mkdirSync(outDir, { recursive: true });
  writeFileSync(join(outDir, 'plutus.json'), 'STALE\n');
  const result = compileCustody({
    sourcePath: SOURCE,
    seedPath: SEED,
    casesPath: CASES,
    expectedReleasePath: E13,
    expectedRefundPath: E11,
    residualPath: RESIDUAL,
    manifest: unitManifest(),
    outDir,
    aikenRun() {
      return { status: 7, stdout: '', stderr: 'forced-exit-7' };
    },
  });
  assert.equal(result.ok, false);
  assert.equal(result.code, 'AIKEN_CHECK');
  assert.equal(result.check.status, 7);
  assert.equal(result.blueprintPath, null);
  assert.equal(existsSync(join(outDir, 'plutus.json')), false);
  const admission = JSON.parse(readFileSync(join(outDir, 'admission.json'), 'utf8'));
  assert.equal(admission.ok, false);
  assert.equal(admission.ok, result.ok);
  assert.equal(admission.checkStatus, 7);
});

```
