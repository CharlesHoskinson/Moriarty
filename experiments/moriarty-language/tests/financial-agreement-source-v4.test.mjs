import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE,
  formatFinancialAgreementSourceV4,
  parseFinancialAgreementSourceV4,
} from '../src/successor/financial-agreement-source-v4-frontend.ts';
import { createFinancialAgreementSourceV3 } from '../src/successor/financial-agreement-source-v3.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
  parseFinancialAgreementSourceV3,
} from '../src/successor/financial-agreement-source-v3-frontend.ts';
import { createFinancialAgreementSourceV2 } from '../src/successor/financial-agreement-source-v2.ts';
import { FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE } from '../src/successor/financial-agreement-source-v2-frontend.ts';
import { createFinancialAgreementSourceV1 } from '../src/successor/financial-agreement-source-v1.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V2 } from '../src/successor/financial-expression-v2.ts';
import {
  FINANCIAL_EXPRESSION_CONTRACT_V3,
  createFinancialExpressionContractV3,
} from '../src/successor/financial-expression-v3.ts';
import { SOURCE_KEYWORDS } from '../src/successor/frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE;
const P3 = FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE;
const P2 = FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE;
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const fixture = (stem) => readFileSync(example(stem), 'utf8');
const sourceText = fixture('financial-postconditions-payment.mori');
const v3SourceText = fixture('financial-state-payment.mori');
const snapshotsText = fixture('financial-state-payment.snapshots.json');
const stateText = fixture('financial-state-payment.state.json');

const REPAY_BODY = [
  'requires magnitude(outstanding<Cash>("Due100")) > 0;',
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
].join(' ');

const INSTALLMENT_BODY = [
  'requires magnitude(outstanding<Cash>("Due100")) > 0;',
  'let nominal = quantity<Units<Cash,1>,0>(20);',
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
].join(' ');

const POST_ENSURES = [
  'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);',
  'ensures post_principal<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);',
  'ensures post_accrued<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);',
  'ensures post_balance<Cash>("Payer") == amount<Cash>(0);',
  'ensures post_allowance_remaining<Cash>("Payer") == amount<Cash>(0);',
  'ensures post_allowance_spent<Cash>("Payer") == amount<Cash>(100);',
].join(' ');

const REMAINING_BODY = [
  'let nominal = outstanding<Cash>("Due100");',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'requires balance<Cash>("Payer") >= amount<Cash>(payment);',
  'requires allowance_remaining<Cash>("Payer") >= amount<Cash>(payment);',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
  POST_ENSURES,
].join(' ');

const REPAY_PARAMS = 'nominal: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text';
const INSTALLMENT_PARAMS = 'transferId: Text, allocationId: Text';
const REMAINING_PARAMS = 'transferId: Text, allocationId: Text';

function declarations(extra = '') {
  return `
  unit Cash;
  asset Cash: Asset;
  record TransferFields {
    id: Text;
    from: Text;
    to: Text;
    settlementAsset: Text;
    transferAmount: Amount<Cash>;
  }
  record RepayFields {
    allocationId: Text;
    transferId: Text;
    obligationId: Text;
    payer: Text;
    nominalAmount: Quantity<Units<Cash,1>,0>;
  }
  operation Transfer: TransferFields;
  operation Repay: RepayFields;
  state due: UInt128;
  state paid: UInt128;
  ${extra}`;
}

function defaultActions() {
  return ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} } action repay_remaining(${REMAINING_PARAMS}) { ${REMAINING_BODY} }`;
}

function source(extraDecls = '', actions = defaultActions()) {
  return `profile "${P}"; agreement FundedPayment {${declarations(extraDecls)}${actions}}`;
}

function language() {
  return createFinancialAgreementSourceV4();
}

function sourceSlice(text, result) {
  const bytes = new TextEncoder().encode(text);
  return new TextDecoder().decode(bytes.subarray(Number(result.span.start), Number(result.span.end)));
}

function assertRejected(result) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal('post' in result, false, JSON.stringify(result));
  assert.equal('financialPost' in result, false, JSON.stringify(result));
  assert.equal('descriptors' in result, false, JSON.stringify(result));
  assert.equal('effects' in result, false, JSON.stringify(result));
  assert.equal('result' in result, false, JSON.stringify(result));
  return result;
}

function remainingSnapshot(workInitial, Pre = { due: '100', paid: '0' }, Args = {
  allocationId: 'Alloc1', transferId: 'T1',
}) {
  return canonical({ Args, Obs: {}, Pre, workInitial });
}

function repaySnapshot(workInitial, Pre = { due: '100', paid: '0' }, Args = {
  allocationId: 'Alloc1', nominal: '30', transferId: 'T1',
}) {
  return canonical({ Args, Obs: {}, Pre, workInitial });
}

function namedSnapshot(workInitial, Pre, Args) {
  return canonical({ Args, Obs: {}, Pre, workInitial });
}

function parsedState() {
  return JSON.parse(stateText);
}

function withWork(remaining, spent = '0', closureReserve = '16') {
  const state = parsedState();
  state.work = { remaining, spent, closureReserve };
  return JSON.stringify(state);
}

function obligation(overrides = {}) {
  return {
    id: 'Due100',
    debtor: 'Payer',
    creditor: 'Lender',
    denomination: 'Cash',
    settlementAsset: 'Cash',
    principal: '100',
    accrued: '0',
    outstanding: '100',
    allocationRule: 'AccrualFirst',
    conversion: { mantissa: '1', scale: '0', rounding: 'none' },
    status: 'Outstanding',
    ...overrides,
  };
}

function countConstructors(node) {
  let count = 0;
  const pending = [node];
  while (pending.length) {
    const value = pending.pop();
    if (value === null || typeof value !== 'object') continue;
    if (Object.hasOwn(value, 'constructor') && typeof value.constructor === 'string') count++;
    pending.push(...Object.values(value));
  }
  return count;
}

function remainingCore() {
  const elaborated = language().elaborate(source());
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  return elaborated.actions.find((item) => item.action === 'repay_remaining');
}

function prefixAndSuffixCounts(core) {
  const statements = core.statements;
  const suffixStart = statements.findIndex((n) => n.constructor === 'Ensure');
  const prefix = statements.slice(0, suffixStart);
  const suffix = statements.slice(suffixStart);
  const ordinary = suffix[0];
  const rest = suffix.slice(1);
  return {
    prefix: prefix.reduce((n, s) => n + countConstructors(s), 0),
    ordinary: countConstructors(ordinary),
    suffixRest: rest.map((s) => countConstructors(s)),
    bound: countConstructors(core),
  };
}

test('factory exposes /4 source plus mandatory action name and does not take schema or AST', () => {
  assert.equal(createFinancialAgreementSourceV4.length, 0);
  const api = language();
  assert.deepEqual(Object.keys(api), ['elaborate', 'check', 'evaluate']);
  assert.equal(api.elaborate.length, 1);
  assert.equal(api.check.length, 1);
  assert.equal(api.evaluate.length, 4);
  assert.equal('evaluateCore' in api, false);
  assert.equal('continue' in api, false);
  assert.equal(P, 'moriarty-financial-agreement-source/4');
  assert.equal(SOURCE_KEYWORDS.includes('post_outstanding'), false);
});

test('parser accepts six post reads under the distinct /4 profile', () => {
  const program = parseFinancialAgreementSourceV4(source());
  assert.equal(program.profile.value, P);
  const actions = program.agreement.declarations.filter((d) => d.tag === 'ActionDecl');
  assert.deepEqual(actions.map((d) => d.name), ['repay', 'repay_installment', 'repay_remaining']);
  const remaining = actions[2];
  const posts = remaining.postconditions.slice(1);
  assert.equal(posts.length, 6);
  assert.deepEqual(posts.map((p) => p.expression.left.name), [
    'post_outstanding', 'post_principal', 'post_accrued',
    'post_balance', 'post_allowance_remaining', 'post_allowance_spent',
  ]);
});

test('elaborate and check return /4 profile and Core contract /3', () => {
  const elaborated = language().elaborate(source());
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  assert.equal(elaborated.sourceProfile, P);
  assert.equal(elaborated.contract, FINANCIAL_EXPRESSION_CONTRACT_V3);
  assert.equal(elaborated.contract, 'moriarty-financial-expression-contract/3');
  assert.notEqual(elaborated.contract, FINANCIAL_EXPRESSION_CONTRACT_V2);
  const constructors = new Set();
  for (const action of elaborated.actions) {
    const walk = (node) => {
      if (node && typeof node === 'object') {
        if (typeof node.constructor === 'string') constructors.add(node.constructor);
        for (const value of Object.values(node)) walk(value);
      }
    };
    walk(action.core);
  }
  for (const name of [
    'ReadPostOutstanding', 'ReadPostPrincipal', 'ReadPostAccrued',
    'ReadPostBalance', 'ReadPostAllowanceRemaining', 'ReadPostAllowanceSpent',
    'ReadOutstanding', 'ReadBalance', 'ReadAllowanceRemaining',
  ]) {
    assert.equal(constructors.has(name), true, name);
  }
  const checked = language().check(source());
  assert.equal(checked.judgmentResult, 'SourceChecked');
  assert.equal(checked.sourceProfile, P);
  assert.equal(checked.actions.length, 3);
});

test('independent repay_remaining constructors are prefix 41, ordinary ensure 6, Quantity 5, Amount 6, bound 80', () => {
  const compiled = remainingCore();
  const counts = prefixAndSuffixCounts(compiled.core);
  assert.equal(counts.prefix, 41, JSON.stringify(counts));
  assert.equal(counts.ordinary, 6, JSON.stringify(counts));
  assert.deepEqual(counts.suffixRest, [5, 5, 5, 6, 6, 6], JSON.stringify(counts));
  assert.equal(counts.bound, 80);
  assert.equal(compiled.staticWorkBound, '80');
});

test('example repay_remaining settles with exact work 82 and six post observations', () => {
  const api = language();
  const result = api.evaluate(sourceText, 'repay_remaining', remainingSnapshot('256'), stateText);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.post.paid, '100');
  assert.equal(result.financialPost.obligations[0].principal, '0');
  assert.equal(result.financialPost.obligations[0].accrued, '0');
  assert.equal(result.financialPost.obligations[0].outstanding, '0');
  assert.equal(result.financialPost.obligations[0].status, 'Settled');
  assert.equal(result.financialPost.balances[0].amount, '0');
  assert.equal(result.financialPost.balances[1].amount, '100');
  assert.equal(result.financialPost.allowances[0].remaining, '0');
  assert.equal(result.financialPost.allowances[0].spent, '100');
  assert.deepEqual(result.financialPost.usedTransferIds, ['T1']);
  assert.deepEqual(result.financialPost.usedAllocationIds, ['Alloc1']);
  assert.equal(result.financialPost.work.remaining, '174');
  assert.equal(result.financialPost.work.spent, '82');
  assert.equal(result.financialPost.work.closureReserve, '16');
  assert.equal(result.financialPost.balances[2].party, 'Other');
  assert.equal(result.financialPost.balances[2].amount, '7');
  assert.equal(result.effects.length, 2);
  assert.equal(result.effects[0].kind, 'Transfer');
  assert.equal(result.effects[1].kind, 'Repayment');
});

test('spendable 82 succeeds remaining 0 reserve 16; 81 late work failure; 43 first ensure; 42 kernel', () => {
  const api = language();
  const exact = api.evaluate(source(), 'repay_remaining', remainingSnapshot('82'), withWork('82'));
  assert.equal(exact.status, 'FundedExpressionPrepared', JSON.stringify(exact));
  assert.equal(exact.financialPost.work.remaining, '0');
  assert.equal(exact.financialPost.work.spent, '82');
  assert.equal(exact.financialPost.work.closureReserve, '16');

  const late = assertRejected(api.evaluate(source(), 'repay_remaining', remainingSnapshot('81'), withWork('81')));
  assert.equal(late.code, 'WORK_EXHAUSTED');
  assert.equal(late.workUsed, '81');
  assert.equal(late.span.kind, 'source');

  const firstEnsure = assertRejected(api.evaluate(source(), 'repay_remaining', remainingSnapshot('43'), withWork('43')));
  assert.equal(firstEnsure.code, 'WORK_EXHAUSTED');
  assert.equal(firstEnsure.workUsed, '43');

  const kernel = assertRejected(api.evaluate(source(), 'repay_remaining', remainingSnapshot('42'), withWork('42')));
  assert.equal(kernel.code, 'INSUFFICIENT_WORK');
  assert.equal(kernel.actionIndex, null);
  assert.equal('workUsed' in kernel, false);
  assert.equal('span' in kernel, false);

  const alsoKernel = assertRejected(api.evaluate(source(), 'repay_remaining', remainingSnapshot('41'), withWork('41')));
  assert.equal(alsoKernel.code, 'INSUFFICIENT_WORK');
  assert.equal(alsoKernel.actionIndex, null);

  const prefix = assertRejected(api.evaluate(source(), 'repay_remaining', remainingSnapshot('40'), withWork('40')));
  assert.equal(prefix.code, 'WORK_EXHAUSTED');
  assert.equal(prefix.workUsed, '40');

  const reserved = JSON.parse(withWork('81'));
  reserved.work.closureReserve = '1000';
  const reserveCannotHelp = assertRejected(api.evaluate(
    source(), 'repay_remaining', remainingSnapshot('81'), JSON.stringify(reserved),
  ));
  assert.equal(reserveCannotHelp.code, 'WORK_EXHAUSTED');
});

test('false first financial ensure workUsed 54; false last 82; preexisting spent 17 persists', () => {
  const api = language();
  const falseFirst = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} } action repay_remaining(${REMAINING_PARAMS}) { ${REMAINING_BODY.replace(
    'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);',
    'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(1);',
  )} }`);
  const first = assertRejected(api.evaluate(falseFirst, 'repay_remaining', remainingSnapshot('256'), stateText));
  assert.equal(first.code, 'ENSURES_FAILED');
  assert.equal(first.workUsed, '54');
  assert.equal(sourceSlice(falseFirst, first).includes('post_outstanding'), true);

  const falseLast = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} } action repay_remaining(${REMAINING_PARAMS}) { ${REMAINING_BODY.replace(
    'ensures post_allowance_spent<Cash>("Payer") == amount<Cash>(100);',
    'ensures post_allowance_spent<Cash>("Payer") == amount<Cash>(99);',
  )} }`);
  const last = assertRejected(api.evaluate(falseLast, 'repay_remaining', remainingSnapshot('256'), stateText));
  assert.equal(last.code, 'ENSURES_FAILED');
  assert.equal(last.workUsed, '82');
  assert.equal(sourceSlice(falseLast, last).includes('post_allowance_spent'), true);

  const spent = api.evaluate(source(), 'repay_remaining', remainingSnapshot('256'), withWork('256', '17'));
  assert.equal(spent.status, 'FundedExpressionPrepared', JSON.stringify(spent));
  assert.equal(spent.financialPost.work.spent, '99');
  assert.equal(spent.financialPost.work.remaining, '174');
  assert.equal(spent.financialPost.work.closureReserve, '16');
});

test('unprefixed reads keep PRE meaning; locals captured before kernel stay 100', () => {
  const body = [
    'let before = outstanding<Cash>("Due100");',
    'let payment = magnitude(before);',
    'requires payment > 0;',
    'requires balance<Cash>("Payer") >= amount<Cash>(payment);',
    'requires allowance_remaining<Cash>("Payer") >= amount<Cash>(payment);',
    'next.paid = pre.paid + payment;',
    'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
    'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: before };',
    'ensures before == quantity<Units<Cash,1>,0>(100);',
    'ensures outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(100);',
    'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);',
  ].join(' ');
  const result = language().evaluate(
    source('', ` action inspect(${REMAINING_PARAMS}) { ${body} } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`),
    'inspect',
    remainingSnapshot('256'),
    stateText,
  );
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.financialPost.obligations[0].outstanding, '0');
});

test('post reads are illegal in requires, let, next, emit, including dead branches', () => {
  const cases = [
    `requires post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(100); next.paid = pre.paid;`,
    `let x = post_balance<Cash>("Payer"); next.paid = pre.paid;`,
    `next.paid = magnitude(post_principal<Cash>("Due100"));`,
    `emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: post_allowance_remaining<Cash>("Payer") };`,
    `requires true or post_accrued<Cash>("Due100") == quantity<Units<Cash,1>,0>(0); next.paid = pre.paid;`,
    `requires false ? post_allowance_spent<Cash>("Payer") == amount<Cash>(0) : true; next.paid = pre.paid;`,
  ];
  for (const body of cases) {
    const text = source('', ` action repay(${REPAY_PARAMS}) { ${body} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
    const result = assertRejected(language().check(text));
    assert.equal(result.code, 'TYPE_POST_SCOPE', body + ' ' + JSON.stringify(result));
    assert.equal(result.span.kind, 'source');
    assert.equal(result.workUsed, '0');
  }
  const unselected = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { let x = post_outstanding<Cash>("Due100"); next.paid = pre.paid; }`);
  const later = assertRejected(language().check(unselected));
  assert.equal(later.code, 'TYPE_POST_SCOPE');
  assert.equal(sourceSlice(unselected, later).includes('post_outstanding'), true);
});

test('skipped suffix post read is not charged and does not look up; static still visits', () => {
  const skipped = source('', ` action repay(${REPAY_PARAMS}) {
    ${REPAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures true or post_outstanding<Cash>("Missing") == quantity<Units<Cash,1>,0>(0);')}
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const result = language().evaluate(skipped, 'repay', repaySnapshot('256'), stateText);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  const compiled = language().elaborate(skipped);
  const repay = compiled.actions[0];
  const ensure = repay.core.statements.find((n) => n.constructor === 'Ensure');
  assert.equal(ensure.operands.condition.constructor, 'Or');
  const typedEnsure = source('', ` action repay(${REPAY_PARAMS}) {
    next.paid = pre.paid;
    emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(1) };
    ensures true or post_outstanding<Cash>(1) == quantity<Units<Cash,1>,0>(0);
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().check(typedEnsure)).code, 'TYPE_MISMATCH');
});

test('kernel failure precedes a false ensure under /4; /3 keeps ensure-before-kernel', () => {
  const poor = parsedState();
  poor.balances[0].amount = '10';
  const v4False = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY.replace(
    'ensures post.paid == pre.paid + payment;',
    'ensures false;',
  )} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const v4 = assertRejected(language().evaluate(v4False, 'repay', repaySnapshot('256'), JSON.stringify(poor)));
  assert.equal(v4.code, 'INSUFFICIENT_BALANCE');
  assert.equal(v4.actionIndex, 0);

  const v3False = v3SourceText.replace('ensures post.paid == pre.paid + payment;', 'ensures false;');
  const v3 = createFinancialAgreementSourceV3().evaluate(v3False, 'repay', snapshotsText, JSON.stringify(poor));
  assert.equal(v3.status, 'Rejected');
  assert.equal(v3.code, 'ENSURES_FAILED');
});

test('empty descriptor action with false ensure is EMPTY_BATCH before suffix', () => {
  const empty = source('', ` action inspect() { next.paid = pre.paid; ensures false; } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const result = assertRejected(language().evaluate(
    empty, 'inspect', namedSnapshot('256', { due: '100', paid: '0' }, {}), stateText,
  ));
  assert.equal(result.code, 'EMPTY_BATCH');
  assert.equal('workUsed' in result, false);
});

test('transfer-only batch is legal; post reads see created receiver and unchanged debt', () => {
  const body = [
    'next.paid = pre.paid;',
    'emit Transfer { id: transferId, from: "Payer", to: "Receiver", settlementAsset: "Cash", transferAmount: amount<Cash>(5) };',
    'ensures post_balance<Cash>("Receiver") == amount<Cash>(5);',
    'ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(100);',
    'ensures outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(100);',
  ].join(' ');
  const result = language().evaluate(
    source('', ` action send(transferId: Text) { ${body} } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`),
    'send',
    namedSnapshot('256', { due: '100', paid: '0' }, { transferId: 'T9' }),
    stateText,
  );
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  const receiver = result.financialPost.balances.find((row) => row.party === 'Receiver' && row.asset === 'Cash');
  assert.equal(receiver.amount, '5');
  assert.equal(result.financialPost.obligations[0].outstanding, '100');
  assert.equal(parsedState().balances.some((row) => row.party === 'Receiver'), false);
});

test('missing candidate post rows never equal zero', () => {
  const missingOb = source('', ` action repay(${REPAY_PARAMS}) {
    ${REPAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures post_outstanding<Cash>("Missing") == quantity<Units<Cash,1>,0>(0);')}
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const ob = assertRejected(language().evaluate(missingOb, 'repay', repaySnapshot('256'), stateText));
  assert.equal(ob.code, 'MISSING_OBLIGATION');
  assert.equal(ob.span.kind, 'source');

  const missingBal = source('', ` action repay(${REPAY_PARAMS}) {
    ${REPAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures post_balance<Cash>("Missing") == amount<Cash>(0);')}
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().evaluate(missingBal, 'repay', repaySnapshot('256'), stateText)).code, 'MISSING_BALANCE');

  const missingAllow = source('', ` action repay(${REPAY_PARAMS}) {
    ${REPAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures post_allowance_remaining<Cash>("Missing") == amount<Cash>(0);')}
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().evaluate(missingAllow, 'repay', repaySnapshot('256'), stateText)).code, 'MISSING_ALLOWANCE');
});

test('exact newline identity, invalid units, signed128 quantity overflow and full U128 Amount', () => {
  const newline = source('', ` action repay(${REPAY_PARAMS}) {
    ${REPAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures post_outstanding<Cash>("Due100\\n") == quantity<Units<Cash,1>,0>(0);')}
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const nl = assertRejected(language().evaluate(newline, 'repay', repaySnapshot('256'), stateText));
  assert.equal(nl.code, 'INVALID_IDENTIFIER');

  const unknown = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; ensures post_outstanding<Missing>("Due100") == quantity<Units<Cash,1>,0>(0); }`);
  const typeName = assertRejected(language().check(unknown));
  assert.equal(typeName.code, 'TYPE_NAME');

  const usd = parsedState();
  usd.obligations[0].denomination = 'USD';
  usd.obligations[0].settlementAsset = 'USD';
  const unit = assertRejected(language().evaluate(source(), 'repay_remaining', remainingSnapshot('256'), JSON.stringify(usd)));
  assert.equal(unit.code, 'NOMINAL_UNIT');

  const maxQ = String((1n << 127n) - 1n);
  const overQ = String(1n << 127n);
  const maxA = '340282366920938463463374607431768211455';
  const readable = parsedState();
  readable.obligations[0] = obligation({ principal: maxQ, accrued: '0', outstanding: maxQ });
  readable.balances[0].amount = maxA;
  const inspect = source('', ` action inspect() {
    next.paid = pre.paid;
    emit Transfer { id: "T1", from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(1) };
    ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(${maxQ});
    ensures post_balance<Cash>("Payer") == amount<Cash>(${(BigInt(maxA) - 1n).toString(10)});
  } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const ok = language().evaluate(
    inspect, 'inspect', namedSnapshot('256', { due: '100', paid: '0' }, {}), JSON.stringify(readable),
  );
  assert.equal(ok.status, 'FundedExpressionPrepared', JSON.stringify(ok));

  const tooBig = parsedState();
  tooBig.obligations[0] = obligation({ principal: overQ, accrued: '0', outstanding: overQ });
  const range = assertRejected(language().evaluate(
    source('', ` action inspect() {
      next.paid = pre.paid;
      emit Transfer { id: "T1", from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(1) };
      ensures post_outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);
    } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`),
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    JSON.stringify(tooBig),
  ));
  assert.equal(range.code, 'ARITH_RANGE');
});

test('complete kernel state admission and work mismatch beat suffix', () => {
  const guarded = source('', ` action repay(${REPAY_PARAMS}) { requires false; ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const duplicate = parsedState();
  duplicate.obligations = [duplicate.obligations[0], { ...duplicate.obligations[0] }];
  const dup = assertRejected(language().evaluate(guarded, 'repay', repaySnapshot('256'), JSON.stringify(duplicate)));
  assert.equal(dup.code, 'DUPLICATE');
  assert.equal(dup.actionIndex, null);
  assert.equal('span' in dup, false);

  const capacityItems = [];
  for (let i = 0; i < 129; i++) {
    capacityItems.push({ party: 'P' + String(i).padStart(3, 'a'), asset: 'Cash', amount: '1' });
  }
  const overflow = parsedState();
  overflow.balances = capacityItems;
  const cap = assertRejected(language().evaluate(guarded, 'repay', repaySnapshot('256'), JSON.stringify(overflow)));
  assert.equal(cap.code, 'CAPACITY');
  assert.equal(cap.actionIndex, null);

  const staleWork = parsedState();
  staleWork.work = { remaining: '255', spent: '0', closureReserve: '16' };
  const mismatch = assertRejected(language().evaluate(source(), 'repay_remaining', remainingSnapshot('256'), JSON.stringify(staleWork)));
  assert.equal(mismatch.code, 'WORK_MISMATCH');
  assert.equal('span' in mismatch, false);
  assert.equal('workUsed' in mismatch, false);
});

test('unselected ill-typed action beats selector and runtime inputs', () => {
  const bad = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(nominal: Quantity<Units<Missing,1>,0>) { next.paid = pre.paid; }`);
  const evaluated = assertRejected(language().evaluate(bad, 'missing', '{', '{'));
  assert.equal(evaluated.code, 'TYPE_NAME');
  assert.equal(evaluated.span.kind, 'source');
});

test('atomic funding and repay failures publish no posts, effects or new IDs', () => {
  const poor = parsedState();
  poor.balances[0].amount = '10';
  const balance = assertRejected(language().evaluate(source(), 'repay', repaySnapshot('256'), JSON.stringify(poor)));
  assert.equal(balance.code, 'INSUFFICIENT_BALANCE');
  assert.equal(balance.actionIndex, 0);

  const unfunded = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY.replace('amount<Cash>(payment)', 'amount<Cash>(20)')} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    repaySnapshot('256'),
    stateText,
  ));
  assert.equal(unfunded.code, 'INSUFFICIENT_UNALLOCATED');
  assert.equal(unfunded.actionIndex, 1);

  const noTransfer = source('', ` action repay(${REPAY_PARAMS}) {
    let payment = magnitude(nominal);
    requires payment > 0;
    next.paid = pre.paid + payment;
    emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };
    ensures post.paid == pre.paid + payment;
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const missing = assertRejected(language().evaluate(noTransfer, 'repay', repaySnapshot('256'), stateText));
  assert.equal(missing.code, 'TRANSFER_NOT_IN_STEP');

  const reused = parsedState();
  reused.usedTransferIds = ['T1'];
  const duplicate = assertRejected(language().evaluate(source(), 'repay', repaySnapshot('256'), JSON.stringify(reused)));
  assert.equal(duplicate.code, 'DUPLICATE');
});

test('old profiles keep post_* as legal identifiers and reject new constructors', () => {
  const v3Named = `profile "${P3}"; agreement FundedPayment {${declarations()}
    action post_outstanding(${REPAY_PARAMS}) { requires pre.due > 0; next.paid = pre.paid; }
    action repay_installment(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }
  }`;
  const checked = createFinancialAgreementSourceV3().check(v3Named);
  assert.equal(checked.judgmentResult, 'SourceChecked', JSON.stringify(checked));

  const v4Reserved = source('', ` action post_outstanding(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().check(v4Reserved)).code, 'SOURCE_RESERVED_NAME');

  const v3Read = createFinancialAgreementSourceV3().check(
    `profile "${P3}"; agreement FundedPayment {${declarations()} action repay(${REPAY_PARAMS}) { let x = post_outstanding<Cash>("Due100"); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }}`,
  );
  assert.equal(v3Read.status, 'Rejected');

  assert.equal(assertRejected(createFinancialAgreementSourceV3().check(source())).code, 'PROFILE_MISMATCH');
  assert.equal(assertRejected(createFinancialAgreementSourceV2().check(source())).code, 'PROFILE_MISMATCH');
  assert.equal(assertRejected(createFinancialAgreementSourceV1().check(source())).code, 'PROFILE_MISMATCH');
  assert.throws(
    () => parseFinancialAgreementSourceV3(source()),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
  assert.throws(
    () => parseFinancialAgreementSourceV4(v3SourceText),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
});

test('source, Core /3 and unchanged input retry agree; mutated elaborated Core cannot influence evaluate', () => {
  const api = language();
  const compiled = remainingCore();
  const snap = remainingSnapshot('256');
  const sourceResult = api.evaluate(source(), 'repay_remaining', snap, stateText);
  assert.equal(sourceResult.status, 'FundedExpressionPrepared');
  const parsedSnap = JSON.parse(snap);
  const core = createFinancialExpressionContractV3(canonical(compiled.schema), stateText);
  const coreResult = core.evaluate(canonical({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V3,
    source: source(),
    core: compiled.core,
    Pre: parsedSnap.Pre,
    Args: parsedSnap.Args,
    Obs: parsedSnap.Obs,
    workInitial: parsedSnap.workInitial,
  }));
  assert.deepEqual(coreResult, sourceResult);
  const again = api.evaluate(source(), 'repay_remaining', snap, stateText);
  assert.deepEqual(again, sourceResult);
  const elaborated = api.elaborate(source());
  elaborated.actions.find((item) => item.action === 'repay_remaining').core.statements.length = 0;
  const afterMutation = api.evaluate(source(), 'repay_remaining', snap, stateText);
  assert.deepEqual(afterMutation, sourceResult);
});

test('format is idempotent for /4 and preserves evaluation', () => {
  const formatted = formatFinancialAgreementSourceV4(sourceText);
  assert.equal(formatFinancialAgreementSourceV4(formatted), formatted);
  const again = language().evaluate(formatted, 'repay_remaining', remainingSnapshot('256'), stateText);
  const original = language().evaluate(sourceText, 'repay_remaining', remainingSnapshot('256'), stateText);
  assert.deepEqual(again, original);
});

test('/2 stays closed and historical 100-work fixture is not rewritten', () => {
  const v2Source = fixture('multiple-action-payment.mori');
  const v2Snapshots = fixture('multiple-action-payment.snapshots.json');
  const oldState = fixture('expression-funded-payment.state.json');
  const v2 = createFinancialAgreementSourceV2().evaluate(v2Source, 'repay', v2Snapshots, oldState);
  assert.equal(v2.status, 'FundedExpressionPrepared');
  assert.equal(JSON.parse(oldState).work.remaining, '100');
  assert.equal(P2, 'moriarty-financial-agreement-source/2');
});
