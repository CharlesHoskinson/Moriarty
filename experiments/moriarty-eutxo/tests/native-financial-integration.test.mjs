import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { ENCODING_VKEY_HEX, encodeUnsignedBody, paymentKeyHashOfVkeyCbor } from '../src/native-body.mjs';
import { compareClockParity, runNativeCaller } from '../src/native-caller.mjs';
import { bindSignedWitnessCount, countVkeyWitnesses } from '../src/native-integrate.mjs';
import { dispatchOwnedProducer, selectOwnedFunder } from '../src/native-producer.mjs';
import { parseQueryUtxoText } from '../src/native-bootstrap.mjs';
import { produceOwnedRuntime, queryOwnedUtxos } from '../src/native-runtime.mjs';
import { refuseCallerMinAdaFormula } from '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-71/own71/schemas.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const OWN = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-f207';
const CHECKS = join(OWN, 'checks', 'integrate');
mkdirSync(CHECKS, { recursive: true });
const PROTOCOL = join(TASK, 'tooling/runtime/actual-protocol-parameters.json');
const CLI = join(TASK, 'tooling/runtime/devnet/cardano-node/bin/cardano-cli');

function bindUnixSocket(path) {
  mkdirSync(dirname(path), { recursive: true });
  if (existsSync(path)) rmSync(path);
  const run = spawnSync('python3', ['-c', 'import socket,sys; p=sys.argv[1]; s=socket.socket(socket.AF_UNIX); s.bind(p); s.listen(1)', path], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(run.stderr || 'unix-socket');
}

function nativeReviews(dir) {
  mkdirSync(dir, { recursive: true });
  const sourcePath = join(dir, 'source.txt');
  const sourceText = 'financial-integration-probe\n';
  writeFileSync(sourcePath, sourceText);
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const body = JSON.stringify({
    sourceHash,
    files: [{ path: sourcePath, sha256: sourceHash, bytes: Buffer.byteLength(sourceText) }],
  });
  writeFileSync(join(dir, 'manifest.json'), body);
  writeFileSync(join(dir, 'candidate.json'), body);
  const manifestHash = createHash('sha256').update(body).digest('hex');
  writeFileSync(join(dir, 'authority.json'), JSON.stringify({ manifestSha256: manifestHash }));
  const review = (reviewer) => ({
    verdict: 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION',
    evidenceClass: 'reviewed-implementation',
    reviewClass: 'native-implementation',
    reviewer,
    candidate: manifestHash,
    sourceHash,
    selfApproval: false,
    unitGateProbe: true,
  });
  writeFileSync(join(dir, 'a.json'), JSON.stringify(review('probe-a')));
  writeFileSync(join(dir, 'b.json'), JSON.stringify(review('probe-b')));
  return {
    reviewA: join(dir, 'a.json'),
    reviewB: join(dir, 'b.json'),
    candidate: join(dir, 'candidate.json'),
    manifest: join(dir, 'manifest.json'),
    reviewAuthority: join(dir, 'authority.json'),
  };
}

function ownedRequest(home, runId, reviews) {
  return {
    executeNow: true,
    arm: true,
    authority: reviews,
    ownedHome: home,
    runId,
    nodePort: 14041,
    submitApiPort: 14042,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    supplyLovelace: 9000000,
  };
}

test('unarmed caller keeps the financial continuation and does not execute parity', async () => {
  const root = mkdtempSync(join(CHECKS, 'unarmed-'));
  const reviews = nativeReviews(join(root, 'reviews'));
  const report = await runNativeCaller({
    authority: reviews,
    ownedProducer: ownedRequest(join(root, 'owned'), 't1-104-probe', reviews),
  });
  assert.equal(report.code, 'REAL_NODE_NOT_ARMED');
  assert.equal(report.ledgerExecution, false);
  assert.equal(report.nativePins, false);
  assert.equal(report.stages.ownedProducer.spawned, false);
  const financial = report.stages.financial;
  assert.equal(financial.code, 'FINANCIAL_CONTINUATION_BOUND');
  assert.equal(financial.kernel.cashQuantity, '100');
  assert.equal(financial.kernel.headQuantity, '1');
  assert.equal(financial.kernel.constructorWork, 49);
  assert.equal(financial.kernel.collectionCapacity, 128);
  assert.equal(financial.kernel.headFields, 13);
  assert.equal(financial.kernel.quantitiesAreLovelace, false);
  assert.equal(financial.race.bothSignedSubmitsBeforeAnyPoll, true);
  assert.equal(financial.genesisMagicBound, true);
  assert.equal(financial.networkMagic, 764);
  assert.equal(financial.nativeBudgetMeasured, false);
  assert.equal(financial.twelveInputFrameSatisfied, false);
  const created = report.stages.ownedProducer.wouldSpawn.find((step) => step.id === 'create-node');
  assert.equal(created.argv[0].endsWith('yaci-cli'), true);
  assert.equal(created.argv.includes('create-node'), true);
  assert.equal(created.argv[created.argv.indexOf('--start') + 1], 'false');
  const fee = report.stages.ownedProducer.wouldSpawn.find((step) => step.id === 'calculate-min-fee');
  assert.equal(fee.witnessCountProvenance, 'counted-from-signing-key-paths');
  assert.equal(fee.bind, true);
  assert.equal(JSON.stringify(fee.argv || []).includes('"1"'), false);
  const parity = compareClockParity();
  assert.equal(parity.code, 'POSIX_PARITY_NOT_RUN');
  assert.equal(parity.parityExecuted, false);
  rmSync(root, { recursive: true, force: true });
});

test('dispatch binds fixture role utxos, omits Dijkstra, and holds the cost rebuild', () => {
  const root = mkdtempSync(join(CHECKS, 'dispatch-'));
  const reviews = nativeReviews(join(root, 'reviews'));
  const home = join(OWN, 'b');
  mkdirSync(home, { recursive: true });
  const bodyFile = join(home, 'unit-body.tx');
  writeFileSync(bodyFile, 'unit-body-not-a-ledger-tx\n');
  const ready = produceOwnedRuntime({ ...ownedRequest(home, 't1-104-dispatch', reviews), txFile: bodyFile });
  assert.equal(ready.code, 'REAL_NODE_NOT_ARMED');
  assert.equal(ready.inputsReady, true);
  const calls = [];
  const dispatched = dispatchOwnedProducer(ready.schema71, {
    unitOnly: true,
    spawn: (argv, step) => {
      calls.push(step.id);
      const paths = ready.schema71.paths;
      const writeKey = (path, body) => {
        mkdirSync(dirname(path), { recursive: true });
        writeFileSync(path, JSON.stringify({ type: 'PaymentVerificationKeyShelley_ed25519', cborHex: `5820${body}` }));
      };
      if (step.id.startsWith('role-keygen')) {
        const mark = step.id.endsWith('issuer') ? 'a1' : step.id.endsWith('lender') ? 'b2' : 'c4';
        writeKey(argv[argv.indexOf('--verification-key-file') + 1], mark.repeat(32));
        writeKey(argv[argv.indexOf('--signing-key-file') + 1], mark.repeat(32));
      }
      if (step.id === 'legacy-genesis') {
        const bodies = ['0123456789abcdef'.repeat(4), '00112233445566778899aabbccddeeff'.repeat(2), 'fedcba98765432100123456789abcdef'.repeat(2)];
        paths.utxoVkeys.forEach((path, index) => writeKey(path, bodies[index]));
        paths.utxoSkeys.forEach((path, index) => writeKey(path, bodies[index]));
        writeFileSync(paths.shelleyWork, JSON.stringify({
          slotLength: 1,
          epochLength: 3600,
          networkMagic: 764,
          systemStart: '2026-01-01T00:00:00Z',
          minFeeA: 44,
          minFeeB: 155381,
          maxLovelaceSupply: 45000000000000000,
          genDelegs: { ['11'.repeat(28)]: { delegate: '22'.repeat(28), vrf: '33'.repeat(64) } },
        }));
        writeFileSync(paths.byronWork, JSON.stringify({ startTime: 0 }));
        writeFileSync(paths.alonzoWork, JSON.stringify({ seed: 1 }));
        writeFileSync(paths.conwayWork, JSON.stringify({ seed: 1 }));
      }
      if (step.id.startsWith('initial-addr') || step.id.startsWith('role-address')) {
        const out = argv[argv.indexOf('--out-file') + 1];
        mkdirSync(dirname(out), { recursive: true });
        writeFileSync(out, `addr_test1fixture${step.id}\n`);
      }
      if (step.id === 'start-node') bindUnixSocket(paths.socketPath);
      if (step.id === 'query-tip') return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ce'.repeat(32), block: 3 }) };
      if (step.id === 'query-era-history') return { status: 0, stdout: JSON.stringify({ type: 'EraHistory', description: 'EraHistory', cborHex: '9fff' }) };
      if (step.id === 'query-funder-utxo' || step.id.startsWith('funding-confirm')) {
        return { status: 0, stdout: `${'0123456789abcdef'.repeat(4)} 0 9000000\n` };
      }
      if (step.id === 'calculate-min-fee') {
        assert.equal(argv[argv.indexOf('--witness-count') + 1], '1');
        assert.equal(argv.includes('--witness-count'), true);
        return { status: 0, stdout: '{"fee":123456}\n' };
      }
      if (step.id === 'calculate-plutus-script-cost') {
        return { status: 0, stdout: JSON.stringify([{ scriptHash: '11'.repeat(28), executionUnits: { memory: 500, steps: 64100 }, lovelaceCost: 1 }]) };
      }
      return { status: 0, stdout: '' };
    },
  });
  assert.equal(dispatched.code, 'PRODUCER_DISPATCHED', JSON.stringify(dispatched.results?.map((item) => [item.id, item.code])));
  assert.equal(dispatched.nativeBudgetMeasured, false);
  assert.equal(dispatched.ledgerFunding, false);
  assert.equal(dispatched.financial.roleUtxosDisjoint, false);
  assert.equal(dispatched.financial.ok, true);
  assert.equal(dispatched.financial.cashQuantity, '100');
  assert.equal(dispatched.financial.headQuantity, '1');
  const witness = dispatched.results.find((item) => item.id === 'bind-signed-witnesses');
  assert.equal(witness.code, 'WITNESS_COUNT_BOUND_UNIT_ONLY');
  assert.equal(witness.witnessCountVerified, false);
  const fee = dispatched.results.find((item) => item.id === 'calculate-min-fee');
  assert.equal(fee.witnessCountProvenance.witnessCountProvenanceClass, 'counted-from-signing-key-paths');
  assert.equal(fee.witnessCountProvenance.witnessCountVerified, false);
  assert.equal(fee.nativeBudgetMeasured, false);
  const cost = dispatched.results.find((item) => item.id === 'calculate-plutus-script-cost');
  assert.equal(cost.code, 'PLUTUS_COST_PARSED');
  assert.equal(cost.cost.memory, 500);
  assert.equal(cost.rebuild.code, 'PLUTUS_COST_REBUILD_HELD');
  assert.equal(cost.rebuild.applied, false);
  assert.equal(cost.rebuild.scriptDataHash, null);
  assert.equal(cost.nativeBudgetMeasured, false);
  const rendered = JSON.parse(readFileSync(ready.schema71.paths.configuration, 'utf8'));
  assert.equal(Object.hasOwn(rendered, 'DijkstraGenesisFile'), false);
  assert.equal(rendered.ShelleyGenesisFile, './genesis/shelley-genesis.json');
  assert.equal(calls.includes('funding-submit'), true);
  assert.equal(calls.indexOf('bind-signed-witnesses'), -1);
  rmSync(home, { recursive: true, force: true });
  rmSync(root, { recursive: true, force: true });
});

test('fee token, socket type, role chain, clock, and fixture keys stay honest', async () => {
  const root = mkdtempSync(join(CHECKS, 'controls-'));
  const reviews = nativeReviews(join(root, 'reviews'));
  const fake = dispatchOwnedProducer({
    inputsReady: true,
    sequence: [{
      id: 'calculate-min-fee',
      run: true,
      argv: ['cardano-cli', 'conway', 'transaction', 'calculate-min-fee'],
      witnessCountProvenance: 'measured-not-assumed',
    }],
  }, { unitOnly: true, spawn: () => ({ status: 0, stdout: '{"fee":219798.0}\n' }) });
  assert.equal(fake.code, 'FAKE_FEE');
  assert.equal(fake.nativeBudgetMeasured, false);
  const canonical = dispatchOwnedProducer({
    inputsReady: true,
    sequence: [{
      id: 'calculate-min-fee',
      run: true,
      argv: ['cardano-cli', 'conway', 'transaction', 'calculate-min-fee'],
      witnessCountProvenance: 'ASSUMED-unsigned-body-no-key-analysis',
    }],
  }, { unitOnly: true, spawn: () => ({ status: 0, stdout: '{"fee":123456}\n' }) });
  assert.equal(canonical.code, 'PRODUCER_DISPATCHED');
  const feeRow = canonical.results.find((item) => item.id === 'calculate-min-fee');
  assert.equal(feeRow.fee, 123456);
  assert.equal(feeRow.witnessCountProvenance.witnessCountProvenanceClass, 'assumption-not-verified');
  assert.equal(feeRow.witnessCountProvenance.witnessCountVerified, false);
  const counted = dispatchOwnedProducer({
    inputsReady: true,
    sequence: [{
      id: 'calculate-min-fee',
      run: true,
      argv: ['cardano-cli', 'conway', 'transaction', 'calculate-min-fee'],
      witnessCountProvenance: 'counted-from-signing-key-paths',
    }],
  }, { unitOnly: true, spawn: () => ({ status: 0, stdout: '{"fee":123456}\n' }) });
  assert.equal(counted.results[0].witnessCountProvenance.witnessCountProvenanceClass, 'counted-from-signing-key-paths');
  const sock = join(root, 'node.sock');
  bindUnixSocket(sock);
  const stat = spawnSync('python3', ['-c', 'import os,sys; print(os.stat(sys.argv[1]).st_size)', sock], { encoding: 'utf8' });
  assert.equal(stat.stdout.trim(), '0');
  const readySock = dispatchOwnedProducer({
    inputsReady: true,
    paths: { socketPath: sock },
    sequence: [{ id: 'await-socket', local: 'await-socket', run: true }],
  }, { unitOnly: true });
  assert.equal(readySock.code, 'PRODUCER_DISPATCHED');
  const regular = join(root, 'file-node.sock');
  writeFileSync(regular, 'x');
  const fileSock = dispatchOwnedProducer({
    inputsReady: true,
    paths: { socketPath: regular },
    sequence: [{ id: 'await-socket', local: 'await-socket', run: true }],
  }, { unitOnly: true });
  assert.equal(fileSock.code, 'SOCKET_NOT_READY');
  const big = dispatchOwnedProducer({
    inputsReady: true,
    paths: { socketPath: sock },
    sequence: [
      { id: 'await-socket', local: 'await-socket', run: true },
      { id: 'query-funder-utxo', run: true, argv: ['cardano-cli', 'query', 'utxo', '--output-text'] },
      { id: 'funding-confirm-issuer', run: true, argv: ['cardano-cli', 'query', 'utxo'] },
      { id: 'funding-confirm-lender', run: true, argv: ['cardano-cli', 'query', 'utxo'] },
      { id: 'funding-confirm-borrower', run: true, argv: ['cardano-cli', 'query', 'utxo'] },
    ],
  }, {
    unitOnly: true,
    spawn: (argv, step) => {
      const mark = step.id.endsWith('issuer') ? '11' : step.id.endsWith('lender') ? '22' : step.id.endsWith('borrower') ? '33' : '44';
      const amount = step.id === 'query-funder-utxo' ? '45000000000000000' : '1000000';
      return { status: 0, stdout: `${mark.repeat(32)} 0 ${amount}\n` };
    },
  });
  assert.equal(big.code, 'PRODUCER_DISPATCHED', JSON.stringify(big.results?.map((item) => item.code)));
  const funder = big.results.find((item) => item.id === 'query-funder-utxo');
  assert.equal(funder.lovelace, null);
  assert.equal(funder.lovelaceText, '45000000000000000');
  assert.equal(funder.jsSafeInteger, false);
  assert.equal(funder.unitOnly, true);
  assert.equal(big.financial.roleUtxosDisjoint, true);
  assert.equal(big.financial.chain.cashMint.spends.endsWith('#0'), true);
  assert.equal(big.financial.chain.headOpen.spends.startsWith('22'), true);
  assert.equal(big.financial.quantitiesAreLovelace, false);
  assert.equal(countVkeyWitnesses('8240a10081824040'), 1);
  const unitWitness = dispatchOwnedProducer({
    inputsReady: true,
    paths: {},
    sequence: [{ id: 'bind-signed-witnesses', local: 'bind-signed-witnesses', run: true }],
  }, { unitOnly: true });
  assert.equal(unitWitness.results[0].code, 'WITNESS_COUNT_BOUND_UNIT_ONLY');
  assert.equal(unitWitness.results[0].witnessCountVerified, false);
  const mismatch = bindSignedWitnessCount({ unitOnly: false, signedCborHex: '8240a10081824040', signingKeyCount: 2 });
  assert.equal(mismatch.code, 'WITNESS_SET_NOT_COUNTED');
  assert.equal(mismatch.witnessCountVerified, false);
  const lender = `5820${'12'.repeat(32)}`;
  const encoding = selectOwnedFunder({
    roles: {
      lender: { vkeyHex: ENCODING_VKEY_HEX },
      borrower: { vkeyHex: `5820${'34'.repeat(32)}` },
      issuer: { vkeyHex: `5820${'56'.repeat(32)}` },
    },
  });
  assert.equal(encoding.code, 'ENCODING_FIXTURE_NOT_A_ROLE');
  const shaped = selectOwnedFunder({
    unitOnly: true,
    roles: {
      lender: { vkeyHex: lender },
      borrower: { vkeyHex: `5820${'34'.repeat(32)}` },
      issuer: { vkeyHex: `5820${'56'.repeat(32)}` },
    },
    utxo: {
      lovelaceText: '45000000000000000',
      ownerHash: paymentKeyHashOfVkeyCbor(lender),
      address: 'addr_test1fixture',
      txId: '0123456789abcdef'.repeat(4),
      index: 0,
    },
  });
  assert.equal(shaped.code, 'FUNDER_SHAPE_READY');
  assert.equal(shaped.fundedInNodeInitialFunds, false);
  assert.equal(shaped.bootstrapComplete, false);
  const refusedMagic = encodeUnsignedBody({
    networkMagic: 42,
    outputLovelace: '3000000',
    feeLovelace: '200000',
    paymentVkeyCbor: lender,
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '22'.repeat(32), index: 0 },
    collateral: { txId: '33'.repeat(32), index: 0 },
  });
  assert.equal(refusedMagic.code, 'MAGIC_SOURCE_REFUSED');
  assert.equal(refusedMagic.spawned, false);
  const missingFee = encodeUnsignedBody({
    networkMagic: 764,
    paymentVkeyCbor: lender,
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '22'.repeat(32), index: 0 },
    collateral: { txId: '33'.repeat(32), index: 0 },
  });
  assert.equal(missingFee.code, 'FEE_UNMEASURED');
  const fixtureSigner = encodeUnsignedBody({
    networkMagic: 764,
    outputLovelace: '3000000',
    feeLovelace: '200000',
    paymentVkeyCbor: ENCODING_VKEY_HEX,
    head: { txId: '11'.repeat(32), index: 0 },
    fee: { txId: '22'.repeat(32), index: 0 },
    collateral: { txId: '33'.repeat(32), index: 0 },
  });
  assert.equal(fixtureSigner.code, 'ENCODING_FIXTURE_NOT_A_ROLE');
  const formula = refuseCallerMinAdaFormula(44, 200);
  assert.equal(formula.ok, false);
  assert.equal(formula.code, 'MIN_ADA_FORMULA_IS_NOT_THE_CLI_COMMAND');
  const funderRef = `${'ab'.repeat(32)}#4`;
  const parsedBig = parseQueryUtxoText(`{"${funderRef}":{"address":"addr_test1fixture","value":{"lovelace":45000000000000000}}}`);
  assert.equal(parsedBig.code, 'FUNDER_UTXO_JSON_PARSED');
  assert.equal(parsedBig.lovelace, null);
  assert.equal(parsedBig.lovelaceText, '45000000000000000');
  assert.equal(parsedBig.jsSafeInteger, false);
  assert.equal(parsedBig.index, 4);
  assert.equal(parsedBig.ledgerFunding, false);
  const fraction = parseQueryUtxoText(`{"${funderRef}":{"value":{"lovelace":219798.0}}}`);
  assert.equal(fraction.code, 'UTXO_JSON_NON_INTEGER');
  const exponent = parseQueryUtxoText(`{"${funderRef}":{"value":{"lovelace":1e2}}}`);
  assert.equal(exponent.code, 'UTXO_JSON_NON_INTEGER');
  const emptyMap = parseQueryUtxoText('{}\n');
  assert.equal(emptyMap.code, 'FUNDER_UTXO_ABSENT');
  assert.equal(emptyMap.ledgerFunding, false);
  const arrayBody = parseQueryUtxoText('[]');
  assert.equal(arrayBody.code, 'UTXO_JSON_NOT_AN_OBJECT');
  const magicCli = join(root, 'magic42.sh');
  const magicMark = join(root, 'magic42-spawned');
  writeFileSync(magicCli, `#!/bin/sh\ntouch ${magicMark}\nexit 0\n`);
  spawnSync('chmod', ['+x', magicCli]);
  const refusedQuery = queryOwnedUtxos({ cliPath: magicCli, socket: sock, runDir: root, execute: true, magic: 42 });
  assert.equal(refusedQuery.code, 'MAGIC_SOURCE_REFUSED');
  assert.equal(refusedQuery.spawned, false);
  assert.equal(existsSync(magicMark), false);
  const jsonCli = join(root, 'json-utxo.sh');
  const jsonBody = `{"${'cd'.repeat(32)}#7":{"value":{"lovelace":45000000000000000}}}`;
  writeFileSync(jsonCli, `#!/bin/sh\nout=""\nwhile [ $# -gt 0 ]; do\n  if [ "$1" = "--out-file" ]; then out="$2"; shift 2; else shift; fi\ndone\nprintf '%s\\n' '${jsonBody}' > "$out"\n`);
  spawnSync('chmod', ['+x', jsonCli]);
  const jsonQuery = queryOwnedUtxos({ cliPath: jsonCli, socket: sock, runDir: root, execute: true, magic: 764 });
  assert.equal(jsonQuery.code, 'UTXO_QUERIED');
  assert.equal(jsonQuery.unitOnly, true);
  assert.equal(jsonQuery.ledgerExecution, false);
  assert.equal(jsonQuery.selector, '--whole-utxo');
  assert.equal(jsonQuery.utxos[0].index, 7);
  assert.equal(jsonQuery.utxos[0].value.lovelace, '45000000000000000');
  assert.equal(readFileSync(jsonQuery.transcriptPath, 'utf8').includes('runId='), false);
  assert.match(readFileSync(`${jsonQuery.transcriptPath}.provenance`, 'utf8'), /^runId=/);
  const home = join(root, 'clock');
  const plan = produceOwnedRuntime(ownedRequest(home, 't1-104-clock', reviews)).schema71;
  mkdirSync(dirname(plan.paths.shelleyWork), { recursive: true });
  mkdirSync(dirname(plan.paths.utxoVkeys[0]), { recursive: true });
  const bodies = ['0123456789abcdef'.repeat(4), '00112233445566778899aabbccddeeff'.repeat(2), 'fedcba98765432100123456789abcdef'.repeat(2)];
  plan.paths.utxoVkeys.forEach((path, index) => writeFileSync(path, JSON.stringify({ cborHex: `5820${bodies[index]}` })));
  plan.paths.addrOuts.forEach((path, index) => writeFileSync(path, `addr_test1clock${index}\n`));
  writeFileSync(plan.paths.byronWork, JSON.stringify({ startTime: 0 }));
  writeFileSync(plan.paths.shelleyWork, JSON.stringify({
    slotLength: 1,
    epochLength: 3600,
    networkMagic: 764,
    systemStart: '2017-09-23T21:44:51Z',
    minFeeA: 44,
    minFeeB: 155381,
    maxLovelaceSupply: 45000000000000000,
    genDelegs: { ['11'.repeat(28)]: { delegate: '22'.repeat(28), vrf: '33'.repeat(64) } },
  }));
  const stale = dispatchOwnedProducer({
    inputsReady: true,
    magic: 764,
    supplyLovelace: 9000000,
    paths: plan.paths,
    sequence: [{ id: 'bind-initial-funds', local: 'bind-initial-funds', run: true }],
  }, { unitOnly: true });
  assert.equal(stale.code, 'STALE_TEMPLATE_CLOCK');
  assert.equal(stale.parityExecuted, false);
  const datum = await runNativeCaller({
    authority: reviews,
    outputFee: {
      kind: 'output-minimum',
      cliPath: CLI,
      protocolPath: PROTOCOL,
      protocolProvenance: createHash('sha256').update(readFileSync(PROTOCOL)).digest('hex'),
      workDir: join(root, 'datum'),
      output: {
        address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
        lovelace: 2000000,
        assets: [],
        datumData: { constructor: 0, fields: [{ int: '100' }] },
      },
    },
  });
  assert.equal(datum.code, 'MALFORMED_DATUM');
  assert.equal(datum.stages.outputFee.nativeBudgetMeasured, false);
  rmSync(root, { recursive: true, force: true });
});
