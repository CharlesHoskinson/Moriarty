/**
 * Lifecycle collection-capacity envelopes for the custody compile path.
 * The bound is LIFECYCLE_BOUNDS.collectionCapacity. These spends are
 * source/UPLC-unit correspondence. They are not native inclusion.
 */
import { createFinancialAgreementSourceV5 } from '../../moriarty-language/src/successor/financial-agreement-source-v5.ts';
import { canonicalEncode } from '../../moriarty-language/src/codec.ts';
import { admitFinancialLifecycleStateJSON, LIFECYCLE_BOUNDS } from '../../moriarty-language/src/successor/financial-lifecycle.ts';
import { admitCustody, fenceLifecycle } from './admit-custody.mjs';
import { actionTimeGuards } from './custody-time.mjs';

export const APPROVED_COLLECTION_CAPACITY = 128;
const TRANSFER_ID = 'CapRelease';

function refuse(code, detail) {
  return { ok: false, code, detail, ledgerExecution: false, nativePins: false };
}

function snap(transferId, workInitial, window) {
  return canonicalEncode({
    Args: { actor: 'Lender', lower: window.lower, recipient: 'Lender', transferId, upper: window.upper },
    Obs: {},
    Pre: { locked: '1', revision: '0' },
    workInitial,
  });
}

function ids(n) {
  return Array.from({ length: n }, (_, i) => `U${i}`);
}

function fillers(n) {
  return Array.from({ length: n }, (_, i) => ({ amount: '0', asset: 'Cash', party: `F${i}` }));
}

function idState(seed, n) {
  const state = structuredClone(seed);
  state.usedTransferIds = ids(n);
  return state;
}

function balanceState(seed, total, includeLender) {
  const state = structuredClone(seed);
  const escrow = state.balances.find((row) => row.party === 'Escrow');
  const borrower = state.balances.find((row) => row.party === 'Borrower');
  const lender = state.balances.find((row) => row.party === 'Lender');
  const head = [structuredClone(escrow), structuredClone(borrower)];
  if (includeLender) head.push(structuredClone(lender));
  state.balances = [...head, ...fillers(total - head.length)];
  state.usedTransferIds = [];
  return state;
}

function manualTransfer(state) {
  const post = structuredClone(state);
  const amount = 100n;
  const sender = post.balances.find((row) => row.party === 'Escrow' && row.asset === 'Cash');
  sender.amount = (BigInt(sender.amount) - amount).toString(10);
  let receiver = post.balances.find((row) => row.party === 'Lender' && row.asset === 'Cash');
  if (!receiver) {
    receiver = { party: 'Lender', asset: 'Cash', amount: '0' };
    post.balances.push(receiver);
  }
  receiver.amount = (BigInt(receiver.amount) + amount).toString(10);
  const allow = post.allowances.find((row) => row.party === 'Escrow' && row.asset === 'Cash');
  allow.remaining = (BigInt(allow.remaining) - amount).toString(10);
  allow.spent = (BigInt(allow.spent) + amount).toString(10);
  post.usedTransferIds.push(TRANSFER_ID);
  post.work.remaining = (BigInt(post.work.remaining) - 49n).toString(10);
  post.work.spent = (BigInt(post.work.spent) + 49n).toString(10);
  return post;
}

function release(v5, sourceText, raw, window) {
  const text = JSON.stringify(raw);
  const admitted = admitFinancialLifecycleStateJSON(text);
  if (!admitted.ok) return { admitted: false, code: admitted.result.code, actionIndex: admitted.result.actionIndex ?? null };
  const pre = admitted.value;
  const evaluated = v5.evaluate(sourceText, 'release', snap(TRANSFER_ID, pre.work.remaining, window), JSON.stringify(pre));
  return {
    admitted: true,
    pre,
    status: evaluated.status,
    code: evaluated.code ?? null,
    actionIndex: evaluated.actionIndex ?? null,
    post: evaluated.financialPost ?? null,
    fence: fenceLifecycle(pre),
  };
}

function expectPrepared(row, balances, idLength, lenderAmount) {
  if (!row.admitted || row.status !== 'FundedExpressionPrepared' || row.code) return `status ${row.status} ${row.code}`;
  if (row.post.work.spent !== '49' || row.post.work.remaining !== '975' || row.post.work.closureReserve !== '16') return 'work';
  if (row.post.balances.length !== balances || row.post.usedTransferIds.length !== idLength) return 'lengths';
  if (row.post.usedTransferIds.at(-1) !== TRANSFER_ID) return 'transfer id';
  const lender = row.post.balances.find((item) => item.party === 'Lender' && item.asset === 'Cash');
  const escrow = row.post.balances.find((item) => item.party === 'Escrow' && item.asset === 'Cash');
  if (!lender || lender.amount !== lenderAmount || !escrow || escrow.amount !== '0') return 'amounts';
  return null;
}

export function bindCapacity({ sourceText, seed, manifest, timeBinding = null }) {
  const capacity = LIFECYCLE_BOUNDS.collectionCapacity;
  if (capacity !== APPROVED_COLLECTION_CAPACITY) return refuse('CAPACITY_BOUND', String(capacity));
  const window = actionTimeGuards(sourceText, 'release');
  if (!window.ok) return { ...window, ledgerExecution: false, nativePins: false };
  if (timeBinding != null) {
    if (timeBinding.evidenceClass !== 'UNIT/SYNTHETIC' || timeBinding.productionArtifact !== false) {
      return refuse('UNIT_PARITY_REFUSED', 'capacity time binding');
    }
    const refund = actionTimeGuards(sourceText, 'refund');
    const bounds = timeBinding.bounds;
    if (!refund.ok || !bounds || bounds.releaseLower !== window.lower || bounds.partition !== window.exclusiveUpper || bounds.refundInclusive !== refund.upper) {
      return refuse('ORACLE_DIVERGENCE', 'time binding does not match the source action guards');
    }
  }
  const v5 = createFinancialAgreementSourceV5();
  const spend = (raw) => release(v5, sourceText, raw, window);
  const ids127 = spend(idState(seed, capacity - 1));
  const ids128 = spend(idState(seed, capacity));
  const balances127 = spend(balanceState(seed, capacity - 1, false));
  const balances128 = spend(balanceState(seed, capacity, false));
  const balances128Update = spend(balanceState(seed, capacity, true));
  const prepared = [
    ['ids127', expectPrepared(ids127, 3, capacity, '100')],
    ['balances127', expectPrepared(balances127, capacity, 1, '100')],
    ['balances128Update', expectPrepared(balances128Update, capacity, 1, '100')],
  ];
  for (const [name, problem] of prepared) {
    if (problem) return refuse('ORACLE_DIVERGENCE', `${name}:${problem}`);
  }
  for (const [name, row] of [['ids128', ids128], ['balances128', balances128]]) {
    if (!row.admitted || row.status !== 'Rejected' || row.code !== 'CAPACITY' || row.actionIndex !== 0) {
      return refuse('ORACLE_DIVERGENCE', `${name}:${row.status}:${row.code}:${row.actionIndex}`);
    }
  }
  if (!ids127.fence.ok || !balances127.fence.ok) return refuse('ORACLE_DIVERGENCE', 'fence');
  const seedAdmit = (state) => admitCustody({ sourceText, seedText: JSON.stringify(state), manifest });
  const ids128Seed = seedAdmit(ids128.pre);
  const updateSeed = seedAdmit(balances128Update.pre);
  const missingSeed = seedAdmit(balances128.pre);
  if (ids128Seed.ok || ids128Seed.code !== 'USED_ID_FENCE') return refuse('ORACLE_DIVERGENCE', `ids128 seed ${ids128Seed.code}`);
  if (missingSeed.ok || missingSeed.code !== 'MISSING_BALANCE') return refuse('ORACLE_DIVERGENCE', `balances128 seed ${missingSeed.code}`);
  if (!updateSeed.ok) return refuse('ORACLE_DIVERGENCE', `balances128 update seed ${updateSeed.code}`);
  return {
    ok: true,
    capacity,
    transferId: TRANSFER_ID,
    ids127: { pre: ids127.pre, post: ids127.post },
    ids128: { pre: ids128.pre, candidate: manualTransfer(ids128.pre) },
    balances127: { pre: balances127.pre, post: balances127.post },
    balances128: { pre: balances128.pre, candidate: manualTransfer(balances128.pre) },
    balances128Update: { pre: balances128Update.pre, post: balances128Update.post },
    reachability: {
      ids128Seed: ids128Seed.code,
      balances128MissingLender: missingSeed.code,
      balances128UpdateAdmitted: true,
      repeatedIdsAreNotOneShotSeed: true,
      liveLifecycleCorrespondenceRequired: true,
    },
  };
}
