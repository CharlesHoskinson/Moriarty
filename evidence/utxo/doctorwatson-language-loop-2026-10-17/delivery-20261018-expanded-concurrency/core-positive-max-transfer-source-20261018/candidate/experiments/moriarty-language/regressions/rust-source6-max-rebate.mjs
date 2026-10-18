// Opt-in public Rust CLI -> actual provisional Source/6/Core/5 consumers.
// Independent max-rebate oracle custody; earlier transfer freezes are untouched.
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseAndLowerSource6 } from '../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../src/successor/mil4-s0-source-v6.ts';

const fixtures = fileURLToPath(new URL('./fixtures/source6-max-rebate/', import.meta.url));
const binary = process.env.MORI_RUST_BIN;
const observations = process.env.MORI_SOURCE6_OBSERVATIONS;
assert.ok(binary && isAbsolute(binary), 'MORI_RUST_BIN must name the absolute reviewed Rust binary');
assert.ok(observations && isAbsolute(observations), 'MORI_SOURCE6_OBSERVATIONS must name a fresh absolute directory');
mkdirSync(observations);
const json = (name) => JSON.parse(readFileSync(join(fixtures, name), 'utf8'));
const pins = json('FIXTURE-PINS.json');
for (const [name, expected] of Object.entries(pins.files)) {
  const bytes = readFileSync(join(fixtures, name));
  assert.equal(createHash('sha256').update(bytes).digest('hex'), expected,
    `custodian fixture bytes changed: ${name}`);
}
const facts = json('oracle/EXPECTED-SEMANTICS.json');
assert.equal(facts.format, 'independent-max-transfer-semantic-oracle/1');
assert.equal(facts.cases.length, 1);

// These helpers only serialize observations and select named financial fields.
// They never calculate amounts, simulate a transfer or construct product ASTs.
function captureJson(path, value) {
  const encoded = JSON.stringify(value, null, 2) + '\n';
  writeFileSync(path, encoded);
  return JSON.parse(encoded);
}
function errorRecord(error) {
  return {
    name: error?.name ?? typeof error,
    message: error?.message ?? String(error),
    code: error?.code ?? null,
    offset: error?.offset ?? null,
    stack: error?.stack ?? null,
  };
}
function financialFrame(state) {
  return {
    domain: state.domain, asset: state.asset,
    balances: state.balances, allowances: state.allowances,
  };
}

captureJson(join(observations, 'run.json'), {
  node: process.version, binary,
  binarySha256: createHash('sha256').update(readFileSync(binary)).digest('hex'),
  oracleFreezeSha256: pins.custodianFreezeSha256,
  oracleManifestSha256: pins.custodianManifestSha256,
  expectedSemanticsSha256: pins.expectedSemanticsSha256,
  unasserted: ['whole emitted wire bytes/whitespace', 'Core status/qualification/gates',
    'measured work/cost and post-work counters', 'complete Core post/global replay inventory',
    'whole Program/Target/Artifact', 'authentication', 'native/proof/ledger', 'parent acceptance'],
});
// Preserve exact run inputs and the oracle manifest alongside the observations.
for (const name of ['max-rebate.mori', 'max-rebate.scenario.json']) {
  writeFileSync(join(observations, name), readFileSync(join(fixtures, 'oracle/inputs', name)));
}
writeFileSync(join(observations, 'oracle-MANIFEST.json'), readFileSync(join(fixtures, 'oracle/MANIFEST.json')));
writeFileSync(join(observations, 'oracle-FREEZE.sha256'), readFileSync(join(fixtures, 'oracle/FREEZE.sha256')));

for (const expected of facts.cases) {
  test(`${expected.action}: actual public Source/6 bytes against independent semantic observations`, () => {
    const expectedAst = json(`oracle/${expected.source6AstFile}`);
    const post = expected.conditionalClosingFinancialProjection;
    const args = ['--quiet', '--color', 'never', 'expand',
      join(fixtures, 'oracle/inputs/max-rebate.mori'), '--action', expected.action,
      '--scenario', join(fixtures, 'oracle/inputs/max-rebate.scenario.json'),
      '--emit-source6-transfer'];
    const source = join(fixtures, 'oracle/inputs/max-rebate.mori');
    const checkArgs = ['--quiet', '--color', 'never', 'check', source, '--json'];
    const check = spawnSync(binary, checkArgs, {
      cwd: fixtures, stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      timeout: 30_000, maxBuffer: 1024 * 1024,
    });
    const output = spawnSync(binary, args, {
      cwd: fixtures, stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      timeout: 30_000, maxBuffer: 1024 * 1024,
    });
    const destination = join(observations, expected.action);
    mkdirSync(destination);
    const checkStdout = check.stdout ?? Buffer.alloc(0);
    const checkStderr = check.stderr ?? Buffer.alloc(0);
    writeFileSync(join(destination, 'check.stdout.json'), checkStdout);
    writeFileSync(join(destination, 'check.stderr.txt'), checkStderr);
    captureJson(join(destination, 'check.process.json'), {
      executable: binary, args: checkArgs, cwd: fixtures, status: check.status, signal: check.signal,
      error: check.error ? errorRecord(check.error) : null,
      stdoutSha256: createHash('sha256').update(checkStdout).digest('hex'),
      stderrSha256: createHash('sha256').update(checkStderr).digest('hex'),
    });
    const stdout = output.stdout ?? Buffer.alloc(0);
    const stderr = output.stderr ?? Buffer.alloc(0);
    // Every actual process byte is saved before any product result assertion.
    writeFileSync(join(destination, 'stdout.source6'), stdout);
    writeFileSync(join(destination, 'stderr.txt'), stderr);
    captureJson(join(destination, 'process.json'), {
      executable: binary, args, cwd: fixtures, status: output.status, signal: output.signal,
      error: output.error ? errorRecord(output.error) : null,
      stdoutSha256: createHash('sha256').update(stdout).digest('hex'),
      stderrSha256: createHash('sha256').update(stderr).digest('hex'),
    });

    const actual = {
      decodeError: null, loweringAttempted: false, loweringError: null, lowered: null,
      preparationAttempted: false, preparationError: null, outcome: null,
    };
    let source6;
    try { source6 = new TextDecoder('utf-8', { fatal: true, ignoreBOM: true }).decode(stdout); }
    catch (error) { actual.decodeError = errorRecord(error); }
    if (source6 !== undefined) {
      // Untouched decoded stdout goes directly to both actual APIs. No trim,
      // fixture Source/6 substitution, JSON unwrap, AST construction or replay.
      actual.loweringAttempted = true;
      try { actual.lowered = parseAndLowerSource6(source6); }
      catch (error) { actual.loweringError = errorRecord(error); }
      actual.preparationAttempted = true;
      try { actual.outcome = prepareSource6S0Unqualified(source6); }
      catch (error) { actual.preparationError = errorRecord(error); }
    }
    // Capture complete returned objects or thrown errors before checking them.
    // JSON is the observation representation: undefined optional members are
    // omitted by JSON serialization; no product field value is recalculated.
    const observed = captureJson(join(destination, 'actual-consumer.json'), actual);

    assert.ifError(check.error);
    assert.equal(check.signal, null);
    assert.equal(check.status, 0, checkStderr.toString('utf8'));
    assert.equal(checkStderr.length, 0);
    const checked = JSON.parse(new TextDecoder('utf-8', { fatal: true }).decode(checkStdout));
    assert.equal(checked.status, 'accepted');
    assert.equal(checked.agreement.name, expectedAst.programId);
    assert.equal(checked.source_hash, pins.files['oracle/inputs/max-rebate.mori']);
    assert.deepEqual(checked.actions.map(({ name, support }) => ({ name, support })),
      [{ name: expected.action, support: 'local_s0' }]);
    assert.deepEqual(checked.diagnostics, []);
    assert.ifError(output.error);
    assert.equal(output.signal, null);
    assert.equal(output.status, 0, stderr.toString('utf8'));
    assert.equal(stderr.length, 0, '--quiet must suppress the adopted public status line');
    assert.ok(stdout.length > 0);
    assert.equal(stdout.at(-1), 10, 'adopted public output includes its final LF');
    assert.notEqual(stdout.at(-2), 10, 'adopted public output has a single final LF');
    assert.equal(observed.decodeError, null);
    assert.equal(observed.loweringError, null);
    assert.equal(observed.preparationError, null);
    assert.ok(observed.loweringAttempted && observed.preparationAttempted);
    const { ast, state, intent } = observed.lowered;
    // Full independent Source/6 AST, including every ordered effect and all
    // source/intent/claim/input fields. This is not a Core Program oracle.
    assert.deepEqual(ast, expectedAst);
    assert.deepEqual(ast.submitted.effects, expected.source6OrderedEffects);
    assert.equal(ast.submitted.effects.length, expected.effectCount);
    assert.deepEqual(observed.outcome.ast, expectedAst);

    assert.deepEqual(financialFrame(state), expected.openingFinancialFrame);
    assert.equal(state.head, expectedAst.authenticated.head);
    assert.equal(state.round, expectedAst.authenticated.round);
    assert.equal(state.workRemaining, expected.source6InputWork.remaining);
    assert.equal(state.workSpent, expected.source6InputWork.spent);
    assert.equal(state.consumedReplay.includes(post.selectedReplay.encodedKey), false);
    assert.deepEqual({
      sourceProfile: intent.sourceProfile, programId: intent.programId,
      sourceHash: intent.sourceHash, policyDigest: intent.policyDigest, keyRef: intent.keyRef,
      domain: intent.domain, asset: intent.asset, signer: intent.signer, nonce: intent.nonce,
      preHead: intent.preHead, notBefore: intent.notBefore, notAfter: intent.notAfter,
      grossCap: intent.grossCap, feeCap: intent.feeCap, netFloor: intent.netFloor,
      kind: intent.kind, recipient: intent.recipient, feeRecipient: intent.feeRecipient,
      amount: intent.amount, fee: intent.fee,
    }, {
      sourceProfile: expectedAst.profile, programId: expectedAst.selected.actionId,
      sourceHash: expectedAst.selected.sourceHash, policyDigest: expectedAst.selected.policyDigest,
      keyRef: expectedAst.intent.keyRef, domain: expectedAst.domain, asset: expectedAst.settlement.asset,
      signer: expectedAst.intent.signer, nonce: expectedAst.intent.nonce,
      preHead: expectedAst.intent.preHead, notBefore: expectedAst.intent.notBefore,
      notAfter: expectedAst.intent.notAfter, grossCap: expected.grossCap,
      feeCap: expected.feeCap, netFloor: expected.netFloor,
      kind: expectedAst.intent.signedAction.kind, recipient: expectedAst.intent.signedAction.to,
      feeRecipient: expectedAst.intent.signedAction.feeTo, amount: expected.value, fee: expected.fee,
    });
    assert.deepEqual(observed.lowered.submittedEffects, expected.core5SemanticOrderedEffects);
    assert.equal(observed.lowered.proposedPostHead, post.head);

    // Require the actual financial projection for the positive comparison;
    // status, qualification, required-premise and work-cost strings are only
    // observed in actual-consumer.json, never asserted as an oracle verdict.
    const candidate = observed.outcome.candidate;
    assert.ok(candidate && candidate.candidatePost, 'actual consumer must expose the compared financial candidate');
    assert.equal(candidate.preHead, expectedAst.authenticated.head);
    assert.deepEqual(candidate.effects, expected.core5SemanticOrderedEffects);
    assert.equal(candidate.effects.length, expected.effectCount);
    assert.deepEqual(financialFrame(candidate.candidatePost), {
      domain: post.domain, asset: post.asset, balances: post.balances, allowances: post.allowances,
    });
    assert.equal(candidate.candidatePost.head, post.head);
    assert.equal(candidate.candidatePost.round, post.round);
    // The independent oracle fixes the selected key, not a global replay
    // inventory or a security/history judgment about consuming that key.
    const selectedKey = candidate.candidatePost.consumedReplay
      .find((key) => key === post.selectedReplay.encodedKey);
    assert.equal(selectedKey, post.selectedReplay.encodedKey);
    assert.deepEqual(JSON.parse(selectedKey), post.selectedReplay.tuple);
    // No complete Core post, residual-duty DTO, post-work or qualification
    // oracle is invented. Their actual fields remain in the captured result.
  });
}
