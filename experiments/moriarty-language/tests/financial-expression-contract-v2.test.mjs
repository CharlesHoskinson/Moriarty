import test from 'node:test';
import assert from 'node:assert/strict';
import {
  createFinancialExpressionContractV1,
  FINANCIAL_EXPRESSION_CONTRACT_V1,
} from '../src/successor/financial-expression-v1.ts';
import {
  createFinancialExpressionContractV2,
  FINANCIAL_EXPRESSION_CONTRACT_V2,
} from '../src/successor/financial-expression-v2.ts';
import { J } from './expression-v1-support.mjs';
import { N, U, B, Bin, Act, Req, Let, Write } from './financial-expression-v1-support.mjs';

const P = { kind: 'synthetic', start: '0', end: '0' };
const T = (value) => N('LitText', { value });
const state = {
  balances: [
    { party: 'Payer', asset: 'Cash', amount: '100' },
    { party: 'Lender', asset: 'Cash', amount: '0' },
  ],
  allowances: [
    { party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' },
  ],
  obligations: [{
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
  }],
  usedTransferIds: [],
  usedAllocationIds: [],
  work: { remaining: '256', spent: '0', closureReserve: '16' },
};

function schema() {
  return {
    units: ['Cash'],
    assets: ['Cash'],
    vaults: [],
    parties: [],
    recordTypes: {},
    enumTypes: {},
    variantTypes: {},
    fields: { paid: { type: ['UInt128'], writeClass: 'ordinary' } },
    args: {},
    observations: {},
    operations: {},
  };
}

function request(core, extras = {}) {
  return J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V2,
    source: '',
    core,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '100',
    ...extras,
  });
}

function bound() {
  return createFinancialExpressionContractV2(J(schema()), JSON.stringify(state));
}

function unbound() {
  return createFinancialExpressionContractV2(J(schema()));
}

test('Core /2 factory arity and contract name stay distinct from /1', () => {
  assert.equal(FINANCIAL_EXPRESSION_CONTRACT_V2, 'moriarty-financial-expression-contract/2');
  assert.notEqual(FINANCIAL_EXPRESSION_CONTRACT_V2, FINANCIAL_EXPRESSION_CONTRACT_V1);
  assert.equal(createFinancialExpressionContractV2.length, 1);
  const api = unbound();
  assert.deepEqual(Object.keys(api), ['evaluate', 'check']);
});

test('static check of a financial read does not require live state', () => {
  const core = N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') });
  const checked = unbound().check(request(core));
  assert.equal(checked.judgmentResult, 'ExpressionChecked', JSON.stringify(checked));
});

test('evaluate without financial context rejects FINANCIAL_CONTEXT_REQUIRED', () => {
  const core = N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') });
  const result = unbound().evaluate(request(core));
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, 'FINANCIAL_CONTEXT_REQUIRED');
  assert.equal('post' in result, false);
});

test('six read constructors type and reduce against validated pre-state', () => {
  const api = bound();
  const expect = (core, value, type) => {
    const result = api.evaluate(request(core));
    assert.equal(result.judgmentResult, 'ExpressionValue', JSON.stringify(result));
    assert.deepEqual(result.type, type);
    assert.equal(result.value, value);
    return result;
  };
  expect(N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }), '100', ['Quantity', [['Cash', '1']], '0']);
  expect(N('ReadPrincipal', { unit: 'Cash', identity: T('Due100') }), '100', ['Quantity', [['Cash', '1']], '0']);
  expect(N('ReadAccrued', { unit: 'Cash', identity: T('Due100') }), '0', ['Quantity', [['Cash', '1']], '0']);
  expect(N('ReadBalance', { asset: 'Cash', identity: T('Payer') }), '100', ['Amount', 'Cash']);
  expect(N('ReadAllowanceRemaining', { asset: 'Cash', identity: T('Payer') }), '100', ['Amount', 'Cash']);
  expect(N('ReadAllowanceSpent', { asset: 'Cash', identity: T('Payer') }), '0', ['Amount', 'Cash']);
});

test('a read plus Text literal costs two reductions', () => {
  const result = bound().evaluate(request(N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') })));
  assert.equal(result.judgmentResult, 'ExpressionValue');
  assert.equal(result.workRemaining, '98');
});

test('Core /1 rejects new constructors even in an unselected branch', () => {
  const core = N('Select', {
    condition: B(true),
    consequent: U(1),
    alternative: N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }),
  });
  const result = createFinancialExpressionContractV1(J(schema())).evaluate(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    source: '',
    core,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '100',
  }));
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, 'TYPE_CONSTRUCTOR');
});

test('Core /2 request remains closed and ignores caller Pre for financial reads', () => {
  const extra = unbound().evaluate(request(N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }), {
    financialPre: state,
  }));
  assert.equal(extra.status, 'Rejected');
  assert.equal(extra.code, 'INPUT_SCHEMA');

  const v1Contract = bound().evaluate(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    source: '',
    core: N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }),
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '100',
  }));
  assert.equal(v1Contract.status, 'Rejected');
  assert.equal(v1Contract.code, 'INPUT_SCHEMA');
});

test('unknown generic and non-Text identity are static', () => {
  assert.equal(unbound().check(request(N('ReadOutstanding', { unit: 'USD', identity: T('Due100') }))).code, 'TYPE_NAME');
  assert.equal(unbound().check(request(N('ReadBalance', { asset: 'USD', identity: T('Payer') }))).code, 'TYPE_NAME');
  assert.equal(unbound().check(request(N('ReadOutstanding', { unit: 'Cash', identity: U(1) }))).code, 'TYPE_MISMATCH');
});

test('runtime missing, invalid id, unit mismatch and range keep the read node', () => {
  const missing = bound().evaluate(request(N('ReadOutstanding', { unit: 'Cash', identity: T('Missing') })));
  assert.equal(missing.code, 'MISSING_OBLIGATION');
  assert.notEqual(missing.workUsed, '0');
  const invalid = bound().evaluate(request(N('ReadOutstanding', { unit: 'Cash', identity: T('') })));
  assert.equal(invalid.code, 'INVALID_IDENTIFIER');
});

test('short-circuit And/Or does not reduce the skipped read', () => {
  const core = N('Or', {
    left: B(true),
    right: N('Eq', {
      left: N('ReadOutstanding', { unit: 'Cash', identity: T('Missing') }),
      right: N('LitQuantity', { units: [['Cash', '1']], scale: '0', mantissa: '0' }),
    }),
  });
  const result = bound().evaluate(request(core));
  assert.equal(result.judgmentResult, 'ExpressionValue', JSON.stringify(result));
  assert.equal(result.value, true);
});

test('action statements still type-check every read including Ensure', () => {
  const core = Act(
    Req(N('Eq', {
      left: N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }),
      right: N('LitQuantity', { units: [['Cash', '1']], scale: '0', mantissa: '100' }),
    })),
    Let('held', N('ReadBalance', { asset: 'Cash', identity: T('Payer') })),
    Write('paid', U(0)),
  );
  const checked = unbound().check(request(core));
  assert.equal(checked.judgmentResult, 'ExpressionChecked', JSON.stringify(checked));
  const evaluated = bound().evaluate(request(core));
  assert.equal(evaluated.status, 'ExpressionPrepared', JSON.stringify(evaluated));
});

function assertExpressionAdmissionReject(result, code) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal(result.code, code, JSON.stringify(result));
  assert.deepEqual(result.span, { kind: 'synthetic', start: '0', end: '0' });
  assert.deepEqual(result.nodePath, []);
  assert.equal(result.workUsed, '0');
  assert.equal('actionIndex' in result, false, JSON.stringify(result));
  assert.equal('post' in result, false);
}

test('bound factory validates financial state itself and does not accept objects', () => {
  const objectBound = createFinancialExpressionContractV2(J(schema()), state);
  const result = objectBound.evaluate(request(N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') })));
  assert.equal(result.status, 'Rejected');
  assertExpressionAdmissionReject(result, 'INPUT_SCHEMA');
  const duplicate = JSON.parse(JSON.stringify(state));
  duplicate.obligations = [duplicate.obligations[0], duplicate.obligations[0]];
  const invalid = createFinancialExpressionContractV2(J(schema()), JSON.stringify(duplicate));
  const rejected = invalid.evaluate(request(N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') })));
  assertExpressionAdmissionReject(rejected, 'DUPLICATE');
});

test('Core /2 malformed and duplicate state use expression rejection fields, not kernel actionIndex', () => {
  const malformed = createFinancialExpressionContractV2(J(schema()), '{}').evaluate(request(U(1)));
  assertExpressionAdmissionReject(malformed, 'SCHEMA');
  const duplicate = JSON.parse(JSON.stringify(state));
  duplicate.obligations = [duplicate.obligations[0], duplicate.obligations[0]];
  const duplicated = createFinancialExpressionContractV2(J(schema()), JSON.stringify(duplicate)).evaluate(request(U(1)));
  assertExpressionAdmissionReject(duplicated, 'DUPLICATE');
});
