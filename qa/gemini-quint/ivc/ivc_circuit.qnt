// Circuit-level Quint model of SherlockIvcRelation (crates/mori-recursion/src/ivc.rs)
// around commit 1edd79146 ("Fix the IVC node genesis accumulator mismatch (SHKIVC05)").
//
// Models:
// 1. Field arithmetic modulo a prime P (with height bound HMAX < P and step bound S_MAX),
//    boolean/selector gadgets (is_zero, not, select, assert_equal, assert_not_equal_to_fixed),
//    and optional SRC-004 range checks (assert_lower_than_fixed).
// 2. Accumulator structure at the gadget level:
//    - variable-base MSM terms (bases + scalars) on lhs and rhs,
//    - unresolved self fixed-base scalar map (SELF_NAME),
//    - KZG pairing defect `defect` in Z_P (0 iff the accumulator pairs validly under the SRS),
//    - `scale_by_bit(bit, acc)` (scales variable scalars, fixed scalars, and defect by `bit`),
//    - `collapse(acc)` (collapses variable-base MSM terms to 1 canonical base and scalar 1;
//      when all variable scalars are 0, the collapsed base is the identity point 0),
//    - `accumulate([a0, a1, a2])` (derives Fiat-Shamir challenge r from the FULL structural
//      encoding of [a0, a1, a2] including variable-base count and bases, then folds with powers
//      of r).
// 3. Reference semantics modeled separately from the circuit constraint system:
//    - `refValidChain`: contiguous valid leaf transitions starting at h=0, init_d, with
//      1 <= steps <= S_MAX and no modular wrap-around.
// 4. Three circuit modes selectable by constants:
//    - Mode 0 (PRE_SHKIVC05): before 1edd79146 (proof_acc not collapsed before accumulate).
//    - Mode 1 (SHKIVC05_ACTUAL): commit 1edd79146 as in crates/mori-recursion/src/ivc.rs today.
//    - Mode 2 (HARDENED_TASK23): with SRC-004 range checks + SRC-005 scaling/collapse of prev_acc.

module ivc_circuit {
  const MODE: int    // 0 = PRE_SHKIVC05, 1 = SHKIVC05_ACTUAL, 2 = HARDENED_TASK23
  const S_MAX: int   // max machine steps per leaf (SRC-014)

  // Small prime field modulus P = 97 and height bound HMAX = 32 (so 2 * HMAX < P, mirroring
  // 2^64 < r in BLS12-381 while keeping wrap-around reachable in bounded traces).
  pure val P: int = 97
  pure val HMAX: int = 32
  pure val ID_BASE: int = 0
  pure val SELF_VK_PIN: int = 7

  // Field operations in F_P
  pure def fmod(x: int): int = ((x % P) + P) % P
  pure def fadd(a: int, b: int): int = fmod(a + b)
  pure def fsub(a: int, b: int): int = fmod(a - b)
  pure def fmul(a: int, b: int): int = fmod(a * b)

  // In-circuit boolean / native gadget primitives (ivc.rs:316-321)
  pure def isZeroGadget(x: int): int = if (fmod(x) == 0) 1 else 0
  pure def notGadget(b: int): int = 1 - b
  pure def isBit(b: int): bool = (b == 0 or b == 1)
  pure def selectGadget(b: int, onTrue: int, onFalse: int): int =
    if (b == 1) onTrue else onFalse

  // Accumulator representation at the gadget level (ivc.rs:171-181, 327-379)
  // - `numBases`: number of variable bases in the MSM (1 when collapsed, >1 right after prepare)
  // - `baseTag`: abstract encoding of the variable base(s) (ID_BASE = 0 for C::identity/default)
  // - `varScalar`: variable-base scalar (1 when collapsed, 0 after scale_by_bit(0) before collapse)
  // - `selfScalar`: unresolved SELF_NAME fixed-base scalar (0 in trivial_acc)
  // - `defect`: KZG pairing error term in F_P (0 iff e(lhs, [tau]_2) == e(rhs, [1]_2))
  type Acc = {
    numBases: int,
    baseTag: int,
    varScalar: int,
    selfScalar: int,
    defect: int,
  }

  // Canonical trivial accumulator: `SherlockIvcRelation::trivial_acc` (ivc.rs:171-181)
  // One default (identity) base, scalar 1, and every self fixed-base scalar 0.
  pure val trivialAcc: Acc = {
    numBases: 1,
    baseTag: ID_BASE,
    varScalar: 1,
    selfScalar: 0,
    defect: 0,
  }

  // `AssignedAccumulator::scale_by_bit` (ivc.rs:353)
  // Multiplies variable scalars and fixed-base scalars by `bit`, leaving `numBases` and `baseTag`
  // untouched! Before collapse, a zero-scaled accumulator still carries its parsed `numBases` and
  // `baseTag`.
  pure def scaleByBit(bit: int, a: Acc): Acc = {
    numBases: a.numBases,
    baseTag: a.baseTag,
    varScalar: fmul(bit, a.varScalar),
    selfScalar: fmul(bit, a.selfScalar),
    defect: fmul(bit, a.defect),
  }

  // `AssignedAccumulator::collapse` / `Accumulator::collapse` (ivc.rs:227, 361, 378)
  // Evaluates the variable-base MSM: `newBase = varScalar * baseTag`, sets `numBases = 1` and
  // `varScalar = 1`, and leaves `selfScalar` (unresolved fixed bases) and `defect` unchanged.
  // Crucially, when `a` was scaled by bit = 0, `varScalar == 0` and `selfScalar == 0`, so
  // `collapse(scaleByBit(0, a))` produces `{ numBases: 1, baseTag: 0, varScalar: 1, selfScalar: 0, defect: 0 }`
  // which equals `trivialAcc` for ANY `a`!
  pure def collapseAcc(a: Acc): Acc = {
    numBases: 1,
    baseTag: fmul(a.varScalar, a.baseTag),
    varScalar: 1,
    selfScalar: a.selfScalar,
    defect: a.defect,
  }

  // Poseidon transcript challenge inside `Accumulator::accumulate` (ivc.rs:239, 377).
  // Hashes the FULL encoding of all input accumulators (including `numBases`, `baseTag`,
  // `varScalar`, `selfScalar`). Always non-zero in {1..P-1}.
  pure def foldChallenge(a0: Acc, a1: Acc, a2: Acc): int = {
    val raw = fmod(
      3 * a0.numBases + 5 * a0.baseTag + 7 * a0.varScalar + 11 * a0.selfScalar +
      13 * a1.numBases + 17 * a1.baseTag + 19 * a1.varScalar + 23 * a1.selfScalar +
      29 * a2.numBases + 31 * a2.baseTag + 37 * a2.varScalar + 41 * a2.selfScalar + 1
    )
    if (raw == 0) 1 else raw
  }

  // `Accumulator::accumulate(&[a0, a1, a2])` (ivc.rs:239, 377)
  // Folds three accumulators with powers (1, r, r^2) of the challenge r.
  pure def accumulate3(a0: Acc, a1: Acc, a2: Acc): Acc = {
    val r = foldChallenge(a0, a1, a2)
    val r2 = fmul(r, r)
    val b0 = fmul(a0.varScalar, a0.baseTag)
    val b1 = fmul(a1.varScalar, a1.baseTag)
    val b2 = fmul(a2.varScalar, a2.baseTag)
    {
      numBases: a0.numBases + a1.numBases + a2.numBases,
      baseTag: fadd(b0, fadd(fmul(r, b1), fmul(r2, b2))),
      varScalar: 1,
      selfScalar: fadd(a0.selfScalar, fadd(fmul(r, a1.selfScalar), fmul(r2, a2.selfScalar))),
      defect: fadd(a0.defect, fadd(fmul(r, a1.defect), fmul(r2, a2.defect))),
    }
  }

  // Public statement of a node proof: `[self_repr, prog_d, init_d, cur_d, h, acc]` (ivc.rs:184-189)
  type NodeInstance = {
    selfRepr: int,
    progD: int,
    initD: int,
    curD: int,
    h: int,
    acc: Acc,
  }

  // Witness of a node step: `IvcWitness` (ivc.rs:113-126) paired with abstract proof validity
  type NodeWitness = {
    selfRepr: int,
    progD: int,
    initD: int,
    prevD: int,
    prevH: int,
    curD: int,
    rule: int,
    leafBinding: int,
    leafSteps: int,
    // Previous proof's uncollapsed accumulator produced by `verifier.prepare` (ivc.rs:346-352)
    rawProofAcc: Acc,
    // Witness-supplied carried accumulator `w.prev_acc` (ivc.rs:327-336)
    prevAcc: Acc,
    // Leaf proof's uncollapsed accumulator produced by `verify_child_incircuit_with` (ivc.rs:365-374)
    rawLeafAcc: Acc,
    // True machine trace height (integer, not mod P) for reference comparison
    truePrevH: int,
    trueSteps: int,
    // Whether the predecessor statement was genuinely certified from genesis in the reference trace
    prevChainValid: bool,
    // Whether the leaf transition (prevD -> curD over leafSteps) is valid on the reference machine
    leafTransitionValid: bool,
  }

  // ---- Off-circuit `SherlockIvcRelation::next_acc` (ivc.rs:205-242) ----
  pure def nextAccOffCircuit(w: NodeWitness): Acc = {
    val genesis = (fmod(w.prevH) == 0)
    val proofAcc =
      if (genesis) trivialAcc
      else if (MODE == 0) w.rawProofAcc
      else collapseAcc(w.rawProofAcc)
    val prevAccUsed =
      if (MODE == 2 and genesis) trivialAcc
      else w.prevAcc
    val acc = accumulate3(proofAcc, prevAccUsed, w.rawLeafAcc)
    collapseAcc(acc)
  }

  // ---- In-circuit `SherlockIvcRelation::circuit` (ivc.rs:286-380) ----
  // Evaluates the exact constraints of `ivc.rs` and returns whether the circuit is satisfied
  // for `(inst, w)`.
  pure def circuitSatisfied(inst: NodeInstance, w: NodeWitness): bool = {
    val prevH = fmod(w.prevH)
    val steps = fmod(w.leafSteps)
    // ivc.rs:316 `std.assert_not_equal_to_fixed(l, &steps, F::ZERO)`
    val stepsNonZero = (steps != 0)
    // ivc.rs:317 `let h = std.add(l, &prev_h, &steps)`
    val h = fadd(prevH, steps)
    // ivc.rs:318-319 `is_genesis = std.is_zero(l, &prev_h); not_genesis = std.not(l, &is_genesis)`
    val isGenesis = isZeroGadget(prevH)
    val notGenesis = notGadget(isGenesis)
    // ivc.rs:320-321 `chosen = std.select(l, &is_genesis, &init_d, &prev_d); assert_equal(chosen, prev_d)`
    val chosen = selectGadget(isGenesis, w.initD, w.prevD)
    val genesisLinkOk = (chosen == w.prevD)

    // Mode 2 adds Task 2.3 / SRC-004 range checks (`prev_h < HMAX`, `1 <= steps <= S_MAX`, `h < HMAX`).
    // Retargeted 2026-11-09 (council review 2.14, item 19): the checks apply to the raw witness
    // values `w.prevH`, `w.leafSteps` and their raw sum, before the mod-P reduction. A range
    // check on the reduced value cannot refuse attack A1 (`prevH = P` reduces to 0 and passes
    // `0 <= prevH < HMAX`); in the circuit the lane is the previous node's range-checked `h`, so
    // its raw value is below HMAX. The reduced `h` lane is still checked.
    val rangeChecksOk =
      if (MODE == 2) and {
        0 <= w.prevH, w.prevH < HMAX,
        1 <= w.leafSteps, w.leafSteps <= S_MAX,
        w.prevH + w.leafSteps < HMAX,
        0 <= h, h < HMAX,
      } else true

    // ivc.rs:353 `AssignedAccumulator::scale_by_bit(l, scalar, &not_genesis, &mut proof_acc)`
    val scaledProofAcc = scaleByBit(notGenesis, w.rawProofAcc)
    // ivc.rs:361 `proof_acc.collapse(l, bls, scalar)` (added in commit 1edd79146, MODE >= 1)
    val inCircuitProofAcc =
      if (MODE == 0) scaledProofAcc
      else collapseAcc(scaledProofAcc)

    // ivc.rs:327-336, 377: in MODE 0 and MODE 1 (1edd79146), `prev_acc` is NOT scaled by `not_genesis`
    // and NOT constrained to `trivialAcc` at genesis! In MODE 2 (Task 2.3 SRC-005 fix), `prev_acc`
    // is scaled by `not_genesis` and collapsed (or constrained equal to `trivialAcc` at genesis).
    val inCircuitPrevAcc =
      if (MODE == 2) collapseAcc(scaleByBit(notGenesis, w.prevAcc))
      else w.prevAcc

    // ivc.rs:377-378 `let mut nxt = verifier.accumulate(l, &[proof_acc, prev_acc, leaf_acc]); nxt.collapse(...)`
    val nxt = collapseAcc(accumulate3(inCircuitProofAcc, inCircuitPrevAcc, w.rawLeafAcc))

    // ivc.rs:298-304, 322-324, 379 public input copy constraints
    val publicCopyOk = and {
      inst.selfRepr == w.selfRepr,
      inst.progD == w.progD,
      inst.initD == w.initD,
      inst.curD == w.curD,
      inst.h == h,
      inst.acc == nxt,
    }

    and {
      isBit(isGenesis),
      isBit(notGenesis),
      stepsNonZero,
      genesisLinkOk,
      rangeChecksOk,
      publicCopyOk,
    }
  }

  // ---- Decider verification: `RunDeciderRelation::circuit` (run_decider.rs:201-289) ----
  // Pins `selfRepr == SELF_VK_PIN`, folds the last node proof's own accumulator with its carried
  // accumulator `inst.acc`, and checks the KZG pairing (`defect == 0`).
  pure def deciderAccepts(inst: NodeInstance, finalProofDefect: int): bool = and {
    inst.selfRepr == SELF_VK_PIN,
    finalProofDefect == 0,
    inst.acc.defect == 0,
  }

  // ---- Reference Semantics (modeled independently of the circuit) ----
  // A witness and resulting instance are valid according to the reference machine semantics iff:
  // 1. The leaf proves between 1 and S_MAX valid machine steps (`leafTransitionValid`).
  // 2. Either this is true genesis (`truePrevH == 0` and `prevD == initD`), or the predecessor
  //    chain was genuinely certified (`truePrevH > 0`, `prevChainValid`, `rawProofAcc.defect == 0`,
  //    and `prevAcc.defect == 0`).
  // 3. The published height `inst.h` equals the true un-wrapped integer step count
  //    `truePrevH + trueSteps` below `HMAX`.
  pure def referenceAccepts(inst: NodeInstance, w: NodeWitness): bool = and {
    w.leafTransitionValid,
    w.rawLeafAcc.defect == 0,
    1 <= w.trueSteps,
    w.trueSteps <= S_MAX,
    0 <= w.truePrevH,
    w.truePrevH + w.trueSteps < HMAX,
    inst.h == w.truePrevH + w.trueSteps,
    if (w.truePrevH == 0)
      w.prevD == w.initD
    else and {
      w.prevChainValid,
      w.rawProofAcc.defect == 0,
      w.prevAcc.defect == 0,
    }
  }

  // Honest witness constructor for genesis (`truePrevH == 0`) and non-genesis (`truePrevH > 0`)
  pure def mkHonestWitness(prevH: int, steps: int, dummyProofBases: int, dummyProofTag: int): NodeWitness = {
    val isGen = (prevH == 0)
    val initD = 10
    val prevD = if (isGen) initD else 10 + prevH
    val curD = 10 + prevH + steps
    // At genesis, `verifier.prepare` on an empty proof parses `dummyProofBases >= 2` bases with
    // non-zero `dummyProofTag`; at non-genesis, it produces a valid uncollapsed accumulator.
    val rawProof: Acc = {
      numBases: dummyProofBases,
      baseTag: dummyProofTag,
      varScalar: 1,
      selfScalar: if (isGen) 5 else 3,
      defect: if (isGen) 9 else 0,
    }
    val carried: Acc =
      if (isGen) trivialAcc
      else { numBases: 1, baseTag: 4, varScalar: 1, selfScalar: 2, defect: 0 }
    val leafAcc: Acc = { numBases: 2, baseTag: 6, varScalar: 1, selfScalar: 0, defect: 0 }
    {
      selfRepr: SELF_VK_PIN,
      progD: 1,
      initD: initD,
      prevD: prevD,
      prevH: prevH,
      curD: curD,
      rule: 42,
      leafBinding: 99,
      leafSteps: steps,
      rawProofAcc: rawProof,
      prevAcc: carried,
      rawLeafAcc: leafAcc,
      truePrevH: prevH,
      trueSteps: steps,
      prevChainValid: not(isGen),
      leafTransitionValid: true,
    }
  }

  pure def mkInstanceFromOffCircuit(w: NodeWitness): NodeInstance = {
    selfRepr: w.selfRepr,
    progD: w.progD,
    initD: w.initD,
    curD: w.curD,
    h: fadd(fmod(w.prevH), fmod(w.leafSteps)),
    acc: nextAccOffCircuit(w),
  }

  // ---- State Machine for `quint run` / `quint verify` exploration ----
  type TraceState = {
    lastWitness: NodeWitness,
    lastInst: NodeInstance,
    lastKind: str,
    circuitOk: bool,
    deciderOk: bool,
    refOk: bool,
  }

  var st: TraceState

  action recordAttempt(w: NodeWitness, kind: str): bool = {
    val inst = mkInstanceFromOffCircuit(w)
    val cOk = circuitSatisfied(inst, w)
    val dOk = cOk and deciderAccepts(inst, 0)
    val rOk = referenceAccepts(inst, w)
    st' = {
      lastWitness: w,
      lastInst: inst,
      lastKind: kind,
      circuitOk: cOk,
      deciderOk: dOk,
      refOk: rOk,
    }
  }

  action init = {
    val w = mkHonestWitness(0, 1, 3, 12)
    recordAttempt(w, "init_honest_genesis")
  }

  // 1. Honest genesis step (with multi-base dummy `rawProofAcc` from `verifier.prepare`)
  action stepHonestGenesis = {
    nondet steps = 1.to(S_MAX).oneOf()
    nondet bases = Set(2, 3, 4).oneOf()
    nondet tag = Set(5, 12, 20).oneOf()
    recordAttempt(mkHonestWitness(0, steps, bases, tag), "honest_genesis")
  }

  // 2. Honest non-genesis step (`1 <= prevH < HMAX - S_MAX`)
  action stepHonestNonGenesis = {
    nondet prevH = 1.to(10).oneOf()
    nondet steps = 1.to(S_MAX).oneOf()
    nondet bases = Set(2, 3).oneOf()
    nondet tag = Set(7, 15).oneOf()
    recordAttempt(mkHonestWitness(prevH, steps, bases, tag), "honest_non_genesis")
  }

  // 3. Adversarial Attack A1 (ivc.rs:311, 315-318): Non-genesis step after modular wrap-around
  //    where a prior step had `truePrevH > 0` that wrapped `prevH` to `P` (== 0 mod P).
  //    This triggers `is_genesis == 1` (`not_genesis == 0`) at a non-genesis step, so
  //    `scale_by_bit` + `collapse` drops the invalid/forged `rawProofAcc` (`defect != 0`,
  //    `prevChainValid == false`) by replacing it with `trivialAcc`!
  action attackWrapAroundDropsPrevAcc = {
    val base = mkHonestWitness(0, 1, 3, 12)
    val w = {
      ...base,
      prevH: P,                // P == 0 mod P, so `is_zero(prev_h)` returns 1!
      truePrevH: P,            // Non-genesis in the true run (97 steps elapsed)
      prevChainValid: false,   // Predecessor proof is forged / invalid!
      rawProofAcc: { ...base.rawProofAcc, defect: 44 },
    }
    recordAttempt(w, "attack_wrap_around_drops_prev_acc")
  }

  // 4. Adversarial Attack A2 (ivc.rs:315-317): Step-count bound / height wrap-around forgery
  //    (`steps > S_MAX` or `steps = P - 1` decrementing `h`).
  action attackStepBoundOrWrap = {
    nondet badSteps = Set(S_MAX + 1, P - 1).oneOf()
    val base = mkHonestWitness(2, 1, 2, 7)
    val w = {
      ...base,
      leafSteps: badSteps,
      trueSteps: badSteps,
    }
    recordAttempt(w, "attack_step_bound_or_wrap")
  }

  // 5. Adversarial Attack A3 (ivc.rs:327-336, 377): Unscaled `prev_acc` at genesis cancels
  //    an invalid leaf accumulator's KZG defect (`rawLeafAcc.defect != 0`)!
  //    At genesis, `proof_acc` collapses to `trivialAcc` (`defect = 0`), while `prev_acc` is
  //    unauthenticated and folded with weight `r` and `rawLeafAcc` with weight `r^2`.
  //    Choosing `prevAcc.defect = fsub(0, fmul(r, leafDefect))` makes
  //    `r * prevAcc.defect + r^2 * leafDefect == 0 (mod P)`, so `nxt.defect == 0`!
  action attackUnscaledGenesisCarryCancelsBadLeaf = {
    val base = mkHonestWitness(0, 1, 3, 12)
    val badLeaf: Acc = { ...base.rawLeafAcc, defect: 19 }
    val r = foldChallenge(trivialAcc, base.prevAcc, badLeaf)
    // Choose `prevAcc.defect == -r * badLeaf.defect (mod P)` without changing `prevAcc`'s
    // public point/scalar encoding that enters `foldChallenge`:
    val cancellingPrevAcc: Acc = { ...base.prevAcc, defect: fsub(0, fmul(r, badLeaf.defect)) }
    val w = {
      ...base,
      prevAcc: cancellingPrevAcc,
      rawLeafAcc: badLeaf,
      leafTransitionValid: false,
    }
    recordAttempt(w, "attack_unscaled_genesis_carry_cancels_bad_leaf")
  }

  // 6. SRC-005 Realization (b) Completeness Probe (ivc.rs:327-336, 377; specs/sherlock-recursion/spec.md:41-43):
  //    "WHEN two genesis node proofs differ only in a well-formed ignored carry. THEN both pass the run decider."
  //    Supplies an honest genesis witness whose `prevAcc` is a well-formed non-neutral carry (`defect != 0`
  //    because its random well-formed curve points are not a valid KZG opening pair).
  action probeNonNeutralIgnoredGenesisCarry = {
    val base = mkHonestWitness(0, 1, 3, 12)
    val nonNeutralCarry: Acc = { numBases: 1, baseTag: 8, varScalar: 1, selfScalar: 4, defect: 33 }
    val w = {
      ...base,
      prevAcc: nonNeutralCarry,
    }
    recordAttempt(w, "probe_non_neutral_ignored_genesis_carry")
  }

  action step = any {
    stepHonestGenesis,
    stepHonestNonGenesis,
    attackWrapAroundDropsPrevAcc,
    attackStepBoundOrWrap,
    attackUnscaledGenesisCarryCancelsBadLeaf,
    probeNonNeutralIgnoredGenesisCarry,
  }

  // ---- Invariants ----
  // 1. Completeness on honest witnesses: every honest genesis or non-genesis witness satisfies
  //    the circuit and passes the decider. (Fails in MODE 0 / PRE_SHKIVC05; holds in MODE 1 & 2.)
  val completenessHonestInv: bool =
    (st.lastKind == "init_honest_genesis" or st.lastKind == "honest_genesis" or st.lastKind == "honest_non_genesis")
      implies st.deciderOk

  // 2. Soundness: no witness rejected by the reference semantics can pass the circuit + decider.
  //    (Fails in MODE 1 / SHKIVC05_ACTUAL on attacks A1, A2, A3; holds in MODE 2 / HARDENED_TASK23.)
  val soundnessInv: bool =
    st.deciderOk implies st.refOk

  // 3. No non-genesis step can drop a predecessor proof accumulator via `is_genesis == 1`.
  //    (Fails in MODE 1 when `truePrevH == P` wraps to `0 mod P`; holds in MODE 2.)
  val noNonGenesisDropInv: bool =
    (st.deciderOk and st.lastWitness.truePrevH > 0) implies st.lastWitness.prevChainValid

  // 4. SRC-005 Realization (b) ignored-carry completeness: a genesis step with a well-formed
  //    non-neutral carry `prev_acc` still passes the decider because the carry is scaled/collapsed.
  //    (Fails in MODE 1 because `prev_acc` is unscaled; holds in MODE 2.)
  val genesisCarryIrrelevantInv: bool =
    (st.lastKind == "probe_non_neutral_ignored_genesis_carry") implies st.deciderOk

  // 5. All properties combined (holds in MODE 2 / HARDENED_TASK23).
  val allHardenedInvariants: bool = and {
    completenessHonestInv,
    soundnessInv,
    noNonGenesisDropInv,
    genesisCarryIrrelevantInv,
  }
}

module ivc_pre_shkivc05 { import ivc_circuit(MODE = 0, S_MAX = 4).* }
module ivc_shkivc05_actual { import ivc_circuit(MODE = 1, S_MAX = 4).* }
module ivc_hardened_task23 { import ivc_circuit(MODE = 2, S_MAX = 4).* }
