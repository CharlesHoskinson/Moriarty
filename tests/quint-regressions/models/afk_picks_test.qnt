// -*- mode: Bluespec; -*-
// Deterministic unit, non-vacuity, mutant-attack, and EXHAUSTIVE finite-domain tests
// for QA lane `afk-picks`. Every `run` name ends in `Test` so `quint test` executes all of them.

module afk_picks_test {
  import afk_picks_configs.* from "./afk_picks"
  import afk_picks_base(CFG = CFG_SOUND).* from "./afk_picks"

  // =========================================================================
  // 1. Stateful trace tests on `afk_picks_sound`
  // =========================================================================

  run soundInitAndStepRefineTest =
    init
      .expect(acc and invRefine)
      .then(stepCheckGP4Shape)
      .expect(acc and invGP4NotNarrowed and invRefine)
      .then(stepLongRunAfterAdmission)
      .expect(acc and invLongRunSurvives and invRefine)
      .then(stepAttackRevokedForeign)
      .expect(not(acc) and invNoRevokedForeign and invRefine)
      .then(stepAttackMixedRootJoin)
      .expect(not(acc) and invHistoryJoinSafe and invRefine)
      .then(stepOpenClaimLifecycle)
      .expect(invRefine)
      .then(stepAttackBusOrStitchTamper)
      .expect(not(acc) and invRefine)

  // =========================================================================
  // 2. Non-vacuity: Honest witness accepted & refines under sound AND every mutant
  //    (`PROMPT.md` line 18)
  // =========================================================================

  // Mutants 1, 2, 3, 4, 8, 9 preserve the honest baseline witness acceptance and full `pureRefines`:
  run honestWitnessRefinesUnderNonLayoutMutantsTest =
    init.expect(
      val non_layout_cfgs = Set(
        CFG_SOUND,
        CFG_OPT_RECHECK_OPEN,
        CFG_OPT_FOREIGN_BINDS_ROOT,
        CFG_M_LATEST_ROOT_ONLY,
        CFG_M_NO_REVOCATION,
        CFG_M_JOIN_MIXED_ROOTS,
        CFG_M_SHAPE_NO_INTERVALS,
        CFG_M_AS_BUILT_N1_ZERO_REG_ROOT,
        CFG_M_AS_BUILT_N1_ZERO_SHAPE
      )
      non_layout_cfgs.forall(c =>
        val s0 = mkBaseState(c, 20, 20)
        val a0 = evalAccepted(c, s0)
        a0 and pureRefines(c, s0, a0)
      )
    )

  // Mutants 5, 6, 7 (`bound_4400`, `realistic_stacks_vd_5400`, `unsplit_core2_5400`) intentionally
  // change the static key-set column/constraint bound itself (`PROMPT.md` line 18: "unless the mutant
  // changes the honest layout itself; then say so"). Verify their honest DAG/ledger witness STILL
  // accepts and satisfies all non-budget soundness & history invariants, failing ONLY `budgetFitsAt`:
  run honestWitnessUnderLayoutBoundMutantsTest =
    init.expect(
      val s_m5 = mkBaseState(CFG_M_BOUND_4400, 0, 0)
      val s_m6 = mkBaseState(CFG_M_REALISTIC_STACKS_VD_5400, 71, 62)
      val s_m7 = mkBaseState(CFG_M_UNSPLIT_CORE2_5400, 0, 62)
      evalAccepted(CFG_M_BOUND_4400, s_m5)
        and pureWitnessSoundnessRefines(s_m5, true)
        and pureNoRevokedForeign(CFG_M_BOUND_4400, s_m5, true)
        and pureHistoryJoinSafe(CFG_M_BOUND_4400, s_m5, true)
        and not(budgetFitsAt(CFG_M_BOUND_4400, 0, 0))
        and evalAccepted(CFG_M_REALISTIC_STACKS_VD_5400, s_m6)
        and pureWitnessSoundnessRefines(s_m6, true)
        and pureNoRevokedForeign(CFG_M_REALISTIC_STACKS_VD_5400, s_m6, true)
        and pureHistoryJoinSafe(CFG_M_REALISTIC_STACKS_VD_5400, s_m6, true)
        and not(budgetFitsAt(CFG_M_REALISTIC_STACKS_VD_5400, 71, 62))
        and evalAccepted(CFG_M_UNSPLIT_CORE2_5400, s_m7)
        and pureWitnessSoundnessRefines(s_m7, true)
        and pureNoRevokedForeign(CFG_M_UNSPLIT_CORE2_5400, s_m7, true)
        and pureHistoryJoinSafe(CFG_M_UNSPLIT_CORE2_5400, s_m7, true)
        and not(budgetFitsAt(CFG_M_UNSPLIT_CORE2_5400, 0, 62))
    )

  // =========================================================================
  // 3. Deterministic Real-Attack / Failure Witnesses for all 9 Mutants
  // =========================================================================

  // Mutant 1 (`latest_root_only`): honest long run started at `R0` spanning admission of `FKEY_2` (`R1`)
  // is accepted in `afk_picks_sound` and refused as stale in `afk_picks_m_latest_root_only`.
  run mutantLatestRootOnlyCaughtTest =
    init.expect(
      val sound_st = nextLongRunAfterAdmission(CFG_SOUND, 10, 10)
      val mut_st = nextLongRunAfterAdmission(CFG_M_LATEST_ROOT_ONLY, 10, 10)
      val a_sound = evalAccepted(CFG_SOUND, sound_st)
      val a_mut = evalAccepted(CFG_M_LATEST_ROOT_ONLY, mut_st)
      a_sound and pureRefines(CFG_SOUND, sound_st, a_sound)
        and not(a_mut)
        and not(pureLongRunSurvives(mut_st, a_mut))
        and not(pureRefines(CFG_M_LATEST_ROOT_ONLY, mut_st, a_mut))
    )

  // Mutant 2 (`no_revocation`): collection under `R0` using revoked `FKEY_1` after `R2`
  // is refused in `afk_picks_sound` and accepted in `afk_picks_m_no_revocation`.
  run mutantNoRevocationCaughtTest =
    init.expect(
      val sound_st = nextAttackRevokedForeign(CFG_SOUND, 10, 10)
      val mut_st = nextAttackRevokedForeign(CFG_M_NO_REVOCATION, 10, 10)
      val a_sound = evalAccepted(CFG_SOUND, sound_st)
      val a_mut = evalAccepted(CFG_M_NO_REVOCATION, mut_st)
      not(a_sound) and pureRefines(CFG_SOUND, sound_st, a_sound)
        and a_mut
        and not(pureNoRevokedForeign(CFG_M_NO_REVOCATION, mut_st, a_mut))
        and not(pureRefines(CFG_M_NO_REVOCATION, mut_st, a_mut))
    )

  // Mutant 3 (`join_mixed_roots`): joining valid `R2` left subtree with revoked `R0` (or unregistered `R_UNREG`)
  // right subtree is refused by `RegRootEqualAcrossSlots` in `afk_picks_sound`, and accepted in `CFG_M_JOIN_MIXED_ROOTS`.
  run mutantJoinMixedRootsCaughtTest =
    init.expect(
      Set(R0, R_UNREG).forall(bad_r =>
        val st_s = nextAttackMixedRootJoin(CFG_SOUND, bad_r, 10, 10)
        val st_m = nextAttackMixedRootJoin(CFG_M_JOIN_MIXED_ROOTS, bad_r, 10, 10)
        val a_s = evalAccepted(CFG_SOUND, st_s)
        val a_m = evalAccepted(CFG_M_JOIN_MIXED_ROOTS, st_m)
        not(a_s) and pureRefines(CFG_SOUND, st_s, a_s)
          and a_m
          and not(pureHistoryJoinSafe(CFG_M_JOIN_MIXED_ROOTS, st_m, a_m))
          and not(pureNoRevokedForeign(CFG_M_JOIN_MIXED_ROOTS, st_m, a_m))
          and not(pureRefines(CFG_M_JOIN_MIXED_ROOTS, st_m, a_m))
      )
    )

  // Mutant 4 (`shape_no_intervals`): two N2 trees `[0,2)+[2,8)` vs `[0,5)+[5,8)` have distinct `shape_d`
  // in `afk_picks_sound`, but collide on `shape_d` under N-7 option "claim graph only" (`CFG_M_SHAPE_NO_INTERVALS`).
  run mutantShapeNoIntervalsCaughtTest =
    init.expect(
      val st_s = nextAttackN2IntervalCollision(CFG_SOUND, 5, 10, 10)
      val st_m = nextAttackN2IntervalCollision(CFG_M_SHAPE_NO_INTERVALS, 5, 10, 10)
      val a_s = evalAccepted(CFG_SOUND, st_s)
      val a_m = evalAccepted(CFG_M_SHAPE_NO_INTERVALS, st_m)
      not(a_s) and pureRefines(CFG_SOUND, st_s, a_s)
        and a_m
        and not(pureWitnessSoundnessRefines(st_m, a_m))
        and not(pureRefines(CFG_M_SHAPE_NO_INTERVALS, st_m, a_m))
    )

  // Mutants 8 & 9 (AS-BUILT code defects in `stmt.rs:854, 980, 1036`):
  //   - M8 (`as_built_n1_zero_reg_root`): N1 hardcodes `reg_root = 0`, refusing GP-3/GP-4 composition with Foreign `R0 = 10`.
  //   - M9 (`as_built_n1_zero_shape`): N1 sets `shape_d = 0`, erasing BOTH binary-N1 leaf-cuts `[0,1)+[1,4)` vs `[0,2)+[2,4)`
  //     AND binary-N2 segment-cuts `[0,2)+[2,8)` vs `[0,5)+[5,8)` over unary N1 leaves!
  run mutantsAsBuiltRustGapsCaughtTest =
    init.expect(
      val st_s4 = nextCheckGP4Shape(CFG_SOUND, 4, 10, 10)
      val st_m8 = nextCheckGP4Shape(CFG_M_AS_BUILT_N1_ZERO_REG_ROOT, 4, 10, 10)
      val a_s4 = evalAccepted(CFG_SOUND, st_s4)
      val a_m8 = evalAccepted(CFG_M_AS_BUILT_N1_ZERO_REG_ROOT, st_m8)

      val st_m9_n1 = nextAttackN1InternalSplitCollision(CFG_M_AS_BUILT_N1_ZERO_SHAPE, 2, 10, 10)
      val st_m9_n2 = nextAttackN2IntervalCollision(CFG_M_AS_BUILT_N1_ZERO_SHAPE, 5, 10, 10)
      val a_m9_n1 = evalAccepted(CFG_M_AS_BUILT_N1_ZERO_SHAPE, st_m9_n1)
      val a_m9_n2 = evalAccepted(CFG_M_AS_BUILT_N1_ZERO_SHAPE, st_m9_n2)

      a_s4 and pureRefines(CFG_SOUND, st_s4, a_s4)
        and not(st_m8.sub_node.ok) and not(a_m8)
        and not(pureGP4NotNarrowed(st_m8, a_m8))
        and not(pureRefines(CFG_M_AS_BUILT_N1_ZERO_REG_ROOT, st_m8, a_m8))
        and a_m9_n1 and not(pureRefines(CFG_M_AS_BUILT_N1_ZERO_SHAPE, st_m9_n1, a_m9_n1))
        and a_m9_n2 and not(pureRefines(CFG_M_AS_BUILT_N1_ZERO_SHAPE, st_m9_n2, a_m9_n2))
    )

  // =========================================================================
  // 4. EXHAUSTIVE Pure Finite-Domain Verification (`PROMPT.md` line 19)
  // =========================================================================

  // Exhaustive Check 1: All 4 GP-4 reference shapes + full `MERGE-WORK -> NW -> NW2 -> N3` ladder interval preservation
  run exhaustiveGP4AllReferenceShapesAndLadderTest =
    init.expect(
      val all_shapes_ok = Set(1, 2, 3, 4).forall(gid =>
        Set(R0, R1).forall(r =>
          val rs = admitKeyR1(INIT_REG_STATE)
          val dag = buildGP4ById(CFG_SOUND, gid, r, rs)
          val s = {
            ...mkBaseState(CFG_SOUND, 0, 0),
            sub_mode: SubCheckGP4Dag,
            reg_st: rs,
            sub_node: dag,
            gp4_shape_id: gid
          }
          val a = evalAccepted(CFG_SOUND, s)
          a and pureRefines(CFG_SOUND, s, a)
        )
      )
      // Verify exact interval preservation `(0, 12)` across every rung `MERGE-WORK -> NW -> NW2 -> N3`:
      val lf0 = mkLeaf(102, 20, 21, 0, 3, OUT_RUNNING, D_EMPTY_DEPS)
      val lf1 = mkLeaf(102, 21, 22, 3, 4, OUT_RUNNING, D_EMPTY_DEPS)
      val wk = mkLeaf(102, 22, 23, 7, 5, OUT_HALTED, D_EMPTY_DEPS)
      val mw = dagMergeWork(CFG_SOUND, dagFirstN1(CFG_SOUND, lf0, false, lf0, R0), dagFirstN1(CFG_SOUND, lf1, false, lf1, R0), wk, 7)
      val nw = dagMerge(CFG_SOUND, mw, false, mw, 4)
      val nw2 = dagMerge(CFG_SOUND, nw, false, nw, 5)
      val n3_seg = dagMerge(CFG_SOUND, nw2, false, nw2, 6)
      val n3_seal = dagSeal(nw2, 6)
      // Also check that skipping NW (`MERGE-WORK` row 7 directly into `NW2` row 5 or `N3` row 6) is refused:
      val skip_nw_bad1 = dagMerge(CFG_SOUND, mw, false, mw, 5)
      val skip_nw_bad2 = dagSeal(mw, 6)
      all_shapes_ok
        and mw.ok and (mw.h_lo == 0) and (mw.h_hi == 12)
        and nw.ok and (nw.h_lo == 0) and (nw.h_hi == 12)
        and nw2.ok and (nw2.h_lo == 0) and (nw2.h_hi == 12)
        and n3_seg.ok and (n3_seg.h_lo == 0) and (n3_seg.h_hi == 12)
        and n3_seal.ok and (n3_seal.exec_splits == List((0, 3), (3, 7), (7, 12)))
        and not(skip_nw_bad1.ok) and not(skip_nw_bad2.ok)
    )

  // Exhaustive Check 2: N-7 interval-binding soundness over ALL binary-N1 splits, ALL binary-N2 splits,
  // and ALL 3-way `MERGE-WORK` splits `(a, b)` over `[0, 6)`.
  run exhaustiveN7IntervalTamperDomainTest =
    init.expect(
      // (a) All 7 x 7 = 49 binary N1 splits `[0, k1) + [k1, 8)` vs `[0, k2) + [k2, 8)`:
      val n1_exact = 1.to(7).forall(k1 =>
        1.to(7).forall(k2 =>
          val seg1 = dagMerge(
            CFG_SOUND,
            dagFirstN1(CFG_SOUND, mkLeaf(300, 1, 2, 0, k1, OUT_RUNNING, D_EMPTY_DEPS), true, mkLeaf(300, 2, 3, k1, 8 - k1, OUT_HALTED, D_EMPTY_DEPS), R0),
            false, INVALID_NODE, 1
          )
          val seg2 = dagMerge(
            CFG_SOUND,
            dagFirstN1(CFG_SOUND, mkLeaf(300, 1, 2, 0, k2, OUT_RUNNING, D_EMPTY_DEPS), true, mkLeaf(300, 2, 3, k2, 8 - k2, OUT_HALTED, D_EMPTY_DEPS), R0),
            false, INVALID_NODE, 1
          )
          val s = {
            ...mkBaseState(CFG_SOUND, 0, 0),
            sub_mode: SubShapePinnedSeg,
            is_tampered: (k1 != k2),
            sub_node: seg2,
            expected_h_lo: 0,
            expected_h_hi: 8,
            expected_shape_d: seg1.shape_d,
            expected_splits: seg1.exec_splits
          }
          val a = evalAccepted(CFG_SOUND, s)
          (a == (k1 == k2)) and pureRefines(CFG_SOUND, s, a)
        )
      )
      // (b) All 7 x 7 = 49 binary N2 splits `unary_N1([0, k1)) + unary_N1([k1, 8))`:
      val n2_exact = 1.to(7).forall(k1 =>
        1.to(7).forall(k2 =>
          val seg1 = dagMerge(
            CFG_SOUND,
            dagFirstN1(CFG_SOUND, mkLeaf(301, 1, 2, 0, k1, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(301, 1, 2, 0, k1, OUT_RUNNING, D_EMPTY_DEPS), R0),
            true,
            dagFirstN1(CFG_SOUND, mkLeaf(301, 2, 3, k1, 8 - k1, OUT_HALTED, D_EMPTY_DEPS), false, mkLeaf(301, 2, 3, k1, 8 - k1, OUT_HALTED, D_EMPTY_DEPS), R0),
            1
          )
          val seg2 = dagMerge(
            CFG_SOUND,
            dagFirstN1(CFG_SOUND, mkLeaf(301, 1, 2, 0, k2, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(301, 1, 2, 0, k2, OUT_RUNNING, D_EMPTY_DEPS), R0),
            true,
            dagFirstN1(CFG_SOUND, mkLeaf(301, 2, 3, k2, 8 - k2, OUT_HALTED, D_EMPTY_DEPS), false, mkLeaf(301, 2, 3, k2, 8 - k2, OUT_HALTED, D_EMPTY_DEPS), R0),
            1
          )
          val s = {
            ...mkBaseState(CFG_SOUND, 0, 0),
            sub_mode: SubShapePinnedSeg,
            is_tampered: (k1 != k2),
            sub_node: seg2,
            expected_h_lo: 0,
            expected_h_hi: 8,
            expected_shape_d: seg1.shape_d,
            expected_splits: seg1.exec_splits
          }
          val a = evalAccepted(CFG_SOUND, s)
          (a == (k1 == k2)) and pureRefines(CFG_SOUND, s, a)
        )
      )
      // (c) All 3-way `MERGE-WORK` splits `[0, a) + [a, b) + inline [b, 6)` (`1 <= a < b <= 5`, 10 x 10 = 100 pairs):
      val mw_cuts = Set((1, 2), (1, 3), (1, 4), (1, 5), (2, 3), (2, 4), (2, 5), (3, 4), (3, 5), (4, 5))
      val mw_exact = mw_cuts.forall(p1 =>
        mw_cuts.forall(p2 =>
          val a1 = p1._1
          val b1 = p1._2
          val a2 = p2._1
          val b2 = p2._2
          val mw1 = dagMergeWork(
            CFG_SOUND,
            dagFirstN1(CFG_SOUND, mkLeaf(302, 1, 2, 0, a1, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(302, 1, 2, 0, a1, OUT_RUNNING, D_EMPTY_DEPS), R0),
            dagFirstN1(CFG_SOUND, mkLeaf(302, 2, 3, a1, b1 - a1, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(302, 2, 3, a1, b1 - a1, OUT_RUNNING, D_EMPTY_DEPS), R0),
            mkLeaf(302, 3, 4, b1, 6 - b1, OUT_HALTED, D_EMPTY_DEPS),
            7
          )
          val mw2 = dagMergeWork(
            CFG_SOUND,
            dagFirstN1(CFG_SOUND, mkLeaf(302, 1, 2, 0, a2, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(302, 1, 2, 0, a2, OUT_RUNNING, D_EMPTY_DEPS), R0),
            dagFirstN1(CFG_SOUND, mkLeaf(302, 2, 3, a2, b2 - a2, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(302, 2, 3, a2, b2 - a2, OUT_RUNNING, D_EMPTY_DEPS), R0),
            mkLeaf(302, 3, 4, b2, 6 - b2, OUT_HALTED, D_EMPTY_DEPS),
            7
          )
          (mw1.shape_d == mw2.shape_d) == (p1 == p2)
        )
      )
      n1_exact and n2_exact and mw_exact
    )

  // Exhaustive Check 3: N-8 registry history, revocation, foreign lift, and mixed-root join/DL/SEALD domain
  // across ALL 3 registry epochs x ALL 5 roots x ALL 3 foreign keys x ALL 5 x 5 root pairs.
  run exhaustiveN8RegistryAndMixedRootDomainTest =
    init.expect(
      val epochs = Set(
        INIT_REG_STATE,
        admitKeyR1(INIT_REG_STATE),
        revokeKey1ToR2(admitKeyR1(INIT_REG_STATE))
      )
      val all_roots = Set(R_ZERO, R0, R1, R2, R_UNREG)
      val all_fkeys = Set(FKEY_1, FKEY_2, FKEY_3)

      // (a) Single-foreign-lift submissions across all epochs x roots x keys:
      val single_lifts_sound = epochs.forall(ep =>
        all_roots.forall(r =>
          all_fkeys.forall(fk =>
            val fnode = dagForeign(CFG_SOUND, r, fk, 777, ep)
            val s = { ...mkBaseState(CFG_SOUND, 0, 0), is_tampered: true, reg_st: ep, sub_node: fnode }
            val a = evalAccepted(CFG_SOUND, s)
            pureRefines(CFG_SOUND, s, a)
              and (a == (
                ep.published_roots.contains(r)
                  and ep.root_fkeys.get(r).contains(fk)
                  and (ep.root_fkeys.get(r).intersect(ep.revoked_fkeys) == Set())
              ))
          )
        )
      )

      // (b) All 5 x 5 = 25 `(r_left, r_right)` pairs in `dagMerge`, `dagJoin`, `dagDlChain`, `dagSealWithDeps`:
      val ep_full = revokeKey1ToR2(admitKeyR1(INIT_REG_STATE))
      val mixed_pairs_refused = all_roots.forall(rL =>
        all_roots.forall(rR =>
          val segL = dagFirstN1(CFG_SOUND, mkLeaf(400, 1, 2, 0, 2, OUT_RUNNING, D_EMPTY_DEPS), false, mkLeaf(400, 1, 2, 0, 2, OUT_RUNNING, D_EMPTY_DEPS), rL)
          val segR = dagFirstN1(CFG_SOUND, mkLeaf(400, 2, 3, 2, 2, OUT_HALTED, D_EMPTY_DEPS), false, mkLeaf(400, 2, 3, 2, 2, OUT_HALTED, D_EMPTY_DEPS), rR)
          val m_seg = dagMerge(CFG_SOUND, segL, true, segR, 1)

          val setL = buildGP4Chain(CFG_SOUND, rL)
          val setR = buildGP4Chain(CFG_SOUND, rR)
          val j_set = dagJoin(CFG_SOUND, setL, setR, 6)

          val dl0 = dagDlChain(CFG_SOUND, setL, false, INVALID_NODE, 2)
          val dl_pair = dagDlChain(CFG_SOUND, setR, true, dl0, 2)

          val dep_seg = dagFirstN1(
            CFG_SOUND,
            mkLeaf(401, 1, 2, 0, 2, OUT_HALTED, hashDepsCons(D_EMPTY_DEPS, setR.head_claim)),
            false,
            mkLeaf(401, 1, 2, 0, 2, OUT_HALTED, hashDepsCons(D_EMPTY_DEPS, setR.head_claim)),
            rL
          )
          val dl_for_seald = dagDlChain(CFG_SOUND, setR, false, INVALID_NODE, 2)
          val seald_pair = dagSealWithDeps(CFG_SOUND, dep_seg, dl_for_seald, 3)

          val s_join = { ...mkBaseState(CFG_SOUND, 0, 0), is_tampered: (rL != rR), reg_st: ep_full, sub_node: j_set }
          val a_join = evalAccepted(CFG_SOUND, s_join)

          ((rL != rR) implies (not(m_seg.ok) and not(j_set.ok) and not(dl_pair.ok) and not(seald_pair.ok) and not(a_join)))
            and ((rL == rR) implies (m_seg.ok and j_set.ok and dl_pair.ok and seald_pair.ok))
            and pureRefines(CFG_SOUND, s_join, a_join)
        )
      )
      single_lifts_sound and mixed_pairs_refused
    )

  // Exhaustive Check 4: Open Design Points 1 & 2 (comparing both options honestly without inventing rulings)
  run exhaustiveOpenDesignPointsComparisonTest =
    init.expect(
      val bools = Set(false, true)
      // Open Point 1: Option A (`CFG_SOUND`: `acceptCollection` finality) vs
      // Option B (`CFG_OPT_RECHECK_OPEN`: `openClaim` re-checks `reg_root` unrevoked).
      val open_pt1_ok = bools.forall(rev_after =>
        bools.forall(already_settled =>
          val st_a = nextOpenClaimLifecycle(CFG_SOUND, rev_after, already_settled, 0, 0)
          val acc_a = evalAccepted(CFG_SOUND, st_a)

          val st_b = nextOpenClaimLifecycle(CFG_OPT_RECHECK_OPEN, rev_after, already_settled, 0, 0)
          val acc_b = evalAccepted(CFG_OPT_RECHECK_OPEN, st_b)

          // Already-settled claims refuse replay in BOTH options;
          // Unopened claims in an already-accepted collection succeed under Option A (`not(already_settled)`)
          // and succeed under Option B iff `not(already_settled) and not(rev_after)`:
          (acc_a == not(already_settled))
            and (acc_b == (not(already_settled) and not(rev_after)))
            and pureRefines(CFG_SOUND, st_a, acc_a)
            and pureRefines(CFG_OPT_RECHECK_OPEN, st_b, acc_b)
        )
      )

      // Open Point 2: `CFG_OPT_FOREIGN_BINDS_ROOT` (`node_equal_reg_root = false, foreign_binds_reg_root = true`)
      // blocks direct `openClaim` on a revoked foreign root, BUT still lets `acceptCollection` accept an opaque
      // `set_pub` containing a revoked/unregistered foreign leaf!
      val st_fr_accept = nextAttackMixedRootJoin(CFG_OPT_FOREIGN_BINDS_ROOT, R0, 0, 0)
      val acc_fr_accept = evalAccepted(CFG_OPT_FOREIGN_BINDS_ROOT, st_fr_accept)

      val st_fr_open_revoked = nextOpenClaimLifecycle(CFG_OPT_FOREIGN_BINDS_ROOT, true, false, 0, 0)
      val acc_fr_open_revoked = evalAccepted(CFG_OPT_FOREIGN_BINDS_ROOT, st_fr_open_revoked)

      open_pt1_ok
        and acc_fr_accept
        and pureHistoryJoinSafe(CFG_OPT_FOREIGN_BINDS_ROOT, st_fr_accept, acc_fr_accept)
        and not(pureNoRevokedForeign(CFG_OPT_FOREIGN_BINDS_ROOT, st_fr_accept, acc_fr_accept))
        and not(acc_fr_open_revoked)
    )

  // Exhaustive Check 5: Exact MERGE-WORK column budget boundary over `0..130` x `0..130` (17,161 pairs)
  // Proves `budgetFitsAt(CFG_SOUND, sc, vc) <==> (sc + vc <= 116)` at bound 5,400 (`5,284 + 116 = 5,400`),
  // and proves `CFG_M_BOUND_4400`, `CFG_M_UNSPLIT_CORE2_5400`, and `CFG_M_REALISTIC_STACKS_VD_5400`
  // (`sc >= 71, vc >= 62 => sc + vc >= 133 > 116`) NEVER fit!
  run exhaustiveMergeWorkBudgetBoundaryTest =
    init.expect(
      val exact_5400_boundary = 0.to(130).forall(sc =>
        0.to(130).forall(vc =>
          (budgetFitsAt(CFG_SOUND, sc, vc) == (sc + vc <= 116))
            and not(budgetFitsAt(CFG_M_BOUND_4400, sc, vc))
            and not(budgetFitsAt(CFG_M_UNSPLIT_CORE2_5400, sc, vc))
        )
      )
      val realistic_never_fits = 71.to(110).forall(sc =>
        62.to(110).forall(vc =>
          not(budgetFitsAt(CFG_M_REALISTIC_STACKS_VD_5400, sc, vc))
            and (totalMergeWorkCols(sc, vc) >= 5417)
        )
      )
      exact_5400_boundary and realistic_never_fits
        and (T8_MERGE_WORK_COLS + T8B_UROWS_DELTA_COLS + T8B_CORE_STEP_PV_COLS == T8B_MERGE_WORK_COLS)
        and (5400 - T8B_MERGE_WORK_COLS == 116)
    )

  // Exhaustive Check 6: Stitch, height-wrap (`H_WRAP_BOUND`), key-table half-row, seal predicate, and bus tampers
  run exhaustiveStitchWrapKeyRowAndBusTamperTest =
    init.expect(
      // (a) Height wrap around `H_WRAP_BOUND = 65536` (`stmt.rs:928-934`, `:1050-1055`):
      val wrap_checked = 1.to(10).forall(stps =>
        val lf = mkLeaf(500, 1, 2, H_WRAP_BOUND - 5, stps, OUT_HALTED, D_EMPTY_DEPS)
        val n1 = dagFirstN1(CFG_SOUND, lf, false, lf, R0)
        n1.ok == (H_WRAP_BOUND - 5 + stps < H_WRAP_BOUND)
      )
      // (b) Universal key table half-row rule (`stmt.rs:950`: Node `< UROWS`, Work `>= UROWS`):
      val lf_a = mkLeaf(501, 1, 2, 0, 2, OUT_RUNNING, D_EMPTY_DEPS)
      val lf_b = mkLeaf(501, 2, 3, 2, 2, OUT_RUNNING, D_EMPTY_DEPS)
      val wk_c = mkLeaf(501, 3, 4, 4, 2, OUT_HALTED, D_EMPTY_DEPS)
      val s_a = dagFirstN1(CFG_SOUND, lf_a, false, lf_a, R0)
      val s_b = dagFirstN1(CFG_SOUND, lf_b, false, lf_b, R0)
      val ukey_halves_ok = 0.to(2 * UROWS - 1).forall(r =>
        val node_try = dagMerge(CFG_SOUND, s_a, true, s_b, r)
        val work_try = dagMergeWork(CFG_SOUND, s_a, s_b, wk_c, r)
        (node_try.ok == (r < UROWS)) and (work_try.ok == (r >= UROWS))
      )
      // (c) `SEAL` refuses non-zero `h_lo`, `OUT_RUNNING`, and non-empty `deps_d` (`gp.rs:320-328`):
      val bad_hlo = dagSeal(dagFirstN1(CFG_SOUND, mkLeaf(502, 1, 2, 1, 3, OUT_HALTED, D_EMPTY_DEPS), false, lf_a, R0), 6)
      val bad_running = dagSeal(dagFirstN1(CFG_SOUND, mkLeaf(502, 1, 2, 0, 3, OUT_RUNNING, D_EMPTY_DEPS), false, lf_a, R0), 6)
      val bad_deps = dagSeal(dagFirstN1(CFG_SOUND, mkLeaf(502, 1, 2, 0, 3, OUT_HALTED, hashDepsCons(D_EMPTY_DEPS, dScalar(9))), false, lf_a, R0), 6)
      // (d) `STEP_PV_BUS` multiplicity `0..12` accepted iff `mult == STEP_PV_ROWS (8)` (`stmt.rs:1048`):
      val bus_mult_ok = 0.to(12).forall(m =>
        val s = { ...mkBaseState(CFG_SOUND, 0, 0), is_tampered: (m != STEP_PV_ROWS), step_pv_send_mult: m }
        evalAccepted(CFG_SOUND, s) == (m == STEP_PV_ROWS)
      )
      wrap_checked and ukey_halves_ok
        and not(bad_hlo.ok) and not(bad_running.ok) and not(bad_deps.ok)
        and bus_mult_ok
    )
}
