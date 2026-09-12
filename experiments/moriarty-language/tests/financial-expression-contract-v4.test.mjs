import test from 'node:test';
import assert from 'node:assert/strict';
import * as financialExpressionV1 from '../src/successor/financial-expression-v1.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V1 } from '../src/successor/financial-expression-v1.ts';
import { FINANCIAL_EXPRESSION_CONTRACT_V3 } from '../src/successor/financial-expression-v3.ts';
import {
  FINANCIAL_EXPRESSION_CONTRACT_V4,
  createFinancialExpressionContractV4,
} from '../src/successor/financial-expression-v4.ts';
import { createFinancialAgreementSourceV5 } from '../src/successor/financial-agreement-source-v5.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';
import { LIFECYCLE_STATE_VERSION } from '../src/successor/financial-lifecycle.ts';

const SYNTHETIC = { kind: 'synthetic', start: '0', end: '0' };
const J = (value) => canonical(value);

function schema() {
  return {
    units: ['Cash'],
    assets: ['Cash'],
    vaults: [],
    parties: [],
    recordTypes: {
      TransferFields: {
        id: ['Text'], from: ['Text'], to: ['Text'], settlementAsset: ['Text'], transferAmount: ['Amount', 'Cash'],
      },
      RepayFields: {
        allocationId: ['Text'],
        nominalAmount: ['Quantity', [['Cash', '1']], '0'],
        obligationId: ['Text'],
        payer: ['Text'],
        transferId: ['Text'],
      },
      ConversionFields: {
        mantissa: ['UInt128'], scale: ['UInt128'], rounding: ['Text'],
      },
      AccrualTermsFields: {
        numerator: ['UInt128'], denominator: ['UInt128'], rounding: ['Text'],
        periodSeconds: ['UInt64'], firstPeriodStart: ['UInt64'],
      },
      OriginateFields: {
        obligationId: ['Text'], transferId: ['Text'], originationId: ['Text'],
        debtor: ['Text'], creditor: ['Text'],
        nominalAmount: ['Quantity', [['Cash', '1']], '0'],
        denomination: ['Text'], settlementAsset: ['Text'],
        conversion: ['Record', 'ConversionFields'],
        allocationRule: ['Text'],
        accrualTerms: ['Record', 'AccrualTermsFields'],
        nominalLiabilityCap: ['Quantity', [['Cash', '1']], '0'],
      },
      AccrueFields: {
        accrualId: ['Text'], obligationId: ['Text'], periodIndex: ['UInt64'], observedTime: ['UInt64'],
      },
    },
    enumTypes: {},
    variantTypes: {},
    fields: { paid: { type: ['UInt128'], writeClass: 'ordinary' } },
    args: {},
    observations: {},
    operations: {
      Transfer: 'TransferFields',
      Repay: 'RepayFields',
      Originate: 'OriginateFields',
      Accrue: 'AccrueFields',
    },
  };
}

function state() {
  return {
    schemaVersion: LIFECYCLE_STATE_VERSION,
    balances: [
      { party: 'Lender', asset: 'Cash', amount: '100' },
      { party: 'Borrower', asset: 'Cash', amount: '10' },
    ],
    allowances: [
      { party: 'Lender', asset: 'Cash', remaining: '100', spent: '0' },
    ],
    obligations: [],
    usedTransferIds: [],
    usedAllocationIds: [],
    usedOriginationIds: [],
    usedAccrualIds: [],
    work: { remaining: '256', spent: '0', closureReserve: '16' },
  };
}

function N(constructor, operands) {
  return { constructor, operands, span: SYNTHETIC };
}
function T(value) { return N('LitText', { value }); }
function U(value) { return N('LitUInt', { width: '128', value: String(value) }); }
function U64(value) { return N('LitUInt', { width: '64', value: String(value) }); }
function Q(value) {
  return N('LitQuantity', { units: [['Cash', '1']], scale: '0', mantissa: String(value) });
}
function Act(...statements) {
  return { statements, span: SYNTHETIC };
}
function Write(field, value) { return N('NextWrite', { field, value }); }
function Ens(condition) { return N('Ensure', { condition }); }
function Bin(constructor, left, right) { return N(constructor, { left, right }); }
function Rec(recordType, fields) {
  return N('ConstructRecord', {
    recordType,
    fields: fields.map(([name, value]) => ({ name, value })),
  });
}

function request(core, extras = {}) {
  return J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V4,
    source: '',
    core,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '256',
    ...extras,
  });
}

test('Core /4 staging continuation is not a public module export', () => {
  assert.equal('evaluateFinancialExpressionV3Prefix' in financialExpressionV1, false);
  assert.equal('continueSuffix' in financialExpressionV1, false);
  assert.equal(typeof financialExpressionV1.createFinancialExpressionContractV4, 'function');
  assert.equal(Object.hasOwn(createFinancialExpressionContractV4(J(schema())), 'continue'), false);
});

test('Core /4 factory arity stays distinct from /3', () => {
  assert.equal(FINANCIAL_EXPRESSION_CONTRACT_V4, 'moriarty-financial-expression-contract/4');
  assert.notEqual(FINANCIAL_EXPRESSION_CONTRACT_V4, FINANCIAL_EXPRESSION_CONTRACT_V3);
  assert.notEqual(FINANCIAL_EXPRESSION_CONTRACT_V4, FINANCIAL_EXPRESSION_CONTRACT_V1);
  assert.equal(createFinancialExpressionContractV4.length, 1);
});

test('check needs no financial state; evaluate without state is FINANCIAL_CONTEXT_REQUIRED', () => {
  const core = Act(Write('paid', U(1)), Ens(N('LitBool', { value: true })));
  const unbound = createFinancialExpressionContractV4(J(schema()));
  assert.equal(unbound.check(request(core)).judgmentResult, 'ExpressionChecked');
  const result = unbound.evaluate(request(core));
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, 'FINANCIAL_CONTEXT_REQUIRED');
});

test('extra request fields including financialPost reject INPUT_SCHEMA', () => {
  const core = Act(Write('paid', U(0)), Ens(N('LitBool', { value: true })));
  const extra = JSON.parse(request(core));
  extra.financialPost = state();
  const injected = createFinancialExpressionContractV4(J(schema())).evaluate(J(extra));
  assert.equal(injected.status, 'Rejected');
  assert.equal(injected.code, 'INPUT_SCHEMA');
  const objectBound = createFinancialExpressionContractV4(J(schema()), state());
  assert.equal(objectBound.evaluate(request(core)).code, 'INPUT_SCHEMA');
});

test('integrated Core /4 originate then post-read uses lifecycle state', () => {
  const core = Act(
    Write('paid', U(100)),
    N('Emit', {
      operation: 'Transfer',
      fields: Rec('TransferFields', [
        ['id', T('D1')],
        ['from', T('Lender')],
        ['to', T('Borrower')],
        ['settlementAsset', T('Cash')],
        ['transferAmount', N('ConstructAmount', { asset: 'Cash', value: U(100) })],
      ]),
    }),
    N('Emit', {
      operation: 'Originate',
      fields: Rec('OriginateFields', [
        ['obligationId', T('Loan1')],
        ['transferId', T('D1')],
        ['originationId', T('O1')],
        ['debtor', T('Borrower')],
        ['creditor', T('Lender')],
        ['nominalAmount', Q(100)],
        ['denomination', T('Cash')],
        ['settlementAsset', T('Cash')],
        ['conversion', Rec('ConversionFields', [
          ['mantissa', U(1)], ['scale', U(0)], ['rounding', T('none')],
        ])],
        ['allocationRule', T('AccrualFirst')],
        ['accrualTerms', Rec('AccrualTermsFields', [
          ['numerator', U(1)], ['denominator', U(10)], ['rounding', T('floor')],
          ['periodSeconds', U64(60)], ['firstPeriodStart', U64(1000)],
        ])],
        ['nominalLiabilityCap', Q(110)],
      ]),
    }),
    Ens(Bin('Eq', N('ReadPostOutstanding', { unit: 'Cash', identity: T('Loan1') }), Q(100))),
  );
  const result = createFinancialExpressionContractV4(J(schema()), J(state())).evaluate(request(core));
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.financialPost.obligations[0].principal, '100');
  assert.equal(result.financialPost.schemaVersion, LIFECYCLE_STATE_VERSION);
});

test('source /5 and Core /4 share the funded result shape', () => {
  const sourceApi = createFinancialAgreementSourceV5();
  assert.equal(typeof sourceApi.evaluate, 'function');
  const core = createFinancialExpressionContractV4(J(schema()), J(state()));
  assert.equal(typeof core.evaluate, 'function');
});
