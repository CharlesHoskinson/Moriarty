import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import path from 'node:path';
const root = path.resolve(process.argv[2]);
const imp = (p) => import(pathToFileURL(path.join(root, 'experiments/moriarty-language', p)).href);
const { createFinancialAgreementSourceV5 } = await imp('src/successor/financial-agreement-source-v5.ts');
const { createFinancialExpressionContractV4 } = await imp('src/successor/financial-expression-v4.ts');
const { canonical } = await imp('src/successor/expression-wire-v1.ts');
const read = (p) => readFileSync(path.join(root, 'experiments/moriarty-language/spec/successor/examples', p), 'utf8');
const source = read('loan-lifecycle.mori');
const oracle = JSON.parse(readFileSync(new URL('root-complete-oracle-01.json', import.meta.url), 'utf8'));
assert.equal(createHash('sha256').update(source).digest('hex'), oracle.sourceSHA256);
const language = createFinancialAgreementSourceV5();
const elaborated = language.elaborate(source);
assert.equal(elaborated.judgmentResult, 'SourceElaborated');
const initial = { post: JSON.parse(read('loan-lifecycle.snapshots.json')).Pre, financialPost: JSON.parse(read('loan-lifecycle.state.json')) };
const args = [{ transferId: 'D1', originationId: 'O1' }, { accrualId: 'A1', periodIndex: '1', observedTime: '1060' }, { transferId: 'P1', allocationId: 'R1', nominal: '30' }, { transferId: 'P2', allocationId: 'R2' }];
const records = [];
function packet(action, predecessor, actionArgs) {
  const item = elaborated.actions.find((item) => item.action === action);
  const snapshot = { Pre: predecessor.post, Args: actionArgs, Obs: {}, workInitial: predecessor.financialPost.work.remaining };
  return { schema: canonical(item.schema), request: canonical({ contract: elaborated.contract, source, core: item.core, ...snapshot }), financialPreState: JSON.stringify(predecessor.financialPost), snapshot: canonical(snapshot) };
}
function core(p, financial = p.financialPreState) { return createFinancialExpressionContractV4(p.schema, financial).evaluate(p.request); }
let predecessor = initial;
const packets = [];
for (let i = 0; i < oracle.stages.length; i++) {
  const stage = oracle.stages[i];
  const p = packet(stage.action, predecessor, args[i]); packets.push(p);
  const before = JSON.stringify(predecessor);
  const actualSource = language.evaluate(source, stage.action, p.snapshot, p.financialPreState);
  const actualCore = core(p);
  assert.deepEqual(actualSource, stage.expected);
  assert.deepEqual(actualCore, stage.expected);
  assert.equal(JSON.stringify(predecessor), before);
  records.push({ id: stage.action, sourceAndCore: 'match complete independent oracle' });
  predecessor = actualSource;
}
const p = packets[0];
const expected = oracle.stages[0].expected;
for (const [id, text] of [
  ['financial-whitespace', ' \n' + p.financialPreState + '\t'],
  ['financial-last-duplicate-key', p.financialPreState.replace('"remaining":"512"', '"remaining":"0","remaining":"512"')],
  ['financial-unicode-escape', p.financialPreState.replace('Lender', '\\u004cender')],
]) { assert.deepEqual(core(p, text), expected); records.push({ id, result: 'complete success' }); }
for (const text of ['{', '', 'nul']) {
  assert.deepEqual(core(p, text), { status: 'Rejected', code: 'INPUT_SCHEMA' });
  records.push({ id: 'malformed-financial-' + JSON.stringify(text), result: 'INPUT_SCHEMA without kernel fields' });
}
assert.deepEqual(createFinancialExpressionContractV4(p.schema).evaluate(p.request), { status: 'Rejected', code: 'FINANCIAL_CONTEXT_REQUIRED', span: { kind: 'synthetic', start: '0', end: '0' }, nodePath: [], workUsed: '0' });
records.push({ id: 'omitted-context', result: 'FINANCIAL_CONTEXT_REQUIRED' });
const badRequest = JSON.parse(p.request); badRequest.contract = 'wrong-contract';
const earlier = createFinancialExpressionContractV4(p.schema, p.financialPreState).evaluate(canonical(badRequest));
assert.equal(earlier.status, 'Rejected');
assert.deepEqual(createFinancialExpressionContractV4(p.schema, '{').evaluate(canonical(badRequest)), earlier);
records.push({ id: 'static-request-before-financial-parse', result: earlier });
const mismatch = JSON.parse(p.request); mismatch.workInitial = '511';
assert.deepEqual(createFinancialExpressionContractV4(p.schema, p.financialPreState).evaluate(canonical(mismatch)), { status: 'Rejected', code: 'WORK_MISMATCH' });
records.push({ id: 'work-mismatch', result: 'WORK_MISMATCH' });
console.log(JSON.stringify({ sourceSHA256: oracle.sourceSHA256, actualPublicAPIChecks: records.length, records, kInvocations: 0, scope: 'Independent Source/5 and direct Core/4 leg; no K agreement claim' }, null, 2));
