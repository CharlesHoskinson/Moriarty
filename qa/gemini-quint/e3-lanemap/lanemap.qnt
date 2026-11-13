// Soundness & completeness model of the D-9 reader `decode` lane map in `lane-src/iface.rs`
// (openvm `e3-iface` 099d6d8).
//
// Models:
// - A small log of 11 entries covering {NOP, PERM, H2, SPONGE(2), SPONGE(3), PUB, DIGEST}
//   with 1 to 4 small-integer values per non-NOP entry (and 0 values for trailing NOP),
//   flattened into `vs` with offset table `offs` as in `decode`.
// - A small call vector of 8 lanes (lanes 0..1 are binding/comm, lanes 2..7 are effect lanes).
// - 4 lane-map pairs `(lane, ent)` with `(0, 0)` as the unused sentinel.
// - The exact reader rule from `decode` (including `o + vals(op, param) - 1`, `s[lane] != 0`,
//   and the post-lane-map `PUB(i)` check) alongside `nativeStatement`.
// - Named mutants (`mut_any_sponge`, `mut_first_value_not_last`, `mut_no_dup_check`,
//   `mut_any_op`, `mut_pub_before_lanemap`) and a fixed duplicate-check reader `decodeFixedDup`.

module lanemap {

  // ---- Op codes (matching `lane-src/iface.rs`) ------------------------------------------------
  pure val OP_NOP: int = 0
  pure val OP_PERM: int = 1
  pure val OP_H2: int = 2
  pure val OP_SPONGE: int = 3
  pure val OP_PUB: int = 7
  pure val OP_DIGEST: int = 8

  // ---- Digest tags (matching `lane-src/iface.rs`) ---------------------------------------------
  pure val SHK_PROGC: int = 0
  pure val SHK_LOCK: int = 1
  pure val SHK_INPUT: int = 2
  pure val SHK_OUTCOME: int = 3

  // ---- Model dimensions -----------------------------------------------------------------------
  pure val N_LANES: int = 8
  pure val N_PAIRS: int = 4
  pure val PAIR_INDICES: Set[int] = Set(0, 1, 2, 3)
  pure val LANE_INDICES: Set[int] = Set(0, 1, 2, 3, 4, 5, 6, 7)

  // Honest lane indices (shifted down from 9, 19, 33, 38 into 2..5 of the 8-lane call vector;
  // lanes 0 and 1 remain the binding and comm lanes, lane 6 is checked by PUB(6), lane 7 is spare).
  pure val HONEST_LANE_A_IN: int = 2
  pure val HONEST_LANE_CLAIM_0: int = 3
  pure val HONEST_LANE_CLAIM_1: int = 4
  pure val HONEST_LANE_OUTCOME: int = 5
  pure val ANCHOR_IN_ENT: int = 4

  // ---- Types ----------------------------------------------------------------------------------
  type LogEntry = {
    op: int,
    param: int,
    values: List[int],
  }

  type EntryOff = {
    op: int,
    param: int,
    off: int,
  }

  type LanePair = {
    lane: int,
    ent: int,
  }

  type DecodeResult = {
    ok: bool,
    stmt: List[int],
    errTag: str,
  }

  type AnchorTags = {
    ok: bool,
    tagIn: int,
    tagOut: int,
  }

  type OffBuildAcc = {
    list: List[EntryOff],
    curOff: int,
  }

  type FormAcc = {
    ok: bool,
    ended: bool,
  }

  type FixedLoopAcc = {
    ok: bool,
    stmt: List[int],
    seenLanes: Set[int],
    errTag: str,
  }

  type StepKind =
    | StepInit
    | StepHonest
    | StepAdversarial

  type ModelState = {
    mode: int,
    chunkJ: int,
    chunkK: int,
    isHonest: bool,
    stepKind: StepKind,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair],
    decoded: DecodeResult,
  }

  // ---- Typed empty collections (avoids polymorphic empty collection inference issues) ---------
  pure val EMPTY_INT_LIST: List[int] = List()
  pure val EMPTY_OFF_LIST: List[EntryOff] = List()
  pure val EMPTY_INT_SET: Set[int] = Set()

  // ---- Pure helper functions (`vals`, offsets, native statement) ------------------------------

  // Logged value count `vals(op, param)` in the small model (1..4 values for used ops, 0 for NOP).
  pure def entryValCount(op: int, param: int): int =
    if (op == OP_NOP) 0
    else if (op == OP_PUB) 1
    else if (op == OP_DIGEST) 2
    else if (op == OP_H2) 3
    else if (op == OP_SPONGE) param + 1
    else if (op == OP_PERM) 4
    else 0

  pure def isUnusedPair(p: LanePair): bool =
    p.lane == 0 and p.ent == 0

  pure def isAllowedLaneMapEntry(op: int, param: int): bool =
    op == OP_DIGEST or (op == OP_SPONGE and param == 3)

  pure def lastEntryValue(e: LogEntry): int =
    if (e.values.length() > 0)
      e.values.nth(e.values.length() - 1)
    else
      0

  // Flatten logged values into `vs` as in `Run::words` / `decode`.
  pure def flattenValues(log: List[LogEntry]): List[int] =
    log.foldl(EMPTY_INT_LIST, (acc, e) => acc.concat(e.values))

  // Build `(op, param, off)` table `offs` as in `decode`.
  pure def buildOffs(log: List[LogEntry]): List[EntryOff] = {
    val initAcc: OffBuildAcc = { list: EMPTY_OFF_LIST, curOff: 0 }
    val built = log.foldl(initAcc, (acc, e) => {
      val nextOff = acc.curOff + entryValCount(e.op, e.param)
      {
        list: acc.list.append({ op: e.op, param: e.param, off: acc.curOff }),
        curOff: nextOff,
      }
    })
    built.list
  }

  // Check padding rule: no non-NOP entry after a NOP entry, and every entry's value length matches `entryValCount`.
  pure def logWellFormed(log: List[LogEntry]): bool = {
    val initAcc: FormAcc = { ok: true, ended: false }
    val st = log.foldl(initAcc, (acc, e) => {
      val lenOk = e.values.length() == entryValCount(e.op, e.param)
      if (not(acc.ok) or not(lenOk))
        { ok: false, ended: acc.ended }
      else if (e.op == OP_NOP)
        { ok: true, ended: true }
      else if (acc.ended)
        { ok: false, ended: true }
      else
        { ok: true, ended: false }
    })
    st.ok
  }

  // Anchor tags `(tag_in, tag_out)` from `(mode, j, k)` as in `anchor_tags`.
  pure def anchorTags(mode: int, j: int, k: int): AnchorTags =
    if (mode == 0)
      { ok: true, tagIn: SHK_PROGC, tagOut: SHK_PROGC }
    else if (mode == 1 and j >= 1 and j <= k)
      {
        ok: true,
        tagIn: if (j == 1) SHK_PROGC else SHK_LOCK,
        tagOut: if (j == k) SHK_PROGC else SHK_LOCK,
      }
    else
      { ok: false, tagIn: 0, tagOut: 0 }

  // Check reservation `R` and `claim_pub` input binding as in `decode` lines 605..618.
  pure def checkReservation(mode: int, j: int, k: int, offs: List[EntryOff], vs: List[int]): bool = {
    val tags = anchorTags(mode, j, k)
    val minLen = if (mode == 1) 6 else 5
    if (not(tags.ok) or offs.length() < minLen or vs.length() < 12)
      false
    else {
      val e0 = offs.nth(0)
      val e1 = offs.nth(1)
      val e2 = offs.nth(2)
      val e3 = offs.nth(3)
      val e4 = offs.nth(4)
      val rBaseOk = and {
        e0.op == OP_DIGEST and e0.param == SHK_PROGC,
        e1.op == OP_DIGEST and e1.param == SHK_INPUT,
        e2.op == OP_DIGEST and e2.param == SHK_OUTCOME,
        e3.op == OP_SPONGE and e3.param == 3,
        e4.op == OP_DIGEST and e4.param == tags.tagIn,
      }
      val rChunkOk = if (mode == 1) {
        val e5 = offs.nth(5)
        e5.op == OP_DIGEST and e5.param == tags.tagOut
      } else true
      // `sp = R_OFF[3] = 6`: `vs[6..9] == [vs[1], vs[3], vs[5]]`
      val claimInputsOk = and {
        vs.nth(6) == vs.nth(1),
        vs.nth(7) == vs.nth(3),
        vs.nth(8) == vs.nth(5),
      }
      rBaseOk and rChunkOk and claimInputsOk
    }
  }

  // `Run::statement`: the native statement sets each non-(0,0) lane-map lane to the last value of its entry.
  pure def nativeStatement(log: List[LogEntry], callLanes: List[int], laneMap: List[LanePair]): List[int] =
    laneMap.foldl(callLanes, (s, p) =>
      if (isUnusedPair(p))
        s
      else if (p.ent >= 0 and p.ent < log.length() and p.lane >= 0 and p.lane < s.length()) {
        val e = log.nth(p.ent)
        s.replaceAt(p.lane, lastEntryValue(e))
      } else
        s
    )

  // Check `PUB(i)` entries against statement vector `s` (`decode` lines 644..648).
  pure def checkPubEntries(offs: List[EntryOff], vs: List[int], s: List[int]): bool =
    offs.foldl(true, (ok, eo) =>
      if (not(ok))
        false
      else if (eo.op == OP_PUB)
        eo.param >= 0 and eo.param < s.length() and s.nth(eo.param) == vs.nth(eo.off)
      else
        true
    )

  // ---- The D-9 reader `decode` (exact lane-map + PUB logic from `lane-src/iface.rs`) ----------

  pure def decode(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)))
      { ok: false, stmt: callLanes, errTag: "log_form" }
    else if (not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "reservation" }
    else if (laneMap.length() != N_PAIRS or callLanes.length() != N_LANES)
      { ok: false, stmt: callLanes, errTag: "dims" }
    else {
      val initLoop: DecodeResult = { ok: true, stmt: callLanes, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok))
            acc
          else if (isUnusedPair(p))
            acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ok: false, stmt: acc.stmt, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            val opOk = isAllowedLaneMapEntry(eo.op, eo.param)
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            val laneZero = if (laneInRange) acc.stmt.nth(p.lane) == 0 else false
            if (not(opOk) or not(laneInRange) or not(laneZero))
              { ok: false, stmt: acc.stmt, errTag: "lane_map_pair" }
            else {
              val outIdx = eo.off + entryValCount(eo.op, eo.param) - 1
              val outVal = vs.nth(outIdx)
              { ok: true, stmt: acc.stmt.replaceAt(p.lane, outVal), errTag: "" }
            }
          }
      )
      if (not(loopRes.ok))
        loopRes
      else if (not(checkPubEntries(offs, vs, loopRes.stmt)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else
        loopRes
    }
  }

  // Fixed reader variant that tracks mapped lanes in an explicit set so that a zero-valued
  // entry output cannot allow a second pair to overwrite the same lane.
  pure def decodeFixedDup(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)) or not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "pre" }
    else {
      val initLoop: FixedLoopAcc = { ok: true, stmt: callLanes, seenLanes: EMPTY_INT_SET, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok))
            acc
          else if (isUnusedPair(p))
            acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ...acc, ok: false, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            val opOk = isAllowedLaneMapEntry(eo.op, eo.param)
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            val notSeen = not(acc.seenLanes.contains(p.lane))
            val laneZero = if (laneInRange) callLanes.nth(p.lane) == 0 else false
            if (not(opOk) or not(laneInRange) or not(notSeen) or not(laneZero))
              { ...acc, ok: false, errTag: "lane_map_pair" }
            else {
              val outIdx = eo.off + entryValCount(eo.op, eo.param) - 1
              val outVal = vs.nth(outIdx)
              {
                ok: true,
                stmt: acc.stmt.replaceAt(p.lane, outVal),
                seenLanes: acc.seenLanes.union(Set(p.lane)),
                errTag: "",
              }
            }
          }
      )
      if (not(loopRes.ok))
        { ok: false, stmt: loopRes.stmt, errTag: loopRes.errTag }
      else if (not(checkPubEntries(offs, vs, loopRes.stmt)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else
        { ok: true, stmt: loopRes.stmt, errTag: "" }
    }
  }

  // ---- Named mutants of `decode` (each relaxes or alters one reader check) --------------------

  // Mutant 1: accepts any `SPONGE(n)` entry (including `SPONGE(2)`), not only `SPONGE(3)`.
  pure def mut_any_sponge(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)) or not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "pre" }
    else {
      val initLoop: DecodeResult = { ok: true, stmt: callLanes, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok)) acc
          else if (isUnusedPair(p)) acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ok: false, stmt: acc.stmt, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            // BUG: accepts any OP_SPONGE regardless of param
            val opOk = eo.op == OP_DIGEST or eo.op == OP_SPONGE
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            val laneZero = if (laneInRange) acc.stmt.nth(p.lane) == 0 else false
            if (not(opOk) or not(laneInRange) or not(laneZero))
              { ok: false, stmt: acc.stmt, errTag: "lane_map_pair" }
            else {
              val outIdx = eo.off + entryValCount(eo.op, eo.param) - 1
              { ok: true, stmt: acc.stmt.replaceAt(p.lane, vs.nth(outIdx)), errTag: "" }
            }
          }
      )
      if (not(loopRes.ok)) loopRes
      else if (not(checkPubEntries(offs, vs, loopRes.stmt)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else loopRes
    }
  }

  // Mutant 2: reads the first value `vs[o]` of the mapped entry instead of the last value `vs[o + vals - 1]`.
  pure def mut_first_value_not_last(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)) or not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "pre" }
    else {
      val initLoop: DecodeResult = { ok: true, stmt: callLanes, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok)) acc
          else if (isUnusedPair(p)) acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ok: false, stmt: acc.stmt, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            val opOk = isAllowedLaneMapEntry(eo.op, eo.param)
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            val laneZero = if (laneInRange) acc.stmt.nth(p.lane) == 0 else false
            if (not(opOk) or not(laneInRange) or not(laneZero))
              { ok: false, stmt: acc.stmt, errTag: "lane_map_pair" }
            else {
              // BUG: uses `eo.off` (first value) instead of `eo.off + entryValCount - 1` (last value)
              val outIdx = eo.off
              { ok: true, stmt: acc.stmt.replaceAt(p.lane, vs.nth(outIdx)), errTag: "" }
            }
          }
      )
      if (not(loopRes.ok)) loopRes
      else if (not(checkPubEntries(offs, vs, loopRes.stmt)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else loopRes
    }
  }

  // Mutant 3: omits the `s[lane] != 0` duplicate / zero-limb check.
  pure def mut_no_dup_check(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)) or not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "pre" }
    else {
      val initLoop: DecodeResult = { ok: true, stmt: callLanes, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok)) acc
          else if (isUnusedPair(p)) acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ok: false, stmt: acc.stmt, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            val opOk = isAllowedLaneMapEntry(eo.op, eo.param)
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            // BUG: missing `acc.stmt.nth(p.lane) == 0` check
            if (not(opOk) or not(laneInRange))
              { ok: false, stmt: acc.stmt, errTag: "lane_map_pair" }
            else {
              val outIdx = eo.off + entryValCount(eo.op, eo.param) - 1
              { ok: true, stmt: acc.stmt.replaceAt(p.lane, vs.nth(outIdx)), errTag: "" }
            }
          }
      )
      if (not(loopRes.ok)) loopRes
      else if (not(checkPubEntries(offs, vs, loopRes.stmt)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else loopRes
    }
  }

  // Mutant 4: accepts any non-NOP op in the lane map (e.g., PERM, H2, PUB, SPONGE(2)).
  pure def mut_any_op(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)) or not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "pre" }
    else {
      val initLoop: DecodeResult = { ok: true, stmt: callLanes, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok)) acc
          else if (isUnusedPair(p)) acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ok: false, stmt: acc.stmt, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            // BUG: accepts any op with at least 1 value
            val opOk = entryValCount(eo.op, eo.param) >= 1
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            val laneZero = if (laneInRange) acc.stmt.nth(p.lane) == 0 else false
            if (not(opOk) or not(laneInRange) or not(laneZero))
              { ok: false, stmt: acc.stmt, errTag: "lane_map_pair" }
            else {
              val outIdx = eo.off + entryValCount(eo.op, eo.param) - 1
              { ok: true, stmt: acc.stmt.replaceAt(p.lane, vs.nth(outIdx)), errTag: "" }
            }
          }
      )
      if (not(loopRes.ok)) loopRes
      else if (not(checkPubEntries(offs, vs, loopRes.stmt)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else loopRes
    }
  }

  // Mutant 5: checks PUB(i) against the pre-lane-map `callLanes` instead of post-lane-map `s`.
  pure def mut_pub_before_lanemap(
    mode: int,
    j: int,
    k: int,
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair]
  ): DecodeResult = {
    val vs = flattenValues(log)
    val offs = buildOffs(log)
    if (not(logWellFormed(log)) or not(checkReservation(mode, j, k, offs, vs)))
      { ok: false, stmt: callLanes, errTag: "pre" }
    else {
      val initLoop: DecodeResult = { ok: true, stmt: callLanes, errTag: "" }
      val loopRes = laneMap.foldl(
        initLoop,
        (acc, p) =>
          if (not(acc.ok)) acc
          else if (isUnusedPair(p)) acc
          else if (p.ent < 0 or p.ent >= offs.length())
            { ok: false, stmt: acc.stmt, errTag: "lane_map_entry" }
          else {
            val eo = offs.nth(p.ent)
            val opOk = isAllowedLaneMapEntry(eo.op, eo.param)
            val laneInRange = p.lane >= 0 and p.lane < N_LANES
            val laneZero = if (laneInRange) acc.stmt.nth(p.lane) == 0 else false
            if (not(opOk) or not(laneInRange) or not(laneZero))
              { ok: false, stmt: acc.stmt, errTag: "lane_map_pair" }
            else {
              val outIdx = eo.off + entryValCount(eo.op, eo.param) - 1
              { ok: true, stmt: acc.stmt.replaceAt(p.lane, vs.nth(outIdx)), errTag: "" }
            }
          }
      )
      if (not(loopRes.ok)) loopRes
      // BUG: checks `callLanes` instead of `loopRes.stmt`
      else if (not(checkPubEntries(offs, vs, callLanes)))
        { ok: false, stmt: loopRes.stmt, errTag: "pub_mismatch" }
      else loopRes
    }
  }

  // ---- Concrete logs, call vectors, and honest `case_run` configurations ----------------------

  // Build an 11-entry log for `(mode, j, k)` containing R (entries 0..4 or 0..5) and
  // discretionary entries covering DIGEST, PERM, H2, SPONGE(2), PUB(6), and trailing NOP.
  pure def makeCaseLog(mode: int, j: int, k: int): List[LogEntry] = {
    val tags = anchorTags(mode, j, k)
    val aInVal = if (tags.tagIn == SHK_PROGC) 21 else 24
    val aInIn = if (tags.tagIn == SHK_PROGC) 11 else 14
    val aOutVal = if (tags.tagOut == SHK_PROGC) 21 else 25
    val aOutIn = if (tags.tagOut == SHK_PROGC) 11 else 15
    List(
      // 0: DIGEST(SHK_PROGC), 2 values: input 11, output 21
      { op: OP_DIGEST, param: SHK_PROGC, values: List(11, 21) },
      // 1: DIGEST(SHK_INPUT), 2 values: input 12, output 22
      { op: OP_DIGEST, param: SHK_INPUT, values: List(12, 22) },
      // 2: DIGEST(SHK_OUTCOME), 2 values: input 13, output 23
      { op: OP_DIGEST, param: SHK_OUTCOME, values: List(13, 23) },
      // 3: SPONGE(3) (`claim_pub` in R), 4 values: inputs [21, 22, 23], output 33
      { op: OP_SPONGE, param: 3, values: List(21, 22, 23, 33) },
      // 4: ANCHOR_IN DIGEST(tagIn), 2 values
      { op: OP_DIGEST, param: tags.tagIn, values: List(aInIn, aInVal) },
      // 5: ANCHOR_OUT DIGEST(tagOut) in chunk mode, or discretionary DIGEST(SHK_LOCK) in mode 0
      { op: OP_DIGEST, param: if (mode == 1) tags.tagOut else SHK_LOCK, values: List(aOutIn, aOutVal) },
      // 6: PERM, 4 values
      { op: OP_PERM, param: 0, values: List(1, 2, 3, 44) },
      // 7: H2, 3 values
      { op: OP_H2, param: 0, values: List(4, 5, 55) },
      // 8: SPONGE(2), 3 values (must be refused by lane map since param != 3)
      { op: OP_SPONGE, param: 2, values: List(6, 7, 66) },
      // 9: PUB(6), 1 value matching unmapped lane 6
      { op: OP_PUB, param: 6, values: List(77) },
      // 10: NOP padding entry at the end of the entry table
      { op: OP_NOP, param: 0, values: EMPTY_INT_LIST }
    )
  }

  // Log where entry 4 (`ANCHOR_IN`) has output 0 (`values = List(11, 0)`), exposing the
  // `s[lane] != 0` duplicate-lane collision when two pairs target the same lane (Question 2).
  pure val ZERO_OUTPUT_LOG: List[LogEntry] =
    makeCaseLog(0, 0, 0).replaceAt(4, { op: OP_DIGEST, param: SHK_PROGC, values: List(11, 0) })

  // Log where discretionary entry 8 is a second `SPONGE(3)` outside R with inputs [81, 82, 83]
  // and output 84 (Question 4: accepted by `decode` if the wrap does not fix lane_map).
  pure val EXTRA_SPONGE3_LOG: List[LogEntry] =
    makeCaseLog(0, 0, 0).replaceAt(8, { op: OP_SPONGE, param: 3, values: List(81, 82, 83, 84) })

  // Default call vector:
  // lane 0 = 90 (binding), lane 1 = 91 (comm), lanes 2..5 = 0 (mapped lanes),
  // lane 6 = 77 (checked by PUB(6)), lane 7 = 0 (unmapped zero lane).
  pure val DEFAULT_CALL_LANES: List[int] = List(90, 91, 0, 0, 0, 0, 77, 0)

  // Honest lane map corresponding to `case_run` `[(9, 4), (19, 3), (33, 3), (38, 2)]`:
  // maps `a_in` -> entry 4, `claim_pub[0]` -> entry 3, `claim_pub[1]` -> entry 3, `outcome` -> entry 2.
  pure val HONEST_LANE_MAP: List[LanePair] = List(
    { lane: HONEST_LANE_A_IN, ent: ANCHOR_IN_ENT },
    { lane: HONEST_LANE_CLAIM_0, ent: 3 },
    { lane: HONEST_LANE_CLAIM_1, ent: 3 },
    { lane: HONEST_LANE_OUTCOME, ent: 2 }
  )

  // ---- Soundness & Completeness predicates ----------------------------------------------------

  // No two active pairs in `laneMap` target the same lane.
  pure def noDuplicateMappedLanes(laneMap: List[LanePair]): bool =
    PAIR_INDICES.forall(i =>
      PAIR_INDICES.forall(j =>
        if (i < j) {
          val pi = laneMap.nth(i)
          val pj = laneMap.nth(j)
          isUnusedPair(pi) or isUnusedPair(pj) or pi.lane != pj.lane
        } else true
      )
    )

  // Soundness predicate for any decoder result `res`:
  // If `res.ok` holds, then:
  // 1. No lane is set twice by `laneMap`.
  // 2. Every active pair `(lane, ent)` has `lane` in `0..N_LANES-1`, `ent` in `0..log.length()-1`,
  //    points to a `DIGEST` or `SPONGE(3)` entry `e`, had `callLanes[lane] == 0`, and sets
  //    `res.stmt[lane] == lastEntryValue(e)`.
  // 3. `res.stmt` equals `nativeStatement(log, callLanes, laneMap)`.
  // 4. Every `PUB(i)` entry in `log` equals `res.stmt[i]`.
  pure def soundForResult(
    log: List[LogEntry],
    callLanes: List[int],
    laneMap: List[LanePair],
    res: DecodeResult
  ): bool = {
    val offs = buildOffs(log)
    val vs = flattenValues(log)
    res.ok implies and {
      noDuplicateMappedLanes(laneMap),
      PAIR_INDICES.forall(idx => {
        val p = laneMap.nth(idx)
        isUnusedPair(p) or and {
          p.lane >= 0 and p.lane < N_LANES,
          p.ent >= 0 and p.ent < log.length(),
          isAllowedLaneMapEntry(log.nth(p.ent).op, log.nth(p.ent).param),
          callLanes.nth(p.lane) == 0,
          res.stmt.nth(p.lane) == lastEntryValue(log.nth(p.ent)),
        }
      }),
      res.stmt == nativeStatement(log, callLanes, laneMap),
      checkPubEntries(offs, vs, res.stmt),
    }
  }

  // Check an honest `(mode, j, k)` run decodes and matches `nativeStatement`.
  pure def honestCaseDecodes(mode: int, j: int, k: int): bool = {
    val log = makeCaseLog(mode, j, k)
    val res = decode(mode, j, k, log, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    res.ok and res.stmt == nativeStatement(log, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
  }

  // ---- State Machine --------------------------------------------------------------------------

  var state: ModelState

  // @invariant
  val SOUND: bool =
    soundForResult(state.log, state.callLanes, state.laneMap, state.decoded)

  // @invariant
  val COMPLETE: bool = and {
    state.isHonest implies (
      state.decoded.ok and
      state.decoded.stmt == nativeStatement(state.log, state.callLanes, state.laneMap)
    ),
    // Ordinary case_run (mode 0) and chunk cases (j=1..3 of k=3, and j=1 of k=1) all decode:
    honestCaseDecodes(0, 0, 0),
    honestCaseDecodes(1, 1, 3),
    honestCaseDecodes(1, 2, 3),
    honestCaseDecodes(1, 3, 3),
    honestCaseDecodes(1, 1, 1),
  }

  // Witnesses for `quint run --witnesses`:
  val witnessHonestAccepted: bool =
    state.stepKind == StepHonest and state.decoded.ok

  val witnessAdversarialAccepted: bool =
    state.stepKind == StepAdversarial and state.decoded.ok

  val witnessAdversarialRefused: bool =
    state.stepKind == StepAdversarial and not(state.decoded.ok)

  // Candidate pairs for adversarial exploration:
  // Includes:
  // - (0, 0) unused sentinel
  // - valid pairs to DIGEST (entries 0, 1, 2, 4, 5) and SPONGE(3) (entry 3)
  // - invalid op pairs to PERM (6), H2 (7), SPONGE(2) (8), PUB (9), NOP (10)
  // - out-of-range entry (11) and out-of-range lane (8)
  // - mapping lane 0 (binding) or lane 6 (non-zero call-vector lane)
  pure val ADVERSARIAL_PAIRS: Set[LanePair] = Set(
    { lane: 0, ent: 0 },
    { lane: 2, ent: 4 },
    { lane: 2, ent: 0 },
    { lane: 3, ent: 3 },
    { lane: 4, ent: 3 },
    { lane: 5, ent: 2 },
    { lane: 7, ent: 5 },
    { lane: 2, ent: 6 },
    { lane: 2, ent: 7 },
    { lane: 3, ent: 8 },
    { lane: 4, ent: 9 },
    { lane: 5, ent: 10 },
    { lane: 2, ent: 11 },
    { lane: 8, ent: 3 },
    { lane: 0, ent: 3 },
    { lane: 6, ent: 2 }
  )

  pure val HONEST_CONFIGS: Set[(int, int, int)] = Set(
    (0, 0, 0),
    (1, 1, 3),
    (1, 2, 3),
    (1, 3, 3),
    (1, 1, 1)
  )

  pure def applyHonestConfig(cfg: (int, int, int)): ModelState = {
    val m = cfg._1
    val j = cfg._2
    val k = cfg._3
    val log = makeCaseLog(m, j, k)
    val dec = decode(m, j, k, log, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    {
      mode: m,
      chunkJ: j,
      chunkK: k,
      isHonest: true,
      stepKind: StepHonest,
      log: log,
      callLanes: DEFAULT_CALL_LANES,
      laneMap: HONEST_LANE_MAP,
      decoded: dec,
    }
  }

  pure def applyAdversarialConfig(
    cfg: (int, int, int),
    p0: LanePair,
    p1: LanePair,
    p2: LanePair,
    p3: LanePair,
    lane2Override: int
  ): ModelState = {
    val m = cfg._1
    val j = cfg._2
    val k = cfg._3
    val log = makeCaseLog(m, j, k)
    val callLanes = DEFAULT_CALL_LANES.replaceAt(2, lane2Override)
    val lmap = List(p0, p1, p2, p3)
    val dec = decode(m, j, k, log, callLanes, lmap)
    {
      mode: m,
      chunkJ: j,
      chunkK: k,
      isHonest: false,
      stepKind: StepAdversarial,
      log: log,
      callLanes: callLanes,
      laneMap: lmap,
      decoded: dec,
    }
  }

  action init: bool = {
    val log = makeCaseLog(0, 0, 0)
    val dec = decode(0, 0, 0, log, DEFAULT_CALL_LANES, HONEST_LANE_MAP)
    all {
      state' = {
        mode: 0,
        chunkJ: 0,
        chunkK: 0,
        isHonest: true,
        stepKind: StepInit,
        log: log,
        callLanes: DEFAULT_CALL_LANES,
        laneMap: HONEST_LANE_MAP,
        decoded: dec,
      }
    }
  }

  action stepHonest(cfg: (int, int, int)): bool = all {
    HONEST_CONFIGS.contains(cfg),
    state' = applyHonestConfig(cfg),
  }

  action stepAdversarial(
    cfg: (int, int, int),
    p0: LanePair,
    p1: LanePair,
    p2: LanePair,
    p3: LanePair,
    lane2Override: int
  ): bool = all {
    HONEST_CONFIGS.contains(cfg),
    state' = applyAdversarialConfig(cfg, p0, p1, p2, p3, lane2Override),
  }

  action step: bool = {
    nondet cfg = HONEST_CONFIGS.oneOf()
    nondet p0 = ADVERSARIAL_PAIRS.oneOf()
    nondet p1 = ADVERSARIAL_PAIRS.oneOf()
    nondet p2 = ADVERSARIAL_PAIRS.oneOf()
    nondet p3 = ADVERSARIAL_PAIRS.oneOf()
    nondet lane2Override = Set(0, 99).oneOf()
    any {
      stepHonest(cfg),
      stepAdversarial(cfg, p0, p1, p2, p3, lane2Override),
    }
  }

}
