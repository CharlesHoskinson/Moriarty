// -*- mode: Bluespec; -*-
//
// Quint Formal Specification of C1 Dense Tail-Buffered Merkle Memory
// (`crates/mori-chip/src/membackend.rs`, `src/dense.rs`, `src/dense/tests.rs`, `src/poseidon.rs`)
//
// Models the circuit AS BUILT:
//   1. `WindowCfg` (`membackend.rs`): frontier bit decompositions (`len_pre`, `len_post`, `q_pre`),
//      window rows (`addr`, `kind`, `act`, `imp`, `kfix`), bus provider tuples `(chip, key, d0, d1, d2)`,
//      and increment rows (`inc`).
//   2. `DenseCfg` (`dense.rs`): capacity-tagged sponges (`DOM_REC`, `DOM_PAD`, `DOM_EMPTY`,
//      `DOM_NODE` with height `h`), `mask` rows, segment trees (`treeSeg`), `W` flush units
//      (walk index chain `idx_j = idx_{j-1} + f_j`, old walk from `EMPTY_SEG`, new walk sharing
//      `[s, b, acc_in, acc_out]`, `"dense flush"` gate), post-tail mux chain, and `cold_read`
//      (`TOP + SEG_BITS` walk + `"dense geq"` gate).
//   3. CORE stand-in (`dense/tests.rs`): append chain (`AP`, `AQ`, `inc` from `len_pre` to `len_post`)
//      and read port (`RD`, `RQ`, range-checked `diff` and `d2`, routing warm reads to bus and cold
//      reads to `cold_read`).

module dense {
  const MUTANT: str

  // Small parameters so exhaustive and symbolic runs finish in seconds:
  // SEGC = 4 records per segment (SEG_BITS = 2), TOP = 2 levels in top tree (4 segment slots,
  // total capacity MAX_LEN = SEGC * 2^TOP = 16 records, len in 0..15).
  // Bus key uses the exact circuit formula `8 * addr + kind` with kind in 0..=4.
  pure val SEGC: int = 4
  pure val SEG_BITS: int = 2
  pure val TOP: int = 2
  pure val NB: int = TOP + SEG_BITS // 4 bits: 0..15
  pure val TOP_SLOTS: int = 4       // 2^TOP
  pure val MAX_LEN: int = 16        // 2^(TOP + SEG_BITS)

  pure val CHIP_NONE: int = 0
  pure val CHIP_MEM: int = 2

  pure val KIND_PAD: int = 0
  pure val KIND_DATA: int = 1
  pure val KIND_FIELD: int = 2
  pure val KIND_ENV: int = 3
  pure val KIND_SLOT: int = 4

  pure val DOM_REC: int = 1
  pure val DOM_PAD: int = 2
  pure val DOM_EMPTY: int = 3
  pure val DOM_NODE: int = 4

  // Prime modulus for injective capacity-tagged sponge (all intermediate products < 2 * 10^12 << INT64_MAX)
  pure val HP: int = 1000003

  // Record `[kind, d0, d1, d2]` (`dense.rs:65`)
  type Rec = {
    kind: int,
    d0: int,
    d1: int,
    d2: int
  }

  pure val ZERO_REC: Rec = { kind: 0, d0: 0, d1: 0, d2: 0 }

  // Bus tuple `(chip, key, d0, d1, d2)` (`bus.rs:36`)
  type BusTuple = {
    chip: int,
    bkey: int,
    d0: int,
    d1: int,
    d2: int
  }

  pure val ZERO_TUPLE: BusTuple = { chip: 0, bkey: 0, d0: 0, d1: 0, d2: 0 }

  pure def busKey(addr: int, kind: int): int = 8 * addr + kind

  // ---------------------------------------------------------------- Capacity-Tagged Sponge (`poseidon.rs`)
  // Capacity tag `iv(dom, sub)` (`dense.rs:60-62`).
  // When `MUTANT == "domain_tag_dropped"`, `iv` is 0 so tags and node heights are omitted.
  pure def iv(mt: str, dom: int, sub: int): int =
    if (mt == "domain_tag_dropped") 0
    else dom * 1000 + sub * 10

  pure def modP(x: int): int = {
    val r = x % HP
    if (r < 0) r + HP else r
  }

  pure def perm1(s0: int, s1: int, s2: int): int = {
    val u0 = modP(s0 + 17)
    val u1 = modP(s1 + 31)
    val u2 = modP(s2 + 43)
    val m0 = modP(u0 * 313 + u1 * 919 + u2 * 3571)
    modP(m0 * m0 + u0 * 7919 + u1 * 2971 + u2 * 4099 + 12345)
  }

  // `sponge_native_iv` for 2 inputs (`poseidon.rs:107-120`)
  pure def sponge2Iv(inputs0: int, inputs1: int, ivVal: int): int =
    perm1(inputs0, inputs1, 2 + ivVal)

  // `sponge_native_iv` for 4 inputs (`poseidon.rs:107-120`, 2 permutations)
  pure def sponge4Iv(in0: int, in1: int, in2: int, in3: int, ivVal: int): int = {
    val st0 = perm1(in0, in1, 4 + ivVal)
    val st1 = perm1(modP(in2 + st0), in3 + 19, 4 + ivVal + 73)
    st1
  }

  pure def recD(mt: str, r: Rec): int =
    sponge4Iv(r.kind, r.d0, r.d1, r.d2, iv(mt, DOM_REC, 0))

  pure def padD(mt: str): int =
    sponge2Iv(0, 0, iv(mt, DOM_PAD, 0))

  pure def emptySeg(mt: str): int =
    sponge2Iv(0, 0, iv(mt, DOM_EMPTY, 0))

  pure def nodeH(mt: str, h: int, x: int, y: int): int =
    sponge2Iv(x, y, iv(mt, DOM_NODE, h))

  // Segment tree over SEGC = 4 leaves (heights 1, 2)
  pure def treeSeg(mt: str, leaves: List[int]): int = {
    val n0 = nodeH(mt, 1, leaves[0], leaves[1])
    val n1 = nodeH(mt, 1, leaves[2], leaves[3])
    nodeH(mt, 2, n0, n1)
  }

  // Canonical tail digest of `recs` (`len(recs) < SEGC`) (`dense.rs:113-119`)
  pure def tailD(mt: str, recs: List[Rec]): int = {
    val emptyLeaves: List[int] = []
    val leaves = range(0, SEGC).foldl(emptyLeaves, (acc, i) =>
      if (i < recs.length()) acc.append(recD(mt, recs[i]))
      else acc.append(padD(mt))
    )
    treeSeg(mt, leaves)
  }

  // Empty node at height `h >= 2` (`h = 2` is an empty segment slot `EMPTY_SEG`;
  // `h = 3, 4` are the top tree levels for `SEGC = 4, TOP = 2`).
  pure def emptyAt(mt: str, h: int): int = {
    val e2 = emptySeg(mt)
    val e3 = nodeH(mt, 3, e2, e2)
    val e4 = nodeH(mt, 4, e3, e3)
    if (h <= 2) e2
    else if (h == 3) e3
    else e4
  }

  // Top tree root over 4 segment slot digests `segDigests[0..3]` (heights 3, 4)
  pure def topRootFromSlots(mt: str, segDigests: List[int]): int = {
    val n0 = nodeH(mt, 3, segDigests[0], segDigests[1])
    val n1 = nodeH(mt, 3, segDigests[2], segDigests[3])
    nodeH(mt, 4, n0, n1)
  }

  // ---------------------------------------------------------------- Reference Model (`DenseModel`, `dense.rs:131-306`)
  type ArenaState = {
    len: int,
    root: int,
    tailDigest: int
  }

  type RefArena = {
    allRecs: List[Rec],
    segSlots: List[int], // 4 segment slot digests
    tailRecs: List[Rec]  // < 4 uncommitted tail records
  }

  pure def initRefArena(mt: str): RefArena = {
    val e2 = emptySeg(mt)
    {
      allRecs: [],
      segSlots: [e2, e2, e2, e2],
      tailRecs: []
    }
  }

  pure def refPushOne(mt: str, m: RefArena, r: Rec): RefArena = {
    val newAll = m.allRecs.append(r)
    val nextTail = m.tailRecs.append(r)
    if (nextTail.length() == SEGC) {
      val q = m.allRecs.length() / SEGC
      val emptyL: List[int] = []
      val leaves = range(0, SEGC).foldl(emptyL, (acc, i) => acc.append(recD(mt, nextTail[i])))
      val segRoot = treeSeg(mt, leaves)
      val newSlots =
        if (q >= 0 and q < TOP_SLOTS) m.segSlots.replaceAt(q, segRoot)
        else m.segSlots
      { allRecs: newAll, segSlots: newSlots, tailRecs: [] }
    } else {
      { allRecs: newAll, segSlots: m.segSlots, tailRecs: nextTail }
    }
  }

  pure def refAppendList(mt: str, m: RefArena, recs: List[Rec]): RefArena =
    range(0, recs.length()).foldl(m, (acc, i) => refPushOne(mt, acc, recs[i]))

  pure def refArenaState(mt: str, m: RefArena): ArenaState = {
    len: m.allRecs.length(),
    root: topRootFromSlots(mt, m.segSlots),
    tailDigest: tailD(mt, m.tailRecs)
  }

  // Siblings for top tree slot `q` (`0..3`, heights 3 and 4)
  pure def refSibsTop(mt: str, segSlots: List[int], q: int): List[int] = {
    val qClamped = if (q >= 0 and q < TOP_SLOTS) q else 0
    val b0 = qClamped % 2
    val b1 = (qClamped / 2) % 2
    val sib0 = if (b0 == 0) segSlots[qClamped + 1] else segSlots[qClamped - 1]
    val n0 = nodeH(mt, 3, segSlots[0], segSlots[1])
    val n1 = nodeH(mt, 3, segSlots[2], segSlots[3])
    val sib1 = if (b1 == 0) n1 else n0
    [sib0, sib1]
  }

  // Siblings for cold read of record at `addr` (`0..15`, heights 1, 2, 3, 4)
  pure def refSibsCold(mt: str, m: RefArena, addr: int): List[int] = {
    val q = addr / SEGC
    val r = addr % SEGC
    val base = q * SEGC
    val emptyL: List[int] = []
    val l0 = range(0, SEGC).foldl(emptyL, (acc, i) =>
      if (base + i < m.allRecs.length()) acc.append(recD(mt, m.allRecs[base + i]))
      else acc.append(padD(mt))
    )
    val sib1 = if (r % 2 == 0) l0[r + 1] else l0[r - 1]
    val n0 = nodeH(mt, 1, l0[0], l0[1])
    val n1 = nodeH(mt, 1, l0[2], l0[3])
    val sib2 = if ((r / 2) % 2 == 0) n1 else n0
    val topSibs = refSibsTop(mt, m.segSlots, q)
    [sib1, sib2, topSibs[0], topSibs[1]]
  }

  // ---------------------------------------------------------------- Circuit Row & Witness Types
  // Bits decomposition chain (`WindowCfg::bits`, `membackend.rs:330-357`)
  type BitRowW = {
    b: int,
    accIn: int,
    accOut: int
  }

  pure def buildBits(v: int, nb: int): List[BitRowW] = {
    val emptyB: List[BitRowW] = []
    val res = range(0, nb).foldl({ cur: v, rows: emptyB }, (st, _) => {
      val bit = if (st.cur >= 0) st.cur % 2 else 0
      val nextAcc = if (st.cur >= 0) st.cur / 2 else -1
      {
        cur: nextAcc,
        rows: st.rows.append({ b: bit, accIn: st.cur, accOut: nextAcc })
      }
    })
    res.rows
  }

  // Window row (`WindowCfg::window`, `membackend.rs:248-305, 397-450`)
  type WinRowCircuit = {
    bus: BusTuple,
    addr: int,
    kind: int,
    act: int,
    imp: int,
    lenPost: int,
    lenPre: int,
    qPre: int,
    kfix: int
  }

  // One level of a Merkle walk (`DenseCfg::walk`, `dense.rs:393-425, 548-617`)
  type WalkLevelW = {
    in0: int,
    in1: int,
    x: int,
    s: int,
    b: int,
    accIn: int,
    accOut: int,
    out: int
  }

  // One flush unit (`DenseCfg::persist`, `dense.rs:676-704`)
  type FlushUnitCircuit = {
    f: int,
    idx: int,
    oldLevels: List[WalkLevelW],
    newLevels: List[WalkLevelW],
    oldRoot: int,
    newRoot: int,
    rin: int,
    rout: int
  }

  // CORE append row pair (`AP` + `AQ` + `inc`, `dense/tests.rs:102-118, 193-211`)
  type AppendPortW = {
    addrIn: int,
    addrNext: int,
    kind: int,
    kfix: int,
    bkey: int,
    d0: int,
    d1: int,
    d2: int,
    req: BusTuple
  }

  // CORE read port + backend `cold_read` (`dense/tests.rs:119-150, 229-296`, `dense.rs:716-752`)
  type ReadPortW = {
    addr: int,
    atPost: bool,
    lenCur: int,
    base: int,
    cold: int,
    diff: int,
    d2: int,
    kind: int,
    kfix: int,
    bkey: int,
    d0: int,
    d1: int,
    d2w: int,
    diffBits: List[BitRowW],
    d2Bits: List[BitRowW],
    req: BusTuple,
    coldEnable: int,
    coldRecHash: int,
    coldLevels: List[WalkLevelW],
    coldRoot: int
  }

  // Full per-arena leaf circuit witness
  type ArenaLeafW = {
    kfix: int,
    wUnits: int,
    pre: ArenaState,
    post: ArenaState,
    lenPreBits: List[BitRowW],
    lenPostBits: List[BitRowW],
    winRows: List[WinRowCircuit],
    recHashes: List[int],
    mImp: List[int],
    tPre: int,
    mAct: List[int],
    ts: List[int],
    units: List[FlushUnitCircuit],
    tPost: int,
    appends: List[AppendPortW],
    appendEndAddr: int,
    reads: List[ReadPortW]
  }

  // 2-Leaf Chain Witness for testing cross-leaf boundary, reads, flushes, and 4-arena isolation
  type ChainW = {
    scenId: int,
    leaf1: ArenaLeafW,
    leaf2: ArenaLeafW,
    // Second arena for 4-arena cross-kind test (`kfix = KIND_FIELD`)
    hasArena2: bool,
    arena2Leaf: ArenaLeafW,
    // Expected reference values
    refPost1: ArenaState,
    refPost2: ArenaState,
    refReadVals1: List[Rec],
    refReadVals2: List[Rec],
    refArena2Post: ArenaState
  }

  // ---------------------------------------------------------------- Walk Builder (`DenseCfg::walk`)
  pure def buildWalkLevels(
    mt: str,
    x0: int,
    h0: int,
    idx: int,
    sibs: List[int]
  ): List[WalkLevelW] = {
    val n = sibs.length()
    val bRows = buildBits(idx, n)
    val emptyLvls: List[WalkLevelW] = []
    val res = range(0, n).foldl({ curX: x0, lvls: emptyLvls }, (st, l) => {
      val br = bRows[l]
      val s = sibs[l]
      val b = br.b
      val in0 = st.curX + b * (s - st.curX)
      val in1 = s + b * (st.curX - s)
      val out = nodeH(mt, h0 + l, in0, in1)
      val lvl: WalkLevelW = {
        in0: in0,
        in1: in1,
        x: st.curX,
        s: s,
        b: b,
        accIn: br.accIn,
        accOut: br.accOut,
        out: out
      }
      { curX: out, lvls: st.lvls.append(lvl) }
    })
    res.lvls
  }

  // ---------------------------------------------------------------- Honest Arena Leaf Builder
  pure def buildHonestArenaLeaf(
    mt: str,
    refPre: RefArena,
    kfix: int,
    wUnits: int,
    appRecs: List[Rec],
    readSpecs: List[{ addr: int, atPost: bool }]
  ): { leaf: ArenaLeafW, refPost: RefArena, expectedReads: List[Rec] } = {
    val preSt = refArenaState(mt, refPre)
    val refPost = refAppendList(mt, refPre, appRecs)
    val postSt = refArenaState(mt, refPost)
    val qPre = preSt.len / SEGC
    val rPre = preSt.len % SEGC
    val base = qPre * SEGC
    val nw = SEGC * (wUnits + 1)

    val lenPreBits = buildBits(preSt.len, NB)
    val lenPostBits = buildBits(postSt.len, NB)

    // Window rows
    val emptyWin: List[WinRowCircuit] = []
    val winRows = range(0, nw).foldl(emptyWin, (acc, i) => {
      val addr = base + i
      val imp = if (i < rPre) 1 else 0
      val act = if (i < rPre + appRecs.length()) 1 else 0
      val r =
        if (i < rPre) refPre.tailRecs[i]
        else if (i < rPre + appRecs.length()) appRecs[i - rPre]
        else ZERO_REC
      val bus: BusTuple =
        if (act == 1) { chip: CHIP_MEM, bkey: busKey(addr, r.kind), d0: r.d0, d1: r.d1, d2: r.d2 }
        else ZERO_TUPLE
      acc.append({
        bus: bus,
        addr: addr,
        kind: r.kind,
        act: act,
        imp: imp,
        lenPost: postSt.len,
        lenPre: preSt.len,
        qPre: if (i == 0) qPre else 0,
        kfix: kfix
      })
    })

    // Record hashes (last row is not hashed, set to 0 as in `dense.rs:647-649`)
    val emptyH: List[int] = []
    val recHashes = range(0, nw).foldl(emptyH, (acc, i) =>
      if (i + 1 == nw) acc.append(0)
      else {
        val rw = winRows[i]
        acc.append(recD(mt, { kind: rw.kind, d0: rw.bus.d0, d1: rw.bus.d1, d2: rw.bus.d2 }))
      }
    )

    // Pre tail mask and treeSeg
    val mImp = range(0, SEGC).foldl(emptyH, (acc, i) => {
      val rw = winRows[i]
      val h = recHashes[i]
      acc.append(padD(mt) + rw.imp * (h - padD(mt)))
    })
    val tPre = treeSeg(mt, mImp)

    // Active mask and window segment trees `ts[0..=wUnits]`
    val mAct = range(0, nw).foldl(emptyH, (acc, i) => {
      val rw = winRows[i]
      val h = recHashes[i]
      acc.append(padD(mt) + rw.act * (h - padD(mt)))
    })
    val ts = range(0, wUnits + 1).foldl(emptyH, (acc, j) => {
      val segLeaves = range(0, SEGC).foldl(emptyH, (lAcc, k) => lAcc.append(mAct[SEGC * j + k]))
      acc.append(treeSeg(mt, segLeaves))
    })

    // Flush units (`dense.rs:675-704`)
    val emptyUnits: List[FlushUnitCircuit] = []
    val flushFold = range(0, wUnits).foldl(
      { curRoot: preSt.root, curIdx: qPre, curSlots: refPre.segSlots, units: emptyUnits },
      (st, j) => {
        val f = winRows[SEGC * j + SEGC - 1].act
        val idx =
          if (mt == "walk_idx_unconditional_inc") qPre + j
          else if (j == 0) st.curIdx
          else st.curIdx + f
        val sibs = refSibsTop(mt, st.curSlots, idx)
        val oldLvls = buildWalkLevels(mt, emptySeg(mt), 3, idx, sibs)
        val newLvls = buildWalkLevels(mt, ts[j], 3, idx, sibs)
        val oldR = oldLvls[oldLvls.length() - 1].out
        val newR = newLvls[newLvls.length() - 1].out
        val rout = st.curRoot + f * (newR - st.curRoot)
        val nextSlots =
          if (f == 1 and idx >= 0 and idx < TOP_SLOTS) st.curSlots.replaceAt(idx, ts[j])
          else st.curSlots
        val u: FlushUnitCircuit = {
          f: f,
          idx: idx,
          oldLevels: oldLvls,
          newLevels: newLvls,
          oldRoot: oldR,
          newRoot: newR,
          rin: st.curRoot,
          rout: rout
        }
        { curRoot: rout, curIdx: idx, curSlots: nextSlots, units: st.units.append(u) }
      }
    )

    // Post tail mux chain (`dense.rs:706-710`)
    val tPost = range(0, wUnits).foldl(ts[0], (curT, j) => {
      val f = winRows[SEGC * j + SEGC - 1].act
      curT + f * (ts[j + 1] - curT)
    })

    // CORE append chain (`dense/tests.rs:193-211`)
    val emptyApps: List[AppendPortW] = []
    val appsFold = range(0, appRecs.length()).foldl(
      { curAddr: preSt.len, apps: emptyApps },
      (st, j) => {
        val r = appRecs[j]
        val bk = busKey(st.curAddr, r.kind)
        val ap: AppendPortW = {
          addrIn: st.curAddr,
          addrNext: st.curAddr + 1,
          kind: r.kind,
          kfix: r.kind,
          bkey: bk,
          d0: r.d0,
          d1: r.d1,
          d2: r.d2,
          req: { chip: CHIP_MEM, bkey: bk, d0: r.d0, d1: r.d1, d2: r.d2 }
        }
        { curAddr: st.curAddr + 1, apps: st.apps.append(ap) }
      }
    )

    // CORE read ports (`dense/tests.rs:229-296`)
    val emptyReads: List[ReadPortW] = []
    val emptyRecs: List[Rec] = []
    val readsFold = range(0, readSpecs.length()).foldl(
      { rds: emptyReads, exp: emptyRecs },
      (st, ri) => {
        val sp = readSpecs[ri]
        val lc = if (sp.atPost) postSt.len else preSt.len
        val isCold = sp.addr < base
        val coldf = if (isCold) 1 else 0
        val r =
          if (sp.addr >= 0 and sp.addr < refPost.allRecs.length()) refPost.allRecs[sp.addr]
          else { kind: KIND_DATA, d0: 0, d1: 0, d2: 0 }
        val diff = if (isCold) base - 1 - sp.addr else sp.addr - base
        val d2 = lc - 1 - sp.addr
        val bk = busKey(sp.addr, r.kind)
        val reqTuple =
          if (isCold) ZERO_TUPLE
          else { chip: CHIP_MEM, bkey: bk, d0: r.d0, d1: r.d1, d2: r.d2 }
        val sibs =
          if (isCold) refSibsCold(mt, refPre, sp.addr)
          else [0, 0, 0, 0]
        val rHash = recD(mt, r)
        val cLvls = buildWalkLevels(mt, rHash, 1, sp.addr, sibs)
        val cRoot = cLvls[cLvls.length() - 1].out
        val rp: ReadPortW = {
          addr: sp.addr,
          atPost: sp.atPost,
          lenCur: lc,
          base: base,
          cold: coldf,
          diff: diff,
          d2: d2,
          kind: r.kind,
          kfix: r.kind,
          bkey: bk,
          d0: r.d0,
          d1: r.d1,
          d2w: r.d2,
          diffBits: buildBits(diff, NB),
          d2Bits: buildBits(d2, NB),
          req: reqTuple,
          coldEnable: coldf,
          coldRecHash: rHash,
          coldLevels: cLvls,
          coldRoot: cRoot
        }
        { rds: st.rds.append(rp), exp: st.exp.append(r) }
      }
    )

    val finalPost: ArenaState = {
      len: postSt.len,
      root: flushFold.curRoot,
      tailDigest: tPost
    }

    {
      leaf: {
        kfix: kfix,
        wUnits: wUnits,
        pre: preSt,
        post: finalPost,
        lenPreBits: lenPreBits,
        lenPostBits: lenPostBits,
        winRows: winRows,
        recHashes: recHashes,
        mImp: mImp,
        tPre: tPre,
        mAct: mAct,
        ts: ts,
        units: flushFold.units,
        tPost: tPost,
        appends: appsFold.apps,
        appendEndAddr: appsFold.curAddr,
        reads: readsFold.rds
      },
      refPost: refPost,
      expectedReads: readsFold.exp
    }
  }

  // ---------------------------------------------------------------- Circuit Gates (`membackend.rs`, `dense.rs`, `dense/tests.rs`)

  // `"mem bits step"` and `"mem bits last"` (`membackend.rs:306-321`)
  pure def checkBits(mt: str, rows: List[BitRowW], inputVal: int): bool = {
    val n = rows.length()
    and {
      n == NB,
      rows[0].accIn == inputVal,
      rows.indices().forall(i => {
        val r = rows[i]
        and {
          r.b == 0 or r.b == 1,
          r.accIn == 2 * r.accOut + r.b,
          if (i + 1 < n) rows[i + 1].accIn == r.accOut
          else (if (mt == "counter_wrap_2_64") true else r.accOut == 0)
        }
      })
    }
  }

  // `"mem win row"`, `"mem win first"`, `"mem win chain"`, `"mem win last"` (`membackend.rs:248-305`)
  pure def checkWindow(mt: str, lf: ArenaLeafW): bool = {
    val nw = lf.winRows.length()
    val qPreCell = lf.lenPreBits[SEG_BITS].accIn
    and {
      nw == SEGC * (lf.wUnits + 1),
      checkBits(mt, lf.lenPreBits, lf.pre.len),
      checkBits(mt, lf.lenPostBits, lf.post.len),
      // Row 0 copies (`membackend.rs:425-429`)
      lf.winRows[0].lenPost == lf.post.len,
      lf.winRows[0].lenPre == lf.pre.len,
      lf.winRows[0].qPre == qPreCell,
      // `"mem win first"` (`membackend.rs:282-297`)
      lf.winRows[0].addr == lf.winRows[0].qPre * SEGC,
      (1 - lf.winRows[0].act) * (lf.winRows[0].addr - lf.winRows[0].lenPost) == 0,
      (1 - lf.winRows[0].imp) * (lf.winRows[0].addr - lf.winRows[0].lenPre) == 0,
      // `"mem win last"` (`membackend.rs:302-305`)
      lf.winRows[nw - 1].act == 0,
      // Per-row gate `"mem win row"` (`membackend.rs:248-268`)
      lf.winRows.indices().forall(i => {
        val rw = lf.winRows[i]
        val na = 1 - rw.act
        and {
          rw.act == 0 or rw.act == 1,
          rw.imp == 0 or rw.imp == 1,
          rw.imp * na == 0,
          if (mt == "pad_row_on_bus") true else rw.bus.chip == rw.act * CHIP_MEM,
          if (mt == "pad_row_on_bus") true else rw.bus.bkey == rw.act * busKey(rw.addr, rw.kind),
          na * rw.kind == 0,
          if (mt == "pad_row_on_bus") true else (na * rw.bus.d0 == 0 and na * rw.bus.d1 == 0 and na * rw.bus.d2 == 0),
          if (mt == "kind_out_of_range") true else (rw.kind >= 0 and rw.kind <= 4),
          if (mt == "arena_kind_pinned") true else (rw.act * rw.kfix * (rw.kind - rw.kfix) == 0),
          rw.kfix == lf.kfix
        }
      }),
      // Chain gate `"mem win first"` + `"mem win chain"` (`membackend.rs:269-281`)
      0.to(nw - 2).forall(i => {
        val cur = lf.winRows[i]
        val nxt = lf.winRows[i + 1]
        and {
          if (mt == "address_reuse_window_row") true else (nxt.addr == cur.addr + 1),
          nxt.lenPost == cur.lenPost,
          nxt.lenPre == cur.lenPre,
          nxt.act * (1 - cur.act) == 0,
          nxt.imp * (1 - cur.imp) == 0,
          if (mt == "close_advances_len") true else (cur.act * (1 - nxt.act) * (cur.addr + 1 - cur.lenPost) == 0),
          cur.imp * (1 - nxt.imp) * (cur.addr + 1 - cur.lenPre) == 0
        }
      })
    }
  }

  // `"dense walk"`, `"dense walk empty"`, `"dense walk last"` (`dense.rs:393-425, 548-617`)
  pure def checkWalk(
    mt: str,
    lvls: List[WalkLevelW],
    x0: int,
    checkX0: bool,
    h0: int,
    idx: int,
    isEmptyFirst: bool
  ): bool = {
    val n = lvls.length()
    and {
      n >= 1,
      if (checkX0) lvls[0].x == x0 else true,
      lvls[0].accIn == idx,
      lvls.indices().forall(l => {
        val w = lvls[l]
        // Exact selector priority from `dense.rs:593-599`:
        // `if l + 1 == n { WALK_LAST } else if l == 0 && first == First::Empty { WALK_EMPTY } else { WALK }`
        // Under `top1_walk_empty_shadowed` (or when `n == 1`), `isLast` is true at `l == 0`,
        // which shadows `WALK_EMPTY`!
        val isLast = (l + 1 == n)
        val isWalkEmpty = (not(isLast) and not(mt == "top1_walk_empty_shadowed") and l == 0 and isEmptyFirst)
        and {
          w.b == 0 or w.b == 1,
          w.in0 == w.x + w.b * (w.s - w.x),
          w.in1 == w.s + w.b * (w.x - w.s),
          w.accIn == 2 * w.accOut + w.b,
          w.out == nodeH(mt, h0 + l, w.in0, w.in1),
          if (l > 0) (w.x == lvls[l - 1].out and w.accIn == lvls[l - 1].accOut) else true,
          if (isWalkEmpty and mt != "empty_seg_forgery") w.x == emptySeg(mt) else true,
          if (isLast) w.accOut == 0 else true
        }
      })
    }
  }

  // `DenseCfg::persist` (`dense.rs:624-714`)
  pure def checkPersist(mt: str, lf: ArenaLeafW): bool = {
    val nw = lf.winRows.length()
    val qPreCell = lf.lenPreBits[SEG_BITS].accIn
    and {
      lf.recHashes.length() == nw,
      0.to(nw - 2).forall(i => {
        val rw = lf.winRows[i]
        lf.recHashes[i] == recD(mt, { kind: rw.kind, d0: rw.bus.d0, d1: rw.bus.d1, d2: rw.bus.d2 })
      }),
      // Pre tail mask (`"dense mask"`) and `treeSeg`
      lf.mImp.length() == SEGC,
      lf.mImp.indices().forall(i =>
        lf.mImp[i] == padD(mt) + lf.winRows[i].imp * (lf.recHashes[i] - padD(mt))
      ),
      lf.tPre == treeSeg(mt, lf.mImp),
      // Tail opening equality (`dense.rs:661`)
      if (mt == "forged_tail_across_leaf_boundary") true else (lf.tPre == lf.pre.tailDigest),
      // Active masks and window segments `ts[0..=wUnits]`
      lf.mAct.length() == nw,
      lf.mAct.indices().forall(i =>
        if (mt == "non_canonical_tail") true
        else lf.mAct[i] == padD(mt) + lf.winRows[i].act * (lf.recHashes[i] - padD(mt))
      ),
      lf.ts.length() == lf.wUnits + 1,
      lf.ts.indices().forall(j => {
        val emptyL: List[int] = []
        val segLeaves = range(0, SEGC).foldl(emptyL, (acc, k) => acc.append(lf.mAct[SEGC * j + k]))
        lf.ts[j] == treeSeg(mt, segLeaves)
      }),
      // Flush units (`dense.rs:675-704`)
      lf.units.length() == lf.wUnits,
      lf.units.indices().forall(j => {
        val u = lf.units[j]
        val fRow = lf.winRows[SEGC * j + SEGC - 1].act
        val expIdx =
          if (mt == "walk_idx_unconditional_inc") qPreCell + j
          else if (j == 0) qPreCell
          else lf.units[j - 1].idx + fRow
        val expRin = if (j == 0) lf.pre.root else lf.units[j - 1].rout
        and {
          u.f == fRow,
          u.idx == expIdx,
          u.rin == expRin,
          u.oldLevels.length() == TOP,
          u.newLevels.length() == TOP,
          checkWalk(mt, u.oldLevels, emptySeg(mt), false, 3, u.idx, true),
          checkWalk(mt, u.newLevels, lf.ts[j], true, 3, u.idx, false),
          u.oldRoot == u.oldLevels[TOP - 1].out,
          u.newRoot == u.newLevels[TOP - 1].out,
          // Shared `[s, b, acc_in, acc_out]` copies (`dense.rs:572-579`)
          if (mt == "new_walk_unshared_sibs") true
          else u.oldLevels.indices().forall(l => {
            val ol = u.oldLevels[l]
            val nl = u.newLevels[l]
            nl.s == ol.s and nl.b == ol.b and nl.accIn == ol.accIn and nl.accOut == ol.accOut
          }),
          // `"dense flush"` gate (`dense.rs:376-384`)
          u.f == 0 or u.f == 1,
          if (mt == "address_reuse_occupied_slot") true else (u.f * (u.oldRoot - u.rin) == 0),
          if (mt == "flush_skipped" and u.f == 1) true
          else if (mt == "flush_spurious" and u.f == 0) true
          else (u.rout == u.rin + u.f * (u.newRoot - u.rin))
        }
      }),
      lf.post.root == lf.units[lf.wUnits - 1].rout,
      // Post-tail mux chain (`dense.rs:706-713`)
      lf.tPost == range(0, lf.wUnits).foldl(lf.ts[0], (curT, j) => {
        val f = lf.winRows[SEGC * j + SEGC - 1].act
        curT + f * (lf.ts[j + 1] - curT)
      }),
      lf.post.tailDigest == lf.tPost
    }
  }

  // Bus lookup (`bus.rs:50-61`): every request tuple must match a provider tuple in `allWinRows`
  // (or `[0, 0, 0, 0, 0]`, which is provided by the inactive last row of every window).
  pure def busLookupHolds(req: BusTuple, allWinRows: List[WinRowCircuit]): bool =
    allWinRows.indices().exists(i => allWinRows[i].bus == req)

  // CORE append & read ports + `cold_read` (`dense/tests.rs:102-150, 193-296`, `dense.rs:716-752`)
  pure def checkCoreAndReads(mt: str, lf: ArenaLeafW, allWinRows: List[WinRowCircuit]): bool = {
    val baseCell = lf.winRows[0].addr
    and {
      // Append chain (`AP`, `AQ`, `inc`, `dense/tests.rs:193-211`)
      if (lf.appends.length() == 0) (lf.appendEndAddr == lf.pre.len)
      else and {
        lf.appends[0].addrIn == lf.pre.len,
        lf.appends[lf.appends.length() - 1].addrNext == lf.appendEndAddr,
        lf.appends.indices().forall(j => {
          val ap = lf.appends[j]
          and {
            ap.addrNext == ap.addrIn + 1,
            if (j > 0) ap.addrIn == lf.appends[j - 1].addrNext else true,
            ap.bkey == busKey(ap.addrIn, ap.kind),
            if (mt == "arena_kind_pinned") true else (ap.kind == ap.kfix),
            if (lf.kfix != 0 and mt != "arena_kind_pinned") ap.kfix == lf.kfix else true,
            ap.req == { chip: CHIP_MEM, bkey: ap.bkey, d0: ap.d0, d1: ap.d1, d2: ap.d2 },
            if (mt == "append_value_mismatch") true else busLookupHolds(ap.req, allWinRows)
          }
        })
      },
      if (mt == "close_advances_len") true else (lf.appendEndAddr == lf.post.len),

      // Read ports + `cold_read` (`RD`, `RQ`, `dense.rs:716-752`)
      lf.reads.indices().forall(ri => {
        val rd = lf.reads[ri]
        val expLc = if (rd.atPost) lf.post.len else lf.pre.len
        val warm = 1 - rd.cold
        val rdRec: Rec = {
          kind: rd.kind,
          d0: rd.d0,
          d1: rd.d1,
          d2: rd.d2w
        }
        and {
          rd.lenCur == expLc,
          rd.base == baseCell,
          rd.cold == 0 or rd.cold == 1,
          rd.diff == rd.cold * (rd.base - 1 - rd.addr) + (1 - rd.cold) * (rd.addr - rd.base),
          rd.d2 == rd.lenCur - 1 - rd.addr,
          rd.bkey == busKey(rd.addr, rd.kind),
          if (mt == "wrong_kind_warm" or mt == "wrong_kind_cold") true else (rd.kind == rd.kfix),
          if (mt == "cold_route_forged") true else checkBits(mt, rd.diffBits, rd.diff),
          if (mt == "read_at_len" or mt == "read_beyond_len" or mt == "future_record_read") true
          else checkBits(mt, rd.d2Bits, rd.d2),
          // `"core read request"` (`RQ`)
          rd.req == {
            chip: warm * CHIP_MEM,
            bkey: warm * rd.bkey,
            d0: warm * rd.d0,
            d1: warm * rd.d1,
            d2: warm * rd.d2w
          },
          busLookupHolds(rd.req, allWinRows),
          // `DenseCfg::cold_read` (`dense.rs:716-752`)
          rd.coldEnable == rd.cold,
          rd.coldRecHash == recD(mt, rdRec),
          rd.coldLevels.length() == NB,
          checkWalk(mt, rd.coldLevels, rd.coldRecHash, true, 1, rd.addr, false),
          rd.coldRoot == rd.coldLevels[NB - 1].out,
          // `"dense geq"` gate (`dense.rs:385-392`)
          rd.coldEnable == 0 or rd.coldEnable == 1,
          if (mt == "cold_value_forged" or mt == "wrong_kind_cold") true
          else (rd.coldEnable * (rd.coldRoot - lf.pre.root) == 0)
        }
      })
    }
  }

  // Full acceptance check of a 2-leaf chain (plus optional second arena)
  pure def acceptsChain(mt: str, cw: ChainW): bool = {
    val allWin1 =
      if (cw.hasArena2) cw.leaf1.winRows.concat(cw.arena2Leaf.winRows)
      else cw.leaf1.winRows
    val allWin2 = cw.leaf2.winRows
    and {
      checkWindow(mt, cw.leaf1),
      checkPersist(mt, cw.leaf1),
      checkCoreAndReads(mt, cw.leaf1, allWin1),
      // Boundary stitch between Leaf 1 and Leaf 2
      cw.leaf2.pre == cw.leaf1.post,
      checkWindow(mt, cw.leaf2),
      checkPersist(mt, cw.leaf2),
      checkCoreAndReads(mt, cw.leaf2, allWin2),
      if (cw.hasArena2) and {
        checkWindow(mt, cw.arena2Leaf),
        checkPersist(mt, cw.arena2Leaf),
        checkCoreAndReads(mt, cw.arena2Leaf, allWin1)
      } else true
    }
  }

  // Reference refinement predicate:
  // 1. Both leaves' boundary states `(len, root, tailDigest)` match the canonical reference model;
  // 2. Every read port returns the exact record `[kind, d0, d1, d2]` at `addr < lenCur` with `kind == kfix`;
  // 3. `0 <= len <= MAX_LEN - 1` (no counter wrap);
  // 4. In 4-arena mode (`hasArena2`), Arena 2's boundary state also matches its canonical reference model.
  pure def refinesChain(cw: ChainW): bool = {
    and {
      cw.leaf1.post == cw.refPost1,
      cw.leaf2.post == cw.refPost2,
      cw.leaf1.pre.len >= 0 and cw.leaf1.pre.len < MAX_LEN,
      cw.leaf1.post.len >= 0 and cw.leaf1.post.len < MAX_LEN,
      cw.leaf2.post.len >= 0 and cw.leaf2.post.len < MAX_LEN,
      cw.leaf1.reads.length() == cw.refReadVals1.length(),
      cw.leaf1.reads.indices().forall(i => {
        val rd = cw.leaf1.reads[i]
        val exp = cw.refReadVals1[i]
        and {
          rd.addr >= 0 and rd.addr < rd.lenCur,
          rd.kind == rd.kfix,
          { kind: rd.kind, d0: rd.d0, d1: rd.d1, d2: rd.d2w } == exp
        }
      }),
      cw.leaf2.reads.length() == cw.refReadVals2.length(),
      cw.leaf2.reads.indices().forall(i => {
        val rd = cw.leaf2.reads[i]
        val exp = cw.refReadVals2[i]
        and {
          rd.addr >= 0 and rd.addr < rd.lenCur,
          rd.kind == rd.kfix,
          { kind: rd.kind, d0: rd.d0, d1: rd.d1, d2: rd.d2w } == exp
        }
      }),
      if (cw.hasArena2) cw.arena2Leaf.post == cw.refArena2Post else true
    }
  }

  // ---------------------------------------------------------------- Scenarios & Attack Builder
  pure def mkRec(kind: int, x: int): Rec =
    { kind: kind, d0: x, d1: x + 100, d2: 7 }

  // Base honest scenario:
  // Start with 5 records in `refPre1` (`len = 5`, `q_pre = 1`, `r_pre = 1`: 1 flushed segment at q=0, 1 tail record at addr=4).
  // Leaf 1 appends 3 records (addrs 5, 6, 7 -> crosses segment boundary `q = 1`, flushes segment 1, post `len = 8, r_post = 0`),
  // and reads:
  //   - read 0: cold read of `addr = 1` (`KIND_FIELD`, `atPost = true`)
  //   - read 1: warm tail read of `addr = 4` (`KIND_DATA`, `atPost = true`)
  //   - read 2: warm new-record read of `addr = 6` (`KIND_ENV`, `atPost = true`)
  // Leaf 2 appends 2 records (addrs 8, 9 -> no flush, `r_post = 2`), and reads `addr = 8` (`atPost = true`).
  pure def buildBaseHonest(mt: str): ChainW = {
    val r0 = initRefArena(mt)
    val initRecs = [
      mkRec(KIND_DATA, 10),
      mkRec(KIND_FIELD, 11),
      mkRec(KIND_ENV, 12),
      mkRec(KIND_SLOT, 13),
      mkRec(KIND_DATA, 14)
    ]
    val refPre1 = refAppendList(mt, r0, initRecs)
    val app1 = [mkRec(KIND_FIELD, 20), mkRec(KIND_ENV, 21), mkRec(KIND_SLOT, 22)]
    val rds1 = [
      { addr: 1, atPost: true },
      { addr: 4, atPost: true },
      { addr: 6, atPost: true }
    ]
    val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 1, app1, rds1)

    val app2 = [mkRec(KIND_DATA, 30), mkRec(KIND_FIELD, 31)]
    val rds2 = [{ addr: 8, atPost: true }]
    val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, app2, rds2)

    // Arena 2 (`kfix = KIND_FIELD`) for 4-arena scenario
    val refA2Pre = refAppendList(mt, r0, [mkRec(KIND_FIELD, 50)])
    val bA2 = buildHonestArenaLeaf(mt, refA2Pre, KIND_FIELD, 1, [mkRec(KIND_FIELD, 51)], [{ addr: 0, atPost: true }])

    {
      scenId: 0,
      leaf1: b1.leaf,
      leaf2: b2.leaf,
      hasArena2: false,
      arena2Leaf: bA2.leaf,
      refPost1: refArenaState(mt, b1.refPost),
      refPost2: refArenaState(mt, b2.refPost),
      refReadVals1: b1.expectedReads,
      refReadVals2: b2.expectedReads,
      refArena2Post: refArenaState(mt, bA2.refPost)
    }
  }

  // Honest scenario 1: Two flush units (`W = 2`), appending 6 records from `r_pre = 3` (`len = 3 -> 9`, two flushes!)
  // and Leaf 2 is a 0-append pass-through leaf (`len = 9 -> 9`).
  pure def buildTwoFlushHonest(mt: str): ChainW = {
    val r0 = initRefArena(mt)
    val refPre1 = refAppendList(mt, r0, [mkRec(KIND_DATA, 1), mkRec(KIND_FIELD, 2), mkRec(KIND_ENV, 3)])
    val app1 = [
      mkRec(KIND_SLOT, 4), mkRec(KIND_DATA, 5), mkRec(KIND_FIELD, 6),
      mkRec(KIND_ENV, 7), mkRec(KIND_SLOT, 8), mkRec(KIND_DATA, 9)
    ]
    val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 2, app1, [{ addr: 8, atPost: true }])
    // Leaf 2: pass-through (0 appends)
    val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [], [{ addr: 2, atPost: false }])
    {
      scenId: 1,
      leaf1: b1.leaf,
      leaf2: b2.leaf,
      hasArena2: false,
      arena2Leaf: b2.leaf,
      refPost1: refArenaState(mt, b1.refPost),
      refPost2: refArenaState(mt, b2.refPost),
      refReadVals1: b1.expectedReads,
      refReadVals2: b2.expectedReads,
      refArena2Post: refArenaState(mt, b2.refPost)
    }
  }

  // Honest scenario 2: Counter edge at the last segment (`q_pre = 3`, `len_pre = 13 -> len_post = 15 = MAX_LEN - 1`)
  // with `W = 2` flush units (`c1_last_segment_two_units` & `c1_counter_edge_2_64_minus_1`).
  pure def buildLastSegmentHonest(mt: str): ChainW = {
    val r0 = initRefArena(mt)
    val emptyR: List[Rec] = []
    val thirteenRecs = range(0, 13).foldl(emptyR, (acc, i) => acc.append(mkRec(1 + (i % 4), i + 1)))
    val refPre1 = refAppendList(mt, r0, thirteenRecs)
    val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 1, [mkRec(KIND_DATA, 90)], [])
    // Leaf 2 at `len_pre = 14` with `W = 2` flush units appending 1 record -> `len_post = 15`
    val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 2, [mkRec(KIND_FIELD, 91)], [{ addr: 14, atPost: true }])
    {
      scenId: 2,
      leaf1: b1.leaf,
      leaf2: b2.leaf,
      hasArena2: false,
      arena2Leaf: b1.leaf,
      refPost1: refArenaState(mt, b1.refPost),
      refPost2: refArenaState(mt, b2.refPost),
      refReadVals1: b1.expectedReads,
      refReadVals2: b2.expectedReads,
      refArena2Post: refArenaState(mt, b1.refPost)
    }
  }

  // Attack / Tamper constructor (`tKind in 0..23`):
  //   0: Honest base scenario
  //   1: `forged_tail_across_leaf_boundary` (Leaf 2 imports forged tail record at row 0 and recomputes post tail)
  //   2: `non_canonical_tail` (Leaf 2 masks non-pad digest `999` above `r_post` into `mAct[2]` / `tPost`)
  //   3: `flush_skipped` (Leaf 1 fills segment 1 (`f = 1`) but keeps `pre.root`)
  //   4: `flush_spurious` (Leaf 2 crosses no boundary (`f = 0`) but changes `post.root`)
  //   5: `address_reuse_occupied_slot` (Leaf 1 flushes into already-occupied slot `q = 1`, `oldRoot != rin`)
  //   6: `empty_seg_forgery` (as 5, with old walk started at the occupying segment root so `oldRoot == rin`)
  //   7: `address_reuse_window_row` (Middle imported tail row `i = 1` reuses `addr = 4` and serves forged `d0 = 999` to read)
  //   8: `read_at_len` (Read of `addr = lenCur`)
  //   9: `read_beyond_len` (Read of `addr = lenCur + 1`)
  //  10: `future_record_read` (Read at `lenCur = lenPre` of record appended in this leaf)
  //  11: `wrong_kind_warm` (Warm read claims `KIND_SLOT` on a `KIND_DATA` window record)
  //  12: `wrong_kind_cold` (Cold read claims `KIND_ENV` on a `KIND_FIELD` old record)
  //  13: `cold_value_forged` (Cold read claims forged data word `d0 = 999`)
  //  14: `cold_route_forged` (Window address `addr = 4 >= base` routed to cold path, returning stale value `d0 = 777`)
  //  15: `counter_wrap_2_64` (Leaf at `lenPre = 15 = MAX_LEN - 1` appends 1 record -> `lenPost = 16 = MAX_LEN`)
  //  16: `pad_row_on_bus` (Inactive padding window row offers `chip = CHIP_MEM` with forged record read by CORE)
  //  17: `kind_out_of_range` (Window rows at `addrs 8, 9` use `kind = 9` and `kind = -7` to swap bus keys `8*8+9 = 73` and `8*9-7 = 65`)
  //  18: `arena_kind_pinned` (In 4-arena mode, Arena 2 with `kfix = KIND_FIELD` writes a `KIND_DATA` record read by Arena 1)
  //  19: `close_advances_len` (`post.len = 11` is 1 higher than CORE's 2 appends ending at `appendEndAddr = 10`)
  //  20: `append_value_mismatch` (Window row holds forged `d0 = 999` while CORE append requests `d0 = 30`)
  //  21: `new_walk_unshared_sibs` (New walk uses forged sibling `s + 1` in flush unit 0)
  //  22: `domain_tag_dropped` (Without domain/height tags, `padD == emptySeg` and `nodeH(1..2) == nodeH(3..4)`, so a height-2 tail digest opens as a height-4 top tree)
  //  23: `top1_walk_empty_shadowed` (`TOP == 1` bug in `dense.rs:593-599`: `WALK_LAST` shadows `WALK_EMPTY` at `l = 0`, allowing `empty_seg_forgery`)
  pure def buildTampered(mt: str, tKind: int): ChainW = {
    val base = buildBaseHonest(mt)
    if (tKind == 0) {
      base
    } else if (tKind == 1) {
      // 1. `forged_tail_across_leaf_boundary`:
      // Build a chain where Leaf 1 ends with `len = 6` (`r_post = 2`), and Leaf 2 imports a forged tail record at row 0.
      val r0 = initRefArena(mt)
      val refPre1 = refAppendList(mt, r0, [mkRec(KIND_DATA, 1), mkRec(KIND_FIELD, 2), mkRec(KIND_ENV, 3), mkRec(KIND_SLOT, 4)])
      val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 1, [mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11)], [])
      val forgedRefPre2: RefArena = {
        ...b1.refPost,
        tailRecs: [mkRec(KIND_DATA, 999), mkRec(KIND_FIELD, 11)]
      }
      val b2Forged = buildHonestArenaLeaf(mt, forgedRefPre2, 0, 1, [mkRec(KIND_ENV, 12)], [])
      val b2Honest = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [mkRec(KIND_ENV, 12)], [])
      val lf2: ArenaLeafW = { ...b2Forged.leaf, pre: b1.leaf.post }
      {
        ...base,
        leaf1: b1.leaf,
        leaf2: lf2,
        refPost1: refArenaState(mt, b1.refPost),
        refPost2: refArenaState(mt, b2Honest.refPost),
        refReadVals1: [],
        refReadVals2: []
      }
    } else if (tKind == 2) {
      // 2. `non_canonical_tail`: Leaf 2's `mAct[2]` (inactive row above `r_post = 2`) uses non-pad digest `999` instead of `padD`,
      // while `mImp` and `tPre` stay honest so `tPre == pre.tailDigest` holds.
      val lf2 = base.leaf2
      val badMAct = lf2.mAct.replaceAt(2, 999)
      val badTs0 = treeSeg(mt, [badMAct[0], badMAct[1], badMAct[2], badMAct[3]])
      val badTs = lf2.ts.replaceAt(0, badTs0)
      val lf2Mut: ArenaLeafW = {
        ...lf2,
        mAct: badMAct,
        ts: badTs,
        tPost: badTs0,
        post: { ...lf2.post, tailDigest: badTs0 }
      }
      { ...base, leaf2: lf2Mut }
    } else if (tKind == 3) {
      // 3. `flush_skipped`: Leaf 1 crosses segment boundary (`f = 1`) but keeps `rout = pre.root`
      val lf1 = base.leaf1
      val u0 = { ...lf1.units[0], rout: lf1.pre.root }
      val lf1Mut: ArenaLeafW = {
        ...lf1,
        units: [u0],
        post: { ...lf1.post, root: lf1.pre.root }
      }
      val lf2Mut: ArenaLeafW = {
        ...base.leaf2,
        pre: lf1Mut.post,
        units: [{ ...base.leaf2.units[0], rin: lf1.pre.root, rout: lf1.pre.root }],
        post: { ...base.leaf2.post, root: lf1.pre.root }
      }
      { ...base, leaf1: lf1Mut, leaf2: lf2Mut }
    } else if (tKind == 4) {
      // 4. `flush_spurious`: Leaf 2 crosses no boundary (`f = 0`) but sets `rout = 77777`
      val lf2 = base.leaf2
      val u0 = { ...lf2.units[0], rout: 77777 }
      val lf2Mut: ArenaLeafW = {
        ...lf2,
        units: [u0],
        post: { ...lf2.post, root: 77777 }
      }
      { ...base, leaf2: lf2Mut }
    } else if (tKind == 5 or tKind == 6 or tKind == 23) {
      // 5 (`address_reuse_occupied_slot`), 6 (`empty_seg_forgery`), and 23 (`top1_walk_empty_shadowed`):
      // Pre-state already has occupying segments at `q = 1` and `q = 2`; Leaf 1 flushes into `q = 1`.
      val r0 = initRefArena(mt)
      val initRecs = [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11),
        mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14)
      ]
      val refPre1 = refAppendList(mt, r0, initRecs)
      val occRoot = treeSeg(mt, [recD(mt, mkRec(1, 90)), recD(mt, mkRec(1, 91)), recD(mt, mkRec(1, 92)), recD(mt, mkRec(1, 93))])
      val occupiedPre: RefArena = {
        ...refPre1,
        segSlots: refPre1.segSlots.replaceAt(1, occRoot).replaceAt(2, occRoot)
      }
      val app1 = [mkRec(KIND_FIELD, 20), mkRec(KIND_ENV, 21), mkRec(KIND_SLOT, 22)]
      val b1 = buildHonestArenaLeaf(mt, occupiedPre, 0, 1, app1, [])
      val u0Orig = b1.leaf.units[0]
      val sibs = refSibsTop(mt, occupiedPre.segSlots, 1)
      // For `tKind == 6` (`empty_seg_forgery`) or `tKind == 23` (`top1_walk_empty_shadowed`),
      // start old walk at `occRoot` instead of `emptySeg` so `oldRoot == rin`!
      val forgedOldLvls =
        if (tKind == 6 or tKind == 23) buildWalkLevels(mt, occRoot, 3, 1, sibs)
        else u0Orig.oldLevels
      val u0Mut: FlushUnitCircuit = {
        ...u0Orig,
        oldLevels: forgedOldLvls,
        oldRoot: forgedOldLvls[TOP - 1].out
      }
      val lf1Mut: ArenaLeafW = { ...b1.leaf, units: [u0Mut] }
      val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [], [])
      val canonPost = refArenaState(mt, refAppendList(mt, refPre1, app1))
      {
        ...base,
        leaf1: lf1Mut,
        leaf2: b2.leaf,
        refPost1: canonPost,
        refPost2: canonPost,
        refReadVals1: [],
        refReadVals2: []
      }
    } else if (tKind == 7) {
      // 7. `address_reuse_window_row`:
      // Start with `len_pre = 7` (`q_pre = 1, r_pre = 3`, imported tail rows `i = 0, 1, 2` at `addrs 4, 5, 6`),
      // and 1 append at `addr = 7` (`len_post = 8`).
      // Interior imported tail row `i = 1` (`addr = 5`, holding `mkRec(KIND_DATA, 999)`) sets `addr = 4`
      // (reusing row 0's `addr = 4`!), while row 0 (`addr = 4`) and row 2 (`addr = 6`, `imp` 1->0 boundary)
      // keep their addresses and `tPre == pre.tailDigest` still holds!
      // Read 0 of `addr = 4` is served `d0 = 999` from row 1 instead of `d0 = 14` from row 0.
      val r0 = initRefArena(mt)
      val refPre1 = refAppendList(mt, r0, [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11),
        mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14), mkRec(KIND_DATA, 999), mkRec(KIND_ENV, 16)
      ])
      val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 1, [mkRec(KIND_SLOT, 20)], [{ addr: 4, atPost: true }])
      val lf = b1.leaf
      val rw1: WinRowCircuit = {
        ...lf.winRows[1],
        addr: 4,
        bus: { ...lf.winRows[1].bus, bkey: busKey(4, KIND_DATA) }
      }
      val winMut = lf.winRows.replaceAt(1, rw1)
      val rd0: ReadPortW = {
        ...lf.reads[0],
        d0: 999,
        d1: 1099,
        d2w: 7,
        req: { chip: CHIP_MEM, bkey: busKey(4, KIND_DATA), d0: 999, d1: 1099, d2: 7 }
      }
      val lfMut: ArenaLeafW = { ...lf, winRows: winMut, reads: [rd0] }
      val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [], [])
      {
        ...base,
        leaf1: lfMut,
        leaf2: b2.leaf,
        refPost1: lfMut.post,
        refPost2: b2.leaf.post,
        refReadVals1: b1.expectedReads,
        refReadVals2: []
      }
    } else if (tKind == 8 or tKind == 9 or tKind == 10) {
      // 8 (`read_at_len`: addr = 5 == lenCur), 9 (`read_beyond_len`: addr = 6 > lenCur),
      // 10 (`future_record_read`: addr = 5 at `atPost = false`, so `lenCur = lenPre = 5`)
      val addrUsed = if (tKind == 8) 5 else if (tKind == 9) 6 else 5
      val r0 = initRefArena(mt)
      val refPre1 = refAppendList(mt, r0, [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11),
        mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14)
      ])
      val app1 = [mkRec(KIND_FIELD, 20), mkRec(KIND_ENV, 21), mkRec(KIND_SLOT, 22)]
      val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 1, app1, [{ addr: addrUsed, atPost: false }])
      val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [], [])
      {
        ...base,
        leaf1: b1.leaf,
        leaf2: b2.leaf,
        refPost1: b1.leaf.post,
        refPost2: b2.leaf.post,
        refReadVals1: b1.expectedReads,
        refReadVals2: []
      }
    } else if (tKind == 11) {
      // 11. `wrong_kind_warm`: Read of `addr = 4` (`KIND_DATA`) claims `kfix = KIND_SLOT` while keeping `kind = KIND_DATA` on the bus
      val lf1 = base.leaf1
      val rd1 = { ...lf1.reads[1], kfix: KIND_SLOT }
      val lf1Mut: ArenaLeafW = { ...lf1, reads: lf1.reads.replaceAt(1, rd1) }
      { ...base, leaf1: lf1Mut }
    } else if (tKind == 12) {
      // 12. `wrong_kind_cold`: Cold read of `addr = 1` (`KIND_FIELD`) claims `kind = KIND_ENV, kfix = KIND_ENV`
      val lf1 = base.leaf1
      val rd0Orig = lf1.reads[0]
      val forgedR: Rec = { kind: KIND_ENV, d0: rd0Orig.d0, d1: rd0Orig.d1, d2: rd0Orig.d2w }
      val rHash = recD(mt, forgedR)
      val cLvls = buildWalkLevels(mt, rHash, 1, rd0Orig.addr, [0, 0, 0, 0])
      val rd0: ReadPortW = {
        ...rd0Orig,
        kind: KIND_ENV,
        kfix: KIND_ENV,
        bkey: busKey(1, KIND_ENV),
        coldRecHash: rHash,
        coldLevels: cLvls,
        coldRoot: cLvls[NB - 1].out
      }
      val lf1Mut: ArenaLeafW = { ...lf1, reads: lf1.reads.replaceAt(0, rd0) }
      { ...base, leaf1: lf1Mut }
    } else if (tKind == 13) {
      // 13. `cold_value_forged`: Cold read of `addr = 1` returns forged `d0 = 999`
      val lf1 = base.leaf1
      val rd0Orig = lf1.reads[0]
      val forgedR: Rec = { kind: rd0Orig.kind, d0: 999, d1: rd0Orig.d1, d2: rd0Orig.d2w }
      val rHash = recD(mt, forgedR)
      val cLvls = buildWalkLevels(mt, rHash, 1, rd0Orig.addr, [0, 0, 0, 0])
      val rd0: ReadPortW = {
        ...rd0Orig,
        d0: 999,
        coldRecHash: rHash,
        coldLevels: cLvls,
        coldRoot: cLvls[NB - 1].out
      }
      val lf1Mut: ArenaLeafW = { ...lf1, reads: lf1.reads.replaceAt(0, rd0) }
      { ...base, leaf1: lf1Mut }
    } else if (tKind == 14) {
      // 14. `cold_route_forged`: Window address `addr = 4 >= base` routed to cold (`cold = 1`, `diff = -1`)
      // on a non-flushing leaf (`f = 0`) where `pre.root` holds a stale segment at `q = 1`.
      val r0 = initRefArena(mt)
      val refPre1 = refAppendList(mt, r0, [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11),
        mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14)
      ])
      val staleSeg = [mkRec(KIND_DATA, 777), mkRec(KIND_DATA, 778), mkRec(KIND_DATA, 779), mkRec(KIND_DATA, 780)]
      val staleLeaves = [recD(mt, staleSeg[0]), recD(mt, staleSeg[1]), recD(mt, staleSeg[2]), recD(mt, staleSeg[3])]
      val stalePre: RefArena = {
        ...refPre1,
        segSlots: refPre1.segSlots.replaceAt(1, treeSeg(mt, staleLeaves))
      }
      val b1 = buildHonestArenaLeaf(mt, stalePre, 0, 1, [mkRec(KIND_FIELD, 20)], [{ addr: 4, atPost: true }])
      val lf1 = b1.leaf
      val sibsCold4 = [
        staleLeaves[1],
        nodeH(mt, 1, staleLeaves[2], staleLeaves[3]),
        stalePre.segSlots[0],
        nodeH(mt, 3, stalePre.segSlots[2], stalePre.segSlots[3])
      ]
      val rHash = recD(mt, staleSeg[0])
      val cLvls = buildWalkLevels(mt, rHash, 1, 4, sibsCold4)
      val rd0Orig = lf1.reads[0]
      val rd0Cold: ReadPortW = {
        ...rd0Orig,
        cold: 1,
        diff: 4 - 1 - 4,
        diffBits: buildBits(-1, NB),
        d0: 777,
        d1: 877,
        d2w: 7,
        req: ZERO_TUPLE,
        coldEnable: 1,
        coldRecHash: rHash,
        coldLevels: cLvls,
        coldRoot: cLvls[NB - 1].out
      }
      val lf1Mut: ArenaLeafW = { ...lf1, reads: [rd0Cold] }
      val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [], [])
      {
        ...base,
        leaf1: lf1Mut,
        leaf2: b2.leaf,
        refPost1: lf1Mut.post,
        refPost2: b2.leaf.post,
        refReadVals1: [mkRec(KIND_DATA, 14)],
        refReadVals2: []
      }
    } else if (tKind == 15) {
      // 15. `counter_wrap_2_64`: Start at `len = 14`, Leaf 1 appends 1 (`len = 15`), Leaf 2 appends 1 (`len = 16 = MAX_LEN`)
      val r0 = initRefArena(mt)
      val emptyR: List[Rec] = []
      val fourteenRecs = range(0, 14).foldl(emptyR, (acc, i) => acc.append(mkRec(1 + (i % 4), i + 1)))
      val refPre1 = refAppendList(mt, r0, fourteenRecs)
      val b1 = buildHonestArenaLeaf(mt, refPre1, 0, 1, [mkRec(KIND_DATA, 90)], [])
      val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [mkRec(KIND_FIELD, 91)], [])
      {
        ...base,
        leaf1: b1.leaf,
        leaf2: b2.leaf,
        refPost1: b1.leaf.post,
        refPost2: b2.leaf.post,
        refReadVals1: [],
        refReadVals2: []
      }
    } else if (tKind == 16) {
      // 16. `pad_row_on_bus`: Inactive window row 3 of Leaf 2 (`act = 0, addr = 11`) puts `CHIP_MEM` on the bus with `d0 = 999`,
      // serving Leaf 2's read of `addr = 8` with `d0 = 999` instead of `30`.
      val lf2 = base.leaf2
      val forgedTuple: BusTuple = { chip: CHIP_MEM, bkey: busKey(8, KIND_DATA), d0: 999, d1: 130, d2: 7 }
      val rw3: WinRowCircuit = {
        ...lf2.winRows[3],
        bus: forgedTuple
      }
      val rd0: ReadPortW = {
        ...lf2.reads[0],
        d0: 999,
        req: forgedTuple
      }
      val lf2Mut: ArenaLeafW = {
        ...lf2,
        winRows: lf2.winRows.replaceAt(3, rw3),
        reads: [rd0]
      }
      { ...base, leaf2: lf2Mut }
    } else if (tKind == 17) {
      // 17. `kind_out_of_range`:
      // In Leaf 2 (`base = 8`), Append 0 at `addr = 8` writes `(KIND_DATA, 30)` (`bkey = 65`),
      // and Append 1 at `addr = 9` writes `(KIND_DATA, 31)` (`bkey = 73`).
      // Window row 0 (`addr = 8`) sets `kind = 9, d0 = 31` (`8*8 + 9 = 73`, serving Append 1!),
      // and window row 1 (`addr = 9`) sets `kind = -7, d0 = 30` (`8*9 - 7 = 65`, serving Append 0!).
      val r0 = initRefArena(mt)
      val refPre2 = refAppendList(mt, r0, [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11), mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14), mkRec(KIND_FIELD, 20), mkRec(KIND_ENV, 21), mkRec(KIND_SLOT, 22)
      ])
      val app2 = [mkRec(KIND_DATA, 30), mkRec(KIND_DATA, 31)]
      val b2 = buildHonestArenaLeaf(mt, refPre2, 0, 1, app2, [])
      val lf2 = b2.leaf
      val rw0: WinRowCircuit = {
        ...lf2.winRows[0],
        kind: 9,
        bus: { chip: CHIP_MEM, bkey: busKey(8, 9), d0: 31, d1: 131, d2: 7 }
      }
      val rw1: WinRowCircuit = {
        ...lf2.winRows[1],
        kind: -7,
        bus: { chip: CHIP_MEM, bkey: busKey(9, -7), d0: 30, d1: 130, d2: 7 }
      }
      val winMut = lf2.winRows.replaceAt(0, rw0).replaceAt(1, rw1)
      val h0 = recD(mt, { kind: 9, d0: 31, d1: 131, d2: 7 })
      val h1 = recD(mt, { kind: -7, d0: 30, d1: 130, d2: 7 })
      val recHMut = lf2.recHashes.replaceAt(0, h0).replaceAt(1, h1)
      val mActMut = lf2.mAct.replaceAt(0, h0).replaceAt(1, h1)
      val ts0Mut = treeSeg(mt, [mActMut[0], mActMut[1], mActMut[2], mActMut[3]])
      val lf2Mut: ArenaLeafW = {
        ...lf2,
        winRows: winMut,
        recHashes: recHMut,
        mAct: mActMut,
        ts: lf2.ts.replaceAt(0, ts0Mut),
        tPost: ts0Mut,
        post: { ...lf2.post, tailDigest: ts0Mut }
      }
      {
        ...base,
        leaf2: lf2Mut,
        refPost2: refArenaState(mt, b2.refPost),
        refReadVals2: []
      }
    } else if (tKind == 18) {
      // 18. `arena_kind_pinned`: Two arenas share the bus:
      // Arena 1 has `kfix = KIND_DATA (1)`, Arena 2 has `kfix = KIND_FIELD (2)`.
      // In Arena 2 (`kfix = KIND_FIELD`), window row 1 (`addr = 1`) is given `kind = KIND_DATA (1)` with forged `d0 = 999`,
      // and Arena 1 reads `(addr = 1, KIND_DATA)` on the shared bus, receiving Arena 2's forged `d0 = 999`!
      val r0 = initRefArena(mt)
      val refA1Pre = refAppendList(mt, r0, [mkRec(KIND_DATA, 10), mkRec(KIND_DATA, 11)])
      val bA1 = buildHonestArenaLeaf(mt, refA1Pre, KIND_DATA, 1, [], [{ addr: 1, atPost: true }])
      val bA1Next = buildHonestArenaLeaf(mt, bA1.refPost, KIND_DATA, 1, [], [])

      val refA2Pre = refAppendList(mt, r0, [mkRec(KIND_FIELD, 50)])
      val bA2Honest = buildHonestArenaLeaf(mt, refA2Pre, KIND_FIELD, 1, [mkRec(KIND_FIELD, 51)], [])
      val bA2Forged = buildHonestArenaLeaf(mt, refA2Pre, KIND_FIELD, 1, [mkRec(KIND_DATA, 999)], [])
      val rdA1: ReadPortW = {
        ...bA1.leaf.reads[0],
        d0: 999,
        d1: 1099,
        d2w: 7,
        req: { chip: CHIP_MEM, bkey: busKey(1, KIND_DATA), d0: 999, d1: 1099, d2: 7 }
      }
      val lf1Mut: ArenaLeafW = { ...bA1.leaf, reads: [rdA1] }
      {
        scenId: 0,
        leaf1: lf1Mut,
        leaf2: bA1Next.leaf,
        hasArena2: true,
        arena2Leaf: bA2Forged.leaf,
        refPost1: refArenaState(mt, bA1.refPost),
        refPost2: refArenaState(mt, bA1Next.refPost),
        refReadVals1: bA1.expectedReads,
        refReadVals2: [],
        refArena2Post: refArenaState(mt, bA2Honest.refPost)
      }
    } else if (tKind == 19) {
      // 19. `close_advances_len`: Leaf 2 has 2 CORE appends (`lenPre = 8 -> appendEndAddr = 10`),
      // but sets `post.len = 11` (3 active window rows at `addrs 8, 9, 10`).
      val r0 = initRefArena(mt)
      val refPre2 = refAppendList(mt, r0, [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11), mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14), mkRec(KIND_FIELD, 20), mkRec(KIND_ENV, 21), mkRec(KIND_SLOT, 22)
      ])
      val b2Extra = buildHonestArenaLeaf(
        mt,
        refPre2,
        0,
        1,
        [mkRec(KIND_DATA, 30), mkRec(KIND_FIELD, 31), mkRec(KIND_ENV, 32)],
        [{ addr: 8, atPost: true }]
      )
      val lf2Mut: ArenaLeafW = {
        ...b2Extra.leaf,
        appends: [b2Extra.leaf.appends[0], b2Extra.leaf.appends[1]],
        appendEndAddr: 10
      }
      { ...base, leaf2: lf2Mut }
    } else if (tKind == 20) {
      // 20. `append_value_mismatch`: Leaf 2's append 0 requests `d0 = 30`, but window row 0 holds `d0 = 999`
      val lf2 = base.leaf2
      val rw0: WinRowCircuit = {
        ...lf2.winRows[0],
        bus: { ...lf2.winRows[0].bus, d0: 999 }
      }
      val h0 = recD(mt, { kind: KIND_DATA, d0: 999, d1: 130, d2: 7 })
      val recHMut = lf2.recHashes.replaceAt(0, h0)
      val mActMut = lf2.mAct.replaceAt(0, h0)
      val ts0Mut = treeSeg(mt, [mActMut[0], mActMut[1], mActMut[2], mActMut[3]])
      val rd0: ReadPortW = {
        ...lf2.reads[0],
        d0: 999,
        req: { ...lf2.reads[0].req, d0: 999 }
      }
      val lf2Mut: ArenaLeafW = {
        ...lf2,
        winRows: lf2.winRows.replaceAt(0, rw0),
        recHashes: recHMut,
        mAct: mActMut,
        ts: lf2.ts.replaceAt(0, ts0Mut),
        tPost: ts0Mut,
        post: { ...lf2.post, tailDigest: ts0Mut },
        reads: [rd0]
      }
      { ...base, leaf2: lf2Mut }
    } else if (tKind == 21) {
      // 21. `new_walk_unshared_sibs`: In Leaf 1's flush unit 0 (`f = 1`), the new walk uses forged sibling `s + 1` at level 0
      val lf1 = base.leaf1
      val u0 = lf1.units[0]
      val oldSibs = [u0.oldLevels[0].s, u0.oldLevels[1].s]
      val forgedSibs = [oldSibs[0] + 1, oldSibs[1]]
      val forgedNewLvls = buildWalkLevels(mt, lf1.ts[0], 3, u0.idx, forgedSibs)
      val forgedNewR = forgedNewLvls[TOP - 1].out
      val u0Mut: FlushUnitCircuit = {
        ...u0,
        newLevels: forgedNewLvls,
        newRoot: forgedNewR,
        rout: forgedNewR
      }
      val lf1Mut: ArenaLeafW = {
        ...lf1,
        units: [u0Mut],
        post: { ...lf1.post, root: forgedNewR }
      }
      val lf2Mut: ArenaLeafW = {
        ...base.leaf2,
        pre: lf1Mut.post,
        units: [{ ...base.leaf2.units[0], rin: forgedNewR, rout: forgedNewR }],
        post: { ...base.leaf2.post, root: forgedNewR }
      }
      { ...base, leaf1: lf1Mut, leaf2: lf2Mut }
    } else {
      // 22. `domain_tag_dropped`:
      // When domain/height tags are dropped (`iv = 0`), `padD == emptySeg` and `nodeH(1..2, x, y) == nodeH(3..4, x, y)`.
      // Thus a height-2 tail tree `[recD(r14), padD, padD, padD]` is accepted by `old_walk` (heights 3..4)
      // as a height-4 top tree `[recD(r14), emptySeg, emptySeg, emptySeg]` with `emptySeg` at slot 1!
      val r0 = initRefArena(mt)
      val initRecs = [
        mkRec(KIND_DATA, 10), mkRec(KIND_FIELD, 11),
        mkRec(KIND_ENV, 12), mkRec(KIND_SLOT, 13),
        mkRec(KIND_DATA, 14)
      ]
      val refPre1 = refAppendList(mt, r0, initRecs)
      val forgedPre1: RefArena = {
        ...refPre1,
        segSlots: [recD(mt, mkRec(KIND_DATA, 14)), padD(mt), padD(mt), padD(mt)]
      }
      val app1 = [mkRec(KIND_FIELD, 20), mkRec(KIND_ENV, 21), mkRec(KIND_SLOT, 22)]
      val b1 = buildHonestArenaLeaf(mt, forgedPre1, 0, 1, app1, [])
      val b2 = buildHonestArenaLeaf(mt, b1.refPost, 0, 1, [], [])
      val canonPost = refArenaState(mt, refAppendList(mt, refPre1, app1))
      {
        ...base,
        leaf1: b1.leaf,
        leaf2: b2.leaf,
        refPost1: canonPost,
        refPost2: canonPost,
        refReadVals1: [],
        refReadVals2: []
      }
    }
  }

  // General single-cell tamper over window and backend witness fields (`target in 0..11`, `idx in 0..3`, `newVal` in finite set)
  pure def tamperCell(mt: str, target: int, idx: int, newVal: int): ChainW = {
    val base = buildBaseHonest(mt)
    val lf1 = base.leaf1
    val i4 = idx % 4
    val i3 = idx % 3
    if (target == 0) {
      // Tamper window row `i4` value `d0`
      val rw = lf1.winRows[i4]
      val rwMut = { ...rw, bus: { ...rw.bus, d0: newVal } }
      { ...base, leaf1: { ...lf1, winRows: lf1.winRows.replaceAt(i4, rwMut) } }
    } else if (target == 1) {
      // Tamper window row `i4` `kind`
      val rw = lf1.winRows[i4]
      val rwMut = { ...rw, kind: newVal }
      { ...base, leaf1: { ...lf1, winRows: lf1.winRows.replaceAt(i4, rwMut) } }
    } else if (target == 2) {
      // Tamper window row `i4` `addr`
      val rw = lf1.winRows[i4]
      val rwMut = { ...rw, addr: newVal }
      { ...base, leaf1: { ...lf1, winRows: lf1.winRows.replaceAt(i4, rwMut) } }
    } else if (target == 3) {
      // Tamper window row `i4` `act` flag
      val rw = lf1.winRows[i4]
      val rwMut = { ...rw, act: newVal }
      { ...base, leaf1: { ...lf1, winRows: lf1.winRows.replaceAt(i4, rwMut) } }
    } else if (target == 4) {
      // Tamper window row `i4` `imp` flag
      val rw = lf1.winRows[i4]
      val rwMut = { ...rw, imp: newVal }
      { ...base, leaf1: { ...lf1, winRows: lf1.winRows.replaceAt(i4, rwMut) } }
    } else if (target == 5) {
      // Tamper window row `i4` `bus.chip`
      val rw = lf1.winRows[i4]
      val rwMut = { ...rw, bus: { ...rw.bus, chip: newVal } }
      { ...base, leaf1: { ...lf1, winRows: lf1.winRows.replaceAt(i4, rwMut) } }
    } else if (target == 6) {
      // Tamper post length `leaf1.post.len`
      val lf1Mut = { ...lf1, post: { ...lf1.post, len: newVal } }
      val lf2Mut = { ...base.leaf2, pre: lf1Mut.post }
      { ...base, leaf1: lf1Mut, leaf2: lf2Mut }
    } else if (target == 7) {
      // Tamper flush unit `f` and `rout` / `post.root`
      val u0 = { ...lf1.units[0], f: newVal % 2, rout: newVal }
      val lf1Mut = { ...lf1, units: [u0], post: { ...lf1.post, root: newVal } }
      val lf2Mut = { ...base.leaf2, pre: lf1Mut.post }
      { ...base, leaf1: lf1Mut, leaf2: lf2Mut }
    } else if (target == 8) {
      // Tamper tail mask `mAct[i4]` and `tPost` / `post.tailDigest` of Leaf 2
      val lf2 = base.leaf2
      val mActMut = lf2.mAct.replaceAt(i4, newVal)
      val ts0Mut = treeSeg(mt, [mActMut[0], mActMut[1], mActMut[2], mActMut[3]])
      val lf2Mut = {
        ...lf2,
        mAct: mActMut,
        ts: lf2.ts.replaceAt(0, ts0Mut),
        tPost: ts0Mut,
        post: { ...lf2.post, tailDigest: ts0Mut }
      }
      { ...base, leaf2: lf2Mut }
    } else if (target == 9) {
      // Tamper read port `i3` `cold` route flag
      val rd = lf1.reads[i3]
      val cFlag = newVal % 2
      val rdMut = { ...rd, cold: cFlag, coldEnable: cFlag }
      { ...base, leaf1: { ...lf1, reads: lf1.reads.replaceAt(i3, rdMut) } }
    } else if (target == 10) {
      // Tamper read port `i3` data word `d0`
      val rd = lf1.reads[i3]
      val rdMut = { ...rd, d0: newVal, req: { ...rd.req, d0: newVal } }
      { ...base, leaf1: { ...lf1, reads: lf1.reads.replaceAt(i3, rdMut) } }
    } else {
      // Tamper read port `i3` `kind`
      val rd = lf1.reads[i3]
      val rdMut = { ...rd, kind: newVal }
      { ...base, leaf1: { ...lf1, reads: lf1.reads.replaceAt(i3, rdMut) } }
    }
  }

  // ---------------------------------------------------------------- State Machine (Nondeterministic Prover)
  pure val TAMPER_DOM: Set[int] = 0.to(23)
  pure val TARGET_DOM: Set[int] = 0.to(11)
  pure val IDX_DOM: Set[int] = 0.to(3)
  pure val VAL_DOM: Set[int] = Set(0, 1, 2, 4, 5, 8, 9, 14, 20, 99)
  pure val MODE_DOM: Set[int] = Set(0, 1, 2)

  var w: ChainW
  var acc: bool
  var tamperKind: int
  var isTampered: bool

  action sample = {
    nondet mode = MODE_DOM.oneOf()
    nondet tk = TAMPER_DOM.oneOf()
    nondet target = TARGET_DOM.oneOf()
    nondet idx = IDX_DOM.oneOf()
    nondet newVal = VAL_DOM.oneOf()
    val honest = buildBaseHonest(MUTANT)
    val cand =
      if (mode == 0) honest
      else if (mode == 1) buildTampered(MUTANT, tk)
      else tamperCell(MUTANT, target, idx, newVal)
    all {
      w' = cand,
      acc' = acceptsChain(MUTANT, cand),
      tamperKind' = if (mode == 1) tk else 0,
      isTampered' = (cand != honest)
    }
  }

  action init = sample
  action step = sample

  // ---------------------------------------------------------------- Invariants
  // Refinement invariant: every accepted witness refines the canonical reference memory
  val invRefine: bool =
    acc implies refinesChain(w)

  // Non-vacuity reachability invariants (negated: MUST be violated on `dense_sound`)
  val reachHonestFlushAndReads: bool =
    not(acc and not(isTampered) and w.leaf1.units[0].f == 1 and w.leaf1.reads.length() == 3)

  val reachHonestCrossLeafTail: bool =
    not(acc and not(isTampered) and w.leaf2.units[0].f == 0 and w.leaf2.post.len == 10)

  val reachAcceptedBadTamper: bool =
    not(acc and not(refinesChain(w)))
}

// Sound module instance
module dense_sound { import dense(MUTANT = "none").* }

// Mutant module instances (covering all 21 mutants in `dense/tests.rs` + walk sibling sharing + domain tags + TOP=1 walk bug)
module dense_forged_tail_across_leaf_boundary { import dense(MUTANT = "forged_tail_across_leaf_boundary").* }
module dense_non_canonical_tail { import dense(MUTANT = "non_canonical_tail").* }
module dense_flush_skipped { import dense(MUTANT = "flush_skipped").* }
module dense_flush_spurious { import dense(MUTANT = "flush_spurious").* }
module dense_address_reuse_occupied_slot { import dense(MUTANT = "address_reuse_occupied_slot").* }
module dense_empty_seg_forgery { import dense(MUTANT = "empty_seg_forgery").* }
module dense_address_reuse_window_row { import dense(MUTANT = "address_reuse_window_row").* }
module dense_read_at_len { import dense(MUTANT = "read_at_len").* }
module dense_read_beyond_len { import dense(MUTANT = "read_beyond_len").* }
module dense_future_record_read { import dense(MUTANT = "future_record_read").* }
module dense_wrong_kind_warm { import dense(MUTANT = "wrong_kind_warm").* }
module dense_wrong_kind_cold { import dense(MUTANT = "wrong_kind_cold").* }
module dense_cold_value_forged { import dense(MUTANT = "cold_value_forged").* }
module dense_cold_route_forged { import dense(MUTANT = "cold_route_forged").* }
module dense_counter_wrap_2_64 { import dense(MUTANT = "counter_wrap_2_64").* }
module dense_pad_row_on_bus { import dense(MUTANT = "pad_row_on_bus").* }
module dense_kind_out_of_range { import dense(MUTANT = "kind_out_of_range").* }
module dense_arena_kind_pinned { import dense(MUTANT = "arena_kind_pinned").* }
module dense_close_advances_len { import dense(MUTANT = "close_advances_len").* }
module dense_append_value_mismatch { import dense(MUTANT = "append_value_mismatch").* }
module dense_new_walk_unshared_sibs { import dense(MUTANT = "new_walk_unshared_sibs").* }
module dense_domain_tag_dropped { import dense(MUTANT = "domain_tag_dropped").* }
module dense_top1_walk_empty_shadowed { import dense(MUTANT = "top1_walk_empty_shadowed").* }
