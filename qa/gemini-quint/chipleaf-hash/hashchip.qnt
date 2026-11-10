// Soundness Review Model — Poseidon-only HASH Chip and Bus Lookup
// Design under test:
//   - crates/mori-chip/src/poseidon.rs (PoseidonCfg::configure, PoseidonCfg::sponge, tests)
//   - crates/mori-chip/src/bus.rs (BusConfig::configure, lookup_any on lane 0 and lane 1)
//   - crates/mori-chip/src/rfcore.rs (RfLeaf::synthesize pos block)
//   - crates/mori-chip/src/alu.rs (AluConfig::configure provider chip pin)
//   - research/sherlock-processor-analogy-20261110/experiments-3.md sections 2 and 4
//   - openspec/changes/sherlock-vm-consensus-20261107/design.md section 2.7.8 (D1, D4, D8)

module hashchip {
  const MUTANT: str

  // ---------------------------------------------------------------- Constants
  // Chip IDs on the bus (0 is reserved for inactive / no request, D8)
  pure val CHIP_NONE: int = 0
  pure val CHIP_ALU: int = 1
  pure val CHIP_HASH: int = 3

  // Bus key for a 2-input Poseidon hash provider
  pure val BUS_KEY: int = 2

  // Permutation geometry: 1 absorb row + 2 round rows (R = 2) + 1 output row = 4 rows
  pure val WIDTH: int = 3
  pure val ROUNDS: int = 2
  pure val ROWS_PER_PERM: int = 4
  pure val ADVICE_COLS: int = 10

  // Fixed round constants (RC0 on absorb rows; RC_R0 on full round 0; RC_R1 on partial round 1)
  pure val RC0: List[int] = [1, 2, 3]
  pure val RC_R0: List[int] = [4, 5, 6]
  pure val RC_R1: List[int] = [0, 0, 0]

  pure val ZERO_TUPLE: List[int] = [0, 0, 0, 0, 0]
  pure val ZERO_ROW: List[int] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]

  // Layout row offsets (48 rows total: indices 0..47):
  //   Rows  0..3 : Provider sponge 0 (n = 2, inputs [1, 2], provide = true)
  //   Rows  4..7 : Provider sponge 1 (n = 2, inputs [2, 1], provide = true)
  //   Rows  8..11: In-region sponge 0 (n = 2, inputs [1, 3], provide = false)
  //   Rows 12..19: In-region sponge 1 (n = 3, inputs [1, 2, 3], provide = false; 2 perms: first + tail)
  //   Rows 20..27: In-region sponge 2 (n = 4, inputs [1, 2, 1, 3], provide = false; 2 perms: first + chain)
  //   Rows 28..39: In-region sponge 3 (n = 5, inputs [2, 1, 3, 2, 1], provide = false; 3 perms: first + chain + tail)
  //   Rows 40..43: Padding rows in standard layout; 1-input sponge (n = 1, input [2]) in H12 `one_input_sponge`
  //   Row  44    : ALU provider row (q_prov = 1, q_alu = true, inputs (key=2, in0=1, in1=2))
  //   Row  45    : Padding row (q_prov = 0, q_req = 0, no selectors)
  //   Row  46    : Request row 0 (q_req = 1, both lane 0 and lane 1 active)
  //   Row  47    : Request row 1 (q_req = 1, lane 0 active, lane 1 inactive)
  pure val TOTAL_ROWS: int = 48

  // ---------------------------------------------------------------- Types
  type FixedRow = {
    q_req: int,
    q_prov: int,
    rc: List[int],
    q_full: bool,
    q_part: bool,
    q_first: bool,
    q_chain: bool,
    q_tail: bool,
    q_alu: bool
  }

  type CellPos = {
    row: int,
    col: int
  }

  type CopyEq = {
    lhs: CellPos,
    rhs: CellPos
  }

  type InputPin = {
    pos: CellPos,
    expected: int
  }

  type SpongeSpec = {
    spongeId: int,
    startRow: int,
    inputs: List[int],
    provide: bool,
    outCell: CellPos
  }

  type Layout = {
    fixedRows: List[FixedRow],
    copyEqs: List[CopyEq],
    inputPins: List[InputPin],
    sponges: List[SpongeSpec]
  }

  type Witness = {
    advice: List[List[int]]
  }

  type SpongeBuildAcc = {
    rows: List[List[int]],
    st: List[int]
  }

  // ---------------------------------------------------------------- Opaque Injective Round Functions & Real Addition
  // Strictly increasing nonlinear bijection on integers: sbox(x) = x*|x| + x + 1.
  // Because sbox is nonlinear and strictly increasing, addition in absorb cannot commute with round_r,
  // and combined with invertible positive-weight mixing matrices, roundFull and roundPart are injective
  // in both state s[0..3] and round constants rc[0..3].
  // LEAD REPAIR (execution): the lane's integer sbox x*|x| + x + 1 overflowed quint's 64-bit
  // integers after a few rounds (QNT601), so no test or run could execute. Rounds now work in the
  // field Z_p, p = 1000003 (prime; gcd(5, p - 1) = 1, so x^5 is a bijection of Z_p), and the
  // mixing matrices (det -1148 and -5396, not divisible by p) are invertible mod p. Each round is
  // therefore a bijection of Z_p^3, as the task asks (opaque, injective). Absorb stays real addition.
  pure val PRIME: int = 1000003
  pure def md(x: int): int = ((x % PRIME) + PRIME) % PRIME
  pure def sbox(x: int): int = {
    val y = md(x)
    val y2 = (y * y) % PRIME
    val y4 = (y2 * y2) % PRIME
    (y4 * y) % PRIME
  }

  pure def roundFull(s: List[int], rc: List[int]): List[int] = {
    val u0 = sbox(s[0] + rc[0] + 1)
    val u1 = sbox(s[1] + rc[1] + 2)
    val u2 = sbox(s[2] + rc[2] + 3)
    val n0 = md(11 * u0 + 13 * u1 + 17 * u2 + 2 * rc[0] + 3 * rc[1] + 5 * rc[2] + 7)
    val n1 = md(13 * u0 + 17 * u1 + 11 * u2 + 3 * rc[0] + 5 * rc[1] + 2 * rc[2] + 11)
    val n2 = md(17 * u0 + 11 * u1 + 13 * u2 + 5 * rc[0] + 2 * rc[1] + 3 * rc[2] + 13)
    [n0, n1, n2]
  }

  pure def roundPart(s: List[int], rc: List[int]): List[int] = {
    val u0 = md(s[0] + rc[0] + 1)
    val u1 = md(s[1] + rc[1] + 2)
    val u2 = sbox(s[2] + rc[2] + 3)
    val n0 = md(19 * u0 + 23 * u1 + 29 * u2 + 2 * rc[0] + 3 * rc[1] + 5 * rc[2] + 17)
    val n1 = md(23 * u0 + 29 * u1 + 19 * u2 + 3 * rc[0] + 5 * rc[1] + 2 * rc[2] + 19)
    val n2 = md(29 * u0 + 19 * u1 + 23 * u2 + 5 * rc[0] + 2 * rc[1] + 3 * rc[2] + 23)
    [n0, n1, n2]
  }

  // One full permutation P(state): ROUNDS = 2 rounds (round 0 full with RC_R0, round 1 partial with RC_R1)
  pure def permuteP(s: List[int]): List[int] = {
    val s1 = roundFull(s, RC_R0)
    roundPart(s1, RC_R1)
  }

  // Real componentwise addition on width-3 state vectors
  pure def addState3(a: List[int], b: List[int]): List[int] =
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]

  // Reference fixed-length sponge hashRef(xs) for len(xs) >= 1 (stdlib form):
  //   state = (0, 0, len(xs))
  //   for each chunk of 2 (last chunk may have 1, adding only to element 0):
  //     state = state + chunk + RC0
  //     state = P(state)
  //   output = state[0]
  pure def hashRef(xs: List[int]): int = {
    val n = xs.length()
    val nChunks = (n + 1) / 2
    val initSt = [0, 0, n]
    val finalSt = range(0, nChunks).foldl(initSt, (st, ci) => {
      val idx0 = 2 * ci
      val idx1 = 2 * ci + 1
      val in0 = xs[idx0]
      val in1 = if (idx1 < n) xs[idx1] else 0
      val absorbed = addState3(addState3(st, [in0, in1, 0]), RC0)
      permuteP(absorbed)
    })
    finalSt[0]
  }

  // Reference ALU function for chip id 1 (ALU):
  //   For key = k, in0 = x, in1 = y: aluRef(k, x, y) = 100 * k + 10 * x + y + 1 (always distinct from hashRef)
  pure def aluRef(k: int, x: int, y: int): int =
    100 * k + 10 * x + y + 1

  // ---------------------------------------------------------------- Fixed Layout Construction (Keygen)
  pure val emptyFixedRow: FixedRow = {
    q_req: 0,
    q_prov: 0,
    rc: [0, 0, 0],
    q_full: false,
    q_part: false,
    q_first: false,
    q_chain: false,
    q_tail: false,
    q_alu: false
  }

  // Build fixed rows for one sponge of length n at startRow
  pure def buildSpongeFixedRows(mt: str, n: int, provide: bool): List[FixedRow] = {
    val nChunks = (n + 1) / 2
    val emptyF: List[FixedRow] = []
    range(0, nChunks).foldl(emptyF, (acc, ci) => {
      val rem = n - 2 * ci
      val isFirst = (ci == 0)
      val isFullChunk = (rem >= 2)
      // H6 `len_wrong`: folds chunk count nChunks instead of n, and also adds it on chain rows
      val foldedLen =
        if (mt == "len_wrong") nChunks
        else if (mt == "capacity_advice") 0
        else if (isFirst) n
        else 0
      val absorbRc = [RC0[0], RC0[1], RC0[2] + foldedLen]
      // H11 `prov_on_chain_row`: enables q_prov on the chain absorb row of the n = 4 sponge
      val provBit =
        if (isFirst and provide) 1
        else if (mt == "prov_on_chain_row" and n == 4 and ci == 1) 1
        else 0
      val rAbsorb: FixedRow = {
        ...emptyFixedRow,
        q_prov: provBit,
        rc: absorbRc,
        q_first: isFirst,
        q_chain: (not(isFirst) and isFullChunk),
        q_tail: (not(isFirst) and not(isFullChunk))
      }
      val rRound0: FixedRow = {
        ...emptyFixedRow,
        rc: RC_R0,
        q_full: true
      }
      val rRound1: FixedRow = {
        ...emptyFixedRow,
        rc: RC_R1,
        q_part: true
      }
      val rOut: FixedRow = emptyFixedRow
      acc.append(rAbsorb).append(rRound0).append(rRound1).append(rOut)
    })
  }

  pure def buildLayout(mt: str): Layout = {
    // Sponges:
    //   0: provider n=2 [1, 2] at row 0..3
    //   1: provider n=2 [2, 1] at row 4..7
    //   2: in-region n=2 [1, 3] at row 8..11
    //   3: in-region n=3 [1, 2, 3] at row 12..19
    //   4: in-region n=4 [1, 2, 1, 3] at row 20..27
    //   5: in-region n=5 [2, 1, 3, 2, 1] at row 28..39
    //   (plus in H12 `one_input_sponge`: in-region n=1 [2] at row 40..43)
    val f0 = buildSpongeFixedRows(mt, 2, true)
    val f1 = buildSpongeFixedRows(mt, 2, true)
    val f2 = buildSpongeFixedRows(mt, 2, false)
    val f3 = buildSpongeFixedRows(mt, 3, false)
    val f4 = buildSpongeFixedRows(mt, 4, false)
    val f5 = buildSpongeFixedRows(mt, 5, false)
    val f6 =
      if (mt == "one_input_sponge")
        buildSpongeFixedRows(mt, 1, false)
      else
        [emptyFixedRow, emptyFixedRow, emptyFixedRow, emptyFixedRow]
    val fAlu: FixedRow = { ...emptyFixedRow, q_prov: 1, q_alu: true }
    val fPad: FixedRow = emptyFixedRow
    val fReq0: FixedRow = { ...emptyFixedRow, q_req: 1 }
    val fReq1: FixedRow = { ...emptyFixedRow, q_req: 1 }

    val baseFixed = f0
      .concat(f1)
      .concat(f2)
      .concat(f3)
      .concat(f4)
      .concat(f5)
      .concat(f6)
      .append(fAlu)
      .append(fPad)
      .append(fReq0)
      .append(fReq1)

    // H13 `round_gate_dropped`: round 0 of in-region sponge 0 (row 9) has its q_full selector dropped
    val fixedRows =
      if (mt == "round_gate_dropped")
        baseFixed.replaceAt(9, { ...baseFixed[9], q_full: false })
      else
        baseFixed

    // Provider out copy constraints:
    //   Provider 0: (0, 4) == (3, 0) (or (1, 0) in H10 `out_from_nonfinal`, omitted in H9 `out_not_copied`)
    //   Provider 1: (4, 4) == (7, 0)
    val prov0OutSource: CellPos =
      if (mt == "out_from_nonfinal") { row: 1, col: 0 } else { row: 3, col: 0 }
    // LEAD REPAIR (vacuity): under `out_from_nonfinal` the honest builder reads `out` from row 1 of
    // EVERY provider sponge, but the lane moved only provider 0's copy, so provider 1's copy
    // constraint (4, 4) == (7, 0) rejected the honest witness under the mutant. Both providers
    // now copy from their non-final row 1 under the mutant, as the builder assumes.
    val prov1OutSource: CellPos =
      if (mt == "out_from_nonfinal") { row: 5, col: 0 } else { row: 7, col: 0 }
    val copyEqs: List[CopyEq] =
      if (mt == "out_not_copied")
        [{ lhs: { row: 4, col: 4 }, rhs: { row: 7, col: 0 } }]
      else
        [
          { lhs: { row: 0, col: 4 }, rhs: prov0OutSource },
          { lhs: { row: 4, col: 4 }, rhs: prov1OutSource }
        ]

    // Input copy constraints for in-region sponges (copied from caller cells):
    // Note: tail absorb row in1 (col 3 of row 16 and row 36) is NOT copied (padding)!
    // Note: in H12 `one_input_sponge`, row 40 col 2 is copied to 2, while row 40 col 3 (in1) is NOT copied!
    val basePins: List[InputPin] = [
      // In-region sponge 0 (n = 2, rows 8..11): [1, 3]
      { pos: { row: 8, col: 2 }, expected: 1 },
      { pos: { row: 8, col: 3 }, expected: 3 },
      // In-region sponge 1 (n = 3, rows 12..19): [1, 2, 3]
      { pos: { row: 12, col: 2 }, expected: 1 },
      { pos: { row: 12, col: 3 }, expected: 2 },
      { pos: { row: 16, col: 2 }, expected: 3 },
      // In-region sponge 2 (n = 4, rows 20..27): [1, 2, 1, 3]
      { pos: { row: 20, col: 2 }, expected: 1 },
      { pos: { row: 20, col: 3 }, expected: 2 },
      { pos: { row: 24, col: 2 }, expected: 1 },
      { pos: { row: 24, col: 3 }, expected: 3 },
      // In-region sponge 3 (n = 5, rows 28..39): [2, 1, 3, 2, 1]
      { pos: { row: 28, col: 2 }, expected: 2 },
      { pos: { row: 28, col: 3 }, expected: 1 },
      { pos: { row: 32, col: 2 }, expected: 3 },
      { pos: { row: 32, col: 3 }, expected: 2 },
      { pos: { row: 36, col: 2 }, expected: 1 }
    ]
    val inputPins =
      if (mt == "one_input_sponge")
        basePins.append({ pos: { row: 40, col: 2 }, expected: 2 })
      else
        basePins

    // H10 `out_from_nonfinal`: also returns first perm's output row (23, 0) for the n = 4 sponge
    val sp2OutCell: CellPos =
      if (mt == "out_from_nonfinal") { row: 23, col: 0 } else { row: 27, col: 0 }

    val baseSponges: List[SpongeSpec] = [
      { spongeId: 0, startRow: 0, inputs: [1, 2], provide: true, outCell: prov0OutSource },
      { spongeId: 1, startRow: 4, inputs: [2, 1], provide: true, outCell: { row: 7, col: 0 } },
      { spongeId: 2, startRow: 8, inputs: [1, 3], provide: false, outCell: { row: 11, col: 0 } },
      { spongeId: 3, startRow: 12, inputs: [1, 2, 3], provide: false, outCell: { row: 19, col: 0 } },
      { spongeId: 4, startRow: 20, inputs: [1, 2, 1, 3], provide: false, outCell: sp2OutCell },
      { spongeId: 5, startRow: 28, inputs: [2, 1, 3, 2, 1], provide: false, outCell: { row: 39, col: 0 } }
    ]
    val sponges =
      if (mt == "one_input_sponge")
        baseSponges.append({
          spongeId: 6,
          startRow: 40,
          inputs: [2],
          provide: false,
          outCell: { row: 43, col: 0 }
        })
      else
        baseSponges

    {
      fixedRows: fixedRows,
      copyEqs: copyEqs,
      inputPins: inputPins,
      sponges: sponges
    }
  }

  // ---------------------------------------------------------------- Circuit Gates & Bus Lookups
  // Evaluate all gate equations on row rIdx (rotations -1, 0, +1)
  pure def rowGatesOk(mt: str, lay: Layout, adv: List[List[int]], rIdx: int): bool = {
    val fx = lay.fixedRows[rIdx]
    val cur = adv[rIdx]
    val prev = if (rIdx > 0) adv[rIdx - 1] else ZERO_ROW
    val nx = if (rIdx + 1 < TOTAL_ROWS) adv[rIdx + 1] else ZERO_ROW

    // 1. Full round gate: nx[0..3] == roundFull(cur[0..3], fx.rc)
    val fullOk =
      if (fx.q_full) {
        val exp = roundFull([cur[0], cur[1], cur[2]], fx.rc)
        nx[0] == exp[0] and nx[1] == exp[1] and nx[2] == exp[2]
      } else true

    // 2. Partial round gate: nx[0..3] == roundPart(cur[0..3], fx.rc)
    val partOk =
      if (fx.q_part) {
        val exp = roundPart([cur[0], cur[1], cur[2]], fx.rc)
        nx[0] == exp[0] and nx[1] == exp[1] and nx[2] == exp[2]
      } else true

    // 3. First absorb gate (q_first):
    //    - H1 `skip_absorb_first`: does not add in0/in1
    //    - H5 `capacity_advice`: adds advice cell cur[5] to element 2 instead of folding len into fixed rc[2]
    //    - Provider pin when q_prov == 1: pins chip == CHIP_HASH (dropped in H8 `chip_unpinned`)
    //      and key == BUS_KEY (dropped in H7 `key_unpinned`).
    val firstOk =
      if (fx.q_first) {
        val in0 = if (mt == "skip_absorb_first") 0 else cur[2]
        val in1 = if (mt == "skip_absorb_first") 0 else cur[3]
        val capAdv = if (mt == "capacity_advice") cur[5] else 0
        val stOk = and {
          nx[0] == in0 + fx.rc[0],
          nx[1] == in1 + fx.rc[1],
          nx[2] == capAdv + fx.rc[2]
        }
        val provPinOk =
          if (fx.q_prov == 1) and {
            (mt == "chip_unpinned") or (cur[0] == CHIP_HASH),
            (mt == "key_unpinned") or (cur[1] == BUS_KEY)
          } else true
        stOk and provPinOk
      } else true

    // 4. Chain absorb gate (q_chain):
    //    - H2 `skip_absorb_chain`: does not add cur[2], cur[3]
    //    - H4 `chain_no_prev`: does not add prev[0..3]
    val chainOk =
      if (fx.q_chain) {
        val p0 = if (mt == "chain_no_prev") 0 else prev[0]
        val p1 = if (mt == "chain_no_prev") 0 else prev[1]
        val p2 = if (mt == "chain_no_prev") 0 else prev[2]
        val in0 = if (mt == "skip_absorb_chain") 0 else cur[2]
        val in1 = if (mt == "skip_absorb_chain") 0 else cur[3]
        and {
          nx[0] == p0 + in0 + fx.rc[0],
          nx[1] == p1 + in1 + fx.rc[1],
          nx[2] == p2 + fx.rc[2]
        }
      } else true

    // 5. Tail absorb gate (q_tail):
    //    - Absorbs in0 (cur[2]) only; in H3 `tail_absorbs_in1`, also adds cur[3] (in1)
    val tailOk =
      if (fx.q_tail) {
        val in0 = cur[2]
        val in1 = if (mt == "tail_absorbs_in1") cur[3] else 0
        and {
          nx[0] == prev[0] + in0 + fx.rc[0],
          nx[1] == prev[1] + in1 + fx.rc[1],
          nx[2] == prev[2] + fx.rc[2]
        }
      } else true

    // 6. Second chip gate (ALU, q_alu):
    //    - Pins chip == CHIP_ALU (dropped in H8 `chip_unpinned`) and out == aluRef(key, in0, in1)
    val aluOk =
      if (fx.q_alu) and {
        (mt == "chip_unpinned") or (cur[0] == CHIP_ALU),
        cur[4] == aluRef(cur[1], cur[2], cur[3])
      } else true

    // 7. Request row gate (q_req == 1):
    //    - Active request lane gates its chip column to the target chip id (CHIP_HASH or CHIP_ALU);
    //    - Inactive request lane has chip == 0 (the lookup then forces the whole tuple to ZERO_TUPLE).
    val reqOk =
      if (fx.q_req == 1) and {
        (cur[0] == CHIP_NONE or cur[0] == CHIP_ALU or cur[0] == CHIP_HASH),
        (cur[5] == CHIP_NONE or cur[5] == CHIP_ALU or cur[5] == CHIP_HASH)
      } else true

    fullOk and partOk and firstOk and chainOk and tailOk and aluOk and reqOk
  }

  // Copy constraints (permutation argument): provider out copies + caller input cell copies
  pure def copyConstraintsOk(lay: Layout, adv: List[List[int]]): bool = and {
    lay.copyEqs.indices().forall(i => {
      val ce = lay.copyEqs[i]
      adv[ce.lhs.row][ce.lhs.col] == adv[ce.rhs.row][ce.rhs.col]
    }),
    lay.inputPins.indices().forall(i => {
      val pin = lay.inputPins[i]
      adv[pin.pos.row][pin.pos.col] == pin.expected
    })
  }

  // Bus table side: { q_prov(row) * a[0..5](row) : every row of the region }
  // In H14 `prov_zero_not_zero`, rows with q_prov == 0 put their unscaled tuple a[0..5] on the table side!
  pure def providerTableSet(mt: str, lay: Layout, adv: List[List[int]]): Set[List[int]] = {
    val emptySet: Set[List[int]] = Set()
    range(0, TOTAL_ROWS).foldl(emptySet, (acc, rIdx) => {
      val qp = lay.fixedRows[rIdx].q_prov
      val r = adv[rIdx]
      val tup =
        if (qp == 1 or mt == "prov_zero_not_zero")
          [r[0], r[1], r[2], r[3], r[4]]
        else
          ZERO_TUPLE
      acc.union(Set(tup))
    })
  }

  // Both bus lookups: for every row, (q_req * lane0) in providerTableSet and (q_req * lane1) in providerTableSet
  pure def busLookupsOk(mt: str, lay: Layout, adv: List[List[int]]): bool = {
    val pSet = providerTableSet(mt, lay, adv)
    range(0, TOTAL_ROWS).foldl(true, (ok, rIdx) => {
      val qr = lay.fixedRows[rIdx].q_req
      val r = adv[rIdx]
      val l0 = if (qr == 1) [r[0], r[1], r[2], r[3], r[4]] else ZERO_TUPLE
      val l1 = if (qr == 1) [r[5], r[6], r[7], r[8], r[9]] else ZERO_TUPLE
      ok and pSet.contains(l0) and pSet.contains(l1)
    })
  }

  // Complete circuit acceptance predicate
  pure def acceptsWithMutant(mt: str, w: Witness): bool = {
    val lay = buildLayout(mt)
    and {
      w.advice.length() == TOTAL_ROWS,
      range(0, TOTAL_ROWS).foldl(true, (ok, rIdx) =>
        ok and w.advice[rIdx].length() == ADVICE_COLS and rowGatesOk(mt, lay, w.advice, rIdx)
      ),
      copyConstraintsOk(lay, w.advice),
      busLookupsOk(mt, lay, w.advice)
    }
  }

  pure def accepts(w: Witness): bool =
    acceptsWithMutant(MUTANT, w)

  // ---------------------------------------------------------------- Refinement Invariant `invRefine`
  // Check a single request lane tuple [chip, key, in0, in1, out]:
  //   (a) Every active HASH request (chip == CHIP_HASH) has key == 2 and out == hashRef([in0, in1]).
  //   (c) Every active ALU request (chip == CHIP_ALU) has out == aluRef(key, in0, in1).
  //   Inactive requests (chip == 0) must be ZERO_TUPLE.
  pure def laneRefines(tup: List[int]): bool = {
    val chip = tup[0]
    val k = tup[1]
    val in0 = tup[2]
    val in1 = tup[3]
    val out = tup[4]
    if (chip == CHIP_HASH)
      (k == BUS_KEY and out == hashRef([in0, in1]))
    else if (chip == CHIP_ALU)
      (out == aluRef(k, in0, in1))
    else
      (tup == ZERO_TUPLE)
  }

  // Refinement invariant `invRefine`:
  //   (a) Every active HASH request on either lane of any request row has key = 2 and out = hashRef([in0, in1]);
  //   (b) Every in-region sponge's output cell equals hashRef(inputs);
  //   (c) Every ALU request is a true ALU tuple.
  pure def refinesWith(mt: str, w: Witness): bool = {
    val lay = buildLayout(mt)
    val reqsOk = range(0, TOTAL_ROWS).foldl(true, (ok, rIdx) => {
      if (lay.fixedRows[rIdx].q_req == 1) {
        val r = w.advice[rIdx]
        val l0 = [r[0], r[1], r[2], r[3], r[4]]
        val l1 = [r[5], r[6], r[7], r[8], r[9]]
        ok and laneRefines(l0) and laneRefines(l1)
      } else ok
    })
    val spongesOk = lay.sponges.indices().forall(i => {
      val sp = lay.sponges[i]
      if (not(sp.provide)) {
        val actualOut = w.advice[sp.outCell.row][sp.outCell.col]
        actualOut == hashRef(sp.inputs)
      } else true
    })
    reqsOk and spongesOk
  }

  pure def refines(w: Witness): bool =
    refinesWith(MUTANT, w)

  // ---------------------------------------------------------------- Honest Witness Builder & Downstream Recompute
  // Build honest rows for one sponge
  pure def buildHonestSpongeRows(mt: str, fixedSlice: List[FixedRow], inputs: List[int], provide: bool): List[List[int]] = {
    val n = inputs.length()
    val nChunks = (n + 1) / 2
    val emptyRows: List[List[int]] = []
    val initAcc: SpongeBuildAcc = { rows: emptyRows, st: [0, 0, 0] }
    val built = range(0, nChunks).foldl(initAcc, (acc, ci) => {
      val baseOff = ci * ROWS_PER_PERM
      val fxAbs = fixedSlice[baseOff]
      val fxR0 = fixedSlice[baseOff + 1]
      val fxR1 = fixedSlice[baseOff + 2]
      val idx0 = 2 * ci
      val idx1 = 2 * ci + 1
      val in0 = inputs[idx0]
      val in1 = if (idx1 < n) inputs[idx1] else 0
      val chipVal = if (ci == 0 and provide) CHIP_HASH else 0
      val keyVal = if (ci == 0 and provide) BUS_KEY else 0
      val capAdv = if (ci == 0) n else 0
      val absIn0 = if (mt == "skip_absorb_first" and ci == 0) 0
                   else if (mt == "skip_absorb_chain" and fxAbs.q_chain) 0
                   else in0
      val absIn1 = if (mt == "skip_absorb_first" and ci == 0) 0
                   else if (mt == "skip_absorb_chain" and fxAbs.q_chain) 0
                   else if (fxAbs.q_tail and mt != "tail_absorbs_in1") 0
                   else in1
      val prevSt = if (ci == 0 or (fxAbs.q_chain and mt == "chain_no_prev")) [0, 0, 0] else acc.st
      val c2Extra = if (ci == 0 and mt == "capacity_advice") capAdv else 0
      val sAfterAbsorb = [
        prevSt[0] + absIn0 + fxAbs.rc[0],
        prevSt[1] + absIn1 + fxAbs.rc[1],
        prevSt[2] + c2Extra + fxAbs.rc[2]
      ]
      val sAfterR0 = roundFull(sAfterAbsorb, fxR0.rc)
      val sAfterR1 = roundPart(sAfterR0, fxR1.rc)

      val rowAbsorb = [chipVal, keyVal, in0, in1, 0, capAdv, 0, 0, 0, 0]
      val rowR0 = [sAfterAbsorb[0], sAfterAbsorb[1], sAfterAbsorb[2], 0, 0, 0, 0, 0, 0, 0]
      val rowR1 = [sAfterR0[0], sAfterR0[1], sAfterR0[2], 0, 0, 0, 0, 0, 0, 0]
      val rowOut = [sAfterR1[0], sAfterR1[1], sAfterR1[2], 0, 0, 0, 0, 0, 0, 0]
      {
        rows: acc.rows.append(rowAbsorb).append(rowR0).append(rowR1).append(rowOut),
        st: sAfterR1
      }
    })
    if (provide) {
      val outRowOff = if (mt == "out_from_nonfinal") 1 else (nChunks * ROWS_PER_PERM - 1)
      val finalOut = built.rows[outRowOff][0]
      val firstRow = built.rows[0].replaceAt(4, finalOut)
      built.rows.replaceAt(0, firstRow)
    } else {
      built.rows
    }
  }

  pure def buildHonestWitness(mt: str): Witness = {
    val lay = buildLayout(mt)
    val r0 = buildHonestSpongeRows(mt, lay.fixedRows.slice(0, 4), [1, 2], true)
    val r1 = buildHonestSpongeRows(mt, lay.fixedRows.slice(4, 8), [2, 1], true)
    val r2 = buildHonestSpongeRows(mt, lay.fixedRows.slice(8, 12), [1, 3], false)
    val r3 = buildHonestSpongeRows(mt, lay.fixedRows.slice(12, 20), [1, 2, 3], false)
    val r4 = buildHonestSpongeRows(mt, lay.fixedRows.slice(20, 28), [1, 2, 1, 3], false)
    val r5 = buildHonestSpongeRows(mt, lay.fixedRows.slice(28, 40), [2, 1, 3, 2, 1], false)
    val r6 =
      if (mt == "one_input_sponge")
        buildHonestSpongeRows(mt, lay.fixedRows.slice(40, 44), [2], false)
      else
        [ZERO_ROW, ZERO_ROW, ZERO_ROW, ZERO_ROW]

    val aluOut = aluRef(2, 1, 2)
    val rAlu = [CHIP_ALU, 2, 1, 2, aluOut, 0, 0, 0, 0, 0]
    val rPad = ZERO_ROW

    val h12Out = r0[0][4]
    val h21Out = r1[0][4]
    val rReq0 = [CHIP_HASH, BUS_KEY, 1, 2, h12Out, CHIP_ALU, 2, 1, 2, aluOut]
    val rReq1 = [CHIP_HASH, BUS_KEY, 2, 1, h21Out, 0, 0, 0, 0, 0]

    val allRows = r0
      .concat(r1)
      .concat(r2)
      .concat(r3)
      .concat(r4)
      .concat(r5)
      .concat(r6)
      .append(rAlu)
      .append(rPad)
      .append(rReq0)
      .append(rReq1)
    { advice: allRows }
  }

  // Honest witness where Provider Sponge 0 ([1, 2]) is reused for two HASH requests
  pure val hw_provider_reused: Witness = {
    val base = buildHonestWitness("none")
    val h12Out = base.advice[0][4]
    val aluOut = aluRef(2, 1, 2)
    val req0 = [CHIP_HASH, BUS_KEY, 1, 2, h12Out, CHIP_ALU, 2, 1, 2, aluOut]
    val req1 = [CHIP_HASH, BUS_KEY, 1, 2, h12Out, 0, 0, 0, 0, 0]
    { advice: base.advice.replaceAt(46, req0).replaceAt(47, req1) }
  }

  // Recompute all downstream state rows from row `tamperRow` onwards within the sponges,
  // so that after a single-cell tamper at `tamperRow`, only the gate producing `tamperRow` can fail.
  pure def cascadeFromRow(mt: str, adv: List[List[int]], tamperRow: int): List[List[int]] = {
    val lay = buildLayout(mt)
    val afterStates = range(0, 43).foldl(adv, (curAdv, rIdx) => {
      if (rIdx < tamperRow) curAdv
      else {
        val fx = lay.fixedRows[rIdx]
        val cur = curAdv[rIdx]
        val prev = if (rIdx > 0) curAdv[rIdx - 1] else ZERO_ROW
        val oldNx = curAdv[rIdx + 1]
        if (fx.q_first) {
          val in0 = if (mt == "skip_absorb_first") 0 else cur[2]
          val in1 = if (mt == "skip_absorb_first") 0 else cur[3]
          val capAdv = if (mt == "capacity_advice") cur[5] else 0
          val newNx = oldNx
            .replaceAt(0, in0 + fx.rc[0])
            .replaceAt(1, in1 + fx.rc[1])
            .replaceAt(2, capAdv + fx.rc[2])
          curAdv.replaceAt(rIdx + 1, newNx)
        } else if (fx.q_chain) {
          val p0 = if (mt == "chain_no_prev") 0 else prev[0]
          val p1 = if (mt == "chain_no_prev") 0 else prev[1]
          val p2 = if (mt == "chain_no_prev") 0 else prev[2]
          val in0 = if (mt == "skip_absorb_chain") 0 else cur[2]
          val in1 = if (mt == "skip_absorb_chain") 0 else cur[3]
          val newNx = oldNx
            .replaceAt(0, p0 + in0 + fx.rc[0])
            .replaceAt(1, p1 + in1 + fx.rc[1])
            .replaceAt(2, p2 + fx.rc[2])
          curAdv.replaceAt(rIdx + 1, newNx)
        } else if (fx.q_tail) {
          val in0 = cur[2]
          val in1 = if (mt == "tail_absorbs_in1") cur[3] else 0
          val newNx = oldNx
            .replaceAt(0, prev[0] + in0 + fx.rc[0])
            .replaceAt(1, prev[1] + in1 + fx.rc[1])
            .replaceAt(2, prev[2] + fx.rc[2])
          curAdv.replaceAt(rIdx + 1, newNx)
        } else if (fx.q_full) {
          val exp = roundFull([cur[0], cur[1], cur[2]], fx.rc)
          val newNx = oldNx.replaceAt(0, exp[0]).replaceAt(1, exp[1]).replaceAt(2, exp[2])
          curAdv.replaceAt(rIdx + 1, newNx)
        } else if (fx.q_part) {
          val exp = roundPart([cur[0], cur[1], cur[2]], fx.rc)
          val newNx = oldNx.replaceAt(0, exp[0]).replaceAt(1, exp[1]).replaceAt(2, exp[2])
          curAdv.replaceAt(rIdx + 1, newNx)
        } else {
          curAdv
        }
      }
    })
    // If provider 0's output changed and out_not_copied is not active (or tamper wasn't on (0, 4)),
    // sync provider 0's out cell (0, 4) when tamperRow <= 3 and tamperRow != 0
    if (mt != "out_not_copied" and tamperRow >= 1 and tamperRow <= 3) {
      val outSrcRow = if (mt == "out_from_nonfinal") 1 else 3
      val newOut0 = afterStates[outSrcRow][0]
      val upd0 = afterStates[0].replaceAt(4, newOut0)
      val updReq0 = afterStates[46].replaceAt(4, newOut0)
      afterStates.replaceAt(0, upd0).replaceAt(46, updReq0)
    } else {
      afterStates
    }
  }

  // Tamper a single advice cell (rIdx, cIdx) to newVal and cascade downstream state rows
  pure def tamperSingleCell(mt: str, rIdx: int, cIdx: int, newVal: int): Witness = {
    val base = buildHonestWitness(mt)
    val updatedRow = base.advice[rIdx].replaceAt(cIdx, newVal)
    val adv1 = base.advice.replaceAt(rIdx, updatedRow)
    // If we tampered an absorb row's input/capacity/key/out, cascade from rIdx;
    // if we tampered a state cell on row rIdx, cascade from rIdx (so rIdx's round gate recomputes rIdx+1..end)
    val cascaded = cascadeFromRow(mt, adv1, rIdx)
    // Also sync provider 0 out cell (0, 4) and request 0 if row 0's capacity (col 5) or inputs changed
    val synced =
      if (rIdx == 0 and cIdx == 5) {
        val newOut = cascaded[3][0]
        cascaded
          .replaceAt(0, cascaded[0].replaceAt(4, newOut))
          .replaceAt(46, cascaded[46].replaceAt(4, newOut))
      } else if (rIdx == 0 and cIdx == 1) {
        // Key tamper on provider 0: also put newVal as key on request row 46 lane 0
        cascaded.replaceAt(46, cascaded[46].replaceAt(1, newVal))
      } else if (rIdx == 0 and cIdx == 4) {
        // Out tamper on provider 0: also put newVal as out on request row 46 lane 0
        cascaded.replaceAt(46, cascaded[46].replaceAt(4, newVal))
      } else if (rIdx == 0 and cIdx == 0) {
        // Chip tamper on provider 0: also put newVal as chip on request row 46 lane 0
        cascaded.replaceAt(46, cascaded[46].replaceAt(0, newVal))
      } else if (rIdx == 44 and cIdx == 0) {
        // Chip tamper on ALU provider row 44: also request it on request row 47 lane 0
        val aluTup = cascaded[44]
        val reqRow = [newVal, aluTup[1], aluTup[2], aluTup[3], aluTup[4], 0, 0, 0, 0, 0]
        cascaded.replaceAt(47, reqRow)
      } else {
        cascaded
      }
    { advice: synced }
  }

  // Attack builder indexed by attackId (0 = honest, 1..14 = canonical attacks for H1..H14, 15 = provider reuse)
  pure def applyAttack(mt: str, attackId: int): Witness = {
    if (attackId == 0)
      buildHonestWitness(mt)
    else if (attackId == 15)
      hw_provider_reused
    else if (attackId == 1 or attackId == 2 or attackId == 4 or attackId == 6 or attackId == 10)
      // H1, H2, H4, H6, H10: witness built under the mutated gate/layout
      buildHonestWitness(mt)
    else if (attackId == 3)
      // H3 `tail_absorbs_in1`: tamper uncopied padding cell in1 (col 3) of tail row 16 to 3
      tamperSingleCell(mt, 16, 3, 3)
    else if (attackId == 5)
      // H5 `capacity_advice`: tamper capacity advice cell (col 5) of in-region sponge 0 (row 8) from 2 to 3
      tamperSingleCell(mt, 8, 5, 3)
    else if (attackId == 7)
      // H7 `key_unpinned`: tamper key (col 1) of provider sponge 0 (row 0) from 2 to 5
      tamperSingleCell(mt, 0, 1, 5)
    else if (attackId == 8)
      // H8 `chip_unpinned`: tamper chip (col 0) of HASH provider row 0 from CHIP_HASH (3) to CHIP_ALU (1);
      // request row 46 lane 0 is synced, so it asks ALU for (2, 1, 2) and receives a hash output.
      // LEAD REPAIR (vacuity): the lane tampered the ALU provider row 44 to CHIP_HASH instead. That removed
      // the only ALU tuple from the table, so the honest ALU request on row 46 lane 1 failed the lookup and
      // the witness was never accepted, under any gate (mutantH8 test failed on its first assert).
      tamperSingleCell(mt, 0, 0, CHIP_ALU)
    else if (attackId == 9)
      // H9 `out_not_copied`: tamper out (col 4) of provider sponge 0 (row 0) to 99
      tamperSingleCell(mt, 0, 4, 99)
    else if (attackId == 11) {
      // H11 `prov_on_chain_row`: row 24 is q_chain of n=4 sponge with q_prov = 1;
      // prover sets (chip=HASH, key=2, in0=1, in1=3, out=99) on row 24 and requests it on row 47 lane 0
      val base = buildHonestWitness(mt)
      val forgedChainRow = base.advice[24].replaceAt(0, CHIP_HASH).replaceAt(1, BUS_KEY).replaceAt(4, 99)
      val forgedReqRow = [CHIP_HASH, BUS_KEY, 1, 3, 99, 0, 0, 0, 0, 0]
      { advice: base.advice.replaceAt(24, forgedChainRow).replaceAt(47, forgedReqRow) }
    } else if (attackId == 12)
      // H12 `one_input_sponge`: tamper uncopied in1 (col 3) of 1-input sponge at row 40 from 0 to 3
      tamperSingleCell(mt, 40, 3, 3)
    else if (attackId == 13)
      // H13 `round_gate_dropped`: tamper state cell (10, 0) after dropped round gate on row 9 to 3
      tamperSingleCell(mt, 10, 0, 3)
    else if (attackId == 14) {
      // H14 `prov_zero_not_zero`: padding row 45 (q_prov = 0) holds [CHIP_HASH, 2, 1, 2, 99],
      // and request row 47 lane 0 requests [CHIP_HASH, 2, 1, 2, 99]
      val base = buildHonestWitness(mt)
      val forgedPadRow = [CHIP_HASH, BUS_KEY, 1, 2, 99, 0, 0, 0, 0, 0]
      val forgedReqRow = [CHIP_HASH, BUS_KEY, 1, 2, 99, 0, 0, 0, 0, 0]
      { advice: base.advice.replaceAt(45, forgedPadRow).replaceAt(47, forgedReqRow) }
    } else
      buildHonestWitness(mt)
  }

  // Replace an entire 5-element request lane tuple on reqRow (46 or 47), lane (0 or 1)
  pure def tamperRequestTuple(mt: str, reqRow: int, lane: int, tup: List[int]): Witness = {
    val base = buildHonestWitness(mt)
    val origRow = base.advice[reqRow]
    val baseCol = lane * 5
    val newRow = origRow
      .replaceAt(baseCol + 0, tup[0])
      .replaceAt(baseCol + 1, tup[1])
      .replaceAt(baseCol + 2, tup[2])
      .replaceAt(baseCol + 3, tup[3])
      .replaceAt(baseCol + 4, tup[4])
    { advice: base.advice.replaceAt(reqRow, newRow) }
  }

  // Named honest and counterexample witnesses for REPORT.md and hashchip_test.qnt
  pure val hw_honest: Witness = buildHonestWitness("none")
  pure val cex_H1_skip_absorb_first: Witness = applyAttack("skip_absorb_first", 1)
  pure val cex_H2_skip_absorb_chain: Witness = applyAttack("skip_absorb_chain", 2)
  pure val cex_H3_tail_absorbs_in1: Witness = applyAttack("tail_absorbs_in1", 3)
  pure val cex_H4_chain_no_prev: Witness = applyAttack("chain_no_prev", 4)
  pure val cex_H5_capacity_advice: Witness = applyAttack("capacity_advice", 5)
  pure val cex_H6_len_wrong: Witness = applyAttack("len_wrong", 6)
  pure val cex_H7_key_unpinned: Witness = applyAttack("key_unpinned", 7)
  pure val cex_H8_chip_unpinned: Witness = applyAttack("chip_unpinned", 8)
  pure val cex_H9_out_not_copied: Witness = applyAttack("out_not_copied", 9)
  pure val cex_H10_out_from_nonfinal: Witness = applyAttack("out_from_nonfinal", 10)
  pure val cex_H11_prov_on_chain_row: Witness = applyAttack("prov_on_chain_row", 11)
  pure val cex_H12_one_input_sponge: Witness = applyAttack("one_input_sponge", 12)
  pure val cex_H13_round_gate_dropped: Witness = applyAttack("round_gate_dropped", 13)
  pure val cex_H14_prov_zero_not_zero: Witness = applyAttack("prov_zero_not_zero", 14)

  // ---------------------------------------------------------------- State Machine (Nondeterministic Prover)
  pure val MODES: Set[int] = Set(0, 1, 2, 3)
  pure val ATTACK_IDS: Set[int] = 0.to(15)
  pure val ROW_DOM: Set[int] = Set(0, 1, 2, 3, 8, 9, 10, 11, 12, 16, 20, 24, 28, 32, 36, 40, 44, 45, 46, 47)
  pure val COL_DOM: Set[int] = 0.to(9)
  pure val VAL_DOM: Set[int] = Set(0, 1, 2, 3, 5, 99)
  pure val REQ_ROWS: Set[int] = Set(46, 47)
  pure val REQ_LANES: Set[int] = Set(0, 1)
  pure val REQ_CHIPS: Set[int] = Set(CHIP_NONE, CHIP_ALU, CHIP_HASH)
  pure val REQ_KEYS: Set[int] = Set(0, 2, 5)
  pure val REQ_INS: Set[int] = Set(0, 1, 2, 3)
  pure val REQ_OUTS: Set[int] = Set(0, 99, hashRef([1, 2]), hashRef([2, 1]), aluRef(2, 1, 2))

  var w: Witness
  var acc: bool
  var isTampered: bool

  action sample = {
    nondet mode = MODES.oneOf()
    nondet attackId = ATTACK_IDS.oneOf()
    nondet rIdx = ROW_DOM.oneOf()
    nondet cIdx = COL_DOM.oneOf()
    nondet newVal = VAL_DOM.oneOf()
    nondet reqRow = REQ_ROWS.oneOf()
    nondet reqLane = REQ_LANES.oneOf()
    nondet reqChip = REQ_CHIPS.oneOf()
    nondet reqKey = REQ_KEYS.oneOf()
    nondet reqIn0 = REQ_INS.oneOf()
    nondet reqIn1 = REQ_INS.oneOf()
    nondet reqOut = REQ_OUTS.oneOf()
    val cand =
      if (mode == 0)
        if (attackId == 15) hw_provider_reused else buildHonestWitness("none")
      else if (mode == 1)
        applyAttack(MUTANT, attackId)
      else if (mode == 2)
        tamperSingleCell(MUTANT, rIdx, cIdx, newVal)
      else
        tamperRequestTuple(MUTANT, reqRow, reqLane, [reqChip, reqKey, reqIn0, reqIn1, reqOut])
    val honest = buildHonestWitness("none")
    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = (cand != honest and cand != hw_provider_reused)
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- Invariants
  // Main refinement invariant: accepted witness implies (a), (b), (c)
  val invRefine: bool =
    acc implies refines(w)

  // Non-vacuity reachability invariants (negated: MUST be violated with exit 1 in hash_sound):
  // 1. An honest accepted witness where a HASH request is served
  val reachHashRequestServed: bool =
    not(acc and not(isTampered) and w.advice[46][0] == CHIP_HASH and w.advice[46][4] == hashRef([1, 2]))

  // 2. An honest accepted witness where a tail chunk occurs (row 16 on n=3 sponge and row 36 on n=5 sponge)
  val reachTailChunk: bool = {
    val lay = buildLayout(MUTANT)
    not(acc and not(isTampered) and lay.fixedRows[16].q_tail and lay.fixedRows[36].q_tail)
  }

  // 3. An honest accepted witness with a chained sponge (row 24 on n=4 sponge and row 32 on n=5 sponge)
  val reachChainedSponge: bool = {
    val lay = buildLayout(MUTANT)
    not(acc and not(isTampered) and lay.fixedRows[24].q_chain and lay.fixedRows[32].q_chain)
  }

  // 4. An accepted witness that does not refine (must NOT be violated in hash_sound: exit 0; violated in H1..H14: exit 1)
  val reachAcceptedBadTamper: bool =
    not(acc and not(refines(w)))
}

// Sound module instance
module hash_sound { import hashchip(MUTANT = "none").* }

// Required mutant module instances H1..H14 (each MUST violate invRefine with exit 1)
module hash_skip_absorb_first { import hashchip(MUTANT = "skip_absorb_first").* }
module hash_skip_absorb_chain { import hashchip(MUTANT = "skip_absorb_chain").* }
module hash_tail_absorbs_in1 { import hashchip(MUTANT = "tail_absorbs_in1").* }
module hash_chain_no_prev { import hashchip(MUTANT = "chain_no_prev").* }
module hash_capacity_advice { import hashchip(MUTANT = "capacity_advice").* }
module hash_len_wrong { import hashchip(MUTANT = "len_wrong").* }
module hash_key_unpinned { import hashchip(MUTANT = "key_unpinned").* }
module hash_chip_unpinned { import hashchip(MUTANT = "chip_unpinned").* }
module hash_out_not_copied { import hashchip(MUTANT = "out_not_copied").* }
module hash_out_from_nonfinal { import hashchip(MUTANT = "out_from_nonfinal").* }
module hash_prov_on_chain_row { import hashchip(MUTANT = "prov_on_chain_row").* }
module hash_one_input_sponge { import hashchip(MUTANT = "one_input_sponge").* }
module hash_round_gate_dropped { import hashchip(MUTANT = "round_gate_dropped").* }
module hash_prov_zero_not_zero { import hashchip(MUTANT = "prov_zero_not_zero").* }
