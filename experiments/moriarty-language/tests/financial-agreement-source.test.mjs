import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { createFinancialAgreementSourceV1 } from '../src/successor/financial-agreement-source-v1.ts';
import {
  FINANCIAL_AGREEMENT_SOURCE_PROFILE,
  formatFinancialAgreementSource,
  parseFinancialAgreementSource,
} from '../src/successor/financial-agreement-source-frontend.ts';
import { createFundedFinancialExpressionSourceV1 } from '../src/successor/funded-expression-source-v1.ts';
import { createFinancialExpressionSourceV1 } from '../src/successor/financial-expression-source-v1.ts';
import { createExpressionSourceV1 } from '../src/successor/expression-source-v1.ts';
import {
  FINANCIAL_EXPRESSION_SOURCE_PROFILE,
  parseSuccessorFinancialExpressionSource,
  parseSuccessorSource,
  SOURCE_KEYWORDS,
} from '../src/successor/frontend.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const P = FINANCIAL_AGREEMENT_SOURCE_PROFILE;
const example = (stem) => fileURLToPath(new URL(`../spec/successor/examples/${stem}`, import.meta.url));
const fixture = (stem) => readFileSync(example(stem), 'utf8');
const schemaText = fixture('expression-funded-payment.schema.json');
const snapshotsText = fixture('expression-funded-payment.snapshots.json');
const stateText = fixture('expression-funded-payment.state.json');
const oldSource = fixture('expression-funded-payment.mori');
const sourceText = fixture('source-defined-payment.mori');

const PAY_BODY = [
  'requires pre.due > 0;',
  'let nominal = first + second;',
  'requires is_negative(nominal) == false;',
  'let payment = magnitude(nominal);',
  'requires payment > 0;',
  'next.paid = pre.paid + payment;',
  'emit Transfer { id: transferId, from: "Payer", to: "Lender", settlementAsset: "Cash", transferAmount: amount<Cash>(payment) };',
  'emit Repay { allocationId: allocationId, transferId: transferId, obligationId: "Due100", payer: "Payer", nominalAmount: nominal };',
  'ensures post.paid == pre.paid + payment;',
].join(' ');

const PAY_PARAMS = 'first: Quantity<Units<Cash,1>,0>, second: Quantity<Units<Cash,1>,0>, transferId: Text, allocationId: Text';

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

function source(extraDecls = '', body = PAY_BODY, params = PAY_PARAMS) {
  return `profile "${P}"; agreement FundedPayment {${declarations(extraDecls)} action pay(${params}) { ${body} } }`;
}

function language() {
  return createFinancialAgreementSourceV1();
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
  allocationId: 'Alloc1', first: '10', second: '20', transferId: 'T1',
}) {
  return canonical({ Args, Obs: {}, Pre, workInitial });
}

function obligation(id, extra = {}) {
  return {
    id,
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
    ...extra,
  };
}

function state(extra = {}) {
  const work = { remaining: '100', spent: '0', closureReserve: '16', ...(extra.work ?? {}) };
  const { work: _ignored, ...rest } = extra;
  return {
    balances: [
      { party: 'Payer', asset: 'Cash', amount: '100' },
      { party: 'Lender', asset: 'Cash', amount: '0' },
      { party: 'Unrelated', asset: 'Cash', amount: '5' },
    ],
    allowances: [{ party: 'Payer', asset: 'Cash', remaining: '100', spent: '0' }],
    obligations: [
      obligation('Due100'),
      obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }),
    ],
    usedTransferIds: ['Earlier'],
    usedAllocationIds: ['Prior'],
    work,
    ...rest,
  };
}

function stateJSON(extra = {}) {
  return JSON.stringify(state(extra));
}

test('factory exposes only source/snapshot/state strings and does not take schema or AST', () => {
  assert.equal(createFinancialAgreementSourceV1.length, 0);
  const api = language();
  assert.deepEqual(Object.keys(api), ['elaborate', 'check', 'evaluate']);
  assert.equal(api.elaborate.length, 1);
  assert.equal(api.check.length, 1);
  assert.equal(api.evaluate.length, 3);
  assert.equal('evaluateCore' in api, false);
  assert.equal(SOURCE_KEYWORDS.includes('record'), false);
  assert.equal(SOURCE_KEYWORDS.includes('operation'), false);
});

test('parser emits distinct uninitialized state, record and operation declarations', () => {
  const program = parseFinancialAgreementSource(sourceText);
  assert.equal(program.profile.value, P);
  const tags = Object.fromEntries(program.agreement.declarations.map((d) => [d.name + ':' + d.tag, d]));
  assert.equal(tags['due:UninitializedStateDecl'].type.name, 'UInt128');
  assert.equal('value' in tags['due:UninitializedStateDecl'], false);
  assert.equal(tags['TransferFields:RecordDecl'].fields.map((f) => f.name).join(','),
    'id,from,to,settlementAsset,transferAmount');
  assert.equal(tags['Transfer:OperationDecl'].type.name, 'TransferFields');
  assert.equal(tags['Transfer:OperationDecl'].type.arguments.length, 0);
  const oldState = parseSuccessorFinancialExpressionSource(
    `profile "${FINANCIAL_EXPRESSION_SOURCE_PROFILE}"; agreement Demo { action step() { next.paid = 1; } }`,
  ).agreement.declarations.find((d) => d.tag === 'ActionDecl');
  assert.equal(oldState.tag, 'ActionDecl');
});

test('generated schema equals the existing funded example schema canonically', () => {
  const elaborated = language().elaborate(sourceText);
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  assert.equal(elaborated.sourceProfile, P);
  assert.equal(canonical(elaborated.schema), schemaText);
  assert.equal(language().elaborate(sourceText).core.span.kind, 'source');
  for (const field of Object.values(elaborated.schema.fields)) {
    assert.equal(field.writeClass, 'ordinary');
  }
});

test('same action, snapshots and state equal the schema-based funded adapter result and work', () => {
  const expected = createFundedFinancialExpressionSourceV1(schemaText)
    .evaluate(oldSource, snapshotsText, stateText);
  assert.equal(expected.status, 'FundedExpressionPrepared', JSON.stringify(expected));
  const result = language().evaluate(sourceText, snapshotsText, stateText);
  assert.deepEqual(result, expected);
  assert.equal(result.post.paid, '30');
  assert.equal(result.financialPost.balances[0].amount, '70');
  assert.equal(result.financialPost.obligations[0].principal, '70');
  assert.equal(result.financialPost.work.closureReserve, '16');
  const oldExpression = createFinancialExpressionSourceV1(schemaText).evaluate(oldSource, snapshotsText);
  const newChecked = language().check(sourceText);
  assert.equal(newChecked.judgmentResult, 'SourceChecked');
  assert.equal(newChecked.staticWorkBound, createFinancialExpressionSourceV1(schemaText).check(oldSource).staticWorkBound);
  assert.equal(result.workRemaining, expected.workRemaining);
  assert.equal(oldExpression.status, 'ExpressionPrepared');
});

test('due100/pay30, interest-first P100/I10/pay7, continuation and failed funding/guard/ensure/work', () => {
  const api = language();
  const pay30 = api.evaluate(source(), snapshot('100'), stateJSON());
  assert.equal(pay30.status, 'FundedExpressionPrepared', JSON.stringify(pay30));
  assert.equal(pay30.post.paid, '30');
  assert.equal(pay30.financialPost.balances[0].amount, '70');
  assert.equal(pay30.financialPost.allowances[0].spent, '30');
  assert.equal(pay30.financialPost.work.closureReserve, '16');

  const interest = api.evaluate(
    source(),
    snapshot('100', { due: '100', paid: '0' }, { allocationId: 'Alloc1', first: '7', second: '0', transferId: 'T1' }),
    stateJSON({
      obligations: [
        obligation('Due100', { principal: '100', accrued: '10', outstanding: '110' }),
        obligation('Other', { principal: '40', accrued: '0', outstanding: '40' }),
      ],
    }),
  );
  assert.equal(interest.status, 'FundedExpressionPrepared', JSON.stringify(interest));
  assert.equal(interest.financialPost.obligations[0].principal, '100');
  assert.equal(interest.financialPost.obligations[0].accrued, '3');
  assert.equal(interest.effects[1].accruedDischarged, '7');
  assert.equal(interest.effects[1].principalDischarged, '0');

  const remaining = pay30.financialPost.work.remaining;
  const second = api.evaluate(
    source(),
    snapshot(remaining, { due: '100', paid: '30' }, {
      allocationId: 'Alloc2', first: '10', second: '10', transferId: 'T2',
    }),
    JSON.stringify(pay30.financialPost),
  );
  assert.equal(second.status, 'FundedExpressionPrepared', JSON.stringify(second));
  assert.equal(second.post.paid, '50');
  assert.deepEqual(second.financialPost.usedTransferIds, ['Earlier', 'T1', 'T2']);
  assert.equal(second.financialPost.work.closureReserve, '16');

  const unfunded = assertRejected(api.evaluate(
    source('', PAY_BODY.replace('amount<Cash>(payment)', 'amount<Cash>(20)')),
    snapshot('100'),
    stateJSON(),
  ));
  assert.equal(unfunded.code, 'INSUFFICIENT_UNALLOCATED');

  const guard = assertRejected(api.evaluate(source('', 'next.paid = 1; requires false;'), snapshot('100'), stateJSON()));
  assert.equal(guard.code, 'GUARD_FAILED');
  const ensure = assertRejected(api.evaluate(
    source('', PAY_BODY.replace('ensures post.paid == pre.paid + payment;', 'ensures false;')),
    snapshot('100'),
    stateJSON(),
  ));
  assert.equal(ensure.code, 'ENSURES_FAILED');

  const E = BigInt(createFinancialExpressionSourceV1(schemaText).check(oldSource).staticWorkBound);
  const short = assertRejected(api.evaluate(source(), snapshot('1'), stateJSON({
    work: { remaining: '1', spent: '0', closureReserve: '16' },
  })));
  assert.ok(['INSUFFICIENT_WORK', 'WORK_EXHAUSTED'].includes(short.code), short.code);
  assert.ok(E > 1n);
});

test('duplicate declarations, fields and parameters reject with original spans', () => {
  const unitDup = source('unit Cash;');
  const unit = assertRejected(language().check(unitDup));
  assert.equal(unit.code, 'SOURCE_DUPLICATE_DECLARATION');
  assert.equal(unit.span.kind, 'source');
  assert.equal(Number(unit.span.start), utf8Offset(unitDup, 'unit Cash;', -1));
  assert.equal(unit.workUsed, '0');

  const assetDup = source('asset Cash: Asset;');
  assert.equal(assertRejected(language().check(assetDup)).code, 'SOURCE_DUPLICATE_DECLARATION');

  const topDup = source('record Cash { n: UInt128; }');
  assert.equal(assertRejected(language().check(topDup)).code, 'SOURCE_DUPLICATE_DECLARATION');

  const fieldSource = source().replace('from: Text;', 'id: Text;');
  const field = assertRejected(language().check(fieldSource));
  assert.equal(field.code, 'SOURCE_DUPLICATE_DECLARATION');
  assert.equal(field.span.kind, 'source');
  assert.equal(Number(field.span.start), utf8Offset(fieldSource, 'id: Text;', fieldSource.indexOf('id: Text;') + 1));

  const params = assertRejected(language().check(source('', PAY_BODY, 'first: Text, first: Text, transferId: Text, allocationId: Text')));
  assert.equal(params.code, 'SOURCE_PARAMETER_NAMES');
});

test('forward record references succeed and unknown or cyclic records reject', () => {
  const forward = source(`
    record Later {
      inner: Record<Inner>;
    }
    record Inner {
      n: UInt128;
    }
  `);
  assert.equal(language().check(forward).judgmentResult, 'SourceChecked');

  const unknown = source().replace('Amount<Cash>', 'Amount<Missing>');
  const missingAsset = assertRejected(language().check(unknown));
  assert.ok(['TYPE_NAME', 'SOURCE_TYPE_SHAPE'].includes(missingAsset.code), missingAsset.code);
  assert.equal(missingAsset.span.kind, 'source');

  const missingRecord = source().replace('operation Transfer: TransferFields;', 'operation Transfer: MissingFields;');
  const missing = assertRejected(language().check(missingRecord));
  assert.ok(['TYPE_NAME', 'OPERATION_BINDING', 'SOURCE_DECLARATION_TYPE'].includes(missing.code), missing.code);

  const cyclic = `profile "${P}"; agreement Cycle {
    unit Cash;
    asset Cash: Asset;
    record A { b: Record<B>; }
    record B { a: Record<A>; }
    operation Transfer: TransferFields;
    operation Repay: RepayFields;
    record TransferFields { id: Text; from: Text; to: Text; settlementAsset: Text; transferAmount: Amount<Cash>; }
    record RepayFields { allocationId: Text; transferId: Text; obligationId: Text; payer: Text; nominalAmount: Quantity<Units<Cash,1>,0>; }
    state due: UInt128;
    state paid: UInt128;
    action pay(${PAY_PARAMS}) { ${PAY_BODY} }
  }`;
  const cycle = assertRejected(language().check(cyclic));
  assert.equal(cycle.code, 'TYPE_SCHEMA_CYCLE');
  assert.equal(cycle.span.kind, 'source');
});

test('mixed units and altered protected operations reject at check and evaluate', () => {
  const mixed = source('unit USD;').replace(
    'nominalAmount: Quantity<Units<Cash,1>,0>',
    'nominalAmount: Quantity<Units<USD,1>,0>',
  );
  const mixedResult = assertRejected(language().check(mixed));
  assert.equal(mixedResult.code, 'OPERATION_BINDING');

  const extraOp = source('operation Notice: TransferFields;');
  assert.equal(assertRejected(language().check(extraOp)).code, 'OPERATION_BINDING');

  const missingOp = source().replace('operation Repay: RepayFields;', '');
  assert.equal(assertRejected(language().check(missingOp)).code, 'OPERATION_BINDING');

  const scalar = source().replace('transferAmount: Amount<Cash>;', 'transferAmount: UInt128;');
  assert.equal(assertRejected(language().check(scalar)).code, 'OPERATION_BINDING');

  const power = source().replace(
    'nominalAmount: Quantity<Units<Cash,1>,0>',
    'nominalAmount: Quantity<Units<Cash,2>,0>',
  );
  assert.equal(assertRejected(language().check(power)).code, 'OPERATION_BINDING');

  const scale = source().replace(
    'nominalAmount: Quantity<Units<Cash,1>,0>',
    'nominalAmount: Quantity<Units<Cash,1>,1>',
  );
  assert.equal(assertRejected(language().check(scale)).code, 'OPERATION_BINDING');

  const extraField = source().replace('transferAmount: Amount<Cash>;', 'transferAmount: Amount<Cash>; extra: Text;');
  assert.equal(assertRejected(language().check(extraField)).code, 'OPERATION_BINDING');

  const renamed = source().replace('operation Transfer: TransferFields;', 'operation Send: TransferFields;');
  assert.equal(assertRejected(language().check(renamed)).code, 'OPERATION_BINDING');

  assert.equal(assertRejected(language().evaluate(mixed, snapshotsText, stateText)).code, 'OPERATION_BINDING');
});

test('initialized state, const, unsupported declarations and financial reclassification reject', () => {
  const initialized = `profile "${P}"; agreement FundedPayment {${declarations()} state extra: UInt128 = 0; action pay(${PAY_PARAMS}) { ${PAY_BODY} } }`;
  const init = assertRejected(language().check(initialized));
  assert.ok(['UNEXPECTED_TOKEN', 'SOURCE_DECLARATION'].includes(init.code), init.code);

  const constDecl = `profile "${P}"; agreement FundedPayment {${declarations()} const n: UInt128 = 1; action pay(${PAY_PARAMS}) { ${PAY_BODY} } }`;
  assert.equal(assertRejected(language().check(constDecl)).code, 'UNKNOWN_DECLARATION');

  for (const decl of ['enum E { A }', 'variant V { A: UInt128 }', 'observation rate: Rate<0>;', 'vault Pool;']) {
    const unsupported = `profile "${P}"; agreement FundedPayment {${declarations()} ${decl} action pay(${PAY_PARAMS}) { ${PAY_BODY} } }`;
    assert.equal(assertRejected(language().check(unsupported)).code, 'UNKNOWN_DECLARATION');
  }

  const elaborated = language().elaborate(source('state balances: UInt128;'));
  assert.equal(elaborated.judgmentResult, 'SourceElaborated', JSON.stringify(elaborated));
  assert.equal(elaborated.schema.fields.balances.writeClass, 'ordinary');
  assert.equal(elaborated.schema.fields.due.writeClass, 'ordinary');
});

test('comments, Unicode and earlier declarations preserve original source offsets', () => {
  const prefix = '/* café 😀 */\n// note\n';
  const failing = `${prefix}${source('unit Cash;')}`;
  const result = assertRejected(language().check(failing));
  assert.equal(result.code, 'SOURCE_DUPLICATE_DECLARATION');
  assert.equal(Number(result.span.start), utf8Offset(failing, 'unit Cash;', -1));
  const token = failing.slice(Number(result.span.start), Number(result.span.end));
  assert.equal(token.startsWith('unit') || token.includes('Cash'), true, token);

  const syntax = `${prefix}profile "${P}"; agreement Demo { unit Cash; unknown X; }`;
  try {
    parseFinancialAgreementSource(syntax);
    assert.fail('expected parse failure');
  } catch (error) {
    assert.equal(error.code, 'UNKNOWN_DECLARATION');
    assert.equal(error.start, utf8Offset(syntax, 'unknown'));
  }
});

test('format is idempotent, schema-stable and simulation-equivalent', () => {
  const formatted = formatFinancialAgreementSource(sourceText);
  assert.equal(formatFinancialAgreementSource(formatted), formatted);
  const api = language();
  assert.equal(canonical(api.elaborate(sourceText).schema), canonical(api.elaborate(formatted).schema));
  assert.deepEqual(
    api.evaluate(sourceText, snapshotsText, stateText),
    api.evaluate(formatted, snapshotsText, stateText),
  );
  const program = parseFinancialAgreementSource(sourceText);
  const again = parseFinancialAgreementSource(formatted);
  const strip = (value) => Array.isArray(value)
    ? value.map(strip)
    : value && typeof value === 'object'
      ? Object.fromEntries(Object.entries(value).filter(([key]) => key !== 'span').map(([key, child]) => [key, strip(child)]))
      : value;
  assert.deepEqual(strip(program.agreement.declarations.map((d) => d.tag)), strip(again.agreement.declarations.map((d) => d.tag)));
});

function sourceSlice(input, result) {
  assert.equal(result.status, 'Rejected', JSON.stringify(result));
  assert.equal(result.span.kind, 'source', JSON.stringify(result));
  return Buffer.from(input).subarray(Number(result.span.start), Number(result.span.end)).toString();
}

test('declared type and binding failures keep original UTF-8 spans on the offending form', () => {
  const prefix = '// original UTF-8 offsets: λ🙂\n';
  const located = (before, after, expected, code) => {
    assert.equal(sourceText.includes(before), true, before);
    const input = prefix + sourceText.replace(before, after);
    const result = language().check(input);
    const location = sourceSlice(input, result);
    assert.equal(result.code, code, JSON.stringify(result));
    assert.equal(location.includes(expected), true, JSON.stringify({ location, expected, result }));
    return location;
  };
  located('first: Quantity<Units<Cash,1>,0>', 'first: Quantity<Units<Missing,1>,0>', 'Missing', 'TYPE_NAME');
  located('state paid: UInt128;', 'state paid: ScaledAmount<Missing,2>;', 'Missing', 'TYPE_NAME');
  located('allocationId: Text) {', 'allocationId: Record<Missing>) {', 'Missing', 'TYPE_NAME');
  const repay = located(
    'nominalAmount: Quantity<Units<Cash,1>,0>;',
    'nominalAmount: UInt128;',
    'nominalAmount',
    'OPERATION_BINDING',
  );
  assert.equal(repay.includes('TransferFields'), false, repay);

  const nested = [
    ['state paid: UInt128;', 'state paid: Option<ScaledAmount<Missing,2>>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: Collection<Record<Missing>,4>;', 'Missing', 'TYPE_NAME'],
    ['first: Quantity<Units<Cash,1>,0>', 'first: Option<Quantity<Units<Missing,1>,0>>', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: SignedAmount<Missing>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: NetAmount<Missing>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: SignedScaledAmount<Missing,1>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: AmountProduct<Missing,Cash>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: Price<Missing,Cash,0>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: Shares<Missing,Payer>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: Variant<Missing>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: Enum<Missing>;', 'Missing', 'TYPE_NAME'],
    ['state paid: UInt128;', 'state paid: Operation<Missing>;', 'Missing', 'TYPE_NAME'],
    ['transferAmount: Amount<Cash>;', 'transferAmount: UInt128;', 'transferAmount', 'OPERATION_BINDING'],
  ];
  for (const [before, after, expected, code] of nested) {
    located(before, after, expected, code);
  }
});

test('reserved metadata names keep original UTF-8 spans in every source namespace', () => {
  const prefix = '// λ🙂 original offsets\n';
  const located = (input, expected, code = 'SOURCE_SCHEMA_NAME') => {
    const prefixed = prefix + input;
    const result = language().check(prefixed);
    const location = sourceSlice(prefixed, result);
    assert.equal(result.code, code, JSON.stringify(result));
    assert.equal(location.includes(expected), true, JSON.stringify({ location, expected, result }));
    return location;
  };
  located(sourceText.replace('state paid: UInt128;', 'state some: UInt128;'), 'some');
  located(sourceText.replace('first: Quantity', 'some: Quantity'), 'some');
  located(source('unit some;'), 'some');
  located(source('asset some: Asset;'), 'some');
  located(source('party some;'), 'some');
  located(source('record some { n: UInt128; }'), 'some');
  located(source('record Extra { some: Text; }'), 'some');
  located(source('operation some: TransferFields;'), 'some');
  located(sourceText.replace('state paid: UInt128;', 'state quantity: UInt128;'), 'quantity');
  located(sourceText.replace('state paid: UInt128;', 'state amount: UInt128;'), 'amount');
});

test('operation and nested collection cycles keep referring TypeNode spans', () => {
  const prefix = '// λ🙂 original offsets\n';
  const located = (input, expected, code = 'TYPE_SCHEMA_CYCLE') => {
    const prefixed = prefix + input;
    const result = language().check(prefixed);
    const location = sourceSlice(prefixed, result);
    assert.equal(result.code, code, JSON.stringify(result));
    assert.equal(location.includes(expected), true, JSON.stringify({ location, expected, result }));
    assert.equal(result.span.kind, 'source');
    return location;
  };
  located(sourceText.replace('payer: Text;', 'payer: Operation<Repay>;'), 'Operation<Repay>');
  located(sourceText.replace('payer: Text;', 'payer: Option<Operation<Repay>>;'), 'Operation<Repay>');
  located(source(`
    record Loop {
      link: Option<Collection<Operation<LoopOp>,1>>;
    }
    operation LoopOp: Loop;
  `), 'Operation<LoopOp>');
  const unknown = language().check(prefix + sourceText.replace('payer: Text;', 'payer: Operation<Missing>;'));
  assert.equal(unknown.status, 'Rejected');
  assert.equal(unknown.code, 'TYPE_NAME');
  assert.equal(unknown.span.kind, 'source');
  assert.equal(sourceSlice(prefix + sourceText.replace('payer: Text;', 'payer: Operation<Missing>;'), unknown).includes('Missing'), true);
});

test('compound-invalid parameter faults reject before protected binding on check, elaborate and evaluate', () => {
  const prefix = '// UTF-8 λ🙂\n';
  const api = language();
  const cases = [
    {
      name: 'duplicate parameter and missing Repay',
      input: prefix + sourceText
        .replace('second: Quantity<Units<Cash,1>,0>', 'first: Quantity<Units<Cash,1>,0>')
        .replace('operation Repay: RepayFields;', ''),
      code: 'SOURCE_PARAMETER_NAMES',
      needle: 'first: Quantity<Units<Cash,1>,0>',
      from: 1,
    },
    {
      name: 'duplicate parameter and extra operation',
      input: prefix + sourceText
        .replace('second: Quantity<Units<Cash,1>,0>', 'first: Quantity<Units<Cash,1>,0>')
        .replace('operation Repay: RepayFields;', 'operation Repay: RepayFields; operation Notice: RepayFields;'),
      code: 'SOURCE_PARAMETER_NAMES',
      needle: 'first: Quantity<Units<Cash,1>,0>',
      from: 1,
    },
    {
      name: 'shared field name and missing Repay',
      input: prefix + sourceText
        .replace('second: Quantity<Units<Cash,1>,0>', 'due: Quantity<Units<Cash,1>,0>')
        .replace('operation Repay: RepayFields;', ''),
      code: 'SOURCE_RESERVED_NAME',
      needle: 'due: Quantity<Units<Cash,1>,0>',
      from: 0,
    },
    {
      name: 'shared field name and type-mismatched Repay field',
      input: prefix + sourceText
        .replace('second: Quantity<Units<Cash,1>,0>', 'due: Quantity<Units<Cash,1>,0>')
        .replace('nominalAmount: Quantity<Units<Cash,1>,0>;', 'nominalAmount: UInt128;'),
      code: 'SOURCE_RESERVED_NAME',
      needle: 'due: Quantity<Units<Cash,1>,0>',
      from: 0,
    },
  ];
  for (const item of cases) {
    const start = utf8Offset(item.input, item.needle, item.from === 1
      ? item.input.indexOf(item.needle) + 1
      : 0);
    for (const method of ['check', 'elaborate', 'evaluate']) {
      const result = method === 'evaluate'
        ? api.evaluate(item.input, snapshotsText, stateText)
        : api[method](item.input);
      const rejected = assertRejected(result);
      assert.equal(rejected.code, item.code, JSON.stringify({ name: item.name, method, result }));
      assert.equal(rejected.span.kind, 'source');
      assert.equal(Number(rejected.span.start), start, JSON.stringify({ name: item.name, method, result }));
      assert.deepEqual(rejected.nodePath, []);
      assert.equal(rejected.workUsed, '0');
      assert.equal(sourceSlice(item.input, rejected).includes(item.needle.split(':')[0]), true);
    }
  }
});

test('static source failures reject before snapshot-dependent work', () => {
  const dup = source('unit Cash;');
  const result = assertRejected(language().evaluate(dup, '{', '{'));
  assert.equal(result.code, 'SOURCE_DUPLICATE_DECLARATION');
  assert.equal(result.workUsed, '0');
  assert.equal(result.span.kind, 'source');
});

test('transport bounds and old-profile source regressions stay unchanged', () => {
  assert.equal(assertRejected(language().check(' '.repeat(65537))).code, 'SOURCE_BOUND');
  assert.equal(assertRejected(language().check(sourceText.replace(P, FINANCIAL_EXPRESSION_SOURCE_PROFILE))).code, 'PROFILE_MISMATCH');
  assert.throws(
    () => parseSuccessorFinancialExpressionSource(sourceText),
    (error) => error.code === 'PROFILE_MISMATCH',
  );
  assert.throws(
    () => parseSuccessorSource(`profile "moriarty-successor-syntax/0"; agreement Demo { record X { n: UInt128; } }`),
    (error) => error.code === 'UNKNOWN_DECLARATION',
  );
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
  assert.equal(language().check(oldFinancial).code, 'PROFILE_MISMATCH');
});
