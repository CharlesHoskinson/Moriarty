import assert from 'node:assert/strict';
import { runLifecycleCorpus, compareLifecycleResult } from '../../experiments/moriarty-language/formal/k/lifecycle-corpus.mjs';
import { createFinancialExpressionContractV4 } from '../../experiments/moriarty-language/src/successor/financial-expression-v1.ts';

const report = await runLifecycleCorpus();
assert.equal(report.ok, true, JSON.stringify(report.failures));
assert.equal(report.kExecuted, false);
assert.equal(report.kStatus, 'not-executed');
assert.deepEqual(report.stages.map(s => s.source.result.financialPost.obligations[0].outstanding), ['100', '110', '80', '0']);
assert.deepEqual(report.stages.map(s => s.source.result.financialPost.work.spent), ['116', '181', '269', '355']);
assert.equal(report.cases.length, 9);
assert.equal(report.cases.filter(c => c.retry).length, 8);
for (const c of report.cases) for (const kind of ['source', 'core']) {
  assert.deepEqual(c[kind].result, c.expected);
  for (const key of ['post', 'financialPost', 'effects', 'descriptors', 'workRemaining']) assert.equal(Object.hasOwn(c[kind].result, key), false);
}
const final = report.stages[3].expected;
const mutationPaths = [
  ['financialPost', 'work', 'spent'],
  ['financialPost', 'allowances', 1, 'spent'],
  ['financialPost', 'obligations', 0, 'liabilityIncurred'],
  ['financialPost', 'obligations', 0, 'lastAccruedPeriod'],
  ['effects', 0, 'amount'],
];
for (const parts of mutationPaths) {
  const changed = structuredClone(final);
  let node = changed;
  for (const part of parts.slice(0, -1)) node = node[part];
  node[parts.at(-1)] = '999';
  assert.equal(compareLifecycleResult(changed, final).ok, false, parts.join('/'));
}
assert.equal(compareLifecycleResult({a: undefined}, {}).ok, false);
assert.equal(compareLifecycleResult([undefined], Array(1)).ok, false);
assert.equal(compareLifecycleResult({'0': 'a'}, ['a']).ok, false);
assert.equal(compareLifecycleResult({a: '1'}, {a: 1}).ok, false);

const coreInputs = [];
const divergent = await runLifecycleCorpus({coreFactory(schema, financial) {
  const real = createFinancialExpressionContractV4(schema, financial);
  return {evaluate(request) {
    coreInputs.push(JSON.parse(request));
    const output = structuredClone(real.evaluate(request));
    if (coreInputs.length === 1) output.post.phase = '77';
    return output;
  }};
}});
assert.equal(divergent.ok, false);
assert.equal(coreInputs[1].Pre.phase, '77', 'Core must consume its own changed predecessor');
assert.equal(divergent.stages[1].source.input.snapshot.Pre.phase, '1', 'Source must keep its independent predecessor');
assert.equal(divergent.stages[1].source.result.post.phase, '2');
assert.ok(divergent.failures.some(f => f.label.includes('independent')));

const failed = await runLifecycleCorpus({sourceFactory() { throw new Error('independent startup failure'); }});
assert.equal(failed.ok, false);
assert.equal(failed.kExecuted, false);
assert.equal(failed.stages.length, 0);
assert.ok(failed.failures.some(f => f.message === 'independent startup failure'));
console.log(JSON.stringify({passed: true, stages: 4, rejectionCases: 9, retryCases: 8, corePredecessorIsolation: true, kExecuted: false}));
