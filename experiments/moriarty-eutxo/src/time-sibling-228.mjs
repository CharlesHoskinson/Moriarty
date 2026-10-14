/**
 * UNIT/SYNTHETIC sibling compile. This is not a production artifact and not a ledger run.
 * The designer source file is read and left unchanged.
 */
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { compileCustody } from './compile-custody.mjs';

const DESIGNER = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time';
const manifest = JSON.parse(readFileSync(new URL('../examples/custody-unit.manifest.json', import.meta.url), 'utf8'));
const outDir = process.argv[2];
const synthetic = process.argv.includes('--synthetic');
if (!outDir) {
  process.stderr.write('usage: time-sibling-228.mjs <outDir> [--synthetic]\n');
  process.exit(2);
}

const syntheticTime = synthetic
  ? {
    evidenceClass: 'UNIT/SYNTHETIC',
    productionArtifact: false,
    bounds: {
      releaseLower: '1700000030000',
      partition: '1700000090000',
      refundInclusive: '1700000149999',
    },
  }
  : null;

const result = compileCustody({
  sourcePath: join(DESIGNER, 'escrow-custody-v5.mori'),
  seedPath: join(DESIGNER, 'financial-seed-custody-v5.json'),
  casesPath: join(DESIGNER, 'cases.json'),
  expectedReleasePath: join(DESIGNER, 'E13-release-real-custody.expected.json'),
  expectedRefundPath: join(DESIGNER, 'E11-refund-real-custody.expected.json'),
  residualPath: join(DESIGNER, 'financial-residual-custody-v5.json'),
  manifest,
  outDir,
  syntheticTime,
});

const summary = {
  ok: result.ok === true,
  code: result.code || null,
  detail: result.detail || null,
  evidenceClass: synthetic ? 'UNIT/SYNTHETIC' : 'source-uplc-unit',
  productionArtifact: false,
  ledgerExecution: result.ledgerExecution === true,
  nativePins: result.nativePins === true,
  checkStatus: result.check?.status ?? result.checkStatus ?? null,
  buildStatus: result.build?.status ?? result.buildStatus ?? null,
  missingTests: result.missingTests || null,
  blueprintPath: result.blueprintPath || null,
  layer: result.layer || null,
};
process.stdout.write(`${JSON.stringify(summary, null, 2)}\n`);
process.exitCode = result.ok === true ? 0 : 1;
