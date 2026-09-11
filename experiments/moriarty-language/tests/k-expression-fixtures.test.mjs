import test from 'node:test';
import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {createExpressionContractV1} from '../src/successor/expression-v1.ts';
import {canonical} from '../src/successor/expression-wire-v1.ts';
const fixture=JSON.parse(readFileSync(new URL('../formal/k/fixtures/expression-v1.json',import.meta.url)));
for(const row of fixture.cases)test('independent K-input expectation vs published all40: '+row.id,()=>{
 const actual=createExpressionContractV1(row.schema).evaluate(row.request);
 assert.equal(canonical(actual),canonical(row.expected));
});
