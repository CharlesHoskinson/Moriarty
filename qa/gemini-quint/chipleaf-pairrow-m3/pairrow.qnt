// Soundness Review Model — Part 1: Layout B (Pair Rows with Copy Links and Replicas)
// Design under test:
//   - research/sherlock-processor-analogy-20261110/experiments-2.md section 2 (Layout B "pair rows")
//   - crates/mori-chip/src/rf.rs and crates/mori-chip/src/rfcore.rs (Arch::Narrow)
//   - openspec/changes/sherlock-vm-consensus-20261107/design.md section 2.7
//
// Models:
//   - 8 registers (0..7) grouped across 3 pair rows (P0, P1, P2) in a 7-row slot:
//       Row 0: F_canon (canonical rule flags f0..f3, step count cnt)
//       Row 1: P0      (pair row 0: registers CTL_TAG=0, CTL_A=1, CTL_B=2)
//       Row 2: L_canon (canonical request lanes: aluOut, memVal, memLink)
//       Row 3: P1      (pair row 1: registers TOP0=3, FP=4, LEN=5)
//       Row 4: F_rep   (replica of rule flags f0..f3 + routed operand cells ctlA, ctlB, top0)
//       Row 5: P2      (pair row 2: registers R0=6, V0=7)
//       Row 6: L_rep   (replica of request lanes: aluOut, memVal, memLink)
//   - Digest covers registers 0..6 (NDIG = 7); register V0 = 7 is a scratch register outside the digest.
//   - S step slots i = 0..S-1 plus slot S holding the leaf's final pre state and total step count.
//   - Copy constraints: post[i][j] == pre[i+1][j], fRep == fCanon, lRep == lCanon, routed == source pre.
//   - Frame gate active on every slot i < S (enabled or not), reading only adjacent rows (rotations -1, 0, +1).
//   - Two chained leaves (leaf2.preD == leaf1.postD).
//   - Nondeterministic single-cell tamper prover + required mutants P1..P9 and enable_not_prefix.

module pairrow {
  const S: int
  const MUTANT: str

  // ---------------------------------------------------------------- Registers & Rules
  pure val NREG: int = 8
  pure val NDIG: int = 7
  pure val NRULE: int = 4

  // Pair row P0 (row 1): reads F_canon (row 0) and L_canon (row 2)
  pure val R_TAG: int = 0
  pure val R_A: int = 1
  pure val R_B: int = 2

  // Pair row P1 (row 3): reads L_canon (row 2) and F_rep + routed operands (row 4)
  pure val R_TOP: int = 3
  pure val R_FP: int = 4
  pure val R_LEN: int = 5

  // Pair row P2 (row 5): reads F_rep + routed operands (row 4) and L_rep (row 6)
  pure val R_R0: int = 6
  pure val R_V0: int = 7 // scratch register outside the digest (j = 7 >= NDIG)

  // 4 rules (each names a distinct subset of registers)
  pure val RULE_PUSH: int = 0
  pure val RULE_REWRITE: int = 1
  pure val RULE_ARITH: int = 2
  pure val RULE_POP: int = 3

  // Control tags (tag = rule + 1)
  pure val T_PUSH: int = 1
  pure val T_REWRITE: int = 2
  pure val T_ARITH: int = 3
  pure val T_POP: int = 4

  pure val REG_INDICES: List[int] = [0, 1, 2, 3, 4, 5, 6, 7]
  pure val RULE_INDICES: List[int] = [0, 1, 2, 3]

  // ---------------------------------------------------------------- Types
  type LaneRow = {
    aluOut: int,
    memVal: int,
    memLink: int
  }

  type RoutedRow = {
    ctlA: int,
    ctlB: int,
    top0: int
  }

  type SlotW = {
    pre: List[int],
    post: List[int],
    fCanon: List[int],
    fRep: List[int],
    lCanon: LaneRow,
    lRep: LaneRow,
    routed: RoutedRow,
    cnt: int
  }

  type LeafW = {
    slots: List[SlotW],
    finalPre: List[int],
    finalCnt: int,
    preD: List[int],
    postD: List[int],
    steps: int
  }

  type ChainW = {
    progId: int,
    leaf1: LeafW,
    leaf2: LeafW
  }

  type RefState = {
    regs: List[int],
    rules: List[int],
    steps: int
  }

  type BuildLeafAcc = {
    slots: List[SlotW],
    curPre: List[int],
    curCnt: int
  }

  // ---------------------------------------------------------------- Pure Helpers
  pure val ZERO_LANES: LaneRow = { aluOut: 0, memVal: 0, memLink: 0 }
  pure val ZERO_FLAGS: List[int] = [0, 0, 0, 0]

  pure def flagSum(fl: List[int]): int =
    fl.foldl(0, (acc, b) => acc + b)

  pure def oneHotFlags(r: int): List[int] = {
    val emptyFlags: List[int] = []
    RULE_INDICES.foldl(emptyFlags, (acc, idx) => acc.append(if (idx == r) 1 else 0))
  }

  // Initial register states for two test programs
  // Program 0: 3 enabled steps in Leaf 1 + 3 enabled steps in Leaf 2 (full leaves)
  // Program 1: 2 enabled steps + 1 disabled slot in Leaf 1 + 3 enabled steps in Leaf 2
  pure def initRegs(progId: int): List[int] =
    if (progId == 0)
      // [TAG=1(PUSH), A=1, B=2, TOP=0, FP=0, LEN=1, R0=0, V0=0]
      [T_PUSH, 1, 2, 0, 0, 1, 0, 0]
    else
      // [TAG=1(PUSH), A=2, B=1, TOP=0, FP=0, LEN=1, R0=0, V0=0]
      [T_PUSH, 2, 1, 0, 0, 1, 0, 0]

  pure def leafTargetSteps(progId: int, leafIdx: int): int =
    if (progId == 0) S
    else if (leafIdx == 0) S - 1
    else S

  // Canonical bus lane values for an enabled step from `pre` under rule `r`
  pure def honestLanes(pre: List[int], r: int): LaneRow =
    if (r == RULE_PUSH)
      { aluOut: 1, memVal: pre[R_TOP], memLink: pre[R_FP] }
    else if (r == RULE_REWRITE)
      ZERO_LANES
    else if (r == RULE_ARITH)
      { aluOut: (pre[R_A] + pre[R_B]) % 8, memVal: 0, memLink: 0 }
    else if (r == RULE_POP)
      { aluOut: 1, memVal: pre[R_B], memLink: if (pre[R_FP] > 0) pre[R_FP] - 1 else 0 }
    else
      ZERO_LANES

  pure def honestRouted(pre: List[int]): RoutedRow = {
    ctlA: pre[R_A],
    ctlB: pre[R_B],
    top0: pre[R_TOP]
  }

  // ---------------------------------------------------------------- Reference Register Machine
  pure def refRuleOfTag(tag: int): int =
    tag - 1

  // Lead repair (2026-11-10): the reference takes the design variant `mt`. For `scratch_read` the
  // reference rule set also reads V0 (as the circuit rule does), so an honest run refines and only a
  // leaf-boundary forgery of V0 is caught; before, the honest run already diverged.
  pure def refStepOne(mt: str, pre: List[int], r: int): List[int] = {
    val ln = honestLanes(pre, r)
    val refScratch = if (mt == "scratch_read") pre[R_V0] else 0
    val nextTag =
      if (r == RULE_PUSH) T_REWRITE
      else if (r == RULE_REWRITE) T_ARITH
      else if (r == RULE_ARITH) T_POP
      else T_PUSH
    val nextA = if (r == RULE_ARITH) ln.aluOut else pre[R_A]
    val nextB = pre[R_B]
    val nextTop =
      if (r == RULE_PUSH) pre[R_A]
      else if (r == RULE_REWRITE) pre[R_TOP] + pre[R_B]
      else if (r == RULE_POP) ln.memVal
      else pre[R_TOP]
    val nextFp =
      if (r == RULE_PUSH) pre[R_LEN]
      else if (r == RULE_POP) ln.memLink
      else pre[R_FP]
    val nextLen =
      if (r == RULE_PUSH) pre[R_LEN] + 1
      else pre[R_LEN]
    val nextR0 =
      if (r == RULE_ARITH) pre[R_R0] + ln.aluOut + refScratch
      else if (r == RULE_POP) pre[R_R0] + pre[R_TOP] + refScratch
      else pre[R_R0]
    val nextV0 = ln.aluOut
    [nextTag, nextA, nextB, nextTop, nextFp, nextLen, nextR0, nextV0]
  }

  pure def refRunByRules(mt: str, initR: List[int], ruleSeq: List[int]): RefState = {
    val emptyRules: List[int] = []
    ruleSeq.foldl({ regs: initR, rules: emptyRules, steps: 0 }, (st, r) => {
      val expectedR = refRuleOfTag(st.regs[R_TAG])
      val actualR = if (r == expectedR) r else expectedR
      {
        regs: refStepOne(mt, st.regs, actualR),
        rules: st.rules.append(actualR),
        steps: st.steps + 1
      }
    })
  }

  pure def refRunSteps(mt: str, initR: List[int], nSteps: int): RefState = {
    val emptyRules: List[int] = []
    range(0, 2 * S).foldl({ regs: initR, rules: emptyRules, steps: 0 }, (st, idx) => {
      if (idx < nSteps) {
        val r = refRuleOfTag(st.regs[R_TAG])
        {
          regs: refStepOne(mt, st.regs, r),
          rules: st.rules.append(r),
          steps: st.steps + 1
        }
      } else st
    })
  }

  // ---------------------------------------------------------------- Pair-Row Gate Semantics
  // Gate on pair row Pk reads ONLY the F/L rows and routed cells adjacent to Pk (rotations -1, 0, +1):
  //   - P0 (j in 0..2): reads fCanon (row 0) and lCanon (row 2)
  //   - P1 (j in 3..5): reads fRep (row 4), lCanon (row 2), and routed (row 4)
  //   - P2 (j in 6..7): reads fRep (row 4), lRep (row 6), and routed (row 4)
  pure def nextStateCell(mt: str, sl: SlotW, j: int): int = {
    val p = sl.pre[j]
    // Select the flag vector visible to j's pair row
    val fl = if (j <= R_B) sl.fCanon else sl.fRep
    val en = flagSum(fl)
    val off = 1 - en
    val fPush = fl[RULE_PUSH]
    val fRew = fl[RULE_REWRITE]
    val fArith = fl[RULE_ARITH]
    val fPop = fl[RULE_POP]

    if (j == R_TAG)
      off * p + fPush * T_REWRITE + fRew * T_ARITH + fArith * T_POP + fPop * T_PUSH
    else if (j == R_A)
      (1 - fArith) * p + fArith * sl.lCanon.aluOut
    else if (j == R_B)
      p
    else if (j == R_TOP)
      (1 - fPush - fRew - fPop) * p
        + fPush * sl.routed.ctlA
        + fRew * (p + sl.routed.ctlB)
        + fPop * sl.lCanon.memVal
    else if (j == R_FP)
      (1 - fPush - fPop) * p
        + fPush * sl.pre[R_LEN]
        + fPop * sl.lCanon.memLink
    else if (j == R_LEN)
      p + fPush
    else if (j == R_R0) {
      // P9 scratch_read: rule on P2 reads the undigested scratch register pre[R_V0]
      val scratchVal = if (mt == "scratch_read") sl.pre[R_V0] else 0
      (1 - fArith - fPop) * p
        + fArith * (p + sl.lRep.aluOut + scratchVal)
        + fPop * (p + sl.routed.top0 + scratchVal)
    } else
      // R_V0 (scratch register on P2, outside the digest)
      off * p + en * sl.lRep.aluOut
  }

  // State digest: injective projection over covered words 0..NDIG-1 (excludes V0 = 7).
  // Mutant P8 (digest_omits_register) leaves out R_B (which RULE_REWRITE and RULE_ARITH read).
  pure def digestOf(mt: str, regs: List[int]): List[int] =
    if (mt == "digest_omits_register")
      [regs[R_TAG], regs[R_A], regs[R_TOP], regs[R_FP], regs[R_LEN], regs[R_R0]]
    else
      [regs[R_TAG], regs[R_A], regs[R_B], regs[R_TOP], regs[R_FP], regs[R_LEN], regs[R_R0]]

  // Extract enabled rule id from canonical flags
  pure def enabledRuleOfFlags(fl: List[int]): int =
    if (fl[RULE_PUSH] == 1) RULE_PUSH
    else if (fl[RULE_REWRITE] == 1) RULE_REWRITE
    else if (fl[RULE_ARITH] == 1) RULE_ARITH
    else if (fl[RULE_POP] == 1) RULE_POP
    else -1

  pure def leafExecutedRules(lw: LeafW): List[int] = {
    val emptyRules: List[int] = []
    lw.slots.foldl(emptyRules, (acc, sl) => {
      if (flagSum(sl.fCanon) == 1) acc.append(enabledRuleOfFlags(sl.fCanon))
      else acc
    })
  }

  pure def chainExecutedRules(w: ChainW): List[int] =
    leafExecutedRules(w.leaf1).concat(leafExecutedRules(w.leaf2))

  // ---------------------------------------------------------------- Circuit Acceptance Predicate
  // Frame gate on slot `sl`: enforces post[j] == nextStateCell(mt, sl, j) for every register j.
  // - P1 `frame_gate_dropped`: frame gate for R_B is missing.
  // - P2 `frame_gate_enabled_only`: frame gate is multiplied by `en`, so it is off when en == 0.
  pure def slotFrameGatesOk(mt: str, sl: SlotW): bool = {
    val en = flagSum(sl.fCanon)
    if (mt == "frame_gate_enabled_only" and en == 0)
      true
    else
      REG_INDICES.indices().forall(j => {
        if (mt == "frame_gate_dropped" and j == R_B) true
        else sl.post[j] == nextStateCell(mt, sl, j)
      })
  }

  // Canonical F row, DEC lookup, and canonical L row bus check on slot `sl`
  pure def slotCanonGatesOk(sl: SlotW): bool = {
    val en = flagSum(sl.fCanon)
    and {
      sl.pre.length() == NREG,
      sl.post.length() == NREG,
      sl.fCanon.length() == NRULE,
      sl.fRep.length() == NRULE,
      sl.fCanon.indices().forall(t => sl.fCanon[t] == 0 or sl.fCanon[t] == 1),
      en == 0 or en == 1,
      if (en == 1) {
        val r = enabledRuleOfFlags(sl.fCanon)
        and {
          r >= 0,
          sl.pre[R_TAG] == r + 1,
          sl.lCanon == honestLanes(sl.pre, r)
        }
      } else {
        sl.lCanon == ZERO_LANES
      }
    }
  }

  // Intra-slot copy constraints (permutation argument between canonical cells and replicas/routed cells)
  // - P4 `flag_replica_unlinked`: F-row replica fRep is not copied to fCanon.
  // - P5 `lane_replica_unlinked`: L-row replica lRep is not copied to lCanon.
  // - P6 `routed_operand_unlinked`: routed operand ctlA is not copied to pre[R_A].
  pure def slotCopiesOk(mt: str, sl: SlotW): bool = and {
    (mt == "flag_replica_unlinked") or (sl.fRep == sl.fCanon),
    (mt == "lane_replica_unlinked") or (sl.lRep == sl.lCanon),
    (mt == "routed_operand_unlinked") or (sl.routed.ctlA == sl.pre[R_A]),
    sl.routed.ctlB == sl.pre[R_B],
    sl.routed.top0 == sl.pre[R_TOP]
  }

  // Inter-slot chain copy constraints: post[i][j] == pre[i+1][j] for all i < S and all j
  // - P3 `chain_link_dropped`: post[0][R_TOP] -> pre[1][R_TOP] copy constraint is missing.
  pure def leafChainCopiesOk(mt: str, lw: LeafW): bool =
    0.to(S - 1).forall(i => {
      val nextPre = if (i + 1 < S) lw.slots[i + 1].pre else lw.finalPre
      REG_INDICES.indices().forall(j => {
        if (mt == "chain_link_dropped" and i == 0 and j == R_TOP) true
        else lw.slots[i].post[j] == nextPre[j]
      })
    })

  // Enable prefix and step count constraints
  pure def leafPrefixAndCountOk(mt: str, lw: LeafW): bool = and {
    lw.slots[0].cnt == 0,
    0.to(S - 1).forall(i => {
      val en = flagSum(lw.slots[i].fCanon)
      val nextCnt = if (i + 1 < S) lw.slots[i + 1].cnt else lw.finalCnt
      nextCnt == lw.slots[i].cnt + en
    }),
    lw.steps == lw.finalCnt,
    (mt == "enable_not_prefix") or and {
      flagSum(lw.slots[0].fCanon) == 1,
      0.to(S - 2).forall(i => {
        val enCur = flagSum(lw.slots[i].fCanon)
        val enNext = flagSum(lw.slots[i + 1].fCanon)
        enNext * (1 - enCur) == 0
      })
    }
  }

  // Leaf statement digest constraints:
  // - P7 `post_d_wrong_slot`: postD digests slot S-1's pre cells instead of slot S's pre cells (finalPre).
  pure def leafDigestOk(mt: str, lw: LeafW): bool = {
    val postSlotPre = if (mt == "post_d_wrong_slot") lw.slots[S - 1].pre else lw.finalPre
    and {
      lw.preD == digestOf(mt, lw.slots[0].pre),
      lw.postD == digestOf(mt, postSlotPre)
    }
  }

  pure def leafAccepted(mt: str, lw: LeafW): bool = and {
    lw.slots.length() == S,
    lw.finalPre.length() == NREG,
    0.to(S - 1).forall(i => slotCanonGatesOk(lw.slots[i])),
    0.to(S - 1).forall(i => slotFrameGatesOk(mt, lw.slots[i])),
    0.to(S - 1).forall(i => slotCopiesOk(mt, lw.slots[i])),
    leafChainCopiesOk(mt, lw),
    leafPrefixAndCountOk(mt, lw),
    leafDigestOk(mt, lw)
  }

  // Two chained leaves acceptance
  pure def acceptsWithMutant(mt: str, w: ChainW): bool = and {
    w.leaf1.slots[0].pre == initRegs(w.progId),
    leafAccepted(mt, w.leaf1),
    leafAccepted(mt, w.leaf2),
    w.leaf2.preD == w.leaf1.postD
  }

  pure def accepts(w: ChainW): bool =
    acceptsWithMutant(MUTANT, w)

  // ---------------------------------------------------------------- Main Refinement Invariant
  // For every accepted witness of the two chained leaves, the circuit's post state
  // (all digest-covered registers 0..NDIG-1) and the sequence of rules executed equal
  // those of the reference machine run on the same rule sequence from the same pre state,
  // and total steps equals the number of reference steps.
  pure def refines(w: ChainW): bool = refinesWith(MUTANT, w)

  pure def refinesWith(mt: str, w: ChainW): bool = {
    val initR = initRegs(w.progId)
    val totalSteps = w.leaf1.steps + w.leaf2.steps
    val refBySteps = refRunSteps(mt, initR, totalSteps)
    val cirRules = chainExecutedRules(w)
    val refByRules = refRunByRules(mt, initR, cirRules)
    and {
      digestOf("none", w.leaf2.finalPre) == digestOf("none", refBySteps.regs),
      digestOf("none", w.leaf2.finalPre) == digestOf("none", refByRules.regs),
      // Lead repair: post_d in the statement format of the variant (P8 has a shorter digest).
      w.leaf2.postD == digestOf(mt, refBySteps.regs),
      cirRules == refBySteps.rules,
      totalSteps == refBySteps.steps,
      totalSteps == cirRules.length()
    }
  }

  // ---------------------------------------------------------------- Honest Witness Builder & Nondeterministic Tamper Prover
  pure def recomputeSlotPost(mt: str, sl: SlotW): SlotW = {
    val emptyRegs: List[int] = []
    val postRegs = REG_INDICES.foldl(emptyRegs, (acc, j) => acc.append(nextStateCell(mt, sl, j)))
    { ...sl, post: postRegs }
  }

  pure def buildHonestSlot(mt: str, pre: List[int], cnt: int, slotOn: bool): SlotW =
    if (not(slotOn)) {
      {
        pre: pre,
        post: pre,
        fCanon: ZERO_FLAGS,
        fRep: ZERO_FLAGS,
        lCanon: ZERO_LANES,
        lRep: ZERO_LANES,
        routed: honestRouted(pre),
        cnt: cnt
      }
    } else {
      val r = refRuleOfTag(pre[R_TAG])
      val fl = oneHotFlags(r)
      val ln = honestLanes(pre, r)
      val rt = honestRouted(pre)
      val tempSl = {
        pre: pre,
        post: pre,
        fCanon: fl,
        fRep: fl,
        lCanon: ln,
        lRep: ln,
        routed: rt,
        cnt: cnt
      }
      recomputeSlotPost(mt, tempSl)
    }

  pure def buildHonestLeaf(mt: str, pre0: List[int], nEnabled: int): LeafW = {
    val emptySlots: List[SlotW] = []
    val initAcc: BuildLeafAcc = { slots: emptySlots, curPre: pre0, curCnt: 0 }
    val folded = range(0, S).foldl(initAcc, (acc, i) => {
      val en = i < nEnabled
      val sl = buildHonestSlot(mt, acc.curPre, acc.curCnt, en)
      {
        slots: acc.slots.append(sl),
        curPre: sl.post,
        curCnt: acc.curCnt + (if (en) 1 else 0)
      }
    })
    val postSlotPre = if (mt == "post_d_wrong_slot") folded.slots[S - 1].pre else folded.curPre
    {
      slots: folded.slots,
      finalPre: folded.curPre,
      finalCnt: folded.curCnt,
      preD: digestOf(mt, folded.slots[0].pre),
      postD: digestOf(mt, postSlotPre),
      steps: folded.curCnt
    }
  }

  pure def buildHonestChain(mt: str, progId: int): ChainW = {
    val pre0 = initRegs(progId)
    val leaf1 = buildHonestLeaf(mt, pre0, leafTargetSteps(progId, 0))
    val pre2 = if (mt == "post_d_wrong_slot") leaf1.slots[S - 1].pre else leaf1.finalPre
    val leaf2 = buildHonestLeaf(mt, pre2, leafTargetSteps(progId, 1))
    { progId: progId, leaf1: leaf1, leaf2: leaf2 }
  }

  // Propagate changes forward from `startSlot` within a leaf (recomputing downstream slots honestly
  // except keeping any tampered cell at `startSlot` if `keepStartPost` is true)
  pure def cascadeLeafFrom(mt: str, lw: LeafW, startSlot: int, keepStartPost: bool): LeafW = {
    val emptySlots: List[SlotW] = []
    val initAcc: BuildLeafAcc = { slots: emptySlots, curPre: lw.slots[0].pre, curCnt: 0 }
    val folded = range(0, S).foldl(initAcc, (acc, i) => {
      val orig = lw.slots[i]
      if (i < startSlot) {
        val en = flagSum(orig.fCanon)
        {
          slots: acc.slots.append(orig),
          curPre: orig.post,
          curCnt: orig.cnt + en
        }
      } else if (i == startSlot) {
        val en = flagSum(orig.fCanon)
        val slUpdated = if (keepStartPost) { ...orig, cnt: acc.curCnt }
                        else recomputeSlotPost(mt, { ...orig, cnt: acc.curCnt })
        {
          slots: acc.slots.append(slUpdated),
          curPre: slUpdated.post,
          curCnt: acc.curCnt + en
        }
      } else {
        val en = flagSum(orig.fCanon) == 1
        val sl = buildHonestSlot(mt, acc.curPre, acc.curCnt, en)
        {
          slots: acc.slots.append(sl),
          curPre: sl.post,
          curCnt: acc.curCnt + (if (en) 1 else 0)
        }
      }
    })
    val postSlotPre = if (mt == "post_d_wrong_slot") folded.slots[S - 1].pre else folded.curPre
    {
      slots: folded.slots,
      finalPre: folded.curPre,
      finalCnt: folded.curCnt,
      preD: digestOf(mt, folded.slots[0].pre),
      postD: digestOf(mt, postSlotPre),
      steps: folded.curCnt
    }
  }

  // Tamper kinds for the nondeterministic prover & exhaustive pure checks:
  //   0: Honest (no tamper, or P7 wrong post_d slot)
  //   1: Tamper a slot's `post[regIdx]` to `newVal` (with or without downstream cascade) -> tests P1, P2
  //   2: Tamper a slot's `pre[regIdx]` to `newVal` and cascade downstream -> tests P3, P8, P9
  //   3: Tamper a slot's F-row replica `fRep` (set to ZERO_FLAGS) and cascade -> tests P4
  //   4: Tamper a slot's L-row replica `lRep.aluOut` to `newVal` and cascade -> tests P5
  //   5: Tamper a slot's routed operand `routed.ctlA` to `newVal` and cascade -> tests P6
  //   6: Swap enabled/disabled slots into a non-prefix pattern `[1, 0, 1]` -> tests enable_not_prefix
  pure def tamperLeaf(
    mt: str,
    lw: LeafW,
    tKind: int,
    slotIdx: int,
    regIdx: int,
    newVal: int,
    cascade: bool
  ): LeafW = {
    val sl = lw.slots[slotIdx]
    if (tKind == 1) {
      val slTampered = { ...sl, post: sl.post.replaceAt(regIdx, newVal) }
      val lw1 = { ...lw, slots: lw.slots.replaceAt(slotIdx, slTampered) }
      if (cascade) cascadeLeafFrom(mt, lw1, slotIdx, true) else lw1
    } else if (tKind == 2) {
      val en = flagSum(sl.fCanon) == 1
      val newPre = sl.pre.replaceAt(regIdx, newVal)
      val slTampered = buildHonestSlot(mt, newPre, sl.cnt, en)
      val lw1 = { ...lw, slots: lw.slots.replaceAt(slotIdx, slTampered) }
      if (cascade) cascadeLeafFrom(mt, lw1, slotIdx, true) else lw1
    } else if (tKind == 3) {
      val slTampered = recomputeSlotPost(mt, { ...sl, fRep: ZERO_FLAGS })
      val lw1 = { ...lw, slots: lw.slots.replaceAt(slotIdx, slTampered) }
      if (cascade) cascadeLeafFrom(mt, lw1, slotIdx, true) else lw1
    } else if (tKind == 4) {
      val slTampered = recomputeSlotPost(mt, { ...sl, lRep: { ...sl.lRep, aluOut: newVal } })
      val lw1 = { ...lw, slots: lw.slots.replaceAt(slotIdx, slTampered) }
      if (cascade) cascadeLeafFrom(mt, lw1, slotIdx, true) else lw1
    } else if (tKind == 5) {
      val slTampered = recomputeSlotPost(mt, { ...sl, routed: { ...sl.routed, ctlA: newVal } })
      val lw1 = { ...lw, slots: lw.slots.replaceAt(slotIdx, slTampered) }
      if (cascade) cascadeLeafFrom(mt, lw1, slotIdx, true) else lw1
    } else if (tKind == 6) {
      val s0 = buildHonestSlot(mt, lw.slots[0].pre, 0, true)
      val s1 = buildHonestSlot(mt, s0.post, 1, false)
      val s2 = buildHonestSlot(mt, s1.post, 1, true)
      val newSlots = [s0, s1, s2]
      {
        slots: newSlots,
        finalPre: s2.post,
        finalCnt: 2,
        preD: digestOf(mt, s0.pre),
        postD: digestOf(mt, s2.post),
        steps: 2
      }
    } else {
      lw
    }
  }

  pure def applyTamper(
    mt: str,
    progId: int,
    tKind: int,
    leafIdx: int,
    slotIdx: int,
    regIdx: int,
    newVal: int,
    cascade: bool
  ): ChainW = {
    val baseChain = buildHonestChain(mt, progId)
    if (tKind == 0)
      baseChain
    else if (leafIdx == 0) {
      val l1 = tamperLeaf(mt, baseChain.leaf1, tKind, slotIdx, regIdx, newVal, cascade)
      val pre2 = if (mt == "post_d_wrong_slot") l1.slots[S - 1].pre else l1.finalPre
      val l2 = if (cascade) buildHonestLeaf(mt, pre2, leafTargetSteps(progId, 1)) else baseChain.leaf2
      { progId: progId, leaf1: l1, leaf2: l2 }
    } else {
      val l2 = tamperLeaf(mt, baseChain.leaf2, tKind, slotIdx, regIdx, newVal, cascade)
      { progId: progId, leaf1: baseChain.leaf1, leaf2: l2 }
    }
  }

  // ---------------------------------------------------------------- Named Honest & Counterexample Witnesses
  // Honest full two-leaf witness (3 + 3 steps)
  pure val hw_full_two_leaf: ChainW = buildHonestChain("none", 0)

  // Honest partial two-leaf witness with a disabled slot in Leaf 1 (2 + 3 steps)
  pure val hw_disabled_slot_two_leaf: ChainW = buildHonestChain("none", 1)

  // P1 `frame_gate_dropped`: R_B frame gate dropped; slot 0 of Leaf 1 changes post[R_B] from 2 to 3
  pure val cex_P1_frame_gate_dropped: ChainW =
    applyTamper("frame_gate_dropped", 0, 1, 0, 0, R_B, 3, true)

  // P2 `frame_gate_enabled_only`: frame gate off in disabled slot 2 of Leaf 1 (prog 1); changes post[2][R_A] to 3
  pure val cex_P2_frame_gate_enabled_only: ChainW =
    applyTamper("frame_gate_enabled_only", 1, 1, 0, 2, R_A, 3, true)

  // P3 `chain_link_dropped`: copy post[0][R_TOP] -> pre[1][R_TOP] dropped; changes pre[1][R_TOP] to 3
  pure val cex_P3_chain_link_dropped: ChainW =
    applyTamper("chain_link_dropped", 0, 2, 0, 1, R_TOP, 3, true)

  // P4 `flag_replica_unlinked`: F-row replica fRep on slot 0 unlinked (set to all-0 while fCanon is PUSH)
  pure val cex_P4_flag_replica_unlinked: ChainW =
    applyTamper("flag_replica_unlinked", 0, 3, 0, 0, 0, 0, true)

  // P5 `lane_replica_unlinked`: L-row replica lRep on slot 2 (ARITH) unlinked; lRep.aluOut set to 7 instead of 3
  pure val cex_P5_lane_replica_unlinked: ChainW =
    applyTamper("lane_replica_unlinked", 0, 4, 0, 2, R_R0, 7, true)

  // P6 `routed_operand_unlinked`: routed operand ctlA on slot 0 (PUSH) set to 3 instead of pre[R_A] = 1
  pure val cex_P6_routed_operand_unlinked: ChainW =
    applyTamper("routed_operand_unlinked", 0, 5, 0, 0, R_TOP, 3, true)

  // P7 `post_d_wrong_slot`: post_d digests slot S-1's pre cells instead of slot S's pre cells
  pure val cex_P7_post_d_wrong_slot: ChainW =
    buildHonestChain("post_d_wrong_slot", 0)

  // P8 `digest_omits_register`: R_B omitted from digest; Leaf 2 slot 0 sets pre[R_B] = 3 instead of 2
  pure val cex_P8_digest_omits_register: ChainW =
    applyTamper("digest_omits_register", 0, 2, 1, 0, R_B, 3, true)

  // P9 `scratch_read`: rule reads undigested scratch register V0 at Leaf 2 slot 0 (POP); sets pre[V0] = 7
  pure val cex_P9_scratch_read: ChainW =
    applyTamper("scratch_read", 0, 2, 1, 0, R_V0, 7, true)

  // Non-prefix enable witness: slots enabled [1, 0, 1] in Leaf 1
  pure val w_enable_not_prefix: ChainW =
    applyTamper("enable_not_prefix", 1, 6, 0, 0, 0, 0, true)

  // ---------------------------------------------------------------- State Machine (Nondeterministic Prover)
  pure val PROG_IDS: Set[int] = Set(0, 1)
  pure val TAMPER_KINDS: Set[int] = Set(0, 1, 2, 3, 4, 5, 6)
  pure val LEAF_IDXS: Set[int] = Set(0, 1)
  pure val SLOT_IDXS: Set[int] = Set(0, 1, 2)
  pure val REG_DOM: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7)
  pure val VAL_DOM: Set[int] = Set(0, 1, 2, 3, 7)

  var w: ChainW
  var acc: bool
  var isTampered: bool

  action sample = {
    nondet progId = PROG_IDS.oneOf()
    nondet tKind = TAMPER_KINDS.oneOf()
    nondet leafIdx = LEAF_IDXS.oneOf()
    nondet slotIdx = SLOT_IDXS.oneOf()
    nondet regIdx = REG_DOM.oneOf()
    nondet newVal = VAL_DOM.oneOf()
    nondet cascade = Set(false, true).oneOf()
    val cand = applyTamper(MUTANT, progId, tKind, leafIdx, slotIdx, regIdx, newVal, cascade)
    val honest = buildHonestChain("none", progId)
    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = (cand != honest)
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- Invariants
  // Main soundness invariant: accepted(w) implies refines(w)
  val invRefine: bool =
    acc implies refines(w)

  // Non-vacuity reachability invariants (negated: MUST be violated with exit 1 on pairrowSound)
  // 1. An honest two-leaf run with a disabled slot is accepted
  val reachHonestDisabledSlot: bool =
    not(acc and not(isTampered) and w.progId == 1 and w.leaf1.steps == S - 1 and w.leaf2.steps == S)

  // 2. An honest full two-leaf run is accepted
  val reachHonestFullChain: bool =
    not(acc and not(isTampered) and w.progId == 0 and w.leaf1.steps == S and w.leaf2.steps == S)

  // 3. A tampered witness that violates refinement is accepted (violated ONLY in mutants P1..P9, holds in pairrowSound)
  val reachAcceptedBadTamper: bool =
    not(acc and not(refines(w)))
}

// Sound base module
module pairrowSound { import pairrow(S = 3, MUTANT = "none").* }

// Required mutants P1..P9 (each MUST violate invRefine)
module pairrow_P1_frame_gate_dropped { import pairrow(S = 3, MUTANT = "frame_gate_dropped").* }
module pairrow_P2_frame_gate_enabled_only { import pairrow(S = 3, MUTANT = "frame_gate_enabled_only").* }
module pairrow_P3_chain_link_dropped { import pairrow(S = 3, MUTANT = "chain_link_dropped").* }
module pairrow_P4_flag_replica_unlinked { import pairrow(S = 3, MUTANT = "flag_replica_unlinked").* }
module pairrow_P5_lane_replica_unlinked { import pairrow(S = 3, MUTANT = "lane_replica_unlinked").* }
module pairrow_P6_routed_operand_unlinked { import pairrow(S = 3, MUTANT = "routed_operand_unlinked").* }
module pairrow_P7_post_d_wrong_slot { import pairrow(S = 3, MUTANT = "post_d_wrong_slot").* }
module pairrow_P8_digest_omits_register { import pairrow(S = 3, MUTANT = "digest_omits_register").* }
module pairrow_P9_scratch_read { import pairrow(S = 3, MUTANT = "scratch_read").* }

// Enable prefix dropped (NOT caught by register refinement alone; see REPORT.md)
module pairrow_enable_not_prefix { import pairrow(S = 3, MUTANT = "enable_not_prefix").* }
