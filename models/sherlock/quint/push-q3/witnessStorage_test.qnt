module witnessStorage_test {
  import witnessStorage.* from "./witnessStorage"
  action begin(a: int, b: int, storage: Storage, warm: bool): bool =
    evaluation' = start(a, b, storage, warm)
  action eraseWorkForCacheHits = evaluation' = {...evaluation,
    after: {...evaluation.after, work: evaluation.after.work - evaluation.hits}}
  run uncachedLastUseTest = begin(1, 2, LastUse, false)
    .then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(published and evaluation.slots.get(4) == List(1, 2, 1, 2) and evaluation.executed == 5
      and evaluation.reclaimed == 3 and evaluation.peak == 3 and modelInv)
  run warmReadsStillExecuteTest = begin(1, 2, LastUse, true)
    .then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(published and evaluation.hits == 3 and evaluation.reads == 2
      and evaluation.executed == 2 and evaluation.scheduled == List(0, 1, 2, 3, 4) and modelInv)
  run changedArgumentsMissCacheTest = begin(3, 2, LastUse, true)
    .then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(evaluation.slots.get(4) == List(3, 2, 3, 2) and evaluation.hits == 0 and modelInv)
  run orderedArgumentsCannotCollideTest = begin(2, 1, LastUse, true)
    .then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(evaluation.slots.get(4) == List(2, 1, 2, 1) and evaluation.hits == 0 and modelInv)
  run retainAllHasSameResultTest = begin(1, 2, RetainAll, false)
    .then(tick).then(tick).then(tick).then(tick).then(tick)
    .expect(evaluation.slots.get(4) == List(1, 2, 1, 2) and evaluation.peak == 5 and evaluation.reclaimed == 0 and modelInv)
  run dependenciesReleasedAfterConsumerTest = begin(1, 2, LastUse, false)
    .then(tick).then(tick).then(tick)
    .expect(evaluation.live == Set(2) and evaluation.slots.get(2) == List(1, 2) and evaluation.peak == 3 and modelInv)
  run cacheCannotEraseSemanticChargesTest = begin(1, 2, LastUse, true)
    .then(tick).then(tick).then(tick).then(tick).then(tick)
    .then(eraseWorkForCacheHits).expect(not(modelInv))
}
