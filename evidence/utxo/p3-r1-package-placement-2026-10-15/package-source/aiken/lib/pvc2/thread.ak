//// S2: thread genesis, settlement, revocation and closing for ABI v2 (P3S1-R1 successor,
//// PROPOSED, unreviewed, uncompiled). This is the ScriptContext consumer: every fact it
//// binds comes from the transaction, the spent datum or a recomputation; the redeemer
//// only proposes values that the claim digest, the signed intents and the physical checks
//// must all agree with. ABI-V2.md is normative.
////
//// P3S1-R1 changes (review RR-01, RR-02, RR-04): S2 takes the reviewed C2 script hash as
//// a parameter and pins it at genesis and settle; the structural shape gate refuses
//// unsorted or duplicate cells; every production gate (genesis, settle, revoke, close) is
//// a list of named components that the validator conjoins and tests inspect; signer
//// window, `check_effects` and native co-authority are boolean components instead of an
//// `expect`; signed intents use `moriarty-cardano-intent-domain/2`.

use aiken/builtin
use aiken/collection/dict
use aiken/collection/list
use cardano/address.{Script}
use cardano/assets.{PolicyId}
use cardano/transaction.{InlineDatum, Input, OutputReference, Spend, Transaction}
use pvc2/bytes
use pvc2/catalogue
use pvc2/claim
use pvc2/core.{CDuty, CMove, CoreEffectV2, MoveV2, StatementCtxV2}
use pvc2/intent.{IntentDomainV2, SigCheckV2}
use pvc2/types.{
  AccountV2, BalanceRowV2, CatalogueV2, CellRefV2, Close, DutyRefV2, DutyRowV2,
  EnvV2, External, Lovelace, PayoutLegV2, PayoutTagV2, PinnedDatum, Revoke,
  Settle, SettleRedeemerV2, TagDatum, ThreadDatumV2, Token, VBool, VUInt,
  ValueV2,
}

pub const evt_tag: ByteArray = "MRG16-PVC-EVT-02"

pub const revoke_tag: ByteArray = "moriarty-cardano-revoke/1"

// ## Named gate components

/// True when every named component holds.
pub fn all_parts(parts: List<(ByteArray, Bool)>) -> Bool {
  list.all(parts, fn(p) { p.2nd })
}

/// Names of the components that do not hold, in gate order.
pub fn false_parts(parts: List<(ByteArray, Bool)>) -> List<ByteArray> {
  list.filter_map(
    parts,
    fn(p) {
      if p.2nd {
        None
      } else {
        Some(p.1st)
      }
    },
  )
}

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

/// One-shot genesis components: the seed is spent; the token is
/// blake2b_256(serialise_data(seed)); one output at this script with a well-formed datum,
/// seq 0, a zero event chain, an exact custody value and (R1, RR-02) the reviewed claim
/// script `c2_hash` that parameterizes this S2.
pub fn genesis_parts(
  c2_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  expect [Pair(name, qty)] = dict.to_pairs(assets.tokens(self.mint, policy_id))
  expect [out] = list.filter(self.outputs, fn(o) { claim.holds_policy(o, policy_id) })
  expect Script(holder) = out.address.payment_credential
  expect InlineDatum(raw) = out.datum
  expect td: ThreadDatumV2 = raw
  let seed_data: Data = seed
  let cat = td.catalogue
  [
    ("mint_one", qty == 1),
    ("seed_spent", list.any(self.inputs, fn(i) { i.output_reference == seed })),
    (
      "token_name",
      name == builtin.blake2b_256(builtin.serialise_data(seed_data)),
    ),
    ("holder", holder == policy_id),
    ("no_ref_script", out.reference_script == None),
    ("version", td.version == 2),
    ("thread_id", td.thread_id == name),
    (
      "claim_script",
      and {
        bytes.len(td.claim_hash) == 28,
        td.claim_hash != policy_id,
        td.claim_hash == c2_hash,
      },
    ),
    ("seq_zero", td.seq == 0),
    ("chain_zero", td.event_chain == bytes.zero32),
    ("catalogue", catalogue.well_formed(cat)),
    ("state", state_ok(cat, td.state)),
    ("rows", rows_ok(cat, td.balances)),
    ("duties", duties_ok(cat, td.liabilities, self)),
    ("reserve_floor", td.reserve >= cat.reserve_floor),
    (
      "bounds",
      list.length(td.balances) <= 4096 && list.length(td.liabilities) <= 1024,
    ),
    (
      "custody_value",
      out.value == catalogue.custody_value(
        cat,
        td.balances,
        td.reserve,
        policy_id,
        name,
      ),
    ),
  ]
}

pub fn genesis(
  c2_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> Bool {
  all_parts(genesis_parts(c2_hash, policy_id, seed, self))
}

/// Burn path of the thread policy (used by Close): only negative quantities.
pub fn burn_only(policy_id: PolicyId, self: Transaction) -> Bool {
  list.all(dict.values(assets.tokens(self.mint, policy_id)), fn(q) { q < 0 })
}

// ## Settle

fn index_of_cell(cells: List<CellRefV2>, asset: Int, account: ByteArray) -> Option<Int> {
  list.indexed_map(cells, fn(i, c) { (i, c) })
    |> list.find(fn(ic) { ic.2nd.asset == asset && ic.2nd.account == account })
    |> option_first
}

fn option_first(o: Option<(Int, a)>) -> Option<Int> {
  when o is {
    Some((i, _)) -> Some(i)
    None -> None
  }
}

fn index_of_duty(duties: List<DutyRefV2>, name: ByteArray) -> Option<Int> {
  list.indexed_map(duties, fn(i, d) { (i, d) })
    |> list.find(fn(id) { id.2nd.name == name })
    |> option_first
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

/// One verified-intent record (R1): window, `check_effects` and native co-authority are
/// boolean components (in P3S1 they were an `expect`, so a limit breach was an error).
pub type SignerCheckV2 {
  signer: ByteArray,
  message: ByteArray,
  sig: SigCheckV2,
  window: Bool,
  effects: Bool,
  native: Bool,
}

/// Everything S2 derives for one settle step. `parts` are the named components of the
/// production gate in gate order; `gate` is their conjunction (every check except
/// signature validity). Tests compare the digests with the frozen R1 author-oracle values
/// and inspect `parts`.
pub type SettleFactsV2 {
  statement: ByteArray,
  statement_digest: ByteArray,
  shape_digest: ByteArray,
  catalogue_digest: ByteArray,
  scenario_hash: ByteArray,
  intents_digest: ByteArray,
  messages: List<(ByteArray, ByteArray)>,
  next: ThreadDatumV2,
  sigs: List<SigCheckV2>,
  parts: List<(ByteArray, Bool)>,
  gate: Bool,
}

pub fn settle_facts(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> SettleFactsV2 {
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
  // Structural shape gate (R1: two named components instead of one `expect`).
  let shape_structure = core.shape_structure_ok(cat, shape)
  let shape_cells = core.cells_ordered(shape)
  let shape_digest = core.blake2b(core.shape_bytes(cat, entry, shape))

  // Typed effects and the pre values S2 authenticates itself.
  let effects = core.effect_view(cat, shape, effect_values)
  let ms = core.moves(effects)
  let indexed_cells = list.indexed_map(shape.cells, fn(i, c) { (i, c) })
  let pre_cells =
    list.map(indexed_cells, fn(ic) { core.pre_cell(cat, td.balances, ms, ic.1st, ic.2nd) })
  let pre_duties = list.map(shape.duties, fn(d) { core.pre_duty(td.liabilities, d) })
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

  // Defence in depth: post values re-derived from the effects.
  let recomputed =
    and {
      core.cells_recomputed(pre_cells, post_cells, ms),
      list.length(post_duties) == list.length(shape.duties),
      list.all(
        list.indexed_map(list.zip(pre_duties, post_duties), fn(j, p) { (j, p) }),
        fn(jp) { core.duty_recomputed(jp.1st, jp.2nd.1st, jp.2nd.2nd, effects) },
      ),
      list.length(post_states) == list.length(cat.states),
      list.all(
        list.indexed_map(list.zip(td.state, post_states), fn(k, p) { (k, p) }),
        fn(kp) { core.state_recomputed(kp.1st, kp.2nd.1st, kp.2nd.2nd, effects) },
      ),
    }

  // Complete next snapshot: represented rows take post values, every other row is
  // copied (frame), new registry duties are inserted, nothing is removed. With distinct
  // cells (shape_cells) each custody row matches at most one represented cell.
  let new_balances =
    list.map(
      td.balances,
      fn(row) {
        when index_of_cell(shape.cells, row.asset, row.account) is {
          Some(i) -> {
            expect Some(v) = list.at(post_cells, i)
            BalanceRowV2 { ..row, amount: v }
          }
          None -> row
        }
      },
    )
  let updated_duties =
    list.map(
      td.liabilities,
      fn(row) {
        when index_of_duty(shape.duties, row.name) is {
          Some(j) -> {
            expect Some(p) = list.at(post_duties, j)
            expect p.present
            DutyRowV2 { ..row, amount: p.amount }
          }
          None -> row
        }
      },
    )
  let new_liabilities =
    list.foldl(
      list.indexed_map(shape.duties, fn(j, d) { (j, d) }),
      updated_duties,
      fn(jd, rows) {
        expect Some(pre) = list.at(pre_duties, jd.1st)
        expect Some(post) = list.at(post_duties, jd.1st)
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

  // Core scenario bounds on the complete snapshot.
  let n_bal = list.length(td.balances) + list.length(premises)
  let n_liab = list.length(td.liabilities)
  let bounds_ok =
    and {
      n_bal <= 4096,
      n_liab <= 1024,
      ( list.length(cat.states) + n_bal + n_liab + shape.liability_statements + 1 ) * (
        shape.statement_count + 2
      ) <= 65536,
    }

  // The one transition claim naming this outref.
  let claims =
    list.filter_map(
      self.inputs,
      fn(i) {
        if i.output.address.payment_credential == Script(td.claim_hash) {
          expect InlineDatum(raw) = i.output.datum
          expect env: EnvV2 = raw
          if env.settle_outref == own_ref {
            Some((raw, env, i))
          } else {
            None
          }
        } else {
          None
        }
      },
    )
  expect [(claim_raw, env, claim_in)] = claims
  let claim_ok =
    and {
      claim.well_formed(env),
      assets.quantity_of(
        claim_in.output.value,
        td.claim_hash,
        claim.envelope_name(claim_raw),
      ) == 1,
      env.predicate_digest == entry.predicate_digest,
      env.vk_digest == entry.vk_digest,
    }

  // Signed intents: all-account authority and thread-advance authority.
  let signers = list.map(intents, fn(si) { si.signer })
  let required = required_signers(effects)
  let authority_shape_ok =
    and {
      !list.is_empty(signers),
      bytes.strictly_ascending(signers, bytes.bytes_lt),
      list.all(required, fn(n) { list.has(signers, n) }),
      list.is_empty(entry.advancers) || list.any(
        entry.advancers,
        fn(a) { list.has(signers, a) },
      ),
    }
  let scenario_hash =
    builtin.sha2_256(
      intent.scenario_json(
        cat,
        shape.inputs,
        inputs,
        td.state,
        td.balances,
        premises,
        td.liabilities,
      ),
    )
  let dom =
    IntentDomainV2 {
      action: entry.name,
      settle_hash: own_hash,
      thread_id: td.thread_id,
      predicate_digest: entry.predicate_digest,
      vk_digest: entry.vk_digest,
      shape_digest: entry.shape_digest,
      predecessor: own_ref,
      seq: td.seq,
      scenario_hash,
    }
  let checked =
    list.map(
      intents,
      fn(si) {
        expect Some(acc) = catalogue.find_account(cat, si.signer)
        let message =
          intent.signing_message(intent.owner_intent_json(cat, dom, acc, si))
        let native_present =
          when acc.native is {
            None -> True
            Some(c) -> catalogue.native_ok(c, self)
          }
        SignerCheckV2 {
          signer: si.signer,
          message,
          sig: SigCheckV2 {
            key: acc.key,
            digest: builtin.sha2_256(message),
            signature: si.signature,
          },
          window: intent.window_ok(si, self),
          effects: core.check_effects(si.signer, si.limits, effects, new_liabilities),
          native: native_present,
        }
      },
    )
  let intents_digest =
    core.blake2b(
      bytes.cat(
        list.map(
          checked,
          fn(c) { bytes.cat([bytes.lp(c.signer), c.sig.digest]) },
        ),
      ),
    )

  // Statement and claim digest.
  let catalogue_digest = catalogue.digest(cat)
  let statement =
    core.statement(
      StatementCtxV2 {
        predicate_digest: entry.predicate_digest,
        shape_digest,
        vk_digest: entry.vk_digest,
        claimant_vkh: env.claimant_vkh,
        network_id: cat.network_id,
        settle_hash: own_hash,
        thread_id: td.thread_id,
        consumed_txid: own_ref.transaction_id,
        consumed_ix: own_ref.output_index,
        seq: td.seq,
        catalogue_digest,
        intents_digest,
      },
      core.body(
        cat,
        shape,
        inputs,
        td.state,
        pre_cells,
        pre_duties,
        post_states,
        post_cells,
        post_duties,
        effects,
      ),
    )
  let statement_digest = core.blake2b(statement)

  // Continuation: complete expected datum and exact custody value.
  let next =
    ThreadDatumV2 {
      ..td,
      state: post_states,
      balances: new_balances,
      liabilities: new_liabilities,
      reserve: td.reserve + topup,
      seq: td.seq + 1,
      event_chain: core.blake2b(
        bytes.cat([evt_tag, td.event_chain, #"01", statement_digest]),
      ),
    }
  let next_data: Data = next
  expect Some(cont) = list.at(self.outputs, continuation)
  let continuation_ok =
    and {
      cont.address == own.output.address,
      cont.value == catalogue.custody_value(
        cat,
        new_balances,
        next.reserve,
        own_hash,
        td.thread_id,
      ),
      cont.datum == InlineDatum(next_data),
      cont.reference_script == None,
    }

  // Tagged payouts: exactly one leg per effect crediting an external account.
  let indexed_effects = list.indexed_map(effects, fn(i, e) { (i, e) })
  let required_payouts =
    list.filter_map(
      indexed_effects,
      fn(ie) {
        when ie.2nd is {
          CMove(m) -> {
            expect Some(acc) = catalogue.find_account(cat, m.to)
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
  let payouts_ok =
    and {
      list.map(payouts, fn(l) { l.effect }) == list.map(
        required_payouts,
        fn(p) { p.1st },
      ),
      list.all(
        list.zip(payouts, required_payouts),
        fn(lp) { payout_ok(cat, td, lp.1st, lp.2nd.1st, lp.2nd.2nd, lp.2nd.3rd, self) },
      ),
    }

  // The production gate as named components, every one evaluated (gate order).
  let parts =
    [
      (
        "custody_pre",
        own.output.value == catalogue.custody_value(
          cat,
          td.balances,
          td.reserve,
          own_hash,
          td.thread_id,
        ),
      ),
      ("claim_script", td.claim_hash == c2_hash),
      ("shape_structure", shape_structure),
      ("shape_cells", shape_cells),
      ("shape_digest", shape_digest == entry.shape_digest),
      ("recomputed", recomputed),
      ("bounds", bounds_ok),
      ("claim", claim_ok),
      ("authority_shape", authority_shape_ok),
      ("intent_window", list.all(checked, fn(c) { c.window })),
      ("intent_effects", list.all(checked, fn(c) { c.effects })),
      ("intent_native", list.all(checked, fn(c) { c.native })),
      ("statement_digest", statement_digest == env.statement_digest),
      ("continuation", continuation_ok),
      ("payouts", payouts_ok),
      ("own_mint", no_own_mint(self, own_hash, td.thread_id)),
      ("topup", topup >= 0),
      ("reserve_floor", next.reserve >= cat.reserve_floor),
      ("global_outputs", global_outputs_distinct(own_hash, self)),
    ]
  SettleFactsV2 {
    statement,
    statement_digest,
    shape_digest,
    catalogue_digest,
    scenario_hash,
    intents_digest,
    messages: list.map(checked, fn(c) { (c.signer, c.message) }),
    next,
    sigs: list.map(checked, fn(c) { c.sig }),
    parts,
    gate: all_parts(parts),
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
  let facts = settle_facts(c2_hash, td, own_ref, r, self)
  expect facts.gate
  facts.sigs
}

/// Full settlement: every check, then every signature.
pub fn settle(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> Bool {
  list.all(settle_checks(c2_hash, td, own_ref, r, self), intent.verify)
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

/// Revoke components: the signer kills every outstanding intent on this predecessor by
/// advancing `seq` without any other change. Authorized by the signer's intent key over
/// the revocation message, or (empty signature) by its native co-authority. Any
/// catalogue account may revoke; this is a disclosed liveness lever, not a safety risk.
pub fn revoke_parts(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  signer: ByteArray,
  signature: ByteArray,
  continuation: Int,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  let own = own_input(self, own_ref)
  expect Script(own_hash) = own.output.address.payment_credential
  expect Some(acc) = catalogue.find_account(td.catalogue, signer)
  let digest = builtin.sha2_256(revocation_message(td, own_ref, signer))
  let authorized =
    if signature == #"" {
      when acc.native is {
        Some(c) -> catalogue.native_ok(c, self)
        None -> False
      }
    } else {
      intent.verify(SigCheckV2 { key: acc.key, digest, signature })
    }
  let next =
    ThreadDatumV2 {
      ..td,
      seq: td.seq + 1,
      event_chain: core.blake2b(bytes.cat([evt_tag, td.event_chain, #"02", digest])),
    }
  let next_data: Data = next
  expect Some(cont) = list.at(self.outputs, continuation)
  [
    ("authorized", authorized),
    (
      "custody_pre",
      own.output.value == catalogue.custody_value(
        td.catalogue,
        td.balances,
        td.reserve,
        own_hash,
        td.thread_id,
      ),
    ),
    ("cont_address", cont.address == own.output.address),
    ("cont_value", cont.value == own.output.value),
    ("cont_datum", cont.datum == InlineDatum(next_data)),
    ("cont_ref_script", cont.reference_script == None),
    ("own_mint", no_own_mint(self, own_hash, td.thread_id)),
    ("global_outputs", global_outputs_distinct(own_hash, self)),
  ]
}

pub fn revoke(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  signer: ByteArray,
  signature: ByteArray,
  continuation: Int,
  self: Transaction,
) -> Bool {
  all_parts(revoke_parts(td, own_ref, signer, signature, continuation, self))
}

// ## Close

/// Close components: only an empty thread with no residual duty can end. Present-zero
/// rows may be dropped (no residual); any nonzero liability blocks closing. The thread
/// token is burned and the reserve goes exactly to the reserve owner, who authorizes.
/// Residual (review RR-08): program state that is not a liability is not protected by
/// Close; programs must model obligations as liabilities.
pub fn close_parts(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  reserve_output: Int,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  let own = own_input(self, own_ref)
  expect Script(own_hash) = own.output.address.payment_credential
  let cat = td.catalogue
  expect Some(out) = list.at(self.outputs, reserve_output)
  [
    ("rows_zero", list.all(td.balances, fn(row) { row.amount == 0 })),
    ("duties_zero", list.all(td.liabilities, fn(d) { d.amount == 0 })),
    (
      "custody_pre",
      own.output.value == catalogue.custody_value(
        cat,
        td.balances,
        td.reserve,
        own_hash,
        td.thread_id,
      ),
    ),
    ("burn", assets.quantity_of(self.mint, own_hash, td.thread_id) == -1),
    (
      "reserve_owner_auth",
      catalogue.native_ok(cat.reserve_owner.payment_credential, self),
    ),
    ("reserve_address", out.address == cat.reserve_owner),
    ("reserve_value", out.value == assets.from_lovelace(td.reserve)),
    ("reserve_ref_script", out.reference_script == None),
    ("global_outputs", global_outputs_distinct(own_hash, self)),
  ]
}

pub fn close(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  reserve_output: Int,
  self: Transaction,
) -> Bool {
  all_parts(close_parts(td, own_ref, reserve_output, self))
}
