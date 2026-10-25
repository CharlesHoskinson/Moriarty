import assert from 'node:assert/strict';
import {mkdtempSync, readFileSync, rmSync} from 'node:fs';
import {join} from 'node:path';
import {tmpdir} from 'node:os';
import test from 'node:test';
import {submissionRefusal, recordSubmissionRefusal, quietSubmission} from './rollup-refusal.mjs';

test('extracts only public RPC refusal codes through cyclic Effect causes', () => {
  for (const code of [115, 104, 244]) {
    const error = new Error(`1010: Invalid Transaction: Custom error: ${code}`);
    Object.defineProperty(error, 'txData', {get() { throw Error('private transaction inspected'); }});
    const outer = new Error('PRIVATE_TEST_SENTINEL wallet value 104');
    outer[Symbol.for('effect/Runtime/FiberFailure/Cause')] = {_tag: 'Fail', error};
    error.cause = outer;
    assert.deepEqual(submissionRefusal(outer), {kind: 'rpc-invalid-transaction', codes: [code]});
  }
  assert.deepEqual(submissionRefusal(Error('PRIVATE_TEST_SENTINEL wallet value 115')),
    {kind: 'unclassified', codes: []});
  assert.deepEqual(submissionRefusal(Error('1010: Invalid Transaction: Custom error: 115123456')),
    {kind: 'unclassified', codes: []});
});

test('persists and logs no provider details and never proposes a retry', () => {
  const root = mkdtempSync(join(tmpdir(), 'rollup-refusal-')), original = console.error, logs = [];
  try {
    console.error = text => logs.push(text);
    recordSubmissionRefusal(new Error('PRIVATE_TEST_SENTINEL 1010: Invalid Transaction: Custom error: 104'), root, 'settleValidity');
    const bytes = readFileSync(join(root, 'refusal.json'), 'utf8'), report = JSON.parse(bytes);
    assert.deepEqual(report.codes, [104]); assert.equal(report.willRetry, false);
    assert.equal(report.purpose, 'settleValidity');
    assert.ok(!bytes.includes('PRIVATE_TEST_SENTINEL') && !logs.join('').includes('PRIVATE_TEST_SENTINEL'));
    assert.match(logs.join(''), /104/);
  } finally { console.error = original; rmSync(root, {recursive: true, force: true}); }
});

test('suppresses the SDK logger before an RPC failure reaches the application catch', async () => {
  const names = ['log', 'error', 'warn', 'info', 'debug', 'trace'];
  const original = Object.fromEntries(names.map(name => [name, console[name]])), logs = [];
  try {
    for (const name of names) console[name] = (...args) => logs.push(args);
    for (const fail of [false, true]) {
      const operation = async () => {
        await Promise.resolve();
        for (const name of names) console[name]('PRIVATE_TEST_SENTINEL');
        if (fail) throw Error('1010: Invalid Transaction: Custom error: 115 PRIVATE_TEST_SENTINEL');
        return 'submitted';
      };
      if (fail) await assert.rejects(quietSubmission(operation), error => submissionRefusal(error).codes[0] === 115);
      else assert.equal(await quietSubmission(operation), 'submitted');
      assert.deepEqual(logs, []);
      console.error('public output restored');
      assert.deepEqual(logs.pop(), ['public output restored']);
    }
  } finally { Object.assign(console, original); }
});
