import test from 'node:test';
import assert from 'node:assert/strict';
import { prepareSource6S0Unqualified } from '../src/successor/mil4-s0-source-v6.ts';
import { parseAndLowerSource6 } from '../src/successor/financial-agreement-source-v6-frontend.ts';
import { prepareMil4S0 } from '../src/successor/mil4-s0-core-v5.ts';
import { parseFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5-frontend.ts';

const U = (1n << 128n) - 1n;
const S = (1n << 127n) - 1n;

function transfer(overrides = {}) {
  const p = {
    signedRecipient: 'Recipient', submittedRecipient: 'Recipient',
    value: '10', fee: '1', grossCap: '11', feeCap: '1', netFloor: '10',
    ownerBalance: '100', recipientBalance: '0', feeBalance: '0',
    allowance: '100', spent: '0', workRemaining: '10', workSpent: '0',
    preHead: 'h0', head: 'h0', postHead: 'h1', replay: 'unused',
    includeFeeCredit: true, effectRecipient: 'Recipient',
    ...overrides,
  };
  const gross = (BigInt(p.value) + BigInt(p.fee)).toString();
  return `profile "moriarty-financial-agreement-source/6";
agreement Agreement1 {
  domain Midnight;
  settlement A scale 0;
  selected TransferLiteralFee source_hash "src1" digest "policy1";
  intent {
    signer Owner key "key1";
    nonce "n1";
    pre_head "${p.preHead}";
    valid 0..10;
    gross_cap ${p.grossCap};
    fee_cap ${p.feeCap};
    net_floor ${p.netFloor};
    failure success_only;
    signed_action transfer from Owner to ${p.signedRecipient} fee_to Fee value ${p.value} fee ${p.fee};
    observations empty;
    disclosures empty;
    retained_effects empty;
    retained_duties empty;
    delegation none;
    recovery none;
  }
  authenticated {
    head "${p.head}";
    predecessor "genesis";
    round 1;
    balance Owner ${p.ownerBalance};
    balance Recipient ${p.recipientBalance};
    balance Fee ${p.feeBalance};
    allowance Owner remaining ${p.allowance} spent ${p.spent};
    replay ${p.replay};
    work_remaining ${p.workRemaining};
    work_spent ${p.workSpent};
  }
  submit transfer from Owner to ${p.submittedRecipient} fee_to Fee value ${p.value} fee ${p.fee};
  effects {
    debit Owner ${gross};
    credit ${p.effectRecipient} ${p.value};
    ${p.includeFeeCredit ? `credit Fee ${p.fee};` : ''}
    use_allowance Owner ${gross};
    use_replay "n1";
    advance_head "${p.preHead}" "${p.postHead}";
  }
  post_head "${p.postHead}";
}`;
}
function commonTransfer(overrides = {}) {
  return transfer({ allowance: '11', workRemaining: '1', netFloor: '0', ...overrides })
    .replace('round 1;', 'round 0;');
}

function repay(overrides = {}) {
  const p = {
    amount: '30', principal: '1000', accrued: '10', outstanding: '1010',
    postPrincipal: '980', postAccrued: '0', postOutstanding: '980', postStatus: 'outstanding',
    payerBalance: '2000', creditorBalance: '0', allowance: '2000', spent: '0',
    grossCap: '30', workRemaining: '10', preHead: 'h0', head: 'h0', postHead: 'h1',
    replay: 'unused', creditAccount: 'Creditor', ...overrides,
  };
  return `profile "moriarty-financial-agreement-source/6";
agreement Agreement1 {
  domain Midnight;
  settlement A scale 0;
  selected RepayAccrualFirst source_hash "src1" digest "policy1";
  intent {
    signer Payer key "key1";
    nonce "n1";
    pre_head "${p.preHead}";
    valid 0..10;
    gross_cap ${p.grossCap};
    fee_cap 0;
    net_floor 0;
    failure success_only;
    signed_action repay obligation Loan payer Payer amount ${p.amount} conversion identity;
    observations empty;
    disclosures empty;
    retained_effects empty;
    retained_duties empty;
    delegation none;
    recovery none;
  }
  authenticated {
    head "${p.head}";
    predecessor "genesis";
    round 1;
    balance Payer ${p.payerBalance};
    balance Creditor ${p.creditorBalance};
    allowance Payer remaining ${p.allowance} spent ${p.spent};
    obligation Loan {
      debtor Payer;
      creditor Creditor;
      asset A;
      principal ${p.principal};
      accrued ${p.accrued};
      outstanding ${p.outstanding};
      status outstanding;
    }
    replay ${p.replay};
    work_remaining ${p.workRemaining};
    work_spent 0;
  }
  submit repay obligation Loan payer Payer amount ${p.amount} conversion identity;
  effects {
    debit Payer ${p.amount};
    credit ${p.creditAccount} ${p.amount};
    set_obligation Loan principal ${p.postPrincipal} accrued ${p.postAccrued} outstanding ${p.postOutstanding} status ${p.postStatus};
    use_allowance Payer ${p.amount};
    use_replay "n1";
    advance_head "${p.preHead}" "${p.postHead}";
  }
  post_head "${p.postHead}";
}`;
}

function prepared(source) {
  const result = prepareSource6S0Unqualified(source);
  assert.equal(result.status, 'PreparedUnqualified', JSON.stringify(result));
  return result.candidate;
}
function rejected(source, status, judgment, code) {
  const result = prepareSource6S0Unqualified(source);
  assert.equal(result.status, status, JSON.stringify(result));
  assert.equal(status === 'SourceRejected' ? result.code : result.rejection.code, code);
  if (judgment) {
    assert.equal(result.rejection.judgment, judgment);
    assert.equal(result.rejection.diagnosticWork, 1);
    assert.equal(result.rejection.publishedPost, null);
    assert.equal(result.rejection.publishedEffects, null);
  }
  return result;
}

test('T-10-1 preserves gross debit, fee credit, allowance, replay, head, and work', () => {
  const result = prepared(transfer());
  assert.deepEqual(result.effects.map((e) => e.kind),
    ['Debit', 'Credit', 'Credit', 'UseAllowance', 'UseReplay', 'AdvanceHead']);
  assert.deepEqual(result.effects.slice(0, 3).map((e) => e.amount), ['11', '10', '1']);
  assert.deepEqual(result.candidatePost.balances.map((v) => v.amount), ['89', '10', '1']);
  assert.deepEqual(result.candidatePost.allowances[0], { owner: 'Owner', remaining: '89', spent: '11' });
  assert.equal(result.candidatePost.head, 'h1');
  assert.equal(result.candidatePost.workRemaining, '9');
  assert.equal(result.candidatePost.consumedReplay.length, 1);
});

test('zero fee omits the fee line', () => {
  const result = prepared(transfer({ fee: '0', grossCap: '10', feeCap: '0', includeFeeCredit: false }));
  assert.equal(result.effects.length, 5);
  assert.equal(result.candidatePost.balances[2].amount, '0');
});

test('R-30 pays the bound creditor and accrued amount first', () => {
  const result = prepared(repay());
  assert.deepEqual(result.candidatePost.balances.map((v) => v.amount), ['1970', '30']);
  assert.deepEqual(result.candidatePost.obligations[0], {
    id: 'Loan', debtor: 'Payer', creditor: 'Creditor', asset: 'A',
    principal: '980', accrued: '0', outstanding: '980', status: 'Outstanding',
  });
  assert.deepEqual(result.effects.map((e) => e.kind),
    ['Debit', 'Credit', 'SetObligation', 'UseAllowance', 'UseReplay', 'AdvanceHead']);
});

test('full repay settles the obligation', () => {
  const result = prepared(repay({ amount: '1010', grossCap: '1010', postPrincipal: '0',
    postAccrued: '0', postOutstanding: '0', postStatus: 'settled' }));
  assert.equal(result.candidatePost.obligations[0].status, 'Settled');
  assert.equal(result.candidatePost.obligations[0].outstanding, '0');
});

test('R-near-bound reaches UInt128 receiver and spent limits without overflow', () => {
  const result = prepared(repay({ amount: '1', grossCap: '1', principal: (S - 1n).toString(),
    accrued: '1', outstanding: S.toString(), postPrincipal: (S - 1n).toString(),
    postAccrued: '0', postOutstanding: (S - 1n).toString(), payerBalance: '1',
    creditorBalance: (U - 1n).toString(), allowance: '1', spent: (U - 1n).toString() }));
  assert.equal(result.candidatePost.balances[1].amount, U.toString());
  assert.equal(result.candidatePost.allowances[0].spent, U.toString());
  assert.equal(result.candidatePost.obligations[0].principal, (S - 1n).toString());
});

test('a distinct signed nonce permits a later partial repayment', () => {
  const next = repay({ amount: '10', grossCap: '10', principal: '980', accrued: '0',
    outstanding: '980', postPrincipal: '970', postAccrued: '0', postOutstanding: '970',
    preHead: 'h1', head: 'h1', postHead: 'h2' }).replaceAll('"n1"', '"n2"');
  const result = prepared(next);
  assert.equal(result.candidatePost.obligations[0].outstanding, '970');
  assert.equal(result.candidatePost.head, 'h2');
});

test('recipient substitution is an Intent rejection', () => {
  rejected(transfer({ submittedRecipient: 'Other' }), 'CoreRejected', 'intent', 'S0_INTENT_SCOPE');
});
test('fee cap excess is an Intent rejection', () => {
  rejected(transfer({ fee: '2', feeCap: '1', grossCap: '12' }),
    'CoreRejected', 'intent', 'S0_INTENT_SCOPE');
});
test('missing fee credit is an Effect rejection', () => {
  rejected(transfer({ includeFeeCredit: false }), 'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});
test('an explicit zero-valued fee credit is an Effect rejection', () => {
  rejected(transfer({ fee: '0', grossCap: '10', feeCap: '0', includeFeeCredit: true }),
    'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});
test('insufficient allowance and work reject at Authority', () => {
  rejected(transfer({ allowance: '10' }), 'CoreRejected', 'authority', 'S0_AUTH_SCOPE');
  rejected(transfer({ workRemaining: '0' }), 'CoreRejected', 'authority', 'S0_AUTH_SCOPE');
});
test('stale head and consumed replay reject at History', () => {
  rejected(transfer({ preHead: 'old' }), 'CoreRejected', 'history', 'S0_HISTORY_STALE');
  rejected(transfer({ replay: 'consumed' }), 'CoreRejected', 'history', 'S0_HISTORY_REPLAY');
});
test('effect mismatch precedes stale history', () => {
  rejected(transfer({ includeFeeCredit: false, preHead: 'old' }),
    'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});
test('receiver overflow rejects at Effect', () => {
  rejected(transfer({ recipientBalance: U.toString() }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
});
test('effect range precedes an incorrect submitted vector', () => {
  rejected(transfer({ recipientBalance: U.toString(), includeFeeCredit: false }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
  rejected(repay({ amount: '31', grossCap: '31', principal: '20', accrued: '10',
    outstanding: '30', creditAccount: 'Other' }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
});
test('signed scope and caps precede a direct Core transfer alias', () => {
  const lowered = parseAndLowerSource6(transfer());
  const alias = { ...lowered.intent, recipient: lowered.intent.signer };
  const validAlias = prepareMil4S0(lowered.state, alias, lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(validAlias.status, 'Rejected');
  assert.deepEqual([validAlias.judgment, validAlias.code], ['intent', 'S0_INTENT_ALIAS']);
  const aliasWithExtraObligation = { ...lowered.state, obligations: [{
    id: 'Extra', debtor: 'Owner', creditor: 'Recipient', asset: 'A',
    principal: '1', accrued: '0', outstanding: '1', status: 'Outstanding',
  }] };
  coreReject(prepareMil4S0(aliasWithExtraObligation, alias, lowered.submittedEffects,
    lowered.proposedPostHead), 'stage', 'S0_STAGE_UNSUPPORTED');
  const badCapAlias = prepareMil4S0(lowered.state, { ...alias, feeCap: '0' },
    lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(badCapAlias.status, 'Rejected');
  assert.deepEqual([badCapAlias.judgment, badCapAlias.code], ['intent', 'S0_INTENT_SCOPE']);
  const badRoundAlias = prepareMil4S0(lowered.state, { ...alias, notAfter: '0' },
    lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(badRoundAlias.status, 'Rejected');
  assert.deepEqual([badRoundAlias.judgment, badRoundAlias.code], ['intent', 'S0_INTENT_SCOPE']);
});
test('direct Core malformed repayment bindings reject at Stage', () => {
  const lowered = parseAndLowerSource6(repay());
  const selfCredit = { ...lowered.state,
    obligations: [{ ...lowered.state.obligations[0], creditor: lowered.intent.signer }] };
  const alias = prepareMil4S0(selfCredit, lowered.intent, lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(alias.status, 'Rejected');
  assert.deepEqual([alias.judgment, alias.code], ['stage', 'S0_STAGE_UNSUPPORTED']);
  const wrongDebtor = { ...lowered.state,
    obligations: [{ ...lowered.state.obligations[0], debtor: 'Other' }] };
  const debtor = prepareMil4S0(wrongDebtor, lowered.intent, lowered.submittedEffects, lowered.proposedPostHead);
  assert.equal(debtor.status, 'Rejected');
  assert.deepEqual([debtor.judgment, debtor.code], ['stage', 'S0_STAGE_UNSUPPORTED']);
  rejected(repay().replace('creditor Creditor;', 'creditor Payer;'),
    'SourceRejected', null, 'SOURCE6_CELL_SHAPE');
});
test('source nominal bound rejects before Core preparation', () => {
  rejected(transfer({ value: (S + 1n).toString(), grossCap: S.toString() }),
    'SourceRejected', null, 'SOURCE6_RANGE');
});
test('repayment above outstanding rejects at Effect', () => {
  rejected(repay({ amount: '31', grossCap: '31', principal: '20', accrued: '10', outstanding: '30' }),
    'CoreRejected', 'effect', 'S0_EFFECT_RANGE');
});
test('Source/6 formation rejects invalid validity, cell shape, and trailing input', () => {
  rejected(transfer().replace('valid 0..10;', 'valid 10..0;'), 'SourceRejected', null, 'SOURCE6_RANGE');
  rejected(transfer().replace('balance Fee 0;', ''), 'SourceRejected', null, 'SOURCE6_CELL_SHAPE');
  rejected(`${transfer()} trailing`, 'SourceRejected', null, 'SOURCE6_SHAPE');
  rejected(transfer().replace('agreement Agreement1 {', 'agreement post_head {'),
    'SourceRejected', null, 'SOURCE6_SHAPE');
});
test('Source/5 entry rejects a Source/6 profile before accepting the body', () => {
  assert.throws(() => parseFinancialAgreementSourceV5(transfer()),
    (error) => error && error.code === 'PROFILE_MISMATCH');
});

test('Core comparison rejects missing repayment credit and wrong bound creditor', () => {
  const lowered = parseAndLowerSource6(repay());
  const missing = lowered.submittedEffects.filter((effect) => effect.kind !== 'Credit');
  const missingResult = prepareMil4S0(lowered.state, lowered.intent, missing, lowered.proposedPostHead);
  assert.equal(missingResult.status, 'Rejected');
  assert.equal(missingResult.judgment, 'effect');
  assert.equal(missingResult.code, 'S0_EFFECT_MISMATCH');
  rejected(repay({ creditAccount: 'Other' }), 'CoreRejected', 'effect', 'S0_EFFECT_MISMATCH');
});

test('common round-zero T-10-1 has exact effects and complete post-state', () => {
  const result = prepared(commonTransfer());
  const replayKey = '["Midnight","Owner","n1"]';
  assert.deepEqual(result.effects, [
    { kind: 'Debit', account: 'Owner', asset: 'A', amount: '11' },
    { kind: 'Credit', account: 'Recipient', asset: 'A', amount: '10' },
    { kind: 'Credit', account: 'Fee', asset: 'A', amount: '1' },
    { kind: 'UseAllowance', owner: 'Owner', amount: '11' },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(result.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Owner', amount: '89' }, { account: 'Recipient', amount: '10' },
      { account: 'Fee', amount: '1' }],
    allowances: [{ owner: 'Owner', remaining: '0', spent: '11' }],
    obligations: [], consumedReplay: [replayKey],
  });
});

test('common round-zero R-30 has exact effects and complete post-state', () => {
  const result = prepared(repay({ payerBalance: '100', allowance: '100', workRemaining: '1' })
    .replace('round 1;', 'round 0;'));
  const replayKey = '["Midnight","Payer","n1"]';
  assert.deepEqual(result.effects, [
    { kind: 'Debit', account: 'Payer', asset: 'A', amount: '30' },
    { kind: 'Credit', account: 'Creditor', asset: 'A', amount: '30' },
    { kind: 'SetObligation', id: 'Loan', principal: '980', accrued: '0', outstanding: '980', status: 'Outstanding' },
    { kind: 'UseAllowance', owner: 'Payer', amount: '30' },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(result.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Payer', amount: '70' }, { account: 'Creditor', amount: '30' }],
    allowances: [{ owner: 'Payer', remaining: '70', spent: '30' }],
    obligations: [{ id: 'Loan', debtor: 'Payer', creditor: 'Creditor', asset: 'A',
      principal: '980', accrued: '0', outstanding: '980', status: 'Outstanding' }],
    consumedReplay: [replayKey],
  });
});

test('common round-zero near-bound repayment has exact effects and complete post-state', () => {
  const result = prepared(repay({ amount: '1', grossCap: '1', principal: (S - 1n).toString(),
    accrued: '1', outstanding: S.toString(), postPrincipal: (S - 1n).toString(),
    postAccrued: '0', postOutstanding: (S - 1n).toString(), payerBalance: '1',
    creditorBalance: (U - 1n).toString(), allowance: '1', spent: (U - 1n).toString(),
    workRemaining: '1' }).replace('round 1;', 'round 0;'));
  const replayKey = '["Midnight","Payer","n1"]';
  assert.deepEqual(result.effects, [
    { kind: 'Debit', account: 'Payer', asset: 'A', amount: '1' },
    { kind: 'Credit', account: 'Creditor', asset: 'A', amount: '1' },
    { kind: 'SetObligation', id: 'Loan', principal: (S - 1n).toString(),
      accrued: '0', outstanding: (S - 1n).toString(), status: 'Outstanding' },
    { kind: 'UseAllowance', owner: 'Payer', amount: '1' },
    { kind: 'UseReplay', key: replayKey },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(result.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Payer', amount: '0' }, { account: 'Creditor', amount: U.toString() }],
    allowances: [{ owner: 'Payer', remaining: '0', spent: U.toString() }],
    obligations: [{ id: 'Loan', debtor: 'Payer', creditor: 'Creditor', asset: 'A',
      principal: (S - 1n).toString(), accrued: '0', outstanding: (S - 1n).toString(),
      status: 'Outstanding' }],
    consumedReplay: [replayKey],
  });
});

function localTuple(lowered, expectedSuccessor = 'h1', requestedOutcome = {
  phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [],
}) {
  return { intent: lowered.intent, state: lowered.state, round: lowered.state.round,
    expectedSuccessor, requestedOutcome };
}
function coreReject(result, judgment, code) {
  assert.deepEqual(result, { status: 'Rejected', judgment, code,
    diagnosticWork: 1, publishedPost: null, publishedEffects: null });
}
test('direct Core rejects an unknown action at Stage before a bad cap', () => {
  const x = parseAndLowerSource6(transfer());
  const unknown = { ...x.intent, kind: 'Alien', feeCap: 'bad' };
  coreReject(prepareMil4S0(x.state, unknown, x.submittedEffects, x.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
});
test('direct Core enforces closed authenticated cell order and footprint', () => {
  const t = parseAndLowerSource6(transfer());
  const reversed = { ...t.state, balances: [t.state.balances[1], t.state.balances[0], t.state.balances[2]] };
  coreReject(prepareMil4S0(reversed, t.intent, t.submittedEffects, t.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
  const extra = { ...t.state, obligations: [{ id: 'Other', debtor: 'Owner', creditor: 'Recipient',
    asset: 'A', principal: '1', accrued: '0', outstanding: '1', status: 'Outstanding' }] };
  coreReject(prepareMil4S0(extra, t.intent, t.submittedEffects, t.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
  const r = parseAndLowerSource6(repay());
  const repayExtra = { ...r.state, balances: [...r.state.balances, { account: 'Other', amount: '0' }] };
  coreReject(prepareMil4S0(repayExtra, r.intent, r.submittedEffects, r.proposedPostHead),
    'stage', 'S0_STAGE_UNSUPPORTED');
});
test('S1B exact successor and typed failure observations stay local and unqualified', () => {
  const t = parseAndLowerSource6(commonTransfer());
  const accepted = prepareMil4S0(t.state, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, localTuple(t));
  assert.equal(accepted.status, 'PreparedUnqualified');
  assert.deepEqual(accepted.effects, [
    { kind: 'Debit', account: 'Owner', asset: 'A', amount: '11' },
    { kind: 'Credit', account: 'Recipient', asset: 'A', amount: '10' },
    { kind: 'Credit', account: 'Fee', asset: 'A', amount: '1' },
    { kind: 'UseAllowance', owner: 'Owner', amount: '11' },
    { kind: 'UseReplay', key: '["Midnight","Owner","n1"]' },
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' },
  ]);
  assert.deepEqual(accepted.candidatePost, {
    core: 'moriarty-core/5', domain: 'Midnight', asset: 'A', head: 'h1', round: '0',
    workRemaining: '0', workSpent: '1',
    balances: [{ account: 'Owner', amount: '89' }, { account: 'Recipient', amount: '10' },
      { account: 'Fee', amount: '1' }],
    allowances: [{ owner: 'Owner', remaining: '0', spent: '11' }],
    obligations: [], consumedReplay: ['["Midnight","Owner","n1"]'],
  });
  assert.deepEqual(accepted.requiredPremises,
    ['canonical-intent-signature', 'snapshot-to-head', 'head-extension', 'atomic-ledger-compare-and-consume']);
  const alternate = parseAndLowerSource6(commonTransfer({ postHead: 'h9' }));
  const nonArithmetic = prepareMil4S0(alternate.state, alternate.intent, alternate.submittedEffects,
    alternate.proposedPostHead, localTuple(alternate, 'h9').requestedOutcome,
    localTuple(alternate, 'h9'));
  assert.equal(nonArithmetic.status, 'PreparedUnqualified');
  assert.equal(nonArithmetic.candidatePost.head, 'h9');
  assert.deepEqual(nonArithmetic.effects.at(-1),
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h9' });
  coreReject(prepareMil4S0(alternate.state, alternate.intent, alternate.submittedEffects,
    alternate.proposedPostHead, localTuple(alternate).requestedOutcome, localTuple(alternate)),
  'history', 'S0_HISTORY_SUCCESSOR');
  const self = parseAndLowerSource6(commonTransfer({ postHead: 'h0' }));
  coreReject(prepareMil4S0(self.state, self.intent, self.submittedEffects,
    self.proposedPostHead, localTuple(self).requestedOutcome, localTuple(self)),
  'history', 'S0_HISTORY_SUCCESSOR');
  coreReject(prepareMil4S0(self.state, self.intent, self.submittedEffects,
    self.proposedPostHead, localTuple(self, 'h0').requestedOutcome, localTuple(self, 'h0')),
  'history', 'S0_HISTORY_SUCCESSOR');
  const wrongHeadLine = t.submittedEffects.map((line) => line.kind === 'AdvanceHead'
    ? { ...line, successor: 'h9' } : line);
  coreReject(prepareMil4S0(t.state, t.intent, wrongHeadLine, 'h1',
    localTuple(t).requestedOutcome, localTuple(t)), 'effect', 'S0_EFFECT_MISMATCH');
  const stale = parseAndLowerSource6(commonTransfer({ preHead: 'h9', postHead: 'h9' }));
  coreReject(prepareMil4S0(stale.state, stale.intent, stale.submittedEffects,
    stale.proposedPostHead, localTuple(stale).requestedOutcome, localTuple(stale)),
  'history', 'S0_HISTORY_STALE');
  const replayed = { ...alternate.state, consumedReplay: [JSON.stringify(['Midnight', 'Owner', 'n1'])] };
  const replayTuple = { ...localTuple(alternate), state: replayed };
  coreReject(prepareMil4S0(replayed, alternate.intent, alternate.submittedEffects,
    alternate.proposedPostHead, replayTuple.requestedOutcome, replayTuple),
  'history', 'S0_HISTORY_REPLAY');
  const staleOnly = parseAndLowerSource6(commonTransfer({ preHead: 'h9' }));
  const staleReplayed = { ...staleOnly.state, consumedReplay: [JSON.stringify(['Midnight', 'Owner', 'n1'])] };
  const staleReplayTuple = { ...localTuple(staleOnly), state: staleReplayed };
  coreReject(prepareMil4S0(staleReplayed, staleOnly.intent, staleOnly.submittedEffects,
    staleOnly.proposedPostHead, staleReplayTuple.requestedOutcome, staleReplayTuple),
  'history', 'S0_HISTORY_STALE');
  const replayedExpected = { ...t.state, consumedReplay: [JSON.stringify(['Midnight', 'Owner', 'n1'])] };
  const replayedExpectedTuple = { ...localTuple(t), state: replayedExpected };
  coreReject(prepareMil4S0(replayedExpected, t.intent, wrongHeadLine,
    t.proposedPostHead, replayedExpectedTuple.requestedOutcome, replayedExpectedTuple),
  'effect', 'S0_EFFECT_MISMATCH');
  const wrongSuccessor = parseAndLowerSource6(commonTransfer({ postHead: 'h9' }));
  const mismatchedSuccessorLine = wrongSuccessor.submittedEffects.map((line) =>
    line.kind === 'AdvanceHead' ? { ...line, successor: 'h1' } : line);
  assert.equal(wrongSuccessor.proposedPostHead, 'h9');
  assert.deepEqual(mismatchedSuccessorLine.at(-1),
    { kind: 'AdvanceHead', predecessor: 'h0', successor: 'h1' });
  assert.equal(localTuple(wrongSuccessor).expectedSuccessor, 'h1');
  coreReject(prepareMil4S0(wrongSuccessor.state, wrongSuccessor.intent, mismatchedSuccessorLine,
    wrongSuccessor.proposedPostHead, localTuple(wrongSuccessor).requestedOutcome, localTuple(wrongSuccessor)),
  'effect', 'S0_EFFECT_MISMATCH');
  for (const outcome of [
    { phase: 'RequestedFailure', retainedEffects: [], retainedDuties: [] },
    { phase: 'TerminalSuccess', retainedEffects: [{ kind: 'Retain' }], retainedDuties: [] },
    { phase: 'TerminalSuccess', retainedEffects: [], retainedDuties: [{ kind: 'Owe' }] },
  ]) {
    coreReject(prepareMil4S0(t.state, t.intent, t.submittedEffects,
      t.proposedPostHead, outcome, localTuple(t, 'h1', outcome)),
    'failure', 'S0_FAILURE_UNSUPPORTED');
  }
  const failure = { phase: 'RequestedFailure', retainedEffects: [], retainedDuties: [] };
  coreReject(prepareMil4S0(t.state, t.intent, wrongHeadLine,
    t.proposedPostHead, failure, localTuple(t, 'h1', failure)),
  'effect', 'S0_EFFECT_MISMATCH');
  coreReject(prepareMil4S0(t.state, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, null),
  'stage', 'S0_STAGE_PREMISE');
  for (const mismatch of [
    { ...localTuple(t), intent: { ...t.intent, nonce: 'n2' } },
    { ...localTuple(t), state: { ...t.state,
      balances: [{ ...t.state.balances[0], amount: '99' }, ...t.state.balances.slice(1)] } },
    { ...localTuple(t), round: '2' },
    { ...localTuple(t), requestedOutcome: { phase: 'RequestedFailure',
      retainedEffects: [], retainedDuties: [] } },
  ]) coreReject(prepareMil4S0(t.state, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, mismatch),
  'stage', 'S0_STAGE_PREMISE');
  const malformed = { ...t.state, balances: t.state.balances.slice(1) };
  coreReject(prepareMil4S0(malformed, t.intent, t.submittedEffects,
    t.proposedPostHead, localTuple(t).requestedOutcome, null),
  'stage', 'S0_STAGE_UNSUPPORTED');
});
