import assert from 'node:assert/strict';
import { readFileSync, existsSync } from 'node:fs';
import test from 'node:test';

const runtimeURL = new URL('../src/successor/expression-v1.ts', import.meta.url);
const runtime = existsSync(runtimeURL) ? await import(runtimeURL.href) : {};
const canon = v => Array.isArray(v) ? '[' + v.map(canon).join(',') + ']'
  : v !== null && typeof v === 'object' ? '{' + Object.keys(v).sort().map(k => JSON.stringify(k) + ':' + canon(v[k])).join(',') + '}'
  : JSON.stringify(v);
const fixtures = JSON.parse(readFileSync(new URL('../spec/successor/boolean-cases.json', import.meta.url)));

test('the versioned expression API exists independently of funded preparation', () => {
  assert.equal(typeof runtime.createExpressionContractV1, 'function');
});

for (const row of fixtures.cases) test(row.id + ': actual execution', () => {
  assert.equal(typeof runtime.createExpressionContractV1, 'function', 'expression runtime is missing');
  const p = fixtures.inputProfiles[row.inputProfile];
  const evaluator = runtime.createExpressionContractV1(canon(p.schema));
  const actual = evaluator.evaluate(canon({
    contract: 'moriarty-expression-contract/1', source: row.source, core: row.core,
    Pre: p.Pre, Args: p.Args, Obs: p.Obs, workInitial: row.workInitial,
  }));
  assert.deepEqual(actual, row.expected);
});
