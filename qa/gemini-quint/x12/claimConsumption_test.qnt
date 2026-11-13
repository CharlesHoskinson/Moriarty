// X12 tests: positives (completeness of the GP-2 hook, kit names pos_gp2_*), negatives in the
// sound instance, and one deterministic attack per mutant (kit names claim_*): the attack
// succeeds under the mutant and breaks an invariant; the same trace is refused in the sound
// instance (its last action fails).
// Run indices: 0 A, 1 B, 2 R (Rejected 8), 3 C (input 8), 4 E (input 3), 5 D (deps [A, A]),
// 6 F (deps [A, R], input 2 = D's value), 7 G (input 7). Segment 8: non-terminal of A;
// segment 9: terminal tail of A with h_lo = 2.

module claimConsumption_test {
  import ccTypes.* from "./claimConsumption"
  import claimConsumption(MUT = NOMUT).* from "./claimConsumption"

  // ---------------- positives ----------------
  run pos_gp2_empty_deps_plain_seal =
    init.then(sealRun(0)).expect(st.sets.contains(leafOf(0)) and soundAll)

  run pos_gp2_b_consumes_a_halted =
    init.then(sealRun(0))
      .then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, selfSub(leafOf(0))))
      .then(sealDeps(1, st.lastDl, 7, 4))
      .expect(depWithLink(st, 1, 0) and soundAll and membersOf(depOf(1)).contains(cl(0)))

  // A inside JOIN(JOIN(LR, LA), LR) (right of left) consumed by B, and inside
  // JOIN(LR, JOIN(LA, LR)) (left of right) consumed by G: two DESCEND steps each.
  run pos_gp2_claim_from_join_left_and_right_bracketing =
    init.then(sealRun(0)).then(sealRun(2))
      .then(join(leafOf(2), leafOf(0)))
      .then(join(joinOf(leafOf(2), leafOf(0)), leafOf(2)))
      .then(descend(selfSub(joinOf(joinOf(leafOf(2), leafOf(0)), leafOf(2))), "JOIN", 0))
      .then(descend((joinOf(joinOf(leafOf(2), leafOf(0)), leafOf(2)), joinOf(leafOf(2), leafOf(0))), "JOIN", 1))
      .then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, (joinOf(joinOf(leafOf(2), leafOf(0)), leafOf(2)), leafOf(0))))
      .then(sealDeps(1, st.lastDl, 7, 4))
      .expect(depWithLink(st, 1, 0) and soundAll)
      .then(join(leafOf(0), leafOf(2)))
      .then(join(leafOf(2), joinOf(leafOf(0), leafOf(2))))
      .then(descend(selfSub(joinOf(leafOf(2), joinOf(leafOf(0), leafOf(2)))), "JOIN", 1))
      .then(descend((joinOf(leafOf(2), joinOf(leafOf(0), leafOf(2))), joinOf(leafOf(0), leafOf(2))), "JOIN", 0))
      .then(dlPremise(admSub(PC)))
      .then(dlLink(st.lastDl, (joinOf(leafOf(2), joinOf(leafOf(0), leafOf(2))), leafOf(0))))
      .then(sealDeps(7, st.lastDl, 7, 5))
      .expect(depWithLink(st, 7, 0) and soundAll)

  // B's DEP holds LA in its cover; D (deps [A, A]) discharges A once from that cover (DESCEND
  // through a DEP) and once from A's own LEAF: repetition takes two DL-STEPs.
  run pos_gp2_claim_from_dep_cover =
    pos_gp2_b_consumes_a_halted
      .then(descend(selfSub(depOf(1)), "DEP", 0))
      .then(dlPremise(admSub(PC)))
      .then(dlChain(st.lastDl, (hashOf(st, { tag: "DEP", args: [cl(1), 4, leafOf(0), cl(0)] }), leafOf(0))))
      .then(dlChain(st.lastDl, selfSub(leafOf(0))))
      .then(sealDeps(5, st.lastDl, 1, 5))
      .expect(st.seals.exists(lg => lg.claim == cl(5)) and soundAll)

  run pos_gp2_repeated_consumption =
    init.then(sealRun(0))
      .then(dlPremise(admSub(PC)))
      .then(dlChain(st.lastDl, selfSub(leafOf(0))))
      .then(dlChain(st.lastDl, selfSub(leafOf(0))))
      .then(sealDeps(5, st.lastDl, 1, 5))
      .expect(st.seals.exists(lg => lg.claim == cl(5)) and soundAll)

  // Reach witnesses: each negated reach invariant is violated by an explicit trace of the sound model.
  // Uniform `quint run` sampling does not find the chain seal: it needs 5 coordinated steps.
  run pos_gp2_witness_chain_seal =
    pos_gp2_repeated_consumption.expect(not(reachChainSeal) and soundAll)
  run pos_gp2_witness_b_consumes_a =
    pos_gp2_b_consumes_a_halted.expect(not(reachBConsumesA) and soundAll)

  // F: input link D (D Halted 2 = F's input), chain [A, R]: a Rejected claim as metadata.
  run pos_gp2_rejected_as_metadata =
    pos_gp2_repeated_consumption
      .then(sealRun(2))
      .then(dlPremise(admSub(PC)))
      .then(dlLink(st.lastDl, selfSub(hashOf(st, { tag: "DEP", args: [cl(5), 5, joinOf(leafOf(0), leafOf(0)), NO_INPUT_LINK] }))))
      .then(dlChain(st.lastDl, selfSub(leafOf(0))))
      .then(dlChain(st.lastDl, selfSub(leafOf(2))))
      .then(sealDeps(6, st.lastDl, 2, 5))
      .expect(depWithLink(st, 6, 5) and soundAll)

  run pos_gp2_one_producer_two_consumers =
    pos_gp2_b_consumes_a_halted
      .then(dlPremise(admSub(PC)))
      .then(dlLink(st.lastDl, selfSub(leafOf(0))))
      .then(sealDeps(7, st.lastDl, 7, 5))
      .expect(depWithLink(st, 1, 0) and depWithLink(st, 7, 0) and soundAll)

  // Collection (GP-002 acceptance b): JOIN(JOIN(LA, DEP_B), LR); each claim opens from the root.
  run pos_gp2_three_runs_collection =
    pos_gp2_b_consumes_a_halted
      .then(sealRun(2))
      .then(join(leafOf(0), depOf(1)))
      .then(join(joinOf(leafOf(0), depOf(1)), leafOf(2)))
      .expect(Set(cl(0), cl(1), cl(2)).subseteq(membersOf(joinOf(joinOf(leafOf(0), depOf(1)), leafOf(2)))) and soundAll)

  // ---------------- negatives in the sound instance ----------------
  // C (input 8) cannot take R's Rejected value 8.
  run neg_rejected_producer_refused =
    init.then(sealRun(2)).then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, selfSub(leafOf(2))))
      .expect(soundAll).then(sealDeps(3, st.lastDl, 8, 4)).fail()
  // E (input 3) cannot take A's value 7, with either witness.
  run neg_value_mismatch_refused_wv7 =
    init.then(sealRun(0)).then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, selfSub(leafOf(0))))
      .expect(soundAll).then(sealDeps(4, st.lastDl, 7, 4)).fail()
  run neg_value_mismatch_refused_wv3 =
    init.then(sealRun(0)).then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, selfSub(leafOf(0))))
      .expect(soundAll).then(sealDeps(4, st.lastDl, 3, 4)).fail()
  // A FOREIGN head passes the input-link DL-STEP (the spec allows it) but the seal's claim
  // re-hash refuses it: the foreign claim is not a (prog, input, outcome) triple.
  run neg_foreign_as_input_link_refused_at_seal =
    init.then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, selfSub(hashOf(st, { tag: "FOREIGN", args: [st.fclaim] }))))
      .expect(soundAll)
      .expect(soundAll).then(sealDeps(1, st.lastDl, 7, 4)).fail()
  // A premise-only DL does not seal (EmptyDepsOnlyBySeal).
  run neg_premise_only_dl_refused =
    init.then(dlPremise(admSub(PB))).expect(soundAll).then(sealDeps(1, st.lastDl, 7, 4)).fail()
  // A plain SEAL refuses D (non-empty deps_d); SEAL-WITH-DEPS refuses a wrong chain.
  run neg_plain_seal_nonempty_refused = init.expect(soundAll).then(sealRun(5)).fail()
  run neg_chain_short_refused =
    init.then(sealRun(0)).then(dlPremise(admSub(PC)))
      .then(dlChain(st.lastDl, selfSub(leafOf(0))))
      .expect(soundAll).then(sealDeps(5, st.lastDl, 1, 5)).fail()
  // An input link cannot follow a chain element; a second link is refused.
  run neg_link_after_chain_refused =
    init.then(sealRun(0)).then(dlPremise(admSub(PC)))
      .then(dlChain(st.lastDl, selfSub(leafOf(0))))
      .expect(soundAll).then(dlLink(st.lastDl, selfSub(leafOf(0)))).fail()
  run neg_second_link_refused =
    init.then(sealRun(0)).then(dlPremise(admSub(PB)))
      .then(dlLink(st.lastDl, selfSub(leafOf(0))))
      .expect(soundAll).then(dlLink(st.lastDl, selfSub(leafOf(0)))).fail()
  // Unsealed segments never seal.
  run neg_nonterminal_segment_refused = init.expect(soundAll).then(seal(8, admSub(PA), 1)).fail()
  run neg_tail_segment_refused = init.expect(soundAll).then(seal(9, admSub(PA), 3)).fail()
  // A seal needs its own program's admission.
  run neg_wrong_admission_refused = init.expect(soundAll).then(seal(0, admSub(PB), 3)).fail()
}

// ---------------- mutant attacks ----------------
module t_noTerminal { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, noTerminal: true }).* from "./claimConsumption"
  run claim_consume_unsealed_segment = init.then(seal(9, admSub(PA), 3)).expect(not(SegmentNeverConsumed))
  run honest_still_ok = init.then(sealRun(0)).expect(soundAll)
}
module t_ignoreTag { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, ignoreTag: true }).* from "./claimConsumption"
  run claim_descend_wrong_domain = init.then(sealRun(0)).then(descend(selfSub(leafOf(0)), "JOIN", 0)).expect(not(SubRootSound))
  run honest_still_ok = init.then(sealRun(0)).then(dlPremise(admSub(PB))).then(dlLink(st.lastDl, selfSub(leafOf(0))))
    .then(sealDeps(1, st.lastDl, 7, 4)).expect(soundAll)
}
module t_headOfJoin { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, headOfJoin: true }).* from "./claimConsumption"
  run claim_head_of_join = init.then(sealRun(0)).then(sealRun(2)).then(join(leafOf(0), leafOf(2)))
    .then(dlPremise(admSub(PB))).then(dlChain(st.lastDl, selfSub(joinOf(leafOf(0), leafOf(2)))))
    .expect(not(DepsDischargedIffVerifiedSet))
}
module t_subUnrelated { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, subUnrelated: true }).* from "./claimConsumption"
  // forge a producer that "Halted with 3" for E, under a verified root it is not part of
  run claim_sub_root_unrelated = init.then(sealRun(0))
    .then(hashAny("O", HALTED, 0, 3, 0))
    .then(hashAny("CLAIM", PA, hashOf(st, { tag: "IN", args: [5] }), hashOf(st, { tag: "O", args: [HALTED, 0, 3] }), 0))
    .then(hashAny("LEAF", hashOf(st, { tag: "CLAIM", args: [PA, hashOf(st, { tag: "IN", args: [5] }), hashOf(st, { tag: "O", args: [HALTED, 0, 3] })] }), 3, 0, 0))
    .then(dlPremise(admSub(PB)))
    .then(dlLink(st.lastDl, (leafOf(0), hashOf(st, { tag: "LEAF", args: [hashOf(st, { tag: "CLAIM", args: [PA, hashOf(st, { tag: "IN", args: [5] }), hashOf(st, { tag: "O", args: [HALTED, 0, 3] })] }), 3] }))))
    .then(sealDeps(4, st.lastDl, 3, 4))
    .expect(not(InputLinkHaltedOnly) and not(DepsDischargedIffVerifiedSet))
}
module t_subUnrelated_sound { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = NOMUT).* from "./claimConsumption"
  run claim_sub_root_unrelated_refused = init.then(sealRun(0))
    .then(hashAny("O", HALTED, 0, 3, 0))
    .then(hashAny("CLAIM", PA, hashOf(st, { tag: "IN", args: [5] }), hashOf(st, { tag: "O", args: [HALTED, 0, 3] }), 0))
    .then(hashAny("LEAF", hashOf(st, { tag: "CLAIM", args: [PA, hashOf(st, { tag: "IN", args: [5] }), hashOf(st, { tag: "O", args: [HALTED, 0, 3] })] }), 3, 0, 0))
    .then(dlPremise(admSub(PB)))
    .expect(soundAll).then(dlLink(st.lastDl, (leafOf(0), hashOf(st, { tag: "LEAF", args: [hashOf(st, { tag: "CLAIM", args: [PA, hashOf(st, { tag: "IN", args: [5] }), hashOf(st, { tag: "O", args: [HALTED, 0, 3] })] }), 3] }))))
    .fail()
  run claim_head_of_join_refused = init.then(sealRun(0)).then(sealRun(2)).then(join(leafOf(0), leafOf(2)))
    .then(dlPremise(admSub(PB))).expect(soundAll).then(dlChain(st.lastDl, selfSub(joinOf(leafOf(0), leafOf(2))))).fail()
  run claim_descend_wrong_domain_refused = init.then(sealRun(0)).expect(soundAll).then(descend(selfSub(leafOf(0)), "JOIN", 0)).fail()
  run claim_premise_in_chain_refused = init.then(dlPremise(admSub(PB))).expect(soundAll).then(dlChain(st.lastDl, admSub(PA))).fail()
  run claim_premise_as_link_refused = init.then(dlPremise(admSub(PB))).expect(soundAll).then(dlLink(st.lastDl, admSub(PA))).fail()
  run claim_nsteps_forged_refused = init.expect(soundAll).then(seal(0, admSub(PA), 9)).fail()
  run claim_chain_reordered_refused = init.then(sealRun(0)).then(sealRun(2)).then(dlPremise(admSub(PC)))
    .then(dlChain(st.lastDl, selfSub(leafOf(2)))).then(dlChain(st.lastDl, selfSub(leafOf(0))))
    .expect(soundAll).then(sealDeps(6, st.lastDl, 1, 5)).fail()
}
module t_linkAnyStatus { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, linkAnyStatus: true }).* from "./claimConsumption"
  run claim_input_link_rejected_producer = init.then(sealRun(2)).then(dlPremise(admSub(PB)))
    .then(dlLink(st.lastDl, selfSub(leafOf(2)))).then(sealDeps(3, st.lastDl, 8, 4)).expect(not(InputLinkHaltedOnly))
}
module t_noValueTie { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, noValueTie: true }).* from "./claimConsumption"
  run claim_skip_value_opening = init.then(sealRun(0)).then(dlPremise(admSub(PB)))
    .then(dlLink(st.lastDl, selfSub(leafOf(0)))).then(sealDeps(4, st.lastDl, 7, 4)).expect(not(InputLinkHaltedOnly))
}
module t_valueMismatch { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, valueMismatch: true }).* from "./claimConsumption"
  run claim_input_link_value_mismatch = init.then(sealRun(0)).then(dlPremise(admSub(PB)))
    .then(dlLink(st.lastDl, selfSub(leafOf(0)))).then(sealDeps(4, st.lastDl, 3, 4)).expect(not(InputLinkHaltedOnly))
}
module t_nstepsForged { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, nstepsForged: true }).* from "./claimConsumption"
  run claim_nsteps_forged_in_set_leaf = init.then(seal(0, admSub(PA), 9)).expect(not(SegmentNeverConsumed))
}
module t_plainSealNonEmpty { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, plainSealNonEmpty: true }).* from "./claimConsumption"
  run claim_plain_seal_with_nonempty_chain = init.then(sealRun(5)).expect(not(EmptyDepsOnlyBySeal))
}
module t_noChainCheck { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, noChainCheck: true }).* from "./claimConsumption"
  run claim_drop_dependency = init.then(sealRun(0)).then(dlPremise(admSub(PC)))
    .then(dlChain(st.lastDl, selfSub(leafOf(0)))).then(sealDeps(5, st.lastDl, 1, 5))
    .expect(not(ChainOrderAndRepetitionExact))
}
module t_premiseInChain { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, premiseInChain: true }).* from "./claimConsumption"
  run claim_premise_in_chain = init.then(dlPremise(admSub(PB))).then(dlChain(st.lastDl, admSub(PA)))
    .expect(not(DepsDischargedIffVerifiedSet))
}
module t_accumSorted { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, accum: "sorted" }).* from "./claimConsumption"
  run claim_chain_reordered = init.then(sealRun(0)).then(sealRun(2)).then(dlPremise(admSub(PC)))
    .then(dlChain(st.lastDl, selfSub(leafOf(2)))).then(dlChain(st.lastDl, selfSub(leafOf(0))))
    .then(sealDeps(6, st.lastDl, 1, 5)).expect(not(ChainOrderAndRepetitionExact))
}
module t_accumSet { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, accum: "set" }).* from "./claimConsumption"
  run claim_chain_dedup_one_step_for_two = init.then(sealRun(0)).then(dlPremise(admSub(PC)))
    .then(dlChain(st.lastDl, selfSub(leafOf(0)))).then(sealDeps(5, st.lastDl, 1, 5))
    .expect(not(ChainOrderAndRepetitionExact))
}
module t_coverSkipsLater { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, coverSkipsLater: true }).* from "./claimConsumption"
  run claim_wrong_set_root = init.then(sealRun(0)).then(sealRun(2)).then(join(leafOf(2), leafOf(0)))
    .then(descend(selfSub(joinOf(leafOf(2), leafOf(0))), "JOIN", 1))
    .then(dlPremise(admSub(PC)))
    .then(dlChain(st.lastDl, selfSub(leafOf(0))))
    .then(dlChain(st.lastDl, (joinOf(leafOf(2), leafOf(0)), leafOf(0))))
    .expect(not(DepsDischargedIffVerifiedSet))
}
module t_linkNoCover { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, linkNoCover: true }).* from "./claimConsumption"
  run probe_link_not_in_cover = init.then(sealRun(0)).then(dlPremise(admSub(PB)))
    .then(dlLink(st.lastDl, selfSub(leafOf(0)))).expect(not(DepsDischargedIffVerifiedSet))
}
module t_sealDepsEmpty { import ccTypes.* from "./claimConsumption" import claimConsumption(MUT = { ...NOMUT, sealDepsEmpty: true }).* from "./claimConsumption"
  run probe_premise_only_seal = init.then(dlPremise(admSub(PB))).then(sealDeps(1, st.lastDl, 7, 4))
    .expect(not(EmptyDepsOnlyBySeal))
}
