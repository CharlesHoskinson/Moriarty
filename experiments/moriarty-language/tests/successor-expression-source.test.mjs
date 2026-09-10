import test from 'node:test';
import assert from 'node:assert/strict';
import { parseSuccessorSource } from '../src/successor/frontend.ts';
import { createSourceLowering, sourceNode } from '../src/successor/expression-source-lower.ts';
import { createExpressionContractV1 } from '../src/successor/expression-v1.ts';
import { canonical } from '../src/successor/expression-wire-v1.ts';

const schema = { units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {},
  fields: { flag: { type: ['Bool'], writeClass: 'ordinary' } }, args: { ready: ['Bool'] }, observations: {}, operations: {} };
function run(body, ready = true) {
  const source = `profile "moriarty-successor-syntax/0"; agreement Test { action step(ready: Bool) { ${body} } }`;
  const parsed = parseSuccessorSource(source), action = parsed.agreement.declarations[0];
  const lower = createSourceLowering(schema, new Set(['ready']), e => {
    if (e.tag === 'IntegerLiteral') return sourceNode('LitUInt', { width: '128', value: e.value }, e.span);
    return undefined;
  });
  const core = { statements: [...action.statements, ...action.postconditions].map(lower.statement), span: { kind: 'source', start: String(action.span.start), end: String(action.span.end) } };
  const result = createExpressionContractV1(canonical(schema)).evaluate(canonical({ contract: 'moriarty-expression-contract/1', source, core,
    Pre: { flag: false }, Args: { ready }, Obs: {}, workInitial: '100' }));
  return { source, core, result };
}

test('real syntax/0 Boolean statements lower to Core and evaluate immutable pre/post', () => {
  const { result } = run('let condition = ready and not pre.flag; requires condition; next.flag = condition; ensures post.flag and not pre.flag;');
  assert.equal(result.status, 'ExpressionPrepared');
  assert.deepEqual(result.post, { flag: true });
});
test('wrong type in an unselected source Boolean branch rejects statically', () => {
  const { result } = run('requires false and 1;');
  assert.equal(result.code, 'TYPE_MISMATCH');
  assert.equal(result.workUsed, '0');
});
test('source dead branch skips dynamic division failure, selected branch retains source span', () => {
  assert.equal(run('requires true or floor_div(1, 0) == 0;').result.status, 'ExpressionPrepared');
  const { source, result } = run('requires false or floor_div(1, 0) == 0;');
  assert.equal(result.code, 'ARITH_DENOMINATOR');
  assert.equal(Buffer.from(source).subarray(Number(result.span.start), Number(result.span.end)).toString(), 'floor_div(1, 0)');
});
test('source ensure failure rolls back staged writes', () => {
  const { result } = run('next.flag = true; ensures false;');
  assert.equal(result.code, 'ENSURES_FAILED');
  assert.equal(Object.hasOwn(result, 'post'), false);
});

test('runtime exposes static checking without requiring or executing snapshots', () => {
  const { source, core } = run('requires ready; next.flag = true;');
  const runtime = createExpressionContractV1(canonical(schema));
  const result = runtime.check(canonical({ contract: 'moriarty-expression-contract/1', source, core,
    Pre: {}, Args: {}, Obs: {}, workInitial: '0' }));
  assert.deepEqual(result, { judgmentResult: 'ExpressionChecked' });
});

test('source later static mistakes precede failed guards and snapshots', () => {
  for (const [body, code] of [
    ['requires false; requires missing;', 'TYPE_NAME'],
    ['requires false; next.flag = true; next.flag = false;', 'TYPE_DUPLICATE_WRITE'],
    ['requires false; let ready = true;', 'TYPE_DUPLICATE_BINDER'],
    ['requires false; requires post.flag;', 'TYPE_POST_SCOPE'],
  ]) {
    const { result } = run(body);
    assert.equal(result.code, code);
    assert.equal(result.workUsed, '0');
  }
});
test('UTF-8 source span survives earlier multibyte text', () => {
  const { source, result } = run('let label = "é😀"; requires floor_div(1, 0) == 0;');
  assert.equal(result.code, 'ARITH_DENOMINATOR');
  assert.equal(Buffer.from(source).subarray(Number(result.span.start), Number(result.span.end)).toString(), 'floor_div(1, 0)');
});
