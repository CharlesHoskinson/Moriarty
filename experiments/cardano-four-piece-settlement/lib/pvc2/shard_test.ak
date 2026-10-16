//// Sharded settlement tests (PROPOSED; written by hand, never compiled or run by their
//// author). Expectations were frozen in EXPECTATIONS.json before this module existed.
////
//// Evidence classes:
//// - SHARD-POSITIVE: VI's gate returns one check per intent, VS holds and S2' accepts on
////   the R1 positive transaction with only the two shard withdrawals added.
//// - SHARD-SPLIT: an R1 BOOLEAN case. The pair is (VI gate is Some, VS holds). A BOOLEAN
////   case evaluates every stage and component without halting, and each shard evaluates
////   a subset of them on the same values, so a halt here falsifies that argument.
//// - SHARD-REFUSAL-ONLY: `fail` tests (halt or False both pass); no isolation claimed.
//// - SPLIT-ADVERSARIAL (sx*): mutations only the split introduces.
//// Signatures are the R1 placeholders: VI's full validator refuses until FX1.

use aiken/collection/list
use cardano/address.{Address, Script}
use cardano/transaction.{
  DatumHash, Input, Output, OutputReference, Transaction,
}
use pvc2/bytes
use pvc2/fixtures_r1 as fx
use pvc2/shard.{StateShardV1}
use pvc2/shard_harness as h
use pvc2/test_ctx
use pvc2/thread
use pvc2/types.{AccountV2, CatalogueV2, Settle}

fn p01() -> (OutputReference, ByteArray, Transaction) {
  h.single(fx.ctx_p01)
}

fn p02_ref() -> OutputReference {
  let (r, _, _) = h.single(fx.ctx_p02)
  r
}

fn p07() -> (ByteArray, Transaction, List<OutputReference>, List<ByteArray>) {
  h.pair(fx.c2_hash, fx.ctx_p07_0, fx.ctx_p07_1)
}

fn p08_cat() -> CatalogueV2 {
  h.genesis_catalogue(fx.c2_hash, fx.ctx_p08)
}

// ## SHARD-POSITIVE: single-thread settle transactions

test sh_p01_accept() {
  h.accepts1(fx.ctx_p01, fx.c2_hash, fx.intents_digest_p01, 2)
}

test sh_p02_accept() {
  h.accepts1(fx.ctx_p02, fx.c2_hash, fx.intents_digest_p02, 2)
}

test sh_p03_accept() {
  h.accepts1(fx.ctx_p03, fx.c2_hash, fx.intents_digest_p03, 2)
}

test sh_p04_accept() {
  h.accepts1(fx.ctx_p04, fx.c2_hash, fx.intents_digest_p04, 2)
}

test sh_p05_accept() {
  h.accepts1(fx.ctx_p05, fx.c2_hash, fx.intents_digest_p05, 2)
}

test sh_p06_accept() {
  h.accepts1(fx.ctx_p06, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p06), 2)
}

// ## SHARD-POSITIVE: the atomic two-thread transaction (one VI and one VS call)

test sh_p07_accept() {
  let (p, tx, refs, ds) = p07()
  let (r0, _, _) = h.single(fx.ctx_p07_0)
  let (r1, _, tx1) = h.single(fx.ctx_p07_1)
  let t = h.sharded(tx, p, refs, ds)
  and {
    tx.inputs == tx1.inputs,
    tx.outputs == tx1.outputs,
    list.length(refs) == 2,
    list.has(refs, r0),
    list.has(refs, r1),
    h.vi_count(fx.c2_hash, t, p, refs, ds) == 4,
    h.vs_holds(fx.c2_hash, t, p, refs),
    shard.delegated(h.vs_t, r0, t),
    shard.delegated(h.vs_t, r1, t),
  }
}

test sh_p14_accept() {
  h.accepts1(fx.ctx_p14, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p14), 2)
}

test sh_p15_accept() {
  h.accepts1(fx.ctx_p15, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p15), 3)
}

test sh_p16_accept() {
  h.accepts1(fx.ctx_p16, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p16), 1)
}

test sh_p17_accept() {
  h.accepts1(fx.ctx_p17, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p17), 1)
}

// ## VI full validator with placeholder signatures: REFUSE now, ACCEPT only after FX1

test sh_p01_vi_full_refuses_placeholder_signatures() fail {
  let (own_ref, p, tx) = p01()
  let refs = [own_ref]
  let ds = [fx.intents_digest_p01]
  shard.authorize(fx.c2_hash, h.auth_of(p, refs, ds), h.sharded(tx, p, refs, ds))
}

test sh_p07_vi_full_refuses_placeholder_signatures() fail {
  let (p, tx, refs, ds) = p07()
  shard.authorize(fx.c2_hash, h.auth_of(p, refs, ds), h.sharded(tx, p, refs, ds))
}

// ## SHARD-SPLIT: R1 BOOLEAN settle cases, (VI gate Some, VS holds)

test sh_h01_split() {
  h.split1(fx.c2_hash, fx.ctx_h01) == (False, True)
}

test sh_h02_split() {
  h.split1(fx.c2_hash, fx.ctx_h02) == (False, True)
}

test sh_h03_split() {
  h.split1(fx.c2_hash, fx.ctx_h03) == (True, False)
}

test sh_h04_split() {
  h.split1(fx.c2_hash, fx.ctx_h04) == (True, False)
}

test sh_h05_split() {
  h.split1(fx.c2_hash, fx.ctx_h05) == (True, False)
}

test sh_h06_split() {
  h.split1(fx.c2_hash, fx.ctx_h06) == (True, False)
}

test sh_h07_split() {
  h.split1(fx.c2_hash, fx.ctx_h07) == (False, True)
}

test sh_h09_split() {
  h.split1(fx.c2_hash, fx.ctx_h09) == (True, False)
}

test sh_h11_split() {
  h.split1(fx.c2_hash, fx.ctx_h11) == (True, False)
}

test sh_h12_split() {
  h.split1(fx.c2_hash, fx.ctx_h12) == (True, False)
}

test sh_h13_split() {
  h.split1(fx.c2_hash, fx.ctx_h13) == (True, False)
}

test sh_h20_split() {
  h.split1(fx.c2_hash, fx.ctx_h20) == (True, False)
}

test sh_h21_split() {
  h.split1(fx.c2_hash, fx.ctx_h21) == (True, False)
}

test sh_h22_split() {
  h.split1(fx.c2_hash, fx.ctx_h22) == (True, False)
}

test sh_h23_split() {
  h.split1(fx.c2_hash, fx.ctx_h23) == (True, False)
}

test sh_h24_split() {
  h.split1(fx.c2_hash, fx.ctx_h24) == (True, False)
}

test sh_h25_split() {
  h.split1(fx.c2_hash, fx.ctx_h25) == (True, False)
}

test sh_h26_split() {
  h.split1(fx.c2_hash, fx.ctx_h26) == (True, False)
}

test sh_h27_split() {
  h.split1(fx.c2_hash, fx.ctx_h27) == (True, False)
}

test sh_h28_split() {
  h.split1(fx.c2_hash, fx.ctx_h28) == (True, False)
}

test sh_h29_split() {
  h.split1(fx.c2_hash, fx.ctx_h29) == (True, False)
}

test sh_h30_split() {
  h.split1(fx.c2_hash, fx.ctx_h30) == (True, False)
}

test sh_h31_split() {
  h.split1(fx.c2_hash, fx.ctx_h31) == (True, False)
}

test sh_h32_split() {
  h.split1(fx.c2_hash, fx.ctx_h32) == (True, False)
}

test sh_h34_split() {
  h.split1(fx.c2_hash, fx.ctx_h34) == (True, False)
}

test sh_h35_split() {
  h.split1(fx.c2_hash, fx.ctx_h35) == (True, False)
}

test sh_h36_split() {
  h.split1(fx.c2_hash, fx.ctx_h36) == (True, False)
}

test sh_h37_split() {
  h.split1(fx.c2_hash, fx.ctx_h37) == (True, False)
}

test sh_h38_split() {
  h.split1(fx.c2_hash, fx.ctx_h38) == (False, True)
}

/// NC-8: own-policy mint in a settle transaction.
test sh_h39_split() {
  h.split1(fx.c2_hash, fx.ctx_h39) == (True, False)
}

test sh_h42_split() {
  h.split1(fx.c2_hash, fx.ctx_h42) == (False, True)
}

test sh_h43_split() {
  h.split1(fx.c2_hash, fx.ctx_h43) == (True, False)
}

test sh_h44_split() {
  h.split1(fx.c2_hash, fx.ctx_h44) == (True, False)
}

/// H45.0 + H45.1: one transaction; global allocation refuses once in VS.
test sh_h45_split() {
  h.split2(fx.c2_hash, fx.ctx_h45_0, fx.ctx_h45_1) == (True, False)
}

test sh_h56_split() {
  h.split1(fx.c2_hash, fx.ctx_h56) == (True, False)
}

test sh_h57_split() {
  h.split1(fx.c2_hash, fx.ctx_h57) == (True, False)
}

/// H59 / NC-4: the P02 context under shards parameterized by another C2 hash.
test sh_h59_split() {
  h.split1(fx.c2_hash_foreign, fx.ctx_p02) == (True, False)
}

test sh_h60_split() {
  h.split1(fx.c2_hash, fx.ctx_h60) == (True, False)
}

test sh_h61_split() {
  h.split1(fx.c2_hash, fx.ctx_h61) == (False, True)
}

test sh_h62_split() {
  h.split1(fx.c2_hash, fx.ctx_h62) == (False, True)
}

test sh_h65_split() {
  h.split1(fx.c2_hash, fx.ctx_h65) == (False, True)
}

test sh_h66_split() {
  h.split1(fx.c2_hash, fx.ctx_h66) == (False, True)
}

test sh_h69_split() {
  h.split1(fx.c2_hash, fx.ctx_h69) == (True, False)
}

/// H70.0 + H70.1: one transaction.
test sh_h70_split() {
  h.split2(fx.c2_hash, fx.ctx_h70_0, fx.ctx_h70_1) == (True, False)
}

// ## SHARD-REFUSAL-ONLY: R1 ERROR-PATH settle cases (per-thread shard parts)

test sh_h08_vi_refuses() fail {
  h.thread_auth_some(fx.c2_hash, fx.ctx_h08)
}

test sh_h10_vi_refuses() fail {
  h.thread_auth_some(fx.c2_hash, fx.ctx_h10)
}

test sh_h14_vs_refuses() fail {
  h.thread_state_holds(fx.c2_hash, fx.ctx_h14, bytes.zero32)
}

test sh_h33_vs_refuses() fail {
  h.thread_state_holds(fx.c2_hash, fx.ctx_h33, bytes.zero32)
}

test sh_h40_vi_refuses() fail {
  h.thread_auth_some(fx.c2_hash, fx.ctx_h40)
}

test sh_h40_vs_refuses() fail {
  h.thread_state_holds(fx.c2_hash, fx.ctx_h40, bytes.zero32)
}

test sh_h41_vi_refuses() fail {
  h.thread_auth_some(fx.c2_hash, fx.ctx_h41)
}

test sh_h41_vs_refuses() fail {
  h.thread_state_holds(fx.c2_hash, fx.ctx_h41, bytes.zero32)
}

test sh_h64_vs_refuses() fail {
  h.thread_state_holds(fx.c2_hash, fx.ctx_h64, bytes.zero32)
}

test sh_h67_vi_refuses() fail {
  h.thread_auth_some(fx.c2_hash, fx.ctx_h67)
}

test sh_h68_vs_refuses() fail {
  h.thread_state_holds(
    fx.c2_hash,
    fx.ctx_h68,
    h.honest_digest(fx.c2_hash, fx.ctx_h68),
  )
}

// ## S2' genesis: the unchanged gate plus the shard-credential guard

test sh_p08_genesis() {
  h.sharded_genesis(fx.c2_hash, fx.ctx_p08)
}

test sh_p09_genesis() {
  h.sharded_genesis(fx.c2_hash, fx.ctx_p09)
}

test sh_h46_genesis_false() {
  !h.sharded_genesis(fx.c2_hash, fx.ctx_h46)
}

test sh_h47_genesis_false() {
  !h.sharded_genesis(fx.c2_hash, fx.ctx_h47)
}

test sh_h48_genesis_false() {
  !h.sharded_genesis(fx.c2_hash, fx.ctx_h48)
}

test sh_h58_genesis_false() {
  !h.sharded_genesis(fx.c2_hash, fx.ctx_h58)
}

// ## NC-1: Settle via S2' without VS

test sx01_s2p_without_vs_withdrawal() {
  let (own_ref, p, tx) = p01()
  let ad: Data = h.auth_of(p, [own_ref], [fx.intents_digest_p01])
  !shard.delegated(h.vs_t, own_ref, h.with_shard(tx, h.vi_t, ad))
}

test sx01b_s2p_vs_redeemer_without_withdrawal() {
  let (own_ref, p, tx) = p01()
  let sd: Data = h.state_of(p, [own_ref])
  !shard.delegated(h.vs_t, own_ref, h.with_redeemer(tx, h.vs_t, sd))
}

test sx01c_s2p_vs_withdrawal_without_redeemer() {
  let (own_ref, _, tx) = p01()
  !shard.delegated(h.vs_t, own_ref, h.with_withdrawal(tx, h.vs_t))
}

test sx01d_s2p_original_tx_refuses() {
  let (own_ref, _, tx) = p01()
  !shard.delegated(h.vs_t, own_ref, tx)
}

// ## NC-2: a shard names another policy

test sx02_s2p_foreign_policy() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(h.foreign, [own_ref])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.delegated(h.vs_t, own_ref, t)
}

test sx02b_vs_foreign_policy() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(h.foreign, [own_ref])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

// ## NC-3: one of two S2' inputs covered

test sx03_vs_one_of_two() {
  let (p, tx, refs, ds) = p07()
  expect [r0, _] = refs
  let s = h.state_of(p, [r0])
  !shard.state(fx.c2_hash, h.vi_t, s, h.attach(tx, h.auth_of(p, refs, ds), s))
}

test sx03b_vi_one_of_two() {
  let (p, tx, refs, ds) = p07()
  expect [r0, _] = refs
  expect [d0, _] = ds
  let a = h.auth_of(p, [r0], [d0])
  shard.authorize_gate(fx.c2_hash, a, h.attach(tx, a, h.state_of(p, refs))) == None
}

test sx03c_s2p_uncovered_thread() {
  let (p, tx, refs, ds) = p07()
  expect [r0, r1] = refs
  let t = h.attach(tx, h.auth_of(p, refs, ds), h.state_of(p, [r0]))
  !shard.delegated(h.vs_t, r1, t)
}

// ## NC-4: a shard built for a foreign C2 or a foreign producer

test sx04_vs_foreign_c2() {
  let (own_ref, p, tx) = p01()
  let t = h.sharded(tx, p, [own_ref], [fx.intents_digest_p01])
  !shard.state(fx.c2_hash_foreign, h.vi_t, h.state_of(p, [own_ref]), t)
}

test sx04b_s2p_other_vs_hash() {
  let (own_ref, p, tx) = p01()
  let ad: Data = h.auth_of(p, [own_ref], [fx.intents_digest_p01])
  let sd: Data = h.state_of(p, [own_ref])
  let t = tx |> h.with_shard(h.vi_t, ad) |> h.with_shard(h.foreign, sd)
  !shard.delegated(h.vs_t, own_ref, t)
}

test sx04c_vs_other_vi_hash() {
  let (own_ref, p, tx) = p01()
  let t = h.sharded(tx, p, [own_ref], [fx.intents_digest_p01])
  !shard.state(fx.c2_hash, h.foreign, h.state_of(p, [own_ref]), t)
}

// ## NC-5: two programs, inconsistent per-thread data

test sx05_vi_swapped_digests() {
  let (p, tx, refs, ds) = p07()
  let a = h.auth_of(p, refs, list.reverse(ds))
  shard.authorize_gate(fx.c2_hash, a, h.attach(tx, a, h.state_of(p, refs))) == None
}

test sx05b_vs_swapped_digests() {
  let (p, tx, refs, ds) = p07()
  let s = h.state_of(p, refs)
  let t = h.attach(tx, h.auth_of(p, refs, list.reverse(ds)), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx05c_vi_one_digest_altered() {
  let (p, tx, refs, ds) = p07()
  expect [_, d1] = ds
  let a = h.auth_of(p, refs, [bytes.zero32, d1])
  shard.authorize_gate(fx.c2_hash, a, h.attach(tx, a, h.state_of(p, refs))) == None
}

// ## NC-6: replayed, duplicated, reordered or empty coverage

test sx06_vs_foreign_outref() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [p02_ref()])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx06b_vs_duplicate_entry() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [own_ref, own_ref])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx06c_vs_permuted_order() {
  let (p, tx, refs, ds) = p07()
  let s = h.state_of(p, list.reverse(refs))
  !shard.state(fx.c2_hash, h.vi_t, s, h.attach(tx, h.auth_of(p, refs, ds), s))
}

test sx06d_vi_permuted_order() {
  let (p, tx, refs, ds) = p07()
  let a = h.auth_of(p, list.reverse(refs), list.reverse(ds))
  shard.authorize_gate(fx.c2_hash, a, h.attach(tx, a, h.state_of(p, refs))) == None
}

test sx06e_vs_empty_threads() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx06f_vi_empty_threads() {
  let (own_ref, p, tx) = p01()
  let a = h.auth_of(p, [], [])
  shard.authorize_gate(
    fx.c2_hash,
    a,
    h.attach(tx, a, h.state_of(p, [own_ref])),
  ) == None
}

test sx06g_vs_extra_absent_outref() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [own_ref, p02_ref()])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx06h_s2p_foreign_outref() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [p02_ref()])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [fx.intents_digest_p01]), s)
  !shard.delegated(h.vs_t, own_ref, t)
}

// ## NC-7: Revoke and Close are not settle paths

test sx07_revoke_not_covered() {
  let (_, p, tx) = h.single(fx.ctx_p10)
  shard.covered_refs(p, tx) == []
}

test sx07b_close_not_covered() {
  let (_, p, tx) = h.single(fx.ctx_p11)
  shard.covered_refs(p, tx) == []
}

test sx07c_vs_on_revoke_tx_refuses() {
  let (own_ref, p, tx) = h.single(fx.ctx_p10)
  let s = h.state_of(p, [own_ref])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [bytes.zero32]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx07d_settle_as_revoke_refused() {
  let (td, own_ref, r, tx) = h.spending(fx.ctx_p01)
  expect Settle { continuation, .. } = r
  !thread.revoke(td, own_ref, "escrow", #"", continuation, tx)
}

// ## NC-8: genesis invoked inside a settle transaction

test sx08_genesis_refuses_in_settle_tx() fail {
  let (own_ref, p, tx) = p01()
  shard.genesis(fx.c2_hash, h.vi_t, h.vs_t, p, own_ref, tx)
}

// ## Producer/consumer binding introduced by the split

test sx11_vs_without_vi() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [own_ref])
  let sd: Data = s
  !shard.state(fx.c2_hash, h.vi_t, s, h.with_shard(tx, h.vs_t, sd))
}

test sx12_vs_vi_policy_mismatch() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [own_ref])
  let t =
    h.attach(tx, h.auth_of(h.foreign, [own_ref], [fx.intents_digest_p01]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

test sx13_vs_vi_coverage_mismatch() {
  let (p, tx, refs, ds) = p07()
  expect [r0, _] = refs
  expect [d0, _] = ds
  let s = h.state_of(p, refs)
  !shard.state(fx.c2_hash, h.vi_t, s, h.attach(tx, h.auth_of(p, [r0], [d0]), s))
}

test sx14_vi_wrong_digest() {
  let (own_ref, p, tx) = p01()
  let a = h.auth_of(p, [own_ref], [bytes.zero32])
  shard.authorize_gate(
    fx.c2_hash,
    a,
    h.attach(tx, a, h.state_of(p, [own_ref])),
  ) == None
}

test sx14b_vs_wrong_vi_digest() {
  let (own_ref, p, tx) = p01()
  let s = h.state_of(p, [own_ref])
  let t = h.attach(tx, h.auth_of(p, [own_ref], [bytes.zero32]), s)
  !shard.state(fx.c2_hash, h.vi_t, s, t)
}

// ## Shard credentials cannot be catalogue co-authority

test sx15b_guard_refuses_vi_native() {
  let cat = p08_cat()
  let bad =
    CatalogueV2 {
      ..cat,
      accounts: list.map(
        cat.accounts,
        fn(a) { AccountV2 { ..a, native: Some(Script(h.vi_t)) } },
      ),
    }
  !shard.credentials_free(bad, h.vi_t, h.vs_t)
}

test sx15c_guard_refuses_vs_reserve_owner() {
  let cat = p08_cat()
  let owner = cat.reserve_owner
  let bad =
    CatalogueV2 {
      ..cat,
      reserve_owner: Address { ..owner, payment_credential: Script(h.vs_t) },
    }
  !shard.credentials_free(bad, h.vi_t, h.vs_t)
}

test sx15d_guard_refuses_vs_native() {
  let cat = p08_cat()
  let bad =
    CatalogueV2 {
      ..cat,
      accounts: list.map(
        cat.accounts,
        fn(a) { AccountV2 { ..a, native: Some(Script(h.vs_t)) } },
      ),
    }
  !shard.credentials_free(bad, h.vi_t, h.vs_t)
}

test sx15e_guard_accepts_fixture_catalogue() {
  shard.credentials_free(p08_cat(), h.vi_t, h.vs_t)
}

// ## Duplicate shard redeemers

test sx16_s2p_duplicate_vs_redeemer() {
  let (own_ref, p, tx) = p01()
  let sd: Data = h.state_of(p, [own_ref])
  let t =
    h.with_redeemer(
      h.sharded(tx, p, [own_ref], [fx.intents_digest_p01]),
      h.vs_t,
      sd,
    )
  !shard.delegated(h.vs_t, own_ref, t)
}

test sx16b_vs_duplicate_vi_redeemer() {
  let (own_ref, p, tx) = p01()
  let ad: Data = h.auth_of(p, [own_ref], [fx.intents_digest_p01])
  let t =
    h.with_redeemer(
      h.sharded(tx, p, [own_ref], [fx.intents_digest_p01]),
      h.vi_t,
      ad,
    )
  !shard.state(fx.c2_hash, h.vi_t, h.state_of(p, [own_ref]), t)
}

// ## Coverage hygiene

test sx17_covered_refuses_datum_hash() fail {
  let (own_ref, p, tx) = p01()
  let ins =
    list.map(
      tx.inputs,
      fn(i) {
        if i.output_reference == own_ref {
          let o = i.output
          Input { ..i, output: Output { ..o, datum: DatumHash(bytes.zero32) } }
        } else {
          i
        }
      },
    )
  shard.covered_refs(p, Transaction { ..tx, inputs: ins }) == [own_ref]
}

test sx18_is_settle_classifies() {
  let (_, _, s, _) = test_ctx.spending_of(fx.ctx_p01)
  let (_, _, v, _) = test_ctx.spending_of(fx.ctx_p10)
  let (_, _, k, _) = test_ctx.spending_of(fx.ctx_p11)
  and {
    shard.is_settle(s),
    !shard.is_settle(v),
    !shard.is_settle(k),
  }
}

test sx19_vs_short_policy() {
  let (own_ref, p, tx) = p01()
  let t = h.sharded(tx, p, [own_ref], [fx.intents_digest_p01])
  !shard.state(
    fx.c2_hash,
    h.vi_t,
    StateShardV1 { thread_policy: #"00", threads: [] },
    t,
  )
}
