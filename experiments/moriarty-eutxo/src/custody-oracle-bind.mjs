/**
 * Bind packet19 spend envelopes to custody.spend tests.
 * Dropped source fields stay out of this renderer.
 * A source-rejected shape is not given a negative post. Bounded refusal
 * tests reuse the non-negative prestate as the datum.
 */

const LIFECYCLE = 'moriarty-financial-lifecycle-state/1';
const AMOUNT = /^(0|[1-9][0-9]*)$/;
const TAG = { release: 'Release', refund: 'Refund', cancel: 'Cancel' };
const PARTIES = new Set(['Lender', 'Borrower']);
const SOURCE_ONLY = new Set([
  'release-work-initial-mismatch',
  'release-extra-domain-field',
  'release-nonempty-obs',
  'release-leading-zero-amount',
  'e12-cancel-residual',
  'originate-not-an-action',
]);
const REQUIRED_SUCCESS = [
  'e13-release-control',
  'e11-refund-control',
  'cancel-empty-control',
  'release-missing-lender-row',
  'release-lender-balance-7',
  'refund-borrower-balance-3',
  'release-witness-balance',
  'release-lender-allowance',
  'release-prior-used-id',
  'release-alt-transfer-id',
  'release-revision-4',
  'release-work-exact-debit',
  'release-work-additive',
  'release-closure-reserve-1',
  'release-point-1000',
  'release-point-1099',
  'refund-point-1200',
  'refund-full-window',
  'cancel-full-release-window',
];

function akText(text) {
  if (typeof text !== 'string' || text.length === 0 || text.length > 80 || /["\\\n]/.test(text)) return null;
  return `"${text}"`;
}

function akInt(text) {
  return typeof text === 'string' && AMOUNT.test(text) ? text : null;
}

function firstCash(state, party) {
  const row = state?.balances?.find((item) => item.party === party && item.asset === 'Cash');
  return row ? akInt(row.amount) : null;
}

function renderRows(rows, kind) {
  return rows.map((row) => {
    const party = akText(row.party);
    const asset = akText(row.asset);
    if (!party || !asset) return null;
    if (kind === 'balance') {
      const amount = akInt(row.amount);
      if (!amount) return null;
      return `Balance { party: ${party}, asset: ${asset}, amount: ${amount} }`;
    }
    const remaining = akInt(row.remaining);
    const spent = akInt(row.spent);
    if (!remaining || !spent) return null;
    return `Allowance { party: ${party}, asset: ${asset}, remaining: ${remaining}, spent: ${spent} }`;
  });
}

function renderLife(state) {
  if (!state || !Array.isArray(state.obligations) || state.obligations.length !== 0) return null;
  for (const key of ['usedAllocationIds', 'usedOriginationIds', 'usedAccrualIds']) {
    if (!Array.isArray(state[key]) || state[key].length !== 0) return null;
  }
  const balances = renderRows(state.balances || [], 'balance');
  const allowances = renderRows(state.allowances || [], 'allowance');
  if (balances.some((row) => !row) || allowances.some((row) => !row)) return null;
  const ids = (state.usedTransferIds || []).map((id) => akText(id));
  if (ids.some((id) => !id)) return null;
  const remaining = akInt(state.work?.remaining);
  const spent = akInt(state.work?.spent);
  const reserve = akInt(state.work?.closureReserve);
  if (!remaining || !spent || !reserve) return null;
  const schema = state.schemaVersion === LIFECYCLE ? 'lifecycle_schema' : akText(state.schemaVersion);
  if (!schema) return null;
  return `LifecycleState {
    schema_version: ${schema},
    balances: [${balances.join(', ')}],
    allowances: [${allowances.join(', ')}],
    obligations: [],
    used_transfer_ids: [${ids.join(', ')}],
    used_allocation_ids: [],
    used_origination_ids: [],
    used_accrual_ids: [],
    work: Work { remaining: ${remaining}, spent: ${spent}, closure_reserve: ${reserve} },
  }`;
}

function renderHead(life, transportRevision, transportRemaining, locked, revision) {
  return `HeadDatum {
    program_hash, bounds_hash, source_hash, schema_hash, seed_hash,
    domain_network, domain_deployment, instance_id: instance_digest(script_hash, cash_policy),
    transport_revision: ${transportRevision}, transport_remaining: ${transportRemaining},
    ordinary_locked: ${locked}, ordinary_revision: ${revision},
    lifecycle: ${life},
  }`;
}

function parseState(value) {
  if (!value) return null;
  if (typeof value === 'object') return value;
  try {
    return JSON.parse(value);
  } catch {
    return null;
  }
}

export function bindOracleSpends(results) {
  const cases = results?.cases;
  if (!Array.isArray(cases)) throw new Error('ORACLE_DIVERGENCE: packet cases missing');
  const pieces = ['//// Packet19 envelopes bound to custody.spend. Source-only rows are not rendered as datums.'];
  const expectedTests = [];
  const skipped = [];
  const bound = [];
  for (const item of cases) {
    if (SOURCE_ONLY.has(item.id)) {
      skipped.push({ id: item.id, reason: 'source-or-admission layer, not a datum negative' });
      continue;
    }
    const action = TAG[item.action];
    const args = item.sourceArguments?.args;
    const preOrdinary = item.sourceArguments?.pre;
    if (!action || !args || !preOrdinary) {
      skipped.push({ id: item.id, reason: 'not a release, refund, or cancel spend' });
      continue;
    }
    const accepted = item.envelope?.status === 'FundedExpressionPrepared';
    const pre = parseState(item.financialCanonicalJSON);
    const post = accepted ? item.envelope.financialPost : pre;
    if (!pre || !post) {
      skipped.push({ id: item.id, reason: 'prestate did not parse' });
      continue;
    }
    if ((pre.obligations || []).length || (post.obligations || []).length) {
      skipped.push({ id: item.id, reason: 'obligations stay an admission fence' });
      continue;
    }
    const preLife = renderLife(pre);
    const postLife = renderLife(post);
    const actor = akText(args.actor);
    const recipient = akText(args.recipient);
    const transferId = akText(args.transferId);
    const effect = accepted ? item.envelope.effects?.[0] : null;
    const payee = accepted ? akText(effect?.to) : actor;
    const payout = accepted ? akInt(effect?.amount) : '100';
    const lower = akInt(args.lower);
    const upper = akInt(args.upper);
    const locked = akInt(preOrdinary.locked);
    const revision = akInt(preOrdinary.revision);
    const postLocked = accepted ? akInt(item.envelope.post?.locked) : locked;
    const postRevision = accepted ? akInt(item.envelope.post?.revision) : revision;
    const escrowIn = firstCash(pre, 'Escrow');
    const escrowOut = firstCash(post, 'Escrow');
    const partyOk = PARTIES.has(args.actor) && PARTIES.has(accepted ? effect?.to : args.actor);
    if (!preLife || !postLife || !actor || !recipient || !transferId || !payee || !payout || !lower || !upper || !locked || !revision || !postLocked || !postRevision || escrowIn === null || escrowOut === null || !partyOk) {
      skipped.push({ id: item.id, reason: 'not a bounded Lender or Borrower literal', sourceCode: item.envelope?.code || null });
      continue;
    }
    const uname = `oracle_${item.id.replace(/[^A-Za-z0-9_]/g, '_')}`;
    const fail = accepted ? '' : ' fail';
    const upperEx = (BigInt(upper) + 1n).toString(10);
    pieces.push(`test ${uname}()${fail} {
  let pre_head = ${renderHead(preLife, '0', '1', locked, revision)}
  let post_head = ${renderHead(postLife, '1', '0', postLocked, postRevision)}
  let (datum, redeemer, own, tx) = spend_tx(pre_head, post_head, ${action} { actor: ${actor}, recipient: ${recipient}, transfer_id: ${transferId}, lower: ${lower}, upper: ${upper} }, ${accepted ? actor : '"Lender"'}, ${payee}, ${escrowIn}, ${escrowOut}, ${payout}, ${lower}, ${upperEx})
  spend_unit(datum, redeemer, own, tx)
}`);
    expectedTests.push({ title: uname, status: 'success' });
    bound.push({ id: item.id, test: uname, accepted, sourceCode: item.envelope?.code || null });
  }
  const successIds = new Set(bound.filter((row) => row.accepted).map((row) => row.id));
  const missing = REQUIRED_SUCCESS.filter((id) => !successIds.has(id));
  if (missing.length) throw new Error(`ORACLE_DIVERGENCE: unbound ${missing.join(',')}`);
  return {
    source: pieces.join('\n'),
    expectedTests,
    bound,
    skipped,
  };
}
