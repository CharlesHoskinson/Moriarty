import test from 'node:test';
import assert from 'node:assert/strict';
import { mkdtempSync, writeFileSync, rmSync, readFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { spawnSync } from 'node:child_process';

const packageRoot = dirname(fileURLToPath(new URL('../package.json', import.meta.url)));
const tsconfig = join(packageRoot, 'tsconfig.json');

function compile(source) {
  const directory = mkdtempSync(join(tmpdir(), 'moriarty-legacy-funded-result-'));
  try {
    const consumer = join(directory, 'consumer.mts');
    writeFileSync(consumer, source);
    const config = join(directory, 'tsconfig.json');
    writeFileSync(config, JSON.stringify({
      extends: tsconfig,
      include: [join(packageRoot, 'src/**/*.ts'), consumer],
    }));
    return spawnSync('tsc', ['-p', config], { encoding: 'utf8', timeout: 30000 });
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
}

const v4Import = JSON.stringify(join(packageRoot, 'src/successor/financial-agreement-source-v4.ts'));
const v5Import = JSON.stringify(join(packageRoot, 'src/successor/financial-agreement-source-v5.ts'));
const repaymentSwitch = [
  "if(result.status==='FundedExpressionPrepared'){for(const effect of result.effects){",
  "switch(effect.kind){case 'Transfer':break;case 'Repayment':break;",
  'default:{const impossible:never=effect;void impossible;}}}}',
].join('');

test('source /4 public evaluate still typechecks a Transfer/Repayment-only effect switch', () => {
  const compiled = compile(
    `import {createFinancialAgreementSourceV4} from ${v4Import};\n`
    + "const result=createFinancialAgreementSourceV4().evaluate('', '', '', '');\n"
    + repaymentSwitch + '\n',
  );
  assert.equal(compiled.status, 0, compiled.stdout + compiled.stderr);
});

test('source /5 public evaluate is a distinct lifecycle result: Transfer/Repayment-only switch fails', () => {
  const compiled = compile(
    `import {createFinancialAgreementSourceV5} from ${v5Import};\n`
    + "const result=createFinancialAgreementSourceV5().evaluate('', '', '', '');\n"
    + repaymentSwitch + '\n',
  );
  assert.notEqual(compiled.status, 0, compiled.stdout + compiled.stderr);
  assert.match(compiled.stdout + compiled.stderr, /TS2322/);
  assert.match(compiled.stdout + compiled.stderr, /Origination|Accrual/);
});

test('checked-in source /4 exhaustive consumer matches the live public factory', () => {
  const consumer = readFileSync(join(packageRoot, 'src/successor/legacy-funded-result-consumer.ts'), 'utf8');
  assert.match(consumer, /createFinancialAgreementSourceV4/);
  assert.match(consumer, /case 'Transfer'/);
  assert.match(consumer, /case 'Repayment'/);
  assert.match(consumer, /never/);
  assert.equal(/case 'Origination'/.test(consumer), false);
  assert.equal(/case 'Accrual'/.test(consumer), false);
});
