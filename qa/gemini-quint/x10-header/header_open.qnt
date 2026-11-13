// Soundness and completeness model of HeaderOpenAir (Sherlock experiment X10, FP-5).
// Source: lane-src/x_exp_node.rs (HeaderOpenAir, header_trace, header_words, sdk_digest),
// lane-src/padding_free_sponge.rs (PaddingFreeSponge<P, 16, 8, 8>), and
// research/sherlock-final-design-20261111/ARCHITECTURE.md §1.11 FP-5 & BOUNDARY COMMITMENT.

module header_open_base {
  const DROP_CAP_CHAIN: bool       // M1: drop capacity chaining on transition 0 -> 1
  const DROP_PARTIAL_KEEP: bool    // M2: drop row-5 overwrite keep on inputs[2..8]
  const DROP_DOMAIN: bool          // M3: drop inputs[0] == SHK_SD2 when s[0]
  const OUTPUT_ON_ROW7: bool       // M4: check public values on row 7 instead of row 5
  const DROP_SEL_ONEHOT: bool      // M5: drop sum(s[r]) == 1 constraint
  const DROP_FIRST_CAP_ZERO: bool  // M6: drop inputs[8..16] == 0 on row 0

  pure val WIDTH = 16
  pure val RATE = 8
  pure val DIG = 8
  pure val HW = 42
  pure val NPERM = 6
  pure val NROW = 8
  pure val PARTIAL_LEN = 2         // HW - (NPERM - 1) * RATE = 42 - 40 = 2
  pure val SHK_SD2 = 29            // FP-4 BabyBear tag code for SHK_SD2

  type Row = {
    inputs: List[int],
    post: List[int],
    sel: List[int],
  }

  type AirState = {
    tr: List[Row],
    pv: List[int],
    accepted: bool,
    zeroPayload: bool,
  }

  var st: AirState

  // ---- Opaque injective permutation P on 16-word states.
  // Algebraic injectivity proof on Z^16:
  // If perm(a) == perm(b), let d_k = a[k] - b[k] and S = sum_{j=0..15} (j + 1) * d_j.
  // Then S + (k + 1) * d_k == 0 for all k in 0..15.
  // Summing over k = 0..15 gives S = -16 * S, so 17 * S == 0 => S == 0 => d_k == 0 for all k.
  pure def stateSum(vec: List[int]): int =
    range(0, WIDTH).foldl(0, (acc, j) => acc + (j + 1) * vec[j])

  pure def perm(vec: List[int]): List[int] = {
    val tot = stateSum(vec)
    val emptyInts: List[int] = []
    range(0, WIDTH).foldl(emptyInts, (acc, k) => acc.append(tot + (k + 1) * vec[k] + (k + 1)))
  }

  pure def zeros(len: int): List[int] = {
    val emptyInts: List[int] = []
    range(0, len).foldl(emptyInts, (acc, _) => acc.append(0))
  }

  pure def overwritePrefix(dst: List[int], src: List[int], count: int): List[int] = {
    val emptyInts: List[int] = []
    range(0, WIDTH).foldl(emptyInts, (acc, i) => acc.append(if (i < count) src[i] else dst[i]))
  }

  pure def replaceElem(xs: List[int], idx: int, newElem: int): List[int] = {
    val emptyInts: List[int] = []
    range(0, xs.length()).foldl(emptyInts, (acc, i) => acc.append(if (i == idx) newElem else xs[i]))
  }

  pure def replaceRow(rows: List[Row], rIdx: int, newRow: Row): List[Row] = {
    val emptyRows: List[Row] = []
    range(0, rows.length()).foldl(emptyRows, (acc, i) => acc.append(if (i == rIdx) newRow else rows[i]))
  }

  pure def oneHotRow(idx: int): List[int] = {
    val emptyInts: List[int] = []
    range(0, NROW).foldl(emptyInts, (acc, c) => acc.append(if (c == idx) 1 else 0))
  }

  // ---- Reference sponge: PaddingFreeSponge(P, rate 8, out 8) in overwrite mode over 42 words.
  pure def sponge(words: List[int]): List[int] = {
    val initSt = zeros(WIDTH)
    val afterFull = range(0, NPERM - 1).foldl(initSt, (cur, blk) => {
      val chunk = words.slice(blk * RATE, (blk + 1) * RATE)
      perm(overwritePrefix(cur, chunk, RATE))
    })
    val lastChunk = words.slice((NPERM - 1) * RATE, HW)
    val finalSt = perm(overwritePrefix(afterFull, lastChunk, PARTIAL_LEN))
    finalSt.slice(0, DIG)
  }

  // ---- Header word generator:
  // Word 0: SHK_SD2, Word 1: outcome_tag, Words 2..10: outcome_payload,
  // Words 10..18: input_d, Words 18..26: deps_d, Words 26..34: view_root, Words 34..42: body_d.
  pure def mkHeaderWords(tagWord: int, zeroPay: bool, allZeroDigest: bool): List[int] = {
    val emptyInts: List[int] = []
    range(0, HW).foldl(emptyInts, (acc, i) => {
      val w =
        if (i == 0) SHK_SD2
        else if (allZeroDigest) 0
        else if (i == 1) tagWord
        else if (zeroPay and i >= 2 and i < 2 + DIG) 0
        else (i + tagWord + 1)
      acc.append(w)
    })
  }

  pure def padInputRow(seedVal: int): List[int] = {
    val emptyInts: List[int] = []
    if (seedVal == 0) zeros(WIDTH)
    else range(0, WIDTH).foldl(emptyInts, (acc, k) => acc.append(seedVal + k))
  }

  // ---- Honest trace builder for 42 words with arbitrary padding rows 6 and 7.
  pure def honestTrace(words: List[int], pad6: int, pad7: int): List[Row] = {
    val initSt = zeros(WIDTH)
    val emptyRows: List[Row] = []
    val usedRows = range(0, NPERM).foldl({ rows: emptyRows, curSt: initSt }, (acc, r) => {
      val blkLen = if (r < NPERM - 1) RATE else PARTIAL_LEN
      val chunk = words.slice(r * RATE, r * RATE + blkLen)
      val inSt = overwritePrefix(acc.curSt, chunk, blkLen)
      val outSt = perm(inSt)
      val rowObj = { inputs: inSt, post: outSt, sel: oneHotRow(r) }
      { rows: acc.rows.append(rowObj), curSt: outSt }
    }).rows
    val p6In = padInputRow(pad6)
    val p7In = padInputRow(pad7)
    val row6 = { inputs: p6In, post: perm(p6In), sel: oneHotRow(6) }
    val row7 = { inputs: p7In, post: perm(p7In), sel: oneHotRow(7) }
    usedRows.append(row6).append(row7)
  }

  // ---- Recompute rows from startRow..5 honestly after an input or post cell tamper.
  pure def recomputeRow(rows: List[Row], r: int, startRow: int): List[Row] =
    if (r < startRow or r >= NPERM) rows
    else if (r == startRow) {
      val updPost = perm(rows[r].inputs)
      replaceRow(rows, r, { ...rows[r], post: updPost })
    } else if (r < NPERM - 1) {
      val prevPost = rows[r - 1].post
      val updIn = rows[r].inputs.slice(0, RATE).concat(prevPost.slice(RATE, WIDTH))
      val updPost = perm(updIn)
      replaceRow(rows, r, { ...rows[r], inputs: updIn, post: updPost })
    } else {
      val prevPost = rows[r - 1].post
      val updIn = rows[r].inputs.slice(0, PARTIAL_LEN).concat(prevPost.slice(PARTIAL_LEN, WIDTH))
      val updPost = perm(updIn)
      replaceRow(rows, r, { ...rows[r], inputs: updIn, post: updPost })
    }

  pure def recomputeFrom(rows: List[Row], startRow: int): List[Row] =
    range(0, NPERM).foldl(rows, (acc, r) => recomputeRow(acc, r, startRow))

  pure def recomputeAfterPost(rows: List[Row], postRow: int): List[Row] =
    range(0, NPERM).foldl(rows, (acc, r) =>
      if (r <= postRow) acc else recomputeRow(acc, r, postRow))

  // ---- Extraction of the 42 opened header words.
  // Selector-projected extraction: for each block b in 0..5 and rate word k, sums sel[b] * inputs[k]
  // over all 8 rows. When selectors are one-hot (s == I_8), this equals rawOpenedWords(rows).
  pure def selWord(rows: List[Row], blk: int, k: int): int =
    range(0, NROW).foldl(0, (acc, r) => acc + rows[r].sel[blk] * rows[r].inputs[k])

  pure def openedWords(rows: List[Row]): List[int] = {
    val emptyInts: List[int] = []
    val first40 = range(0, NPERM - 1).foldl(emptyInts, (acc, blk) =>
      range(0, RATE).foldl(acc, (inner, k) => inner.append(selWord(rows, blk, k)))
    )
    range(0, PARTIAL_LEN).foldl(first40, (acc, k) => acc.append(selWord(rows, NPERM - 1, k)))
  }

  pure def rawOpenedWords(rows: List[Row]): List[int] = {
    val emptyInts: List[int] = []
    val first40 = range(0, NPERM - 1).foldl(emptyInts, (acc, r) =>
      acc.concat(rows[r].inputs.slice(0, RATE))
    )
    first40.concat(rows[NPERM - 1].inputs.slice(0, PARTIAL_LEN))
  }

  pure def isZeroPayload(words: List[int]): bool =
    2.to(2 + DIG - 1).forall(i => words[i] == 0)

  // ---- HeaderOpenAir constraints (lane-src/x_exp_node.rs:96-145).
  // 1. Poseidon2SubAir on every row: post == P(inputs)
  pure def cPoseidon(rows: List[Row]): bool =
    0.to(NROW - 1).forall(r => rows[r].post == perm(rows[r].inputs))

  // 2a. Selector booleanity: each s[i] in {0, 1}
  pure def cSelBool(rows: List[Row]): bool =
    0.to(NROW - 1).forall(r =>
      0.to(NROW - 1).forall(i => rows[r].sel[i] == 0 or rows[r].sel[i] == 1)
    )

  // 2b. Selector one-hot sum: sum_i s[i] == 1 on every row (dropped in M5)
  pure def cSelSumOne(rows: List[Row]): bool =
    if (DROP_SEL_ONEHOT) true
    else 0.to(NROW - 1).forall(r =>
      range(0, NROW).foldl(0, (acc, i) => acc + rows[r].sel[i]) == 1
    )

  // 2c. First row selector: s[0] == 1 on row 0
  pure def cSelFirst(rows: List[Row]): bool =
    rows[0].sel[0] == 1

  // 2d. Transition shift: next.s[i+1] == local.s[i] for i in 0..7
  pure def cSelShift(rows: List[Row]): bool =
    0.to(NROW - 2).forall(r =>
      0.to(NROW - 2).forall(i => rows[r + 1].sel[i + 1] == rows[r].sel[i])
    )

  // 3. Row 0 domain word: when s[0], inputs[0] == SHK_SD2 (dropped in M3)
  pure def cDomain(rows: List[Row]): bool =
    if (DROP_DOMAIN) true
    else 0.to(NROW - 1).forall(r =>
      (rows[r].sel[0] == 1) implies (rows[r].inputs[0] == SHK_SD2)
    )

  // 4. Row 0 zero capacity: when s[0], inputs[8..16] == 0 (dropped in M6)
  pure def cFirstCapZero(rows: List[Row]): bool =
    if (DROP_FIRST_CAP_ZERO) true
    else 0.to(NROW - 1).forall(r =>
      (rows[r].sel[0] == 1) implies RATE.to(WIDTH - 1).forall(k => rows[r].inputs[k] == 0)
    )

  // 5. Capacity chaining on transitions 0..5: when local.s[i], next.inputs[8..16] == local.post[8..16]
  // (M1 drops capacity chaining on row i == 0)
  pure def cCapChain(rows: List[Row]): bool =
    0.to(NROW - 2).forall(r =>
      0.to(NPERM - 2).forall(i =>
        (DROP_CAP_CHAIN and i == 0) or
        ((rows[r].sel[i] == 1) implies
          RATE.to(WIDTH - 1).forall(k => rows[r + 1].inputs[k] == rows[r].post[k]))
      )
    )

  // 6. Overwrite keep on transition 4 -> 5: when local.s[4], next.inputs[2..8] == local.post[2..8]
  // (dropped in M2)
  pure def cPartialKeep(rows: List[Row]): bool =
    if (DROP_PARTIAL_KEEP) true
    else 0.to(NROW - 2).forall(r =>
      (rows[r].sel[NPERM - 2] == 1) implies
        PARTIAL_LEN.to(RATE - 1).forall(k => rows[r + 1].inputs[k] == rows[r].post[k])
    )

  // 7. Public values match rate output: when s[5], post[0..8] == pv[0..8]
  // (M4 checks s[7] on the last padding row instead of s[5])
  pure def cOutput(rows: List[Row], pubVals: List[int]): bool = {
    val outSel = if (OUTPUT_ON_ROW7) (NROW - 1) else (NPERM - 1)
    0.to(NROW - 1).forall(r =>
      (rows[r].sel[outSel] == 1) implies
        0.to(DIG - 1).forall(k => rows[r].post[k] == pubVals[k])
    )
  }

  pure def airAccepts(rows: List[Row], pubVals: List[int]): bool = and {
    rows.length() == NROW,
    pubVals.length() == DIG,
    cPoseidon(rows),
    cSelBool(rows),
    cSelSumOne(rows),
    cSelFirst(rows),
    cSelShift(rows),
    cDomain(rows),
    cFirstCapZero(rows),
    cCapChain(rows),
    cPartialKeep(rows),
    cOutput(rows, pubVals),
  }

  // ---- Nondeterministic prover actions.
  action buildState(rows: List[Row], pubVals: List[int], zPay: bool): bool = {
    st' = {
      tr: rows,
      pv: pubVals,
      accepted: airAccepts(rows, pubVals),
      zeroPayload: zPay,
    }
  }

  // Honest opening with arbitrary header kind, zero-payload flag, all-zero digest flag, and free padding rows.
  action proveHonest(tagWord: int, zPay: bool, allZeroDig: bool, pad6: int, pad7: int): bool = {
    val words = mkHeaderWords(tagWord, zPay, allZeroDig)
    val rows = honestTrace(words, pad6, pad7)
    val pubVals = rows[NPERM - 1].post.slice(0, DIG)
    buildState(rows, pubVals, zPay)
  }

  // Change one input cell (row r, col c) by delta, recompute rows r..5 honestly, and choose pv.
  action proveTamperInput(r: int, c: int, delta: int, keepHonestPv: bool, pad6: int, pad7: int): bool = {
    val words = mkHeaderWords(2, false, false)
    val baseRows = honestTrace(words, pad6, pad7)
    val honestPv = baseRows[NPERM - 1].post.slice(0, DIG)
    val modIn = replaceElem(baseRows[r].inputs, c, baseRows[r].inputs[c] + delta)
    val modRow = { ...baseRows[r], inputs: modIn, post: perm(modIn) }
    val rows1 = replaceRow(baseRows, r, modRow)
    val rows2 = recomputeFrom(rows1, r)
    val chosenPv = if (keepHonestPv) honestPv else rows2[NPERM - 1].post.slice(0, DIG)
    buildState(rows2, chosenPv, false)
  }

  // Change one output (post) cell (row r, col c) by delta, recompute later rows r+1..5 honestly.
  action proveTamperPost(r: int, c: int, delta: int, keepHonestPv: bool): bool = {
    val words = mkHeaderWords(2, false, false)
    val baseRows = honestTrace(words, 0, 0)
    val honestPv = baseRows[NPERM - 1].post.slice(0, DIG)
    val modPost = replaceElem(baseRows[r].post, c, baseRows[r].post[c] + delta)
    val modRow = { ...baseRows[r], post: modPost }
    val rows1 = replaceRow(baseRows, r, modRow)
    val rows2 = recomputeAfterPost(rows1, r)
    val chosenPv = if (keepHonestPv) honestPv else rows2[NPERM - 1].post.slice(0, DIG)
    buildState(rows2, chosenPv, false)
  }

  // Bind public values to padding row 7 (tests M4 output_on_row7 and free padding row independence).
  action provePaddingPv(pad6: int, pad7: int): bool = {
    val words = mkHeaderWords(2, false, false)
    val rows = honestTrace(words, pad6, pad7)
    val row7Pv = rows[NROW - 1].post.slice(0, DIG)
    buildState(rows, row7Pv, false)
  }

  // Inject an extra active selector bit on padding rows 6 and 7 while satisfying all gated constraints (tests M5).
  action proveNonOneHotSelector: bool = {
    val words = mkHeaderWords(2, false, false)
    val baseRows = honestTrace(words, 0, 0)
    val pubVals = baseRows[NPERM - 1].post.slice(0, DIG)
    val r6In = replaceElem(zeros(WIDTH), 0, SHK_SD2)
    val r6Post = perm(r6In)
    val r7In = zeros(RATE).concat(r6Post.slice(RATE, WIDTH))
    val r7Post = perm(r7In)
    // s[6] has 1s at col 0 and col 6; s[7] has 1s at col 1 and col 7 (shifted from s[6])
    val sel6 = replaceElem(oneHotRow(6), 0, 1)
    val sel7 = replaceElem(oneHotRow(7), 1, 1)
    val rows1 = replaceRow(baseRows, 6, { inputs: r6In, post: r6Post, sel: sel6 })
    val rows2 = replaceRow(rows1, 7, { inputs: r7In, post: r7Post, sel: sel7 })
    buildState(rows2, pubVals, false)
  }

  action init = proveHonest(2, false, false, 0, 0)

  action step = any {
    {
      nondet tagW = Set(0, 1, 2).oneOf()
      nondet zPay = Set(false, true).oneOf()
      nondet zDig = Set(false, true).oneOf()
      nondet p6 = Set(0, 1, 2).oneOf()
      nondet p7 = Set(0, 1, 2).oneOf()
      proveHonest(tagW, zPay, zDig, p6, p7)
    },
    // Tamper capacity on row 1 (targets M1 no_capacity_chain) or any row 0..5
    {
      nondet r = Set(0, 1, 2, 3, 4, 5).oneOf()
      nondet c = RATE.to(WIDTH - 1).oneOf()
      nondet keepPv = Set(false, true).oneOf()
      proveTamperInput(r, c, 1, keepPv, 0, 0)
    },
    // Tamper partial-block kept rate words on row 5 (targets M2 no_partial_keep)
    {
      nondet c = PARTIAL_LEN.to(RATE - 1).oneOf()
      nondet keepPv = Set(false, true).oneOf()
      proveTamperInput(NPERM - 1, c, 1, keepPv, 0, 0)
    },
    // Tamper domain word on row 0 (targets M3 no_domain)
    {
      nondet delta = Set(1, 5).oneOf()
      nondet keepPv = Set(false, true).oneOf()
      proveTamperInput(0, 0, delta, keepPv, 0, 0)
    },
    // Bind pv to row 7 output (targets M4 output_on_row7)
    {
      nondet p6 = Set(0, 1, 2).oneOf()
      nondet p7 = Set(0, 1, 2).oneOf()
      provePaddingPv(p6, p7)
    },
    // Non-one-hot selector injection on padding rows 6..7 (targets M5 selector_not_onehot)
    proveNonOneHotSelector,
    // Tamper initial capacity on row 0 (targets M6 first_capacity_free)
    {
      nondet c = RATE.to(WIDTH - 1).oneOf()
      nondet keepPv = Set(false, true).oneOf()
      proveTamperInput(0, c, 1, keepPv, 0, 0)
    },
    // General single-cell input or post tamper across all 8 rows and 16 columns
    {
      nondet r = 0.to(NROW - 1).oneOf()
      nondet c = 0.to(WIDTH - 1).oneOf()
      nondet keepPv = Set(false, true).oneOf()
      any {
        proveTamperInput(r, c, 1, keepPv, 0, 0),
        proveTamperPost(r, c, 1, keepPv),
      }
    },
  }

  // ---- Refinement invariant invOpen (premise A5 / header_open_sound).
  // If the trace is accepted and the public values are pv, then the 42 opened words w
  // satisfy sponge(w) == pv and w[0] == SHK_SD2.
  val invOpen: bool =
    st.accepted implies (
      sponge(openedWords(st.tr)) == st.pv and
      openedWords(st.tr)[0] == SHK_SD2
    )

  // Fixed-coordinate variant (reads rows 0..5 directly without selector projection)
  val invRawOpen: bool =
    st.accepted implies (
      sponge(rawOpenedWords(st.tr)) == st.pv and
      rawOpenedWords(st.tr)[0] == SHK_SD2
    )

  // Selector canonicality: accepted trace has the exact 8x8 identity matrix on sel
  val invSelectorIdentity: bool =
    st.accepted implies 0.to(NROW - 1).forall(r => st.tr[r].sel == oneHotRow(r))

  // ---- Non-vacuity negated reach invariants (MUST BE VIOLATED in hdr_sound / header_open).
  // 1. Negated reach of an accepted honest opening (with non-zero payload)
  val invNoAcceptedHonest: bool =
    not(st.accepted and not(st.zeroPayload))

  // 2. Negated reach of an accepted opening with a zero payload
  val invNoAcceptedZeroPayload: bool =
    not(st.accepted and st.zeroPayload and isZeroPayload(openedWords(st.tr)))
}

// Sound module (all constraints active)
module header_open {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

module hdr_sound {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

// M1: drop capacity chaining on one row (row 0 -> row 1)
module hdr_no_capacity_chain {
  import header_open_base(
    DROP_CAP_CHAIN = true,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

// M2: drop row-5 constraint on inputs[2..8] (partial block overwrite keep)
module hdr_no_partial_keep {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = true,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

// M3: drop the domain constraint inputs[0] == SHK_SD2 on row 0
module hdr_no_domain {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = true,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

// M4: check public values on row 7 (padding) instead of row 5
module hdr_output_on_row7 {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = true,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

// M5: drop the sum = 1 selector constraint
module hdr_selector_not_onehot {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = true,
    DROP_FIRST_CAP_ZERO = false
  ).*
}

// M6: drop inputs[8..16] == 0 on row 0
module hdr_first_capacity_free {
  import header_open_base(
    DROP_CAP_CHAIN = false,
    DROP_PARTIAL_KEEP = false,
    DROP_DOMAIN = false,
    OUTPUT_ON_ROW7 = false,
    DROP_SEL_ONEHOT = false,
    DROP_FIRST_CAP_ZERO = true
  ).*
}
