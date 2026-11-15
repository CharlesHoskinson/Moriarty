// Deterministic tests of claimsRoot.qnt: design positives and refusals, one attack per mutant.

module claimsRoot_test {
  import claimsRoot(MUT = "none").* from "./claimsRoot"

  // three claims in epoch 0, one write; two consumers use claim 2; consumer 1 cannot use it twice
  run pos_one_write_many_uses =
    init.then(proveMember({ kind: "RUN", v: 1 })).then(proveMember({ kind: "RUN", v: 2 })).then(proveMember({ kind: "RUN", v: 3 }))
      .then(closeEpoch(true)).then(useFromRoot(1, "RUN", 2, 0)).then(useFromRoot(2, "RUN", 2, 0)).then(useFromRoot(1, "RUN", 3, 0))
      .expect(Inv and ledgerWrites == 1 and WitTwoConsumers)

  run neg_use_twice =
    init.then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true)).then(useFromRoot(1, "RUN", 2, 0)).then(useFromRoot(1, "RUN", 2, 0)).fail()

  // the same claim in two epochs and in runs: used once
  run neg_use_twice_two_roots_and_runs =
    init.then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true)).then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true))
      .then(userAcceptRun(2)).then(useFromRoot(1, "RUN", 2, 0)).then(useFromRuns(1, 2)).fail()
  run neg_use_second_root =
    init.then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true)).then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true))
      .then(useFromRoot(1, "RUN", 2, 0)).then(useFromRoot(1, "RUN", 2, 1)).fail()

  // epoch wait: a claim proved in the open epoch is not usable
  run neg_use_before_close = init.then(proveMember({ kind: "RUN", v: 1 })).then(useFromRoot(1, "RUN", 1, 0)).fail()

  // data availability: a withheld list blocks use (liveness only), a later publication unblocks it
  run neg_withheld = init.then(proveMember({ kind: "RUN", v: 1 })).then(closeEpoch(false)).then(useFromRoot(1, "RUN", 1, 0)).fail()
  run pos_published_later =
    init.then(proveMember({ kind: "RUN", v: 1 })).then(closeEpoch(false)).then(publish(0)).then(useFromRoot(1, "RUN", 1, 0)).expect(Inv)

  // a forged claim is never in a root
  run neg_forged = init.then(proveMember({ kind: "RUN", v: 1 })).then(closeEpoch(true)).then(useFromRoot(1, "RUN", 9, 0)).fail()
}

module claimsRoot_mutOpen {
  import claimsRoot(MUT = "per_claim_open").* from "./claimsRoot"
  run mut_per_claim_open =
    init.then(proveMember({ kind: "RUN", v: 1 })).then(closeEpoch(true)).then(useFromRoot(1, "RUN", 1, 0))
      .expect(not(LedgerWritesBounded))
}

module claimsRoot_mutRootKey {
  import claimsRoot(MUT = "root_keyed_nullifier").* from "./claimsRoot"
  run mut_root_keyed_nullifier =
    init.then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true)).then(proveMember({ kind: "RUN", v: 2 })).then(closeEpoch(true))
      .then(useFromRoot(1, "RUN", 2, 0)).then(useFromRoot(1, "RUN", 2, 1)).expect(not(UsedOncePerConsumer))
}

module claimsRoot_mutNoRoot {
  import claimsRoot(MUT = "no_root_check").* from "./claimsRoot"
  run mut_no_root_check = init.then(useFromRoot(1, "RUN", 9, 0)).expect(not(UsedOnlyVerified) and not(UseAfterEpochClose))
}

module claimsRoot_mutUnbound {
  import claimsRoot(MUT = "unbound_root").* from "./claimsRoot"
  run mut_unbound_root =
    init.then(proveMember({ kind: "RUN", v: 1 })).then(closeEpoch(true)).then(useFromRoot(1, "RUN", 9, 0)).expect(not(UsedOnlyVerified))
}
