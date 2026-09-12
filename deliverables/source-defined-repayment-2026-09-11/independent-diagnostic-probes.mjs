import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
const root = process.argv[2];
const { createFinancialAgreementSourceV1 } = await import(pathToFileURL(`${root}/experiments/moriarty-language/src/successor/financial-agreement-source-v1.ts`));
const source = readFileSync(`${root}/experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori`, 'utf8');
const cases = [
  ['parameter unit', 'first: Quantity<Units<Cash,1>,0>', 'first: Quantity<Units<Missing,1>,0>', 'Missing'],
  ['state scaled asset', 'state paid: UInt128;', 'state paid: ScaledAmount<Missing,2>;', 'Missing'],
  ['parameter record', 'allocationId: Text) {', 'allocationId: Record<Missing>) {', 'Missing'],
  ['repay binding', 'nominalAmount: Quantity<Units<Cash,1>,0>;', 'nominalAmount: UInt128;', 'nominalAmount'],
];
let failed = 0;
for (const [name, before, after, expected] of cases) {
  assert(source.includes(before), `probe fixture ${name}`);
  const input = '// original UTF-8 offsets: λ🙂\n' + source.replace(before, after);
  const result = createFinancialAgreementSourceV1().check(input);
  const location = result.span?.kind === 'source'
    ? Buffer.from(input).subarray(Number(result.span.start), Number(result.span.end)).toString() : null;
  const passed = result.status === 'Rejected' && location?.includes(expected) === true;
  console.log(JSON.stringify({ name, passed, result, location }));
  if (!passed) failed++;
}
process.exitCode = failed ? 1 : 0;
