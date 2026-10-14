/**
 * Owned producer and evaluator for the T1 caller.
 * Schema 71 parsers decide the wire. This module builds the supported argv
 * and consumes command output. It does not create a cluster by itself.
 * A unit spawn callback stays unit-only. Ledger flags stay false.
 */
import { spawnSync } from 'node:child_process';
import { copyFileSync, existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { ENCODING_VKEY_HEX, paymentKeyHashOfVkeyCbor } from './native-body.mjs';
import {
  bindFinancialFromDispatch,
  financialContinuation,
  planPlutusCostRebuild,
} from './native-integrate.mjs';
import {
  QUERY_AFTER_SOCKET,
  buildBootstrapPlan,
  canonicalSupply,
  parseQueryUtxoText,
  planApprovedBootstrap,
  readWord64Coin,
  resolveBootstrapArgv,
  runLocalBootstrap,
} from './native-bootstrap.mjs';
import { classifyWitnessCountProvenance, parseFeeStdout } from './output-fee.mjs';
import {
  createNodeArgvShape,
  freshFaucetKeys,
  parseEraHistoryEnvelope,
  parseMinFee,
  parsePlutusScriptCost,
  parseProtocolParams,
  parseTip,
  planWindows,
  portMapping,
  refuseCallerMinAdaFormula,
  slotLengthMsFromShelley,
  yaciLayout,
} from '../owned/runtime-schemas-71/schemas.mjs';
import { planWindows as custodyPlanWindows } from './custody-time.mjs';

export const SCHEMA71_MODULE = '../owned/runtime-schemas-71/schemas.mjs';
export const CREATE_TIME_SYSTEM_START = '2022-09-15T04:09:11.577484Z';
const REFUSED_PROFILES = new Set(['zero_fee', 'zero_min_utxo_value', 'zero_fee_and_min_utxo_value']);
const PINNED_SOURCE_ROOT = '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-68/sources';
const CREATE_CARDANO_DIR = 'cardano-cli/cardano-cli/test/cardano-cli-test/files/input/conway/create-cardano';
export const PINNED_GENESIS_TEMPLATES = {
  byron: join(PINNED_SOURCE_ROOT, CREATE_CARDANO_DIR, 'genesis.byron.spec.json'),
  shelley: join(PINNED_SOURCE_ROOT, 'cardano-cli/configuration/cardano/mainnet-shelley-genesis.json'),
  alonzo: join(PINNED_SOURCE_ROOT, CREATE_CARDANO_DIR, 'genesis.alonzo.spec.json'),
  conway: join(PINNED_SOURCE_ROOT, 'cardano-cli/configuration/cardano/mainnet-conway-genesis.json'),
  nodeConfig: join(PINNED_SOURCE_ROOT, 'yaci-devkit/applications/cli/src/main/resources/localcluster/templates/devnet/templates/configuration.json'),
  topology: join(PINNED_SOURCE_ROOT, 'yaci-devkit/applications/cli/src/main/resources/localcluster/templates/devnet/templates/topology-peer.json'),
};
const REJECTED_GENESIS = {
  rejectedArgv: ['cardano-cli', 'genesis', 'create-cardano', '--slot-length', '1'],
  rejectedCode: 'YACI_CREATE_CARDANO_ARGV_REJECTED_BY_PINNED_CLI',
  supportedCommand: 'cardano-cli legacy genesis create-cardano',
  slotLengthMilliseconds: 1000,
};

function closed(code, extra = {}) {
  return {
    ok: false,
    code,
    spawned: false,
    executeNow: false,
    ledgerExecution: false,
    nativePins: false,
    nativeBudgetMeasured: false,
    ledgerFunding: false,
    runtimePinsCreated: false,
    measured: false,
    parityExecuted: false,
    selfApproval: false,
    ...extra,
  };
}

function readJson(path) {
  return JSON.parse(readFileSync(path, 'utf8'));
}

export function ownedGenesisArgv(input = {}) {
  if (input.approvedBootstrap === true) {
    const planned = planApprovedBootstrap(input);
    if (!planned.ok) return { ...planned, spawned: false, executed: false };
    const step = planned.steps.find((item) => item.id === 'S1-create-testnet-data');
    if (!Array.isArray(step?.argv)) return closed('CREATE_TESTNET_DATA_ARGV_ABSENT');
    return {
      ok: true,
      code: 'CREATE_TESTNET_DATA_ARGV',
      spawned: false,
      executed: false,
      ledgerExecution: false,
      nativePins: false,
      ledgerFunding: false,
      measured: false,
      initialFundsWrittenByThisCommand: false,
      argv: step.argv,
    };
  }
  if (input.topLevelGenesis === true || input.commandStyle === 'toplevel') {
    return closed('YACI_CREATE_CARDANO_ARGV_REJECTED_BY_PINNED_CLI', REJECTED_GENESIS);
  }
  const slot = input.slotLength == null ? 1000 : input.slotLength;
  if (slot !== 1000) {
    return closed('SLOT_LENGTH_REFUSED', {
      ...REJECTED_GENESIS,
      detail: 'legacy genesis --slot-length is milliseconds',
      passed: slot,
    });
  }
  const magicSourceOk = input.magicSource === 'caller-chosen-for-genesis' || input.magicSource === 'shelley-networkMagic';
  if (!Number.isInteger(input.magic) || !magicSourceOk || input.magicSource === 'packaged-comment') {
    return closed('MAGIC_SOURCE_REFUSED', REJECTED_GENESIS);
  }
  const home = input.ownedHome;
  if (typeof home !== 'string' || !isAbsolute(home)) return closed('YACI_HOME_MUST_BE_ABSOLUTE', REJECTED_GENESIS);
  if (typeof input.genesisDir !== 'string' || !isAbsolute(input.genesisDir) || !input.genesisDir.startsWith(`${home}/`)) {
    return closed('GENESIS_TEMPLATE_ABSENT', { ...REJECTED_GENESIS, missing: ['genesisDir'] });
  }
  const files = ['byronTemplate', 'shelleyTemplate', 'alonzoTemplate', 'conwayTemplate'];
  const missing = files.filter((key) => !templatePathAllowed(input[key], home));
  if (missing.length) return closed('GENESIS_TEMPLATE_ABSENT', { ...REJECTED_GENESIS, missing });
  const genUtxoKeys = input.genUtxoKeys == null ? 3 : input.genUtxoKeys;
  if (!Number.isInteger(genUtxoKeys) || genUtxoKeys < 3) return closed('FRESH_KEYS_REQUIRED', REJECTED_GENESIS);
  const supply = canonicalSupply(input.supplyLovelace, { zeroCode: 'EMPTY_FUNDER_NOT_A_UTXO' });
  if (!supply.ok) return closed(supply.code, REJECTED_GENESIS);
  return {
    ok: true,
    code: 'LEGACY_CREATE_CARDANO_ARGV',
    spawned: false,
    executed: false,
    ledgerExecution: false,
    nativePins: false,
    ledgerFunding: false,
    measured: false,
    initialFundsWrittenByThisCommand: false,
    argv: [
      input.cli,
      'legacy', 'genesis', 'create-cardano',
      '--conway-era',
      '--genesis-dir', input.genesisDir,
      '--gen-genesis-keys', '3',
      '--gen-utxo-keys', String(genUtxoKeys),
      '--slot-length', '1000',
      '--security-param', '18',
      '--supply', supply.digits,
      '--testnet-magic', String(input.magic),
      '--byron-template', input.byronTemplate,
      '--shelley-template', input.shelleyTemplate,
      '--alonzo-template', input.alonzoTemplate,
      '--conway-template', input.conwayTemplate,
    ],
    ...REJECTED_GENESIS,
  };
}

function templatePathAllowed(path, home) {
  if (typeof path !== 'string' || !isAbsolute(path) || !existsSync(path)) return false;
  if (path.startsWith(`${home}/`)) return true;
  return path.startsWith(`${PINNED_SOURCE_ROOT}/`);
}

function repeatedKeyMaterial(vkeyHex) {
  const body = String(vkeyHex || '').replace(/^5820/, '');
  if (!/^[0-9a-f]{64}$/.test(body)) return true;
  const banned = new Set(['ab', 'c3', 'd4', 'f5', 'cd']);
  if (banned.has(body.slice(0, 2)) && body === body.slice(0, 2).repeat(32)) return true;
  return body === body[0].repeat(64);
}

export function plutusCostArgv(input = {}) {
  if (input.command === 'evaluate' || input.evaluate === true) {
    return closed('NO_CLI_EVALUATE_COMMAND', {
      command: 'conway transaction calculate-plutus-script-cost',
    });
  }
  const cli = input.cli || input.cliPath;
  if (typeof cli !== 'string' || cli.length === 0) return closed('CLI_REQUIRED');
  const txFile = input.txFile || input.bodyPath;
  if (typeof txFile !== 'string' || txFile.length === 0) {
    return closed('EVALUATION_UNAVAILABLE', { reason: 'body-absent', command: 'conway transaction calculate-plutus-script-cost' });
  }
  const mode = input.mode === 'offline' ? 'offline' : 'online';
  if (mode === 'offline') {
    const start = input.byronGenesis
      ? ['--genesis-file', input.byronGenesis]
      : input.startTimePosix != null
        ? ['--start-time-posix', String(input.startTimePosix)]
        : input.startTime ? ['--start-time-utc', input.startTime] : null;
    if (!start || !input.eraHistoryFile || !input.utxoFile || !input.protocolParamsFile) {
      return closed('EVALUATION_CONTEXT_ABSENT', {
        command: 'conway transaction calculate-plutus-script-cost offline',
        need: ['start', 'era-history-file', 'utxo-file', 'protocol-params-file', 'tx-file'],
      });
    }
    const argv = [
      cli, 'conway', 'transaction', 'calculate-plutus-script-cost', 'offline',
      ...start,
      '--era-history-file', input.eraHistoryFile,
      '--utxo-file', input.utxoFile,
      '--protocol-params-file', input.protocolParamsFile,
      '--tx-file', txFile,
    ];
    if (input.outFile) argv.push('--out-file', input.outFile);
    return {
      ok: true,
      code: 'PLUTUS_COST_ARGV',
      mode: 'offline',
      argv,
      command: 'conway transaction calculate-plutus-script-cost offline',
      measured: false,
      nativeBudgetMeasured: false,
      spawned: false,
    };
  }
  if (input.magicSource !== 'shelley-networkMagic' || !Number.isInteger(input.magic)) {
    return closed('MAGIC_SOURCE_REFUSED', {
      command: 'conway transaction calculate-plutus-script-cost online',
      argv: [cli, 'conway', 'transaction', 'calculate-plutus-script-cost', 'online', '--tx-file', txFile],
    });
  }
  if (typeof input.socket !== 'string' || input.socket.length === 0) {
    return closed('EVALUATION_UNAVAILABLE', { reason: 'socket-absent', command: 'conway transaction calculate-plutus-script-cost online' });
  }
  const argv = [
    cli, 'conway', 'transaction', 'calculate-plutus-script-cost', 'online',
    '--socket-path', input.socket,
    '--testnet-magic', String(input.magic),
    '--tx-file', txFile,
  ];
  if (input.outFile) argv.push('--out-file', input.outFile);
  return {
    ok: true,
    code: 'PLUTUS_COST_ARGV',
    mode: 'online',
    argv,
    command: 'conway transaction calculate-plutus-script-cost online',
    measured: false,
    nativeBudgetMeasured: false,
    spawned: false,
  };
}

export function minRequiredUtxoArgv(input = {}) {
  if (typeof input.cli !== 'string' || typeof input.protocolParamsFile !== 'string' || typeof input.txOut !== 'string') {
    return closed('MIN_ADA_INPUT_ABSENT', { command: 'conway transaction calculate-min-required-utxo' });
  }
  if (String(input.txOut).includes('"')) return closed('QUOTED_TX_OUT');
  const argv = [
    input.cli, 'conway', 'transaction', 'calculate-min-required-utxo',
    '--protocol-params-file', input.protocolParamsFile,
    '--tx-out', input.txOut,
  ];
  if (input.outputJson === true) {
    return closed('MIN_REQUIRED_UTXO_HAS_NO_OUTPUT_JSON', { command: 'conway transaction calculate-min-required-utxo' });
  }
  if (input.inlineDatumFile) argv.push('--tx-out-inline-datum-file', input.inlineDatumFile);
  if (input.referenceScriptFile) argv.push('--tx-out-reference-script-file', input.referenceScriptFile);
  return {
    ok: true,
    code: 'MIN_REQUIRED_UTXO_ARGV',
    argv,
    measured: false,
    nativeBudgetMeasured: false,
    budgetPin: false,
    spawned: false,
  };
}

const COIN_SHOW = /^Coin\s+(\d+)$/;

function word64Text(value) {
  if (typeof value === 'bigint') return readWord64Coin(value.toString(), undefined);
  if (typeof value === 'number') return readWord64Coin(undefined, value);
  if (typeof value === 'string') return readWord64Coin(value, undefined);
  return { ok: false, reason: 'noncanonical' };
}

export function txOutValue(output = {}) {
  const address = output.address;
  const lovelace = word64Text(output.lovelace);
  if (typeof address !== 'string' || address.length === 0 || /[\s"]/.test(address)) {
    return closed('TX_OUT_ADDRESS_REFUSED', { budgetPin: false });
  }
  if (!lovelace.ok) return closed('TX_OUT_LOVELACE_REFUSED', { budgetPin: false, reason: lovelace.reason });
  const assets = Array.isArray(output.assets) ? output.assets : [];
  const parts = [`${address}+${lovelace.digits}`];
  for (const asset of assets) {
    const policy = asset?.policyId;
    const name = asset?.assetNameHex;
    const qty = word64Text(asset?.quantity);
    if (!qty.ok) return closed('TX_OUT_ASSET_REFUSED', { reason: qty.reason || 'quantity', budgetPin: false });
    if (typeof policy !== 'string' || !/^[0-9a-f]{56}$/.test(policy)) {
      return closed('TX_OUT_ASSET_REFUSED', { reason: 'policy-not-56-hex', budgetPin: false });
    }
    if (typeof name !== 'string' || !/^[0-9a-f]+$/.test(name) || name.length % 2 !== 0) {
      return closed('TX_OUT_ASSET_REFUSED', { reason: 'asset-name-not-lowercase-hex', budgetPin: false });
    }
    parts.push(`${qty.digits} ${policy}.${name}`);
  }
  const txOut = parts.join('+');
  if (txOut.includes('"') || txOut.includes('policy ') || txOut.includes('asset ')) {
    return closed('TX_OUT_DISPLAY_WORD_REFUSED', { budgetPin: false });
  }
  return { ok: true, code: 'TX_OUT_VALUE', txOut, quoted: false, nativeBudgetMeasured: false, budgetPin: false, ledgerExecution: false };
}

export function parseMinRequiredUtxo(stdout) {
  const base = {
    command: 'conway transaction calculate-min-required-utxo',
    showFormat: '^Coin\\s+(\\d+)$',
    showFormatCaptured: true,
    minAdaMeasured: false,
    protocolMinAda: 'unmeasured',
    nativeBudgetMeasured: false,
    budgetPin: false,
    cost72ValuesAreBudgetPins: false,
    ledgerExecution: false,
    evidenceClass: 'stdout-parse-not-a-query',
  };
  if (typeof stdout !== 'string') return closed('MIN_REQUIRED_UTXO_SHOW_MISMATCH', { ...base, reason: 'stdout-absent' });
  const text = stdout.endsWith('\n') ? stdout.slice(0, -1) : stdout;
  const match = COIN_SHOW.exec(text);
  if (!match) return closed('MIN_REQUIRED_UTXO_SHOW_MISMATCH', { ...base, reason: 'not-coin-digits' });
  const coin = readWord64Coin(match[1], undefined);
  if (!coin.ok) return closed('MIN_REQUIRED_UTXO_SHOW_MISMATCH', { ...base, reason: coin.reason });
  return {
    ok: true,
    code: 'MIN_REQUIRED_UTXO_PARSED',
    ...base,
    lovelace: coin.value,
    lovelaceText: coin.digits,
    measured: false,
  };
}

export function adoptSpawnedMinAda(parsed, evidence = {}) {
  const armed = process.env.MORIARTY_ALLOW_REAL_NODE === '1'
    && evidence.unitOnly === false
    && evidence.spawnedCli === true
    && evidence.exit === 0
    && parsed?.ok === true;
  if (!armed) {
    return {
      ...(parsed || closed('MIN_REQUIRED_UTXO_SHOW_MISMATCH')),
      minAdaMeasured: false,
      protocolMinAda: 'unmeasured',
      nativeBudgetMeasured: false,
      budgetPin: false,
      cost72ValuesAreBudgetPins: false,
    };
  }
  return {
    ...parsed,
    minAdaMeasured: true,
    protocolMinAda: String(parsed.lovelaceText ?? parsed.lovelace),
    nativeBudgetMeasured: false,
    budgetPin: false,
    cost72ValuesAreBudgetPins: false,
    evidenceClass: 'spawned-cli',
    measured: true,
    ledgerExecution: false,
  };
}

export function minFeeArgv(input = {}) {
  const command = 'conway transaction calculate-min-fee';
  if (typeof input.cli !== 'string' || typeof input.txBodyFile !== 'string' || typeof input.protocolParamsFile !== 'string') {
    return closed('MIN_FEE_INPUT_ABSENT', { command, budgetPin: false });
  }
  if (!Number.isInteger(input.witnessCount) || input.witnessCount < 1) {
    return closed('WITNESS_COUNT_ABSENT', { command, reason: 'do-not-assume-1-or-2', budgetPin: false });
  }
  const argv = [
    input.cli, 'conway', 'transaction', 'calculate-min-fee',
    '--tx-body-file', input.txBodyFile,
    '--protocol-params-file', input.protocolParamsFile,
    '--witness-count', String(input.witnessCount),
  ];
  return {
    ok: true,
    code: 'MIN_FEE_ARGV',
    argv,
    command,
    feeAppliesToSignedTx: false,
    signedWitnessSetRequested: input.witnessCountProvenance === 'signed-witness-set',
    witnessCountProvenance: input.witnessCountProvenance || 'not-a-signed-witness-set',
    measured: false,
    nativeBudgetMeasured: false,
    budgetPin: false,
    spawned: false,
  };
}

export function planOutputMinimums(input = {}) {
  const steps = [];
  let blocker = null;
  if (input.outputs != null) {
    if (!Array.isArray(input.outputs) || input.outputs.length === 0) blocker = 'OUTPUTS_ABSENT';
    else if (typeof input.protocolParamsFile !== 'string' || typeof input.cli !== 'string') blocker = 'MIN_ADA_INPUT_ABSENT';
    else {
      input.outputs.forEach((output, index) => {
        const value = txOutValue(output);
        const argv = value.ok ? minRequiredUtxoArgv({
          cli: input.cli,
          protocolParamsFile: input.protocolParamsFile,
          txOut: value.txOut,
          inlineDatumFile: output.inlineDatumFile,
          referenceScriptFile: output.referenceScriptFile,
        }) : value;
        if (!argv.ok) blocker = blocker || argv.code;
        steps.push({
          id: 'calculate-min-required-utxo',
          argv: argv.argv || null,
          code: argv.code,
          outputIndex: index,
          budgetPin: false,
        });
      });
    }
  }
  if (input.feeRequest != null) {
    const fee = minFeeArgv({
      cli: input.cli,
      protocolParamsFile: input.protocolParamsFile,
      ...input.feeRequest,
    });
    if (!fee.ok) blocker = blocker || fee.code;
    steps.push({
      id: 'calculate-min-fee',
      argv: fee.argv || null,
      code: fee.code,
      signedWitnessSetRequested: fee.signedWitnessSetRequested === true,
      feeAppliesToSignedTx: false,
      budgetPin: false,
    });
  }
  return { steps, blocker };
}

export function consumePlutusCost(value, { unitOnly = true } = {}) {
  const parsed = parsePlutusScriptCost(value);
  if (!parsed.ok) return { ...closed(parsed.code, { detail: parsed.detail }), unitOnly };
  const supplied = parsed.items.length === 0;
  return {
    ok: !supplied,
    code: supplied ? 'EVALUATION_UNAVAILABLE' : 'PLUTUS_COST_PARSED',
    items: parsed.items,
    cost: parsed.items[0]?.executionUnits || null,
    lovelaceCost: parsed.items[0]?.lovelaceCost ?? null,
    measured: false,
    executionSource: null,
    nativeBudgetMeasured: false,
    unitOnly: unitOnly !== false,
    submitBlocked: true,
    ledgerExecution: false,
    nativePins: false,
    spawned: false,
  };
}

export function consumeMinFeeOutput(value, cborHex = '') {
  const parsed = parseMinFee(value);
  if (!parsed.ok) return closed(parsed.code, { detail: parsed.detail });
  const txBytesFromCbor = typeof cborHex === 'string' && /^[0-9a-fA-F]+$/.test(cborHex) && cborHex.length % 2 === 0
    ? cborHex.length / 2
    : null;
  return {
    ...parsed,
    ok: true,
    txBytesFromCbor,
    txBytesFromCommand: null,
    measured: false,
    nativeBudgetMeasured: false,
    ledgerExecution: false,
  };
}

export function enforceExecutionUnits(cost, protocol, txBytes = null) {
  const parsed = parseProtocolParams(protocol);
  if (!parsed.ok) return closed(parsed.code, { detail: parsed.detail, submitBlocked: true });
  if (!cost || !Number.isInteger(cost.memory) || !Number.isInteger(cost.steps)) {
    return closed('EXECUTION_UNITS_REQUIRED', { submitBlocked: true });
  }
  const over = cost.memory > parsed.maxTxExecutionUnits.memory || cost.steps > parsed.maxTxExecutionUnits.steps;
  const sizeOver = txBytes != null && Number(txBytes) > parsed.maxTxSize;
  if (over || sizeOver) {
    return closed('BUDGET_EXCEEDED', { submitBlocked: true, minAdaMeasured: false, protocolMinAda: 'unmeasured' });
  }
  return {
    ok: true,
    code: 'BUDGET_WITHIN_PROTOCOL_MAX',
    submitBlocked: true,
    nativeBudgetMeasured: false,
    minAdaMeasured: false,
    protocolMinAda: 'unmeasured',
    minAdaCommand: 'conway transaction calculate-min-required-utxo',
    ledgerExecution: false,
    nativePins: false,
    measured: false,
  };
}

export function observeOwnedClock({ tip, era, genesis, genesisPath = '', unitOnly = true } = {}) {
  let genesisBody = genesis || null;
  if (!genesisBody && genesisPath && existsSync(genesisPath)) {
    try {
      genesisBody = readJson(genesisPath);
    } catch {
      genesisBody = null;
    }
  }
  const tipParsed = parseTip(tip);
  if (!tipParsed.ok) return closed(tipParsed.code, { detail: tipParsed.detail, unitOnly, siblingEmitted: false });
  if (tipParsed.kind === 'ChainTipAtGenesis' || tipParsed.slot == null) {
    return closed('GENESIS_TIP_HAS_NO_SLOT', { slot: null, defaultedSlot: false, unitOnly, siblingEmitted: false });
  }
  const envelope = parseEraHistoryEnvelope(era);
  if (!envelope.ok) return closed(envelope.code, { detail: envelope.detail, unitOnly, siblingEmitted: false });
  if (!genesisBody || typeof genesisBody !== 'object') {
    return closed('PARITY_NOT_MEASURED', { reason: 'genesis-slot-absent', unitOnly, siblingEmitted: false });
  }
  const slot = slotLengthMsFromShelley(genesisBody.slotLength);
  if (!slot.ok) return closed(slot.code, { detail: slot.detail, unitOnly, siblingEmitted: false });
  const epoch = genesisBody.epochLength ?? genesisBody.epochLengthSlots;
  const windows = planWindows({
    systemStart: genesisBody.systemStart,
    tipSlot: tipParsed.slot,
    slotLengthMs: slot.slotLengthMs,
    epochLengthSlots: epoch,
  });
  if (!windows.ok) return closed(windows.code, { detail: windows.detail, unitOnly, siblingEmitted: false });
  const zeroTime = canonicalZeroTime(genesisBody);
  const zeroSlot = canonicalZeroSlot(genesisBody);
  if (!zeroTime || !zeroSlot) return closed('PARITY_NOT_MEASURED', { reason: 'zero-time-absent', unitOnly, siblingEmitted: false });
  if (String(slot.slotLengthMs) !== '1000') return closed('SLOT_LENGTH_REFUSED', { observed: String(slot.slotLengthMs), unitOnly, siblingEmitted: false });
  const custodyWindows = custodyPlanWindows({
    zeroTime,
    zeroSlot,
    slotLengthMs: '1000',
    tipSlot: String(tipParsed.slot),
  });
  if (!custodyWindows.ok) return closed(custodyWindows.code, { detail: custodyWindows.detail, unitOnly, siblingEmitted: false });
  return {
    ok: true,
    code: 'SLOT_OBSERVED',
    observed: String(slot.slotLengthMs),
    slotLengthSource: 'owned-genesis',
    eraHistory: 'json',
    eraHistoryWire: envelope.wire,
    cborHex: envelope.cborHex,
    innerCborFieldNames: null,
    gap: envelope.gap,
    genesisPath: genesisPath || null,
    unitOnly: unitOnly !== false,
    parityExecuted: false,
    measured: false,
    tipSlot: String(tipParsed.slot),
    tipKind: tipParsed.kind,
    systemStart: genesisBody.systemStart,
    systemStartEqualsCreateLiteral: genesisBody.systemStart === CREATE_TIME_SYSTEM_START,
    byronClockIsWitnessClock: false,
    windows,
    custodyWindows,
    zeroTime,
    zeroSlot,
    measuredBounds: {
      releaseLower: custodyWindows.releaseStart,
      partition: custodyWindows.partition,
      refundInclusive: custodyWindows.refundArgs.upper,
    },
    probeExecuted: false,
    evidenceClass: null,
    siblingEmitted: false,
    magicMeasured: false,
    ledgerExecution: false,
    nativePins: false,
    spawned: false,
  };
}

function canonicalZeroTime(genesis) {
  if (typeof genesis.zeroTime === 'string' && /^(?:0|[1-9][0-9]*)$/.test(genesis.zeroTime)) return genesis.zeroTime;
  if (typeof genesis.systemStart !== 'string') return null;
  const ms = Date.parse(genesis.systemStart);
  if (!Number.isFinite(ms) || ms < 0) return null;
  return String(ms);
}

function canonicalZeroSlot(genesis) {
  if (genesis.zeroSlot == null) return '0';
  const text = typeof genesis.zeroSlot === 'number' && Number.isSafeInteger(genesis.zeroSlot)
    ? String(genesis.zeroSlot)
    : String(genesis.zeroSlot);
  if (!/^(?:0|[1-9][0-9]*)$/.test(text)) return null;
  return text;
}

export function buildOwnedProducerPlan(input = {}) {
  const plan = buildBootstrapPlan(input, {
    ownedGenesisArgv,
    plutusCostArgv,
    planOutputMinimums,
    pinnedTemplates: PINNED_GENESIS_TEMPLATES,
  });
  return { ...plan, financial: financialContinuation(plan) };
}

function consumeStep(step, output, { unitOnly = true } = {}) {
  const status = output?.status ?? 0;
  const text = output?.stdout ?? '';
  if (status !== 0) return closed('PRODUCER_COMMAND_FAILED', { id: step.id, cliStatus: status, unitOnly: true });
  if (step.id === 'query-tip') {
    let parsed;
    try { parsed = JSON.parse(text); } catch { return closed('TIP_OBJECT_REQUIRED', { id: step.id, unitOnly: true }); }
    const tip = parseTip(parsed);
    if (!tip.ok) return { ...closed(tip.code, { id: step.id, unitOnly: true }), detail: tip.detail };
    if (tip.slot == null) return closed('GENESIS_TIP_HAS_NO_SLOT', { id: step.id, slot: null, defaultedSlot: false, unitOnly: true });
    return { ok: true, code: 'TIP_PARSED', id: step.id, tip, unitOnly: true, measured: false, parityExecuted: false, ledgerExecution: false };
  }
  if (step.id === 'query-era-history') {
    let parsed;
    try { parsed = JSON.parse(text); } catch { return closed('ERA_HISTORY_WIRE_IS_TEXT_ENVELOPE', { id: step.id, unitOnly: true }); }
    const era = parseEraHistoryEnvelope(parsed);
    if (!era.ok) return { ...closed(era.code, { id: step.id, unitOnly: true }), detail: era.detail };
    return { ok: true, code: 'ERA_HISTORY_PARSED', id: step.id, era, unitOnly: true, measured: false, parityExecuted: false, ledgerExecution: false };
  }
  if (step.id === 'calculate-plutus-script-cost') {
    let parsed = text;
    try { parsed = JSON.parse(text); } catch { /* parser rejects a non-array */ }
    const cost = consumePlutusCost(parsed, { unitOnly });
    const rebuild = planPlutusCostRebuild(cost, { unitOnly });
    return { ...cost, id: step.id, rebuild };
  }
  if (step.id === 'query-funder-utxo' || step.id.startsWith('funding-confirm-') || step.id.startsWith('fin-')) {
    const parsed = parseQueryUtxoText(text);
    return {
      ...parsed,
      id: step.id,
      unitOnly: unitOnly !== false,
      nodeUtxoQueried: unitOnly === false && parsed.ok === true,
      ledgerFunding: false,
      fundedInNodeInitialFunds: false,
      evidenceClass: unitOnly !== false ? 'unit-fixture' : 'spawned-query',
    };
  }
  if (step.id === 'calculate-min-required-utxo') {
    const parsed = adoptSpawnedMinAda(parseMinRequiredUtxo(text), {
      unitOnly,
      spawnedCli: unitOnly === false,
      exit: status,
    });
    return { ...parsed, id: step.id, unitOnly };
  }
  if (step.id === 'calculate-min-fee') {
    let parsed = text;
    if (typeof text === 'string' && text.includes('"fee"')) {
      try {
        const token = parseFeeStdout(text);
        parsed = { fee: Number(token) };
      } catch (error) {
        return closed(error?.code || 'FAKE_FEE', {
          id: step.id,
          unitOnly,
          nativeBudgetMeasured: false,
          budgetPin: false,
        });
      }
    } else {
      try { parsed = JSON.parse(text); } catch { /* text form is digits Lovelace */ }
    }
    const fee = consumeMinFeeOutput(parsed, output?.cborHex || '');
    const provenance = classifyWitnessCountProvenance(step.witnessCountProvenance);
    const signed = unitOnly === false && step.signedWitnessSetRequested === true && status === 0 && fee.ok === true;
    return {
      ...fee,
      id: step.id,
      unitOnly,
      budgetPin: false,
      feeAppliesToSignedTx: signed,
      nativeBudgetMeasured: false,
      measured: false,
      witnessCountProvenance: provenance,
    };
  }
  return {
    ok: true,
    code: 'PRODUCER_COMMAND_ACCEPTED',
    id: step.id,
    unitOnly: true,
    spawned: true,
    ledgerExecution: false,
    nativePins: false,
    nativeBudgetMeasured: false,
    ledgerFunding: false,
    measured: false,
  };
}

export function dispatchOwnedProducer(plan, deps = {}) {
  if (!plan || !Array.isArray(plan.sequence)) return closed('PRODUCER_SEQUENCE_ABSENT');
  const unitOnly = deps.unitOnly === true;
  if (!unitOnly && process.env.MORIARTY_ALLOW_REAL_NODE !== '1') {
    return closed('REAL_NODE_NOT_ARMED', { inputsReady: plan.inputsReady === true, sequence: plan.sequence });
  }
  if (plan.inputsReady !== true) return closed(plan.code || 'PRODUCER_INPUTS_ABSENT', { blockers: plan.blockers || [], sequence: plan.sequence });
  const needsSpawn = plan.sequence.some((step) => step.run === true && step.local == null);
  if (needsSpawn && typeof deps.spawn !== 'function') return closed('PRODUCER_SPAWN_ABSENT');
  const results = [];
  const bag = {};
  let socketReady = false;
  const held = { ...plan };
  for (const step of plan.sequence) {
    if (step.run !== true) {
      results.push(closed(step.code || 'PRODUCER_STEP_NOT_RUN', { id: step.id, evidenceClass: unitOnly ? 'unit-fixture' : null }));
      break;
    }
    if (QUERY_AFTER_SOCKET.has(step.id) && !socketReady) {
      results.push(closed('NODE_NOT_STARTED', { id: step.id, reason: 'query-before-socket-ready', evidenceClass: unitOnly ? 'unit-fixture' : null }));
      break;
    }
    let result;
    if (step.local) {
      if (step.local === 'funding-conservation') {
        const funderUtxo = bag['query-funder-utxo'];
        if (typeof funderUtxo?.lovelaceText === 'string') held._utxoLovelaceText = funderUtxo.lovelaceText;
        if (funderUtxo?.lovelace != null) held._utxoLovelace = funderUtxo.lovelace;
        if (bag['calculate-min-fee']?.fee != null) held._feeLovelace = bag['calculate-min-fee'].fee;
        if (typeof bag['calculate-min-fee']?.feeText === 'string') held._feeLovelaceText = bag['calculate-min-fee'].feeText;
      }
      result = { ...runLocalBootstrap(step, held, { unitOnly }), id: step.id, unitOnly };
      if (result.byronReplaced === true) held.byronReplaced = true;
    } else {
      const argv = resolveBootstrapArgv(step, held, bag);
      if (!Array.isArray(argv)) {
        const heldArgv = { ...argv, id: step.id, spawned: false, evidenceClass: unitOnly ? 'unit-fixture' : null };
        if (step.hold === true && argv && argv.ok === true) {
          results.push(heldArgv);
          continue;
        }
        results.push(heldArgv);
        break;
      }
      if (argv.includes('evaluate')) {
        results.push(closed('NO_CLI_EVALUATE_COMMAND', { id: step.id }));
        break;
      }
      const output = deps.spawn(argv, { ...step, argv });
      result = {
        ...consumeStep(step, output, { unitOnly }),
        spawned: true,
        unitOnly,
        evidenceClass: unitOnly ? 'unit-fixture' : 'spawned-cli',
      };
      bag[step.id] = { ...result, stdout: output?.stdout ?? '', argv };
    }
    if (step.id === 'S7-node-run') result = { ...result, nodeStarted: false, pidWritten: false };
    if (result.code === 'SOCKET_READY') socketReady = true;
    results.push(result);
    if (result.ok === false) break;
  }
  const failed = results.find((item) => item.ok === false);
  return {
    ok: !failed,
    code: failed ? failed.code : 'PRODUCER_DISPATCHED',
    evidenceClass: unitOnly ? 'unit-fixture' : 'latched-spawn',
    unitOnly,
    spawned: results.some((item) => item.spawned === true),
    ledgerExecution: false,
    nativePins: false,
    nativeBudgetMeasured: false,
    ledgerFunding: false,
    fundedInNodeInitialFunds: false,
    bootstrapComplete: false,
    runtimePinsCreated: false,
    nodeStarted: false,
    confirmed: false,
    measured: false,
    parityExecuted: false,
    selfApproval: false,
    nodeUtxoQueried: results.some((item) => item.nodeUtxoQueried === true),
    results,
    financial: bindFinancialFromDispatch({ results, unitOnly }, plan),
  };
}

export function realProducerSpawn(argv) {
  const [bin, ...args] = argv;
  return spawnSync(bin, args, { encoding: 'utf8' });
}

export function selectOwnedFunder(input = {}) {
  const roles = input.roles || {};
  const names = ['lender', 'borrower', 'issuer'];
  const derived = {};
  for (const name of names) {
    const role = roles[name];
    if (!role || typeof role.vkeyHex !== 'string') return closed('ROLE_KEYS_ABSENT', { role: name });
    const vkeyHex = role.vkeyHex.trim().toLowerCase();
    if (vkeyHex === ENCODING_VKEY_HEX) return closed('ENCODING_FIXTURE_NOT_A_ROLE', { role: name });
    const hash = paymentKeyHashOfVkeyCbor(vkeyHex);
    if (!hash) return closed('ROLE_KEY_UNREADABLE', { role: name });
    derived[name] = hash;
  }
  if (derived.lender === derived.borrower || derived.lender === derived.issuer || derived.borrower === derived.issuer) {
    return closed('PAYOUT_KEY_COLLAPSED');
  }
  const utxo = input.utxo;
  const lovelaceText = typeof utxo?.lovelaceText === 'string'
    ? utxo.lovelaceText
    : (Number.isSafeInteger(utxo?.lovelace) ? String(utxo.lovelace) : '');
  if (typeof utxo?.lovelace === 'number' && !Number.isSafeInteger(utxo.lovelace) && typeof utxo?.lovelaceText !== 'string') {
    return closed('LOSSY_LOVELACE', { derived });
  }
  if (!utxo || !/^[1-9][0-9]*$/.test(lovelaceText)) {
    return closed('EMPTY_FUNDER_NOT_A_UTXO', { derived });
  }
  if (utxo.lovelace != null && String(utxo.lovelace) !== lovelaceText) {
    return closed('LOSSY_LOVELACE', { derived });
  }
  if (utxo.ownerHash !== derived.lender) return closed('FUNDER_OWNER_MISMATCH', { derived, ownerHash: utxo.ownerHash || null });
  if (typeof utxo.address !== 'string' || utxo.address.length === 0) return closed('ADDRESS_NOT_DERIVED', { derived });
  if (!/^[0-9a-f]{64}#\d+$/.test(`${utxo.txId}#${utxo.index}`)) return closed('NO_TX_IN', { derived });
  return {
    ok: true,
    code: 'FUNDER_SHAPE_READY',
    bootstrapComplete: false,
    fundedInNodeInitialFunds: false,
    genesisPotIsNotNodeInitialFunds: true,
    unitOnly: input.unitOnly !== false,
    ledgerFunding: false,
    nodeUtxoQueried: input.queryProvenance === 'spawned-query' && input.unitOnly === false,
    measured: false,
    nativePins: false,
    ledgerExecution: false,
    encodingFixture: false,
    lender: derived.lender,
    borrower: derived.borrower,
    issuer: derived.issuer,
    requiredSigners: [derived.lender, derived.borrower, derived.issuer],
    sourceGenesis: input.sourceGenesis || null,
    utxo: {
      txId: utxo.txId,
      index: utxo.index,
      lovelace: lovelaceText.length < 16 ? Number(lovelaceText) : null,
      lovelaceText,
      address: utxo.address,
    },
  };
}
