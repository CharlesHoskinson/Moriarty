import test from 'node:test';
import assert from 'node:assert/strict';
import { createExpressionContractV1 } from '../src/successor/expression-v1.ts';
import { createFinancialExpressionContractV4 } from '../src/successor/financial-expression-v1.ts';
import { expressionUnicodeCases, lifecycleUnicodeCases } from '../formal/k/fixtures/unicode-cases.mjs';

for (const row of expressionUnicodeCases) test(row.id, () => {
  const request = JSON.parse(row.request);
  const text = row.id.includes('-text-') ? request.core.operands.value : request.source;
  assert.equal(Buffer.byteLength(text), Number(row.id.split('-').at(-1)));
  assert.deepEqual(createExpressionContractV1(row.schema).evaluate(row.request), row.expected);
});
for (const row of lifecycleUnicodeCases) test(row.id, () => {
  const { schema, request, financialPreState } = row.packet;
  const text = row.id.includes('-source-') ? JSON.parse(request).source : financialPreState;
  assert.equal(Buffer.byteLength(text), Number(row.id.split('-').at(-1)));
  assert.ok(text.length < Buffer.byteLength(text));
  assert.deepEqual(createFinancialExpressionContractV4(schema, financialPreState).evaluate(request), row.expected);
});

test('independent expected counts for later native string-hook probes', async () => {
  const { unicodeCounterCases } = await import('../formal/k/fixtures/unicode-cases.mjs');
  for (const row of unicodeCounterCases) {
    assert.equal(Buffer.byteLength(row.text), row.utf8);
    assert.equal(row.text.length, row.utf16);
    assert.equal(Buffer.byteLength(JSON.stringify(row.text)), row.quotedBytes);
    assert.equal(JSON.stringify(row.text).length, row.quotedUnits);
  }
});
