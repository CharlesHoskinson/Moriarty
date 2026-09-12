import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import {
  createFinancialExpressionContractV1,
  FINANCIAL_EXPRESSION_CONTRACT_V1,
} from '../src/successor/financial-expression-v1.ts';
import {
  createFinancialExpressionContractV2,
  FINANCIAL_EXPRESSION_CONTRACT_V2,
} from '../src/successor/financial-expression-v2.ts';
import {
  createFinancialExpressionContractV3,
  FINANCIAL_EXPRESSION_CONTRACT_V3,
} from '../src/successor/financial-expression-v3.ts';
import * as financialExpressionV1 from '../src/successor/financial-expression-v1.ts';
import { createFinancialAgreementSourceV4 } from '../src/successor/financial-agreement-source-v4.ts';
import { J } from './expression-v1-support.mjs';
import { N, U, B, Bin, Act, Req, Let, Write, Ens } from './financial-expression-v1-support.mjs';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = { kind: 'synthetic', start: '0', end: '0' };
const T = (value) => N('LitText', { value });
const Q = (mantissa) => N('LitQuantity', { units: [['Cash', '1']], scale: '0', mantissa: String(mantissa) });
const SYNTHETIC = { kind: 'synthetic', start: '0', end: '0' };
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const stateText = readFileSync(example('financial-state-payment.state.json'), 'utf8');
const sourceText = readFileSync(example('financial-postconditions-payment.mori'), 'utf8');

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
    conversion: { mantissa: '1', rounding: 'none', scale: '0' },
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
    contract: FINANCIAL_EXPRESSION_CONTRACT_V3,
    source: '',
    core,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '256',
    ...extras,
  });
}

function bound(stateJSON = JSON.stringify(state)) {
  return createFinancialExpressionContractV3(J(schema()), stateJSON);
}

function unbound() {
  return createFinancialExpressionContractV3(J(schema()));
}

function expressionEnvelope(result, code) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal(result.code, code, JSON.stringify(result));
  assert.deepEqual(result.span, SYNTHETIC);
  assert.deepEqual(result.nodePath, []);
  assert.equal(result.workUsed, '0');
  assert.equal('actionIndex' in result, false, JSON.stringify(result));
  assert.equal('post' in result, false);
  assert.equal('financialPost' in result, false);
  assert.equal('effects' in result, false);
}

test('Core /3 staging continuation is not a public module export', () => {
  assert.equal('evaluateFinancialExpressionV3Prefix' in financialExpressionV1, false);
  assert.equal('continueSuffix' in financialExpressionV1, false);
  assert.equal(typeof financialExpressionV1.createFinancialExpressionContractV3, 'function');
  assert.equal(Object.hasOwn(financialExpressionV1.createFinancialExpressionContractV3(J(schema())), 'continue'), false);
});

test('Core /3 factory arity and contract name stay distinct from /1 and /2', () => {
  assert.equal(FINANCIAL_EXPRESSION_CONTRACT_V3, 'moriarty-financial-expression-contract/3');
  assert.notEqual(FINANCIAL_EXPRESSION_CONTRACT_V3, FINANCIAL_EXPRESSION_CONTRACT_V2);
  assert.notEqual(FINANCIAL_EXPRESSION_CONTRACT_V3, FINANCIAL_EXPRESSION_CONTRACT_V1);
  assert.equal(createFinancialExpressionContractV3.length, 1);
  const api = unbound();
  assert.deepEqual(Object.keys(api), ['evaluate', 'check']);
  assert.equal('continue' in api, false);
});

test('static check of a post read in Ensure does not require live state', () => {
  const core = Act(Write('paid', U(0)), Ens(Bin('Eq', N('ReadPostOutstanding', { unit: 'Cash', identity: T('Due100') }), Q(100))));
  const checked = unbound().check(request(core));
  assert.equal(checked.judgmentResult, 'ExpressionChecked', JSON.stringify(checked));
});

test('post reads outside Ensure including dead branches reject TYPE_POST_SCOPE at the original node', () => {
  const cases = [
    N('ReadPostOutstanding', { unit: 'Cash', identity: T('Due100') }),
    Act(Let('x', N('ReadPostBalance', { asset: 'Cash', identity: T('Payer') })), Write('paid', U(0))),
    Act(Req(Bin('Eq', N('ReadPostPrincipal', { unit: 'Cash', identity: T('Due100') }), Q(0))), Write('paid', U(0))),
    Act(Write('paid', N('ReadPostAccrued', { unit: 'Cash', identity: T('Due100') }))),
    Act(Req(Bin('Or', B(true), Bin('Eq', N('ReadPostAllowanceRemaining', { asset: 'Cash', identity: T('Payer') }), N('ConstructAmount', { asset: 'Cash', value: U(0) })))), Write('paid', U(0))),
  ];
  for (const core of cases) {
    const result = unbound().check(request(core));
    assert.equal(result.status, 'Rejected', JSON.stringify(result));
    assert.equal(result.code, 'TYPE_POST_SCOPE', JSON.stringify(result));
    if (core && typeof core === 'object' && Object.hasOwn(core, 'statements')) {
      assert.notDeepEqual(result.nodePath, [], JSON.stringify(result));
    }
  }
});

test('integrated evaluate of a standalone expression rejects TYPE_ACTION_REQUIRED after static admission', () => {
  const core = Bin('Eq', N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }), Q(100));
  const checked = unbound().check(request(core));
  assert.equal(checked.judgmentResult, 'ExpressionChecked', JSON.stringify(checked));
  const result = bound().evaluate(request(core));
  expressionEnvelope(result, 'TYPE_ACTION_REQUIRED');
});

test('valid action without financial context rejects FINANCIAL_CONTEXT_REQUIRED after typing', () => {
  const core = Act(Write('paid', U(1)), Ens(B(true)));
  const checked = unbound().check(request(core));
  assert.equal(checked.judgmentResult, 'ExpressionChecked', JSON.stringify(checked));
  const result = unbound().evaluate(request(core));
  expressionEnvelope(result, 'FINANCIAL_CONTEXT_REQUIRED');
});

test('missing-context rejections own nodePath across calls and separate factories', () => {
  const core = Act(Write('paid', U(1)), Ens(B(true)));
  const req = request(core);
  const firstFactory = unbound();
  const first = firstFactory.evaluate(req);
  expressionEnvelope(first, 'FINANCIAL_CONTEXT_REQUIRED');
  const expected = structuredClone(first);
  try { first.nodePath.push('999'); } catch { /* Frozen outputs may reject mutation. */ }
  const second = firstFactory.evaluate(req);
  expressionEnvelope(second, 'FINANCIAL_CONTEXT_REQUIRED');
  assert.deepEqual(second, expected);
  assert.notEqual(second.nodePath, first.nodePath);
  const separate = unbound().evaluate(req);
  expressionEnvelope(separate, 'FINANCIAL_CONTEXT_REQUIRED');
  assert.deepEqual(separate, expected);
  assert.notEqual(separate.nodePath, first.nodePath);
});

test('invalid action Core beats missing context', () => {
  const core = Act(Write('paid', B(true)));
  const result = unbound().evaluate(request(core));
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, 'TYPE_MISMATCH');
  assert.notEqual(result.code, 'FINANCIAL_CONTEXT_REQUIRED');
});

test('closed request rejects extra fields, financialPost, callbacks and object state', () => {
  const core = Act(Write('paid', U(0)), Ens(B(true)));
  const extra = JSON.parse(request(core));
  extra.financialPost = state;
  extra.callback = 'continue';
  const injected = unbound().evaluate(J(extra));
  assert.equal(injected.status, 'Rejected');
  assert.equal(injected.code, 'INPUT_SCHEMA');

  const objectBound = createFinancialExpressionContractV3(J(schema()), state);
  const objectResult = objectBound.evaluate(request(core));
  assert.equal(objectResult.status, 'Rejected');
  assert.equal(objectResult.code, 'INPUT_SCHEMA');
  assert.equal('actionIndex' in objectResult, false);
});

test('Core /3 state-admission faults keep the kernel envelope, unlike Core /2', () => {
  const core = Act(Write('paid', U(0)), Ens(B(true)));
  const duplicate = JSON.parse(JSON.stringify(state));
  duplicate.obligations = [duplicate.obligations[0], duplicate.obligations[0]];
  const rejected = createFinancialExpressionContractV3(J(schema()), JSON.stringify(duplicate)).evaluate(request(core));
  assert.equal(rejected.status, 'Rejected');
  assert.equal(rejected.code, 'DUPLICATE');
  assert.equal(rejected.actionIndex, null);
  assert.equal('span' in rejected, false);
  assert.equal('workUsed' in rejected, false);

  const v2 = createFinancialExpressionContractV2(J(schema()), JSON.stringify(duplicate)).evaluate(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V2,
    source: '',
    core: N('ReadOutstanding', { unit: 'Cash', identity: T('Due100') }),
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '100',
  }));
  assert.equal(v2.status, 'Rejected');
  assert.equal(v2.code, 'DUPLICATE');
  assert.deepEqual(v2.span, SYNTHETIC);
  assert.equal('actionIndex' in v2, false);
});

test('Core /3 work mismatch is bare and precedes snapshot domain errors', () => {
  const core = Act(Write('paid', U(0)), Ens(B(true)));
  const mismatch = bound(JSON.stringify({
    ...state,
    work: { remaining: '255', spent: '0', closureReserve: '16' },
  })).evaluate(request(core, { Pre: { paid: 'nope' } }));
  assert.equal(mismatch.status, 'Rejected');
  assert.equal(mismatch.code, 'WORK_MISMATCH');
  assert.equal('span' in mismatch, false);
  assert.equal('workUsed' in mismatch, false);
});

function fundedSchema() {
  return {
    units: ['Cash'],
    assets: ['Cash'],
    vaults: [],
    parties: [],
    recordTypes: {
      TransferFields: {
        id: ['Text'], from: ['Text'], to: ['Text'], settlementAsset: ['Text'], transferAmount: ['Amount', 'Cash'],
      },
    },
    enumTypes: {},
    variantTypes: {},
    fields: { paid: { type: ['UInt128'], writeClass: 'ordinary' } },
    args: {},
    observations: {},
    operations: { Transfer: 'TransferFields', Repay: 'RepayFields' },
  };
}

function transferEmit() {
  return N('Emit', {
    operation: 'Transfer',
    fields: N('ConstructRecord', {
      recordType: 'TransferFields',
      fields: [
        { name: 'from', value: T('Payer') },
        { name: 'id', value: T('T1') },
        { name: 'settlementAsset', value: T('Cash') },
        { name: 'to', value: T('Lender') },
        { name: 'transferAmount', value: N('ConstructAmount', { asset: 'Cash', value: U(1) }) },
      ],
    }),
  });
}

test('Core /3 newline identity uses a full-string check; old Core /2 keeps dollar-anchor behavior', () => {
  const repayFields = {
    allocationId: ['Text'],
    nominalAmount: ['Quantity', [['Cash', '1']], '0'],
    obligationId: ['Text'],
    payer: ['Text'],
    transferId: ['Text'],
  };
  const funded = fundedSchema();
  funded.recordTypes.RepayFields = repayFields;
  const action = Act(
    Write('paid', U(1)),
    transferEmit(),
    Ens(Bin('Eq', N('ReadPostOutstanding', { unit: 'Cash', identity: T('Due100\n') }), Q(100))),
  );
  const v3 = createFinancialExpressionContractV3(J(funded), JSON.stringify(state)).evaluate(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V3,
    source: '',
    core: action,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '256',
  }));
  assert.equal(v3.status, 'Rejected', JSON.stringify(v3));
  assert.equal(v3.code, 'INVALID_IDENTIFIER');

  const v2 = createFinancialExpressionContractV2(J(schema()), JSON.stringify(state)).evaluate(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V2,
    source: '',
    core: N('ReadOutstanding', { unit: 'Cash', identity: T('Due100\n') }),
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '256',
  }));
  assert.equal(v2.status, 'Rejected', JSON.stringify(v2));
  assert.equal(v2.code, 'INVALID_IDENTIFIER');
});

test('old Core /1 and /2 reject ReadPost constructors including dead branches', () => {
  const dead = Bin('Or', B(true), Bin('Eq', N('ReadPostOutstanding', { unit: 'Cash', identity: T('Due100') }), Q(0)));
  const v1 = createFinancialExpressionContractV1(J(schema())).check(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V1,
    source: '',
    core: dead,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '0',
  }));
  assert.equal(v1.status, 'Rejected');
  assert.equal(v1.code, 'TYPE_CONSTRUCTOR');
  const v2 = createFinancialExpressionContractV2(J(schema())).check(J({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V2,
    source: '',
    core: dead,
    Pre: { paid: '0' },
    Args: {},
    Obs: {},
    workInitial: '0',
  }));
  assert.equal(v2.status, 'Rejected');
  assert.equal(v2.code, 'TYPE_CONSTRUCTOR');
});

test('source /4 and Core /3 evaluate the same remaining action to identical funded results', () => {
  const sourceApi = createFinancialAgreementSourceV4();
  const elaborated = sourceApi.elaborate(sourceText);
  const remaining = elaborated.actions.find((item) => item.action === 'repay_remaining');
  const snap = {
    Args: { allocationId: 'Alloc1', transferId: 'T1' },
    Obs: {},
    Pre: { due: '100', paid: '0' },
    workInitial: '256',
  };
  const sourceResult = sourceApi.evaluate(sourceText, 'repay_remaining', canonical(snap), stateText);
  assert.equal(sourceResult.status, 'FundedExpressionPrepared', JSON.stringify(sourceResult));
  const core = createFinancialExpressionContractV3(canonical(remaining.schema), stateText);
  const coreResult = core.evaluate(canonical({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V3,
    source: sourceText,
    core: remaining.core,
    Pre: snap.Pre,
    Args: snap.Args,
    Obs: snap.Obs,
    workInitial: snap.workInitial,
  }));
  assert.deepEqual(coreResult, sourceResult);
  assert.equal(core.check(canonical({
    contract: FINANCIAL_EXPRESSION_CONTRACT_V3,
    source: sourceText,
    core: remaining.core,
    Pre: {},
    Args: {},
    Obs: {},
    workInitial: '0',
  })).judgmentResult, 'ExpressionChecked');
});

test('Core /3 EMPTY_BATCH is not converted into ordinary-only success', () => {
  const core = Act(Write('paid', U(1)), Ens(B(false)));
  const result = bound().evaluate(request(core));
  assert.equal(result.status, 'Rejected');
  assert.equal(result.code, 'EMPTY_BATCH');
  assert.equal('post' in result, false);
  assert.equal('financialPost' in result, false);
});
