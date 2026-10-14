/**
 * Owned bootstrap for a later reviewed and armed caller.
 * This module builds the command sequence and the local file checks.
 * It does not set MORIARTY_ALLOW_REAL_NODE and it does not spawn a node.
 * Unit dispatch marks evidenceClass unit-fixture. Ledger flags stay false.
 */
import { createHash } from 'node:crypto';
import { copyFileSync, existsSync, mkdirSync, readFileSync, statSync, writeFileSync } from 'node:fs';
import { isAbsolute, join } from 'node:path';
import { ENCODING_VKEY_HEX, paymentKeyHashOfVkeyCbor } from './native-body.mjs';
import {
  bindSignedWitnessCount,
  classifyClockStamp,
  TEMPLATE_SYSTEM_START,
  YACI_CLI,
} from './native-integrate.mjs';
import {
  createNodeArgvShape,
  freshFaucetKeys,
  portMapping,
  yaciLayout,
} from '/home/charl/moriarty-eutxo-validation-2026-10-12/t1-runtime-schemas-71/own71/schemas.mjs';

const CREATE_TIME_SYSTEM_START = '2022-09-15T04:09:11.577484Z';
export { TEMPLATE_SYSTEM_START, YACI_CLI };

export const QUERY_AFTER_SOCKET = new Set([
  'query-tip',
  'query-era-history',
  'query-protocol',
  'query-funder-utxo',
  'calculate-plutus-script-cost',
  'funding-build',
  'funding-sign',
  'funding-submit',
  'funding-confirm-issuer',
  'funding-confirm-lender',
  'funding-confirm-borrower',
  'role-address-issuer',
  'role-address-lender',
  'role-address-borrower',
]);

const ROLE_NAMES = ['issuer', 'lender', 'borrower'];

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
    fundedInNodeInitialFunds: false,
    bootstrapComplete: false,
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

function stable(value) {
  if (Array.isArray(value)) return value.map(stable);
  if (value && typeof value === 'object') {
    const out = {};
    for (const key of Object.keys(value).sort()) out[key] = stable(value[key]);
    return out;
  }
  return value;
}

function sameJson(left, right) {
  return JSON.stringify(stable(left)) === JSON.stringify(stable(right));
}

function vkeyHexFromText(text) {
  try {
    const parsed = JSON.parse(text);
    if (typeof parsed?.cborHex === 'string') return parsed.cborHex.trim().toLowerCase();
  } catch {
    /* raw hex is also accepted */
  }
  const match = String(text || '').match(/5820[0-9a-fA-F]{64}/);
  return match ? match[0].toLowerCase() : '';
}

export function repeatedKeyMaterial(vkeyHex) {
  const body = String(vkeyHex || '').replace(/^5820/, '').toLowerCase();
  if (!/^[0-9a-f]{64}$/.test(body)) return true;
  const banned = new Set(['ab', 'c3', 'd4', 'f5', 'cd']);
  if (banned.has(body.slice(0, 2)) && body === body.slice(0, 2).repeat(32)) return true;
  return body === body[0].repeat(64);
}

function freshVerificationHex(text) {
  const hex = vkeyHexFromText(text);
  if (hex === ENCODING_VKEY_HEX) return { ok: false, code: 'ENCODING_FIXTURE_NOT_A_ROLE' };
  if (repeatedKeyMaterial(hex)) return { ok: false, code: 'ENCODING_FIXTURE_NOT_A_ROLE' };
  const hash = paymentKeyHashOfVkeyCbor(hex);
  if (!hash) return { ok: false, code: 'FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS' };
  return { ok: true, hex, hash };
}

function genesisPaths(home, layout) {
  const genesisDir = layout.genesisKeysFolder;
  return {
    home,
    genesisDir,
    shelleyWork: join(genesisDir, 'shelley-genesis.json'),
    byronWork: join(genesisDir, 'byron-genesis.json'),
    alonzoWork: join(genesisDir, 'alonzo-genesis.json'),
    conwayWork: join(genesisDir, 'conway-genesis.json'),
    utxoVkeys: [0, 1, 2].map((index) => join(genesisDir, 'utxo-keys', `shelley.00${index}.vkey`)),
    utxoSkeys: [0, 1, 2].map((index) => join(genesisDir, 'utxo-keys', `shelley.00${index}.skey`)),
    addrOuts: [0, 1, 2].map((index) => join(genesisDir, `addr-shelley.00${index}`)),
    clusterShelley: layout.shelleyGenesis,
    clusterByron: layout.byronGenesis,
    clusterAlonzo: layout.alonzoGenesis,
    clusterConway: layout.conwayGenesis,
    socketPath: layout.socketPath,
    configuration: layout.configuration,
    topology: join(layout.nodeDir, 'topology.json'),
    database: join(layout.nodeDir, 'db'),
    utxoDir: layout.utxoKeysDir,
    nodeDir: layout.nodeDir,
    roles: ROLE_NAMES.map((name) => ({
      name,
      vkey: join(home, 'roles', `${name}.vkey`),
      skey: join(home, 'roles', `${name}.skey`),
      addr: join(home, 'roles', `${name}.addr`),
    })),
    fundingBody: join(layout.clusterFolder, 'funding', 'body.json'),
    fundingSigned: join(layout.clusterFolder, 'funding', 'signed.json'),
    protocolOut: join(layout.nodeDir, 'protocol.json'),
  };
}

function templateFeeRefused(path) {
  try {
    const body = readJson(path);
    const minFeeA = body.minFeeA ?? body.protocolParams?.minFeeA;
    const minFeeB = body.minFeeB ?? body.protocolParams?.minFeeB;
    return minFeeA === 0 || minFeeB === 0;
  } catch {
    return false;
  }
}

function step(id, argv, code, inputsReady, extra = {}) {
  return { id, argv: argv || null, code, run: inputsReady === true, ...extra };
}

export function parseJsonExact(text) {
  const src = String(text);
  let index = 0;
  const fail = (code) => {
    const error = new Error(code);
    error.code = code;
    throw error;
  };
  const skip = () => {
    while (index < src.length && ' \n\r\t'.includes(src[index])) index += 1;
  };
  const parseString = () => {
    index += 1;
    let out = '';
    while (index < src.length) {
      const char = src[index];
      index += 1;
      if (char === '"') return out;
      if (char !== '\\') {
        out += char;
        continue;
      }
      const escaped = src[index];
      index += 1;
      if (escaped === 'u') {
        const hex = src.slice(index, index + 4);
        if (!/^[0-9a-fA-F]{4}$/.test(hex)) fail('UTXO_JSON_STRING');
        out += String.fromCharCode(Number.parseInt(hex, 16));
        index += 4;
      } else if (escaped === '"' || escaped === '\\' || escaped === '/') out += escaped;
      else if (escaped === 'b') out += '\b';
      else if (escaped === 'f') out += '\f';
      else if (escaped === 'n') out += '\n';
      else if (escaped === 'r') out += '\r';
      else if (escaped === 't') out += '\t';
      else fail('UTXO_JSON_STRING');
    }
    fail('UTXO_JSON_STRING');
    return out;
  };
  const parseNumber = () => {
    const start = index;
    if (src[index] === '-') index += 1;
    if (src[index] === '0') index += 1;
    else if (src[index] >= '1' && src[index] <= '9') {
      while (src[index] >= '0' && src[index] <= '9') index += 1;
    } else fail('UTXO_JSON_NUMBER');
    if (src[index] === '.' || src[index] === 'e' || src[index] === 'E') fail('UTXO_JSON_NON_INTEGER');
    return src.slice(start, index);
  };
  const parseValue = () => {
    skip();
    const char = src[index];
    if (char === '{') {
      index += 1;
      const object = {};
      skip();
      if (src[index] === '}') {
        index += 1;
        return object;
      }
      while (index < src.length) {
        skip();
        if (src[index] !== '"') fail('UTXO_JSON_OBJECT');
        const key = parseString();
        skip();
        if (src[index] !== ':') fail('UTXO_JSON_OBJECT');
        index += 1;
        object[key] = parseValue();
        skip();
        if (src[index] === ',') {
          index += 1;
          continue;
        }
        if (src[index] === '}') {
          index += 1;
          return object;
        }
        fail('UTXO_JSON_OBJECT');
      }
      fail('UTXO_JSON_OBJECT');
    }
    if (char === '[') {
      index += 1;
      const list = [];
      skip();
      if (src[index] === ']') {
        index += 1;
        return list;
      }
      while (index < src.length) {
        list.push(parseValue());
        skip();
        if (src[index] === ',') {
          index += 1;
          continue;
        }
        if (src[index] === ']') {
          index += 1;
          return list;
        }
        fail('UTXO_JSON_ARRAY');
      }
      fail('UTXO_JSON_ARRAY');
    }
    if (char === '"') return parseString();
    if (src.startsWith('true', index)) {
      index += 4;
      return true;
    }
    if (src.startsWith('false', index)) {
      index += 5;
      return false;
    }
    if (src.startsWith('null', index)) {
      index += 4;
      return null;
    }
    if (char === '-' || (char >= '0' && char <= '9')) return parseNumber();
    fail('UTXO_JSON_TOKEN');
    return null;
  };
  const value = parseValue();
  skip();
  if (index !== src.length) fail('UTXO_JSON_TRAILING');
  return value;
}

function oneUtxoEntry(parsed) {
  const entries = Object.entries(parsed).filter(([ref]) => /^[0-9a-f]{64}#\d+$/.test(ref));
  if (entries.length !== Object.keys(parsed).length) return { ok: false, code: 'UTXO_JSON_KEY' };
  if (entries.length === 0) return { ok: false, code: 'FUNDER_UTXO_ABSENT' };
  if (entries.length !== 1) return { ok: false, code: 'UTXO_SELECTION_REQUIRED', txIns: entries.map(([ref]) => ref) };
  return { ok: true, ref: entries[0][0], body: entries[0][1] };
}

export function parseQueryUtxoText(stdout) {
  const base = {
    nodeUtxoQueried: false,
    ledgerFunding: false,
    fundedInNodeInitialFunds: false,
    nativeBudgetMeasured: false,
    measured: false,
  };
  if (typeof stdout !== 'string' || stdout.trim().length === 0) {
    return closed('FUNDER_UTXO_ABSENT', base);
  }
  const trimmed = stdout.trim();
  if (trimmed.startsWith('{') || trimmed.startsWith('[')) {
    let parsed;
    try {
      parsed = parseJsonExact(trimmed);
    } catch (error) {
      return closed(error.code || 'UTXO_JSON_TOKEN', base);
    }
    if (!parsed || typeof parsed !== 'object' || Array.isArray(parsed)) return closed('UTXO_JSON_NOT_AN_OBJECT', base);
    const selected = oneUtxoEntry(parsed);
    if (!selected.ok) return closed(selected.code, { ...base, txIns: selected.txIns || null });
    const body = selected.body;
    if (!body || typeof body !== 'object' || Array.isArray(body)) return closed('UTXO_JSON_OUTPUT', base);
    const lovelace = body.value && typeof body.value === 'object' ? body.value.lovelace : undefined;
    const digits = lovelace == null ? '0' : String(lovelace);
    if (!/^(?:0|[1-9][0-9]*)$/.test(digits)) return closed('FUNDER_UTXO_ABSENT', base);
    if (digits === '0') return closed('EMPTY_FUNDER_NOT_A_UTXO', { ...base, lovelaceText: '0' });
    const [txId, indexText] = selected.ref.split('#');
    const jsSafe = digits.length < 16 && Number.isSafeInteger(Number(digits));
    return {
      ok: true,
      code: 'FUNDER_UTXO_JSON_PARSED',
      txIn: selected.ref,
      txId,
      index: Number(indexText),
      lovelace: jsSafe ? Number(digits) : null,
      lovelaceText: digits,
      jsSafeInteger: jsSafe,
      address: typeof body.address === 'string' ? body.address : null,
      inlineDatum: body.inlineDatum ?? null,
      inlineDatumRaw: typeof body.inlineDatumRaw === 'string' ? body.inlineDatumRaw : null,
      datumIsNull: body.datum === null,
      ...base,
    };
  }
  const match = stdout.match(/^\s*([0-9a-f]{64})\s+(\d+)\s+([0-9]+)/im);
  if (!match) return closed('FUNDER_UTXO_ABSENT', base);
  const digits = match[3];
  if (!/^[1-9][0-9]*$/.test(digits)) return closed('EMPTY_FUNDER_NOT_A_UTXO', base);
  const jsSafe = digits.length < 16 && Number.isSafeInteger(Number(digits));
  return {
    ok: true,
    code: 'FUNDER_UTXO_TEXT_PARSED',
    txIn: `${match[1]}#${match[2]}`,
    txId: match[1],
    index: Number(match[2]),
    lovelace: jsSafe ? Number(digits) : null,
    lovelaceText: digits,
    jsSafeInteger: jsSafe,
    ...base,
  };
}

export function assessFundingConservation({ inputLovelace, feeLovelace, roleCount = 3 } = {}) {
  const flags = {
    fundedInNodeInitialFunds: false,
    ledgerFunding: false,
    nativeBudgetMeasured: false,
    feeAppliesToSignedTx: false,
    budgetPin: false,
    bootstrapComplete: false,
  };
  if (!Number.isInteger(inputLovelace) || inputLovelace <= 0) return closed('EMPTY_FUNDER_NOT_A_UTXO', flags);
  const each = Math.floor(inputLovelace / 10);
  const roleSum = each * roleCount;
  if (each < 1 || roleSum >= inputLovelace) {
    return closed('CONSERVATION_FAILED', { ...flags, reason: 'no-fee-room', each, roleSum });
  }
  if (!Number.isInteger(feeLovelace) || feeLovelace <= 0 || feeLovelace >= inputLovelace - roleSum) {
    return closed('CONSERVATION_FAILED', { ...flags, reason: 'fee-does-not-fit', each, roleSum, feeLovelace: feeLovelace ?? null });
  }
  return {
    ok: true,
    code: 'FUNDING_FEE_ROOM_OK',
    each,
    roleSum,
    feeLovelace,
    changeRoom: inputLovelace - roleSum - feeLovelace,
    ...flags,
    ledgerExecution: false,
    measured: false,
  };
}

export function renderPinnedNodeConfig(templatePath, destination) {
  const raw = readFileSync(templatePath, 'utf8');
  let text = raw;
  text = text.replace('{{#mainnet}}2{{/mainnet}}{{^mainnet}}2{{/mainnet}}', '2');
  text = text.replaceAll('{{^mainnet}}', '');
  text = text.replaceAll('{{/mainnet}}', '');
  text = text.replaceAll('{{#conway_era}}', '');
  text = text.replaceAll('{{/conway_era}}', '');
  text = text.replaceAll('{{enableP2P}}', 'false');
  text = text.replaceAll('{{peerSharing}}', 'false');
  text = text.replaceAll('{{conwayHardForkAtEpoch}}', '0');
  if (text.includes('{{') || text.includes('${')) {
    return closed('NODE_CONFIG_TEMPLATE_UNRENDERED');
  }
  let parsed;
  try {
    parsed = JSON.parse(text);
  } catch {
    return closed('NODE_CONFIG_TEMPLATE_UNRENDERED', { reason: 'json' });
  }
  if (parsed.ShelleyGenesisFile !== './genesis/shelley-genesis.json') {
    return closed('NODE_CONFIG_TEMPLATE_UNRENDERED', { reason: 'shelley-genesis-file' });
  }
  let dijkstraGenesisOmitted = false;
  if (Object.prototype.hasOwnProperty.call(parsed, 'DijkstraGenesisFile')) {
    delete parsed.DijkstraGenesisFile;
    dijkstraGenesisOmitted = true;
  }
  mkdirSync(join(destination, '..'), { recursive: true });
  writeFileSync(destination, `${JSON.stringify(parsed, null, 2)}\n`);
  return {
    ok: true,
    code: 'NODE_CONFIG_RENDERED',
    path: destination,
    ledgerExecution: false,
    spawned: false,
    dijkstraGenesisWritten: false,
    dijkstraGenesisOmitted,
    dijkstraOmitReason: dijkstraGenesisOmitted
      ? 'pinned-node-11-configuration-does-not-name-DijkstraGenesisFile'
      : null,
  };
}

export function renderPinnedTopology(templatePath, destination, port) {
  let text = readFileSync(templatePath, 'utf8');
  text = text.replaceAll('${adminNodeHost}', '127.0.0.1');
  text = text.replaceAll('${adminNodePort}', String(port));
  if (text.includes('${') || text.includes('{{')) return closed('NODE_CONFIG_TEMPLATE_UNRENDERED', { reason: 'topology' });
  try {
    JSON.parse(text);
  } catch {
    return closed('NODE_CONFIG_TEMPLATE_UNRENDERED', { reason: 'topology-json' });
  }
  mkdirSync(join(destination, '..'), { recursive: true });
  writeFileSync(destination, text.endsWith('\n') ? text : `${text}\n`);
  return { ok: true, code: 'TOPOLOGY_RENDERED', path: destination, ledgerExecution: false, spawned: false };
}

export function bindFreshInitialFunds(paths, { unitOnly = true, magic, supplyLovelace, templateShelley } = {}) {
  if (paths?.evidenceClass != null) return closed('EVIDENCE_CLASS_NOT_CALLER_SUPPLIED');
  const flags = { fundedInNodeInitialFunds: false, ledgerFunding: false, bootstrapComplete: false, evidenceClass: unitOnly ? 'unit-fixture' : 'generated-file' };
  let shelley;
  let byron;
  let template;
  try {
    shelley = readJson(paths.shelleyWork);
    byron = readJson(paths.byronWork);
    template = readJson(templateShelley);
  } catch {
    return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { ...flags, reason: 'generated-genesis-absent' });
  }
  if (!byron || typeof byron !== 'object' || Array.isArray(byron) || Object.keys(byron).length === 0) {
    return closed('BYRON_GENESIS_ABSENT', flags);
  }
  if (typeof shelley.slotLength !== 'number') return closed('SHELLEY_SLOT_LENGTH_IS_SECONDS_NUMBER', flags);
  if (shelley.slotLength !== 1) return closed('SHELLEY_SLOT_LENGTH_IS_SECONDS_NUMBER', flags);
  if (shelley.epochLength !== 3600) return closed('EPOCH_LENGTH_REQUIRED_3600', flags);
  if (shelley.networkMagic !== magic) return closed('MAGIC_SOURCE_REFUSED', flags);
  const clock = classifyClockStamp(shelley.systemStart);
  if (!clock.ok) return closed(clock.code, { ...flags, parityExecuted: false });
  const minFeeA = shelley.minFeeA ?? shelley.protocolParams?.minFeeA;
  const minFeeB = shelley.minFeeB ?? shelley.protocolParams?.minFeeB;
  if (minFeeA === 0 || minFeeB === 0) return closed('GENESIS_PROFILE_REFUSED', flags);
  const delegs = shelley.genDelegs;
  if (!delegs || typeof delegs !== 'object' || Array.isArray(delegs) || Object.keys(delegs).length === 0) {
    return closed('GENESIS_DELEGATION_ABSENT', flags);
  }
  if (sameJson(delegs, template.genDelegs || {})) return closed('GENESIS_DELEGATION_NOT_FRESH', flags);
  if (!Number.isInteger(supplyLovelace) || supplyLovelace < 3) return closed('SUPPLY_ABSENT', flags);
  const hashes = [];
  for (const path of paths.utxoVkeys) {
    if (!existsSync(path)) return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { ...flags, reason: 'vkey-absent', path });
    const checked = freshVerificationHex(readFileSync(path, 'utf8'));
    if (!checked.ok) return closed(checked.code, flags);
    hashes.push(checked.hash);
  }
  if (new Set(hashes).size !== hashes.length) return closed('PAYOUT_KEY_COLLAPSED', flags);
  const addresses = [];
  for (const path of paths.addrOuts) {
    if (!existsSync(path)) return closed('ADDRESS_NOT_DERIVED', { ...flags, path });
    const address = readFileSync(path, 'utf8').trim();
    if (!address.startsWith('addr')) return closed('ADDRESS_NOT_DERIVED', flags);
    addresses.push(address);
  }
  if (new Set(addresses).size !== addresses.length) return closed('PAYOUT_KEY_COLLAPSED', flags);
  let maxLovelaceSupply = shelley.maxLovelaceSupply;
  let maxAdjusted = false;
  if (!Number.isInteger(maxLovelaceSupply) || maxLovelaceSupply < supplyLovelace) {
    if (Number.isInteger(maxLovelaceSupply) && maxLovelaceSupply > 0 && maxLovelaceSupply < supplyLovelace) {
      maxLovelaceSupply = supplyLovelace + Math.floor(supplyLovelace / 10);
      maxAdjusted = true;
    } else if (!Number.isInteger(maxLovelaceSupply)) {
      return closed('SUPPLY_EXCEEDS_TEMPLATE_MAX', flags);
    } else {
      maxLovelaceSupply = supplyLovelace + Math.floor(supplyLovelace / 10);
      maxAdjusted = true;
    }
  }
  const each = Math.floor(supplyLovelace / 3);
  const remainder = supplyLovelace - each * 3;
  if (each < 1) return closed('SUPPLY_ABSENT', flags);
  const initialFunds = {
    [addresses[0]]: each + remainder,
    [addresses[1]]: each,
    [addresses[2]]: each,
  };
  const written = { ...shelley, maxLovelaceSupply, initialFunds };
  writeFileSync(paths.shelleyWork, `${JSON.stringify(written, null, 2)}\n`);
  return {
    ok: true,
    code: unitOnly ? 'FRESH_FUNDS_BOUND_UNIT_ONLY' : 'FRESH_FUNDS_BOUND',
    ...flags,
    unitOnly: unitOnly !== false,
    ledgerExecution: false,
    measured: false,
    spawned: false,
    addresses,
    paymentKeyHashes: hashes,
    initialFundsKeyedByAddress: true,
    initialFundsKeyedByPaymentHash: false,
    maxLovelaceSupplyAdjusted: maxAdjusted,
    byronRead: true,
    byronClockIsWitnessClock: false,
    systemStart: written.systemStart,
  };
}

export function installOwnedGenesis(paths, { nodePort, unitOnly = true } = {}) {
  const flags = { fundedInNodeInitialFunds: false, ledgerFunding: false, bootstrapComplete: false, evidenceClass: unitOnly ? 'unit-fixture' : 'installed-owned-genesis' };
  for (const path of [paths.shelleyWork, paths.byronWork, paths.alonzoWork, paths.conwayWork]) {
    if (!existsSync(path)) return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { ...flags, reason: 'install-source-absent', path });
  }
  mkdirSync(join(paths.clusterShelley, '..'), { recursive: true });
  mkdirSync(paths.utxoDir, { recursive: true });
  copyFileSync(paths.shelleyWork, paths.clusterShelley);
  copyFileSync(paths.byronWork, paths.clusterByron);
  copyFileSync(paths.alonzoWork, paths.clusterAlonzo);
  copyFileSync(paths.conwayWork, paths.clusterConway);
  for (let index = 0; index < 3; index += 1) {
    if (!existsSync(paths.utxoVkeys[index]) || !existsSync(paths.utxoSkeys[index])) {
      return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { ...flags, reason: 'utxo-key-absent' });
    }
    copyFileSync(paths.utxoVkeys[index], join(paths.utxoDir, `utxo${index + 1}.vkey`));
    copyFileSync(paths.utxoSkeys[index], join(paths.utxoDir, `utxo${index + 1}.skey`));
  }
  const config = renderPinnedNodeConfig(paths.nodeConfigTemplate, paths.configuration);
  if (!config.ok) return { ...config, ...flags };
  const topology = renderPinnedTopology(paths.topologyTemplate, paths.topology, nodePort);
  if (!topology.ok) return { ...topology, ...flags };
  const installed = readJson(paths.clusterShelley);
  const addresses = Object.keys(installed.initialFunds || {});
  if (addresses.length !== 3 || addresses.some((item) => !item.startsWith('addr'))) {
    return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { ...flags, reason: 'installed-funds-not-address-keyed' });
  }
  return {
    ok: true,
    code: 'OWNED_GENESIS_INSTALLED',
    ...flags,
    unitOnly: unitOnly !== false,
    ledgerExecution: false,
    spawned: false,
    nodeStarted: false,
    dijkstraGenesisWritten: false,
    dijkstraGenesisOmitted: config.dijkstraGenesisOmitted === true,
    dijkstraOmitReason: config.dijkstraOmitReason || null,
  };
}

export function awaitOwnedSocket(socketPath, { unitOnly = true } = {}) {
  const flags = {
    nodeStarted: false,
    ledgerExecution: false,
    fundedInNodeInitialFunds: false,
    ledgerFunding: false,
    evidenceClass: unitOnly ? 'unit-fixture' : 'socket-stat',
    spawned: false,
  };
  if (typeof socketPath !== 'string' || !socketPath.endsWith('node.sock')) return closed('SOCKET_NOT_NODE_SOCK', flags);
  if (!existsSync(socketPath)) return closed('READINESS_TIMEOUT', flags);
  const stat = statSync(socketPath);
  if (typeof stat.isSocket !== 'function' || stat.isSocket() !== true) {
    return closed('SOCKET_NOT_READY', { ...flags, reason: 'not-a-socket', bytes: stat.size });
  }
  return { ok: true, code: 'SOCKET_READY', ...flags, bytes: stat.size, isSocket: true, measured: false };
}

function readAddressFile(path) {
  if (!existsSync(path)) return null;
  const address = readFileSync(path, 'utf8').trim();
  if (!address.startsWith('addr')) return null;
  return address;
}

export function resolveBootstrapArgv(step, plan, bag) {
  if (Array.isArray(step.argv) && step.bind !== true) return step.argv;
  const paths = plan.paths;
  const magic = String(plan.magic);
  const cli = plan.cli;
  if (step.id === 'funding-build') {
    const utxo = bag['query-funder-utxo'];
    if (!utxo?.ok || typeof utxo.txIn !== 'string') return closed('NO_TX_IN', { id: step.id });
    const funder = readAddressFile(paths.addrOuts[0]);
    const roles = paths.roles.map((role) => readAddressFile(role.addr));
    if (!funder || roles.some((item) => item == null)) return closed('ADDRESS_NOT_DERIVED', { id: step.id });
    const room = assessFundingConservation({ inputLovelace: utxo.lovelace, feeLovelace: 1, roleCount: 3 });
    if (!room.ok && room.reason === 'no-fee-room') return room;
    const each = Math.floor(utxo.lovelace / 10);
    if (each < 1) return closed('CONSERVATION_FAILED', { id: step.id, reason: 'role-output-zero' });
    const argv = [
      cli, 'conway', 'transaction', 'build',
      '--socket-path', paths.socketPath,
      '--testnet-magic', magic,
      '--tx-in', utxo.txIn,
      '--change-address', funder,
      '--out-file', paths.fundingBody,
    ];
    for (const address of roles) argv.push('--tx-out', `${address}+${each}`);
    return argv;
  }
  if (step.id === 'calculate-min-fee') {
    const skey = paths.utxoSkeys?.[0];
    if (!skey || !existsSync(skey)) return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { id: step.id });
    const checked = freshVerificationHex(readFileSync(skey, 'utf8'));
    if (!checked.ok) return closed(checked.code, { id: step.id });
    const signers = [skey];
    return [
      cli, 'conway', 'transaction', 'calculate-min-fee',
      '--tx-body-file', paths.fundingBody,
      '--protocol-params-file', paths.protocolOut,
      '--witness-count', String(signers.length),
    ];
  }
  if (step.id === 'funding-sign') {
    const skey = paths.utxoSkeys[0];
    if (!existsSync(skey)) return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { id: step.id });
    const checked = freshVerificationHex(readFileSync(skey, 'utf8'));
    if (!checked.ok) return closed(checked.code, { id: step.id });
    return [
      cli, 'conway', 'transaction', 'sign',
      '--tx-body-file', paths.fundingBody,
      '--signing-key-file', skey,
      '--testnet-magic', magic,
      '--out-file', paths.fundingSigned,
    ];
  }
  if (step.id.startsWith('role-address-') || step.id.startsWith('funding-confirm-')) {
    const role = paths.roles.find((item) => step.id.endsWith(item.name));
    if (!role || !existsSync(role.vkey)) return closed('FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', { id: step.id });
    const checked = freshVerificationHex(readFileSync(role.vkey, 'utf8'));
    if (!checked.ok) return closed(checked.code, { id: step.id });
    if (step.id.startsWith('funding-confirm-')) {
      const address = readAddressFile(role.addr);
      if (!address) return closed('ADDRESS_NOT_DERIVED', { id: step.id });
      return step.argv.concat(['--address', address]);
    }
    return step.argv;
  }
  if (step.id === 'query-funder-utxo') {
    const funder = readAddressFile(paths.addrOuts[0]);
    if (!funder) return closed('ADDRESS_NOT_DERIVED', { id: step.id });
    return step.argv.concat(['--address', funder]);
  }
  return step.argv || closed('PRODUCER_STEP_NOT_RUN', { id: step.id });
}

export function runLocalBootstrap(step, plan, { unitOnly = true } = {}) {
  const paths = plan.paths;
  if (step.local === 'bind-initial-funds') {
    return bindFreshInitialFunds(paths, {
      unitOnly,
      magic: plan.magic,
      supplyLovelace: plan.supplyLovelace,
      templateShelley: paths.templateShelley,
    });
  }
  if (step.local === 'install-owned-genesis') {
    return installOwnedGenesis(paths, { nodePort: plan.nodePort, unitOnly });
  }
  if (step.local === 'await-socket') return awaitOwnedSocket(paths.socketPath, { unitOnly });
  if (step.local === 'funding-conservation') {
    return assessFundingConservation({
      inputLovelace: plan._utxoLovelace,
      feeLovelace: plan._feeLovelace,
    });
  }
  if (step.local === 'bind-signed-witnesses') {
    let signedCborHex = null;
    if (unitOnly === false && existsSync(paths.fundingSigned)) {
      try {
        signedCborHex = JSON.parse(readFileSync(paths.fundingSigned, 'utf8')).cborHex || null;
      } catch {
        signedCborHex = null;
      }
    }
    const skey = paths.utxoSkeys?.[0];
    const keyOk = skey && existsSync(skey) ? freshVerificationHex(readFileSync(skey, 'utf8')).ok === true : false;
    return bindSignedWitnessCount({
      unitOnly,
      signedCborHex,
      signingKeyCount: keyOk ? 1 : 0,
    });
  }
  return closed('PRODUCER_STEP_NOT_RUN', { id: step.id });
}

export function buildBootstrapPlan(input, deps) {
  const home = input.ownedHome;
  const runId = input.runId;
  const layout = yaciLayout(home, runId);
  const ports = portMapping(input.nodePort, input.submitApiPort);
  const nodeShape = layout.ok && ports.ok
    ? createNodeArgvShape(home, runId, input.nodePort, input.submitApiPort)
    : ports.ok ? layout : ports;
  const keys = layout.ok ? freshFaucetKeys(home, runId) : layout;
  const profile = input.genesisProfile == null ? null : 'GENESIS_PROFILE_REFUSED';
  const magicOk = input.magicSource === 'caller-chosen-for-genesis'
    && Number.isInteger(input.magic)
    && input.magic !== 42;
  const pinned = deps.pinnedTemplates;
  const templates = {
    genesisDir: layout.ok ? layout.genesisKeysFolder : null,
    byronTemplate: input.templates?.byronTemplate || pinned.byron,
    shelleyTemplate: input.templates?.shelleyTemplate || pinned.shelley,
    alonzoTemplate: input.templates?.alonzoTemplate || pinned.alonzo,
    conwayTemplate: input.templates?.conwayTemplate || pinned.conway,
  };
  const genesis = deps.ownedGenesisArgv({
    cli: input.cli,
    ownedHome: home,
    magic: input.magic,
    magicSource: input.magicSource,
    topLevelGenesis: input.topLevelGenesis === true,
    slotLength: input.slotLength,
    genUtxoKeys: 3,
    supplyLovelace: input.supplyLovelace,
    ...templates,
  });
  const paths = layout.ok ? genesisPaths(home, layout) : {};
  if (layout.ok) {
    paths.templateShelley = templates.shelleyTemplate;
    paths.nodeConfigTemplate = pinned.nodeConfig;
    paths.topologyTemplate = pinned.topology;
  }
  const nodeBin = input.nodeBin;
  const nodeOk = typeof nodeBin === 'string' && isAbsolute(nodeBin) && nodeBin.endsWith('cardano-node') && existsSync(nodeBin);
  const socketOk = !input.socketPath || input.socketPath === layout.socketPath;
  const templatesExist = ['byronTemplate', 'shelleyTemplate', 'alonzoTemplate', 'conwayTemplate']
    .every((key) => typeof templates[key] === 'string' && isAbsolute(templates[key]) && existsSync(templates[key]));
  const zeroFeeTemplate = templatesExist && templateFeeRefused(templates.shelleyTemplate);
  const blockers = [];
  if (!layout.ok) blockers.push(layout.code);
  else if (!ports.ok) blockers.push(ports.code);
  else if (profile) blockers.push(profile);
  else if (input.proofTxtOnly === true) blockers.push('PROOF_TXT_NOT_READINESS');
  else if (String(input.launcher || '').includes('start.mjs')) blockers.push('YACI_WRAPPER_REFUSED');
  else if (input.evaluate === true || input.command === 'evaluate') blockers.push('NO_CLI_EVALUATE_COMMAND');
  else if (input.topLevelGenesis === true) blockers.push('YACI_CREATE_CARDANO_ARGV_REJECTED_BY_PINNED_CLI');
  else if (input.slotLength != null && input.slotLength !== 1000) blockers.push('SLOT_LENGTH_REFUSED');
  else if (!magicOk) blockers.push('MAGIC_SOURCE_REFUSED');
  else if (!Number.isInteger(input.supplyLovelace) || input.supplyLovelace < 3) {
    blockers.push(input.supplyLovelace === 0 ? 'EMPTY_FUNDER_NOT_A_UTXO' : 'SUPPLY_ABSENT');
  } else if (!templatesExist) blockers.push('GENESIS_TEMPLATE_ABSENT');
  else if (zeroFeeTemplate) blockers.push('GENESIS_PROFILE_REFUSED');
  else if (!genesis.ok) blockers.push(genesis.code);
  else if (!socketOk) blockers.push('SOCKET_NOT_NODE_SOCK');
  else if (!nodeOk) blockers.push('NODE_BINARY_ABSENT');
  const outputPlan = deps.planOutputMinimums(input);
  if (outputPlan.blocker) blockers.push(outputPlan.blocker);
  const inputsReady = blockers.length === 0;
  const magic = magicOk ? input.magic : null;
  const cost = deps.plutusCostArgv({
    cli: input.cli,
    mode: input.costMode || 'online',
    txFile: input.txFile || '',
    socket: layout.ok ? layout.socketPath : '',
    magic,
    magicSource: magicOk ? 'shelley-networkMagic' : input.magicSource,
    eraHistoryFile: input.eraHistoryFile,
    utxoFile: input.utxoFile,
    protocolParamsFile: input.protocolParamsFile,
    startTime: input.startTime,
    byronGenesis: input.byronGenesis,
    outFile: input.costOutFile,
  });
  const cli = input.cli;
  const socketPath = layout.ok ? layout.socketPath : '';
  const magicText = magic == null ? null : String(magic);
  const roles = paths.roles || [];
  const sequence = [];
  if (layout.ok && roles.length === 3) {
    for (const role of roles) {
      sequence.push(step(`role-keygen-${role.name}`, [
        cli, 'address', 'key-gen',
        '--normal-key',
        '--verification-key-file', role.vkey,
        '--signing-key-file', role.skey,
      ], 'ROLE_KEYGEN_ARGV', inputsReady));
    }
  }
  sequence.push(step('legacy-genesis', genesis.argv || null, genesis.code, inputsReady, {
    initialFundsWrittenByThisCommand: false,
  }));
  if (layout.ok) {
    paths.utxoVkeys.forEach((vkey, index) => {
      sequence.push(step(`initial-addr-${index}`, [
        cli, 'legacy', 'genesis', 'initial-addr',
        '--verification-key-file', vkey,
        '--testnet-magic', magicText || '0',
        '--out-file', paths.addrOuts[index],
      ], 'INITIAL_ADDR_ARGV', inputsReady));
    });
  }
  sequence.push(step('bind-initial-funds', null, 'BIND_INITIAL_FUNDS', inputsReady, { local: 'bind-initial-funds' }));
  sequence.push(step('install-owned-genesis', null, 'INSTALL_OWNED_GENESIS', inputsReady, { local: 'install-owned-genesis' }));
  const yaciPresent = existsSync(YACI_CLI);
  const createArgv = nodeShape.ok && yaciPresent ? [YACI_CLI, ...nodeShape.argv] : null;
  if (inputsReady && nodeShape.ok && !yaciPresent) blockers.push('YACI_CLI_ABSENT');
  sequence.push(step('create-node', createArgv, nodeShape.ok && yaciPresent ? 'CREATE_NODE_ARGV' : (nodeShape.ok ? 'YACI_CLI_ABSENT' : nodeShape.code), inputsReady && yaciPresent, { start: false }));
  sequence.push(step('start-node', nodeOk && layout.ok ? [
    nodeBin, 'run',
    '--topology', paths.topology,
    '--database-path', paths.database,
    '--socket-path', paths.socketPath,
    '--config', paths.configuration,
    '--host-addr', '127.0.0.1',
    '--port', String(ports.nodePort),
  ] : null, 'NODE_RUN_ARGV', inputsReady, { start: true }));
  sequence.push(step('await-socket', null, 'AWAIT_SOCKET', inputsReady, { local: 'await-socket' }));
  sequence.push(step('query-tip', layout.ok && magicText ? [
    cli, 'query', 'tip', '--socket-path', socketPath, '--testnet-magic', magicText, '--output-json',
  ] : null, 'QUERY_TIP_ARGV', inputsReady));
  sequence.push(step('query-era-history', layout.ok && magicText ? [
    cli, 'query', 'era-history', '--socket-path', socketPath, '--testnet-magic', magicText,
  ] : null, 'QUERY_ERA_HISTORY_ARGV', inputsReady));
  sequence.push(step('query-protocol', layout.ok && magicText ? [
    cli, 'query', 'protocol-parameters', '--socket-path', socketPath, '--testnet-magic', magicText, '--out-file', paths.protocolOut,
  ] : null, 'QUERY_PROTOCOL_ARGV', inputsReady));
  for (const role of roles) {
    sequence.push(step(`role-address-${role.name}`, magicText ? [
      cli, 'address', 'build',
      '--payment-verification-key-file', role.vkey,
      '--testnet-magic', magicText,
      '--out-file', role.addr,
    ] : null, 'ROLE_ADDRESS_ARGV', inputsReady, { bind: true }));
  }
  sequence.push(step('query-funder-utxo', layout.ok && magicText ? [
    cli, 'query', 'utxo', '--output-text', '--socket-path', socketPath, '--testnet-magic', magicText,
  ] : null, 'QUERY_FUNDER_UTXO_ARGV', inputsReady, { bind: true }));
  sequence.push(step('funding-build', null, 'FUNDING_BUILD_ARGV', inputsReady, { bind: true }));
  sequence.push(step('calculate-min-fee', null, 'MIN_FEE_ARGV', inputsReady, {
    bind: true,
    witnessCountProvenance: 'counted-from-signing-key-paths',
    feeAppliesToSignedTx: false,
    budgetPin: false,
  }));
  sequence.push(step('funding-conservation', null, 'FUNDING_CONSERVATION', inputsReady, { local: 'funding-conservation' }));
  sequence.push(step('funding-sign', null, 'FUNDING_SIGN_ARGV', inputsReady, { bind: true }));
  sequence.push(step('bind-signed-witnesses', null, 'WITNESS_COUNT_BOUND', inputsReady, { local: 'bind-signed-witnesses' }));
  sequence.push(step('funding-submit', layout.ok && magicText ? [
    cli, 'conway', 'transaction', 'submit',
    '--testnet-magic', magicText,
    '--socket-path', socketPath,
    '--tx-file', paths.fundingSigned,
  ] : null, 'FUNDING_SUBMIT_ARGV', inputsReady));
  for (const role of roles) {
    sequence.push(step(`funding-confirm-${role.name}`, layout.ok && magicText ? [
      cli, 'query', 'utxo', '--output-text', '--socket-path', socketPath, '--testnet-magic', magicText,
    ] : null, 'FUNDING_CONFIRM_ARGV', inputsReady, { bind: true }));
  }
  sequence.push(step('calculate-plutus-script-cost', cost.argv || null, cost.code, inputsReady && cost.ok === true));
  if (input.financialFrame === true && layout.ok && magicText) {
    const financialSlots = [
      'fin-cash-seed', 'fin-head', 'fin-mint-fee', 'fin-mint-collateral',
      'fin-head-fee', 'fin-head-collateral', 'fin-release-fee', 'fin-release-collateral',
      'fin-refund-fee', 'fin-refund-collateral', 'fin-cancel-fee', 'fin-cancel-collateral',
    ];
    for (const id of financialSlots) {
      sequence.push(step(id, [
        cli, 'query', 'utxo', '--output-text', '--socket-path', socketPath, '--testnet-magic', magicText,
      ], 'FINANCIAL_ROLE_QUERY', inputsReady, { bind: true, financialSlot: id }));
    }
  }
  for (const extra of outputPlan.steps) {
    sequence.push({ ...extra, run: inputsReady && Array.isArray(extra.argv) });
  }
  return {
    ok: inputsReady,
    code: inputsReady ? 'PRODUCER_INPUTS_READY' : blockers[0],
    blockers,
    inputsReady,
    spawned: false,
    executed: false,
    executeNow: false,
    ledgerExecution: false,
    nativePins: false,
    nativeBudgetMeasured: false,
    ledgerFunding: false,
    measured: false,
    parityExecuted: false,
    selfApproval: false,
    bootstrapComplete: false,
    callerShelleyIgnored: input.shelleyGenesisPath || null,
    callerInitialFundsIgnored: true,
    layout: layout.ok ? layout : { ok: false, code: layout.code },
    keys,
    nodeShape,
    genesis,
    fund: { ok: false, code: 'FRESH_UTXO_KEY_NOT_IN_NODE_INITIAL_FUNDS', fundedInNodeInitialFunds: false, reason: 'bind-runs-after-genesis' },
    cost,
    sequence,
    paths,
    cli,
    magic,
    nodePort: ports.ok ? ports.nodePort : null,
    supplyLovelace: input.supplyLovelace,
    fundedInNodeInitialFunds: false,
    schema71Adopted: true,
    schema68Adopted: false,
    helper77ValuesAreBudgetPins: false,
  };
}

const APPROVED_BOOTSTRAP_ROOT = '/home/charl/moriarty-eutxo-validation-2026-10-12/bootstrap151-verified-candidate-freeze155/primary/cardano-cli/test/cardano-cli-golden/files/input';
const APPROVED_SPEC_PINS = {
  alonzo: {
    file: 'genesis.alonzo.spec.json',
    sha256: '104bdbd36634fc48c0221dc481bb37f2556a3c0265aa26ce0820e28e3830cbd3',
    bytes: 17615,
  },
  conway: {
    file: 'genesis.conway.spec.json',
    sha256: '16052a510f369584e4a621e3ac2a1ecab121666ebf2b59e868975978046abba5',
    bytes: 5331,
  },
};

function exactJsonInteger(digits) {
  if (!/^(?:0|[1-9][0-9]*)$/.test(digits)) return null;
  const value = Number(digits);
  return String(value) === digits ? value : null;
}

function approvedStart(nowMs, leadSeconds) {
  if (!Number.isInteger(nowMs) || !Number.isInteger(leadSeconds) || leadSeconds <= 0) return null;
  const seconds = Math.floor(nowMs / 1000) + leadSeconds;
  return new Date(seconds * 1000).toISOString().replace('.000Z', 'Z');
}

function pinFile(kind) {
  const pin = APPROVED_SPEC_PINS[kind];
  const path = join(APPROVED_BOOTSTRAP_ROOT, pin.file);
  if (!existsSync(path)) return closed('GENESIS_SPEC_ABSENT', { kind });
  const bytes = readFileSync(path);
  if (bytes.length !== pin.bytes) return closed('GENESIS_SPEC_PIN_MISMATCH', { kind, bytes: bytes.length });
  const sha256 = createHash('sha256').update(bytes).digest('hex');
  if (sha256 !== pin.sha256) return closed('GENESIS_SPEC_PIN_MISMATCH', { kind });
  return { ok: true, path, sha256, bytes: bytes.length };
}

function shelleySpec(start) {
  const supply = exactJsonInteger('45000000000000000');
  if (supply == null) return null;
  return {
    activeSlotsCoeff: 0.05,
    systemStart: start,
    genDelegs: {},
    updateQuorum: 1,
    maxMajorPV: 10,
    maxLovelaceSupply: supply,
    initialFunds: {},
    networkMagic: 0,
    networkId: 'Testnet',
    epochLength: 3600,
    securityParam: 18,
    slotLength: 1,
    slotsPerKESPeriod: 3600,
    maxKESEvolutions: 64,
    protocolParams: {
      a0: 0,
      decentralisationParam: 0,
      eMax: 0,
      extraEntropy: { tag: 'NeutralNonce' },
      keyDeposit: 0,
      maxBlockBodySize: 2097152,
      maxBlockHeaderSize: 8192,
      maxTxSize: 16384,
      minFeeA: 1,
      minFeeB: 1,
      minUTxOValue: 1,
      minPoolCost: 0,
      nOpt: 100,
      poolDeposit: 0,
      protocolVersion: { major: 10, minor: 0 },
      rho: 0,
      tau: 0,
    },
  };
}

const BYRON_PROTOCOL_PARAMS = {
  heavyDelThd: '300000000000',
  maxBlockSize: '2000000',
  maxTxSize: '4096',
  maxHeaderSize: '2000000',
  maxProposalSize: '700',
  mpcThd: '20000000000000',
  scriptVersion: 0,
  slotDuration: '1000',
  softforkRule: {
    initThd: '900000000000000',
    minThd: '600000000000000',
    thdDecrement: '50000000000000',
  },
  txFeePolicy: {
    multiplier: '43946000000',
    summand: '155381000000000',
  },
  unlockStakeEpoch: '18446744073709551615',
  updateImplicit: '10000',
  updateProposalThd: '100000000000000',
  updateVoteThd: '1000000000000',
};

export function producerRoleFrame(parsed) {
  if (!parsed?.ok) return closed(parsed?.code || 'FUNDER_UTXO_ABSENT', { nodeUtxoQueried: false, confirmed: false });
  const digits = parsed.lovelaceText;
  if (typeof digits !== 'string' || !/^[1-9][0-9]*$/.test(digits)) {
    return closed('LOSSY_LOVELACE', { nodeUtxoQueried: false, confirmed: false });
  }
  const quantity = BigInt(digits);
  if (parsed.lovelace != null && (!Number.isSafeInteger(parsed.lovelace) || BigInt(parsed.lovelace) !== quantity)) {
    return closed('LOSSY_LOVELACE', { nodeUtxoQueried: false, confirmed: false });
  }
  return {
    ok: true,
    code: 'PRODUCER_ROLE_FRAME',
    txIn: parsed.txIn || null,
    lovelaceText: digits,
    lovelaceExact: true,
    numberUsed: false,
    nodeUtxoQueried: false,
    ledgerFunding: false,
    nativeBudgetMeasured: false,
    ledgerExecution: false,
    confirmed: false,
    spawned: false,
  };
}

export function planApprovedBootstrap(input = {}) {
  const flags = {
    spawned: false,
    executed: false,
    executeNow: false,
    nodeStarted: false,
    byronReplaced: false,
    hashesBound: false,
    ledgerExecution: false,
    nativeBudgetMeasured: false,
    confirmed: false,
  };
  if (input.signingKey || input.cborHex || input.budget || input.oldGenesis || input.bundledKeys) {
    return closed('BUNDLED_INPUT_REFUSED', flags);
  }
  if (!Number.isInteger(input.magic) || input.magic <= 0 || input.magic === 42) {
    return closed('MAGIC_SOURCE_REFUSED', flags);
  }
  const start = approvedStart(input.nowMs, input.leadSeconds);
  if (!start) return closed('START_UNBOUND', flags);
  if (typeof input.runDir !== 'string' || !isAbsolute(input.runDir)) return closed('RUN_DIR_ABSENT', flags);
  const alonzo = pinFile('alonzo');
  if (!alonzo.ok) return { ...alonzo, ...flags };
  const conway = pinFile('conway');
  if (!conway.ok) return { ...conway, ...flags };
  const shelley = shelleySpec(start);
  if (!shelley) return closed('LOSSY_RECIPE_INTEGER', flags);
  const run = input.runDir;
  const magic = String(input.magic);
  const cli = 'cardano-cli';
  const node = 'cardano-node';
  const steps = [
    {
      id: 'S0-spec-files',
      executed: false,
      writesGenesis: false,
      outputs: ['spec/shelley.spec.json', 'spec/alonzo.spec.json', 'spec/conway.spec.json', 'spec/byron-protocol-params.json'],
    },
    {
      id: 'S1-create-testnet-data',
      executed: false,
      argv: [
        cli, 'conway', 'genesis', 'create-testnet-data',
        '--spec-shelley', join(run, 'spec/shelley.spec.json'),
        '--spec-alonzo', join(run, 'spec/alonzo.spec.json'),
        '--spec-conway', join(run, 'spec/conway.spec.json'),
        '--testnet-magic', magic,
        '--start-time', start,
        '--genesis-keys', '1',
        '--pools', '1',
        '--stake-delegators', '1',
        '--utxo-keys', '1',
        '--total-supply', '45000000000000000',
        '--delegated-supply', '30000000000000000',
        '--out-dir', join(run, 'genesis'),
      ],
    },
    {
      id: 'S2-create-cardano-byron-zero',
      executed: false,
      supplyOmitted: true,
      argv: [
        cli, 'conway', 'genesis', 'create-cardano',
        '--genesis-dir', join(run, 'byron-only'),
        '--gen-genesis-keys', '1',
        '--gen-utxo-keys', '0',
        '--start-time', start,
        '--security-param', '18',
        '--slot-length', '1000',
        '--slot-coefficient', '0.05',
        '--testnet-magic', magic,
        '--byron-template', join(run, 'spec/byron-protocol-params.json'),
        '--shelley-template', join(run, 'spec/shelley.spec.json'),
        '--alonzo-template', join(run, 'genesis/alonzo-genesis.json'),
        '--conway-template', join(run, 'genesis/conway-genesis.json'),
      ],
    },
    { id: 'S3-replace-byron-genesis', executed: false, source: join(run, 'byron-only/byron-genesis.json'), destination: join(run, 'genesis/byron-genesis.json') },
    {
      id: 'S4-hash-after-replacement',
      executed: false,
      hashes: null,
      argv: [
        [cli, 'byron', 'genesis', 'print-genesis-hash', '--genesis-json', join(run, 'genesis/byron-genesis.json')],
        [cli, 'hash', 'genesis-file', '--genesis', join(run, 'genesis/shelley-genesis.json')],
        [cli, 'hash', 'genesis-file', '--genesis', join(run, 'genesis/alonzo-genesis.json')],
        [cli, 'hash', 'genesis-file', '--genesis', join(run, 'genesis/conway-genesis.json')],
      ],
    },
    { id: 'S5-node-config-and-topology', executed: false, configPath: join(run, 'node-config.json') },
    {
      id: 'S6-address-build-funder',
      executed: false,
      argv: [
        cli, 'address', 'build',
        '--payment-verification-key-file', join(run, 'genesis/utxo-keys/utxo1/utxo.vkey'),
        '--testnet-magic', magic,
        '--out-file', join(run, 'genesis/utxo-keys/utxo1/enterprise.addr'),
      ],
    },
    {
      id: 'S7-node-run',
      executed: false,
      spawn: false,
      argv: [
        node, 'run',
        '--config', join(run, 'node-config.json'),
        '--topology', join(run, 'topology.json'),
        '--database-path', join(run, 'db'),
        '--socket-path', join(run, 'node.socket'),
        '--host-addr', '127.0.0.1',
        '--port', '3131',
        '--shelley-kes-key', join(run, 'genesis/pools-keys/pool1/kes.skey'),
        '--shelley-vrf-key', join(run, 'genesis/pools-keys/pool1/vrf.skey'),
        '--shelley-operational-certificate', join(run, 'genesis/pools-keys/pool1/opcert.cert'),
      ],
    },
    {
      id: 'S8-queries',
      executed: false,
      protocolFileIsInput: false,
      forms: [
        [cli, 'latest', 'query', 'tip', '--socket-path', join(run, 'node.socket'), '--testnet-magic', magic],
        [cli, 'latest', 'query', 'era-history', '--socket-path', join(run, 'node.socket'), '--testnet-magic', magic],
        [cli, 'latest', 'query', 'protocol-parameters', '--socket-path', join(run, 'node.socket'), '--testnet-magic', magic, '--out-file', join(run, 'protocol-parameters.json')],
      ],
    },
  ];
  return {
    ok: true,
    code: 'APPROVED_BOOTSTRAP_PLANNED',
    ...flags,
    design: 'bootstrap151-verified-candidate-freeze155',
    runDir: run,
    home: join(run, 'home'),
    socketPath: join(run, 'node.socket'),
    pidPath: join(run, 'node.pid'),
    start,
    magic: input.magic,
    shelley,
    byronProtocolParams: BYRON_PROTOCOL_PARAMS,
    specPins: {
      alonzo: { path: alonzo.path, sha256: alonzo.sha256, bytes: alonzo.bytes, edit: { exUnitsMem: { from: 14000000, to: 16500000 } } },
      conway: { path: conway.path, sha256: conway.sha256, bytes: conway.bytes, edit: null },
    },
    keys: {
      pool: 'genesis/pools-keys/pool1',
      delegator: 'genesis/stake-delegators/delegator1',
      funder: 'genesis/utxo-keys/utxo1',
    },
    steps,
  };
}

export function materializeApprovedBootstrapSpecs(plan, runDir) {
  if (!plan?.ok || plan.code !== 'APPROVED_BOOTSTRAP_PLANNED') return closed('BOOTSTRAP_PLAN_ABSENT', { spawned: false });
  if (runDir !== plan.runDir) return closed('RUN_DIR_MISMATCH', { spawned: false });
  const specDir = join(runDir, 'spec');
  mkdirSync(specDir, { recursive: true });
  writeFileSync(join(specDir, 'shelley.spec.json'), `${JSON.stringify(plan.shelley, null, 2)}\n`);
  writeFileSync(join(specDir, 'byron-protocol-params.json'), `${JSON.stringify(plan.byronProtocolParams, null, 2)}\n`);
  const alonzoBytes = readFileSync(plan.specPins.alonzo.path);
  const edited = alonzoBytes.toString('utf8').replace('"exUnitsMem": 14000000', '"exUnitsMem": 16500000');
  if (edited === alonzoBytes.toString('utf8') || edited.split('"exUnitsMem": 16500000').length !== 2) {
    return closed('ALONZO_SPEC_EDIT_REFUSED', { spawned: false });
  }
  writeFileSync(join(specDir, 'alonzo.spec.json'), edited);
  copyFileSync(plan.specPins.conway.path, join(specDir, 'conway.spec.json'));
  return {
    ok: true,
    code: 'BOOTSTRAP_SPECS_MATERIALIZED',
    spawned: false,
    executed: false,
    nodeStarted: false,
    writesGenesis: false,
    specDir,
    ledgerExecution: false,
    nativeBudgetMeasured: false,
  };
}

export function noteByronReplaced(plan) {
  if (!plan?.ok) return closed('BOOTSTRAP_PLAN_ABSENT');
  return { ...plan, byronReplaced: true };
}

export function bindApprovedNodeConfig(plan, hashes = {}) {
  if (!plan?.ok || plan.byronReplaced !== true) return closed('HASH_BEFORE_BYRON_REPLACEMENT', { spawned: false, hashesBound: false });
  const names = ['byron', 'shelley', 'alonzo', 'conway'];
  for (const name of names) {
    if (!/^[0-9a-f]{64}$/.test(String(hashes[name] || ''))) return closed('GENESIS_HASH_ABSENT', { name, spawned: false });
  }
  if (hashes.byronMethod !== 'print-genesis-hash') return closed('BYRON_HASH_METHOD_REFUSED', { spawned: false });
  const config = {
    RequiresNetworkMagic: 'RequiresMagic',
    ByronGenesisFile: 'genesis/byron-genesis.json',
    ShelleyGenesisFile: 'genesis/shelley-genesis.json',
    AlonzoGenesisFile: 'genesis/alonzo-genesis.json',
    ConwayGenesisFile: 'genesis/conway-genesis.json',
    ByronGenesisHash: hashes.byron,
    ShelleyGenesisHash: hashes.shelley,
    AlonzoGenesisHash: hashes.alonzo,
    ConwayGenesisHash: hashes.conway,
    'LastKnownBlockVersion-Major': 10,
    'LastKnownBlockVersion-Minor': 0,
    'LastKnownBlockVersion-Alt': 0,
    TestShelleyHardForkAtEpoch: 0,
    TestAllegraHardForkAtEpoch: 0,
    TestMaryHardForkAtEpoch: 0,
    TestAlonzoHardForkAtEpoch: 0,
    TestBabbageHardForkAtEpoch: 0,
    TestConwayHardForkAtEpoch: 0,
  };
  return {
    ok: true,
    code: 'NODE_CONFIG_BOUND',
    spawned: false,
    executed: false,
    nodeStarted: false,
    configPath: join(plan.runDir, 'node-config.json'),
    config,
    ledgerExecution: false,
    nativeBudgetMeasured: false,
    confirmed: false,
  };
}
