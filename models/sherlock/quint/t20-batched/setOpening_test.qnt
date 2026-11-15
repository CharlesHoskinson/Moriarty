// Deterministic tests of setOpening.qnt: positives on the design, and one attack per mutant that
// breaks an invariant only in the mutant instance (the same trace is refused in the design).

module setOpening_test {
  import setOpening(MUT = "none").* from "./setOpening"

  // a chain of depth 4: J(J(J(J(L1, L2), L3), F), L2'), opened at the deepest leaf in one call
  // nodes: 0 L1, 1 L2, 2 J(0,1), 3 L3, 4 J(2,3), 5 F201, 6 J(4,5)
  run pos_deep_open_one_call =
    init.then(mkLeaf(1)).then(mkLeaf(2)).then(mkJoin(0, 1)).then(mkLeaf(3)).then(mkJoin(2, 3))
      .then(mkForeign(201)).then(mkJoin(4, 5)).then(acceptCollection(6))
      .then(openClaim(6, 0, 3)).then(openClaim(6, 5, 1))
      .expect(Inv and WitDeepOpen and WitForeignOpen and openCalls.get(1) == 1)

  // diamond: claim 2 under J(L2, DEP(2? no) ...): L2 twice through J(0, 0); the second open is refused
  run pos_diamond_once =
    init.then(mkLeaf(2)).then(mkJoin(0, 0)).then(acceptCollection(1)).then(openClaim(1, 0, 1))
      .then(openClaim(1, 0, 1)).fail()

  // a claim settled by acceptRun cannot be opened again (one runs map), and the reverse
  run pos_cross_entry_replay =
    init.then(mkLeaf(1)).then(acceptCollection(0)).then(acceptRun(1)).then(openClaim(0, 0, 0)).fail()

  // refusals in the design
  run neg_adm_head = init.then(mkPremise("ADM", 101)).then(acceptCollection(0)).then(openClaim(0, 0, 0)).fail()
  run neg_join_head = init.then(mkLeaf(1)).then(mkJoin(0, 0)).then(acceptCollection(1)).then(openClaim(1, 1, 0)).fail()
  run neg_unverified = init.then(mkLeaf(1)).then(openClaim(0, 0, 0)).fail()
}

module setOpening_mutants {
  import setOpening(MUT = "descend_k_open").* from "./setOpening"

  // M5: the stale descend spec spends 1 + ceil(3 / 2) = 3 calls on a depth 3 claim
  run mut_descend_k_open =
    init.then(mkLeaf(1)).then(mkLeaf(2)).then(mkJoin(0, 1)).then(mkLeaf(3)).then(mkJoin(2, 3))
      .then(mkForeign(201)).then(mkJoin(4, 5)).then(acceptCollection(6)).then(openClaim(6, 0, 3))
      .expect(not(OneCallAnyDepth))
}

module setOpening_mutants2 {
  import setOpening(MUT = "head_any_form").* from "./setOpening"
  run mut_head_any_form =
    init.then(mkPremise("ADM", 101)).then(acceptCollection(0)).then(openClaim(0, 0, 0))
      .expect(not(OnlyClaimHeadsSettle))
}

module setOpening_mutants3 {
  import setOpening(MUT = "no_replay_check").* from "./setOpening"
  run mut_no_replay_check =
    init.then(mkLeaf(2)).then(mkJoin(0, 0)).then(acceptCollection(1)).then(openClaim(1, 0, 1))
      .then(openClaim(1, 0, 1)).expect(not(ClaimSettledOnce))
}

module setOpening_mutants4 {
  import setOpening(MUT = "unverified_root").* from "./setOpening"
  run mut_unverified_root =
    init.then(mkLeaf(1)).then(openClaim(0, 0, 0)).expect(not(OpenedOnlyFromVerified))
}
