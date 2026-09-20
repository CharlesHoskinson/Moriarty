/**
 * Educational scenario model for the Federated DeFi Kernel page.
 *
 * Pure and deterministic. The signed policy is immutable for the life of a
 * run; every change to the account is an explicit event applied by `apply`.
 * Amounts are integer hundredths of a toy asset unit: 1100 means 11.00 A.
 *
 * This models an illustration of the target design. It signs nothing, submits
 * nothing, verifies no proof and is not a kernel implementation. Its job is to
 * keep six things apart that a careless account merges: actual spending,
 * incurred fees, pending reservation, confirmed receipt, external observation
 * and accepted duties.
 *
 * Consumed by the React explorer and by kernel-scenarios.test.mjs.
 */

// --- fixture ----------------------------------------------------------------

/**
 * Domain-qualified toy assets. Fixtures, not market instruments. The static
 * article names these identities so the reader can see the domain distinction.
 */
export const ASSETS = Object.freeze({
  A: Object.freeze({ symbol: 'A', domain: 'midnight-illustration', issuer: 'fixture-issuer-a', reference: 'asset-a-ref-01', kind: 'token' }),
  B: Object.freeze({ symbol: 'B', domain: 'foreign-illustration', issuer: 'fixture-issuer-b', reference: 'asset-b-ref-01', kind: 'token' }),
});

/**
 * The signed intention. Gross cap 11.00 A including at most 1.00 A fees, and a
 * goal of at least 20.00 B for the named recipient. Two partial deliveries are
 * permitted. Funding is a stated precondition: the buyer's 11.00 A is already
 * under escrow custody when the illustration opens, so no deposit transition is
 * modeled and no funding movement counts against the gross cap.
 *
 * The fixture follows the approved story exactly: the first fill is finalized
 * locally and the second fill is reserved and submitted to the external
 * domain. The reducer refuses any other ordering, so every reachable state is
 * one the transcripts and tests describe.
 */
export const POLICY = Object.freeze({
  id: 'intent-fixture-01',
  owner: 'buyer',
  provider: 'provider',
  recipient: 'recipient-b-account',
  spend: ASSETS.A,
  receive: ASSETS.B,
  grossCap: 1100,
  feeCap: 100,
  minReceive: 2000,
  fills: 2,
  fillPrincipal: 500,
  fillFee: 50,
  fillReceive: 1000,
  openingCustody: 1100,
  conditions: Object.freeze(['provider-signature', 'recipient-acceptance', 'document-attestation']),
  attestationIssuer: 'issuer-fixture-1',
  attestationFreshnessEpochs: 1,
});

/** Candidate presets. The first two are compliant. The others fail by name. */
export const CANDIDATES = Object.freeze([
  Object.freeze({
    id: 'route-direct',
    label: 'Direct route',
    summary: 'Two fills of 5 A + 0.5 A fee each for 10 B, sent to the named recipient.',
    recipient: POLICY.recipient,
    principalPerFill: 500,
    feePerFill: 50,
    receivePerFill: 1000,
    fills: 2,
  }),
  Object.freeze({
    id: 'route-split',
    label: 'Split-liquidity route',
    summary: 'Same fills through two venues. Different route, same signed constraints.',
    recipient: POLICY.recipient,
    principalPerFill: 500,
    feePerFill: 50,
    receivePerFill: 1000,
    fills: 2,
  }),
  Object.freeze({
    id: 'route-wrong-recipient',
    label: 'Wrong recipient',
    summary: 'Same amounts, but delivery is addressed to the solver’s own account.',
    recipient: 'solver-own-account',
    principalPerFill: 500,
    feePerFill: 50,
    receivePerFill: 1000,
    fills: 2,
  }),
  Object.freeze({
    id: 'route-over-fee',
    label: 'Excessive fee',
    summary: 'Same 5.5 A per fill, but 0.75 A of it is fee. Total fees would reach 1.5 A against a 1 A cap while gross stays within 11 A.',
    recipient: POLICY.recipient,
    principalPerFill: 475,
    feePerFill: 75,
    receivePerFill: 1000,
    fills: 2,
  }),
]);

/** Presentation identity only. Authority rules never read this value. */
export const SOLVER_KINDS = Object.freeze([
  Object.freeze({ id: 'human', label: 'Human solver' }),
  Object.freeze({ id: 'ai', label: 'AI solver' }),
]);

/** Evidence fixtures for the document-attestation condition. */
export const EVIDENCE = Object.freeze([
  Object.freeze({ id: 'fresh', label: 'Fresh, authorized issuer', issuer: POLICY.attestationIssuer, ageEpochs: 0, supported: true }),
  Object.freeze({ id: 'stale', label: 'Stale attestation', issuer: POLICY.attestationIssuer, ageEpochs: 3, supported: true }),
  Object.freeze({ id: 'wrong-issuer', label: 'Wrong issuer', issuer: 'issuer-unknown-9', ageEpochs: 0, supported: true }),
  Object.freeze({ id: 'missing', label: 'No attestation yet', issuer: null, ageEpochs: null, supported: true }),
  Object.freeze({ id: 'unsupported', label: 'Unsupported format', issuer: POLICY.attestationIssuer, ageEpochs: 0, supported: false }),
]);

/** Observation branches after a submitted attempt times out. */
export const OBSERVATIONS = Object.freeze([
  Object.freeze({ id: 'late-success', label: 'Late authenticated success' }),
  Object.freeze({ id: 'auth-failure', label: 'Authenticated failure, fee retained' }),
  Object.freeze({ id: 'still-unknown', label: 'Still unknown' }),
]);

// --- candidate checks -------------------------------------------------------

/**
 * Named checks against the signed policy. Every check is reported, met or not,
 * so a reader sees which condition a rejected candidate actually failed.
 * A rejected candidate touches nothing: this function reads the policy only.
 */
export function checkCandidate(candidate, policy = POLICY) {
  const grossTotal = (candidate.principalPerFill + candidate.feePerFill) * candidate.fills;
  const feeTotal = candidate.feePerFill * candidate.fills;
  const receiveTotal = candidate.receivePerFill * candidate.fills;
  const checks = [
    { id: 'recipient', name: 'Delivery goes to the named recipient', met: candidate.recipient === policy.recipient,
      detail: candidate.recipient === policy.recipient ? policy.recipient : `${candidate.recipient} is not ${policy.recipient}` },
    { id: 'gross', name: 'Gross spend within cap', met: grossTotal <= policy.grossCap,
      detail: `${fmt(grossTotal)} A of ${fmt(policy.grossCap)} A` },
    { id: 'fees', name: 'Fees within fee cap', met: feeTotal <= policy.feeCap,
      detail: `${fmt(feeTotal)} A of ${fmt(policy.feeCap)} A` },
    { id: 'goal', name: 'Full completion reaches the net goal', met: receiveTotal >= policy.minReceive,
      detail: `${fmt(receiveTotal)} B against ${fmt(policy.minReceive)} B` },
    { id: 'fills', name: 'Fill count permitted by policy', met: candidate.fills <= policy.fills,
      detail: `${candidate.fills} of ${policy.fills} permitted` },
  ];
  return Object.freeze({ candidateId: candidate.id, accepted: checks.every((c) => c.met), checks: Object.freeze(checks.map(Object.freeze)) });
}

// --- evidence predicate -----------------------------------------------------

/**
 * Evaluates the document-attestation predicate for one evidence fixture.
 * Four outcomes are distinct: met, unmet, unknown (nothing to evaluate) and
 * unsupported (the policy cannot read it). Missing evidence is not false.
 */
export function evaluateEvidence(evidence, policy = POLICY) {
  if (!evidence.supported) return Object.freeze({ status: 'unsupported', reason: 'The policy names no rule for this evidence format. It establishes nothing either way.' });
  if (evidence.issuer === null) return Object.freeze({ status: 'unknown', reason: 'No attestation has been presented. The predicate is unevaluated, not failed.' });
  if (evidence.issuer !== policy.attestationIssuer) return Object.freeze({ status: 'unmet', reason: `Issuer ${evidence.issuer} is not the authorized ${policy.attestationIssuer}.` });
  if (evidence.ageEpochs > policy.attestationFreshnessEpochs) return Object.freeze({ status: 'unmet', reason: `Attestation is ${evidence.ageEpochs} epochs old; the policy allows ${policy.attestationFreshnessEpochs}.` });
  return Object.freeze({ status: 'met', reason: 'Fresh attestation from the authorized issuer. It establishes the checked predicate, not physical delivery.' });
}

// --- state ------------------------------------------------------------------

/**
 * A fresh, uncommitted illustration. The policy is frozen into the state so
 * that nothing downstream can read a different one.
 */
export function initialState(policy = POLICY, note = null) {
  return Object.freeze({
    policy,
    phase: 'uncommitted',
    candidateId: null,
    candidateVerdict: null,
    solverKind: 'human',
    evidenceId: 'missing',
    conditions: Object.freeze({ 'provider-signature': true, 'recipient-acceptance': true }),
    account: Object.freeze({
      grossDebit: 0,
      fees: 0,
      reserved: 0,
      confirmedReceipt: 0,
      custody: policy.openingCustody,
    }),
    fillsDone: 0,
    attempt: null,
    consumed: Object.freeze([]),
    duties: Object.freeze([]),
    events: Object.freeze([]),
    lastNote: note,
    lastRejected: null,
  });
}

/** The two signed conditions beside the document attestation. */
export const CONDITIONS = Object.freeze([
  Object.freeze({ id: 'provider-signature', label: 'Provider signature' }),
  Object.freeze({ id: 'recipient-acceptance', label: 'Recipient acceptance' }),
]);

// --- events -----------------------------------------------------------------

/** Every event type the bounded fixture can receive, in one place for tests. */
export const EVENT_TYPES = Object.freeze([
  'select-candidate',
  'set-solver-kind',
  'present-evidence',
  'set-condition',
  'provider-accepts-duties',
  'commit-candidate',
  'finalize-fill',
  'reserve-fill',
  'timeout',
  'observe',
  'accept-observed-fill',
  'duplicate-observation',
  'second-solver-reserve',
  'replay-fill',
  'premature-refund',
  'reset',
]);

/**
 * The reducer. Returns a new frozen state. A rejected event returns a state
 * whose account is identical to the input's and whose `lastRejected` explains
 * why. The account never changes without an appended event record.
 */
export function apply(state, event) {
  switch (event.type) {
    case 'reset':
      return initialState(state.policy, 'New illustration. A fresh uncommitted fixture; nothing modeled was rolled back.');

    case 'set-condition': {
      if (!CONDITIONS.some((c) => c.id === event.condition)) return reject(state, event, 'Unknown condition.');
      const present = Boolean(event.present);
      const label = CONDITIONS.find((c) => c.id === event.condition).label;
      return record({ ...state, conditions: Object.freeze({ ...state.conditions, [event.condition]: present }) }, event, {
        title: `${label} ${present ? 'present' : 'withheld'}`,
        responsibility: event.condition === 'provider-signature' ? 'external' : 'owner',
        meaning: present
          ? `${label} is now available to the release check. Availability alone releases nothing.`
          : `${label} is missing. Release needs every condition together, so no fill can finalize until it returns. No account change.`,
        delta: null,
      });
    }

    case 'set-solver-kind': {
      if (!SOLVER_KINDS.some((k) => k.id === event.kind)) return reject(state, event, 'Unknown solver label.');
      return withNote({ ...state, solverKind: event.kind }, 'Solver label changed. Authority and acceptance rules are unchanged.');
    }

    case 'present-evidence': {
      if (!EVIDENCE.some((e) => e.id === event.evidenceId)) return reject(state, event, 'Unknown evidence fixture.');
      const status = evaluateEvidence(EVIDENCE.find((e) => e.id === event.evidenceId), state.policy);
      return record({ ...state, evidenceId: event.evidenceId }, event, {
        title: `Evidence presented: ${EVIDENCE.find((e) => e.id === event.evidenceId).label}`,
        responsibility: 'kernel',
        meaning: `Document predicate ${status.status}. ${status.reason} No account change.`,
        delta: null,
      });
    }

    case 'select-candidate': {
      if (state.phase !== 'uncommitted') return reject(state, event, 'A candidate is already committed. Reset to a new illustration to compare another route.');
      const candidate = CANDIDATES.find((c) => c.id === event.candidateId);
      if (!candidate) return reject(state, event, 'Unknown candidate.');
      const verdict = checkCandidate(candidate, state.policy);
      const failed = verdict.checks.filter((c) => !c.met).map((c) => c.name);
      return record({ ...state, candidateId: candidate.id, candidateVerdict: verdict }, event, {
        title: `Candidate proposed: ${candidate.label}`,
        responsibility: 'solver',
        meaning: verdict.accepted
          ? 'Every named check holds. Still a proposal: no accepted effects and no duties imposed on anyone.'
          : `Rejected before commitment: ${failed.join('; ')}. The agreement account is untouched.`,
        delta: null,
      });
    }

    case 'provider-accepts-duties': {
      if (state.phase !== 'uncommitted') return reject(state, event, 'Duties are accepted before commitment in this fixture.');
      if (state.duties.some((d) => d.id === 'deliver-remaining')) return reject(state, event, 'The provider has already accepted these duties.');
      const duties = [
        ...state.duties,
        Object.freeze({ id: 'deliver-remaining', bearer: state.policy.provider, duty: `Deliver the remaining B up to ${fmt(state.policy.minReceive)} B against accepted fills`, discharge: 'Authenticated confirmed receipt of every required fill, with each fill accepted against all signed predicates', status: 'accepted' }),
        Object.freeze({ id: 'remedy-on-failure', bearer: state.policy.provider, duty: 'Remedy an authenticated failed fill under the signed remedy terms', discharge: 'Authenticated remedy transition or authenticated failure-free completion', status: 'accepted' }),
      ];
      return record({ ...state, duties: Object.freeze(duties) }, event, {
        title: 'Provider consents to delivery and remedy duties',
        responsibility: 'owner',
        meaning: 'Consent to material terms precedes any duty. A request alone could not impose these on the provider.',
        delta: null,
      });
    }

    case 'commit-candidate': {
      if (state.phase !== 'uncommitted') return reject(state, event, 'Already committed.');
      if (!state.candidateVerdict) return reject(state, event, 'Select a candidate first.');
      if (!state.candidateVerdict.accepted) return reject(state, event, 'This candidate failed a named check and cannot be committed. Account unchanged.');
      if (!state.duties.some((d) => d.id === 'deliver-remaining')) return reject(state, event, 'The provider has not accepted its duties. Delivery duties cannot arise without consent.');
      return record({ ...state, phase: 'committed' }, event, {
        title: 'Candidate committed under the signed policy',
        responsibility: 'moriarty',
        meaning: 'The route is now bound to the intention. Still no spending: commitment authorizes stages, it does not execute them.',
        delta: null,
      });
    }

    case 'finalize-fill': {
      if (state.phase !== 'committed') return reject(state, event, phaseHint(state, 'A fill can be finalized only from a committed, idle agreement.'));
      if (state.fillsDone >= state.policy.fills) return reject(state, event, 'All permitted fills are used.');
      if (state.fillsDone !== 0) return reject(state, event, 'In this fixture only the first fill is finalized locally. The second fill is reserved and submitted to the external domain.');
      const unmet = unmetConditions(state);
      if (unmet) return reject(state, event, `Release requires every condition together. ${unmet}`);
      const fillId = `fill-${state.fillsDone + 1}`;
      const cost = state.policy.fillPrincipal + state.policy.fillFee;
      const capFailure = capCheck(state, cost);
      if (capFailure) return reject(state, event, capFailure);
      const next = {
        ...state,
        fillsDone: state.fillsDone + 1,
        account: Object.freeze({
          ...state.account,
          grossDebit: state.account.grossDebit + cost,
          fees: state.account.fees + state.policy.fillFee,
          custody: state.account.custody - cost,
          confirmedReceipt: state.account.confirmedReceipt + state.policy.fillReceive,
        }),
        consumed: Object.freeze([...state.consumed, fillId]),
      };
      const done = next.fillsDone >= state.policy.fills;
      return record(finishIfComplete(next), event, {
        title: `${ordinal(next.fillsDone)} fill finalized: provider signature, recipient acceptance and fresh attestation all hold`,
        responsibility: 'external',
        meaning: done
          ? 'Every fill is confirmed and the net goal is met. Completion is a judgment about the whole obligation, now discharged.'
          : 'An accepted prefix. Its debit and fee are permanent. The remaining delivery duty persists against the remaining authority.',
        delta: { grossDebit: cost, fees: state.policy.fillFee, custody: -cost, confirmedReceipt: state.policy.fillReceive },
      });
    }

    case 'reserve-fill': {
      if (state.phase !== 'committed') return reject(state, event, phaseHint(state, 'A reservation needs a committed, idle agreement.'));
      if (state.fillsDone >= state.policy.fills) return reject(state, event, 'All permitted fills are used.');
      if (state.fillsDone !== 1) return reject(state, event, 'In this fixture the first fill is finalized locally before any external attempt. Finalize the first fill, then reserve the second.');
      const cost = state.policy.fillPrincipal + state.policy.fillFee;
      const capFailure = capCheck(state, cost);
      if (capFailure) return reject(state, event, capFailure);
      const attemptId = `attempt-${state.fillsDone + 1}`;
      if (state.consumed.includes(attemptId)) {
        return reject(state, event, `${attemptId} is already consumed by its authenticated result. A retry would be a new attempt with a fresh identifier, which this fixture does not model. Account unchanged.`);
      }
      return record({
        ...state,
        phase: 'in-flight',
        attempt: Object.freeze({ id: attemptId, status: 'submitted', fill: state.fillsDone + 1 }),
        account: Object.freeze({ ...state.account, reserved: state.account.reserved + cost }),
      }, event, {
        title: `${ordinal(state.fillsDone + 1)} fill reserved and submitted to the external domain`,
        responsibility: 'kernel',
        meaning: `Spent plus reserved equals ${fmt(state.account.grossDebit + state.account.reserved + cost)} A. A reservation is exposure against authority, not a debit and not proof the asset sits in escrow. Another solver cannot reserve it again.`,
        delta: { reserved: cost },
      });
    }

    case 'timeout': {
      if (state.phase !== 'in-flight' || state.attempt?.status !== 'submitted') return reject(state, event, 'Nothing is in flight to time out.');
      return record({ ...state, attempt: Object.freeze({ ...state.attempt, status: 'timed-out' }) }, event, {
        title: 'Deadline passes with no result',
        responsibility: 'external',
        meaning: 'The exposure is unchanged. A timeout changes which transitions may be attempted; it does not prove the other chain failed to execute. The escrow balance shown is the last confirmed figure; the current location of the reserved amount is unknown.',
        delta: null,
      });
    }

    case 'observe': {
      if (!OBSERVATIONS.some((o) => o.id === event.outcome)) return reject(state, event, 'Unknown observation.');
      const conflict = terminalConflict(state, event.outcome);
      if (conflict) return reject(state, event, conflict);
      if (state.phase !== 'in-flight') return reject(state, event, 'Nothing is in flight to observe.');
      const cost = state.policy.fillPrincipal + state.policy.fillFee;
      if (event.outcome === 'late-success') {
        // The authenticated external result is accounted unconditionally: the
        // destination executed the transfer, so the principal and fee left
        // custody and the B sits at the named recipient account. Whether that
        // fill is an accepted stage of this agreement is a separate judgment.
        const observed = {
          ...state,
          phase: 'observed',
          attempt: Object.freeze({ ...state.attempt, status: 'resolved', outcome: 'late-success', accepted: false }),
          account: Object.freeze({
            ...state.account,
            grossDebit: state.account.grossDebit + cost,
            fees: state.account.fees + state.policy.fillFee,
            reserved: state.account.reserved - cost,
            custody: state.account.custody - cost,
            confirmedReceipt: state.account.confirmedReceipt + state.policy.fillReceive,
          }),
          consumed: Object.freeze([...state.consumed, state.attempt.id]),
        };
        const delta = { grossDebit: cost, fees: state.policy.fillFee, reserved: -cost, custody: -cost, confirmedReceipt: state.policy.fillReceive };
        const unmet = unmetConditions(state);
        if (unmet) {
          return record(observed, event, {
            title: 'Late authenticated success; acceptance withheld',
            responsibility: 'external',
            meaning: `The destination executed the transfer: ${fmt(cost)} A left custody, ${fmt(state.policy.fillFee)} A of it as fee, and ${fmt(state.policy.fillReceive)} B sits at ${state.policy.recipient}. That is accounted and ${state.attempt.id} is consumed; no later result can release it or say it did not happen. The fill is not yet an accepted stage of this agreement: ${unmet} The delivery duty stays open until every predicate holds and the observed fill is accepted once.`,
            delta,
          });
        }
        return record(acceptObserved(observed), event, {
          title: 'Late authenticated success',
          responsibility: 'external',
          meaning: 'The reservation converts to an actual debit, the receipt is confirmed, and the attempt identifier is consumed. Every predicate holds, so the fill is accepted in the same step. Late does not mean invalid.',
          delta,
        });
      }
      if (event.outcome === 'auth-failure') {
        const fee = state.policy.fillFee;
        const duties = state.duties.map((d) => (d.id === 'remedy-on-failure' ? Object.freeze({ ...d, status: 'pending' }) : d));
        const after = { grossDebit: state.account.grossDebit + fee, fees: state.account.fees + fee };
        return record({
          ...state,
          phase: 'committed',
          attempt: Object.freeze({ ...state.attempt, status: 'resolved', outcome: 'auth-failure' }),
          account: Object.freeze({
            ...state.account,
            grossDebit: after.grossDebit,
            fees: after.fees,
            reserved: state.account.reserved - cost,
            custody: state.account.custody - fee,
          }),
          consumed: Object.freeze([...state.consumed, state.attempt.id]),
          duties: Object.freeze(duties),
        }, event, {
          title: 'Authenticated failed attempt; its fee is retained',
          responsibility: 'external',
          meaning: `The fixture's evidence policy establishes principal nonexecution, so the reservation releases. The ${fmt(fee)} A fee is gone and counts against both caps. ${capacityPhrase(state.policy, after)} The remedy duty is now pending.`,
          delta: { grossDebit: fee, fees: fee, reserved: -cost, custody: -fee },
        });
      }
      return record({ ...state, attempt: Object.freeze({ ...state.attempt, status: 'unknown' }) }, event, {
        title: 'Reconciliation finds no authenticated result',
        responsibility: 'kernel',
        meaning: 'Nothing changes. No invented recovery and no automatic release: unknown is a retained state, not a failure. The current location of the reserved amount stays unknown; the escrow row is still the last confirmed balance.',
        delta: null,
      });
    }

    case 'accept-observed-fill': {
      if (state.phase !== 'observed' || state.attempt?.outcome !== 'late-success') return reject(state, event, 'No observed fill is awaiting acceptance.');
      if (state.attempt.accepted) return reject(state, event, 'This observed fill was already accepted once. A second acceptance would be a second discharge.');
      const unmet = unmetConditions(state);
      if (unmet) return reject(state, event, `The observed fill stays accounted but not accepted. ${unmet}`);
      return record(acceptObserved(state), event, {
        title: 'Observed fill accepted against the agreement',
        responsibility: 'moriarty',
        meaning: `Every predicate now holds, so the already-accounted ${state.attempt.id} result is reconciled exactly once. No new debit or receipt: the external effect was recorded when it was observed.`,
        delta: null,
      });
    }

    case 'duplicate-observation': {
      if (!state.attempt || state.attempt.status !== 'resolved') return reject(state, event, 'No terminal observation exists yet to replay.');
      // A duplicate is the same authenticated result presented again. It goes
      // through the same conflict check as any observation; the refusal is
      // recorded so the reader sees the reason.
      const refusal = terminalConflict(state, state.attempt.outcome);
      if (!refusal) return reject(state, event, 'The consumption rule did not recognize this attempt as consumed. This is a model defect, not a payout.');
      return record(state, event, {
        title: `Duplicate ${state.attempt.outcome === 'late-success' ? 'success' : 'failure'} result for ${state.attempt.id} arrives`,
        responsibility: 'moriarty',
        meaning: `${refusal} No second payment, no second discharge, no account change.`,
        delta: null,
      });
    }

    case 'second-solver-reserve': {
      if (state.phase !== 'in-flight') return reject(state, event, 'Nothing is reserved for another solver to contend with. Reserve a fill first.');
      const cost = state.policy.fillPrincipal + state.policy.fillFee;
      const exposure = state.account.grossDebit + state.account.reserved;
      // Two distinct reasons, each named only when it actually applies: the
      // pending attempt already holds this authority, and the cap arithmetic.
      const reasons = [`${state.attempt.id} already holds ${fmt(state.account.reserved)} A of reserved authority and a reservation is exclusive until that attempt resolves`];
      if (exposure + cost > state.policy.grossCap) reasons.push(`spent plus reserved is ${fmt(exposure)} A, so another ${fmt(cost)} A would reach ${fmt(exposure + cost)} A against the ${fmt(state.policy.grossCap)} A cap`);
      return record(state, event, {
        title: 'A second solver tries to reserve the same authority',
        responsibility: 'moriarty',
        meaning: `Refused: ${reasons.join('; and ')}. Account unchanged.`,
        delta: null,
      });
    }

    case 'replay-fill': {
      if (state.consumed.length === 0) return reject(state, event, 'No consumed identifier exists yet to replay.');
      return record(state, event, {
        title: `Replay of ${state.consumed[0]} presented`,
        responsibility: 'moriarty',
        meaning: 'The identifier is consumed. A valid-looking receipt for a spent stage confers nothing. Account unchanged.',
        delta: null,
      });
    }

    case 'premature-refund': {
      if (state.phase !== 'in-flight') return reject(state, event, 'A refund attempt only makes sense against an unresolved attempt.');
      return record(state, event, {
        title: 'Refund attempted on the strength of elapsed time and a recovery signature',
        responsibility: 'moriarty',
        meaning: 'Refused. Elapsed time is not nonexecution and a recovery grant alone is not eligibility. Without authenticated evidence, controlled assets and an exclusive consumption rule, the outcome stays unresolved.',
        delta: null,
      });
    }

    default:
      return reject(state, event, `Unknown event ${String(event.type)}.`);
  }
}

// --- derived views ----------------------------------------------------------

/** Remaining authority, derived from the one account. */
export function derive(state) {
  const p = state.policy;
  const a = state.account;
  // A pending attempt encumbers its fee as well as its principal. Incurred and
  // freely usable fee authority are therefore different numbers while in flight.
  const feeEncumbered = state.phase === 'in-flight' ? p.fillFee : 0;
  return Object.freeze({
    exposure: a.grossDebit + a.reserved,
    grossRemaining: p.grossCap - a.grossDebit - a.reserved,
    feeRemaining: p.feeCap - a.fees,
    feeEncumbered,
    feeFree: p.feeCap - a.fees - feeEncumbered,
    goalMet: a.confirmedReceipt >= p.minReceive,
    complete: state.phase === 'complete',
    openDuties: state.duties.filter((d) => d.status !== 'discharged'),
  });
}

/** Which events the explorer may offer next. Used by UI and tests. */
export function availableEvents(state) {
  const out = [];
  if (state.phase === 'uncommitted') {
    if (!state.duties.some((d) => d.id === 'deliver-remaining')) out.push('provider-accepts-duties');
    if (state.candidateVerdict?.accepted && state.duties.some((d) => d.id === 'deliver-remaining')) out.push('commit-candidate');
  }
  // The approved story: first fill finalized locally, second fill submitted
  // externally. After an authenticated failure the reservation control stays
  // offered so the reader can see the cap refusal; the reducer refuses it.
  if (state.phase === 'committed' && state.fillsDone === 0) out.push('finalize-fill');
  if (state.phase === 'committed' && state.fillsDone === 1) out.push('reserve-fill');
  if (state.phase === 'in-flight') {
    if (state.attempt?.status === 'submitted') out.push('timeout');
    else out.push('observe');
    out.push('second-solver-reserve', 'premature-refund');
  }
  if (state.phase === 'observed' && !state.attempt?.accepted && unmetConditions(state) === null) out.push('accept-observed-fill');
  if (state.consumed.length > 0) out.push('replay-fill');
  if (state.attempt?.status === 'resolved') out.push('duplicate-observation');
  return Object.freeze(out);
}

/** Named presets that produce the static transcripts and the walkthrough. */
export const PRESETS = Object.freeze({
  success: [
    { type: 'select-candidate', candidateId: 'route-direct' },
    { type: 'provider-accepts-duties' },
    { type: 'commit-candidate' },
    { type: 'present-evidence', evidenceId: 'fresh' },
    { type: 'finalize-fill' },
    { type: 'reserve-fill' },
    { type: 'timeout' },
    { type: 'observe', outcome: 'late-success' },
    { type: 'duplicate-observation' },
  ],
  failure: [
    { type: 'select-candidate', candidateId: 'route-direct' },
    { type: 'provider-accepts-duties' },
    { type: 'commit-candidate' },
    { type: 'present-evidence', evidenceId: 'fresh' },
    { type: 'finalize-fill' },
    { type: 'reserve-fill' },
    { type: 'timeout' },
    { type: 'observe', outcome: 'auth-failure' },
  ],
  unknown: [
    { type: 'select-candidate', candidateId: 'route-direct' },
    { type: 'provider-accepts-duties' },
    { type: 'commit-candidate' },
    { type: 'present-evidence', evidenceId: 'fresh' },
    { type: 'finalize-fill' },
    { type: 'reserve-fill' },
    { type: 'timeout' },
    { type: 'observe', outcome: 'still-unknown' },
    { type: 'premature-refund' },
  ],
});

export function run(events, state = initialState()) {
  return events.reduce((s, e) => apply(s, e), state);
}

/** Formats integer hundredths as a decimal string with no rounding. */
export function fmt(hundredths) {
  const sign = hundredths < 0 ? '-' : '';
  const n = Math.abs(hundredths);
  const whole = Math.floor(n / 100);
  const frac = n % 100;
  if (frac === 0) return `${sign}${whole}`;
  if (frac % 10 === 0) return `${sign}${whole}.${frac / 10}`;
  return `${sign}${whole}.${String(frac).padStart(2, '0')}`;
}

// --- internals ----------------------------------------------------------------

function record(state, event, entry) {
  const n = state.events.length + 1;
  return Object.freeze({
    ...state,
    events: Object.freeze([...state.events, Object.freeze({ n, type: event.type, ...entry, account: state.account })]),
    lastNote: entry.title,
    lastRejected: null,
  });
}

function withNote(state, note) {
  return Object.freeze({ ...state, lastNote: note, lastRejected: null });
}

function reject(state, event, reason) {
  return Object.freeze({ ...state, lastRejected: Object.freeze({ type: event.type, reason }), lastNote: null });
}

/**
 * Completion is a judgment about the whole obligation. The delivery duty is
 * discharged by the last accepted fill. The conditional remedy duty is
 * discharged by authenticated failure-free completion, which is exactly the
 * case where it was never triggered; a pending remedy is never cleared by a
 * later delivery.
 */
function finishIfComplete(state) {
  if (state.fillsDone < state.policy.fills) return state;
  const duties = state.duties.map((d) => {
    if (d.id === 'deliver-remaining') return Object.freeze({ ...d, status: 'discharged' });
    if (d.id === 'remedy-on-failure' && d.status === 'accepted') return Object.freeze({ ...d, status: 'discharged' });
    return d;
  });
  return { ...state, phase: 'complete', duties: Object.freeze(duties) };
}

/**
 * Accepts an observed successful external fill as a stage of the agreement,
 * exactly once. The account was already moved when the result was observed;
 * acceptance consumes the fill identifier and advances the fill count.
 */
function acceptObserved(state) {
  const fillId = `fill-${state.attempt.fill}`;
  return finishIfComplete({
    ...state,
    phase: 'committed',
    fillsDone: state.fillsDone + 1,
    attempt: Object.freeze({ ...state.attempt, accepted: true }),
    consumed: Object.freeze([...state.consumed, fillId]),
  });
}

/**
 * Names why a terminal observation cannot be applied to an attempt that has
 * already resolved. An authenticated result, once observed, is reality; a
 * later contradicting result cannot release its reservation or undo it.
 */
function terminalConflict(state, outcome) {
  const a = state.attempt;
  if (!a || a.status !== 'resolved') return null;
  if (outcome === a.outcome) return `${a.id} is already consumed by this same ${outcome === 'late-success' ? 'success' : 'failure'} result.`;
  if (a.outcome === 'late-success') return `${a.id} already resolved as authenticated success and its effects are accounted. A later ${outcome === 'auth-failure' ? 'failure' : 'unknown'} report cannot release that debit or make the receipt unhappen.`;
  return `${a.id} already resolved as authenticated failure and its identifier is consumed. A later ${outcome === 'late-success' ? 'success' : 'unknown'} report for the same attempt cannot be credited.`;
}

/** Remaining capacity, derived from the numbers rather than asserted. */
function capacityPhrase(policy, account) {
  const gross = policy.grossCap - account.grossDebit;
  const fee = policy.feeCap - account.fees;
  return `${fmt(gross)} A of ordinary capacity and ${fee === 0 ? 'no' : `${fmt(fee)} A of`} fee capacity remain.`;
}

/**
 * Both caps are checked and both failures are named. After a retained failure
 * fee, a further 5.5 A fill breaks the gross cap and the fee cap together.
 */
function capCheck(state, cost) {
  const p = state.policy;
  const a = state.account;
  const failures = [];
  if (a.grossDebit + a.reserved + cost > p.grossCap) failures.push(`spent plus reserved would reach ${fmt(a.grossDebit + a.reserved + cost)} A against the ${fmt(p.grossCap)} A gross cap`);
  if (a.fees + p.fillFee > p.feeCap) failures.push(`fees would reach ${fmt(a.fees + p.fillFee)} A against the ${fmt(p.feeCap)} A fee cap`);
  if (failures.length === 0) return null;
  return `Refused: ${failures.join(', and ')}. ${fmt(p.grossCap - a.grossDebit - a.reserved)} A of ordinary capacity and ${fmt(p.feeCap - a.fees)} A of fee capacity remain.`;
}

/** Names every unmet release condition, or returns null when all hold. */
function unmetConditions(state) {
  const missing = [];
  for (const c of CONDITIONS) if (!state.conditions[c.id]) missing.push(`${c.label} is missing.`);
  const ev = evaluateEvidence(EVIDENCE.find((e) => e.id === state.evidenceId), state.policy);
  if (ev.status !== 'met') missing.push(`Document predicate is ${ev.status}: ${ev.reason}`);
  return missing.length ? missing.join(' ') : null;
}

function phaseHint(state, base) {
  if (state.phase === 'uncommitted') return `${base} Commit a candidate first.`;
  if (state.phase === 'in-flight') return `${base} An attempt is in flight; resolve it first.`;
  if (state.phase === 'observed') return `${base} An observed fill is accounted but awaits acceptance; restore the missing predicates and accept it.`;
  if (state.phase === 'complete') return `${base} The agreement is complete.`;
  return base;
}

function ordinal(n) {
  return n === 1 ? 'First' : n === 2 ? 'Second' : `${n}th`;
}
