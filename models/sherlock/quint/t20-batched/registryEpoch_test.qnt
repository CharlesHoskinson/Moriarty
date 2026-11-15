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
  // regressions of Carbon lane qa-20261114-t20-batched F-9.1 and F-9.2 (failed before the fix)
  run pos_revoke_then_add_registered = init.then(changeKey(1, false)).then(changeKey(1, true)).then(closeRegEpoch)
    .expect(roots.get(current).contains(1) and regWrites.keys().size() == 0)
  run pos_add_then_revoke_not_registered = init.then(changeKey(2, true)).then(changeKey(2, false)).then(closeRegEpoch)
    .expect(not(roots.get(current).contains(2)) and regWrites.keys().size() == 0)
  run pos_noop_add_no_write = init.then(changeKey(1, true)).then(closeRegEpoch).expect(Inv and regWrites.keys().size() == 0)
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
