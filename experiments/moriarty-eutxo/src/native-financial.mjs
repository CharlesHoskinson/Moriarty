/**
 * Source-evaluated terminal posts for the T1 financial consumer.
 * Posts come from the custody evaluator. They are not a ledger result.
 * constructor work 49 and collection capacity 128 stay off the datum.
 */
import { readFileSync } from 'node:fs';
import { join } from 'node:path';
import { createFinancialAgreementSourceV5 } from '../../moriarty-language/src/successor/financial-agreement-source-v5.ts';
import { fenceLifecycle } from './admit-custody.mjs';

export const COLLECTION_CAPACITY = 128;
export const CONSTRUCTOR_WORK = 49;
const DESIGNER = '/home/charl/moriarty-eutxo-validation-2026-10-12/designers/02-escrow-time';

const TERMINALS = {
  release: {
    caseId: 'E13-release-real-custody',
    action: 'release',
    constructor: 0,
  },
  refund: {
    caseId: 'E11-refund-real-custody',
    action: 'refund',
    constructor: 1,
  },
  cancel: {
    caseId: null,
    action: 'cancel',
    constructor: 2,
    input: JSON.stringify({
      Args: { actor: 'Borrower', lower: '1050', recipient: 'Borrower', transferId: 'Cancel1', upper: '1051' },
      Obs: {},
      Pre: { locked: '1', revision: '0' },
      workInitial: '1024',
    }),
  },
};

function closed(code, extra = {}) {
  return { ok: false, code, ledgerExecution: false, nativePins: false, cashCanMint: false, ...extra };
}

function dataBytes(text) {
  return { bytes: Buffer.from(String(text), 'utf8').toString('hex') };
}

function asInt(value) {
  if (typeof value === 'number' && Number.isInteger(value)) return value;
  if (typeof value === 'string' && /^(0|[1-9][0-9]*)$/.test(value)) return Number(value);
  return null;
}

const ALLOCATION_RULE = { AccrualFirst: 0, PrincipalFirst: 1, ProRata: 2 };
const OBLIGATION_STATUS = { Outstanding: 0, Settled: 1 };
const ROUNDING = { RoundingNone: 0, RoundingFloor: 1, RoundingCeil: 2 };
const ACCRUAL_ROUND = { AccrualFloor: 0, AccrualCeil: 1 };
const ECONOMIC_PACKET = '/home/charl/moriarty-eutxo-validation-2026-10-12/economic173-verified-freeze176/packet173/expected-cases.json';

function enumCtor(table, value) {
  const index = Object.prototype.hasOwnProperty.call(table, String(value)) ? table[value] : 0;
  return { constructor: index, fields: [] };
}

function obligationPlutus(row) {
  if (row && Number.isInteger(row.constructor) && Array.isArray(row.fields)) return row;
  const text = (value) => dataBytes(value ?? '');
  const num = (value) => ({ int: asInt(value) ?? 0 });
  const conv = row?.conversion || {};
  const terms = row?.accrual_terms || row?.accrualTerms || {};
  return {
    constructor: 0,
    fields: [
      text(row?.id),
      text(row?.debtor),
      text(row?.creditor),
      text(row?.denomination),
      text(row?.settlement_asset ?? row?.settlementAsset),
      num(row?.principal),
      num(row?.accrued),
      num(row?.outstanding),
      enumCtor(ALLOCATION_RULE, row?.allocation_rule ?? row?.allocationRule),
      { constructor: 0, fields: [num(conv.mantissa), num(conv.scale), enumCtor(ROUNDING, conv.rounding)] },
      enumCtor(OBLIGATION_STATUS, row?.status),
      text(row?.origination_id ?? row?.originationId),
      text(row?.origination_transfer_id ?? row?.originationTransferId),
      num(row?.initial_principal ?? row?.initialPrincipal),
      num(row?.nominal_liability_cap ?? row?.nominalLiabilityCap),
      num(row?.liability_incurred ?? row?.liabilityIncurred),
      {
        constructor: 0,
        fields: [
          num(terms.numerator),
          num(terms.denominator),
          enumCtor(ACCRUAL_ROUND, terms.rounding),
          num(terms.period_seconds ?? terms.periodSeconds),
          num(terms.first_period_start ?? terms.firstPeriodStart),
        ],
      },
      num(row?.last_accrued_period ?? row?.lastAccruedPeriod),
      num(row?.next_accrual_at ?? row?.nextAccrualAt),
    ],
  };
}

function pickCase(cases, id) {
  return cases?.cases?.find((item) => item.id === id) || null;
}

export function assertObligationsPreserved(pre, post) {
  const before = Array.isArray(pre) ? pre : null;
  const after = Array.isArray(post) ? post : null;
  if (!before || !after) return closed('OBLIGATION_ERASURE');
  if (JSON.stringify(before) !== JSON.stringify(after)) return closed('OBLIGATION_ERASURE');
  return { ok: true, code: 'OBLIGATIONS_PRESERVED' };
}

function plutusLifecycle(post) {
  const balance = (row) => ({ constructor: 0, fields: [dataBytes(row.party), dataBytes(row.asset), { int: asInt(row.amount) }] });
  const allowance = (row) => ({
    constructor: 0,
    fields: [dataBytes(row.party), dataBytes(row.asset), { int: asInt(row.remaining) }, { int: asInt(row.spent) }],
  });
  const ids = (rows) => ({ list: (rows || []).map((row) => dataBytes(row)) });
  return {
    constructor: 0,
    fields: [
      dataBytes(post.schemaVersion),
      { list: post.balances.map(balance) },
      { list: post.allowances.map(allowance) },
      { list: (post.obligations || []).map(obligationPlutus) },
      ids(post.usedTransferIds),
      ids(post.usedAllocationIds),
      ids(post.usedOriginationIds),
      ids(post.usedAccrualIds),
      {
        constructor: 0,
        fields: [
          { int: asInt(post.work.remaining) },
          { int: asInt(post.work.spent) },
          { int: asInt(post.work.closureReserve) },
        ],
      },
    ],
  };
}

export function datumFromPost(hashes, instanceId, terminal) {
  const post = terminal.financialPost;
  const fields = {
    program_hash: hashes.programHash,
    bounds_hash: hashes.boundsHash,
    source_hash: hashes.sourceHash,
    schema_hash: hashes.schemaHash,
    seed_hash: hashes.seedHash,
    domain_network: 'yaci-isolated-t1',
    domain_deployment: 't1-custody-revision03',
    instance_id: instanceId,
    transport_revision: asInt(terminal.transportRevision),
    transport_remaining: asInt(terminal.transportRemaining),
    ordinary_locked: asInt(terminal.post.locked),
    ordinary_revision: asInt(terminal.post.revision),
    lifecycle: {
      schema_version: post.schemaVersion,
      balances: post.balances.map((row) => ({ party: row.party, asset: row.asset, amount: asInt(row.amount) })),
      allowances: post.allowances.map((row) => ({
        party: row.party,
        asset: row.asset,
        remaining: asInt(row.remaining),
        spent: asInt(row.spent),
      })),
      obligations: post.obligations,
      used_transfer_ids: post.usedTransferIds || [],
      used_allocation_ids: post.usedAllocationIds || [],
      used_origination_ids: post.usedOriginationIds || [],
      used_accrual_ids: post.usedAccrualIds || [],
      usedTransferIds: post.usedTransferIds || [],
      usedAllocationIds: post.usedAllocationIds || [],
      usedOriginationIds: post.usedOriginationIds || [],
      usedAccrualIds: post.usedAccrualIds || [],
      work: {
        remaining: asInt(post.work.remaining),
        spent: asInt(post.work.spent),
        closure_reserve: asInt(post.work.closureReserve),
      },
      constructorWorkCount: CONSTRUCTOR_WORK,
      collectionCapacity: COLLECTION_CAPACITY,
    },
  };
  const plutus = {
    constructor: 0,
    fields: [
      { bytes: fields.program_hash },
      { bytes: fields.bounds_hash },
      { bytes: fields.source_hash },
      { bytes: fields.schema_hash },
      { bytes: fields.seed_hash },
      dataBytes(fields.domain_network),
      dataBytes(fields.domain_deployment),
      { bytes: fields.instance_id },
      { int: fields.transport_revision },
      { int: fields.transport_remaining },
      { int: fields.ordinary_locked },
      { int: fields.ordinary_revision },
      plutusLifecycle(post),
    ],
  };
  return { fields, plutus };
}

export function redeemerFromArgs(kind, args) {
  const spec = TERMINALS[kind];
  if (!spec || !args) return closed('MALFORMED_DATA');
  const lower = asInt(args.lower);
  const upper = asInt(args.upper);
  if (lower == null || upper == null || upper < lower) return closed('MALFORMED_DATA');
  return {
    ok: true,
    kind,
    lower,
    upper,
    invalidBefore: lower,
    invalidHereafter: upper + 1,
    plutus: {
      constructor: spec.constructor,
      fields: [dataBytes(args.actor), dataBytes(args.recipient), dataBytes(args.transferId), { int: lower }, { int: upper }],
    },
    payoutParty: args.recipient,
    transferId: args.transferId,
  };
}

function terminalFromEval(kind, evaluated, args, preObligations) {
  if (!evaluated || evaluated.status !== 'FundedExpressionPrepared') {
    return closed(evaluated?.code || 'SOURCE_POST_REJECTED', { kind, status: evaluated?.status || null });
  }
  const post = evaluated.financialPost;
  const kept = assertObligationsPreserved(preObligations, post?.obligations);
  if (!kept.ok) return { ...kept, kind };
  const fence = fenceLifecycle(post);
  if (!fence.ok) return { ...fence, kind, bodyBuilt: false, fenceBypass: false };
  if (asInt(post?.work?.spent) !== CONSTRUCTOR_WORK) return closed('CONSTRUCTOR_WORK_MISMATCH', { kind, spent: post?.work?.spent });
  const effect = evaluated.effects?.[0];
  if (!effect || effect.asset !== 'Cash' || asInt(effect.amount) !== 100) return closed('CASH_NOT_CONSERVED', { kind });
  const redeemer = redeemerFromArgs(kind, args);
  if (!redeemer.ok) return redeemer;
  return {
    ok: true,
    kind,
    financialPost: post,
    post: evaluated.post,
    effects: evaluated.effects,
    transportRevision: '1',
    transportRemaining: '0',
    args,
    redeemer,
    constructorWorkCount: CONSTRUCTOR_WORK,
    collectionCapacity: COLLECTION_CAPACITY,
    continuingCash: 0,
    payoutCash: 100,
    headQuantity: 1,
  };
}

export function loadSourceTerminals(sourceText) {
  if (typeof sourceText !== 'string' || sourceText.length < 1000) return closed('SOURCE_POST_ABSENT');
  const cases = JSON.parse(readFileSync(join(DESIGNER, 'cases.json'), 'utf8'));
  const seedText = readFileSync(join(DESIGNER, 'financial-seed-custody-v5.json'), 'utf8');
  const residualText = readFileSync(join(DESIGNER, 'financial-residual-custody-v5.json'), 'utf8');
  const evaluator = createFinancialAgreementSourceV5();
  const preObligations = [];
  const terminals = {};
  for (const kind of ['release', 'refund']) {
    const spec = TERMINALS[kind];
    const row = pickCase(cases, spec.caseId);
    if (!row) return closed('SOURCE_POST_ABSENT', { kind });
    const ordinary = JSON.parse(row.ordinaryInputCanonicalJSON);
    const evaluated = evaluator.evaluate(sourceText, spec.action, row.ordinaryInputCanonicalJSON, seedText);
    const built = terminalFromEval(kind, evaluated, ordinary.Args, preObligations);
    if (!built.ok) return built;
    terminals[kind] = built;
  }
  const cancelOrdinary = JSON.parse(TERMINALS.cancel.input);
  const cancelEval = evaluator.evaluate(sourceText, 'cancel', TERMINALS.cancel.input, seedText);
  const cancel = terminalFromEval('cancel', cancelEval, cancelOrdinary.Args, preObligations);
  if (!cancel.ok) return cancel;
  terminals.cancel = cancel;
  const residualCase = pickCase(cases, 'E12-cancel-preserves-duty');
  const residualEval = residualCase
    ? evaluator.evaluate(sourceText, 'cancel', residualCase.ordinaryInputCanonicalJSON, residualText)
    : null;
  const residualObligations = residualEval?.financialPost?.obligations || [];
  const residualFence = fenceLifecycle(residualEval?.financialPost || {});
  return {
    ok: true,
    code: 'SOURCE_POSTS_EVALUATED',
    terminals,
    residual: {
      bodyBuilt: false,
      fenceBypass: false,
      code: residualFence.ok ? 'RESIDUAL_FENCE_MISSED' : residualFence.code,
      obligationsPreserved: residualObligations.length > 0,
      obligationCount: residualObligations.length,
    },
    ledgerExecution: false,
    nativePins: false,
  };
}

function partyAmount(balances, party) {
  const row = (balances || []).find((item) => item.party === party && item.asset === 'Cash');
  return row ? asInt(row.amount) : null;
}

function allowanceOf(allowances, party) {
  const row = (allowances || []).find((item) => item.party === party && item.asset === 'Cash');
  if (!row) return null;
  return { remaining: asInt(row.remaining), spent: asInt(row.spent) };
}

export function expectedFrame(terminal, identity) {
  const post = terminal.financialPost;
  return {
    program_hash: identity.programHash || null,
    bounds_hash: identity.boundsHash || null,
    source_hash: identity.sourceHash,
    schema_hash: identity.schemaHash || null,
    seed_hash: identity.seedHash || null,
    domain_network: 'yaci-isolated-t1',
    domain_deployment: 't1-custody-revision03',
    balances: {
      Escrow: partyAmount(post.balances, 'Escrow'),
      Borrower: partyAmount(post.balances, 'Borrower'),
      Lender: partyAmount(post.balances, 'Lender'),
    },
    allowances: {
      Escrow: allowanceOf(post.allowances, 'Escrow'),
      Borrower: allowanceOf(post.allowances, 'Borrower'),
      Lender: allowanceOf(post.allowances, 'Lender'),
    },
    obligations: post.obligations,
    work: {
      remaining: asInt(post.work.remaining),
      spent: asInt(post.work.spent),
      closureReserve: asInt(post.work.closureReserve),
    },
    usedTransferIds: post.usedTransferIds,
    usedAllocationIds: post.usedAllocationIds || [],
    usedOriginationIds: post.usedOriginationIds || [],
    usedAccrualIds: post.usedAccrualIds || [],
    transport_remaining: asInt(terminal.transportRemaining),
    transport_revision: asInt(terminal.transportRevision),
    ordinary_locked: asInt(terminal.post.locked),
    ordinary_revision: asInt(terminal.post.revision),
    sourceHash: identity.sourceHash,
    instanceId: identity.instanceId,
    headQuantity: 1,
    continuingCash: 0,
    payoutCash: 100,
    constructorWorkCount: CONSTRUCTOR_WORK,
    collectionCapacity: COLLECTION_CAPACITY,
  };
}

function sameJson(left, right) {
  return JSON.stringify(left) === JSON.stringify(right);
}

export function compareFinancialFrame(decoded, expected, link = {}) {
  if (!decoded || !expected) return closed('READBACK_FINANCIAL_MISMATCH', { confirmed: false });
  if (link.runId && decoded.runId && link.runId !== decoded.runId) {
    return closed('READBACK_STALE_RUN', { confirmed: false, runId: decoded.runId, expectedRunId: link.runId });
  }
  if (link.bodySha256 && decoded.bodySha256 && link.bodySha256 !== decoded.bodySha256) {
    return closed('READBACK_BODY_CHANGED', { confirmed: false });
  }
  if (link.outputRef && decoded.outputRef && link.outputRef === link.previousOutputRef) {
    return closed('READBACK_OUTPUT_REUSED', { confirmed: false, outputRef: decoded.outputRef });
  }
  const mismatches = [];
  const check = (name, got, want) => {
    if (!sameJson(got, want)) mismatches.push(name);
  };
  check('balances', decoded.balances, expected.balances);
  check('allowances', decoded.allowances, expected.allowances);
  check('obligations', decoded.obligations, expected.obligations);
  check('work', decoded.work, expected.work);
  check('usedTransferIds', decoded.usedTransferIds, expected.usedTransferIds);
  for (const name of ['usedAllocationIds', 'usedOriginationIds', 'usedAccrualIds']) {
    if (Object.hasOwn(expected, name)) check(name, decoded[name] || [], expected[name]);
  }
  check('transport_remaining', decoded.transport_remaining, expected.transport_remaining);
  check('ordinary_locked', decoded.ordinary_locked, expected.ordinary_locked);
  check('sourceHash', decoded.sourceHash, expected.sourceHash);
  check('instanceId', decoded.instanceId, expected.instanceId);
  check('headQuantity', decoded.headQuantity, expected.headQuantity);
  check('continuingCash', decoded.continuingCash, expected.continuingCash);
  check('payoutCash', decoded.payoutCash, expected.payoutCash);
  for (const name of [
    'program_hash', 'bounds_hash', 'source_hash', 'schema_hash', 'seed_hash',
    'domain_network', 'domain_deployment', 'transport_revision', 'ordinary_revision',
  ]) {
    if (Object.hasOwn(expected, name)) check(name, decoded[name], expected[name]);
  }
  if (mismatches.length) return closed('READBACK_FINANCIAL_MISMATCH', { confirmed: false, mismatches });
  return { ok: true, code: 'FINANCIAL_FRAME_MATCHED', financialMatch: true, confirmed: false, ledgerExecution: false };
}

function loadEconomicPacket() {
  try {
    const packet = JSON.parse(readFileSync(ECONOMIC_PACKET, 'utf8'));
    const fields = packet?.datum?.fields;
    if (!Array.isArray(fields)) return null;
    const lifecycle = fields.find((field) => field.name === 'lifecycle');
    const instance = fields.find((field) => field.name === 'instance_id');
    const rejectedInstanceIds = new Set(['0'.repeat(64)]);
    for (const digest of String(instance?.open || '').match(/[0-9a-f]{64}/g) || []) rejectedInstanceIds.add(digest);
    const actions = {};
    for (const action of ['release', 'refund', 'cancel']) {
      const post = lifecycle?.[`${action}Post`];
      if (!post?.work) return null;
      const balances = {};
      for (const row of post.balances || []) {
        if (row.asset === 'Cash') balances[row.party] = row.amount;
      }
      const allowances = {};
      for (const row of post.allowances || []) {
        if (row.asset === 'Cash') allowances[row.party] = { remaining: row.remaining, spent: row.spent };
      }
      const counters = {};
      for (const field of fields) {
        if (field.name === 'lifecycle' || field.name === 'instance_id') continue;
        const value = field[`${action}Post`];
        if (value === 'preserved' && typeof field.open === 'string' && !/^[0-9a-f]{64}$/.test(field.open)) {
          counters[field.name] = field.open;
        } else if (typeof value === 'number') counters[field.name] = value;
      }
      const recipient = ['Lender', 'Borrower', 'Escrow'].find((party) => balances[party] === 100 && party !== 'Escrow') || null;
      actions[action] = {
        balances,
        allowances,
        obligations: post.obligations || [],
        usedAllocationIds: post.used_allocation_ids || [],
        usedOriginationIds: post.used_origination_ids || [],
        usedAccrualIds: post.used_accrual_ids || [],
        fixtureTransferId: (post.used_transfer_ids || [])[0] || null,
        work: {
          remaining: post.work.remaining,
          spent: post.work.spent,
          closureReserve: post.work.closure_reserve,
        },
        recipient,
        amount: 100,
        counters,
      };
    }
    return { ok: true, path: ECONOMIC_PACKET, actions, rejectedInstanceIds };
  } catch {
    return null;
  }
}

export function judgeEconomicAction(decoded, action, context = {}) {
  const packet = loadEconomicPacket();
  const refused = (mismatches) => ({
    ok: false,
    code: 'ECONOMIC_ACTION_REFUSED',
    action: action || null,
    mismatches,
    confirmed: false,
    ledgerExecution: false,
    nativeBudgetMeasured: false,
    providerQueried: false,
    packetLoaded: packet?.ok === true,
    packetPath: packet?.path || ECONOMIC_PACKET,
  });
  const spec = packet?.actions?.[action];
  if (!packet?.ok || !spec || !decoded) return refused(['action']);
  const mismatches = [];
  const reject = (name, ok) => {
    if (!ok) mismatches.push(name);
  };
  const redeemer = context.redeemer || decoded.redeemer || {};
  const legalTransferId = redeemer.transferId || spec.fixtureTransferId;
  const balances = decoded.balances || {};
  reject('balances', balances.Escrow === spec.balances.Escrow
    && balances.Borrower === spec.balances.Borrower
    && balances.Lender === spec.balances.Lender);
  const allowances = decoded.allowances || {};
  reject('allowances', ['Escrow', 'Borrower', 'Lender'].every((party) => (
    allowances[party]?.remaining === spec.allowances[party]?.remaining
    && allowances[party]?.spent === spec.allowances[party]?.spent
  )));
  reject('obligations', sameJson(decoded.obligations || [], spec.obligations));
  reject('work', decoded.work?.remaining === spec.work.remaining
    && decoded.work?.spent === spec.work.spent
    && (decoded.work?.closureReserve ?? decoded.work?.closure_reserve) === spec.work.closureReserve);
  reject('transfer', Array.isArray(decoded.usedTransferIds)
    && decoded.usedTransferIds.length === 1
    && decoded.usedTransferIds[0] === legalTransferId);
  reject('usedAllocationIds', sameJson(decoded.usedAllocationIds || [], spec.usedAllocationIds));
  reject('usedOriginationIds', sameJson(decoded.usedOriginationIds || [], spec.usedOriginationIds));
  reject('usedAccrualIds', sameJson(decoded.usedAccrualIds || [], spec.usedAccrualIds));
  reject('transport', decoded.transport_revision === spec.counters.transport_revision
    && decoded.transport_remaining === spec.counters.transport_remaining);
  reject('ordinary', decoded.ordinary_locked === spec.counters.ordinary_locked
    && decoded.ordinary_revision === spec.counters.ordinary_revision);
  reject('domain', decoded.domain_network === spec.counters.domain_network
    && decoded.domain_deployment === spec.counters.domain_deployment);
  const redeemerRecipient = redeemer.recipient || redeemer.payoutParty || null;
  if (redeemerRecipient) reject('recipient', redeemerRecipient === spec.recipient);
  const redeemerAmount = redeemer.amount == null ? spec.amount : asInt(redeemer.amount);
  reject('amount', redeemerAmount === spec.amount);
  reject('headQuantity', decoded.headQuantity === 1);
  reject('continuingCash', decoded.continuingCash === spec.balances.Escrow);
  reject('payoutCash', decoded.payoutCash === spec.amount);
  reject('phase', decoded.phase == null && decoded.nonce == null);
  reject('fakeConfirmed', decoded.confirmed !== true);
  reject('identity', decoded.program_hash !== '00'.repeat(32)
    && decoded.instanceId !== '00'.repeat(32)
    && !packet.rejectedInstanceIds.has(decoded.instanceId));
  if (context.pre && typeof context.pre === 'object') {
    const pairs = [
      ['program_hash', 'program_hash'],
      ['bounds_hash', 'bounds_hash'],
      ['source_hash', 'source_hash'],
      ['schema_hash', 'schema_hash'],
      ['seed_hash', 'seed_hash'],
      ['instanceId', 'instance_id'],
    ];
    for (const [decodedName, preName] of pairs) {
      const prior = context.pre[preName] ?? context.pre[decodedName];
      if (prior != null && decoded[decodedName] != null) reject(decodedName, decoded[decodedName] === prior);
    }
  }
  if (mismatches.length) return refused(mismatches);
  return {
    ok: true,
    code: 'ECONOMIC_ACTION_MATCHED',
    action,
    recipient: spec.recipient,
    transferId: legalTransferId,
    fixtureTransferId: spec.fixtureTransferId,
    amount: spec.amount,
    confirmed: false,
    ledgerExecution: false,
    nativeBudgetMeasured: false,
    providerQueried: false,
    packetLoaded: true,
    packetPath: packet.path,
    spendMint: 'empty',
    continuationHead: 1,
  };
}
