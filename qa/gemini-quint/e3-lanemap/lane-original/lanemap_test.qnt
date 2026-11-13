// Deterministic unit tests, mutant-catching tests, edge-case counterexamples, and
// randomized adversarial lane-map tests for `lanemap.qnt`.
// All `run` definitions end with `Test` so `quint test` executes every one by default.

module lanemap_test {
  import lanemap.* from "./lanemap"

  // 1. Question 1: Acceptance rule and output index `o + vals(op, param) - 1` for DIGEST and SPONGE(3).
  run acceptanceAndOutputIndexTest = {
    val log = makeCaseLog(0, 0, 0)
    val offs = buildOffs(log)
    val vs = flattenValues(log)
    // Entry 2 is DIGEST(SHK_OUTCOME): off = 4, vals = 2 -> output index = 4 + 2 - 1 = 5 (value 23)
    val eo2 = offs.nth(2)
    val idx2 = eo2.off + entryValCount(eo2.op, eo2.param) - 1
    // Entry 3 is SPONGE(3): off = 6, vals = 4 -> output index = 6 + 4 - 1 = 9 (value 33)
    val eo3 = offs.nth(3)
    val idx3 = eo3.off + entryValCount(eo3.op, eo3.param) - 1
    // Disallowed ops in lane map: PERM (6), H2 (7), SPONGE(2) (8), PUB (9), NOP (10)
    val mapPerm = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 6 })
    val mapH2 = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 7 })
    val mapSponge2 = HONEST_LANE_MAP.replaceAt(1, { lane: 3, ent: 8 })
    val mapPub = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 9 })
    val mapNop = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 10 })
    init.expect(and {
      idx2 == 5 and vs.nth(idx2) == 23 and vs.nth(idx2) == lastEntryValue(log.nth(2)),
      idx3 == 9 and vs.nth(idx3) == 33 and vs.nth(idx3) == lastEntryValue(log.nth(3)),
      state.decoded.ok,
      state.decoded.stmt == List(90, 91, 21, 33, 33, 23, 77, 0),
      not(decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapPerm).ok),
      not(decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapH2).ok),
      not(decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapSponge2).ok),
      not(decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapPub).ok),
      not(decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapNop).ok),
    })
  }

  // 2. Question 2: Uniqueness (`s[lane] != 0` duplicate test, zero-output overwrite edge case,
  //    mapping lanes 0 and 1, and `(0, 0)` unused sentinel safety).
  run uniquenessAndUnusedZeroPairTest = {
    val log = makeCaseLog(0, 0, 0)
    // Duplicate lane 2 when first mapped entry (4) has non-zero output 21 -> refused by `s[2] != 0`
    val dupNonZeroMap = List(
      { lane: 2, ent: 4 },
      { lane: 2, ent: 5 },
      { lane: 3, ent: 3 },
      { lane: 5, ent: 2 }
    )
    val dupNonZeroRes = decode(0, 0, 0, log, DEFAULT_CALL_LANES, dupNonZeroMap)

    // Edge case (Question 2a): in ZERO_OUTPUT_LOG, entry 4 has output 0 (`values = List(11, 0)`).
    // Pair 0 sets `s[2] = 0`; Pair 1 tests `s[2] != 0` (which is false!) and overwrites `s[2] = 25`.
    // `decode` accepts this duplicate-lane map and violates `soundForResult`, whereas `decodeFixedDup` refuses it.
    val dupZeroRes = decode(0, 0, 0, ZERO_OUTPUT_LOG, DEFAULT_CALL_LANES, dupNonZeroMap)
    val dupZeroFixedRes = decodeFixedDup(0, 0, 0, ZERO_OUTPUT_LOG, DEFAULT_CALL_LANES, dupNonZeroMap)

    // Question 2b: Can a pair set lane 0 or lane 1 (binding and comm lanes)?
    // When `callLanes[0] == 0` and `callLanes[1] == 0`, `(0, 3)` and `(1, 0)` are not `(0, 0)`
    // and `decode` has no `lane >= 2` check, so `decode` accepts both!
    val zeroPrefixLanes = DEFAULT_CALL_LANES.replaceAt(0, 0).replaceAt(1, 0)
    val mapLane0And1 = List(
      { lane: 0, ent: 3 },
      { lane: 1, ent: 0 },
      { lane: 3, ent: 3 },
      { lane: 5, ent: 2 }
    )
    val lane0And1Res = decode(0, 0, 0, log, zeroPrefixLanes, mapLane0And1)

    // Question 2c: `(0, 0)` is ignored as unused even when `callLanes[0] == 90 != 0`,
    // while `(2, 0)` maps effect lane 2 to entry 0 (`DIGEST(SHK_PROGC)`).
    val unusedAndEnt0Map = List(
      { lane: 2, ent: 0 },
      { lane: 0, ent: 0 },
      { lane: 3, ent: 3 },
      { lane: 0, ent: 0 }
    )
    val unusedAndEnt0Res = decode(0, 0, 0, log, DEFAULT_CALL_LANES, unusedAndEnt0Map)

    init.expect(and {
      not(dupNonZeroRes.ok),
      dupZeroRes.ok and dupZeroRes.stmt.nth(2) == 25,
      not(soundForResult(ZERO_OUTPUT_LOG, DEFAULT_CALL_LANES, dupNonZeroMap, dupZeroRes)),
      not(dupZeroFixedRes.ok),
      lane0And1Res.ok and lane0And1Res.stmt.nth(0) == 33 and lane0And1Res.stmt.nth(1) == 21,
      unusedAndEnt0Res.ok and unusedAndEnt0Res.stmt.nth(0) == 90 and unusedAndEnt0Res.stmt.nth(2) == 21,
    })
  }

  // 3. Question 3: Out-of-range entry index (`ent >= offs.length()`, `ent < 0`, or padding `NOP`)
  //    and out-of-range lane index (`lane >= N_LANES`, `lane < 0`).
  run outOfRangeRefusedTest = {
    val log = makeCaseLog(0, 0, 0)
    val mapEntBeyondB = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 11 })
    val mapEntNeg = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: -1 })
    val mapEntAtNop = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 10 })
    val mapLaneBeyond = HONEST_LANE_MAP.replaceAt(0, { lane: 8, ent: 4 })
    val mapLaneNeg = HONEST_LANE_MAP.replaceAt(0, { lane: -1, ent: 4 })
    val resEntBeyondB = decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapEntBeyondB)
    val resEntNeg = decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapEntNeg)
    val resEntAtNop = decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapEntAtNop)
    val resLaneBeyond = decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapLaneBeyond)
    val resLaneNeg = decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapLaneNeg)
    init.expect(and {
      not(resEntBeyondB.ok) and resEntBeyondB.errTag == "lane_map_entry",
      not(resEntNeg.ok) and resEntNeg.errTag == "lane_map_entry",
      not(resEntAtNop.ok) and resEntAtNop.errTag == "lane_map_pair",
      not(resLaneBeyond.ok) and resLaneBeyond.errTag == "lane_map_pair",
      not(resLaneNeg.ok) and resLaneNeg.errTag == "lane_map_pair",
    })
  }

  // 4. Question 4: Binding of R (`r_entries` and `claim_pub` inputs) vs. what the reader leaves to the wrap
  //    (pointing `claim_pub` lane at a discretionary DIGEST or a discretionary SPONGE(3) outside R).
  run bindingAndDiscretionaryTargetTest = {
    val log = makeCaseLog(0, 0, 0)
    // Tampering with R's SPONGE(3) inputs so `vs[6..9] != [vs[1], vs[3], vs[5]]` is refused:
    val badClaimInLog = log.replaceAt(3, { op: OP_SPONGE, param: 3, values: List(99, 22, 23, 33) })
    val badClaimInRes = decode(0, 0, 0, badClaimInLog, DEFAULT_CALL_LANES, HONEST_LANE_MAP)

    // Pointing `claim_pub` lane 3 at discretionary DIGEST entry 5 (output 25) is ACCEPTED by `decode`:
    val mapClaimToDiscDigest = HONEST_LANE_MAP.replaceAt(1, { lane: 3, ent: 5 })
    val resDiscDigest = decode(0, 0, 0, log, DEFAULT_CALL_LANES, mapClaimToDiscDigest)

    // Pointing `claim_pub` lane 3 at discretionary SPONGE(3) entry 8 outside R (output 84) is ACCEPTED by `decode`:
    val mapClaimToExtraSponge3 = HONEST_LANE_MAP.replaceAt(1, { lane: 3, ent: 8 })
    val resExtraSponge3 = decode(0, 0, 0, EXTRA_SPONGE3_LOG, DEFAULT_CALL_LANES, mapClaimToExtraSponge3)

    init.expect(and {
      not(badClaimInRes.ok) and badClaimInRes.errTag == "reservation",
      resDiscDigest.ok and resDiscDigest.stmt.nth(3) == 25,
      resExtraSponge3.ok and resExtraSponge3.stmt.nth(3) == 84,
    })
  }

  // 5. Question 5: Interplay with `PUB(i)` (checked after the lane map) and the zero-limb rule.
  run pubAndZeroLimbInterplayTest = {
    val log = makeCaseLog(0, 0, 0)
    // Non-zero limb in mapped lane 2 of call vector -> refused by `s[lane] != 0`
    val nonZeroMappedCall = DEFAULT_CALL_LANES.replaceAt(2, 12)
    val resNonZeroMapped = decode(0, 0, 0, log, nonZeroMappedCall, HONEST_LANE_MAP)

    // PUB(6) mismatch on unmapped lane 6 -> refused by post-lane-map PUB check
    val badPubCall = DEFAULT_CALL_LANES.replaceAt(6, 78)
    val resBadPub = decode(0, 0, 0, log, badPubCall, HONEST_LANE_MAP)

    // PUB(2) targeting a MAPPED lane (lane 2, mapped to entry 4 whose output is 21):
    // Since PUB is checked AFTER the lane map updates `s`, PUB(2) must log 21 (post-lane-map `s[2]`),
    // NOT 0 (pre-lane-map `callLanes[2]`).
    val logPubMappedPost = log.replaceAt(9, { op: OP_PUB, param: 2, values: List(21) })
    val logPubMappedPre = log.replaceAt(9, { op: OP_PUB, param: 2, values: List(0) })
    val resPubMappedPost = decode(0, 0, 0, logPubMappedPost, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    val resPubMappedPre = decode(0, 0, 0, logPubMappedPre, DEFAULT_CALL_LANES, HONEST_LANE_MAP)

    init.expect(and {
      not(resNonZeroMapped.ok) and resNonZeroMapped.errTag == "lane_map_pair",
      not(resBadPub.ok) and resBadPub.errTag == "pub_mismatch",
      resPubMappedPost.ok,
      not(resPubMappedPre.ok) and resPubMappedPre.errTag == "pub_mismatch",
    })
  }

  // 6. Question 6: Completeness across ordinary (`mode = 0`) and chunk (`mode = 1`) honest runs.
  run completenessHonestCasesTest =
    init
      .then(stepHonest((0, 0, 0)))
      .expect(SOUND and COMPLETE and state.decoded.stmt == List(90, 91, 21, 33, 33, 23, 77, 0))
      .then(stepHonest((1, 1, 3)))
      .expect(SOUND and COMPLETE and state.decoded.stmt == List(90, 91, 21, 33, 33, 23, 77, 0))
      .then(stepHonest((1, 2, 3)))
      .expect(SOUND and COMPLETE and state.decoded.stmt == List(90, 91, 24, 33, 33, 23, 77, 0))
      .then(stepHonest((1, 3, 3)))
      .expect(SOUND and COMPLETE and state.decoded.stmt == List(90, 91, 24, 33, 33, 23, 77, 0))
      .then(stepHonest((1, 1, 1)))
      .expect(SOUND and COMPLETE and state.decoded.stmt == List(90, 91, 21, 33, 33, 23, 77, 0))

  // 7. Mutant 1 test: catches `mut_any_sponge` accepting `SPONGE(2)` (entry 8).
  run mutAnySpongeCaughtTest = {
    val log = makeCaseLog(0, 0, 0)
    val sponge2Map = HONEST_LANE_MAP.replaceAt(1, { lane: 3, ent: 8 })
    val realRes = decode(0, 0, 0, log, DEFAULT_CALL_LANES, sponge2Map)
    val mutRes = mut_any_sponge(0, 0, 0, log, DEFAULT_CALL_LANES, sponge2Map)
    init.expect(and {
      not(realRes.ok),
      mutRes.ok,
      not(soundForResult(log, DEFAULT_CALL_LANES, sponge2Map, mutRes)),
    })
  }

  // 8. Mutant 2 test: catches `mut_first_value_not_last` reading `vs[o]` instead of `vs[o + vals - 1]`.
  run mutFirstValueNotLastCaughtTest = {
    val log = makeCaseLog(0, 0, 0)
    val realRes = decode(0, 0, 0, log, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    val mutRes = mut_first_value_not_last(0, 0, 0, log, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    init.expect(and {
      realRes.ok and soundForResult(log, DEFAULT_CALL_LANES, HONEST_LANE_MAP, realRes),
      mutRes.ok,
      // Entry 4 first value is 11 (last is 21); Entry 3 first value is 21 (last is 33)
      mutRes.stmt.nth(2) == 11 and mutRes.stmt.nth(3) == 21,
      not(soundForResult(log, DEFAULT_CALL_LANES, HONEST_LANE_MAP, mutRes)),
    })
  }

  // 9. Mutant 3 test: catches `mut_no_dup_check` accepting duplicate lane mappings and non-zero call lanes.
  run mutNoDupCheckCaughtTest = {
    val log = makeCaseLog(0, 0, 0)
    val dupMap = List(
      { lane: 2, ent: 4 },
      { lane: 2, ent: 5 },
      { lane: 3, ent: 3 },
      { lane: 5, ent: 2 }
    )
    val realDupRes = decode(0, 0, 0, log, DEFAULT_CALL_LANES, dupMap)
    val mutDupRes = mut_no_dup_check(0, 0, 0, log, DEFAULT_CALL_LANES, dupMap)
    val nonZeroLanes = DEFAULT_CALL_LANES.replaceAt(2, 99)
    val realNzRes = decode(0, 0, 0, log, nonZeroLanes, HONEST_LANE_MAP)
    val mutNzRes = mut_no_dup_check(0, 0, 0, log, nonZeroLanes, HONEST_LANE_MAP)
    init.expect(and {
      not(realDupRes.ok),
      mutDupRes.ok,
      not(soundForResult(log, DEFAULT_CALL_LANES, dupMap, mutDupRes)),
      not(realNzRes.ok),
      mutNzRes.ok,
      not(soundForResult(log, nonZeroLanes, HONEST_LANE_MAP, mutNzRes)),
    })
  }

  // 10. Mutant 4 test: catches `mut_any_op` accepting PERM (6), H2 (7), and PUB (9) entries.
  run mutAnyOpCaughtTest = {
    val log = makeCaseLog(0, 0, 0)
    val permMap = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 6 })
    val h2Map = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 7 })
    val pubMap = HONEST_LANE_MAP.replaceAt(0, { lane: 2, ent: 9 })
    val realPerm = decode(0, 0, 0, log, DEFAULT_CALL_LANES, permMap)
    val mutPerm = mut_any_op(0, 0, 0, log, DEFAULT_CALL_LANES, permMap)
    val realH2 = decode(0, 0, 0, log, DEFAULT_CALL_LANES, h2Map)
    val mutH2 = mut_any_op(0, 0, 0, log, DEFAULT_CALL_LANES, h2Map)
    val realPub = decode(0, 0, 0, log, DEFAULT_CALL_LANES, pubMap)
    val mutPub = mut_any_op(0, 0, 0, log, DEFAULT_CALL_LANES, pubMap)
    init.expect(and {
      not(realPerm.ok) and mutPerm.ok and not(soundForResult(log, DEFAULT_CALL_LANES, permMap, mutPerm)),
      not(realH2.ok) and mutH2.ok and not(soundForResult(log, DEFAULT_CALL_LANES, h2Map, mutH2)),
      not(realPub.ok) and mutPub.ok and not(soundForResult(log, DEFAULT_CALL_LANES, pubMap, mutPub)),
    })
  }

  // 11. Mutant 5 test: catches `mut_pub_before_lanemap` checking PUB(i) before lane-map updates.
  run mutPubBeforeLanemapCaughtTest = {
    val log = makeCaseLog(0, 0, 0)
    val logPub2Zero = log.replaceAt(9, { op: OP_PUB, param: 2, values: List(0) })
    val realRes = decode(0, 0, 0, logPub2Zero, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    val mutRes = mut_pub_before_lanemap(0, 0, 0, logPub2Zero, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    init.expect(and {
      not(realRes.ok),
      mutRes.ok,
      not(soundForResult(logPub2Zero, DEFAULT_CALL_LANES, HONEST_LANE_MAP, mutRes)),
    })
  }

  // 12. Random adversarial lane-map generator test.
  run randomAdversarialLaneMapTest = {
    nondet cfg = HONEST_CONFIGS.oneOf()
    nondet p0 = ADVERSARIAL_PAIRS.oneOf()
    nondet p1 = ADVERSARIAL_PAIRS.oneOf()
    nondet p2 = ADVERSARIAL_PAIRS.oneOf()
    nondet p3 = ADVERSARIAL_PAIRS.oneOf()
    nondet lane2Override = Set(0, 99).oneOf()
    init
      .then(stepAdversarial(cfg, p0, p1, p2, p3, lane2Override))
      .expect(SOUND and COMPLETE)
  }

}
