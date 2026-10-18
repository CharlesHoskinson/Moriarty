//// Generated from Moriarty Core. Public atomic experiment; not production/PCD.
//// programHash ced6488bf2bfcceb76e08b76292d3708bed379774440f9617a144dd47e1055f1
//// sourceHash 3fee959e4a314a2b137e9acef850fd3669053636f21e0405700d6f482a671121
//// timePredicate TIME_PREDICATE_v1
//// nativePredicate public-single-head-wallet-transfer/1
//// genesis: external consumed-seed one-shot mint; this spend validator does not authenticate issuance.
use aiken/collection/list
use aiken/interval.{Interval, IntervalBound, Finite}
use cardano/address.{Address, Credential, Script, VerificationKey}
use cardano/assets.{Value, PolicyId, AssetName}
use cardano/transaction.{Transaction, Output, OutputReference, Input, InlineDatum, NoDatum, Datum}
use cardano/transaction as tx
use moriarty/public.{checked_add, checked_sub, checked_mul, checked_div, last_valid_posix, max_u128, in_u128}

pub const program_hash: ByteArray = #"ced6488bf2bfcceb76e08b76292d3708bed379774440f9617a144dd47e1055f1"
pub const bounds_hash: ByteArray = #"ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c"
pub const source_hash: ByteArray = #"3fee959e4a314a2b137e9acef850fd3669053636f21e0405700d6f482a671121"
pub const domain_network: ByteArray = "isolated-cardano-test-network"
pub const domain_deployment: ByteArray = "moriarty-eutxo-public-20261012"
pub const thread_policy: PolicyId = #"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
pub const thread_name: AssetName = #"48454144"
pub const lifetime: Int = 8
pub const horizon: Int = 2000000000

pub type StateValues {
  count: Int,
}

pub fn genesis_values() -> StateValues {
  StateValues { count: 0 }
}

pub type HeadDatum {
  program_hash: ByteArray,
  bounds_hash: ByteArray,
  source_hash: ByteArray,
  domain_network: ByteArray,
  domain_deployment: ByteArray,
  instance_id: ByteArray,
  revision: Int,
  remaining: Int,
  values: StateValues,
}

pub type StepArgs {
  actor: ByteArray,
  payout: Int,
}

pub type AtomicRedeemer {
  Step(StepArgs)
}

pub fn actor_vkh(name: ByteArray) -> ByteArray {
  if name == "payer" { #"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb" }
  else if name == "payee" { #"cccccccccccccccccccccccccccccccccccccccccccccccccccccccc" }
  else { fail }
}

pub fn actor_address(name: ByteArray) -> Address {
  Address { payment_credential: VerificationKey(actor_vkh(name)), stake_credential: None }
}

pub fn asset_policy(asset: ByteArray) -> PolicyId {
  if asset == "CASH" { #"dddddddddddddddddddddddddddddddddddddddddddddddddddddddd" }
  else { fail }
}

pub fn asset_name(asset: ByteArray) -> AssetName {
  if asset == "CASH" { #"43415348" }
  else { fail }
}

fn bindings_match(head: HeadDatum) -> Bool {
  and {
    head.program_hash == program_hash,
    head.bounds_hash == bounds_hash,
    head.source_hash == source_hash,
    head.domain_network == domain_network,
    head.domain_deployment == domain_deployment,
  }
}

fn thread_qty(value: Value) -> Int {
  assets.quantity_of(value, thread_policy, thread_name)
}

fn is_script(cred: Credential) -> Bool {
  when cred is {
    Script(_) -> True
    VerificationKey(_) -> False
  }
}

fn take_payout(
  outputs: List<Output>,
  used: List<Int>,
  idx: Int,
  target: Address,
  policy: PolicyId,
  name: AssetName,
  qty: Int,
) -> List<Int> {
  when outputs is {
    [] -> fail
    [o, ..rest] -> {
      if list.has(used, idx) {
        take_payout(rest, used, idx + 1, target, policy, name, qty)
      } else if o.address == target && assets.quantity_of(o.value, policy, name) == qty {
        [idx, ..used]
      } else {
        take_payout(rest, used, idx + 1, target, policy, name, qty)
      }
    }
  }
}

fn qty_at_cred_outputs(outputs: List<Output>, cred: Credential, policy: PolicyId, name: AssetName) -> Int {
  list.foldl(
    outputs,
    0,
    fn(o, acc) {
      if o.address.payment_credential == cred {
        acc + assets.quantity_of(o.value, policy, name)
      } else {
        acc
      }
    },
  )
}

fn qty_at_cred_inputs(inputs: List<Input>, cred: Credential, policy: PolicyId, name: AssetName) -> Int {
  qty_at_cred_outputs(list.map(inputs, fn(i) { i.output }), cred, policy, name)
}

fn party_net(self: Transaction, vkh: ByteArray, policy: PolicyId, name: AssetName) -> Int {
  let cred = VerificationKey(vkh)
  qty_at_cred_inputs(self.inputs, cred, policy, name) - qty_at_cred_outputs(self.outputs, cred, policy, name)
}

fn expect_single_script_head(inputs: List<Input>, own_ref: OutputReference) -> Input {
  let scripts = list.filter(inputs, fn(i) { is_script(i.output.address.payment_credential) })
  expect [own] = scripts
  expect own.output_reference == own_ref
  expect thread_qty(own.output.value) == 1
  own
}

fn expect_one_continuation(outputs: List<Output>, script: Credential) -> Output {
  let heads =
    list.filter(
      outputs,
      fn(o) { o.address.payment_credential == script && thread_qty(o.value) == 1 },
    )
  expect [cont] = heads
  expect list.length(list.filter(outputs, fn(o) { thread_qty(o.value) > 0 })) == 1
  cont
}

fn head_value_preserved(own: Output, cont: Output) -> Bool {
  assets.flatten(own.value) == assets.flatten(cont.value)
}

fn run_step(head: HeadDatum, redeemer: AtomicRedeemer, now: Int, self: Transaction, own_ref: OutputReference) -> Bool {
  expect Step(args) = redeemer
  expect bindings_match(head)
  expect in_u128(head.revision)
  expect in_u128(head.remaining)
  expect in_u128(head.values.count)
  expect in_u128(args.payout)
  expect head.remaining > 0
  expect checked_add(head.revision, head.remaining) == lifetime
  expect now < horizon
  expect head.revision != 0 || head.values == genesis_values()
  expect head.revision != 0 || head.remaining == lifetime
  let v0 = now < 2000000000
  expect v0
  let v1 = args.actor == "payer"
  expect v1
  let v2 = args.payout > 0
  expect v2
  let v3 = head.remaining > 1
  expect v3
  let v4 = head.remaining > 0
  let v7 = if v4 { True } else {
    let v6 = checked_div(1, 0)
    let v5 = v6 > 0
    v5
  }
  expect v7
  let v9 = 0 > head.remaining
  let v12 = if v9 {
    let v11 = checked_div(1, 0)
    let v10 = v11 > 0
    v10
  } else { False }
  let v8 = !v12
  expect v8
  let v13 = checked_div(args.payout, 1)
  let v14 = v13 == args.payout
  expect v14
  let v15 = checked_add(head.values.count, 1)
  let own = expect_single_script_head(self.inputs, own_ref)
  expect Script(script_hash) = own.output.address.payment_credential
  expect list.has(self.extra_signatories, actor_vkh(args.actor))
  expect assets.quantity_of(self.mint, thread_policy, thread_name) == 0
  expect assets.quantity_of(self.mint, asset_policy("CASH"), asset_name("CASH")) == 0
  let cont = expect_one_continuation(self.outputs, Script(script_hash))
  expect InlineDatum(post_raw) = cont.datum
  expect post_raw: HeadDatum = post_raw
  let post = HeadDatum {
    program_hash: head.program_hash,
    bounds_hash: head.bounds_hash,
    source_hash: head.source_hash,
    domain_network: head.domain_network,
    domain_deployment: head.domain_deployment,
    instance_id: head.instance_id,
    revision: checked_add(head.revision, 1),
    remaining: checked_sub(head.remaining, 1),
    values: StateValues { count: v15 },
  }
  expect post_raw == post
  expect cont.address == own.output.address
  expect head_value_preserved(own.output, cont)
  let used: List<Int> = []
  expect _used0 = take_payout(self.outputs, used, 0, actor_address("payee"), asset_policy("CASH"), asset_name("CASH"), args.payout)
  expect party_net(self, actor_vkh("payer"), asset_policy("CASH"), asset_name("CASH")) == (0 + args.payout)
  expect (0 + args.payout) <= 0 || list.has(self.extra_signatories, actor_vkh("payer"))
  expect party_net(self, actor_vkh("payee"), asset_policy("CASH"), asset_name("CASH")) == (0 - args.payout)
  True
}

validator atomic {
  spend(datum: Option<HeadDatum>, redeemer: AtomicRedeemer, own_ref: OutputReference, tx: Transaction) {
    expect Some(head) = datum
    let now = last_valid_posix(tx.validity_range)
    when redeemer is {
      Step(_) -> run_step(head, redeemer, now, tx, own_ref)
    }
  }

  else(_) {
    fail
  }
}

fn head(revision: Int, remaining: Int, count: Int, instance_id: ByteArray) -> HeadDatum {
  HeadDatum {
    program_hash,
    bounds_hash,
    source_hash,
    domain_network,
    domain_deployment,
    instance_id,
    revision,
    remaining,
    values: StateValues { count: count },
  }
}

fn script_addr() -> Address {
  Address { payment_credential: Script(#"eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee"), stake_credential: None }
}

fn other_script_addr() -> Address {
  Address { payment_credential: Script(#"ffffffffffffffffffffffffffffffffffffffffffffffffffffffff"), stake_credential: None }
}

fn valued(addr: Address, with_thread: Bool, cash_qty: Int, datum: Datum) -> Output {
  let ada = assets.from_lovelace(2_000_000)
  let threaded = if with_thread { assets.merge(ada, assets.from_asset(thread_policy, thread_name, 1)) } else { ada }
  let value = if cash_qty > 0 { assets.merge(threaded, assets.from_asset(asset_policy("CASH"), asset_name("CASH"), cash_qty)) } else { threaded }
  Output { address: addr, value, datum, reference_script: None }
}

fn finite_range(lower: Int, upper_exclusive: Int) -> Interval {
  Interval {
    lower_bound: IntervalBound { bound_type: Finite(lower), is_inclusive: True },
    upper_bound: IntervalBound { bound_type: Finite(upper_exclusive), is_inclusive: False },
  }
}

fn ref_at(i: Int) -> OutputReference {
  OutputReference { transaction_id: tx.placeholder.id, output_index: i }
}

fn own_ref() -> OutputReference {
  ref_at(0)
}

fn base_tx(inputs: List<Input>, outputs: List<Output>, signers: List<ByteArray>, range: Interval) -> Transaction {
  Transaction { ..tx.placeholder, inputs, outputs, extra_signatories: signers, validity_range: range }
}


fn positive_tx() -> (Option<HeadDatum>, AtomicRedeemer, OutputReference, Transaction) {
  let pre = head(0, lifetime, 0, "instance-1")
  let post = head(1, lifetime - 1, 1, "instance-1")
  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(pre)) }
  let payer = Input { output_reference: ref_at(1), output: valued(actor_address("payer"), False, 10, NoDatum) }
  let cont = valued(script_addr(), True, 0, InlineDatum(post))
  let pay = valued(actor_address("payee"), False, 5, NoDatum)
  let change = valued(actor_address("payer"), False, 5, NoDatum)
  let rdmr = Step(StepArgs { actor: "payer", payout: 5 })
  let txb = base_tx([own, payer], [cont, pay, change], [actor_vkh("payer")], finite_range(1000, 2000))
  (Some(pre), rdmr, own_ref(), txb)
}

test positive_step() {
  let (d, r, o, t) = positive_tx()
  atomic.spend(d, r, o, t)
}

test missing_continuation() fail {
  let (d, r, o, t) = positive_tx()
  let t2 = Transaction { ..t, outputs: list.filter(t.outputs, fn(x) { thread_qty(x.value) == 0 }) }
  atomic.spend(d, r, o, t2)
}

test multiple_continuation() fail {
  let (d, r, o, t) = positive_tx()
  expect [c, p, ch] = t.outputs
  let t2 = Transaction { ..t, outputs: [c, c, p, ch] }
  atomic.spend(d, r, o, t2)
}

test wrong_token() fail {
  let (d, r, o, t) = positive_tx()
  let bad = valued(script_addr(), False, 0, InlineDatum(head(1, lifetime - 1, 1, "instance-1")))
  expect [_, p, ch] = t.outputs
  atomic.spend(d, r, o, Transaction { ..t, outputs: [bad, p, ch] })
}

test wrong_address() fail {
  let (d, r, o, t) = positive_tx()
  let stolen = valued(actor_address("payee"), True, 0, InlineDatum(head(1, lifetime - 1, 1, "instance-1")))
  expect [_, p, ch] = t.outputs
  atomic.spend(d, r, o, Transaction { ..t, outputs: [stolen, p, ch] })
}

test wrong_datum() fail {
  let (d, r, o, t) = positive_tx()
  let mutated = valued(script_addr(), True, 0, InlineDatum(head(1, lifetime - 1, 9, "instance-1")))
  expect [_, p, ch] = t.outputs
  atomic.spend(d, r, o, Transaction { ..t, outputs: [mutated, p, ch] })
}

test changed_result() fail {
  let (d, r, o, t) = positive_tx()
  expect Some(pre) = d
  let mutated = HeadDatum { ..pre, values: StateValues { count: 4 } }
  atomic.spend(Some(mutated), r, o, t)
}

test signer_missing() fail {
  let (d, r, o, t) = positive_tx()
  atomic.spend(d, r, o, Transaction { ..t, extra_signatories: [] })
}

test stale_program_hash() fail {
  let (d, r, o, t) = positive_tx()
  expect Some(pre) = d
  let stale = HeadDatum { ..pre, program_hash: #"ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff" }
  expect [_, payer] = t.inputs
  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(stale)) }
  atomic.spend(Some(stale), r, o, Transaction { ..t, inputs: [own, payer] })
}

test conflicting_predecessor() fail {
  let (d, r, o, t) = positive_tx()
  expect [own, payer] = t.inputs
  let other_ref = OutputReference { transaction_id: tx.placeholder.id, output_index: 2 }
  let other = Input { output_reference: other_ref, output: own.output }
  atomic.spend(d, r, o, Transaction { ..t, inputs: [own, payer, other] })
}

test shared_payout() fail {
  let (d, r, o, t) = positive_tx()
  expect [c, _, ch] = t.outputs
  atomic.spend(d, r, o, Transaction { ..t, outputs: [c, ch] })
}

test unbounded_time() fail {
  let (d, r, o, t) = positive_tx()
  atomic.spend(d, r, o, Transaction { ..t, validity_range: tx.placeholder.validity_range })
}

test wallet_debit_required() fail {
  let (d, r, o, t) = positive_tx()
  expect [own, _] = t.inputs
  let sponsor = Input { output_reference: ref_at(1), output: valued(actor_address("payee"), False, 5, NoDatum) }
  expect [c, p, _] = t.outputs
  atomic.spend(d, r, o, Transaction { ..t, inputs: [own, sponsor], outputs: [c, p] })
}

test cross_script_head() fail {
  let (d, r, o, t) = positive_tx()
  let other_value = assets.merge(assets.from_lovelace(2_000_000), assets.from_asset(#"ffffffffffffffffffffffffffffffffffffffffffffffffffffffff", #"4f544852", 1))
  let other = Input { output_reference: ref_at(2), output: Output { address: other_script_addr(), value: other_value, datum: NoDatum, reference_script: None } }
  expect [own, payer] = t.inputs
  atomic.spend(d, r, o, Transaction { ..t, inputs: [own, payer, other] })
}

test forged_genesis_values() fail {
  let (d, r, o, t) = positive_tx()
  expect Some(pre) = d
  let forged = HeadDatum { ..pre, values: StateValues { count: 4 } }
  let post = HeadDatum { ..forged, revision: 1, remaining: lifetime - 1, values: StateValues { count: 5 } }
  expect [_, payer] = t.inputs
  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(forged)) }
  let cont = valued(script_addr(), True, 0, InlineDatum(post))
  expect [_, p, ch] = t.outputs
  atomic.spend(Some(forged), r, o, Transaction { ..t, inputs: [own, payer], outputs: [cont, p, ch] })
}

test prestate_out_of_range() fail {
  let (d, r, o, t) = positive_tx()
  expect Some(pre) = d
  let bad = HeadDatum { ..pre, values: StateValues { count: -1 } }
  expect [_, payer] = t.inputs
  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(bad)) }
  atomic.spend(Some(bad), r, o, Transaction { ..t, inputs: [own, payer] })
}

test argument_out_of_range() fail {
  let (d, _r, o, t) = positive_tx()
  let rdmr = Step(StepArgs { actor: "payer", payout: -1 })
  atomic.spend(d, rdmr, o, t)
}

test arithmetic_boundary_count() fail {
  let (d, r, o, t) = positive_tx()
  expect Some(pre) = d
  let overflowed = HeadDatum { ..pre, revision: 1, remaining: lifetime - 1, values: StateValues { count: max_u128 } }
  expect [_, payer] = t.inputs
  let own = Input { output_reference: own_ref(), output: valued(script_addr(), True, 0, InlineDatum(overflowed)) }
  let post = HeadDatum { ..overflowed, revision: 2, remaining: lifetime - 2, values: StateValues { count: 0 } }
  let cont = valued(script_addr(), True, 0, InlineDatum(post))
  expect [_, p, ch] = t.outputs
  atomic.spend(Some(overflowed), r, o, Transaction { ..t, inputs: [own, payer], outputs: [cont, p, ch] })
}

test shortcircuit_or() {
  let left = True
  let v = if left { True } else { checked_div(1, 0) > 0 }
  v
}

test shortcircuit_and() {
  let left = False
  let v = if left { checked_div(1, 0) > 0 } else { False }
  !v
}
