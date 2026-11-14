// -*- mode: quint; -*-
// Sherlock T10 bound wrap & ledger contract soundness model (QA lane t10-bound).
// Covers:
//   Q1: L_Fr gen_r_check + gen_ops_check (operand limbs, Canon, open/mode bits, A_0 = C_P)
//   Q2: N1 HhiRangeAir & HHI_RANGE_BUS, Path C steps bound, MERGE-WORK (Kind::Work) gap
//   Q3: Root StmtAir log tie on TIE_BUS, OnionAir ENT_BUS/VAL_BUS/LOG_BUS, child (D_EMPTY, 0)
//   Q4: SherlockLedger acceptRun / acceptCollection / openClaim (R1..R4, zero anchor, privacy)

module t10_bound_defs {
  // ---------------------------------------------------------------------------
  // 1. Types
  // ---------------------------------------------------------------------------

  type BbDigest = {
    bTag: int,
    bX: int,
    bY: int,
  }

  type PubDigest = {
    pKind: int,
    pA: int,
    pB: int,
    pC: int,
  }

  type FrLimbs = {
    loLimb: int,
    hiLimb: int,
  }

  type LogEnt = {
    opCode: int,
    opParam: int,
  }

  type OnionOut = {
    onD: BbDigest,
    onN: int,
  }

  type ModelCfg = {
    opsSkipIdx: int,
    openFree: bool,
    anchorNotCp: bool,
    chunkTranscript: bool,
    hhiOneLimb: bool,
    hhiValidFree: bool,
    workHhiUnranged: bool,
    tieMultPlus: bool,
    tieDropped: bool,
    onionSkipVal: bool,
    childLogFreePc1: bool,
    ledgerNoReplay: bool,
    ledgerTwoMaps: bool,
  }

  type WitnessState = {
    isTampered: bool,
    attemptTag: int,
    entryKind: int,

    // Verified execution / seal truth
    runProg: int,
    inputId: int,
    trueOut: int,
    sealOut: int,
    pcBit: int,
    useWorkNode: bool,

    // Q2: N1 (First), Path C, and Kind::Work h_hi witness
    n1V0: int,
    n1V1: int,
    leaf0Hpre: List[int],
    leaf0Steps: int,
    leaf1Hpre: List[int],
    leaf1Steps: int,
    n1Hhi0: List[int],
    n1Cry0: List[int],
    n1Hhi1: List[int],
    n1Cry1: List[int],
    n1Mult0: int,
    n1Mult1: int,
    workSteps: int,
    workWhi: List[int],
    workWcry: List[int],
    rootHlo: List[int],
    rootHhi: List[int],

    // Q1: L_Fr operand limbs, selectors, and anchor preimages
    rOp0Limbs: FrLimbs,
    rOp1Limbs: FrLimbs,
    rOp2Limbs: FrLimbs,
    a4Limbs: FrLimbs,
    a5Limbs: FrLimbs,
    param4Word: int,
    openInvW: int,
    openSel: int,
    modeBit: int,
    chunkJ: int,
    chunkK: int,

    // Q3: Child log, L interface tables, OnionAir & TIE_BUS
    childLogD: BbDigest,
    childLogN: int,
    extraEntOp: int,
    extraEntParam: int,
    tableExtraVal: int,
    valUsedCnt: int,
    unusedValWord: int,
    onionIncExtra: bool,
    onionIncNop: bool,
    onionExtraVal: int,
    exposedLogD: BbDigest,
    exposedLogN: int,

    // Q4: SherlockLedger call arguments, oracle flags, pre/post state
    proofValidBit: bool,
    collectionSoundBit: bool,
    collectionHasClaim: bool,
    leaksPrivate: bool,
    ledgerAnchor: PubDigest,
    ledgerRegRoot: PubDigest,
    callCp: PubDigest,
    callInputPub: PubDigest,
    callOutcomePub: PubDigest,
    callClaimPub: PubDigest,
    callRegPub: PubDigest,
    callSetPub: PubDigest,

    preRuns: Set[PubDigest],
    preOpenRuns: Set[PubDigest],
    preRoots: Set[PubDigest],
    preHistory: List[PubDigest],

    postRuns: Set[PubDigest],
    postOpenRuns: Set[PubDigest],
    postRoots: Set[PubDigest],
    postHistory: List[PubDigest],
  }

  // ---------------------------------------------------------------------------
  // 2. Scaled Constants
  // ---------------------------------------------------------------------------

  // Prime fields: BB_P models BabyBear, FR_P models BLS12-381 Fr
  pure val BB_P: int = 17
  pure val FR_P: int = 37

  // Limb bounds: 2-bit limbs (bound 4) scale 2^16 for h_hi;
  // 2 limbs of 3 bits (bound 8) scale 9 x 30-bit limbs for Fr values (< 37)
  pure val HHI_BOUND: int = 4
  pure val H64_BOUND: int = 256
  pure val OP_LIMB_BOUND: int = 8

  // Programs, inputs, outcomes
  pure val PROG_A: int = 1
  pure val PROG_B: int = 2
  pure val IN_0: int = 0
  pure val IN_1: int = 1
  pure val OUT_OK: int = 10
  pure val OUT_ALT: int = 11

  // Contract entry kinds
  pure val ENTRY_ACCEPT_RUN: int = 1
  pure val ENTRY_ACCEPT_COLL: int = 2
  pure val ENTRY_OPEN_CLAIM: int = 3

  // Attempt tags for non-vacuity reachability checks
  pure val ATTEMPT_HONEST: int = 0
  pure val ATTEMPT_R1: int = 1
  pure val ATTEMPT_R2: int = 2
  pure val ATTEMPT_R3: int = 3
  pure val ATTEMPT_R4: int = 4
  pure val ATTEMPT_ZERO_ANCHOR: int = 5
  pure val ATTEMPT_TAMPER: int = 6

  // Public DIGEST tag indices (sherlock-lfr/iface.rs:61-66)
  pure val TAG_SHK_PROGC: int = 0
  pure val TAG_SHK_LOCK: int = 1
  pure val TAG_SHK_INPUT: int = 2
  pure val TAG_SHK_OUTCOME: int = 3
  pure val TAG_SHK_REG: int = 4
  pure val TAG_SHK_SET: int = 5

  // Log op codes (sherlock-lfr/iface.rs:50-58)
  pure val OP_NOP: int = 0
  pure val OP_SPONGE: int = 3
  pure val OP_PUB: int = 7
  pure val OP_DIGEST: int = 8

  // BabyBear internal sponge tags (ARCHITECTURE.md FP-4)
  pure val TAG_BB_EMPTY: int = 1
  pure val TAG_BB_STEP: int = 2
  pure val TAG_BB_ENT: int = 4

  pure val D_EMPTY: BbDigest = { bTag: TAG_BB_EMPTY, bX: 0, bY: 0 }
  pure val ZERO_PUB: PubDigest = { pKind: 0, pA: 0, pB: 0, pC: 0 }

  // Configurations: sound + 12 required mutants + 3 diagnostic mutants
  pure val CFG_SOUND: ModelCfg = {
    opsSkipIdx: -1,
    openFree: false,
    anchorNotCp: false,
    chunkTranscript: false,
    hhiOneLimb: false,
    hhiValidFree: false,
    workHhiUnranged: false,
    tieMultPlus: false,
    tieDropped: false,
    onionSkipVal: false,
    childLogFreePc1: false,
    ledgerNoReplay: false,
    ledgerTwoMaps: false,
  }

  pure val CFG_M_OPS_SKIP_0: ModelCfg = { ...CFG_SOUND, opsSkipIdx: 0 }
  pure val CFG_M_OPS_SKIP_1: ModelCfg = { ...CFG_SOUND, opsSkipIdx: 1 }
  pure val CFG_M_OPS_SKIP_2: ModelCfg = { ...CFG_SOUND, opsSkipIdx: 2 }
  pure val CFG_M_OPEN_FREE: ModelCfg = { ...CFG_SOUND, openFree: true }
  pure val CFG_M_ANCHOR_NOT_CP: ModelCfg = { ...CFG_SOUND, anchorNotCp: true }
  pure val CFG_M_HHI_ONE_LIMB: ModelCfg = { ...CFG_SOUND, hhiOneLimb: true }
  pure val CFG_M_HHI_VALID_FREE: ModelCfg = { ...CFG_SOUND, hhiValidFree: true }
  pure val CFG_M_TIE_MULT_PLUS: ModelCfg = { ...CFG_SOUND, tieMultPlus: true }
  pure val CFG_M_ONION_SKIP_VAL: ModelCfg = { ...CFG_SOUND, onionSkipVal: true }
  pure val CFG_M_CHILD_LOG_FREE_PC1: ModelCfg = { ...CFG_SOUND, childLogFreePc1: true }
  pure val CFG_M_LEDGER_NO_REPLAY: ModelCfg = { ...CFG_SOUND, ledgerNoReplay: true }
  pure val CFG_M_LEDGER_TWO_MAPS: ModelCfg = { ...CFG_SOUND, ledgerTwoMaps: true }
  pure val CFG_M_TIE_DROPPED: ModelCfg = { ...CFG_SOUND, tieDropped: true }
  pure val CFG_M_WORK_HHI_UNRANGED: ModelCfg = { ...CFG_SOUND, workHhiUnranged: true }
  pure val CFG_M_CHUNK_J1_ANCHOR: ModelCfg = { ...CFG_SOUND, chunkTranscript: true }

  // ---------------------------------------------------------------------------
  // 3. Pure Arithmetic, Limb Packing, and Injective Digest Helpers
  // ---------------------------------------------------------------------------

  pure def modPos(x: int, modulus: int): int =
    ((x % modulus) + modulus) % modulus

  pure def isBit(x: int): bool =
    or { x == 0, x == 1 }

  // Cantor pairing on non-negative integers: strictly injective N x N -> N
  pure def pairNat(x: int, y: int): int =
    // LEAD REPAIR: reduced mod a 30-bit prime so nested pairing stays inside 64-bit integers.
    (((x + y) * (x + y + 1)) / 2 + y) % 1000000007

  pure def encBb(d: BbDigest): int =
    pairNat(d.bTag, pairNat(d.bX, d.bY))

  pure def encPub(d: PubDigest): int =
    pairNat(d.pKind, pairNat(d.pA, pairNat(d.pB, d.pC)))

  // Public PoseidonFr_4(tag, operandFr) modeled as an injective tuple
  pure def digestPub(shkTag: int, operandFr: int): PubDigest =
    { pKind: 1, pA: shkTag, pB: operandFr, pC: 0 }

  // Public SPONGE(3) / Compact transientHash([c_p, input_pub, outcome_pub])
  pure def claimHashPub(cpDig: PubDigest, inDig: PubDigest, outDig: PubDigest): PubDigest =
    { pKind: 2, pA: encPub(cpDig), pB: encPub(inDig), pC: encPub(outDig) }

  // Canonical Fr limb helpers (decode.rs:342-368)
  pure def limbsRawVal(fl: FrLimbs): int =
    fl.loLimb + OP_LIMB_BOUND * fl.hiLimb

  pure def packLimbsModFr(fl: FrLimbs): int =
    modPos(limbsRawVal(fl), FR_P)

  pure def limbsCanon(fl: FrLimbs): bool = and {
    fl.loLimb >= 0,
    fl.loLimb < OP_LIMB_BOUND,
    fl.hiLimb >= 0,
    fl.hiLimb < OP_LIMB_BOUND,
    limbsRawVal(fl) < FR_P,
  }

  pure def limbsOfFr(v: int): FrLimbs = {
    val vm = modPos(v, FR_P)
    { loLimb: vm % OP_LIMB_BOUND, hiLimb: vm / OP_LIMB_BOUND }
  }

  // Packed RootSealAir operands (stmt.rs:413-418, 1528-1541)
  // Distinct canonical elements in 1..FR_P-1 for every (prog, inputId, outcome)
  pure def sealOp0(progId: int): int =
    if (progId == PROG_A) 11 else 21

  pure def sealOp1(progId: int, inputId: int): int =
    if (progId == PROG_A) (13 + inputId) else (23 + inputId)

  pure def sealOp2(progId: int, inputId: int, outVal: int): int =
    if (progId == PROG_A)
      (if (outVal == OUT_OK) (17 + inputId) else (19 + inputId))
    else
      (if (outVal == OUT_OK) (27 + inputId) else (29 + inputId))

  pure def cpPubOf(progId: int): PubDigest =
    digestPub(TAG_SHK_PROGC, sealOp0(progId))

  pure def inputPubOf(progId: int, inputId: int): PubDigest =
    digestPub(TAG_SHK_INPUT, sealOp1(progId, inputId))

  pure def outcomePubOf(progId: int, inputId: int, outVal: int): PubDigest =
    digestPub(TAG_SHK_OUTCOME, sealOp2(progId, inputId, outVal))

  pure val REG_PUB_HONEST: PubDigest = digestPub(TAG_SHK_REG, 5)
  pure val SET_PUB_HONEST: PubDigest = digestPub(TAG_SHK_SET, 7)
  pure val SET_ROOT_PRE: int = 7

  // BabyBear inverse of (param4 - TAG_SHK_SET) mod BB_P for honest is_zero advice
  pure def invDiffTagSet(param4Word: int): int = {
    val diff = modPos(param4Word - TAG_SHK_SET, BB_P)
    if (diff == 0) 0
    else
      1.to(BB_P - 1).filter(w => modPos(diff * w, BB_P) == 1).fold(0, (accW, w) => w)
  }

  // 4-limb h_hi integer decoding (base HHI_BOUND = 4, bound 4^4 = 256)
  pure def decodeH64(limbs: List[int]): int =
    limbs[0] + HHI_BOUND * limbs[1] + 16 * limbs[2] + 64 * limbs[3]

  pure def encodeH64(val64: int): List[int] =
    List(
      val64 % HHI_BOUND,
      (val64 / HHI_BOUND) % HHI_BOUND,
      (val64 / 16) % HHI_BOUND,
      (val64 / 64) % HHI_BOUND
    )

  pure def carriesOfAdd(hPre: List[int], addSteps: int): List[int] = {
    val s0 = hPre[0] + addSteps
    val c0 = s0 / HHI_BOUND
    val s1 = hPre[1] + c0
    val c1 = s1 / HHI_BOUND
    val s2 = hPre[2] + c1
    val c2 = s2 / HHI_BOUND
    val s3 = hPre[3] + c2
    val c3 = s3 / HHI_BOUND
    List(c0, c1, c2, c3)
  }

  pure def allLimbsInBound(limbs: List[int]): bool =
    limbs.indices().forall(i => and { limbs[i] >= 0, limbs[i] < HHI_BOUND })

  // ---------------------------------------------------------------------------
  // 4. Q3 Reference & Circuit Onion Computation
  // ---------------------------------------------------------------------------

  pure def entDigestBb(ent: LogEnt, valWord: int): BbDigest =
    { bTag: TAG_BB_ENT, bX: ent.opCode * 100 + ent.opParam, bY: valWord }

  pure def stepOnionBb(prevD: BbDigest, entD: BbDigest): BbDigest =
    { bTag: TAG_BB_STEP, bX: encBb(prevD), bY: encBb(entD) }

  pure def baseRCount(param4Word: int, modeBit: int): int = {
    val isOpen = if (param4Word == TAG_SHK_SET) 1 else 0
    5 + isOpen + modeBit
  }

  // Build the active reservation + optional discretionary entries and values
  pure def buildTableEntries(w: WitnessState): List[LogEnt] = {
    val rBase = List(
      { opCode: OP_DIGEST, opParam: TAG_SHK_PROGC },
      { opCode: OP_DIGEST, opParam: TAG_SHK_INPUT },
      { opCode: OP_DIGEST, opParam: TAG_SHK_OUTCOME },
      { opCode: OP_SPONGE, opParam: 3 },
      { opCode: OP_DIGEST, opParam: w.param4Word }
    )
    val with5 =
      if (or { w.param4Word == TAG_SHK_SET, w.modeBit == 1 })
        rBase.append({ opCode: OP_DIGEST, opParam: TAG_SHK_PROGC })
      else
        rBase
    if (w.extraEntOp != OP_NOP)
      with5.append({ opCode: w.extraEntOp, opParam: w.extraEntParam })
    else
      with5
  }

  pure def buildTableValWords(w: WitnessState): List[int] = {
    val baseWords = List(
      limbsRawVal(w.rOp0Limbs),
      limbsRawVal(w.rOp1Limbs),
      limbsRawVal(w.rOp2Limbs),
      0,
      limbsRawVal(w.a4Limbs)
    )
    val with5 =
      if (or { w.param4Word == TAG_SHK_SET, w.modeBit == 1 })
        baseWords.append(limbsRawVal(w.a5Limbs))
      else
        baseWords
    if (w.extraEntOp != OP_NOP)
      with5.append(w.tableExtraVal)
    else
      with5
  }

  pure def buildOnionEntries(w: WitnessState): List[LogEnt] = {
    val rBase = List(
      { opCode: OP_DIGEST, opParam: TAG_SHK_PROGC },
      { opCode: OP_DIGEST, opParam: TAG_SHK_INPUT },
      { opCode: OP_DIGEST, opParam: TAG_SHK_OUTCOME },
      { opCode: OP_SPONGE, opParam: 3 },
      { opCode: OP_DIGEST, opParam: w.param4Word }
    )
    val with5 =
      if (or { w.param4Word == TAG_SHK_SET, w.modeBit == 1 })
        rBase.append({ opCode: OP_DIGEST, opParam: TAG_SHK_PROGC })
      else
        rBase
    val withExtra =
      if (w.onionIncExtra)
        with5.append({ opCode: OP_PUB, opParam: w.extraEntParam })
      else
        with5
    if (w.onionIncNop)
      withExtra.append({ opCode: OP_NOP, opParam: 0 })
    else
      withExtra
  }

  pure def buildOnionValWords(w: WitnessState): List[int] = {
    val baseWords = List(
      limbsRawVal(w.rOp0Limbs),
      limbsRawVal(w.rOp1Limbs),
      limbsRawVal(w.rOp2Limbs),
      0,
      limbsRawVal(w.a4Limbs)
    )
    val with5 =
      if (or { w.param4Word == TAG_SHK_SET, w.modeBit == 1 })
        baseWords.append(limbsRawVal(w.a5Limbs))
      else
        baseWords
    val withExtra =
      if (w.onionIncExtra)
        with5.append(w.onionExtraVal)
      else
        with5
    if (w.onionIncNop)
      withExtra.append(0)
    else
      withExtra
  }

  pure def evalOnion(ents: List[LogEnt], valWords: List[int]): OnionOut = {
    val nEnts = ents.length()
    val finalD = range(0, nEnts).foldl(D_EMPTY, (curD, idx) =>
      stepOnionBb(curD, entDigestBb(ents[idx], valWords[idx]))
    )
    { onD: finalD, onN: nEnts }
  }

  // ---------------------------------------------------------------------------
  // 5. Layer-by-Layer Constraint & Bus Balance Predicates
  // ---------------------------------------------------------------------------

  // Q2: Limb-addition equation hhi[i] + HHI_BOUND * cry[i] == hPre[i] + cin (mod BB_P)
  pure def limbAddConstraintsOk(hPre: List[int], stepCnt: int, hhi: List[int], cry: List[int]): bool = and {
    hPre.length() == 4,
    hhi.length() == 4,
    cry.length() == 4,
    0.to(3).forall(i => isBit(cry[i])),
    cry[3] == 0,
    0.to(3).forall(i => {
      val cin = if (i == 0) stepCnt else cry[i - 1]
      modPos(hhi[i] + HHI_BOUND * cry[i], BB_P) == modPos(hPre[i] + cin, BB_P)
    }),
  }

  // Q2: HHI_RANGE_BUS exact LogUp balance against HhiRangeAir (rows 0..HHI_BOUND-1 with free -m_v)
  // Any limb value in HHI_BOUND..BB_P-1 has no provider row in HhiRangeAir, so its net send
  // multiplicity across First rows 0 and 1 must be 0 mod BB_P.
  pure def hhiRangeBusOk(cfg: ModelCfg, w: WitnessState): bool = {
    val checkedIndices = if (cfg.hhiOneLimb) Set(0) else Set(0, 1, 2, 3)
    val multTied = if (cfg.hhiValidFree) true else and {
      w.n1Mult0 == w.n1V0,
      w.n1Mult1 == w.n1V1,
    }
    val busBalanced = HHI_BOUND.to(BB_P - 1).forall(badVal => {
      val m0 = checkedIndices.fold(0, (accM, idx) =>
        if (modPos(w.n1Hhi0[idx], BB_P) == badVal) accM + w.n1Mult0 else accM
      )
      val m1 = checkedIndices.fold(0, (accM, idx) =>
        if (modPos(w.n1Hhi1[idx], BB_P) == badVal) accM + w.n1Mult1 else accM
      )
      modPos(m0 + m1, BB_P) == 0
    })
    and { multTied, busBalanced }
  }

  // Q2: Tree + root statement height constraints (N1 -> N2 -> N3 -> L, Path C -> L, or Work)
  pure def checkQ2Heights(cfg: ModelCfg, w: WitnessState): bool = {
    // CORE admitted leaf bounds: h_pre limbs in 0..HHI_BOUND-1 and steps in 1..HHI_BOUND-1
    val leaf0CoreOk = and {
      allLimbsInBound(w.leaf0Hpre),
      w.leaf0Steps >= 1,
      w.leaf0Steps < HHI_BOUND,
    }
    val leaf1CoreOk = and {
      allLimbsInBound(w.leaf1Hpre),
      w.leaf1Steps >= 1,
      w.leaf1Steps < HHI_BOUND,
    }
    if (w.pcBit == 0) {
      // Full tree: StmtAir::eval_first + StmtAir::eval_root (npc == 1)
      val firstRowFlagsOk = and {
        isBit(w.n1V0),
        isBit(w.n1V1),
        w.n1V0 == 1,
        isBit(w.n1V0 - w.n1V1),
      }
      val row0AddOk = limbAddConstraintsOk(w.leaf0Hpre, w.leaf0Steps, w.n1Hhi0, w.n1Cry0)
      val row1AddOk =
        if (w.n1V1 == 1)
          and {
            leaf1CoreOk,
            w.leaf1Hpre == w.n1Hhi0,
            limbAddConstraintsOk(w.leaf1Hpre, w.leaf1Steps, w.n1Hhi1, w.n1Cry1),
          }
        else
          limbAddConstraintsOk(List(0, 0, 0, 0), 0, w.n1Hhi1, w.n1Cry1)
      val n1OutHhi = if (w.n1V1 == 1) w.n1Hhi1 else w.n1Hhi0
      val workOk =
        if (w.useWorkNode)
          and {
            cfg.workHhiUnranged,
            w.workSteps >= 1,
            w.workSteps < HHI_BOUND,
            // Kind::Work (stmt.rs:1079-1086): limb equation & boolean carries, NO HHI_RANGE_BUS!
            limbAddConstraintsOk(n1OutHhi, w.workSteps, w.workWhi, w.workWcry),
            w.rootHlo == w.leaf0Hpre,
            w.rootHhi == w.workWhi,
          }
        else
          and {
            w.rootHlo == w.leaf0Hpre,
            w.rootHhi == n1OutHhi,
          }
      and {
        leaf0CoreOk,
        firstRowFlagsOk,
        row0AddOk,
        row1AddOk,
        hhiRangeBusOk(cfg, w),
        workOk,
      }
    } else {
      // Path C (pc == 1, stmt.rs:1329-1341, 1366-1368): no N1, no HhiRangeAir
      and {
        w.pcBit == 1,
        not(w.useWorkNode),
        leaf0CoreOk,
        w.leaf0Hpre == List(0, 0, 0, 0),
        w.rootHlo == List(0, 0, 0, 0),
        w.rootHhi == List(w.leaf0Steps, 0, 0, 0),
      }
    }
  }

  // Q3: Child (D_EMPTY, 0), LogEntry/LogValue/OnionAir buses (LOG_BUS, ENT_BUS, VAL_BUS, TIE_BUS)
  pure def checkQ3LogTie(cfg: ModelCfg, w: WitnessState): bool = {
    val childEmpty = and { w.childLogD == D_EMPTY, w.childLogN == 0 }
    val childRuleOk =
      if (w.pcBit == 0)
        childEmpty
      else
        or { cfg.childLogFreePc1, childEmpty }

    val entryBoundsOk = or {
      and { w.extraEntOp == OP_NOP, w.extraEntParam == 0 },
      and { w.extraEntOp == OP_PUB, w.extraEntParam == 21 },
    }
    val tEnts = buildTableEntries(w)
    val expectedUsedVals = tEnts.length()

    // LOG_BUS: LogEntry sends off_end (+1), LogValue receives cnt (-1); unused limbs forced 0
    val logBusOk = and {
      w.valUsedCnt == expectedUsedVals,
      w.unusedValWord == 0,
    }

    // ENT_BUS: LogEntry sends used = 1 - b[NOP]; OnionAir receives -fa at sequential offsets
    val entBusOk = and {
      w.onionIncExtra == (w.extraEntOp == OP_PUB),
      not(w.onionIncNop),
    }

    // VAL_BUS: LogValue sends +u per limb; OnionAir receives -m per absorbed limb
    val valBusOk =
      if (or { cfg.onionSkipVal, w.extraEntOp == OP_NOP })
        true
      else
        w.onionExtraVal == w.tableExtraVal

    // TIE_BUS: OnionAir pushes (onD, onN) with +O_FIN (+1);
    // StmtAir::eval_root pushes (exposedLogD, exposedLogN) with -valid (-1 in sound, +1 in tieMultPlus)
    val onionOut = evalOnion(buildOnionEntries(w), buildOnionValWords(w))
    val stmtRootValid = 1
    val onionMult = if (cfg.tieDropped) 0 else 1
    val stmtMult =
      if (cfg.tieDropped) 0
      else if (cfg.tieMultPlus) modPos(stmtRootValid, BB_P)
      else modPos(-stmtRootValid, BB_P)

    val sameTieMsg = and {
      w.exposedLogD == onionOut.onD,
      w.exposedLogN == onionOut.onN,
    }
    val tieBusOk =
      if (sameTieMsg)
        modPos(onionMult + stmtMult, BB_P) == 0
      else
        and {
          modPos(onionMult, BB_P) == 0,
          modPos(stmtMult, BB_P) == 0,
        }

    and {
      isBit(w.pcBit),
      childRuleOk,
      entryBoundsOk,
      logBusOk,
      entBusOk,
      valBusOk,
      tieBusOk,
    }
  }

  // Q1: RootSealAir + L_Fr (gen_r_check, gen_ops_check) + Wrap decoder (decode.rs)
  pure def checkQ1LfrAndWrap(cfg: ModelCfg, w: WitnessState): bool = {
    // RootSealAir reconciles opened terminal header outcome with POST_D of verified tree
    val sealOk = (w.sealOut == w.trueOut)
    val sOp0 = sealOp0(w.runProg)
    val sOp1 = sealOp1(w.runProg, w.inputId)
    val sOp2 = sealOp2(w.runProg, w.inputId, w.sealOut)

    // Wrap decoder checks 30-bit limb range and Canon (< FR_P) on all logged Fr values
    val canonOk = and {
      limbsCanon(w.rOp0Limbs),
      limbsCanon(w.rOp1Limbs),
      limbsCanon(w.rOp2Limbs),
      limbsCanon(w.a4Limbs),
      limbsCanon(w.a5Limbs),
    }

    val v0 = packLimbsModFr(w.rOp0Limbs)
    val v1 = packLimbsModFr(w.rOp1Limbs)
    val v2 = packLimbsModFr(w.rOp2Limbs)
    val a4 = packLimbsModFr(w.a4Limbs)
    val a5 = packLimbsModFr(w.a5Limbs)

    // gen_r_check: mode is boolean; chunk order 1 <= j <= k when mode == 1
    val modeBoolOk = isBit(w.modeBit)
    val chunkOrderOk =
      if (w.modeBit == 1)
        and { w.chunkJ >= 1, w.chunkJ <= w.chunkK, w.chunkK < HHI_BOUND }
      else
        and { w.chunkJ >= 0, w.chunkJ < HHI_BOUND, w.chunkK >= 0, w.chunkK < HHI_BOUND }

    val tagIn = if (and { w.modeBit == 1, w.chunkJ != 1 }) TAG_SHK_LOCK else TAG_SHK_PROGC

    // gen_r_check: open = is_zero(param4 - SHK_SET) over BabyBear
    // is_zero constraints: diff * invW == 1 - open (mod BB_P) and diff * open == 0 (mod BB_P)
    val diff4 = modPos(w.param4Word - TAG_SHK_SET, BB_P)
    val isZeroOk = and {
      modPos(diff4 * w.openInvW, BB_P) == modPos(1 - w.openSel, BB_P),
      modPos(diff4 * w.openSel, BB_P) == 0,
      modPos((1 - w.openSel) * (w.param4Word - tagIn), BB_P) == 0,
    }
    val openOk =
      if (cfg.openFree)
        and {
          w.openSel >= 0,
          w.openSel < FR_P,
          or { w.param4Word == tagIn, w.param4Word == TAG_SHK_SET },
        }
      else
        isZeroOk

    // gen_ops_check: three R operands against RootSealAir ops, then A_0 = C_P cswap tie
    val ops0Ok = or { cfg.opsSkipIdx == 0, v0 == sOp0 }
    val ops1Ok = or { cfg.opsSkipIdx == 1, v1 == sOp1 }
    val ops2Ok = or { cfg.opsSkipIdx == 2, v2 == sOp2 }

    val aInCswap = modPos(a4 + w.openSel * (a5 - a4), FR_P)
    val lhsAnchor = modPos(aInCswap + w.modeBit * (v0 - aInCswap), FR_P)
    val anchorTieOk = or { cfg.anchorNotCp, lhsAnchor == v0 }

    // Native replay outputs & call-vector lane bindings (decode.rs:519-605)
    val pubCp = digestPub(TAG_SHK_PROGC, v0)
    val pubIn = digestPub(TAG_SHK_INPUT, v1)
    val pubOut = digestPub(TAG_SHK_OUTCOME, v2)
    val pubClaim = claimHashPub(pubCp, pubIn, pubOut)

    val eInIs5 = (w.param4Word == TAG_SHK_SET)
    val anchorPre = if (eInIs5) a5 else a4
    val lane10Anchor = digestPub(tagIn, anchorPre)

    // Transcript template check: SherlockLedger entries have TPL_ORD only (mode == 0)
    // unless chunkTranscript mutant is enabled
    val transcriptModeOk = or { cfg.chunkTranscript, w.modeBit == 0 }

    // Entry-specific lane map check (acceptRun: open=0; openClaim: open=1 with SHK_SET at entry 4)
    val entryFormOk =
      if (w.entryKind == ENTRY_ACCEPT_RUN)
        and {
          not(eInIs5),
          lane10Anchor == w.ledgerAnchor,
          w.callClaimPub == pubClaim,
          w.callOutcomePub == pubOut,
        }
      else if (w.entryKind == ENTRY_OPEN_CLAIM)
        and {
          eInIs5,
          lane10Anchor == w.ledgerAnchor,
          w.callSetPub == digestPub(TAG_SHK_SET, a4),
          w.callClaimPub == pubClaim,
          w.callOutcomePub == pubOut,
        }
      else
        false

    and {
      sealOk,
      canonOk,
      modeBoolOk,
      chunkOrderOk,
      openOk,
      ops0Ok,
      ops1Ok,
      ops2Ok,
      anchorTieOk,
      transcriptModeOk,
      entryFormOk,
    }
  }

  // Q4: SherlockLedger contract checks (sherlock-ledger.compact) + wrap verifier oracle
  pure def isCallAccepted(cfg: ModelCfg, w: WitnessState): bool = {
    val prologueOk = (w.ledgerAnchor != ZERO_PUB)
    val claimComputed = claimHashPub(w.callCp, w.callInputPub, w.callOutcomePub)
    if (w.entryKind == ENTRY_ACCEPT_RUN) {
      val replayOk = or { cfg.ledgerNoReplay, not(w.preRuns.contains(claimComputed)) }
      and {
        prologueOk,
        w.proofValidBit,
        w.callClaimPub == claimComputed,
        not(w.leaksPrivate),
        replayOk,
        checkQ2Heights(cfg, w),
        checkQ3LogTie(cfg, w),
        checkQ1LfrAndWrap(cfg, w),
      }
    } else if (w.entryKind == ENTRY_ACCEPT_COLL) {
      and {
        prologueOk,
        w.proofValidBit,
        w.collectionSoundBit,
        not(w.leaksPrivate),
        w.callRegPub == w.ledgerRegRoot,
        not(w.preRoots.contains(w.callSetPub)),
      }
    } else if (w.entryKind == ENTRY_OPEN_CLAIM) {
      val rootAccepted = w.preRoots.contains(w.callSetPub)
      val openReplayOk =
        if (cfg.ledgerTwoMaps)
          not(w.preOpenRuns.contains(claimComputed))
        else
          not(w.preRuns.contains(claimComputed))
      and {
        prologueOk,
        w.proofValidBit,
        w.collectionHasClaim,
        rootAccepted,
        w.callClaimPub == claimComputed,
        not(w.leaksPrivate),
        openReplayOk,
        checkQ2Heights(cfg, w),
        checkQ3LogTie(cfg, w),
        checkQ1LfrAndWrap(cfg, w),
      }
    } else {
      false
    }
  }

  // Synchronize post-state ledger maps with acceptance outcome
  pure def finalizeTransition(cfg: ModelCfg, cand: WitnessState): WitnessState = {
    val ok = isCallAccepted(cfg, cand)
    if (not(ok)) {
      {
        ...cand,
        postRuns: cand.preRuns,
        postOpenRuns: cand.preOpenRuns,
        postRoots: cand.preRoots,
        postHistory: cand.preHistory,
      }
    } else if (cand.entryKind == ENTRY_ACCEPT_RUN) {
      {
        ...cand,
        postRuns: cand.preRuns.union(Set(cand.callClaimPub)),
        postOpenRuns: cand.preOpenRuns,
        postRoots: cand.preRoots,
        postHistory: cand.preHistory.append(cand.callClaimPub),
      }
    } else if (cand.entryKind == ENTRY_ACCEPT_COLL) {
      {
        ...cand,
        postRuns: cand.preRuns,
        postOpenRuns: cand.preOpenRuns,
        postRoots: cand.preRoots.union(Set(cand.callSetPub)),
        postHistory: cand.preHistory,
      }
    } else {
      val updatedRuns =
        if (cfg.ledgerTwoMaps) cand.preRuns
        else cand.preRuns.union(Set(cand.callClaimPub))
      val updatedOpen =
        if (cfg.ledgerTwoMaps) cand.preOpenRuns.union(Set(cand.callClaimPub))
        else cand.preOpenRuns
      {
        ...cand,
        postRuns: updatedRuns,
        postOpenRuns: updatedOpen,
        postRoots: cand.preRoots,
        postHistory: cand.preHistory.append(cand.callClaimPub),
      }
    }
  }

  pure def isAcceptedCfg(cfg: ModelCfg, w: WitnessState): bool =
    isCallAccepted(cfg, w)

  // ---------------------------------------------------------------------------
  // 6. Reference Semantics (invRefine Target)
  // ---------------------------------------------------------------------------

  pure def noDuplicateSettlements(hist: List[PubDigest]): bool =
    hist.indices().forall(i =>
      hist.indices().forall(j =>
        (i != j) implies (hist[i] != hist[j])
      )
    )

  pure def trueTotalSteps(w: WitnessState): int =
    if (w.pcBit == 1)
      w.leaf0Steps
    else
      w.leaf0Steps
        + (if (w.n1V1 == 1) w.leaf1Steps else 0)
        + (if (w.useWorkNode) w.workSteps else 0)

  pure def refinesWitness(w: WitnessState): bool = {
    val trueCp = cpPubOf(w.runProg)
    val trueIn = inputPubOf(w.runProg, w.inputId)
    val trueOutPub = outcomePubOf(w.runProg, w.inputId, w.trueOut)
    val trueClaim = claimHashPub(trueCp, trueIn, trueOutPub)
    val refOn = evalOnion(buildTableEntries(w), buildTableValWords(w))

    val commonOk = and {
      w.ledgerAnchor != ZERO_PUB,
      w.ledgerAnchor == cpPubOf(PROG_A),
      w.proofValidBit,
      not(w.leaksPrivate),
      noDuplicateSettlements(w.postHistory),
    }

    if (w.entryKind == ENTRY_ACCEPT_COLL) {
      and {
        commonOk,
        w.collectionSoundBit,
        w.callRegPub == w.ledgerRegRoot,
      }
    } else {
      and {
        commonOk,
        // Q1 & Q4: Run belongs to the ledger's anchored program A and matches public args
        w.runProg == PROG_A,
        w.callCp == trueCp,
        w.callInputPub == trueIn,
        w.callOutcomePub == trueOutPub,
        w.callClaimPub == trueClaim,
        // Open-claim extra reference condition
        (w.entryKind == ENTRY_OPEN_CLAIM) implies and {
          w.preRoots.contains(w.callSetPub),
          w.collectionHasClaim,
        },
        // Q2: Every h_lo and h_hi limb is < HHI_BOUND and h_hi = h_lo + steps < 2^64 (scaled 256)
        allLimbsInBound(w.rootHlo),
        allLimbsInBound(w.rootHhi),
        decodeH64(w.rootHhi) == decodeH64(w.rootHlo) + trueTotalSteps(w),
        decodeH64(w.rootHhi) < H64_BOUND,
        // Q3: Child logs nothing and exposed (log_d, log_n) equals reference onion of L tables
        w.childLogD == D_EMPTY,
        w.childLogN == 0,
        w.exposedLogD == refOn.onD,
        w.exposedLogN == refOn.onN,
      }
    }
  }

  // ---------------------------------------------------------------------------
  // 7. Honest Witness & Tamper Builders
  // ---------------------------------------------------------------------------

  // Base honest witness for an ordinary acceptRun call (1-shard full tree) on fresh ledger
  pure def mkHonestRunBase(progId: int, inputId: int, outVal: int, ledgerProg: int): WitnessState = {
    val hPre0 = List(2, 1, 0, 0)
    val st0 = 1
    val hHi0 = encodeH64(decodeH64(hPre0) + st0)
    val c0 = carriesOfAdd(hPre0, st0)

    val op0 = sealOp0(progId)
    val op1 = sealOp1(progId, inputId)
    val op2 = sealOp2(progId, inputId, outVal)
    val aPre = sealOp0(ledgerProg)

    val cp = cpPubOf(progId)
    val inP = inputPubOf(progId, inputId)
    val outP = outcomePubOf(progId, inputId, outVal)
    val clP = claimHashPub(cp, inP, outP)
    val lAnchor = cpPubOf(ledgerProg)

    val emptySet: Set[PubDigest] = Set()
    val emptyHist: List[PubDigest] = List()

    val draft = {
      isTampered: false,
      attemptTag: ATTEMPT_HONEST,
      entryKind: ENTRY_ACCEPT_RUN,

      runProg: progId,
      inputId: inputId,
      trueOut: outVal,
      sealOut: outVal,
      pcBit: 0,
      useWorkNode: false,

      n1V0: 1,
      n1V1: 0,
      leaf0Hpre: hPre0,
      leaf0Steps: st0,
      leaf1Hpre: List(0, 0, 0, 0),
      leaf1Steps: 1,
      n1Hhi0: hHi0,
      n1Cry0: c0,
      n1Hhi1: List(0, 0, 0, 0),
      n1Cry1: List(0, 0, 0, 0),
      n1Mult0: 1,
      n1Mult1: 0,
      workSteps: 1,
      workWhi: hHi0,
      workWcry: List(0, 0, 0, 0),
      rootHlo: hPre0,
      rootHhi: hHi0,

      rOp0Limbs: limbsOfFr(op0),
      rOp1Limbs: limbsOfFr(op1),
      rOp2Limbs: limbsOfFr(op2),
      a4Limbs: limbsOfFr(aPre),
      a5Limbs: limbsOfFr(0),
      param4Word: TAG_SHK_PROGC,
      openInvW: invDiffTagSet(TAG_SHK_PROGC),
      openSel: 0,
      modeBit: 0,
      chunkJ: 0,
      chunkK: 0,

      childLogD: D_EMPTY,
      childLogN: 0,
      extraEntOp: OP_PUB,
      extraEntParam: 21,
      tableExtraVal: 5,
      valUsedCnt: 6,
      unusedValWord: 0,
      onionIncExtra: true,
      onionIncNop: false,
      onionExtraVal: 5,
      exposedLogD: D_EMPTY,
      exposedLogN: 0,

      proofValidBit: true,
      collectionSoundBit: true,
      collectionHasClaim: true,
      leaksPrivate: false,
      ledgerAnchor: lAnchor,
      ledgerRegRoot: REG_PUB_HONEST,
      callCp: cp,
      callInputPub: inP,
      callOutcomePub: outP,
      callClaimPub: clP,
      callRegPub: REG_PUB_HONEST,
      callSetPub: SET_PUB_HONEST,

      preRuns: emptySet,
      preOpenRuns: emptySet,
      preRoots: emptySet,
      preHistory: emptyHist,

      postRuns: emptySet,
      postOpenRuns: emptySet,
      postRoots: emptySet,
      postHistory: emptyHist,
    }
    val on = evalOnion(buildOnionEntries(draft), buildOnionValWords(draft))
    { ...draft, exposedLogD: on.onD, exposedLogN: on.onN }
  }

  pure def withPreLedgerFrom(prev: WitnessState, cand: WitnessState): WitnessState =
    {
      ...cand,
      preRuns: prev.postRuns,
      preOpenRuns: prev.postOpenRuns,
      preRoots: prev.postRoots,
      preHistory: prev.postHistory,
    }

  pure def refreshOnion(w: WitnessState): WitnessState = {
    val tEnts = buildTableEntries(w)
    val on = evalOnion(buildOnionEntries(w), buildOnionValWords(w))
    {
      ...w,
      valUsedCnt: tEnts.length(),
      exposedLogD: on.onD,
      exposedLogN: on.onN,
    }
  }

  pure def honestInitForCfg(cfg: ModelCfg): WitnessState =
    finalizeTransition(cfg, mkHonestRunBase(PROG_A, IN_0, OUT_OK, PROG_A))

  // Honest 2-shard binary N1 run on fresh claim IN_1
  pure def mkHonestBinaryN1(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val hPre0 = List(3, 0, 0, 0)
    val st0 = 2
    val hHi0 = encodeH64(decodeH64(hPre0) + st0)
    val c0 = carriesOfAdd(hPre0, st0)
    val st1 = 2
    val hHi1 = encodeH64(decodeH64(hHi0) + st1)
    val c1 = carriesOfAdd(hHi0, st1)
    val cand = refreshOnion({
      ...base,
      n1V0: 1,
      n1V1: 1,
      leaf0Hpre: hPre0,
      leaf0Steps: st0,
      leaf1Hpre: hHi0,
      leaf1Steps: st1,
      n1Hhi0: hHi0,
      n1Cry0: c0,
      n1Hhi1: hHi1,
      n1Cry1: c1,
      n1Mult0: 1,
      n1Mult1: 1,
      rootHlo: hPre0,
      rootHhi: hHi1,
    })
    finalizeTransition(cfg, cand)
  }

  // Honest Path C (1-leaf, pc == 1) run on fresh claim IN_1
  pure def mkHonestPathC(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val st0 = 3
    val cand = refreshOnion({
      ...base,
      pcBit: 1,
      leaf0Hpre: List(0, 0, 0, 0),
      leaf0Steps: st0,
      rootHlo: List(0, 0, 0, 0),
      rootHhi: List(st0, 0, 0, 0),
    })
    finalizeTransition(cfg, cand)
  }

  // Honest acceptCollection call
  pure def mkHonestAcceptCollection(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val cand = {
      ...base,
      entryKind: ENTRY_ACCEPT_COLL,
      callRegPub: REG_PUB_HONEST,
      callSetPub: SET_PUB_HONEST,
    }
    finalizeTransition(cfg, cand)
  }

  // Honest openClaim call (entry 4 = SHK_SET, anchor at entry 5) for inputId
  pure def mkHonestOpenClaim(cfg: ModelCfg, prev: WitnessState, inputId: int): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, inputId, OUT_OK, PROG_A))
    val cand = refreshOnion({
      ...base,
      entryKind: ENTRY_OPEN_CLAIM,
      param4Word: TAG_SHK_SET,
      openInvW: 0,
      openSel: 1,
      a4Limbs: limbsOfFr(SET_ROOT_PRE),
      a5Limbs: limbsOfFr(sealOp0(PROG_A)),
      callSetPub: SET_PUB_HONEST,
    })
    finalizeTransition(cfg, cand)
  }

  // R1: Changed outcome argument against honest proof of Program A
  pure def mkRefusalR1(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val badOut = outcomePubOf(PROG_A, IN_1, OUT_ALT)
    val badCl = claimHashPub(base.callCp, base.callInputPub, badOut)
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_R1,
      callOutcomePub: badOut,
      callClaimPub: badCl,
    }
    finalizeTransition(cfg, cand)
  }

  // R2: Proof bit flipped
  pure def mkRefusalR2(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_R2,
      proofValidBit: false,
    }
    finalizeTransition(cfg, cand)
  }

  // R3a: Replay of already-settled claim IN_0 via acceptRun
  pure def mkReplayAcceptRun(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_0, OUT_OK, PROG_A))
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_R3,
    }
    finalizeTransition(cfg, cand)
  }

  // R3b: Cross-entry replay of already-settled claim IN_0 via openClaim
  pure def mkReplayOpenClaim(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = mkHonestOpenClaim(cfg, prev, IN_0)
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_R3,
      preRuns: prev.postRuns,
      preOpenRuns: prev.postOpenRuns,
      preRoots: prev.postRoots,
      preHistory: prev.postHistory,
    }
    finalizeTransition(cfg, cand)
  }

  // R4: Program B's run and arguments submitted against Program A's ledger anchor
  // Anchor entry 4 carries A's anchor preimage so lane 10 matches A's anchor, while run is B
  pure def mkAttackR4WrongProg(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_B, IN_1, OUT_OK, PROG_A))
    val cand = refreshOnion({
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_R4,
    })
    finalizeTransition(cfg, cand)
  }

  // Zero / unset anchor attempt
  pure def mkZeroAnchorAttempt(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_ZERO_ANCHOR,
      ledgerAnchor: ZERO_PUB,
    }
    finalizeTransition(cfg, cand)
  }

  // Attack for ops_check_skip_k (k in 0..2): forge logged operand k while tree/seal is honest
  pure def mkAttackSkipOp(cfg: ModelCfg, prev: WitnessState, skipK: int): WitnessState = {
    if (skipK == 0) {
      // Run is Program B, logged R[0] and R[4] both set to A's op0 so A_0 == vals[0] holds!
      val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_B, IN_1, OUT_OK, PROG_A))
      val forgedOp0 = sealOp0(PROG_A)
      val forgedCp = digestPub(TAG_SHK_PROGC, forgedOp0)
      val forgedCl = claimHashPub(forgedCp, base.callInputPub, base.callOutcomePub)
      val cand = refreshOnion({
        ...base,
        isTampered: true,
        attemptTag: ATTEMPT_TAMPER,
        rOp0Limbs: limbsOfFr(forgedOp0),
        a4Limbs: limbsOfFr(forgedOp0),
        callCp: forgedCp,
        callClaimPub: forgedCl,
      })
      finalizeTransition(cfg, cand)
    } else if (skipK == 1) {
      // Run is Program A on IN_0, logged R[1] forged to IN_1's input digest
      val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_0, OUT_OK, PROG_A))
      val forgedOp1 = sealOp1(PROG_A, IN_1)
      val forgedIn = digestPub(TAG_SHK_INPUT, forgedOp1)
      val forgedCl = claimHashPub(base.callCp, forgedIn, base.callOutcomePub)
      val cand = refreshOnion({
        ...base,
        isTampered: true,
        attemptTag: ATTEMPT_TAMPER,
        rOp1Limbs: limbsOfFr(forgedOp1),
        callInputPub: forgedIn,
        callClaimPub: forgedCl,
      })
      finalizeTransition(cfg, cand)
    } else {
      // Run is Program A with trueOut = OUT_OK, logged R[2] forged to OUT_ALT!
      val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
      val forgedOp2 = sealOp2(PROG_A, IN_1, OUT_ALT)
      val forgedOut = digestPub(TAG_SHK_OUTCOME, forgedOp2)
      val forgedCl = claimHashPub(base.callCp, base.callInputPub, forgedOut)
      val cand = refreshOnion({
        ...base,
        isTampered: true,
        attemptTag: ATTEMPT_TAMPER,
        rOp2Limbs: limbsOfFr(forgedOp2),
        callOutcomePub: forgedOut,
        callClaimPub: forgedCl,
      })
      finalizeTransition(cfg, cand)
    }
  }

  // Attack for open_free: non-boolean openSel = 10 in Fr interpolates a4 + 10*(a5 - a4) = 11 + 10*1 = 21 = sealOp0(B)
  pure def mkAttackOpenFree(cfg: ModelCfg, prev: WitnessState, openWitness: int): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_B, IN_1, OUT_OK, PROG_A))
    val cand = refreshOnion({
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      a4Limbs: limbsOfFr(11),
      a5Limbs: limbsOfFr(12),
      openSel: openWitness,
    })
    finalizeTransition(cfg, cand)
  }

  // Attack for chunk_j1_anchor: modeBit = 1, chunkJ = 1, chunkK = 1 bypasses cswap(mode, a_in, vals[0])
  pure def mkAttackChunkJ1Anchor(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_B, IN_1, OUT_OK, PROG_A))
    val cand = refreshOnion({
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      modeBit: 1,
      chunkJ: 1,
      chunkK: 1,
      a4Limbs: limbsOfFr(sealOp0(PROG_A)),
      a5Limbs: limbsOfFr(sealOp0(PROG_A)),
    })
    finalizeTransition(cfg, cand)
  }

  // Attack for hhi_valid_free: hhi[0] = 2 + 1 - 4 mod 17 = 16 (>= 4) with cry[0] = 1 and lookup mult = 0
  pure def mkAttackHhiValidFree(cfg: ModelCfg, prev: WitnessState, multWitness: int): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val badHhi = List(modPos(2 + 1 - HHI_BOUND, BB_P), 2, 0, 0)
    val badCry = List(1, 0, 0, 0)
    val cand = refreshOnion({
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      n1Hhi0: badHhi,
      n1Cry0: badCry,
      n1Mult0: multWitness,
      rootHhi: badHhi,
    })
    finalizeTransition(cfg, cand)
  }

  // Attack for hhi_one_limb: limb 0 kept in range (3 < 4), limb 1 overflows to 1 - 4 mod 17 = 14 (>= 4) with cry[1] = 1
  pure def mkAttackHhiOneLimb(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val badHhi = List(3, modPos(1 - HHI_BOUND, BB_P), 1, 0)
    val badCry = List(0, 1, 0, 0)
    val cand = refreshOnion({
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      n1Hhi0: badHhi,
      n1Cry0: badCry,
      rootHhi: badHhi,
    })
    finalizeTransition(cfg, cand)
  }

  // Attack for work_hhi_unranged (Kind::Work in stmt.rs:1079-1086): whi[0] = 3 + 1 - 4 = 0? Use steps = 1 on hhi0 = [2,1,0,0]:
  // whi[0] = 2 + 1 - 4 mod 17 = 16 (>= 4), wcry[0] = 1, whi[1] = 1 + 1 = 2
  pure def mkAttackWorkHhiUnranged(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val hPre0 = List(1, 1, 0, 0)
    val st0 = 1
    val n1Out = List(2, 1, 0, 0)
    val wSt = 1
    val badWhi = List(modPos(2 + wSt - HHI_BOUND, BB_P), 2, 0, 0)
    val badWcry = List(1, 0, 0, 0)
    val cand = refreshOnion({
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      useWorkNode: true,
      leaf0Hpre: hPre0,
      leaf0Steps: st0,
      n1Hhi0: n1Out,
      n1Cry0: List(0, 0, 0, 0),
      workSteps: wSt,
      workWhi: badWhi,
      workWcry: badWcry,
      rootHlo: hPre0,
      rootHhi: badWhi,
    })
    finalizeTransition(cfg, cand)
  }

  // Attack for onion_skip_value: LogValue slot changed to newTableVal while OnionAir absorbs old 5
  pure def mkAttackOnionSkipValue(cfg: ModelCfg, prev: WitnessState, newTableVal: int): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val on = evalOnion(buildOnionEntries(base), buildOnionValWords(base))
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      tableExtraVal: newTableVal,
      onionExtraVal: 5,
      exposedLogD: on.onD,
      exposedLogN: on.onN,
    }
    finalizeTransition(cfg, cand)
  }

  // Attack for tie_dropped / tie_mult_plus: forge exposed (log_d, log_n) to D_EMPTY, 0
  pure def mkAttackForgedExposedLog(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = withPreLedgerFrom(prev, mkHonestRunBase(PROG_A, IN_1, OUT_OK, PROG_A))
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      exposedLogD: D_EMPTY,
      exposedLogN: 0,
    }
    finalizeTransition(cfg, cand)
  }

  // Attack for child_log_free_pc1: Path C leaf carries non-empty child log (silently dropped)
  pure def mkAttackChildLogPc1(cfg: ModelCfg, prev: WitnessState): WitnessState = {
    val base = mkHonestPathC(cfg, prev)
    val nonEmptyChild = stepOnionBb(D_EMPTY, entDigestBb({ opCode: OP_PUB, opParam: 22 }, 9))
    val cand = {
      ...base,
      isTampered: true,
      attemptTag: ATTEMPT_TAMPER,
      childLogD: nonEmptyChild,
      childLogN: 1,
      preRuns: prev.postRuns,
      preOpenRuns: prev.postOpenRuns,
      preRoots: prev.postRoots,
      preHistory: prev.postHistory,
    }
    finalizeTransition(cfg, cand)
  }
}

// =============================================================================
// Parametric Base State Machine
// =============================================================================
module t10_bound_base {
  import t10_bound_defs.*

  const C_OPS_SKIP: int
  const C_OPEN_FREE: bool
  const C_ANCHOR_NOT_CP: bool
  const C_CHUNK_TRANSCRIPT: bool
  const C_HHI_ONE_LIMB: bool
  const C_HHI_VALID_FREE: bool
  const C_WORK_HHI_UNRANGED: bool
  const C_TIE_MULT_PLUS: bool
  const C_TIE_DROPPED: bool
  const C_ONION_SKIP_VAL: bool
  const C_CHILD_LOG_FREE_PC1: bool
  const C_LEDGER_NO_REPLAY: bool
  const C_LEDGER_TWO_MAPS: bool

  pure val activeCfg: ModelCfg = {
    opsSkipIdx: C_OPS_SKIP,
    openFree: C_OPEN_FREE,
    anchorNotCp: C_ANCHOR_NOT_CP,
    chunkTranscript: C_CHUNK_TRANSCRIPT,
    hhiOneLimb: C_HHI_ONE_LIMB,
    hhiValidFree: C_HHI_VALID_FREE,
    workHhiUnranged: C_WORK_HHI_UNRANGED,
    tieMultPlus: C_TIE_MULT_PLUS,
    tieDropped: C_TIE_DROPPED,
    onionSkipVal: C_ONION_SKIP_VAL,
    childLogFreePc1: C_CHILD_LOG_FREE_PC1,
    ledgerNoReplay: C_LEDGER_NO_REPLAY,
    ledgerTwoMaps: C_LEDGER_TWO_MAPS,
  }

  var ws: WitnessState
  var acc: bool

  // Core soundness refinement invariant: accepted => reference semantics hold
  val invRefine: bool = acc implies refinesWitness(ws)

  // Negated reachability invariants (MUST be violated with exit 1 in sound instance)
  val reachHonestAccepted: bool =
    not(and { acc, not(ws.isTampered), ws.runProg == PROG_A, ws.entryKind == ENTRY_ACCEPT_RUN })

  val reachBinaryN1Accepted: bool =
    not(and { acc, ws.pcBit == 0, ws.n1V1 == 1 })

  val reachPathCAccepted: bool =
    not(and { acc, ws.pcBit == 1 })

  val reachOpenClaimAccepted: bool =
    not(and { acc, ws.entryKind == ENTRY_OPEN_CLAIM, ws.param4Word == TAG_SHK_SET })

  val reachR1Refused: bool =
    not(and { ws.attemptTag == ATTEMPT_R1, not(acc) })

  val reachR2Refused: bool =
    not(and { ws.attemptTag == ATTEMPT_R2, not(acc) })

  val reachR3Refused: bool =
    not(and { ws.attemptTag == ATTEMPT_R3, not(acc) })

  val reachR4Refused: bool =
    not(and { ws.attemptTag == ATTEMPT_R4, not(acc) })

  val reachZeroAnchorRefused: bool =
    not(and { ws.attemptTag == ATTEMPT_ZERO_ANCHOR, not(acc) })

  // Must NOT be violated (exit 0) in sound instance; violated (exit 1) in soundness mutants
  val reachAcceptedBadTamper: bool =
    not(and { ws.isTampered, acc, not(refinesWitness(ws)) })

  action applyCandidate(candWs: WitnessState): bool = all {
    ws' = candWs,
    acc' = isAcceptedCfg(activeCfg, candWs),
  }

  action stepAttackSkipOp: bool = {
    nondet skipK = Set(0, 1, 2).oneOf()
    applyCandidate(mkAttackSkipOp(activeCfg, ws, skipK))
  }

  action stepAttackOpenFree: bool = {
    nondet openWit = Set(0, 1, 2, 10).oneOf()
    applyCandidate(mkAttackOpenFree(activeCfg, ws, openWit))
  }

  action stepAttackHhiValidFree: bool = {
    nondet multWit = Set(0, 1).oneOf()
    applyCandidate(mkAttackHhiValidFree(activeCfg, ws, multWit))
  }

  action stepAttackOnionSkipVal: bool = {
    nondet badVal = Set(6, 7).oneOf()
    applyCandidate(mkAttackOnionSkipValue(activeCfg, ws, badVal))
  }

  // init builds the honest witness of Program A on SherlockLedger
  action init: bool =
    applyCandidate(honestInitForCfg(activeCfg))

  // step lets a nondeterministic prover / caller attempt honest branches, R1..R4, or witness tampers
  action step: bool = any {
    // Honest branches across tree paths & contract entry points
    applyCandidate(mkHonestBinaryN1(activeCfg, ws)),
    applyCandidate(mkHonestPathC(activeCfg, ws)),
    applyCandidate(mkHonestAcceptCollection(activeCfg, ws)),
    applyCandidate(mkHonestOpenClaim(activeCfg, ws, IN_1)),

    // Design contract refusal targets R1..R4 and zero anchor
    applyCandidate(mkRefusalR1(activeCfg, ws)),
    applyCandidate(mkRefusalR2(activeCfg, ws)),
    applyCandidate(mkReplayAcceptRun(activeCfg, ws)),
    applyCandidate(mkReplayOpenClaim(activeCfg, ws)),
    applyCandidate(mkAttackR4WrongProg(activeCfg, ws)),
    applyCandidate(mkZeroAnchorAttempt(activeCfg, ws)),

    // Q1 prover tampers: skipped operand check, non-boolean open bit, chunk j=1 mode
    stepAttackSkipOp,
    stepAttackOpenFree,
    applyCandidate(mkAttackChunkJ1Anchor(activeCfg, ws)),

    // Q2 prover tampers: h_hi lookup multiplicity, higher limb >= bound, Kind::Work whi
    stepAttackHhiValidFree,
    applyCandidate(mkAttackHhiOneLimb(activeCfg, ws)),
    applyCandidate(mkAttackWorkHhiUnranged(activeCfg, ws)),

    // Q3 prover tampers: VAL_BUS skip, TIE_BUS mismatch, Path C non-empty child log
    stepAttackOnionSkipVal,
    applyCandidate(mkAttackForgedExposedLog(activeCfg, ws)),
    applyCandidate(mkAttackChildLogPc1(activeCfg, ws)),
  }
}

// =============================================================================
// Sound Concrete Instance
// =============================================================================
module t10_bound {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

// =============================================================================
// Required Mutant Concrete Instances (t10_bound_m_<mutant>)
// =============================================================================

module t10_bound_m_ops_check_skip_0 {
  import t10_bound_base(
    C_OPS_SKIP = 0,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_ops_check_skip_1 {
  import t10_bound_base(
    C_OPS_SKIP = 1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_ops_check_skip_2 {
  import t10_bound_base(
    C_OPS_SKIP = 2,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_open_free {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = true,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_anchor_not_cp {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = true,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_hhi_one_limb {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = true,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_hhi_valid_free {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = true,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_tie_mult_plus {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = true,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_onion_skip_value {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = true,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_child_log_free_pc1 {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = true,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_ledger_no_replay {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = true,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_ledger_two_maps {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = true
  ).*
}

// =============================================================================
// Additional Diagnostic Mutants (Code Findings & Tie Companion)
// =============================================================================

module t10_bound_m_tie_dropped {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = true,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_work_hhi_unranged {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = false,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = true,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}

module t10_bound_m_chunk_j1_anchor {
  import t10_bound_base(
    C_OPS_SKIP = -1,
    C_OPEN_FREE = false,
    C_ANCHOR_NOT_CP = false,
    C_CHUNK_TRANSCRIPT = true,
    C_HHI_ONE_LIMB = false,
    C_HHI_VALID_FREE = false,
    C_WORK_HHI_UNRANGED = false,
    C_TIE_MULT_PLUS = false,
    C_TIE_DROPPED = false,
    C_ONION_SKIP_VAL = false,
    C_CHILD_LOG_FREE_PC1 = false,
    C_LEDGER_NO_REPLAY = false,
    C_LEDGER_TWO_MAPS = false
  ).*
}
