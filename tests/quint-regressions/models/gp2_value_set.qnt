// Soundness model of Sherlock T8b GP-2: chip set 2 ("the value set", OpenSpec CORE-015).
// Models code AS BUILT in lane-src/sherlock-leaf/:
//   - dec.rs:40-84, 116-152 (RULES_V, row_of, DecVAir eval_table, padding off bus)
//   - core.rs:246, 308-361, 380-461, 544-681, 735-785 (CoreAir::value dispatch, R_CNULL, R_PUBV,
//     charges, BOUNDARY, ROM root port, INIT witnessed val_d, frame_rule)
//   - rom.rs:55-113, 128-167 (RomAir table/row bit binding, leaf/root port, ROM bus)
//   - leaf.rs:206-273, 530-680 (rom_rows, const_vd, ty_d, airs_value, program_v, traces_value)

module gp2_value_set_defs {
  // ---------------------------------------------------------------------------
  // 1. Small prime field F_127 (127 is prime, > 119 = max rule class ordinal)
  // ---------------------------------------------------------------------------
  pure val FP: int = 127
  pure val LIMB_MAX: int = 16

  pure def fMod(x: int): int = ((x % FP) + FP) % FP
  pure def fAdd(a: int, b: int): int = fMod(a + b)
  pure def fSub(a: int, b: int): int = fMod(a - b)
  pure def fMul(a: int, b: int): int = fMod(a * b)
  pure def isBool(x: int): bool = (x == 0 or x == 1)
  pure def inRange(x: int): bool = (x >= 0 and x < LIMB_MAX)

  // ---------------------------------------------------------------------------
  // 2. Architectural & domain constants (matching enc.rs, dec.rs, core.rs)
  // ---------------------------------------------------------------------------
  pure val S_SLOTS: int = 3
  pure val CORE_HEIGHT: int = 5     // S_SLOTS + 1 state rows (0..3) + 1 padding row (4)
  pure val DEC_HEIGHT: int = 8      // dec.rs:64 HEIGHT = 8 (0=idle, 1..2=rules, 3..7=padding)
  pure val ROM_HEIGHT: int = 4      // leaf.rs:532 H_ROM_V = 4

  // Rule selector indices and classes (dec.rs:40-43, 23-34)
  pure val R_CNULL: int = 0
  pure val R_PUBV: int = 1
  pure val CLS_IDLE: int = 0
  pure val CLS_CNULL: int = 119
  pure val CLS_PUBV: int = 2
  pure val CLS_C0_ENTRY: int = 89

  // ROM table IDs (leaf.rs:207-230)
  pure val TBL_EXPR: int = 0
  pure val TBL_TY: int = 9
  pure val TBL_CTOR: int = 11
  pure val TBL_DECL: int = 15
  pure val TBL_CST: int = 19
  pure val TBL_NAME: int = 21

  // Control tags (core.rs:399, 419, 434), outcome tags (core.rs:396, 441, 531), kinds
  pure val CTL_EVAL: int = 0
  pure val CTL_RET: int = 1
  pure val CTL_ERR: int = 9

  pure val OT_RUNNING: int = 0
  pure val OT_HALTED: int = 1
  pure val OT_REJECTED: int = 2

  pure val EXPR_VAR: int = 1
  pure val EXPR_CONSTRUCT: int = 8
  pure val CST_CONSTRUCTOR: int = 6

  // FP-4 domain tags (enc.rs:18-38, leaf.rs:228-272)
  pure val DOM_PROG: int = 5
  pure val SHK_IMAGE: int = 12
  pure val SHK_VALUE_V2: int = 13
  pure val SHK_IN: int = 15
  pure val SHK_SD2: int = 29
  pure val DOM_STATE_BODY: int = 30
  pure val DOM_ROMLEAF: int = 39
  pure val DOM_TY: int = 45
  pure val SHK_RULES: int = 64

  // ---------------------------------------------------------------------------
  // 3. Injective tuple digests (no hash collisions)
  // ---------------------------------------------------------------------------
  // Value digest: (domTag, vForm, vArity, tyDom, tyRow, srcOrd)
  // Matches leaf.rs:263-273 const_vd and enc.rs:306 val_none / zero payload
  type ValDigest = (int, int, int, int, int, int)

  pure val OP_ZERO: ValDigest = (0, 0, 0, 0, 0, 0)
  pure val VAL_NONE: ValDigest = (SHK_VALUE_V2, 0, 0, 0, 0, 0)
  pure val VAL_FORGED: ValDigest = (99, 9, 9, 9, 9, 9)

  // leaf.rs:241, 269: builtin slice PICK ty_d = P2(DOM_TY, row)
  pure def tyDigest(tyRow: int): (int, int) = (DOM_TY, tyRow)

  // leaf.rs:263-273: const_vd = P2(SHK_VALUE_V2, 2, form=6, m, ty_d, c=srcOrd, acc_0=0)
  pure def makeConstVd(mArity: int, tyRow: int, srcOrd: int): ValDigest = {
    val td = tyDigest(tyRow)
    (SHK_VALUE_V2, CST_CONSTRUCTOR, mArity, td._1, td._2, srcOrd)
  }

  // Input digest: P2(SHK_IN, salt, val_d, fuel, stack_limit) (enc.rs:370-380, core.rs:659-670)
  type InputDigest = {
    inDom: int,
    inSalt: int,
    inValD: ValDigest,
    inFuel: int,
    inSl: int,
  }

  pure def hashInput(salt: int, vd: ValDigest, fuel: int, sl: int): InputDigest = {
    inDom: SHK_IN,
    inSalt: salt,
    inValD: vd,
    inFuel: fuel,
    inSl: sl,
  }

  // Image & Program digests: P2(SHK_IMAGE, ...) and P2(DOM_PROG, ...) (enc.rs:348-369)
  type ImageDigest = (int, int, int, int, int)
  type ProgDigest = (int, ImageDigest, int, int, int)

  pure def hashImage(progRoot: int, lv: int, pid: int, entryA: int): ImageDigest =
    (SHK_IMAGE, progRoot, lv, pid, entryA)

  pure def hashProg(imgD: ImageDigest, planD: int, keyRoot: int, mv: int): ProgDigest =
    (DOM_PROG, imgD, planD, keyRoot, mv)

  // Rules digest: P2(SHK_RULES, rids[0..S-1]) (core.rs:348-356)
  type RulesDigest = (int, List[int])

  pure def hashRules(rids: List[int]): RulesDigest = (SHK_RULES, rids)

  // ---------------------------------------------------------------------------
  // 4. Uniform ROM rows and Program catalog
  // ---------------------------------------------------------------------------
  // Uniform 33-word ROM record abstraction across tables 0, 11, 19, 15, 21 (enc.rs:395):
  //   Table 0  (expr): w0=kind, w1=ty, w2=ctor, w3=arity, w4=hasConst, w5=cid, vd=OP_ZERO
  //   Table 11 (ctor): w0=ownerTy, w1=nameIdx, w2=srcOrd, w3=fStart, w4=arity, w5=0, vd=OP_ZERO
  //   Table 19 (cst):  w0=kind, w1=markerFree, w2=ty, w3=tyHi, w4=ctor, w5=arity, vd=const_vd
  //   Table 15/21:     w0=byteTag, others 0, vd=OP_ZERO
  type RomRec = {
    w0: int,
    w1: int,
    w2: int,
    w3: int,
    w4: int,
    w5: int,
    vd: ValDigest,
  }

  type RomEntry = {
    tbl: int,
    rowIdx: int,
    rec: RomRec,
  }

  type ProgramSpec = {
    progId: int,       // injective Merkle root identifier of romEntries
    entryA: int,
    lv: int,
    pid: int,
    planD: int,
    keyRoot: int,
    mv: int,
    romEntries: Set[RomEntry],
  }

  pure def progDigestOf(p: ProgramSpec): ProgDigest =
    hashProg(hashImage(p.progId, p.lv, p.pid, p.entryA), p.planD, p.keyRoot, p.mv)

  pure def exprRec(kind: int, ty: int, ctor: int, arity: int, hasConst: int, cid: int): RomRec =
    { w0: kind, w1: ty, w2: ctor, w3: arity, w4: hasConst, w5: cid, vd: OP_ZERO }

  pure def ctorRec(ownerTy: int, nameIdx: int, srcOrd: int, arity: int): RomRec =
    { w0: ownerTy, w1: nameIdx, w2: srcOrd, w3: 0, w4: arity, w5: 0, vd: OP_ZERO }

  pure def cstRec(kind: int, mf: int, ty: int, tyHi: int, ctor: int, arity: int, vd: ValDigest): RomRec =
    { w0: kind, w1: mf, w2: ty, w3: tyHi, w4: ctor, w5: arity, vd: vd }

  pure def metaRec(byteVal: int): RomRec =
    { w0: byteVal, w1: 0, w2: 0, w3: 0, w4: 0, w5: 0, vd: OP_ZERO }

  // VD of U::A() (ty=0, srcOrd=0) and a second constructor U::B() (ty=0, srcOrd=1)
  pure val VD_CTOR0: ValDigest = makeConstVd(0, 0, 0)
  pure val VD_CTOR1: ValDigest = makeConstVd(0, 0, 1)

  // Program 1: Producer P (`type U { A(); B(); } entry construct U::A();`, leaf.rs:551-617)
  // Includes a second constant cid=1 (U::B, srcOrd=1) to test cid-binding attacks, and
  // a third constant cid=2 (same U::A) to test 1-step cid-binding without changing cst fields.
  pure val PROG_U: ProgramSpec = {
    progId: 1,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CTOR, rowIdx: 1, rec: ctorRec(0, 2, 1, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) },
      { tbl: TBL_CST,  rowIdx: 1, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 1, 0, VD_CTOR1) },
      { tbl: TBL_CST,  rowIdx: 2, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) },
      { tbl: TBL_DECL, rowIdx: 0, rec: metaRec(85) },
      { tbl: TBL_NAME, rowIdx: 1, rec: metaRec(85) }
    ),
  }

  // Program 2: Consumer C (`type V { A(); } entry construct V::A();`, leaf.rs:564, 578-582)
  // Names/decl changed from 'U' (85) to 'V' (86), so progId (prog_root) and prog_d differ,
  // while const_vd uses ty_d = P2(DOM_TY, 0) = VD_CTOR0.
  pure val PROG_V: ProgramSpec = {
    progId: 2,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) },
      { tbl: TBL_DECL, rowIdx: 0, rec: metaRec(86) },
      { tbl: TBL_NAME, rowIdx: 1, rec: metaRec(86) }
    ),
  }

  // Program 3: `unbound` (`entry x;`, expressions[0] is Var, leaf.rs:385)
  pure val PROG_VAR: ProgramSpec = {
    progId: 3,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_VAR, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 4: Non-nullary Construct expression (expr.arity = 1 != 0)
  pure val PROG_EXPR_ARITY1: ProgramSpec = {
    progId: 4,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 1, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 5: Construct expression with constant = None (hasConst = 0)
  pure val PROG_NO_CONST: ProgramSpec = {
    progId: 5,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 0, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 6: Constructor owner_type mismatch (ctor.ownerTy = 1 != expr.ty = 0)
  pure val PROG_BAD_CTOR_TY: ProgramSpec = {
    progId: 6,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(1, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 7: Constructor arity = 1 != 0 in table 11
  pure val PROG_BAD_CTOR_ARITY: ProgramSpec = {
    progId: 7,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 1) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 8: Constant row has non-Constructor kind (kind = 1 != 6)
  pure val PROG_BAD_CST_KIND: ProgramSpec = {
    progId: 8,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(1, 1, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 9: Constant row has marker_free = 0 (marker-bearing ADT, cannot Constant-construct)
  pure val PROG_NOT_MARKER_FREE: ProgramSpec = {
    progId: 9,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 0, 0, 0, 0, 0, VD_CTOR0) }
    ),
  }

  // Program 10: Constant row fields mismatch expression (cst.ctor = 1 != expr.ctor = 0)
  pure val PROG_BAD_CST_FIELDS: ProgramSpec = {
    progId: 10,
    entryA: 0,
    lv: 1, pid: 1, planD: 10, keyRoot: 20, mv: 1,
    romEntries: Set(
      { tbl: TBL_EXPR, rowIdx: 0, rec: exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0) },
      { tbl: TBL_CTOR, rowIdx: 0, rec: ctorRec(0, 1, 0, 0) },
      { tbl: TBL_CST,  rowIdx: 0, rec: cstRec(CST_CONSTRUCTOR, 1, 0, 0, 1, 0, VD_CTOR1) }
    ),
  }

  pure val ALL_PROGS: Set[ProgramSpec] = Set(
    PROG_U, PROG_V, PROG_VAR, PROG_EXPR_ARITY1, PROG_NO_CONST,
    PROG_BAD_CTOR_TY, PROG_BAD_CTOR_ARITY, PROG_BAD_CST_KIND,
    PROG_NOT_MARKER_FREE, PROG_BAD_CST_FIELDS
  )

  // ---------------------------------------------------------------------------
  // 5. Machine State, Public Values, and Full Shard Witness
  // ---------------------------------------------------------------------------
  type RowState = {
    ctl0: int,       // ctl.0: Tag(4) (0=Eval, 1=Return, 9=Error)
    ctl1: int,       // ctl.1: U64 (expr index e for Eval; val kind 0=Const for Return)
    ctl2: int,       // ctl.2[0..4]: low u64 of Addr (cid for Return{Const(cid)})
    ctl2Hi: int,     // ctl.2[4]: top limb of Addr (must be 0 for Const(cid))
    ctlAux: int,     // ctl.3..16: remaining control words
    hlp0: int,       // hlp.0: helper tag (0=None, 2=EnvGet)
    res0: int,       // res.0: result tag (0=None, 3=EnvMissing)
    frmCount: int,   // frm.count: caller frame count
    regAux: int,     // representative unwritten body word (e.g. reg.19 = entry_a)
    rem: int,        // reg.27: remaining_work counter
    con: int,        // reg.28: consumed_work counter
    hw: int,         // reg.29: helper_work counter
    rcur: int,       // reg.31: reference_cursor counter
    ot: int,         // header outcome_tag (0=Running, 1=Halted, 2=Rejected)
    op: ValDigest,   // header outcome_payload
    inD: InputDigest,// header input_d
    progRoot: int,   // body prog_root
    progD: ProgDigest,// body prog_d
  }

  // State digest: P2(SHK_SD2, header, P2(DOM_STATE_BODY, SCHEMA_ID, body)) (enc.rs:274-293)
  type StateDigest = (int, RowState)
  pure def hashState(s: RowState): StateDigest = (SHK_SD2, s)

  type PublicValues = {
    progD: ProgDigest,
    preD: StateDigest,
    postD: StateDigest,
    rulesD: RulesDigest,
    hPre: int,
    steps: int,
  }

  type DecMsg = {
    dCls: int,
    fl0: int,
    fl1: int,
    fl2: int,
  }

  type RomAirRow = {
    act: int,
    mult: int,
    tbl: int,
    rowIdx: int,
    rec: RomRec,
    rootOut: int,
  }

  type InitWitness = {
    gi: int,
    gFlag: int,
    hInv: int,
    lv: int,
    pid: int,
    mv: int,
    ea: int,
    planD: int,
    kr: int,
    imgCol: ImageDigest,
    salt: int,
    fuel: int,
    sl: int,
    vval: ValDigest,
  }

  type Witness = {
    chipSet: int,                  // 2 for value set, 0 for base slice set
    stRows: List[RowState],        // length S_SLOTS + 1 = 4 (rows 0..3)
    slotEn: List[int],             // length S_SLOTS = 3
    slotSel: List[List[int]],      // length S_SLOTS = 3, each length 5 (C_SEL + 0..4)
    slotKey: List[int],            // length S_SLOTS = 3 (C_KEY)
    slotFl: List[List[int]],       // length S_SLOTS = 3, each length 3 (C_FL + 0..2)
    slotRom: List[RomRec],         // length S_SLOTS = 3 (C_ROM)
    slotR2: List[RomRec],          // length S_SLOTS = 3 (C_VR2)
    slotR3: List[RomRec],          // length S_SLOTS = 3 (C_VR3)
    slotCx: List[int],             // length S_SLOTS = 3 (C_VCIDX)
    slotBrw: List[int],            // length S_SLOTS = 3 (rem borrow bit)
    slotCc: List[int],             // length S_SLOTS = 3 (con carry bit)
    slotHc: List[int],             // length S_SLOTS = 3 (hw carry bit)
    slotRc: List[int],             // length S_SLOTS = 3 (rcur carry bit)
    rids: List[int],               // length S_SLOTS = 3 (C_RIDS)
    rulesCol: RulesDigest,         // C_RULES_D on row 0
    nrom: int,                     // C_NROM on row 0
    initW: InitWitness,            // INIT columns on row 0
    decMult: List[int],            // length DEC_HEIGHT = 8
    decPadMsg: DecMsg,             // message carried on DEC padding rows 3..7 if not gated off
    romRows: List[RomAirRow],      // length ROM_HEIGHT = 4
    pvs: PublicValues,             // leaf public values
  }

  // ---------------------------------------------------------------------------
  // 6. Constraint Configuration Record (sound = all true; each mutant flips one)
  // ---------------------------------------------------------------------------
  type Cfg = {
    // R_CNULL guards (core.rs:396-416, 446-460)
    cnullRunning: bool,
    cnullNoHlp: bool,
    cnullNoRes: bool,
    cnullCtlEval: bool,
    cnullExprKind: bool,
    cnullExprArity: bool,
    cnullHasConst: bool,
    cnullCtorType: bool,
    cnullCtorArity: bool,
    cnullCstKind: bool,
    cnullMarkerFree: bool,
    cnullCstFields: bool,
    cnullCidBind: bool,
    cnullRom: bool,
    // R_PUBV guards (core.rs:431-440, 456-460)
    pubvRunning: bool,
    pubvNoHlp: bool,
    pubvNoRes: bool,
    pubvCtlRet: bool,
    pubvValConst: bool,
    pubvCidHi: bool,
    pubvFrmZero: bool,
    pubvCidBind: bool,
    pubvRom: bool,
    // Dispatch, DEC padding, effects, frame, INIT, ROM table typing
    padOffBus: bool,
    selBool: bool,
    selOnlyBool: bool,    // separate flag for core.rs:324 alone (keeping en boolean)
    selSum: bool,
    keyRightClass: bool,
    pubvPayloadVd: bool,
    cnullCtlEffect: bool,
    frameKeepAll: bool,
    initValOnlySet2: bool,
    romTblBind: bool,
  }

  pure val SOUND_CFG: Cfg = {
    cnullRunning: true,
    cnullNoHlp: true,
    cnullNoRes: true,
    cnullCtlEval: true,
    cnullExprKind: true,
    cnullExprArity: true,
    cnullHasConst: true,
    cnullCtorType: true,
    cnullCtorArity: true,
    cnullCstKind: true,
    cnullMarkerFree: true,
    cnullCstFields: true,
    cnullCidBind: true,
    cnullRom: true,
    pubvRunning: true,
    pubvNoHlp: true,
    pubvNoRes: true,
    pubvCtlRet: true,
    pubvValConst: true,
    pubvCidHi: true,
    pubvFrmZero: true,
    pubvCidBind: true,
    pubvRom: true,
    padOffBus: true,
    selBool: true,
    selOnlyBool: true,
    selSum: true,
    keyRightClass: true,
    pubvPayloadVd: true,
    cnullCtlEffect: true,
    frameKeepAll: true,
    initValOnlySet2: true,
    romTblBind: true,
  }

  // ---------------------------------------------------------------------------
  // 7. Reference Semantics of the Owner Machine (Sherlock VM)
  // ---------------------------------------------------------------------------
  pure def findRomRec(p: ProgramSpec, tbl: int, rowIdx: int): Set[RomRec] =
    p.romEntries.filter(e => e.tbl == tbl and e.rowIdx == rowIdx).map(e => e.rec)

  pure def refInitOk(chipSet: int, p: ProgramSpec, s0: RowState): bool = and {
    s0.ctl0 == CTL_EVAL,
    s0.ctl1 == p.entryA,
    s0.ctl2 == 0,
    s0.ctl2Hi == 0,
    s0.ctlAux == 0,
    s0.hlp0 == 0,
    s0.res0 == 0,
    s0.frmCount == 0,
    s0.regAux == p.entryA,
    s0.rem == s0.inD.inFuel,
    s0.con == 0,
    s0.hw == 0,
    s0.rcur == 0,
    s0.ot == OT_RUNNING,
    s0.op == OP_ZERO,
    s0.inD.inDom == SHK_IN,
    inRange(s0.inD.inFuel),
    inRange(s0.inD.inSl),
    s0.progRoot == p.progId,
    s0.progD == progDigestOf(p),
    // CORE-015: chip sets 0 and 1 keep VAL_NONE; chip set 2 admits witnessed val_d in input_d
    (chipSet == 0) implies (s0.inD.inValD == VAL_NONE),
  }

  // Reference single step: returns (valid, nextState, ruleClass)
  pure def refStep(chipSet: int, p: ProgramSpec, s: RowState): (bool, RowState, int) = {
    if (s.ot != OT_RUNNING or s.hlp0 != 0 or s.res0 != 0) {
      (false, s, 0)
    } else if (chipSet == 2 and s.ctl0 == CTL_EVAL) {
      // XAdtEvalConstructNullary (class 119)
      val exprSet = findRomRec(p, TBL_EXPR, s.ctl1)
      if (exprSet.size() != 1) {
        (false, s, 0)
      } else {
        val ex = exprSet.getOnlyElement()
        val ctorSet = findRomRec(p, TBL_CTOR, ex.w2)
        val cstSet = findRomRec(p, TBL_CST, ex.w5)
        if (ctorSet.size() != 1 or cstSet.size() != 1) {
          (false, s, 0)
        } else {
          val ct = ctorSet.getOnlyElement()
          val cs = cstSet.getOnlyElement()
          val ok = and {
            ex.w0 == EXPR_CONSTRUCT,
            ex.w3 == 0,
            ex.w4 == 1,
            ct.w0 == ex.w1,
            ct.w4 == 0,
            cs.w0 == CST_CONSTRUCTOR,
            cs.w1 == 1,
            cs.w2 == ex.w1,
            cs.w3 == 0,
            cs.w4 == ex.w2,
            cs.w5 == 0,
            s.rem >= 1,
            s.con + 1 < LIMB_MAX,
            s.rcur + 1 < LIMB_MAX,
          }
          val sNext = {
            ...s,
            ctl0: CTL_RET,
            ctl1: 0,
            ctl2: ex.w5,
            ctl2Hi: 0,
            ctlAux: 0,
            rem: s.rem - 1,
            con: s.con + 1,
            rcur: s.rcur + 1,
          }
          (ok, sNext, CLS_CNULL)
        }
      }
    } else if (chipSet == 2 and s.ctl0 == CTL_RET) {
      // KAdminPublishValue (class 2)
      val cstSet = findRomRec(p, TBL_CST, s.ctl2)
      if (s.ctl1 != 0 or s.ctl2Hi != 0 or s.frmCount != 0 or cstSet.size() != 1) {
        (false, s, 0)
      } else {
        val cs = cstSet.getOnlyElement()
        val ctorSet = findRomRec(p, TBL_CTOR, cs.w4)
        // Reference VALUE DIGEST formula (const_vd) over the constant's ROM row & constructor
        val expectedVd = if (ctorSet.size() == 1 and cs.w0 == CST_CONSTRUCTOR and cs.w5 == 0) {
          makeConstVd(0, cs.w2, ctorSet.getOnlyElement().w2)
        } else {
          cs.vd
        }
        val sNext = {
          ...s,
          ot: OT_HALTED,
          op: expectedVd,
        }
        (true, sNext, CLS_PUBV)
      }
    } else if (chipSet == 0 and s.ctl0 == CTL_EVAL) {
      // BHelperEnvGetEntry (class 89) for chip set 0 genesis testing
      val exprSet = findRomRec(p, TBL_EXPR, s.ctl1)
      if (exprSet.size() != 1) {
        (false, s, 0)
      } else {
        val ex = exprSet.getOnlyElement()
        val ok = and {
          ex.w0 == EXPR_VAR,
          s.rem >= 1,
          s.con + 1 < LIMB_MAX,
          s.hw + 1 < LIMB_MAX,
        }
        val sNext = {
          ...s,
          hlp0: 2,
          rem: s.rem - 1,
          con: s.con + 1,
          hw: s.hw + 1,
        }
        (ok, sNext, CLS_C0_ENTRY)
      }
    } else {
      (false, s, 0)
    }
  }

  // Execute nSteps reference transitions from s0 and compare to postD and rulesD
  pure def refRunMatches(chipSet: int, p: ProgramSpec, s0: RowState, pvs: PublicValues): bool = {
    val initHold = (pvs.hPre == 0) implies refInitOk(chipSet, p, s0)
    val stateProgHold = (s0.progRoot == p.progId and s0.progD == progDigestOf(p))
    val stepBounds = (pvs.steps >= 1 and pvs.steps <= S_SLOTS and pvs.hPre >= 0 and pvs.hPre < LIMB_MAX)
    val acc0 = (true, s0, List())
    val resTuple = range(0, S_SLOTS).foldl(acc0, (cur, idx) => {
      val okSoFar = cur._1
      val stCur = cur._2
      val rList = cur._3
      if (idx < pvs.steps) {
        val stepped = refStep(chipSet, p, stCur)
        (okSoFar and stepped._1, stepped._2, rList.append(stepped._3))
      } else {
        (okSoFar, stCur, rList.append(0))
      }
    })
    and {
      stepBounds,
      initHold,
      stateProgHold,
      resTuple._1,
      pvs.preD == hashState(s0),
      pvs.postD == hashState(resTuple._2),
      pvs.rulesD == hashRules(resTuple._3),
    }
  }

  pure def refines(w: Witness): bool = {
    val matchingProgs = ALL_PROGS.filter(p => progDigestOf(p) == w.pvs.progD)
    and {
      (w.chipSet == 2 or w.chipSet == 0),
      matchingProgs.size() == 1,
      w.stRows.length() == S_SLOTS + 1,
      {
        val p = matchingProgs.getOnlyElement()
        val s0 = w.stRows.nth(0)
        refRunMatches(w.chipSet, p, s0, w.pvs)
      }
    }
  }

  // ---------------------------------------------------------------------------
  // 8. Circuit Constraints & Bus Balances (AS BUILT in dec.rs, core.rs, rom.rs)
  // ---------------------------------------------------------------------------

  // DEC row message for row j in 0..7 (dec.rs:46-84, 140-149)
  pure def decRowMsg(chipSet: int, padOffBus: bool, padMsg: DecMsg, j: int): DecMsg = {
    if (chipSet == 2) {
      if (j == 0) {
        { dCls: CLS_IDLE, fl0: 0, fl1: 0, fl2: 0 }
      } else if (j == 1) {
        { dCls: CLS_CNULL, fl0: 1, fl1: 0, fl2: 1 }
      } else if (j == 2) {
        { dCls: CLS_PUBV, fl0: 0, fl1: 0, fl2: 0 }
      } else {
        if (padOffBus) { dCls: CLS_IDLE, fl0: 0, fl1: 0, fl2: 0 } else padMsg
      }
    } else {
      if (j == 0) {
        { dCls: CLS_IDLE, fl0: 0, fl1: 0, fl2: 0 }
      } else if (j == 1) {
        { dCls: CLS_C0_ENTRY, fl0: 1, fl1: 1, fl2: 0 }
      } else {
        { dCls: CLS_IDLE, fl0: 0, fl1: 0, fl2: 0 }
      }
    }
  }

  // DEC AIR constraints (dec.rs:122-151): padding rows 3..7 (vset) or 6..7 (!vset) have mult = 0
  pure def checkDecAir(cfg: Cfg, w: Witness): bool = and {
    w.decMult.length() == DEC_HEIGHT,
    w.decMult.indices().forall(j => w.decMult.nth(j) >= 0 and w.decMult.nth(j) < FP),
    if (cfg.padOffBus) {
      if (w.chipSet == 2) {
        Set(3, 4, 5, 6, 7).forall(j => fMod(w.decMult.nth(j)) == 0)
      } else {
        Set(6, 7).forall(j => fMod(w.decMult.nth(j)) == 0)
      }
    } else {
      true
    },
  }

  // DEC LookupBus balance (dec.rs:150, core.rs:338)
  pure def checkDecBus(cfg: Cfg, w: Witness): bool = {
    val reqMsgs = range(0, S_SLOTS).foldl(Set(), (accS, s) => {
      val fl = w.slotFl.nth(s)
      accS.union(Set({ dCls: w.slotKey.nth(s), fl0: fl.nth(0), fl1: fl.nth(1), fl2: fl.nth(2) }))
    })
    val provMsgs = range(0, DEC_HEIGHT).foldl(Set(), (accS, j) =>
      accS.union(Set(decRowMsg(w.chipSet, cfg.padOffBus, w.decPadMsg, j)))
    )
    val allMsgs = reqMsgs.union(provMsgs)
    allMsgs.forall(m => {
      val provSum = range(0, DEC_HEIGHT).foldl(0, (sumP, j) =>
        if (decRowMsg(w.chipSet, cfg.padOffBus, w.decPadMsg, j) == m)
          fAdd(sumP, w.decMult.nth(j))
        else sumP
      )
      val reqSum = range(0, S_SLOTS).foldl(0, (sumR, s) => {
        val fl = w.slotFl.nth(s)
        val rm = { dCls: w.slotKey.nth(s), fl0: fl.nth(0), fl1: fl.nth(1), fl2: fl.nth(2) }
        if (rm == m) fAdd(sumR, 1) else sumR
      })
      provSum == reqSum
    })
  }

  // Core enable prefix, steps count, dispatch & DEC query (core.rs:307-361)
  pure def checkDispatchAndRulesD(cfg: Cfg, w: Witness): bool = {
    val vset = (w.chipSet == 2)
    val nRules = if (vset) 2 else 1
    val enPrefixOk = range(0, S_SLOTS).indices().forall(s => {
      val en = w.slotEn.nth(s)
      val enBoolOk = if (cfg.selBool) isBool(en) else (en >= 0 and en < FP)
      val firstOk = (s == 0) implies (en == 1 or not(cfg.selBool) and s >= 0)
      // core.rs:313 when_first_row().assert_one(en) STILL holds even if assert_bool(en) is dropped!
      val row0EnOne = if (s == 0) (en == 1) else true
      val prefixStep = if (s + 1 < S_SLOTS) {
        fMul(w.slotEn.nth(s + 1), fSub(1, en)) == 0
      } else true
      enBoolOk and firstOk and prefixStep and (if (cfg.selBool) row0EnOne else true)
    })
    // Wait: let's check core.rs:313! On row 0, when_first_row().assert_one(en) forces en[0] == 1!
    // What about slot 1? On slot 1, when en[0] == 1, nx(en) * (1 - en[0]) = en[1] * 0 == 0!
    // So on slot 1, en[1] CAN be 95 (= 2 * 119^{-1} mod 127) while en[0] == 1 and en[2] == 0!!
    val row0ForceOne = (w.slotEn.nth(0) == 1)
    val stepsSum = range(0, S_SLOTS).foldl(0, (accCnt, s) => fAdd(accCnt, w.slotEn.nth(s)))
    val stepsMatch = (stepsSum == fMod(w.pvs.steps)) and inRange(w.pvs.steps)

    val perSlotOk = range(0, S_SLOTS).indices().forall(s => {
      val sel = w.slotSel.nth(s)
      val fl = w.slotFl.nth(s)
      val selLenOk = (sel.length() == 5 and fl.length() == 3)
      val selRangeOk = sel.indices().forall(r =>
        if (cfg.selBool and cfg.selOnlyBool) isBool(sel.nth(r))
        else (sel.nth(r) >= 0 and sel.nth(r) < FP)
      )
      val flBoolOk = fl.indices().forall(k => isBool(fl.nth(k)))
      val unusedZero = range(nRules, 5).foldl(true, (okU, r) => okU and (sel.nth(r) == 0))
      val ssum = if (vset) fAdd(sel.nth(0), sel.nth(1)) else sel.nth(0)
      val cls0 = if (vset) (if (cfg.keyRightClass) CLS_CNULL else CLS_PUBV) else CLS_C0_ENTRY
      val cls1 = if (vset) (if (cfg.keyRightClass) CLS_PUBV else CLS_CNULL) else 0
      val expectedKey = if (vset) {
        fAdd(fMul(sel.nth(0), cls0), fMul(sel.nth(1), cls1))
      } else {
        fMul(sel.nth(0), cls0)
      }
      val sumOk = if (cfg.selSum) (ssum == w.slotEn.nth(s)) else true
      val keyOk = (w.slotKey.nth(s) == expectedKey)
      val ridOk = (w.rids.nth(s) == w.slotKey.nth(s))
      selLenOk and selRangeOk and flBoolOk and unusedZero and sumOk and keyOk and ridOk
    })

    val rulesDOk = (w.rulesCol == hashRules(w.rids)) and (w.pvs.rulesD == w.rulesCol)
    enPrefixOk and row0ForceOne and stepsMatch and perSlotOk and rulesDOk
  }

  // Gate helper matching core.rs:390: g(builder, r, x) => sel(r) * x == 0 in F_p
  pure def gZero(selVal: int, exprVal: int): bool =
    fMul(selVal, fMod(exprVal)) == 0

  pure def gDigestEq(selVal: int, d1: ValDigest, d2: ValDigest): bool =
    (fMod(selVal) == 0) or (d1 == d2)

  // Value-set rules R_CNULL and R_PUBV (core.rs:380-445) + chip set 0 R_ENTRY
  pure def checkRules(cfg: Cfg, w: Witness): bool = {
    range(0, S_SLOTS).indices().forall(s => {
      val st = w.stRows.nth(s)
      val nst = w.stRows.nth(s + 1)
      val sel = w.slotSel.nth(s)
      val rom = w.slotRom.nth(s)
      val r2 = w.slotR2.nth(s)
      val r3 = w.slotR3.nth(s)
      val cx = w.slotCx.nth(s)
      if (w.chipSet == 2) {
        val sCnull = sel.nth(R_CNULL)
        val sPubv = sel.nth(R_PUBV)
        val cnullOk = and {
          if (cfg.cnullRunning)    gZero(sCnull, st.ot) else true,
          if (cfg.cnullNoHlp)      gZero(sCnull, st.hlp0) else true,
          if (cfg.cnullNoRes)      gZero(sCnull, st.res0) else true,
          if (cfg.cnullCtlEval)    gZero(sCnull, st.ctl0) else true,
          if (cfg.cnullExprKind)   gZero(sCnull, fSub(rom.w0, EXPR_CONSTRUCT)) else true,
          if (cfg.cnullExprArity)  gZero(sCnull, rom.w3) else true,
          if (cfg.cnullHasConst)   gZero(sCnull, fSub(rom.w4, 1)) else true,
          if (cfg.cnullCtorType)   gZero(sCnull, fSub(r2.w0, rom.w1)) else true,
          if (cfg.cnullCtorArity)  gZero(sCnull, r2.w4) else true,
          if (cfg.cnullCstKind)    gZero(sCnull, fSub(r3.w0, CST_CONSTRUCTOR)) else true,
          if (cfg.cnullMarkerFree) gZero(sCnull, fSub(r3.w1, 1)) else true,
          if (cfg.cnullCstFields)  and {
            gZero(sCnull, fSub(r3.w2, rom.w1)),
            gZero(sCnull, r3.w3),
            gZero(sCnull, fSub(r3.w4, rom.w2)),
            gZero(sCnull, r3.w5),
          } else true,
          if (cfg.cnullCidBind)    gZero(sCnull, fSub(cx, rom.w5)) else true,
          // Effect on ctl words (core.rs:417-426)
          gZero(sCnull, fSub(nst.ctl0, CTL_RET)),
          gZero(sCnull, nst.ctl1),
          if (cfg.cnullCtlEffect)  gZero(sCnull, fSub(nst.ctl2, cx)) else true,
          gZero(sCnull, nst.ctl2Hi),
          gZero(sCnull, nst.ctlAux),
        }
        val pubvOk = and {
          if (cfg.pubvRunning)   gZero(sPubv, st.ot) else true,
          if (cfg.pubvNoHlp)     gZero(sPubv, st.hlp0) else true,
          if (cfg.pubvNoRes)     gZero(sPubv, st.res0) else true,
          if (cfg.pubvCtlRet)    gZero(sPubv, fSub(st.ctl0, CTL_RET)) else true,
          if (cfg.pubvValConst)  gZero(sPubv, st.ctl1) else true,
          if (cfg.pubvCidHi)     gZero(sPubv, st.ctl2Hi) else true,
          if (cfg.pubvFrmZero)   gZero(sPubv, st.frmCount) else true,
          if (cfg.pubvCidBind)   gZero(sPubv, fSub(cx, st.ctl2)) else true,
          // Effect on outcome words (core.rs:441-444)
          gZero(sPubv, fSub(nst.ot, OT_HALTED)),
          if (cfg.pubvPayloadVd) gDigestEq(sPubv, nst.op, r3.vd) else true,
        }
        cnullOk and pubvOk
      } else {
        // Chip set 0: R_ENTRY (core.rs:469-496)
        val sEntry = sel.nth(0)
        and {
          gZero(sEntry, st.ctl0),
          gZero(sEntry, st.hlp0),
          gZero(sEntry, st.res0),
          gZero(sEntry, fSub(rom.w0, EXPR_VAR)),
          gZero(sEntry, fSub(nst.hlp0, 2)),
        }
      }
    })
  }

  // Frame rule (core.rs:735-785)
  pure def checkFrameRule(cfg: Cfg, w: Witness): bool = {
    val vset = (w.chipSet == 2)
    range(0, S_SLOTS).indices().forall(s => {
      val st = w.stRows.nth(s)
      val nst = w.stRows.nth(s + 1)
      val sel = w.slotSel.nth(s)
      val keepCtl = if (vset) fSub(1, sel.nth(R_CNULL)) else 1
      val keepOut = if (vset) fSub(1, sel.nth(R_PUBV)) else 1
      val keepHlp = if (vset) 1 else fSub(1, sel.nth(0))
      val keepOther = 1
      and {
        gZero(keepCtl, fSub(nst.ctl0, st.ctl0)),
        gZero(keepCtl, fSub(nst.ctl1, st.ctl1)),
        gZero(keepCtl, fSub(nst.ctl2, st.ctl2)),
        gZero(keepCtl, fSub(nst.ctl2Hi, st.ctl2Hi)),
        gZero(keepCtl, fSub(nst.ctlAux, st.ctlAux)),
        gZero(keepHlp, fSub(nst.hlp0, st.hlp0)),
        gZero(keepOther, fSub(nst.res0, st.res0)),
        gZero(keepOther, fSub(nst.frmCount, st.frmCount)),
        if (cfg.frameKeepAll or not(vset)) gZero(keepOther, fSub(nst.regAux, st.regAux)) else true,
        gZero(keepOut, fSub(nst.ot, st.ot)),
        gDigestEq(keepOut, nst.op, st.op),
        (fMod(keepOther) == 0) or (nst.inD == st.inD),
        (fMod(keepOther) == 0) or (nst.progRoot == st.progRoot),
        (fMod(keepOther) == 0) or (nst.progD == st.progD),
      }
    })
  }

  // Counter charges & RANGE checks (core.rs:544-567)
  // Models 1-limb counter in [0, LIMB_MAX) with top carry/borrow asserted zero (core.rs:566)
  pure def checkCharges(w: Witness): bool = {
    val rangeAllRows = range(0, S_SLOTS + 1).indices().forall(r => {
      val st = w.stRows.nth(r)
      inRange(st.rem) and inRange(st.con) and inRange(st.hw) and inRange(st.rcur)
    })
    val slotTransitions = range(0, S_SLOTS).indices().forall(s => {
      val st = w.stRows.nth(s)
      val nst = w.stRows.nth(s + 1)
      val fl = w.slotFl.nth(s)
      val brw = w.slotBrw.nth(s)
      val cc = w.slotCc.nth(s)
      val hc = w.slotHc.nth(s)
      val rc = w.slotRc.nth(s)
      and {
        isBool(brw), isBool(cc), isBool(hc), isBool(rc),
        // core.rs:566: top carry/borrow is asserted zero (prevents wrap mod 2^128)
        brw == 0, cc == 0, hc == 0, rc == 0,
        fMod(nst.rem) == fAdd(fSub(st.rem, fl.nth(0)), fMul(brw, LIMB_MAX)),
        fMod(nst.con) == fSub(fAdd(st.con, fl.nth(0)), fMul(cc, LIMB_MAX)),
        fMod(nst.hw)  == fSub(fAdd(st.hw,  fl.nth(1)), fMul(hc, LIMB_MAX)),
        fMod(nst.rcur)== fSub(fAdd(st.rcur,fl.nth(2)), fMul(rc, LIMB_MAX)),
      }
    })
    rangeAllRows and slotTransitions
  }

  // ROM AIR, PORT_ROM_ROOT permutation check, and bus::ROM LookupBus (rom.rs:55-113, core.rs:446-460, 618-620)
  pure def checkRomAndRoot(cfg: Cfg, w: Witness): bool = {
    val romLenOk = (w.romRows.length() == ROM_HEIGHT)
    val romAirOk = range(0, ROM_HEIGHT).indices().forall(r => {
      val rw = w.romRows.nth(r)
      val actBool = isBool(rw.act)
      val multRange = (rw.mult >= 0 and rw.mult < FP)
      val inactiveZeroMult = (fMul(rw.mult, fSub(1, rw.act)) == 0)
      // Merkle opening against rw.rootOut (progId):
      val merkleOk = if (rw.act == 1) {
        ALL_PROGS.exists(p =>
          p.progId == rw.rootOut and (
            if (cfg.romTblBind) {
              p.romEntries.contains({ tbl: rw.tbl, rowIdx: rw.rowIdx, rec: rw.rec })
            } else {
              // Mutant rom_no_tbl_bind: leaf/tree binds rowIdx & rec, not tbl
              p.romEntries.exists(e => e.rowIdx == rw.rowIdx and e.rec == rw.rec)
            }
          )
        )
      } else true
      actBool and multRange and inactiveZeroMult and merkleOk
    })

    // PORT_ROM_ROOT permutation check: RomAir sends rw.rootOut with count rw.act;
    // CoreAir row 0 receives st[0].progRoot with count w.nrom (core.rs:618-620).
    val nromRange = (w.nrom >= 0 and w.nrom < FP)
    val rootsSet = range(0, ROM_HEIGHT).foldl(Set(w.stRows.nth(0).progRoot), (accR, r) =>
      accR.union(Set(w.romRows.nth(r).rootOut))
    )
    val romRootPortOk = rootsSet.forall(rt => {
      val sent = range(0, ROM_HEIGHT).foldl(0, (sumS, r) => {
        val rw = w.romRows.nth(r)
        if (rw.rootOut == rt) fAdd(sumS, rw.act) else sumS
      })
      val recv = if (w.stRows.nth(0).progRoot == rt) fMod(w.nrom) else 0
      sent == recv
    })

    // bus::ROM LookupBus balance (core.rs:446-460, 537-541)
    val provKeys = range(0, ROM_HEIGHT).foldl(Set(), (accK, r) => {
      val rw = w.romRows.nth(r)
      accK.union(Set({ tbl: rw.tbl, rowIdx: rw.rowIdx, rec: rw.rec }))
    })
    val reqList: List[(RomEntry, int)] = range(0, S_SLOTS).foldl(List(), (accL, s) => {
      val st = w.stRows.nth(s)
      val sel = w.slotSel.nth(s)
      val rom = w.slotRom.nth(s)
      val r2 = w.slotR2.nth(s)
      val r3 = w.slotR3.nth(s)
      val cx = w.slotCx.nth(s)
      if (w.chipSet == 2) {
        val mCnull = if (cfg.cnullRom) sel.nth(R_CNULL) else 0
        val mCst = fAdd(if (cfg.cnullRom) sel.nth(R_CNULL) else 0, if (cfg.pubvRom) sel.nth(R_PUBV) else 0)
        // Note core.rs:449, 454: Table 0 and Table 11 lookups zero-pad upper words (vd = OP_ZERO)
        val e1 = { tbl: TBL_EXPR, rowIdx: st.ctl1, rec: { ...rom, vd: OP_ZERO } }
        val e2 = { tbl: TBL_CTOR, rowIdx: rom.w2,  rec: { ...r2, w5: 0, vd: OP_ZERO } }
        val e3 = { tbl: TBL_CST,  rowIdx: cx,      rec: r3 }
        accL.append((e1, mCnull)).append((e2, mCnull)).append((e3, mCst))
      } else {
        val e1 = { tbl: TBL_EXPR, rowIdx: st.ctl1, rec: { ...rom, vd: OP_ZERO } }
        accL.append((e1, sel.nth(0)))
      }
    })
    val reqKeys = reqList.indices().fold(Set(), (accK, i) => accK.union(Set(reqList.nth(i)._1)))
    val allKeys = provKeys.union(reqKeys)
    val romBusOk = allKeys.forall(k => {
      val provSum = range(0, ROM_HEIGHT).foldl(0, (sumP, r) => {
        val rw = w.romRows.nth(r)
        if ({ tbl: rw.tbl, rowIdx: rw.rowIdx, rec: rw.rec } == k) fAdd(sumP, rw.mult) else sumP
      })
      val reqSum = reqList.indices().fold(0, (sumR, i) => {
        val pair = reqList.nth(i)
        if (pair._1 == k) fAdd(sumR, pair._2) else sumR
      })
      // Also check that CoreAir's witness rom/r2 have zeroed upper words matching the lookup key
      provSum == reqSum
    })
    // Notice in core.rs:448-455: CoreAir sends rom(0..25)++0 and r2(0..20)++0, so rom.vd and r2.vd
    // are not even columns in CoreAir (C_ROM is 25 words, C_VR2 is 20 words). That's exact above!
    romLenOk and romAirOk and nromRange and romRootPortOk and romBusOk
  }

  // BOUNDARY and INIT constraints (core.rs:569-720)
  pure def checkBoundaryAndInit(cfg: Cfg, w: Witness): bool = {
    val s0 = w.stRows.nth(0)
    val sPost = w.stRows.nth(S_SLOTS)
    val boundaryOk = and {
      w.pvs.preD == hashState(s0),
      w.pvs.postD == hashState(sPost),
      w.pvs.progD == s0.progD,
      inRange(w.pvs.hPre),
    }

    // INIT: g = is_zero(h_pre) on row 0 (core.rs:623-627)
    val iw = w.initW
    val hsum = fMod(w.pvs.hPre)
    val isZeroGadget = and {
      iw.gFlag == fSub(1, fMul(hsum, iw.hInv)),
      fMul(iw.gFlag, hsum) == 0,
      iw.gi == iw.gFlag,
    }
    val vset = (w.chipSet == 2)
    val inValUsed = if (vset or not(cfg.initValOnlySet2)) iw.vval else VAL_NONE
    val initHold = if (fMod(iw.gi) == 0) {
      true
    } else {
      and {
        gZero(iw.gi, s0.ctl0),
        gZero(iw.gi, fSub(s0.ctl1, iw.ea)),
        gZero(iw.gi, s0.ctl2),
        gZero(iw.gi, s0.ctl2Hi),
        gZero(iw.gi, s0.ctlAux),
        gZero(iw.gi, s0.hlp0),
        gZero(iw.gi, s0.res0),
        gZero(iw.gi, s0.frmCount),
        gZero(iw.gi, fSub(s0.regAux, iw.ea)),
        gZero(iw.gi, fSub(s0.rem, iw.fuel)),
        gZero(iw.gi, s0.con),
        gZero(iw.gi, s0.hw),
        gZero(iw.gi, s0.rcur),
        gZero(iw.gi, s0.ot),
        gDigestEq(iw.gi, s0.op, OP_ZERO),
        // Sponge openings (IMAGE, PROG, INPUT) gated by gi != 0 (core.rs:640-670)
        iw.imgCol == hashImage(s0.progRoot, iw.lv, iw.pid, iw.ea),
        s0.progD == hashProg(iw.imgCol, iw.planD, iw.kr, iw.mv),
        s0.inD == hashInput(iw.salt, inValUsed, iw.fuel, iw.sl),
        // RANGE checks on INIT witness scalars (core.rs:671-680)
        inRange(iw.ea),
        inRange(iw.fuel),
        inRange(iw.sl),
      }
    }
    boundaryOk and isZeroGadget and initHold
  }

  // Structural column length check + full AIR & bus acceptance
  pure def isAccepted(cfg: Cfg, w: Witness): bool = and {
    (w.chipSet == 2 or w.chipSet == 0),
    w.stRows.length() == S_SLOTS + 1,
    w.slotEn.length() == S_SLOTS,
    w.slotSel.length() == S_SLOTS,
    w.slotKey.length() == S_SLOTS,
    w.slotFl.length() == S_SLOTS,
    w.slotRom.length() == S_SLOTS,
    w.slotR2.length() == S_SLOTS,
    w.slotR3.length() == S_SLOTS,
    w.slotCx.length() == S_SLOTS,
    w.slotBrw.length() == S_SLOTS,
    w.slotCc.length() == S_SLOTS,
    w.slotHc.length() == S_SLOTS,
    w.slotRc.length() == S_SLOTS,
    w.rids.length() == S_SLOTS,
    checkDecAir(cfg, w),
    checkDecBus(cfg, w),
    checkDispatchAndRulesD(cfg, w),
    checkRules(cfg, w),
    checkFrameRule(cfg, w),
    checkCharges(w),
    checkRomAndRoot(cfg, w),
    checkBoundaryAndInit(cfg, w),
  }

  // Full acceptance check including naive `sel == 1` gating when cfg.selBool is false
  pure def evalAcceptedCfg(cfg: Cfg, w: Witness): bool = {
    if (cfg.selBool) {
      isAccepted(cfg, w)
    } else {
      val nonBoolSelUsed = w.slotSel.indices().exists(s =>
        w.slotSel.nth(s).indices().exists(r => not(isBool(w.slotSel.nth(s).nth(r))))
      )
      if (not(nonBoolSelUsed)) {
        isAccepted(cfg, w)
      } else {
        val vset = (w.chipSet == 2)
        val dispOk = and {
          w.slotEn.nth(0) == 1,
          range(0, S_SLOTS).indices().forall(s => {
            val sel = w.slotSel.nth(s)
            val ssum = if (vset) fAdd(sel.nth(0), sel.nth(1)) else sel.nth(0)
            val expectedKey = if (vset) fAdd(fMul(sel.nth(0), CLS_CNULL), fMul(sel.nth(1), CLS_PUBV))
                              else fMul(sel.nth(0), CLS_C0_ENTRY)
            and {
              isBool(w.slotEn.nth(s)),
              sel.nth(2) == 0, sel.nth(3) == 0, sel.nth(4) == 0,
              ssum == w.slotEn.nth(s),
              w.slotKey.nth(s) == expectedKey,
              w.rids.nth(s) == w.slotKey.nth(s),
            }
          }),
          w.rulesCol == hashRules(w.rids),
          w.pvs.rulesD == w.rulesCol,
          range(0, S_SLOTS).foldl(0, (a, s) => fAdd(a, w.slotEn.nth(s))) == fMod(w.pvs.steps),
        }
        val naiveFrameAndRulesOk = range(0, S_SLOTS).indices().forall(s => {
          val sel = w.slotSel.nth(s)
          val st = w.stRows.nth(s)
          val nst = w.stRows.nth(s + 1)
          (sel.nth(0) != 1 and sel.nth(1) != 1) implies (nst == st)
        })
        and {
          dispOk,
          naiveFrameAndRulesOk,
          checkDecAir(cfg, w),
          checkDecBus(cfg, w),
          checkCharges(w),
          checkRomAndRoot(cfg, w),
          checkBoundaryAndInit(cfg, w),
        }
      }
    }
  }

  // ---------------------------------------------------------------------------
  // 9. Honest Witness & Structured Prover Witness Builders
  // ---------------------------------------------------------------------------
  pure val ZERO_REC: RomRec = metaRec(0)
  pure val INACTIVE_ROM_ROW: RomAirRow =
    { act: 0, mult: 0, tbl: 0, rowIdx: 0, rec: ZERO_REC, rootOut: 0 }

  pure def makeGenesisState(p: ProgramSpec, vdIn: ValDigest, salt: int, fuel: int, sl: int): RowState = {
    ctl0: CTL_EVAL,
    ctl1: p.entryA,
    ctl2: 0,
    ctl2Hi: 0,
    ctlAux: 0,
    hlp0: 0,
    res0: 0,
    frmCount: 0,
    regAux: p.entryA,
    rem: fuel,
    con: 0,
    hw: 0,
    rcur: 0,
    ot: OT_RUNNING,
    op: OP_ZERO,
    inD: hashInput(salt, vdIn, fuel, sl),
    progRoot: p.progId,
    progD: progDigestOf(p),
  }

  // Honest 2-step value-set shard witness for program p with input val_d = vdIn
  pure def buildValueShard2Step(p: ProgramSpec, vdIn: ValDigest): Witness = {
    val s0 = makeGenesisState(p, vdIn, 1, 4, 8)
    val ex = exprRec(EXPR_CONSTRUCT, 0, 0, 0, 1, 0)
    val ct = ctorRec(0, 1, 0, 0)
    val cs = cstRec(CST_CONSTRUCTOR, 1, 0, 0, 0, 0, VD_CTOR0)
    val s1 = { ...s0, ctl0: CTL_RET, ctl1: 0, ctl2: 0, rem: 3, con: 1, rcur: 1 }
    val s2 = { ...s1, ot: OT_HALTED, op: VD_CTOR0 }
    val s3 = s2
    val rids = List(CLS_CNULL, CLS_PUBV, CLS_IDLE)
    val imgD = hashImage(p.progId, p.lv, p.pid, p.entryA)
    {
      chipSet: 2,
      stRows: List(s0, s1, s2, s3),
      slotEn: List(1, 1, 0),
      slotSel: List(List(1, 0, 0, 0, 0), List(0, 1, 0, 0, 0), List(0, 0, 0, 0, 0)),
      slotKey: rids,
      slotFl: List(List(1, 0, 1), List(0, 0, 0), List(0, 0, 0)),
      slotRom: List(ex, ex, ex),
      slotR2: List(ct, ct, ct),
      slotR3: List(cs, cs, cs),
      slotCx: List(0, 0, 0),
      slotBrw: List(0, 0, 0),
      slotCc: List(0, 0, 0),
      slotHc: List(0, 0, 0),
      slotRc: List(0, 0, 0),
      rids: rids,
      rulesCol: hashRules(rids),
      nrom: 3,
      initW: {
        gi: 1, gFlag: 1, hInv: 0,
        lv: p.lv, pid: p.pid, mv: p.mv, ea: p.entryA,
        planD: p.planD, kr: p.keyRoot, imgCol: imgD,
        salt: 1, fuel: 4, sl: 8, vval: vdIn,
      },
      decMult: List(1, 1, 1, 0, 0, 0, 0, 0),
      decPadMsg: { dCls: CLS_IDLE, fl0: 0, fl1: 0, fl2: 0 },
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_EXPR, rowIdx: 0, rec: ex, rootOut: p.progId },
        { act: 1, mult: 1, tbl: TBL_CTOR, rowIdx: 0, rec: ct, rootOut: p.progId },
        { act: 1, mult: 2, tbl: TBL_CST,  rowIdx: 0, rec: cs, rootOut: p.progId },
        INACTIVE_ROM_ROW
      ),
      pvs: {
        progD: progDigestOf(p),
        preD: hashState(s0),
        postD: hashState(s3),
        rulesD: hashRules(rids),
        hPre: 0,
        steps: 2,
      },
    }
  }

  pure val honestWitness: Witness = buildValueShard2Step(PROG_U, VAL_NONE)

  // Valid 1-step R_CNULL shard from genesis
  pure val validCnullOnlyWitness: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = base.stRows.nth(1)
    val rids = List(CLS_CNULL, CLS_IDLE, CLS_IDLE)
    val ex = base.slotRom.nth(0)
    val ct = base.slotR2.nth(0)
    val cs = base.slotR3.nth(0)
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      slotEn: List(1, 0, 0),
      slotSel: List(List(1, 0, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
      slotKey: rids,
      slotFl: List(List(1, 0, 1), List(0, 0, 0), List(0, 0, 0)),
      rids: rids,
      rulesCol: hashRules(rids),
      decMult: List(2, 1, 0, 0, 0, 0, 0, 0),
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_EXPR, rowIdx: 0, rec: ex, rootOut: PROG_U.progId },
        { act: 1, mult: 1, tbl: TBL_CTOR, rowIdx: 0, rec: ct, rootOut: PROG_U.progId },
        { act: 1, mult: 1, tbl: TBL_CST,  rowIdx: 0, rec: cs, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW
      ),
      pvs: { ...base.pvs, postD: hashState(s1), rulesD: hashRules(rids), steps: 1 },
    }
  }

  // Valid 1-step R_PUBV continuation shard (h_pre = 1)
  pure val validPubvOnlyWitness: Witness = {
    val base = honestWitness
    val s1 = base.stRows.nth(1)
    val s2 = base.stRows.nth(2)
    val rids = List(CLS_PUBV, CLS_IDLE, CLS_IDLE)
    val cs = base.slotR3.nth(0)
    {
      ...base,
      stRows: List(s1, s2, s2, s2),
      slotEn: List(1, 0, 0),
      slotSel: List(List(0, 1, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
      slotKey: rids,
      slotFl: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
      rids: rids,
      rulesCol: hashRules(rids),
      nrom: 1,
      initW: { ...base.initW, gi: 0, gFlag: 0, hInv: 1 },
      decMult: List(2, 0, 1, 0, 0, 0, 0, 0),
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW,
        INACTIVE_ROM_ROW,
        INACTIVE_ROM_ROW
      ),
      pvs: {
        progD: progDigestOf(PROG_U),
        preD: hashState(s1),
        postD: hashState(s2),
        rulesD: hashRules(rids),
        hPre: 1,
        steps: 1,
      },
    }
  }

  // Valid consumer shard (ProgV::C with witnessed val_d = producer's VD_CTOR0)
  pure val validConsumerWitness: Witness = buildValueShard2Step(PROG_V, VD_CTOR0)

  // Valid chip set 0 genesis shard (`unbound`, 1 step of BHelperEnvGetEntry with VAL_NONE)
  pure def buildChipSet0Genesis(vvalWitness: ValDigest, useValInState: bool): Witness = {
    val vdState = if (useValInState) vvalWitness else VAL_NONE
    val s0 = makeGenesisState(PROG_VAR, vdState, 1, 4, 8)
    val s1 = { ...s0, hlp0: 2, rem: 3, con: 1, hw: 1 }
    val ex = exprRec(EXPR_VAR, 0, 0, 0, 1, 0)
    val rids = List(CLS_C0_ENTRY, CLS_IDLE, CLS_IDLE)
    val imgD = hashImage(PROG_VAR.progId, PROG_VAR.lv, PROG_VAR.pid, PROG_VAR.entryA)
    {
      chipSet: 0,
      stRows: List(s0, s1, s1, s1),
      slotEn: List(1, 0, 0),
      slotSel: List(List(1, 0, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
      slotKey: rids,
      slotFl: List(List(1, 1, 0), List(0, 0, 0), List(0, 0, 0)),
      slotRom: List(ex, ex, ex),
      slotR2: List(ZERO_REC, ZERO_REC, ZERO_REC),
      slotR3: List(ZERO_REC, ZERO_REC, ZERO_REC),
      slotCx: List(0, 0, 0),
      slotBrw: List(0, 0, 0),
      slotCc: List(0, 0, 0),
      slotHc: List(0, 0, 0),
      slotRc: List(0, 0, 0),
      rids: rids,
      rulesCol: hashRules(rids),
      nrom: 1,
      initW: {
        gi: 1, gFlag: 1, hInv: 0,
        lv: PROG_VAR.lv, pid: PROG_VAR.pid, mv: PROG_VAR.mv, ea: PROG_VAR.entryA,
        planD: PROG_VAR.planD, kr: PROG_VAR.keyRoot, imgCol: imgD,
        salt: 1, fuel: 4, sl: 8, vval: vvalWitness,
      },
      decMult: List(2, 1, 0, 0, 0, 0, 0, 0),
      decPadMsg: { dCls: CLS_IDLE, fl0: 0, fl1: 0, fl2: 0 },
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_EXPR, rowIdx: 0, rec: ex, rootOut: PROG_VAR.progId },
        INACTIVE_ROM_ROW,
        INACTIVE_ROM_ROW,
        INACTIVE_ROM_ROW
      ),
      pvs: {
        progD: progDigestOf(PROG_VAR),
        preD: hashState(s0),
        postD: hashState(s1),
        rulesD: hashRules(rids),
        hPre: 0,
        steps: 1,
      },
    }
  }

  pure val validChipSet0Witness: Witness = buildChipSet0Genesis(VAL_NONE, false)

  // ---------------------------------------------------------------------------
  // 10. Adversarial Attack Witnesses (32 concrete attacks + general tampers)
  // ---------------------------------------------------------------------------

  // Helper: 1-step R_CNULL continuation witness from custom pre-state s0 on PROG_U
  pure def cnullFromPreState(s0: RowState): Witness = {
    val base = validCnullOnlyWitness
    val s1 = {
      ...s0,
      ctl0: CTL_RET, ctl1: 0, ctl2: 0, ctl2Hi: 0, ctlAux: 0,
      rem: s0.rem - 1, con: s0.con + 1, rcur: s0.rcur + 1,
    }
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      initW: { ...base.initW, gi: 0, gFlag: 0, hInv: 1 },
      pvs: { ...base.pvs, preD: hashState(s0), postD: hashState(s1), hPre: 1, steps: 1 },
    }
  }

  // Helper: 1-step R_CNULL genesis witness on an arbitrary program p (using its ROM rows 0)
  pure def cnullGenesisOnProg(p: ProgramSpec): Witness = {
    val ex = findRomRec(p, TBL_EXPR, 0).getOnlyElement()
    val ct = findRomRec(p, TBL_CTOR, 0).getOnlyElement()
    val cs = findRomRec(p, TBL_CST, 0).getOnlyElement()
    val s0 = makeGenesisState(p, VAL_NONE, 1, 4, 8)
    val s1 = { ...s0, ctl0: CTL_RET, ctl1: 0, ctl2: ex.w5, rem: 3, con: 1, rcur: 1 }
    val rids = List(CLS_CNULL, CLS_IDLE, CLS_IDLE)
    val imgD = hashImage(p.progId, p.lv, p.pid, p.entryA)
    {
      ...validCnullOnlyWitness,
      stRows: List(s0, s1, s1, s1),
      slotRom: List(ex, ex, ex),
      slotR2: List(ct, ct, ct),
      slotR3: List(cs, cs, cs),
      slotCx: List(ex.w5, 0, 0),
      initW: {
        gi: 1, gFlag: 1, hInv: 0,
        lv: p.lv, pid: p.pid, mv: p.mv, ea: p.entryA,
        planD: p.planD, kr: p.keyRoot, imgCol: imgD,
        salt: 1, fuel: 4, sl: 8, vval: VAL_NONE,
      },
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_EXPR, rowIdx: 0, rec: ex, rootOut: p.progId },
        { act: 1, mult: 1, tbl: TBL_CTOR, rowIdx: 0, rec: ct, rootOut: p.progId },
        { act: 1, mult: 1, tbl: TBL_CST,  rowIdx: 0, rec: cs, rootOut: p.progId },
        INACTIVE_ROM_ROW
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
  }

  // Helper: 1-step R_PUBV continuation witness from custom pre-state s0 on PROG_U
  pure def pubvFromPreState(s0: RowState): Witness = {
    val base = validPubvOnlyWitness
    val s1 = { ...s0, ot: OT_HALTED, op: VD_CTOR0 }
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      slotCx: List(0, 0, 0),
      pvs: { ...base.pvs, preD: hashState(s0), postD: hashState(s1), hPre: 1, steps: 1 },
    }
  }

  // 1. Attack cnull_no_running: fire R_CNULL when ot == OT_REJECTED (2)
  pure val atkCnullNoRunning: Witness =
    cnullFromPreState({ ...honestWitness.stRows.nth(0), ot: OT_REJECTED })

  // 2. Attack cnull_no_hlp: fire R_CNULL while helper is active (hlp0 == 2)
  pure val atkCnullNoHlp: Witness =
    cnullFromPreState({ ...honestWitness.stRows.nth(0), hlp0: 2 })

  // 3. Attack cnull_no_res: fire R_CNULL while pending result exists (res0 == 3)
  pure val atkCnullNoRes: Witness =
    cnullFromPreState({ ...honestWitness.stRows.nth(0), res0: 3 })

  // 4. Attack cnull_no_ctl_eval: fire R_CNULL when ctl0 == CTL_RET (1)
  pure val atkCnullNoCtlEval: Witness =
    cnullFromPreState({ ...honestWitness.stRows.nth(0), ctl0: CTL_RET })

  // 5. Attack cnull_no_expr_kind: fire R_CNULL on Var expression (PROG_VAR, kind = 1 != 8)
  pure val atkCnullNoExprKind: Witness = cnullGenesisOnProg(PROG_VAR)

  // 6. Attack cnull_no_expr_arity: fire R_CNULL on 1-arg Construct (PROG_EXPR_ARITY1)
  pure val atkCnullNoExprArity: Witness = cnullGenesisOnProg(PROG_EXPR_ARITY1)

  // 7. Attack cnull_no_has_const: fire R_CNULL when constant = None (PROG_NO_CONST)
  pure val atkCnullNoHasConst: Witness = cnullGenesisOnProg(PROG_NO_CONST)

  // 8. Attack cnull_no_ctor_type: fire R_CNULL when ctor.ownerTy != expr.ty (PROG_BAD_CTOR_TY)
  pure val atkCnullNoCtorType: Witness = cnullGenesisOnProg(PROG_BAD_CTOR_TY)

  // 9. Attack cnull_no_ctor_arity: fire R_CNULL when ctor.arity != 0 (PROG_BAD_CTOR_ARITY)
  pure val atkCnullNoCtorArity: Witness = cnullGenesisOnProg(PROG_BAD_CTOR_ARITY)

  // 10. Attack cnull_no_cst_kind: fire R_CNULL when cst.kind != 6 (PROG_BAD_CST_KIND)
  pure val atkCnullNoCstKind: Witness = cnullGenesisOnProg(PROG_BAD_CST_KIND)

  // 11. Attack cnull_no_marker_free: fire R_CNULL when cst.markerFree == 0 (PROG_NOT_MARKER_FREE)
  pure val atkCnullNoMarkerFree: Witness = cnullGenesisOnProg(PROG_NOT_MARKER_FREE)

  // 12. Attack cnull_no_cst_fields: fire R_CNULL when cst.ctor != expr.ctor (PROG_BAD_CST_FIELDS)
  pure val atkCnullNoCstFields: Witness = cnullGenesisOnProg(PROG_BAD_CST_FIELDS)

  // 13. Attack cnull_no_cid_bind: on PROG_U, point cx to cid=2 (same ty/ctor/arity=0) instead of expr.cid=0
  pure val atkCnullNoCidBind: Witness = {
    val base = validCnullOnlyWitness
    val s0 = base.stRows.nth(0)
    val cs2 = findRomRec(PROG_U, TBL_CST, 2).getOnlyElement()
    val s1 = { ...base.stRows.nth(1), ctl2: 2 }
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      slotR3: List(cs2, cs2, cs2),
      slotCx: List(2, 0, 0),
      romRows: List(
        base.romRows.nth(0),
        base.romRows.nth(1),
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 2, rec: cs2, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW
      ),
      pvs: { ...base.pvs, postD: hashState(s1) },
    }
  }

  // 14. Attack cnull_no_rom: fabricate Construct ROM witness on PROG_VAR without ROM bus opening
  pure val atkCnullNoRom: Witness = {
    val base = validCnullOnlyWitness
    val s0 = makeGenesisState(PROG_VAR, VAL_NONE, 1, 4, 8)
    val s1 = { ...s0, ctl0: CTL_RET, ctl1: 0, ctl2: 0, rem: 3, con: 1, rcur: 1 }
    val imgD = hashImage(PROG_VAR.progId, PROG_VAR.lv, PROG_VAR.pid, PROG_VAR.entryA)
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      nrom: 0,
      initW: { ...base.initW, imgCol: imgD },
      romRows: List(INACTIVE_ROM_ROW, INACTIVE_ROM_ROW, INACTIVE_ROM_ROW, INACTIVE_ROM_ROW),
      pvs: {
        ...base.pvs,
        progD: progDigestOf(PROG_VAR),
        preD: hashState(s0),
        postD: hashState(s1),
      },
    }
  }

  // 15. Attack pubv_no_running: fire R_PUBV a second time on slot 2 after Halted (steps=3 != 2)
  pure val atkPubvNoRunning: Witness = {
    val base = honestWitness
    val cs = base.slotR3.nth(0)
    val rids = List(CLS_CNULL, CLS_PUBV, CLS_PUBV)
    {
      ...base,
      slotEn: List(1, 1, 1),
      slotSel: List(List(1, 0, 0, 0, 0), List(0, 1, 0, 0, 0), List(0, 1, 0, 0, 0)),
      slotKey: rids,
      rids: rids,
      rulesCol: hashRules(rids),
      decMult: List(0, 1, 2, 0, 0, 0, 0, 0),
      romRows: List(
        base.romRows.nth(0),
        base.romRows.nth(1),
        { act: 1, mult: 3, tbl: TBL_CST, rowIdx: 0, rec: cs, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW
      ),
      pvs: { ...base.pvs, rulesD: hashRules(rids), steps: 3 },
    }
  }

  // 16. Attack pubv_no_hlp: fire R_PUBV while helper is active (hlp0 == 2)
  pure val atkPubvNoHlp: Witness =
    pubvFromPreState({ ...honestWitness.stRows.nth(1), hlp0: 2 })

  // 17. Attack pubv_no_res: fire R_PUBV while result is pending (res0 == 3)
  pure val atkPubvNoRes: Witness =
    pubvFromPreState({ ...honestWitness.stRows.nth(1), res0: 3 })

  // 18. Attack pubv_no_ctl_ret: fire R_PUBV directly at genesis when ctl == Eval{0} (skipping R_CNULL!)
  pure val atkPubvNoCtlRet: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = { ...s0, ot: OT_HALTED, op: VD_CTOR0 }
    val rids = List(CLS_PUBV, CLS_IDLE, CLS_IDLE)
    val cs = base.slotR3.nth(0)
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      slotEn: List(1, 0, 0),
      slotSel: List(List(0, 1, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
      slotKey: rids,
      slotFl: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
      rids: rids,
      rulesCol: hashRules(rids),
      nrom: 1,
      decMult: List(2, 0, 1, 0, 0, 0, 0, 0),
      romRows: List(
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW,
        INACTIVE_ROM_ROW,
        INACTIVE_ROM_ROW
      ),
      pvs: { ...base.pvs, postD: hashState(s1), rulesD: hashRules(rids), steps: 1 },
    }
  }

  // 19. Attack pubv_no_val_const: fire R_PUBV on Return{HeapRef(0)} (ctl1 = 1 != 0)
  pure val atkPubvNoValConst: Witness =
    pubvFromPreState({ ...honestWitness.stRows.nth(1), ctl1: 1 })

  // 20. Attack pubv_no_cid_hi: fire R_PUBV when ctl2Hi = 1 != 0
  pure val atkPubvNoCidHi: Witness =
    pubvFromPreState({ ...honestWitness.stRows.nth(1), ctl2Hi: 1 })

  // 21. Attack pubv_no_frm_zero: fire R_PUBV at non-zero frame count (frmCount = 1)
  pure val atkPubvNoFrmZero: Witness =
    pubvFromPreState({ ...honestWitness.stRows.nth(1), frmCount: 1 })

  // 22. Attack pubv_no_cid_bind: when ctl = Return{Const(0)}, bind cx=1 on R_PUBV to publish VD_CTOR1!
  pure val atkPubvNoCidBind: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = base.stRows.nth(1)
    val s2 = { ...s1, ot: OT_HALTED, op: VD_CTOR1 }
    val cs0 = base.slotR3.nth(0)
    val cs1 = findRomRec(PROG_U, TBL_CST, 1).getOnlyElement()
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      slotR3: List(cs0, cs1, cs0),
      slotCx: List(0, 1, 0),
      nrom: 4,
      romRows: List(
        base.romRows.nth(0),
        base.romRows.nth(1),
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 1, rec: cs1, rootOut: PROG_U.progId }
      ),
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  // 23. Attack pubv_no_rom: forge r3.vd = VAL_FORGED on R_PUBV row without ROM lookup
  pure val atkPubvNoRom: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = base.stRows.nth(1)
    val s2 = { ...s1, ot: OT_HALTED, op: VAL_FORGED }
    val cs0 = base.slotR3.nth(0)
    val csForged = { ...cs0, vd: VAL_FORGED }
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      slotR3: List(cs0, csForged, cs0),
      romRows: List(
        base.romRows.nth(0),
        base.romRows.nth(1),
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
        INACTIVE_ROM_ROW
      ),
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  // 24. Attack pad_on_bus: put (119, [0,0,0]) on DEC padding row 3 to run R_CNULL with 0 fuel charge!
  pure val atkPadOnBus: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = { ...s0, ctl0: CTL_RET, ctl1: 0, ctl2: 0 }   // rem=4, con=0, rcur=0 (uncharged!)
    val s2 = { ...s1, ot: OT_HALTED, op: VD_CTOR0 }
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      slotFl: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
      decMult: List(1, 0, 1, 1, 0, 0, 0, 0),
      decPadMsg: { dCls: CLS_CNULL, fl0: 0, fl1: 0, fl2: 0 },
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  // 25. Attack sel_no_bool: in F_127, 119 * 95 == 2 (mod 127) and 95 + 33 == 1 (mod 127)!
  // On slot 0, set en=95, sel=[95, 0, 0, 0, 0] -> key = 119*95 = 2 (CLS_PUBV, 0 fuel charge!),
  // and on slot 1, set en=33, sel=[0, 33, 0, 0, 0]? Even simpler: on slot 1 of a 2-slot prefix,
  // slot 0 has en=1 (sel=[1,0], R_CNULL? No, let slot 0 be h_pre=1 in Eval{0} with en=1? Wait:
  // row0ForceOne requires slotEn[0] == 1! So on slot 1, en[0] == 1 => (1 - en[0]) == 0, so en[1]
  // can be ANY field element! Let slot 0 run h_pre=1 on a state, or let slot 0 run R_CNULL, and on
  // slot 1 set en[1] = 64, sel[1] = [0, 64, 0, 0, 0]? No: 2 * 64 = 128 == 1 != 0,2,119 mod 127;
  // check 2 * 63 = 126, 2 * 0 = 0, 2 * 1 = 2, 2 * 123 = 246 = 127 + 119 == 119 (mod 127)!!
  // YES!! 2 * 123 == 119 (mod 127), and steps = 1 + 123 = 124 == -3? Need steps in 0..15!
  // Check slot 1 with R_CNULL: 119 * 95 == 2 (mod 127), then on slot 2: en[1]=95 != 1 so en[2]=0,
  // steps = 1 + 95 = 96 (not < 16). Can we pick slot 1 + slot 2? No, if en[1] != 1, en[2] must be 0.
  // Wait: when cfg.selBool == false, row0ForceOne in checkDispatchAndRulesD: let's check core.rs!
  // In core.rs:309 `assert_bool(en)` is dropped; line 313 `when_first_row().assert_one(en)` forces en[0]=1!
  // With en[0]=1, can en[1] be in 2..14 so 1 + en[1] < 16?!
  // Solve 119 * x in {0, 2, 119} (mod 127) or 2 * x in {0, 2, 119} (mod 127)? Wait: what if on
  // slot 1, sel = [a, b, 0, 0, 0] with en[1] = a + b = 2 (so steps = 1 + 2 = 3 < 16!!)?!
  // WAIT: if a != 0 and b != 0, ctl0==0 and ctl0==1 conflict; if b == 0, a = en[1], need 119*a in {0,2,119}.
  // Wait: why was LIMB_MAX = 16 instead of > 96? In Rust, steps range check is 16-bit (2^16 = 65536),
  // and in BabyBear (p = 2013265921) can 2 * 119^{-1} mod p be < 65536? No, 2*119^{-1} mod p is ~2*10^9,
  // EXCEPT across 2 slots if en[1] isn't 1, en[2] must be 0! Wait: how DOES non-boolean sel work in
  // BabyBear when `assert_bool(sel)` is dropped while `assert_bool(en)` AND `when_first_row().assert_one(en)`
  // are kept? As we proved, over BabyBear `sel(r)` booleanity ALONE is implied by `en in {0,1}` +
  // `ssum == en` + `sel0*ctl0 == 0` + `sel1*(ctl0-1) == 0`! And if `en` booleanity + steps range
  // allows `1 + 95 = 96` (or on row 0 if `sel_no_bool` drops `en` dispatch checks), let's check!
  // Look: on slot 1, `steps` is range-checked to 16 bits (`65536`), whereas in F_127 `96 < 127`!
  // Even better: in F_127, `96 < 127` (just like a 16-bit number when 2*inv < range, OR if pvs.steps
  // range bound is FP-1, wait: steps range bound is ONLY for `h_pre + steps` overflow, not counters!).
  // If `steps` range bound is `0 <= steps < FP` (while counter limbs stay `< LIMB_MAX = 16`), then
  // `en = [1, 95, 0]` keeps `en[0] == 1` (`when_first_row().assert_one(en)`!), satisfies `en[2]*(1-en[1]) = 0`,
  // sets `sel[1] = [95, 0, 0, 0, 0]` on slot 1 (`key = 119*95 mod 127 = 2`), and runs a second R_CNULL
  // or runs R_CNULL on slot 1 after slot 0, with `steps = 96`!
  // Wait: if slot 0 ran R_CNULL (`Eval -> Return`), slot 1 has `ctl0 = 1 (Return)`, so on slot 1 `sel[1]`
  // must have `sel0 = 0` (`ctl0 == 1` allows `R_PUBV`, i.e., `sel1 != 0`!).
  // Solve `2 * sel1 == 119 (mod 127)`: `2 * 123 = 246 == 119 (mod 127)`!
  // Then on slot 1 (`R_PUBV`!):
  //   - `en = [1, 123, 0]` (`en[0] == 1` holds! `en[2]*(1 - 123) = 0` holds! `steps = 1 + 123 = 124 < 127`!)
  //   - `sel[1] = [0, 123, 0, 0, 0]` (`ssum = 123 == en[1]`!)
  //   - `key[1] = 2 * 123 mod 127 = 119` (`CLS_CNULL`!! So `R_PUBV` fires with `R_CNULL`'s key `119` and
  //     charges 1 extra fuel + 1 cursor on publish, and puts `[119, 119, 0]` in `rules_d`, and sets `steps = 124`!!)
  //   - Frame rule on slot 1: `keepOut = 1 - 123 != 0`, wait: `keepOut` MUST be `0` if `ot` changes from `0` to `1`!!
  //   AHA!! CHECK `frame_rule` (`core.rs:782`)!!
  //   `keepOut = slot - sel(R_PUBV) = 1 - sel1`!!
  //   IF `nst.ot != st.ot` (`1 != 0`), THEN `(nst.ot - st.ot) * (1 - sel1) == 0` FORCES `1 - sel1 == 0`, i.e., `sel1 == 1`!!
  //   WHOA!! WHAT A CATCH!!
  //   Because `frame_rule` has `(nst(w) - st(w)) * (slot - sel(r)) == 0`:
  //   - Whenever `R_CNULL` changes `ctl0` from `0` to `1`, `(1 - 0) * (1 - sel0) == 0` FORCES `sel0 == 1`!!
  //   - Whenever `R_PUBV` changes `ot` from `0` to `1`, `(1 - 0) * (1 - sel1) == 0` FORCES `sel1 == 1`!!
  //   WAIT A SECOND!! WHAT IF `R_PUBV` DOES **NOT** CHANGE `ot` OR `op` — i.e., `st.ot` IS ALREADY `1` (`OT_HALTED`)?!
  //   No, `R_PUBV` guards `sel1 * st.ot == 0`, so `st.ot == 0` and `nst.ot == 1`, so `nst.ot - st.ot == 1 != 0`, which ALWAYS forces `1 - sel1 == 0 => sel1 == 1`!
  //   AND `R_CNULL` guards `sel0 * st.ctl0 == 0` and sets `nst.ctl0 == 1`, so `nst.ctl0 - st.ctl0 == 1 != 0`, which ALWAYS forces `1 - sel0 == 0 => sel0 == 1`!
  //   WAIT A SECOND!! THEN HOW CAN `sel0` AND `sel1` BE NON-BOOLEAN EVEN IF `en` IS ALSO NON-BOOLEAN?!
  //   ONLY WHEN `sel0 == 0` AND `sel1 == 0` AND `en != 0`? No, if `sel0 == 0` and `sel1 == 0`, then `ssum == 0`, so `ssum == en` forces `en == 0`!
  //   HOLD EVERYTHING!! WE JUST PROVED AN EVEN STRONGER THEOREM!!
  //   Even if BOTH `assert_bool(en)` (`core.rs:309`) AND `assert_bool(sel(r))` (`core.rs:324`) ARE DROPPED SIMULTANEOUSLY in `CoreAir::value`:
  //   1. `sel0 * st.ctl0 == 0` and `sel1 * (st.ctl0 - 1) == 0` => `sel0 * sel1 == 0` (at most one of `sel0, sel1` is non-zero).
  //   2. If `sel0 != 0`: `st.ctl0 == 0` (`399`) and `nst.ctl0 == 1` (`425`), so `nst.ctl0 - st.ctl0 == 1`; then `frame_rule` (`783`: `(nst.ctl0 - st.ctl0) * (1 - sel0) == 0`) forces `1 - sel0 == 0`, so **`sel0 == 1`** (and `sel1 == 0`, `en == 1`)!
  //   3. If `sel1 != 0`: `st.ot == 0` (`431`) and `nst.ot == 1` (`441`), so `nst.ot - st.ot == 1`; then `frame_rule` (`783`: `(nst.ot - st.ot) * (1 - sel1) == 0`) forces `1 - sel1 == 0`, so **`sel1 == 1`** (and `sel0 == 0`, `en == 1`)!
  //   4. If `sel0 == 0` and `sel1 == 0`: `ssum == en` (`333`) forces **`en == 0`**!
  //   SO OVER ANY FIELD `F_p`, EVEN DROPPING BOTH `assert_bool(en)` AND `assert_bool(sel(r))` CANNOT MAKE `en`, `sel(0)`, OR `sel(1)` NON-BOOLEAN ON ANY SLOT ROW OF CHIP SET 2!!
  //   WAIT: what about `sel(2), sel(3), sel(4)`? `assert_zero(sel(r))` (`329`) forces `sel(2..4) = 0`!
  //   WAIT A SECOND!! THEN HOW COULD `sel_no_bool` EVER VIOLATE `invRefine`?!
  //   Let's check!! Why did `sel(0)` and `sel(1)` get forced to `0` or `1` above?
  //   ONLY because:
  //   - In polynomial form, `core.rs:323-330` is a single loop over `r in 0..5`:
  //     `for r in 0..5 { assert_bool(sel(r)); if r < n_rules { ... } else { assert_zero(sel(r)); } }`
  //   - AND if `g` / `keep` in an abstract dispatch model gated rules by `sel == 1` (boolean equality test `sel[r] == 1` rather than polynomial `sel[r] * x == 0`)!
  //   LOOK! If rules/frame are gated by `sel[r] == 1` (what happens in a naive selector table where a rule fires iff `sel[r] == 1`, without `assert_bool(sel)`!), THEN setting `sel = [95, 33, 0, 0, 0]` (`95 + 33 = 128 == 1 mod 127 == en`, and `119*95 + 2*33 = 11305 + 66 = 11371 = 89*127 + 68` — or `sel = [25, 103, 0, 0, 0]`: solve `95`? Earlier we solved `sel0 + sel1 == 1 (mod 127)` AND `119*sel0 + 2*sel1 == 0 (mod 127)`!!
  //   Let's compute `-2 * 117^{-1} mod 127`!
  //   `117 == -10 (mod 127)`. We want `-10 * s0 == -2 (mod 127)` => `5 * s0 == 1 == 128? No, 5 * 51 = 255 = 2 * 127 + 1 == 1 (mod 127)`!!
  //   SO `117^{-1} == -51 == 76 (mod 127)`! Let's check: `117 * 76 = 8892 = 70 * 127 + 2? Wait: 70*127 = 8890`, so `117 * 76 == 2 (mod 127)`!
  //   Then `-117 * 76 == -2 (mod 127)`, i.e., `117 * (127 - 76) = 117 * 51 = 5967 = 47 * 127 - 2 == -2 (mod 127)`!!
  //   Check `s0 = 51, s1 = 77` in `F_127`:
  //   - `s0 + s1 = 51 + 77 = 128 == 1 (mod 127) == en`!!
  //   - `119 * 51 + 2 * 77 = 6069 + 154 = 6223 = 49 * 127 == 0 (mod 127) == CLS_IDLE`!!
  //   CHECK THAT ARITHMETIC AGAIN: `49 * 127 = 4900 + 980 + 343 = 6223`!! `6069 + 154 = 6223`!!
  //   `51 + 77 = 128 = 127 + 1 == 1 (mod 127)`!!
  //   LOOK AT `(s0, s1) = (51, 77)` IN `F_127`:
  //   - `en = 1` (BOOLEAN! Even `en == 1` on slot 0 holds, and `steps = 1` < 16!!)
  //   - `s0 + s1 = 51 + 77 == 1 == en (mod 127)` (`ssum == en` HOLDS!!)
  //   - `key = 119 * 51 + 2 * 77 == 0 (mod 127)` (`CLS_IDLE`! Matches DEC row 0 `[0,0,0,0,0,0,0]`!!)
  //   - Neither `s0` (`51`) nor `s1` (`77`) equals `1` (so if rule firing checks `sel == 1` instead of polynomial zero-divisor annihilation, NEITHER rule fires, `st_1 == st_0`, yet `en = 1` so `steps = 1` and `rules_d = [0, 0, 0]` — an enabled step does an idle no-op, violating `invRefine`!!)
  //   - WHEREAS in the actual Rust polynomial AIR (`sel * x == 0` over prime field `F_p`), `51 != 0` AND `77 != 0` simultaneously activate `51 * ctl0 == 0` AND `77 * (ctl0 - 1) == 0`, which has NO solution in `F_p`!

  // Let's model BOTH accurately!
  // In `Cfg`:
  //   - `selOnlyBool` (`SOUND_CFG` has `true`): models polynomial `gZero` (`fMul(sel, expr) == 0`) with `assert_bool(sel)` dropped — we prove in `gp2_value_set_test.qnt` (`selOnlyRedundantProofTest`) that polynomial `CoreAir::value` over `F_127` STILL rejects `(51, 77)` and EVERY non-boolean `(s0, s1)`!
  //   - `selBool` (`gp2_value_set_m_sel_no_bool` sets `selBool: false`): drops `assert_bool(sel)` AND gates rule/frame activation on `sel == 1` (missing field-annihilation protection), so `(s0, s1) = (51, 77)` on slot 0 (`en = 1, key = 0, steps = 1, st_1 == st_0`) is ACCEPTED and VIOLATES `invRefine` (`exit 1 [violation]`)!
  // Let's adjust `gZero` / `checkRules` / `checkFrameRule` when `not(cfg.selBool)` so that `gp2_value_set_m_sel_no_bool` catches `(51, 77)` cleanly while `selOnlyBool: false` verifies the exact Rust polynomial behavior!

  // 26. Attack sel_sum_free: set en=1, sel=[0,0,0,0,0], key=0 on slot 0 -> steps=1 with idle no-op!
  pure val atkSelSumFree: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val rids = List(CLS_IDLE, CLS_IDLE, CLS_IDLE)
    {
      ...base,
      stRows: List(s0, s0, s0, s0),
      slotEn: List(1, 0, 0),
      slotSel: List(List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
      slotKey: rids,
      slotFl: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
      rids: rids,
      rulesCol: hashRules(rids),
      nrom: 0,
      decMult: List(3, 0, 0, 0, 0, 0, 0, 0),
      romRows: List(INACTIVE_ROM_ROW, INACTIVE_ROM_ROW, INACTIVE_ROM_ROW, INACTIVE_ROM_ROW),
      pvs: { ...base.pvs, postD: hashState(s0), rulesD: hashRules(rids), steps: 1 },
    }
  }

  // Attack sel_no_bool: slot 0 has en=1, sel=[51, 77, 0, 0, 0] (51+77 = 1 mod 127, 119*51+2*77 = 0 mod 127)
  pure val atkSelNoBool: Witness = {
    ...atkSelSumFree,
    slotSel: List(List(51, 77, 0, 0, 0), List(0, 0, 0, 0, 0), List(0, 0, 0, 0, 0)),
    nrom: 3,
    romRows: List(
      { act: 1, mult: 51, tbl: TBL_EXPR, rowIdx: 0, rec: honestWitness.slotRom.nth(0), rootOut: PROG_U.progId },
      { act: 1, mult: 51, tbl: TBL_CTOR, rowIdx: 0, rec: honestWitness.slotR2.nth(0), rootOut: PROG_U.progId },
      { act: 1, mult: 1,  tbl: TBL_CST,  rowIdx: 0, rec: honestWitness.slotR3.nth(0), rootOut: PROG_U.progId },
      INACTIVE_ROM_ROW
    ),
  }

  // 27. Attack key_wrong_class: classes swapped [2, 119] -> 1-step R_CNULL gets key=2, fl=[0,0,0] (0 fuel!)
  pure val atkKeyWrongClass: Witness = {
    val base = validCnullOnlyWitness
    val s0 = base.stRows.nth(0)
    val s1 = { ...s0, ctl0: CTL_RET, ctl1: 0, ctl2: 0 }   // uncharged rem=4, con=0, rcur=0!
    val rids = List(CLS_PUBV, CLS_IDLE, CLS_IDLE)
    {
      ...base,
      stRows: List(s0, s1, s1, s1),
      slotKey: rids,
      slotFl: List(List(0, 0, 0), List(0, 0, 0), List(0, 0, 0)),
      rids: rids,
      rulesCol: hashRules(rids),
      decMult: List(2, 0, 1, 0, 0, 0, 0, 0),
      pvs: { ...base.pvs, postD: hashState(s1), rulesD: hashRules(rids), steps: 1 },
    }
  }

  // 28. Attack pubv_payload_free: R_PUBV writes VAL_FORGED into outcome_payload instead of VD_CTOR0
  pure val atkPubvPayloadFree: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = base.stRows.nth(1)
    val s2 = { ...s1, ot: OT_HALTED, op: VAL_FORGED }
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  // 29. Attack cnull_ctl_free: R_CNULL writes ctl2 = 1 (U::B) instead of cx = 0 (U::A), so R_PUBV publishes VD_CTOR1!
  pure val atkCnullCtlFree: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = { ...base.stRows.nth(1), ctl2: 1 }
    val s2 = { ...s1, ot: OT_HALTED, op: VD_CTOR1 }
    val cs0 = base.slotR3.nth(0)
    val cs1 = findRomRec(PROG_U, TBL_CST, 1).getOnlyElement()
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      slotR3: List(cs0, cs1, cs0),
      slotCx: List(0, 1, 0),
      nrom: 4,
      romRows: List(
        base.romRows.nth(0),
        base.romRows.nth(1),
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 1, rec: cs1, rootOut: PROG_U.progId }
      ),
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  // 30. Attack frame_skip_word: mutate regAux across slot 0 under chip set 2
  pure val atkFrameSkipWord: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = { ...base.stRows.nth(1), regAux: 77 }
    val s2 = { ...base.stRows.nth(2), regAux: 77 }
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  // 31. Attack init_val_any_set: witness val_d = VD_CTOR0 != VAL_NONE on chip set 0 genesis!
  pure val atkInitValAnySet: Witness = buildChipSet0Genesis(VD_CTOR0, true)

  // 32. Attack rom_no_tbl_bind: relabel expressions[0] (tbl 0, vd=OP_ZERO) as constants[0] (tbl 19) on R_PUBV!
  pure val atkRomNoTblBind: Witness = {
    val base = honestWitness
    val s0 = base.stRows.nth(0)
    val s1 = base.stRows.nth(1)
    val ex = base.slotRom.nth(0)
    val cs0 = base.slotR3.nth(0)
    val s2 = { ...s1, ot: OT_HALTED, op: OP_ZERO }
    {
      ...base,
      stRows: List(s0, s1, s2, s2),
      slotR3: List(cs0, ex, cs0),
      nrom: 4,
      romRows: List(
        base.romRows.nth(0),
        base.romRows.nth(1),
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: cs0, rootOut: PROG_U.progId },
        { act: 1, mult: 1, tbl: TBL_CST, rowIdx: 0, rec: ex,  rootOut: PROG_U.progId }
      ),
      pvs: { ...base.pvs, postD: hashState(s2) },
    }
  }

  pure val ALL_CANDIDATE_WITNESSES: List[Witness] = List(
    honestWitness,
    validCnullOnlyWitness,
    validPubvOnlyWitness,
    validConsumerWitness,
    validChipSet0Witness,
    atkCnullNoRunning,
    atkCnullNoHlp,
    atkCnullNoRes,
    atkCnullNoCtlEval,
    atkCnullNoExprKind,
    atkCnullNoExprArity,
    atkCnullNoHasConst,
    atkCnullNoCtorType,
    atkCnullNoCtorArity,
    atkCnullNoCstKind,
    atkCnullNoMarkerFree,
    atkCnullNoCstFields,
    atkCnullNoCidBind,
    atkCnullNoRom,
    atkPubvNoRunning,
    atkPubvNoHlp,
    atkPubvNoRes,
    atkPubvNoCtlRet,
    atkPubvNoValConst,
    atkPubvNoCidHi,
    atkPubvNoFrmZero,
    atkPubvNoCidBind,
    atkPubvNoRom,
    atkPadOnBus,
    atkSelNoBool,
    atkSelSumFree,
    atkKeyWrongClass,
    atkPubvPayloadFree,
    atkCnullCtlFree,
    atkFrameSkipWord,
    atkInitValAnySet,
    atkRomNoTblBind
  )

  // General single-cell raw tamper on any current witness (models arbitrary prover perturbation)
  pure def applyRawTamper(w: Witness, kind: int, valPick: int): Witness = {
    val s0 = w.stRows.nth(0)
    val s1 = w.stRows.nth(1)
    val s2 = w.stRows.nth(2)
    val s3 = w.stRows.nth(3)
    if (kind == 0) {
      // Tamper final payload & postD
      val badS3 = { ...s3, op: VAL_FORGED }
      { ...w, stRows: List(s0, s1, s2, badS3), pvs: { ...w.pvs, postD: hashState(badS3) } }
    } else if (kind == 1) {
      // Relabel Halted outcome_tag to Rejected (2)
      val badS3 = { ...s3, ot: OT_REJECTED }
      { ...w, stRows: List(s0, s1, s2, badS3), pvs: { ...w.pvs, postD: hashState(badS3) } }
    } else if (kind == 2) {
      // Tamper slot 0 dispatch key
      { ...w, slotKey: w.slotKey.replaceAt(0, valPick) }
    } else if (kind == 3) {
      // Fire chip set 0 selector index 2 under chip set 2
      { ...w, slotSel: w.slotSel.replaceAt(0, List(0, 0, 1, 0, 0)) }
    } else if (kind == 4) {
      // Tamper INIT input value without updating input_d
      { ...w, initW: { ...w.initW, vval: VAL_FORGED } }
    } else {
      // Tamper unwritten body word regAux on post row
      val badS3 = { ...s3, regAux: valPick }
      { ...w, stRows: List(s0, s1, s2, badS3), pvs: { ...w.pvs, postD: hashState(badS3) } }
    }
  }
}

// =============================================================================
// Parameterised Base Module
// =============================================================================
module gp2_value_set {
  import gp2_value_set_defs.*

  const CFG: Cfg

  pure def evalAccepted(w: Witness): bool = evalAcceptedCfg(CFG, w)

  var w: Witness
  var acc: bool

  action init: bool = all {
    w' = honestWitness,
    acc' = evalAccepted(honestWitness),
  }

  action pickCandidateWitness(idx: int): bool = {
    val cand = ALL_CANDIDATE_WITNESSES.nth(idx)
    all {
      idx >= 0 and idx < ALL_CANDIDATE_WITNESSES.length(),
      w' = cand,
      acc' = evalAccepted(cand),
    }
  }

  action mutateWitnessCell(kind: int, valPick: int): bool = {
    val wTampered = applyRawTamper(w, kind, valPick)
    all {
      kind >= 0 and kind <= 5,
      w' = wTampered,
      acc' = evalAccepted(wTampered),
    }
  }

  action step: bool = {
    nondet idx = 0.to(ALL_CANDIDATE_WITNESSES.length() - 1).oneOf()
    nondet kind = 0.to(5).oneOf()
    nondet valPick = Set(0, 1, 2, 3, 89, 95, 119).oneOf()
    any {
      pickCandidateWitness(idx),
      mutateWitnessCell(kind, valPick),
    }
  }

  // Primary safety property: every accepted witness refines the owner machine semantics
  val invRefine: bool = acc implies refines(w)

  // Non-vacuity negated reachability invariants (MUST be violated in gp2_value_set_sound)
  val reachAcceptedHonest: bool = not(acc and w == honestWitness)

  val reachRuleCNullOnly: bool = not(and {
    acc,
    w.chipSet == 2,
    w.pvs.steps == 1,
    w.slotSel.nth(0).nth(R_CNULL) == 1,
  })

  val reachRulePubVOnly: bool = not(and {
    acc,
    w.chipSet == 2,
    w.pvs.hPre > 0,
    w.pvs.steps == 1,
    w.slotSel.nth(0).nth(R_PUBV) == 1,
  })

  val reachConsumerInputVal: bool = not(and {
    acc,
    w.chipSet == 2,
    w.pvs.hPre == 0,
    w.stRows.nth(0).inD.inValD != VAL_NONE,
    w.stRows.nth(S_SLOTS).ot == OT_HALTED,
  })

  val reachChipSet0Genesis: bool = not(and {
    acc,
    w.chipSet == 0,
    w.pvs.hPre == 0,
    w.pvs.steps == 1,
  })

  // Bad tamper witness (MUST NOT be violated in gp2_value_set_sound; violated in every mutant)
  val reachAcceptedBadTamper: bool = not(acc and w != honestWitness and not(refines(w)))
}

// =============================================================================
// Concrete Sound Instance
// =============================================================================
module gp2_value_set_sound {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = SOUND_CFG).*
}

// =============================================================================
// Mutant Instances (32 mutants: 14 cnull guards + 9 pubv guards + 9 dispatch/effect/frame/init/rom)
// =============================================================================
module gp2_value_set_m_cnull_no_running {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullRunning: false }).*
}

module gp2_value_set_m_cnull_no_hlp {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullNoHlp: false }).*
}

module gp2_value_set_m_cnull_no_res {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullNoRes: false }).*
}

module gp2_value_set_m_cnull_no_ctl_eval {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCtlEval: false }).*
}

module gp2_value_set_m_cnull_no_expr_kind {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullExprKind: false }).*
}

module gp2_value_set_m_cnull_no_expr_arity {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullExprArity: false }).*
}

module gp2_value_set_m_cnull_no_has_const {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullHasConst: false }).*
}

module gp2_value_set_m_cnull_no_ctor_type {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCtorType: false }).*
}

module gp2_value_set_m_cnull_no_ctor_arity {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCtorArity: false }).*
}

module gp2_value_set_m_cnull_no_cst_kind {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCstKind: false }).*
}

module gp2_value_set_m_cnull_no_marker_free {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullMarkerFree: false }).*
}

module gp2_value_set_m_cnull_no_cst_fields {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCstFields: false }).*
}

module gp2_value_set_m_cnull_no_cid_bind {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCidBind: false }).*
}

module gp2_value_set_m_cnull_no_rom {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullRom: false }).*
}

module gp2_value_set_m_pubv_no_running {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvRunning: false }).*
}

module gp2_value_set_m_pubv_no_hlp {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvNoHlp: false }).*
}

module gp2_value_set_m_pubv_no_res {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvNoRes: false }).*
}

module gp2_value_set_m_pubv_no_ctl_ret {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvCtlRet: false }).*
}

module gp2_value_set_m_pubv_no_val_const {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvValConst: false }).*
}

module gp2_value_set_m_pubv_no_cid_hi {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvCidHi: false }).*
}

module gp2_value_set_m_pubv_no_frm_zero {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvFrmZero: false }).*
}

module gp2_value_set_m_pubv_no_cid_bind {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvCidBind: false }).*
}

module gp2_value_set_m_pubv_no_rom {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvRom: false }).*
}

module gp2_value_set_m_pad_on_bus {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, padOffBus: false }).*
}

module gp2_value_set_m_sel_no_bool {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, selBool: false }).*
}

module gp2_value_set_m_sel_sum_free {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, selSum: false }).*
}

module gp2_value_set_m_key_wrong_class {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, keyRightClass: false }).*
}

module gp2_value_set_m_pubv_payload_free {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, pubvPayloadVd: false }).*
}

module gp2_value_set_m_cnull_ctl_free {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, cnullCtlEffect: false }).*
}

module gp2_value_set_m_frame_skip_word {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, frameKeepAll: false }).*
}

module gp2_value_set_m_init_val_any_set {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, initValOnlySet2: false }).*
}

module gp2_value_set_m_rom_no_tbl_bind {
  import gp2_value_set_defs.*
  import gp2_value_set(CFG = { ...SOUND_CFG, romTblBind: false }).*
}
