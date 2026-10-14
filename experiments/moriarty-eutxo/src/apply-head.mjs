/**
 * Compile Cash first, then apply definite canonical parameters to financial_head.
 * Hash agreement is Aiken tag 0x03, Lucid mintingPolicyToId, and both cardano-cli commands.
 * Unit parameters are not RuntimePins. The fail-closed Cash validator cannot mint.
 */
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { cpSync, existsSync, lstatSync, mkdtempSync, mkdirSync, readFileSync, readdirSync, realpathSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { blake2b } from '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/node_modules/@noble/hashes/blake2b.js';
import { Data, Constr } from '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/node_modules/@lucid-evolution/plutus/dist/index.js';
import { mintingPolicyToId } from '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/node_modules/@lucid-evolution/utils/dist/index.js';
import { definiteDataHex, genesisDigest, instanceDigest } from './same-script-identity.mjs';

export const AIKEN = '/home/charl/.aiken/bin/aiken';
export const CARDANO_CLI = '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/cardano-node/bin/cardano-cli';

export const CASH_VALIDATOR = `use cardano/transaction.{Transaction}

/// Cash is compiled first. It takes no parameter from financial_head.
/// Reviewed measurement text: every purpose fails. This policy cannot mint.
validator cash {
  mint(_redeemer: Data, _policy_id: ByteArray, _self: Transaction) {
    fail
  }
  else(_) {
    fail
  }
}
`;

function refuse(code, detail) {
  return { ok: false, code, detail, ledgerExecution: false, nativePins: false, runtimePinsCreated: false };
}

function tag03(compiledHex) {
  return Buffer.from(blake2b(Buffer.concat([Buffer.from([0x03]), Buffer.from(compiledHex, 'hex')]), { dkLen: 28 })).toString('hex');
}

export function scriptHashV3(compiledHex) {
  return tag03(compiledHex);
}

function run(cmd, args) {
  return spawnSync(cmd, args, { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
}

function cliHashes(compiledHex, label) {
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-script-hash-'));
  try {
    const file = join(dir, `${label}.script.json`);
    writeFileSync(file, JSON.stringify({ type: 'PlutusScriptV3', description: label, cborHex: compiledHex }));
    const hash = run(CARDANO_CLI, ['hash', 'script', '--script-file', file]);
    const policy = run(CARDANO_CLI, ['conway', 'transaction', 'policyid', '--script-file', file]);
    return {
      hashStatus: hash.status,
      policyStatus: policy.status,
      hashScript: String(hash.stdout || '').trim(),
      policyId: String(policy.stdout || '').trim(),
      stderr: `${hash.stderr || ''}${policy.stderr || ''}`.trim(),
    };
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
}

function agree(compiledHex, aikenHash, label) {
  const local = tag03(compiledHex);
  const lucid = mintingPolicyToId({ type: 'PlutusV3', script: compiledHex });
  const cli = cliHashes(compiledHex, label);
  const ok = local === aikenHash && lucid === aikenHash && cli.hashStatus === 0 && cli.policyStatus === 0
    && cli.hashScript === aikenHash && cli.policyId === aikenHash;
  return { ok, local, lucid, cli, aikenHash };
}

function findValidator(blueprint, purposeSuffix, namePart) {
  const found = blueprint.validators?.find((item) => item.title.endsWith(`.${purposeSuffix}`) && item.title.includes(namePart));
  return found || null;
}

function containsRaw(hayHex, needleHex) {
  if (!needleHex || needleHex.length % 2 !== 0) return false;
  return hayHex.toLowerCase().includes(needleHex.toLowerCase());
}

export function applyUnitParameters({ blueprintPath, isolatedDir, hashes, manifest, seedTxId, seedIndex, aiken = AIKEN, persistDir = null }) {
  const blueprint = JSON.parse(readFileSync(blueprintPath, 'utf8'));
  const cash = findValidator(blueprint, 'mint', 'cash');
  const head = findValidator(blueprint, 'mint', 'financial_head');
  const spend = findValidator(blueprint, 'spend', 'financial_head');
  const other = findValidator(blueprint, 'else', 'financial_head');
  if (!cash || !head || !spend || !other) return refuse('AIKEN_CHECK', 'blueprint titles');
  if (head.compiledCode !== spend.compiledCode || head.compiledCode !== other.compiledCode) {
    return refuse('AIKEN_CHECK', 'unapplied purpose bytes differ');
  }
  const cashAgreement = agree(cash.compiledCode, cash.hash, 'cash-unapplied');
  if (!cashAgreement.ok) return refuse('SCRIPT_HASH_DIVERGENCE', `cash ${JSON.stringify(cashAgreement)}`);
  const unappliedAgreement = agree(head.compiledCode, head.hash, 'head-unapplied');
  if (!unappliedAgreement.ok) return refuse('SCRIPT_HASH_DIVERGENCE', `unapplied ${JSON.stringify(unappliedAgreement)}`);
  if (!/^[0-9a-f]{56}$/.test(String(cash.hash || ''))) return refuse('SCRIPT_HASH_DIVERGENCE', 'compiled cash hash');
  const appliedCash = cash.hash;
  const manifestMatchesCompiledCash = manifest.cashPolicyId === appliedCash;
  const genesis = genesisDigest({
    hashes,
    cashPolicyId: appliedCash,
    lender: manifest.lenderVkh,
    borrower: manifest.borrowerVkh,
    seedTxId,
    seedIndex,
  });
  if (!genesis.ok) return genesis;
  const seedLocal = definiteDataHex('output-reference', { transactionId: seedTxId, index: seedIndex });
  const cashLocal = definiteDataHex('bytes', appliedCash);
  const lenderLocal = definiteDataHex('bytes', manifest.lenderVkh);
  const borrowerLocal = definiteDataHex('bytes', manifest.borrowerVkh);
  const genesisLocal = definiteDataHex('bytes', genesis.genesis);
  if (!seedLocal.ok || !cashLocal.ok || !lenderLocal.ok || !borrowerLocal.ok || !genesisLocal.ok) {
    return refuse('NON_CANONICAL_VALUE', 'definite data');
  }
  const seedLucid = Data.to(new Constr(0, [seedTxId, BigInt(seedIndex)]), undefined, { canonical: true });
  const cashLucid = Data.to(appliedCash, undefined, { canonical: true });
  const lenderLucid = Data.to(manifest.lenderVkh, undefined, { canonical: true });
  const borrowerLucid = Data.to(manifest.borrowerVkh, undefined, { canonical: true });
  const genesisLucid = Data.to(genesis.genesis, undefined, { canonical: true });
  if (seedLucid !== seedLocal.hex || cashLucid !== cashLocal.hex || lenderLucid !== lenderLocal.hex || borrowerLucid !== borrowerLocal.hex || genesisLucid !== genesisLocal.hex) {
    return refuse('ENCODER_DISAGREEMENT', 'lucid definite data differs from the local encoder');
  }
  const params = [
    ['seed', seedLocal.hex],
    ['cash', cashLocal.hex],
    ['lender', lenderLocal.hex],
    ['borrower', borrowerLocal.hex],
    ['genesis', genesisLocal.hex],
  ];
  let input = blueprintPath;
  const steps = [];
  for (let i = 0; i < params.length; i += 1) {
    const out = join(isolatedDir, `apply-${i}.json`);
    const result = run(aiken, ['blueprint', 'apply', '-i', input, '-o', out, '-m', 'financial_head', '-v', 'financial_head', params[i][1]]);
    steps.push({ name: params[i][0], status: result.status });
    if (result.status !== 0) return refuse('APPLY_FAILED', `${params[i][0]} ${result.stderr || result.stdout}`);
    input = out;
  }
  const appliedBp = JSON.parse(readFileSync(input, 'utf8'));
  const appliedMint = findValidator(appliedBp, 'mint', 'financial_head');
  const appliedSpend = findValidator(appliedBp, 'spend', 'financial_head');
  const appliedElse = findValidator(appliedBp, 'else', 'financial_head');
  if (!appliedMint || appliedMint.compiledCode !== appliedSpend?.compiledCode || appliedMint.compiledCode !== appliedElse?.compiledCode) {
    return refuse('AIKEN_CHECK', 'applied purpose bytes differ');
  }
  const appliedAgreement = agree(appliedMint.compiledCode, appliedMint.hash, 'head-applied');
  if (!appliedAgreement.ok) return refuse('SCRIPT_HASH_DIVERGENCE', `applied ${JSON.stringify(appliedAgreement)}`);
  const instance = instanceDigest(appliedMint.hash, appliedCash);
  if (!instance.ok) return instance;
  const code = appliedMint.compiledCode.toLowerCase();
  const present = {
    ownHash: containsRaw(code, appliedMint.hash),
    instanceDigest: containsRaw(code, instance.instanceId),
    cashParameter: containsRaw(code, appliedCash),
    unitScriptHash: containsRaw(code, manifest.scriptHash),
    unitManifestCash: !manifestMatchesCompiledCash && containsRaw(code, manifest.cashPolicyId),
  };
  if (present.ownHash || present.instanceDigest || present.unitScriptHash) {
    return refuse('IDENTITY_LITERAL', JSON.stringify(present));
  }
  const digest = createHash('sha256').update(appliedMint.compiledCode, 'utf8').digest('hex');
  const report = {
    ok: true,
    ledgerExecution: false,
    nativePins: false,
    runtimePinsCreated: false,
    cashFailClosed: true,
    cashCanMint: false,
    cashHash: cash.hash,
    appliedCashPolicyId: appliedCash,
    manifestCashPolicyId: manifest.cashPolicyId,
    manifestMatchesCompiledCash,
    cashBinding: 'same-run-compiled-script-hash',
    nativePinsEligible: false,
    genesisInputs: {
      hashes,
      cashPolicyId: appliedCash,
      lender: manifest.lenderVkh,
      borrower: manifest.borrowerVkh,
      seedTxId,
      seedIndex,
    },
    cashBytes: cash.compiledCode.length / 2,
    unappliedHash: head.hash,
    appliedHash: appliedMint.hash,
    appliedBytes: appliedMint.compiledCode.length / 2,
    appliedCodeSha256: digest,
    genesis: genesis.genesis,
    genesisBytes: genesis.preimage.length,
    instanceId: instance.instanceId,
    instanceBytes: instance.preimage.length,
    agreements: {
      cash: cashAgreement.aikenHash,
      unapplied: unappliedAgreement.aikenHash,
      applied: appliedAgreement.aikenHash,
    },
    literals: present,
    applySteps: steps,
    unitSeedNotRuntimePin: true,
    cashPolicyCode: 'CASH_POLICY_FAIL_CLOSED',
  };
  if (persistDir) {
    writeFileSync(join(persistDir, 'plutus.applied.json'), JSON.stringify(appliedBp));
    writeFileSync(join(persistDir, 'cash.script.json'), JSON.stringify({
      type: 'PlutusScriptV3',
      description: 'fail-closed cash measurement validator',
      cborHex: cash.compiledCode,
    }));
    writeFileSync(join(persistDir, 'apply-report.json'), JSON.stringify(report, null, 2) + '\n');
  }
  return report;
}

export const CASH_SOURCE_SHA256 = '3f897ac9ef56c01e11d191dba31736da508dc77982ca484f1684e66013b242c4';
export const FAIL_CLOSED_CASH = '332f90fe4531f72bc89151eb530cdeddbd90c8bb7c1aa077ee072ed9';
export const UNAPPLIED_HEAD = 'a34f6bccd249d1524b4b72129b8d10babeb4e86145d44f27651798ca';
export const STDLIB_TREE_SHA256 = '0e3ef3e902f2fb61cd53eaba8b4bc9ef9b746d5487569e8a7bab1fffc761bd88';
export const STDLIB_TREE = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-cash-repair-53/discriminator/head-open/build/packages';
const CASH_PROJECT_TOML = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-cash-repair-53/prototype/aiken.toml';
const CASH_PROJECT_LOCK = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-cash-repair-53/prototype/aiken.lock';

export function stdlibTreeSha256(root) {
  const files = [];
  const visit = (dir) => {
    for (const name of readdirSync(dir)) {
      const path = join(dir, name);
      const st = lstatSync(path);
      if (st.isSymbolicLink()) return { ok: false, code: 'STDLIB_SYMLINK' };
      if (st.isDirectory()) {
        const nested = visit(path);
        if (nested && nested.ok === false) return nested;
      } else if (st.isFile()) files.push(path);
    }
    return null;
  };
  const bad = visit(root);
  if (bad) return bad;
  const lines = files.map((path) => {
    const rel = path.slice(root.length + 1).split('\\').join('/');
    return `${createHash('sha256').update(readFileSync(path)).digest('hex')} ${rel}`;
  });
  lines.sort();
  return {
    ok: true,
    sha256: createHash('sha256').update(`${lines.join('\n')}\n`).digest('hex'),
    files: lines.length,
  };
}

export function buildApprovedCashPolicy({ sourcePath, seedTxId, seedIndex, workDir, aiken = AIKEN } = {}) {
  if (!sourcePath || !existsSync(sourcePath) || !workDir) return refuse('CASH_SOURCE_ABSENT', 'cash fixture path');
  const sourceBytes = readFileSync(sourcePath);
  const sourceHash = createHash('sha256').update(sourceBytes).digest('hex');
  if (sourceHash !== CASH_SOURCE_SHA256) return refuse('CASH_DESIGN_SOURCE_MISMATCH', sourceHash);
  if (!/^[0-9a-f]{64}$/.test(String(seedTxId || '')) || !/^\d+$/.test(String(seedIndex))) {
    return refuse('CASH_SEED_ABSENT', 'seed outref');
  }
  const pin = stdlibTreeSha256(STDLIB_TREE);
  if (!pin.ok || pin.sha256 !== STDLIB_TREE_SHA256 || pin.files !== 65) {
    return refuse('STDLIB_PIN_MISMATCH', JSON.stringify(pin));
  }
  const identity = run(aiken, ['--version']);
  let realAiken = aiken;
  try { realAiken = realpathSync(aiken); } catch { return refuse('AIKEN_IDENTITY', 'realpath'); }
  if (identity.status !== 0 || !String(identity.stdout || '').includes('aiken v1.1.21')) {
    return refuse('AIKEN_IDENTITY', identity.stderr || identity.stdout);
  }
  mkdirSync(workDir, { recursive: true });
  const cachePath = join(workDir, 'cash-policy.json');
  if (existsSync(cachePath)) {
    const cached = JSON.parse(readFileSync(cachePath, 'utf8'));
    if (cached.sourceHash === sourceHash && cached.seedTxId === seedTxId && String(cached.seedIndex) === String(seedIndex)
      && cached.aikenRealpath === realAiken && cached.stdlibSha256 === STDLIB_TREE_SHA256
      && cached.policyId && existsSync(cached.scriptPath)) {
      return { ...cached, ok: true, cacheHit: true, compilerInvoked: false, ledgerExecution: false, nativePins: false, cashCanMint: false };
    }
  }
  const project = join(workDir, 'project');
  rmSync(project, { recursive: true, force: true });
  mkdirSync(join(project, 'validators'), { recursive: true });
  cpSync(CASH_PROJECT_TOML, join(project, 'aiken.toml'));
  cpSync(CASH_PROJECT_LOCK, join(project, 'aiken.lock'));
  writeFileSync(join(project, 'validators/cash_fixture.ak'), sourceBytes);
  cpSync(STDLIB_TREE, join(project, 'build/packages'), { recursive: true });
  const check = spawnSync(aiken, ['check'], { cwd: project, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
  const build = spawnSync(aiken, ['build'], { cwd: project, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024 });
  const blueprintPath = join(project, 'plutus.json');
  if (check.status !== 0 || build.status !== 0 || !existsSync(blueprintPath)) {
    return refuse('CASH_COMPILE_FAILED', `check ${check.status} build ${build.status}`);
  }
  const blueprint = JSON.parse(readFileSync(blueprintPath, 'utf8'));
  const cash = findValidator(blueprint, 'mint', 'cash_fixture') || findValidator(blueprint, 'mint', 'cash');
  if (!cash?.compiledCode || !cash.hash) return refuse('CASH_COMPILE_FAILED', 'cash_fixture mint missing');
  const unapplied = agree(cash.compiledCode, cash.hash, 'cash-fixture-unapplied');
  if (!unapplied.ok) return refuse('SCRIPT_HASH_DIVERGENCE', 'unapplied cash fixture');
  const seedLocal = definiteDataHex('output-reference', { transactionId: seedTxId, index: Number(seedIndex) });
  if (!seedLocal.ok) return seedLocal;
  const appliedPath = join(workDir, 'cash.applied.json');
  const appliedRun = spawnSync(aiken, ['blueprint', 'apply', '-i', blueprintPath, '-o', appliedPath, '-m', 'cash_fixture', '-v', 'cash_fixture', seedLocal.hex], {
    cwd: project, encoding: 'utf8', maxBuffer: 64 * 1024 * 1024,
  });
  if (appliedRun.status !== 0 || !existsSync(appliedPath)) return refuse('APPLY_FAILED', appliedRun.stderr || appliedRun.stdout);
  const appliedBp = JSON.parse(readFileSync(appliedPath, 'utf8'));
  const applied = findValidator(appliedBp, 'mint', 'cash_fixture') || findValidator(appliedBp, 'mint', 'cash');
  if (!applied?.compiledCode) return refuse('APPLY_FAILED', 'applied cash mint missing');
  const agreement = agree(applied.compiledCode, applied.hash, 'cash-fixture-applied');
  if (!agreement.ok) return refuse('SCRIPT_HASH_DIVERGENCE', 'applied cash fixture');
  if (applied.hash === FAIL_CLOSED_CASH || applied.hash === 'aa'.repeat(28)) {
    return refuse('SYNTHETIC_POLICY_REJECTED', applied.hash);
  }
  const scriptPath = join(workDir, 'cash.applied.script.json');
  writeFileSync(scriptPath, JSON.stringify({ type: 'PlutusScriptV3', description: 'approved cash fixture applied to the cash seed', cborHex: applied.compiledCode }));
  const record = {
    ok: true,
    code: 'CASH_POLICY_COMPILED',
    cacheHit: false,
    compilerInvoked: true,
    sourceHash,
    seedTxId,
    seedIndex: Number(seedIndex),
    aikenRealpath: realAiken,
    aikenVersion: String(identity.stdout || '').trim(),
    stdlibSha256: STDLIB_TREE_SHA256,
    stdlibFiles: 65,
    checkStatus: check.status,
    buildStatus: build.status,
    unappliedHash: cash.hash,
    policyId: applied.hash,
    scriptPath,
    blueprintSha256: createHash('sha256').update(readFileSync(blueprintPath)).digest('hex'),
    ledgerExecution: false,
    nativePins: false,
    cashCanMint: false,
    ledgerFunding: false,
  };
  writeFileSync(cachePath, `${JSON.stringify(record, null, 2)}\n`);
  return record;
}

export function applyHeadToCashPolicy({ blueprintPath, cashPolicyId, hashes, manifest, seedTxId, seedIndex, workDir, aiken = AIKEN } = {}) {
  if (!blueprintPath || !existsSync(blueprintPath)) return refuse('HEAD_BLUEPRINT_ABSENT', blueprintPath || '');
  if (!/^[0-9a-f]{56}$/.test(String(cashPolicyId || '')) || cashPolicyId === FAIL_CLOSED_CASH) {
    return refuse('CASH_POLICY_NOT_COMPILED', cashPolicyId || '');
  }
  const blueprint = JSON.parse(readFileSync(blueprintPath, 'utf8'));
  const embedded = findValidator(blueprint, 'mint', 'cash');
  const head = findValidator(blueprint, 'mint', 'financial_head');
  if (!embedded || !head) return refuse('AIKEN_CHECK', 'head blueprint titles');
  if (embedded.hash !== FAIL_CLOSED_CASH) return refuse('FAIL_CLOSED_CASH_UNEXPECTED', embedded.hash);
  if (head.hash !== UNAPPLIED_HEAD) return refuse('UNAPPLIED_HEAD_UNEXPECTED', head.hash);
  const genesis = genesisDigest({
    hashes,
    cashPolicyId,
    lender: manifest.lenderVkh,
    borrower: manifest.borrowerVkh,
    seedTxId,
    seedIndex,
  });
  if (!genesis.ok) return genesis;
  const seedLocal = definiteDataHex('output-reference', { transactionId: seedTxId, index: seedIndex });
  const cashLocal = definiteDataHex('bytes', cashPolicyId);
  const lenderLocal = definiteDataHex('bytes', manifest.lenderVkh);
  const borrowerLocal = definiteDataHex('bytes', manifest.borrowerVkh);
  const genesisLocal = definiteDataHex('bytes', genesis.genesis);
  if (!seedLocal.ok || !cashLocal.ok || !lenderLocal.ok || !borrowerLocal.ok || !genesisLocal.ok) {
    return refuse('NON_CANONICAL_VALUE', 'definite data');
  }
  mkdirSync(workDir, { recursive: true });
  const params = [
    ['seed', seedLocal.hex],
    ['cash', cashLocal.hex],
    ['lender', lenderLocal.hex],
    ['borrower', borrowerLocal.hex],
    ['genesis', genesisLocal.hex],
  ];
  let input = blueprintPath;
  for (let i = 0; i < params.length; i += 1) {
    const out = join(workDir, `head-apply-${i}.json`);
    const result = run(aiken, ['blueprint', 'apply', '-i', input, '-o', out, '-m', 'financial_head', '-v', 'financial_head', params[i][1]]);
    if (result.status !== 0) return refuse('APPLY_FAILED', `${params[i][0]} ${result.stderr || result.stdout}`);
    input = out;
  }
  const appliedBp = JSON.parse(readFileSync(input, 'utf8'));
  const appliedMint = findValidator(appliedBp, 'mint', 'financial_head');
  if (!appliedMint?.compiledCode) return refuse('APPLY_FAILED', 'applied head missing');
  const agreement = agree(appliedMint.compiledCode, appliedMint.hash, 'head-applied-approved-cash');
  if (!agreement.ok) return refuse('SCRIPT_HASH_DIVERGENCE', 'applied head');
  const instance = instanceDigest(appliedMint.hash, cashPolicyId);
  if (!instance.ok) return instance;
  const scriptPath = join(workDir, 'head.applied.script.json');
  writeFileSync(scriptPath, JSON.stringify({
    type: 'PlutusScriptV3',
    description: 'financial head applied to the approved cash policy',
    cborHex: appliedMint.compiledCode,
  }));
  return {
    ok: true,
    code: 'HEAD_APPLIED_TO_CASH_POLICY',
    appliedHash: appliedMint.hash,
    unappliedHash: head.hash,
    cashPolicyId,
    unusedFailClosedCash: embedded.hash,
    instanceId: instance.instanceId,
    genesis: genesis.genesis,
    scriptPath,
    unitRoles: manifest.unitFixture === true,
    ledgerExecution: false,
    nativePins: false,
    cashCanMint: false,
  };
}
