// Quint specification of the Sherlock self-recursive IVC node SHKIVC06
// (`crates/mori-recursion/src/ivc.rs`, `tests/ivc_node_mock.rs`, `tests/ivc_cost.rs`)
// and its Lean reference semantics (`clean/fork/Clean/Sherlock/Aggregation.lean`:
// `AccAlgebra.nodeFold`, `genesis_collapse_both`, `non_genesis_keeps_both`,
// `src004_height_bridge`, `linNode`, `lin_refines_chain`).
//
// Avoids Quint built-in names `fields`, `next`, and `fold`.

module ivc06 {
  // ---- Relation and Field Constants (ivc.rs:64-80) ----
  // Prime modulus P = 257 and height bound HMAX = 64 satisfy 2 * HMAX <= P
  // (mirroring 2^65 <= r in `src004_height_bridge`, Aggregation.lean:1201)
  // while keeping S_MAX = 16 (`ivc.rs:77`) and wrap-around reachable in bounded runs.
  pure val P: int = 257
  pure val HMAX: int = 64
  pure val S_MAX: int = 16
  pure val ID_PT: int = 0
  pure val SELF_VK_PIN: int = 7
  // Modular inverse of 2 mod 257: 2 * 129 = 258 == 1 (mod 257).
  // Used for the mid-chain non-collapsed carry re-encoding (B/2, scalar 2)
  // mirroring `let half = two.invert().unwrap()` in `tests/ivc_node_mock.rs:424`.
  pure val HALF_MOD_P: int = 129

  // ---- Field Arithmetic in F_P ----
  pure def fmod(x: int): int = ((x % P) + P) % P
  pure def fadd(a: int, b: int): int = fmod(a + b)
  pure def fsub(a: int, b: int): int = fmod(a - b)
  pure def fmul(a: int, b: int): int = fmod(a * b)

  // ---- In-Circuit Boolean, Selection, and Range-Check Gadgets (ivc.rs:363-421, 497-501) ----
  pure def isBit(b: int): bool = (b == 0 or b == 1)
  pure def isZeroGadget(x: int): int = if (fmod(x) == 0) 1 else 0
  pure def notGadget(b: int): int = 1 - b
  pure def selectGadget(b: int, onTrue: int, onFalse: int): int =
    if (b == 1) onTrue else onFalse

  // `std.assert_lower_than_fixed(l, &x, &bound)` (`ivc.rs:370, 372, 374`; `Aggregation.lean:1511`).
  // On a canonical field element in {0..P-1} (or a raw witness value assigned to a wire),
  // accepts iff 0 <= x < bound.
  pure def assertLowerThanFixed(x: int, bound: int): bool =
    (0 <= x and x < bound)

  // ---- Accumulator Representation (ivc.rs:200-261, 507-564; Aggregation.lean:1230-1253) ----
  // Models `Accumulator<S>` / `AssignedAccumulator<S>` with:
  // - `numBases`: variable-base count per side (1 for collapsed or assigned carried accumulators,
  //               >= 1 for uncollapsed accumulators from `prepare`)
  // - `lhsBase`, `lhsScalar`: lhs variable-base point tag and scalar
  // - `rhsBase`, `rhsScalar`: rhs variable-base point tag and scalar
  // - `selfScalar`: unresolved `SELF_NAME` ("shk_node") fixed-base scalar on rhs
  // - `defect`: KZG pairing defect in F_P (0 iff `acc.check(params, fixed_bases)` holds)
  type Acc = {
    numBases: int,
    lhsBase: int,
    lhsScalar: int,
    rhsBase: int,
    rhsScalar: int,
    selfScalar: int,
    defect: int,
  }

  // `SherlockIvcRelation::trivial_acc` (`ivc.rs:200-210`; `alg.trivial` in `Aggregation.lean:1231`):
  // One default (identity) base, scalar one per side, and every self fixed-base scalar zero.
  pure val trivialAcc: Acc = {
    numBases: 1,
    lhsBase: ID_PT,
    lhsScalar: 1,
    rhsBase: ID_PT,
    rhsScalar: 1,
    selfScalar: 0,
    defect: 0,
  }

  // Public-input encoding of a 1-base-per-side carried accumulator (`ivc.rs:232, 240, 253, 517`):
  // `[lhsBase, lhsScalar, rhsBase, rhsScalar, selfScalar]`.
  // `carry_scalar_lanes` (`ivc.rs:239-248`) are indices 1 (`lhsScalar`) and 3 (`rhsScalar`).
  pure def accPublicLanes(a: Acc): List[int] =
    [a.lhsBase, a.lhsScalar, a.rhsBase, a.rhsScalar, a.selfScalar]

  // `SherlockIvcRelation::is_neutral` (`ivc.rs:231-234`):
  // True iff `a` has the public encoding of `trivial_acc`.
  pure def isNeutral(a: Acc): bool = and {
    a.numBases == 1,
    accPublicLanes(a) == accPublicLanes(trivialAcc),
  }

  // `SherlockIvcRelation::is_collapsed_carry` (`ivc.rs:252-261`):
  // True iff `a` has 1 base per side and both variable-base scalar lanes equal `F::ONE`.
  pure def isCollapsedCarry(a: Acc): bool = and {
    a.numBases == 1,
    a.lhsScalar == 1,
    a.rhsScalar == 1,
  }

  // `AssignedAccumulator::scale_by_bit` (`ivc.rs:538`; `alg.scale` in `Aggregation.lean:1233`):
  // Scales variable-base scalars, fixed-base scalars, and KZG defect by `bit`, while keeping
  // `numBases`, `lhsBase`, and `rhsBase` unchanged until `collapse`.
  pure def scaleByBit(bit: int, a: Acc): Acc = {
    numBases: a.numBases,
    lhsBase: a.lhsBase,
    lhsScalar: fmul(bit, a.lhsScalar),
    rhsBase: a.rhsBase,
    rhsScalar: fmul(bit, a.rhsScalar),
    selfScalar: fmul(bit, a.selfScalar),
    defect: fmul(bit, a.defect),
  }

  // `AssignedAccumulator::collapse` / `Accumulator::collapse` (`ivc.rs:333, 345, 546, 563`;
  // `alg.collapse` in `Aggregation.lean:1235`):
  // Evaluates the variable-base MSM on each side (`scalar * base`) and normalizes to 1 base
  // with variable scalar 1 per side, preserving `selfScalar` and `defect`.
  pure def collapseAcc(a: Acc): Acc = {
    numBases: 1,
    lhsBase: fmul(a.lhsScalar, a.lhsBase),
    lhsScalar: 1,
    rhsBase: fmul(a.rhsScalar, a.rhsBase),
    rhsScalar: 1,
    selfScalar: a.selfScalar,
    defect: a.defect,
  }

  // Poseidon transcript challenge inside `Accumulator::accumulate` (`ivc.rs:344, 562`).
  // Hashes the full structural encoding of all three input accumulators (`numBases`, bases,
  // variable scalars, and fixed scalars). Always in {1..P-1}.
  pure def foldChallenge(a0: Acc, a1: Acc, a2: Acc): int = {
    val raw = fmod(
      3 * a0.numBases + 5 * a0.lhsBase + 7 * a0.lhsScalar + 11 * a0.rhsBase + 13 * a0.rhsScalar + 17 * a0.selfScalar +
      19 * a1.numBases + 23 * a1.lhsBase + 29 * a1.lhsScalar + 31 * a1.rhsBase + 37 * a1.rhsScalar + 41 * a1.selfScalar +
      43 * a2.numBases + 47 * a2.lhsBase + 53 * a2.lhsScalar + 59 * a2.rhsBase + 61 * a2.rhsScalar + 67 * a2.selfScalar + 1
    )
    if (raw == 0) 1 else raw
  }

  // `Accumulator::accumulate(&[a0, a1, a2])` (`ivc.rs:344, 562`; `alg.accumulate` in `Aggregation.lean:1232`).
  pure def accumulate3(a0: Acc, a1: Acc, a2: Acc): Acc = {
    val r = foldChallenge(a0, a1, a2)
    val r2 = fmul(r, r)
    val l0 = fmul(a0.lhsScalar, a0.lhsBase)
    val l1 = fmul(a1.lhsScalar, a1.lhsBase)
    val l2 = fmul(a2.lhsScalar, a2.lhsBase)
    val u0 = fmul(a0.rhsScalar, a0.rhsBase)
    val u1 = fmul(a1.rhsScalar, a1.rhsBase)
    val u2 = fmul(a2.rhsScalar, a2.rhsBase)
    {
      numBases: a0.numBases + a1.numBases + a2.numBases,
      lhsBase: fadd(l0, fadd(fmul(r, l1), fmul(r2, l2))),
      lhsScalar: 1,
      rhsBase: fadd(u0, fadd(fmul(r, u1), fmul(r2, u2))),
      rhsScalar: 1,
      selfScalar: fadd(a0.selfScalar, fadd(fmul(r, a1.selfScalar), fmul(r2, a2.selfScalar))),
      defect: fadd(a0.defect, fadd(fmul(r, a1.defect), fmul(r2, a2.defect))),
    }
  }

  // ---- Reference `AccAlgebra.nodeFold` (Aggregation.lean:1248-1252) vs Circuit Fold (ivc.rs:538-563) ----

  // Lean `AccAlgebra.nodeFold`: scales and collapses BOTH `proofAcc` and `carried` before `accumulate`.
  pure def leanNodeFold(notGenesisBit: int, proofAcc: Acc, carried: Acc, leafAcc: Acc): Acc = {
    val p = collapseAcc(scaleByBit(notGenesisBit, proofAcc))
    val c = collapseAcc(scaleByBit(notGenesisBit, carried))
    collapseAcc(accumulate3(p, c, leafAcc))
  }

  // SHKIVC06 in-circuit fold (`ivc.rs:538, 546, 562-563`):
  // Scales and collapses `proofAcc`, and folds `carried` (`prev_acc`) directly without a second
  // in-circuit variable-base MSM, relying on `constrain_genesis_neutral` + `constrain_collapsed_carry`.
  pure def circuitFold(notGenesisBit: int, proofAcc: Acc, carried: Acc, leafAcc: Acc): Acc = {
    val p = collapseAcc(scaleByBit(notGenesisBit, proofAcc))
    collapseAcc(accumulate3(p, carried, leafAcc))
  }

  // ---- SHKIVC06 Helper Constraints (ivc.rs:363-421) ----

  // `constrain_heights` (`ivc.rs:363-376`):
  // 1. `std.assert_lower_than_fixed(l, prev_h, &2^64)`
  // 2. `steps_m1 = std.add_constant(l, steps, -F::ONE); std.assert_lower_than_fixed(l, &steps_m1, &S_MAX)`
  // 3. `h = std.add(l, prev_h, steps); std.assert_lower_than_fixed(l, &h, &2^64)`
  pure def constrainHeights(rawPrevH: int, rawSteps: int): { ok: bool, hOut: int } = {
    val prevH = fmod(rawPrevH)
    val steps = fmod(rawSteps)
    val stepsM1 = fsub(steps, 1)
    val h = fadd(prevH, steps)
    val ok = and {
      // Raw and field checks on `prev_h < 2^64` (refuses both rawPrevH >= HMAX and negative/wrapped field elements)
      assertLowerThanFixed(rawPrevH, HMAX),
      assertLowerThanFixed(prevH, HMAX),
      // `steps - 1 in [0, S_MAX)` in F_P (refuses steps == 0 since fsub(0,1) = P-1 >= S_MAX, and refuses steps > S_MAX)
      0 <= rawSteps,
      rawSteps < P,
      assertLowerThanFixed(stepsM1, S_MAX),
      // `h = (prev_h + steps) mod P < 2^64`
      assertLowerThanFixed(h, HMAX),
    }
    { ok: ok, hOut: h }
  }

  // `constrain_genesis_neutral` (`ivc.rs:383-400`):
  // `is_genesis * (lane - neutral) == 0` for every public-input lane of `prev_acc`.
  // By `valid_trivial` (`Aggregation.lean:1241`), an accumulator whose points and scalars equal
  // `trivial_acc` is the canonical trivial accumulator (identity points, zero fixed scalars, defect 0).
  pure def constrainGenesisNeutral(isGenesis: int, prevAcc: Acc): bool = {
    val lanes = accPublicLanes(prevAcc)
    val neutral = accPublicLanes(trivialAcc)
    and {
      prevAcc.numBases == 1,
      0.to(4).forall(i => fmul(isGenesis, fsub(lanes[i], neutral[i])) == 0),
    }
  }

  // `constrain_collapsed_carry` (`ivc.rs:409-421`):
  // Asserts `lanes[idx] == F::ONE` for both variable-base scalar lanes `carry_scalar_lanes()`.
  pure def constrainCollapsedCarry(prevAcc: Acc): bool =
    isCollapsedCarry(prevAcc)

  // ---- Node Statement and Witness Types (ivc.rs:119-146, 213-227) ----
  type NodeInstance = {
    selfRepr: int,
    progD: int,
    initD: int,
    curD: int,
    h: int,
    acc: Acc,
  }

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
    rawProofAcc: Acc,
    prevAcc: Acc,
    rawLeafAcc: Acc,
    truePrevH: int,
    trueSteps: int,
    prevChainValid: bool,
    leafTransitionValid: bool,
  }

  // ---- Host Functions: `next_acc_raw` and `next_acc` (ivc.rs:277-347) ----

  // `SherlockIvcRelation::next_acc_raw` (`ivc.rs:306-347`):
  // Computes the folded accumulator without checking `is_neutral` or `is_collapsed_carry`,
  // used by `tests/ivc_node_mock.rs:147-172` (`statement`) to build negative-test instances.
  pure def nextAccRaw(w: NodeWitness): Acc = {
    val genesis = (fmod(w.prevH) == 0)
    val proofAcc = if (genesis) trivialAcc else collapseAcc(w.rawProofAcc)
    // When `prevAcc` has the neutral public encoding (`isNeutral`), its curve points are C::identity()
    // and fixed scalars are 0, so its KZG pairing defect is 0 (`valid_trivial`).
    val effectivePrevAcc = if (isNeutral(w.prevAcc)) trivialAcc else w.prevAcc
    collapseAcc(accumulate3(proofAcc, effectivePrevAcc, w.rawLeafAcc))
  }

  // `SherlockIvcRelation::next_acc` (`ivc.rs:277-300`):
  // Host-side check that refuses a non-neutral genesis carry or a non-collapsed carry.
  pure def nextAccHostOk(w: NodeWitness): bool = and {
    (fmod(w.prevH) == 0) implies isNeutral(w.prevAcc),
    isCollapsedCarry(w.prevAcc),
  }

  pure def mkInstance(w: NodeWitness): NodeInstance = {
    selfRepr: w.selfRepr,
    progD: w.progD,
    initD: w.initD,
    curD: w.curD,
    h: fadd(fmod(w.prevH), fmod(w.leafSteps)),
    acc: nextAccRaw(w),
  }

  // ---- SHKIVC06 In-Circuit Constraint System (`SherlockIvcRelation::circuit`, ivc.rs:456-565) ----
  pure def circuitSatisfied(inst: NodeInstance, w: NodeWitness): bool = {
    val prevH = fmod(w.prevH)
    // 1. Height & step range checks (`constrain_heights`, ivc.rs:497)
    val hc = constrainHeights(w.prevH, w.leafSteps)
    // 2. Genesis bit & state digest link (`ivc.rs:498-501`)
    val isGenesis = isZeroGadget(prevH)
    val notGenesis = notGadget(isGenesis)
    val chosen = selectGadget(isGenesis, w.initD, w.prevD)
    val genesisLinkOk = (chosen == w.prevD)
    // 3. Carried accumulator constraints (`constrain_genesis_neutral` & `constrain_collapsed_carry`, ivc.rs:520-521)
    val genesisNeutralOk = constrainGenesisNeutral(isGenesis, w.prevAcc)
    val collapsedCarryOk = constrainCollapsedCarry(w.prevAcc)
    // When `isNeutral(w.prevAcc)` holds, all points/scalars equal `trivialAcc`, so KZG defect is 0 (`valid_trivial`).
    val effectivePrevAcc = if (isNeutral(w.prevAcc)) trivialAcc else w.prevAcc
    // 4. Fold, collapse, and public input copy constraints (`ivc.rs:502-504, 538-564`)
    val foldedAcc = circuitFold(notGenesis, w.rawProofAcc, effectivePrevAcc, w.rawLeafAcc)
    val publicCopyOk = and {
      inst.selfRepr == w.selfRepr,
      inst.progD == w.progD,
      inst.initD == w.initD,
      inst.curD == w.curD,
      inst.h == hc.hOut,
      inst.acc == foldedAcc,
    }
    and {
      isBit(isGenesis),
      isBit(notGenesis),
      hc.ok,
      genesisLinkOk,
      genesisNeutralOk,
      collapsedCarryOk,
      publicCopyOk,
    }
  }

  // ---- Final Pairing / Decider Check (`check_acc` in ivc.rs:350-356, `RunDeciderRelation` in run_decider.rs:206, 276) ----
  pure def deciderAccepts(inst: NodeInstance, finalProofDefect: int): bool = and {
    inst.selfRepr == SELF_VK_PIN,
    finalProofDefect == 0,
    inst.acc.defect == 0,
  }

  // ---- Lean Reference Semantics (`Aggregation.lean`: `src004_height_bridge`, `nodeFold`, `linNode`, `lin_refines_chain`) ----

  // `src004_height_bridge` (`Aggregation.lean:1201-1206`):
  // `h = prevH + steps /\ prevH + steps < HMAX /\ prevH < h /\ h - prevH <= S_MAX`
  pure def src004HeightBridgeHolds(w: NodeWitness, inst: NodeInstance): bool = and {
    0 <= w.prevH,
    w.prevH < HMAX,
    1 <= w.leafSteps,
    w.leafSteps <= S_MAX,
    inst.h == w.prevH + w.leafSteps,
    w.prevH + w.leafSteps < HMAX,
    w.prevH < inst.h,
    inst.h - w.prevH <= S_MAX,
  }

  // `linNode` (`Aggregation.lean:1381-1385`) and `lin_refines_chain` (`Aggregation.lean:1454-1485`):
  // An accepted linear node step links `(prog, init, prev_d, prev_h)` with leaf `(prog, prev_d, cur_d, prev_h, steps)`
  // to certify the prefix chain `(prog, init, cur_d, 0, prev_h + steps)` with positive height `h > 0`.
  pure def referenceAccepts(inst: NodeInstance, w: NodeWitness): bool = {
    val notGenesisBit = if (w.truePrevH == 0) 0 else 1
    val effectivePrevAcc = if (isNeutral(w.prevAcc)) trivialAcc else w.prevAcc
    and {
      src004HeightBridgeHolds(w, inst),
      w.prevH == w.truePrevH,
      w.leafSteps == w.trueSteps,
      inst.h > 0,
      w.leafTransitionValid,
      w.rawLeafAcc.defect == 0,
      inst.acc == leanNodeFold(notGenesisBit, w.rawProofAcc, effectivePrevAcc, w.rawLeafAcc),
      if (w.truePrevH == 0) and {
        w.prevD == w.initD,
        isNeutral(w.prevAcc),
        // `genesis_collapse_both` (`Aggregation.lean:1260-1263`):
        inst.acc == collapseAcc(accumulate3(trivialAcc, trivialAcc, w.rawLeafAcc)),
      } else and {
        w.prevChainValid,
        w.rawProofAcc.defect == 0,
        w.prevAcc.defect == 0,
        isCollapsedCarry(w.prevAcc),
        // `non_genesis_keeps_both` (`Aggregation.lean:1290-1294`):
        inst.acc == collapseAcc(accumulate3(collapseAcc(w.rawProofAcc), collapseAcc(w.prevAcc), w.rawLeafAcc)),
      }
    }
  }

  // ---- Witness Constructors (Honest & Adversarial) ----

  pure def mkHonestWitness(prevH: int, steps: int, dummyProofBases: int, dummyProofPt: int): NodeWitness = {
    val isGen = (prevH == 0)
    val initD = 10
    val prevD = if (isGen) initD else 10 + prevH
    val curD = 10 + prevH + steps
    val rawProof: Acc = {
      numBases: dummyProofBases,
      lhsBase: dummyProofPt,
      lhsScalar: 1,
      rhsBase: fadd(dummyProofPt, 1),
      rhsScalar: 1,
      selfScalar: if (isGen) 5 else 3,
      defect: if (isGen) 9 else 0,
    }
    val carried: Acc =
      if (isGen) trivialAcc
      else {
        numBases: 1,
        lhsBase: 4,
        lhsScalar: 1,
        rhsBase: 8,
        rhsScalar: 1,
        selfScalar: 2,
        defect: 0,
      }
    val leafAcc: Acc = {
      numBases: 2,
      lhsBase: 6,
      lhsScalar: 1,
      rhsBase: 11,
      rhsScalar: 1,
      selfScalar: 0,
      defect: 0,
    }
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

  // Re-encodes a collapsed carried accumulator `(B, scalar 1)` as `(B * HALF_MOD_P, scalar 2)` on
  // both sides (`tests/ivc_node_mock.rs:420-427`).
  // Because `2 * HALF_MOD_P == 1 (mod P)`, `collapseAcc(reencodeCarryScalar2(a)) == collapseAcc(a)`.
  pure def reencodeCarryScalar2(a: Acc): Acc = {
    ...a,
    lhsBase: fmul(a.lhsBase, HALF_MOD_P),
    lhsScalar: 2,
    rhsBase: fmul(a.rhsBase, HALF_MOD_P),
    rhsScalar: 2,
  }

  // ---- State Machine for Bounded Exploration (`quint run` / `quint verify`) ----
  type TraceState = {
    lastWitness: NodeWitness,
    lastInst: NodeInstance,
    lastKind: str,
    isHonestStep: bool,
    circuitOk: bool,
    hostOk: bool,
    deciderOk: bool,
    refOk: bool,
    foldMatchesNodeFold: bool,
    heightBridgeOk: bool,
  }

  var st: TraceState

  action recordAttempt(w: NodeWitness, kind: str, honest: bool): bool = {
    val inst = mkInstance(w)
    val cOk = circuitSatisfied(inst, w)
    val hOk = nextAccHostOk(w)
    val dOk = cOk and deciderAccepts(inst, 0)
    val rOk = referenceAccepts(inst, w)
    val notGenBit = notGadget(isZeroGadget(fmod(w.prevH)))
    val effectivePrevAcc = if (isNeutral(w.prevAcc)) trivialAcc else w.prevAcc
    val fMatch =
      circuitFold(notGenBit, w.rawProofAcc, effectivePrevAcc, w.rawLeafAcc) ==
        leanNodeFold(notGenBit, w.rawProofAcc, effectivePrevAcc, w.rawLeafAcc)
    val hbOk = src004HeightBridgeHolds(w, inst)
    st' = {
      lastWitness: w,
      lastInst: inst,
      lastKind: kind,
      isHonestStep: honest,
      circuitOk: cOk,
      hostOk: hOk,
      deciderOk: dOk,
      refOk: rOk,
      foldMatchesNodeFold: fMatch,
      heightBridgeOk: hbOk,
    }
  }

  action init = {
    val w = mkHonestWitness(0, 2, 3, 12)
    recordAttempt(w, "init_honest_genesis", true)
  }

  // 1. Honest genesis step (`prevH == 0`, `1 <= steps <= S_MAX = 16`, `tests/ivc_node_mock.rs:230-255`)
  action stepHonestGenesis = {
    nondet steps = Set(1, 2, 8, S_MAX).oneOf()
    nondet bases = Set(2, 3, 4).oneOf()
    nondet pt = Set(5, 12, 20).oneOf()
    recordAttempt(mkHonestWitness(0, steps, bases, pt), "honest_genesis", true)
  }

  // 2. Honest non-genesis step (`1 <= prevH < HMAX - S_MAX`, `1 <= steps <= S_MAX`, `tests/ivc_node_mock.rs:406-418`)
  action stepHonestNonGenesis = {
    nondet prevH = Set(1, 2, 15, HMAX - S_MAX - 1).oneOf()
    nondet steps = Set(1, 3, S_MAX).oneOf()
    nondet bases = Set(2, 3).oneOf()
    nondet pt = Set(7, 15).oneOf()
    recordAttempt(mkHonestWitness(prevH, steps, bases, pt), "honest_non_genesis", true)
  }

  // 3. Adversarial Witness: Height wrap (`tests/ivc_node_mock.rs:445-464`, `Aggregation.lean:1179, 1211`)
  //    - `prevH = P - 2, steps = 2` -> field sum wraps to `h = 0` (`step1 wrap h=0`)
  //    - `prevH = HMAX - 1, steps = 1` -> sum reaches `h = HMAX` (`step1 h=2^64`)
  //    - `prevH = P, steps = 1` -> raw height `P == 0 mod P` at `truePrevH = P` (`CEX-1`)
  action attackHeightWrap = {
    nondet wvariant = Set(0, 1, 2).oneOf()
    val base = mkHonestWitness(2, 2, 2, 7)
    val w =
      if (wvariant == 0) {
        ...base,
        prevH: P - 2,
        truePrevH: P - 2,
        leafSteps: 2,
        trueSteps: 2,
      } else if (wvariant == 1) {
        ...base,
        prevH: HMAX - 1,
        truePrevH: HMAX - 1,
        leafSteps: 1,
        trueSteps: 1,
      } else {
        ...mkHonestWitness(0, 1, 3, 12),
        prevH: P,
        truePrevH: P,
        prevChainValid: false,
        rawProofAcc: { ...base.rawProofAcc, defect: 44 },
      }
    recordAttempt(w, "attack_height_wrap", false)
  }

  // 4. Adversarial Witness: Steps 0 and 17 (`S_MAX + 1`) (`tests/ivc_node_mock.rs:248-260`)
  action attackSteps0And17 = {
    nondet badSteps = Set(0, S_MAX + 1).oneOf()
    nondet prevH = Set(0, 2).oneOf()
    val base = mkHonestWitness(prevH, 1, 2, 7)
    val w = {
      ...base,
      leafSteps: badSteps,
      trueSteps: badSteps,
    }
    recordAttempt(w, "attack_steps_0_or_17", false)
  }

  // 5. Adversarial Witness: Non-neutral carried accumulator at genesis (`tests/ivc_node_mock.rs:262-290`)
  //    Includes both a well-formed collapsed carry with generator bases + fixed scalar 5 and a
  //    defect-cancelling genesis carry (`CEX-3`).
  action attackNonNeutralGenesisCarry = {
    nondet cancelBadLeaf = Set(false, true).oneOf()
    val base = mkHonestWitness(0, 2, 3, 12)
    val nonNeutralCarry: Acc = {
      numBases: 1,
      lhsBase: 1,
      lhsScalar: 1,
      rhsBase: 1,
      rhsScalar: 1,
      selfScalar: 5,
      defect: 33,
    }
    val badLeaf: Acc = { ...base.rawLeafAcc, defect: 19 }
    val r = foldChallenge(trivialAcc, nonNeutralCarry, badLeaf)
    val cancellingCarry: Acc = { ...nonNeutralCarry, defect: fsub(0, fmul(r, badLeaf.defect)) }
    val w =
      if (cancelBadLeaf) {
        ...base,
        prevAcc: cancellingCarry,
        rawLeafAcc: badLeaf,
        leafTransitionValid: false,
      } else {
        ...base,
        prevAcc: nonNeutralCarry,
      }
    recordAttempt(w, "attack_non_neutral_genesis_carry", false)
  }

  // 6. Adversarial Witness: Non-collapsed carried accumulator mid-chain (`tests/ivc_node_mock.rs:420-443`)
  //    Same algebraic accumulator value at `prevH = 2, steps = 3` re-encoded as `(B / 2, scalar 2)`.
  action attackNonCollapsedCarryMidChain = {
    val base = mkHonestWitness(2, 3, 2, 7)
    val w = {
      ...base,
      prevAcc: reencodeCarryScalar2(base.prevAcc),
    }
    recordAttempt(w, "attack_non_collapsed_carry_mid_chain", false)
  }

  // 7. Adversarial Witness: Invalid leaf (`tests/ivc_node_mock.rs:292-328, 358-395`)
  //    Leaf proof prepared for another statement: satisfies the node's deferred verifier in-circuit
  //    (`circuitOk == true`), but embeds a non-zero KZG pairing defect (`rawLeafAcc.defect != 0`)
  //    that the final pairing check / decider refuses (`deciderOk == false`).
  action attackInvalidLeaf = {
    nondet prevH = Set(0, 2).oneOf()
    val base = mkHonestWitness(prevH, 2, 3, 12)
    val w = {
      ...base,
      rawLeafAcc: { ...base.rawLeafAcc, defect: 19 },
      leafTransitionValid: false,
    }
    recordAttempt(w, "attack_invalid_leaf", false)
  }

  // 8. Adversarial Witness: Genesis state-digest mismatch (`prevD != initD` at `prevH = 0`, `ivc.rs:500-501`)
  action attackGenesisDigestMismatch = {
    val base = mkHonestWitness(0, 2, 3, 12)
    val w = {
      ...base,
      prevD: fadd(base.initD, 1),
    }
    recordAttempt(w, "attack_genesis_digest_mismatch", false)
  }

  action step = any {
    stepHonestGenesis,
    stepHonestNonGenesis,
    attackHeightWrap,
    attackSteps0And17,
    attackNonNeutralGenesisCarry,
    attackNonCollapsedCarryMidChain,
    attackInvalidLeaf,
    attackGenesisDigestMismatch,
  }

  // ---- State Invariants ----

  // I1. Soundness: every witness accepted by the circuit and final pairing satisfies the reference semantics.
  val soundnessInv: bool =
    (st.circuitOk and st.deciderOk) implies st.refOk

  // I2. Completeness: every honest step is accepted by the circuit, host `next_acc`, decider, and reference semantics.
  val completenessInv: bool =
    st.isHonestStep implies (st.circuitOk and st.hostOk and st.deciderOk and st.refOk)

  // I3. `nodeFold` Equivalence: on every witness accepted by the SHKIVC06 constraints (`circuitOk`),
  //     the circuit's fold (`circuitFold`) equals Lean's `AccAlgebra.nodeFold` (`leanNodeFold`).
  val nodeFoldEquivalenceInv: bool =
    st.circuitOk implies st.foldMatchesNodeFold

  // I4. Height Bridge (`src004_height_bridge`): on every witness accepted by the SHKIVC06 constraints,
  //     the natural height arithmetic without wrap holds (`1 <= h - prevH <= S_MAX` and `h < HMAX`).
  val heightBridgeInv: bool =
    st.circuitOk implies st.heightBridgeOk

  // I5. Adversarial Refusal: every adversarial witness is refused by the node circuit constraints
  //     or (for a deferred invalid leaf) by the final pairing check.
  val adversarialRefusedInv: bool =
    not(st.isHonestStep) implies not(st.deciderOk)

  val allInvariants: bool = and {
    soundnessInv,
    completenessInv,
    nodeFoldEquivalenceInv,
    heightBridgeInv,
    adversarialRefusedInv,
  }
}
