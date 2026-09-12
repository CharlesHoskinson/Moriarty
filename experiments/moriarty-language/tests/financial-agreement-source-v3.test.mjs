import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createFinancialAgreementSourceV3 } from '../src/successor/financial-agreement-source-v3.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE,
  formatFinancialAgreementSourceV3,
  parseFinancialAgreementSourceV3,
} from '../src/successor/financial-agreement-source-v3-frontend.ts';
import { createFinancialAgreementSourceV2 } from '../src/successor/financial-agreement-source-v2.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
  parseFinancialAgreementSourceV2,
} from '../src/successor/financial-agreement-source-v2-frontend.ts';
import { createFinancialAgreementSourceV1 } from '../src/successor/financial-agreement-source-v1.ts';
import { FINANCIAL_AGREEMENT_SOURCE_PROFILE } from '../src/successor/financial-agreement-source-frontend.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V1 } from '../src/successor/financial-expression-v1.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V2 } from '../src/successor/financial-expression-v2.ts';
import { SOURCE_KEYWORDS } from '../src/successor/frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = FINANCIAL_AGREEMENT_SOURCE_V3_PROFILE;
const P2 = FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE;
const P1 = FINANCIAL_AGREEMENT_SOURCE_PROFILE;
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const fixture = (stem) => readFileSync(example(stem), 'utf8');
const sourceText = fixture('financial-state-payment.mori');
const snapshotsText = fixture('financial-state-payment.snapshots.json');
const stateText = fixture('financial-state-payment.state.json');
const oldStateText = fixture('expression-funded-payment.state.json');

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
  return createFinancialAgreementSourceV3();
}

function utf8Offset(text, needle, from = 0) {
  const index = from < 0 ? text.lastIndexOf(needle) : text.indexOf(needle, from);
  assert.notEqual(index, -1, needle);
  return new TextEncoder().encode(text.slice(0, index)).length;
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

function snapshot(workInitial, Pre = { due: '100', paid: '0' }, Args = {
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

function withState(overrides) {
  return JSON.stringify({ ...parsedState(), ...overrides });
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

test('factory exposes /3 source plus mandatory action name and does not take schema or AST', () => {
  assert.equal(createFinancialAgreementSourceV3.length, 0);
  const api = language();
  assert.deepEqual(Object.keys(api), ['elaborate', 'check', 'evaluate']);
  assert.equal(api.elaborate.length, 1);
  assert.equal(api.check.length, 1);
  assert.equal(api.evaluate.length, 4);
  assert.equal('evaluateCore' in api, false);
  assert.equal(P, 'moriarty-financial-agreement-source/3');
  assert.equal(SOURCE_KEYWORDS.includes('outstanding'), false);
});

test('parser accepts six generic reads under the distinct /3 profile', () => {
  const program = parseFinancialAgreementSourceV3(source());
  assert.equal(program.profile.value, P);
  const actions = program.agreement.declarations.filter((d) => d.tag === 'ActionDecl');
  assert.deepEqual(actions.map((d) => d.name), ['repay', 'repay_installment', 'repay_remaining']);
  const remaining = actions[2];
  const first = remaining.statements[0];
  assert.equal(first.tag, 'Let');
  assert.equal(first.expression.tag, 'Call');
  assert.equal(first.expression.name, 'outstanding');
  assert.equal(first.expression.typeArguments[0].name, 'Cash');
});

test('elaborate and check return /3 profile and Core contract /2', () => {
  const elaborated = language().elaborate(source());
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  assert.equal(elaborated.sourceProfile, P);
  assert.equal(elaborated.contract, FINANCIAL_EXPRESSION_CONTRACT_V2);
  assert.equal(elaborated.contract, 'moriarty-financial-expression-contract/2');
  assert.notEqual(elaborated.contract, FINANCIAL_EXPRESSION_CONTRACT_V1);
  assert.deepEqual(elaborated.actions.map((item) => item.action), ['repay', 'repay_installment', 'repay_remaining']);
  assert.deepEqual(Object.keys(elaborated.actions[2].schema.args).sort(), ['allocationId', 'transferId']);
  assert.equal('nominal' in elaborated.actions[2].schema.args, false);
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
    'ReadOutstanding', 'ReadPrincipal', 'ReadAccrued',
    'ReadBalance', 'ReadAllowanceRemaining', 'ReadAllowanceSpent',
  ]) {
    if (name === 'ReadOutstanding' || name === 'ReadBalance' || name === 'ReadAllowanceRemaining') {
      assert.equal(constructors.has(name), true, name);
    }
  }
  const checked = language().check(source());
  assert.equal(checked.judgmentResult, 'SourceChecked');
  assert.equal(checked.sourceProfile, P);
  assert.equal(checked.actions.length, 3);
});

test('fixture repay 30 then 20 then remaining 50 settles with unrelated rows preserved', () => {
  const api = language();
  const first = api.evaluate(sourceText, 'repay', snapshotsText, stateText);
  assert.equal(first.status, 'FundedExpressionPrepared', JSON.stringify(first));
  assert.equal(first.post.paid, '30');
  assert.equal(first.post.due, '100');
  assert.equal(first.financialPost.obligations[0].principal, '70');
  assert.equal(first.financialPost.obligations[0].accrued, '0');
  assert.equal(first.financialPost.obligations[0].outstanding, '70');
  assert.equal(first.financialPost.obligations[0].status, 'Outstanding');
  assert.equal(first.financialPost.balances[0].amount, '70');
  assert.equal(first.financialPost.balances[1].amount, '30');
  assert.equal(first.financialPost.allowances[0].remaining, '70');
  assert.equal(first.financialPost.allowances[0].spent, '30');
  assert.deepEqual(first.financialPost.usedTransferIds, ['T1']);
  assert.deepEqual(first.financialPost.usedAllocationIds, ['Alloc1']);
  assert.equal(first.financialPost.work.closureReserve, '16');
  assert.equal(first.financialPost.work.remaining, '211');
  assert.equal(first.financialPost.work.spent, '45');
  assert.equal(first.financialPost.balances[2].party, 'Other');
  assert.equal(first.financialPost.balances[2].amount, '7');
  assert.equal(first.financialPost.allowances[1].remaining, '3');

  const second = api.evaluate(
    sourceText,
    'repay_installment',
    namedSnapshot(first.financialPost.work.remaining, first.post, { allocationId: 'Alloc2', transferId: 'T2' }),
    JSON.stringify(first.financialPost),
  );
  assert.equal(second.status, 'FundedExpressionPrepared', JSON.stringify(second));
  assert.equal(second.post.paid, '50');
  assert.equal(second.financialPost.obligations[0].principal, '50');
  assert.equal(second.financialPost.obligations[0].outstanding, '50');
  assert.equal(second.financialPost.balances[0].amount, '50');
  assert.equal(second.financialPost.allowances[0].spent, '50');
  assert.deepEqual(second.financialPost.usedTransferIds, ['T1', 'T2']);
  assert.deepEqual(second.financialPost.usedAllocationIds, ['Alloc1', 'Alloc2']);
  assert.equal(second.financialPost.balances[2].amount, '7');
  assert.equal(second.financialPost.work.remaining, '164');
  assert.equal(second.financialPost.work.spent, '92');

  const third = api.evaluate(
    sourceText,
    'repay_remaining',
    namedSnapshot(second.financialPost.work.remaining, second.post, { allocationId: 'Alloc3', transferId: 'T3' }),
    JSON.stringify(second.financialPost),
  );
  assert.equal(third.status, 'FundedExpressionPrepared', JSON.stringify(third));
  assert.equal(third.post.paid, '100');
  assert.equal(third.financialPost.obligations[0].principal, '0');
  assert.equal(third.financialPost.obligations[0].accrued, '0');
  assert.equal(third.financialPost.obligations[0].outstanding, '0');
  assert.equal(third.financialPost.obligations[0].status, 'Settled');
  assert.equal(third.financialPost.balances[0].amount, '0');
  assert.equal(third.financialPost.balances[1].amount, '100');
  assert.equal(third.financialPost.allowances[0].remaining, '0');
  assert.equal(third.financialPost.allowances[0].spent, '100');
  assert.deepEqual(third.financialPost.usedTransferIds, ['T1', 'T2', 'T3']);
  assert.deepEqual(third.financialPost.usedAllocationIds, ['Alloc1', 'Alloc2', 'Alloc3']);
  assert.equal(third.financialPost.work.closureReserve, '16');
  assert.equal(third.financialPost.work.remaining, '115');
  assert.equal(third.financialPost.work.spent, '141');
  assert.equal(third.financialPost.balances[2].amount, '7');
  assert.equal(third.financialPost.allowances[1].spent, '1');
  const initial = 256n;
  const remaining = BigInt(third.financialPost.work.remaining);
  const spent = BigInt(third.financialPost.work.spent);
  assert.equal(remaining + spent, initial);
  const firstUsed = initial - BigInt(first.financialPost.work.remaining);
  const secondUsed = BigInt(first.financialPost.work.remaining) - BigInt(second.financialPost.work.remaining);
  const thirdUsed = BigInt(second.financialPost.work.remaining) - remaining;
  assert.equal(firstUsed, BigInt(first.effects.length) + (firstUsed - BigInt(first.effects.length)));
  assert.ok(firstUsed > 2n);
  assert.ok(secondUsed > 2n);
  assert.ok(thirdUsed > 2n);

  const fourth = assertRejected(api.evaluate(
    sourceText,
    'repay_remaining',
    namedSnapshot(third.financialPost.work.remaining, third.post, { allocationId: 'Alloc4', transferId: 'T4' }),
    JSON.stringify(third.financialPost),
  ));
  assert.equal(fourth.code, 'GUARD_FAILED');
});

test('all six reads return typed kernel pre-state and never default missing to zero', () => {
  const inspect = source('', ` action inspect() {
    requires outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(100);
    requires principal<Cash>("Due100") == quantity<Units<Cash,1>,0>(100);
    requires accrued<Cash>("Due100") == quantity<Units<Cash,1>,0>(0);
    requires balance<Cash>("Payer") == amount<Cash>(100);
    requires allowance_remaining<Cash>("Payer") == amount<Cash>(100);
    requires allowance_spent<Cash>("Payer") == amount<Cash>(0);
    next.paid = pre.paid;
  } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const empty = assertRejected(language().evaluate(
    inspect,
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    stateText,
  ));
  assert.equal(empty.code, 'EMPTY_BATCH');

  const missingObligation = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { requires outstanding<Cash>("Missing") >= quantity<Units<Cash,1>,0>(0); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(missingObligation.code, 'MISSING_OBLIGATION');
  assert.equal(missingObligation.span.kind, 'source');
  assert.notEqual(missingObligation.workUsed, '0');

  const missingBalance = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { requires balance<Cash>("Missing") >= amount<Cash>(0); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(missingBalance.code, 'MISSING_BALANCE');

  const missingAllowance = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { requires allowance_remaining<Cash>("Missing") >= amount<Cash>(0); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(missingAllowance.code, 'MISSING_ALLOWANCE');
});

test('dynamic Text identities and settled zero outstanding remain readable', () => {
  const body = [
    'let id = obligationId;',
    'let remaining = outstanding<Cash>(id);',
    'requires remaining == quantity<Units<Cash,1>,0>(0);',
    'requires principal<Cash>(id) == quantity<Units<Cash,1>,0>(0);',
    'requires accrued<Cash>(id) == quantity<Units<Cash,1>,0>(0);',
    'next.paid = pre.paid;',
  ].join(' ');
  const settledSource = source('', ` action inspect(obligationId: Text) { ${body} } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const settled = withState({
    obligations: [obligation({
      principal: '0', accrued: '0', outstanding: '0', status: 'Settled',
    })],
  });
  const result = assertRejected(language().evaluate(
    settledSource,
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, { obligationId: 'Due100' }),
    settled,
  ));
  assert.equal(result.code, 'EMPTY_BATCH');
});

test('ordinary Pre/Obs cannot supply financial values', () => {
  const spoofed = language().evaluate(
    source(),
    'repay',
    canonical({
      Args: { allocationId: 'Alloc1', nominal: '30', transferId: 'T1' },
      Obs: {},
      Pre: { due: '0', paid: '0' },
      workInitial: '256',
    }),
    stateText,
  );
  assert.equal(spoofed.status, 'FundedExpressionPrepared', JSON.stringify(spoofed));
  assert.equal(spoofed.post.paid, '30');
  assert.equal(spoofed.post.due, '0');
  assert.equal(spoofed.financialPost.obligations[0].outstanding, '70');

  const extraObs = assertRejected(language().evaluate(
    source(),
    'repay',
    canonical({
      Args: { allocationId: 'Alloc1', nominal: '30', transferId: 'T1' },
      Obs: { outstanding: '1' },
      Pre: { due: '100', paid: '0' },
      workInitial: '256',
    }),
    stateText,
  ));
  assert.equal(extraObs.code, 'INPUT_SCHEMA');
});

test('invalid state is admitted before false guards and missing lookups', () => {
  const guarded = source('', ` action repay(${REPAY_PARAMS}) { requires false; let x = outstanding<Cash>("Missing"); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const duplicate = JSON.parse(stateText);
  duplicate.obligations = [duplicate.obligations[0], { ...duplicate.obligations[0] }];
  const dup = assertRejected(language().evaluate(guarded, 'repay', snapshot('256'), JSON.stringify(duplicate)));
  assert.equal(dup.code, 'DUPLICATE');
  assert.equal(dup.actionIndex, null);
  assert.equal('span' in dup, false);
  assert.equal('workUsed' in dup, false);

  const capacityItems = [];
  for (let i = 0; i < 129; i++) {
    capacityItems.push({ party: 'P' + String(i).padStart(3, 'a'), asset: 'Cash', amount: '1' });
  }
  const overflow = JSON.parse(stateText);
  overflow.balances = capacityItems;
  const cap = assertRejected(language().evaluate(guarded, 'repay', snapshot('256'), JSON.stringify(overflow)));
  assert.equal(cap.code, 'CAPACITY');
  assert.equal(cap.actionIndex, null);

  const inconsistent = JSON.parse(stateText);
  inconsistent.obligations[0].outstanding = '99';
  const inv = assertRejected(language().evaluate(guarded, 'repay', snapshot('256'), JSON.stringify(inconsistent)));
  assert.equal(inv.code, 'INVARIANT');

  const badAmount = JSON.parse(stateText);
  badAmount.balances[0].amount = '01';
  const amount = assertRejected(language().evaluate(guarded, 'repay', snapshot('256'), JSON.stringify(badAmount)));
  assert.equal(amount.code, 'INVALID_AMOUNT');

  const staleWork = JSON.parse(stateText);
  staleWork.work = { remaining: '255', spent: '0', closureReserve: '16' };
  const mismatch = assertRejected(language().evaluate(source(), 'repay', snapshot('256'), JSON.stringify(staleWork)));
  assert.equal(mismatch.code, 'WORK_MISMATCH');
});

test('malformed unselected action beats selector and runtime inputs', () => {
  const bad = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(nominal: Quantity<Units<Missing,1>,0>) { next.paid = pre.paid; }`);
  const evaluated = assertRejected(language().evaluate(bad, 'missing', '{', '{'));
  assert.equal(evaluated.code, 'TYPE_NAME');
  assert.equal(evaluated.span.kind, 'source');
});

test('unknown unit or asset is TYPE_NAME even in an unselected action', () => {
  const unknown = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { let x = outstanding<Missing>("Due100"); next.paid = pre.paid; }`);
  const result = assertRejected(language().check(unknown));
  assert.equal(result.code, 'TYPE_NAME');
  assert.equal(sourceSlice(unknown, result).includes('Missing'), true);
});

test('non-Text identity is TYPE_MISMATCH and extra type arguments reject', () => {
  const mismatch = source('', ` action repay(${REPAY_PARAMS}) { let x = outstanding<Cash>(1); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().check(mismatch)).code, 'TYPE_MISMATCH');
  const extra = source('', ` action repay(${REPAY_PARAMS}) { let x = outstanding<Cash, Cash>("Due100"); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const program = parseFinancialAgreementSourceV3(extra);
  const actions = program.agreement.declarations.filter((d) => d.tag === 'ActionDecl');
  const call = actions[0].statements[0].expression;
  assert.equal(call.tag, 'Call');
  assert.equal(call.name, 'outstanding');
  assert.equal(call.typeArguments.length, 2);
  const formatted = formatFinancialAgreementSourceV3(extra);
  assert.equal(formatFinancialAgreementSourceV3(formatted), formatted);
  assert.equal(formatted.includes('outstanding<Cash, Cash>'), true);
  assert.equal(assertRejected(language().check(extra)).code, 'SOURCE_ARITY');
});

test('canonical /3 grammar spells financialRead with typeArgs', () => {
  const grammar = readFileSync(new URL('../spec/successor/financial-agreement-source-v3-grammar.ebnf', import.meta.url), 'utf8');
  const start = grammar.indexOf('financialRead =');
  const end = grammar.indexOf('recordLiteral', start);
  const production = grammar.slice(start, end);
  assert.equal(production.includes('typeArgs'), true, production);
  assert.equal(production.includes('"<", typeArgument, ">"'), false, production);
});

test('runtime identifier and denomination failures keep the read span', () => {
  const invalid = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { let x = outstanding<Cash>(""); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(invalid.code, 'INVALID_IDENTIFIER');
  assert.equal(invalid.span.kind, 'source');
  assert.notEqual(invalid.workUsed, '0');

  const newline = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { let x = outstanding<Cash>("A\\nB"); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(newline.code, 'INVALID_IDENTIFIER');

  const long = 'A'.repeat(65);
  const tooLong = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { let x = outstanding<Cash>("${long}"); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(tooLong.code, 'INVALID_IDENTIFIER');

  const proto = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { let x = outstanding<Cash>("__proto__"); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(proto.code, 'INVALID_IDENTIFIER');

  const usd = JSON.parse(stateText);
  usd.obligations[0].denomination = 'USD';
  usd.obligations[0].settlementAsset = 'USD';
  const unit = assertRejected(language().evaluate(source(), 'repay', snapshot('256'), JSON.stringify(usd)));
  assert.equal(unit.code, 'NOMINAL_UNIT');
});

test('exact (party, asset) lookup is not first-match-by-unit', () => {
  const state = JSON.parse(stateText);
  state.balances = [
    { party: 'Payer', asset: 'Token', amount: '9' },
    { party: 'Payer', asset: 'Cash', amount: '100' },
    { party: 'Lender', asset: 'Cash', amount: '0' },
    { party: 'Other', asset: 'Cash', amount: '4' },
  ];
  state.allowances = [
    { party: 'Payer', asset: 'Token', remaining: '9', spent: '0' },
    { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
  ];
  const inspect = source('', ` action inspect() {
    requires balance<Cash>("Payer") == amount<Cash>(100);
    requires allowance_remaining<Cash>("Payer") == amount<Cash>(100);
    requires balance<Cash>("Other") == amount<Cash>(4);
    next.paid = pre.paid;
  } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const result = assertRejected(language().evaluate(
    inspect,
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    JSON.stringify(state),
  ));
  assert.equal(result.code, 'EMPTY_BATCH');
});

test('Quantity 2^127-1 reads and 2^127 rejects ARITH_RANGE; Amount permits UInt128 max', () => {
  const maxQ = String((1n << 127n) - 1n);
  const overQ = String(1n << 127n);
  const maxA = '340282366920938463463374607431768211455';
  const readable = JSON.parse(stateText);
  readable.obligations[0] = obligation({ principal: maxQ, accrued: '0', outstanding: maxQ });
  readable.balances[0].amount = maxA;
  const inspect = source('', ` action inspect() {
    requires outstanding<Cash>("Due100") == quantity<Units<Cash,1>,0>(${maxQ});
    requires balance<Cash>("Payer") == amount<Cash>(${maxA});
    next.paid = pre.paid;
  } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const ok = assertRejected(language().evaluate(
    inspect,
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    JSON.stringify(readable),
  ));
  assert.equal(ok.code, 'EMPTY_BATCH');

  const tooBig = JSON.parse(stateText);
  tooBig.obligations[0] = obligation({ principal: overQ, accrued: '0', outstanding: overQ });
  const range = assertRejected(language().evaluate(
    source('', ` action inspect() { let x = outstanding<Cash>("Due100"); next.paid = pre.paid; } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`),
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    JSON.stringify(tooBig),
  ));
  assert.equal(range.code, 'ARITH_RANGE');
});

test('short-circuit skips runtime lookup while static checking still visits the branch', () => {
  const skipped = source('', ` action repay(${REPAY_PARAMS}) {
    requires true or outstanding<Cash>("Missing") == quantity<Units<Cash,1>,0>(0);
    ${REPAY_BODY}
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  const result = language().evaluate(skipped, 'repay', snapshot('256'), stateText);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));

  const typed = source('', ` action repay(${REPAY_PARAMS}) {
    requires true or outstanding<Cash>(1) == quantity<Units<Cash,1>,0>(0);
    next.paid = pre.paid;
  } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().check(typed)).code, 'TYPE_MISMATCH');
});

test('repeated reads before and after emit see the same financial pre-state', () => {
  const body = [
    'let before = outstanding<Cash>("Due100");',
    'let payment = magnitude(nominal);',
    'requires payment > 0;',
    'next.paid = pre.paid + payment;',
    'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
    'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
    'let after = outstanding<Cash>("Due100");',
    'ensures before == after;',
    'ensures post.paid == pre.paid + payment;',
  ].join(' ');
  const result = language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { ${body} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  );
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.financialPost.obligations[0].outstanding, '70');
});

test('interest-first control discharges accrued before principal', () => {
  const interest = JSON.parse(stateText);
  interest.obligations[0] = obligation({
    principal: '80', accrued: '20', outstanding: '100', allocationRule: 'AccrualFirst',
  });
  const result = language().evaluate(source(), 'repay', snapshot('256'), JSON.stringify(interest));
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.financialPost.obligations[0].accrued, '0');
  assert.equal(result.financialPost.obligations[0].principal, '70');
  assert.equal(result.financialPost.obligations[0].outstanding, '70');
});

test('atomic funding faults publish no posts or effects', () => {
  const poor = JSON.parse(stateText);
  poor.balances[0].amount = '10';
  const balance = assertRejected(language().evaluate(source(), 'repay', snapshot('256'), JSON.stringify(poor)));
  assert.equal(balance.code, 'INSUFFICIENT_BALANCE');
  assert.equal(balance.actionIndex, 0);

  const allow = JSON.parse(stateText);
  allow.allowances[0].remaining = '10';
  const allowance = assertRejected(language().evaluate(source(), 'repay', snapshot('256'), JSON.stringify(allow)));
  assert.equal(allowance.code, 'INSUFFICIENT_ALLOWANCE');
  assert.equal(allowance.actionIndex, 0);

  const unfunded = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY.replace('amount<Cash>(payment)', 'amount<Cash>(20)')} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`),
    'repay',
    snapshot('256'),
    stateText,
  ));
  assert.equal(unfunded.code, 'INSUFFICIENT_UNALLOCATED');
  assert.equal(unfunded.actionIndex, 1);
});

test('work: E+N succeeds exactly, expression exhaustion and kernel shortfall differ', () => {
  const prepared = language().evaluate(source(), 'repay', snapshot('256'), stateText);
  assert.equal(prepared.status, 'FundedExpressionPrepared');
  const ePlusN = 256n - BigInt(prepared.financialPost.work.remaining);
  const n = BigInt(prepared.effects.length);
  const e = ePlusN - n;
  assert.equal(n, 2n);
  assert.ok(e > 0n);

  const exact = JSON.parse(stateText);
  exact.work.remaining = String(ePlusN);
  const exactSnap = snapshot(String(ePlusN));
  const ok = language().evaluate(source(), 'repay', exactSnap, JSON.stringify(exact));
  assert.equal(ok.status, 'FundedExpressionPrepared', JSON.stringify(ok));
  assert.equal(ok.financialPost.work.remaining, '0');

  const shortN = JSON.parse(stateText);
  shortN.work.remaining = String(e);
  const kernel = assertRejected(language().evaluate(source(), 'repay', snapshot(String(e)), JSON.stringify(shortN)));
  assert.equal(kernel.code, 'INSUFFICIENT_WORK');

  const shortE = JSON.parse(stateText);
  shortE.work.remaining = '1';
  const exhausted = assertRejected(language().evaluate(source(), 'repay', snapshot('1'), JSON.stringify(shortE)));
  assert.equal(exhausted.code, 'WORK_EXHAUSTED');
});

test('selected Args must match the action; extra nominal on repay_remaining rejects', () => {
  const extra = assertRejected(language().evaluate(
    source(),
    'repay_remaining',
    canonical({
      Args: { allocationId: 'Alloc3', nominal: '50', transferId: 'T3' },
      Obs: {},
      Pre: { due: '100', paid: '0' },
      workInitial: '256',
    }),
    stateText,
  ));
  assert.equal(extra.code, 'INPUT_SCHEMA');
});

test('new intrinsic names remain legal in /2 and are reserved in /3', () => {
  const v2 = `profile "${P2}"; agreement FundedPayment {${declarations()}
    action outstanding(${REPAY_PARAMS}) { requires pre.due > 0; next.paid = pre.paid; }
    action repay_installment(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }
  }`;
  const checked = createFinancialAgreementSourceV2().check(v2);
  assert.equal(checked.judgmentResult, 'SourceChecked', JSON.stringify(checked));

  const v3 = source('', ` action outstanding(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().check(v3)).code, 'SOURCE_RESERVED_NAME');

  const local = source('', ` action repay(${REPAY_PARAMS}) { let balance = magnitude(nominal); next.paid = pre.paid; } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }`);
  assert.equal(assertRejected(language().check(local)).code, 'SOURCE_RESERVED_NAME');

  const unavailable = `profile "${P2}"; agreement FundedPayment {${declarations()}
    action repay(${REPAY_PARAMS}) { let x = outstanding<Cash>("Due100"); next.paid = pre.paid; }
    action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; }
  }`;
  const v2Read = createFinancialAgreementSourceV2().check(unavailable);
  assert.equal(v2Read.status, 'Rejected');
  assert.ok(
    ['SOURCE_CALL', 'UNEXPECTED_TOKEN', 'CHAINED_COMPARISON', 'PROFILE_MISMATCH'].includes(v2Read.code)
      || String(v2Read.code).startsWith('SOURCE'),
    v2Read.code,
  );
});

test('/1 and /2 public contracts stay closed and reject /3 profile text', () => {
  const v1 = createFinancialAgreementSourceV1();
  const v2 = createFinancialAgreementSourceV2();
  assert.equal(v1.evaluate.length, 3);
  assert.equal(v2.evaluate.length, 4);
  assert.equal(assertRejected(v1.check(source())).code, 'PROFILE_MISMATCH');
  assert.equal(assertRejected(v2.check(source())).code, 'PROFILE_MISMATCH');
  assert.throws(
    () => parseFinancialAgreementSourceV2(source()),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
  assert.throws(
    () => parseFinancialAgreementSourceV3(`profile "${P2}"; agreement FundedPayment {${declarations()} action repay(${REPAY_PARAMS}) { next.paid = pre.paid; }}`),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
});

test('historical 100-work fixture remains usable by /2 and is not rewritten', () => {
  const v2Source = fixture('multiple-action-payment.mori');
  const v2Snapshots = fixture('multiple-action-payment.snapshots.json');
  const v2 = createFinancialAgreementSourceV2().evaluate(v2Source, 'repay', v2Snapshots, oldStateText);
  assert.equal(v2.status, 'FundedExpressionPrepared');
  assert.equal(JSON.parse(oldStateText).work.remaining, '100');
  assert.equal(JSON.parse(stateText).work.remaining, '256');
});

test('format is idempotent for /3', () => {
  const formatted = formatFinancialAgreementSourceV3(sourceText);
  assert.equal(formatFinancialAgreementSourceV3(formatted), formatted);
  const again = language().evaluate(formatted, 'repay', snapshotsText, stateText);
  const original = language().evaluate(sourceText, 'repay', snapshotsText, stateText);
  assert.deepEqual(again, original);
});

test('evaluate compiles fresh and does not consume returned Core or mutated state', () => {
  const api = language();
  const elaborated = api.elaborate(source());
  elaborated.actions[0].core.statements.length = 0;
  const owned = JSON.parse(stateText);
  const first = api.evaluate(source(), 'repay', snapshot('256'), stateText);
  owned.obligations[0].outstanding = '1';
  const second = api.evaluate(source(), 'repay', snapshot('256'), stateText);
  assert.equal(first.status, 'FundedExpressionPrepared');
  assert.deepEqual(first, second);
});

test('lookup positions 1 and 128 and case-sensitive identities', () => {
  const state = JSON.parse(stateText);
  const obligations = [];
  for (let i = 1; i <= 128; i++) {
    const id = i === 1 ? 'First' : i === 128 ? 'Last' : 'O' + String(i);
    obligations.push(obligation({
      id,
      principal: String(i),
      accrued: '0',
      outstanding: String(i),
    }));
  }
  state.obligations = obligations;
  const inspect = source('', ` action inspect() {
    requires outstanding<Cash>("First") == quantity<Units<Cash,1>,0>(1);
    requires outstanding<Cash>("Last") == quantity<Units<Cash,1>,0>(128);
    next.paid = pre.paid;
  } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`);
  const result = assertRejected(language().evaluate(
    inspect,
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    JSON.stringify(state),
  ));
  assert.equal(result.code, 'EMPTY_BATCH');
  const missingCase = assertRejected(language().evaluate(
    source('', ` action inspect() { let x = outstanding<Cash>("due100"); next.paid = pre.paid; } action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }`),
    'inspect',
    namedSnapshot('256', { due: '100', paid: '0' }, {}),
    stateText,
  ));
  assert.equal(missingCase.code, 'MISSING_OBLIGATION');
});
