//// W0 per-component cost benchmarks (PROPOSED; written by hand, never compiled or run by
//// their author). Contexts: P01, P04 and both threads of P07 (P07.0, P07.1).
////
//// Each test decodes one fixture context through `test_ctx` (harness decode, NOT the
//// compiled ledger wrapper), builds ONLY the settle stages its subject needs, and then
//// evaluates that subject once. No test builds `diag.settle_facts` or a named parts list
//// except `*_diag_gate`, which measures exactly that. The attribution rule, tolerance and
//// falsifiers are fixed in COST-ATTRIBUTION-PLAN.json before any run; the reported
//// mem/cpu are host CEK observations, not ledger ExUnits and not predicted values.
////
//// Expected truth: every component is True on these POSITIVE contexts
//// (EXPECTED-CASES-R1.json `falseParts: []`, R1 author, pre-W1). Stage tests return a
//// cheap structural forcing predicate (W0-author derived, labelled separately in
//// EXPECTED-COMPONENTS.json). Gate tests expect one signature check per intent (2), as the
//// R1 `accepted(ctx, 2)` tests do.

use aiken/collection/list
use cardano/transaction.{OutputReference, Transaction}
use pvc2/bytes
use pvc2/diag
use pvc2/fixtures_r1 as fx
use pvc2/test_ctx
use pvc2/thread.{
  SettleBaseV2, SettleClaimV2, SettleEffectsV2, SettleSnapshotV2,
  SettleStatementV2,
}
use pvc2/types.{Settle, SettleRedeemerV2, ThreadDatumV2}

// ## Harness

fn decoded(
  ctx: ByteArray,
) -> (ThreadDatumV2, OutputReference, SettleRedeemerV2, Transaction) {
  let (own_ref, raw, rd, tx) = test_ctx.spending_of(ctx)
  expect td: ThreadDatumV2 = raw
  expect r: SettleRedeemerV2 = rd
  (td, own_ref, r, tx)
}

fn base(ctx: ByteArray) -> SettleBaseV2 {
  let (td, own_ref, r, tx) = decoded(ctx)
  thread.settle_base(fx.c2_hash, td, own_ref, r, tx)
}

/// Stages B, SD, E, C, A, SC, M, ST (the statement closure).
fn upto_statement(
  ctx: ByteArray,
) -> (SettleBaseV2, SettleEffectsV2, SettleClaimV2, SettleStatementV2) {
  let b = base(ctx)
  let shape_digest = thread.settle_shape_digest_of(b)
  let e = thread.settle_effects(b)
  let c = thread.settle_claim_input(b)
  let signers = thread.settle_signers(b)
  let m =
    thread.settle_messages(b, signers, thread.settle_scenario_hash(b, e))
  (b, e, c, thread.settle_statement(b, e, c, shape_digest, m.intents_digest))
}

/// Statement closure plus stages S and N.
fn upto_next(ctx: ByteArray) -> (SettleBaseV2, SettleSnapshotV2, ThreadDatumV2) {
  let (b, e, _, st) = upto_statement(ctx)
  let snap = thread.settle_snapshot(b, e)
  (b, snap, thread.settle_next(b, snap, st.statement_digest))
}

// ## Stage benches (cumulative closures; see COST-ATTRIBUTION-PLAN.json)

/// D: decode only (same shape as R1 `sequential_chain_reserve_r1`).
fn stage_decode(ctx: ByteArray) -> Bool {
  let (td, _, r, _) = decoded(ctx)
  when r is {
    Settle { .. } -> td.version == 2
    _ -> False
  }
}

/// D + B.
fn stage_base(ctx: ByteArray) -> Bool {
  bytes.len(base(ctx).own_hash) == 28
}

/// D + B + SD.
fn stage_shape_digest(ctx: ByteArray) -> Bool {
  bytes.len(thread.settle_shape_digest_of(base(ctx))) == 32
}

/// D + B + E.
fn stage_effects(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  let e = thread.settle_effects(b)
  list.length(e.pre_cells) == list.length(b.shape.cells)
}

/// D + B + E + S.
fn stage_snapshot(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  let snap = thread.settle_snapshot(b, thread.settle_effects(b))
  list.length(snap.balances) == list.length(b.td.balances)
}

/// D + B + C.
fn stage_claim_input(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_claim_input(b).env.settle_outref == b.own_ref
}

/// D + B + A.
fn stage_signers(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  list.length(thread.settle_signers(b)) == list.length(b.intents)
}

/// D + B + E + SC.
fn stage_scenario(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  bytes.len(thread.settle_scenario_hash(b, thread.settle_effects(b))) == 32
}

/// D + B + E + A + SC + M.
fn stage_messages(ctx: ByteArray, n: Int) -> Bool {
  let b = base(ctx)
  let e = thread.settle_effects(b)
  let signers = thread.settle_signers(b)
  let m =
    thread.settle_messages(b, signers, thread.settle_scenario_hash(b, e))
  list.length(m.signed) == n
}

/// Statement closure (D + B + SD + E + C + A + SC + M + ST).
fn stage_statement(ctx: ByteArray) -> Bool {
  let (_, _, _, st) = upto_statement(ctx)
  bytes.len(st.statement_digest) == 32
}

/// Statement closure + S + N.
fn stage_next(ctx: ByteArray) -> Bool {
  let (b, _, next) = upto_next(ctx)
  next.seq == b.td.seq + 1
}

/// Production gate (lean W1 path, every stage, no signatures verified).
fn gate(ctx: ByteArray, n: Int) -> Bool {
  let (td, own_ref, r, tx) = decoded(ctx)
  when thread.settle_gate(fx.c2_hash, td, own_ref, r, tx) is {
    Some(sigs) -> list.length(sigs) == n
    None -> False
  }
}

/// Named diagnostics (every stage and every component, labels and lists).
fn diag_gate(ctx: ByteArray) -> Bool {
  let (td, own_ref, r, tx) = decoded(ctx)
  diag.settle_facts(fx.c2_hash, td, own_ref, r, tx).gate
}

// ## Component benches: closure of the component's stages, then the component once

fn c_custody_pre(ctx: ByteArray) -> Bool {
  thread.settle_custody_pre(base(ctx))
}

fn c_claim_script(ctx: ByteArray) -> Bool {
  thread.settle_claim_script(base(ctx))
}

fn c_shape_structure(ctx: ByteArray) -> Bool {
  thread.settle_shape_structure(base(ctx))
}

fn c_shape_cells(ctx: ByteArray) -> Bool {
  thread.settle_shape_cells(base(ctx))
}

fn c_shape_digest(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_shape_digest(b, thread.settle_shape_digest_of(b))
}

fn c_recomputed(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_recomputed(b, thread.settle_effects(b))
}

fn c_bounds(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_bounds(b, thread.settle_effects(b))
}

fn c_claim(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_claim(b, thread.settle_claim_input(b))
}

fn c_authority_shape(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_authority_shape(b, thread.settle_effects(b))
}

fn c_intent_window(ctx: ByteArray) -> Bool {
  thread.settle_intent_window(base(ctx))
}

fn c_intent_effects(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  let e = thread.settle_effects(b)
  thread.settle_intent_effects(b, e, thread.settle_snapshot(b, e))
}

fn c_intent_native(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_intent_native(b, thread.settle_signers(b))
}

fn c_statement_digest(ctx: ByteArray) -> Bool {
  let (_, _, c, st) = upto_statement(ctx)
  thread.settle_statement_digest(c, st)
}

fn c_continuation(ctx: ByteArray) -> Bool {
  let (b, snap, next) = upto_next(ctx)
  thread.settle_continuation(b, snap, next)
}

fn c_payouts(ctx: ByteArray) -> Bool {
  let b = base(ctx)
  thread.settle_payouts(b, thread.settle_effects(b))
}

fn c_own_mint(ctx: ByteArray) -> Bool {
  thread.settle_own_mint(base(ctx))
}

fn c_topup(ctx: ByteArray) -> Bool {
  thread.settle_topup(base(ctx))
}

fn c_reserve_floor(ctx: ByteArray) -> Bool {
  thread.settle_reserve_floor(base(ctx))
}

fn c_global_outputs(ctx: ByteArray) -> Bool {
  thread.settle_global_outputs(base(ctx))
}

// ## P01

test w0_p01_decode() {
  stage_decode(fx.ctx_p01)
}

test w0_p01_stage_base() {
  stage_base(fx.ctx_p01)
}

test w0_p01_stage_shape_digest() {
  stage_shape_digest(fx.ctx_p01)
}

test w0_p01_stage_effects() {
  stage_effects(fx.ctx_p01)
}

test w0_p01_stage_snapshot() {
  stage_snapshot(fx.ctx_p01)
}

test w0_p01_stage_claim_input() {
  stage_claim_input(fx.ctx_p01)
}

test w0_p01_stage_signers() {
  stage_signers(fx.ctx_p01)
}

test w0_p01_stage_scenario() {
  stage_scenario(fx.ctx_p01)
}

test w0_p01_stage_messages() {
  stage_messages(fx.ctx_p01, 2)
}

test w0_p01_stage_statement() {
  stage_statement(fx.ctx_p01)
}

test w0_p01_stage_next() {
  stage_next(fx.ctx_p01)
}

test w0_p01_gate() {
  gate(fx.ctx_p01, 2)
}

test w0_p01_diag_gate() {
  diag_gate(fx.ctx_p01)
}

test w0_p01_custody_pre() {
  c_custody_pre(fx.ctx_p01)
}

test w0_p01_claim_script() {
  c_claim_script(fx.ctx_p01)
}

test w0_p01_shape_structure() {
  c_shape_structure(fx.ctx_p01)
}

test w0_p01_shape_cells() {
  c_shape_cells(fx.ctx_p01)
}

test w0_p01_shape_digest() {
  c_shape_digest(fx.ctx_p01)
}

test w0_p01_recomputed() {
  c_recomputed(fx.ctx_p01)
}

test w0_p01_bounds() {
  c_bounds(fx.ctx_p01)
}

test w0_p01_claim() {
  c_claim(fx.ctx_p01)
}

test w0_p01_authority_shape() {
  c_authority_shape(fx.ctx_p01)
}

test w0_p01_intent_window() {
  c_intent_window(fx.ctx_p01)
}

test w0_p01_intent_effects() {
  c_intent_effects(fx.ctx_p01)
}

test w0_p01_intent_native() {
  c_intent_native(fx.ctx_p01)
}

test w0_p01_statement_digest() {
  c_statement_digest(fx.ctx_p01)
}

test w0_p01_continuation() {
  c_continuation(fx.ctx_p01)
}

test w0_p01_payouts() {
  c_payouts(fx.ctx_p01)
}

test w0_p01_own_mint() {
  c_own_mint(fx.ctx_p01)
}

test w0_p01_topup() {
  c_topup(fx.ctx_p01)
}

test w0_p01_reserve_floor() {
  c_reserve_floor(fx.ctx_p01)
}

test w0_p01_global_outputs() {
  c_global_outputs(fx.ctx_p01)
}

// ## P04

test w0_p04_decode() {
  stage_decode(fx.ctx_p04)
}

test w0_p04_stage_base() {
  stage_base(fx.ctx_p04)
}

test w0_p04_stage_shape_digest() {
  stage_shape_digest(fx.ctx_p04)
}

test w0_p04_stage_effects() {
  stage_effects(fx.ctx_p04)
}

test w0_p04_stage_snapshot() {
  stage_snapshot(fx.ctx_p04)
}

test w0_p04_stage_claim_input() {
  stage_claim_input(fx.ctx_p04)
}

test w0_p04_stage_signers() {
  stage_signers(fx.ctx_p04)
}

test w0_p04_stage_scenario() {
  stage_scenario(fx.ctx_p04)
}

test w0_p04_stage_messages() {
  stage_messages(fx.ctx_p04, 2)
}

test w0_p04_stage_statement() {
  stage_statement(fx.ctx_p04)
}

test w0_p04_stage_next() {
  stage_next(fx.ctx_p04)
}

test w0_p04_gate() {
  gate(fx.ctx_p04, 2)
}

test w0_p04_diag_gate() {
  diag_gate(fx.ctx_p04)
}

test w0_p04_custody_pre() {
  c_custody_pre(fx.ctx_p04)
}

test w0_p04_claim_script() {
  c_claim_script(fx.ctx_p04)
}

test w0_p04_shape_structure() {
  c_shape_structure(fx.ctx_p04)
}

test w0_p04_shape_cells() {
  c_shape_cells(fx.ctx_p04)
}

test w0_p04_shape_digest() {
  c_shape_digest(fx.ctx_p04)
}

test w0_p04_recomputed() {
  c_recomputed(fx.ctx_p04)
}

test w0_p04_bounds() {
  c_bounds(fx.ctx_p04)
}

test w0_p04_claim() {
  c_claim(fx.ctx_p04)
}

test w0_p04_authority_shape() {
  c_authority_shape(fx.ctx_p04)
}

test w0_p04_intent_window() {
  c_intent_window(fx.ctx_p04)
}

test w0_p04_intent_effects() {
  c_intent_effects(fx.ctx_p04)
}

test w0_p04_intent_native() {
  c_intent_native(fx.ctx_p04)
}

test w0_p04_statement_digest() {
  c_statement_digest(fx.ctx_p04)
}

test w0_p04_continuation() {
  c_continuation(fx.ctx_p04)
}

test w0_p04_payouts() {
  c_payouts(fx.ctx_p04)
}

test w0_p04_own_mint() {
  c_own_mint(fx.ctx_p04)
}

test w0_p04_topup() {
  c_topup(fx.ctx_p04)
}

test w0_p04_reserve_floor() {
  c_reserve_floor(fx.ctx_p04)
}

test w0_p04_global_outputs() {
  c_global_outputs(fx.ctx_p04)
}

// ## P07.0 (thread A of the atomic two-thread transaction)

test w0_p07_0_decode() {
  stage_decode(fx.ctx_p07_0)
}

test w0_p07_0_stage_base() {
  stage_base(fx.ctx_p07_0)
}

test w0_p07_0_stage_shape_digest() {
  stage_shape_digest(fx.ctx_p07_0)
}

test w0_p07_0_stage_effects() {
  stage_effects(fx.ctx_p07_0)
}

test w0_p07_0_stage_snapshot() {
  stage_snapshot(fx.ctx_p07_0)
}

test w0_p07_0_stage_claim_input() {
  stage_claim_input(fx.ctx_p07_0)
}

test w0_p07_0_stage_signers() {
  stage_signers(fx.ctx_p07_0)
}

test w0_p07_0_stage_scenario() {
  stage_scenario(fx.ctx_p07_0)
}

test w0_p07_0_stage_messages() {
  stage_messages(fx.ctx_p07_0, 2)
}

test w0_p07_0_stage_statement() {
  stage_statement(fx.ctx_p07_0)
}

test w0_p07_0_stage_next() {
  stage_next(fx.ctx_p07_0)
}

test w0_p07_0_gate() {
  gate(fx.ctx_p07_0, 2)
}

test w0_p07_0_diag_gate() {
  diag_gate(fx.ctx_p07_0)
}

test w0_p07_0_custody_pre() {
  c_custody_pre(fx.ctx_p07_0)
}

test w0_p07_0_claim_script() {
  c_claim_script(fx.ctx_p07_0)
}

test w0_p07_0_shape_structure() {
  c_shape_structure(fx.ctx_p07_0)
}

test w0_p07_0_shape_cells() {
  c_shape_cells(fx.ctx_p07_0)
}

test w0_p07_0_shape_digest() {
  c_shape_digest(fx.ctx_p07_0)
}

test w0_p07_0_recomputed() {
  c_recomputed(fx.ctx_p07_0)
}

test w0_p07_0_bounds() {
  c_bounds(fx.ctx_p07_0)
}

test w0_p07_0_claim() {
  c_claim(fx.ctx_p07_0)
}

test w0_p07_0_authority_shape() {
  c_authority_shape(fx.ctx_p07_0)
}

test w0_p07_0_intent_window() {
  c_intent_window(fx.ctx_p07_0)
}

test w0_p07_0_intent_effects() {
  c_intent_effects(fx.ctx_p07_0)
}

test w0_p07_0_intent_native() {
  c_intent_native(fx.ctx_p07_0)
}

test w0_p07_0_statement_digest() {
  c_statement_digest(fx.ctx_p07_0)
}

test w0_p07_0_continuation() {
  c_continuation(fx.ctx_p07_0)
}

test w0_p07_0_payouts() {
  c_payouts(fx.ctx_p07_0)
}

test w0_p07_0_own_mint() {
  c_own_mint(fx.ctx_p07_0)
}

test w0_p07_0_topup() {
  c_topup(fx.ctx_p07_0)
}

test w0_p07_0_reserve_floor() {
  c_reserve_floor(fx.ctx_p07_0)
}

test w0_p07_0_global_outputs() {
  c_global_outputs(fx.ctx_p07_0)
}

// ## P07.1 (thread B of the atomic two-thread transaction)

test w0_p07_1_decode() {
  stage_decode(fx.ctx_p07_1)
}

test w0_p07_1_stage_base() {
  stage_base(fx.ctx_p07_1)
}

test w0_p07_1_stage_shape_digest() {
  stage_shape_digest(fx.ctx_p07_1)
}

test w0_p07_1_stage_effects() {
  stage_effects(fx.ctx_p07_1)
}

test w0_p07_1_stage_snapshot() {
  stage_snapshot(fx.ctx_p07_1)
}

test w0_p07_1_stage_claim_input() {
  stage_claim_input(fx.ctx_p07_1)
}

test w0_p07_1_stage_signers() {
  stage_signers(fx.ctx_p07_1)
}

test w0_p07_1_stage_scenario() {
  stage_scenario(fx.ctx_p07_1)
}

test w0_p07_1_stage_messages() {
  stage_messages(fx.ctx_p07_1, 2)
}

test w0_p07_1_stage_statement() {
  stage_statement(fx.ctx_p07_1)
}

test w0_p07_1_stage_next() {
  stage_next(fx.ctx_p07_1)
}

test w0_p07_1_gate() {
  gate(fx.ctx_p07_1, 2)
}

test w0_p07_1_diag_gate() {
  diag_gate(fx.ctx_p07_1)
}

test w0_p07_1_custody_pre() {
  c_custody_pre(fx.ctx_p07_1)
}

test w0_p07_1_claim_script() {
  c_claim_script(fx.ctx_p07_1)
}

test w0_p07_1_shape_structure() {
  c_shape_structure(fx.ctx_p07_1)
}

test w0_p07_1_shape_cells() {
  c_shape_cells(fx.ctx_p07_1)
}

test w0_p07_1_shape_digest() {
  c_shape_digest(fx.ctx_p07_1)
}

test w0_p07_1_recomputed() {
  c_recomputed(fx.ctx_p07_1)
}

test w0_p07_1_bounds() {
  c_bounds(fx.ctx_p07_1)
}

test w0_p07_1_claim() {
  c_claim(fx.ctx_p07_1)
}

test w0_p07_1_authority_shape() {
  c_authority_shape(fx.ctx_p07_1)
}

test w0_p07_1_intent_window() {
  c_intent_window(fx.ctx_p07_1)
}

test w0_p07_1_intent_effects() {
  c_intent_effects(fx.ctx_p07_1)
}

test w0_p07_1_intent_native() {
  c_intent_native(fx.ctx_p07_1)
}

test w0_p07_1_statement_digest() {
  c_statement_digest(fx.ctx_p07_1)
}

test w0_p07_1_continuation() {
  c_continuation(fx.ctx_p07_1)
}

test w0_p07_1_payouts() {
  c_payouts(fx.ctx_p07_1)
}

test w0_p07_1_own_mint() {
  c_own_mint(fx.ctx_p07_1)
}

test w0_p07_1_topup() {
  c_topup(fx.ctx_p07_1)
}

test w0_p07_1_reserve_floor() {
  c_reserve_floor(fx.ctx_p07_1)
}

test w0_p07_1_global_outputs() {
  c_global_outputs(fx.ctx_p07_1)
}
