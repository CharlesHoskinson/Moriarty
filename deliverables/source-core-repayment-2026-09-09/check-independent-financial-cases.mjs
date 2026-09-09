import { readFileSync } from 'node:fs';
import assert from 'node:assert/strict';
import { prepareRepayment } from '../../experiments/moriarty-language/src/successor/repayment.ts';
const { cases } = JSON.parse(readFileSync(new URL('./independent-financial-cases.json', import.meta.url), 'utf8'));
for (const c of cases) assert.deepEqual(prepareRepayment(JSON.stringify(c.input)), c.expected, c.id);
console.log(`${cases.length} independently specified full financial observations matched the reference kernel`);
