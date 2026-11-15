// Deterministic tests of chunkCap.qnt.

module chunkCap_test {
  import chunkCap(MUT = "none").* from "./chunkCap"
  run pos_measured_one_call = init.then(submit(474)).then(submit(699)).then(submit(1015))
    .expect(Inv and calls == 3 and anchorWrites == 0)
  run pos_fallback_two_chunks = init.then(submit(1016)).then(submit(1980)).expect(Inv and calls == 4 and anchorWrites == 4)
  run pos_refused_over_cap = init.then(submit(1981)).then(submit(5000)).expect(Inv and calls == 0 and WitRefused)
}

module chunkCap_mutNoCap {
  import chunkCap(MUT = "no_cap").* from "./chunkCap"
  run mut_no_cap = init.then(submit(5000)).expect(not(ChunksAtMostCap) and calls == 6)
}

module chunkCap_mutSmall {
  import chunkCap(MUT = "small_ltx").* from "./chunkCap"
  run mut_small_ltx = init.then(submit(474)).expect(not(MeasuredEntriesOneCall))
}

module chunkCap_mutTrunc {
  import chunkCap(MUT = "cap_truncate").* from "./chunkCap"
  run mut_cap_truncate = init.then(submit(2970)).expect(not(NoSilentTruncation))
}

module chunkCap_mutFour {
  import chunkCap(MUT = "cap_four").* from "./chunkCap"
  run mut_cap_four = init.then(submit(2970)).expect(not(TtlCoversProving))
}
