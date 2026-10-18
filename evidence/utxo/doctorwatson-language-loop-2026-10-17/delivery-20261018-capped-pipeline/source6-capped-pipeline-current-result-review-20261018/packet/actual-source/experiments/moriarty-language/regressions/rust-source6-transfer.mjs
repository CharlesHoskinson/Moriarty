// Opt-in public Rust CLI -> existing experimental Source/6/Core/5 consumer.
// This is a local, provisional, unqualified comparison; never ledger admission.
import test from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { parseAndLowerSource6 } from '../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareSource6S0Unqualified } from '../src/successor/mil4-s0-source-v6.ts';

const fixtures = fileURLToPath(new URL('./fixtures/source6-transfer/', import.meta.url));
const binary = process.env.MORI_RUST_BIN;
const observations = process.env.MORI_SOURCE6_OBSERVATIONS;
assert.ok(binary && isAbsolute(binary), 'MORI_RUST_BIN must name the absolute reviewed Rust binary');
assert.ok(observations && isAbsolute(observations), 'MORI_SOURCE6_OBSERVATIONS must name a fresh absolute directory');
// Refuse an existing destination so a run cannot overwrite earlier observations.
mkdirSync(observations);
const json = (name) => JSON.parse(readFileSync(join(fixtures, name), 'utf8'));
const pins = json('FIXTURE-PINS.json');
for (const [name, expected] of Object.entries(pins.files)) {
  const actual = createHash('sha256').update(readFileSync(join(fixtures, name))).digest('hex');
  assert.equal(actual, expected, `custodian fixture bytes changed: ${name}`);
}
const facts = json('FINANCIAL-PRE-POST.json');
writeFileSync(join(observations, 'run.json'), JSON.stringify({
  node: process.version, binary,
  binarySha256: createHash('sha256').update(readFileSync(binary)).digest('hex'),
  financialFreezeSha256: pins.custodianFreezeSha256,
  qualification: 'local-stipulation-only; provisional S0; no runtime qualification beyond recorded checks',
  unasserted: ['whole emitted wire golden', 'success work policy/cost/post counters',
    'authentication', 'native embedding', 'proof', 'ledger', 'parent acceptance'],
}, null, 2) + '\n');

// Only a field-name adapter into the independent constructor/arguments schema.
// No quantities, effects or post-state are calculated for the product here.
function semanticEffects(effects, asset) {
  return effects.map((effect) => {
    let args;
    switch (effect.kind) {
      case 'Debit':
      case 'Credit':
        assert.equal(effect.asset, asset);
        args = [effect.account, effect.amount];
        break;
      case 'UseAllowance': args = [effect.owner, effect.amount]; break;
      // The public Source/6 contract defines the Core replay tuple encoding.
      case 'UseReplay': args = [JSON.parse(effect.key)]; break;
      case 'AdvanceHead': args = [effect.predecessor, effect.successor]; break;
      default: assert.fail(`unexpected effect in transfer-only case: ${effect.kind}`);
    }
    return { constructor: effect.kind, arguments: args };
  });
}

for (const expected of facts.cases) {
  test(`${expected.name}: public Rust output through provisional real Source/6/Core/5 consumer`, () => {
    const scenario = json(expected.scenario);
    const quantities = expected.exact_quantities;
    const pre = expected.financial_pre;
    const post = expected.financial_post_if_local_S0_transfer_admitted;
    const args = ['--quiet', '--color', 'never', 'expand', join(fixtures, expected.source),
      '--action', expected.action, '--scenario', join(fixtures, expected.scenario),
      '--emit-source6-transfer'];
    const output = spawnSync(binary, args, {
      cwd: fixtures, stdio: ['ignore', 'pipe', 'pipe'], shell: false,
      timeout: 30_000, maxBuffer: 1024 * 1024,
    });
    // Capture bytes before any result assertion. Never substitute fixture Source/6.
    const destination = join(observations, expected.name);
    mkdirSync(destination);
    writeFileSync(join(destination, 'stdout.source6'), output.stdout ?? Buffer.alloc(0));
    writeFileSync(join(destination, 'stderr.txt'), output.stderr ?? Buffer.alloc(0));
    writeFileSync(join(destination, 'process.json'), JSON.stringify({
      executable: binary, args, cwd: fixtures, status: output.status, signal: output.signal,
      error: output.error ? { code: output.error.code, message: output.error.message } : null,
    }, null, 2) + '\n');
    assert.ifError(output.error);
    assert.equal(output.signal, null);
    assert.equal(output.status, 0, output.stderr.toString('utf8'));
    assert.equal(output.stderr.length, 0, '--quiet must suppress the status line');
    assert.ok(output.stdout.length > 0);
    assert.equal(output.stdout.at(-1), 10, 'adopted public output includes its final LF');
    assert.notEqual(output.stdout.at(-2), 10, 'adopted public output has a single final LF');
    const source6 = new TextDecoder('utf-8', { fatal: true }).decode(output.stdout);
    // No trim, JSON unwrap, AST fabrication, Node beta expansion or regeneration.
    const lowered = parseAndLowerSource6(source6);
    const outcome = prepareSource6S0Unqualified(source6);
    writeFileSync(join(destination, 'actual-consumer.json'), JSON.stringify({
      lowered, outcome,
    }, null, 2) + '\n');

    const { ast, state, intent } = lowered;
    assert.equal(ast.domain, pre.domain);
    assert.equal(ast.settlement.asset, pre.asset);
    assert.equal(ast.authenticated.head, pre.head);
    assert.equal(ast.authenticated.predecessor, scenario.predecessor);
    assert.equal(ast.authenticated.round, scenario.round);
    assert.deepEqual(ast.authenticated.balances, pre.balances);
    assert.deepEqual(ast.authenticated.allowance, pre.allowance);
    assert.equal(ast.authenticated.replay, pre.replay.status);
    assert.equal(ast.authenticated.workRemaining, expected.work.opening_remaining);
    assert.equal(ast.authenticated.workSpent, expected.work.opening_spent);
    assert.equal(state.domain, pre.domain);
    assert.equal(state.asset, pre.asset);
    assert.equal(state.head, pre.head);
    assert.equal(state.round, scenario.round);
    assert.deepEqual(state.balances, pre.balances);
    assert.deepEqual(state.allowances, [pre.allowance]);
    assert.deepEqual(state.obligations, []);
    assert.deepEqual(state.consumedReplay, []);
    assert.equal(state.workRemaining, expected.work.opening_remaining);
    assert.equal(state.workSpent, expected.work.opening_spent);
    for (const action of [ast.intent.signedAction, ast.submitted.action]) {
      assert.equal(action.kind, 'Transfer');
      assert.equal(action.value, quantities.operation_value_atomic);
      assert.equal(action.fee, quantities.operation_fee_atomic);
    }
    assert.equal(intent.kind, 'Transfer');
    assert.equal(intent.amount, quantities.operation_value_atomic);
    assert.equal(intent.fee, quantities.operation_fee_atomic);
    assert.equal(intent.grossCap, quantities.gross_cap_atomic);
    assert.equal(intent.feeCap, quantities.fee_cap_atomic);
    assert.equal(intent.netFloor, quantities.net_floor_atomic);
    assert.deepEqual(semanticEffects(lowered.submittedEffects, pre.asset), expected.ordered_semantic_effects);
    assert.equal(lowered.proposedPostHead, post.head);

    // This classification is the provisional local consumer's callable predicate,
    // not an independently frozen accepted flag or real admission assertion.
    assert.equal(outcome.status, 'PreparedUnqualified', JSON.stringify(outcome));
    const candidate = outcome.candidate;
    assert.equal(candidate.status, 'PreparedUnqualified');
    assert.deepEqual(semanticEffects(candidate.effects, pre.asset), expected.ordered_semantic_effects);
    assert.equal(candidate.preHead, pre.head);
    assert.equal(candidate.candidatePost.domain, post.domain);
    assert.equal(candidate.candidatePost.asset, post.asset);
    assert.equal(candidate.candidatePost.head, post.head);
    assert.deepEqual(candidate.candidatePost.balances, post.balances);
    assert.deepEqual(candidate.candidatePost.allowances, [post.allowance]);
    assert.deepEqual(candidate.candidatePost.obligations, []);
    assert.deepEqual(candidate.candidatePost.consumedReplay.map((key) => JSON.parse(key)),
      [post.replay.key_components]);
    // Work post counters/cost are observed in actual-consumer.json, never asserted.
    // Residual duties have no independent DTO field in this S0State projection.
  });
}
