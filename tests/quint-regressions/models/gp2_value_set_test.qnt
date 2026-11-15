// Deterministic unit tests, mutant non-vacuity & real-attack checks, and exhaustive pure
// soundness sweeps for Sherlock T8b GP-2 (chip set 2, "the value set", OpenSpec CORE-015).

module gp2_value_set_test {
  import gp2_value_set_defs.* from "./gp2_value_set"
  import gp2_value_set(CFG = SOUND_CFG).* from "./gp2_value_set"

  // ---------------------------------------------------------------------------
  // 1. Mutant registry: (mutantId, mutantCfg, realAttackWitness, honestStillAccepted)
  // ---------------------------------------------------------------------------
  type MutantEntry = {
    mId: int,
    mCfg: Cfg,
    mAtk: Witness,
    honestOk: bool,
  }

  pure val ALL_MUTANTS: List[MutantEntry] = List(
    // Q2: R_CNULL guards (1..14)
    { mId: 1,  mCfg: { ...SOUND_CFG, cnullRunning: false },    mAtk: atkCnullNoRunning,    honestOk: true },
    { mId: 2,  mCfg: { ...SOUND_CFG, cnullNoHlp: false },      mAtk: atkCnullNoHlp,        honestOk: true },
    { mId: 3,  mCfg: { ...SOUND_CFG, cnullNoRes: false },      mAtk: atkCnullNoRes,        honestOk: true },
    { mId: 4,  mCfg: { ...SOUND_CFG, cnullCtlEval: false },    mAtk: atkCnullNoCtlEval,    honestOk: true },
    { mId: 5,  mCfg: { ...SOUND_CFG, cnullExprKind: false },   mAtk: atkCnullNoExprKind,   honestOk: true },
    { mId: 6,  mCfg: { ...SOUND_CFG, cnullExprArity: false },  mAtk: atkCnullNoExprArity,  honestOk: true },
    { mId: 7,  mCfg: { ...SOUND_CFG, cnullHasConst: false },   mAtk: atkCnullNoHasConst,   honestOk: true },
    { mId: 8,  mCfg: { ...SOUND_CFG, cnullCtorType: false },   mAtk: atkCnullNoCtorType,   honestOk: true },
    { mId: 9,  mCfg: { ...SOUND_CFG, cnullCtorArity: false },  mAtk: atkCnullNoCtorArity,  honestOk: true },
    { mId: 10, mCfg: { ...SOUND_CFG, cnullCstKind: false },    mAtk: atkCnullNoCstKind,    honestOk: true },
    { mId: 11, mCfg: { ...SOUND_CFG, cnullMarkerFree: false }, mAtk: atkCnullNoMarkerFree, honestOk: true },
    { mId: 12, mCfg: { ...SOUND_CFG, cnullCstFields: false },  mAtk: atkCnullNoCstFields,  honestOk: true },
    { mId: 13, mCfg: { ...SOUND_CFG, cnullCidBind: false },    mAtk: atkCnullNoCidBind,    honestOk: true },
    // Mutant 14 (cnull_no_rom) drops R_CNULL's ROM bus sends, changing honest RomAir C_MULT from [1,1,2,0] to [0,0,1,0]
    { mId: 14, mCfg: { ...SOUND_CFG, cnullRom: false },        mAtk: atkCnullNoRom,        honestOk: false },
    // Q2: R_PUBV guards (15..23)
    { mId: 15, mCfg: { ...SOUND_CFG, pubvRunning: false },     mAtk: atkPubvNoRunning,     honestOk: true },
    { mId: 16, mCfg: { ...SOUND_CFG, pubvNoHlp: false },       mAtk: atkPubvNoHlp,         honestOk: true },
    { mId: 17, mCfg: { ...SOUND_CFG, pubvNoRes: false },       mAtk: atkPubvNoRes,         honestOk: true },
    { mId: 18, mCfg: { ...SOUND_CFG, pubvCtlRet: false },      mAtk: atkPubvNoCtlRet,      honestOk: true },
    { mId: 19, mCfg: { ...SOUND_CFG, pubvValConst: false },    mAtk: atkPubvNoValConst,    honestOk: true },
    { mId: 20, mCfg: { ...SOUND_CFG, pubvCidHi: false },       mAtk: atkPubvNoCidHi,       honestOk: true },
    { mId: 21, mCfg: { ...SOUND_CFG, pubvFrmZero: false },     mAtk: atkPubvNoFrmZero,     honestOk: true },
    { mId: 22, mCfg: { ...SOUND_CFG, pubvCidBind: false },     mAtk: atkPubvNoCidBind,     honestOk: true },
    // Mutant 23 (pubv_no_rom) drops R_PUBV's ROM bus send, changing honest RomAir C_MULT from [1,1,2,0] to [1,1,1,0]
    { mId: 23, mCfg: { ...SOUND_CFG, pubvRom: false },         mAtk: atkPubvNoRom,         honestOk: false },
    // Q1, Q2 effects, Q3 frame, Q4/Q5 INIT & ROM typing (24..32)
    { mId: 24, mCfg: { ...SOUND_CFG, padOffBus: false },       mAtk: atkPadOnBus,          honestOk: true },
    { mId: 25, mCfg: { ...SOUND_CFG, selBool: false },         mAtk: atkSelNoBool,         honestOk: true },
    { mId: 26, mCfg: { ...SOUND_CFG, selSum: false },          mAtk: atkSelSumFree,        honestOk: true },
    // Mutant 27 (key_wrong_class) swaps [119, 2] to [2, 119] in C_KEY, changing the honest witness C_KEY layout
    { mId: 27, mCfg: { ...SOUND_CFG, keyRightClass: false },   mAtk: atkKeyWrongClass,     honestOk: false },
    { mId: 28, mCfg: { ...SOUND_CFG, pubvPayloadVd: false },   mAtk: atkPubvPayloadFree,   honestOk: true },
    { mId: 29, mCfg: { ...SOUND_CFG, cnullCtlEffect: false },  mAtk: atkCnullCtlFree,      honestOk: true },
    { mId: 30, mCfg: { ...SOUND_CFG, frameKeepAll: false },    mAtk: atkFrameSkipWord,     honestOk: true },
    { mId: 31, mCfg: { ...SOUND_CFG, initValOnlySet2: false }, mAtk: atkInitValAnySet,     honestOk: true },
    { mId: 32, mCfg: { ...SOUND_CFG, romTblBind: false },      mAtk: atkRomNoTblBind,      honestOk: true }
  )

  // ---------------------------------------------------------------------------
  // 2. Baseline valid witnesses under SOUND_CFG (stepping through all 5 valid branches)
  // ---------------------------------------------------------------------------
  run validWitnessesAcceptedAndRefineTest =
    init
      .expect(acc and invRefine and refines(honestWitness))
      .then(pickCandidateWitness(1))
      .expect(acc and invRefine and w == validCnullOnlyWitness)
      .then(pickCandidateWitness(2))
      .expect(acc and invRefine and w == validPubvOnlyWitness)
      .then(pickCandidateWitness(3))
      .expect(acc and invRefine and w == validConsumerWitness)
      .then(pickCandidateWitness(4))
      .expect(acc and invRefine and w == validChipSet0Witness)

  // ---------------------------------------------------------------------------
  // 3. Mutant non-vacuity: honest witness refines under every non-layout mutant,
  //    multiplicity-rebalanced honest witness refines under the 2 ROM-bus mutants,
  //    and every mutant is caught by a REAL attack (accepted by mutant, rejected by
  //    SOUND_CFG, and violates refines).
  // ---------------------------------------------------------------------------
  run honestWitnessUnderAllMutantsTest = init.expect({
    val cs0 = honestWitness.slotR3.nth(0)
    val honestCnullNoRom = {
      ...honestWitness,
      nrom: 1,
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW, INACTIVE_ROM_ROW, INACTIVE_ROM_ROW
      ),
    }
    val honestPubvNoRom = {
      ...honestWitness,
      romRows: List(
        honestWitness.romRows.nth(0),
        honestWitness.romRows.nth(1),
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW
      ),
    }
    and {
      ALL_MUTANTS.indices().forall(i => {
        val m = ALL_MUTANTS.nth(i)
        (evalAcceptedCfg(m.mCfg, honestWitness) == m.honestOk) and refines(honestWitness)
      }),
      evalAcceptedCfg({ ...SOUND_CFG, cnullRom: false }, honestCnullNoRom),
      refines(honestCnullNoRom),
      evalAcceptedCfg({ ...SOUND_CFG, pubvRom: false }, honestPubvNoRom),
      refines(honestPubvNoRom),
    }
  })

  run all32MutantsCaughtByRealAttackTest = init.expect(and {
    ALL_MUTANTS.length() == 32,
    ALL_MUTANTS.indices().forall(i => {
      val m = ALL_MUTANTS.nth(i)
      and {
        not(evalAcceptedCfg(SOUND_CFG, m.mAtk)),
        evalAcceptedCfg(m.mCfg, m.mAtk),
        not(refines(m.mAtk)),
      }
    }),
  })

  // ---------------------------------------------------------------------------
  // 4. Q1 Exhaustive pure checks: Dispatch, DEC padding, rule-swap, idle, and
  //    the algebraic redundancy theorem for core.rs:324 over F_127.
  // ---------------------------------------------------------------------------
  run q1PolynomialSelectorBooleanTheoremTest = all {
    init,
    // Theorem (F1.5): Over prime field F_127, even if assert_bool(sel(r)) at core.rs:324
    // is omitted (`selOnlyBool: false`), the constraints ssum == en (core.rs:333),
    // sel(0) * st.ctl0 == 0 (core.rs:399), sel(1) * (st.ctl0 - 1) == 0 (core.rs:434),
    // and frame_rule (core.rs:782-783) force (sel(0), sel(1)) to be boolean!
    assert(0.to(FP - 1).forall(s0 =>
      Set(0, 1).forall(en =>
        Set(0, 1, 2, 51, 77, 126).forall(ctl0 => {
          val s1 = fSub(en, s0)
          val guardsHold = (fMul(s0, ctl0) == 0) and (fMul(s1, fSub(ctl0, 1)) == 0)
          guardsHold implies (isBool(s0) and isBool(s1))
        })
      )
    )),
    // Verify that dropping core.rs:324 ALONE (`selOnlyBool: false`, keeping polynomial gZero)
    // still rejects atkSelNoBool, whereas naive `sel == 1` gating (`selBool: false`) accepts it:
    assert(not(evalAcceptedCfg({ ...SOUND_CFG, selOnlyBool: false }, atkSelNoBool))),
    assert(evalAcceptedCfg({ ...SOUND_CFG, selBool: false }, atkSelNoBool)),
  }

  run q1ExhaustiveDispatchAndDecPaddingSoundTest = all {
    init,
    // Exhaustively sweep slot 0 dispatch & DEC padding columns over finite tampering domain:
    //   en0 in {0, 1, 2, 95}, s0 in {0, 1, 51, 95}, s1 in {0, 1, 77, 123},
    //   s2 (chip set 0 selector index 2) in {0, 1}, key0 in {0, 2, 89, 119},
    //   padMult3 in {0, 1}
    assert(
      Set(0, 1, 2, 95).forall(en0 =>
        Set(0, 1, 51, 95).forall(s0 =>
          Set(0, 1, 77, 123).forall(s1 =>
            Set(0, 1).forall(s2 =>
              Set(0, 2, 89, 119).forall(key0 =>
                Set(0, 1).forall(padM => {
                  val base = validCnullOnlyWitness
                  val wTry = {
                    ...base,
                    slotEn: base.slotEn.replaceAt(0, en0),
                    slotSel: base.slotSel.replaceAt(0, List(s0, s1, s2, 0, 0)),
                    slotKey: base.slotKey.replaceAt(0, key0),
                    rids: base.rids.replaceAt(0, key0),
                    rulesCol: hashRules(base.rids.replaceAt(0, key0)),
                    decMult: base.decMult.replaceAt(3, padM),
                    decPadMsg: { dCls: key0, fl0: 1, fl1: 0, fl2: 1 },
                    pvs: {
                      ...base.pvs,
                      steps: en0,
                      rulesD: hashRules(base.rids.replaceAt(0, key0)),
                    },
                  }
                  evalAcceptedCfg(SOUND_CFG, wTry) implies and {
                    en0 == 1,
                    s0 == 1,
                    s1 == 0,
                    s2 == 0,
                    key0 == CLS_CNULL,
                    padM == 0,
                    refines(wTry),
                  }
                })
              )
            )
          )
        )
      )
    ),
  }

  run q1NoRuleSwapAndNoIdleStateChangeSoundTest = all {
    init,
    // 1. Firing R_PUBV where R_CNULL is required (at Eval{e} genesis across all 10 programs) is rejected:
    assert(ALL_PROGS.forall(p => {
      val s0 = makeGenesisState(p, VAL_NONE, 1, 4, 8)
      val s1 = { ...s0, ot: OT_HALTED, op: VD_CTOR0 }
      val rids = List(CLS_PUBV, CLS_IDLE, CLS_IDLE)
      val cs = cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0)
      val imgD = hashImage(p.progId, p.lv, p.pid, p.entryA)
      val wSwap = {
        ...honestWitness,
        stRows: List(s0, s1, s1, s1),
        slotEn: List(1, 0, 0),
        slotSel: List(List(0, 1, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
        slotKey: rids,
        slotFl: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
        rids: rids,
        rulesCol: hashRules(rids),
        nrom: 1,
        initW: {
          ...honestWitness.initW,
          lv: p.lv, pid: p.pid, mv: p.mv, ea: p.entryA,
          planD: p.planD, kr: p.keyRoot, imgCol: imgD,
        },
        decMult: List(2, 0, 1, 0, 0, 0, 0, 0),
        romRows: List(
          { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs, rootOut: p.progId },
          INACTIVE_ROM_ROW, INACTIVE_ROM_ROW, INACTIVE_ROM_ROW
        ),
        pvs: {
          progD: progDigestOf(p),
          preD: hashState(s0),
          postD: hashState(s1),
          rulesD: hashRules(rids),
          hPre: 0,
          steps: 1,
        },
      }
      not(evalAcceptedCfg(SOUND_CFG, wSwap))
    })),
    // 2. Firing R_CNULL where R_PUBV is required (at Return{Const(0)}) is rejected:
    assert(not(evalAcceptedCfg(SOUND_CFG, atkCnullNoCtlEval))),
  }

  // ---------------------------------------------------------------------------
  // 5. Q2 & Q3 Exhaustive pure checks: All 18 RowState fields across R_CNULL,
  //    R_PUBV, and Idle (every written word pinned; every unwritten word framed).
  // ---------------------------------------------------------------------------
  pure def mutateRowField(s: RowState, fieldIdx: int): RowState = {
    if (fieldIdx == 0)       { ...s, ctl0: 2 }
    else if (fieldIdx == 1)  { ...s, ctl1: 3 }
    else if (fieldIdx == 2)  { ...s, ctl2: 2 }
    else if (fieldIdx == 3)  { ...s, ctl2Hi: 1 }
    else if (fieldIdx == 4)  { ...s, ctlAux: 5 }
    else if (fieldIdx == 5)  { ...s, hlp0: 2 }
    else if (fieldIdx == 6)  { ...s, res0: 3 }
    else if (fieldIdx == 7)  { ...s, frmCount: 1 }
    else if (fieldIdx == 8)  { ...s, regAux: 9 }
    else if (fieldIdx == 9)  { ...s, rem: if (s.rem == 2) 1 else 2 }
    else if (fieldIdx == 10) { ...s, con: if (s.con == 2) 3 else 2 }
    else if (fieldIdx == 11) { ...s, hw: 1 }
    else if (fieldIdx == 12) { ...s, rcur: if (s.rcur == 2) 3 else 2 }
    else if (fieldIdx == 13) { ...s, ot: OT_REJECTED }
    else if (fieldIdx == 14) { ...s, op: VAL_FORGED }
    else if (fieldIdx == 15) { ...s, inD: hashInput(9, VAL_FORGED, 4, 8) }
    else if (fieldIdx == 16) { ...s, progRoot: 99 }
    else                     { ...s, progD: progDigestOf(PROG_V) }
  }

  run q2AndQ3ExhaustiveStateWordPartitionSoundTest = all {
    init,
    // Across all 18 RowState fields:
    //  (a) Mutating ANY field of post-row s1 on validCnullOnlyWitness (slot 0 = R_CNULL) is rejected;
    //  (b) Mutating ANY field of post-row s1 on validPubvOnlyWitness (slot 0 = R_PUBV) is rejected;
    //  (c) Mutating ANY field of idle post-row s3 on honestWitness (slot 2 = Idle) is rejected!
    assert(0.to(17).forall(fIdx => {
      val s1CnullBad = mutateRowField(validCnullOnlyWitness.stRows.nth(1), fIdx)
      val wCnullBad = {
        ...validCnullOnlyWitness,
        stRows: List(validCnullOnlyWitness.stRows.nth(0), s1CnullBad, s1CnullBad, s1CnullBad),
        pvs: { ...validCnullOnlyWitness.pvs, postD: hashState(s1CnullBad) },
      }

      val s1PubvBad = mutateRowField(validPubvOnlyWitness.stRows.nth(1), fIdx)
      val wPubvBad = {
        ...validPubvOnlyWitness,
        stRows: List(validPubvOnlyWitness.stRows.nth(0), s1PubvBad, s1PubvBad, s1PubvBad),
        pvs: { ...validPubvOnlyWitness.pvs, postD: hashState(s1PubvBad) },
      }

      val s3IdleBad = mutateRowField(honestWitness.stRows.nth(3), fIdx)
      val wIdleBad = {
        ...honestWitness,
        stRows: List(
          honestWitness.stRows.nth(0),
          honestWitness.stRows.nth(1),
          honestWitness.stRows.nth(2),
          s3IdleBad
        ),
        pvs: { ...honestWitness.pvs, postD: hashState(s3IdleBad) },
      }

      and {
        not(evalAcceptedCfg(SOUND_CFG, wCnullBad)),
        not(evalAcceptedCfg(SOUND_CFG, wPubvBad)),
        not(evalAcceptedCfg(SOUND_CFG, wIdleBad)),
      }
    })),
  }

  run q2ExhaustiveProgramAndFuelPreconditionSweepTest = all {
    init,
    // Exhaustively test R_CNULL genesis execution across all 10 programs in ALL_PROGS:
    // ONLY PROG_U and PROG_V have valid nullary marker-free Construct at expressions[0];
    // all 8 invalid programs (PROG_VAR..PROG_BAD_CST_FIELDS) MUST be rejected by SOUND_CFG!
    assert(ALL_PROGS.forall(p => {
      val wProg = cnullGenesisOnProg(p)
      val accepted = evalAcceptedCfg(SOUND_CFG, wProg)
      val shouldAccept = (p == PROG_U or p == PROG_V)
      (accepted == shouldAccept) and (accepted implies refines(wProg))
    })),
    // Also verify fuel exhaustion (rem = 0) and counter overflow (con = 15 or rcur = 15) on R_CNULL
    // are rejected by checkCharges (core.rs:547-566):
    assert(Set(0, 1, 2).forall(mode => {
      val s0Base = honestWitness.stRows.nth(0)
      val s0Bad = if (mode == 0) { ...s0Base, rem: 0, inD: hashInput(1, VAL_NONE, 0, 8) }
                  else if (mode == 1) { ...s0Base, con: LIMB_MAX - 1 }
                  else { ...s0Base, rcur: LIMB_MAX - 1 }
      val s1Try = {
        ...s0Bad,
        ctl0: CTL_RET, ctl1: 0, ctl2: 0,
        rem: fSub(s0Bad.rem, 1),
        con: fAdd(s0Bad.con, 1),
        rcur: fAdd(s0Bad.rcur, 1),
      }
      val wBadFuel = {
        ...validCnullOnlyWitness,
        stRows: List(s0Bad, s1Try, s1Try, s1Try),
        initW: { ...validCnullOnlyWitness.initW, gi: 0, gFlag: 0, hInv: 1 },
        pvs: {
          ...validCnullOnlyWitness.pvs,
          preD: hashState(s0Bad),
          postD: hashState(s1Try),
          hPre: 1,
        },
      }
      not(evalAcceptedCfg(SOUND_CFG, wBadFuel))
    })),
  }

  // ---------------------------------------------------------------------------
  // 6. Q4 (Q3 INIT val_d & Q5 const_vd / ROM typing) Exhaustive pure checks
  // ---------------------------------------------------------------------------
  run q4ExhaustiveInitValAndRomTypingSoundTest = all {
    init,
    // 1. Exhaustive sweep over chipSet in {0, 2}, vvalWitness in {VAL_NONE, VD_CTOR0, VAL_FORGED},
    //    and stateVal in {VAL_NONE, VD_CTOR0, VAL_FORGED} at genesis (hPre == 0):
    assert(
      Set(VAL_NONE, VD_CTOR0, VAL_FORGED).forall(wVd =>
        Set(VAL_NONE, VD_CTOR0, VAL_FORGED).forall(sVd => {
          // Chip set 0 genesis: accepted iff state has VAL_NONE (wVd column is ignored by core.rs:664!)
          val wSet0 = {
            ...buildChipSet0Genesis(wVd, true),
            stRows: List(
              makeGenesisState(PROG_VAR, sVd, 1, 4, 8),
              { ...makeGenesisState(PROG_VAR, sVd, 1, 4, 8), hlp0: 2, rem: 3, con: 1, hw: 1 },
              { ...makeGenesisState(PROG_VAR, sVd, 1, 4, 8), hlp0: 2, rem: 3, con: 1, hw: 1 },
              { ...makeGenesisState(PROG_VAR, sVd, 1, 4, 8), hlp0: 2, rem: 3, con: 1, hw: 1 }
            ),
            pvs: {
              ...buildChipSet0Genesis(wVd, true).pvs,
              preD: hashState(makeGenesisState(PROG_VAR, sVd, 1, 4, 8)),
              postD: hashState({ ...makeGenesisState(PROG_VAR, sVd, 1, 4, 8), hlp0: 2, rem: 3, con: 1, hw: 1 }),
            },
          }
          // Chip set 2 genesis: accepted iff sVd == wVd (bound by SHK_IN sponge opening at core.rs:662!)
          val base2 = buildValueShard2Step(PROG_V, sVd)
          val wSet2 = { ...base2, initW: { ...base2.initW, vval: wVd } }

          and {
            evalAcceptedCfg(SOUND_CFG, wSet0) == (sVd == VAL_NONE),
            evalAcceptedCfg(SOUND_CFG, wSet0) implies refines(wSet0),
            evalAcceptedCfg(SOUND_CFG, wSet2) == (sVd == wVd),
            evalAcceptedCfg(SOUND_CFG, wSet2) implies refines(wSet2),
          }
        })
      )
    ),
    // 2. Exhaustive ROM table-tag relabelling sweep across all 6 tables {0, 9, 11, 15, 19, 21}:
    assert(Set(TBL_EXPR, TBL_TY, TBL_CTOR, TBL_DECL, TBL_CST, TBL_NAME).forall(wrongTbl => {
      val ex = honestWitness.slotRom.nth(0)
      val cs0 = honestWitness.slotR3.nth(0)
      val s1 = honestWitness.stRows.nth(1)
      val s2Bad = { ...s1, ot: OT_HALTED, op: OP_ZERO }
      val wRelabel = {
        ...honestWitness,
        stRows: List(honestWitness.stRows.nth(0), s1, s2Bad, s2Bad),
        slotR3: List(cs0, ex, cs0),
        nrom: 4,
        romRows: List(
          honestWitness.romRows.nth(0),
          honestWitness.romRows.nth(1),
          { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
          { act: 1, mult: 1, tbl: wrongTbl, rowIdx: 0, rec: ex, rootOut: PROG_U.progId }
        ),
        pvs: { ...honestWitness.pvs, postD: hashState(s2Bad) },
      }
      not(evalAcceptedCfg(SOUND_CFG, wRelabel))
    })),
    // 3. Demonstrate Finding F4.2 (prose vs code on ty_d / decl_d):
    //    PROG_U (`type U { A(); }`) and PROG_V (`type V { A(); }`) have distinct program digests,
    //    yet both halt with the EXACT SAME outcome_payload VD_CTOR0 because leaf.rs:269 computes
    //    ty_d = P2(DOM_TY, 0) from the table-9 row index alone, ignoring decl_d and names!
    assert(progDigestOf(PROG_U) != progDigestOf(PROG_V)),
    assert(honestWitness.stRows.nth(S_SLOTS).op == validConsumerWitness.stRows.nth(S_SLOTS).op),
  }

  // ---------------------------------------------------------------------------
  // 7. Exhaustive check over ALL candidate witnesses and ALL single-cell raw tampers
  //    (37 + 37 * 6 * 7 = 1,591 witnesses)
  // ---------------------------------------------------------------------------
  run exhaustiveCandidateAndTamperSuiteSoundTest = all {
    init,
    assert(ALL_CANDIDATE_WITNESSES.indices().forall(idx => {
      val cand = ALL_CANDIDATE_WITNESSES.nth(idx)
      val candSound = evalAcceptedCfg(SOUND_CFG, cand) implies refines(cand)
      val tampersSound = 0.to(5).forall(k =>
        Set(0, 1, 2, 3, 89, 95, 119).forall(vp => {
          val wT = applyRawTamper(cand, k, vp)
          evalAcceptedCfg(SOUND_CFG, wT) implies refines(wT)
        })
      )
      candSound and tampersSound
    })),
  }
}
