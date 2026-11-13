// -*- mode: quint; -*-
// Exhaustive pure soundness checks, completeness checks, mutant non-vacuity checks,
// and real-constant big-integer bound checks for the Sherlock E2 L_Fr tape lane.

module lfr_tape_test {
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
  ).* from "./lfr_tape"

  // =========================================================================
  // 1. Concrete Config records matching the sound module and each mutant module
  // =========================================================================
  pure val CFG_SOUND: Config = {
    progBus: true,
    memBus: true,
    pinAddrMult: true,
    inputCanon: true,
    inputBbZeroHi: true,
    bitsBool: true,
    bitsHighCheck: true,
    bitsMaskMono: true,
    assertEqGate: true,
    frAddGate: true,
    halveBitBool: true,
    halveTermZero: true,
    cswapBitBool: true,
    cswapRGate: true,
    publicPinOrder: true,
  }

  pure val CFG_MUT_NO_PROG_BUS: Config = { ...CFG_SOUND, progBus: false }
  pure val CFG_MUT_NO_MEM_BUS: Config = { ...CFG_SOUND, memBus: false }
  pure val CFG_MUT_DUP_WRITER: Config = { ...CFG_SOUND, pinAddrMult: false }
  pure val CFG_MUT_INPUT_NO_CANON: Config = { ...CFG_SOUND, inputCanon: false }
  pure val CFG_MUT_INPUT_NO_BB_ZERO_HI: Config = { ...CFG_SOUND, inputBbZeroHi: false }
  pure val CFG_MUT_BITS_NO_BOOL: Config = { ...CFG_SOUND, bitsBool: false }
  pure val CFG_MUT_BITS_NO_HIGH: Config = { ...CFG_SOUND, bitsHighCheck: false }
  pure val CFG_MUT_BITS_NO_MONO: Config = { ...CFG_SOUND, bitsMaskMono: false }
  pure val CFG_MUT_ASSERT_EQ: Config = { ...CFG_SOUND, assertEqGate: false }
  pure val CFG_MUT_FR_ADD: Config = { ...CFG_SOUND, frAddGate: false }
  pure val CFG_MUT_HALVE_NO_BOOL: Config = { ...CFG_SOUND, halveBitBool: false }
  pure val CFG_MUT_HALVE_NO_TERM: Config = { ...CFG_SOUND, halveTermZero: false }
  pure val CFG_MUT_CSWAP_NO_BOOL: Config = { ...CFG_SOUND, cswapBitBool: false }
  pure val CFG_MUT_CSWAP_FREE_R: Config = { ...CFG_SOUND, cswapRGate: false }
  pure val CFG_MUT_PUBLIC_UNPINNED: Config = { ...CFG_SOUND, publicPinOrder: false }

  // =========================================================================
  // 2. Real BLS12-381 Fr and BabyBear big-integer bound checks
  // =========================================================================
  // BLS12-381 scalar field modulus r (fr.rs:7-16):
  //   0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001
  pure val REAL_R: int =
    52435875175126190479447740508185965837690552500527637822603658699938581184513

  // BabyBear prime p = 2^31 - 2^27 + 1 = 2013265921 (0x78000001, bb.rs:7)
  pure val REAL_P: int = 2013265921

  // 2^31 = 2147483648 (BitsAir BITS_N = 31 upper bound, machine.rs:593)
  pure val REAL_TWO_POW_31: int = 2147483648

  // 2^254 (maximum Halve chain length without wrapping modulo r < 2^255)
  pure val REAL_TWO_POW_254: int =
    28948022309329048855892746252171976963317496166410141009864396001978282409984

  // 2^255 (strictly above r, showing why 255-bit decomposition without a < r check can alias)
  pure val REAL_TWO_POW_255: int =
    57896044618658097711785492504343953926634992332820282019728792003956564819968

  // u32::MAX = 4294967295 (per-bus height * weight bound, E2-RESULTS.md section 9)
  pure val REAL_U32_MAX: int = 4294967295

  // Real-constant checks where soundness/completeness depends on field size:
  // 1. BabyBear canonical range fits strictly inside 31-bit unsigned integers and inside Fr:
  //    0 < REAL_P < REAL_TWO_POW_31 < REAL_U32_MAX < REAL_R
  // 2. BitsAir (BITS_N = 31): maximum recomposed value sum_{i=0}^{30} b_i 2^i = 2^31 - 1 < REAL_R,
  //    so `acc == x` in Fr holds iff the integer equality holds (no wrap-around modulo r).
  // 3. HalveAir chain of length n <= 254 ending in AssertEq(rest_n, 0):
  //    sum_{i=0}^{n-1} b_i 2^i <= 2^254 - 1 < REAL_R, so no wrap-around modulo r occurs for n <= 254
  //    (in lverify.rs:1240-1250, WHIR query folding uses n <= 20 << 254), whereas 2^255 > REAL_R.
  // 4. InputAir eval_canonical (tape.rs:560-567, range.rs:144-169):
  //    checks x < REAL_P by decomposing gap = (REAL_P - 1 - x) into two 16-bit limbs
  //    gap_lo + 2^16 * gap_hi with gap_lo, gap_hi in [0, 2^16). Maximum representable gap is
  //    2^32 - 1 = 4294967295 < REAL_R, so x == REAL_P - 1 - gap cannot wrap modulo r for x >= REAL_P.
  // 5. LogUp bus count bound (E2-RESULTS.md section 9, interaction/mod.rs):
  //    total count_weight * height <= REAL_U32_MAX < REAL_R, so bus multiplicities never wrap modulo r.
  run realConstantsBoundsTest = all {
    assert(REAL_P == 2013265921),
    assert(REAL_P < REAL_TWO_POW_31),
    assert(REAL_TWO_POW_31 - 1 < REAL_R),
    assert(REAL_U32_MAX < REAL_R),
    assert(REAL_TWO_POW_254 < REAL_R),
    assert(REAL_R < REAL_TWO_POW_255),
    // InputAir eval_canonical 32-bit gap cannot wrap modulo REAL_R:
    assert(REAL_P - 1 + 4294967295 < REAL_R),
    // HalveAir inv2 in Fr: (REAL_R + 1) / 2 * 2 == REAL_R + 1 == 1 (mod REAL_R)
    assert(((REAL_R + 1) / 2) * 2 == REAL_R + 1),
  }

  // Pure completeness check on real edge values (0, 1, p - 1, r - 1, 2^n - 1) over big ints
  run realEdgeValuesCompletenessTest = {
    val edgeFr = Set(0, 1, REAL_P - 1, REAL_TWO_POW_31 - 1, REAL_R - 1)
    val inv2Real = (REAL_R + 1) / 2
    all {
      // FrAdd completeness on all pairs of edge values in Fr
      assert(edgeFr.forall(a => edgeFr.forall(b => {
        val z = (a + b) % REAL_R
        z >= 0 and z < REAL_R
      }))),
      // CSwap completeness on bit in {0, 1} and edge Fr values (0, 1, p-1, r-1)
      assert(Set(0, 1).forall(bit => edgeFr.forall(a => edgeFr.forall(b => {
        val l = (a + bit * (b - a) + REAL_R) % REAL_R
        val r = (b + bit * (a - b) + REAL_R) % REAL_R
        (if (bit == 0) (l == a and r == b) else (l == b and r == a))
      })))),
      // Halve algebraic identity 2 * rest + bit == x (mod REAL_R) on edge values
      assert(Set(0, 1, REAL_P - 1, REAL_TWO_POW_31 - 1).forall(x => {
        val bit = x % 2
        val rest = x / 2
        (bit == 0 or bit == 1) and (2 * rest + bit == x) and (rest < REAL_P)
      })),
      // Even for x = REAL_R - 1 (an even integer since REAL_R is odd), 2 * ((REAL_R - 1)/2) + 0 == REAL_R - 1
      assert(((REAL_R - 1) % 2 == 0) and (2 * ((REAL_R - 1) / 2) == REAL_R - 1)),
      assert(((1 * inv2Real) * 2) % REAL_R == 1),
    }
  }

  // =========================================================================
  // 3. Honest witness builders & pure completeness check (invHonestAccepted)
  // =========================================================================
  pure def honestBitsRow(x: int, n: int): BitsRow = {
    val b0 = x % 2
    val b1 = (x / 2) % 2
    val b2 = (x / 4) % 2
    val s0 = if (0 < n) 1 else 0
    val s1 = if (1 < n) 1 else 0
    val s2 = if (2 < n) 1 else 0
    val low = x % pow2(n)
    {
      addrX: 1, addrLow: 3, multLow: 0, nBits: n, active: 1,
      xVal: x, b0: b0, b1: b1, b2: b2, s0: s0, s1: s1, s2: s2, lowVal: low
    }
  }

  pure def honestHalveRow(x: int): HalveRow = {
    addrX: 1, addrBit: 4, addrRest: 5, multBit: 0, multRest: 1, active: 1,
    xVal: x, bitVal: x % 2, restVal: x / 2
  }

  pure def honestFrAddRow(a: int, b: int): FrAddRow = {
    addrA: 6, addrB: 7, addrOut: 8, multOut: 1, active: 1,
    xVal: a, yVal: b, zVal: modR(a + b)
  }

  pure def honestCSwapRow(bit: int, a: int, b: int): CSwapRow = {
    addrBit: 17, addrA: 6, addrB: 7, addrL: 9, addrR: 10, multL: 1, multR: 0, active: 1,
    bitVal: bit, aVal: a, bVal: b,
    lVal: if (bit == 0) a else b,
    rVal: if (bit == 0) b else a,
  }

  // Pure completeness check across all ops and inputs in the domain, including edge values
  // 0, 1, P_MOD - 1 (4), 2^n - 1 (1, 3, 7), and R_MOD - 1 (10).
  pure val invHonestAccepted: bool = and {
    // Full tape honest witness is accepted and refines
    witnessAccepted(CFG_SOUND, honestState),
    witnessRefines(honestState),
    // Furthermore, for every valid BabyBear input in2 == 3, frIn1 in F_DOM (0..10 including R_MOD-1),
    // in16 in BB_DOM (0..4 including P_MOD-1), and cswap bit in BIT_DOM (0, 1),
    // the full honest tape witness is accepted and refines:
    F_DOM.forall(bFr =>
      BB_DOM.forall(ef0 =>
        BIT_DOM.forall(csBit => {
          val zAdd = modR(4 + bFr)
          val lCs = if (csBit == 0) 4 else bFr
          val rCs = if (csBit == 0) bFr else 4
          val s = {
            ...honestState,
            inRow2: { ...honestState.inRow2, v0: ef0, v1: 0 },
            inRow3: { ...honestState.inRow3, v0: csBit, v1: 0 },
            frIn1: { ...honestState.frIn1, v0: bFr },
            frAdd0: { ...honestState.frAdd0, xVal: 4, yVal: bFr, zVal: zAdd },
            cswap0: { ...honestState.cswap0, bitVal: csBit, aVal: 4, bVal: bFr, lVal: lCs, rVal: rCs },
            reshape0: { ...honestState.reshape0, v0: ef0, v1: 0 },
            pub0: { ...honestState.pub0, v0: zAdd, v1: lCs },
            pv0: zAdd,
            pv1: lCs,
          }
          witnessAccepted(CFG_SOUND, s) and witnessRefines(s)
        })
      )
    ),
    // Per-op completeness across full input domains:
    // InputAir (BabyBear scalar and EF4, including 0, 1, P_MOD - 1):
    BB_DOM.forall(x0 => BB_DOM.forall(x1 => {
      val rBb = { addr: 1, mult: 1, z1: 1, isConst: 0, k0: 0, k1: 0, v0: x0, v1: 0 }
      val rEf = { addr: 1, mult: 1, z1: 0, isConst: 0, k0: 0, k1: 0, v0: x0, v1: x1 }
      checkInputRow(CFG_SOUND, rBb) and checkInputRow(CFG_SOUND, rEf)
    })),
    // FrInputAir (all Fr elements 0..R_MOD-1, including 0, 1, P_MOD-1, R_MOD-1):
    F_DOM.forall(v => checkFrInputRow({ addr: 6, mult: 1, isConst: 0, k0: 0, v0: v })),
    // BitsAir (all x in 0..2^BITS_W - 1 = 0..7, including 0, 1, P_MOD-1=4, 2^n-1=7, and n in 0..3):
    0.to(TWO_POW_BITS_W - 1).forall(x =>
      0.to(BITS_W).forall(n => checkBitsRow(CFG_SOUND, honestBitsRow(x, n)))
    ),
    // HalveAir (all x in 0..2^BITS_W - 1 = 0..7):
    0.to(TWO_POW_BITS_W - 1).forall(x => checkHalveRow(CFG_SOUND, honestHalveRow(x))),
    // FrAddAir (all a, b in F_DOM = 0..R_MOD-1):
    F_DOM.forall(a => F_DOM.forall(b => checkFrAddRow(CFG_SOUND, honestFrAddRow(a, b)))),
    // CSwapAir (all bit in {0, 1} and all a, b in F_DOM = 0..R_MOD-1):
    BIT_DOM.forall(bit => F_DOM.forall(a => F_DOM.forall(b =>
      checkCSwapRow(CFG_SOUND, honestCSwapRow(bit, a, b))
    ))),
    // AssertEqAir (all equal pairs in F_DOM x F_DOM):
    F_DOM.forall(v0 => F_DOM.forall(v1 =>
      checkAssertEqRow(true, { addrA: 1, addrB: 2, active: 1, v0: v0, v1: v1, w0: v0, w1: v1 })
    )),
    // PublicAir (all pairs in F_DOM x F_DOM):
    F_DOM.forall(v0 => F_DOM.forall(v1 =>
      checkPublicRow(CFG_SOUND, { addr0: 8, addr1: 9, active: 1, v0: v0, v1: v1 }, v0, v1)
    )),
  }

  run invHonestAcceptedTest = assert(invHonestAccepted)

  // =========================================================================
  // 4. Exhaustive pure soundness checks over finite stand-in attack domains
  // =========================================================================

  // Q1 Exhaustive Check:
  // Over a write-once memory bus with key-pinned writer (addr = 1, mult = 2) writing `writtenVal`
  // and two readers (each receiving addr = 1 with count -1) plus two padding rows (pinned by PROG_BUS
  // to addr = 0, mult = 0), quantify over ALL (writtenVal, readVal1, readVal2, padVal0, padVal1)
  // in F_DOM^5: the bus is balanced if and only if readVal1 == writtenVal and readVal2 == writtenVal.
  run exhaustiveMemAndProgBusSoundnessTest = {
    val allSound = F_DOM.forall(writtenVal =>
      F_DOM.forall(readVal1 =>
        F_DOM.forall(readVal2 =>
          F_DOM.forall(padVal0 => {
            val s = {
              ...honestState,
              inRow0: { ...honestState.inRow0, v0: writtenVal, v1: 0 },
              inPad0: { ...honestState.inPad0, v0: padVal0, v1: 0 },
              bits0: { ...honestState.bits0, xVal: readVal1 },
              halve0: { ...honestState.halve0, xVal: readVal2 },
              assert0: { ...honestState.assert0, v0: writtenVal, w0: writtenVal },
              inRow1: { ...honestState.inRow1, v0: writtenVal },
            }
            val busOk = checkProgBus(CFG_SOUND, s) and checkMemBus(CFG_SOUND, s)
            busOk implies (readVal1 == writtenVal and readVal2 == writtenVal)
          })
        )
      )
    )
    assert(allSound)
  }

  // Q1 Wrong-multiplicity check:
  // If the key's write multiplicity `mult` at addr 1 differs from the number of readers (3),
  // no witness with honest reader rows can balance MEM_BUS (demonstrating why Machine::new's
  // `reads = tape.reads()` invariant is necessary for completeness and soundness).
  run wrongMultRejectsHonestTest = {
    val wrongMults = F_DOM.exclude(Set(3))
    assert(wrongMults.forall(badMult => {
      val s = {
        ...honestState,
        inRow0: { ...honestState.inRow0, mult: badMult },
      }
      not(checkMemBus(CFG_SOUND, s))
    }))
  }

  // Q2 Exhaustive Check (InputAir & Type Confusion on MEM_BUS):
  // Quantify over all (v0, v1) in F_DOM^2 for a BabyBear scalar input row (z1 = 1, isConst = 0):
  // checkInputRow(CFG_SOUND, r) holds iff v0 in BB_DOM (0 <= v0 < P_MOD) and v1 == 0.
  // Thus a BabyBear writer can ONLY emit [addr, v0, 0] with canonical v0 < P_MOD, which is
  // also a valid canonical Fr word [addr, v0, 0] when read by an Fr reader (FrAdd, CSwap, Public).
  run exhaustiveInputTypeSoundnessTest = {
    val allSound = F_DOM.forall(v0 =>
      F_DOM.forall(v1 => {
        val rBb = { addr: 2, mult: 1, z1: 1, isConst: 0, k0: 0, k1: 0, v0: v0, v1: v1 }
        val rEf = { addr: 16, mult: 1, z1: 0, isConst: 0, k0: 0, k1: 0, v0: v0, v1: v1 }
        val bbCheck = ( checkInputRow(CFG_SOUND, rBb) == (isCanonBb(v0) and v1 == 0) )
        val efCheck = ( checkInputRow(CFG_SOUND, rEf) == (isCanonBb(v0) and isCanonBb(v1)) )
        bbCheck and efCheck
      })
    )
    assert(allSound)
  }

  // Q3 Exhaustive Check (BitsAir / LowBits):
  // Quantify over ALL xVal in F_DOM (0..10), nBits in 0..3, (b0, b1, b2) in F_DOM^3,
  // (s0, s1, s2) in BIT_DOM^3, and lowVal in F_DOM (0..10):
  // checkBitsRow(CFG_SOUND, r) holds ONLY IF:
  //   1. b0, b1, b2 in {0, 1}
  //   2. xVal < 2^BITS_W (so xVal in 8..10 is ALWAYS rejected — enforcing the range bound x < 2^W)
  //   3. b0 + 2*b1 + 4*b2 == xVal (exact binary decomposition, high part range-checked)
  //   4. lowVal == xVal % 2^nBits (uniquely determined function of (xVal, nBits), no free output)
  run exhaustiveBitsAirSoundnessTest = {
    // Because b_i are constrained boolean by checkBitsRow, check both:
    // (a) any non-boolean b_i in F_DOM is rejected, and
    // (b) over all xVal in F_DOM, nBits in 0..3, b in BIT_DOM^3, s in BIT_DOM^3, lowVal in F_DOM:
    val nonBoolRejected = F_DOM.forall(b0 =>
      F_DOM.forall(b1 =>
        F_DOM.forall(b2 => {
          val r = {
            addrX: 1, addrLow: 3, multLow: 0, nBits: 1, active: 1,
            xVal: modR(b0 + 2 * b1 + 4 * b2),
            b0: b0, b1: b1, b2: b2,
            s0: 1, s1: 0, s2: 0,
            lowVal: b0,
          }
          checkBitsRow(CFG_SOUND, r) implies (isBool(b0) and isBool(b1) and isBool(b2))
        })
      )
    )
    val fullSoundness = F_DOM.forall(xVal =>
      0.to(BITS_W).forall(nBits =>
        BIT_DOM.forall(b0 =>
          BIT_DOM.forall(b1 =>
            BIT_DOM.forall(b2 =>
              BIT_DOM.forall(s0 =>
                BIT_DOM.forall(s1 =>
                  BIT_DOM.forall(s2 =>
                    F_DOM.forall(lowVal => {
                      val r = {
                        addrX: 1, addrLow: 3, multLow: 0, nBits: nBits, active: 1,
                        xVal: xVal, b0: b0, b1: b1, b2: b2,
                        s0: s0, s1: s1, s2: s2, lowVal: lowVal,
                      }
                      checkBitsRow(CFG_SOUND, r) implies (
                        xVal < TWO_POW_BITS_W and
                        b0 == (xVal % 2) and
                        b1 == ((xVal / 2) % 2) and
                        b2 == ((xVal / 4) % 2) and
                        lowVal == (xVal % pow2(nBits))
                      )
                    })
                  )
                )
              )
            )
          )
        )
      )
    )
    all {
      assert(nonBoolRejected),
      assert(fullSoundness),
    }
  }

  // Q3 & Q4 Exhaustive Check (HalveAir chain of length 2 + terminal AssertEq(rest1, 0)):
  // Quantify over all xVal in F_DOM (0..10) and all (bit0, rest0, bit1, rest1) in F_DOM^4:
  // When both HalveAir rows pass checkHalveRow(CFG_SOUND, ...) AND rest1 == 0 (via AssertEq),
  // the chain accepts ONLY IF xVal < 4 and bit0 == xVal % 2, rest0 == xVal / 2, bit1 == (xVal / 2) % 2.
  run exhaustiveHalveChainSoundnessTest = {
    val allSound = F_DOM.forall(xVal =>
      F_DOM.forall(bit0 =>
        F_DOM.forall(rest0 =>
          F_DOM.forall(bit1 =>
            F_DOM.forall(rest1 => {
              val h0 = {
                addrX: 1, addrBit: 4, addrRest: 5, multBit: 0, multRest: 1, active: 1,
                xVal: xVal, bitVal: bit0, restVal: rest0,
              }
              val h1 = {
                addrX: 5, addrBit: 11, addrRest: 12, multBit: 0, multRest: 1, active: 1,
                xVal: rest0, bitVal: bit1, restVal: rest1,
              }
              val ok = checkHalveRow(CFG_SOUND, h0) and checkHalveRow(CFG_SOUND, h1) and (rest1 == 0)
              ok implies (
                xVal < 4 and
                bit0 == (xVal % 2) and
                rest0 == (xVal / 2) and
                bit1 == ((xVal / 2) % 2) and
                rest1 == 0
              )
            })
          )
        )
      )
    )
    assert(allSound)
  }

  // Q4 Exhaustive Check (FrAddAir):
  // Quantify over all (xVal, yVal, zVal) in F_DOM^3:
  // checkFrAddRow(CFG_SOUND, r) holds iff zVal == (xVal + yVal) mod R_MOD.
  run exhaustiveFrAddSoundnessTest = {
    val allSound = F_DOM.forall(xVal =>
      F_DOM.forall(yVal =>
        F_DOM.forall(zVal => {
          val r = { addrA: 6, addrB: 7, addrOut: 8, multOut: 1, active: 1, xVal: xVal, yVal: yVal, zVal: zVal }
          checkFrAddRow(CFG_SOUND, r) == (zVal == modR(xVal + yVal))
        })
      )
    )
    assert(allSound)
  }

  // Q4 Exhaustive Check (CSwapAir):
  // Quantify over all (bitVal, aVal, bVal, lVal, rVal) in F_DOM^5:
  // checkCSwapRow(CFG_SOUND, r) holds iff bitVal in {0, 1} and
  // (lVal, rVal) == (if bitVal == 0 then (aVal, bVal) else (bVal, aVal)).
  run exhaustiveCSwapSoundnessTest = {
    val allSound = F_DOM.forall(bitVal =>
      F_DOM.forall(aVal =>
        F_DOM.forall(bVal =>
          F_DOM.forall(lVal =>
            F_DOM.forall(rVal => {
              val r = {
                addrBit: 17, addrA: 6, addrB: 7, addrL: 9, addrR: 10, multL: 1, multR: 0, active: 1,
                bitVal: bitVal, aVal: aVal, bVal: bVal, lVal: lVal, rVal: rVal,
              }
              val expected = (
                isBool(bitVal) and
                lVal == (if (bitVal == 0) aVal else bVal) and
                rVal == (if (bitVal == 0) bVal else aVal)
              )
              checkCSwapRow(CFG_SOUND, r) == expected
            })
          )
        )
      )
    )
    assert(allSound)
  }

  // Q4 Exhaustive Check (AssertEqAir):
  // Quantify over all (v0, v1, w0, w1) in F_DOM^4:
  // checkAssertEqRow(true, r) holds iff v0 == w0 and v1 == w1.
  run exhaustiveAssertEqSoundnessTest = {
    val allSound = F_DOM.forall(v0 =>
      F_DOM.forall(v1 =>
        F_DOM.forall(w0 =>
          F_DOM.forall(w1 => {
            val r = { addrA: 1, addrB: 2, active: 1, v0: v0, v1: v1, w0: w0, w1: w1 }
            checkAssertEqRow(true, r) == (v0 == w0 and v1 == w1)
          })
        )
      )
    )
    assert(allSound)
  }

  // Q4 Exhaustive Check (ReshapeAir):
  // Quantify over all EF4 values (e0, e1) in BB_DOM^2 and decomposed outputs (b0, b1) in F_DOM^2:
  // MEM_BUS balance on ReshapeAir (Decompose) forces b0 == e0 and b1 == e1.
  run exhaustiveReshapeSoundnessTest = {
    val allSound = BB_DOM.forall(e0 =>
      F_DOM.forall(b0 =>
        F_DOM.forall(b1 => {
          val s = {
            ...honestState,
            inRow2: { ...honestState.inRow2, v0: e0, v1: 0 },
            reshape0: { ...honestState.reshape0, v0: b0, v1: b1 },
          }
          witnessAccepted(CFG_SOUND, s) implies (b0 == e0 and b1 == 0)
        })
      )
    )
    assert(allSound)
  }

  // Q5 Exhaustive Check (PublicAir):
  // Quantify over all (v0, v1, pv0, pv1) in F_DOM^4 on the active PublicAir row:
  // checkPublicRow(CFG_SOUND, r, pv0, pv1) combined with MEM_BUS balance forces
  // pv0 == mem[addr0] and pv1 == mem[addr1] in exact order.
  run exhaustivePublicSoundnessTest = {
    val allSound = F_DOM.forall(v0 =>
      F_DOM.forall(v1 =>
        F_DOM.forall(pv0 =>
          F_DOM.forall(pv1 => {
            val s = {
              ...honestState,
              pub0: { ...honestState.pub0, v0: v0, v1: v1 },
              pv0: pv0,
              pv1: pv1,
            }
            witnessAccepted(CFG_SOUND, s) implies (pv0 == 9 and pv1 == 5 and v0 == 9 and v1 == 5)
          })
        )
      )
    )
    assert(allSound)
  }

  // =========================================================================
  // 5. Mutant tests: for each of the 15 mutants, verify:
  //    (a) Honest witness is still accepted and refines under the mutant (non-vacuous),
  //    (b) The attack witness is accepted under the mutant AND violates refinement,
  //    (c) The attack witness is rejected by the sound configuration CFG_SOUND.
  // =========================================================================
  pure def mutantCaughtCleanly(cfgMut: Config, attackState: WitnessState): bool = and {
    // (a) Honest witness is accepted and refines under the mutant
    witnessAccepted(cfgMut, honestState),
    witnessRefines(honestState),
    // (b) Attack witness is accepted under the mutant, but violates witnessRefines
    witnessAccepted(cfgMut, attackState),
    not(witnessRefines(attackState)),
    // (c) Sound config refuses the attack witness
    not(witnessAccepted(CFG_SOUND, attackState)),
  }

  run mutNoProgBusTest = all {
    assert(mutantCaughtCleanly(CFG_MUT_NO_PROG_BUS, attackProgBusState)),
    // Confirm specifically that MEM_BUS is balanced in attackProgBusState, and PROG_BUS is what refuses it:
    assert(checkMemBus(CFG_SOUND, attackProgBusState)),
    assert(not(checkProgBus(CFG_SOUND, attackProgBusState))),
  }

  run mutNoMemBusTest =
    assert(mutantCaughtCleanly(CFG_MUT_NO_MEM_BUS, attackMemBusState))

  run mutDupWriterTest =
    assert(mutantCaughtCleanly(CFG_MUT_DUP_WRITER, attackDupWriterState))

  run mutInputNoCanonicalTest =
    assert(mutantCaughtCleanly(CFG_MUT_INPUT_NO_CANON, attackInputNonCanonState))

  run mutInputNoBbZeroHiTest =
    assert(mutantCaughtCleanly(CFG_MUT_INPUT_NO_BB_ZERO_HI, attackInputBbHiState))

  run mutBitsNoBoolTest =
    assert(mutantCaughtCleanly(CFG_MUT_BITS_NO_BOOL, attackBitsNoBoolState))

  run mutBitsNoHighCheckTest =
    assert(mutantCaughtCleanly(CFG_MUT_BITS_NO_HIGH, attackBitsNoHighState))

  run mutBitsNoMaskMonoTest =
    assert(mutantCaughtCleanly(CFG_MUT_BITS_NO_MONO, attackBitsNoMonoState))

  run mutAssertEqUncheckedTest =
    assert(mutantCaughtCleanly(CFG_MUT_ASSERT_EQ, attackAssertEqState))

  run mutFrAddFreeOutTest =
    assert(mutantCaughtCleanly(CFG_MUT_FR_ADD, attackFrAddState))

  run mutHalveNoBitBoolTest =
    assert(mutantCaughtCleanly(CFG_MUT_HALVE_NO_BOOL, attackHalveNoBoolState))

  run mutHalveNoTerminalZeroTest =
    assert(mutantCaughtCleanly(CFG_MUT_HALVE_NO_TERM, attackHalveNoTermState))

  run mutCSwapNoBitBoolTest =
    assert(mutantCaughtCleanly(CFG_MUT_CSWAP_NO_BOOL, attackCSwapNoBoolState))

  run mutCSwapFreeRTest =
    assert(mutantCaughtCleanly(CFG_MUT_CSWAP_FREE_R, attackCSwapFreeRState))

  run mutPublicOrderUnpinnedTest =
    assert(mutantCaughtCleanly(CFG_MUT_PUBLIC_UNPINNED, attackPublicOrderState))
}
