import { test } from 'node:test';
import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { chmodSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { CARDANO_CLI, ENCODING_VKEY_HEX, paymentKeyHashOfVkeyCbor } from '../src/native-body.mjs';
import { compileGuardSibling, decideConfirmation } from '../src/native-caller.mjs';
import { compareFinancialFrame } from '../src/native-financial.mjs';
import {
  consumeMinFeeOutput,
  consumePlutusCost,
  dispatchOwnedProducer,
  adoptSpawnedMinAda,
  minFeeArgv,
  minRequiredUtxoArgv,
  observeOwnedClock,
  ownedGenesisArgv,
  parseMinRequiredUtxo,
  plutusCostArgv,
  selectOwnedFunder,
  txOutValue,
} from '../src/native-producer.mjs';
import { acquireBodyEvaluation, produceOwnedRuntime } from '../src/native-runtime.mjs';
import { refuseCallerMinAdaFormula } from '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-71/own71/schemas.mjs';

const TASK = '/home/charl/moriarty-eutxo-validation-2026-10-12';
const SOURCE = join(TASK, 'designers/02-escrow-time/escrow-custody-v5.mori');
const BODY = join(TASK, 't1-native-consumer-repair-70/artifacts/cash-mint.body.json');
const MINT_TXID = '4daa8d22b9a57a2549f9d5cba91eb5955721e6dbd8a2d28119f827ac2aa6ac83';
const AIKEN = '/home/charl/.aiken/bin/aiken';

function bindUnixSocket(path) {
  mkdirSync(dirname(path), { recursive: true });
  if (existsSync(path)) rmSync(path);
  const run = spawnSync('python3', ['-c', 'import socket,sys; p=sys.argv[1]; s=socket.socket(socket.AF_UNIX); s.bind(p); s.listen(1)', path], { encoding: 'utf8' });
  if (run.status !== 0) throw new Error(run.stderr || 'unix-socket');
}

function writeExec(path, lines) {
  writeFileSync(path, `${lines.join('\n')}\n`);
  chmodSync(path, 0o755);
  return path;
}

function nativeReviews(dir) {
  mkdirSync(dir, { recursive: true });
  const sourcePath = join(dir, 'source.txt');
  const sourceText = 'producer-gate-probe\n';
  writeFileSync(sourcePath, sourceText);
  const sourceHash = createHash('sha256').update(sourceText).digest('hex');
  const manifest = {
    sourceHash,
    files: [{ path: sourcePath, sha256: sourceHash, bytes: Buffer.byteLength(sourceText) }],
  };
  const body = JSON.stringify(manifest);
  const manifestPath = join(dir, 'manifest.json');
  const candidatePath = join(dir, 'candidate.json');
  writeFileSync(manifestPath, body);
  writeFileSync(candidatePath, body);
  const manifestHash = createHash('sha256').update(body).digest('hex');
  const authorityPath = join(dir, 'authority.json');
  writeFileSync(authorityPath, JSON.stringify({ manifestSha256: manifestHash }));
  const review = (reviewer, extra = {}) => ({
    verdict: 'APPROVE_SCOPED_T1_NATIVE_IMPLEMENTATION',
    evidenceClass: 'reviewed-implementation',
    reviewClass: 'native-implementation',
    reviewer,
    candidate: manifestHash,
    sourceHash,
    selfApproval: false,
    unitGateProbe: true,
    ...extra,
  });
  writeFileSync(join(dir, 'a.json'), JSON.stringify(review('probe-a')));
  writeFileSync(join(dir, 'b.json'), JSON.stringify(review('probe-b')));
  return {
    reviewA: join(dir, 'a.json'),
    reviewB: join(dir, 'b.json'),
    candidate: candidatePath,
    manifest: manifestPath,
    reviewAuthority: authorityPath,
  };
}

test('owned producer dispatches supported commands and fails closed on bad inputs', () => {
  const held = produceOwnedRuntime({ executeNow: true });
  assert.equal(held.code, 'REAL_NODE_NOT_ARMED');
  assert.equal(held.spawned, false);
  assert.equal(held.executeNow, false);
  assert.equal(held.schema71.spawned, false);
  assert.equal(held.schema71.fundedInNodeInitialFunds, false);
  assert.equal(held.schema71.genesis.supportedCommand, 'cardano-cli legacy genesis create-cardano');
  assert.equal(held.schema71.genesis.rejectedCode, 'YACI_CREATE_CARDANO_ARGV_REJECTED_BY_PINNED_CLI');
  assert.equal(held.schema71.genesis.slotLengthMilliseconds, 1000);
  assert.equal(held.schema71.schema68Adopted, false);
  assert.equal(String(held.schema71.genesis.rejectedArgv.join(' ')).includes('genesis create-cardano'), true);

  const bare = produceOwnedRuntime({ executeNow: true, arm: true, authority: { approved: true } });
  assert.equal(bare.code, 'REAL_NODE_NOT_ARMED');
  assert.equal(bare.spawned, false);

  const root = mkdtempSync(join(tmpdir(), 'moriarty-producer-75-'));
  try {
    const reviews = nativeReviews(join(root, 'reviews'));
    const home = join(root, 'owned');
    const runId = 't1-75-probe';
    mkdirSync(join(home, 'local-clusters', runId, 'utxo-keys'), { recursive: true });
    mkdirSync(join(home, 'local-clusters', runId, 'node', 'genesis'), { recursive: true });
    const vkeyHex = `5820${'11'.repeat(32)}`;
    const hash = paymentKeyHashOfVkeyCbor(vkeyHex);
    assert.notEqual(hash, null);
    const vkeyPath = join(home, 'local-clusters', runId, 'utxo-keys', 'utxo1.vkey');
    writeFileSync(vkeyPath, JSON.stringify({ type: 'PaymentVerificationKeyShelley_ed25519', cborHex: vkeyHex }));
    const shelleyPath = join(home, 'local-clusters', runId, 'node', 'genesis', 'shelley-genesis.json');
    const shelley = {
      slotLength: 1,
      epochLength: 3600,
      networkMagic: 764,
      systemStart: '2026-01-01T00:00:00Z',
      initialFunds: { [hash]: 5000000 },
    };
    writeFileSync(shelleyPath, JSON.stringify(shelley));
    const common = {
      executeNow: true,
      arm: true,
      authority: reviews,
      ownedHome: home,
      runId,
      nodePort: 14021,
      submitApiPort: 14022,
      shelleyGenesisPath: shelleyPath,
      txFile: BODY,
      magic: 764,
      magicSource: 'caller-chosen-for-genesis',
      supplyLovelace: 9000000,
    };
    const refusedHome = produceOwnedRuntime({ ...common, ownedHome: join(home, '.yaci-cli') });
    assert.equal(refusedHome.code, 'DEFAULT_YACI_HOME_REFUSED');
    assert.equal(refusedHome.spawned, false);
    const refusedDevnet = produceOwnedRuntime({
      ...common,
      ownedHome: '/home/charl/moriarty-eutxo-validation-2026-10-12/tooling/runtime/devnet/private',
    });
    assert.equal(refusedDevnet.code, 'DEVNET_HOME_REFUSED');
    const refusedRun = produceOwnedRuntime({ ...common, runId: 'default' });
    assert.equal(refusedRun.code, 'RUN_ID_REFUSED');
    const refusedProfile = produceOwnedRuntime({ ...common, genesisProfile: 'zero_fee' });
    assert.equal(refusedProfile.code, 'GENESIS_PROFILE_REFUSED');
    assert.equal(refusedProfile.spawned, false);
    const refusedProof = produceOwnedRuntime({ ...common, proofTxtOnly: true });
    assert.equal(refusedProof.code, 'PROOF_TXT_NOT_READINESS');
    const refusedWrapper = produceOwnedRuntime({ ...common, launcher: 'node start.mjs' });
    assert.equal(refusedWrapper.code, 'YACI_WRAPPER_REFUSED');
    const refusedEval = produceOwnedRuntime({ ...common, evaluate: true });
    assert.equal(refusedEval.code, 'NO_CLI_EVALUATE_COMMAND');
    assert.equal(refusedEval.spawned, false);
    const refusedTop = produceOwnedRuntime({ ...common, topLevelGenesis: true });
    assert.equal(refusedTop.code, 'YACI_CREATE_CARDANO_ARGV_REJECTED_BY_PINNED_CLI');
    const refusedSlot = produceOwnedRuntime({ ...common, slotLength: 1 });
    assert.equal(refusedSlot.code, 'SLOT_LENGTH_REFUSED');
    const refusedMagic = produceOwnedRuntime({ ...common, magic: 42 });
    assert.equal(refusedMagic.code, 'MAGIC_SOURCE_REFUSED');
    const unfunded = produceOwnedRuntime({
      ...common,
      shelleyGenesisPath: join(root, 'missing-shelley.json'),
    });
    assert.equal(unfunded.code, 'REAL_NODE_NOT_ARMED');
    assert.equal(unfunded.inputsReady, true);
    assert.equal(unfunded.schema71.fundedInNodeInitialFunds, false);
    assert.equal(unfunded.schema71.bootstrapComplete, false);
    assert.equal(unfunded.spawned, false);
    writeFileSync(shelleyPath, JSON.stringify({ ...shelley, initialFunds: {} }));
    const missingMap = produceOwnedRuntime(common);
    assert.equal(missingMap.code, 'REAL_NODE_NOT_ARMED');
    assert.equal(missingMap.schema71.fundedInNodeInitialFunds, false);
    assert.equal(missingMap.schema71.callerInitialFundsIgnored, true);
    assert.equal(missingMap.spawned, false);
    writeFileSync(shelleyPath, JSON.stringify({ ...shelley, initialFunds: { [hash]: 0 } }));
    const emptyValue = produceOwnedRuntime(common);
    assert.equal(emptyValue.code, 'REAL_NODE_NOT_ARMED');
    assert.equal(emptyValue.schema71.fundedInNodeInitialFunds, false);
    const emptySupply = produceOwnedRuntime({ ...common, supplyLovelace: 0 });
    assert.equal(emptySupply.code, 'EMPTY_FUNDER_NOT_A_UTXO');
    writeFileSync(shelleyPath, JSON.stringify(shelley));
    const ready = produceOwnedRuntime(common);
    assert.equal(ready.code, 'REAL_NODE_NOT_ARMED', JSON.stringify(ready.blockers || ready.schema71?.blockers));
    assert.equal(ready.spawned, false);
    assert.equal(ready.inputsReady, true);
    assert.equal(ready.authorityAccepted, true);
    assert.equal(ready.ledgerFunding, false);
    assert.equal(ready.schema71.fundedInNodeInitialFunds, false);
    assert.equal(ready.schema71.bootstrapComplete, false);
    const startAt = ready.wouldSpawn.findIndex((step) => step.id === 'start-node');
    const tipAt = ready.wouldSpawn.findIndex((step) => step.id === 'query-tip');
    const createAt = ready.wouldSpawn.findIndex((step) => step.id === 'create-node');
    assert.equal(createAt < startAt, true);
    assert.equal(startAt < tipAt, true);
    const legacy = ready.wouldSpawn.find((step) => step.id === 'legacy-genesis');
    assert.equal(legacy.argv[1], 'legacy');
    assert.equal(legacy.argv.includes('--conway-era'), true);
    assert.equal(legacy.argv[legacy.argv.indexOf('--slot-length') + 1], '1000');
    assert.equal(legacy.argv.includes('evaluate'), false);
    const created = ready.wouldSpawn.find((step) => step.id === 'create-node');
    assert.equal(created.argv.includes('create-node'), true);
    assert.equal(created.argv[created.argv.indexOf('--slot-length') + 1], '1');
    assert.equal(created.argv[created.argv.indexOf('--start') + 1], 'false');
    assert.equal(created.argv.includes('600'), false);
    const tipArgv = ready.wouldSpawn.find((step) => step.id === 'query-tip').argv;
    assert.equal(tipArgv.some((item) => item.endsWith('node.sock')), true);
    assert.equal(tipArgv.includes('764'), true);
    const bareDispatch = dispatchOwnedProducer(ready.schema71, {
      unitOnly: true,
      spawn: () => ({ status: 0, stdout: '' }),
    });
    assert.notEqual(bareDispatch.code, 'PRODUCER_DISPATCHED');
    assert.equal(bareDispatch.unitOnly, true);
    assert.equal(bareDispatch.evidenceClass, 'unit-fixture');
    assert.equal(bareDispatch.ledgerExecution, false);
    assert.equal(bareDispatch.nativeBudgetMeasured, false);
    assert.equal(bareDispatch.ledgerFunding, false);
    assert.equal(bareDispatch.fundedInNodeInitialFunds, false);
    const calls = [];
    const dispatched = dispatchOwnedProducer(ready.schema71, {
      unitOnly: true,
      spawn: (argv, step) => {
        calls.push({ id: step.id, argv });
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
            networkMagic: ready.schema71.magic,
            systemStart: '2026-01-01T00:00:00Z',
            minFeeA: 44,
            minFeeB: 155381,
            maxLovelaceSupply: 45000000000000000,
            genDelegs: { ['11'.repeat(28)]: { delegate: '22'.repeat(28), vrf: '33'.repeat(64) } },
            initialFunds: {},
          }));
          writeFileSync(paths.byronWork, JSON.stringify({ startTime: 0, protocolConsts: { k: 18 } }));
          writeFileSync(paths.alonzoWork, JSON.stringify({ seed: 1 }));
          writeFileSync(paths.conwayWork, JSON.stringify({ seed: 1 }));
        }
        if (step.id.startsWith('initial-addr') || step.id.startsWith('role-address')) {
          const out = argv[argv.indexOf('--out-file') + 1];
          mkdirSync(dirname(out), { recursive: true });
          writeFileSync(out, `addr_test1fixture${step.id}\n`);
        }
        if (step.id === 'start-node') {
          mkdirSync(dirname(paths.socketPath), { recursive: true });
          bindUnixSocket(paths.socketPath);
        }
        if (step.id === 'query-tip') {
          return { status: 0, stdout: JSON.stringify({ slot: 10, hash: 'ce'.repeat(32), block: 3 }) };
        }
        if (step.id === 'query-era-history') {
          return { status: 0, stdout: JSON.stringify({ type: 'EraHistory', description: 'EraHistory', cborHex: '9fff' }) };
        }
        if (step.id === 'query-funder-utxo' || step.id.startsWith('funding-confirm')) {
          return { status: 0, stdout: `${'0123456789abcdef'.repeat(4)} 0 9000000\n` };
        }
        if (step.id === 'calculate-min-fee') return { status: 0, stdout: '{"fee":123456}\n' };
        if (step.id === 'calculate-plutus-script-cost') {
          return {
            status: 0,
            stdout: JSON.stringify([{
              scriptHash: '11'.repeat(28),
              executionUnits: { memory: 500, steps: 64100 },
              lovelaceCost: 1,
            }]),
          };
        }
        return { status: 0, stdout: '' };
      },
    });
    assert.equal(dispatched.code, 'PRODUCER_DISPATCHED', JSON.stringify(dispatched.results?.map((item) => item.code)));
    assert.equal(dispatched.unitOnly, true);
    assert.equal(dispatched.evidenceClass, 'unit-fixture');
    assert.equal(dispatched.ledgerExecution, false);
    assert.equal(dispatched.nativePins, false);
    assert.equal(dispatched.nativeBudgetMeasured, false);
    assert.equal(dispatched.ledgerFunding, false);
    assert.equal(dispatched.fundedInNodeInitialFunds, false);
    assert.equal(dispatched.bootstrapComplete, false);
    const spawnedIds = ready.schema71.sequence.filter((step) => step.run === true && step.local == null).map((step) => step.id);
    assert.deepEqual(calls.map((item) => item.id), spawnedIds);
    const era = dispatched.results.find((item) => item.id === 'query-era-history');
    assert.equal(era.era.innerCborFieldNames, null);
    assert.equal(era.era.cborHex, '9fff');
    assert.equal(era.measured, false);
    const cost = dispatched.results.find((item) => item.id === 'calculate-plutus-script-cost');
    assert.equal(cost.code, 'PLUTUS_COST_PARSED');
    assert.equal(cost.nativeBudgetMeasured, false);
    assert.equal(cost.unitOnly, true);
    assert.equal(cost.cost.memory, 500);
    const genesisTip = observeOwnedClock({
      tip: {},
      era: { type: 'EraHistory', description: 'EraHistory', cborHex: '9fff' },
      genesis: shelley,
    });
    assert.equal(genesisTip.code, 'GENESIS_TIP_HAS_NO_SLOT');
    assert.equal(genesisTip.slot, null);
    assert.equal(genesisTip.defaultedSlot, false);
    const bound = observeOwnedClock({
      tip: { slot: 10, hash: 'ab', block: 1 },
      era: { boundSlot: 1, boundTime: 2 },
      genesis: shelley,
    });
    assert.equal(bound.code, 'ERA_HISTORY_WIRE_IS_TEXT_ENVELOPE');
    const quotedSlot = observeOwnedClock({
      tip: { slot: 10, hash: 'ab', block: 1 },
      era: { type: 'EraHistory', description: 'EraHistory', cborHex: '9fff' },
      genesis: { ...shelley, slotLength: '1' },
    });
    assert.equal(quotedSlot.code, 'SHELLEY_SLOT_LENGTH_IS_SECONDS_NUMBER');
    assert.equal(consumePlutusCost({ memory: 1, steps: 1 }).code, 'SUPPLIED_UNITS_NOT_A_MEASUREMENT');
    assert.equal(consumeMinFeeOutput({ fee: 165897, txBytes: 200 }).code, 'TX_BYTES_NOT_A_COMMAND_FIELD');
    assert.equal(parseMinRequiredUtxo().code, 'MIN_REQUIRED_UTXO_SHOW_MISMATCH');
    assert.equal(parseMinRequiredUtxo('Lovelace 1\n').code, 'MIN_REQUIRED_UTXO_SHOW_MISMATCH');
    assert.equal(parseMinRequiredUtxo('Coin 1 extra').code, 'MIN_REQUIRED_UTXO_SHOW_MISMATCH');
    const capturedCoin = readFileSync(join(TASK, 't1-offline-output-costs-72/captures/min-utxo-cash-mint-0-v0.stdout'), 'utf8');
    const parsedCoin = parseMinRequiredUtxo(capturedCoin);
    assert.equal(parsedCoin.code, 'MIN_REQUIRED_UTXO_PARSED');
    assert.equal(parsedCoin.showFormatCaptured, true);
    assert.equal(parsedCoin.minAdaMeasured, false);
    assert.equal(parsedCoin.nativeBudgetMeasured, false);
    assert.equal(parsedCoin.budgetPin, false);
    assert.equal(parsedCoin.protocolMinAda, 'unmeasured');
    assert.equal(parsedCoin.lovelace, Number(capturedCoin.match(/(\d+)/)[1]));
    const unarmedCoin = adoptSpawnedMinAda(parsedCoin, { unitOnly: false, spawnedCli: true, exit: 0 });
    assert.equal(unarmedCoin.minAdaMeasured, false);
    assert.equal(process.env.MORIARTY_ALLOW_REAL_NODE, undefined);
    const coinStep = dispatchOwnedProducer({
      inputsReady: true,
      sequence: [{ id: 'calculate-min-required-utxo', run: true, argv: ['cardano-cli', 'conway', 'transaction', 'calculate-min-required-utxo'] }],
    }, { unitOnly: true, spawn: () => ({ status: 0, stdout: capturedCoin }) });
    assert.equal(coinStep.results[0].code, 'MIN_REQUIRED_UTXO_PARSED');
    assert.equal(coinStep.results[0].minAdaMeasured, false);
    assert.equal(coinStep.nativeBudgetMeasured, false);
    assert.equal(refuseCallerMinAdaFormula(4310, 100).code, 'MIN_ADA_FORMULA_IS_NOT_THE_CLI_COMMAND');
    const displayWord = txOutValue({
      address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
      lovelace: 2000000,
      assets: [{ quantity: 100, policyId: 'policy c760611894de8a0b57c3464ec40c49dce50440d9f238abb9d13d4555', assetNameHex: '43617368' }],
    });
    assert.equal(displayWord.code, 'TX_OUT_ASSET_REFUSED');
    const asciiName = txOutValue({
      address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
      lovelace: 2000000,
      assets: [{ quantity: 100, policyId: 'c760611894de8a0b57c3464ec40c49dce50440d9f238abb9d13d4555', assetNameHex: 'Cash' }],
    });
    assert.equal(asciiName.code, 'TX_OUT_ASSET_REFUSED');
    const shapedOut = txOutValue({
      address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v',
      lovelace: 2000000,
      assets: [{ quantity: 100, policyId: 'c760611894de8a0b57c3464ec40c49dce50440d9f238abb9d13d4555', assetNameHex: '43617368' }],
    });
    assert.equal(shapedOut.ok, true);
    assert.equal(shapedOut.budgetPin, false);
    assert.equal(shapedOut.txOut.includes('"'), false);
    assert.equal(shapedOut.txOut.includes('policy '), false);
    assert.equal(shapedOut.txOut.endsWith('43617368'), true);
    const refusedOutputs = produceOwnedRuntime({
      ...common,
      protocolParamsFile: join(TASK, 'tooling/runtime/actual-protocol-parameters.json'),
      outputs: [{ address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v', lovelace: 2000000, assets: [{ quantity: 100, policyId: 'policy aabb', assetNameHex: '43617368' }] }],
    });
    assert.equal(refusedOutputs.code, 'TX_OUT_ASSET_REFUSED');
    assert.equal(refusedOutputs.spawned, false);
    const withOutput = produceOwnedRuntime({
      ...common,
      protocolParamsFile: join(TASK, 'tooling/runtime/actual-protocol-parameters.json'),
      outputs: [{ address: 'addr_test1vr6aqdeafz0rspg0zruve0kfexy2wad665n70v3e7ftuh9s0t7p9v', lovelace: 2000000, assets: [{ quantity: 100, policyId: 'c760611894de8a0b57c3464ec40c49dce50440d9f238abb9d13d4555', assetNameHex: '43617368' }] }],
    });
    assert.equal(withOutput.code, 'REAL_NODE_NOT_ARMED');
    assert.equal(withOutput.inputsReady, true);
    assert.equal(withOutput.spawned, false);
    const minStep = withOutput.wouldSpawn.find((step) => step.id === 'calculate-min-required-utxo');
    assert.equal(minStep.argv.includes('calculate-min-required-utxo'), true);
    assert.equal(minStep.argv.includes('--output-json'), false);
    assert.equal(minStep.argv.includes(shapedOut.txOut), true);
    const assumedFee = minFeeArgv({
      cli: CARDANO_CLI,
      txBodyFile: BODY,
      protocolParamsFile: join(TASK, 'tooling/runtime/actual-protocol-parameters.json'),
      witnessCount: 1,
      witnessCountProvenance: 'ASSUMED-unsigned-body-no-key-analysis',
    });
    assert.equal(assumedFee.ok, true);
    assert.equal(assumedFee.feeAppliesToSignedTx, false);
    assert.equal(assumedFee.budgetPin, false);
    assert.equal(assumedFee.argv.includes('--witness-count'), true);
    assert.equal(assumedFee.argv.includes('--output-json'), false);
    assert.equal(minFeeArgv({
      cli: CARDANO_CLI,
      txBodyFile: BODY,
      protocolParamsFile: join(TASK, 'tooling/runtime/actual-protocol-parameters.json'),
    }).code, 'WITNESS_COUNT_ABSENT');
    const minAda = minRequiredUtxoArgv({
      cli: CARDANO_CLI,
      protocolParamsFile: join(TASK, 'tooling/runtime/actual-protocol-parameters.json'),
      txOut: 'addr_test1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq 2000000',
    });
    assert.equal(minAda.ok, true);
    assert.equal(minAda.argv.includes('--output-json'), false);
    assert.equal(minAda.argv.includes('calculate-min-required-utxo'), true);
    const offline = plutusCostArgv({
      cli: CARDANO_CLI,
      mode: 'offline',
      txFile: BODY,
      eraHistoryFile: join(root, 'era.json'),
      utxoFile: join(root, 'utxo.json'),
      protocolParamsFile: join(root, 'protocol.json'),
      startTime: '2026-01-01T00:00:00Z',
    });
    assert.equal(offline.argv.includes('offline'), true);
    assert.equal(offline.argv.includes('--era-history-file'), true);
    assert.equal(offline.argv.includes('evaluate'), false);
    assert.equal(ownedGenesisArgv({ topLevelGenesis: true }).code, 'YACI_CREATE_CARDANO_ARGV_REJECTED_BY_PINNED_CLI');
    const encoding = selectOwnedFunder({
      roles: {
        lender: { vkeyHex: ENCODING_VKEY_HEX },
        borrower: { vkeyHex: `5820${'22'.repeat(32)}` },
        issuer: { vkeyHex: `5820${'33'.repeat(32)}` },
      },
    });
    assert.equal(encoding.code, 'ENCODING_FIXTURE_NOT_A_ROLE');
    const emptyFunder = selectOwnedFunder({
      fundedInNodeInitialFunds: true,
      roles: {
        lender: { vkeyHex },
        borrower: { vkeyHex: `5820${'22'.repeat(32)}` },
        issuer: { vkeyHex: `5820${'33'.repeat(32)}` },
      },
      utxo: { value: {}, lovelace: 0, ownerHash: hash, address: 'addr', txId: 'ab'.repeat(32), index: 0 },
    });
    assert.equal(emptyFunder.code, 'EMPTY_FUNDER_NOT_A_UTXO');
    const shaped = selectOwnedFunder({
      fundedInNodeInitialFunds: true,
      unitOnly: true,
      sourceGenesis: shelleyPath,
      roles: {
        lender: { vkeyHex },
        borrower: { vkeyHex: `5820${'22'.repeat(32)}` },
        issuer: { vkeyHex: `5820${'33'.repeat(32)}` },
      },
      utxo: { lovelace: 5000000, ownerHash: hash, address: 'addr_test1owned', txId: 'cd'.repeat(32), index: 0 },
    });
    assert.equal(shaped.code, 'FUNDER_SHAPE_READY');
    assert.equal(shaped.bootstrapComplete, false);
    assert.equal(shaped.ledgerFunding, false);
    assert.equal(shaped.nodeUtxoQueried, false);
    assert.equal(shaped.encodingFixture, false);
    assert.notEqual(shaped.lender, shaped.borrower);
    const socket = join(root, 'node.sock');
    writeFileSync(socket, '');
    const costCli = writeExec(join(root, 'cost.sh'), [
      '#!/bin/sh',
      'printf %s \'[{"scriptHash":"11","executionUnits":{"memory":20,"steps":30},"lovelaceCost":4}]\'',
      'exit 0',
    ]);
    const parsedCost = acquireBodyEvaluation({
      cliPath: costCli,
      bodyPath: BODY,
      socket,
      execute: true,
      magic: 764,
      magicSource: 'shelley-networkMagic',
    });
    assert.equal(parsedCost.code, 'PLUTUS_COST_PARSED');
    assert.equal(parsedCost.nativeBudgetMeasured, false);
    assert.equal(parsedCost.unitOnly, true);
    assert.equal(parsedCost.submitBlocked, true);
    const supplied = writeExec(join(root, 'supplied.sh'), [
      '#!/bin/sh',
      'printf %s \'{"memory":1,"steps":1}\'',
      'exit 0',
    ]);
    const suppliedCost = acquireBodyEvaluation({
      cliPath: supplied,
      bodyPath: BODY,
      socket,
      execute: true,
      magic: 764,
      magicSource: 'shelley-networkMagic',
    });
    assert.equal(suppliedCost.code, 'SUPPLIED_UNITS_NOT_A_MEASUREMENT');
    assert.equal(suppliedCost.nativeBudgetMeasured, false);
    const evaluate = acquireBodyEvaluation({ cliPath: costCli, bodyPath: BODY, execute: true, evaluate: true });
    assert.equal(evaluate.code, 'NO_CLI_EVALUATE_COMMAND');
    assert.equal(evaluate.spawned, false);
    const realHeld = acquireBodyEvaluation({ cliPath: CARDANO_CLI, bodyPath: BODY, socket, execute: true });
    assert.equal(realHeld.code, 'REAL_QUERY_NOT_ARMED');
    assert.equal(realHeld.spawned, false);
    assert.match(realHeld.command, /calculate-plutus-script-cost/);
    const confirmation = decideConfirmation({
      code: 'HEAD_READBACK',
      fieldCount: 13,
      unitOnly: true,
      queryProvenance: true,
      txidLinked: true,
      helperConfirmed: true,
      financialMatch: true,
    }, 'confirmed');
    assert.equal(confirmation.code, 'READBACK_UNIT_ONLY');
    assert.equal(confirmation.confirmed, false);
    assert.equal(confirmation.receiptStatusIgnored, 'confirmed');
    const datum = compareFinancialFrame(
      { balances: [], allowances: [], obligations: [], work: { spent: '0' }, continuingCash: 100, payoutCash: 0 },
      { balances: [], allowances: [], obligations: [], work: { spent: '49' }, continuingCash: 0, payoutCash: 100 },
    );
    assert.equal(datum.code, 'READBACK_FINANCIAL_MISMATCH');
    assert.equal(datum.confirmed, false);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test('cached source admit and the frozen cash body stay on the real CLI', () => {
  const txid = spawnSync(CARDANO_CLI, ['conway', 'transaction', 'txid', '--tx-body-file', BODY, '--output-text'], { encoding: 'utf8' });
  assert.equal(txid.status, 0, txid.stderr);
  assert.equal(txid.stdout.trim(), MINT_TXID);
  const sibling = compileGuardSibling({
    compilerBin: AIKEN,
    sourcePath: SOURCE,
    previousProgramHash: '00'.repeat(32),
  });
  assert.equal(sibling.code, 'SIBLING_COMPILED', sibling.code);
  assert.equal(sibling.spawned, false);
  assert.equal(sibling.compilerInvoked, false);
  assert.equal(sibling.cacheHit, true);
  assert.equal(sibling.siblingEmitted, true);
  const dir = mkdtempSync(join(tmpdir(), 'moriarty-hashline-'));
  try {
    const sourcePath = join(dir, 'guard.mori');
    writeFileSync(sourcePath, readFileSync(SOURCE));
    const compiler = writeExec(join(dir, 'compiler.sh'), [
      '#!/bin/sh',
      'echo programHash=' + 'cd'.repeat(32),
    ]);
    const hashLine = compileGuardSibling({
      compilerBin: compiler,
      sourcePath,
      previousProgramHash: 'ab'.repeat(32),
    });
    assert.equal(hashLine.code, 'HASH_LINE_NOT_A_COMPILE');
    assert.equal(hashLine.siblingEmitted, false);
    assert.equal(hashLine.unitOnly, true);
  } finally {
    rmSync(dir, { recursive: true, force: true });
  }
});
