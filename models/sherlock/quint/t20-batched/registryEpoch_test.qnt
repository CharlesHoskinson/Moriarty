// Deterministic tests of registryEpoch.qnt.

module registryEpoch_test {
  import registryEpoch(MUT = "none").* from "./registryEpoch"
  run pos_three_changes_one_write =
    init.then(changeKey(2, true)).then(changeKey(3, true)).then(changeKey(1, false)).then(closeRegEpoch)
      .then(acceptCollection(1, Set(2, 3)))
      .expect(Inv and regWrites.get(0) == 1 and WitBatchedUpdate and WitNewKeyUsed)
  run neg_stale_root =
    init.then(changeKey(2, true)).then(closeRegEpoch).then(acceptCollection(0, Set(1))).fail()
  run neg_unregistered_key = init.then(acceptCollection(0, Set(2))).fail()
  run pos_empty_epoch_no_write = init.then(closeRegEpoch).expect(Inv and regWrites.keys().size() == 0)
}

module registryEpoch_mutPerKey {
  import registryEpoch(MUT = "per_key_write").* from "./registryEpoch"
  run mut_per_key_write = init.then(changeKey(2, true)).then(changeKey(3, true))
    .expect(not(RegWritesPerEpochAtMostOne) and regEpoch == 0)
}

module registryEpoch_mutNoCheck {
  import registryEpoch(MUT = "no_reg_check").* from "./registryEpoch"
  run mut_no_reg_check = init.then(changeKey(2, true)).then(closeRegEpoch).then(acceptCollection(0, Set(1)))
    .expect(not(StaleOrForeignRegRootRefused))
}

module registryEpoch_mutUnbound {
  import registryEpoch(MUT = "unbound_membership").* from "./registryEpoch"
  run mut_unbound_membership = init.then(acceptCollection(0, Set(2))).expect(not(KeysUsedAreRegistered))
}
