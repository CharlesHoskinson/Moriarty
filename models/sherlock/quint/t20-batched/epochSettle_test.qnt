// Deterministic tests of epochSettle.qnt: design positives, and one attack per mutant.

module epochSettle_test {
  import epochSettle(MUT = "none").* from "./epochSettle"

  // four jobs and a user run in epoch 0, one collection write at close
  run pos_four_jobs_one_write =
    init.then(proveJob(1)).then(proveJob(2)).then(proveJob(3)).then(proveJob(4)).then(userAcceptRun(11)).then(closeEpoch)
      .expect(Inv and WitBatch and ledgerWrites == 2 and opWrites.get(0) == 1 and roots.length() == 1)

  // an empty epoch writes nothing
  run pos_empty_epoch_no_write = init.then(closeEpoch).then(closeEpoch).expect(Inv and ledgerWrites == 0)

  // a user run is accepted once
  run neg_user_replay = init.then(userAcceptRun(11)).then(userAcceptRun(11)).fail()
}

module epochSettle_mutA {
  import epochSettle(MUT = "per_job_accept_run").* from "./epochSettle"
  run mut_per_job_accept_run =
    init.then(proveJob(1)).then(proveJob(2)).expect(not(OperatorWritesPerEpochAtMostOne) and not(NoJobInRuns))
}

module epochSettle_mutB {
  import epochSettle(MUT = "collection_drops_job").* from "./epochSettle"
  run mut_collection_drops_job =
    init.then(proveJob(1)).then(proveJob(2)).then(closeEpoch).expect(not(JobsSettledAtClose))
}

module epochSettle_mutC {
  import epochSettle(MUT = "split_epoch").* from "./epochSettle"
  run mut_split_epoch =
    init.then(proveJob(1)).then(proveJob(2)).then(closeEpoch).expect(not(OperatorWritesPerEpochAtMostOne) and not(LedgerWritesBounded))
}
