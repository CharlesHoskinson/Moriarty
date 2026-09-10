import assert from 'node:assert/strict';
import { createExpressionContractV1 } from '../../experiments/moriarty-language/src/successor/expression-v1.ts';
// Independent canonical writer; this reproducer contains only ASCII keys.
const J = v => typeof v !== 'object' ? JSON.stringify(v) : Array.isArray(v)
  ? '[' + v.map(J).join(',') + ']'
  : '{' + Object.keys(v).sort().map(k => JSON.stringify(k) + ':' + J(v[k])).join(',') + '}';
const schema = {units:[],assets:[],vaults:[],parties:[],recordTypes:{},enumTypes:{},fields:{},args:{},observations:{},operations:{}};
let type = ['Bool'];
for (let i = 0; i < 509; i++) type = ['Option', type];
const core = {constructor:'ConstructNone', operands:{elementType:type}, span:{kind:'synthetic',start:'0',end:'0'}};
const request = {contract:'moriarty-expression-contract/1',source:'',core,Pre:{},Args:{},Obs:{},workInitial:'1'};
const actual = createExpressionContractV1(J(schema)).evaluate(J(request));
console.log(JSON.stringify({coreBytes:Buffer.byteLength(J(core)),valueWBytes:Buffer.byteLength(J({type:['Option',type],value:[]})),actual},null,2));
// Normative expectation: one AST node, one None value node, constructor/value
// depth1, both encodings below65536. Type metadata adds no value depth.
assert.equal(actual.judgmentResult, 'ExpressionValue');
assert.deepEqual(actual.value, []);
assert.equal(actual.workRemaining, '0');
