import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE,
  formatFinancialAgreementSourceV5,
  parseFinancialAgreementSourceV5,
} from '../src/successor/financial-agreement-source-v5-frontend.ts';
import { createFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4.ts';
import { FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE } from '../src/successor/financial-agreement-source-v4-frontend.ts';
import {
  FINANCIAL_EXPRESSION_CONTRACT_V4,
  createFinancialExpressionContractV4,
} from '../src/successor/financial-expression-v4.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V3 } from '../src/successor/financial-expression-v3.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';
import {
  prepareFinancialLifecycle,
  LIFECYCLE_VERSION,
  LIFECYCLE_STATE_VERSION,
} from '../src/successor/financial-lifecycle.ts';

const P = FINANCIAL_AGREEMENT_SOURCE_V5_PROFILE;
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const fixture = (stem) => readFileSync(example(stem), 'utf8');

function declarations() {
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
  record ConversionFields {
    mantissa: UInt128;
    scale: UInt128;
    rounding: Text;
  }
  record AccrualTermsFields {
    numerator: UInt128;
    denominator: UInt128;
    rounding: Text;
    periodSeconds: UInt64;
    firstPeriodStart: UInt64;
  }
  record OriginateFields {
    obligationId: Text;
    transferId: Text;
    originationId: Text;
    debtor: Text;
    creditor: Text;
    nominalAmount: Quantity<Units<Cash,1>,0>;
    denomination: Text;
    settlementAsset: Text;
    conversion: Record<ConversionFields>;
    allocationRule: Text;
    accrualTerms: Record<AccrualTermsFields>;
    nominalLiabilityCap: Quantity<Units<Cash,1>,0>;
  }
  record AccrueFields {
    accrualId: Text;
    obligationId: Text;
    periodIndex: UInt64;
    observedTime: UInt64;
  }
  operation Transfer: TransferFields;
  operation Repay: RepayFields;
  operation Originate: OriginateFields;
  operation Accrue: AccrueFields;
  state due: UInt128;
  state paid: UInt128;`;
}

const ORIGINATE_BODY = [
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.due = pre.due + payment;',
  'emit Transfer { id: transferId, from: "Lender", to: "Borrower", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Originate { obligationId: "Loan1", transferId: transferId, originationId: originationId, debtor: "Borrower", creditor: "Lender", nominalAmount: nominal, denomination: "Cash", settlementAsset: "Cash", conversion: record<ConversionFields>{ mantissa: 1, scale: 0, rounding: "none" }, allocationRule: "AccrualFirst", accrualTerms: record<AccrualTermsFields>{ numerator: 1, denominator: 10, rounding: "floor", periodSeconds: u64(60), firstPeriodStart: u64(1000) }, nominalLiabilityCap: cap };',
  'ensures post.due == pre.due + payment;',
  'ensures post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(100);',
].join(' ');

const ACCRUE_BODY = [
  'requires magnitude(outstanding<Cash>("Loan1")) > 0;',
  'emit Accrue { accrualId: accrualId, obligationId: "Loan1", periodIndex: periodIndex, observedTime: observedTime };',
  'ensures post_accrued<Cash>("Loan1") == quantity<Units<Cash,1>,0>(10);',
].join(' ');

const REPAY_BODY = [
  'requires magnitude(outstanding<Cash>("Loan1")) > 0;',
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Borrower", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Loan1", payer: "Borrower", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
  'ensures post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(80);',
].join(' ');

function source(extra = '', actions) {
  const body = actions ?? [
    ` action originate(nominal: Quantity<Units<Cash,1>,0>, cap: Quantity<Units<Cash,1>,0>, transferId: Text, originationId: Text) { ${ORIGINATE_BODY} }`,
    ` action accrue(accrualId: Text, periodIndex: UInt64, observedTime: UInt64) { ${ACCRUE_BODY} }`,
    ` action repay(nominal: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text) { ${REPAY_BODY} }`,
  ].join(' ');
  return `profile "${P}"; agreement LifecycleLoan {${declarations()}${extra}${body}}`;
}

function seedState(over = {}) {
  return {
    schemaVersion: LIFECYCLE_STATE_VERSION,
    balances: [
      { party: 'Lender', asset: 'Cash', amount: '100' },
      { party: 'Borrower', asset: 'Cash', amount: '10' },
      { party: 'Other', asset: 'Token', amount: '7' },
    ],
    allowances: [
      { party: 'Lender', asset: 'Cash', remaining: '100', spent: '0' },
      { party: 'Borrower', asset: 'Cash', remaining: '110', spent: '0' },
      { party: 'Other', asset: 'Token', remaining: '3', spent: '1' },
    ],
    obligations: [],
    usedTransferIds: [],
    usedAllocationIds: [],
    usedOriginationIds: [],
    usedAccrualIds: [],
    work: { remaining: '256', spent: '17', closureReserve: '16' },
    ...over,
  };
}

function snap(workInitial, Args, Pre = { due: '0', paid: '0' }) {
  return canonical({ Args, Obs: {}, Pre, workInitial });
}

function assertRejected(result) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal('post' in result, false);
  assert.equal('financialPost' in result, false);
  assert.equal('effects' in result, false);
  return result;
}

test('factory exposes /5 plus Core /4 and does not export a resumable prefix', () => {
  assert.equal(createFinancialAgreementSourceV5.length, 0);
  const api = createFinancialAgreementSourceV5();
  assert.deepEqual(Object.keys(api), ['elaborate', 'check', 'evaluate']);
  assert.equal(api.evaluate.length, 4);
  assert.equal('continue' in api, false);
  assert.equal(P, 'moriarty-financial-agreement-source/5');
  assert.equal(FINANCIAL_EXPRESSION_CONTRACT_V4, 'moriarty-financial-expression-contract/4');
  assert.notEqual(FINANCIAL_EXPRESSION_CONTRACT_V4, FINANCIAL_EXPRESSION_CONTRACT_V3);
});

test('parser accepts nested record constructors and u64 literals', () => {
  const program = parseFinancialAgreementSourceV5(source());
  assert.equal(program.profile.value, P);
  const formatted = formatFinancialAgreementSourceV5(source());
  assert.equal(parseFinancialAgreementSourceV5(formatted).profile.value, P);
});

test('elaborate and check bind four protected operations structurally', () => {
  const api = createFinancialAgreementSourceV5();
  const elaborated = api.elaborate(source());
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  assert.equal(elaborated.sourceProfile, P);
  assert.equal(elaborated.contract, FINANCIAL_EXPRESSION_CONTRACT_V4);
  const checked = api.check(source());
  assert.equal(checked.judgmentResult, 'SourceChecked');
  assert.deepEqual(checked.actions.map((a) => a.action), ['originate', 'accrue', 'repay']);
});

test('source consumer originates, accrues and repays through the real kernel', () => {
  const api = createFinancialAgreementSourceV5();
  const text = source();
  const originated = api.evaluate(
    text,
    'originate',
    snap('256', {
      nominal: '100', cap: '110', transferId: 'D1', originationId: 'O1',
    }),
    JSON.stringify(seedState()),
  );
  assert.equal(originated.status, 'FundedExpressionPrepared', JSON.stringify(originated));
  const kernel = prepareFinancialLifecycle(JSON.stringify({
    schemaVersion: LIFECYCLE_VERSION,
    state: seedState(),
    actions: JSON.parse(JSON.stringify(originated.effects)).map((effect) => {
      if (effect.kind === 'Transfer') return effect;
      return {
        kind: 'Originate',
        obligationId: effect.obligationId,
        transferId: effect.transferId,
        originationId: effect.originationId,
        debtor: effect.debtor,
        creditor: effect.creditor,
        nominalAmount: effect.nominalAmount,
        denomination: effect.denomination,
        settlementAsset: effect.settlementAsset,
        conversion: effect.conversion,
        allocationRule: effect.allocationRule,
        accrualTerms: effect.accrualTerms,
        nominalLiabilityCap: effect.nominalLiabilityCap,
      };
    }),
  }));
  assert.equal(kernel.status, 'Prepared');
  const { remaining, spent, closureReserve } = originated.financialPost.work;
  assert.equal(originated.financialPost.obligations[0].principal, '100');
  assert.equal(originated.financialPost.obligations[0].originationId, 'O1');
  assert.equal(closureReserve, '16');
  assert.equal(originated.workRemaining, remaining);
  const kernelUsed = BigInt(kernel.post.work.spent) - 17n;
  const sourceUsed = BigInt(spent) - 17n;
  assert.equal(sourceUsed >= kernelUsed, true);
  assert.equal(originated.financialPost.balances.find((b) => b.asset === 'Token').amount, '7');

  const accrued = api.evaluate(
    text,
    'accrue',
    snap(remaining, { accrualId: 'A1', periodIndex: '1', observedTime: '1060' }, { due: '100', paid: '0' }),
    JSON.stringify(originated.financialPost),
  );
  assert.equal(accrued.status, 'FundedExpressionPrepared', JSON.stringify(accrued));
  assert.equal(accrued.financialPost.obligations[0].accrued, '10');
  assert.equal(accrued.financialPost.obligations[0].liabilityIncurred, '110');

  const repaid = api.evaluate(
    text,
    'repay',
    snap(accrued.financialPost.work.remaining, {
      nominal: '30', transferId: 'P1', allocationId: 'R1',
    }, { due: '100', paid: '0' }),
    JSON.stringify(accrued.financialPost),
  );
  assert.equal(repaid.status, 'FundedExpressionPrepared', JSON.stringify(repaid));
  assert.equal(repaid.financialPost.obligations[0].outstanding, '80');
  assert.equal(repaid.financialPost.obligations[0].principal, '80');
  assert.equal(repaid.financialPost.obligations[0].liabilityIncurred, '110');
  assert.equal(repaid.effects[1].principalDischarged, '20');
  assert.equal(repaid.effects[1].accruedDischarged, '10');
});

test('example file check and evaluate match the API', () => {
  const text = fixture('financial-lifecycle-payment.mori');
  const state = fixture('financial-lifecycle-payment.state.json');
  const snapshots = fixture('financial-lifecycle-payment.snapshots.json');
  const api = createFinancialAgreementSourceV5();
  const checked = api.check(text);
  assert.equal(checked.judgmentResult, 'SourceChecked');
  const result = api.evaluate(text, 'originate', snapshots, state);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
});

test('structural binding rejects missing Accrue, Quantity/Amount mix and UInt128 period fields', () => {
  const api = createFinancialAgreementSourceV5();
  const missing = source().replace('operation Accrue: AccrueFields;', '');
  assert.equal(api.check(missing).code, 'OPERATION_BINDING');
  const mixed = source().replace(
    'nominalAmount: Quantity<Units<Cash,1>,0>;',
    'nominalAmount: Amount<Cash>;',
  );
  assert.equal(api.check(mixed).code, 'OPERATION_BINDING');
  const period = source().replace('periodIndex: UInt64;', 'periodIndex: UInt128;');
  assert.equal(api.check(period).code, 'OPERATION_BINDING');
});

test('Accrue against a different denomination rejects NOMINAL_UNIT', () => {
  const api = createFinancialAgreementSourceV5();
  const originated = api.evaluate(
    source(),
    'originate',
    snap('256', { nominal: '100', cap: '110', transferId: 'D1', originationId: 'O1' }),
    JSON.stringify(seedState()),
  );
  assert.equal(originated.status, 'FundedExpressionPrepared');
  const foreign = JSON.parse(JSON.stringify(originated.financialPost));
  foreign.obligations.push({
    ...foreign.obligations[0],
    id: 'OtherLoan',
    denomination: 'Token',
    originationId: 'O9',
    originationTransferId: 'D9',
  });
  foreign.usedOriginationIds.push('O9');
  foreign.usedTransferIds.push('D9');
  const accrueOther = source('', ` action accrueOther(accrualId: Text, periodIndex: UInt64, observedTime: UInt64) { emit Accrue { accrualId: accrualId, obligationId: "OtherLoan", periodIndex: periodIndex, observedTime: observedTime }; } action originate(nominal: Quantity<Units<Cash,1>,0>, cap: Quantity<Units<Cash,1>,0>, transferId: Text, originationId: Text) { ${ORIGINATE_BODY} }`);
  const rejected = assertRejected(api.evaluate(
    accrueOther,
    'accrueOther',
    snap(foreign.work.remaining, { accrualId: 'A9', periodIndex: '1', observedTime: '1060' }, { due: '100', paid: '0' }),
    JSON.stringify(foreign),
  ));
  assert.equal(rejected.code, 'NOMINAL_UNIT');
});

test('/4 profile still rejects Originate and keeps Core /3', () => {
  const v4 = createFinancialAgreementSourceV4();
  const text = source().replace(P, FINANCIAL_AGREEMENT_SOURCE_V4_PROFILE);
  const result = v4.check(text);
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, 'OPERATION_BINDING');
});

test('false ensure after origination publishes no IDs or debit', () => {
  const api = createFinancialAgreementSourceV5();
  const failing = source().replace(
    'ensures post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(100);',
    'ensures post_outstanding<Cash>("Loan1") == quantity<Units<Cash,1>,0>(0);',
  );
  const rejected = assertRejected(api.evaluate(
    failing,
    'originate',
    snap('256', { nominal: '100', cap: '110', transferId: 'D1', originationId: 'O1' }),
    JSON.stringify(seedState()),
  ));
  assert.equal(rejected.code, 'ENSURES_FAILED');
  assert.equal('effects' in rejected, false);
});
