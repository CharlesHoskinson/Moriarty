// -*- mode: quint; -*-
// Soundness and completeness model of the Sherlock E2 L_Fr tape machine:
// - Write-once memory bus (MEM_BUS, tape.rs) and shared ProgramTableAir (PROG_BUS, program.rs)
// - InputAir (tape.rs), FrInputAir, ReshapeAir, AssertEqAir, BitsAir, FrAddAir,
//   HalveAir, CSwapAir, PublicAir (machine.rs), and Halve-chain bit decomposition (lverify.rs).
//
// Small-prime stand-in parameters:
//   P_MOD  = 5   (stand-in for BabyBear p = 2^31 - 2^27 + 1)
//   BITS_W = 3   (stand-in for BITS_N = 31 in BitsAir, so 2^BITS_W = 8)
//   R_MOD  = 11  (stand-in for BLS12-381 scalar field r, prime with P_MOD < 2^BITS_W < R_MOD)

module lfr_tape_base {
  // =========================================================================
  // 1. Constraint configuration parameters (true in sound instance, false in mutant)
  // =========================================================================
  const C_PROG_BUS: bool
  const C_MEM_BUS: bool
  const C_PIN_ADDR_MULT: bool
  const C_INPUT_CANON: bool
  const C_INPUT_BB_ZERO_HI: bool
  const C_BITS_BOOL: bool
  const C_BITS_HIGH_CHECK: bool
  const C_BITS_MASK_MONO: bool
  const C_ASSERT_EQ: bool
  const C_FR_ADD: bool
  const C_HALVE_BIT_BOOL: bool
  const C_HALVE_TERM_ZERO: bool
  const C_CSWAP_BIT_BOOL: bool
  const C_CSWAP_R_GATE: bool
  const C_PUBLIC_PIN_ORDER: bool

  // =========================================================================
  // 2. Small-prime stand-in constants and domains
  // =========================================================================
  pure val P_MOD: int = 5
  pure val BITS_W: int = 3
  pure val TWO_POW_BITS_W: int = 8
  pure val R_MOD: int = 11
  // Modular inverse of 2 mod 11: 2 * 6 = 12 == 1 (mod 11)
  pure val INV2_MOD_R: int = 6

  pure val F_DOM: Set[int] = 0.to(10)
  pure val BB_DOM: Set[int] = 0.to(4)
  pure val BIT_DOM: Set[int] = Set(0, 1)

  // Program bus tags (matching crates/sherlock-lfr/src/program.rs:37-49)
  pure val TAG_INPUT: int = 1
  pure val TAG_FR_INPUT: int = 2
  pure val TAG_RESHAPE: int = 7
  pure val TAG_ASSERT_EQ: int = 8
  pure val TAG_BITS: int = 9
  pure val TAG_FR_ADD: int = 10
  pure val TAG_HALVE: int = 11
  pure val TAG_CSWAP: int = 12
  pure val TAG_PUBLIC: int = 13

  // =========================================================================
  // 3. Types for rows and machine state
  // =========================================================================
  type Config = {
    progBus: bool,
    memBus: bool,
    pinAddrMult: bool,
    inputCanon: bool,
    inputBbZeroHi: bool,
    bitsBool: bool,
    bitsHighCheck: bool,
    bitsMaskMono: bool,
    assertEqGate: bool,
    frAddGate: bool,
    halveBitBool: bool,
    halveTermZero: bool,
    cswapBitBool: bool,
    cswapRGate: bool,
    publicPinOrder: bool,
  }

  // InputAir row: program [addr, mult, z1, isConst, k0, k1]; main [v0, v1]
  // (2-cell stand-in for 4-cell [v0, v1, v2, v3]: v0 is base BabyBear, v1 is extension coeff)
  type InputRow = {
    addr: int,
    mult: int,
    z1: int,
    isConst: int,
    k0: int,
    k1: int,
    v0: int,
    v1: int,
  }

  // FrInputAir row: program [addr, mult, isConst, k0]; main [v0]
  type FrInputRow = {
    addr: int,
    mult: int,
    isConst: int,
    k0: int,
    v0: int,
  }

  // BitsAir row: program [addrX, addrLow, multLow, nBits, active];
  // main [xVal, b0, b1, b2, s0, s1, s2, lowVal]
  type BitsRow = {
    addrX: int,
    addrLow: int,
    multLow: int,
    nBits: int,
    active: int,
    xVal: int,
    b0: int,
    b1: int,
    b2: int,
    s0: int,
    s1: int,
    s2: int,
    lowVal: int,
  }

  // HalveAir row: program [addrX, addrBit, addrRest, multBit, multRest, active];
  // main [xVal, bitVal, restVal]
  type HalveRow = {
    addrX: int,
    addrBit: int,
    addrRest: int,
    multBit: int,
    multRest: int,
    active: int,
    xVal: int,
    bitVal: int,
    restVal: int,
  }

  // FrAddAir row: program [addrA, addrB, addrOut, multOut, active]; main [xVal, yVal, zVal]
  type FrAddRow = {
    addrA: int,
    addrB: int,
    addrOut: int,
    multOut: int,
    active: int,
    xVal: int,
    yVal: int,
    zVal: int,
  }

  // CSwapAir row: program [addrBit, addrA, addrB, addrL, addrR, multL, multR, active];
  // main [bitVal, aVal, bVal, lVal, rVal]
  type CSwapRow = {
    addrBit: int,
    addrA: int,
    addrB: int,
    addrL: int,
    addrR: int,
    multL: int,
    multR: int,
    active: int,
    bitVal: int,
    aVal: int,
    bVal: int,
    lVal: int,
    rVal: int,
  }

  // ReshapeAir row: program [addrEf, addrBb0, addrBb1, comp, decomp, multEf, multBb0, multBb1];
  // main [v0, v1]
  type ReshapeRow = {
    addrEf: int,
    addrBb0: int,
    addrBb1: int,
    comp: int,
    decomp: int,
    multEf: int,
    multBb0: int,
    multBb1: int,
    v0: int,
    v1: int,
  }

  // AssertEqAir row: program [addrA, addrB, active];
  // main [v0, v1] (plus [w0, w1] to model the mutant where the two reads do not share cells)
  type AssertEqRow = {
    addrA: int,
    addrB: int,
    active: int,
    v0: int,
    v1: int,
    w0: int,
    w1: int,
  }

  // PublicAir row: program [addr0, addr1, active]; main [v0, v1]
  type PublicRow = {
    addr0: int,
    addr1: int,
    active: int,
    v0: int,
    v1: int,
  }

  type WitnessState = {
    inRow0: InputRow,
    inRow1: InputRow,
    inRow2: InputRow,
    inRow3: InputRow,
    inPad0: InputRow,
    inPad1: InputRow,
    frIn0: FrInputRow,
    frIn1: FrInputRow,
    frIn2: FrInputRow,
    bits0: BitsRow,
    halve0: HalveRow,
    halve1: HalveRow,
    frAdd0: FrAddRow,
    cswap0: CSwapRow,
    reshape0: ReshapeRow,
    assert0: AssertEqRow,
    assert1: AssertEqRow,
    pub0: PublicRow,
    pubPad: PublicRow,
    pv0: int,
    pv1: int,
    stepTag: str,
  }

  // =========================================================================
  // 4. Modular arithmetic and helper pure functions
  // =========================================================================
  pure def modR(x: int): int = {
    val rem = x % R_MOD
    if (rem < 0) rem + R_MOD else rem
  }

  pure def isBool(x: int): bool = (x == 0 or x == 1)

  pure def isCanonBb(x: int): bool = (x >= 0 and x < P_MOD)

  pure def isFrElem(x: int): bool = (x >= 0 and x < R_MOD)

  pure def pow2(k: int): int =
    if (k <= 0) 1
    else if (k == 1) 2
    else if (k == 2) 4
    else 8

  pure val activeConfig: Config = {
    progBus: C_PROG_BUS,
    memBus: C_MEM_BUS,
    pinAddrMult: C_PIN_ADDR_MULT,
    inputCanon: C_INPUT_CANON,
    inputBbZeroHi: C_INPUT_BB_ZERO_HI,
    bitsBool: C_BITS_BOOL,
    bitsHighCheck: C_BITS_HIGH_CHECK,
    bitsMaskMono: C_BITS_MASK_MONO,
    assertEqGate: C_ASSERT_EQ,
    frAddGate: C_FR_ADD,
    halveBitBool: C_HALVE_BIT_BOOL,
    halveTermZero: C_HALVE_TERM_ZERO,
    cswapBitBool: C_CSWAP_BIT_BOOL,
    cswapRGate: C_CSWAP_R_GATE,
    publicPinOrder: C_PUBLIC_PIN_ORDER,
  }

  // =========================================================================
  // 5. Key-fixed program rows (ProgramTableAir preprocessed trace)
  // =========================================================================
  pure val keyInRow0: InputRow = {
    addr: 1, mult: 3, z1: 1, isConst: 1, k0: 3, k1: 0, v0: 3, v1: 0
  }
  pure val keyInRow1: InputRow = {
    addr: 2, mult: 1, z1: 1, isConst: 0, k0: 0, k1: 0, v0: 3, v1: 0
  }
  pure val keyInRow2: InputRow = {
    addr: 16, mult: 1, z1: 1, isConst: 0, k0: 0, k1: 0, v0: 2, v1: 0
  }
  pure val keyInRow3: InputRow = {
    addr: 17, mult: 1, z1: 1, isConst: 0, k0: 0, k1: 0, v0: 1, v1: 0
  }
  pure val keyInPad: InputRow = {
    addr: 0, mult: 0, z1: 0, isConst: 0, k0: 0, k1: 0, v0: 0, v1: 0
  }

  pure val honestState: WitnessState = {
    inRow0: keyInRow0,
    inRow1: keyInRow1,
    inRow2: keyInRow2,
    inRow3: keyInRow3,
    inPad0: keyInPad,
    inPad1: keyInPad,
    frIn0: { addr: 6, mult: 2, isConst: 1, k0: 4, v0: 4 },
    frIn1: { addr: 7, mult: 2, isConst: 0, k0: 0, v0: 5 },
    frIn2: { addr: 13, mult: 1, isConst: 1, k0: 0, v0: 0 },
    bits0: {
      addrX: 1, addrLow: 3, multLow: 0, nBits: 1, active: 1,
      xVal: 3, b0: 1, b1: 1, b2: 0, s0: 1, s1: 0, s2: 0, lowVal: 1
    },
    halve0: {
      addrX: 1, addrBit: 4, addrRest: 5, multBit: 0, multRest: 1, active: 1,
      xVal: 3, bitVal: 1, restVal: 1
    },
    halve1: {
      addrX: 5, addrBit: 11, addrRest: 12, multBit: 0, multRest: 1, active: 1,
      xVal: 1, bitVal: 1, restVal: 0
    },
    frAdd0: {
      addrA: 6, addrB: 7, addrOut: 8, multOut: 1, active: 1,
      xVal: 4, yVal: 5, zVal: 9
    },
    cswap0: {
      addrBit: 17, addrA: 6, addrB: 7, addrL: 9, addrR: 10, multL: 1, multR: 0, active: 1,
      bitVal: 1, aVal: 4, bVal: 5, lVal: 5, rVal: 4
    },
    reshape0: {
      addrEf: 16, addrBb0: 14, addrBb1: 15, comp: 0, decomp: 1,
      multEf: 0, multBb0: 0, multBb1: 0, v0: 2, v1: 0
    },
    assert0: {
      addrA: 1, addrB: 2, active: 1, v0: 3, v1: 0, w0: 3, w1: 0
    },
    assert1: {
      addrA: 12, addrB: 13, active: 1, v0: 0, v1: 0, w0: 0, w1: 0
    },
    pub0: {
      addr0: 8, addr1: 9, active: 1, v0: 9, v1: 5
    },
    pubPad: {
      addr0: 0, addr1: 0, active: 0, v0: 0, v1: 0
    },
    pv0: 9,
    pv1: 5,
    stepTag: "init",
  }

  // =========================================================================
  // 6. Per-AIR gate checks and bus balance checks
  // =========================================================================

  // InputAir row check (tape.rs:548-578)
  pure def checkInputRow(cfg: Config, r: InputRow): bool = and {
    isFrElem(r.v0),
    isFrElem(r.v1),
    if (cfg.inputCanon) (isCanonBb(r.v0) and isCanonBb(r.v1)) else true,
    if (cfg.inputBbZeroHi) (modR(r.z1 * r.v1) == 0) else true,
    modR(r.isConst * (r.v0 - r.k0)) == 0,
    modR(r.isConst * (r.v1 - r.k1)) == 0,
  }

  // FrInputAir row check (machine.rs:155-162)
  pure def checkFrInputRow(r: FrInputRow): bool = and {
    isFrElem(r.v0),
    modR(r.isConst * (r.v0 - r.k0)) == 0,
  }

  // BitsAir row check (machine.rs:661-695)
  pure def checkBitsRow(cfg: Config, r: BitsRow): bool = {
    val bBool = if (cfg.bitsBool) (isBool(r.b0) and isBool(r.b1) and isBool(r.b2)) else true
    val sBool = isBool(r.s0) and isBool(r.s1) and isBool(r.s2)
    val sMono = if (cfg.bitsMaskMono) (
      modR(r.s1 * (1 - r.s0)) == 0 and modR(r.s2 * (1 - r.s1)) == 0
    ) else true
    val acc = modR(r.b0 + 2 * r.b1 + 4 * r.b2)
    val low = modR(r.s0 * r.b0 + 2 * r.s1 * r.b1 + 4 * r.s2 * r.b2)
    val sumS = modR(r.s0 + r.s1 + r.s2)
    and {
      isFrElem(r.xVal),
      isFrElem(r.b0),
      isFrElem(r.b1),
      isFrElem(r.b2),
      isFrElem(r.lowVal),
      bBool,
      sBool,
      sMono,
      if (cfg.bitsHighCheck) (r.xVal == acc) else true,
      r.lowVal == low,
      sumS == modR(r.nBits),
    }
  }

  // HalveAir row check (machine.rs:822-846)
  pure def checkHalveRow(cfg: Config, r: HalveRow): bool = and {
    isFrElem(r.xVal),
    isFrElem(r.bitVal),
    isFrElem(r.restVal),
    if (cfg.halveBitBool) isBool(r.bitVal) else true,
    r.xVal == modR(2 * r.restVal + r.bitVal),
  }

  // FrAddAir row check (machine.rs:745-768)
  pure def checkFrAddRow(cfg: Config, r: FrAddRow): bool = and {
    isFrElem(r.xVal),
    isFrElem(r.yVal),
    isFrElem(r.zVal),
    if (cfg.frAddGate) (r.zVal == modR(r.xVal + r.yVal)) else true,
  }

  // CSwapAir row check (machine.rs:905-934)
  pure def checkCSwapRow(cfg: Config, r: CSwapRow): bool = and {
    isFrElem(r.bitVal),
    isFrElem(r.aVal),
    isFrElem(r.bVal),
    isFrElem(r.lVal),
    isFrElem(r.rVal),
    if (cfg.cswapBitBool) isBool(r.bitVal) else true,
    r.lVal == modR(r.aVal + r.bitVal * (r.bVal - r.aVal)),
    if (cfg.cswapRGate) (r.rVal == modR(r.bVal + r.bitVal * (r.aVal - r.bVal))) else true,
  }

  // ReshapeAir row check (machine.rs:525-543)
  pure def checkReshapeRow(r: ReshapeRow): bool = and {
    isFrElem(r.v0),
    isFrElem(r.v1),
  }

  // AssertEqAir row check (machine.rs:581-587)
  // In the real AIR, both MEM_BUS reads use the same witness columns v[0..4];
  // when enforceEq is false, the second read is allowed to use independent w0, w1.
  pure def checkAssertEqRow(enforceEq: bool, r: AssertEqRow): bool = and {
    isFrElem(r.v0),
    isFrElem(r.v1),
    isFrElem(r.w0),
    isFrElem(r.w1),
    if (enforceEq) (r.v0 == r.w0 and r.v1 == r.w1) else true,
  }

  // PublicAir row check (machine.rs:984-999)
  pure def checkPublicRow(cfg: Config, r: PublicRow, pv0: int, pv1: int): bool = and {
    isFrElem(r.v0),
    isFrElem(r.v1),
    isFrElem(pv0),
    isFrElem(pv1),
    if (cfg.publicPinOrder) (
      modR(r.active * (r.v0 - pv0)) == 0 and modR(r.active * (r.v1 - pv1)) == 0
    ) else true,
  }

  // PROG_BUS check (program.rs:72-140):
  // Every execution row's program columns must match the key's ProgramTableAir rows.
  pure def inputProgMatchesKey(cfg: Config, r: InputRow, k: InputRow): bool = and {
    if (cfg.pinAddrMult) (r.addr == k.addr and r.mult == k.mult) else true,
    r.z1 == k.z1,
    r.isConst == k.isConst,
    r.k0 == k.k0,
    r.k1 == k.k1,
  }

  pure def padProgMatchesNop(cfg: Config, r: InputRow): bool =
    if (cfg.progBus) (
      r.addr == 0 and r.mult == 0 and r.z1 == 0 and r.isConst == 0 and r.k0 == 0 and r.k1 == 0
    ) else true

  pure def checkProgBus(cfg: Config, s: WitnessState): bool = and {
    inputProgMatchesKey(cfg, s.inRow0, keyInRow0),
    inputProgMatchesKey(cfg, s.inRow1, keyInRow1),
    inputProgMatchesKey(cfg, s.inRow2, keyInRow2),
    inputProgMatchesKey(cfg, s.inRow3, keyInRow3),
    padProgMatchesNop(cfg, s.inPad0),
    padProgMatchesNop(cfg, s.inPad1),
    s.frIn0.addr == 6 and s.frIn0.mult == 2 and s.frIn0.isConst == 1 and s.frIn0.k0 == 4,
    s.frIn1.addr == 7 and s.frIn1.mult == 2 and s.frIn1.isConst == 0 and s.frIn1.k0 == 0,
    s.frIn2.addr == 13 and s.frIn2.mult == 1 and s.frIn2.isConst == 1 and s.frIn2.k0 == 0,
    s.bits0.addrX == 1 and s.bits0.addrLow == 3 and s.bits0.multLow == 0 and s.bits0.nBits == 1 and s.bits0.active == 1,
    s.halve0.addrX == 1 and s.halve0.addrBit == 4 and s.halve0.addrRest == 5 and s.halve0.multBit == 0 and s.halve0.multRest == 1 and s.halve0.active == 1,
    s.halve1.addrX == 5 and s.halve1.addrBit == 11 and s.halve1.addrRest == 12 and s.halve1.multBit == 0 and s.halve1.multRest == 1 and s.halve1.active == 1,
    s.frAdd0.addrA == 6 and s.frAdd0.addrB == 7 and s.frAdd0.addrOut == 8 and s.frAdd0.multOut == 1 and s.frAdd0.active == 1,
    s.cswap0.addrBit == 17 and s.cswap0.addrA == 6 and s.cswap0.addrB == 7 and s.cswap0.addrL == 9 and s.cswap0.addrR == 10 and s.cswap0.multL == 1 and s.cswap0.multR == 0 and s.cswap0.active == 1,
    s.reshape0.addrEf == 16 and s.reshape0.addrBb0 == 14 and s.reshape0.addrBb1 == 15 and s.reshape0.comp == 0 and s.reshape0.decomp == 1 and s.reshape0.multEf == 0 and s.reshape0.multBb0 == 0 and s.reshape0.multBb1 == 0,
    s.assert0.addrA == 1 and s.assert0.addrB == 2 and s.assert0.active == 1,
    s.assert1.addrA == 12 and s.assert1.addrB == 13 and s.assert1.active == 1,
    if (cfg.publicPinOrder) (
      s.pub0.addr0 == 8 and s.pub0.addr1 == 9 and s.pub0.active == 1
    ) else (
      s.pub0.active == 1
    ),
    s.pubPad.addr0 == 0 and s.pubPad.addr1 == 0 and s.pubPad.active == 0,
  }

  // MEM_BUS net count in Fr (mod R_MOD) for a message (addr, c0, c1)
  pure def msgMatch(a: int, x0: int, x1: int, qAddr: int, q0: int, q1: int, cnt: int): int =
    if (a == qAddr and x0 == q0 and x1 == q1) cnt else 0

  pure def memNetCount(s: WitnessState, qAddr: int, q0: int, q1: int): int = {
    val wIn0 = msgMatch(s.inRow0.addr, s.inRow0.v0, s.inRow0.v1, qAddr, q0, q1, s.inRow0.mult)
    val wIn1 = msgMatch(s.inRow1.addr, s.inRow1.v0, s.inRow1.v1, qAddr, q0, q1, s.inRow1.mult)
    val wIn2 = msgMatch(s.inRow2.addr, s.inRow2.v0, s.inRow2.v1, qAddr, q0, q1, s.inRow2.mult)
    val wIn3 = msgMatch(s.inRow3.addr, s.inRow3.v0, s.inRow3.v1, qAddr, q0, q1, s.inRow3.mult)
    val wPad0 = msgMatch(s.inPad0.addr, s.inPad0.v0, s.inPad0.v1, qAddr, q0, q1, s.inPad0.mult)
    val wPad1 = msgMatch(s.inPad1.addr, s.inPad1.v0, s.inPad1.v1, qAddr, q0, q1, s.inPad1.mult)
    val wFr0 = msgMatch(s.frIn0.addr, s.frIn0.v0, 0, qAddr, q0, q1, s.frIn0.mult)
    val wFr1 = msgMatch(s.frIn1.addr, s.frIn1.v0, 0, qAddr, q0, q1, s.frIn1.mult)
    val wFr2 = msgMatch(s.frIn2.addr, s.frIn2.v0, 0, qAddr, q0, q1, s.frIn2.mult)
    val rBitsX = msgMatch(s.bits0.addrX, s.bits0.xVal, 0, qAddr, q0, q1, -s.bits0.active)
    val wBitsL = msgMatch(s.bits0.addrLow, s.bits0.lowVal, 0, qAddr, q0, q1, s.bits0.multLow)
    val rH0X = msgMatch(s.halve0.addrX, s.halve0.xVal, 0, qAddr, q0, q1, -s.halve0.active)
    val wH0B = msgMatch(s.halve0.addrBit, s.halve0.bitVal, 0, qAddr, q0, q1, s.halve0.multBit)
    val wH0R = msgMatch(s.halve0.addrRest, s.halve0.restVal, 0, qAddr, q0, q1, s.halve0.multRest)
    val rH1X = msgMatch(s.halve1.addrX, s.halve1.xVal, 0, qAddr, q0, q1, -s.halve1.active)
    val wH1B = msgMatch(s.halve1.addrBit, s.halve1.bitVal, 0, qAddr, q0, q1, s.halve1.multBit)
    val wH1R = msgMatch(s.halve1.addrRest, s.halve1.restVal, 0, qAddr, q0, q1, s.halve1.multRest)
    val rAddA = msgMatch(s.frAdd0.addrA, s.frAdd0.xVal, 0, qAddr, q0, q1, -s.frAdd0.active)
    val rAddB = msgMatch(s.frAdd0.addrB, s.frAdd0.yVal, 0, qAddr, q0, q1, -s.frAdd0.active)
    val wAddZ = msgMatch(s.frAdd0.addrOut, s.frAdd0.zVal, 0, qAddr, q0, q1, s.frAdd0.multOut)
    val rCsBit = msgMatch(s.cswap0.addrBit, s.cswap0.bitVal, 0, qAddr, q0, q1, -s.cswap0.active)
    val rCsA = msgMatch(s.cswap0.addrA, s.cswap0.aVal, 0, qAddr, q0, q1, -s.cswap0.active)
    val rCsB = msgMatch(s.cswap0.addrB, s.cswap0.bVal, 0, qAddr, q0, q1, -s.cswap0.active)
    val wCsL = msgMatch(s.cswap0.addrL, s.cswap0.lVal, 0, qAddr, q0, q1, s.cswap0.multL)
    val wCsR = msgMatch(s.cswap0.addrR, s.cswap0.rVal, 0, qAddr, q0, q1, s.cswap0.multR)
    val rwResEf = msgMatch(
      s.reshape0.addrEf, s.reshape0.v0, s.reshape0.v1, qAddr, q0, q1,
      s.reshape0.comp * s.reshape0.multEf - s.reshape0.decomp
    )
    val rwResBb0 = msgMatch(
      s.reshape0.addrBb0, s.reshape0.v0, 0, qAddr, q0, q1,
      s.reshape0.decomp * s.reshape0.multBb0 - s.reshape0.comp
    )
    val rwResBb1 = msgMatch(
      s.reshape0.addrBb1, s.reshape0.v1, 0, qAddr, q0, q1,
      s.reshape0.decomp * s.reshape0.multBb1 - s.reshape0.comp
    )
    val rEq0A = msgMatch(s.assert0.addrA, s.assert0.v0, s.assert0.v1, qAddr, q0, q1, -s.assert0.active)
    val rEq0B = msgMatch(s.assert0.addrB, s.assert0.w0, s.assert0.w1, qAddr, q0, q1, -s.assert0.active)
    val rEq1A = msgMatch(s.assert1.addrA, s.assert1.v0, s.assert1.v1, qAddr, q0, q1, -s.assert1.active)
    val rEq1B = msgMatch(s.assert1.addrB, s.assert1.w0, s.assert1.w1, qAddr, q0, q1, -s.assert1.active)
    val rPub0 = msgMatch(s.pub0.addr0, s.pub0.v0, 0, qAddr, q0, q1, -s.pub0.active)
    val rPub1 = msgMatch(s.pub0.addr1, s.pub0.v1, 0, qAddr, q0, q1, -s.pub0.active)
    val rPubPad0 = msgMatch(s.pubPad.addr0, s.pubPad.v0, 0, qAddr, q0, q1, -s.pubPad.active)
    val rPubPad1 = msgMatch(s.pubPad.addr1, s.pubPad.v1, 0, qAddr, q0, q1, -s.pubPad.active)
    modR(
      wIn0 + wIn1 + wIn2 + wIn3 + wPad0 + wPad1 +
      wFr0 + wFr1 + wFr2 +
      rBitsX + wBitsL +
      rH0X + wH0B + wH0R + rH1X + wH1B + wH1R +
      rAddA + rAddB + wAddZ +
      rCsBit + rCsA + rCsB + wCsL + wCsR +
      rwResEf + rwResBb0 + rwResBb1 +
      rEq0A + rEq0B + rEq1A + rEq1B +
      rPub0 + rPub1 + rPubPad0 + rPubPad1
    )
  }

  // All messages emitted by any row in s must have net count 0 mod R_MOD on MEM_BUS
  pure def allEmittedMessages(s: WitnessState): Set[(int, int, int)] = Set(
    (s.inRow0.addr, s.inRow0.v0, s.inRow0.v1),
    (s.inRow1.addr, s.inRow1.v0, s.inRow1.v1),
    (s.inRow2.addr, s.inRow2.v0, s.inRow2.v1),
    (s.inRow3.addr, s.inRow3.v0, s.inRow3.v1),
    (s.inPad0.addr, s.inPad0.v0, s.inPad0.v1),
    (s.inPad1.addr, s.inPad1.v0, s.inPad1.v1),
    (s.frIn0.addr, s.frIn0.v0, 0),
    (s.frIn1.addr, s.frIn1.v0, 0),
    (s.frIn2.addr, s.frIn2.v0, 0),
    (s.bits0.addrX, s.bits0.xVal, 0),
    (s.bits0.addrLow, s.bits0.lowVal, 0),
    (s.halve0.addrX, s.halve0.xVal, 0),
    (s.halve0.addrBit, s.halve0.bitVal, 0),
    (s.halve0.addrRest, s.halve0.restVal, 0),
    (s.halve1.addrX, s.halve1.xVal, 0),
    (s.halve1.addrBit, s.halve1.bitVal, 0),
    (s.halve1.addrRest, s.halve1.restVal, 0),
    (s.frAdd0.addrA, s.frAdd0.xVal, 0),
    (s.frAdd0.addrB, s.frAdd0.yVal, 0),
    (s.frAdd0.addrOut, s.frAdd0.zVal, 0),
    (s.cswap0.addrBit, s.cswap0.bitVal, 0),
    (s.cswap0.addrA, s.cswap0.aVal, 0),
    (s.cswap0.addrB, s.cswap0.bVal, 0),
    (s.cswap0.addrL, s.cswap0.lVal, 0),
    (s.cswap0.addrR, s.cswap0.rVal, 0),
    (s.reshape0.addrEf, s.reshape0.v0, s.reshape0.v1),
    (s.reshape0.addrBb0, s.reshape0.v0, 0),
    (s.reshape0.addrBb1, s.reshape0.v1, 0),
    (s.assert0.addrA, s.assert0.v0, s.assert0.v1),
    (s.assert0.addrB, s.assert0.w0, s.assert0.w1),
    (s.assert1.addrA, s.assert1.v0, s.assert1.v1),
    (s.assert1.addrB, s.assert1.w0, s.assert1.w1),
    (s.pub0.addr0, s.pub0.v0, 0),
    (s.pub0.addr1, s.pub0.v1, 0),
    (s.pubPad.addr0, s.pubPad.v0, 0),
    (s.pubPad.addr1, s.pubPad.v1, 0)
  )

  pure def checkMemBus(cfg: Config, s: WitnessState): bool =
    if (cfg.memBus)
      allEmittedMessages(s).forall(m => memNetCount(s, m._1, m._2, m._3) == 0)
    else true

  pure def witnessAccepted(cfg: Config, s: WitnessState): bool = and {
    checkProgBus(cfg, s),
    checkMemBus(cfg, s),
    checkInputRow(cfg, s.inRow0),
    checkInputRow(cfg, s.inRow1),
    checkInputRow(cfg, s.inRow2),
    checkInputRow(cfg, s.inRow3),
    checkInputRow(cfg, s.inPad0),
    checkInputRow(cfg, s.inPad1),
    checkFrInputRow(s.frIn0),
    checkFrInputRow(s.frIn1),
    checkFrInputRow(s.frIn2),
    checkBitsRow(cfg, s.bits0),
    checkHalveRow(cfg, s.halve0),
    checkHalveRow(cfg, s.halve1),
    checkFrAddRow(cfg, s.frAdd0),
    checkCSwapRow(cfg, s.cswap0),
    checkReshapeRow(s.reshape0),
    checkAssertEqRow(cfg.assertEqGate, s.assert0),
    checkAssertEqRow(cfg.halveTermZero, s.assert1),
    checkPublicRow(cfg, s.pub0, s.pv0, s.pv1),
    checkPublicRow(cfg, s.pubPad, s.pv0, s.pv1),
  }

  // =========================================================================
  // 7. Reference semantics of the straight-line tape program
  // =========================================================================
  pure def witnessRefines(s: WitnessState): bool = {
    // 1. Constants equal key values; inputs are canonical of their declared kind
    val const1Ok = (s.inRow0.v0 == 3 and s.inRow0.v1 == 0)
    val in2Ok = (isCanonBb(s.inRow1.v0) and s.inRow1.v1 == 0)
    val in16Ok = (isCanonBb(s.inRow2.v0) and s.inRow2.v1 == 0)
    val in17Ok = (isCanonBb(s.inRow3.v0) and s.inRow3.v1 == 0)
    val frConst6Ok = (s.frIn0.v0 == 4)
    val frIn7Ok = isFrElem(s.frIn1.v0)
    val frConst13Ok = (s.frIn2.v0 == 0)

    // 2. Write-once memory: every op reads the exact value written to its input address(es),
    //    and every op's output cells equal the reference function of its inputs.
    val xConst = 3
    val aFr = 4
    val bFr = s.frIn1.v0
    val cswapBit = s.inRow3.v0

    // AssertEq(1, 2): mem[1] == mem[2]
    val assertEq0Ok = (s.inRow1.v0 == xConst and s.inRow1.v1 == 0)

    // LowBits(addr 1, n = 1): low == xConst % 2^1 == 1, with 0 <= xConst < 2^BITS_W
    val bitsOk = (
      s.bits0.xVal == xConst and
      s.bits0.lowVal == (xConst % pow2(1))
    )

    // Halve chain of 2 steps on addr 1 ending in AssertEq(rest_1, 0):
    // bit0 == xConst % 2 == 1, rest0 == xConst / 2 == 1,
    // bit1 == rest0 % 2 == 1, rest1 == rest0 / 2 == 0, and xConst < 2^2
    val halveChainOk = (
      s.halve0.xVal == xConst and
      s.halve0.bitVal == (xConst % 2) and
      s.halve0.restVal == (xConst / 2) and
      s.halve1.xVal == (xConst / 2) and
      s.halve1.bitVal == ((xConst / 2) % 2) and
      s.halve1.restVal == 0
    )

    // FrAdd(addr 6, addr 7): z == (aFr + bFr) mod R_MOD
    val frAddOk = (
      s.frAdd0.xVal == aFr and
      s.frAdd0.yVal == bFr and
      s.frAdd0.zVal == modR(aFr + bFr)
    )

    // CSwap(addr 17, addr 6, addr 7): bit is boolean, (l, r) = (a, b) if 0 else (b, a)
    val cswapOk = (
      isBool(cswapBit) and
      s.cswap0.bitVal == cswapBit and
      s.cswap0.aVal == aFr and
      s.cswap0.bVal == bFr and
      s.cswap0.lVal == (if (cswapBit == 0) aFr else bFr) and
      s.cswap0.rVal == (if (cswapBit == 0) bFr else aFr)
    )

    // Reshape(Decompose, addr 16): outputs equal the 2 coefficients of mem[16]
    val reshapeOk = (
      isCanonBb(s.reshape0.v0) and
      s.reshape0.v0 == s.inRow2.v0 and
      s.reshape0.v1 == 0
    )

    // Public([addr 8, addr 9]): public values equal [mem[8], mem[9]] in exact order
    val publicOk = (
      s.pv0 == s.frAdd0.zVal and
      s.pv1 == s.cswap0.lVal
    )

    and {
      const1Ok,
      in2Ok,
      in16Ok,
      in17Ok,
      frConst6Ok,
      frIn7Ok,
      frConst13Ok,
      assertEq0Ok,
      bitsOk,
      halveChainOk,
      frAddOk,
      cswapOk,
      reshapeOk,
      publicOk,
    }
  }

  // =========================================================================
  // 8. Concrete attack witnesses for each of the 15 attack questions / mutants
  // =========================================================================

  // Attack 1 (Q1, mut_no_prog_bus, E2-RESULTS 8b):
  // Two InputAir padding rows forge (addr=1, [2,0], mult=+3) and (addr=1, [3,0], mult=-3 = 8 mod 11),
  // cancelling the honest constant 3 at addr 1 on MEM_BUS so readers of addr 1 see 2 instead of 3.
  pure val attackProgBusState: WitnessState = {
    ...honestState,
    inRow1: { ...honestState.inRow1, v0: 2 },
    inPad0: { addr: 1, mult: 3, z1: 1, isConst: 0, k0: 0, k1: 0, v0: 2, v1: 0 },
    inPad1: { addr: 1, mult: 8, z1: 1, isConst: 0, k0: 0, k1: 0, v0: 3, v1: 0 },
    bits0: { ...honestState.bits0, xVal: 2, b0: 0, b1: 1, b2: 0, lowVal: 0 },
    halve0: { ...honestState.halve0, xVal: 2, bitVal: 0, restVal: 1 },
    assert0: { ...honestState.assert0, v0: 2, w0: 2 },
    stepTag: "attack_prog_bus",
  }

  // Attack 2 (Q1, mut_no_mem_bus, i4_tape.rs control 1):
  // FrAddAir reads xVal = 5 instead of mem[6] = 4 (its own gate z = x + y holds with z = 10).
  pure val attackMemBusState: WitnessState = {
    ...honestState,
    frAdd0: { ...honestState.frAdd0, xVal: 5, yVal: 5, zVal: 10 },
    pub0: { ...honestState.pub0, v0: 10 },
    pv0: 10,
    stepTag: "attack_mem_bus",
  }

  // Attack 3 (Q1, mut_dup_writer):
  // Without unique writer / pinned (addr, mult) per writer row, InputAir row 0 sets mult = 1
  // (read by AssertEq(1,2) as 3) while InputAir row 2 sets addr = 1, mult = 2, v0 = 2
  // (read by BitsAir and HalveAir as 2, and ReshapeAir also reads [1, 2, 0] with mult = 3).
  pure val attackDupWriterState: WitnessState = {
    ...honestState,
    inRow0: { ...honestState.inRow0, mult: 1 },
    inRow2: { ...honestState.inRow2, addr: 1, mult: 3, v0: 2, v1: 0 },
    bits0: { ...honestState.bits0, xVal: 2, b0: 0, b1: 1, b2: 0, lowVal: 0 },
    halve0: { ...honestState.halve0, xVal: 2, bitVal: 0, restVal: 1 },
    reshape0: { ...honestState.reshape0, addrEf: 1, v0: 2, v1: 0 },
    stepTag: "attack_dup_writer",
  }

  // Attack 4 (Q2, mut_input_no_canonical, i4_tape.rs control 3):
  // InputAir row 2 (addr 16, ValKind::Bb) writes non-canonical v0 = 2 + P_MOD = 7 (< R_MOD = 11),
  // accepted by MEM_BUS and ReshapeAir when InputAir omits eval_canonical.
  pure val attackInputNonCanonState: WitnessState = {
    ...honestState,
    inRow2: { ...honestState.inRow2, v0: 7, v1: 0 },
    reshape0: { ...honestState.reshape0, v0: 7, v1: 0 },
    stepTag: "attack_input_non_canon",
  }

  // Attack 5 (Q2, mut_input_no_bb_zero_hi):
  // InputAir row 2 (addr 16, ValKind::Bb, z1 = 1) writes non-zero extension coeff v1 = 3,
  // smuggling an EF4 value where a scalar BabyBear value is required.
  pure val attackInputBbHiState: WitnessState = {
    ...honestState,
    inRow2: { ...honestState.inRow2, v0: 2, v1: 3 },
    reshape0: { ...honestState.reshape0, v0: 2, v1: 3 },
    stepTag: "attack_input_bb_hi",
  }

  // Attack 6 (Q3, mut_bits_no_bool):
  // BitsAir row 0 uses non-boolean bits b = [0, 7, 0] (0 + 2*7 + 4*0 = 14 == 3 mod 11),
  // so acc == xVal == 3 holds mod 11, but lowVal = s0*b0 = 0 != (3 % 2 = 1).
  pure val attackBitsNoBoolState: WitnessState = {
    ...honestState,
    bits0: { ...honestState.bits0, b0: 0, b1: 7, b2: 0, lowVal: 0 },
    stepTag: "attack_bits_no_bool",
  }

  // Attack 7 (Q3, mut_bits_no_high_check):
  // BitsAir row 0 omits x == acc (high part range check), setting boolean b = [0, 1, 0]
  // for xVal = 3 so lowVal = 0 != 1.
  pure val attackBitsNoHighState: WitnessState = {
    ...honestState,
    bits0: { ...honestState.bits0, b0: 0, b1: 1, b2: 0, lowVal: 0 },
    stepTag: "attack_bits_no_high",
  }

  // Attack 8 (Q3, mut_bits_no_mask_mono):
  // BitsAir row 0 omits s_{i+1} * (1 - s_i) = 0, setting s = [0, 0, 1] (boolean, sum == 1)
  // on xVal = 3 (b = [1, 1, 0]), yielding lowVal = 4 * b2 = 0 != 1.
  pure val attackBitsNoMonoState: WitnessState = {
    ...honestState,
    bits0: { ...honestState.bits0, s0: 0, s1: 0, s2: 1, lowVal: 0 },
    stepTag: "attack_bits_no_mono",
  }

  // Attack 9 (Q4, mut_assert_eq_unchecked):
  // InputAir row 1 (addr 2) is set to [4, 0] while addr 1 is [3, 0]; AssertEq(1, 2) accepts.
  pure val attackAssertEqState: WitnessState = {
    ...honestState,
    inRow1: { ...honestState.inRow1, v0: 4, v1: 0 },
    assert0: { ...honestState.assert0, v0: 3, v1: 0, w0: 4, w1: 0 },
    stepTag: "attack_assert_eq",
  }

  // Attack 10 (Q4, mut_fr_add_free_out):
  // FrAddAir row 0 outputs zVal = 10 instead of (4 + 5) mod 11 = 9.
  pure val attackFrAddState: WitnessState = {
    ...honestState,
    frAdd0: { ...honestState.frAdd0, zVal: 10 },
    pub0: { ...honestState.pub0, v0: 10 },
    pv0: 10,
    stepTag: "attack_fr_add",
  }

  // Attack 11 (Q4, mut_halve_no_bit_bool):
  // HalveAir row 0 sets non-boolean bitVal = 3, restVal = 0 (2*0 + 3 == 3),
  // and HalveAir row 1 sets xVal = 0, bitVal = 0, restVal = 0 (still satisfying AssertEq(rest1, 0)).
  pure val attackHalveNoBoolState: WitnessState = {
    ...honestState,
    halve0: { ...honestState.halve0, bitVal: 3, restVal: 0 },
    halve1: { ...honestState.halve1, xVal: 0, bitVal: 0, restVal: 0 },
    stepTag: "attack_halve_no_bool",
  }

  // Attack 12 (Q3 & Q4, mut_halve_no_terminal_zero):
  // Without the terminal AssertEq(rest1, 0), HalveAir row 0 flips bitVal to 0 (boolean!)
  // with restVal = 7 (2*7 + 0 = 14 == 3 mod 11), and HalveAir row 1 sets xVal = 7,
  // bitVal = 1 (boolean!), restVal = 3 (2*3 + 1 = 7), with assert1 reading v0 = 3, w0 = 0.
  pure val attackHalveNoTermState: WitnessState = {
    ...honestState,
    halve0: { ...honestState.halve0, bitVal: 0, restVal: 7 },
    halve1: { ...honestState.halve1, xVal: 7, bitVal: 1, restVal: 3 },
    assert1: { ...honestState.assert1, v0: 3, w0: 0 },
    stepTag: "attack_halve_terminal",
  }

  // Attack 13 (Q4, mut_cswap_no_bit_bool):
  // InputAir row 3 (addr 17, CSwap bit) is set to 2 (canonical Bb < 5, but non-boolean);
  // CSwapAir computes lVal = 4 + 2*(5-4) = 6 and rVal = 5 + 2*(4-5) = 3.
  pure val attackCSwapNoBoolState: WitnessState = {
    ...honestState,
    inRow3: { ...honestState.inRow3, v0: 2 },
    cswap0: { ...honestState.cswap0, bitVal: 2, lVal: 6, rVal: 3 },
    pub0: { ...honestState.pub0, v1: 6 },
    pv1: 6,
    stepTag: "attack_cswap_no_bool",
  }

  // Attack 14 (Q4, mut_cswap_free_r):
  // CSwapAir row 0 leaves rVal = 0 instead of 4 when the r gate is omitted.
  pure val attackCSwapFreeRState: WitnessState = {
    ...honestState,
    cswap0: { ...honestState.cswap0, rVal: 0 },
    stepTag: "attack_cswap_free_r",
  }

  // Attack 15 (Q5, mut_public_order_unpinned):
  // PublicAir exposes pv = [9, 9] (repeating addr 8's value 9 and skipping addr 9's value 5).
  pure val attackPublicOrderState: WitnessState = {
    ...honestState,
    pv0: 9,
    pv1: 9,
    stepTag: "attack_public_order",
  }

  // =========================================================================
  // 9. State variables, invariants, and transitions
  // =========================================================================
  var state: WitnessState
  var acc: bool

  val progBusOk: bool = checkProgBus(activeConfig, state)
  val memBusOk: bool = checkMemBus(activeConfig, state)

  // Safety invariant: every accepted witness refines the tape reference semantics
  val invRefine: bool = acc implies witnessRefines(state)

  // Negated reachability invariants (MUST be violated in lfr_tape_sound for non-vacuity)
  val reachHonestAccepted: bool = not(acc and witnessRefines(state))
  val reachTamperRefused: bool = not(state.stepTag != "init" and not(acc) and not(witnessRefines(state)))
  val reachForgedPadRefused: bool = not(state.stepTag == "attack_prog_bus" and not(acc) and memBusOk and not(progBusOk))
  val reachHalveFlipRefused: bool = not(state.stepTag == "attack_halve_terminal" and not(acc))
  val reachPublicSwapRefused: bool = not(state.stepTag == "attack_public_order" and not(acc))

  action assignCandidate(cand: WitnessState): bool = all {
    state' = cand,
    acc' = witnessAccepted(activeConfig, cand),
  }

  action init: bool = assignCandidate(honestState)

  action stepRandomTamper: bool = {
    nondet v = F_DOM.oneOf()
    nondet slot = 0.to(6).oneOf()
    val cand =
      if (slot == 0) { ...honestState, inRow1: { ...honestState.inRow1, v0: v }, stepTag: "rand_in1" }
      else if (slot == 1) { ...honestState, inRow2: { ...honestState.inRow2, v0: v }, stepTag: "rand_in2" }
      else if (slot == 2) { ...honestState, bits0: { ...honestState.bits0, lowVal: v }, stepTag: "rand_bits" }
      else if (slot == 3) { ...honestState, frAdd0: { ...honestState.frAdd0, zVal: v }, stepTag: "rand_fradd" }
      else if (slot == 4) { ...honestState, halve0: { ...honestState.halve0, bitVal: v }, stepTag: "rand_halve" }
      else if (slot == 5) { ...honestState, cswap0: { ...honestState.cswap0, lVal: v }, stepTag: "rand_cswap" }
      else { ...honestState, pv1: v, stepTag: "rand_pub" }
    assignCandidate(cand)
  }

  action step: bool = any {
    assignCandidate(attackProgBusState),
    assignCandidate(attackMemBusState),
    assignCandidate(attackDupWriterState),
    assignCandidate(attackInputNonCanonState),
    assignCandidate(attackInputBbHiState),
    assignCandidate(attackBitsNoBoolState),
    assignCandidate(attackBitsNoHighState),
    assignCandidate(attackBitsNoMonoState),
    assignCandidate(attackAssertEqState),
    assignCandidate(attackFrAddState),
    assignCandidate(attackHalveNoBoolState),
    assignCandidate(attackHalveNoTermState),
    assignCandidate(attackCSwapNoBoolState),
    assignCandidate(attackCSwapFreeRState),
    assignCandidate(attackPublicOrderState),
    stepRandomTamper,
  }
}

// ===========================================================================
// Concrete instances: sound module + 15 mutant modules
// ===========================================================================

module lfr_tape_sound {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Default module matching file stem lfr_tape.qnt
module lfr_tape {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 1 (Q1): InputAir padding rows not bound on PROG_BUS (forged write from two padding rows)
module mut_no_prog_bus {
  import lfr_tape_base(
    C_PROG_BUS = false,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 2 (Q1): MEM_BUS balance omitted (reader reads value different from written value)
module mut_no_mem_bus {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = false,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 3 (Q1): Writer (addr, mult) not pinned to unique-writer key (duplicate writer to addr 1)
module mut_dup_writer {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = false,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 4 (Q2): InputAir omits eval_canonical (non-canonical v0 + p accepted as BabyBear)
module mut_input_no_canonical {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = false,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 5 (Q2): InputAir omits z_k * v_k = 0 for k > 0 (BabyBear input carries EF4 high coeffs)
module mut_input_no_bb_zero_hi {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = false,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 6 (Q3): BitsAir omits assert_bool(b_i)
module mut_bits_no_bool {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = false,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 7 (Q3): BitsAir omits full recomposition x == acc (high part not range-checked)
module mut_bits_no_high_check {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = false,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 8 (Q3): BitsAir omits mask monotonicity s_{i+1} * (1 - s_i) = 0
module mut_bits_no_mask_mono {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = false,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 9 (Q4): AssertEqAir allows distinct read cells for addr_a and addr_b
module mut_assert_eq_unchecked {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = false,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 10 (Q4): FrAddAir omits z == x + y gate
module mut_fr_add_free_out {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = false,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 11 (Q4): HalveAir omits assert_bool(bit)
module mut_halve_no_bit_bool {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = false,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 12 (Q3 & Q4): Halve chain omits terminal AssertEq(rest, 0)
module mut_halve_no_terminal_zero {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = false,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 13 (Q4): CSwapAir omits assert_bool(bit)
module mut_cswap_no_bit_bool {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = false,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 14 (Q4): CSwapAir omits r == b + bit * (a - b) gate
module mut_cswap_free_r {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = false,
    C_PUBLIC_PIN_ORDER = true
  ).*
}

// Mutant 15 (Q5): PublicAir omits active * (v_i - pv_i) = 0 / ordered address binding
module mut_public_order_unpinned {
  import lfr_tape_base(
    C_PROG_BUS = true,
    C_MEM_BUS = true,
    C_PIN_ADDR_MULT = true,
    C_INPUT_CANON = true,
    C_INPUT_BB_ZERO_HI = true,
    C_BITS_BOOL = true,
    C_BITS_HIGH_CHECK = true,
    C_BITS_MASK_MONO = true,
    C_ASSERT_EQ = true,
    C_FR_ADD = true,
    C_HALVE_BIT_BOOL = true,
    C_HALVE_TERM_ZERO = true,
    C_CSWAP_BIT_BOOL = true,
    C_CSWAP_R_GATE = true,
    C_PUBLIC_PIN_ORDER = false
  ).*
}
