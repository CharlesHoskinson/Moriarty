//// S2: thread genesis, settlement, revocation and closing for ABI v2 (P3S1-R1 W0/W1
//// successor, PROPOSED, unreviewed, uncompiled). This is the ScriptContext consumer: every
//// fact it binds comes from the transaction, the spent datum or a recomputation; the
//// redeemer only proposes values that the claim digest, the signed intents and the physical
//// checks must all agree with. ABI-V2.md is normative.
////
//// P3S1-R1 changes (review RR-01, RR-02, RR-04): S2 takes the reviewed C2 script hash as
//// a parameter and pins it at genesis and settle; the structural shape gate refuses
//// unsorted or duplicate cells; signer window, `check_effects` and native co-authority are
//// boolean components instead of an `expect`; signed intents use
//// `moriarty-cardano-intent-domain/2`.
////
//// W1 changes (actual-result review NEXT-CAPABILITY W1, AR-07): every gate component is a
//// `pub fn ... -> Bool` defined once here. The production gates (`genesis`, `settle_gate`,
//// `revoke`, `close`) conjoin those functions without names, lists or labels. The named
//// diagnostic lists live in `pvc2/diag`, which no validator or production module imports,
//// and call the same functions on the same stage values. Settle work is built in stages
//// (`settle_base` .. `settle_next`), each computed at most once per run and only when the
//// gate reaches it. No component, conjunct or `expect` of P3S1-R1 is dropped.

use aiken/builtin
use aiken/collection/dict
use aiken/collection/list
use cardano/address.{Script}
use cardano/assets.{PolicyId}
use cardano/transaction.{
  InlineDatum, Input, Output, OutputReference, Spend, Transaction,
}
use pvc2/bytes
use pvc2/catalogue
use pvc2/claim
use pvc2/core.{CDuty, CMove, CoreEffectV2, MoveV2, StatementCtxV2}
use pvc2/intent.{IntentDomainV2, SigCheckV2}
use pvc2/types.{
  AccountV2, ActionEntryV2, BalanceRowV2, CatalogueV2, CellRefV2, Close,
  DutyRefV2, DutyRowV2, DutyValV2, EffectValV2, EnvV2, External, Lovelace,
  PayoutLegV2, PayoutTagV2, PinnedDatum, Revoke, Settle, SettleRedeemerV2,
  ShapeV2, SignedIntentV2, TagDatum, ThreadDatumV2, Token, VBool, VUInt,
  ValueV2,
}

pub const evt_tag: ByteArray = "MRG16-PVC-EVT-02"

pub const revoke_tag: ByteArray = "moriarty-cardano-revoke/1"

fn own_input(self: Transaction, own_ref: OutputReference) -> Input {
  expect Some(own) = list.find(self.inputs, fn(i) { i.output_reference == own_ref })
  own
}

fn no_own_mint(self: Transaction, own_hash: ByteArray, thread_id: ByteArray) -> Bool {
  and {
    assets.quantity_of(self.mint, own_hash, thread_id) == 0,
    list.all(dict.values(assets.tokens(self.mint, own_hash)), fn(q) { q < 0 }),
  }
}

// ## Global output allocation (atomic multi-thread transactions)

fn outputs_claimed(r: SettleRedeemerV2) -> List<Int> {
  when r is {
    Settle { payouts, continuation, .. } ->
      [continuation, ..list.map(payouts, fn(l) { l.output })]
    Revoke { continuation, .. } -> [continuation]
    Close { reserve_output } -> [reserve_output]
  }
}

/// Every output index claimed by any S2 spend redeemer in this transaction (thread
/// continuations, payouts, reserve refunds) is claimed once. Each thread input runs
/// this same check, so one output can never satisfy two legs, two threads or two roles.
/// Scope: spends of this exact S2 script hash only (review RR-16: another S2 version in
/// the same transaction is a documented cross-version residual).
pub fn global_outputs_distinct(own_hash: ByteArray, self: Transaction) -> Bool {
  let claimed =
    list.foldr(
      self.redeemers,
      [],
      fn(p, acc) {
        let Pair(purpose, data) = p
        when purpose is {
          Spend(ref) ->
            when list.find(self.inputs, fn(i) { i.output_reference == ref }) is {
              Some(i) ->
                if i.output.address.payment_credential == Script(own_hash) {
                  expect r: SettleRedeemerV2 = data
                  list.concat(outputs_claimed(r), acc)
                } else {
                  acc
                }
              None -> acc
            }
          _ -> acc
        }
      },
    )
  bytes.ints_distinct(claimed)
}

// ## Genesis

fn value_has_ty(v: ValueV2, ty: Int) -> Bool {
  when v is {
    VUInt(n) -> ty == 0 && bytes.is_u128(n)
    VBool(_) -> ty == 1
  }
}

fn state_ok(cat: CatalogueV2, state: List<ValueV2>) -> Bool {
  list.length(state) == list.length(cat.states) && list.all(
    list.zip(cat.states, state),
    fn(p) { value_has_ty(p.2nd, p.1st.ty) },
  )
}

fn row_key_lt(cat: CatalogueV2, a: BalanceRowV2, b: BalanceRowV2) -> Bool {
  let ia = catalogue.asset_map(cat, a.asset).id
  let ib = catalogue.asset_map(cat, b.asset).id
  if ia == ib {
    bytes.bytes_lt(a.account, b.account)
  } else {
    bytes.asset_lt(ia, ib)
  }
}

fn rows_ok(cat: CatalogueV2, rows: List<BalanceRowV2>) -> Bool {
  and {
    list.all(
      rows,
      fn(r) {
        and {
          r.asset >= 0,
          r.asset < list.length(cat.assets),
          bytes.is_u128(r.amount),
          when catalogue.find_account(cat, r.account) is {
            Some(acc) -> catalogue.is_custody(acc)
            None -> False
          },
        }
      },
    ),
    bytes.strictly_ascending(rows, fn(a, b) { row_key_lt(cat, a, b) }),
  }
}

/// Initial liabilities (incoming duties, registered or not) need the debtor's native
/// consent in the genesis transaction; present-zero rows included.
fn duties_ok(cat: CatalogueV2, rows: List<DutyRowV2>, self: Transaction) -> Bool {
  and {
    bytes.strictly_ascending(list.map(rows, fn(d) { d.name }), bytes.bytes_lt),
    list.all(
      rows,
      fn(d) {
        and {
          bytes.core_string(d.name),
          bytes.core_string(d.debtor),
          bytes.core_string(d.creditor),
          d.asset >= 0,
          d.asset < list.length(cat.assets),
          bytes.is_u128(d.amount),
          when catalogue.find_account(cat, d.debtor) is {
            Some(acc) ->
              when acc.native is {
                Some(c) -> catalogue.native_ok(c, self)
                None -> False
              }
            None -> False
          },
        }
      },
    ),
  }
}

/// Parsed genesis mint: the single minted token of this policy and the single output
/// holding it, with its thread datum.
pub type GenesisBaseV2 {
  c2_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  tx: Transaction,
  name: ByteArray,
  qty: Int,
  out: Output,
  holder: ByteArray,
  td: ThreadDatumV2,
}

pub fn genesis_base(
  c2_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> GenesisBaseV2 {
  expect [Pair(name, qty)] = dict.to_pairs(assets.tokens(self.mint, policy_id))
  expect [out] = list.filter(self.outputs, fn(o) { claim.holds_policy(o, policy_id) })
  expect Script(holder) = out.address.payment_credential
  expect InlineDatum(raw) = out.datum
  expect td: ThreadDatumV2 = raw
  GenesisBaseV2 { c2_hash, policy_id, seed, tx: self, name, qty, out, holder, td }
}

// One-shot genesis components: the seed is spent; the token is
// blake2b_256(serialise_data(seed)); one output at this script with a well-formed datum,
// seq 0, a zero event chain, an exact custody value and (R1, RR-02) the reviewed claim
// script `c2_hash` that parameterizes this S2.

pub fn genesis_mint_one(g: GenesisBaseV2) -> Bool {
  g.qty == 1
}

pub fn genesis_seed_spent(g: GenesisBaseV2) -> Bool {
  list.any(g.tx.inputs, fn(i) { i.output_reference == g.seed })
}

pub fn genesis_token_name(g: GenesisBaseV2) -> Bool {
  let seed_data: Data = g.seed
  g.name == builtin.blake2b_256(builtin.serialise_data(seed_data))
}

pub fn genesis_holder(g: GenesisBaseV2) -> Bool {
  g.holder == g.policy_id
}

pub fn genesis_no_ref_script(g: GenesisBaseV2) -> Bool {
  g.out.reference_script == None
}

pub fn genesis_version(g: GenesisBaseV2) -> Bool {
  g.td.version == 2
}

pub fn genesis_thread_id(g: GenesisBaseV2) -> Bool {
  g.td.thread_id == g.name
}

pub fn genesis_claim_script(g: GenesisBaseV2) -> Bool {
  and {
    bytes.len(g.td.claim_hash) == 28,
    g.td.claim_hash != g.policy_id,
    g.td.claim_hash == g.c2_hash,
  }
}

pub fn genesis_seq_zero(g: GenesisBaseV2) -> Bool {
  g.td.seq == 0
}

pub fn genesis_chain_zero(g: GenesisBaseV2) -> Bool {
  g.td.event_chain == bytes.zero32
}

pub fn genesis_catalogue(g: GenesisBaseV2) -> Bool {
  catalogue.well_formed(g.td.catalogue)
}

pub fn genesis_state(g: GenesisBaseV2) -> Bool {
  state_ok(g.td.catalogue, g.td.state)
}

pub fn genesis_rows(g: GenesisBaseV2) -> Bool {
  rows_ok(g.td.catalogue, g.td.balances)
}

pub fn genesis_duties(g: GenesisBaseV2) -> Bool {
  duties_ok(g.td.catalogue, g.td.liabilities, g.tx)
}

pub fn genesis_reserve_floor(g: GenesisBaseV2) -> Bool {
  g.td.reserve >= g.td.catalogue.reserve_floor
}

pub fn genesis_bounds(g: GenesisBaseV2) -> Bool {
  list.length(g.td.balances) <= 4096 && list.length(g.td.liabilities) <= 1024
}

pub fn genesis_custody_value(g: GenesisBaseV2) -> Bool {
  g.out.value == catalogue.custody_value(
    g.td.catalogue,
    g.td.balances,
    g.td.reserve,
    g.policy_id,
    g.name,
  )
}

/// Production genesis gate: the conjunction of every genesis component, in the
/// P3S1-R1 component order (`diag.genesis_parts` names the same calls).
pub fn genesis(
  c2_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> Bool {
  let g = genesis_base(c2_hash, policy_id, seed, self)
  and {
    genesis_mint_one(g),
    genesis_seed_spent(g),
    genesis_token_name(g),
    genesis_holder(g),
    genesis_no_ref_script(g),
    genesis_version(g),
    genesis_thread_id(g),
    genesis_claim_script(g),
    genesis_seq_zero(g),
    genesis_chain_zero(g),
    genesis_catalogue(g),
    genesis_state(g),
    genesis_rows(g),
    genesis_duties(g),
    genesis_reserve_floor(g),
    genesis_bounds(g),
    genesis_custody_value(g),
  }
}

/// Burn path of the thread policy (used by Close): only negative quantities.
pub fn burn_only(policy_id: PolicyId, self: Transaction) -> Bool {
  list.all(dict.values(assets.tokens(self.mint, policy_id)), fn(q) { q < 0 })
}

// ## Settle: helpers

/// Index of the first represented cell naming (asset, account).
fn cell_index(cells: List<CellRefV2>, asset: Int, account: ByteArray, i: Int) -> Option<Int> {
  when cells is {
    [] -> None
    [c, ..rest] ->
      if c.asset == asset && c.account == account {
        Some(i)
      } else {
        cell_index(rest, asset, account, i + 1)
      }
  }
}

/// Index of the first represented duty named `name`.
fn duty_index(duties: List<DutyRefV2>, name: ByteArray, j: Int) -> Option<Int> {
  when duties is {
    [] -> None
    [d, ..rest] ->
      if d.name == name {
        Some(j)
      } else {
        duty_index(rest, name, j + 1)
      }
  }
}

fn insert_duty(rows: List<DutyRowV2>, row: DutyRowV2) -> List<DutyRowV2> {
  when rows is {
    [] -> [row]
    [r, ..rest] ->
      if bytes.bytes_lt(row.name, r.name) {
        [row, r, ..rest]
      } else {
        expect r.name != row.name
        [r, ..insert_duty(rest, row)]
      }
  }
}

/// P3-7 required signers: every `from` of a Transfer/Fee (custody and external, any
/// amount), the debtor of every accrual, the creditor of every discharge.
pub fn required_signers(effects: List<CoreEffectV2>) -> List<ByteArray> {
  list.filter_map(
    effects,
    fn(e) {
      when e is {
        CMove(m) -> Some(m.from)
        CDuty { debtor, creditor, discharge, .. } ->
          if discharge {
            Some(creditor)
          } else {
            Some(debtor)
          }
        _ -> None
      }
    },
  )
    |> list.unique
}

fn payout_ok(
  cat: CatalogueV2,
  td: ThreadDatumV2,
  leg: PayoutLegV2,
  effect_index: Int,
  m: MoveV2,
  acc: AccountV2,
  self: Transaction,
) -> Bool {
  expect External { address, datum_rule } = acc.kind
  expect Some(o) = list.at(self.outputs, leg.output)
  let map = catalogue.asset_map(cat, m.asset)
  expect Some(unit) = map.unit
  let q = m.amount * bytes.pow10(map.exp)
  expect q < bytes.two_63 && leg.allowance >= 0 && leg.allowance < bytes.two_63
  let expected =
    when unit is {
      Lovelace -> assets.from_lovelace(q + leg.allowance)
      Token { policy, name } ->
        assets.from_lovelace(leg.allowance) |> assets.add(policy, name, q)
    }
  let expected_datum =
    when datum_rule is {
      TagDatum -> {
        let tag: Data =
          PayoutTagV2 { thread_id: td.thread_id, seq: td.seq, effect: effect_index }
        InlineDatum(tag)
      }
      PinnedDatum(d) -> InlineDatum(d)
    }
  and {
    o.address == address,
    o.value == expected,
    o.datum == expected_datum,
    o.reference_script == None,
  }
}

// ## Settle: stages
//
// Each stage is a pure function of the stages it takes. The production gate builds a
// stage only after every component that does not need it has held; `diag.settle_facts`
// builds every stage. Stage dependencies:
//   base <- shape digest, effects, claim input, signers
//   effects <- snapshot, scenario hash
//   signers + scenario hash <- messages
//   effects + claim input + shape digest + messages <- statement
//   snapshot + statement <- next datum

/// Stage B: the parsed spend and its catalogue action entry.
pub type SettleBaseV2 {
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  tx: Transaction,
  own: Input,
  own_hash: ByteArray,
  cat: CatalogueV2,
  entry: ActionEntryV2,
  shape: ShapeV2,
  inputs: List<ValueV2>,
  post_states: List<ValueV2>,
  post_cells: List<Int>,
  post_duties: List<DutyValV2>,
  effect_values: List<EffectValV2>,
  intents: List<SignedIntentV2>,
  payouts: List<PayoutLegV2>,
  continuation: Int,
  topup: Int,
}

pub fn settle_base(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> SettleBaseV2 {
  expect Settle {
    action,
    shape,
    inputs,
    post_states,
    post_cells,
    post_duties,
    effects: effect_values,
    intents,
    payouts,
    continuation,
    topup,
  } = r
  let own = own_input(self, own_ref)
  expect Script(own_hash) = own.output.address.payment_credential
  let cat = td.catalogue
  expect td.version == 2
  expect Some(entry) = list.at(cat.actions, action)
  SettleBaseV2 {
    c2_hash,
    td,
    own_ref,
    tx: self,
    own,
    own_hash,
    cat,
    entry,
    shape,
    inputs,
    post_states,
    post_cells,
    post_duties,
    effect_values,
    intents,
    payouts,
    continuation,
    topup,
  }
}

/// Stage SD: the redeemer shape digest (`MRG16-PVC-SHP-02`).
pub fn settle_shape_digest_of(b: SettleBaseV2) -> ByteArray {
  core.blake2b(core.shape_bytes(b.cat, b.entry, b.shape))
}

/// Stage E: typed effects and the pre values S2 authenticates itself.
pub type SettleEffectsV2 {
  effects: List<CoreEffectV2>,
  ms: List<MoveV2>,
  pre_cells: List<Int>,
  pre_duties: List<DutyValV2>,
  premises: List<(CellRefV2, Int)>,
}

pub fn settle_effects(b: SettleBaseV2) -> SettleEffectsV2 {
  let cat = b.cat
  let effects = core.effect_view(cat, b.shape, b.effect_values)
  let ms = core.moves(effects)
  let indexed_cells = list.indexed_map(b.shape.cells, fn(i, c) { (i, c) })
  let pre_cells =
    list.map(
      indexed_cells,
      fn(ic) { core.pre_cell(cat, b.td.balances, ms, ic.1st, ic.2nd) },
    )
  let pre_duties =
    list.map(b.shape.duties, fn(d) { core.pre_duty(b.td.liabilities, d) })
  let premises =
    list.filter_map(
      indexed_cells,
      fn(ic) {
        expect Some(acc) = catalogue.find_account(cat, ic.2nd.account)
        if catalogue.is_custody(acc) {
          None
        } else {
          Some((ic.2nd, core.gross_of_cell(ms, ic.1st)))
        }
      },
    )
  SettleEffectsV2 { effects, ms, pre_cells, pre_duties, premises }
}

/// Stage S: the complete next snapshot. Represented rows take post values, every other
/// row is copied (frame), new registry duties are inserted, nothing is removed. With
/// distinct cells (`settle_shape_cells`) each custody row matches at most one cell.
pub type SettleSnapshotV2 {
  balances: List<BalanceRowV2>,
  liabilities: List<DutyRowV2>,
}

pub fn settle_snapshot(b: SettleBaseV2, e: SettleEffectsV2) -> SettleSnapshotV2 {
  let balances =
    list.map(
      b.td.balances,
      fn(row) {
        when cell_index(b.shape.cells, row.asset, row.account, 0) is {
          Some(i) -> {
            expect Some(v) = list.at(b.post_cells, i)
            BalanceRowV2 { ..row, amount: v }
          }
          None -> row
        }
      },
    )
  let updated_duties =
    list.map(
      b.td.liabilities,
      fn(row) {
        when duty_index(b.shape.duties, row.name, 0) is {
          Some(j) -> {
            expect Some(p) = list.at(b.post_duties, j)
            expect p.present
            DutyRowV2 { ..row, amount: p.amount }
          }
          None -> row
        }
      },
    )
  let liabilities =
    list.foldl(
      list.indexed_map(b.shape.duties, fn(j, d) { (j, d) }),
      updated_duties,
      fn(jd, rows) {
        expect Some(pre) = list.at(e.pre_duties, jd.1st)
        expect Some(post) = list.at(b.post_duties, jd.1st)
        if !pre.present && post.present {
          insert_duty(
            rows,
            DutyRowV2 {
              name: jd.2nd.name,
              asset: jd.2nd.asset,
              debtor: jd.2nd.debtor,
              creditor: jd.2nd.creditor,
              amount: post.amount,
            },
          )
        } else {
          rows
        }
      },
    )
  SettleSnapshotV2 { balances, liabilities }
}

/// Stage C: the one transition claim naming this outref.
pub type SettleClaimV2 {
  raw: Data,
  env: EnvV2,
  input: Input,
}

pub fn settle_claim_input(b: SettleBaseV2) -> SettleClaimV2 {
  let claims =
    list.filter_map(
      b.tx.inputs,
      fn(i) {
        if i.output.address.payment_credential == Script(b.td.claim_hash) {
          expect InlineDatum(raw) = i.output.datum
          expect env: EnvV2 = raw
          if env.settle_outref == b.own_ref {
            Some((raw, env, i))
          } else {
            None
          }
        } else {
          None
        }
      },
    )
  expect [(raw, env, input)] = claims
  SettleClaimV2 { raw, env, input }
}

/// Stage A: each signed intent with its catalogue account (unknown signer halts).
pub type SignerV2 {
  intent: SignedIntentV2,
  account: AccountV2,
}

pub fn settle_signers(b: SettleBaseV2) -> List<SignerV2> {
  list.map(
    b.intents,
    fn(si) {
      expect Some(acc) = catalogue.find_account(b.cat, si.signer)
      SignerV2 { intent: si, account: acc }
    },
  )
}

/// Stage SC: SHA-256 of the canonical Core scenario.
pub fn settle_scenario_hash(b: SettleBaseV2, e: SettleEffectsV2) -> ByteArray {
  builtin.sha2_256(
    intent.scenario_json(
      b.cat,
      b.shape.inputs,
      b.inputs,
      b.td.state,
      b.td.balances,
      e.premises,
      b.td.liabilities,
    ),
  )
}

/// One rebuilt owner-intent signing message and its signature check.
pub type SignedMessageV2 {
  signer: ByteArray,
  message: ByteArray,
  sig: SigCheckV2,
}

/// Stage M: every signing message, rendered from fragments shared by all signers of the
/// step (`intent.intent_shared`, computed once and only when there is a signer), and the
/// intents digest.
pub type SettleMessagesV2 {
  signed: List<SignedMessageV2>,
  intents_digest: ByteArray,
}

pub fn settle_messages(
  b: SettleBaseV2,
  signers: List<SignerV2>,
  scenario_hash: ByteArray,
) -> SettleMessagesV2 {
  let dom =
    IntentDomainV2 {
      action: b.entry.name,
      settle_hash: b.own_hash,
      thread_id: b.td.thread_id,
      predicate_digest: b.entry.predicate_digest,
      vk_digest: b.entry.vk_digest,
      shape_digest: b.entry.shape_digest,
      predecessor: b.own_ref,
      seq: b.td.seq,
      scenario_hash,
    }
  let signed =
    when signers is {
      [] -> []
      _ -> {
        let shared = intent.intent_shared(b.cat, dom)
        list.map(
          signers,
          fn(s) {
            let message =
              intent.signing_message(
                intent.owner_intent_json_shared(b.cat, shared, s.account, s.intent),
              )
            SignedMessageV2 {
              signer: s.intent.signer,
              message,
              sig: SigCheckV2 {
                key: s.account.key,
                digest: builtin.sha2_256(message),
                signature: s.intent.signature,
              },
            }
          },
        )
      }
    }
  let intents_digest =
    core.blake2b(
      bytes.cat(
        list.map(signed, fn(s) { bytes.cat([bytes.lp(s.signer), s.sig.digest]) }),
      ),
    )
  SettleMessagesV2 { signed, intents_digest }
}

/// Stage ST: catalogue digest and the `MRG16-PVC-STMT-2` statement and its digest.
pub type SettleStatementV2 {
  catalogue_digest: ByteArray,
  statement: ByteArray,
  statement_digest: ByteArray,
}

pub fn settle_statement(
  b: SettleBaseV2,
  e: SettleEffectsV2,
  c: SettleClaimV2,
  shape_digest: ByteArray,
  intents_digest: ByteArray,
) -> SettleStatementV2 {
  let catalogue_digest = catalogue.digest(b.cat)
  let statement =
    core.statement(
      StatementCtxV2 {
        predicate_digest: b.entry.predicate_digest,
        shape_digest,
        vk_digest: b.entry.vk_digest,
        claimant_vkh: c.env.claimant_vkh,
        network_id: b.cat.network_id,
        settle_hash: b.own_hash,
        thread_id: b.td.thread_id,
        consumed_txid: b.own_ref.transaction_id,
        consumed_ix: b.own_ref.output_index,
        seq: b.td.seq,
        catalogue_digest,
        intents_digest,
      },
      core.body(
        b.cat,
        b.shape,
        b.inputs,
        b.td.state,
        e.pre_cells,
        e.pre_duties,
        b.post_states,
        b.post_cells,
        b.post_duties,
        e.effects,
      ),
    )
  SettleStatementV2 {
    catalogue_digest,
    statement,
    statement_digest: core.blake2b(statement),
  }
}

/// Stage N: the complete expected continuation datum.
pub fn settle_next(
  b: SettleBaseV2,
  snap: SettleSnapshotV2,
  statement_digest: ByteArray,
) -> ThreadDatumV2 {
  let td = b.td
  ThreadDatumV2 {
    ..td,
    state: b.post_states,
    balances: snap.balances,
    liabilities: snap.liabilities,
    reserve: td.reserve + b.topup,
    seq: td.seq + 1,
    event_chain: core.blake2b(
      bytes.cat([evt_tag, td.event_chain, #"01", statement_digest]),
    ),
  }
}

// ## Settle: components (each defined once; gate order as in P3S1-R1)

pub fn settle_custody_pre(b: SettleBaseV2) -> Bool {
  b.own.output.value == catalogue.custody_value(
    b.cat,
    b.td.balances,
    b.td.reserve,
    b.own_hash,
    b.td.thread_id,
  )
}

pub fn settle_claim_script(b: SettleBaseV2) -> Bool {
  b.td.claim_hash == b.c2_hash
}

pub fn settle_shape_structure(b: SettleBaseV2) -> Bool {
  core.shape_structure_ok(b.cat, b.shape)
}

pub fn settle_shape_cells(b: SettleBaseV2) -> Bool {
  core.cells_ordered(b.shape)
}

pub fn settle_shape_digest(b: SettleBaseV2, shape_digest: ByteArray) -> Bool {
  shape_digest == b.entry.shape_digest
}

/// Defence in depth: post values re-derived from the effects.
pub fn settle_recomputed(b: SettleBaseV2, e: SettleEffectsV2) -> Bool {
  and {
    core.cells_recomputed(e.pre_cells, b.post_cells, e.ms),
    list.length(b.post_duties) == list.length(b.shape.duties),
    list.all(
      list.indexed_map(list.zip(e.pre_duties, b.post_duties), fn(j, p) { (j, p) }),
      fn(jp) { core.duty_recomputed(jp.1st, jp.2nd.1st, jp.2nd.2nd, e.effects) },
    ),
    list.length(b.post_states) == list.length(b.cat.states),
    list.all(
      list.indexed_map(list.zip(b.td.state, b.post_states), fn(k, p) { (k, p) }),
      fn(kp) { core.state_recomputed(kp.1st, kp.2nd.1st, kp.2nd.2nd, e.effects) },
    ),
  }
}

/// Core scenario bounds on the complete snapshot.
pub fn settle_bounds(b: SettleBaseV2, e: SettleEffectsV2) -> Bool {
  let n_bal = list.length(b.td.balances) + list.length(e.premises)
  let n_liab = list.length(b.td.liabilities)
  and {
    n_bal <= 4096,
    n_liab <= 1024,
    ( list.length(b.cat.states) + n_bal + n_liab + b.shape.liability_statements + 1 ) * (
      b.shape.statement_count + 2
    ) <= 65536,
  }
}

pub fn settle_claim(b: SettleBaseV2, c: SettleClaimV2) -> Bool {
  and {
    claim.well_formed(c.env),
    assets.quantity_of(
      c.input.output.value,
      b.td.claim_hash,
      claim.envelope_name(c.raw),
    ) == 1,
    c.env.predicate_digest == b.entry.predicate_digest,
    c.env.vk_digest == b.entry.vk_digest,
  }
}

/// Signed intents: all-account authority and thread-advance authority.
pub fn settle_authority_shape(b: SettleBaseV2, e: SettleEffectsV2) -> Bool {
  let signers = list.map(b.intents, fn(si) { si.signer })
  let required = required_signers(e.effects)
  and {
    !list.is_empty(signers),
    bytes.strictly_ascending(signers, bytes.bytes_lt),
    list.all(required, fn(n) { list.has(signers, n) }),
    list.is_empty(b.entry.advancers) || list.any(
      b.entry.advancers,
      fn(a) { list.has(signers, a) },
    ),
  }
}

pub fn settle_intent_window(b: SettleBaseV2) -> Bool {
  list.all(b.intents, fn(si) { intent.window_ok(si, b.tx) })
}

pub fn settle_intent_effects(
  b: SettleBaseV2,
  e: SettleEffectsV2,
  snap: SettleSnapshotV2,
) -> Bool {
  list.all(
    b.intents,
    fn(si) { core.check_effects(si.signer, si.limits, e.effects, snap.liabilities) },
  )
}

pub fn settle_intent_native(b: SettleBaseV2, signers: List<SignerV2>) -> Bool {
  list.all(
    signers,
    fn(s) {
      when s.account.native is {
        None -> True
        Some(c) -> catalogue.native_ok(c, b.tx)
      }
    },
  )
}

pub fn settle_statement_digest(c: SettleClaimV2, st: SettleStatementV2) -> Bool {
  st.statement_digest == c.env.statement_digest
}

/// Continuation: complete expected datum and exact custody value.
pub fn settle_continuation(
  b: SettleBaseV2,
  snap: SettleSnapshotV2,
  next: ThreadDatumV2,
) -> Bool {
  let next_data: Data = next
  expect Some(cont) = list.at(b.tx.outputs, b.continuation)
  and {
    cont.address == b.own.output.address,
    cont.value == catalogue.custody_value(
      b.cat,
      snap.balances,
      next.reserve,
      b.own_hash,
      b.td.thread_id,
    ),
    cont.datum == InlineDatum(next_data),
    cont.reference_script == None,
  }
}

/// Tagged payouts: exactly one leg per effect crediting an external account.
pub fn settle_payouts(b: SettleBaseV2, e: SettleEffectsV2) -> Bool {
  let indexed_effects = list.indexed_map(e.effects, fn(i, x) { (i, x) })
  let required_payouts =
    list.filter_map(
      indexed_effects,
      fn(ie) {
        when ie.2nd is {
          CMove(m) -> {
            expect Some(acc) = catalogue.find_account(b.cat, m.to)
            if catalogue.is_custody(acc) || m.amount == 0 {
              None
            } else {
              Some((ie.1st, m, acc))
            }
          }
          _ -> None
        }
      },
    )
  and {
    list.map(b.payouts, fn(l) { l.effect }) == list.map(
      required_payouts,
      fn(p) { p.1st },
    ),
    list.all(
      list.zip(b.payouts, required_payouts),
      fn(lp) {
        payout_ok(b.cat, b.td, lp.1st, lp.2nd.1st, lp.2nd.2nd, lp.2nd.3rd, b.tx)
      },
    ),
  }
}

pub fn settle_own_mint(b: SettleBaseV2) -> Bool {
  no_own_mint(b.tx, b.own_hash, b.td.thread_id)
}

pub fn settle_topup(b: SettleBaseV2) -> Bool {
  b.topup >= 0
}

/// The next reserve is `td.reserve + topup` (the `settle_next` field).
pub fn settle_reserve_floor(b: SettleBaseV2) -> Bool {
  b.td.reserve + b.topup >= b.cat.reserve_floor
}

pub fn settle_global_outputs(b: SettleBaseV2) -> Bool {
  global_outputs_distinct(b.own_hash, b.tx)
}

// ## Settle: production gate

/// The production settle gate: every component, in P3S1-R1 gate order, with each stage
/// built only once the components before it hold. `Some(sigs)` exactly when every
/// component holds (the signature checks still to verify); `None` when one is False.
/// Halts where P3S1-R1 halted (`expect`s in the stages and components), so it can refuse
/// by error or by `None`; it never yields `Some` on an input where `diag.settle_facts`
/// halts or reports a False component.
pub fn settle_gate(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> Option<List<SigCheckV2>> {
  let b = settle_base(c2_hash, td, own_ref, r, self)
  let base_ok =
    and {
      settle_custody_pre(b),
      settle_claim_script(b),
      settle_shape_structure(b),
      settle_shape_cells(b),
    }
  if base_ok {
    let shape_digest = settle_shape_digest_of(b)
    if settle_shape_digest(b, shape_digest) {
      let e = settle_effects(b)
      if settle_recomputed(b, e) && settle_bounds(b, e) {
        let c = settle_claim_input(b)
        let claim_stage_ok =
          and {
            settle_claim(b, c),
            settle_authority_shape(b, e),
            settle_intent_window(b),
          }
        if claim_stage_ok {
          let snap = settle_snapshot(b, e)
          if settle_intent_effects(b, e, snap) {
            let signers = settle_signers(b)
            if settle_intent_native(b, signers) {
              let m = settle_messages(b, signers, settle_scenario_hash(b, e))
              let st = settle_statement(b, e, c, shape_digest, m.intents_digest)
              let final_ok =
                and {
                  settle_statement_digest(c, st),
                  settle_continuation(
                    b,
                    snap,
                    settle_next(b, snap, st.statement_digest),
                  ),
                  settle_payouts(b, e),
                  settle_own_mint(b),
                  settle_topup(b),
                  settle_reserve_floor(b),
                  settle_global_outputs(b),
                }
              if final_ok {
                Some(list.map(m.signed, fn(s) { s.sig }))
              } else {
                None
              }
            } else {
              None
            }
          } else {
            None
          }
        } else {
          None
        }
      } else {
        None
      }
    } else {
      None
    }
  } else {
    None
  }
}

/// Every check except signature validity; fails (refuses) unless all hold.
pub fn settle_checks(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> List<SigCheckV2> {
  expect Some(sigs) = settle_gate(c2_hash, td, own_ref, r, self)
  sigs
}

/// Full settlement: every check, then every signature.
pub fn settle(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> Bool {
  when settle_gate(c2_hash, td, own_ref, r, self) is {
    Some(sigs) -> list.all(sigs, intent.verify)
    None -> False
  }
}

// ## Revoke

pub fn revocation_message(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  signer: ByteArray,
) -> ByteArray {
  bytes.cat(
    [
      revoke_tag, #"00", td.thread_id, own_ref.transaction_id,
      bytes.u16(own_ref.output_index), bytes.u64(td.seq), bytes.lp(signer),
    ],
  )
}

/// Parsed revoke: the signer's account, the revocation digest, the expected next datum
/// and the continuation output.
pub type RevokeBaseV2 {
  td: ThreadDatumV2,
  tx: Transaction,
  own: Input,
  own_hash: ByteArray,
  account: AccountV2,
  signature: ByteArray,
  digest: ByteArray,
  next_data: Data,
  cont: Output,
}

pub fn revoke_base(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  signer: ByteArray,
  signature: ByteArray,
  continuation: Int,
  self: Transaction,
) -> RevokeBaseV2 {
  let own = own_input(self, own_ref)
  expect Script(own_hash) = own.output.address.payment_credential
  expect Some(account) = catalogue.find_account(td.catalogue, signer)
  let digest = builtin.sha2_256(revocation_message(td, own_ref, signer))
  let next =
    ThreadDatumV2 {
      ..td,
      seq: td.seq + 1,
      event_chain: core.blake2b(bytes.cat([evt_tag, td.event_chain, #"02", digest])),
    }
  let next_data: Data = next
  expect Some(cont) = list.at(self.outputs, continuation)
  RevokeBaseV2 {
    td,
    tx: self,
    own,
    own_hash,
    account,
    signature,
    digest,
    next_data,
    cont,
  }
}

// Revoke components: the signer kills every outstanding intent on this predecessor by
// advancing `seq` without any other change. Authorized by the signer's intent key over
// the revocation message, or (empty signature) by its native co-authority. Any
// catalogue account may revoke; this is a disclosed liveness lever, not a safety risk.

pub fn revoke_authorized(v: RevokeBaseV2) -> Bool {
  if v.signature == #"" {
    when v.account.native is {
      Some(c) -> catalogue.native_ok(c, v.tx)
      None -> False
    }
  } else {
    intent.verify(
      SigCheckV2 { key: v.account.key, digest: v.digest, signature: v.signature },
    )
  }
}

pub fn revoke_custody_pre(v: RevokeBaseV2) -> Bool {
  v.own.output.value == catalogue.custody_value(
    v.td.catalogue,
    v.td.balances,
    v.td.reserve,
    v.own_hash,
    v.td.thread_id,
  )
}

pub fn revoke_cont_address(v: RevokeBaseV2) -> Bool {
  v.cont.address == v.own.output.address
}

pub fn revoke_cont_value(v: RevokeBaseV2) -> Bool {
  v.cont.value == v.own.output.value
}

pub fn revoke_cont_datum(v: RevokeBaseV2) -> Bool {
  v.cont.datum == InlineDatum(v.next_data)
}

pub fn revoke_cont_ref_script(v: RevokeBaseV2) -> Bool {
  v.cont.reference_script == None
}

pub fn revoke_own_mint(v: RevokeBaseV2) -> Bool {
  no_own_mint(v.tx, v.own_hash, v.td.thread_id)
}

pub fn revoke_global_outputs(v: RevokeBaseV2) -> Bool {
  global_outputs_distinct(v.own_hash, v.tx)
}

/// Production revoke gate (`diag.revoke_parts` names the same calls).
pub fn revoke(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  signer: ByteArray,
  signature: ByteArray,
  continuation: Int,
  self: Transaction,
) -> Bool {
  let v = revoke_base(td, own_ref, signer, signature, continuation, self)
  and {
    revoke_authorized(v),
    revoke_custody_pre(v),
    revoke_cont_address(v),
    revoke_cont_value(v),
    revoke_cont_datum(v),
    revoke_cont_ref_script(v),
    revoke_own_mint(v),
    revoke_global_outputs(v),
  }
}

// ## Close

/// Parsed close: the spent thread and the reserve output.
pub type CloseBaseV2 {
  td: ThreadDatumV2,
  tx: Transaction,
  own: Input,
  own_hash: ByteArray,
  out: Output,
}

pub fn close_base(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  reserve_output: Int,
  self: Transaction,
) -> CloseBaseV2 {
  let own = own_input(self, own_ref)
  expect Script(own_hash) = own.output.address.payment_credential
  expect Some(out) = list.at(self.outputs, reserve_output)
  CloseBaseV2 { td, tx: self, own, own_hash, out }
}

// Close components: only an empty thread with no residual duty can end. Present-zero
// rows may be dropped (no residual); any nonzero liability blocks closing. The thread
// token is burned and the reserve goes exactly to the reserve owner, who authorizes.
// Residual (review RR-08): program state that is not a liability is not protected by
// Close; programs must model obligations as liabilities.

pub fn close_rows_zero(k: CloseBaseV2) -> Bool {
  list.all(k.td.balances, fn(row) { row.amount == 0 })
}

pub fn close_duties_zero(k: CloseBaseV2) -> Bool {
  list.all(k.td.liabilities, fn(d) { d.amount == 0 })
}

pub fn close_custody_pre(k: CloseBaseV2) -> Bool {
  k.own.output.value == catalogue.custody_value(
    k.td.catalogue,
    k.td.balances,
    k.td.reserve,
    k.own_hash,
    k.td.thread_id,
  )
}

pub fn close_burn(k: CloseBaseV2) -> Bool {
  assets.quantity_of(k.tx.mint, k.own_hash, k.td.thread_id) == -1
}

pub fn close_reserve_owner_auth(k: CloseBaseV2) -> Bool {
  catalogue.native_ok(k.td.catalogue.reserve_owner.payment_credential, k.tx)
}

pub fn close_reserve_address(k: CloseBaseV2) -> Bool {
  k.out.address == k.td.catalogue.reserve_owner
}

pub fn close_reserve_value(k: CloseBaseV2) -> Bool {
  k.out.value == assets.from_lovelace(k.td.reserve)
}

pub fn close_reserve_ref_script(k: CloseBaseV2) -> Bool {
  k.out.reference_script == None
}

pub fn close_global_outputs(k: CloseBaseV2) -> Bool {
  global_outputs_distinct(k.own_hash, k.tx)
}

/// Production close gate (`diag.close_parts` names the same calls).
pub fn close(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  reserve_output: Int,
  self: Transaction,
) -> Bool {
  let k = close_base(td, own_ref, reserve_output, self)
  and {
    close_rows_zero(k),
    close_duties_zero(k),
    close_custody_pre(k),
    close_burn(k),
    close_reserve_owner_auth(k),
    close_reserve_address(k),
    close_reserve_value(k),
    close_reserve_ref_script(k),
    close_global_outputs(k),
  }
}
