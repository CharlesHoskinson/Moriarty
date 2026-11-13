// -*- mode: quint; -*-
// Deterministic, exhaustive pure checks, real-constant big-integer checks,
// completeness checks, and mutant non-vacuity / attack tests for lfr_poseidon.qnt.

module lfr_poseidon_test {
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ).* from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = false,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_PoseidonT from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = false,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_PoseidonY from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = false,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_PoseidonOut from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = false,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_ProgNoBus from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = false,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_ProgNoTag from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = false,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_SplitNoH from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = false,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_SplitNoIsEq from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = false,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_SplitNoLow from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = false,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_SplitNoCanon from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = false,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_PackWriter from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = false,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = true
  ) as M_PackUnused from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = false,
    C_MEM_PAD_ZERO = true
  ) as M_MemNoRead from "./lfr_poseidon"
  import lfr_poseidon_base(
    C_POSEIDON_SBOX_T = true,
    C_POSEIDON_SBOX_Y = true,
    C_POSEIDON_OUT = true,
    C_PROG_BUS = true,
    C_PROG_TAG = true,
    C_SPLIT_H_BOUND = true,
    C_SPLIT_ISEQ_ONE = true,
    C_SPLIT_LOW_BOUND = true,
    C_SPLIT_D_CANON = true,
    C_PACK_CANON_WRITER = true,
    C_PACK_UNUSED_ZERO = true,
    C_MEM_READ = true,
    C_MEM_PAD_ZERO = false
  ) as M_MemPad from "./lfr_poseidon"

  // =========================================================================
  // 1. Real BLS12-381 Fr and BabyBear Big-Integer Constants (fr.rs, bb.rs)
  // =========================================================================

  pure val TWO_16: int = 65536
  pure val TWO_27: int = 134217728
  pure val TWO_31: int = 2147483648
  pure val TWO_32: int = 4294967296
  pure val TWO_48: int = TWO_16 * TWO_32
  pure val TWO_64: int = TWO_32 * TWO_32
  pure val TWO_101: int = TWO_64 * TWO_32 * 32
  pure val TWO_112: int = TWO_64 * TWO_48
  pure val TWO_155: int = TWO_112 * TWO_32 * 2048
  pure val TWO_160: int = TWO_112 * TWO_48
  pure val TWO_248: int = TWO_160 * TWO_64 * 16777216
  pure val TWO_254: int = TWO_248 * 64

  // BabyBear modulus p = 2^31 - 2^27 + 1 = 2013265921 (bb.rs:12)
  pure val REAL_P: int = TWO_31 - TWO_27 + 1
  pure val REAL_P_POW2: int = REAL_P * REAL_P
  pure val REAL_P_POW3: int = REAL_P_POW2 * REAL_P
  pure val REAL_P_POW4: int = REAL_P_POW3 * REAL_P
  pure val REAL_P_POW5: int = REAL_P_POW4 * REAL_P

  // BLS12-381 scalar field modulus r = z^4 - z^2 + 1 where |z| = 0xd201000000010000
  pure val BLS_Z: int = 53761 * TWO_48 + TWO_16
  pure val BLS_Z2: int = BLS_Z * BLS_Z
  pure val REAL_R: int = BLS_Z2 * BLS_Z2 - BLS_Z2 + 1

  // Split constants H = floor((r - 1) / p^5) and L = (r - 1) - H * p^5 (fr.rs:45-59)
  pure val REAL_RM1: int = REAL_R - 1
  pure val REAL_H_MAX: int = REAL_RM1 / REAL_P_POW5
  pure val REAL_LOW_MAX: int = REAL_RM1 % REAL_P_POW5

  // Real Split row integer check (fr.rs:7-12, machine.rs:424-475)
  pure def realSplitRowCheck(
    x: int,
    d0: int, d1: int, d2: int, d3: int, d4: int,
    h: int, g: int, isEq: int, v: int
  ): bool = {
    val low = d0 + d1 * REAL_P + d2 * REAL_P_POW2 + d3 * REAL_P_POW3 + d4 * REAL_P_POW4
    val total = low + h * REAL_P_POW5
    val reconModR = (total % REAL_R) == (x % REAL_R)
    val dCanon = and {
      d0 >= 0, d0 < REAL_P,
      d1 >= 0, d1 < REAL_P,
      d2 >= 0, d2 < REAL_P,
      d3 >= 0, d3 < REAL_P,
      d4 >= 0, d4 < REAL_P,
    }
    val hLimbsOk = h >= 0 and h < TWO_112
    val gLimbsOk = g >= 0 and g < TWO_112
    val eModR = ((REAL_H_MAX - h) % REAL_R + REAL_R) % REAL_R
    val hBoundOk = (eModR == (g % REAL_R)) and gLimbsOk
    val isEqGadgetOk = if (eModR == 0) (isEq == 1) else (isEq == 0)
    val vLimbsOk = v >= 0 and v < TWO_160
    val lhsLow = ((isEq * (REAL_LOW_MAX - low)) % REAL_R + REAL_R) % REAL_R
    val lowBoundOk = (lhsLow == (v % REAL_R)) and vLimbsOk
    reconModR and dCanon and hLimbsOk and hBoundOk and isEqGadgetOk and lowBoundOk
  }

  // Honest real Split witness generator for any x in [0, REAL_R)
  pure def realHonestSplitAccepted(x: int): bool = {
    val d0 = x % REAL_P
    val q1 = x / REAL_P
    val d1 = q1 % REAL_P
    val q2 = q1 / REAL_P
    val d2 = q2 % REAL_P
    val q3 = q2 / REAL_P
    val d3 = q3 % REAL_P
    val q4 = q3 / REAL_P
    val d4 = q4 % REAL_P
    val h = q4 / REAL_P
    val low = d0 + d1 * REAL_P + d2 * REAL_P_POW2 + d3 * REAL_P_POW3 + d4 * REAL_P_POW4
    val g = REAL_H_MAX - h
    val isEq = if (h == REAL_H_MAX) 1 else 0
    val v = if (isEq == 1) (REAL_LOW_MAX - low) else 0
    realSplitRowCheck(x, d0, d1, d2, d3, d4, h, g, isEq, v)
  }

  // =========================================================================
  // 2. Pure Exhaustive Checks & Real-Constant Proofs
  // =========================================================================

  // Completeness test: honest witness accepted across the entire stand-in domain
  run honestWitnessAcceptedTest = {
    val allXOk = 0.to(R_MOD - 1).forall(x => {
      val sr = honestSplitRow(x, keySplitProg0)
      val pr = honestPoseidonRow(x, fAdd(x, 1), fAdd(x, 2), keyPermProg0)
      val frR: FrInputRow = { v: x, prog: { ...keyFrProg0, p3: x } }
      and {
        splitRowAccepted(sr),
        (sr.d0, sr.d1, sr.h) == refSplit(x),
        poseidonRowGate(pr),
        (pr.out0, pr.out1, pr.out2) == refPoseidon(x, fAdd(x, 1), fAdd(x, 2)),
        frInputRowGate(frR),
      }
    })
    val allPackOk = 0.to(P_BB - 1).forall(w0 =>
      0.to(P_BB - 1).forall(w1 => {
        val in0: InputRow = { v: w0, prog: keyInProg0 }
        val in1: InputRow = { v: w1, prog: { ...keyInProg1, p3: w1 } }
        val pk: PackRow = { x0: w0, x1: w1, x2: 0, out: refPack2(w0, w1), prog: keyPackProg0 }
        inputRowGate(in0) and inputRowGate(in1) and packRowGate(pk)
      })
    )
    init.expect(all {
      invHonestAccepted,
      invRefine,
      allXOk,
      allPackOk,
    })
  }

  // Completeness on real BLS12-381 Fr and BabyBear edge values:
  // x = 0, x = p - 1, x = p^5 - 1 (where h = 0 < H and low > L!), x = H * p^5, x = r - 1
  run realConstantsCompletenessEdgeTest = {
    init.expect(all {
      realHonestSplitAccepted(0),
      realHonestSplitAccepted(REAL_P - 1),
      realHonestSplitAccepted(REAL_P_POW5 - 1),
      realHonestSplitAccepted(REAL_H_MAX * REAL_P_POW5),
      realHonestSplitAccepted(REAL_R - 1),
    })
  }

  // Attack 3 & 4 real-constant big-integer soundness inequalities (fr.rs, machine.rs:424-475, alu.rs:135-152)
  run realSplitBigIntSoundnessTest = {
    // 1. Canonical BabyBear gadget (alu.rs:135-152): P - 1 = 0x7800 * 2^16 + 0
    val pMinus1DecompOk = (REAL_P - 1) == (30720 * TWO_16 + 0)
    // 2. Split constants and limb bit bounds (fr.rs:45-59)
    val hMaxFits101Bits = REAL_H_MAX > 0 and REAL_H_MAX < TWO_101
    val hMaxFits112Bits = REAL_H_MAX < TWO_112
    val pPow5Fits155Bits = REAL_P_POW5 > 0 and REAL_P_POW5 < TWO_155
    val pPow5Fits160Bits = REAL_P_POW5 < TWO_160
    val lowMaxBelowPPow5 = REAL_LOW_MAX >= 0 and REAL_LOW_MAX < REAL_P_POW5
    val rm1Identity = REAL_RM1 == (REAL_H_MAX * REAL_P_POW5 + REAL_LOW_MAX)
    // 3. No mod-r wraparound in H - h == g (mod r) when h, g in [0, 2^112)
    val hPlusGNoWrap = (TWO_112 - 1) + (TWO_112 - 1) < REAL_R
    // 4. No mod-r wraparound in L - low == v (mod r) when low in [0, p^5) and v in [0, 2^160)
    val lowPlusVNoWrap = (REAL_P_POW5 - 1) + (TWO_160 - 1) < REAL_R
    // 5. Maximum integer T = low + h * p^5 when h <= H - 1 and low <= p^5 - 1 is below r - 1
    val maxWhenHBelowHMax = (REAL_P_POW5 - 1) + (REAL_H_MAX - 1) * REAL_P_POW5
    // 6. Maximum integer T = low + h * p^5 when h == H and low <= L is exactly r - 1 < r
    val maxWhenHEqHMax = REAL_LOW_MAX + REAL_H_MAX * REAL_P_POW5
    // 7. Smallest alias x + r for x = 0 has h' = H and low' = L + 1 > L (rejected by low <= L)
    val aliasZeroH = REAL_R / REAL_P_POW5
    val aliasZeroLow = REAL_R % REAL_P_POW5
    val aliasZeroRejected = (aliasZeroH == REAL_H_MAX) and (aliasZeroLow == REAL_LOW_MAX + 1)
    // 8. Pack bound (fr.rs:29-36): 8 canonical words in [0, p) packed in base 2^31 fit below 2^248 < 2^254 < r
    val packInjectiveBase = REAL_P < TWO_31
    val packNoWrapModR = TWO_248 < TWO_254 and TWO_254 < REAL_R

    init.expect(all {
      REAL_P == 2013265921,
      REAL_R == 52435875175126190479447740508185965837690552500527637822603658699938581184513,
      pMinus1DecompOk,
      hMaxFits101Bits,
      hMaxFits112Bits,
      pPow5Fits155Bits,
      pPow5Fits160Bits,
      lowMaxBelowPPow5,
      rm1Identity,
      hPlusGNoWrap,
      lowPlusVNoWrap,
      maxWhenHBelowHMax < REAL_RM1,
      maxWhenHEqHMax == REAL_RM1,
      aliasZeroRejected,
      packInjectiveBase,
      packNoWrapModR,
    })
  }

  // Attack 1 exhaustive pure check: over all single-cell and propagating tampers of a Poseidon row
  // in F_89, poseidonRowGate accepts iff every S-box cell and output cell equals honestPoseidonRow.
  run exhaustivePoseidonRowSoundnessTest = {
    val base = honestPoseidonRow(6, 17, 6, keyPermProg0)
    val m2 = mdsRow(base.y0, base.y1, base.y2, 2)
    val x3 = fAdd(m2, 6)
    val u0 = fAdd(mdsRow(base.y0, base.y1, base.y2, 0), 4)
    val u1 = fAdd(mdsRow(base.y0, base.y1, base.y2, 1), 5)

    // Exhaustively vary t3 in 0..88 and propagate y3 = t3^2 * x3 and out0..2
    val t3Sound = 0.to(R_MOD - 1).forall(candT3 => {
      val candY3 = fMul(fMul(candT3, candT3), x3)
      val r: PoseidonRow = {
        ...base,
        t3: candT3,
        y3: candY3,
        out0: mdsRow(u0, u1, candY3, 0),
        out1: mdsRow(u0, u1, candY3, 1),
        out2: mdsRow(u0, u1, candY3, 2),
      }
      poseidonRowGate(r) iff (candT3 == base.t3 and (r.out0, r.out1, r.out2) == (base.out0, base.out1, base.out2))
    })

    // Exhaustively vary y3 in 0..88 and propagate out0..2
    val y3Sound = 0.to(R_MOD - 1).forall(candY3 => {
      val r: PoseidonRow = {
        ...base,
        y3: candY3,
        out0: mdsRow(u0, u1, candY3, 0),
        out1: mdsRow(u0, u1, candY3, 1),
        out2: mdsRow(u0, u1, candY3, 2),
      }
      poseidonRowGate(r) iff (candY3 == base.y3 and (r.out0, r.out1, r.out2) == (base.out0, base.out1, base.out2))
    })

    // Exhaustively vary out0 in 0..88
    val out0Sound = 0.to(R_MOD - 1).forall(candOut0 => {
      val r: PoseidonRow = { ...base, out0: candOut0 }
      poseidonRowGate(r) iff (candOut0 == base.out0)
    })

    init.expect(all {
      t3Sound,
      y3Sound,
      out0Sound,
    })
  }

  // Attack 2 exhaustive pure check: Program bus rejects any row with wrong tag, wrong cells,
  // duplicate key row, non-zero padding row, or wrong execution trace height (taller/shorter than padded(n)).
  run exhaustiveProgramBusSoundnessTest = {
    // Table for one AIR kind with 1 key row keyFrProg0 and 1 nop row zeroProg (padded(1) = 2)
    val tableEntries: List[ProgBusEntry] = List(
      { msg: mkProgMsg(TAG_FR_INPUT, keyFrProg0), cnt: 1 },
      { msg: mkProgMsg(TAG_FR_INPUT, zeroProg), cnt: 1 }
    )

    // 1. Vary tag0, tag1 in 1..6 and cells in {keyFrProg0, keyInProg0, zeroProg}
    val cellDomain = Set(keyFrProg0, keyInProg0, zeroProg)
    val tagDomain = Set(TAG_INPUT, TAG_FR_INPUT, TAG_PERM, TAG_PACK, TAG_SPLIT)
    val multisetPinned = tagDomain.forall(t0 =>
      tagDomain.forall(t1 =>
        cellDomain.forall(c0 =>
          cellDomain.forall(c1 => {
            val execEntries: List[ProgBusEntry] = List(
              { msg: mkProgMsg(t0, c0), cnt: R_MOD - 1 },
              { msg: mkProgMsg(t1, c1), cnt: R_MOD - 1 }
            )
            val balanced = progBusBalanced(tableEntries.concat(execEntries))
            val validAssignment = and {
              t0 == TAG_FR_INPUT,
              t1 == TAG_FR_INPUT,
              or {
                (c0 == keyFrProg0 and c1 == zeroProg),
                (c0 == zeroProg and c1 == keyFrProg0),
              }
            }
            balanced iff validAssignment
          })
        )
      )
    )

    // 2. Vary execution height h in 0..5 (number of nop rows received vs 1 sent by table):
    // balances iff height == 2 == padded(1).
    val heightPinned = 0.to(5).forall(nopReceives => {
      val netNopCnt = fSub(1, nopReceives)
      val entries: List[ProgBusEntry] = List(
        { msg: mkProgMsg(TAG_FR_INPUT, keyFrProg0), cnt: 0 },
        { msg: mkProgMsg(TAG_FR_INPUT, zeroProg), cnt: netNopCnt }
      )
      progBusBalanced(entries) iff (nopReceives == 1)
    })

    init.expect(all {
      multisetPinned,
      heightPinned,
    })
  }

  // Attack 3 exhaustive pure check: for representative and edge x values in F_89,
  // iterate over all candidate (d0, d1, h, isEq) in [0..7] x [0..7] x [0..7] x [0..1];
  // verify splitRowAccepted holds iff (d0, d1, h) == refSplit(x).
  run exhaustiveSplitRowSoundnessTest = {
    val testXs = Set(0, 1, 4, 5, 6, 13, 24, 25, 75, 88)
    val splitUnique = testXs.forall(x =>
      0.to(H_RANGE - 1).forall(d0 =>
        0.to(H_RANGE - 1).forall(d1 =>
          0.to(H_RANGE - 1).forall(h =>
            0.to(1).forall(isEq => {
              val low = d0 + P_BB * d1
              val e = fSub(H_MAX, h)
              val invE = if (e == 0) 0 else fInv(e)
              val g = fSub(H_MAX, h)
              val v = fMul(isEq, fSub(LOW_MAX, low))
              val r: SplitRow = {
                x: x, d0: d0, d1: d1, h: h, g: g, invE: invE, isEq: isEq, v: v,
                prog: keySplitProg0,
              }
              splitRowAccepted(r) iff (
                (d0, d1, h) == refSplit(x) and (isEq == if (h == H_MAX) 1 else 0)
              )
            })
          )
        )
      )
    )
    init.expect(splitUnique)
  }

  // Attack 4 exhaustive pure check: Pack row soundness over all (w0, w1) in [0, P_BB)^2,
  // unused slot x2 in 0..88, and injectivity over [0, P_BB)^2 vs collisions when >= B_PACK.
  run exhaustivePackRowSoundnessTest = {
    val unusedAndOutPinned = 0.to(P_BB - 1).forall(w0 =>
      0.to(P_BB - 1).forall(w1 =>
        0.to(R_MOD - 1).forall(x2 => {
          val r: PackRow = {
            x0: w0, x1: w1, x2: x2, out: refPack2(w0, w1), prog: keyPackProg0
          }
          packRowGate(r) iff (x2 == 0)
        })
      )
    )
    val injectiveOnCanonical = 0.to(P_BB - 1).forall(w0 =>
      0.to(P_BB - 1).forall(w1 =>
        0.to(P_BB - 1).forall(u0 =>
          0.to(P_BB - 1).forall(u1 =>
            (refPack2(w0, w1) == refPack2(u0, u1)) iff (w0 == u0 and w1 == u1)
          )
        )
      )
    )
    // Without canonical bound w0 < B_PACK, (w0 + B_PACK, w1 - 1) collides with (w0, w1)
    val collisionWithoutCanon =
      fAdd(1 + B_PACK, fMul(B_PACK, 2 - 1)) == fAdd(1, fMul(B_PACK, 2))

    init.expect(all {
      unusedAndOutPinned,
      injectiveOnCanonical,
      collisionWithoutCanon,
    })
  }

  // Attack 5 exhaustive pure check: Memory bus rejects any reader value mismatch or
  // non-zero padding multiplicity.
  run exhaustiveMemBusSoundnessTest = {
    val readValPinned = 0.to(R_MOD - 1).forall(readVal => {
      val entries: List[MemBusEntry] = List(
        { msg: mkMemMsg(3, 6), cnt: 1 },
        { msg: mkMemMsg(3, readVal), cnt: R_MOD - 1 }
      )
      memBusBalanced(entries) iff (readVal == 6)
    })
    val padMultPinned = 0.to(R_MOD - 1).forall(padMult => {
      val entries: List[MemBusEntry] = List(
        { msg: mkMemMsg(3, 6), cnt: 1 },
        { msg: mkMemMsg(3, 6), cnt: R_MOD - 1 },
        { msg: mkMemMsg(99, 42), cnt: padMult }
      )
      memBusBalanced(entries) iff (padMult == 0)
    })
    init.expect(all {
      readValPinned,
      padMultPinned,
    })
  }

  // =========================================================================
  // 3. Mutant Non-Vacuity (Honest Accepted & Refines) and Real Attack Tests
  // =========================================================================

  run mutantsHonestAndAttackTest = {
    // 1. Every mutant accepts the honest witness and refines on it (non-vacuity)
    val honestAcceptedInAllMutants = and {
      M_PoseidonT::invHonestAccepted,
      M_PoseidonY::invHonestAccepted,
      M_PoseidonOut::invHonestAccepted,
      M_ProgNoBus::invHonestAccepted,
      M_ProgNoTag::invHonestAccepted,
      M_SplitNoH::invHonestAccepted,
      M_SplitNoIsEq::invHonestAccepted,
      M_SplitNoLow::invHonestAccepted,
      M_SplitNoCanon::invHonestAccepted,
      M_PackWriter::invHonestAccepted,
      M_PackUnused::invHonestAccepted,
      M_MemNoRead::invHonestAccepted,
      M_MemPad::invHonestAccepted,
    }

    // 2. Every mutant accepts its respective attack witness while violating refinement
    val attacksBreakAllMutants = and {
      M_PoseidonT::attackPoseidonSboxT.acc and not(M_PoseidonT::refinesRef(M_PoseidonT::attackPoseidonSboxT)),
      M_PoseidonY::attackPoseidonSboxY.acc and not(M_PoseidonY::refinesRef(M_PoseidonY::attackPoseidonSboxY)),
      M_PoseidonOut::attackPoseidonOut.acc and not(M_PoseidonOut::refinesRef(M_PoseidonOut::attackPoseidonOut)),
      M_ProgNoBus::attackProgNoBus.acc and not(M_ProgNoBus::refinesRef(M_ProgNoBus::attackProgNoBus)),
      M_ProgNoTag::attackProgNoTag.acc and not(M_ProgNoTag::refinesRef(M_ProgNoTag::attackProgNoTag)),
      M_SplitNoH::attackSplitNoHBound.acc and not(M_SplitNoH::refinesRef(M_SplitNoH::attackSplitNoHBound)),
      M_SplitNoIsEq::attackSplitNoIsEqOne.acc and not(M_SplitNoIsEq::refinesRef(M_SplitNoIsEq::attackSplitNoIsEqOne)),
      M_SplitNoLow::attackSplitNoLowBound.acc and not(M_SplitNoLow::refinesRef(M_SplitNoLow::attackSplitNoLowBound)),
      M_SplitNoCanon::attackSplitNoDCanon.acc and not(M_SplitNoCanon::refinesRef(M_SplitNoCanon::attackSplitNoDCanon)),
      M_PackWriter::attackPackUnconstrainedWriter.acc and not(M_PackWriter::refinesRef(M_PackWriter::attackPackUnconstrainedWriter)),
      M_PackUnused::attackPackNoUnusedZero.acc and not(M_PackUnused::refinesRef(M_PackUnused::attackPackNoUnusedZero)),
      M_MemNoRead::attackMemNoRead.acc and not(M_MemNoRead::refinesRef(M_MemNoRead::attackMemNoRead)),
      M_MemPad::attackMemPadNonZero.acc and not(M_MemPad::refinesRef(M_MemPad::attackMemPadNonZero)),
    }

    // 3. The sound circuit rejects all 13 attacks
    val soundRejectsAllAttacks = and {
      not(attackPoseidonSboxT.acc),
      not(attackPoseidonSboxY.acc),
      not(attackPoseidonOut.acc),
      not(attackProgNoBus.acc),
      not(attackProgNoTag.acc),
      not(attackSplitNoHBound.acc),
      not(attackSplitNoIsEqOne.acc),
      not(attackSplitNoLowBound.acc),
      not(attackSplitNoDCanon.acc),
      not(attackPackUnconstrainedWriter.acc),
      not(attackPackNoUnusedZero.acc),
      not(attackMemNoRead.acc),
      not(attackMemPadNonZero.acc),
    }

    init.expect(all {
      honestAcceptedInAllMutants,
      attacksBreakAllMutants,
      soundRejectsAllAttacks,
    })
  }

  // State machine trace test: init then 5 steps preserves invRefine in sound
  run stateMachineSoundAndReachTest = {
    init
      .expect(all { invRefine, not(reachHonestAccepted) })
      .then(5.reps(_ => step))
      .expect(invRefine)
  }
}
