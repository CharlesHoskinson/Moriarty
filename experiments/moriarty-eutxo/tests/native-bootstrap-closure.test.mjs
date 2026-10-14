import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { runNativeCaller } from '../src/native-caller.mjs';
import { ENCODING_VKEY_HEX, paymentKeyHashOfVkeyCbor } from '../src/native-body.mjs';
import { dispatchOwnedProducer, selectOwnedFunder } from '../src/native-producer.mjs';
import { produceOwnedRuntime } from '../src/native-runtime.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const PROTOCOL = join(TASK, 'tooling/runtime/actual-protocol-parameters.json');
const BODY = join(TASK, 't1-native-consumer-repair-70/artifacts/cash-mint.body.json');
const CLI = join(TASK, 'tooling/runtime/devnet/cardano-node/bin/cardano-cli');
const ZERO_FEE = join(TASK, 't1-runtime-schemas-68/sources/cardano-cli/cardano-cli/test/cardano-cli-test/files/input/conway/create-cardano/genesis.shelley.spec.json');

function bindUnixSocket(path) {
  mkdirSync(dirname(path), { recursive: true });
  if (existsSync(path)) rmSync(path);
  const run = spawnSync('python3', ['-c', 'import socket,sys; p=sys.argv[1]; s=socket.socket(socket.AF_UNIX); s.bind(p); s.listen(1)', path], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(run.stderr || 'unix-socket');
}

function nativeReviews(dir) {
  mkdirSync(dir, { recursive: true });
  const sourcePath = join(dir, 'source.txt');
  const sourceText = 'bootstrap-gate-probe\n';
  writeFileSync(sourcePath, sourceText);
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const manifest = {
    sourceHash,
    files: [{ path: sourcePath, sha256: sourceHash, bytes: Buffer.byteLength(sourceText) }],
  };
  const body = JSON.stringify(manifest);
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

function ownedRequest(home, runId, reviews, extra = {}) {
  return {
    executeNow: true,
    arm: true,
    authority: reviews,
    ownedHome: home,
    runId,
    nodePort: 14031,
    submitApiPort: 14032,
    magic: 764,
    magicSource: 'caller-chosen-for-genesis',
    supplyLovelace: 9000000,
    ...extra,
  };
}

test('runNativeCaller exposes the owned bootstrap and keeps the latch closed', async () => {
  const root = mkdtempSync(join(tmpdir(), 'moriarty-bootstrap-83-'));
  try {
    const reviews = nativeReviews(join(root, 'reviews'));
    const home = join(root, 'owned');
    const runId = 't1-83-probe';
    const report = await runNativeCaller({
      authority: reviews,
      ownedProducer: ownedRequest(home, runId, reviews),
    });
    assert.equal(report.ledgerExecution, false);
    assert.equal(report.nativePins, false);
    assert.equal(report.ledgerSubmitted, false);
    assert.equal(report.code, 'REAL_NODE_NOT_ARMED');
    const owned = report.stages.ownedProducer;
    assert.equal(owned.spawned, false);
    assert.equal(owned.inputsReady, true);
    assert.equal(owned.authorityAccepted, true);
    assert.equal(owned.schema71.fundedInNodeInitialFunds, false);
    assert.equal(owned.schema71.bootstrapComplete, false);
    assert.equal(owned.schema71.nativeBudgetMeasured, false);
    const ids = owned.wouldSpawn.map((step) => step.id);
    assert.equal(ids.indexOf('legacy-genesis') < ids.indexOf('bind-initial-funds'), true);
    assert.equal(ids.indexOf('bind-initial-funds') < ids.indexOf('install-owned-genesis'), true);
    assert.equal(ids.indexOf('create-node') < ids.indexOf('start-node'), true);
    assert.equal(ids.indexOf('start-node') < ids.indexOf('await-socket'), true);
    assert.equal(ids.indexOf('await-socket') < ids.indexOf('query-tip'), true);
    assert.equal(ids.indexOf('query-tip') < ids.indexOf('query-funder-utxo'), true);
    assert.equal(ids.indexOf('query-funder-utxo') < ids.indexOf('funding-build'), true);
    assert.equal(ids.indexOf('funding-build') < ids.indexOf('calculate-min-fee'), true);
    assert.equal(ids.indexOf('calculate-min-fee') < ids.indexOf('funding-conservation'), true);
    assert.equal(ids.indexOf('funding-sign') < ids.indexOf('funding-submit'), true);
    assert.equal(ids.includes('evaluate'), false);
    assert.equal(owned.wouldSpawn.find((step) => step.id === 'create-node').start, false);
    assert.equal(owned.wouldSpawn.find((step) => step.id === 'legacy-genesis').argv.includes('--conway-era'), true);
    assert.equal(owned.wouldSpawn.find((step) => step.id === 'legacy-genesis').argv.includes('1000'), true);
    const lender = `5820${'12'.repeat(32)}`;
    const mismatch = await runNativeCaller({
      authority: reviews,
      funder: {
        fundedInNodeInitialFunds: true,
        unitOnly: true,
        roles: {
          lender: { vkeyHex: lender },
          borrower: { vkeyHex: `5820${'34'.repeat(32)}` },
          issuer: { vkeyHex: `5820${'56'.repeat(32)}` },
        },
        utxo: {
          lovelace: 9000000,
          ownerHash: '00'.repeat(28),
          address: 'addr_test1fixture',
          txId: '0123456789abcdef'.repeat(4),
          index: 0,
        },
      },
    });
    assert.equal(mismatch.code, 'FUNDER_OWNER_MISMATCH');
    assert.equal(mismatch.stages.funder.ledgerFunding, false);
    assert.equal(mismatch.ledgerExecution, false);
    const shaped = selectOwnedFunder({
      fundedInNodeInitialFunds: true,
      unitOnly: true,
      roles: {
        lender: { vkeyHex: lender },
        borrower: { vkeyHex: `5820${'34'.repeat(32)}` },
        issuer: { vkeyHex: `5820${'56'.repeat(32)}` },
      },
      utxo: {
        lovelace: 9000000,
        ownerHash: paymentKeyHashOfVkeyCbor(lender),
        address: 'addr_test1fixture',
        txId: '0123456789abcdef'.repeat(4),
        index: 0,
      },
    });
    assert.equal(shaped.code, 'FUNDER_SHAPE_READY');
    assert.equal(shaped.bootstrapComplete, false);
    assert.equal(shaped.ledgerFunding, false);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('bootstrap dispatch fails closed on socket, clock, key, fee, and argv faults', () => {
  const root = mkdtempSync(join(tmpdir(), 'moriarty-bootstrap-fault-'));
  try {
    const reviews = nativeReviews(join(root, 'reviews'));
    const home = join(root, 'owned');
    const plan = produceOwnedRuntime(ownedRequest(home, 't1-83-fault', reviews)).schema71;
    assert.equal(plan.inputsReady, true);
    const earlyTip = dispatchOwnedProducer({
      inputsReady: true,
      paths: plan.paths,
      sequence: [{ id: 'query-tip', run: true, argv: ['cardano-cli', 'query', 'tip'] }],
    }, { unitOnly: true, spawn: () => { throw new Error('must-not-spawn'); } });
    assert.equal(earlyTip.code, 'NODE_NOT_STARTED');
    assert.equal(earlyTip.spawned, false);
    const missingSock = dispatchOwnedProducer({
      inputsReady: true,
      paths: { ...plan.paths, socketPath: join(root, 'missing', 'node.sock') },
      sequence: [{ id: 'await-socket', local: 'await-socket', run: true }],
    }, { unitOnly: true });
    assert.equal(missingSock.code, 'READINESS_TIMEOUT');
    const emptyPath = join(root, 'empty', 'node.sock');
    mkdirSync(join(root, 'empty'), { recursive: true });
    writeFileSync(emptyPath, '');
    const emptySock = dispatchOwnedProducer({
      inputsReady: true,
      paths: { ...plan.paths, socketPath: emptyPath },
      sequence: [{ id: 'await-socket', local: 'await-socket', run: true }],
    }, { unitOnly: true });
    assert.equal(emptySock.code, 'SOCKET_NOT_READY');
    const evaluated = dispatchOwnedProducer({
      inputsReady: true,
      sequence: [{ id: 'legacy-genesis', run: true, argv: ['cardano-cli', 'evaluate'] }],
    }, { unitOnly: true, spawn: () => { throw new Error('must-not-spawn'); } });
    assert.equal(evaluated.code, 'NO_CLI_EVALUATE_COMMAND');
    const zeroFee = produceOwnedRuntime(ownedRequest(home, 't1-83-fault', reviews, {
      templates: { shelleyTemplate: ZERO_FEE },
    }));
    assert.equal(zeroFee.code, 'GENESIS_PROFILE_REFUSED');
    const paths = plan.paths;
    mkdirSync(join(paths.genesisDir, 'utxo-keys'), { recursive: true });
    const bodies = ['0123456789abcdef'.repeat(4), '00112233445566778899aabbccddeeff'.repeat(2), 'fedcba98765432100123456789abcdef'.repeat(2)];
    const writeKey = (path, body) => writeFileSync(path, JSON.stringify({ cborHex: `5820${body}` }));
    paths.utxoVkeys.forEach((path, index) => writeKey(path, bodies[index]));
    const baseShelley = {
      slotLength: 1,
      epochLength: 3600,
      networkMagic: 764,
      systemStart: '2026-01-01T00:00:00Z',
      minFeeA: 44,
      minFeeB: 155381,
      maxLovelaceSupply: 45000000000000000,
      genDelegs: { ['11'.repeat(28)]: { delegate: '22'.repeat(28), vrf: '33'.repeat(64) } },
      initialFunds: {},
    };
    writeFileSync(paths.byronWork, JSON.stringify({ startTime: 0 }));
    paths.addrOuts.forEach((path, index) => writeFileSync(path, `addr_test1fixture${index}\n`));
    writeFileSync(paths.shelleyWork, JSON.stringify({ ...baseShelley, systemStart: '2022-09-15T04:09:11.577484Z' }));
    const staleClock = dispatchOwnedProducer({
      inputsReady: true,
      magic: 764,
      supplyLovelace: 9000000,
      paths,
      sequence: [{ id: 'bind-initial-funds', local: 'bind-initial-funds', run: true }],
    }, { unitOnly: true });
    assert.equal(staleClock.code, 'CREATE_TIME_CLOCK_NOT_REWRITTEN');
    const template = JSON.parse(readFileSync(paths.templateShelley, 'utf8'));
    writeFileSync(paths.shelleyWork, JSON.stringify({ ...baseShelley, genDelegs: template.genDelegs }));
    const copiedDeleg = dispatchOwnedProducer({
      inputsReady: true,
      magic: 764,
      supplyLovelace: 9000000,
      paths,
      sequence: [{ id: 'bind-initial-funds', local: 'bind-initial-funds', run: true }],
    }, { unitOnly: true });
    assert.equal(copiedDeleg.code, 'GENESIS_DELEGATION_NOT_FRESH');
    writeKey(paths.utxoVkeys[0], 'ab'.repeat(32));
    writeFileSync(paths.shelleyWork, JSON.stringify(baseShelley));
    const encoding = dispatchOwnedProducer({
      inputsReady: true,
      magic: 764,
      supplyLovelace: 9000000,
      paths,
      sequence: [{ id: 'bind-initial-funds', local: 'bind-initial-funds', run: true }],
    }, { unitOnly: true });
    assert.equal(encoding.code, 'ENCODING_FIXTURE_NOT_A_ROLE');
    assert.equal(encoding.code === ENCODING_VKEY_HEX, false);
    writeKey(paths.utxoVkeys[0], bodies[0]);
    writeFileSync(paths.addrOuts[0], 'not-an-address\n');
    const badAddr = dispatchOwnedProducer({
      inputsReady: true,
      magic: 764,
      supplyLovelace: 9000000,
      paths,
      sequence: [{ id: 'bind-initial-funds', local: 'bind-initial-funds', run: true }],
    }, { unitOnly: true });
    assert.equal(badAddr.code, 'ADDRESS_NOT_DERIVED');
    const hungry = dispatchOwnedProducer({
      inputsReady: true,
      _utxoLovelace: 9000000,
      _feeLovelace: 8000000,
      paths,
      sequence: [{ id: 'funding-conservation', local: 'funding-conservation', run: true }],
    }, { unitOnly: true });
    assert.equal(hungry.code, 'CONSERVATION_FAILED');
    assert.equal(hungry.ledgerFunding, false);
    assert.equal(hungry.nativeBudgetMeasured, false);
    const socket = join(root, 'node', 'node.sock');
    mkdirSync(join(root, 'node'), { recursive: true });
    bindUnixSocket(socket);
    const jsonUtxo = dispatchOwnedProducer({
      inputsReady: true,
      paths: { ...paths, socketPath: socket },
      sequence: [
        { id: 'await-socket', local: 'await-socket', run: true },
        { id: 'query-funder-utxo', run: true, argv: ['cardano-cli', 'query', 'utxo', '--output-text'] },
      ],
    }, { unitOnly: true, spawn: () => ({ status: 0, stdout: '{}\n' }) });
    assert.equal(jsonUtxo.code, 'FUNDER_UTXO_ABSENT');
    assert.equal(jsonUtxo.nodeUtxoQueried, false);
    assert.equal(jsonUtxo.ledgerFunding, false);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('caller adopts helper77 offline coin and fee without a budget pin', async () => {
  const root = mkdtempSync(join(tmpdir(), 'moriarty-helper77-'));
  try {
    const reviews = nativeReviews(join(root, 'reviews'));
    const protocolSha = createHash('sha256').update(readFileSync(PROTOCOL)).digest('hex');
    const datum = {
      constructor: 0,
      fields: Array.from({ length: 13 }, (_, index) => ({ int: index })),
    };
    const coin = await runNativeCaller({
      authority: reviews,
      outputFee: {
        kind: 'output-minimum',
        cliPath: CLI,
        protocolPath: PROTOCOL,
        protocolProvenance: protocolSha,
        workDir: join(root, 'coin'),
        output: {
          address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
          lovelace: 2000000,
          assets: [{
            policyId: 'c760611894de8a0b57c3464ec40c49dce50440d9f238abb9d13d4555',
            assetNameHex: '43617368',
            quantity: 100,
          }],
          datumData: datum,
        },
      },
    });
    assert.equal(coin.stages.outputFee.ok, true, coin.stages.outputFee.detail || coin.stages.outputFee.code);
    assert.equal(coin.stages.outputFee.nativeBudgetMeasured, false);
    assert.equal(coin.stages.outputFee.ledgerExecuted, false);
    assert.equal(coin.stages.outputFee.ledgerExecution, false);
    assert.equal(coin.stages.outputFee.liveProtocolQueried, false);
    assert.equal(coin.stages.outputFee.budgetPin, false);
    assert.equal(coin.stages.outputFee.valuesAreBudgetPins, false);
    assert.equal(coin.stages.outputFee.evidenceClass, 'offline-pinned-protocol-file');
    assert.equal(coin.stages.outputFee.datum.fieldCount, 13);
    assert.equal(coin.stages.outputFee.datum.passedAsFieldCount, false);
    assert.match(coin.stages.outputFee.parsed.lovelace, /^(0|[1-9][0-9]*)$/);
    assert.equal(coin.ledgerExecution, false);
    const fee = await runNativeCaller({
      authority: reviews,
      outputFee: {
        kind: 'body-fee',
        cliPath: CLI,
        protocolPath: PROTOCOL,
        protocolProvenance: { expectedSha256: protocolSha },
        workDir: join(root, 'fee'),
        bodyPath: BODY,
        witnessCount: 1,
        witnessCountProvenance: 'ASSUMED-unsigned-body-no-key-analysis',
      },
    });
    assert.equal(fee.stages.outputFee.ok, true, fee.stages.outputFee.detail || fee.stages.outputFee.code);
    assert.equal(fee.stages.outputFee.nativeBudgetMeasured, false);
    assert.equal(fee.stages.outputFee.feeAppliesToSignedTx, false);
    assert.equal(fee.stages.outputFee.witnessCountProvenance.witnessCountVerified, false);
    assert.equal(fee.stages.outputFee.witnessCountProvenance.witnessCountProvenanceClass, 'assumption-not-verified');
    assert.match(String(fee.stages.outputFee.parsed.fee), /^(0|[1-9][0-9]*)$/);
    assert.equal(fee.ledgerExecution, false);
    const witnessScript = await runNativeCaller({
      authority: reviews,
      outputFee: {
        kind: 'output-minimum',
        cliPath: CLI,
        protocolPath: PROTOCOL,
        protocolProvenance: protocolSha,
        workDir: join(root, 'reject'),
        output: {
          address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
          lovelace: 2000000,
          witnessScript: 'not-a-reference-script',
        },
      },
    });
    assert.equal(witnessScript.stages.outputFee.ok, false);
    assert.equal(witnessScript.stages.outputFee.code, 'UNSUPPORTED_OUTPUT');
    assert.equal(witnessScript.stages.outputFee.nativeBudgetMeasured, false);
    const missingScript = await runNativeCaller({
      authority: reviews,
      outputFee: {
        kind: 'output-minimum',
        cliPath: CLI,
        protocolPath: PROTOCOL,
        protocolProvenance: protocolSha,
        workDir: join(root, 'missing-script'),
        output: {
          address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
          lovelace: 2000000,
          assets: [],
          referenceScriptPath: join(root, 'absent.plutus'),
        },
      },
    });
    assert.equal(missingScript.code, 'REFERENCE_SCRIPT_MISSING');
    assert.equal(missingScript.stages.outputFee.ledgerExecution, false);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
