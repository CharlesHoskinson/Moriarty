// Reproduce only this directory's candidate tests; no financial/native claims.
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { encodeAuthorization, authorizationDigest } from './codec.mjs';

const dir = new URL('./', import.meta.url);
const cwd = fileURLToPath(new URL('../../../../../', import.meta.url));
const testPath = 'experiments/moriarty-language/formal/mil4/wire/codec.test.mjs';
const args = ['--test', '--test-reporter=tap', testPath];
const started = new Date().toISOString();
const run = spawnSync(process.execPath, args, { cwd, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
const output = (run.stdout ?? '') + (run.stderr ?? '');
await writeFile(new URL('test-output.tap', dir), output);
const counts = {};
for (const name of ['tests', 'pass', 'fail', 'cancelled', 'skipped', 'todo']) {
  const match = output.match(new RegExp(`^# ${name} (\\d+)$`, 'm'));
  counts[name] = match ? Number(match[1]) : null;
}
const { fixtures } = JSON.parse(await readFile(new URL('fixtures.json', dir)));
const referencePath = 'experiments/moriarty-language/formal/mil4/wire/reference-vectors.py';
const referenceArgs = [referencePath, '--json'];
const reference = spawnSync('python3', referenceArgs, { cwd, encoding: 'utf8', maxBuffer: 8 * 1024 * 1024 });
let referenceResult = null;
if (reference.status === 0) referenceResult = JSON.parse(reference.stdout);
const pythonVersion = spawnSync('python3', ['--version'], { cwd, encoding: 'utf8' });
const comparisons = fixtures.map(fixture => ({
  id: fixture.id,
  encodedBytes: encodeAuthorization(fixture.authorization).length,
  exactBytesMatch: encodeAuthorization(fixture.authorization).toString('hex') === fixture.expected.wireHex,
  digest: authorizationDigest(fixture.authorization),
  expectedDigestMatch: authorizationDigest(fixture.authorization) === fixture.expected.digestHex,
  goldenWireDigestMatch: createHash('sha256').update(Buffer.from(fixture.expected.wireHex, 'hex')).digest('hex') === fixture.expected.digestHex,
  independentDigestMatch: referenceResult?.vectors.find(value => value.id === fixture.id)?.calculated.digestHex === authorizationDigest(fixture.authorization),
}));
const names = ['SPEC.md', 'PLAN.md', 'codec.mjs', 'codec.test.mjs', 'fixtures.json', 'record-results.mjs', 'reference-vectors.py', 'REFERENCE-VECTORS-RECEIPT.md', 'RESULT.md', 'red-output.tap', 'post-audit-red-output.tap', 'metadata-snapshot-red-output.tap', 'test-output.tap'];
const artifacts = {};
for (const name of names) artifacts[name] = createHash('sha256').update(await readFile(new URL(name, dir))).digest('hex');
const result = {
  status: 'PROVISIONAL_W_D2_CODEC_EXPERIMENT', startedAt: started, finishedAt: new Date().toISOString(),
  cwd, command: [process.execPath, ...args], runtime: process.version, platform: process.platform,
  exitCode: run.status, signal: run.signal, spawnError: run.error?.message ?? null,
  counts, fixtureComparisons: comparisons, artifacts,
  independentReference: {
    command: ['python3', ...referenceArgs], runtime: pythonVersion.stdout.trim(),
    exitCode: reference.status, signal: reference.signal, spawnError: reference.error?.message ?? null,
    stderr: reference.stderr, passed: referenceResult?.passed === true,
    vectors: referenceResult?.vectors ?? null,
    provenance: 'post-audit independent reproduction; no claim that original vectors preceded Node codec',
  },
  acceptedClaims: ['targeted canonical byte/digest fixture agreement', 'targeted round-trip and hostile codec outcomes'],
  unverified: ['wallet/signature interoperability', 'effect commitment correspondence', 'Source/Core/K/Quint correspondence', 'native proof/hash binding', 'ledger verification and atomic acceptance', 'cap feasibility', 'W-D1/W-D2 normative closure'],
};
await writeFile(new URL('results.json', dir), JSON.stringify(result, null, 2) + '\n');
const passed = run.status === 0 && counts.fail === 0 && counts.tests === 114 && counts.pass === 114
  && reference.status === 0 && referenceResult?.passed === true
  && comparisons.every(value => value.exactBytesMatch && value.expectedDigestMatch && value.goldenWireDigestMatch && value.independentDigestMatch);
console.log(JSON.stringify({ passed, exitCode: run.status, counts, fixtureComparisons: comparisons }, null, 2));
if (!passed) process.exitCode = 1;
