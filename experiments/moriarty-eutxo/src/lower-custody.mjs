/**
 * Lower admitted financial-source/5 Core into one Aiken spend validator.
 * Interval binding and the UTXO frame are the approved custody rules.
 * Expected post datums in the tests come from the caller-supplied envelopes.
 * Debit is recounted from the Core before any validator text is rendered.
 */
import { hashDomain } from '../../moriarty-language/src/codec.ts';
import { LIFECYCLE_BOUNDS } from '../../moriarty-language/src/successor/financial-lifecycle.ts';
import { BOUNDS_RECORD, scanCore } from './admit-custody.mjs';

const SAFE_TEXT = /^[A-Za-z][A-Za-z0-9_]*$/;
const AMOUNT = /^(0|[1-9][0-9]*)$/;
const ARG = {
  actor: 'actor',
  recipient: 'recipient',
  transferId: 'transfer_id',
  lower: 'lower',
  upper: 'upper',
};
const TAG = { release: 'Release', refund: 'Refund', cancel: 'Cancel' };

class LowerError extends Error {
  constructor(code, detail) {
    super(detail ? `${code}: ${detail}` : code);
    this.code = code;
  }
}

function quote(text) {
  if (typeof text !== 'string' || !SAFE_TEXT.test(text)) throw new LowerError('EXPRESSION_UNSUPPORTED', text);
  return `"${text}"`;
}

function atom(node) {
  if (!node || typeof node.constructor !== 'string') throw new LowerError('MALFORMED_DATA');
  if (node.constructor === 'LitUInt') {
    if (node.operands.width !== '128' || !AMOUNT.test(node.operands.value)) throw new LowerError('UINT_WIDTH');
    return node.operands.value;
  }
  if (node.constructor === 'LitText') return quote(node.operands.value);
  if (node.constructor === 'ReadArg') {
    const name = ARG[node.operands.name];
    if (!name) throw new LowerError('EXPRESSION_UNSUPPORTED', node.operands.name);
    return name;
  }
  if (node.constructor === 'ReadPre') {
    const field = node.operands.field;
    if (field !== 'locked' && field !== 'revision') throw new LowerError('EXPRESSION_UNSUPPORTED', field);
    if (node.operands.view === 'pre') return `head.ordinary_${field}`;
    if (node.operands.view === 'post') return `wrote_${field}`;
    throw new LowerError('EXPRESSION_UNSUPPORTED', node.operands.view);
  }
  if (node.constructor === 'Add') return `checked_add(${atom(node.operands.left)}, ${atom(node.operands.right)})`;
  throw new LowerError('EXPRESSION_UNSUPPORTED', node.constructor);
}

function boolExpr(node) {
  const op = { Eq: '==', Lt: '<', Lte: '<=', Gt: '>', Gte: '>=' }[node?.constructor];
  if (!op) throw new LowerError('EXPRESSION_UNSUPPORTED', node?.constructor);
  return `(${atom(node.operands.left)} ${op} ${atom(node.operands.right)})`;
}

function transferFields(statement) {
  const record = statement.operands?.fields;
  if (record?.constructor !== 'ConstructRecord' || record.operands?.recordType !== 'TransferFields') {
    throw new LowerError('EFFECT_KIND_UNSUPPORTED');
  }
  const fields = {};
  for (const field of record.operands.fields) fields[field.name] = field.value;
  for (const name of ['id', 'from', 'to', 'settlementAsset', 'transferAmount']) {
    if (!fields[name]) throw new LowerError('EFFECT_KIND_UNSUPPORTED', name);
  }
  const amount = fields.transferAmount;
  if (amount.constructor !== 'ConstructAmount' || amount.operands.asset !== 'Cash') throw new LowerError('ASSET_UNSUPPORTED');
  return {
    id: atom(fields.id),
    from: atom(fields.from),
    to: atom(fields.to),
    asset: atom(fields.settlementAsset),
    amount: atom(amount.operands.value),
  };
}

function derivedDebit(action, summary) {
  const scanned = scanCore(action?.core);
  if (!scanned.ok) throw new LowerError(scanned.code || 'EXPRESSION_UNSUPPORTED', scanned.detail || summary?.action);
  if (scanned.count !== 48 || String(summary?.staticWorkBound) !== '48') {
    throw new LowerError('WORK_BOUND', `${summary?.action}:${scanned.count}:${summary?.staticWorkBound}`);
  }
  const debit = scanned.count + 1;
  if (Number(summary?.debit) !== debit) throw new LowerError('DEBIT_MISMATCH', `${summary?.debit}!=${debit}`);
  return debit;
}

function lowerAction(action, debit) {
  const lines = [];
  const writes = [];
  let amount = null;
  for (const statement of action.core.statements) {
    if (statement.constructor === 'Require' || statement.constructor === 'Ensure') {
      lines.push(`expect ${boolExpr(statement.operands.condition)}`);
    } else if (statement.constructor === 'NextWrite') {
      const field = statement.operands.field;
      if (field !== 'locked' && field !== 'revision') throw new LowerError('ORDINARY_FIELDS', field);
      lines.push(`let wrote_${field} = ${atom(statement.operands.value)}`);
      writes.push(field);
    } else if (statement.constructor === 'Emit') {
      const fields = transferFields(statement);
      amount = fields.amount;
      lines.push('expect lower_inclusive(tx.validity_range) == lower');
      lines.push('expect last_valid_posix(tx.validity_range) == upper');
      lines.push('expect list.has(tx.extra_signatories, vkh_of(actor, lender, borrower))');
      lines.push(`let moved = move_cash(head.lifecycle, ${fields.from}, ${fields.to}, ${fields.asset}, ${fields.amount}, ${fields.id}, ${debit})`);
    } else {
      throw new LowerError(statement.constructor === 'Let' ? 'LET' : 'EXPRESSION_UNSUPPORTED', statement.constructor);
    }
  }
  if (writes.join(',') !== 'locked,revision') throw new LowerError('ORDINARY_FIELDS', writes.join(','));
  if (amount === null) throw new LowerError('EFFECT_KIND_UNSUPPORTED');
  return { lines, amount };
}

function partyAmount(rows, party) {
  const row = rows.find((item) => item.party === party && item.asset === 'Cash');
  if (!row || !AMOUNT.test(row.amount)) throw new LowerError('MISSING_BALANCE', party);
  return row.amount;
}

function renderLife(state) {
  const balances = state.balances.map((row) => `Balance { party: ${quote(row.party)}, asset: ${quote(row.asset)}, amount: ${row.amount} }`).join(', ');
  const allowances = state.allowances.map((row) => `Allowance { party: ${quote(row.party)}, asset: ${quote(row.asset)}, remaining: ${row.remaining}, spent: ${row.spent} }`).join(', ');
  const ids = state.usedTransferIds.map((id) => quote(id)).join(', ');
  return `LifecycleState {
    schema_version: lifecycle_schema,
    balances: [${balances}],
    allowances: [${allowances}],
    obligations: [],
    used_transfer_ids: [${ids}],
    used_allocation_ids: [],
    used_origination_ids: [],
    used_accrual_ids: [],
    work: Work { remaining: ${state.work.remaining}, spent: ${state.work.spent}, closure_reserve: ${state.work.closureReserve} },
  }`;
}

function renderHead(name, spec, instanceExpr, withParam) {
  const sig = withParam ? `${name}(instance: ByteArray)` : `${name}()`;
  return `fn ${sig} -> HeadDatum {
  HeadDatum {
    program_hash,
    bounds_hash,
    source_hash,
    schema_hash,
    seed_hash,
    domain_network,
    domain_deployment,
    instance_id: ${instanceExpr},
    transport_revision: ${spec.transportRevision},
    transport_remaining: ${spec.transportRemaining},
    ordinary_locked: ${spec.locked},
    ordinary_revision: ${spec.revision},
    lifecycle: ${renderLife(spec.life)},
  }
}`;
}

function renderTx(name, preName, postName, tag, spec) {
  const upperExclusive = (BigInt(spec.args.upper) + 1n).toString(10);
  return `fn ${name}() -> (Option<HeadDatum>, Redeemer, OutputReference, Transaction) {
  spend_tx(${preName}(), ${postName}(), ${tag} { actor: ${quote(spec.args.actor)}, recipient: ${quote(spec.args.recipient)}, transfer_id: ${quote(spec.args.transferId)}, lower: ${spec.args.lower}, upper: ${spec.args.upper} }, ${quote(spec.args.actor)}, ${quote(spec.effect.to)}, ${partyAmount(spec.inputLife.balances, 'Escrow')}, ${partyAmount(spec.life.balances, 'Escrow')}, ${spec.effect.amount}, ${spec.args.lower}, ${upperExclusive})
}`;
}

const PRELUDE = `//// Generated from designers/02-escrow-time/escrow-custody-v5.mori.
//// Representation: moriarty-financial-agreement-source/5 Core, contract moriarty-financial-expression-contract/4.
//// Script: validator financial_head. Parameters are seed, cash, lender, borrower, genesis.
//// Ledger context: synthetic Transaction fixtures. source/UPLC-unit. ledgerExecution false. nativePins false.
//// Original guards stay 1000/1100/1200. This file is not a calibrated sibling and not a node spend.
//// Genesis lock is a trusted fixture. Private, PCD, and Complete are not verified.
use aiken/cbor.{deserialise}
use aiken/collection/list
use aiken/crypto.{sha2_256}
use aiken/interval.{Interval, IntervalBound, Finite, PositiveInfinity}
use aiken/primitive/bytearray
use cardano/address.{Address, Script, VerificationKey}
use cardano/assets.{AssetName, PolicyId, Value}
use cardano/transaction.{Datum, InlineDatum, Input, NoDatum, Output, OutputReference, Transaction}
use cardano/assets
use cardano/transaction as tx

const max_u128: Int = 340282366920938463463374607431768211455
const collection_capacity: Int = COLLECTION_CAPACITY

fn checked_add(a: Int, b: Int) -> Int {
  expect a >= 0 && a <= max_u128
  expect b >= 0 && b <= max_u128
  expect max_u128 - a >= b
  a + b
}

fn checked_sub(a: Int, b: Int) -> Int {
  expect a >= 0 && a <= max_u128
  expect b >= 0 && b <= max_u128
  expect a >= b
  a - b
}

fn last_valid_posix(range: Interval) -> Int {
  expect Finite(lower) = range.lower_bound.bound_type
  expect Finite(upper) = range.upper_bound.bound_type
  expect range.lower_bound.is_inclusive
  let last = if range.upper_bound.is_inclusive { upper } else { upper - 1 }
  expect last >= lower
  expect last >= 0
  expect last <= max_u128
  last
}

fn lower_inclusive(range: Interval) -> Int {
  expect Finite(lower) = range.lower_bound.bound_type
  expect range.lower_bound.is_inclusive
  expect lower >= 0
  lower
}

fn is_letter(b: Int) -> Bool {
  b >= 65 && b <= 90 || b >= 97 && b <= 122
}

fn is_body(b: Int) -> Bool {
  is_letter(b) || b >= 48 && b <= 57 || b == 95
}

fn identifier_from(bytes: ByteArray, i: Int, n: Int) -> Bool {
  if i >= n { True } else if is_body(bytearray.at(bytes, i)) { identifier_from(bytes, i + 1, n) } else { False }
}

fn is_identifier(bytes: ByteArray) -> Bool {
  let n = bytearray.length(bytes)
  if n < 1 || n > 64 { False } else if is_letter(bytearray.at(bytes, 0)) { identifier_from(bytes, 1, n) } else { False }
}

pub type Balance { party: ByteArray, asset: ByteArray, amount: Int }
pub type Allowance { party: ByteArray, asset: ByteArray, remaining: Int, spent: Int }
pub type Work { remaining: Int, spent: Int, closure_reserve: Int }
pub type Rounding { RoundingNone RoundingFloor RoundingCeil }
pub type AccrualRounding { AccrualFloor AccrualCeil }
pub type AllocationRule { AccrualFirst PrincipalFirst ProRata }
pub type ObligationStatus { Outstanding Settled }
pub type Conversion { mantissa: Int, scale: Int, rounding: Rounding }
pub type AccrualTerms { numerator: Int, denominator: Int, rounding: AccrualRounding, period_seconds: Int, first_period_start: Int }
pub type Obligation {
  id: ByteArray, debtor: ByteArray, creditor: ByteArray, denomination: ByteArray, settlement_asset: ByteArray,
  principal: Int, accrued: Int, outstanding: Int, allocation_rule: AllocationRule, conversion: Conversion, status: ObligationStatus,
  origination_id: ByteArray, origination_transfer_id: ByteArray, initial_principal: Int, nominal_liability_cap: Int, liability_incurred: Int,
  accrual_terms: AccrualTerms, last_accrued_period: Int, next_accrual_at: Int,
}
pub type LifecycleState {
  schema_version: ByteArray, balances: List<Balance>, allowances: List<Allowance>, obligations: List<Obligation>,
  used_transfer_ids: List<ByteArray>, used_allocation_ids: List<ByteArray>, used_origination_ids: List<ByteArray>, used_accrual_ids: List<ByteArray>, work: Work,
}
pub type HeadDatum {
  program_hash: ByteArray, bounds_hash: ByteArray, source_hash: ByteArray, schema_hash: ByteArray, seed_hash: ByteArray,
  domain_network: ByteArray, domain_deployment: ByteArray, instance_id: ByteArray,
  transport_revision: Int, transport_remaining: Int, ordinary_locked: Int, ordinary_revision: Int, lifecycle: LifecycleState,
}
pub type Redeemer {
  Release { actor: ByteArray, recipient: ByteArray, transfer_id: ByteArray, lower: Int, upper: Int }
  Refund { actor: ByteArray, recipient: ByteArray, transfer_id: ByteArray, lower: Int, upper: Int }
  Cancel { actor: ByteArray, recipient: ByteArray, transfer_id: ByteArray, lower: Int, upper: Int }
}

const lifecycle_schema: ByteArray = "moriarty-financial-lifecycle-state/1"

fn append_id(ids: List<ByteArray>, id: ByteArray) -> List<ByteArray> {
  list.reverse([id, ..list.reverse(ids)])
}

fn t1_fence(state: LifecycleState) -> Bool {
  state.schema_version == lifecycle_schema && list.is_empty(state.obligations) && list.is_empty(state.used_allocation_ids) && list.is_empty(state.used_origination_ids) && list.is_empty(state.used_accrual_ids)
}

fn pair_count_balance(rows: List<Balance>, party: ByteArray, asset: ByteArray) -> Int {
  list.foldl(rows, 0, fn(row, n) { if row.party == party && row.asset == asset { n + 1 } else { n } })
}

fn pair_count_allowance(rows: List<Allowance>, party: ByteArray, asset: ByteArray) -> Int {
  list.foldl(rows, 0, fn(row, n) { if row.party == party && row.asset == asset { n + 1 } else { n } })
}

fn balances_injective(rows: List<Balance>) -> Bool {
  list.foldl(rows, True, fn(row, ok) { ok && pair_count_balance(rows, row.party, row.asset) == 1 })
}

fn allowances_injective(rows: List<Allowance>) -> Bool {
  list.foldl(rows, True, fn(row, ok) { ok && pair_count_allowance(rows, row.party, row.asset) == 1 })
}

fn move_cash(life: LifecycleState, from: ByteArray, to: ByteArray, asset: ByteArray, amount: Int, transfer_id: ByteArray, debit: Int) -> LifecycleState {
  expect t1_fence(life)
  expect is_identifier(transfer_id)
  expect amount > 0
  expect from != to
  expect !list.has(life.used_transfer_ids, transfer_id)
  expect balances_injective(life.balances)
  expect allowances_injective(life.allowances)
  expect pair_count_balance(life.balances, from, asset) == 1
  expect pair_count_allowance(life.allowances, from, asset) == 1
  let receiver_count = pair_count_balance(life.balances, to, asset)
  expect receiver_count < 2
  let balance_capacity_ok = if receiver_count == 0 {
    expect list.length(life.balances) < collection_capacity
    True
  } else {
    True
  }
  expect balance_capacity_ok
  expect list.length(life.used_transfer_ids) < collection_capacity
  expect Some(sender) = list.find(life.balances, fn(row) { row.party == from && row.asset == asset })
  expect Some(allow) = list.find(life.allowances, fn(row) { row.party == from && row.asset == asset })
  expect sender.amount >= amount
  expect allow.remaining >= amount
  let sender_next = checked_sub(sender.amount, amount)
  let prior_receiver = if receiver_count == 0 {
    0
  } else {
    expect Some(found) = list.find(life.balances, fn(row) { row.party == to && row.asset == asset })
    found.amount
  }
  let receiver_next = checked_add(prior_receiver, amount)
  let remaining_next = checked_sub(allow.remaining, amount)
  let spent_next = checked_add(allow.spent, amount)
  expect life.work.remaining >= debit
  let updated = list.map(life.balances, fn(row) {
    if row.party == from && row.asset == asset { Balance { ..row, amount: sender_next } } else { row }
  })
  let balances = if receiver_count == 0 {
    list.reverse([Balance { party: to, asset, amount: receiver_next }, ..list.reverse(updated)])
  } else {
    list.map(updated, fn(row) {
      if row.party == to && row.asset == asset { Balance { ..row, amount: receiver_next } } else { row }
    })
  }
  let allowances = list.map(life.allowances, fn(row) {
    if row.party == from && row.asset == asset { Allowance { ..row, remaining: remaining_next, spent: spent_next } } else { row }
  })
  LifecycleState {
    schema_version: life.schema_version,
    balances,
    allowances,
    obligations: life.obligations,
    used_transfer_ids: append_id(life.used_transfer_ids, transfer_id),
    used_allocation_ids: life.used_allocation_ids,
    used_origination_ids: life.used_origination_ids,
    used_accrual_ids: life.used_accrual_ids,
    work: Work { remaining: checked_sub(life.work.remaining, debit), spent: checked_add(life.work.spent, debit), closure_reserve: life.work.closure_reserve },
  }
}
`;

const FRAME = `
fn join(parts: List<ByteArray>) -> ByteArray {
  list.foldl(parts, #"", fn(part, acc) { bytearray.concat(acc, part) })
}

fn lower_from(bytes: ByteArray, i: Int, acc: ByteArray) -> ByteArray {
  if i < 0 {
    acc
  } else {
    let b = bytearray.at(bytes, i)
    let c = if b >= 65 && b <= 70 { b + 32 } else { b }
    lower_from(bytes, i - 1, bytearray.push(acc, c))
  }
}

fn lower_hex(bytes: ByteArray) -> ByteArray {
  lower_from(bytes, bytearray.length(bytes) - 1, #"")
}

fn hex_ascii(bytes: ByteArray) -> ByteArray {
  lower_hex(bytearray.from_string(bytearray.to_hex(bytes)))
}

fn digits(n: Int, acc: ByteArray) -> ByteArray {
  if n == 0 { acc } else { digits(n / 10, bytearray.push(acc, 48 + (n % 10))) }
}

fn decimal(n: Int) -> ByteArray {
  if n == 0 { "0" } else { digits(n, #"") }
}

pub fn instance_preimage(thread_policy: ByteArray, cash: ByteArray) -> ByteArray {
  join([
    instance_domain,
    #"00",
    "{\\"cashAssetNameHex\\":\\"",
    hex_ascii(cash_name),
    "\\",\\"cashPolicyId\\":\\"",
    hex_ascii(cash),
    "\\",\\"domainDeployment\\":\\"",
    domain_deployment,
    "\\",\\"domainNetwork\\":\\"",
    domain_network,
    "\\",\\"threadAssetNameHex\\":\\"",
    hex_ascii(thread_name),
    "\\",\\"threadPolicyId\\":\\"",
    hex_ascii(thread_policy),
    "\\"}",
  ])
}

pub fn instance_digest(thread_policy: ByteArray, cash: ByteArray) -> ByteArray {
  sha2_256(instance_preimage(thread_policy, cash))
}

fn instance_clean(digest: ByteArray) -> Bool {
  bytearray.length(digest) == 32 && digest != zero32 && digest != shape_instance
}

pub fn genesis_preimage(seed: OutputReference, cash: ByteArray, lender: ByteArray, borrower: ByteArray) -> ByteArray {
  expect bytearray.length(seed.transaction_id) == 32
  expect seed.output_index >= 0 && seed.output_index <= max_u128
  join([
    genesis_domain,
    #"00",
    "{\\"borrowerKeyHash\\":\\"",
    hex_ascii(borrower),
    "\\",\\"boundsHash\\":\\"",
    hex_ascii(bounds_hash),
    "\\",\\"cashAssetNameHex\\":\\"",
    hex_ascii(cash_name),
    "\\",\\"cashPolicyId\\":\\"",
    hex_ascii(cash),
    "\\",\\"domainDeployment\\":\\"",
    domain_deployment,
    "\\",\\"domainNetwork\\":\\"",
    domain_network,
    "\\",\\"lenderKeyHash\\":\\"",
    hex_ascii(lender),
    "\\",\\"programHash\\":\\"",
    hex_ascii(program_hash),
    "\\",\\"schemaHash\\":\\"",
    hex_ascii(schema_hash),
    "\\",\\"seedHash\\":\\"",
    hex_ascii(seed_hash),
    "\\",\\"seedOutRefIndex\\":\\"",
    decimal(seed.output_index),
    "\\",\\"seedOutRefTxId\\":\\"",
    hex_ascii(seed.transaction_id),
    "\\",\\"sourceHash\\":\\"",
    hex_ascii(source_hash),
    "\\",\\"threadAssetNameHex\\":\\"",
    hex_ascii(thread_name),
    "\\",\\"version\\":\\"",
    genesis_version,
    "\\"}",
  ])
}

pub fn genesis_digest(seed: OutputReference, cash: ByteArray, lender: ByteArray, borrower: ByteArray) -> ByteArray {
  sha2_256(genesis_preimage(seed, cash, lender, borrower))
}

fn role_clean(id: ByteArray) -> Bool {
  bytearray.length(id) == 28 && id != zero28 && id != shape_a && id != shape_b && id != shape_c && id != shape_d && id != shape_e
}

pub fn roles_ok(own: ByteArray, cash: ByteArray, lender: ByteArray, borrower: ByteArray) -> Bool {
  role_clean(own) && role_clean(cash) && role_clean(lender) && role_clean(borrower) && cash != own && lender != own && borrower != own && lender != borrower && lender != cash && borrower != cash
}

fn bound_identity(head: HeadDatum, own: ByteArray, cash: ByteArray) -> Bool {
  let digest = instance_digest(own, cash)
  instance_clean(digest) && head.program_hash == program_hash && head.bounds_hash == bounds_hash && head.source_hash == source_hash && head.schema_hash == schema_hash && head.seed_hash == seed_hash && head.domain_network == domain_network && head.domain_deployment == domain_deployment && head.instance_id == digest
}

fn vkh_of(party: ByteArray, lender: ByteArray, borrower: ByteArray) -> ByteArray {
  if party == "Lender" { lender } else { expect party == "Borrower" borrower }
}

fn thread_qty(value: Value, own: ByteArray) -> Int { assets.quantity_of(value, own, thread_name) }
fn cash_qty(value: Value, cash: ByteArray) -> Int { assets.quantity_of(value, cash, cash_name) }

fn escrow_cash(life: LifecycleState) -> Int {
  expect Some(found) = list.find(life.balances, fn(row) { row.party == "Escrow" && row.asset == "Cash" })
  found.amount
}

fn script_inputs(inputs: List<Input>, own: ByteArray) -> List<Input> {
  list.filter(inputs, fn(input) { when input.output.address.payment_credential is { Script(hash) -> hash == own _ -> False } })
}

fn script_outputs(outputs: List<Output>, own: ByteArray) -> List<Output> {
  list.filter(outputs, fn(output) { when output.address.payment_credential is { Script(hash) -> hash == own _ -> False } })
}

fn non_script(outputs: List<Output>) -> List<Output> {
  list.filter(outputs, fn(output) { when output.address.payment_credential is { Script(_) -> False _ -> True } })
}

fn asset_admitted(policy: PolicyId, name: AssetName, qty: Int, thread_q: Int, cash_q: Int, own: ByteArray, cash: ByteArray) -> Bool {
  if policy == #"" && name == #"" {
    True
  } else if policy == own && name == thread_name {
    qty == thread_q
  } else if policy == cash && name == cash_name {
    qty == cash_q
  } else {
    False
  }
}

fn admitted_value(value: Value, thread_q: Int, cash_q: Int, own: ByteArray, cash: ByteArray) -> Bool {
  assets.quantity_of(value, own, thread_name) == thread_q && assets.quantity_of(value, cash, cash_name) == cash_q && list.foldl(assets.flatten(value), True, fn(item, ok) {
    let (policy, name, qty) = item
    ok && asset_admitted(policy, name, qty, thread_q, cash_q, own, cash)
  })
}

fn expect_frame(head: HeadDatum, post: HeadDatum, self: Transaction, own_ref: OutputReference, recipient: ByteArray, amount: Int, own: ByteArray, cash: ByteArray, lender: ByteArray, borrower: ByteArray) -> Bool {
  expect [only] = script_inputs(self.inputs, own)
  expect only.output_reference == own_ref
  expect admitted_value(only.output.value, 1, escrow_cash(head.lifecycle), own, cash)
  expect InlineDatum(input_raw) = only.output.datum
  expect input_raw: HeadDatum = input_raw
  expect input_raw == head
  expect [cont] = script_outputs(self.outputs, own)
  expect cont.address == only.output.address
  expect admitted_value(cont.value, 1, escrow_cash(post.lifecycle), own, cash)
  expect assets.lovelace_of(cont.value) == assets.lovelace_of(only.output.value)
  expect InlineDatum(raw) = cont.datum
  expect raw: HeadDatum = raw
  expect raw == post
  let others = non_script(self.outputs)
  let paid = list.filter(others, fn(output) { cash_qty(output.value, cash) > 0 })
  expect [pay] = paid
  expect cash_qty(pay.value, cash) == amount
  expect thread_qty(pay.value, own) == 0
  expect assets.lovelace_of(pay.value) == unit_payout_sponsorship
  expect VerificationKey(payee) = pay.address.payment_credential
  expect payee == vkh_of(recipient, lender, borrower)
  expect list.foldl(others, True, fn(output, acc) { acc && admitted_value(output.value, 0, cash_qty(output.value, cash), own, cash) })
  expect list.foldl(self.inputs, 0, fn(input, acc) { acc + cash_qty(input.output.value, cash) }) == list.foldl(self.outputs, 0, fn(output, acc) { acc + cash_qty(output.value, cash) })
  expect assets.is_zero(self.mint)
  True
}

fn count_ref(inputs: List<Input>, seed: OutputReference) -> Int {
  list.foldl(inputs, 0, fn(input, n) { if input.output_reference == seed { n + 1 } else { n } })
}

fn seed_consumed(self: Transaction, seed: OutputReference) -> Bool {
  count_ref(self.inputs, seed) == 1 && count_ref(self.reference_inputs, seed) == 0
}

fn mint_exact(mint: Value, policy: ByteArray) -> Bool {
  assets.lovelace_of(mint) == 0 && assets.quantity_of(mint, policy, thread_name) == 1 && list.foldl(assets.flatten(mint), True, fn(item, ok) {
    let (p, name, qty) = item
    ok && p == policy && name == thread_name && qty == 1
  })
}

fn other_outputs_lovelace_only(outputs: List<Output>, policy: ByteArray, cash: ByteArray) -> Bool {
  let rest = list.filter(outputs, fn(output) {
    when output.address.payment_credential is {
      Script(hash) -> hash != policy
      _ -> True
    }
  })
  list.foldl(rest, True, fn(output, ok) { ok && admitted_value(output.value, 0, 0, policy, cash) })
}

fn own_id(self: Transaction, own_ref: OutputReference) -> ByteArray {
  expect Some(spent) = list.find(self.inputs, fn(input) { input.output_reference == own_ref })
  expect Script(hash) = spent.output.address.payment_credential
  hash
}
`;

const FIXTURES = `
fn script_addr() -> Address { Address { payment_credential: Script(script_hash), stake_credential: None } }
fn vkh_addr(vkh: ByteArray) -> Address { Address { payment_credential: VerificationKey(vkh), stake_credential: None } }

fn valued(addr: Address, with_thread: Bool, cash: Int, lovelace: Int, datum: Datum) -> Output {
  let ada = assets.from_lovelace(lovelace)
  let threaded = if with_thread { assets.merge(ada, assets.from_asset(thread_policy, thread_name, 1)) } else { ada }
  let value = if cash > 0 { assets.merge(threaded, assets.from_asset(cash_policy, cash_name, cash)) } else { threaded }
  Output { address: addr, value, datum, reference_script: None }
}

fn finite_range(lower: Int, upper_exclusive: Int) -> Interval {
  Interval { lower_bound: IntervalBound { bound_type: Finite(lower), is_inclusive: True }, upper_bound: IntervalBound { bound_type: Finite(upper_exclusive), is_inclusive: False } }
}

fn ref_at(i: Int) -> OutputReference { OutputReference { transaction_id: tx.placeholder.id, output_index: i } }
fn own_ref() -> OutputReference { ref_at(0) }

fn base_tx(inputs: List<Input>, outputs: List<Output>, signers: List<ByteArray>, range: Interval) -> Transaction {
  Transaction { ..tx.placeholder, inputs, outputs, extra_signatories: signers, validity_range: range }
}

fn spend_tx(pre: HeadDatum, post: HeadDatum, redeemer: Redeemer, actor: ByteArray, recipient: ByteArray, input_cash: Int, output_cash: Int, payout: Int, lower: Int, upper_exclusive: Int) -> (Option<HeadDatum>, Redeemer, OutputReference, Transaction) {
  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, input_cash, 2_000_000, InlineDatum(pre)) }
  let fee = Input { output_reference: ref_at(1), output: valued(vkh_addr(fee_vkh), False, 0, 3_000_000, NoDatum) }
  let collateral = Input { output_reference: ref_at(2), output: valued(vkh_addr(collateral_vkh), False, 0, 5_000_000, NoDatum) }
  let cont = valued(script_addr(), True, output_cash, 2_000_000, InlineDatum(post))
  let pay = valued(vkh_addr(vkh_of(recipient, lender_vkh, borrower_vkh)), False, payout, unit_payout_sponsorship, NoDatum)
  let built = base_tx([own, fee, collateral], [cont, pay], [vkh_of(actor, lender_vkh, borrower_vkh)], finite_range(lower, upper_exclusive))
  (Some(pre), redeemer, own_ref(), built)
}

fn hostile_obligation() -> Obligation {
  Obligation {
    id: "BadOb", debtor: "Borrower", creditor: "Lender", denomination: "Cash", settlement_asset: "Cash",
    principal: 40, accrued: 0, outstanding: 40, allocation_rule: AccrualFirst,
    conversion: Conversion { mantissa: 1, scale: 0, rounding: RoundingNone }, status: Outstanding,
    origination_id: "PriorOrigination", origination_transfer_id: "PriorTransfer", initial_principal: 40,
    nominal_liability_cap: 40, liability_incurred: 40,
    accrual_terms: AccrualTerms { numerator: 0, denominator: 1, rounding: AccrualFloor, period_seconds: 60, first_period_start: 1000 },
    last_accrued_period: 0, next_accrual_at: 1060,
  }
}
`;

const E13_PRE = 'd8798d582093ed829602f6a554d3f47879ee220c7382140045250bf53b21470ee6091297b05820b3d02ace7a7ac7daec45318f2814beff4d27ad26f91d45405942c30d04fa846d5820facb73a98d78c4a8839e6aa05af30a7702e15798d3d7ebb15e4eb1a4b78bc73658202254f3ccbe5494a83bef188de808473f64b220b4f2f1178976916baa7d335b4c5820fc93bf4a17f0fb0a7dd70761e36a3b903bf66843d7beea29a210329d5d1c327f50796163692d69736f6c617465642d74315574312d637573746f64792d7265766973696f6e30335820f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d400010100d8798958246d6f7269617274792d66696e616e6369616c2d6c6966656379636c652d73746174652f3183d8798346457363726f7744436173681864d8798348426f72726f776572444361736800d87983464c656e64657244436173680083d8798446457363726f774443617368186400d8798448426f72726f77657244436173680000d87984464c656e646572444361736800008080808080d879831904000010';
const E13_POST = 'd8798d582093ed829602f6a554d3f47879ee220c7382140045250bf53b21470ee6091297b05820b3d02ace7a7ac7daec45318f2814beff4d27ad26f91d45405942c30d04fa846d5820facb73a98d78c4a8839e6aa05af30a7702e15798d3d7ebb15e4eb1a4b78bc73658202254f3ccbe5494a83bef188de808473f64b220b4f2f1178976916baa7d335b4c5820fc93bf4a17f0fb0a7dd70761e36a3b903bf66843d7beea29a210329d5d1c327f50796163692d69736f6c617465642d74315574312d637573746f64792d7265766973696f6e30335820f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d401000001d8798958246d6f7269617274792d66696e616e6369616c2d6c6966656379636c652d73746174652f3183d8798346457363726f77444361736800d8798348426f72726f776572444361736800d87983464c656e6465724443617368186483d8798446457363726f774443617368001864d8798448426f72726f77657244436173680000d87984464c656e6465724443617368000080814852656c6561736531808080d879831903cf183110';

function renderTests(releaseRemaining) {
  const mutated = (BigInt(releaseRemaining) - 1n).toString(10);
  return `
const e13_pre: ByteArray = #"${E13_PRE}"
const e13_post: ByteArray = #"${E13_POST}"
const lucid_definite_balance: ByteArray = #"d8798346457363726f7744436173681864"
const lucid_definite_release: ByteArray = #"d87985464c656e646572464c656e6465724852656c65617365311903e819044b"

fn head_fields(decoded: HeadDatum, transport_revision: Int, transport_remaining: Int, ordinary_locked: Int, ordinary_revision: Int, work_remaining: Int, work_spent: Int) -> Bool {
  expect Some(escrow) = list.at(decoded.lifecycle.balances, 0)
  expect Some(lender) = list.at(decoded.lifecycle.balances, 2)
  decoded.domain_network == "yaci-isolated-t1" && decoded.domain_deployment == "t1-custody-revision03" && decoded.transport_revision == transport_revision && decoded.transport_remaining == transport_remaining && decoded.ordinary_locked == ordinary_locked && decoded.ordinary_revision == ordinary_revision && decoded.lifecycle.work.remaining == work_remaining && decoded.lifecycle.work.spent == work_spent && decoded.lifecycle.work.closure_reserve == 16 && escrow.party == "Escrow" && lender.party == "Lender" && list.is_empty(decoded.lifecycle.obligations)
}

test decode_lucid_definite_e13_pre() {
  expect Some(data) = deserialise(e13_pre)
  expect decoded: HeadDatum = data
  expect Some(escrow) = list.at(decoded.lifecycle.balances, 0)
  head_fields(decoded, 0, 1, 1, 0, 1024, 0) && escrow.amount == 100 && list.is_empty(decoded.lifecycle.used_transfer_ids)
}

test decode_lucid_definite_e13_post() {
  expect Some(data) = deserialise(e13_post)
  expect decoded: HeadDatum = data
  expect Some(escrow) = list.at(decoded.lifecycle.balances, 0)
  expect Some(lender) = list.at(decoded.lifecycle.balances, 2)
  expect Some(id) = list.at(decoded.lifecycle.used_transfer_ids, 0)
  head_fields(decoded, 1, 0, 0, 1, 975, 49) && escrow.amount == 0 && lender.amount == 100 && id == "Release1"
}

test decode_lucid_definite_balance() {
  expect Some(data) = deserialise(lucid_definite_balance)
  expect decoded: Balance = data
  decoded.party == "Escrow" && decoded.asset == "Cash" && decoded.amount == 100
}

test decode_lucid_definite_release() {
  expect Some(data) = deserialise(lucid_definite_release)
  expect decoded: Redeemer = data
  expect Release { actor, recipient, transfer_id, lower, upper } = decoded
  actor == "Lender" && recipient == "Lender" && transfer_id == "Release1" && lower == 1000 && upper == 1099
}

test frozen_shape_sample_instance_differs() {
  expect Some(data) = deserialise(e13_pre)
  expect decoded: HeadDatum = data
  decoded.instance_id != instance_digest(script_hash, cash_policy) && decoded.program_hash == program_hash
}

test source_uplc_unit_release() {
  let (d, r, o, t) = release_tx()
  spend_unit(d, r, o, t)
}

test source_uplc_unit_refund() {
  let (d, r, o, t) = refund_tx()
  spend_unit(d, r, o, t)
}

test source_uplc_unit_cancel_empty() {
  let (d, r, o, t) = cancel_tx()
  spend_unit(d, r, o, t)
}

test nonempty_obligations() fail {
  let (_d, r, o, t) = release_tx()
  let base = seed_head()
  let life = base.lifecycle
  let bad = HeadDatum { ..base, lifecycle: LifecycleState { ..life, obligations: [hostile_obligation()] } }
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(bad) } }
  spend_unit(Some(bad), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test n01_refund_args_wider_interval() fail {
  let (d, r, o, t) = refund_tx()
  spend_unit(d, r, o, Transaction { ..t, validity_range: finite_range(1100, 1201) })
}

test missing_signatory() fail {
  let (d, r, o, t) = release_tx()
  spend_unit(d, r, o, Transaction { ..t, extra_signatories: [] })
}

test wrong_recipient_payout() fail {
  let (d, r, o, t) = release_tx()
  expect [cont, pay] = t.outputs
  let stolen = Output { ..pay, address: vkh_addr(borrower_vkh) }
  spend_unit(d, r, o, Transaction { ..t, outputs: [cont, stolen] })
}

test shared_payout() fail {
  let (d, r, o, t) = release_tx()
  expect [cont, pay] = t.outputs
  spend_unit(d, r, o, Transaction { ..t, outputs: [cont, pay, pay] })
}

test duplicate_transfer_id() fail {
  let (_d, r, o, t) = release_tx()
  let base = seed_head()
  let life = base.lifecycle
  let bad = HeadDatum { ..base, lifecycle: LifecycleState { ..life, used_transfer_ids: ["Release1"] } }
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(bad) } }
  spend_unit(Some(bad), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test mutated_work() fail {
  let (d, r, o, t) = release_tx()
  let base = release_post()
  let life = base.lifecycle
  let work = life.work
  let bad = HeadDatum { ..base, lifecycle: LifecycleState { ..life, work: Work { ..work, remaining: ${mutated} } } }
  expect [_cont, pay] = t.outputs
  let output = valued(script_addr(), True, 0, 2_000_000, InlineDatum(bad))
  spend_unit(d, r, o, Transaction { ..t, outputs: [output, pay] })
}

test stale_bounds_hash() fail {
  let (_d, r, o, t) = release_tx()
  let base = seed_head()
  let bad = HeadDatum { ..base, bounds_hash: #"1111111111111111111111111111111111111111111111111111111111111111" }
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(bad) } }
  spend_unit(Some(bad), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test schema_version() fail {
  let (_d, r, o, t) = release_tx()
  let base = seed_head()
  let life = base.lifecycle
  let bad = HeadDatum { ..base, lifecycle: LifecycleState { ..life, schema_version: "not-the-lifecycle" } }
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(bad) } }
  spend_unit(Some(bad), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test seed_cash_utxo_mismatch() fail {
  let (d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let bad = Input { output_reference: own.output_reference, output: valued(script_addr(), True, 50, 2_000_000, own.output.datum) }
  spend_unit(d, r, o, Transaction { ..t, inputs: [bad, fee, collateral] })
}

test wrong_actor_redeemer() fail {
  let (d, r, o, t) = release_tx()
  expect Release { actor: _actor, recipient, transfer_id, lower, upper } = r
  spend_unit(d, Release { actor: "Borrower", recipient, transfer_id, lower, upper }, o, t)
}

test mint_nonzero() fail {
  let (d, r, o, t) = release_tx()
  spend_unit(d, r, o, Transaction { ..t, mint: assets.from_asset(thread_policy, thread_name, 1) })
}

test second_script_input() fail {
  let (d, r, o, t) = release_tx()
  let extra = Input { output_reference: ref_at(3), output: valued(script_addr(), True, 0, 2_000_000, InlineDatum(seed_head())) }
  spend_unit(d, r, o, Transaction { ..t, inputs: list.push(t.inputs, extra) })
}

test replay_used_id() fail {
  let base = release_post()
  let replay = HeadDatum { ..base, ordinary_locked: 1 }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: valued(script_addr(), True, 0, 2_000_000, InlineDatum(replay)) }
  spend_unit(Some(replay), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test wrong_transport() fail {
  let (d, r, o, t) = release_tx()
  let base = release_post()
  let bad = HeadDatum { ..base, transport_revision: 0, transport_remaining: 1 }
  expect [_cont, pay] = t.outputs
  let output = valued(script_addr(), True, 0, 2_000_000, InlineDatum(bad))
  spend_unit(d, r, o, Transaction { ..t, outputs: [output, pay] })
}

test sponsorship_moved() fail {
  let (d, r, o, t) = release_tx()
  expect [cont, pay] = t.outputs
  let output = valued(script_addr(), True, 0, 1_000_000, cont.datum)
  spend_unit(d, r, o, Transaction { ..t, outputs: [output, pay] })
}

test frozen_shape_sample_head_not_spent() fail {
  expect Some(data) = deserialise(e13_pre)
  expect decoded: HeadDatum = data
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(decoded) } }
  spend_unit(Some(decoded), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test release_missing_lender_row() {
  let pins = seed_head()
  let life = pins.lifecycle
  expect [escrow, borrower, _lender] = life.balances
  let pre = HeadDatum { ..pins, lifecycle: LifecycleState { ..life, balances: [escrow, borrower] } }
  let post = release_post()
  let (d, r, o, t) = spend_tx(pre, post, Release { actor: "Lender", recipient: "Lender", transfer_id: "Release1", lower: 1000, upper: 1099 }, "Lender", "Lender", 100, 0, 100, 1000, 1100)
  spend_unit(d, r, o, t)
}

test generic_absent_party_and_asset_appends() {
  let pins = seed_head()
  let life = pins.lifecycle
  expect [_escrow, borrower, lender] = life.balances
  expect [_escrow_allow, borrower_allow, lender_allow] = life.allowances
  let pre = LifecycleState {
    ..life,
    balances: [Balance { party: "Escrow", asset: "Bond", amount: 40 }, borrower, lender],
    allowances: [Allowance { party: "Escrow", asset: "Bond", remaining: 40, spent: 0 }, borrower_allow, lender_allow],
  }
  let post = move_cash(pre, "Escrow", "Witness", "Bond", 40, "Release1", 49)
  expect [sender, kept_borrower, kept_lender, created] = post.balances
  expect [allow_sender, allow_borrower, allow_lender] = post.allowances
  sender.party == "Escrow" && sender.asset == "Bond" && sender.amount == 0 && kept_borrower == borrower && kept_lender == lender && created.party == "Witness" && created.asset == "Bond" && created.amount == 40 && allow_sender.party == "Escrow" && allow_sender.asset == "Bond" && allow_sender.remaining == 0 && allow_sender.spent == 40 && allow_borrower == borrower_allow && allow_lender == lender_allow && post.used_transfer_ids == ["Release1"] && post.work.remaining == 975 && post.work.spent == 49 && post.work.closure_reserve == 16 && post.schema_version == life.schema_version && post.obligations == life.obligations
}

test hostile_duplicate_lender_rows_do_not_inflate() fail {
  let pins = seed_head()
  let life = pins.lifecycle
  expect [escrow, borrower, lender] = life.balances
  let pre = HeadDatum { ..pins, lifecycle: LifecycleState { ..life, balances: [escrow, borrower, lender, lender] } }
  let zero_escrow = Balance { ..escrow, amount: 0 }
  let credited = Balance { ..lender, amount: 100 }
  expect [escrow_allow, borrower_allow, lender_allow] = life.allowances
  let post_life = LifecycleState {
    ..life,
    balances: [zero_escrow, borrower, credited, credited],
    allowances: [Allowance { ..escrow_allow, remaining: 0, spent: 100 }, borrower_allow, lender_allow],
    used_transfer_ids: ["Release1"],
    work: Work { remaining: 975, spent: 49, closure_reserve: 16 },
  }
  let post = HeadDatum { ..pins, transport_revision: 1, transport_remaining: 0, ordinary_locked: 0, ordinary_revision: 1, lifecycle: post_life }
  let (d, r, o, t) = spend_tx(pre, post, Release { actor: "Lender", recipient: "Lender", transfer_id: "Release1", lower: 1000, upper: 1099 }, "Lender", "Lender", 100, 0, 100, 1000, 1100)
  spend_unit(d, r, o, t)
}

test oracle_noninjective_escrow_rows() fail {
  let base = seed_head()
  let life = base.lifecycle
  expect [escrow, borrower, lender] = life.balances
  let dup = HeadDatum { ..base, lifecycle: LifecycleState { ..life, balances: [escrow, escrow, borrower, lender] } }
  let posted = release_post()
  let post_life = posted.lifecycle
  expect [escrow0, borrower0, lender100] = post_life.balances
  let post = HeadDatum { ..posted, lifecycle: LifecycleState { ..post_life, balances: [escrow0, escrow0, borrower0, lender100] } }
  let (_datum, redeemer, own, tx) = release_tx()
  expect [script_in, fee, collateral] = tx.inputs
  expect [_cont, pay] = tx.outputs
  let input = Input { output_reference: script_in.output_reference, output: valued(script_addr(), True, 100, 2_000_000, InlineDatum(dup)) }
  let output = valued(script_addr(), True, 0, 2_000_000, InlineDatum(post))
  spend_unit(Some(dup), redeemer, own, Transaction { ..tx, inputs: [input, fee, collateral], outputs: [output, pay] })
}

test duplicate_sender_allowance_refused() fail {
  let pins = seed_head()
  let life = pins.lifecycle
  expect [escrow_allow, borrower_allow, lender_allow] = life.allowances
  let pre = HeadDatum { ..pins, lifecycle: LifecycleState { ..life, allowances: [escrow_allow, escrow_allow, borrower_allow, lender_allow] } }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(pre) } }
  spend_unit(Some(pre), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test duplicate_untouched_balance_refused() fail {
  let pins = seed_head()
  let life = pins.lifecycle
  let witness = Balance { party: "Witness", asset: "Cash", amount: 5 }
  let pre = HeadDatum { ..pins, lifecycle: LifecycleState { ..life, balances: [witness, witness, ..life.balances] } }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(pre) } }
  spend_unit(Some(pre), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test hostile_other_asset_dropped() fail {
  let (d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let extra = assets.from_asset(#"33333333333333333333333333333333333333333333333333333333", #"5a5a", 7)
  let bad = Input { output_reference: own.output_reference, output: Output { ..own.output, value: assets.merge(own.output.value, extra) } }
  spend_unit(d, r, o, Transaction { ..t, inputs: [bad, fee, collateral] })
}

test oracle_foreign_asset_on_continuation() fail {
  let (datum, redeemer, own, tx) = release_tx()
  expect [cont, pay] = tx.outputs
  let extra = assets.from_asset(#"33333333333333333333333333333333333333333333333333333333", #"99", 1)
  let tainted = Output { ..cont, value: assets.merge(cont.value, extra) }
  spend_unit(datum, redeemer, own, Transaction { ..tx, outputs: [tainted, pay] })
}

test oracle_foreign_asset_on_payout() fail {
  let (datum, redeemer, own, tx) = release_tx()
  expect [cont, pay] = tx.outputs
  let extra = assets.from_asset(#"33333333333333333333333333333333333333333333333333333333", #"99", 1)
  let tainted = Output { ..pay, value: assets.merge(pay.value, extra) }
  spend_unit(datum, redeemer, own, Transaction { ..tx, outputs: [cont, tainted] })
}

test oracle_foreign_mint() fail {
  let (datum, redeemer, own, tx) = release_tx()
  let extra = assets.from_asset(#"33333333333333333333333333333333333333333333333333333333", #"99", 1)
  spend_unit(datum, redeemer, own, Transaction { ..tx, mint: extra })
}

test unadmitted_name_on_admitted_cash_policy() fail {
  let (d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let extra = assets.from_asset(cash_policy, #"99", 1)
  let bad = Input { output_reference: own.output_reference, output: Output { ..own.output, value: assets.merge(own.output.value, extra) } }
  spend_unit(d, r, o, Transaction { ..t, inputs: [bad, fee, collateral] })
}

test unadmitted_name_on_admitted_thread_policy() fail {
  let (d, r, o, t) = release_tx()
  expect [cont, pay] = t.outputs
  let extra = assets.from_asset(thread_policy, #"99", 1)
  let tainted = Output { ..cont, value: assets.merge(cont.value, extra) }
  spend_unit(d, r, o, Transaction { ..t, outputs: [tainted, pay] })
}

test unadmitted_mint_other_name() fail {
  let (d, r, o, t) = release_tx()
  spend_unit(d, r, o, Transaction { ..t, mint: assets.from_asset(cash_policy, #"99", 1) })
}

test hostile_change_lovelace_merged_into_payout() fail {
  let (d, r, o, t) = release_tx()
  expect [cont, pay] = t.outputs
  let merged = Output { ..pay, value: assets.merge(assets.from_lovelace(9_000_000), assets.from_asset(cash_policy, cash_name, 100)) }
  spend_unit(d, r, o, Transaction { ..t, outputs: [cont, merged] })
}

test hostile_separate_lovelace_change_output() {
  let (d, r, o, t) = release_tx()
  expect [cont, pay] = t.outputs
  let change = valued(vkh_addr(fee_vkh), False, 0, 1_500_000, NoDatum)
  spend_unit(d, r, o, Transaction { ..t, outputs: [cont, pay, change] })
}

test oracle_lovelace_change_is_not_kernel_cash() {
  let (datum, redeemer, own, tx) = release_tx()
  expect [cont, pay] = tx.outputs
  let change = valued(vkh_addr(fee_vkh), False, 0, 1_500_000, NoDatum)
  spend_unit(datum, redeemer, own, Transaction { ..tx, outputs: [cont, pay, change] })
}

test oracle_fee_lovelace_does_not_enter_kernel() {
  let (datum, redeemer, own, tx) = release_tx()
  expect [script_in, fee, collateral] = tx.inputs
  let fee2 = Input { output_reference: fee.output_reference, output: valued(fee.output.address, False, 0, 9_000_000, NoDatum) }
  spend_unit(datum, redeemer, own, Transaction { ..tx, inputs: [script_in, fee2, collateral] })
}

test work_shortfall_reaches_validator() fail {
  let pre = seed_head()
  let short = HeadDatum { ..pre, lifecycle: LifecycleState { ..pre.lifecycle, work: Work { remaining: 48, spent: 0, closure_reserve: 16 } } }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(short) } }
  spend_unit(Some(short), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test balance_shortfall_reaches_validator() fail {
  let pre = seed_head()
  expect [escrow, borrower, lender] = pre.lifecycle.balances
  let short = HeadDatum { ..pre, lifecycle: LifecycleState { ..pre.lifecycle, balances: [Balance { ..escrow, amount: 99 }, borrower, lender] } }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(short) } }
  spend_unit(Some(short), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test allowance_shortfall_reaches_validator() fail {
  let pre = seed_head()
  expect [escrow_allow, borrower_allow, lender_allow] = pre.lifecycle.allowances
  let short = HeadDatum { ..pre, lifecycle: LifecycleState { ..pre.lifecycle, allowances: [Allowance { ..escrow_allow, remaining: 99, spent: 0 }, borrower_allow, lender_allow] } }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(short) } }
  spend_unit(Some(short), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test missing_allowance_reaches_validator() fail {
  let pre = seed_head()
  expect [_escrow_allow, borrower_allow, lender_allow] = pre.lifecycle.allowances
  let short = HeadDatum { ..pre, lifecycle: LifecycleState { ..pre.lifecycle, allowances: [borrower_allow, lender_allow] } }
  let (_d, r, o, t) = release_tx()
  expect [own, fee, collateral] = t.inputs
  let input = Input { output_reference: own.output_reference, output: Output { ..own.output, datum: InlineDatum(short) } }
  spend_unit(Some(short), r, o, Transaction { ..t, inputs: [input, fee, collateral] })
}

test hostile_unbounded_upper() fail {
  let (d, r, o, t) = release_tx()
  let range = Interval { lower_bound: IntervalBound { bound_type: Finite(1000), is_inclusive: True }, upper_bound: IntervalBound { bound_type: PositiveInfinity, is_inclusive: False } }
  spend_unit(d, r, o, Transaction { ..t, validity_range: range })
}

test mint_seed_head_unit() {
  let policy = script_hash
  let digest = instance_digest(policy, cash_policy)
  let head = mint_seed_head(digest)
  let seed = seed_fix()
  let seeded = assets.merge(assets.from_lovelace(4_000_000), assets.from_asset(cash_policy, cash_name, 100))
  let seed_in = Input { output_reference: seed, output: Output { address: vkh_addr(lender_vkh), value: seeded, datum: NoDatum, reference_script: None } }
  let fee = Input { output_reference: ref_at(1), output: valued(vkh_addr(fee_vkh), False, 0, 3_000_000, NoDatum) }
  let minted = assets.from_asset(policy, thread_name, 1)
  let head_val = assets.merge(assets.merge(assets.from_lovelace(4_000_000), minted), assets.from_asset(cash_policy, cash_name, 100))
  let out = Output { address: script_addr(), value: head_val, datum: InlineDatum(head), reference_script: None }
  let tx = Transaction { ..tx.placeholder, inputs: [seed_in, fee], reference_inputs: [], outputs: [out], mint: minted, extra_signatories: [lender_vkh] }
  financial_head.mint(seed, cash_policy, lender_vkh, borrower_vkh, genesis_digest(seed, cash_policy, lender_vkh, borrower_vkh), 0, policy, tx)
}

test mint_forged_instance() fail {
  let policy = script_hash
  let head = mint_seed_head(#"9999999999999999999999999999999999999999999999999999999999999999")
  let seed = seed_fix()
  let seeded = assets.merge(assets.from_lovelace(4_000_000), assets.from_asset(cash_policy, cash_name, 100))
  let seed_in = Input { output_reference: seed, output: Output { address: vkh_addr(lender_vkh), value: seeded, datum: NoDatum, reference_script: None } }
  let fee = Input { output_reference: ref_at(1), output: valued(vkh_addr(fee_vkh), False, 0, 3_000_000, NoDatum) }
  let minted = assets.from_asset(policy, thread_name, 1)
  let head_val = assets.merge(assets.merge(assets.from_lovelace(4_000_000), minted), assets.from_asset(cash_policy, cash_name, 100))
  let out = Output { address: script_addr(), value: head_val, datum: InlineDatum(head), reference_script: None }
  let tx = Transaction { ..tx.placeholder, inputs: [seed_in, fee], reference_inputs: [], outputs: [out], mint: minted, extra_signatories: [lender_vkh] }
  financial_head.mint(seed, cash_policy, lender_vkh, borrower_vkh, genesis_digest(seed, cash_policy, lender_vkh, borrower_vkh), 0, policy, tx)
}
`;
}

function renderCapacitySpend(name, pre, post, fail) {
  return `test ${name}()${fail ? ' fail' : ''} {
  let pins = seed_head()
  let pre = HeadDatum { ..pins, lifecycle: ${renderLife(pre)} }
  let post = HeadDatum { ..pins, transport_revision: 1, transport_remaining: 0, ordinary_locked: 0, ordinary_revision: 1, lifecycle: ${renderLife(post)} }
  expect list.length(pre.lifecycle.balances) == ${pre.balances.length}
  expect list.length(post.lifecycle.balances) == ${post.balances.length}
  expect list.length(pre.lifecycle.used_transfer_ids) == ${pre.usedTransferIds.length}
  expect list.length(post.lifecycle.used_transfer_ids) == ${post.usedTransferIds.length}
  let (datum, redeemer, own, tx) = spend_tx(pre, post, Release { actor: "Lender", recipient: "Lender", transfer_id: "CapRelease", lower: 1000, upper: 1099 }, "Lender", "Lender", 100, 0, 100, 1000, 1100)
  spend_unit(datum, redeemer, own, tx)
}`;
}

function renderCapacityTests(capacity) {
  if (!capacity?.ok || capacity.capacity !== 128) throw new LowerError('ORACLE_DIVERGENCE', 'capacity envelopes');
  return [
    renderCapacitySpend('cap_ids_127_source_post_spends', capacity.ids127.pre, capacity.ids127.post, false),
    renderCapacitySpend('cap_ids_128_candidate_post_spends', capacity.ids128.pre, capacity.ids128.candidate, true),
    renderCapacitySpend('cap_balances_127_append_spends', capacity.balances127.pre, capacity.balances127.post, false),
    renderCapacitySpend('cap_balances_128_candidate_append_spends', capacity.balances128.pre, capacity.balances128.candidate, true),
    renderCapacitySpend('cap_balances_128_update_spends', capacity.balances128Update.pre, capacity.balances128Update.post, false),
  ].join('\n\n');
}

const APPROVED_BOUNDS_HASH = 'b3d02ace7a7ac7daec45318f2814beff4d27ad26f91d45405942c30d04fa846d';

function collectionCapacity(admitted) {
  const capacity = LIFECYCLE_BOUNDS.collectionCapacity;
  if (capacity !== 128) throw new LowerError('CAPACITY_BOUND', String(capacity));
  if (Object.hasOwn(BOUNDS_RECORD, 'collectionCapacity')) {
    throw new LowerError('CAPACITY_BOUND', 'approved bounds record must not replace the lifecycle constant');
  }
  if (BOUNDS_RECORD.workBudget?.staticWorkBoundPerAction !== '48' || BOUNDS_RECORD.workBudget?.kernelTransferUnit !== '1') {
    throw new LowerError('WORK_BOUND', 'approved bounds record');
  }
  const recomputed = hashDomain('MORIARTY-BOUNDS-financial-agreement/5', BOUNDS_RECORD);
  if (recomputed !== APPROVED_BOUNDS_HASH || admitted?.hashes?.boundsHash !== APPROVED_BOUNDS_HASH) {
    throw new LowerError('BOUNDS_HASH', `${recomputed}:${admitted?.hashes?.boundsHash ?? ''}`);
  }
  return capacity;
}

function hexConst(name, type, hex) {
  if (!/^[0-9a-f]+$/.test(hex) || hex.length % 2 !== 0) throw new LowerError('MALFORMED_ID', name);
  return `const ${name}: ${type} = #"${hex}"`;
}

const OUTER_BUILDERS = `
fn addr_of(policy: ByteArray) -> Address {
  Address { payment_credential: Script(policy), stake_credential: None }
}

fn vkh_address(vkh: ByteArray) -> Address {
  Address { payment_credential: VerificationKey(vkh), stake_credential: None }
}

fn value_with(policy: ByteArray, cash: ByteArray, with_thread: Bool, cash_qty: Int, lovelace: Int) -> Value {
  let ada = assets.from_lovelace(lovelace)
  let threaded = if with_thread { assets.merge(ada, assets.from_asset(policy, thread_name, 1)) } else { ada }
  if cash_qty > 0 { assets.merge(threaded, assets.from_asset(cash, cash_name, cash_qty)) } else { threaded }
}

fn output_with(policy: ByteArray, cash: ByteArray, address: Address, with_thread: Bool, cash_qty: Int, lovelace: Int, datum: Datum) -> Output {
  Output { address, value: value_with(policy, cash, with_thread, cash_qty, lovelace), datum, reference_script: None }
}

pub fn fixture_lender() -> ByteArray { lender_vkh }
pub fn fixture_borrower() -> ByteArray { borrower_vkh }
pub fn fixture_seed() -> OutputReference { seed_fix() }

pub fn positive_mint_tx(policy: ByteArray, cash: ByteArray) -> Transaction {
  let digest = instance_digest(policy, cash)
  let head = mint_seed_head(digest)
  let seed = seed_fix()
  let seeded = assets.merge(assets.from_lovelace(4_000_000), assets.from_asset(cash, cash_name, 100))
  let seed_in = Input { output_reference: seed, output: Output { address: vkh_address(lender_vkh), value: seeded, datum: NoDatum, reference_script: None } }
  let fee = Input { output_reference: ref_at(1), output: output_with(policy, cash, vkh_address(fee_vkh), False, 0, 3_000_000, NoDatum) }
  let minted = assets.from_asset(policy, thread_name, 1)
  let head_val = assets.merge(assets.merge(assets.from_lovelace(4_000_000), minted), assets.from_asset(cash, cash_name, 100))
  let out = Output { address: addr_of(policy), value: head_val, datum: InlineDatum(head), reference_script: None }
  Transaction { ..tx.placeholder, inputs: [seed_in, fee], reference_inputs: [], outputs: [out], mint: minted, extra_signatories: [lender_vkh] }
}

pub fn forged_mint_tx(policy: ByteArray, cash: ByteArray) -> Transaction {
  let head = mint_seed_head(#"9999999999999999999999999999999999999999999999999999999999999999")
  let seed = seed_fix()
  let seeded = assets.merge(assets.from_lovelace(4_000_000), assets.from_asset(cash, cash_name, 100))
  let seed_in = Input { output_reference: seed, output: Output { address: vkh_address(lender_vkh), value: seeded, datum: NoDatum, reference_script: None } }
  let fee = Input { output_reference: ref_at(1), output: output_with(policy, cash, vkh_address(fee_vkh), False, 0, 3_000_000, NoDatum) }
  let minted = assets.from_asset(policy, thread_name, 1)
  let head_val = assets.merge(assets.merge(assets.from_lovelace(4_000_000), minted), assets.from_asset(cash, cash_name, 100))
  let out = Output { address: addr_of(policy), value: head_val, datum: InlineDatum(head), reference_script: None }
  Transaction { ..tx.placeholder, inputs: [seed_in, fee], reference_inputs: [], outputs: [out], mint: minted, extra_signatories: [lender_vkh] }
}

fn release_pair(policy: ByteArray, cash: ByteArray, pre: HeadDatum, post: HeadDatum, credential: ByteArray) -> (HeadDatum, Redeemer, OutputReference, Transaction) {
  let own_ref = ref_at(0)
  let own = Input { output_reference: own_ref, output: output_with(credential, cash, addr_of(credential), True, 100, 2_000_000, InlineDatum(pre)) }
  let fee = Input { output_reference: ref_at(1), output: output_with(credential, cash, vkh_address(fee_vkh), False, 0, 3_000_000, NoDatum) }
  let collateral = Input { output_reference: ref_at(2), output: output_with(credential, cash, vkh_address(collateral_vkh), False, 0, 5_000_000, NoDatum) }
  let cont = output_with(credential, cash, addr_of(credential), True, 0, 2_000_000, InlineDatum(post))
  let pay = output_with(credential, cash, vkh_address(lender_vkh), False, 100, 2_000_000, NoDatum)
  let redeemer = Release { actor: "Lender", recipient: "Lender", transfer_id: "Release1", lower: 1000, upper: 1099 }
  let built = base_tx([own, fee, collateral], [cont, pay], [lender_vkh], finite_range(1000, 1100))
  (pre, redeemer, own_ref, built)
}

pub fn positive_release(policy: ByteArray, cash: ByteArray) -> (HeadDatum, Redeemer, OutputReference, Transaction) {
  let digest = instance_digest(policy, cash)
  release_pair(policy, cash, mint_seed_head(digest), release_post_for(digest), policy)
}

pub fn forged_spend(policy: ByteArray, cash: ByteArray) -> (HeadDatum, Redeemer, OutputReference, Transaction) {
  let forged = #"9999999999999999999999999999999999999999999999999999999999999999"
  release_pair(policy, cash, mint_seed_head(forged), release_post_for(forged), policy)
}

pub fn foreign_spend_mismatch(policy: ByteArray, cash: ByteArray) -> (HeadDatum, Redeemer, OutputReference, Transaction) {
  let foreign = #"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  let digest = instance_digest(policy, cash)
  release_pair(policy, cash, mint_seed_head(digest), release_post_for(digest), foreign)
}

pub fn foreign_spend_match(policy: ByteArray, cash: ByteArray) -> (HeadDatum, Redeemer, OutputReference, Transaction) {
  let foreign = #"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
  let digest = instance_digest(foreign, cash)
  release_pair(policy, cash, mint_seed_head(digest), release_post_for(digest), foreign)
}
`;

export function lowerCustody({ admitted, fixtures }) {
  const capacityBound = collectionCapacity(admitted);
  const byName = Object.fromEntries(admitted.elaborated.actions.map((action) => [action.action, action]));
  const rendered = [];
  for (const summary of admitted.actions) {
    const action = byName[summary.action];
    const debit = derivedDebit(action, summary);
    const lowered = lowerAction(action, debit);
    const tag = TAG[summary.action];
    rendered.push(`fn apply_${summary.action}(head: HeadDatum, actor: ByteArray, recipient: ByteArray, transfer_id: ByteArray, lower: Int, upper: Int, tx: Transaction, own_ref: OutputReference, own: ByteArray, cash: ByteArray, lender: ByteArray, borrower: ByteArray) -> Bool {
  expect bound_identity(head, own, cash)
  expect roles_ok(own, cash, lender, borrower)
  expect t1_fence(head.lifecycle)
  ${lowered.lines.join('\n  ')}
  let post_head = HeadDatum {
    program_hash: head.program_hash, bounds_hash: head.bounds_hash, source_hash: head.source_hash, schema_hash: head.schema_hash, seed_hash: head.seed_hash,
    domain_network: head.domain_network, domain_deployment: head.domain_deployment, instance_id: head.instance_id,
    transport_revision: checked_add(head.transport_revision, 1), transport_remaining: checked_sub(head.transport_remaining, 1),
    ordinary_locked: wrote_locked, ordinary_revision: wrote_revision, lifecycle: moved,
  }
  expect_frame(head, post_head, tx, own_ref, recipient, ${lowered.amount}, own, cash, lender, borrower)
}`);
    rendered.push(`// ${tag} debit ${debit} = constructor count ${debit - 1} + kernel unit`);
  }
  const manifest = admitted.manifest;
  const hashes = admitted.hashes;
  if (manifest.threadPolicyId !== manifest.scriptHash) {
    throw new LowerError('THREAD_POLICY_MUST_EQUAL_SCRIPT_HASH', 'lowerer');
  }
  const header = [
    hexConst('program_hash', 'ByteArray', hashes.programHash),
    hexConst('bounds_hash', 'ByteArray', hashes.boundsHash),
    hexConst('source_hash', 'ByteArray', hashes.sourceHash),
    hexConst('schema_hash', 'ByteArray', hashes.schemaHash),
    hexConst('seed_hash', 'ByteArray', hashes.seedHash),
    `const domain_network: ByteArray = "yaci-isolated-t1"`,
    `const domain_deployment: ByteArray = "t1-custody-revision03"`,
    hexConst('thread_name', 'AssetName', manifest.threadAssetNameHex),
    hexConst('cash_name', 'AssetName', manifest.cashAssetNameHex),
    'const instance_domain: ByteArray = "MORIARTY-INSTANCE-financial-agreement/5"',
    'const genesis_domain: ByteArray = "MORIARTY-GENESIS-financial-agreement/5"',
    'const genesis_version: ByteArray = "GenesisCommitment/1"',
    hexConst('zero28', 'ByteArray', '00'.repeat(28)),
    hexConst('shape_a', 'ByteArray', '24ba87f0a8b4f617d991e86e7a84fc3d4d90855dae7b2ce94e451a79'),
    hexConst('shape_b', 'ByteArray', '1d038a1069d649adf83745f02ff5c84575e32839194593263b0a2855'),
    hexConst('shape_c', 'ByteArray', 'b39aa2a57e9c3730b8234ada7592743d512edca5c23f60ba03c9dd4b'),
    hexConst('shape_d', 'ByteArray', '72f72192639eb173fce5342d2df730b73a0d838c48d2422ebf91e48c'),
    hexConst('shape_e', 'ByteArray', '64b8d588ca2e8207014edd458014a7b49007f8c01fab78ff809925fe'),
    hexConst('shape_instance', 'ByteArray', 'f36bb067a92c4c1c82d65e518bf2a5a633a95602b0937042be82d5cdf11924d4'),
    hexConst('zero32', 'ByteArray', '00'.repeat(32)),
    '//// unit_payout_sponsorship is the explicit unit fixture quantity. It is not protocol min-ADA, not kernel Cash, and not a bounds-hash field.',
    'const unit_payout_sponsorship: Int = 2_000_000',
  ];
  const mintSeed = renderHead('mint_seed_head', fixtures.seedHead, 'instance', true);
  const posts = [
    renderHead('release_post', fixtures.release.postHead, 'instance_digest(script_hash, cash_policy)', false),
    renderHead('release_post_for', fixtures.release.postHead, 'instance', true).replace(/^fn /, 'pub fn '),
    renderHead('refund_post', fixtures.refund.postHead, 'instance_digest(script_hash, cash_policy)', false),
    renderHead('cancel_post', fixtures.cancel.postHead, 'instance_digest(script_hash, cash_policy)', false),
  ];
  const fixturePins = [
    hexConst('script_hash', 'ByteArray', manifest.scriptHash),
    hexConst('thread_policy', 'PolicyId', manifest.threadPolicyId),
    hexConst('cash_policy', 'PolicyId', manifest.cashPolicyId),
    hexConst('lender_vkh', 'ByteArray', manifest.lenderVkh),
    hexConst('borrower_vkh', 'ByteArray', manifest.borrowerVkh),
    hexConst('fee_vkh', 'ByteArray', '11'.repeat(28)),
    hexConst('collateral_vkh', 'ByteArray', '22'.repeat(28)),
    hexConst('fix_seed_tx', 'ByteArray', '88'.repeat(32)),
    'fn seed_fix() -> OutputReference { OutputReference { transaction_id: fix_seed_tx, output_index: 0 } }',
    'fn seed_head() -> HeadDatum { mint_seed_head(instance_digest(script_hash, cash_policy)) }',
    'fn spend_unit(datum: Option<HeadDatum>, redeemer: Redeemer, own_ref: OutputReference, self: Transaction) -> Bool {',
    '  financial_head.spend(seed_fix(), cash_policy, lender_vkh, borrower_vkh, genesis_digest(seed_fix(), cash_policy, lender_vkh, borrower_vkh), datum, redeemer, own_ref, self)',
    '}',
  ].join('\n');
  const txs = [
    renderTx('release_tx', 'seed_head', 'release_post', 'Release', fixtures.release),
    renderTx('refund_tx', 'seed_head', 'refund_post', 'Refund', fixtures.refund),
    renderTx('cancel_tx', 'seed_head', 'cancel_post', 'Cancel', fixtures.cancel),
  ];
  if (fixtures.refund.args.lower !== '1100' || fixtures.refund.args.upper !== '1100') {
    throw new LowerError('ORACLE_DIVERGENCE', 'refund args are not the approved 1100/1100 vector');
  }
  const validator = `validator financial_head(seed: OutputReference, cash: PolicyId, lender: ByteArray, borrower: ByteArray, genesis: ByteArray) {
  mint(_redeemer: Data, policy_id: PolicyId, self: Transaction) {
    expect roles_ok(policy_id, cash, lender, borrower)
    expect genesis_digest(seed, cash, lender, borrower) == genesis
    expect seed_consumed(self, seed)
    expect mint_exact(self.mint, policy_id)
    let digest = instance_digest(policy_id, cash)
    expect instance_clean(digest)
    let head = mint_seed_head(digest)
    expect [only] = script_outputs(self.outputs, policy_id)
    expect only.address == Address { payment_credential: Script(policy_id), stake_credential: None }
    expect admitted_value(only.value, 1, escrow_cash(head.lifecycle), policy_id, cash)
    expect InlineDatum(raw) = only.datum
    expect decoded: HeadDatum = raw
    expect decoded == head
    expect other_outputs_lovelace_only(self.outputs, policy_id, cash)
    expect list.foldl(self.inputs, 0, fn(input, acc) { acc + cash_qty(input.output.value, cash) }) == list.foldl(self.outputs, 0, fn(output, acc) { acc + cash_qty(output.value, cash) })
    True
  }
  spend(datum: Option<HeadDatum>, redeemer: Redeemer, own_ref: OutputReference, self: Transaction) {
    expect Some(head) = datum
    let own = own_id(self, own_ref)
    expect roles_ok(own, cash, lender, borrower)
    expect genesis_digest(seed, cash, lender, borrower) == genesis
    when redeemer is {
      Release { actor, recipient, transfer_id, lower, upper } -> apply_release(head, actor, recipient, transfer_id, lower, upper, self, own_ref, own, cash, lender, borrower)
      Refund { actor, recipient, transfer_id, lower, upper } -> apply_refund(head, actor, recipient, transfer_id, lower, upper, self, own_ref, own, cash, lender, borrower)
      Cancel { actor, recipient, transfer_id, lower, upper } -> apply_cancel(head, actor, recipient, transfer_id, lower, upper, self, own_ref, own, cash, lender, borrower)
    }
  }
  else(_) { fail }
}`;
  const prelude = PRELUDE.replace(
    'const collection_capacity: Int = COLLECTION_CAPACITY',
    `const collection_capacity: Int = ${capacityBound}`,
  );
  if (prelude.includes('COLLECTION_CAPACITY')) throw new LowerError('CAPACITY_BOUND', 'placeholder');
  const source = [
    prelude,
    ...header,
    FRAME,
    ...rendered,
    mintSeed,
    validator,
    fixturePins,
    FIXTURES,
    ...posts,
    OUTER_BUILDERS,
    ...txs,
    renderTests(fixtures.release.postHead.life.work.remaining),
    renderCapacityTests(fixtures.capacity),
  ].join('\n');
  return {
    validator: source,
    expectedTests: [
      'source_uplc_unit_release', 'source_uplc_unit_refund', 'source_uplc_unit_cancel_empty',
      'nonempty_obligations', 'n01_refund_args_wider_interval', 'missing_signatory', 'wrong_recipient_payout',
      'shared_payout', 'duplicate_transfer_id', 'mutated_work', 'stale_bounds_hash', 'schema_version',
      'seed_cash_utxo_mismatch', 'wrong_actor_redeemer', 'mint_nonzero', 'second_script_input',
      'replay_used_id', 'wrong_transport', 'sponsorship_moved', 'decode_lucid_definite_e13_pre',
      'decode_lucid_definite_e13_post', 'decode_lucid_definite_balance', 'decode_lucid_definite_release',
      'frozen_shape_sample_instance_differs', 'frozen_shape_sample_head_not_spent',
      'release_missing_lender_row', 'generic_absent_party_and_asset_appends',
      'hostile_duplicate_lender_rows_do_not_inflate', 'oracle_noninjective_escrow_rows',
      'duplicate_sender_allowance_refused', 'duplicate_untouched_balance_refused',
      'hostile_other_asset_dropped', 'oracle_foreign_asset_on_continuation',
      'oracle_foreign_asset_on_payout', 'oracle_foreign_mint',
      'unadmitted_name_on_admitted_cash_policy', 'unadmitted_name_on_admitted_thread_policy',
      'unadmitted_mint_other_name', 'hostile_change_lovelace_merged_into_payout',
      'hostile_separate_lovelace_change_output', 'oracle_lovelace_change_is_not_kernel_cash',
      'oracle_fee_lovelace_does_not_enter_kernel', 'work_shortfall_reaches_validator',
      'balance_shortfall_reaches_validator', 'allowance_shortfall_reaches_validator',
      'missing_allowance_reaches_validator', 'hostile_unbounded_upper',
      'cap_ids_127_source_post_spends', 'cap_ids_128_candidate_post_spends',
      'cap_balances_127_append_spends', 'cap_balances_128_candidate_append_spends',
      'cap_balances_128_update_spends',
      'mint_seed_head_unit', 'mint_forged_instance',
    ].map((title) => ({ title, status: 'success' })),
  };
}

export { LowerError };
