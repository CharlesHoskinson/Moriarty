import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createFinancialAgreementSourceV2 } from '../src/successor/financial-agreement-source-v2.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE,
  formatFinancialAgreementSourceV2,
  parseFinancialAgreementSourceV2,
} from '../src/successor/financial-agreement-source-v2-frontend.ts';
import { createFinancialAgreementSourceV1 } from '../src/successor/financial-agreement-source-v1.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_PROFILE,
  formatFinancialAgreementSource,
  parseFinancialAgreementSource,
} from '../src/successor/financial-agreement-source-frontend.ts';
import {
  FINANCIAL_EXPRESSION_SOURCE_PROFILE,
  SOURCE_KEYWORDS,
  parseSuccessorFinancialExpressionSource,
  parseSuccessorSource,
} from '../src/successor/frontend.ts';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = FINANCIAL_AGREEMENT_SOURCE_V2_PROFILE;
const P1 = FINANCIAL_AGREEMENT_SOURCE_PROFILE;
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const fixture = (stem) => readFileSync(example(stem), 'utf8');
const snapshotsText = fixture('expression-funded-payment.snapshots.json');
const stateText = fixture('expression-funded-payment.state.json');
const sourceText = fixture('multiple-action-payment.mori');
const repaySnapshotsText = fixture('multiple-action-payment.snapshots.json');

const REPAY_BODY = [
  'requires pre.due > 0;',
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
].join(' ');

const INSTALLMENT_BODY = [
  'requires pre.due > 0;',
  'let nominal = quantity<Units<Cash,1>,0>(20);',
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
].join(' ');

const REPAY_PARAMS = 'nominal: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text';
const INSTALLMENT_PARAMS = 'transferId: Text, allocationId: Text';

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

function source(extraDecls = '', actions = defaultActions()) {
  return `profile "${P}"; agreement FundedPayment {${declarations(extraDecls)}${actions}}`;
}

function defaultActions() {
  return ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }`;
}

function isolatedV1(name, params, body) {
  return `profile "${P1}"; agreement FundedPayment {${declarations()} action ${name}(${params}) { ${body} }}`;
}

function language() {
  return createFinancialAgreementSourceV2();
}

function utf8Offset(text, needle, from = 0) {
  const index = from < 0 ? text.lastIndexOf(needle) : text.indexOf(needle, from);
  assert.notEqual(index, -1, needle);
  return new TextEncoder().encode(text.slice(0, index)).length;
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

function installmentSnapshot(workInitial, Pre, Args = {
  allocationId: 'Alloc2', transferId: 'T2',
}) {
  return canonical({ Args, Obs: {}, Pre, workInitial });
}

test('factory exposes source plus mandatory action name and does not take schema or AST', () => {
  assert.equal(createFinancialAgreementSourceV2.length, 0);
  const api = language();
  assert.deepEqual(Object.keys(api), ['elaborate', 'check', 'evaluate']);
  assert.equal(api.elaborate.length, 1);
  assert.equal(api.check.length, 1);
  assert.equal(api.evaluate.length, 4);
  assert.equal('evaluateCore' in api, false);
  assert.equal(SOURCE_KEYWORDS.includes('record'), false);
  assert.equal(SOURCE_KEYWORDS.includes('operation'), false);
});

test('parser accepts multiple actions under the distinct /2 profile', () => {
  const program = parseFinancialAgreementSourceV2(source());
  assert.equal(program.profile.value, P);
  const actions = program.agreement.declarations.filter((d) => d.tag === 'ActionDecl');
  assert.deepEqual(actions.map((d) => d.name), ['repay', 'repay_installment']);
  assert.deepEqual(actions[0].parameters.map((p) => p.name), ['nominal', 'transferId', 'allocationId']);
  assert.deepEqual(actions[1].parameters.map((p) => p.name), ['transferId', 'allocationId']);
});

test('elaborate and check return per-action artifacts in source order without an aggregate bound', () => {
  const elaborated = language().elaborate(source());
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  assert.equal(elaborated.sourceProfile, P);
  assert.equal(elaborated.agreement, 'FundedPayment');
  assert.equal('action' in elaborated, false);
  assert.equal('staticWorkBound' in elaborated, false);
  assert.equal('schema' in elaborated, false);
  assert.equal('core' in elaborated, false);
  assert.deepEqual(elaborated.actions.map((item) => item.action), ['repay', 'repay_installment']);
  assert.deepEqual(Object.keys(elaborated.actions[0].schema.args).sort(), ['allocationId', 'nominal', 'transferId']);
  assert.deepEqual(Object.keys(elaborated.actions[1].schema.args).sort(), ['allocationId', 'transferId']);
  assert.equal('nominal' in elaborated.actions[1].schema.args, false);
  assert.deepEqual(elaborated.actions[0].schema.units, ['Cash']);
  assert.deepEqual(elaborated.actions[0].schema.operations, elaborated.actions[1].schema.operations);
  assert.equal(elaborated.actions[0].schema.fields.due.writeClass, 'ordinary');
  assert.equal(elaborated.actions[0].core.span.kind, 'source');
  assert.notEqual(elaborated.actions[0].staticWorkBound, elaborated.actions[1].staticWorkBound);

  const checked = language().check(source());
  assert.equal(checked.judgmentResult, 'SourceChecked');
  assert.equal(checked.sourceProfile, P);
  assert.equal('staticWorkBound' in checked, false);
  assert.deepEqual(checked.actions.map((item) => item.action), ['repay', 'repay_installment']);
  assert.equal(checked.actions[0].staticWorkBound, elaborated.actions[0].staticWorkBound);
  assert.equal(checked.actions[1].staticWorkBound, elaborated.actions[1].staticWorkBound);
  assert.equal('schema' in checked, false);
});

test('evaluate requires a primitive selector even for one /2 action', () => {
  const one = `profile "${P}"; agreement FundedPayment {${declarations()} action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }}`;
  const missing = assertRejected(language().evaluate(one, snapshot('100'), stateText));
  assert.equal(missing.code, 'SOURCE_ACTION_NAME');
  assert.equal(missing.span.kind, 'synthetic');
  assert.deepEqual(missing.nodePath, []);
  assert.equal(missing.workUsed, '0');

  const prepared = language().evaluate(one, 'repay', snapshot('100'), stateText);
  assert.equal(prepared.status, 'FundedExpressionPrepared', JSON.stringify(prepared));
  assert.equal(prepared.post.paid, '30');
});

test('malformed selectors reject SOURCE_ACTION_NAME with a synthetic span', () => {
  const input = source();
  const cases = [
    '',
    ' ',
    '1repay',
    '__proto__',
    'repay repay',
    'a'.repeat(65),
    0,
    1,
    true,
    false,
    null,
    undefined,
    ['repay'],
    { toString() { return 'repay'; } },
    { valueOf() { return 'repay'; } },
    Object('repay'),
  ];
  for (const actionName of cases) {
    const result = assertRejected(language().evaluate(input, actionName, snapshot('100'), stateText));
    assert.equal(result.code, 'SOURCE_ACTION_NAME', JSON.stringify({ actionName, result }));
    assert.equal(result.span.kind, 'synthetic');
    assert.deepEqual(result.nodePath, []);
    assert.equal(result.workUsed, '0');
  }
  const getter = { get toString() { throw new Error('coerced'); } };
  const hostile = assertRejected(language().evaluate(input, getter, snapshot('100'), stateText));
  assert.equal(hostile.code, 'SOURCE_ACTION_NAME');
});

test('unknown, case-mismatched and prototype names reject SOURCE_ACTION_UNKNOWN', () => {
  const input = source();
  for (const actionName of ['missing', 'Repay', 'REPAY', 'constructor', 'toString', 'valueOf']) {
    const result = assertRejected(language().evaluate(input, actionName, snapshot('100'), stateText));
    assert.equal(result.code, 'SOURCE_ACTION_UNKNOWN', JSON.stringify({ actionName, result }));
    assert.equal(result.span.kind, 'synthetic');
    assert.deepEqual(result.nodePath, []);
    assert.equal(result.workUsed, '0');
    assert.equal(typeof result.code, 'string');
  }
  const named = `profile "${P}"; agreement FundedPayment {${declarations()}
    action constructor(${REPAY_PARAMS}) { ${REPAY_BODY} }
    action toString(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }
  }`;
  const constructed = language().evaluate(named, 'constructor', snapshot('100'), stateText);
  assert.equal(constructed.status, 'FundedExpressionPrepared', JSON.stringify(constructed));
  assert.equal(constructed.post.paid, '30');
  const stringed = language().evaluate(
    named,
    'toString',
    installmentSnapshot('100', { due: '100', paid: '0' }, { allocationId: 'Alloc1', transferId: 'T1' }),
    stateText,
  );
  assert.equal(stringed.status, 'FundedExpressionPrepared', JSON.stringify(stringed));
  assert.equal(stringed.post.paid, '20');
});

test('zero, duplicate and colliding actions reject at the offending declaration', () => {
  const none = `profile "${P}"; agreement FundedPayment {${declarations()}}`;
  const zero = assertRejected(language().check(none));
  assert.equal(zero.code, 'SOURCE_ACTION_COUNT');
  assert.equal(zero.span.kind, 'source');

  const duplicated = `profile "${P}"; agreement FundedPayment {${declarations()}
    action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }
    action repay(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }
  }`;
  const dup = assertRejected(language().check(duplicated));
  assert.equal(dup.code, 'SOURCE_DUPLICATE_DECLARATION');
  assert.equal(Number(dup.span.start), utf8Offset(duplicated, 'action repay(', duplicated.indexOf('action repay(') + 1));

  const collide = source('record repay { n: UInt128; }');
  assert.equal(assertRejected(language().check(collide)).code, 'SOURCE_DUPLICATE_DECLARATION');
});

test('static errors in an unselected action reject before selector handling', () => {
  const bad = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(nominal: Quantity<Units<Missing,1>,0>) { next.paid = pre.paid; }`);
  const checked = assertRejected(language().check(bad));
  assert.equal(checked.code, 'TYPE_NAME');
  assert.equal(checked.span.kind, 'source');
  assert.equal(sourceSlice(bad, checked).includes('Missing'), true);
  const evaluated = assertRejected(language().evaluate(bad, 'missing', '{', '{'));
  assert.equal(evaluated.code, 'TYPE_NAME');
  assert.equal(evaluated.span.kind, 'source');
});

test('well-typed unselected runtime guards do not execute', () => {
  const guarded = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action blocked(${INSTALLMENT_PARAMS}) { requires false; next.paid = pre.paid; }`);
  const result = language().evaluate(guarded, 'repay', snapshot('100'), stateText);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.post.paid, '30');
  const blocked = assertRejected(language().evaluate(
    guarded,
    'blocked',
    installmentSnapshot('100', { due: '100', paid: '0' }, { allocationId: 'Alloc1', transferId: 'T1' }),
    stateText,
  ));
  assert.equal(blocked.code, 'GUARD_FAILED');
});

test('parameters and locals may reuse names across actions, including different types', () => {
  const reused = `profile "${P}"; agreement FundedPayment {${declarations()}
    action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }
    action other(nominal: Text, transferId: Text, allocationId: Text) {
      requires nominal == "ok";
      next.paid = pre.paid;
    }
    action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }
  }`;
  const checked = language().check(reused);
  assert.equal(checked.judgmentResult, 'SourceChecked', JSON.stringify(checked));
  const result = language().evaluate(reused, 'repay', snapshot('100'), stateText);
  assert.equal(result.status, 'FundedExpressionPrepared');
  const other = assertRejected(language().evaluate(
    reused,
    'other',
    canonical({ Args: { allocationId: 'Alloc1', nominal: 'ok', transferId: 'T1' }, Obs: {}, Pre: { due: '100', paid: '0' }, workInitial: '100' }),
    stateText,
  ));
  assert.equal(other.code, 'EMPTY_BATCH');
});

test('references to another action binding reject during that action check', () => {
  const leak = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(${INSTALLMENT_PARAMS}) { next.paid = pre.paid + magnitude(nominal); }`);
  const result = assertRejected(language().check(leak));
  assert.equal(result.status, 'Rejected');
  assert.notEqual(result.code, 'SOURCE_ACTION_COUNT');
});

test('runtime snapshots require exactly the selected Args', () => {
  const extra = assertRejected(language().evaluate(
    source(),
    'repay_installment',
    canonical({
      Args: { allocationId: 'Alloc1', first: '10', nominal: '30', second: '20', transferId: 'T1' },
      Obs: {},
      Pre: { due: '100', paid: '0' },
      workInitial: '100',
    }),
    stateText,
  ));
  assert.equal(extra.code, 'INPUT_SCHEMA');

  const missing = assertRejected(language().evaluate(
    source(),
    'repay',
    installmentSnapshot('100', { due: '100', paid: '0' }),
    stateText,
  ));
  assert.equal(missing.code, 'INPUT_SCHEMA');
});

test('selected result and work are independent of unselected action order and extra well-typed actions', () => {
  const reversed = `profile "${P}"; agreement FundedPayment {${declarations()}
    action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }
    action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }
  }`;
  const extra = source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action noop(${INSTALLMENT_PARAMS}) { next.paid = pre.paid; } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }`);
  const expected = language().evaluate(source(), 'repay', snapshot('100'), stateText);
  const reversedResult = language().evaluate(reversed, 'repay', snapshot('100'), stateText);
  const extraResult = language().evaluate(extra, 'repay', snapshot('100'), stateText);
  assert.deepEqual(reversedResult, expected);
  assert.deepEqual(extraResult, expected);
  const isolated = createFinancialAgreementSourceV1().evaluate(
    isolatedV1('repay', REPAY_PARAMS, REPAY_BODY),
    snapshot('100'),
    stateText,
  );
  assert.deepEqual(expected, isolated);
  assert.equal(expected.workRemaining, isolated.workRemaining);
});

test('fixture repay30 then installment20 leaves50 with complete continuation projection', () => {
  const api = language();
  const first = api.evaluate(sourceText, 'repay', repaySnapshotsText, stateText);
  assert.equal(first.status, 'FundedExpressionPrepared', JSON.stringify(first));
  assert.equal(first.post.paid, '30');
  assert.equal(first.post.due, '100');
  assert.equal(first.financialPost.obligations[0].principal, '70');
  assert.equal(first.financialPost.obligations[0].outstanding, '70');
  assert.equal(first.financialPost.balances[0].amount, '70');
  assert.equal(first.financialPost.balances[1].amount, '30');
  assert.equal(first.financialPost.allowances[0].remaining, '70');
  assert.equal(first.financialPost.allowances[0].spent, '30');
  assert.deepEqual(first.financialPost.usedTransferIds, ['T1']);
  assert.deepEqual(first.financialPost.usedAllocationIds, ['Alloc1']);
  assert.equal(first.financialPost.work.closureReserve, '16');

  const second = api.evaluate(
    sourceText,
    'repay_installment',
    installmentSnapshot(first.financialPost.work.remaining, first.post),
    JSON.stringify(first.financialPost),
  );
  assert.equal(second.status, 'FundedExpressionPrepared', JSON.stringify(second));
  assert.equal(second.post.paid, '50');
  assert.equal(second.post.due, '100');
  assert.equal(second.financialPost.obligations[0].principal, '50');
  assert.equal(second.financialPost.obligations[0].outstanding, '50');
  assert.equal(second.financialPost.balances[0].amount, '50');
  assert.equal(second.financialPost.balances[1].amount, '50');
  assert.equal(second.financialPost.allowances[0].remaining, '50');
  assert.equal(second.financialPost.allowances[0].spent, '50');
  assert.deepEqual(second.financialPost.usedTransferIds, ['T1', 'T2']);
  assert.deepEqual(second.financialPost.usedAllocationIds, ['Alloc1', 'Alloc2']);
  assert.equal(second.financialPost.work.closureReserve, '16');

  const isolatedFirst = createFinancialAgreementSourceV1().evaluate(
    isolatedV1('repay', REPAY_PARAMS, REPAY_BODY),
    repaySnapshotsText,
    stateText,
  );
  assert.deepEqual(first, isolatedFirst);
  const isolatedSecond = createFinancialAgreementSourceV1().evaluate(
    isolatedV1('repay_installment', INSTALLMENT_PARAMS, INSTALLMENT_BODY),
    installmentSnapshot(first.financialPost.work.remaining, first.post),
    JSON.stringify(first.financialPost),
  );
  assert.deepEqual(second, isolatedSecond);
});

test('protected bindings, funding, guard, ensure and work failures still reject', () => {
  const extraOp = source('operation Notice: TransferFields;');
  assert.equal(assertRejected(language().check(extraOp)).code, 'OPERATION_BINDING');
  const mixed = source('unit USD;').replace(
    'nominalAmount: Quantity<Units<Cash,1>,0>',
    'nominalAmount: Quantity<Units<USD,1>,0>',
  );
  assert.equal(assertRejected(language().check(mixed)).code, 'OPERATION_BINDING');

  const unfunded = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY.replace('amount<Cash>(payment)', 'amount<Cash>(20)')} } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }`),
    'repay',
    snapshot('100'),
    stateText,
  ));
  assert.equal(unfunded.code, 'INSUFFICIENT_UNALLOCATED');

  const guard = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { requires false; next.paid = 1; } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }`),
    'repay',
    snapshot('100'),
    stateText,
  ));
  assert.equal(guard.code, 'GUARD_FAILED');

  const ensure = assertRejected(language().evaluate(
    source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures false;')} } action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }`),
    'repay',
    snapshot('100'),
    stateText,
  ));
  assert.equal(ensure.code, 'ENSURES_FAILED');

  const short = assertRejected(language().evaluate(source(), 'repay', snapshot('1'), JSON.stringify({
    ...JSON.parse(stateText),
    work: { remaining: '1', spent: '0', closureReserve: '16' },
  })));
  assert.ok(['INSUFFICIENT_WORK', 'WORK_EXHAUSTED'].includes(short.code), short.code);
});

test('later-action diagnostics keep original UTF-8 spans after comments and Unicode', () => {
  const prefix = '/* café 😀 */\n// note\n';
  const failing = `${prefix}${source('', ` action repay(${REPAY_PARAMS}) { ${REPAY_BODY} } action later(nominal: Quantity<Units<Missing,1>,0>) { next.paid = pre.paid; }`)}`;
  const result = assertRejected(language().check(failing));
  assert.equal(result.code, 'TYPE_NAME');
  assert.equal(Number(result.span.start), utf8Offset(failing, 'Missing'));
  assert.equal(sourceSlice(failing, result).includes('Missing'), true);
});

test('evaluate compiles fresh and never consumes returned Core', () => {
  const api = language();
  const elaborated = api.elaborate(source());
  elaborated.actions[0].core.statements.length = 0;
  elaborated.actions[0].schema.args = {};
  const result = api.evaluate(source(), 'repay', snapshot('100'), stateText);
  assert.equal(result.status, 'FundedExpressionPrepared', JSON.stringify(result));
  assert.equal(result.post.paid, '30');
});

test('format is idempotent and simulation-equivalent', () => {
  const formatted = formatFinancialAgreementSourceV2(sourceText);
  assert.equal(formatFinancialAgreementSourceV2(formatted), formatted);
  const api = language();
  assert.deepEqual(
    api.evaluate(sourceText, 'repay', repaySnapshotsText, stateText),
    api.evaluate(formatted, 'repay', repaySnapshotsText, stateText),
  );
  const program = parseFinancialAgreementSourceV2(sourceText);
  const again = parseFinancialAgreementSourceV2(formatted);
  assert.deepEqual(
    program.agreement.declarations.map((d) => d.tag + ':' + d.name),
    again.agreement.declarations.map((d) => d.tag + ':' + d.name),
  );
});

test('/1 signatures, exactly-one-action rejection and old profiles stay unchanged', () => {
  const v1 = createFinancialAgreementSourceV1();
  assert.equal(v1.evaluate.length, 3);
  const two = `profile "${P1}"; agreement FundedPayment {${declarations()}
    action repay(${REPAY_PARAMS}) { ${REPAY_BODY} }
    action repay_installment(${INSTALLMENT_PARAMS}) { ${INSTALLMENT_BODY} }
  }`;
  assert.equal(assertRejected(v1.check(two)).code, 'SOURCE_ACTION_COUNT');
  assert.equal(v1.check(isolatedV1('repay', REPAY_PARAMS, REPAY_BODY)).judgmentResult, 'SourceChecked');
  assert.equal(assertRejected(language().check(isolatedV1('repay', REPAY_PARAMS, REPAY_BODY))).code, 'PROFILE_MISMATCH');
  assert.throws(
    () => parseFinancialAgreementSource(source()),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
  assert.throws(
    () => parseFinancialAgreementSourceV2(isolatedV1('repay', REPAY_PARAMS, REPAY_BODY)),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
  assert.equal(formatFinancialAgreementSource(isolatedV1('repay', REPAY_PARAMS, REPAY_BODY)).includes(P1), true);
  const oldSchema = canonical({
    units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {},
    variantTypes: {}, fields: {}, args: {}, observations: {}, operations: {},
  });
  const oldFinancial = `profile "${FINANCIAL_EXPRESSION_SOURCE_PROFILE}"; agreement Demo { action step() { let operation = 1; requires operation == 1; } }`;
  assert.equal(createFinancialExpressionSourceV1(oldSchema).check(oldFinancial).judgmentResult, 'SourceChecked');
  const oldExpr = oldFinancial.replace(FINANCIAL_EXPRESSION_SOURCE_PROFILE, 'moriarty-expression-source/1');
  const exprSchema = canonical({
    units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {},
    fields: {}, args: {}, observations: {}, operations: {},
  });
  assert.equal(createExpressionSourceV1(exprSchema).check(oldExpr).judgmentResult, 'SourceChecked');
  assert.throws(
    () => parseSuccessorFinancialExpressionSource(source()),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
  assert.throws(
    () => parseSuccessorSource(`profile "moriarty-successor-syntax/0"; agreement Demo { record X { n: UInt128; } }`),
    (error) => error.code === 'UNKNOWN_DECLARATION',
  );
});

test('static source failures reject before snapshot-dependent work and keep transport bounds', () => {
  const dup = source('unit Cash;');
  const result = assertRejected(language().evaluate(dup, 'repay', '{', '{'));
  assert.equal(result.code, 'SOURCE_DUPLICATE_DECLARATION');
  assert.equal(result.workUsed, '0');
  assert.equal(assertRejected(language().check(' '.repeat(65537))).code, 'SOURCE_BOUND');
  assert.equal(assertRejected(language().evaluate(source(), 'repay', snapshotsText, '{')).code, 'INPUT_SCHEMA');
});

function sourceSlice(input, result) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal(result.span.kind, 'source', JSON.stringify(result));
  return Buffer.from(input).subarray(Number(result.span.start), Number(result.span.end)).toString();
}
