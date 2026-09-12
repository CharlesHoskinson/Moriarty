import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const root = process.argv[2];
const pkg = root + '/experiments/moriarty-language';
const {createFinancialAgreementSourceV2} = await import(pathToFileURL(pkg + '/src/successor/financial-agreement-source-v2.ts'));
const {createFinancialAgreementSourceV1} = await import(pathToFileURL(pkg + '/src/successor/financial-agreement-source-v1.ts'));
const {canonical} = await import(pathToFileURL(pkg + '/src/successor/expression-wire-v1.ts'));
const source = readFileSync(pkg + '/spec/successor/examples/multiple-action-payment.mori', 'utf8');
const snapshots = readFileSync(pkg + '/spec/successor/examples/multiple-action-payment.snapshots.json', 'utf8');
const state = readFileSync(pkg + '/spec/successor/examples/expression-funded-payment.state.json', 'utf8');
const api = createFinancialAgreementSourceV2();
const outcomes = [];
function record(name) { outcomes.push(name); }
function reject(value, name) {
  assert.equal(value.status, 'Rejected', name);
  for (const key of ['post', 'financialPost', 'effects', 'descriptors', 'result']) assert(!Object.hasOwn(value, key), `${name}: leaked ${key}`);
  record(name);
  return value;
}
assert.equal(api.evaluate.length, 4);
assert.equal(createFinancialAgreementSourceV1().evaluate.length, 3);
record('public evaluator arities');
const check = api.check(source);
assert.equal(check.judgmentResult, 'SourceChecked');
assert.deepEqual(check.actions.map(x => x.action), ['repay', 'repay_installment']);
assert(!Object.hasOwn(check, 'staticWorkBound'));
record('ordered checks without aggregate work bound');
const artifacts = api.elaborate(source);
assert.equal(artifacts.judgmentResult, 'SourceElaborated');
assert.deepEqual(Object.keys(artifacts.actions[0].schema.args).sort(), ['allocationId', 'nominal', 'transferId']);
assert.deepEqual(Object.keys(artifacts.actions[1].schema.args).sort(), ['allocationId', 'transferId']);
record('per-action argument schemas');
const first = api.evaluate(source, 'repay', snapshots, state);
assert.equal(first.status, 'FundedExpressionPrepared');
assert.equal(first.financialPost.obligations[0].principal, '70');
record('actual example selected repayment');
artifacts.actions[0].schema.args = {};
artifacts.actions[0].core.statements.length = 0;
artifacts.actions.reverse();
assert.deepEqual(api.evaluate(source, 'repay', snapshots, state), first);
record('elaboration output cannot alter execution');
for (const selector of [undefined, null, 1, {}, [], true, '', '__proto__', 'a'.repeat(65)]) {
  const value = reject(api.evaluate(source, selector, snapshots, state), 'invalid selector ' + String(selector));
  assert.equal(value.code, 'SOURCE_ACTION_NAME');
  assert.equal(value.span.kind, 'synthetic');
}
for (const selector of ['constructor', 'toString', 'Repay', 'unknown']) {
  const value = reject(api.evaluate(source, selector, snapshots, state), 'unknown selector ' + selector);
  assert.equal(value.code, 'SOURCE_ACTION_UNKNOWN');
}
let touched = false;
const hostile = Object.create(null, {toString: {get() {touched = true; throw Error('coercion');}}});
reject(api.evaluate(source, hostile, snapshots, state), 'hostile selector object');
assert.equal(touched, false);
for (const position of [0, 2, 3]) {
  const args = [source, 'repay', snapshots, state];
  args[position] = hostile;
  reject(api.evaluate(...args), 'non-string input ' + position);
  assert.equal(touched, false);
}
const next = {Pre: first.post, Args: {transferId: 'T2', allocationId: 'Alloc2'}, Obs: {}, workInitial: first.workRemaining};
const second = api.evaluate(source, 'repay_installment', canonical(next), canonical(first.financialPost));
assert.equal(second.status, 'FundedExpressionPrepared');
assert.equal(second.financialPost.obligations[0].principal, '50');
assert.equal(second.post.paid, '50');
assert.deepEqual(second.financialPost.usedTransferIds, ['T1', 'T2']);
assert.deepEqual(second.financialPost.usedAllocationIds, ['Alloc1', 'Alloc2']);
record('full state continuation through a different action');
const failedSnapshot = structuredClone(next);
failedSnapshot.Pre.due = '0';
reject(api.evaluate(source, 'repay_installment', canonical(failedSnapshot), canonical(first.financialPost)), 'second action guard rollback');
assert.deepEqual(api.evaluate(source, 'repay_installment', canonical(next), canonical(first.financialPost)), second);
record('failed invocation does not mutate reusable inputs');
const originalV1 = readFileSync(pkg + '/spec/successor/examples/source-defined-payment.mori', 'utf8');
reject(api.check(originalV1), '/2 rejects /1 source header');
reject(createFinancialAgreementSourceV1().check(source), '/1 rejects /2 source header');
console.log(JSON.stringify({passed: outcomes.length, outcomes, first, second}, null, 2));
