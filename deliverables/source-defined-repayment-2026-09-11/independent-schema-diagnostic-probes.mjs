import assert from 'node:assert/strict';
import {readFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const root = process.argv[2];
const {createFinancialAgreementSourceV1} = await import(pathToFileURL(root + '/experiments/moriarty-language/src/successor/financial-agreement-source-v1.ts'));
const src = readFileSync(root + '/experiments/moriarty-language/spec/successor/examples/source-defined-payment.mori', 'utf8');
const cases = [];
for (const [name, declaration] of [
  ['unit', 'unit some;'], ['asset', 'asset some: Asset;'],
  ['party', 'party some;'], ['record', 'record some { n: Text; }'],
  ['record field', 'record Extra { some: Text; }'],
  ['state', 'state some: UInt128;'], ['operation', 'operation some: TransferFields;'],
]) cases.push([name, src.replace('  unit Cash;', `  ${declaration}\n  unit Cash;`), 'some']);
cases.push(['parameter', src.replace('first: Quantity', 'some: Quantity'), 'some']);
for (const type of ['Operation<Repay>', 'Option<Operation<Repay>>', 'Collection<Operation<Repay>,1>']) {
  cases.push(['operation cycle ' + type, src.replace('payer: Text;', 'payer: ' + type + ';'), 'Operation<Repay>']);
}
let failed = 0;
for (const [name, source, token] of cases) {
  assert.notEqual(source, src, 'probe mutates source: ' + name);
  const input = '// UTF-8: λ🙂\n' + source;
  const result = createFinancialAgreementSourceV1().check(input);
  const span = result.span;
  const location = span?.kind === 'source' ? Buffer.from(input).subarray(Number(span.start), Number(span.end)).toString() : null;
  const passed = result.status === 'Rejected' && location?.includes(token) === true;
  console.log(JSON.stringify({name, passed, result, location}));
  if (!passed) failed++;
}
process.exitCode = failed ? 1 : 0;
