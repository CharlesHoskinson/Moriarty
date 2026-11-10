// Unit and Counterexample Tests for Design Point D7: The Cached Frame Protocol
// Run: quint test --main cached_test qa/gemini-quint/chipleaf-d7/cached_test.qnt

module cached_test {
  import cached(S = 4, MUTANT = "none").* from "./cached"

  // ---------------------------------------------------------------- 1. Honest Chained-Leaf Witness Tests
  // Program 0: PUSH 10, PUSH 20, PUSH 30, LIT 99 in Leaf 1 | POP, POP, POP, HALT in Leaf 2
  run honestPush3Pop3Test = all {
    assert(acceptsCached(hw_push3_pop3)),
    assert(refinesRefStack(hw_push3_pop3)),
    assert(hw_push3_pop3.leaf1.post.depth == 3),
    assert(hw_push3_pop3.leaf1.post.top == 30),
    assert(hw_push3_pop3.leaf1.post.fp == 2),
    assert(hw_push3_pop3.leaf1.post.len == 3),
    assert(chainPopLog(hw_push3_pop3) == [30, 20, 10]),
    assert(hw_push3_pop3.leaf2.post.depth == 0),
    assert(hw_push3_pop3.leaf2.post.top == 0),
    assert(hw_push3_pop3.leaf2.post.fp == NIL),
    assert(hw_push3_pop3.leaf2.post.halted),
  }

  // Program 1: PUSH 100, PUSH 200, REWRITE 250, PUSH 300 in Leaf 1 | POP, POP, POP, HALT in Leaf 2
  run honestPush3RewritePop3Test = all {
    assert(acceptsCached(hw_push3_rewrite_pop3)),
    assert(refinesRefStack(hw_push3_rewrite_pop3)),
    assert(hw_push3_rewrite_pop3.leaf1.post.depth == 3),
    assert(hw_push3_rewrite_pop3.leaf1.post.top == 300),
    assert(hw_push3_rewrite_pop3.leaf1.post.fp == 2),
    assert(hw_push3_rewrite_pop3.leaf1.post.len == 3),
    assert(chainPopLog(hw_push3_rewrite_pop3) == [300, 250, 100]),
    assert(hw_push3_rewrite_pop3.leaf2.post.depth == 0),
    assert(hw_push3_rewrite_pop3.leaf2.post.top == 0),
    assert(hw_push3_rewrite_pop3.leaf2.post.fp == NIL),
    assert(hw_push3_rewrite_pop3.leaf2.post.halted),
  }

  // Program 2: PUSH 11, PUSH 22, PUSH 33, PUSH 44 in Leaf 1 | REWRITE 45, POP, POP, POP in Leaf 2
  run honestPush4RewriteLeaf2Pop3Test = all {
    assert(acceptsCached(hw_push4_rewrite_leaf2_pop3)),
    assert(refinesRefStack(hw_push4_rewrite_leaf2_pop3)),
    assert(hw_push4_rewrite_leaf2_pop3.leaf1.post.depth == 4),
    assert(hw_push4_rewrite_leaf2_pop3.leaf1.post.top == 44),
    assert(hw_push4_rewrite_leaf2_pop3.leaf1.post.fp == 3),
    assert(hw_push4_rewrite_leaf2_pop3.leaf1.post.len == 4),
    assert(chainPopLog(hw_push4_rewrite_leaf2_pop3) == [45, 33, 22]),
    assert(hw_push4_rewrite_leaf2_pop3.leaf2.post.depth == 1),
    assert(hw_push4_rewrite_leaf2_pop3.leaf2.post.top == 11),
    assert(hw_push4_rewrite_leaf2_pop3.leaf2.post.fp == 0),
  }

  // Program 3: PUSH 10, PUSH 20, PUSH 30, REWRITE 35 in Leaf 1 | PUSH 40, POP (MEM), POP (PAGE), POP (PAGE) in Leaf 2
  run honestIntraAndInterLeafPopTest = all {
    assert(acceptsCached(hw_intra_and_inter_leaf_pop)),
    assert(refinesRefStack(hw_intra_and_inter_leaf_pop)),
    assert(hw_intra_and_inter_leaf_pop.leaf1.post.depth == 3),
    assert(hw_intra_and_inter_leaf_pop.leaf1.post.top == 35),
    assert(hw_intra_and_inter_leaf_pop.leaf1.post.len == 3),
    assert(chainPopLog(hw_intra_and_inter_leaf_pop) == [40, 35, 20]),
    assert(hw_intra_and_inter_leaf_pop.leaf2.post.depth == 1),
    assert(hw_intra_and_inter_leaf_pop.leaf2.post.top == 10),
    assert(hw_intra_and_inter_leaf_pop.leaf2.post.fp == 0),
    assert(hw_intra_and_inter_leaf_pop.leaf2.post.len == 4),
  }

  // ---------------------------------------------------------------- 2. Required Mutant Counterexample Tests
  // Mutant 1: rewrite_appends (F12)
  // REWRITE appends (250, fp=1) at addr=2 and sets fp:=2, causing POP to return [300, 250, 250] instead of [300, 250, 100]
  run mutant1_rewriteAppendsTest = all {
    assert(acceptsWithMutant("rewrite_appends", cex_rewrite_appends)),
    assert(not(acceptsCached(cex_rewrite_appends))),
    assert(not(refinesRefStack(cex_rewrite_appends))),
    assert(chainPopLog(cex_rewrite_appends) == [300, 250, 250]),
    assert(cex_rewrite_appends.leaf2.post.top == 100),
  }

  // Mutant 2: link_from_bus (F13)
  // PUSH 30 at addr=2 stores forged bus link 2 (self-loop) instead of pre.fp=1, causing POP to return [30, 20, 20]
  run mutant2_linkFromBusTest = all {
    assert(acceptsWithMutant("link_from_bus", cex_link_from_bus)),
    assert(not(acceptsCached(cex_link_from_bus))),
    assert(not(refinesRefStack(cex_link_from_bus))),
    assert(chainPopLog(cex_link_from_bus) == [30, 20, 20]),
    assert(cex_link_from_bus.leaf2.post.top == 20),
  }

  // Mutant 3: pop_no_reload
  // POP does not update top from the loaded entry, keeping old top=30 and returning [30, 30, 30]
  run mutant3_popNoReloadTest = all {
    assert(acceptsWithMutant("pop_no_reload", cex_pop_no_reload)),
    assert(not(acceptsCached(cex_pop_no_reload))),
    assert(not(refinesRefStack(cex_pop_no_reload))),
    assert(chainPopLog(cex_pop_no_reload) == [30, 30, 30]),
    assert(cex_pop_no_reload.leaf2.post.top == 30),
  }

  // Mutant 4: top_dropped
  // Leaf statement drops top; Leaf 2 starts with prover-chosen pre.top=999, returning [999, 20, 10]
  run mutant4_topDroppedTest = all {
    assert(acceptsWithMutant("top_dropped", cex_top_dropped)),
    assert(not(acceptsCached(cex_top_dropped))),
    assert(not(refinesRefStack(cex_top_dropped))),
    assert(chainPopLog(cex_top_dropped) == [999, 20, 10]),
  }

  // ---------------------------------------------------------------- 3. Additional Cached-Protocol Mutant Tests
  // Mutant 5: sentinel_unconstrained
  // Depth-0 sentinel (top == 0, fp == NIL) is not enforced at genesis; pre.top=777 is spilled at addr=0
  // and reloaded by the 3rd POP in Leaf 2, leaving final top == 777 != 0 at depth == 0.
  run mutant5_sentinelUnconstrainedTest = all {
    assert(acceptsWithMutant("sentinel_unconstrained", cex_sentinel_unconstrained)),
    assert(not(acceptsCached(cex_sentinel_unconstrained))),
    assert(not(refinesRefStack(cex_sentinel_unconstrained))),
    assert(cex_sentinel_unconstrained.leaf2.post.depth == 0),
    assert(cex_sentinel_unconstrained.leaf2.post.top == 777),
  }

  // Mutant 6: depth_unconstrained
  // POP does not decrement depth by 1, leaving final depth == 3 != 0 after all 3 frames are popped.
  run mutant6_depthUnconstrainedTest = all {
    assert(acceptsWithMutant("depth_unconstrained", cex_depth_unconstrained)),
    assert(not(acceptsCached(cex_depth_unconstrained))),
    assert(not(refinesRefStack(cex_depth_unconstrained))),
    assert(cex_depth_unconstrained.leaf2.post.depth == 3),
  }

  // Mutant 7: pop_no_fp_reload
  // POP does not update fp := b from the loaded entry; all 3 POPs in Leaf 2 replay PAGE[2]=(20, 1),
  // returning [30, 20, 20] instead of [30, 20, 10].
  run mutant7_popNoFpReloadTest = all {
    assert(acceptsWithMutant("pop_no_fp_reload", cex_pop_no_fp_reload)),
    assert(not(acceptsCached(cex_pop_no_fp_reload))),
    assert(not(refinesRefStack(cex_pop_no_fp_reload))),
    assert(chainPopLog(cex_pop_no_fp_reload) == [30, 20, 20]),
    assert(cex_pop_no_fp_reload.leaf2.post.top == 20),
  }

  // Mutant 8: pop_page_unauthenticated
  // PAGE serves an unauthenticated cross-leaf cell (888, 1) at idx=2 in Leaf 2, returning [30, 888, 10].
  run mutant8_popPageUnauthenticatedTest = all {
    assert(acceptsWithMutant("pop_page_unauthenticated", cex_pop_page_unauthenticated)),
    assert(not(acceptsCached(cex_pop_page_unauthenticated))),
    assert(not(refinesRefStack(cex_pop_page_unauthenticated))),
    assert(chainPopLog(cex_pop_page_unauthenticated) == [30, 888, 10]),
  }

  // ---------------------------------------------------------------- 4. State Machine Invariant Test
  run stateMachineRefinementTest =
    init.then(step).expect(invFrameStackRefinement and invTamperRefused)
}
