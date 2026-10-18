//// Four-piece settlement cost benches (PROPOSED; written by hand, never compiled or run by
//// their author). Contexts: P01, P04 and the two-thread P07 transaction, as in
//// `shard_cost_test`, so each piece compares with the two-shard VS bench of the same case.
////
//// Each bench decodes its context through `test_ctx` (harness decode, NOT the compiled
//// ledger wrapper), builds the four-piece transaction, and evaluates one predicate.
//// `*_decode` measures decode plus construction only; subtract it from the other benches
//// of the same context. For P07 the decode bench also includes the two harness digest
//// computations. Reported mem/cpu are host CEK observations under aiken's default cost
//// model: not ledger ExUnits, not predictions.

use aiken/collection/list
use cardano/transaction.{OutputReference, Transaction}
use pvc2/fixtures_r1 as fx
use pvc2/shard4_harness as s4
use pvc2/shard_harness as h

fn sharded1(
  ctx: ByteArray,
  digest: ByteArray,
) -> (OutputReference, ByteArray, Transaction) {
  let (own_ref, p, tx) = h.single(ctx)
  (own_ref, p, s4.sharded4(tx, p, [own_ref], [digest]))
}

fn sharded2() -> (ByteArray, Transaction, List<OutputReference>, List<ByteArray>) {
  let (p, tx, refs, ds) = h.pair(fx.c2_hash, fx.ctx_p07_0, fx.ctx_p07_1)
  (p, s4.sharded4(tx, p, refs, ds), refs, ds)
}

// ## P01

test s4c_p01_decode() {
  let (_, _, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  !list.is_empty(t.inputs)
}

test s4c_p01_vi_gate() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  h.vi_count(fx.c2_hash, t, p, [own_ref], [fx.intents_digest_p01]) == 2
}

test s4c_p01_vs1() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  s4.vs1_on(t, p, [own_ref])
}

test s4c_p01_vs2() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  s4.vs2_on(t, p, [own_ref])
}

test s4c_p01_vs3() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  s4.vs3_on(fx.c2_hash, t, p, [own_ref])
}

test s4c_p01_vs4() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  s4.vs4_on(t, p, [own_ref])
}

test s4c_p01_s2p() {
  let (own_ref, _, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  s4.s2p_on(fx.c2_hash, t, own_ref)
}

// ## P04

test s4c_p04_decode() {
  let (_, _, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  !list.is_empty(t.inputs)
}

test s4c_p04_vi_gate() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  h.vi_count(fx.c2_hash, t, p, [own_ref], [fx.intents_digest_p04]) == 2
}

test s4c_p04_vs1() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  s4.vs1_on(t, p, [own_ref])
}

test s4c_p04_vs2() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  s4.vs2_on(t, p, [own_ref])
}

test s4c_p04_vs3() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  s4.vs3_on(fx.c2_hash, t, p, [own_ref])
}

test s4c_p04_vs4() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  s4.vs4_on(t, p, [own_ref])
}

test s4c_p04_s2p() {
  let (own_ref, _, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  s4.s2p_on(fx.c2_hash, t, own_ref)
}

// ## P07 (two threads, one call of each shard)

test s4c_p07_decode() {
  let (_, t, refs, _) = sharded2()
  list.length(refs) == 2 && !list.is_empty(t.inputs)
}

test s4c_p07_vi_gate() {
  let (p, t, refs, ds) = sharded2()
  h.vi_count(fx.c2_hash, t, p, refs, ds) == 4
}

test s4c_p07_vs1() {
  let (p, t, refs, _) = sharded2()
  s4.vs1_on(t, p, refs)
}

test s4c_p07_vs2() {
  let (p, t, refs, _) = sharded2()
  s4.vs2_on(t, p, refs)
}

test s4c_p07_vs3() {
  let (p, t, refs, _) = sharded2()
  s4.vs3_on(fx.c2_hash, t, p, refs)
}

test s4c_p07_vs4() {
  let (p, t, refs, _) = sharded2()
  s4.vs4_on(t, p, refs)
}

test s4c_p07_s2p_0() {
  let (_, t, refs, _) = sharded2()
  expect [r0, _] = refs
  s4.s2p_on(fx.c2_hash, t, r0)
}

test s4c_p07_s2p_1() {
  let (_, t, refs, _) = sharded2()
  expect [_, r1] = refs
  s4.s2p_on(fx.c2_hash, t, r1)
}
