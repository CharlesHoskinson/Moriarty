// Soundness and completeness model of the X14 BLS12-381 G1 doubling condition row `INV_y`
// over F_13 (curve y^2 = x^3 + 4 mod 13, bus offset o = 1110).

module inv_y_chain {
  const HAS_INV_ROW: bool
  const INV_RECEIVES: bool
  const WRONG_DEN: bool
  const O10_HAS_ID: bool
  const INV_VALID_FREE: bool

  pure val P: int = 13
  pure val FP: Set[int] = 0.to(12)
  pure val CELLS: Set[int] = 0.to(14)

  pure def modp(a: int): int = ((a % P) + P) % P
  pure def addp(a: int, b: int): int = modp(a + b)
  pure def subp(a: int, b: int): int = modp(a - b)
  pure def mulp(a: int, b: int): int = modp(a * b)

  pure def invpMod(c: int): int =
    if (c == 1) 1
    else if (c == 2) 7
    else if (c == 3) 9
    else if (c == 4) 10
    else if (c == 5) 8
    else if (c == 6) 11
    else if (c == 7) 2
    else if (c == 8) 5
    else if (c == 9) 3
    else if (c == 10) 4
    else if (c == 11) 6
    else if (c == 12) 12
    else 0

  pure def invp(a: int): int = invpMod(modp(a))
  pure def inFp(a: int): bool = 0 <= a and a < P
  pure def isBool(b: int): bool = b == 0 or b == 1

  pure def onCurve(x: int, y: int): bool =
    inFp(x) and inFp(y) and mulp(y, y) == addp(mulp(mulp(x, x), x), 4)

  pure def honestSq(x: int): int = mulp(x, x)
  pure def honestLam(x: int, y: int): int = mulp(mulp(3, mulp(x, x)), invp(mulp(2, y)))
  pure def honestX3(lam: int, x: int): int = subp(subp(mulp(lam, lam), x), x)
  pure def honestY3(lam: int, x: int, x3: int, y: int): int = subp(mulp(lam, subp(x, x3)), y)

  type CircuitConfig = {
    has_inv_row: bool,
    inv_receives: bool,
    wrong_den: bool,
    o10_has_id: bool,
    inv_valid_free: bool,
  }

  pure val activeCfg: CircuitConfig = {
    has_inv_row: HAS_INV_ROW,
    inv_receives: INV_RECEIVES,
    wrong_den: WRONG_DEN,
    o10_has_id: O10_HAS_ID,
    inv_valid_free: INV_VALID_FREE,
  }

  pure val cfgSound: CircuitConfig = {
    has_inv_row: true,
    inv_receives: true,
    wrong_den: false,
    o10_has_id: true,
    inv_valid_free: false,
  }

  pure val cfgNoInvRow: CircuitConfig = {
    has_inv_row: false,
    inv_receives: false,
    wrong_den: false,
    o10_has_id: true,
    inv_valid_free: false,
  }

  pure val cfgNoRecv: CircuitConfig = {
    has_inv_row: true,
    inv_receives: false,
    wrong_den: false,
    o10_has_id: true,
    inv_valid_free: false,
  }

  pure val cfgWrongDen: CircuitConfig = {
    has_inv_row: true,
    inv_receives: true,
    wrong_den: true,
    o10_has_id: true,
    inv_valid_free: false,
  }

  pure val cfgNoId: CircuitConfig = {
    has_inv_row: true,
    inv_receives: true,
    wrong_den: false,
    o10_has_id: false,
    inv_valid_free: false,
  }

  pure val cfgInvValidFree: CircuitConfig = {
    has_inv_row: true,
    inv_receives: true,
    wrong_den: false,
    o10_has_id: true,
    inv_valid_free: true,
  }

  type ChainState = {
    pin_id: int,
    pin_x: int,
    pin_y: int,
    sq_valid: int,
    sq_id: int,
    sq_x: int,
    sq_s: int,
    ldbl_valid: int,
    ldbl_id: int,
    ldbl_s: int,
    ldbl_x: int,
    ldbl_y: int,
    ldbl_lam: int,
    invy_valid: int,
    invy_id: int,
    invy_y: int,
    invy_w: int,
    x3_valid: int,
    x3_id: int,
    x3_lam: int,
    x3_x1: int,
    x3_x2: int,
    x3_x3: int,
    y3_valid: int,
    y3_id: int,
    y3_lam: int,
    y3_x1: int,
    y3_x3: int,
    y3_y1: int,
    y3_y3: int,
    sink_id: int,
    sink_x3: int,
    sink_y3: int,
    ldbl1_valid: int,
    ldbl1_id: int,
    ldbl1_y: int,
    invy1_valid: int,
    invy1_id: int,
    invy1_y: int,
    invy1_w: int,
    bus_fault: int,
  }

  // Honest chain state: Row 0 doubles P0 = (2, 5) -> (12, 9); Row 1 doubles P1 = (4, 4) -> (2, 8)
  pure val honestState: ChainState = {
    pin_id: 0,
    pin_x: 2,
    pin_y: 5,
    sq_valid: 1,
    sq_id: 0,
    sq_x: 2,
    sq_s: 4,
    ldbl_valid: 1,
    ldbl_id: 0,
    ldbl_s: 4,
    ldbl_x: 2,
    ldbl_y: 5,
    ldbl_lam: 9,
    invy_valid: 1,
    invy_id: 0,
    invy_y: 5,
    invy_w: 8,
    x3_valid: 1,
    x3_id: 0,
    x3_lam: 9,
    x3_x1: 2,
    x3_x2: 2,
    x3_x3: 12,
    y3_valid: 1,
    y3_id: 0,
    y3_lam: 9,
    y3_x1: 2,
    y3_x3: 12,
    y3_y1: 5,
    y3_y3: 9,
    sink_id: 0,
    sink_x3: 12,
    sink_y3: 9,
    ldbl1_valid: 1,
    ldbl1_id: 1,
    ldbl1_y: 4,
    invy1_valid: 1,
    invy1_id: 1,
    invy1_y: 4,
    invy1_w: 10,
    bus_fault: 0,
  }

  pure def mkZeroYState(forged_lam: int, cand_y_inv: int, cand_w: int, iv_valid: int): ChainState = {
    ...honestState,
    pin_x: 0,
    pin_y: 0,
    sq_x: 0,
    sq_s: 0,
    ldbl_s: 0,
    ldbl_x: 0,
    ldbl_y: 0,
    ldbl_lam: forged_lam,
    invy_valid: iv_valid,
    invy_y: cand_y_inv,
    invy_w: cand_w,
    x3_lam: forged_lam,
    x3_x1: 0,
    x3_x2: 0,
    x3_x3: honestX3(forged_lam, 0),
    y3_lam: forged_lam,
    y3_x1: 0,
    y3_x3: honestX3(forged_lam, 0),
    y3_y1: 0,
    y3_y3: honestY3(forged_lam, 0, honestX3(forged_lam, 0), 0),
    sink_x3: honestX3(forged_lam, 0),
    sink_y3: honestY3(forged_lam, 0, honestX3(forged_lam, 0), 0),
  }

  pure val swappedIdState: ChainState = {
    ...honestState,
    invy_y: 4,
    invy_w: 10,
    invy1_y: 5,
    invy1_w: 8,
  }

  pure def mutateCell(c: int, v: int): ChainState =
    if (c == 0) { ...honestState, pin_id: v }
    else if (c == 1) { ...honestState, pin_x: v }
    else if (c == 2) { ...honestState, pin_y: v }
    else if (c == 3) { ...honestState, sq_valid: v }
    else if (c == 4) { ...honestState, sq_s: v }
    else if (c == 5) { ...honestState, ldbl_valid: v }
    else if (c == 6) { ...honestState, ldbl_lam: v }
    else if (c == 7) { ...honestState, invy_valid: v }
    else if (c == 8) { ...honestState, invy_id: v }
    else if (c == 9) { ...honestState, invy_y: v }
    else if (c == 10) { ...honestState, invy_w: v }
    else if (c == 11) { ...honestState, x3_x3: v }
    else if (c == 12) { ...honestState, y3_y3: v }
    else if (c == 13) { ...honestState, sink_x3: v }
    else { ...honestState, bus_fault: 1 }

  pure def airOk(cfg: CircuitConfig, st: ChainState): bool = and {
    inFp(st.pin_id), inFp(st.pin_x), inFp(st.pin_y), st.pin_id == 0,
    isBool(st.sq_valid), inFp(st.sq_id), inFp(st.sq_x), inFp(st.sq_s), st.sq_id == 0,
    st.sq_valid == 0 or st.sq_s == mulp(st.sq_x, st.sq_x),
    isBool(st.ldbl_valid), inFp(st.ldbl_id), inFp(st.ldbl_s), inFp(st.ldbl_x), inFp(st.ldbl_y), inFp(st.ldbl_lam), st.ldbl_id == 0,
    st.ldbl_valid == 0 or mulp(mulp(2, st.ldbl_y), st.ldbl_lam) == mulp(3, st.ldbl_s),
    isBool(st.ldbl1_valid), inFp(st.ldbl1_id), inFp(st.ldbl1_y), st.ldbl1_id == st.ldbl_id + 1,
    st.ldbl1_valid == 0 or st.ldbl1_y == 4,
    if (not(cfg.has_inv_row)) true else and {
      isBool(st.invy_valid), inFp(st.invy_id), inFp(st.invy_y), inFp(st.invy_w), st.invy_id == 0,
      st.invy_valid == 0 or (
        if (cfg.wrong_den) mulp(st.invy_w, addp(st.invy_y, 1)) == 1
        else mulp(st.invy_w, st.invy_y) == 1
      ),
      isBool(st.invy1_valid), inFp(st.invy1_id), inFp(st.invy1_y), inFp(st.invy1_w), st.invy1_id == st.invy_id + 1,
      st.invy1_valid == 0 or (
        if (cfg.wrong_den) mulp(st.invy1_w, addp(st.invy1_y, 1)) == 1
        else mulp(st.invy1_w, st.invy1_y) == 1
      ),
    },
    isBool(st.x3_valid), inFp(st.x3_id), inFp(st.x3_lam), inFp(st.x3_x1), inFp(st.x3_x2), inFp(st.x3_x3), st.x3_id == 0,
    st.x3_valid == 0 or st.x3_x3 == subp(subp(mulp(st.x3_lam, st.x3_lam), st.x3_x1), st.x3_x2),
    isBool(st.y3_valid), inFp(st.y3_id), inFp(st.y3_lam), inFp(st.y3_x1), inFp(st.y3_x3), inFp(st.y3_y1), inFp(st.y3_y3), st.y3_id == 0,
    st.y3_valid == 0 or st.y3_y3 == subp(mulp(st.y3_lam, subp(st.y3_x1, st.y3_x3)), st.y3_y1),
    inFp(st.sink_id), inFp(st.sink_x3), inFp(st.sink_y3),
  }

  pure def o10Ok(cfg: CircuitConfig, st: ChainState): bool =
    if (not(cfg.has_inv_row) or not(cfg.inv_receives)) true
    else if (cfg.o10_has_id) and {
      st.ldbl_valid == (if (cfg.inv_valid_free) 1 else st.invy_valid),
      (if (cfg.inv_valid_free) 1 else st.invy_valid) == 0 or (st.ldbl_id == st.invy_id and st.ldbl_y == st.invy_y),
      st.ldbl1_valid == (if (cfg.inv_valid_free) 1 else st.invy1_valid),
      (if (cfg.inv_valid_free) 1 else st.invy1_valid) == 0 or (st.ldbl1_id == st.invy1_id and st.ldbl1_y == st.invy1_y),
    } else or {
      and {
        st.ldbl_valid == (if (cfg.inv_valid_free) 1 else st.invy_valid),
        (if (cfg.inv_valid_free) 1 else st.invy_valid) == 0 or st.ldbl_y == st.invy_y,
        st.ldbl1_valid == (if (cfg.inv_valid_free) 1 else st.invy1_valid),
        (if (cfg.inv_valid_free) 1 else st.invy1_valid) == 0 or st.ldbl1_y == st.invy1_y,
      },
      and {
        st.ldbl_valid == (if (cfg.inv_valid_free) 1 else st.invy1_valid),
        (if (cfg.inv_valid_free) 1 else st.invy1_valid) == 0 or st.ldbl_y == st.invy1_y,
        st.ldbl1_valid == (if (cfg.inv_valid_free) 1 else st.invy_valid),
        (if (cfg.inv_valid_free) 1 else st.invy_valid) == 0 or st.ldbl1_y == st.invy_y,
      },
    }

  pure def busOk(cfg: CircuitConfig, st: ChainState): bool = and {
    st.bus_fault == 0,
    st.sq_valid == 1, st.pin_id == st.sq_id, st.pin_x == st.sq_x,
    st.ldbl_valid == 1, st.pin_id == st.ldbl_id, st.pin_y == st.ldbl_y,
    st.sq_valid == st.ldbl_valid, st.sq_id == st.ldbl_id, st.sq_s == st.ldbl_s, st.sq_x == st.ldbl_x,
    st.ldbl_valid == st.x3_valid, st.ldbl_id == st.x3_id, st.ldbl_lam == st.x3_lam, st.ldbl_x == st.x3_x1, st.ldbl_x == st.x3_x2,
    st.ldbl_valid == st.y3_valid, st.ldbl_id == st.y3_id, st.ldbl_lam == st.y3_lam, st.ldbl_x == st.y3_x1, st.ldbl_y == st.y3_y1,
    st.x3_valid == st.y3_valid, st.x3_id == st.y3_id, st.x3_x3 == st.y3_x3,
    st.y3_valid == 1, st.y3_id == st.sink_id, st.y3_x3 == st.sink_x3, st.y3_y3 == st.sink_y3,
    o10Ok(cfg, st),
  }

  pure def chainAccepted(cfg: CircuitConfig, st: ChainState): bool =
    airOk(cfg, st) and busOk(cfg, st)

  pure def checkInvRow(cfg: CircuitConfig, st: ChainState): bool =
    if (not(chainAccepted(cfg, st)) or st.ldbl_valid == 0) true
    else and {
      st.ldbl_y != 0,
      st.ldbl_x == st.pin_x,
      st.ldbl_y == st.pin_y,
      st.ldbl_lam == honestLam(st.pin_x, st.pin_y),
      st.sink_x3 == honestX3(honestLam(st.pin_x, st.pin_y), st.pin_x),
      st.sink_y3 == honestY3(honestLam(st.pin_x, st.pin_y), st.pin_x, honestX3(honestLam(st.pin_x, st.pin_y), st.pin_x), st.pin_y),
      st.invy_valid == 1,
      st.invy_id == st.ldbl_id,
      st.invy_y == st.ldbl_y,
      mulp(st.invy_w, st.ldbl_y) == 1,
    }

  var st: ChainState

  action init = st' = honestState
  action stepHonest = st' = honestState
  action stepZeroY = {
    nondet w = FP.oneOf()
    st' = mkZeroYState(5, 0, w, 1)
  }
  action stepZeroYInactive = {
    nondet w = FP.oneOf()
    st' = mkZeroYState(5, 0, w, 0)
  }
  action stepZeroYDecoupled = {
    nondet flam = FP.oneOf()
    st' = mkZeroYState(flam, 1, 1, 1)
  }
  // lead edit 2026-11-13: under wrong_den the honest second row also uses w = 1 / (y1 + 1) = 1 / 5 = 8 (mod 13)
  action stepZeroYWrongDen = st' = { ...mkZeroYState(5, 0, 1, 1), invy1_w: 8 }
  action stepSwapId = st' = swappedIdState
  action stepMutateCell = {
    nondet c = CELLS.oneOf()
    st' = mutateCell(c, 3)
  }
  action stepBusFault = st' = mutateCell(14, 1)

  action step =
    if (not(HAS_INV_ROW)) stepZeroY
    else if (not(INV_RECEIVES)) stepZeroYDecoupled
    else if (WRONG_DEN) stepZeroYWrongDen
    else if (not(O10_HAS_ID)) stepSwapId
    else if (INV_VALID_FREE) stepZeroYInactive
    else any {
      stepHonest,
      stepZeroY,
      stepZeroYInactive,
      stepZeroYDecoupled,
      stepZeroYWrongDen,
      stepSwapId,
      stepMutateCell,
      stepBusFault,
    }

  val invRow: bool = checkInvRow(activeCfg, st)
  val invNonVacuous: bool = not(chainAccepted(activeCfg, st) and st.pin_x == 2 and st.pin_y == 5)
}

module inv_y { import inv_y_chain(HAS_INV_ROW = true, INV_RECEIVES = true, WRONG_DEN = false, O10_HAS_ID = true, INV_VALID_FREE = false).* }
module m_no_inv_row { import inv_y_chain(HAS_INV_ROW = false, INV_RECEIVES = false, WRONG_DEN = false, O10_HAS_ID = true, INV_VALID_FREE = false).* }
module m_no_recv { import inv_y_chain(HAS_INV_ROW = true, INV_RECEIVES = false, WRONG_DEN = false, O10_HAS_ID = true, INV_VALID_FREE = false).* }
module m_wrong_den { import inv_y_chain(HAS_INV_ROW = true, INV_RECEIVES = true, WRONG_DEN = true, O10_HAS_ID = true, INV_VALID_FREE = false).* }
module m_no_id { import inv_y_chain(HAS_INV_ROW = true, INV_RECEIVES = true, WRONG_DEN = false, O10_HAS_ID = false, INV_VALID_FREE = false).* }
module m_inv_valid_free { import inv_y_chain(HAS_INV_ROW = true, INV_RECEIVES = true, WRONG_DEN = false, O10_HAS_ID = true, INV_VALID_FREE = true).* }
