// Soundness Review Model — Lane `packsel`: Packed Selectors (`vsel.rs`, `rfcore.rs`, `poseidon.rs`, `m3.rs`, `mem.rs`)
// Circuit under test (commit f59ef318f, after merge ec23825e9):
//   - crates/mori-chip/src/vsel.rs (VSel, lagrange, code_column, pack, MAX_DEG = 6)
//   - crates/mori-chip/src/rfcore.rs (Layout::new, core_degrees, configure_with_params 867-962, synthesize 1147-1200)
//   - crates/mori-chip/src/poseidon.rs (one_abs code_column(meta, 3) 173-182, absorb gates 215-253, sponge 307-315)
//   - crates/mori-chip/src/m3.rs (WalkConfig::configure 103-145, walker assign 375-391)
//   - crates/mori-chip/src/mem.rs (SegMemConfig::configure 133-189, assign 207-216)
//   - crates/mori-chip/src/bus.rs, range.rs, dec.rs, and midnight-proofs circuit.rs / keygen.rs / lookup.rs

module packsel {
  const MUTANT: str

  // ---------------------------------------------------------------- Constants
  // Maximum polynomial degree of the leaf constraint system (vsel.rs:25)
  pure val MAX_DEG: int = 6

  // Core architectures (rfcore.rs:92-222)
  pure val ARCH_NARROW: int = 0
  pure val ARCH_WIDE: int = 1
  pure val ARCH_STAGGERED: int = 2

  // Selector polynomial kinds
  //   SEL_SIMPLE   : plain 0/1 selector column (q) of degree 1
  //   SEL_LAGRANGE : Lagrange indicator L_c(q) over 0..=n (or 1..=n in P3) of degree n
  //   SEL_RAW_Q    : raw fixed column value q (m3 walk, mem row) of degree 1
  //   SEL_MEM_CHAIN: quadratic q * (q - 3) (or q * (q - 2) in P5) of degree 2
  pure val SEL_SIMPLE: int = 0
  pure val SEL_LAGRANGE: int = 1
  pure val SEL_RAW_Q: int = 2
  pure val SEL_MEM_CHAIN: int = 3

  // Fixed column indices in fixedGrid (0..11)
  pure val COL_Q_DEC: int = 0
  pure val COL_Q_REQ: int = 1
  pure val COL_Q_PROV: int = 2
  pure val COL_Q_RNG: int = 3
  pure val COL_MEM: int = 4
  pure val COL_ABS: int = 5
  pure val COL_WALK: int = 6
  pure val COL_CORE_BASE: int = 7
  pure val NUM_FIXED_COLS: int = 12

  // Gate IDs
  // Core gate rows gi in 0..6 have gateId = GATE_CORE_BASE + gi (100..106)
  pure val GATE_CORE_BASE: int = 100
  pure val GATE_RF_CNT0: int = 110
  pure val GATE_MEM_ROW: int = 200
  pure val GATE_MEM_CHAIN: int = 201
  pure val GATE_MEM_FIRST: int = 202
  pure val GATE_MEM_LAST: int = 203
  pure val GATE_HASH_FIRST: int = 300
  pure val GATE_HASH_CHAIN: int = 301
  pure val GATE_HASH_TAIL: int = 302
  pure val GATE_M3_WALK: int = 400
  pure val GATE_M3_FIRST: int = 401
  pure val GATE_M3_LAST: int = 402

  // Lookup IDs
  pure val LK_BUS_0: int = 1
  pure val LK_BUS_1: int = 2
  pure val LK_RANGE: int = 3
  pure val LK_DEC: int = 4

  // Measured Narrow core gate body degrees for slot rows [1, 3, 5, 7, 9, 11, 12]
  // from run 20261110T230612Z-chip4-degs: constraints [23, 8, 6, 6, 6, 2, 2], body degrees [2, 3, 2, 2, 1, 1, 2]
  pure val NARROW_GATE_OFFSETS: List[int] = [1, 3, 5, 7, 9, 11, 12]
  pure val NARROW_BODY_DEGS: List[int] = [2, 3, 2, 2, 1, 1, 2]

  // Descending degree levels for stable sort in `pack` (vsel.rs:89)
  pure val DEG_LEVELS_DESC: List[int] = [5, 4, 3, 2, 1, 0]

  // ---------------------------------------------------------------- Types
  // Exact rational number num / den (den != 0)
  type Rat = {
    num: int,
    den: int
  }

  // Layout configuration chosen by the nondeterministic prover step
  type LayCfg = {
    arch: int,
    s: int,
    wideDeg: int,
    stagDeg0: int,
    stagDeg1: int
  }

  // One assign_fixed / selector enable write in synthesis order (vsel.rs:55-62, keygen.rs:117-140)
  type EnableOp = {
    col: int,
    row: int,
    codeVal: int
  }

  // Selector specification attached to a gate or lookup
  type SelSpec = {
    kind: int,
    col: int,
    codeVal: int,
    nCodes: int,
    selDeg: int
  }

  // Bin used during first-fit decreasing `pack` (vsel.rs:90-105)
  type PackColBin = {
    members: List[int],
    maxDeg: int
  }

  // Enforced gate in the circuit: its selector polynomial, body degree, and plain-selector reference rows
  type GateSpec = {
    gateId: int,
    sel: SelSpec,
    bodyDeg: int,
    refRows: Set[int]
  }

  // Lookup argument in the circuit: its selector, input degree, table degree, and required degree (lookup.rs:39-85)
  type LookupSpec = {
    lookupId: int,
    sel: SelSpec,
    inputDeg: int,
    tableDeg: int,
    reqDeg: int
  }

  // Complete synthesized layout
  type Layout = {
    cfg: LayCfg,
    totalRows: int,
    coreRows: int,
    memStart: int,
    hashStart: int,
    walkStart: int,
    packedCoreCols: int,
    enables: List[EnableOp],
    fixedGrid: List[List[int]],
    gates: List[GateSpec],
    lookups: List[LookupSpec]
  }

  // Concrete advice-level witness for testing honest acceptance vs forged-advice attacks on weakened rows
  type AdviceWitness = {
    lay: Layout,
    tamperedGateId: int,
    tamperedRow: int,
    adaptedToMutant: bool
  }

  // ---------------------------------------------------------------- Exact Polynomial Evaluation
  pure def maxInt(a: int, b: int): int =
    if (a >= b) a else b

  // Lagrange polynomial L_c(q) = prod_{j != c} (q - j) / (c - j) (vsel.rs:66-77)
  // In P3 `lagrange_skips_zero`, the product runs over 1..=n instead of 0..=n (zero not excluded).
  pure def lagrangeRat(mt: str, q: int, c: int, n: int): Rat = {
    val startJ = if (mt == "lagrange_skips_zero") 1 else 0
    range(startJ, n + 1).foldl({ num: 1, den: 1 }, (acc, j) => {
      if (j == c) acc
      else {
        num: acc.num * (q - j),
        den: acc.den * (c - j)
      }
    })
  }

  // Evaluate a gate's selector polynomial at stored fixed value `q`
  pure def evalSelRat(mt: str, sel: SelSpec, q: int): Rat = {
    if (sel.kind == SEL_SIMPLE) {
      { num: q, den: 1 }
    } else if (sel.kind == SEL_LAGRANGE) {
      lagrangeRat(mt, q, sel.codeVal, sel.nCodes)
    } else if (sel.kind == SEL_RAW_Q) {
      { num: q, den: 1 }
    } else {
      // SEL_MEM_CHAIN: q * (q - 3) in mem.rs:163; q * (q - 2) in P5 `chain_on_last`
      val root = if (mt == "chain_on_last") 2 else 3
      { num: q * (q - root), den: 1 }
    }
  }

  // ---------------------------------------------------------------- Replay of `vsel::pack` (vsel.rs:87-119)
  // Stable sort of indices `0 .. degs.length() - 1` by `std::cmp::Reverse(degs[i])`
  pure def sortOrderDesc(degs: List[int]): List[int] = {
    val emptyOrder: List[int] = []
    DEG_LEVELS_DESC.foldl(emptyOrder, (accOrder, dLvl) => {
      range(0, degs.length()).foldl(accOrder, (innerAcc, idx) => {
        if (degs[idx] == dLvl) innerAcc.append(idx) else innerAcc
      })
    })
  }

  // First-fit bin index where `m.len() + 1 + (*md).max(d) <= degBound` (vsel.rs:95-98)
  pure def findFirstFitBin(bins: List[PackColBin], d: int, degBound: int): int = {
    range(0, bins.length()).foldl(-1, (foundIdx, bIdx) => {
      if (foundIdx != -1) foundIdx
      else {
        val b = bins[bIdx]
        val newMax = maxInt(b.maxDeg, d)
        if (b.members.length() + 1 + newMax <= degBound) bIdx else -1
      }
    })
  }

  // Replay `pack` column partitioning (vsel.rs:88-105)
  // In P4 `pack_bound_7`, `pack` admits `members + max degree <= 7` instead of 6.
  pure def packBins(mt: str, degs: List[int]): List[PackColBin] = {
    val degBound = if (mt == "pack_bound_7") 7 else MAX_DEG
    val ord = sortOrderDesc(degs)
    val emptyBins: List[PackColBin] = []
    ord.foldl(emptyBins, (bins, idx) => {
      val d = degs[idx]
      val fitIdx = findFirstFitBin(bins, d, degBound)
      if (fitIdx >= 0) {
        val oldBin = bins[fitIdx]
        val updBin: PackColBin = {
          members: oldBin.members.append(idx),
          maxDeg: maxInt(oldBin.maxDeg, d)
        }
        bins.replaceAt(fitIdx, updBin)
      } else {
        bins.append({ members: [idx], maxDeg: d })
      }
    })
  }

  // Assign `VSel::Code { col, code: k + 1, n }` to each packed gate (vsel.rs:106-118)
  // In P1 `same_code`, `pack` gives two members of a column (`k == 0` and `k == 1` of column 0) the same code `1`.
  pure def packSelectors(mt: str, baseCol: int, degs: List[int]): List[SelSpec] = {
    val bins = packBins(mt, degs)
    val dummySel: SelSpec = { kind: SEL_SIMPLE, col: 0, codeVal: 1, nCodes: 1, selDeg: 1 }
    val emptySelOut: List[SelSpec] = []
    val initOut: List[SelSpec] = range(0, degs.length()).foldl(emptySelOut, (acc, _i) => acc.append(dummySel))
    range(0, bins.length()).foldl(initOut, (outList, bIdx) => {
      val b = bins[bIdx]
      val colId = baseCol + bIdx
      val n = b.members.length()
      range(0, n).foldl(outList, (innerOut, k) => {
        val memberIdx = b.members[k]
        val assignedCode =
          if (mt == "same_code" and bIdx == 0 and k == 1) 1
          else k + 1
        val spec: SelSpec = {
          kind: SEL_LAGRANGE,
          col: colId,
          codeVal: assignedCode,
          nCodes: n,
          selDeg: n
        }
        innerOut.replaceAt(memberIdx, spec)
      })
    })
  }

  // ---------------------------------------------------------------- Architecture Helpers (rfcore.rs:116-222)
  pure def archHeight(arch: int): int =
    if (arch == ARCH_NARROW) 13
    else if (arch == ARCH_WIDE) 1
    else 2

  pure def archGateOffsets(arch: int): List[int] =
    if (arch == ARCH_NARROW) NARROW_GATE_OFFSETS
    else if (arch == ARCH_WIDE) [0]
    else [1, 2]

  pure def archBodyDegs(cfg: LayCfg): List[int] =
    if (cfg.arch == ARCH_NARROW) NARROW_BODY_DEGS
    else if (cfg.arch == ARCH_WIDE) [cfg.wideDeg]
    else [cfg.stagDeg0, cfg.stagDeg1]

  // Canonical `Cell::Cnt` row in slot 0 (`lay.place[lay.canon(Cell::Cnt)].1`, rfcore.rs:1199)
  pure def archCanonCntRow(arch: int): int =
    if (arch == ARCH_NARROW) 0
    else if (arch == ARCH_WIDE) 0
    else 1

  // Synthesized `cnt_row` in rfcore.rs:1199-1200:
  // In P2 `cnt_collides`, `rf count0` is enabled on a row that also carries a packed core gate of the same column
  // (row 12 on Narrow, which is slot 0's `gi = 6` gate row and also an `f_row` holding `Cell::Cnt`;
  //  row 2 on Staggered, which is slot 0's `gi = 1` gate row).
  pure def archSynthCntRow(mt: str, arch: int): int =
    if (mt == "cnt_collides") {
      if (arch == ARCH_NARROW) 12
      else if (arch == ARCH_STAGGERED) 2
      else 0
    } else {
      archCanonCntRow(arch)
    }

  // Reference enabled rows for a core gate at offset `gateOff` across slots `0 .. s - 1`
  // (Note: slot `s` has no core gates enabled because `if i < s` at rfcore.rs:1147)
  pure def coreGateRefRows(s: int, h: int, gateOff: int): Set[int] = {
    val emptyRows: Set[int] = Set()
    range(0, s).foldl(emptyRows, (acc, i) => acc.union(Set(i * h + gateOff)))
  }

  // ---------------------------------------------------------------- Synthesizer & Layout Builder
  pure def buildLayout(mt: str, cfg: LayCfg): Layout = {
    val h = archHeight(cfg.arch)
    val gateOffsets = archGateOffsets(cfg.arch)
    val bodyDegs = archBodyDegs(cfg)
    val nGates = gateOffsets.length()
    // DEC host gate index `dgi`: on all 3 architectures `dgi = 0` (`dec_off = gateOffsets[0]`, rfcore.rs:882-883)
    val dgi = 0
    val decOff = gateOffsets[dgi]

    val coreRows = h * (cfg.s + 1)
    val memStart = coreRows
    val hashStart = memStart + 8
    val walkStart = hashStart + 8
    val totalRows = walkStart + 6

    // 1. Configure core gate selectors and `rf count0` (rfcore.rs:886-912)
    // In P8 `dec_packed`, `dgi` is also packed by `pack` instead of using `VSel::Simple(q_dec)`.
    val emptyIdx: List[int] = []
    val packedGateIndices = range(0, nGates).foldl(emptyIdx, (acc, gi) => {
      if (gi != dgi or mt == "dec_packed") acc.append(gi) else acc
    })
    val emptyDegs: List[int] = []
    val packInputDegs = packedGateIndices.foldl(emptyDegs, (acc, gi) => {
      acc.append(bodyDegs[gi])
    }).append(1)

    val packedBins = packBins(mt, packInputDegs)
    val packedSels = packSelectors(mt, COL_CORE_BASE, packInputDegs)

    val simpleQDec: SelSpec = {
      kind: SEL_SIMPLE,
      col: COL_Q_DEC,
      codeVal: 1,
      nCodes: 1,
      selDeg: 1
    }

    val emptySels: List[SelSpec] = []
    val coreGateSels = range(0, nGates).foldl(emptySels, (acc, gi) => {
      if (gi == dgi and mt != "dec_packed") {
        acc.append(simpleQDec)
      } else {
        val k = range(0, packedGateIndices.length()).foldl(0, (found, idx) => {
          if (packedGateIndices[idx] == gi) idx else found
        })
        acc.append(packedSels[k])
      }
    })
    val cnt0Sel = packedSels[packedGateIndices.length()]
    val decLookupSel = if (mt == "dec_packed") coreGateSels[dgi] else simpleQDec

    // 2. Build ordered list of synthesizer `enable` calls (rfcore.rs:1147-1153, 1199-1200, mem.rs, poseidon.rs, m3.rs)
    val emptyEnables: List[EnableOp] = []

    // 2a. Core slot loop `for i in 0..s` (rfcore.rs:1147-1153)
    val afterCoreSlots = range(0, cfg.s).foldl(emptyEnables, (acc, i) => {
      val afterGates = range(0, nGates).foldl(acc, (innerAcc, gi) => {
        val sel = coreGateSels[gi]
        innerAcc.append({ col: sel.col, row: i * h + gateOffsets[gi], codeVal: sel.codeVal })
      })
      // Line 1153: `cfg.q_dec.enable(&mut r, i * h + cfg.dec_off)?;`
      afterGates.append({ col: decLookupSel.col, row: i * h + decOff, codeVal: decLookupSel.codeVal })
    })

    // 2b. Core `rf count0` enable after slot loop (rfcore.rs:1199-1200)
    val synthCntRow = archSynthCntRow(mt, cfg.arch)
    val afterCoreCnt0 = afterCoreSlots.append({
      col: cnt0Sel.col,
      row: synthCntRow,
      codeVal: cnt0Sel.codeVal
    })

    // 2c. MEM segment loop `for i in 0..SEG` (mem.rs:207-216): code 1 on row 0, code 2 on rows 1..6, code 3 on row 7
    val afterMem = range(0, 8).foldl(afterCoreCnt0, (acc, i) => {
      val codeIdx = if (i == 0) 0 else if (i + 1 == 8) 2 else 1
      acc.append({ col: COL_MEM, row: memStart + i, codeVal: codeIdx + 1 })
    })

    // 2d. HASH absorb enables (poseidon.rs:307-315):
    //   hashStart + 0: 2-input provider sponge chunk 0 -> q_first (code 1)
    //   hashStart + 2: 5-input sponge chunk 0 -> q_first (code 1)
    //   hashStart + 4: 5-input sponge chunk 1 -> q_chain (code 2); in P9 `abs_two_codes`, followed by q_first (code 1)
    //   hashStart + 6: 5-input sponge chunk 2 -> q_tail (code 3)
    //   (rows +1, +3, +5, +7 are round/output rows with fixed value 0 in COL_ABS)
    val absBase = afterMem
      .append({ col: COL_ABS, row: hashStart + 0, codeVal: 1 })
      .append({ col: COL_ABS, row: hashStart + 2, codeVal: 1 })
      .append({ col: COL_ABS, row: hashStart + 4, codeVal: 2 })
    val absAfterP9 =
      if (mt == "abs_two_codes")
        absBase.append({ col: COL_ABS, row: hashStart + 4, codeVal: 1 })
      else
        absBase
    val afterHash = absAfterP9.append({ col: COL_ABS, row: hashStart + 6, codeVal: 3 })

    // 2e. M3 walker loop `for walk in 0..2 { for l in 0..TOP }` (m3.rs:330-382)
    //   Representative levels l = 0 (first), l = 1 (middle), l = 2 (TOP - 1, last):
    //   Normal mapping (m3.rs:375-381):
    //     walk == 0 && l == 0     -> index 1 (code 2, L_2 = m3 walk first)
    //     walk == 0 && l == TOP-1 -> index 2 (code 3, L_3 = m3 walk last)
    //     otherwise               -> index 0 (code 1, every level)
    //   In P7 `walk_code_shift`, the indices are shifted to `0 = first, 1 = middle/other, 2 = last`
    //   (or 1-based shifted mod 3), which compiles and runs in Rust, putting code 1 on (walk=0, l=0)
    //   and code 2 (m3 walk first!) on all middle and new-walk levels.
    val enables = range(0, 2).foldl(afterHash, (accW, walk) => {
      range(0, 3).foldl(accW, (accL, l) => {
        val r = walkStart + walk * 3 + l
        val codeIdx =
          if (mt == "walk_code_shift") {
            if (walk == 0 and l == 0) 0
            else if (walk == 0 and l == 2) 2
            else 1
          } else {
            if (walk == 0 and l == 0) 1
            else if (walk == 0 and l == 2) 2
            else 0
          }
        accL.append({ col: COL_WALK, row: r, codeVal: codeIdx + 1 })
      })
    })

    // 3. Apply `enables` with last-write-wins to `fixedGrid` (keygen.rs:134-138, unassigned cells 0)
    val emptyCol: List[int] = []
    val zeroCol: List[int] = range(0, totalRows).foldl(emptyCol, (acc, _r) => acc.append(0))
    val emptyGrid: List[List[int]] = []
    val zeroGrid: List[List[int]] = range(0, NUM_FIXED_COLS).foldl(emptyGrid, (acc, _c) => acc.append(zeroCol))
    val fixedGrid = enables.foldl(zeroGrid, (grid, op) => {
      val colVec = grid[op.col]
      grid.replaceAt(op.col, colVec.replaceAt(op.row, op.codeVal))
    })

    // 4. Construct all enforced gates and their plain-selector reference row sets
    val emptyGates: List[GateSpec] = []
    val coreGates = range(0, nGates).foldl(emptyGates, (acc, gi) => {
      acc.append({
        gateId: GATE_CORE_BASE + gi,
        sel: coreGateSels[gi],
        bodyDeg: bodyDegs[gi],
        refRows: coreGateRefRows(cfg.s, h, gateOffsets[gi])
      })
    })

    val cnt0Gate: GateSpec = {
      gateId: GATE_RF_CNT0,
      sel: cnt0Sel,
      bodyDeg: 1,
      refRows: Set(archCanonCntRow(cfg.arch))
    }

    // MEM segment gates (mem.rs:145-184)
    val memAllRows = Set(
      memStart + 0, memStart + 1, memStart + 2, memStart + 3,
      memStart + 4, memStart + 5, memStart + 6, memStart + 7
    )
    val memChainRows = Set(
      memStart + 0, memStart + 1, memStart + 2, memStart + 3,
      memStart + 4, memStart + 5, memStart + 6
    )
    val memRowGate: GateSpec = {
      gateId: GATE_MEM_ROW,
      sel: { kind: SEL_RAW_Q, col: COL_MEM, codeVal: 1, nCodes: 3, selDeg: 1 },
      bodyDeg: 2,
      refRows: memAllRows
    }
    val memChainGate: GateSpec = {
      gateId: GATE_MEM_CHAIN,
      sel: { kind: SEL_MEM_CHAIN, col: COL_MEM, codeVal: 1, nCodes: 3, selDeg: 2 },
      bodyDeg: 3,
      refRows: memChainRows
    }
    val memFirstGate: GateSpec = {
      gateId: GATE_MEM_FIRST,
      sel: { kind: SEL_LAGRANGE, col: COL_MEM, codeVal: 1, nCodes: 3, selDeg: 3 },
      bodyDeg: 2,
      refRows: Set(memStart + 0)
    }
    val memLastGate: GateSpec = {
      gateId: GATE_MEM_LAST,
      sel: { kind: SEL_LAGRANGE, col: COL_MEM, codeVal: 3, nCodes: 3, selDeg: 3 },
      bodyDeg: 2,
      refRows: Set(memStart + 7)
    }

    // HASH absorb gates (poseidon.rs:215-253)
    // Note: `hash absorb first` includes `L_1(q) * q_prov * (chip - HASH)` of body degree 2 (total degree 5).
    val hashFirstGate: GateSpec = {
      gateId: GATE_HASH_FIRST,
      sel: { kind: SEL_LAGRANGE, col: COL_ABS, codeVal: 1, nCodes: 3, selDeg: 3 },
      bodyDeg: 2,
      refRows: Set(hashStart + 0, hashStart + 2)
    }
    val hashChainGate: GateSpec = {
      gateId: GATE_HASH_CHAIN,
      sel: { kind: SEL_LAGRANGE, col: COL_ABS, codeVal: 2, nCodes: 3, selDeg: 3 },
      bodyDeg: 1,
      refRows: Set(hashStart + 4)
    }
    val hashTailGate: GateSpec = {
      gateId: GATE_HASH_TAIL,
      sel: { kind: SEL_LAGRANGE, col: COL_ABS, codeVal: 3, nCodes: 3, selDeg: 3 },
      bodyDeg: 1,
      refRows: Set(hashStart + 6)
    }

    // M3 walker gates (m3.rs:113-139)
    // In P6 `walk_lagrange1`, `m3 walk` uses `L_1(q)` (degree 3) instead of raw `q` (degree 1).
    val walkAllRows = Set(
      walkStart + 0, walkStart + 1, walkStart + 2,
      walkStart + 3, walkStart + 4, walkStart + 5
    )
    val m3WalkSel: SelSpec =
      if (mt == "walk_lagrange1")
        { kind: SEL_LAGRANGE, col: COL_WALK, codeVal: 1, nCodes: 3, selDeg: 3 }
      else
        { kind: SEL_RAW_Q, col: COL_WALK, codeVal: 1, nCodes: 3, selDeg: 1 }
    val m3WalkGate: GateSpec = {
      gateId: GATE_M3_WALK,
      sel: m3WalkSel,
      bodyDeg: 2,
      refRows: walkAllRows
    }
    val m3FirstGate: GateSpec = {
      gateId: GATE_M3_FIRST,
      sel: { kind: SEL_LAGRANGE, col: COL_WALK, codeVal: 2, nCodes: 3, selDeg: 3 },
      bodyDeg: 1,
      refRows: Set(walkStart + 0)
    }
    val m3LastGate: GateSpec = {
      gateId: GATE_M3_LAST,
      sel: { kind: SEL_LAGRANGE, col: COL_WALK, codeVal: 3, nCodes: 3, selDeg: 3 },
      bodyDeg: 1,
      refRows: Set(walkStart + 2)
    }

    val allGates = coreGates
      .append(cnt0Gate)
      .append(memRowGate)
      .append(memChainGate)
      .append(memFirstGate)
      .append(memLastGate)
      .append(hashFirstGate)
      .append(hashChainGate)
      .append(hashTailGate)
      .append(m3WalkGate)
      .append(m3FirstGate)
      .append(m3LastGate)

    // 5. Lookups (bus.rs:46-62, range.rs:92-102, dec.rs:19-32, rfcore.rs:941-954, lookup.rs:39-85)
    // Required degree of a lookup is `max(4, 2 + input_degree + table_degree)`
    val busReqSel: SelSpec = { kind: SEL_SIMPLE, col: COL_Q_REQ, codeVal: 1, nCodes: 1, selDeg: 1 }
    val rngSel: SelSpec = { kind: SEL_SIMPLE, col: COL_Q_RNG, codeVal: 1, nCodes: 1, selDeg: 1 }
    val decInputDeg = decLookupSel.selDeg + 2
    val decTableDeg = 1
    val decReqDeg = maxInt(4, 2 + decInputDeg + decTableDeg)

    val lookups: List[LookupSpec] = [
      { lookupId: LK_BUS_0, sel: busReqSel, inputDeg: 2, tableDeg: 2, reqDeg: 6 },
      { lookupId: LK_BUS_1, sel: busReqSel, inputDeg: 2, tableDeg: 2, reqDeg: 6 },
      { lookupId: LK_RANGE, sel: rngSel, inputDeg: 2, tableDeg: 1, reqDeg: 5 },
      { lookupId: LK_DEC, sel: decLookupSel, inputDeg: decInputDeg, tableDeg: decTableDeg, reqDeg: decReqDeg }
    ]

    {
      cfg: cfg,
      totalRows: totalRows,
      coreRows: coreRows,
      memStart: memStart,
      hashStart: hashStart,
      walkStart: walkStart,
      packedCoreCols: packedBins.length(),
      enables: enables,
      fixedGrid: fixedGrid,
      gates: allGates,
      lookups: lookups
    }
  }

  // ---------------------------------------------------------------- Acceptance & Refinement (`invRefine`)
  // Well-formed synthesized layout: every body degree passed to `pack` is < MAX_DEG (vsel.rs:94)
  // and every enable operation writes to a valid `(col, row)` within the region (keygen.rs:130-138).
  pure def acceptsWithMutant(mt: str, lay: Layout): bool = {
    val bodyDegsOk = archBodyDegs(lay.cfg).indices().forall(i => archBodyDegs(lay.cfg)[i] < MAX_DEG)
    val enablesInBounds = lay.enables.indices().forall(i => {
      val op = lay.enables[i]
      op.col >= 0 and op.col < NUM_FIXED_COLS and op.row >= 0 and op.row < lay.totalRows
    })
    val expected = buildLayout(mt, lay.cfg)
    bodyDegsOk and enablesInBounds and (lay == expected)
  }

  pure def accepts(lay: Layout): bool =
    acceptsWithMutant(MUTANT, lay)

  // Set of gate IDs whose selector polynomial evaluates to a nonzero rational on row `r`
  pure def activeGatesOnRow(mt: str, lay: Layout, r: int): Set[int] = {
    val emptySet: Set[int] = Set()
    lay.gates.foldl(emptySet, (acc, g) => {
      val q = lay.fixedGrid[g.sel.col][r]
      val v = evalSelRat(mt, g.sel, q)
      if (v.num != 0 and v.den != 0) acc.union(Set(g.gateId)) else acc
    })
  }

  // Set of gate IDs enabled on row `r` by the plain-selector reference circuit
  pure def refGatesOnRow(lay: Layout, r: int): Set[int] = {
    val emptySet: Set[int] = Set()
    lay.gates.foldl(emptySet, (acc, g) => {
      if (g.refRows.contains(r)) acc.union(Set(g.gateId)) else acc
    })
  }

  // Refinement clause 1: on every row of the layout, active gates == plain-selector reference gates
  pure def activeGatesMatchRef(mt: str, lay: Layout): bool =
    range(0, lay.totalRows).foldl(true, (ok, r) =>
      ok and (activeGatesOnRow(mt, lay, r) == refGatesOnRow(lay, r))
    )

  // Refinement clause 2: every enforced gate's total degree (selector degree + body degree) <= 6
  pure def gateDegreesOk(lay: Layout): bool =
    lay.gates.indices().forall(i => {
      val g = lay.gates[i]
      (g.sel.selDeg + g.bodyDeg) <= MAX_DEG
    })

  // Maximum total gate degree in the layout
  pure def maxGateDegree(lay: Layout): int =
    lay.gates.foldl(0, (mx, g) => maxInt(mx, g.sel.selDeg + g.bodyDeg))

  // Refinement clause 3: every lookup's required degree <= 6
  pure def lookupDegreesOk(lay: Layout): bool =
    lay.lookups.indices().forall(i => lay.lookups[i].reqDeg <= MAX_DEG)

  // Maximum lookup required degree in the layout
  pure def maxLookupDegree(lay: Layout): int =
    lay.lookups.foldl(0, (mx, lk) => maxInt(mx, lk.reqDeg))

  // Complete refinement predicate `invRefine`:
  //   1. For every row, active gates == plain-selector reference gates;
  //   2. Every enforced gate's total degree <= 6;
  //   3. Every lookup's degree <= 6.
  pure def refinesWith(mt: str, lay: Layout): bool = and {
    activeGatesMatchRef(mt, lay),
    gateDegreesOk(lay),
    lookupDegreesOk(lay)
  }

  pure def refines(lay: Layout): bool =
    refinesWith(MUTANT, lay)

  // ---------------------------------------------------------------- Advice-Level Witness Semantics
  // Models how an honest or tampered advice witness interacts with the layout's active gates:
  // - On an honest advice witness (`tamperedGateId == -1, adaptedToMutant == false`),
  //   gate `g`'s body polynomial is 0 on `g.refRows` (and for `rf count0`, also on slot 0's `f_row` 12
  //   where `Cell::Cnt == 0` in Narrow), and nonzero (1) on rows where `g` is not intended to hold.
  // - On a forged advice witness (`tamperedGateId == g.gateId, tamperedRow == r`),
  //   the prover corrupts the advice cells checked by `g` on row `r` (so `g`'s body is nonzero on `r`).
  //   If `g`'s selector was erased/zeroed on row `r` by a mutant (e.g. P2, P5, P6, P7, P9),
  //   `sel(q) * body == 0` still holds and the forged advice witness is ACCEPTED!
  pure def gateBodyZeroOnAdvice(mt: str, g: GateSpec, r: int, aw: AdviceWitness): bool = {
    if (aw.adaptedToMutant) {
      // Prover computes advice to satisfy the active gates of the mutated layout on row r
      activeGatesOnRow(mt, aw.lay, r).contains(g.gateId)
    } else if (aw.tamperedGateId == g.gateId and aw.tamperedRow == r) {
      false
    } else if (g.gateId == GATE_RF_CNT0 and aw.lay.cfg.arch == ARCH_NARROW and r == 12) {
      // In Arch::Narrow, row 12 of slot 0 is an `f_row` holding a copy-constrained replica of Cnt = 0
      true
    } else {
      g.refRows.contains(r)
    }
  }

  // Advice witness is accepted by the circuit iff for every row `r` and every gate `g`,
  // either `g`'s selector polynomial is 0 on `r` OR `g`'s body polynomial is 0 on `r`,
  // and the key degree (gates and lookups) is at most MAX_DEG.
  pure def adviceAcceptedWith(mt: str, aw: AdviceWitness): bool = and {
    acceptsWithMutant(mt, aw.lay),
    gateDegreesOk(aw.lay),
    lookupDegreesOk(aw.lay),
    range(0, aw.lay.totalRows).foldl(true, (okR, r) =>
      aw.lay.gates.foldl(okR, (okG, g) => {
        val selNonZero = (evalSelRat(mt, g.sel, aw.lay.fixedGrid[g.sel.col][r]).num != 0)
        okG and (selNonZero implies gateBodyZeroOnAdvice(mt, g, r, aw))
      })
    )
  }

  // Advice witness refines the plain-selector reference iff every gate's body holds on all of its `refRows`
  pure def adviceRefinesWith(mt: str, aw: AdviceWitness): bool =
    range(0, aw.lay.totalRows).foldl(true, (okR, r) =>
      aw.lay.gates.foldl(okR, (okG, g) =>
        okG and (g.refRows.contains(r) implies gateBodyZeroOnAdvice(mt, g, r, aw))
      )
    )

  pure def honestAdviceWitness(mt: str, cfg: LayCfg): AdviceWitness = {
    lay: buildLayout(mt, cfg),
    tamperedGateId: -1,
    tamperedRow: -1,
    adaptedToMutant: false
  }

  pure def forgedAdviceWitness(mt: str, cfg: LayCfg, gateId: int, row: int): AdviceWitness = {
    lay: buildLayout(mt, cfg),
    tamperedGateId: gateId,
    tamperedRow: row,
    adaptedToMutant: false
  }

  // ---------------------------------------------------------------- Canonical Configs & Named Witnesses
  pure val DEFAULT_CFG: LayCfg = {
    arch: ARCH_NARROW,
    s: 1,
    wideDeg: 3,
    stagDeg0: 2,
    stagDeg1: 3
  }

  pure val NARROW_S2_CFG: LayCfg = { ...DEFAULT_CFG, s: 2 }
  pure val NARROW_S3_CFG: LayCfg = { ...DEFAULT_CFG, s: 3 }
  pure val WIDE_DEFAULT_CFG: LayCfg = { ...DEFAULT_CFG, arch: ARCH_WIDE, wideDeg: 3 }
  pure val WIDE_DEG5_CFG: LayCfg = { ...DEFAULT_CFG, arch: ARCH_WIDE, wideDeg: 5 }
  pure val STAG_SHARED_CFG: LayCfg = { ...DEFAULT_CFG, arch: ARCH_STAGGERED, stagDeg0: 2, stagDeg1: 3 }
  pure val STAG_SPLIT_CFG: LayCfg = { ...DEFAULT_CFG, arch: ARCH_STAGGERED, stagDeg0: 2, stagDeg1: 5 }

  // Sound layout witness (`narrow, s = 1`)
  pure val hw_narrow_s1: Layout = buildLayout("none", DEFAULT_CFG)
  pure val hw_narrow_s3: Layout = buildLayout("none", NARROW_S3_CFG)
  pure val hw_wide_s1: Layout = buildLayout("none", WIDE_DEFAULT_CFG)
  pure val hw_stag_shared: Layout = buildLayout("none", STAG_SHARED_CFG)
  pure val hw_stag_split: Layout = buildLayout("none", STAG_SPLIT_CFG)

  // Named counterexample witnesses for P1..P9
  pure val cex_P1_same_code: Layout = buildLayout("same_code", DEFAULT_CFG)
  pure val cex_P2_cnt_collides: Layout = buildLayout("cnt_collides", DEFAULT_CFG)
  pure val cex_P3_lagrange_skips_zero: Layout = buildLayout("lagrange_skips_zero", DEFAULT_CFG)
  pure val cex_P4_pack_bound_7: Layout = buildLayout("pack_bound_7", DEFAULT_CFG)
  pure val cex_P5_chain_on_last: Layout = buildLayout("chain_on_last", DEFAULT_CFG)
  pure val cex_P6_walk_lagrange1: Layout = buildLayout("walk_lagrange1", DEFAULT_CFG)
  pure val cex_P7_walk_code_shift: Layout = buildLayout("walk_code_shift", DEFAULT_CFG)
  pure val cex_P8_dec_packed: Layout = buildLayout("dec_packed", DEFAULT_CFG)
  pure val cex_P9_abs_two_codes: Layout = buildLayout("abs_two_codes", DEFAULT_CFG)

  // ---------------------------------------------------------------- State Machine (Nondeterministic Layout Prover)
  pure val ARCH_DOM: Set[int] = Set(ARCH_NARROW, ARCH_WIDE, ARCH_STAGGERED)
  pure val S_DOM: Set[int] = Set(1, 2, 3)
  pure val DEG_DOM: Set[int] = Set(1, 2, 3, 4, 5)
  pure val STEP_MODES: Set[int] = Set(0, 1)

  var w: Layout
  var acc: bool
  var isTampered: bool

  action init = {
    val cand = buildLayout(MUTANT, DEFAULT_CFG)
    val soundBase = buildLayout("none", DEFAULT_CFG)
    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = (cand != soundBase)
    }
  }

  action step = {
    nondet stepMode = STEP_MODES.oneOf()
    nondet archChoice = ARCH_DOM.oneOf()
    nondet sChoice = S_DOM.oneOf()
    nondet wDeg = DEG_DOM.oneOf()
    nondet sDeg0 = DEG_DOM.oneOf()
    nondet sDeg1 = DEG_DOM.oneOf()
    val chosenArch =
      if (MUTANT != "none" and stepMode == 0) ARCH_NARROW else archChoice
    val cfg: LayCfg = {
      arch: chosenArch,
      s: sChoice,
      wideDeg: wDeg,
      stagDeg0: sDeg0,
      stagDeg1: sDeg1
    }
    val cand = buildLayout(MUTANT, cfg)
    val soundLay = buildLayout("none", cfg)
    all {
      w' = cand,
      acc' = accepts(cand),
      isTampered' = (cand != soundLay)
    }
  }

  // ---------------------------------------------------------------- Invariants
  // Main refinement invariant: accepted layout implies active gates match plain-selector reference on every row,
  // every enforced gate's total degree <= 6, and every lookup's degree <= 6.
  val invRefine: bool =
    acc implies refines(w)

  // Non-vacuity reachability invariants (negated: MUST be violated with exit 1 in `packsel_sound`):
  // 1. Reaches an honest multi-slot Narrow layout (S = 3) with 2 packed core columns
  val reachNarrowMultiSlot: bool =
    not(acc and not(isTampered) and w.cfg.arch == ARCH_NARROW and w.cfg.s == 3 and w.packedCoreCols == 2)

  // 2. Reaches an honest Wide layout (1 packed core column for rf count0; dgi = 0 uses q_dec)
  val reachWideLayout: bool =
    not(acc and not(isTampered) and w.cfg.arch == ARCH_WIDE and w.packedCoreCols == 1)

  // 3. Reaches an honest Staggered layout where gi = 1 and rf count0 share 1 code column (stagDeg1 <= 4)
  val reachStaggeredSharedCol: bool =
    not(acc and not(isTampered) and w.cfg.arch == ARCH_STAGGERED and w.cfg.stagDeg1 <= 4 and w.packedCoreCols == 1)

  // 4. Reaches an honest Staggered layout where gi = 1 (deg 5) and rf count0 split into 2 code columns (stagDeg1 == 5)
  val reachStaggeredSplitCols: bool =
    not(acc and not(isTampered) and w.cfg.arch == ARCH_STAGGERED and w.cfg.stagDeg1 == 5 and w.packedCoreCols == 2)

  // 5. Accepted layout that fails refinement (must NOT be violated in `packsel_sound`: exit 0; violated in P1..P9: exit 1)
  val reachAcceptedBadTamper: bool =
    not(acc and not(refines(w)))
}

// Sound module instance
module packsel_sound { import packsel(MUTANT = "none").* }

// Required mutant module instances P1..P9 (each MUST violate `invRefine` with exit 1)
module packsel_same_code { import packsel(MUTANT = "same_code").* }
module packsel_cnt_collides { import packsel(MUTANT = "cnt_collides").* }
module packsel_lagrange_skips_zero { import packsel(MUTANT = "lagrange_skips_zero").* }
module packsel_pack_bound_7 { import packsel(MUTANT = "pack_bound_7").* }
module packsel_chain_on_last { import packsel(MUTANT = "chain_on_last").* }
module packsel_walk_lagrange1 { import packsel(MUTANT = "walk_lagrange1").* }
module packsel_walk_code_shift { import packsel(MUTANT = "walk_code_shift").* }
module packsel_dec_packed { import packsel(MUTANT = "dec_packed").* }
module packsel_abs_two_codes { import packsel(MUTANT = "abs_two_codes").* }
