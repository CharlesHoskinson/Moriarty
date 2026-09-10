import { createExpressionContractV1 } from '../../experiments/moriarty-language/src/successor/expression-v1.ts';
import { canonical } from '../../experiments/moriarty-language/src/successor/expression-wire-v1.ts';

const span = { kind: 'synthetic', start: '0', end: '0' };
const node = (constructor, operands) => ({ constructor, operands, span });
const uint = value => node('LitUInt', { width: '128', value: String(value) });
const schema = {
  units: [], assets: [], vaults: [], parties: [], recordTypes: {}, enumTypes: {},
  fields: { x: { type: ['UInt128'], writeClass: 'ordinary' } },
  args: {}, observations: {}, operations: {},
};
const evaluator = createExpressionContractV1(canonical(schema));
const request = {
  contract: 'moriarty-expression-contract/1', source: '', workInitial: '100',
  Pre: { x: '10' }, Args: {}, Obs: {},
  core: { span, statements: [
    node('Let', { name: 'y', value: node('Add', {
      left: node('ReadPre', { view: 'pre', field: 'x' }), right: uint(1),
    }) }),
    node('NextWrite', { field: 'x', value: node('ReadLocal', { name: 'y' }) }),
    node('Ensure', { condition: node('Eq', {
      left: node('ReadPre', { view: 'post', field: 'x' }), right: uint(11),
    }) }),
  ] },
};
console.log(canonical(evaluator.evaluate(canonical(request))));
request.core.statements[2].operands.condition.operands.right = uint(12);
console.log(canonical(evaluator.evaluate(canonical(request))));
