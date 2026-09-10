import assert from 'node:assert/strict';
import test from 'node:test';
import { createExpressionContractV1 } from '../src/successor/expression-v1.ts';
import { B, Arg, J, inputs, run } from './expression-v1-support.mjs';

test('schema rejection follows ASCII identifier order rather than locale collation', () => {
  const p = inputs();
  p.schema.args.A = ['Amount', 'MissingAsset'];
  p.schema.args.a = ['Rate', '19'];
  // A precedes a in the canonical scalar order. MissingAsset must therefore
  // reject before the independently invalid scale on the lowercase binding.
  assert.equal(run(B(true), p).code, 'TYPE_NAME');
  delete p.schema.args.A;
  assert.equal(run(B(true), p).code, 'TYPE_LITERAL');
});

test('returned type arrays cannot change the schema of a later evaluation', () => {
  const p = inputs(); p.schema.args.value = ['UInt128']; p.Args.value = '7';
  const evaluator = createExpressionContractV1(J(p.schema));
  const request = {
    contract: 'moriarty-expression-contract/1', source: '', core: Arg('value'),
    Pre: p.Pre, Args: p.Args, Obs: p.Obs, workInitial: '1',
  };
  const first = evaluator.evaluate(J(request));
  assert.equal(first.judgmentResult, 'ExpressionValue');
  try { first.type[0] = 'Bool'; } catch { /* Frozen output is also acceptable. */ }
  const second = evaluator.evaluate(J(request));
  assert.deepEqual(second.type, ['UInt128']);
  assert.equal(second.value, '7');
  request.Args.value = true;
  assert.equal(evaluator.evaluate(J(request)).code, 'INPUT_SCHEMA');
});
