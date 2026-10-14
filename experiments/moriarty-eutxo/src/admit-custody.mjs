/**
 * Admission for public-single-head-script-custody-transfer/1.
 * Calls createFinancialAgreementSourceV5().elaborate. Does not spend.
 */
import { createFinancialAgreementSourceV5 } from '../../moriarty-language/src/successor/financial-agreement-source-v5.ts';
import { hashDomain, hashRawDomain, canonicalEncode, utf8 } from '../../moriarty-language/src/codec.ts';
import { instanceDigest, rolesOk } from './same-script-identity.mjs';

export const PROFILE = 'moriarty-financial-agreement-source/5';
export const CONTRACT = 'moriarty-financial-expression-contract/4';
export const LIFECYCLE = 'moriarty-financial-lifecycle-state/1';
export const PREDICATE = 'public-single-head-script-custody-transfer/1';
export const MANIFEST_SCHEMA = 'moriarty-t1-custody-manifest/1';
const UINT128_MAX = 340282366920938463463374607431768211455n;
const AMOUNT_BODY = /^(0|[1-9][0-9]*)$/;
const HEX28 = /^[0-9a-f]{56}$/;
const ZERO28 = '00'.repeat(28);
const SHAPE_IDS = new Set([
  '24ba87f0a8b4f617d991e86e7a84fc3d4d90855dae7b2ce94e451a79',
  '1d038a1069d649adf83745f02ff5c84575e32839194593263b0a2855',
  'b39aa2a57e9c3730b8234ada7592743d512edca5c23f60ba03c9dd4b',
  '72f72192639eb173fce5342d2df730b73a0d838c48d2422ebf91e48c',
  '64b8d588ca2e8207014edd458014a7b49007f8c01fab78ff809925fe',
  'f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d4',
]);
const ALLOWED = new Set([
  'Require', 'Ensure', 'NextWrite', 'Emit',
  'Eq', 'Lt', 'Lte', 'Gt', 'Gte', 'Add',
  'ReadPre', 'ReadArg', 'LitUInt', 'LitText',
  'ConstructRecord', 'ConstructAmount',
]);
const READ_ARGS = new Set(['actor', 'recipient', 'transferId', 'lower', 'upper']);

export const BOUNDS_RECORD = {
  profile: PROFILE,
  contract: CONTRACT,
  lifecycleState: LIFECYCLE,
  nativePredicate: PREDICATE,
  hashDomainVersion: 'moriarty-hash-domain-financial-agreement/5',
  quantum: '1',
  admittedActions: ['release', 'refund', 'cancel'],
  ordinaryFields: ['locked', 'revision'],
  lifetime: {
    transportRevisionInitial: '0',
    transportRevisionIncrement: '1',
    transportRemainingInitial: '1',
    transportRemainingDecrement: '1',
  },
  horizon: { kind: 'interval-args-equal-converted-txinfo', predicate: 'TIME_PREDICATE_v1' },
  workBudget: {
    initial: '1024',
    staticWorkBoundPerAction: '48',
    kernelTransferUnit: '1',
    spentAfterOneTransfer: '49',
    remainingAfterOneTransfer: '975',
    closureReserve: '16',
    location: 'inside-bounds-only',
  },
  forecastHorizonSec: '1800',
  slotLengthMsRequired: '1000',
  minInclusionSlots: '30',
  minWindowSlots: '60',
  windowSlots: '60',
  windowSlotsRule: 'equals-minWindowSlots',
  slotLengthMismatch: 'SLOT_LENGTH_REFUSED',
};

function refuse(code, detail) {
  return { ok: false, code, detail, ledgerExecution: false, nativePins: false };
}

export function constructorsOf(value, bag) {
  if (value === null || typeof value !== 'object') return;
  if (Object.hasOwn(value, 'constructor') && typeof value.constructor === 'string') {
    bag[value.constructor] = (bag[value.constructor] || 0) + 1;
  }
  if (Array.isArray(value)) value.forEach((item) => constructorsOf(item, bag));
  else Object.values(value).forEach((item) => constructorsOf(item, bag));
}

function walk(value, visit) {
  if (value === null || typeof value !== 'object') return;
  if (Object.hasOwn(value, 'constructor') && typeof value.constructor === 'string') visit(value);
  if (Array.isArray(value)) value.forEach((item) => walk(item, visit));
  else Object.values(value).forEach((item) => walk(item, visit));
}

export function scanCore(core) {
  if (!core || !Array.isArray(core.statements)) return refuse('MALFORMED_DATA');
  const bag = {};
  constructorsOf(core, bag);
  if (bag.FloorDiv) return refuse('FLOOR_DIV');
  if (bag.Let) return refuse('LET');
  if (bag.LitBool) return refuse('ORDINARY_TYPE');
  for (const name of Object.keys(bag)) {
    if (!ALLOWED.has(name)) return refuse('EXPRESSION_UNSUPPORTED', name);
  }
  let problem = null;
  walk(core, (node) => {
    if (problem) return;
    if (node.constructor === 'LitUInt') {
      if (node.operands?.width !== '128') problem = refuse('UINT_WIDTH');
      else if (typeof node.operands.value !== 'string' || !AMOUNT_BODY.test(node.operands.value)) problem = refuse('NON_CANONICAL_VALUE');
      else if (BigInt(node.operands.value) > UINT128_MAX) problem = refuse('OVERFLOW');
    } else if (node.constructor === 'ReadArg' && !READ_ARGS.has(node.operands?.name)) {
      problem = refuse('EXPRESSION_UNSUPPORTED', node.operands?.name);
    } else if (node.constructor === 'ReadPre') {
      const view = node.operands?.view;
      const field = node.operands?.field;
      if ((view !== 'pre' && view !== 'post') || (field !== 'locked' && field !== 'revision')) {
        problem = refuse('EXPRESSION_UNSUPPORTED', `${view}.${field}`);
      }
    } else if (node.constructor === 'Emit' && node.operands?.operation !== 'Transfer') {
      problem = refuse('EFFECT_KIND_UNSUPPORTED', node.operands?.operation);
    } else if (node.constructor === 'LitText' && typeof node.operands?.value !== 'string') {
      problem = refuse('MALFORMED_DATA');
    }
  });
  if (problem) return problem;
  const emits = core.statements.filter((statement) => statement.constructor === 'Emit');
  if (emits.length !== 1) return refuse('EFFECT_KIND_UNSUPPORTED');
  const count = Object.values(bag).reduce((sum, n) => sum + n, 0);
  return { ok: true, count, bag };
}

export function fenceLifecycle(state) {
  if (!state || typeof state !== 'object' || Array.isArray(state)) return refuse('MALFORMED_DATA');
  if (state.schemaVersion !== LIFECYCLE) return refuse('SCHEMA_VERSION');
  if (!Array.isArray(state.obligations) || state.obligations.length !== 0) return refuse('NONEMPTY_OBLIGATIONS');
  for (const key of ['usedAllocationIds', 'usedOriginationIds', 'usedAccrualIds']) {
    if (!Array.isArray(state[key]) || state[key].length !== 0) return refuse('USED_ID_FENCE', key);
  }
  if (!Array.isArray(state.usedTransferIds)) return refuse('MALFORMED_DATA', 'usedTransferIds');
  return { ok: true };
}

function admitManifest(manifest) {
  if (manifest === null || typeof manifest !== 'object' || Array.isArray(manifest)) return refuse('MALFORMED_DATA');
  if (Object.getPrototypeOf(manifest) !== Object.prototype && Object.getPrototypeOf(manifest) !== null) {
    return refuse('NON_CANONICAL_VALUE');
  }
  if (manifest.nativePins === true) return refuse('NATIVE_PINS_ABSENT');
  if (manifest.production === true) return refuse('PRODUCTION_REFUSED');
  if (manifest.schema !== MANIFEST_SCHEMA) return refuse('MANIFEST_SCHEMA');
  if (manifest.unitFixture !== true) return refuse('UNIT_FIXTURE_REQUIRED');
  if (typeof manifest.quantum !== 'string') return refuse('NON_CANONICAL_VALUE');
  if (manifest.quantum !== '1') return refuse('QUANTUM');
  if (manifest.domainNetwork !== 'yaci-isolated-t1' || manifest.domainDeployment !== 't1-custody-revision03') {
    return refuse('DOMAIN');
  }
  if (manifest.threadAssetNameHex !== '48656164' || manifest.cashAssetNameHex !== '43617368') return refuse('ASSET_NAME');
  const ids = ['threadPolicyId', 'cashPolicyId', 'lenderVkh', 'borrowerVkh', 'scriptHash'];
  for (const key of ids) {
    const hex = manifest[key];
    if (typeof hex !== 'string' || !HEX28.test(hex)) return refuse('MALFORMED_ID', key);
    if (hex === ZERO28) return refuse('ALL_ZERO_FORBIDDEN', key);
    if (SHAPE_IDS.has(hex)) return refuse('SHAPE_SAMPLE_FORBIDDEN', key);
  }
  if (manifest.threadPolicyId !== manifest.scriptHash) return refuse('THREAD_POLICY_MUST_EQUAL_SCRIPT_HASH');
  if (!rolesOk(manifest.scriptHash, manifest.cashPolicyId, manifest.lenderVkh, manifest.borrowerVkh)) {
    return refuse('POLICY_NOT_DISTINCT');
  }
  const digest = instanceDigest(manifest.scriptHash, manifest.cashPolicyId);
  if (!digest.ok) return refuse(digest.code, digest.detail);
  if (SHAPE_IDS.has(digest.instanceId)) return refuse('SHAPE_SAMPLE_FORBIDDEN', 'instanceId');
  return { ok: true, instanceId: digest.instanceId };
}

function admitSchema(schema) {
  if (!schema || typeof schema.fields !== 'object') return refuse('MALFORMED_DATA');
  const fields = schema.fields;
  for (const field of Object.values(fields)) {
    const types = field?.type;
    if (Array.isArray(types) && (types.includes('Text') || types.includes('Bool') || types.includes('Amount') || types.includes('Quantity'))) {
      return refuse('ORDINARY_TYPE');
    }
  }
  const keys = Object.keys(fields).sort();
  if (keys.join(',') !== 'locked,revision') return refuse('ORDINARY_FIELDS');
  for (const key of keys) {
    const field = fields[key];
    if (field.writeClass !== 'ordinary' || !Array.isArray(field.type) || field.type.length !== 1 || field.type[0] !== 'UInt128') {
      return refuse('ORDINARY_TYPE', key);
    }
  }
  if (!schema.observations || typeof schema.observations !== 'object' || Object.keys(schema.observations).length !== 0) {
    return refuse('OBSERVATIONS_NONEMPTY');
  }
  if (!Array.isArray(schema.assets) || schema.assets.length !== 1 || schema.assets[0] !== 'Cash') return refuse('ASSET_UNSUPPORTED');
  return { ok: true };
}

function admitSeed(seed) {
  const fence = fenceLifecycle(seed);
  if (!fence.ok) return fence;
  if (seed.usedTransferIds.length !== 0) return refuse('USED_ID_FENCE', 'usedTransferIds');
  const work = seed.work;
  if (!work || work.remaining !== '1024' || work.spent !== '0' || work.closureReserve !== '16') return refuse('WORK_SEED');
  const expectParties = ['Escrow', 'Borrower', 'Lender'];
  const pair = (rows, amountKey) => {
    const seen = new Set();
    if (!Array.isArray(rows)) return refuse('MALFORMED_DATA');
    for (const row of rows) {
      const key = `${row.party}\0${row.asset}`;
      if (seen.has(key)) return refuse('NON_INJECTIVE');
      seen.add(key);
      if (row.asset !== 'Cash') return refuse('ASSET_UNSUPPORTED');
      if (typeof row[amountKey] !== 'string' || !AMOUNT_BODY.test(row[amountKey])) return refuse('NON_CANONICAL_VALUE');
    }
    for (const party of expectParties) {
      if (!seen.has(`${party}\0Cash`)) return refuse('MISSING_BALANCE', party);
    }
    return { ok: true };
  };
  const balances = pair(seed.balances, 'amount');
  if (!balances.ok) return balances;
  const allowances = pair(seed.allowances, 'remaining');
  if (!allowances.ok) return allowances;
  return { ok: true };
}

export function admitCustody({ sourceText, seedText, manifest, elaborated }) {
  const pins = admitManifest(manifest);
  if (!pins.ok) return pins;
  if (typeof sourceText !== 'string') return refuse('MALFORMED_DATA');
  let seed;
  try {
    seed = JSON.parse(seedText);
  } catch {
    return refuse('MALFORMED_DATA');
  }
  const seedOk = admitSeed(seed);
  if (!seedOk.ok) return seedOk;
  let compiled = elaborated;
  if (!compiled) {
    try {
      compiled = createFinancialAgreementSourceV5().elaborate(sourceText);
    } catch (error) {
      return refuse(error.code || 'MALFORMED_DATA', error.message);
    }
  }
  if (!compiled || compiled.status === 'Rejected' || compiled.judgmentResult !== 'SourceElaborated') {
    return refuse(compiled?.code || 'PROFILE_UNSUPPORTED');
  }
  if (compiled.sourceProfile !== PROFILE) return refuse('PROFILE_UNSUPPORTED', compiled.sourceProfile);
  if (compiled.contract !== CONTRACT) return refuse('CONTRACT_UNSUPPORTED', compiled.contract);
  const names = (compiled.actions || []).map((action) => action.action);
  if (names.join(',') !== 'release,refund,cancel') return refuse('ACTIONS', names.join(','));
  const schemaOk = admitSchema(compiled.actions[0].schema);
  if (!schemaOk.ok) return schemaOk;
  let schemaCanon;
  try {
    schemaCanon = canonicalEncode(compiled.actions[0].schema);
  } catch (error) {
    return refuse(error.code || 'NON_CANONICAL_VALUE', error.message);
  }
  const actions = [];
  for (const action of compiled.actions) {
    let canon;
    try {
      canon = canonicalEncode(action.schema);
    } catch (error) {
      return refuse(error.code || 'NON_CANONICAL_VALUE', error.message);
    }
    if (canon !== schemaCanon) return refuse('SCHEMA_DIVERGENCE', action.action);
    const scanned = scanCore(action.core);
    if (!scanned.ok) return scanned;
    if (action.staticWorkBound !== '48' || scanned.count !== 48) {
      return refuse('WORK_BOUND', `${action.action}:${scanned.count}:${action.staticWorkBound}`);
    }
    actions.push({
      action: action.action,
      staticWorkBound: action.staticWorkBound,
      constructorCount: scanned.count,
      debit: scanned.count + 1,
    });
  }
  let hashes;
  try {
    const programPayload = {
      sourceProfile: compiled.sourceProfile,
      contract: compiled.contract,
      agreement: compiled.agreement,
      actions: compiled.actions.map((action) => ({
        action: action.action,
        core: action.core,
        staticWorkBound: action.staticWorkBound,
      })),
    };
    hashes = {
      sourceHash: hashRawDomain('MORIARTY-SOURCE-financial-agreement/5', utf8(sourceText)),
      programHash: hashDomain('MORIARTY-PROGRAM-financial-agreement/5', programPayload),
      schemaHash: hashDomain('MORIARTY-SCHEMA-financial-expression-contract/4', compiled.actions[0].schema),
      seedHash: hashDomain('MORIARTY-SEED-financial-lifecycle-state/1', seed),
      boundsHash: hashDomain('MORIARTY-BOUNDS-financial-agreement/5', BOUNDS_RECORD),
    };
  } catch (error) {
    return refuse(error.code || 'NON_CANONICAL_VALUE', error.message);
  }
  return {
    ok: true,
    ledgerExecution: false,
    nativePins: false,
    layer: 'source-uplc-unit',
    predicate: PREDICATE,
    genesis: 'trusted-lock-fixture-not-proved',
    hashes,
    instanceId: pins.instanceId,
    actions,
    elaborated: compiled,
    seed,
    manifest,
  };
}
