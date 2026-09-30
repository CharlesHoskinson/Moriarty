import test from 'node:test';
import assert from 'node:assert/strict';
import { parseBoundedJson } from '../src/json.ts';
test('rejects decoded duplicate names before lossy JSON parsing', () => {
  assert.throws(() => parseBoundedJson('{"head":"a","\\u0068ead":"b"}'), e=>e.code==='BETA_JSON_DUPLICATE');
});
test('bounds and scalar-validates JSON rather than invoking opaque object input', () => {
  assert.throws(()=>parseBoundedJson('{"x":"\\ud800"}'),e=>e.code==='BETA_JSON_UNICODE');
  assert.throws(()=>parseBoundedJson('['.repeat(34)+'0'+']'.repeat(34)),e=>e.code==='BETA_JSON_DEPTH');
  assert.throws(()=>parseBoundedJson(' '.repeat(65537)),e=>e.code==='BETA_JSON_BOUND');
  assert.throws(()=>parseBoundedJson({}),e=>e.code==='BETA_JSON_TEXT');
});
test('preserves plain data and closes syntax without accepting trailing material',()=>{
  assert.deepEqual(parseBoundedJson('{"a":[true,null,"😀",1]}'),{a:[true,null,'😀',1]});
  assert.throws(()=>parseBoundedJson('{} {}'),e=>e.code==='BETA_JSON_SYNTAX');
});
