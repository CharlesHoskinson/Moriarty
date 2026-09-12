import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const root = process.argv[2];
const baseline = '/home/charl/Moriarty/.worktrees/source-defined-repayment';
const modulePath = '/experiments/moriarty-language/src/successor/financial-agreement-source-v1.ts';
const oldApi = (await import(pathToFileURL(baseline + modulePath))).createFinancialAgreementSourceV1;
const newApi = (await import(pathToFileURL(root + modulePath))).createFinancialAgreementSourceV1;
const ex = baseline + '/experiments/moriarty-language/spec/successor/examples/';
const source = readFileSync(ex + 'source-defined-payment.mori', 'utf8');
const snapshots = readFileSync(ex + 'expression-funded-payment.snapshots.json', 'utf8');
const state = readFileSync(ex + 'expression-funded-payment.state.json', 'utf8');
const parameterFaults = [
  ['none', x => x],
  ['duplicate', x => x.replace('second: Quantity<Units<Cash,1>,0>', 'first: Quantity<Units<Cash,1>,0>')],
  ['shared name', x => x.replace('second: Quantity<Units<Cash,1>,0>', 'due: Quantity<Units<Cash,1>,0>')],
  ['unknown unit', x => x.replace('second: Quantity<Units<Cash,1>,0>', 'second: Quantity<Units<Missing,1>,0>')],
];
const operationFaults = [
  ['none', x => x],
  ['missing', x => x.replace('operation Repay: RepayFields;', '')],
  ['extra', x => x.replace('operation Repay: RepayFields;', 'operation Repay: RepayFields; operation Notice: RepayFields;')],
  ['type mismatch', x => x.replace('nominalAmount: Quantity<Units<Cash,1>,0>;', 'nominalAmount: UInt128;')],
];
const results = [];
let failures = 0;
for (const [pn, pm] of parameterFaults) for (const [on, om] of operationFaults) {
  const input = '// UTF-8 λ🙂\n' + om(pm(source));
  for (const method of ['check', 'elaborate', 'evaluate']) {
    const args = method === 'evaluate' ? [input, snapshots, state] : [input];
    const expected = oldApi()[method](...args);
    const actual = newApi()[method](...args);
    let passed = true;
    try { assert.deepEqual(actual, expected); } catch { passed = false; failures++; }
    results.push({case: `${pn}/${on}/${method}`, passed,
      ...(passed ? {status: actual.status ?? actual.judgmentResult} : {expected, actual})});
  }
}
console.log(JSON.stringify({passed: results.length - failures, failed: failures, results}, null, 2));
process.exitCode = failures ? 1 : 0;
