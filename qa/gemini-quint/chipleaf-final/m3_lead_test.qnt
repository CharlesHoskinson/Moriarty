// Lead additions to the chipleaf-final m3 lane (not the lane's work).
// M9 `first_dropped` in the full leaf: what refuses the lane's witness, and is any witness accepted?
module m3_lead_test {
  import m3(C = 4, D = 3, MUTANT = "none").* from "./m3"

  // The lane's M9 witness: Leaf 2 of partition 1 with every MEM row marked padding.
  // Which part of acceptance refuses it under the mutant?
  run m9WitnessRefusedByTest = all {
    assert(not(acceptsWithMutant("first_dropped", cex_M9_first_dropped))),
    assert(segMemGatesOk("first_dropped", cex_M9_first_dropped.leaf2.memRows)),
    assert(not(coreAndBusOk("first_dropped", cex_M9_first_dropped.leaf2))
      or not(segMemCopiesOk("first_dropped", cex_M9_first_dropped.leaf2))
      or not(walkGatesAndCopiesOk("first_dropped", cex_M9_first_dropped.leaf2))
      or not(digestsOk("first_dropped", cex_M9_first_dropped.leaf2))),
  }

  // Which components refuse it, one by one (for the verdict).
  run m9WitnessComponentsTest = all {
    assert(segMemCopiesOk("first_dropped", cex_M9_first_dropped.leaf2)),
    assert(walkGatesAndCopiesOk("first_dropped", cex_M9_first_dropped.leaf2)),
    assert(digestsOk("first_dropped", cex_M9_first_dropped.leaf2)),
    assert(not(coreAndBusOk("first_dropped", cex_M9_first_dropped.leaf2))),
  }

  // M4 after the lead repair of its witness: accepted under the mutant, refused by the sound circuit,
  // and it does not refine (the walk writes slot 3 while base / C = 1).
  run m4RepairedWitnessTest = all {
    assert(acceptsWithMutant("walk_last_dropped", cex_M4_walk_last_dropped)),
    assert(not(acceptsWithMutant("none", cex_M4_walk_last_dropped))),
    assert(not(refinesWith("walk_last_dropped", cex_M4_walk_last_dropped))),
  }
}
