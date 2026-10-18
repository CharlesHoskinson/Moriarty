//// Named gate diagnostics (W1, TEST-ONLY; PROPOSED, uncompiled). No validator and no
//// production module imports this module, so its labels and lists are not reachable from
//// the compiled C2/S2 scripts. Every entry calls the production component function from
//// `pvc2/thread` or `pvc2/claim` on stage values built by the production stage functions;
//// nothing here re-implements a condition. Lists keep the P3S1-R1 component names and
//// order, so `false_parts` reproduces the frozen `falseParts` vocabulary.
////
//// Source argument (not a formal proof): for each family the production gate conjoins
//// exactly the functions listed here, on the same stage values. Where these lists
//// evaluate without halting, the production gate holds iff `all_parts` holds. Where they
//// halt, the production gate halts or refuses with False/None; it cannot accept, because
//// accepting evaluates every stage and every component evaluated here.

use aiken/collection/list
use cardano/assets.{PolicyId}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/claim
use pvc2/intent.{SigCheckV2}
use pvc2/thread.{
  SettleBaseV2, SettleClaimV2, SettleEffectsV2, SettleSnapshotV2,
  SettleStatementV2, SignerV2,
}
use pvc2/types.{EnvV2, SettleRedeemerV2, ThreadDatumV2}

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

// ## Genesis

pub fn genesis_parts(
  c2_hash: ByteArray,
  policy_id: PolicyId,
  seed: OutputReference,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  let g = thread.genesis_base(c2_hash, policy_id, seed, self)
  [
    ("mint_one", thread.genesis_mint_one(g)),
    ("seed_spent", thread.genesis_seed_spent(g)),
    ("token_name", thread.genesis_token_name(g)),
    ("holder", thread.genesis_holder(g)),
    ("no_ref_script", thread.genesis_no_ref_script(g)),
    ("version", thread.genesis_version(g)),
    ("thread_id", thread.genesis_thread_id(g)),
    ("claim_script", thread.genesis_claim_script(g)),
    ("seq_zero", thread.genesis_seq_zero(g)),
    ("chain_zero", thread.genesis_chain_zero(g)),
    ("catalogue", thread.genesis_catalogue(g)),
    ("state", thread.genesis_state(g)),
    ("rows", thread.genesis_rows(g)),
    ("duties", thread.genesis_duties(g)),
    ("reserve_floor", thread.genesis_reserve_floor(g)),
    ("bounds", thread.genesis_bounds(g)),
    ("custody_value", thread.genesis_custody_value(g)),
  ]
}

// ## Settle

/// Everything S2 derives for one settle step, every stage built. `parts` are the named
/// components in gate order; `gate` is their conjunction (every check except signature
/// validity). Tests compare the digests with the frozen R1 author-oracle values and
/// inspect `parts`.
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

/// The named settle components, in P3S1-R1 gate order, over already-built stages.
pub fn settle_parts(
  b: SettleBaseV2,
  shape_digest: ByteArray,
  e: SettleEffectsV2,
  snap: SettleSnapshotV2,
  c: SettleClaimV2,
  signers: List<SignerV2>,
  st: SettleStatementV2,
  next: ThreadDatumV2,
) -> List<(ByteArray, Bool)> {
  [
    ("custody_pre", thread.settle_custody_pre(b)),
    ("claim_script", thread.settle_claim_script(b)),
    ("shape_structure", thread.settle_shape_structure(b)),
    ("shape_cells", thread.settle_shape_cells(b)),
    ("shape_digest", thread.settle_shape_digest(b, shape_digest)),
    ("recomputed", thread.settle_recomputed(b, e)),
    ("bounds", thread.settle_bounds(b, e)),
    ("claim", thread.settle_claim(b, c)),
    ("authority_shape", thread.settle_authority_shape(b, e)),
    ("intent_window", thread.settle_intent_window(b)),
    ("intent_effects", thread.settle_intent_effects(b, e, snap)),
    ("intent_native", thread.settle_intent_native(b, signers)),
    ("statement_digest", thread.settle_statement_digest(c, st)),
    ("continuation", thread.settle_continuation(b, snap, next)),
    ("payouts", thread.settle_payouts(b, e)),
    ("own_mint", thread.settle_own_mint(b)),
    ("topup", thread.settle_topup(b)),
    ("reserve_floor", thread.settle_reserve_floor(b)),
    ("global_outputs", thread.settle_global_outputs(b)),
  ]
}

/// Builds every settle stage once (as P3S1-R1 `settle_facts` did) and every component.
pub fn settle_facts(
  c2_hash: ByteArray,
  td: ThreadDatumV2,
  own_ref: OutputReference,
  r: SettleRedeemerV2,
  self: Transaction,
) -> SettleFactsV2 {
  let b = thread.settle_base(c2_hash, td, own_ref, r, self)
  let shape_digest = thread.settle_shape_digest_of(b)
  let e = thread.settle_effects(b)
  let snap = thread.settle_snapshot(b, e)
  let c = thread.settle_claim_input(b)
  let signers = thread.settle_signers(b)
  let scenario_hash = thread.settle_scenario_hash(b, e)
  let m = thread.settle_messages(b, signers, scenario_hash)
  let st = thread.settle_statement(b, e, c, shape_digest, m.intents_digest)
  let next = thread.settle_next(b, snap, st.statement_digest)
  let parts = settle_parts(b, shape_digest, e, snap, c, signers, st, next)
  SettleFactsV2 {
    statement: st.statement,
    statement_digest: st.statement_digest,
    shape_digest,
    catalogue_digest: st.catalogue_digest,
    scenario_hash,
    intents_digest: m.intents_digest,
    messages: list.map(m.signed, fn(s) { (s.signer, s.message) }),
    next,
    sigs: list.map(m.signed, fn(s) { s.sig }),
    parts,
    gate: all_parts(parts),
  }
}

// ## Revoke

pub fn revoke_parts(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  signer: ByteArray,
  signature: ByteArray,
  continuation: Int,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  let v = thread.revoke_base(td, own_ref, signer, signature, continuation, self)
  [
    ("authorized", thread.revoke_authorized(v)),
    ("custody_pre", thread.revoke_custody_pre(v)),
    ("cont_address", thread.revoke_cont_address(v)),
    ("cont_value", thread.revoke_cont_value(v)),
    ("cont_datum", thread.revoke_cont_datum(v)),
    ("cont_ref_script", thread.revoke_cont_ref_script(v)),
    ("own_mint", thread.revoke_own_mint(v)),
    ("global_outputs", thread.revoke_global_outputs(v)),
  ]
}

// ## Close

pub fn close_parts(
  td: ThreadDatumV2,
  own_ref: OutputReference,
  reserve_output: Int,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  let k = thread.close_base(td, own_ref, reserve_output, self)
  [
    ("rows_zero", thread.close_rows_zero(k)),
    ("duties_zero", thread.close_duties_zero(k)),
    ("custody_pre", thread.close_custody_pre(k)),
    ("burn", thread.close_burn(k)),
    ("reserve_owner_auth", thread.close_reserve_owner_auth(k)),
    ("reserve_address", thread.close_reserve_address(k)),
    ("reserve_value", thread.close_reserve_value(k)),
    ("reserve_ref_script", thread.close_reserve_ref_script(k)),
    ("global_outputs", thread.close_global_outputs(k)),
  ]
}

// ## C2 release

pub fn release_parts(
  own_ref: OutputReference,
  raw: Data,
  env: EnvV2,
  self: Transaction,
) -> List<(ByteArray, Bool)> {
  let v = claim.release_base(own_ref, raw, env, self)
  [
    ("own_token", claim.release_own_token(v)),
    ("single_token", claim.release_single_token(v)),
    ("burned", claim.release_burned(v)),
    ("claimant_signs", claim.release_claimant_signs(v)),
    ("spent_or_expired", claim.release_spent_or_expired(v)),
  ]
}
