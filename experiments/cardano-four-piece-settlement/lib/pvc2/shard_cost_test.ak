//// Sharded settlement cost benches (PROPOSED; written by hand, never compiled or run by
//// their author). Contexts: P01, P04 and the two-thread P07 transaction.
////
//// Each bench decodes its context through `test_ctx` (harness decode, NOT the compiled
//// ledger wrapper), builds the sharded transaction, and evaluates one shard predicate.
//// `*_decode` measures decode plus construction only; subtract it from the other benches
//// of the same context. For P07 the decode bench also includes the two harness digest
//// computations, because R1 has no P07 digest constants. Reported mem/cpu are host CEK
//// observations under aiken's default cost model: not ledger ExUnits, not predictions.

use aiken/collection/list
use cardano/transaction.{OutputReference, Transaction}
use pvc2/fixtures_r1 as fx
use pvc2/shard
use pvc2/shard_harness as h

fn sharded1(
  ctx: ByteArray,
  digest: ByteArray,
) -> (OutputReference, ByteArray, Transaction) {
  let (own_ref, p, tx) = h.single(ctx)
  (own_ref, p, h.sharded(tx, p, [own_ref], [digest]))
}

fn sharded2() -> (ByteArray, Transaction, List<OutputReference>, List<ByteArray>) {
  let (p, tx, refs, ds) = h.pair(fx.c2_hash, fx.ctx_p07_0, fx.ctx_p07_1)
  (p, h.sharded(tx, p, refs, ds), refs, ds)
}

// ## P01

test shc_p01_decode() {
  let (_, _, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  !list.is_empty(t.inputs)
}

test shc_p01_covered() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  shard.covered_refs(p, t) == [own_ref]
}

test shc_p01_vi_gate() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  h.vi_count(fx.c2_hash, t, p, [own_ref], [fx.intents_digest_p01]) == 2
}

test shc_p01_vs() {
  let (own_ref, p, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  h.vs_holds(fx.c2_hash, t, p, [own_ref])
}

test shc_p01_s2p() {
  let (own_ref, _, t) = sharded1(fx.ctx_p01, fx.intents_digest_p01)
  shard.delegated(h.vs_t, own_ref, t)
}

// ## P04

test shc_p04_decode() {
  let (_, _, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  !list.is_empty(t.inputs)
}

test shc_p04_covered() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  shard.covered_refs(p, t) == [own_ref]
}

test shc_p04_vi_gate() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  h.vi_count(fx.c2_hash, t, p, [own_ref], [fx.intents_digest_p04]) == 2
}

test shc_p04_vs() {
  let (own_ref, p, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  h.vs_holds(fx.c2_hash, t, p, [own_ref])
}

test shc_p04_s2p() {
  let (own_ref, _, t) = sharded1(fx.ctx_p04, fx.intents_digest_p04)
  shard.delegated(h.vs_t, own_ref, t)
}

// ## P07 (two threads, one VI and one VS call)

test shc_p07_decode() {
  let (_, t, refs, _) = sharded2()
  list.length(refs) == 2 && !list.is_empty(t.inputs)
}

test shc_p07_covered() {
  let (p, t, refs, _) = sharded2()
  shard.covered_refs(p, t) == refs
}

test shc_p07_vi_gate() {
  let (p, t, refs, ds) = sharded2()
  h.vi_count(fx.c2_hash, t, p, refs, ds) == 4
}

test shc_p07_vs() {
  let (p, t, refs, _) = sharded2()
  h.vs_holds(fx.c2_hash, t, p, refs)
}

test shc_p07_s2p_0() {
  let (_, t, refs, _) = sharded2()
  expect [r0, _] = refs
  shard.delegated(h.vs_t, r0, t)
}

test shc_p07_s2p_1() {
  let (_, t, refs, _) = sharded2()
  expect [_, r1] = refs
  shard.delegated(h.vs_t, r1, t)
}
