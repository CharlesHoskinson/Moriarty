import test from 'node:test';
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { validateDesigners } from '../src/designers.mjs';

const here = dirname(fileURLToPath(import.meta.url));
const manifest = JSON.parse(readFileSync(join(here, '../examples/revision-step.manifest.json'), 'utf8'));
const designers = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers';

test('20-pattern matrix resolves nested 01 source and separates SOURCE_ACTION_UNKNOWN from OPERATION_BINDING', () => {
  const matrix = validateDesigners({ designers, manifest, compileExample: false });
  assert.equal(matrix.patternCount, 20);
  assert.equal(matrix.verifiers.length, 5);
  assert.equal(matrix.verifiers.every(v => v.ok && v.exit_code === 0), true);
  assert.equal(matrix.discrepancies.some(d => d.reason === 'missing-source'), false, JSON.stringify(matrix.discrepancies));

  const s03 = matrix.rows.filter(r => r.id === 'S03');
  const s03Unknown = s03.find(r => r.file === 'head-step.mori');
  const s03Probe = s03.find(r => r.file === 'unsupported-split.mori');
  assert.ok(s03Unknown, `S03 rows ${JSON.stringify(s03.map(r => r.file))}`);
  assert.ok(s03Probe, 'S03 probe file missing');
  assert.equal(s03Unknown.source.code, 'SOURCE_ACTION_UNKNOWN');
  assert.equal(s03Probe.source.code, 'OPERATION_BINDING');

  const s04 = matrix.rows.filter(r => r.id === 'S04');
  assert.ok(s04.length, 'S04 missing from matrix');
  assert.equal(s04.some(r => r.source.status === 'missing-source'), false, JSON.stringify(s04.map(r => r.source)));
  assert.equal(s04[0].source.code, 'SOURCE_ACTION_UNKNOWN');
  assert.equal(s04[0].file, 'head-step.mori');

  const unknownSkip = matrix.rows.filter(r => r.source.status === 'missing-source' || r.source.status === 'skipped-incomplete-input');
  assert.equal(unknownSkip.length, 0, JSON.stringify(unknownSkip));
  assert.equal(matrix.ok, true, JSON.stringify(matrix.discrepancies));
});
