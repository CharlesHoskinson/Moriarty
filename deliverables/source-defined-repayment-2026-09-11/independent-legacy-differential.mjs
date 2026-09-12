import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
const candidate = process.argv[2];
const baseline = '/home/charl/Moriarty/.worktrees/expression-funded-repayment';
const modulePath = '/experiments/moriarty-language/src/successor/funded-expression-source-v1.ts';
const oldApi = (await import(pathToFileURL(baseline + modulePath))).createFundedFinancialExpressionSourceV1;
const newApi = (await import(pathToFileURL(candidate + modulePath))).createFundedFinancialExpressionSourceV1;
const { canonical } = await import(pathToFileURL(candidate + '/experiments/moriarty-language/src/successor/expression-wire-v1.ts'));
const fixture = baseline + '/experiments/moriarty-language/spec/successor/examples/expression-funded-payment';
const source = readFileSync(fixture + '.mori', 'utf8');
const snapshot = readFileSync(fixture + '.snapshots.json', 'utf8');
const state = readFileSync(fixture + '.state.json', 'utf8');
const schema = JSON.parse(readFileSync(fixture + '.schema.json', 'utf8'));
const cases = [['unchanged', schema]];
for (const operation of ['Transfer', 'Repay']) {
  const missing = structuredClone(schema); delete missing.operations[operation];
  cases.push(['missing operation ' + operation, missing]);
  const renamed = structuredClone(schema); renamed.operations.Other = renamed.operations[operation]; delete renamed.operations[operation];
  cases.push(['renamed operation ' + operation, renamed]);
  const record = schema.operations[operation];
  for (const field of Object.keys(schema.recordTypes[record])) {
    const omitted = structuredClone(schema); delete omitted.recordTypes[record][field];
    cases.push(['missing ' + record + '.' + field, omitted]);
    for (const type of [['UInt128'], ['Text'], ['Amount', 'Cash'], ['Quantity', [['Cash','2']], '0']]) {
      const changed = structuredClone(schema); changed.recordTypes[record][field] = type;
      cases.push(['changed ' + record + '.' + field + ':' + JSON.stringify(type), changed]);
    }
  }
  const extra = structuredClone(schema); extra.recordTypes[record].extra = ['Text'];
  cases.push(['extra field ' + record, extra]);
}
const outcomes = [];
for (const [name, value] of cases) {
  const text = canonical(value);
  const expected = oldApi(text).evaluate(source, snapshot, state);
  const actual = newApi(text).evaluate(source, snapshot, state);
  assert.deepEqual(actual, expected, name);
  outcomes.push({ name, status: actual.status, code: actual.code });
}
console.log(JSON.stringify({ passed: outcomes.length, outcomes }, null, 2));
