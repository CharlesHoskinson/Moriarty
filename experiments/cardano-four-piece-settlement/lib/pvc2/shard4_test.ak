//// Four-piece settlement tests (PROPOSED; written by hand, never compiled or run by their
//// author). Names, fail flags and predictions were frozen in EXPECTATIONS.json before this
//// module existed.
////
//// Evidence classes:
//// - SHARD4-POSITIVE: VI's gate returns one check per intent, every piece holds and S2'
////   accepts on the R1 positive transaction with only the five shard withdrawals added.
//// - FINANCIAL-OUTCOME: VS4's next datum equals the R1 author fixture `datum_post`.
//// - SHARD4-SPLIT: an R1 BOOLEAN case. The list is [VI gate is Some, VS1, VS2, VS3, VS4],
////   derived from the R1 false parts through the partition in DESIGN.md section 4.
//// - SHARD4-REFUSAL-ONLY: `fail` tests (halt or False both pass); no isolation claimed.
//// - Hostile coordination cases: mutations only the four-piece split introduces.
//// Signatures are the R1 placeholders: VI's full validator refuses until FX1.

use aiken/collection/list
use cardano/address.{Address, Script}
use cardano/transaction.{OutputReference, Transaction}
use pvc2/bytes
use pvc2/fixtures_r1 as fx
use pvc2/shard
use pvc2/shard4
use pvc2/shard4_harness as s4
use pvc2/shard_harness as h
use pvc2/types.{AccountV2, CatalogueV2}

fn p01() -> (OutputReference, ByteArray, Transaction) {
  h.single(fx.ctx_p01)
}

fn p02_ref() -> OutputReference {
  let (r, _, _) = h.single(fx.ctx_p02)
  r
}

/// P01 with VI and all four honest pieces.
fn p01_t() -> (OutputReference, ByteArray, Transaction) {
  let (own_ref, p, tx) = p01()
  (own_ref, p, s4.sharded4(tx, p, [own_ref], [fx.intents_digest_p01]))
}

fn p07() -> (ByteArray, Transaction, List<OutputReference>, List<ByteArray>) {
  h.pair(fx.c2_hash, fx.ctx_p07_0, fx.ctx_p07_1)
}

fn p08_cat() -> CatalogueV2 {
  h.genesis_catalogue(fx.c2_hash, fx.ctx_p08)
}

fn with_native(cat: CatalogueV2, hash: ByteArray) -> CatalogueV2 {
  CatalogueV2 {
    ..cat,
    accounts: list.map(
      cat.accounts,
      fn(a) { AccountV2 { ..a, native: Some(Script(hash)) } },
    ),
  }
}

// ## SHARD4-POSITIVE: single-thread settle transactions

test q_p01_accept() {
  s4.accepts1(fx.ctx_p01, fx.c2_hash, fx.intents_digest_p01, 2)
}

test q_p02_accept() {
  s4.accepts1(fx.ctx_p02, fx.c2_hash, fx.intents_digest_p02, 2)
}

test q_p03_accept() {
  s4.accepts1(fx.ctx_p03, fx.c2_hash, fx.intents_digest_p03, 2)
}

test q_p04_accept() {
  s4.accepts1(fx.ctx_p04, fx.c2_hash, fx.intents_digest_p04, 2)
}

test q_p05_accept() {
  s4.accepts1(fx.ctx_p05, fx.c2_hash, fx.intents_digest_p05, 2)
}

test q_p06_accept() {
  s4.accepts1(fx.ctx_p06, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p06), 2)
}

// ## SHARD4-POSITIVE: the atomic two-thread transaction (one call of each shard)

test q_p07_accept() {
  let (p, tx, refs, ds) = p07()
  let (r0, _, _) = h.single(fx.ctx_p07_0)
  let (r1, _, _) = h.single(fx.ctx_p07_1)
  let t = s4.sharded4(tx, p, refs, ds)
  and {
    list.length(refs) == 2,
    list.has(refs, r0),
    list.has(refs, r1),
    h.vi_count(fx.c2_hash, t, p, refs, ds) == 4,
    s4.verdicts(fx.c2_hash, t, p, refs, ds) == [True, True, True, True, True],
    s4.s2p_on(fx.c2_hash, t, r0),
    s4.s2p_on(fx.c2_hash, t, r1),
  }
}

test q_p14_accept() {
  s4.accepts1(fx.ctx_p14, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p14), 2)
}

test q_p15_accept() {
  s4.accepts1(fx.ctx_p15, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p15), 3)
}

test q_p16_accept() {
  s4.accepts1(fx.ctx_p16, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p16), 1)
}

test q_p17_accept() {
  s4.accepts1(fx.ctx_p17, fx.c2_hash, h.honest_digest(fx.c2_hash, fx.ctx_p17), 1)
}

/// SR-11: the two P07 contexts carry one and the same transaction.
test q_p07_shared_transaction() {
  let (_, _, tx0) = h.single(fx.ctx_p07_0)
  let (_, _, tx1) = h.single(fx.ctx_p07_1)
  tx0 == tx1
}

// ## FINANCIAL-OUTCOME: VS4's next datum is the R1 author's post datum

test fin_p01_vs4_next_is_r1_datum_post() {
  and {
    s4.claim_digest(fx.ctx_p01) == fx.stmt_digest_p01,
    s4.vs4_next_bytes(fx.ctx_p01) == fx.datum_post_p01,
  }
}

test fin_p02_vs4_next_is_r1_datum_post() {
  and {
    s4.claim_digest(fx.ctx_p02) == fx.stmt_digest_p02,
    s4.vs4_next_bytes(fx.ctx_p02) == fx.datum_post_p02,
  }
}

test fin_p03_vs4_next_is_r1_datum_post() {
  and {
    s4.claim_digest(fx.ctx_p03) == fx.stmt_digest_p03,
    s4.vs4_next_bytes(fx.ctx_p03) == fx.datum_post_p03,
  }
}

test fin_p04_vs4_next_is_r1_datum_post() {
  and {
    s4.claim_digest(fx.ctx_p04) == fx.stmt_digest_p04,
    s4.vs4_next_bytes(fx.ctx_p04) == fx.datum_post_p04,
  }
}

test fin_p05_vs4_next_is_r1_datum_post() {
  and {
    s4.claim_digest(fx.ctx_p05) == fx.stmt_digest_p05,
    s4.vs4_next_bytes(fx.ctx_p05) == fx.datum_post_p05,
  }
}

// ## SHARD4-SPLIT: R1 BOOLEAN settle cases, [VI, VS1, VS2, VS3, VS4]

test q_h01_split() {
  s4.split1(fx.c2_hash, fx.ctx_h01) == [False, True, True, True, True]
}

test q_h02_split() {
  s4.split1(fx.c2_hash, fx.ctx_h02) == [False, True, True, True, True]
}

test q_h03_split() {
  s4.split1(fx.c2_hash, fx.ctx_h03) == [True, True, False, True, True]
}

test q_h04_split() {
  s4.split1(fx.c2_hash, fx.ctx_h04) == [True, True, False, True, True]
}

test q_h05_split() {
  s4.split1(fx.c2_hash, fx.ctx_h05) == [True, True, False, True, True]
}

test q_h06_split() {
  s4.split1(fx.c2_hash, fx.ctx_h06) == [True, True, False, True, True]
}

test q_h07_split() {
  s4.split1(fx.c2_hash, fx.ctx_h07) == [False, True, True, True, True]
}

test q_h09_split() {
  s4.split1(fx.c2_hash, fx.ctx_h09) == [True, True, False, True, True]
}

test q_h11_split() {
  s4.split1(fx.c2_hash, fx.ctx_h11) == [True, True, False, True, True]
}

/// VS4 is not predicted here: its next datum uses the claim digest, which differs from the
/// recomputed statement digest in this case.
test q_h12_split() {
  list.take(s4.split1(fx.c2_hash, fx.ctx_h12), 4) == [True, True, False, False]
}

test q_h13_split() {
  s4.split1(fx.c2_hash, fx.ctx_h13) == [True, False, True, True, True]
}

test q_h20_split() {
  s4.split1(fx.c2_hash, fx.ctx_h20) == [True, True, True, True, False]
}

test q_h21_split() {
  s4.split1(fx.c2_hash, fx.ctx_h21) == [True, True, True, True, False]
}

test q_h22_split() {
  s4.split1(fx.c2_hash, fx.ctx_h22) == [True, True, True, True, False]
}

test q_h23_split() {
  s4.split1(fx.c2_hash, fx.ctx_h23) == [True, True, True, True, False]
}

test q_h24_split() {
  s4.split1(fx.c2_hash, fx.ctx_h24) == [True, True, True, True, False]
}

test q_h25_split() {
  s4.split1(fx.c2_hash, fx.ctx_h25) == [True, True, True, True, False]
}

test q_h26_split() {
  s4.split1(fx.c2_hash, fx.ctx_h26) == [True, True, True, True, False]
}

test q_h27_split() {
  s4.split1(fx.c2_hash, fx.ctx_h27) == [True, True, True, True, False]
}

test q_h28_split() {
  s4.split1(fx.c2_hash, fx.ctx_h28) == [True, True, True, True, False]
}

test q_h29_split() {
  s4.split1(fx.c2_hash, fx.ctx_h29) == [True, True, True, True, False]
}

test q_h30_split() {
  s4.split1(fx.c2_hash, fx.ctx_h30) == [True, True, True, True, False]
}

test q_h31_split() {
  s4.split1(fx.c2_hash, fx.ctx_h31) == [True, True, True, True, False]
}

test q_h32_split() {
  s4.split1(fx.c2_hash, fx.ctx_h32) == [True, False, True, True, False]
}

test q_h34_split() {
  s4.split1(fx.c2_hash, fx.ctx_h34) == [True, True, False, False, False]
}

test q_h35_split() {
  s4.split1(fx.c2_hash, fx.ctx_h35) == [True, True, True, False, True]
}

test q_h36_split() {
  s4.split1(fx.c2_hash, fx.ctx_h36) == [True, True, True, False, True]
}

test q_h37_split() {
  s4.split1(fx.c2_hash, fx.ctx_h37) == [True, True, False, False, False]
}

test q_h38_split() {
  s4.split1(fx.c2_hash, fx.ctx_h38) == [False, True, True, True, True]
}

/// NC-8: own-policy mint in a settle transaction (VS1).
test q_h39_split() {
  s4.split1(fx.c2_hash, fx.ctx_h39) == [True, False, True, True, True]
}

test q_h42_split() {
  s4.split1(fx.c2_hash, fx.ctx_h42) == [False, True, True, True, True]
}

test q_h43_split() {
  s4.split1(fx.c2_hash, fx.ctx_h43) == [True, True, False, True, True]
}

test q_h44_split() {
  s4.split1(fx.c2_hash, fx.ctx_h44) == [True, True, True, True, False]
}

/// H45.0 + H45.1: one output satisfies two threads; VS1 refuses once per transaction.
test q_h45_split() {
  s4.split2(fx.c2_hash, fx.ctx_h45_0, fx.ctx_h45_1) == [
    True, False, True, True, True,
  ]
}

test q_h56_split() {
  s4.split1(fx.c2_hash, fx.ctx_h56) == [True, False, True, True, True]
}

test q_h57_split() {
  s4.split1(fx.c2_hash, fx.ctx_h57) == [True, False, True, True, True]
}

/// H59 / NC-4: the P02 context with VI and VS3 parameterized by another C2 hash.
test q_h59_split() {
  s4.split1(fx.c2_hash_foreign, fx.ctx_p02) == [True, True, True, False, True]
}

test q_h60_split() {
  s4.split1(fx.c2_hash, fx.ctx_h60) == [True, True, True, False, True]
}

test q_h61_split() {
  s4.split1(fx.c2_hash, fx.ctx_h61) == [False, True, True, True, True]
}

test q_h62_split() {
  s4.split1(fx.c2_hash, fx.ctx_h62) == [False, True, True, True, True]
}

test q_h65_split() {
  s4.split1(fx.c2_hash, fx.ctx_h65) == [False, True, True, True, True]
}

test q_h66_split() {
  s4.split1(fx.c2_hash, fx.ctx_h66) == [False, True, True, True, True]
}

test q_h69_split() {
  s4.split1(fx.c2_hash, fx.ctx_h69) == [True, True, False, True, True]
}

/// H70.0 + H70.1: one transaction.
test q_h70_split() {
  s4.split2(fx.c2_hash, fx.ctx_h70_0, fx.ctx_h70_1) == [
    True, False, True, True, False,
  ]
}

// ## SHARD4-REFUSAL-ONLY: R1 ERROR-PATH settle cases (VS1, VS2 and VS4)

test q_h14_pieces_refuse() fail {
  s4.nonvi_pieces(fx.ctx_h14)
}

test q_h33_pieces_refuse() fail {
  s4.nonvi_pieces(fx.ctx_h33)
}

test q_h40_pieces_refuse() fail {
  s4.nonvi_pieces(fx.ctx_h40)
}

test q_h41_pieces_refuse() fail {
  s4.nonvi_pieces(fx.ctx_h41)
}

test q_h64_pieces_refuse() fail {
  s4.nonvi_pieces(fx.ctx_h64)
}

test q_h68_pieces_refuse() fail {
  s4.nonvi_pieces(fx.ctx_h68)
}

// ## SHARD4-GENESIS: the unchanged gate plus the identity and credential guards

test q_p08_genesis() {
  s4.genesis4(fx.c2_hash, fx.ctx_p08, s4.pieces_t())
}

test q_p09_genesis() {
  s4.genesis4(fx.c2_hash, fx.ctx_p09, s4.pieces_t())
}

test q_h46_genesis_false() {
  !s4.genesis4(fx.c2_hash, fx.ctx_h46, s4.pieces_t())
}

test q_h47_genesis_false() {
  !s4.genesis4(fx.c2_hash, fx.ctx_h47, s4.pieces_t())
}

test q_h48_genesis_false() {
  !s4.genesis4(fx.c2_hash, fx.ctx_h48, s4.pieces_t())
}

test q_h58_genesis_false() {
  !s4.genesis4(fx.c2_hash, fx.ctx_h58, s4.pieces_t())
}

// ## Missing pieces: every piece is non-bypassable at settlement

test q_s2p_missing_vs1() {
  let (own_ref, p, tx) = p01()
  let t = s4.without_piece(tx, p, [own_ref], [fx.intents_digest_p01], 1)
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_s2p_missing_vs2() {
  let (own_ref, p, tx) = p01()
  let t = s4.without_piece(tx, p, [own_ref], [fx.intents_digest_p01], 2)
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_s2p_missing_vs3() {
  let (own_ref, p, tx) = p01()
  let t = s4.without_piece(tx, p, [own_ref], [fx.intents_digest_p01], 3)
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_s2p_missing_vs4() {
  let (own_ref, p, tx) = p01()
  let t = s4.without_piece(tx, p, [own_ref], [fx.intents_digest_p01], 4)
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_s2p_vs2_redeemer_without_withdrawal() {
  let (own_ref, p, tx) = p01()
  let d: Data = s4.piece(2, p, [own_ref])
  let t =
    h.with_redeemer(
      s4.without_piece(tx, p, [own_ref], [fx.intents_digest_p01], 2),
      s4.vs2_t,
      d,
    )
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_s2p_vs4_withdrawal_without_redeemer() {
  let (own_ref, p, tx) = p01()
  let t =
    h.with_withdrawal(
      s4.without_piece(tx, p, [own_ref], [fx.intents_digest_p01], 4),
      s4.vs4_t,
    )
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_s2p_original_tx_refuses() {
  let (own_ref, _, tx) = p01()
  !s4.s2p_on(fx.c2_hash, tx, own_ref)
}

test q_vs3_without_vi() {
  let (own_ref, p, tx) = p01()
  let t = s4.attach(tx, None, s4.honest_pieces(p, [own_ref]))
  !s4.vs3_on(fx.c2_hash, t, p, [own_ref])
}

// ## Duplicated pieces and producers

test q_s2p_duplicate_vs3_redeemer() {
  let (own_ref, p, t) = p01_t()
  let d: Data = s4.piece(3, p, [own_ref])
  !s4.s2p_on(fx.c2_hash, h.with_redeemer(t, s4.vs3_t, d), own_ref)
}

test q_vs3_duplicate_vi_redeemer() {
  let (own_ref, p, t) = p01_t()
  let ad = s4.vi_data(p, [own_ref], [fx.intents_digest_p01])
  !s4.vs3_on(fx.c2_hash, h.with_redeemer(t, h.vi_t, ad), p, [own_ref])
}

// ## Swapped and mis-tagged pieces

test q_s2p_swapped_piece_tags() {
  let (own_ref, p, tx) = p01()
  let refs = [own_ref]
  let t =
    s4.attach(
      tx,
      Some(s4.vi_data(p, refs, [fx.intents_digest_p01])),
      [
        (s4.vs1_t, s4.piece(2, p, refs)),
        (s4.vs2_t, s4.piece(1, p, refs)),
        (s4.vs3_t, s4.piece(3, p, refs)),
        (s4.vs4_t, s4.piece(4, p, refs)),
      ],
    )
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_vs1_refuses_tag_2() {
  let (own_ref, p, t) = p01_t()
  !shard4.vs1_structure(s4.piece(2, p, [own_ref]), t)
}

test q_vs2_refuses_tag_3() {
  let (own_ref, p, t) = p01_t()
  !shard4.vs2_accounting(s4.piece(3, p, [own_ref]), t)
}

test q_vs3_refuses_tag_4() {
  let (own_ref, p, t) = p01_t()
  !shard4.vs3_identity(fx.c2_hash, h.vi_t, s4.piece(4, p, [own_ref]), t)
}

test q_vs4_refuses_tag_1() {
  let (own_ref, p, t) = p01_t()
  !shard4.vs4_custody(s4.piece(1, p, [own_ref]), t)
}

test q_s2p_piece_redeemer_wrong_type() fail {
  let (own_ref, p, tx) = p01()
  let refs = [own_ref]
  let ad = s4.vi_data(p, refs, [fx.intents_digest_p01])
  let t =
    s4.attach(
      h.with_shard(h.with_shard(tx, h.vi_t, ad), s4.vs1_t, ad),
      None,
      [
        (s4.vs2_t, s4.piece(2, p, refs)),
        (s4.vs3_t, s4.piece(3, p, refs)),
        (s4.vs4_t, s4.piece(4, p, refs)),
      ],
    )
  s4.s2p_on(fx.c2_hash, t, own_ref)
}

// ## Aliased identities

test q_alias_vs1_vs2() {
  let (_, p, _) = p01()
  !shard4.identities_distinct(
    fx.c2_hash,
    h.vi_t,
    [s4.vs1_t, s4.vs1_t, s4.vs3_t, s4.vs4_t],
    p,
  )
}

test q_alias_vs3_vi() {
  let (_, p, _) = p01()
  !shard4.identities_distinct(
    fx.c2_hash,
    h.vi_t,
    [s4.vs1_t, s4.vs2_t, h.vi_t, s4.vs4_t],
    p,
  )
}

test q_alias_vs4_c2() {
  let (_, p, _) = p01()
  !shard4.identities_distinct(
    fx.c2_hash,
    h.vi_t,
    [s4.vs1_t, s4.vs2_t, s4.vs3_t, fx.c2_hash],
    p,
  )
}

test q_alias_vs2_policy() {
  let (_, p, _) = p01()
  !shard4.identities_distinct(
    fx.c2_hash,
    h.vi_t,
    [s4.vs1_t, p, s4.vs3_t, s4.vs4_t],
    p,
  )
}

test q_alias_three_pieces() {
  let (_, p, _) = p01()
  !shard4.identities_distinct(
    fx.c2_hash,
    h.vi_t,
    [s4.vs1_t, s4.vs2_t, s4.vs3_t],
    p,
  )
}

test q_distinct_fixture_accepts() {
  let (_, p, _) = p01()
  shard4.identities_distinct(fx.c2_hash, h.vi_t, s4.pieces_t(), p)
}

test q_alias_s2p_refuses() {
  let (own_ref, _, t) = p01_t()
  !shard4.delegated(
    fx.c2_hash,
    h.vi_t,
    [s4.vs1_t, s4.vs1_t, s4.vs3_t, s4.vs4_t],
    own_ref,
    t,
  )
}

test q_alias_genesis_refuses() {
  !s4.genesis4(
    fx.c2_hash,
    fx.ctx_p08,
    [s4.vs1_t, s4.vs1_t, s4.vs3_t, s4.vs4_t],
  )
}

// ## Cross-group pieces

test q_s2p_foreign_policy_vs4() {
  let (own_ref, p, tx) = p01()
  let refs = [own_ref]
  let t =
    s4.attach(
      tx,
      Some(s4.vi_data(p, refs, [fx.intents_digest_p01])),
      [
        (s4.vs1_t, s4.piece(1, p, refs)),
        (s4.vs2_t, s4.piece(2, p, refs)),
        (s4.vs3_t, s4.piece(3, p, refs)),
        (s4.vs4_t, s4.piece(4, h.foreign, refs)),
      ],
    )
  !s4.s2p_on(fx.c2_hash, t, own_ref)
}

test q_vs1_foreign_policy() {
  let (own_ref, _, t) = p01_t()
  !shard4.vs1_structure(s4.piece(1, h.foreign, [own_ref]), t)
}

test q_vs2_foreign_policy() {
  let (own_ref, _, t) = p01_t()
  !shard4.vs2_accounting(s4.piece(2, h.foreign, [own_ref]), t)
}

test q_vs3_vi_policy_mismatch() {
  let (own_ref, p, tx) = p01()
  let refs = [own_ref]
  let t =
    s4.attach(
      tx,
      Some(s4.vi_data(h.foreign, refs, [fx.intents_digest_p01])),
      s4.honest_pieces(p, refs),
    )
  !s4.vs3_on(fx.c2_hash, t, p, refs)
}

// ## Coverage: partial, vacuous, permuted, duplicated, replayed

test q_vs2_one_of_two() {
  let (p, tx, refs, ds) = p07()
  expect [r0, _] = refs
  !shard4.vs2_accounting(s4.piece(2, p, [r0]), s4.sharded4(tx, p, refs, ds))
}

test q_vs4_one_of_two() {
  let (p, tx, refs, ds) = p07()
  expect [r0, _] = refs
  !shard4.vs4_custody(s4.piece(4, p, [r0]), s4.sharded4(tx, p, refs, ds))
}

test q_s2p_uncovered_thread() {
  let (p, tx, refs, ds) = p07()
  expect [r0, r1] = refs
  let t =
    s4.attach(
      tx,
      Some(s4.vi_data(p, refs, ds)),
      [
        (s4.vs1_t, s4.piece(1, p, [r0])),
        (s4.vs2_t, s4.piece(2, p, refs)),
        (s4.vs3_t, s4.piece(3, p, refs)),
        (s4.vs4_t, s4.piece(4, p, refs)),
      ],
    )
  !s4.s2p_on(fx.c2_hash, t, r1)
}

test q_vs3_vi_coverage_mismatch() {
  let (p, tx, refs, ds) = p07()
  expect [r0, _] = refs
  expect [d0, _] = ds
  let t =
    s4.attach(tx, Some(s4.vi_data(p, [r0], [d0])), s4.honest_pieces(p, refs))
  !s4.vs3_on(fx.c2_hash, t, p, refs)
}

test q_vs1_empty_threads() {
  let (_, p, t) = p01_t()
  !shard4.vs1_structure(s4.piece(1, p, []), t)
}

test q_vs4_empty_threads() {
  let (_, p, t) = p01_t()
  !shard4.vs4_custody(s4.piece(4, p, []), t)
}

test q_vs3_permuted_order() {
  let (p, tx, refs, ds) = p07()
  !shard4.vs3_identity(
    fx.c2_hash,
    h.vi_t,
    s4.piece(3, p, list.reverse(refs)),
    s4.sharded4(tx, p, refs, ds),
  )
}

test q_vs2_duplicate_entry() {
  let (own_ref, p, t) = p01_t()
  !shard4.vs2_accounting(s4.piece(2, p, [own_ref, own_ref]), t)
}

test q_vs4_foreign_outref() {
  let (_, p, t) = p01_t()
  !shard4.vs4_custody(s4.piece(4, p, [p02_ref()]), t)
}

test q_vs1_extra_absent_outref() {
  let (own_ref, p, t) = p01_t()
  !shard4.vs1_structure(s4.piece(1, p, [own_ref, p02_ref()]), t)
}

// ## Intermediate values: VI's digest and the claim envelope digest

test q_vs3_swapped_vi_digests() {
  let (p, tx, refs, ds) = p07()
  let t = s4.sharded4(tx, p, refs, list.reverse(ds))
  !s4.vs3_on(fx.c2_hash, t, p, refs)
}

test q_vs3_forged_vi_digest() {
  let (own_ref, p, tx) = p01()
  let t = s4.sharded4(tx, p, [own_ref], [bytes.zero32])
  !s4.vs3_on(fx.c2_hash, t, p, [own_ref])
}

test q_forged_claim_digest_vs3() {
  let (own_ref, p, tx) = p01()
  let forged = s4.forge_claim_digest(tx, fx.c2_hash)
  let t = s4.sharded4(forged, p, [own_ref], [fx.intents_digest_p01])
  !s4.vs3_on(fx.c2_hash, t, p, [own_ref])
}

test q_forged_claim_digest_vs4() {
  let (own_ref, p, tx) = p01()
  let forged = s4.forge_claim_digest(tx, fx.c2_hash)
  let t = s4.sharded4(forged, p, [own_ref], [fx.intents_digest_p01])
  !s4.vs4_on(t, p, [own_ref])
}

// ## Reference inputs are not consumption

test q_thread_reference_input_not_covered() {
  let (own_ref, p, tx) = p01()
  let moved = s4.to_reference(tx, fn(i) { i.output_reference == own_ref })
  let t = s4.sharded4(moved, p, [own_ref], [fx.intents_digest_p01])
  and {
    shard.covered_refs(p, t) == [],
    !s4.vs1_on(t, p, [own_ref]),
    !s4.vs4_on(t, p, [own_ref]),
  }
}

test q_claim_reference_input_vs3_refuses() fail {
  let (own_ref, p, tx) = p01()
  let moved =
    s4.to_reference(
      tx,
      fn(i) { i.output.address.payment_credential == Script(fx.c2_hash) },
    )
  let t = s4.sharded4(moved, p, [own_ref], [fx.intents_digest_p01])
  s4.vs3_on(fx.c2_hash, t, p, [own_ref])
}

test q_claim_reference_input_vs4_refuses() fail {
  let (own_ref, p, tx) = p01()
  let moved =
    s4.to_reference(
      tx,
      fn(i) { i.output.address.payment_credential == Script(fx.c2_hash) },
    )
  let t = s4.sharded4(moved, p, [own_ref], [fx.intents_digest_p01])
  s4.vs4_on(t, p, [own_ref])
}

// ## Revoke is not a settle path

test q_pieces_on_revoke_tx_refuse() {
  let (own_ref, p, tx) = h.single(fx.ctx_p10)
  let refs = [own_ref]
  let t = s4.sharded4(tx, p, refs, [bytes.zero32])
  and {
    !s4.vs1_on(t, p, refs),
    !s4.vs2_on(t, p, refs),
    !s4.vs3_on(fx.c2_hash, t, p, refs),
    !s4.vs4_on(t, p, refs),
  }
}

// ## Shard credentials cannot be catalogue co-authority

test q_guard_refuses_vi_native() {
  !shard4.credentials_free(with_native(p08_cat(), h.vi_t), s4.shards_t())
}

test q_guard_refuses_vs1_native() {
  !shard4.credentials_free(with_native(p08_cat(), s4.vs1_t), s4.shards_t())
}

test q_guard_refuses_vs2_native() {
  !shard4.credentials_free(with_native(p08_cat(), s4.vs2_t), s4.shards_t())
}

test q_guard_refuses_vs3_reserve_owner() {
  let cat = p08_cat()
  let owner = cat.reserve_owner
  let bad =
    CatalogueV2 {
      ..cat,
      reserve_owner: Address { ..owner, payment_credential: Script(s4.vs3_t) },
    }
  !shard4.credentials_free(bad, s4.shards_t())
}

test q_guard_refuses_vs4_native() {
  !shard4.credentials_free(with_native(p08_cat(), s4.vs4_t), s4.shards_t())
}

test q_guard_accepts_fixture_catalogue() {
  shard4.credentials_free(p08_cat(), s4.shards_t())
}
