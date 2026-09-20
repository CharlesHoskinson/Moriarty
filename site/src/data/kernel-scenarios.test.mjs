/**
 * Independent expected outcomes for the kernel illustration fixture.
 *
 * The rows come from the approved page design's accounting table. Each test
 * states the account it expects in integer hundredths and checks the reducer
 * against it. Passing here shows the illustration is internally consistent;
 * it establishes nothing about Moriarty, the kernel or any chain.
 *
 * Run: node --test site/src/data/kernel-scenarios.test.mjs
 */
import { test } from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import {
  CANDIDATES,
  EVENT_TYPES,
  EVIDENCE,
  POLICY,
  PRESETS,
  apply,
  availableEvents,
  checkCandidate,
  derive,
  evaluateEvidence,
  fmt,
  initialState,
  run,
} from './kernel-scenarios.mjs';

const acct = (s) => ({ grossDebit: s.account.grossDebit, fees: s.account.fees, reserved: s.account.reserved, confirmedReceipt: s.account.confirmedReceipt });
const row = (grossDebit, fees, reserved, confirmedReceipt) => ({ grossDebit, fees, reserved, confirmedReceipt });

const opened = () => run([
  { type: 'select-candidate', candidateId: 'route-direct' },
  { type: 'provider-accepts-duties' },
  { type: 'commit-candidate' },
  { type: 'present-evidence', evidenceId: 'fresh' },
]);

test('policy fixture matches the roadmap goal in hundredths', () => {
  assert.equal(POLICY.grossCap, 1100);
  assert.equal(POLICY.feeCap, 100);
  assert.equal(POLICY.minReceive, 2000);
  assert.equal(POLICY.fills * (POLICY.fillPrincipal + POLICY.fillFee), POLICY.grossCap);
  assert.equal(POLICY.fills * POLICY.fillFee, POLICY.feeCap);
  assert.equal(POLICY.fills * POLICY.fillReceive, POLICY.minReceive);
  assert.equal(POLICY.openingCustody, POLICY.grossCap, 'funding is a stated precondition equal to the gross cap');
});

test('a candidate proposal changes no account and imposes no duty', () => {
  const s = apply(initialState(), { type: 'select-candidate', candidateId: 'route-direct' });
  assert.deepEqual(acct(s), row(0, 0, 0, 0));
  assert.equal(s.duties.length, 0);
  assert.equal(s.phase, 'uncommitted');
  assert.equal(s.candidateVerdict.accepted, true);
});

test('wrong recipient fails by its named check and cannot be committed', () => {
  const v = checkCandidate(CANDIDATES.find((c) => c.id === 'route-wrong-recipient'));
  assert.equal(v.accepted, false);
  assert.deepEqual(v.checks.filter((c) => !c.met).map((c) => c.id), ['recipient']);
  const s = run([
    { type: 'select-candidate', candidateId: 'route-wrong-recipient' },
    { type: 'provider-accepts-duties' },
    { type: 'commit-candidate' },
  ]);
  assert.equal(s.phase, 'uncommitted');
  assert.equal(s.lastRejected.type, 'commit-candidate');
  assert.deepEqual(acct(s), row(0, 0, 0, 0));
});

test('excessive fee fails only the fee check; gross stays within cap', () => {
  const v = checkCandidate(CANDIDATES.find((c) => c.id === 'route-over-fee'));
  assert.equal(v.accepted, false);
  assert.deepEqual(v.checks.filter((c) => !c.met).map((c) => c.id), ['fees']);
  assert.equal(v.checks.find((c) => c.id === 'gross').met, true);
});

test('both compliant presets pass every named check', () => {
  for (const id of ['route-direct', 'route-split']) {
    const v = checkCandidate(CANDIDATES.find((c) => c.id === id));
    assert.equal(v.accepted, true, id);
    assert.equal(v.checks.length, 5);
  }
});

test('commitment requires provider consent to duties', () => {
  const s = run([
    { type: 'select-candidate', candidateId: 'route-direct' },
    { type: 'commit-candidate' },
  ]);
  assert.equal(s.phase, 'uncommitted');
  assert.match(s.lastRejected.reason, /consent/);
  const t = apply(apply(s, { type: 'provider-accepts-duties' }), { type: 'commit-candidate' });
  assert.equal(t.phase, 'committed');
  assert.deepEqual(acct(t), row(0, 0, 0, 0), 'commitment spends nothing');
  const discharge = t.duties.find((x) => x.id === 'deliver-remaining').discharge;
  assert.match(discharge, /every required fill/, 'discharge names every required fill');
  assert.match(discharge, /accepted against all signed predicates/, 'discharge requires acceptance, not receipt alone');
});

test('default path completes within caps: 11 A gross, 1 A fees, 20 B', () => {
  const s = run(PRESETS.success);
  assert.equal(s.phase, 'complete');
  assert.deepEqual(acct(s), row(1100, 100, 0, 2000));
  assert.equal(s.account.custody, 0);
  const d = derive(s);
  assert.equal(d.grossRemaining, 0);
  assert.equal(d.feeRemaining, 0);
  assert.equal(d.goalMet, true);
  assert.equal(s.duties.find((x) => x.id === 'deliver-remaining').status, 'discharged');
});

test('first finalized fill is an accepted prefix with persisting duties', () => {
  const s = apply(opened(), { type: 'finalize-fill' });
  assert.deepEqual(acct(s), row(550, 50, 0, 1000));
  assert.equal(s.account.custody, 550);
  assert.equal(s.phase, 'committed');
  assert.equal(s.duties.find((x) => x.id === 'deliver-remaining').status, 'accepted');
  assert.deepEqual(s.consumed, ['fill-1']);
});

test('reservation adds exposure, not debit, and spent plus reserved equals the cap', () => {
  const s = run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened());
  assert.deepEqual(acct(s), row(550, 50, 550, 1000));
  assert.equal(s.account.custody, 550, 'a reservation moves no custody');
  assert.equal(derive(s).exposure, 1100);
  assert.equal(derive(s).grossRemaining, 0);
});

test('timeout leaves the exposure unchanged', () => {
  const before = run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened());
  const s = apply(before, { type: 'timeout' });
  assert.deepEqual(acct(s), acct(before));
  assert.equal(s.attempt.status, 'timed-out');
});

test('late authenticated success converts the reservation and completes', () => {
  const s = run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }, { type: 'timeout' }, { type: 'observe', outcome: 'late-success' }], opened());
  assert.deepEqual(acct(s), row(1100, 100, 0, 2000));
  assert.equal(s.phase, 'complete');
  assert.ok(s.consumed.includes('attempt-2'));
});

/**
 * Audit R1. An authenticated external success is reality whether or not the
 * agreement's predicates hold. The account records it, the attempt identifier
 * is consumed, and no later conflicting result can release it. Acceptance as a
 * stage of the agreement is a separate judgment, taken exactly once when every
 * predicate holds.
 */
const observedWithout = (block) => run([
  { type: 'finalize-fill' },
  ...block,
  { type: 'reserve-fill' },
  { type: 'timeout' },
  { type: 'observe', outcome: 'late-success' },
], opened());

test('late success without the document predicate is accounted but not accepted', () => {
  const s = observedWithout([{ type: 'present-evidence', evidenceId: 'stale' }]);
  assert.equal(s.lastRejected, null, 'the observation is recorded, not refused');
  assert.equal(s.phase, 'observed');
  assert.deepEqual(acct(s), row(1100, 100, 0, 2000), 'debit, fee and receipt are external facts');
  assert.equal(s.account.custody, 0);
  assert.deepEqual(s.consumed, ['fill-1', 'attempt-2'], 'attempt consumed, fill not yet accepted');
  assert.equal(s.fillsDone, 1);
  assert.equal(s.attempt.accepted, false);
  assert.equal(s.duties.find((d) => d.id === 'deliver-remaining').status, 'accepted', 'delivery duty still open');
  assert.equal(derive(s).complete, false);
  assert.match(s.events.at(-1).meaning, /accounted/);
  assert.match(s.events.at(-1).meaning, /Document predicate is unmet/);
});

test('no conflicting later result can release or rewrite an observed success', () => {
  const s = observedWithout([{ type: 'present-evidence', evidenceId: 'stale' }]);
  for (const outcome of ['auth-failure', 'still-unknown', 'late-success']) {
    const t = apply(s, { type: 'observe', outcome });
    assert.equal(t.lastRejected?.type, 'observe', outcome);
    assert.match(t.lastRejected.reason, /attempt-2/);
    assert.deepEqual(acct(t), row(1100, 100, 0, 2000), `${outcome} moved the account`);
    assert.deepEqual(t.consumed, s.consumed);
    assert.deepEqual(t.duties, s.duties);
  }
  const dup = apply(s, { type: 'duplicate-observation' });
  assert.equal(dup.lastRejected, null);
  assert.match(dup.events.at(-1).title, /Duplicate success result for attempt-2/);
  assert.deepEqual(acct(dup), row(1100, 100, 0, 2000));
  const refund = apply(s, { type: 'premature-refund' });
  assert.equal(refund.lastRejected.type, 'premature-refund', 'nothing unresolved to refund against');
  const more = apply(s, { type: 'reserve-fill' });
  assert.equal(more.lastRejected.type, 'reserve-fill');
  assert.match(more.lastRejected.reason, /awaits acceptance/);
});

test('restoring the missing predicate reconciles the observed success exactly once', () => {
  for (const [block, restore] of [
    [[{ type: 'present-evidence', evidenceId: 'stale' }], { type: 'present-evidence', evidenceId: 'fresh' }],
    [[{ type: 'present-evidence', evidenceId: 'missing' }], { type: 'present-evidence', evidenceId: 'fresh' }],
    [[{ type: 'set-condition', condition: 'recipient-acceptance', present: false }], { type: 'set-condition', condition: 'recipient-acceptance', present: true }],
    [[{ type: 'set-condition', condition: 'provider-signature', present: false }], { type: 'set-condition', condition: 'provider-signature', present: true }],
  ]) {
    const s = observedWithout(block);
    const early = apply(s, { type: 'accept-observed-fill' });
    assert.equal(early.lastRejected?.type, 'accept-observed-fill', 'acceptance refused while the predicate is missing');
    assert.ok(!availableEvents(s).includes('accept-observed-fill'));
    const ready = apply(s, restore);
    assert.ok(availableEvents(ready).includes('accept-observed-fill'));
    const done = apply(ready, { type: 'accept-observed-fill' });
    assert.equal(done.lastRejected, null);
    assert.equal(done.phase, 'complete');
    assert.deepEqual(acct(done), row(1100, 100, 0, 2000), 'acceptance moves no money');
    assert.equal(done.events.at(-1).delta, null);
    assert.deepEqual(done.consumed, ['fill-1', 'attempt-2', 'fill-2']);
    assert.deepEqual(done.duties.map((d) => [d.id, d.status]), [['deliver-remaining', 'discharged'], ['remedy-on-failure', 'discharged']]);
    const again = apply(done, { type: 'accept-observed-fill' });
    assert.equal(again.lastRejected?.type, 'accept-observed-fill', 'second acceptance refused');
    assert.deepEqual(again.consumed, done.consumed);
    assert.deepEqual(again.duties, done.duties);
  }
});

/** Audit R2. The fixture enforces the approved ordering at the reducer boundary. */
test('the first fill must be finalized locally before any external attempt', () => {
  const s = apply(opened(), { type: 'reserve-fill' });
  assert.equal(s.lastRejected?.type, 'reserve-fill');
  assert.match(s.lastRejected.reason, /first fill is finalized locally/);
  assert.deepEqual(acct(s), row(0, 0, 0, 0));
  assert.equal(s.attempt, null);
  assert.ok(!availableEvents(opened()).includes('reserve-fill'));
  assert.ok(availableEvents(opened()).includes('finalize-fill'));
});

test('the second fill cannot be finalized locally; it is the external attempt', () => {
  const one = apply(opened(), { type: 'finalize-fill' });
  const two = apply(one, { type: 'finalize-fill' });
  assert.equal(two.lastRejected?.type, 'finalize-fill');
  assert.match(two.lastRejected.reason, /only the first fill is finalized locally/);
  assert.deepEqual(acct(two), row(550, 50, 0, 1000));
  assert.deepEqual(two.consumed, ['fill-1']);
  assert.ok(!availableEvents(one).includes('finalize-fill'));
  assert.ok(availableEvents(one).includes('reserve-fill'));
  const inflight = apply(one, { type: 'reserve-fill' });
  for (const type of ['finalize-fill', 'reserve-fill']) {
    const t = apply(inflight, { type });
    assert.equal(t.lastRejected?.type, type);
    assert.match(t.lastRejected.reason, /in flight/);
    assert.deepEqual(acct(t), acct(inflight));
  }
});

test('no consumed identifier is ever reused and no identifier appears twice', () => {
  const walk = (events, state) => {
    let s = state;
    for (const e of events) {
      s = apply(s, e);
      assert.equal(new Set(s.consumed).size, s.consumed.length, `duplicate identifier after ${e.type}: ${s.consumed}`);
    }
    return s;
  };
  const failed = walk(PRESETS.failure, initialState());
  assert.deepEqual(failed.consumed, ['fill-1', 'attempt-2']);
  const retry = apply(failed, { type: 'reserve-fill' });
  assert.equal(retry.lastRejected?.type, 'reserve-fill');
  assert.match(retry.lastRejected.reason, /fee cap/);
  // With a policy generous enough to pay another fee, the consumption rule is
  // what refuses the reuse of attempt-2.
  const generous = { ...POLICY, grossCap: 2000, feeCap: 200 };
  const failedGenerous = walk(PRESETS.failure, initialState(generous));
  assert.deepEqual(acct(failedGenerous), row(600, 100, 0, 1000));
  const reuse = apply(failedGenerous, { type: 'reserve-fill' });
  assert.equal(reuse.lastRejected?.type, 'reserve-fill');
  assert.match(reuse.lastRejected.reason, /attempt-2 is already consumed/);
  assert.deepEqual(acct(reuse), acct(failedGenerous));
  assert.equal(reuse.attempt.status, 'resolved');
  // A duplicate of the old failure result is not credited to anything either.
  const dup = apply(failedGenerous, { type: 'duplicate-observation' });
  assert.match(dup.events.at(-1).title, /Duplicate failure result for attempt-2/);
  assert.deepEqual(acct(dup), acct(failedGenerous));
  assert.deepEqual(dup.consumed, failedGenerous.consumed);
});

/** Audit R4. Every duty status is asserted on every terminal branch. */
test('duty statuses on the terminal branches', () => {
  const statuses = (s) => s.duties.map((d) => [d.id, d.status]);
  assert.deepEqual(statuses(run(PRESETS.success)), [['deliver-remaining', 'discharged'], ['remedy-on-failure', 'discharged']], 'failure-free completion discharges the conditional remedy');
  assert.deepEqual(derive(run(PRESETS.success)).openDuties, []);
  assert.deepEqual(statuses(run(PRESETS.failure)), [['deliver-remaining', 'accepted'], ['remedy-on-failure', 'pending']]);
  assert.deepEqual(statuses(run(PRESETS.unknown)), [['deliver-remaining', 'accepted'], ['remedy-on-failure', 'accepted']]);
  assert.deepEqual(statuses(apply(opened(), { type: 'finalize-fill' })), [['deliver-remaining', 'accepted'], ['remedy-on-failure', 'accepted']], 'a prefix triggers nothing');
});

test('a pending remedy is never cleared by a later delivery', () => {
  // Reachable only under a more generous policy: failure, then a fresh
  // completion path is impossible in this fixture because attempt-2 is
  // consumed, so the check is that finishIfComplete leaves a pending remedy
  // pending even when it is asked to complete.
  const generous = { ...POLICY, grossCap: 2000, feeCap: 200, fills: 2 };
  const failed = run(PRESETS.failure, initialState(generous));
  assert.equal(failed.duties.find((d) => d.id === 'remedy-on-failure').status, 'pending');
  const after = apply(failed, { type: 'finalize-fill' });
  assert.equal(after.lastRejected?.type, 'finalize-fill', 'ordering rule holds under any policy');
  assert.equal(after.duties.find((d) => d.id === 'remedy-on-failure').status, 'pending');
});

/** Audit R5. Narrative numbers are derived from the state they describe. */
test('event narratives state the arithmetic of the state they describe', () => {
  const contended = apply(run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened()), { type: 'second-solver-reserve' });
  const m = contended.events.at(-1).meaning;
  assert.match(m, /attempt-2 already holds 5\.5 A/);
  assert.match(m, /spent plus reserved is 11 A, so another 5\.5 A would reach 16\.5 A against the 11 A cap/);
  const failed = run(PRESETS.failure);
  assert.match(failed.events.at(-1).meaning, /5 A of ordinary capacity and no fee capacity remain/);
  const generous = { ...POLICY, grossCap: 2000, feeCap: 200 };
  const failedGenerous = run(PRESETS.failure, initialState(generous));
  assert.match(failedGenerous.events.at(-1).meaning, /14 A of ordinary capacity and 1 A of fee capacity remain/);
  const withheld = observedWithout([{ type: 'present-evidence', evidenceId: 'stale' }]);
  assert.match(withheld.events.at(-1).meaning, /5\.5 A left custody, 0\.5 A of it as fee, and 10 B sits at recipient-b-account/);
});

test('authenticated failure retains the fee and releases the reservation', () => {
  const s = run(PRESETS.failure);
  assert.deepEqual(acct(s), row(600, 100, 0, 1000));
  assert.equal(s.account.custody, 500);
  const d = derive(s);
  assert.equal(d.grossRemaining, 500, 'five A of ordinary capacity');
  assert.equal(d.feeRemaining, 0, 'no fee capacity');
  assert.equal(s.duties.find((x) => x.id === 'remedy-on-failure').status, 'pending');
  assert.equal(s.phase, 'committed');
});

test('after the failure, no further fill can be paid for under the fee cap', () => {
  const s = run(PRESETS.failure);
  const r = apply(s, { type: 'reserve-fill' });
  assert.match(r.lastRejected.reason, /fee cap/);
  assert.deepEqual(acct(r), acct(s));
  const f = apply(s, { type: 'finalize-fill' });
  assert.match(f.lastRejected.reason, /only the first fill is finalized locally/, 'the ordering rule refuses a local second fill before the cap is even consulted');
  assert.deepEqual(acct(f), acct(s));
});

test('still unknown changes nothing and refuses a premature refund', () => {
  const s = run(PRESETS.unknown);
  assert.deepEqual(acct(s), row(550, 50, 550, 1000));
  assert.equal(s.phase, 'in-flight');
  assert.equal(s.attempt.status, 'unknown');
  const last = s.events.at(-1);
  assert.equal(last.type, 'premature-refund');
  assert.match(last.meaning, /Refused/);
  assert.equal(last.delta, null);
});

test('a second solver cannot reserve consumed or pending authority', () => {
  const s = run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened());
  const t = apply(s, { type: 'second-solver-reserve' });
  assert.deepEqual(acct(t), acct(s));
  assert.match(t.events.at(-1).meaning, /Refused/);
});

test('duplicate terminal observation records no second discharge', () => {
  const s = run(PRESETS.success);
  const last = s.events.at(-1);
  assert.equal(last.type, 'duplicate-observation');
  assert.equal(last.delta, null);
  assert.deepEqual(acct(s), row(1100, 100, 0, 2000));
  assert.equal(s.events.filter((e) => e.type === 'observe').length, 1);
});

test('replay of a consumed fill identifier changes nothing', () => {
  const s = apply(opened(), { type: 'finalize-fill' });
  const t = apply(s, { type: 'replay-fill' });
  assert.deepEqual(acct(t), acct(s));
  assert.match(t.events.at(-1).title, /fill-1/);
  const u = apply(opened(), { type: 'replay-fill' });
  assert.equal(u.lastRejected.type, 'replay-fill', 'nothing consumed yet');
});

test('solver label changes nothing about authority or account', () => {
  const s = run(PRESETS.success);
  const human = run(PRESETS.success, apply(initialState(), { type: 'set-solver-kind', kind: 'human' }));
  const ai = run(PRESETS.success, apply(initialState(), { type: 'set-solver-kind', kind: 'ai' }));
  assert.deepEqual(acct(human), acct(ai));
  assert.deepEqual(acct(human), acct(s));
  assert.equal(ai.solverKind, 'ai');
  assert.equal(ai.events.length, human.events.length);
});

test('evidence fixtures produce four distinct outcomes; missing is not false', () => {
  const by = Object.fromEntries(EVIDENCE.map((e) => [e.id, evaluateEvidence(e).status]));
  assert.deepEqual(by, { fresh: 'met', stale: 'unmet', 'wrong-issuer': 'unmet', missing: 'unknown', unsupported: 'unsupported' });
});

test('release is withheld while evidence is missing, stale, wrong or unsupported', () => {
  for (const id of ['missing', 'stale', 'wrong-issuer', 'unsupported']) {
    const s = apply(apply(opened(), { type: 'present-evidence', evidenceId: id }), { type: 'finalize-fill' });
    assert.equal(s.lastRejected?.type, 'finalize-fill', id);
    assert.deepEqual(acct(s), row(0, 0, 0, 0), id);
  }
});

test('policy is immutable and candidate changes after commitment are refused', () => {
  const s = opened();
  assert.throws(() => { s.policy.grossCap = 5000; }, TypeError);
  const t = apply(s, { type: 'select-candidate', candidateId: 'route-split' });
  assert.equal(t.lastRejected.type, 'select-candidate');
  assert.equal(t.candidateId, 'route-direct');
});

test('reset restores an uncommitted fixture, announces it, and is not a rollback of anything', () => {
  const s = apply(run(PRESETS.failure), { type: 'reset' });
  assert.match(s.lastNote, /New illustration/);
  assert.match(s.lastNote, /nothing modeled was rolled back/);
  assert.deepEqual({ ...s, lastNote: null }, initialState());
});

test('release needs provider signature and recipient acceptance as well as the document', () => {
  for (const condition of ['provider-signature', 'recipient-acceptance']) {
    const s = apply(apply(opened(), { type: 'set-condition', condition, present: false }), { type: 'finalize-fill' });
    assert.equal(s.lastRejected?.type, 'finalize-fill', condition);
    assert.match(s.lastRejected.reason, /is missing/);
    assert.deepEqual(acct(s), row(0, 0, 0, 0));
    assert.equal(s.duties.find((d) => d.id === 'deliver-remaining').status, 'accepted', 'consent already given; withholding a condition changes no duty');
    const restored = apply(apply(s, { type: 'set-condition', condition, present: true }), { type: 'finalize-fill' });
    assert.deepEqual(acct(restored), row(550, 50, 0, 1000), `${condition} restored`);
  }
});

test('late success without recipient acceptance is accounted, consumed and held open', () => {
  const s = run([
    { type: 'finalize-fill' },
    { type: 'reserve-fill' },
    { type: 'set-condition', condition: 'recipient-acceptance', present: false },
    { type: 'timeout' },
    { type: 'observe', outcome: 'late-success' },
  ], opened());
  assert.equal(s.phase, 'observed');
  assert.deepEqual(acct(s), row(1100, 100, 0, 2000), 'the reservation converted; nothing is left to release');
  assert.match(s.events.at(-1).meaning, /Recipient acceptance is missing/);
  assert.equal(s.duties.find((d) => d.id === 'deliver-remaining').status, 'accepted');
  assert.equal(derive(s).openDuties.length, 2);
});

test('fee capacity distinguishes incurred from freely usable authority while in flight', () => {
  const inflight = run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened());
  const d = derive(inflight);
  assert.equal(d.feeRemaining, 50, 'incurred is 0.5 A, so 0.5 A remains unincurred');
  assert.equal(d.feeEncumbered, 50, 'the pending attempt encumbers its fee');
  assert.equal(d.feeFree, 0, 'nothing is freely usable');
  assert.equal(d.grossRemaining, 0);
  const done = run(PRESETS.success);
  assert.equal(derive(done).feeEncumbered, 0);
  assert.equal(derive(done).feeFree, 0);
});

test('terminal observations out of order or in conflict are refused', () => {
  const beforeSubmit = apply(opened(), { type: 'finalize-fill' });
  const early = apply(beforeSubmit, { type: 'observe', outcome: 'late-success' });
  assert.equal(early.lastRejected.type, 'observe', 'observation before any submission');
  assert.deepEqual(acct(early), acct(beforeSubmit));
  const success = run(PRESETS.success);
  const conflict = apply(success, { type: 'observe', outcome: 'auth-failure' });
  assert.equal(conflict.lastRejected.type, 'observe', 'failure after success');
  assert.deepEqual(acct(conflict), row(1100, 100, 0, 2000));
  const failure = run(PRESETS.failure);
  const conflict2 = apply(failure, { type: 'observe', outcome: 'late-success' });
  assert.equal(conflict2.lastRejected.type, 'observe', 'success after failure');
  assert.deepEqual(acct(conflict2), row(600, 100, 0, 1000));
  const dup = apply(failure, { type: 'duplicate-observation' });
  assert.deepEqual(acct(dup), row(600, 100, 0, 1000), 'duplicate failure retains no second fee');
  assert.equal(dup.lastRejected, null);
  const repeatTimeout = apply(run(PRESETS.unknown), { type: 'timeout' });
  assert.equal(repeatTimeout.lastRejected.type, 'timeout');
  const repeatUnknown = apply(run(PRESETS.unknown), { type: 'observe', outcome: 'still-unknown' });
  assert.deepEqual(acct(repeatUnknown), row(550, 50, 550, 1000), 'repeated uncertainty is stable');
});

test('every event type is reachable in the bounded fixture and rejected events never move the account', () => {
  const seen = new Set();
  const ready = run([{ type: 'select-candidate', candidateId: 'route-direct' }, { type: 'provider-accepts-duties' }]);
  const pending = observedWithout([{ type: 'present-evidence', evidenceId: 'stale' }]);
  const states = [initialState(), ready, opened(), apply(opened(), { type: 'finalize-fill' }), run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened()), run(PRESETS.failure), run(PRESETS.unknown), run(PRESETS.success), pending, apply(pending, { type: 'present-evidence', evidenceId: 'fresh' })];
  const samples = {
    'select-candidate': { type: 'select-candidate', candidateId: 'route-split' },
    'set-solver-kind': { type: 'set-solver-kind', kind: 'ai' },
    'present-evidence': { type: 'present-evidence', evidenceId: 'fresh' },
    'set-condition': { type: 'set-condition', condition: 'provider-signature', present: true },
    observe: { type: 'observe', outcome: 'still-unknown' },
  };
  for (const type of EVENT_TYPES) {
    for (const s of states) {
      const ev = samples[type] ?? { type };
      const t = apply(s, ev);
      if (t.lastRejected) assert.deepEqual(acct(t), acct(s), `${type} rejected but moved the account`);
      else seen.add(type);
      if (type !== 'reset' && !t.lastRejected && type !== 'set-solver-kind') assert.equal(t.events.length, s.events.length + 1, `${type} accepted without a record`);
    }
  }
  for (const type of EVENT_TYPES) assert.ok(seen.has(type), `${type} never accepted from any sampled state`);
});

test('available events agree with the reducer', () => {
  const pending = observedWithout([{ type: 'present-evidence', evidenceId: 'stale' }]);
  const states = [initialState(), opened(), apply(opened(), { type: 'finalize-fill' }), run([{ type: 'finalize-fill' }, { type: 'reserve-fill' }], opened()), run(PRESETS.unknown), run(PRESETS.success), pending, apply(pending, { type: 'present-evidence', evidenceId: 'fresh' })];
  const samples = { observe: { type: 'observe', outcome: 'still-unknown' } };
  for (const s of states) {
    for (const type of availableEvents(s)) {
      const t = apply(s, samples[type] ?? { type });
      assert.equal(t.lastRejected, null, `${type} offered from phase ${s.phase} but rejected: ${t.lastRejected?.reason}`);
    }
  }
});

test('formatting never rounds', () => {
  assert.equal(fmt(1100), '11');
  assert.equal(fmt(550), '5.5');
  assert.equal(fmt(50), '0.5');
  assert.equal(fmt(75), '0.75');
  assert.equal(fmt(0), '0');
});

test('no tooling or vendor attribution in the scenario module', () => {
  const src = readFileSync(new URL('./kernel-scenarios.mjs', import.meta.url), 'utf8');
  assert.equal(src.match(/\b(claude|anthropic|gemini|openai|gpt|copilot|chatgpt|llm|ai-generated)\b/i), null);
});

test('static transcripts in kernel.html agree with the model', () => {
  const html = readFileSync(new URL('../../kernel.html', import.meta.url), 'utf8');
  const rowsOf = (name) => {
    const table = html.split(`data-transcript="${name}"`)[1].split('</table>')[0];
    return [...table.matchAll(/data-account="(\d+),(\d+),(\d+),(\d+)"/g)].map((m) => m.slice(1, 5).map(Number));
  };
  const shared = rowsOf('shared');
  const prefix = PRESETS.success.slice(0, 7);
  assert.equal(shared.length, prefix.length);
  let s = initialState();
  prefix.forEach((e, i) => {
    s = apply(s, e);
    assert.deepEqual(shared[i], [s.account.grossDebit, s.account.fees, s.account.reserved, s.account.confirmedReceipt], `shared row ${i + 1}`);
  });
  const branches = rowsOf('branches');
  const expect = [run(PRESETS.success), run(PRESETS.failure), run(PRESETS.unknown)];
  assert.equal(branches.length, 3);
  expect.forEach((st, i) => assert.deepEqual(branches[i], [st.account.grossDebit, st.account.fees, st.account.reserved, st.account.confirmedReceipt], `branch row ${i + 1}`));
});
