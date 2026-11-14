// Deterministic tests of continuation.qnt: the kit positives, the named mutants as attacks on the
// sound instance, and one module per mutant in which the attack completes and an invariant breaks.

module continuation_test {
  import contTypes.* from "./continuation"
  import continuation(MUT = NOMUT).* from "./continuation"

  // GP-001 shape: 6 leaves completed out of order, tier 1 carry stack, final merge, one SEAL
  run pos_tier1_uninterrupted_out_of_order =
    init.then(proveGenuineAt(3)).then(proveGenuineAt(0)).then(proveGenuineAt(5)).then(proveGenuineAt(1)).then(proveGenuineAt(4)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(push).then(push).then(push)
      .then(mergeOf(top(0), top(1))).then(sealOf(merged(top(0), top(1)))).then(consumeOf(TRUE_CLAIM))
      .expect(soundAll and sets.size() == 1 and accepted.contains(TRUE_CLAIM) and maxDepth <= LOG2L + 2)

  // stop at a power-of-two leaf boundary (h = 4) and resume: the same claim_d and n_steps
  run pos_mode1_resume_pow2 =
    init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(proveGenuineAt(3)).then(push).then(push).then(push).then(push).then(pause).then(resumeFrom(recAt(4)))
      .then(proveGenuineAt(4)).then(proveGenuineAt(5)).then(push).then(push)
      .then(mergeOf(top(0), top(1))).then(sealOf(merged(top(0), top(1)))).then(consumeOf(TRUE_CLAIM))
      .expect(soundAll and sets.size() == 1 and pauses == 1)

  // stop at an odd leaf (h = 3), and two stops (h = 1 and h = 5)
  run pos_mode1_resume_odd_and_twice =
    init.then(proveGenuineAt(0)).then(push).then(pause).then(resumeFrom(recAt(1)))
      .then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(pause).then(resumeFrom(recAt(3)))
      .then(proveGenuineAt(3)).then(proveGenuineAt(4)).then(push).then(push).then(pause).then(resumeFrom(recAt(5)))
      .then(proveGenuineAt(5)).then(push)
      .then(mergeOf(top(0), top(1))).then(sealOf(merged(top(0), top(1))))
      .expect(soundAll and sets.size() == 1 and pauses == 3 and sets.forall(r => r.claim == TRUE_CLAIM and r.n == TRUE_N))

  // the record carries the CKPT tag, never RUN (N-4 (a))
  run pos_record_tagged = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).expect(recs.forall(r => r.dig.tag == "CKPT") and soundAll)

  // the named mutants as attacks on the sound instance: each is refused by a circuit relation
  run cont_refresh_fuel_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(proveForgedAt(recAt(3), "fuel", 0, 3)).expect(soundAll).then(mergeOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 0), 3)).fail())
  run cont_change_input_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(proveForgedAt(recAt(3), "input", 0, 3)).expect(soundAll).then(mergeOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 0), 3)).fail())
  run cont_skip_predecessor_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(proveForgedAt(recAt(3), "none", 0, 4)).expect(soundAll).then(mergeOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec(genuine(3), 0), 4)).fail())
  run cont_reset_height_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).expect(soundAll).then(proveForgedAt(recAt(3), "none", 0, 0).fail())
  run cont_publish_as_run_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).expect(soundAll).then(sealOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }).fail())
  run cont_ckpt_as_gp2_claim_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).expect(soundAll).then(consumeOf(recAt(3).dig).fail())
  run cont_exhausted_as_checkpoint_refused = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(proveGenuineAt(3)).then(proveGenuineAt(4)).then(proveGenuineAt(5)).then(push).then(push).then(push).then(push).then(push).then(push).expect(soundAll).then(pause.fail())
}

module t_noStitch {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, noStitch: true }).* from "./continuation"
  run cont_refresh_fuel_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(proveForgedAt(recAt(3), "fuel", 0, 3)).then(proveForgedAt(recAt(3), "fuel", 1, 4)).then(proveForgedAt(recAt(3), "fuel", 2, 5)).then(proveForgedAt(recAt(3), "fuel", 3, 6)).then(mergeOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 0), 3))).then(mergeOf(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 1), 4))).then(mergeOf(merged(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 1), 4)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 2), 5))).then(mergeOf(merged(merged(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 1), 4)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 2), 5)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 3), 6))).then(sealOf(merged(merged(merged(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 1), 4)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 2), 5)), leafSeg(exec({ pc: 3, inp: INP, fuel: F0 }, 3), 6)))).expect(not(ResumeProducesIdenticalClaimD))
  run cont_change_input_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(proveForgedAt(recAt(3), "input", 0, 3)).then(proveForgedAt(recAt(3), "input", 1, 4)).then(proveForgedAt(recAt(3), "input", 2, 5)).then(mergeOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 0), 3))).then(mergeOf(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 1), 4))).then(mergeOf(merged(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 1), 4)), leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 2), 5))).then(sealOf(merged(merged(merged({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 0), 3)), leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 1), 4)), leafSeg(exec({ pc: 3, inp: INP_OTHER, fuel: 3 }, 2), 5)))).expect(not(ResumeProducesIdenticalClaimD))
}

module t_noInterval {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, noInterval: true }).* from "./continuation"
  run cont_skip_predecessor_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(proveForgedAt(recAt(3), "none", 0, 4)).then(mergeOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 }, leafSeg(exec(genuine(3), 0), 4))).expect(not(OutOfOrderWorkerCompletionPreservesSegInterval))
}

module t_noGenesis {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, noGenesis: true }).* from "./continuation"
  run cont_reset_height_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(proveForgedAt(recAt(3), "none", 0, 0)).expect(not(OutOfOrderWorkerCompletionPreservesSegInterval))
}

module t_noTerminal {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, noTerminal: true }).* from "./continuation"
  run cont_publish_as_run_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(mergeOf({ lo: 0, hi: 2, pre: GEN, post: genuine(2), rank: 1 }, { lo: 2, hi: 3, pre: genuine(2), post: genuine(3), rank: 0 })).then(sealOf({ lo: 0, hi: 3, pre: GEN, post: genuine(3), rank: 2 })).expect(not(ResumeProducesIdenticalClaimD))
}

module t_recordTagRun {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, recordTagRun: true }).* from "./continuation"
  run record_untagged_breaks_host_side_only = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).expect(not(RecordNeverParsesAsClaim) and circuitSide).then(consumeOf(recAt(3).dig).fail())
}

module t_consumeAnyRun {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, consumeAnyRun: true }).* from "./continuation"
  run tag_on_record_stops_tag_only_consumer = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).expect(soundAll).then(consumeOf(recAt(3).dig).fail())
}

module t_anyRunUntagged {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, consumeAnyRun: true, recordTagRun: true }).* from "./continuation"
  run cont_ckpt_as_gp2_claim_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(push).then(push).then(push).then(pause).then(consumeOf(recAt(3).dig)).expect(not(CheckpointRejectedBySealAndGP2))
}

module t_pauseTerminal {
  import contTypes.* from "./continuation"
  import continuation(MUT = { ...NOMUT, pauseTerminal: true }).* from "./continuation"
  run cont_exhausted_as_checkpoint_breaks = init.then(proveGenuineAt(0)).then(proveGenuineAt(1)).then(proveGenuineAt(2)).then(proveGenuineAt(3)).then(proveGenuineAt(4)).then(proveGenuineAt(5)).then(push).then(push).then(push).then(push).then(push).then(push).then(pause).expect(not(NoTerminalCheckpoint))
}
