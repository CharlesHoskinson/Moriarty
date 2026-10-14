/**
 * Unexecuted static check for packet 185.
 * Imports the real modules. The spawn argument records argv and returns.
 * It does not exec cardano-cli, cardano-node, keygen, genesis, fund, mint, sign, or submit.
 */
import { createHash } from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, rmSync, statSync, symlinkSync, writeFileSync } from 'node:fs';
import { homedir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { test } from 'node:test';
import assert from 'node:assert/strict';

const HERE = dirname(fileURLToPath(import.meta.url));
const OWN_WT_SRC = join(HERE, '../src');
const ownModule = (name) => import(pathToFileURL(join(OWN_WT_SRC, name)).href);
const bootstrap = await ownModule('native-bootstrap.mjs');
const producer = await ownModule('native-producer.mjs');
const caller = await ownModule('native-caller.mjs');
const {
  awaitOwnedSocket,
  bindFreshInitialFunds,
  buildBootstrapPlan,
  renderPinnedNodeConfig,
  resolveBootstrapArgv,
  runLocalBootstrap,
} = bootstrap;
const { dispatchOwnedProducer, ownedGenesisArgv } = producer;
const { runNativeCaller } = caller;

const PAST_START = '2022-09-15T04:09:11.577484Z';
const OWN_IO = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-bootstrap-caller-patch-192';
const ALLOWED = join(OWN_IO, 'a');
const NOW_MS = Date.UTC(2026, 9, 13, 12, 0, 0);
const LEAD = 3600;
const MAGIC = 764;
const TOTAL = '45000000000000000';
const DELEGATED = '30000000000000000';
const NON_DELEGATED = '15000000000000000';
const ENTERPRISE = '13500000000000000';
const BASE = '27000000000000000';
const WITHHELD = '4500000000000000';
const ALONZO_PIN = '104bdbd36634fc48c0221dc481bb37f2556a3c0265aa26ce0820e28e3830cbd3';
const CONWAY_PIN = '16052a510f369584e4a621e3ac2a1ecab121666ebf2b59e868975978046abba5';

function bindUnixSocket(path) {
  mkdirSync(dirname(path), { recursive: true });
  if (statSync(path, { throwIfNoEntry: false })) rmSync(path);
  const run = spawnSync('python3', ['-c', 'import socket,sys; p=sys.argv[1]; s=socket.socket(socket.AF_UNIX); s.bind(p); s.listen(1)', path], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(run.stderr || 'unix-socket');
}

function flag(argv, name) {
  const index = argv.indexOf(name);
  assert.notEqual(index, -1, name);
  return argv[index + 1];
}

function approvedInput(runDir) {
  return {
    approvedBootstrap: true,
    magic: MAGIC,
    nowMs: NOW_MS,
    leadSeconds: LEAD,
    runDir,
    executeNow: false,
  };
}

test('connect185 caller walks the approved bootstrap plan without executing it', async () => {
  assert.notEqual(process.env.MORIARTY_ALLOW_REAL_NODE, '1');
  const homeBefore = process.env.HOME;
  mkdirSync(ALLOWED, { recursive: true });
  const runDir = join(ALLOWED, 'run1');
  rmSync(runDir, { recursive: true, force: true });
  mkdirSync(runDir, { recursive: true });
  const recorded = [];
  const recorder = (argv) => {
    recorded.push(argv.slice());
    return { status: 0, stdout: '' };
  };
  const report = await runNativeCaller({
    ownedProducer: {
      ...approvedInput(runDir),
      unitSpawn: recorder,
    },
  });
  const plan = report.stages.ownedProducer?.schema71;
  const ids = (plan?.sequence || []).map((step) => step.id);
  assert.ok(ids.includes('S1-create-testnet-data'), `caller sequence misses S1: ${ids.join(',') || plan?.code || 'no-plan'}`);
  assert.equal(plan.inputsReady, true, JSON.stringify(plan.blockers || plan.code));
  assert.equal(plan.approvedBootstrap, true);
  assert.equal(report.stages.dispatch?.spawned, true);
  assert.equal(report.stages.dispatch?.code, 'READINESS_TIMEOUT');
  assert.equal(report.ledgerExecution, false);
  assert.equal(report.nativePins, false);
  assert.equal(report.confirmed, false);
  assert.equal(report.startedNode, false);
  assert.equal(report.stages.dispatch.ledgerExecution, false);
  assert.equal(report.stages.dispatch.nativeBudgetMeasured, false);
  assert.equal(report.stages.dispatch.bootstrapComplete, false);
  assert.equal(report.stages.dispatch.nodeStarted, false);
  assert.equal(process.env.HOME, homeBefore);
  assert.equal(plan.paths.home, join(runDir, 'home'));
  assert.notEqual(plan.paths.home, homedir());
  assert.notEqual(plan.paths.home, homeBefore);

  const directPlan = buildBootstrapPlan(approvedInput(runDir), { ownedGenesisArgv });
  const directArgv = ownedGenesisArgv(approvedInput(runDir));
  assert.equal(directArgv.code, 'CREATE_TESTNET_DATA_ARGV');
  assert.equal(directArgv.spawned, false);
  assert.equal(directArgv.supportedCommand, undefined);
  const s1 = plan.sequence.find((step) => step.id === 'S1-create-testnet-data');
  const s2 = plan.sequence.find((step) => step.id === 'S2-create-cardano-byron-zero');
  assert.deepEqual(s1.argv, directArgv.argv);
  assert.deepEqual(s1.argv, directPlan.sequence.find((step) => step.id === 'S1-create-testnet-data').argv);
  assert.equal(s1.argv[0], 'cardano-cli');
  assert.equal(s1.argv.slice(1, 4).join(' '), 'conway genesis create-testnet-data');
  assert.equal(flag(s1.argv, '--pools'), '1');
  assert.equal(flag(s1.argv, '--stake-delegators'), '1');
  assert.equal(flag(s1.argv, '--utxo-keys'), '1');
  assert.equal(flag(s1.argv, '--genesis-keys'), '1');
  assert.equal(flag(s1.argv, '--total-supply'), TOTAL);
  assert.equal(flag(s1.argv, '--delegated-supply'), DELEGATED);
  assert.equal(flag(s1.argv, '--testnet-magic'), String(MAGIC));
  assert.equal(s1.argv.includes('--supply'), false);
  assert.equal(s1.argv.includes('--supply-delegated'), false);
  assert.equal(s1.argv.includes('create-staked'), false);
  assert.equal(s2.argv.slice(1, 4).join(' '), 'conway genesis create-cardano');
  assert.equal(s2.argv.includes('--supply'), false);
  assert.equal(flag(s2.argv, '--gen-utxo-keys'), '0');
  assert.equal(flag(s1.argv, '--start-time'), flag(s2.argv, '--start-time'));
  const start = flag(s1.argv, '--start-time');
  assert.match(start, /^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$/);
  assert.notEqual(start, PAST_START);
  assert.ok(Date.parse(start) > NOW_MS);
  assert.equal(BigInt(DELEGATED) < BigInt(TOTAL), true);
  assert.equal(plan.supplyDigits.total, TOTAL);
  assert.equal(plan.supplyDigits.delegated, DELEGATED);
  assert.equal(plan.supplyDigits.nonDelegated, NON_DELEGATED);
  assert.equal(plan.supplyDigits.enterprise, ENTERPRISE);
  assert.equal(plan.supplyDigits.base, BASE);
  assert.equal(plan.supplyDigits.withheld, WITHHELD);
  assert.equal(plan.numberUsed, false);
  assert.equal(plan.reserveMeasured, false);
  assert.equal(plan.linkedBinaryAbi, 'UNKNOWN');
  assert.equal(plan.threeWaySplit, false);
  for (const value of Object.values(plan.supplyDigits)) assert.equal(typeof value, 'string');

  const idAt = (id) => ids.indexOf(id);
  assert.ok(idAt('S1-create-testnet-data') < idAt('S2-create-cardano-byron-zero'));
  assert.ok(idAt('S2-create-cardano-byron-zero') < idAt('S3-replace-byron-genesis'));
  assert.ok(idAt('S3-replace-byron-genesis') < idAt('S4-hash-byron'));
  assert.ok(idAt('S4-hash-byron') < idAt('S4-hash-shelley'));
  assert.ok(idAt('S4-hash-shelley') < idAt('S4-hash-alonzo'));
  assert.ok(idAt('S4-hash-alonzo') < idAt('S4-hash-conway'));
  assert.ok(idAt('S4-hash-conway') < idAt('await-socket'));
  assert.ok(idAt('await-socket') < idAt('S8-query-tip'));
  const byronHash = plan.sequence.find((step) => step.id === 'S4-hash-byron').argv;
  assert.deepEqual(byronHash.slice(0, 4), ['cardano-cli', 'byron', 'genesis', 'print-genesis-hash']);
  assert.equal(byronHash.includes('hash'), false);
  for (const id of ['S4-hash-shelley', 'S4-hash-alonzo', 'S4-hash-conway']) {
    const argv = plan.sequence.find((step) => step.id === id).argv;
    assert.equal(argv.slice(0, 3).join(' '), 'cardano-cli hash genesis-file');
  }
  const heldHash = resolveBootstrapArgv(plan.sequence.find((step) => step.id === 'S4-hash-byron'), plan, {});
  assert.equal(heldHash.code, 'HASH_HELD_UNTIL_REPLACEMENT');
  assert.equal(heldHash.ok, true);
  assert.equal(heldHash.hashes, null);
  assert.equal(heldHash.hashed, false);
  assert.equal(heldHash.spawned, false);
  const hashResult = report.stages.dispatch.results.find((item) => item.id === 'S4-hash-byron');
  assert.equal(hashResult.code, 'HASH_HELD_UNTIL_REPLACEMENT');
  assert.equal(hashResult.spawned, false);
  assert.equal(report.stages.dispatch.results.find((item) => item.id === 'S3-replace-byron-genesis').code, 'BYRON_REPLACEMENT_HELD');
  assert.equal(report.stages.dispatch.results.some((item) => String(item.id).startsWith('S8-')), false);

  const commandPrefix = (argv) => {
    if (argv[0] === 'cardano-node' && argv[1] === 'run') return 'cardano-node run';
    if (argv[1] === 'address' && argv[2] === 'build') return 'cardano-cli address build';
    return argv.slice(0, 4).join(' ');
  };
  assert.deepEqual(recorded.map(commandPrefix), [
    'cardano-cli conway genesis create-testnet-data',
    'cardano-cli conway genesis create-cardano',
    'cardano-cli address build',
    'cardano-node run',
  ]);
  const nodeArgv = recorded[3];
  assert.equal(flag(nodeArgv, '--port'), '3131');
  assert.equal(flag(nodeArgv, '--host-addr'), '127.0.0.1');
  assert.equal(flag(nodeArgv, '--socket-path'), join(runDir, 'node.socket'));
  assert.equal(flag(nodeArgv, '--config'), join(runDir, 'node-config.json'));
  assert.ok(flag(nodeArgv, '--shelley-kes-key').endsWith('genesis/pools-keys/pool1/kes.skey'));
  assert.ok(flag(nodeArgv, '--shelley-vrf-key').endsWith('genesis/pools-keys/pool1/vrf.skey'));
  assert.ok(flag(nodeArgv, '--shelley-operational-certificate').endsWith('genesis/pools-keys/pool1/opcert.cert'));
  assert.equal(nodeArgv.some((item) => item.includes('delegate1')), false);
  assert.equal(recorded.some((argv) => argv.includes('query')), false);
  assert.equal(recorded.some((argv) => argv.includes('create-staked')), false);
  assert.equal(report.stages.ownedProducer.createNode.code, 'APPROVED_NODE_ARGV');
  assert.equal(report.stages.ownedProducer.createNode.spawned, false);
  assert.equal(report.stages.ownedProducer.createNode.nodeStarted, false);
  assert.equal(report.stages.dispatch.results.find((item) => item.id === 'S7-node-run').nodeStarted, false);
  assert.equal(report.stages.dispatch.results.find((item) => item.id === 'S7-node-run').pidWritten, false);

  const flat = JSON.stringify(plan.sequence);
  assert.equal(flat.includes('shelley.000'), false);
  assert.equal(flat.includes('shelley.001'), false);
  assert.equal(flat.includes('shelley.002'), false);
  assert.equal(flat.includes('yaci-devkit'), false);
  assert.equal(plan.keys.genesis1, 'genesis/genesis-keys/genesis1');
  assert.equal(plan.keys.delegate1, 'genesis/delegate-keys/delegate1');
  assert.equal(plan.keys.funder, 'genesis/utxo-keys/utxo1');
  assert.equal(plan.keys.delegator, 'genesis/stake-delegators/delegator1');
  assert.equal(plan.keys.pool, 'genesis/pools-keys/pool1');
  assert.equal(plan.paths.genesisKey, join(runDir, 'genesis/genesis-keys/genesis1/key.vkey'));
  assert.equal(plan.paths.delegateDir, join(runDir, 'genesis/delegate-keys/delegate1'));

  const configPath = join(runDir, 'node-config.json');
  assert.equal(existsSync(configPath), true);
  assert.equal(existsSync(join(runDir, 'genesis', 'node-config.json')), false);
  const config = JSON.parse(readFileSync(configPath, 'utf8'));
  assert.equal(config.RequiresNetworkMagic, 'RequiresMagic');
  assert.equal(config.ByronGenesisFile, 'genesis/byron-genesis.json');
  assert.equal(config.ShelleyGenesisFile, 'genesis/shelley-genesis.json');
  assert.equal(config.AlonzoGenesisFile, 'genesis/alonzo-genesis.json');
  assert.equal(config.ConwayGenesisFile, 'genesis/conway-genesis.json');
  for (const value of [config.ByronGenesisFile, config.ShelleyGenesisFile, config.AlonzoGenesisFile, config.ConwayGenesisFile]) {
    assert.equal(value.startsWith('./'), false);
    assert.equal(value.includes('/'), true);
    assert.equal(join(runDir, value), join(runDir, 'genesis', value.slice('genesis/'.length)));
    assert.notEqual(join(runDir, value.slice('genesis/'.length)), join(runDir, value));
  }
  assert.equal(Object.hasOwn(config, 'ByronGenesisHash'), false);
  assert.equal(Object.hasOwn(config, 'DijkstraGenesisFile'), false);
  assert.equal(Object.hasOwn(config, 'ExperimentalHardForksEnabled'), false);
  assert.equal(config.TestConwayHardForkAtEpoch, 0);
  const inside = renderPinnedNodeConfig(null, join(runDir, 'genesis', 'node-config.json'), { recipe: 'bootstrap-155' });
  assert.equal(inside.ok, false);
  assert.equal(inside.code, 'NODE_CONFIG_INSIDE_GENESIS');
  const topology = JSON.parse(readFileSync(join(runDir, 'topology.json'), 'utf8'));
  assert.deepEqual(topology.localRoots, []);
  assert.deepEqual(topology.publicRoots, []);
  assert.equal(topology.useLedgerAfter, 0);

  const shelley = JSON.parse(readFileSync(join(runDir, 'spec/shelley.spec.json'), 'utf8'));
  assert.equal(shelley.networkId, 'Testnet');
  assert.equal(shelley.epochLength, 3600);
  assert.equal(shelley.slotLength, 1);
  assert.equal(shelley.protocolParams.minFeeA, 1);
  assert.equal(shelley.protocolParams.minFeeB, 1);
  assert.notEqual(shelley.protocolParams.minFeeA, 0);
  assert.equal(shelley.networkMagic, 0);
  assert.notEqual(shelley.networkMagic, 42);
  assert.equal(shelley.systemStart, start);
  assert.equal(String(shelley.maxLovelaceSupply), TOTAL);
  const alonzo = JSON.parse(readFileSync(join(runDir, 'spec/alonzo.spec.json'), 'utf8'));
  assert.equal(alonzo.maxTxExUnits.exUnitsMem, 16500000);
  assert.equal(alonzo.maxTxExUnits.exUnitsSteps, 10000000000);
  assert.notEqual(alonzo.maxTxExUnits.exUnitsMem, 1);
  assert.notEqual(alonzo.maxTxExUnits.exUnitsSteps, 1);
  assert.notEqual(alonzo.maxTxExUnits.exUnitsMem, 14000000);
  assert.equal(alonzo.maxBlockExUnits.exUnitsMem, 62000000);
  assert.equal(alonzo.collateralPercentage, 150);
  assert.equal(alonzo.maxCollateralInputs, 3);
  assert.equal(alonzo.maxValueSize, 5000);
  assert.equal(alonzo.lovelacePerUTxOWord, 4310);
  assert.equal(alonzo.executionPrices.prMem.numerator, 577);
  assert.equal(alonzo.executionPrices.prMem.denominator, 10000);
  assert.ok(alonzo.costModels.PlutusV1);
  assert.ok(alonzo.costModels.PlutusV2);
  const conwayBytes = readFileSync(join(runDir, 'spec/conway.spec.json'));
  assert.equal(createHash('sha256').update(conwayBytes).digest('hex'), CONWAY_PIN);
  const conway = JSON.parse(conwayBytes.toString('utf8'));
  assert.equal(conway.dRepDeposit, 500000000);
  assert.equal(conway.minFeeRefScriptCostPerByte, 15);
  assert.equal(conway.plutusV3CostModel.length, 251);
  const sourceAlonzo = readFileSync(plan.approved.specPins.alonzo.path);
  assert.equal(createHash('sha256').update(sourceAlonzo).digest('hex'), ALONZO_PIN);
  assert.equal(sourceAlonzo.toString('utf8').includes('"exUnitsMem": 14000000'), true);

  const funds = report.stages.dispatch.results.find((item) => item.id === 'approved-funds');
  assert.equal(funds.code, 'APPROVED_FUNDS_BOUND');
  assert.equal(funds.enterpriseLovelace, ENTERPRISE);
  assert.equal(funds.baseLovelace, BASE);
  assert.equal(typeof funds.enterpriseLovelace, 'string');
  assert.equal(typeof funds.baseLovelace, 'string');
  assert.equal(funds.threeWaySplit, false);
  assert.equal(funds.addressesBound, false);
  assert.equal(funds.reserveMeasured, false);
  assert.equal(funds.linkedBinaryAbi, 'UNKNOWN');
  assert.equal(funds.numberUsed, false);
  const directFunds = bindFreshInitialFunds({ approvedBootstrap: true }, { approvedBootstrap: true, unitOnly: true });
  assert.equal(directFunds.enterpriseLovelace, ENTERPRISE);
  assert.equal(directFunds.withheldLovelace, WITHHELD);
  assert.equal(typeof directFunds.totalSupply, 'string');
  const localFunds = runLocalBootstrap(plan.sequence.find((step) => step.id === 'approved-funds'), plan, { unitOnly: true });
  assert.equal(localFunds.code, 'APPROVED_FUNDS_BOUND');
  assert.equal(existsSync(join(runDir, 'genesis')), false);
  assert.equal(existsSync(join(runDir, 'byron-only')), false);
  assert.equal(existsSync(join(runDir, 'node.pid')), false);
  assert.equal(existsSync(join(runDir, 'db')), false);
  assert.equal(existsSync(join(runDir, 'protocol-parameters.json')), false);

  const missing = awaitOwnedSocket(join(runDir, 'node.socket'), { unitOnly: true });
  assert.equal(missing.code, 'READINESS_TIMEOUT', missing.code);
  writeFileSync(join(runDir, 'node.socket'), '');
  assert.equal(awaitOwnedSocket(join(runDir, 'node.socket'), { unitOnly: true }).code, 'SOCKET_NOT_READY');
  rmSync(join(runDir, 'node.socket'));
  bindUnixSocket(join(runDir, 'node.socket'));
  const ready = awaitOwnedSocket(join(runDir, 'node.socket'), { unitOnly: true });
  assert.equal(ready.code, 'SOCKET_READY');
  assert.equal(ready.nodeStarted, false);
  const legacySock = join(ALLOWED, 'old', 'node.sock');
  bindUnixSocket(legacySock);
  assert.equal(awaitOwnedSocket(legacySock, { unitOnly: true }).code, 'SOCKET_READY');
  const again = [];
  const second = dispatchOwnedProducer(plan, {
    unitOnly: true,
    spawn: (argv) => {
      again.push(argv.slice());
      return { status: 0, stdout: '' };
    },
  });
  assert.equal(second.code, 'PRODUCER_DISPATCHED', second.results?.filter((item) => item.ok === false).map((item) => `${item.id}:${item.code}`).join(','));
  assert.equal(second.nodeStarted, false);
  assert.equal(second.ledgerExecution, false);
  assert.equal(second.nativePins, false);
  assert.equal(second.nativeBudgetMeasured, false);
  assert.equal(second.confirmed, false);
  assert.equal(again.filter((argv) => argv.includes('query')).length, 3);
  assert.ok(again.some((argv) => argv[1] === 'latest' && argv[2] === 'query' && argv[3] === 'tip'));
  assert.equal(second.results.find((item) => item.id === 'S8-query-utxo').code, 'QUERY_ADDRESS_UNBOUND');
  assert.equal(second.results.find((item) => item.id === 'S8-query-utxo').spawned, false);
  assert.equal(existsSync(join(runDir, 'protocol-parameters.json')), false);
  assert.equal(existsSync(join(runDir, 'genesis')), false);

  const unmarked = ownedGenesisArgv({
    magic: MAGIC,
    magicSource: 'caller-chosen-for-genesis',
    ownedHome: join(ALLOWED, 'legacy-home'),
  });
  assert.notEqual(unmarked.code, 'CREATE_TESTNET_DATA_ARGV');

  assert.equal(typeof bootstrap.cleanupOwnedRun, 'function');
  const outside = join(OWN_IO, 'k');
  mkdirSync(outside, { recursive: true });
  writeFileSync(join(outside, 'keep.txt'), 'stay');
  const link = join(ALLOWED, 'escape');
  if (existsSync(link)) rmSync(link);
  symlinkSync(outside, link);
  const escaped = bootstrap.cleanupOwnedRun(link, ALLOWED);
  assert.equal(escaped.ok, false);
  assert.equal(existsSync(join(outside, 'keep.txt')), true);
  rmSync(link, { force: true });
  rmSync(outside, { recursive: true, force: true });
  const historical = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-consumer-integrate-177';
  const refusedHistorical = bootstrap.cleanupOwnedRun(historical, ALLOWED);
  assert.equal(refusedHistorical.ok, false);
  assert.equal(existsSync(historical), true);
  const removed = bootstrap.cleanupOwnedRun(runDir, ALLOWED);
  assert.equal(removed.ok, true);
  assert.equal(removed.code, 'OWNED_RUN_REMOVED');
  assert.equal(existsSync(runDir), false);
  assert.equal(existsSync(ALLOWED), true);
  assert.equal(existsSync(historical), true);
  rmSync(legacySock, { force: true });
});

test('connect185 magic 42 writes nothing and does not spawn', async () => {
  assert.notEqual(process.env.MORIARTY_ALLOW_REAL_NODE, '1');
  const runDir = join(ALLOWED, 'magic42');
  rmSync(runDir, { recursive: true, force: true });
  mkdirSync(runDir, { recursive: true });
  const refused = ownedGenesisArgv({
    approvedBootstrap: true,
    magic: 42,
    nowMs: NOW_MS,
    leadSeconds: LEAD,
    runDir,
  });
  assert.equal(refused.code, 'MAGIC_SOURCE_REFUSED');
  assert.equal(refused.spawned, false);
  assert.equal(refused.supportedCommand, undefined);
  const recorded = [];
  const report = await runNativeCaller({
    ownedProducer: {
      approvedBootstrap: true,
      magic: 42,
      nowMs: NOW_MS,
      leadSeconds: LEAD,
      runDir,
      executeNow: false,
      unitSpawn: (argv) => {
        recorded.push(argv);
        return { status: 0, stdout: '' };
      },
    },
  });
  assert.equal(report.stages.ownedProducer.schema71.code, 'MAGIC_SOURCE_REFUSED');
  assert.equal(report.stages.ownedProducer.schema71.inputsReady, false);
  assert.equal(report.stages.dispatch, undefined);
  assert.equal(recorded.length, 0);
  assert.equal(existsSync(join(runDir, 'genesis')), false);
  assert.equal(existsSync(join(runDir, 'node-config.json')), false);
  assert.equal(existsSync(join(runDir, 'spec')), false);
  rmSync(runDir, { recursive: true, force: true });
});
