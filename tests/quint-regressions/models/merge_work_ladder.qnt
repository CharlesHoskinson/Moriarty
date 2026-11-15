// -*- mode: Bluespec; -*-
// Sherlock proving co-processor — QA lane `merge-work-ladder`
// Soundness model of T8 X7 / SHP-003 (MERGE-WORK), T8b Q7 (STEP_PV_BUS split CORE),
// and T8b Q6 / SHP-009 (7-row universal key table `UKeys` and NW -> NW2 -> N3 -> L ladder).

module merge_work_ladder_defs {
  // Small finite field modulus (Fermat prime 2^8 + 1 = 257), analogous to BabyBear p.
  // Two 4-bit limbs give max height 16^2 - 1 = 255 < MOD_P, matching BabyBear where
  // 2 * (2^16 - 1) = 131070 < p and CORE_ROWS = 8 < p, while unconstrained field elements
  // in 0..(MOD_P - 1) can exceed LIMB_BASE = 16 or underflow modulo MOD_P.
  pure val MOD_P: int = 257
  pure val LIMB_BASE: int = 16
  pure val MAX_H: int = 255
  pure val CORE_ROWS: int = 8

  type Config = {
    w_at_right: bool,
    pre_link: bool,
    h_link: bool,
    prog_link: bool,
    carry_bool: bool,
    whi_bounded: bool,
    merge_contig: bool,
    stepv_mult_8: bool,
    stepv_wf_pinned: bool,
    core_send_all_rows: bool,
    all_rows_bound: bool,
    strict_half_split: bool,
    parent_checks_vkc: bool,
  }

  pure val SOUND_CFG: Config = {
    w_at_right: true,
    pre_link: true,
    h_link: true,
    prog_link: true,
    carry_bool: true,
    whi_bounded: true,
    merge_contig: true,
    stepv_mult_8: true,
    stepv_wf_pinned: true,
    core_send_all_rows: true,
    all_rows_bound: true,
    strict_half_split: true,
    parent_checks_vkc: true,
  }

  pure val CFG_W_LEFT: Config = { ...SOUND_CFG, w_at_right: false }
  pure val CFG_NO_PRE_LINK: Config = { ...SOUND_CFG, pre_link: false }
  pure val CFG_NO_H_LINK: Config = { ...SOUND_CFG, h_link: false }
  pure val CFG_NO_PROG_LINK: Config = { ...SOUND_CFG, prog_link: false }
  pure val CFG_H_WRAP: Config = { ...SOUND_CFG, carry_bool: false }
  pure val CFG_WHI_NO_RANGE: Config = { ...SOUND_CFG, whi_bounded: false }
  pure val CFG_MERGE_NO_CONTIG: Config = { ...SOUND_CFG, merge_contig: false }
  pure val CFG_STEPV_ONCE: Config = { ...SOUND_CFG, stepv_mult_8: false }
  pure val CFG_STEPV_WF_FREE: Config = { ...SOUND_CFG, stepv_wf_pinned: false }
  pure val CFG_CORE_SEND_ROW0_ONLY: Config = { ...SOUND_CFG, core_send_all_rows: false }
  pure val CFG_ROW_UNBOUND: Config = { ...SOUND_CFG, all_rows_bound: false }
  pure val CFG_HALF_SPLIT_OFF: Config = { ...SOUND_CFG, strict_half_split: false }
  pure val CFG_KEY_CHECK_NONE: Config = { ...SOUND_CFG, parent_checks_vkc: false }

  type ProgId =
    | ProgA
    | ProgB

  type InputId =
    | In1
    | In2

  type RouteKind =
    | RoutePlain
    | RouteWork

  type StmtTag =
    | TagSeg
    | TagFv

  type KeyId =
    | K_N1
    | K_N2
    | K_FV
    | K_N3
    | K_WORK
    | K_NW
    | K_NW2
    | K_FAKE

  type Limb2 = {
    lo: int,
    hi: int,
  }

  type SegEnv = {
    tag: StmtTag,
    prog_d: ProgId,
    pre_d: int,
    post_d: int,
    h_lo: Limb2,
    h_hi: Limb2,
    keys_d_ok: bool,
  }

  type Vkc = {
    cc_parent: KeyId,
    cc_child: KeyId,
    pre: KeyId,
    is_zero: bool,
  }

  type ChildProof = {
    env: SegEnv,
    actual_vkc: Vkc,
    claimed_vkc: Vkc,
    is_fake: bool,
  }

  type StepWords = {
    prog_d: ProgId,
    pre_d: int,
    post_d: int,
    h_pre: Limb2,
    steps: int,
  }

  type CoreWitness = {
    core_h: int,
    pvk_rows: List[StepWords],
    true_prog: ProgId,
    true_pre: int,
    true_steps: int,
  }

  type WorkWitness = {
    c0: ChildProof,
    c1: ChildProof,
    slot1_on: bool,
    m_merge: bool,
    stmt_h: int,
    wk_rows: List[StepWords],
    wf_rows: List[int],
    whi0: Limb2,
    wcry0: Limb2,
    whi1: Limb2,
    wcry1: Limb2,
    tbl: List[Vkc],
    sel_idx: int,
    pv_seg: SegEnv,
    pv_vkc: Vkc,
    core: CoreWitness,
  }

  type NodeWitness = {
    parent_k: KeyId,
    child_k: KeyId,
    child: ChildProof,
    tbl: List[Vkc],
    sel_idx: int,
    pv_seg: SegEnv,
    pv_vkc: Vkc,
  }

  type NodeRung = {
    parent_k: KeyId,
    child_k: KeyId,
    tbl: List[Vkc],
    sel_idx: int,
    pv_vkc: Vkc,
  }

  type BaseSource =
    | SrcWork
    | SrcPlainN1
    | SrcFv
    | SrcFake

  type RootWitness = {
    src: BaseSource,
    fake_seg: SegEnv,
    fake_claimed_vkc: Vkc,
    n1_keys_d_ok: bool,
    rungs: List[NodeRung],
  }

  type TargetLayer =
    | TargetWork
    | TargetNode
    | TargetRootL

  type Witness = {
    target: TargetLayer,
    work: WorkWitness,
    node: NodeWitness,
    root: RootWitness,
  }

  // ------------------------------------------------------------------
  // Reference machine semantics and limb helpers
  // ------------------------------------------------------------------

  pure def isFieldElem(x: int): bool =
    0 <= x and x < MOD_P

  pure def isCanonLimb(x: int): bool =
    0 <= x and x < LIMB_BASE

  pure def isCanonLimb2(h: Limb2): bool =
    isCanonLimb(h.lo) and isCanonLimb(h.hi)

  pure def limb2Val(h: Limb2): int =
    h.lo + LIMB_BASE * h.hi

  pure def limb2OfInt(v: int): Limb2 =
    { lo: v % LIMB_BASE, hi: (v / LIMB_BASE) % LIMB_BASE }

  pure def initStateOf(inp: InputId): int =
    match inp {
      | In1 => 100
      | In2 => 200
    }

  pure def isInitState(st_dig: int): bool =
    st_dig == 100 or st_dig == 200

  pure def progStride(p: ProgId): int =
    match p {
      | ProgA => 1
      | ProgB => 10
    }

  pure def stepN(p: ProgId, pre_dig: int, n_steps: int): int =
    pre_dig + n_steps * progStride(p)

  pure def isTrueRunSegment(e: SegEnv): bool = and {
    e.tag == TagSeg,
    isCanonLimb2(e.h_lo),
    isCanonLimb2(e.h_hi),
    0 <= limb2Val(e.h_lo),
    limb2Val(e.h_lo) < limb2Val(e.h_hi),
    limb2Val(e.h_hi) <= MAX_H,
    (limb2Val(e.h_lo) == 0) implies isInitState(e.pre_d),
    e.post_d == stepN(e.prog_d, e.pre_d, limb2Val(e.h_hi) - limb2Val(e.h_lo)),
  }

  // ------------------------------------------------------------------
  // Universal key table `UKeys` (`stmt.rs:89, 256-277`, `t8_gp.rs:331-393`)
  // ------------------------------------------------------------------

  pure val VKC_ZERO: Vkc = {
    cc_parent: K_N1,
    cc_child: K_N1,
    pre: K_N1,
    is_zero: true,
  }

  pure def vkcPair(pk: KeyId, ck: KeyId): Vkc = {
    cc_parent: pk,
    cc_child: ck,
    pre: pk,
    is_zero: false,
  }

  pure val BASE_NODE_ROWS: List[Vkc] = List(
    vkcPair(K_N2, K_N1),    // row 0: N2 over N1
    vkcPair(K_N3, K_N2),    // row 1: N3 over N2
    vkcPair(K_N3, K_FV),    // row 2: N3 over FV
    vkcPair(K_N3, K_N3),    // row 3: N3 over N3 (fixed point)
    vkcPair(K_NW, K_WORK),  // row 4: NW over MERGE-WORK
    vkcPair(K_NW2, K_NW),   // row 5: NW2 over NW
    vkcPair(K_N3, K_NW2)    // row 6: N3 over NW2
  )

  pure val BASE_FOREIGN_ROWS: List[Vkc] = List(
    vkcPair(K_FV, K_N1),    // foreign[0]: FV over N1
    vkcPair(K_WORK, K_N3)   // foreign[1]: MERGE-WORK over N3
  )

  pure def padRows(src: List[Vkc], urows: int): List[Vkc] =
    range(0, urows).foldl(List(), (acc_l, i) =>
      if (i < src.length()) acc_l.append(src.nth(i)) else acc_l.append(VKC_ZERO)
    )

  // Mirrors `t8_gp.rs:331-393`: `keys()` takes no program and no route argument.
  // Both halves are padded/truncated to `urows` (`stmt.rs:257-264`).
  pure def ukeysForProgramAndRoute(urows: int, _p: ProgId, _rt: RouteKind): List[Vkc] =
    padRows(BASE_NODE_ROWS, urows).concat(padRows(BASE_FOREIGN_ROWS, urows))

  pure def canonicalUKeys(urows: int): List[Vkc] =
    ukeysForProgramAndRoute(urows, ProgA, RouteWork)

  // `DOM_UKEYS` sponge commitments (`stmt.rs:266-277, 939-944`):
  // `c_node = P2(DOM_UKEYS, rows[0..urows])`, `c_foreign = P2(DOM_UKEYS, rows[urows..2*urows])`,
  // `keys_d = P2(DOM_UKEYS, c_node || c_foreign)`.
  // Under mutant `row_unbound` (`not(cfg.all_rows_bound)`), row 4 (one of the ladder rows 4..7)
  // is omitted from `c_node`.
  pure def sameKeysD(urows: int, cfg: Config, t1: List[Vkc], t2: List[Vkc]): bool = and {
    t1.length() == 2 * urows,
    t2.length() == 2 * urows,
    0.to(urows - 1).forall(i =>
      (not(cfg.all_rows_bound) and i == 4) or (t1.nth(i) == t2.nth(i))
    ),
    urows.to(2 * urows - 1).forall(i =>
      t1.nth(i) == t2.nth(i)
    ),
  }

  // ------------------------------------------------------------------
  // Constructors for honest segments, proofs, and witnesses
  // ------------------------------------------------------------------

  pure def mkTrueSeg(p: ProgId, inp: InputId, h0: int, h1: int, kd_ok: bool): SegEnv = {
    val s0 = stepN(p, initStateOf(inp), h0)
    val s1 = stepN(p, initStateOf(inp), h1)
    {
      tag: TagSeg,
      prog_d: p,
      pre_d: s0,
      post_d: s1,
      h_lo: limb2OfInt(h0),
      h_hi: limb2OfInt(h1),
      keys_d_ok: kd_ok,
    }
  }

  pure def mkN3SegProof(p: ProgId, inp: InputId, h0: int, h1: int, kd_ok: bool): ChildProof = {
    val v = vkcPair(K_N3, K_N2)
    {
      env: mkTrueSeg(p, inp, h0, h1, kd_ok),
      actual_vkc: v,
      claimed_vkc: v,
      is_fake: false,
    }
  }

  pure def mkStepWords(p: ProgId, pre_dig: int, h0: int, n_steps: int): StepWords = {
    prog_d: p,
    pre_d: pre_dig,
    post_d: stepN(p, pre_dig, n_steps),
    h_pre: limb2OfInt(h0),
    steps: n_steps,
  }

  pure val ZERO_STEP_WORDS: StepWords = {
    prog_d: ProgA,
    pre_d: 0,
    post_d: 0,
    h_pre: { lo: 0, hi: 0 },
    steps: 0,
  }

  pure def repeatStepWords(sw: StepWords, cnt: int): List[StepWords] =
    range(0, cnt).foldl(List(), (acc_l, _i) => acc_l.append(sw))

  pure def honestWorkWitnessFor(urows: int, p: ProgId): WorkWitness = {
    val c0 = mkN3SegProof(p, In1, 0, 2, true)
    val c1 = mkN3SegProof(p, In1, 2, 4, true)
    val s4 = stepN(p, initStateOf(In1), 4)
    val sw = mkStepWords(p, s4, 4, 1)
    val utbl = canonicalUKeys(urows)
    val wsel = urows + 1
    {
      c0: c0,
      c1: c1,
      slot1_on: true,
      m_merge: true,
      stmt_h: 2,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      wf_rows: List(1, 0),
      whi0: limb2OfInt(5),
      wcry0: { lo: 0, hi: 0 },
      whi1: { lo: 0, hi: 0 },
      wcry1: { lo: 0, hi: 0 },
      tbl: utbl,
      sel_idx: wsel,
      pv_seg: mkTrueSeg(p, In1, 0, 5, true),
      pv_vkc: utbl.nth(wsel),
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: p,
        true_pre: s4,
        true_steps: 1,
      },
    }
  }

  pure def honestNodeWitness(urows: int): NodeWitness = {
    val utbl = canonicalUKeys(urows)
    val n1_seg = mkTrueSeg(ProgA, In1, 0, 5, true)
    val c = {
      env: n1_seg,
      actual_vkc: vkcPair(K_N1, K_N1),
      claimed_vkc: vkcPair(K_N1, K_N1),
      is_fake: false,
    }
    {
      parent_k: K_N2,
      child_k: K_N1,
      child: c,
      tbl: utbl,
      sel_idx: 0,
      pv_seg: n1_seg,
      pv_vkc: utbl.nth(0),
    }
  }

  pure def honestWorkLadderRungs(urows: int): List[NodeRung] = {
    val utbl = canonicalUKeys(urows)
    List(
      { parent_k: K_NW, child_k: K_WORK, tbl: utbl, sel_idx: 4, pv_vkc: utbl.nth(4) },
      { parent_k: K_NW2, child_k: K_NW, tbl: utbl, sel_idx: 5, pv_vkc: utbl.nth(5) },
      { parent_k: K_N3, child_k: K_NW2, tbl: utbl, sel_idx: 6, pv_vkc: utbl.nth(6) }
    )
  }

  pure def honestPlainLadderRungs(urows: int): List[NodeRung] = {
    val utbl = canonicalUKeys(urows)
    List(
      { parent_k: K_N2, child_k: K_N1, tbl: utbl, sel_idx: 0, pv_vkc: utbl.nth(0) },
      { parent_k: K_N3, child_k: K_N2, tbl: utbl, sel_idx: 1, pv_vkc: utbl.nth(1) }
    )
  }

  pure def honestWitness(urows: int): Witness = {
    target: TargetRootL,
    work: honestWorkWitnessFor(urows, ProgA),
    node: honestNodeWitness(urows),
    root: {
      src: SrcWork,
      fake_seg: {
        tag: TagSeg,
        prog_d: ProgA,
        pre_d: 100,
        post_d: 999,
        h_lo: { lo: 0, hi: 0 },
        h_hi: { lo: 5, hi: 0 },
        keys_d_ok: true,
      },
      fake_claimed_vkc: vkcPair(K_FAKE, K_FAKE),
      n1_keys_d_ok: true,
      rungs: honestWorkLadderRungs(urows),
    },
  }

  // ------------------------------------------------------------------
  // Circuit constraint & bus balance evaluators (AS BUILT + mutant gates)
  // ------------------------------------------------------------------

  pure def isSoundChildProof(cp: ChildProof): bool = and {
    not(cp.is_fake),
    not(cp.actual_vkc.is_zero),
    (cp.env.tag == TagSeg) implies isTrueRunSegment(cp.env),
  }

  // `stmt.rs:928-935` and `stmt.rs:1049-1055`: limb carry equations modulo MOD_P.
  pure def evalLimbAdd(cfg: Config, h_pre: Limb2, steps_in: int, whi: Limb2, wcry: Limb2): bool = and {
    isFieldElem(h_pre.lo),
    isFieldElem(h_pre.hi),
    isFieldElem(steps_in),
    isFieldElem(whi.lo),
    isFieldElem(whi.hi),
    isFieldElem(wcry.lo),
    isFieldElem(wcry.hi),
    (whi.lo + LIMB_BASE * wcry.lo) % MOD_P == (h_pre.lo + steps_in) % MOD_P,
    (whi.hi + LIMB_BASE * wcry.hi) % MOD_P == (h_pre.hi + wcry.lo) % MOD_P,
    cfg.carry_bool implies and {
      wcry.lo == 0 or wcry.lo == 1,
      wcry.hi == 0 or wcry.hi == 1,
      wcry.hi == 0,
    },
    cfg.whi_bounded implies and {
      isCanonLimb(whi.lo),
      isCanonLimb(whi.hi),
    },
  }

  // Split kernel `CoreAir` (`core.rs:123-124, 291-318, 376, 591-625`).
  pure def evalSplitCore(cw: CoreWitness): bool = and {
    cw.core_h.in(Set(1, 4, 8, 16)),
    cw.pvk_rows.length() == cw.core_h,
    1 <= cw.true_steps,
    cw.true_steps <= 4,
    cw.pvk_rows.indices().forall(r => {
      val pr = cw.pvk_rows.nth(r)
      val b0 = (r % CORE_ROWS == 0)
      val bs = (r % CORE_ROWS == 4)
      and {
        b0 implies and {
          pr.prog_d == cw.true_prog,
          pr.pre_d == cw.true_pre,
          isCanonLimb2(pr.h_pre),
          0 <= pr.steps and pr.steps < LIMB_BASE,
          (limb2Val(pr.h_pre) == 0) implies isInitState(cw.true_pre),
        },
        bs implies and {
          pr.post_d == stepN(cw.true_prog, cw.true_pre, cw.true_steps),
          pr.steps == cw.true_steps,
        },
      }
    }),
  }

  pure def coreSendCount(cfg: Config, cw: CoreWitness, msg: StepWords): int =
    if (cfg.core_send_all_rows)
      cw.pvk_rows.foldl(0, (cnt, row_msg) => if (row_msg == msg) cnt + 1 else cnt)
    else
      if (cw.pvk_rows.length() > 0 and cw.pvk_rows.nth(0) == msg) CORE_ROWS else 0

  pure def stmtRecvCount(cfg: Config, ww: WorkWitness, msg: StepWords): int = {
    val mult = if (cfg.stepv_mult_8) CORE_ROWS else 1
    range(0, ww.stmt_h).foldl(0, (cnt, r) =>
      if (ww.wk_rows.nth(r) == msg) cnt + ww.wf_rows.nth(r) * mult else cnt
    )
  }

  // Bus 213 (`STEP_PV_BUS`) LogUp multiset equality over F_p.
  pure def evalStepPvBus(cfg: Config, ww: WorkWitness): bool = {
    val all_msgs = ww.core.pvk_rows.foldl(Set(), (s, m) => s.union(Set(m)))
      .union(ww.wk_rows.foldl(Set(), (s, m) => s.union(Set(m))))
    all_msgs.forall(msg =>
      (coreSendCount(cfg, ww.core, msg) % MOD_P) == (stmtRecvCount(cfg, ww, msg) % MOD_P)
    )
  }

  // `StmtAir` (`Kind::Work`) + `gp::eval` + `STEP_PV_BUS` + split `CoreAir`.
  pure def evalWorkLayer(urows: int, cfg: Config, ww: WorkWitness): bool = {
    val wk0 = if (ww.wk_rows.length() > 0) ww.wk_rows.nth(0) else ZERO_STEP_WORDS
    val wk1 = if (ww.wk_rows.length() > 1) ww.wk_rows.nth(1) else ZERO_STEP_WORDS
    val kd_matches = sameKeysD(urows, cfg, ww.tbl, canonicalUKeys(urows))
    val g = ww.m_merge and ww.slot1_on
    and {
      urows >= 5,
      isSoundChildProof(ww.c0),
      ww.slot1_on implies isSoundChildProof(ww.c1),
      // VerifierSubCircuit<2> compiled over K_N3 (`work.rs:8-20`)
      ww.c0.actual_vkc.pre == K_N3,
      ww.slot1_on implies (ww.c1.actual_vkc.pre == K_N3),
      // Parent check of child key commitment (`stmt.rs:845-848`)
      cfg.parent_checks_vkc implies and {
        ww.c0.actual_vkc == ww.c0.claimed_vkc,
        ww.slot1_on implies (ww.c1.actual_vkc == ww.c1.claimed_vkc),
      },
      // Universal key table commitment & allowed half (`stmt.rs:938-965`: i >= UROWS for Work)
      ww.tbl.length() == 2 * urows,
      0 <= ww.sel_idx and ww.sel_idx < 2 * urows,
      ww.sel_idx >= urows,
      ww.pv_vkc == ww.tbl.nth(ww.sel_idx),
      ww.pv_seg.keys_d_ok == kd_matches,
      ww.c0.env.keys_d_ok == ww.pv_seg.keys_d_ok,
      ww.slot1_on implies (ww.c1.env.keys_d_ok == ww.pv_seg.keys_d_ok),
      // MERGE-WORK mode & slot 1 forced on (`stmt.rs:1009-1010`)
      ww.m_merge,
      ww.slot1_on,
      // Binary MERGE rules (`stmt.rs:990-1003`)
      g implies and {
        ww.c0.env.tag == TagSeg,
        ww.c1.env.tag == TagSeg,
        ww.c0.env.prog_d == ww.c1.env.prog_d,
        cfg.merge_contig implies (ww.c0.env.post_d == ww.c1.env.pre_d),
        ww.c0.env.h_hi == ww.c1.env.h_lo,
      },
      // MERGE-WORK link (`stmt.rs:1011-1037`)
      ww.pv_seg.tag == TagSeg,
      ww.pv_seg.prog_d == ww.c0.env.prog_d,
      cfg.prog_link implies (wk0.prog_d == ww.c0.env.prog_d),
      if (cfg.w_at_right) and {
        cfg.pre_link implies (wk0.pre_d == ww.c1.env.post_d),
        cfg.h_link implies (wk0.h_pre == ww.c1.env.h_hi),
        ww.pv_seg.pre_d == ww.c0.env.pre_d,
        ww.pv_seg.h_lo == ww.c0.env.h_lo,
        ww.pv_seg.post_d == wk0.post_d,
        ww.pv_seg.h_hi == ww.whi0,
      } else and {
        // Mutant `w_left`: inline W linked at the left end of slot 0
        cfg.pre_link implies (wk0.post_d == ww.c0.env.pre_d),
        cfg.h_link implies (ww.whi0 == ww.c0.env.h_lo),
        ww.pv_seg.pre_d == wk0.pre_d,
        ww.pv_seg.h_lo == wk0.h_pre,
        ww.pv_seg.post_d == ww.c1.env.post_d,
        ww.pv_seg.h_hi == ww.c1.env.h_hi,
      },
      // StmtAir rows, `wf` flag, `STEP_PV_BUS`, limb carry equations (`stmt.rs:1040-1055`)
      ww.stmt_h.in(Set(2, 4)),
      ww.wk_rows.length() == ww.stmt_h,
      ww.wf_rows.length() == ww.stmt_h,
      ww.wf_rows.indices().forall(r => ww.wf_rows.nth(r) == 0 or ww.wf_rows.nth(r) == 1),
      cfg.stepv_wf_pinned implies and {
        ww.wf_rows.nth(0) == 1,
        1.to(ww.stmt_h - 1).forall(r => ww.wf_rows.nth(r) == 0),
      },
      evalLimbAdd(cfg, wk0.h_pre, wk0.steps, ww.whi0, ww.wcry0),
      evalLimbAdd(cfg, wk1.h_pre, wk1.steps, ww.whi1, ww.wcry1),
      evalSplitCore(ww.core),
      evalStepPvBus(cfg, ww),
    }
  }

  // Standalone `Kind::Node` unary check (`stmt.rs:844-848, 938-965, 1024-1037`).
  pure def evalNodeLayer(urows: int, cfg: Config, nw: NodeWitness): bool = {
    val kd_matches = sameKeysD(urows, cfg, nw.tbl, canonicalUKeys(urows))
    val is_first_parent = (nw.child_k == K_N1)
    and {
      urows >= 5,
      nw.parent_k.in(Set(K_N2, K_N3, K_NW, K_NW2)),
      isSoundChildProof(nw.child),
      nw.child.actual_vkc.pre == nw.child_k,
      // Note: N2 over N1 IS Kind::Node (`self.kind != Kind::First`), so `stmt.rs:844` checks N1's vkc too!
      cfg.parent_checks_vkc implies (nw.child.actual_vkc == nw.child.claimed_vkc),
      nw.tbl.length() == 2 * urows,
      0 <= nw.sel_idx and nw.sel_idx < 2 * urows,
      if (cfg.strict_half_split) (nw.sel_idx < urows) else (nw.sel_idx <= urows),
      nw.pv_vkc == nw.tbl.nth(nw.sel_idx),
      nw.pv_seg.keys_d_ok == kd_matches,
      nw.child.env.keys_d_ok == nw.pv_seg.keys_d_ok,
      nw.pv_seg.tag == nw.child.env.tag,
      nw.pv_seg.prog_d == nw.child.env.prog_d,
      nw.pv_seg.pre_d == nw.child.env.pre_d,
      nw.pv_seg.post_d == nw.child.env.post_d,
      nw.pv_seg.h_lo == nw.child.env.h_lo,
      nw.pv_seg.h_hi == nw.child.env.h_hi,
      is_first_parent or not(is_first_parent),
    }
  }

  pure def baseProofOf(urows: int, cfg: Config, w: Witness): (bool, ChildProof) =
    match w.root.src {
      | SrcWork => (
          evalWorkLayer(urows, cfg, w.work),
          {
            env: w.work.pv_seg,
            actual_vkc: vkcPair(K_WORK, K_N3),
            claimed_vkc: w.work.pv_vkc,
            is_fake: false,
          }
        )
      | SrcPlainN1 => (
          true,
          {
            env: mkTrueSeg(ProgA, In1, 0, 5, w.root.n1_keys_d_ok),
            actual_vkc: vkcPair(K_N1, K_N1),
            claimed_vkc: vkcPair(K_N1, K_N1),
            is_fake: false,
          }
        )
      | SrcFv => (
          true,
          {
            env: { ...mkTrueSeg(ProgA, In1, 0, 5, true), tag: TagFv },
            actual_vkc: vkcPair(K_FV, K_N1),
            claimed_vkc: vkcPair(K_FV, K_N1),
            is_fake: false,
          }
        )
      | SrcFake => (
          true,
          {
            env: w.root.fake_seg,
            actual_vkc: vkcPair(K_FAKE, K_FAKE),
            claimed_vkc: w.root.fake_claimed_vkc,
            is_fake: true,
          }
        )
    }

  pure def stepRung(urows: int, cfg: Config, state_pair: (bool, ChildProof), rg: NodeRung): (bool, ChildProof) = {
    val prev_ok = state_pair._1
    val cur = state_pair._2
    val kd_matches = sameKeysD(urows, cfg, rg.tbl, canonicalUKeys(urows))
    val step_ok = and {
      prev_ok,
      rg.parent_k.in(Set(K_N2, K_N3, K_NW, K_NW2)),
      not(cur.actual_vkc.is_zero),
      cur.actual_vkc.pre == rg.child_k,
      cfg.parent_checks_vkc implies (cur.actual_vkc == cur.claimed_vkc),
      rg.tbl.length() == 2 * urows,
      0 <= rg.sel_idx and rg.sel_idx < 2 * urows,
      if (cfg.strict_half_split) (rg.sel_idx < urows) else (rg.sel_idx <= urows),
      rg.pv_vkc == rg.tbl.nth(rg.sel_idx),
      cur.env.keys_d_ok == kd_matches,
    }
    val out_proof: ChildProof = {
      env: { ...cur.env, keys_d_ok: kd_matches },
      actual_vkc: vkcPair(rg.parent_k, rg.child_k),
      claimed_vkc: rg.pv_vkc,
      is_fake: cur.is_fake,
    }
    (step_ok, out_proof)
  }

  // Full recursion chain into `L` (`eval_root`, `stmt.rs:1168-1223`).
  pure def evalRootLayer(urows: int, cfg: Config, w: Witness): bool = {
    val base_pair = baseProofOf(urows, cfg, w)
    val n_rungs = w.root.rungs.length()
    val folded = range(0, n_rungs).foldl(base_pair, (st_p, idx) =>
      stepRung(urows, cfg, st_p, w.root.rungs.nth(idx))
    )
    val chain_ok = folded._1
    val top = folded._2
    and {
      urows >= 5,
      n_rungs <= 3,
      chain_ok,
      // `L` pins `pre_hash == n3_pre`, `VKC_PRE == n3_pre`, `cached_commit == VKC_CC`,
      // `KEYS_D == keys_d`, `TAG == STMT_SEG`, `H_LO == 0` (`stmt.rs:1149-1216`).
      not(top.actual_vkc.is_zero),
      top.actual_vkc.pre == K_N3,
      top.claimed_vkc.pre == K_N3,
      top.actual_vkc == top.claimed_vkc,
      top.env.keys_d_ok,
      top.env.tag == TagSeg,
      top.env.h_lo == { lo: 0, hi: 0 },
    }
  }

  pure def evalAccepted(urows: int, cfg: Config, w: Witness): bool =
    match w.target {
      | TargetWork => evalWorkLayer(urows, cfg, w.work)
      | TargetNode => evalNodeLayer(urows, cfg, w.node)
      | TargetRootL => evalRootLayer(urows, cfg, w)
    }

  // ------------------------------------------------------------------
  // Reference refinement specification (Questions 1, 2, 3, 4)
  // ------------------------------------------------------------------

  pure def refinesWorkLayer(urows: int, ww: WorkWitness): bool = {
    val wk0 = if (ww.wk_rows.length() > 0) ww.wk_rows.nth(0) else ZERO_STEP_WORDS
    val true_w_post = stepN(ww.core.true_prog, ww.core.true_pre, ww.core.true_steps)
    val utbl = canonicalUKeys(urows)
    and {
      // Output SEG is a valid true run segment with canonical 4-bit limbs and no wrap at 2^8
      isTrueRunSegment(ww.pv_seg),
      // Q1 stitching: child 0, child 1, and inline W contiguous in order (c0, c1, W) on one prog_d
      ww.slot1_on,
      ww.c0.env.tag == TagSeg,
      ww.c1.env.tag == TagSeg,
      ww.c0.env.prog_d == ww.c1.env.prog_d,
      ww.core.true_prog == ww.c0.env.prog_d,
      ww.c0.env.post_d == ww.c1.env.pre_d,
      ww.c0.env.h_hi == ww.c1.env.h_lo,
      ww.core.true_pre == ww.c1.env.post_d,
      ww.pv_seg.prog_d == ww.c0.env.prog_d,
      ww.pv_seg.pre_d == ww.c0.env.pre_d,
      ww.pv_seg.post_d == true_w_post,
      ww.pv_seg.h_lo == ww.c0.env.h_lo,
      limb2Val(ww.pv_seg.h_hi) == limb2Val(ww.c1.env.h_hi) + ww.core.true_steps,
      // Q2 step-word fidelity: statement row-0 `wk` words match W's true split CORE execution
      wk0.prog_d == ww.core.true_prog,
      wk0.pre_d == ww.core.true_pre,
      wk0.post_d == true_w_post,
      wk0.h_pre == ww.c1.env.h_hi,
      wk0.steps == ww.core.true_steps,
      // Q3 key-table binding at Work layer
      ww.pv_seg.keys_d_ok implies and {
        ww.tbl == utbl,
        urows.to(2 * urows - 1).exists(i => utbl.nth(i) == ww.pv_vkc),
      },
    }
  }

  pure def refinesNodeLayer(urows: int, nw: NodeWitness): bool = {
    val utbl = canonicalUKeys(urows)
    and {
      (nw.pv_seg.tag == TagSeg) implies isTrueRunSegment(nw.pv_seg),
      nw.pv_seg.keys_d_ok implies and {
        nw.tbl == utbl,
        0.to(urows - 1).exists(i => utbl.nth(i) == nw.pv_vkc),
      },
    }
  }

  pure def isCanonicalWorkLadder(urows: int, rungs: List[NodeRung]): bool = {
    val utbl = canonicalUKeys(urows)
    and {
      rungs.length() == 3,
      rungs.nth(0).parent_k == K_NW,
      rungs.nth(0).child_k == K_WORK,
      rungs.nth(0).sel_idx == 4,
      rungs.nth(0).tbl == utbl,
      rungs.nth(1).parent_k == K_NW2,
      rungs.nth(1).child_k == K_NW,
      rungs.nth(1).sel_idx == 5,
      rungs.nth(1).tbl == utbl,
      rungs.nth(2).parent_k == K_N3,
      rungs.nth(2).child_k == K_NW2,
      rungs.nth(2).sel_idx == 6,
      rungs.nth(2).tbl == utbl,
    }
  }

  pure def refinesRootLayer(urows: int, w: Witness): bool = {
    val top_seg = match w.root.src {
      | SrcWork => w.work.pv_seg
      | SrcPlainN1 => mkTrueSeg(ProgA, In1, 0, 5, w.root.n1_keys_d_ok)
      | SrcFv => { ...mkTrueSeg(ProgA, In1, 0, 5, true), tag: TagFv }
      | SrcFake => w.root.fake_seg
    }
    and {
      w.root.src == SrcWork or w.root.src == SrcPlainN1,
      isTrueRunSegment(top_seg),
      top_seg.h_lo == { lo: 0, hi: 0 },
      (w.root.src == SrcWork) implies and {
        refinesWorkLayer(urows, w.work),
        isCanonicalWorkLadder(urows, w.root.rungs),
      },
    }
  }

  pure def refinesWitness(urows: int, w: Witness): bool =
    match w.target {
      | TargetWork => refinesWorkLayer(urows, w.work)
      | TargetNode => refinesNodeLayer(urows, w.node)
      | TargetRootL => refinesRootLayer(urows, w)
    }
}

// ====================================================================
// Parameterised base state machine
// ====================================================================
module merge_work_ladder_base {
  import merge_work_ladder_defs.*

  const UROWS: int
  const CFG: Config

  var wit: Witness
  var acc: bool

  action assignWitness(w: Witness): bool = all {
    wit' = w,
    acc' = evalAccepted(UROWS, CFG, w),
  }

  action init: bool =
    assignWitness(honestWitness(UROWS))

  // Move 1: Prover chooses child segments, inline W segment, and stitching/order (`Q1`).
  action proverStitchMove: bool = {
    nondet tgt = Set(TargetWork, TargetRootL).oneOf()
    nondet p0 = Set(ProgA, ProgB).oneOf()
    nondet inp0 = Set(In1, In2).oneOf()
    nondet inp1 = Set(In1, In2).oneOf()
    nondet inpw = Set(In1, In2).oneOf()
    nondet pw = Set(ProgA, ProgB).oneOf()
    nondet cut = Set((0, 2, 2, 4), (2, 4, 0, 2), (0, 2, 0, 2), (1, 3, 3, 5)).oneOf()
    nondet slot1_on = Set(true, false).oneOf()
    nondet wh = Set(0, 2, 4, 8).oneOf()
    nondet use_c1_post_as_w_pre = Set(true, false).oneOf()
    nondet left_pv = Set(true, false).oneOf()

    val c0 = mkN3SegProof(p0, inp0, cut._1, cut._2, true)
    val c1 = mkN3SegProof(p0, inp1, cut._3, cut._4, true)
    val w_pre = if (use_c1_post_as_w_pre) c1.env.post_d else stepN(pw, initStateOf(inpw), wh)
    val sw = mkStepWords(pw, w_pre, wh, 1)
    val whi_val = wh + 1
    val pv_s: SegEnv = if (left_pv) {
      tag: TagSeg,
      prog_d: c0.env.prog_d,
      pre_d: sw.pre_d,
      post_d: c1.env.post_d,
      h_lo: sw.h_pre,
      h_hi: c1.env.h_hi,
      keys_d_ok: true,
    } else {
      tag: TagSeg,
      prog_d: c0.env.prog_d,
      pre_d: c0.env.pre_d,
      post_d: sw.post_d,
      h_lo: c0.env.h_lo,
      h_hi: limb2OfInt(whi_val),
      keys_d_ok: true,
    }
    val base_w = honestWitness(UROWS)
    val new_ww: WorkWitness = {
      ...base_w.work,
      c0: c0,
      c1: c1,
      slot1_on: slot1_on,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      whi0: limb2OfInt(whi_val),
      wcry0: { lo: (wh % LIMB_BASE + 1) / LIMB_BASE, hi: 0 },
      pv_seg: pv_s,
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: pw,
        true_pre: w_pre,
        true_steps: 1,
      },
    }
    assignWitness({ ...base_w, target: tgt, work: new_ww })
  }

  // Move 2: Prover tampers with `whi` / `wcry` limbs and 2^8 boundary wrap (`Q1` / `h_wrap` / `whi_no_range`).
  action proverLimbMove: bool = {
    nondet tgt = Set(TargetWork, TargetRootL).oneOf()
    nondet near_max = Set(false, true).oneOf()
    nondet mode_pick = Set(0, 1, 2, 3).oneOf()

    val base_w = honestWitness(UROWS)
    val c0 = if (near_max) mkN3SegProof(ProgA, In1, 0, 200, true) else mkN3SegProof(ProgA, In1, 0, 2, true)
    val c1 = if (near_max) mkN3SegProof(ProgA, In1, 200, 255, true) else mkN3SegProof(ProgA, In1, 2, 4, true)
    val wh = if (near_max) 255 else 4
    val w_pre = stepN(ProgA, initStateOf(In1), wh)
    val sw = mkStepWords(ProgA, w_pre, wh, 1)

    // mode_pick:
    // 0 -> honest carry (on wh=4 -> whi=[5,0], wcry=[0,0])
    // 1 -> wrap carry wcry=[1,1], whi=[0,0] on wh=255 (`h_wrap`)
    // 2 -> false carry mod 257 on wh=4: wcry=[1,0], whi=[246,1] (`whi_no_range` as built!)
    // 3 -> wrap bypass without carry on wh=255: wcry=[0,0], whi=[16,15] (`whi_no_range` as built!)
    val whi_c: Limb2 =
      if (mode_pick == 0) limb2OfInt(wh + 1)
      else if (mode_pick == 1) { lo: 0, hi: 0 }
      else if (mode_pick == 2) { lo: 246, hi: 1 }
      else { lo: 16, hi: 15 }

    val wcry_c: Limb2 =
      if (mode_pick == 0) { lo: (wh % LIMB_BASE + 1) / LIMB_BASE, hi: ((wh / LIMB_BASE) + ((wh % LIMB_BASE + 1) / LIMB_BASE)) / LIMB_BASE }
      else if (mode_pick == 1) { lo: 1, hi: 1 }
      else if (mode_pick == 2) { lo: 1, hi: 0 }
      else { lo: 0, hi: 0 }

    val new_ww: WorkWitness = {
      ...base_w.work,
      c0: c0,
      c1: c1,
      wk_rows: List(sw, ZERO_STEP_WORDS),
      whi0: whi_c,
      wcry0: wcry_c,
      pv_seg: {
        ...c0.env,
        post_d: sw.post_d,
        h_hi: whi_c,
      },
      core: {
        core_h: CORE_ROWS,
        pvk_rows: repeatStepWords(sw, CORE_ROWS),
        true_prog: ProgA,
        true_pre: w_pre,
        true_steps: 1,
      },
    }
    assignWitness({ ...base_w, target: tgt, work: new_ww })
  }

  // Move 3: Prover tampers with `STEP_PV_BUS` rows, `core_h`, and `wf` flags (`Q2`).
  action proverStepBusMove: bool = {
    nondet tgt = Set(TargetWork, TargetRootL).oneOf()
    nondet bus_case = Set(0, 1, 2, 3, 4).oneOf()

    val base_w = honestWitness(UROWS)
    val honest_sw = mkStepWords(ProgA, 104, 4, 1)
    val forged_sw = { ...honest_sw, post_d: 999 }

    // bus_case:
    // 0 -> one CORE row forged (row 3 has forged_sw, StmtAir has honest_sw) -> refused in sound
    // 1 -> CORE height 1 (`fewer than 8 rows`, row S=4 skipped) with forged_sw (`stepv_once`)
    // 2 -> `wf` on row 1 (`wf_rows = [0, 1]`): row 1 absorbs honest_sw, row 0 links forged_sw (`stepv_wf_free`)
    // 3 -> rows disagreeing: row 0 has forged_sw, rows 1..7 have honest_sw (`core_send_row0_only`)
    // 4 -> second receive attempt (`stmt_h = 4`, `wf_rows = [1, 0, 1, 0]`, `core_h = 16`)
    val core_h_c =
      if (bus_case == 1) 1
      else if (bus_case == 4) 16
      else CORE_ROWS

    val pvk_c =
      if (bus_case == 0) repeatStepWords(honest_sw, CORE_ROWS).replaceAt(3, forged_sw)
      else if (bus_case == 1) List(forged_sw)
      else if (bus_case == 3) repeatStepWords(honest_sw, CORE_ROWS).replaceAt(0, forged_sw)
      else if (bus_case == 4) repeatStepWords(forged_sw, CORE_ROWS).concat(repeatStepWords(honest_sw, CORE_ROWS))
      else repeatStepWords(honest_sw, CORE_ROWS)

    val stmt_h_c = if (bus_case == 4) 4 else 2
    val wk_c =
      if (bus_case == 0) List(honest_sw, ZERO_STEP_WORDS)
      else if (bus_case == 2) List(forged_sw, honest_sw)
      else if (bus_case == 4) List(forged_sw, ZERO_STEP_WORDS, honest_sw, ZERO_STEP_WORDS)
      else List(forged_sw, ZERO_STEP_WORDS)

    val wf_c =
      if (bus_case == 2) List(0, 1)
      else if (bus_case == 4) List(1, 0, 1, 0)
      else List(1, 0)

    val whi1_c = if (bus_case == 2) limb2OfInt(5) else { lo: 0, hi: 0 }
    val pv_post_c = wk_c.nth(0).post_d

    val new_ww: WorkWitness = {
      ...base_w.work,
      stmt_h: stmt_h_c,
      wk_rows: wk_c,
      wf_rows: wf_c,
      whi0: limb2OfInt(5),
      wcry0: { lo: 0, hi: 0 },
      whi1: whi1_c,
      wcry1: { lo: 0, hi: 0 },
      pv_seg: { ...base_w.work.pv_seg, post_d: pv_post_c },
      core: {
        ...base_w.work.core,
        core_h: core_h_c,
        pvk_rows: pvk_c,
      },
    }
    assignWitness({ ...base_w, target: tgt, work: new_ww })
  }

  // Move 4: Prover tampers with `UKeys` rows, half split, and parent key checks (`Q3` & `Q4`).
  action proverKeyAndLadderMove: bool = {
    nondet ladder_case = Set(0, 1, 2, 3, 4, 5, 6, 7).oneOf()
    val base_w = honestWitness(UROWS)
    val utbl = canonicalUKeys(UROWS)

    // ladder_case:
    // 0 -> honest plain route N1 -> N2 -> N3 -> L (`reachAcceptedPlainLadderToL`)
    // 1 -> honest MERGE-WORK route on ProgB (`reachAcceptedWorkLadderProgB`)
    // 2 -> honest standalone Node witness (`reachAcceptedNodeTarget`)
    // 3 -> Node selects `sel_idx = UROWS` (`foreign[0]` presented as node key: `half_split_off`)
    // 4 -> `row_unbound` attack: SrcFake -> N3 (with tbl[4] = vkcPair(K_N3, K_FAKE), sel=4) -> L
    // 5 -> `key_check_none` attack: SrcFake -> N2 (row 0) -> N3 (row 1) -> L
    // 6 -> shortcut attempt: NW proof presented directly to L (`rungs = [NW]`)
    // 7 -> wrong key row on Work (`foreign[0]` FV row), checked one layer up by NW -> refused at L
    if (ladder_case == 0) {
      assignWitness({
        ...base_w,
        target: TargetRootL,
        root: { ...base_w.root, src: SrcPlainN1, rungs: honestPlainLadderRungs(UROWS) },
      })
    } else if (ladder_case == 1) {
      assignWitness({
        ...base_w,
        target: TargetRootL,
        work: honestWorkWitnessFor(UROWS, ProgB),
        root: { ...base_w.root, src: SrcWork, rungs: honestWorkLadderRungs(UROWS) },
      })
    } else if (ladder_case == 2) {
      assignWitness({ ...base_w, target: TargetNode })
    } else if (ladder_case == 3) {
      val bad_nw: NodeWitness = {
        ...base_w.node,
        sel_idx: UROWS,
        pv_vkc: utbl.nth(UROWS),
      }
      assignWitness({ ...base_w, target: TargetNode, node: bad_nw })
    } else if (ladder_case == 4) {
      val rogue_vkc = vkcPair(K_N3, K_FAKE)
      val rogue_tbl = utbl.replaceAt(4, rogue_vkc)
      val rg0: NodeRung = {
        parent_k: K_N3,
        child_k: K_FAKE,
        tbl: rogue_tbl,
        sel_idx: 4,
        pv_vkc: rogue_vkc,
      }
      assignWitness({
        ...base_w,
        target: TargetRootL,
        root: {
          ...base_w.root,
          src: SrcFake,
          fake_claimed_vkc: vkcPair(K_FAKE, K_FAKE),
          rungs: List(rg0),
        },
      })
    } else if (ladder_case == 5) {
      val rg0: NodeRung = {
        parent_k: K_N2,
        child_k: K_FAKE,
        tbl: utbl,
        sel_idx: 0,
        pv_vkc: utbl.nth(0),
      }
      val rg1: NodeRung = {
        parent_k: K_N3,
        child_k: K_N2,
        tbl: utbl,
        sel_idx: 1,
        pv_vkc: utbl.nth(1),
      }
      assignWitness({
        ...base_w,
        target: TargetRootL,
        root: {
          ...base_w.root,
          src: SrcFake,
          fake_claimed_vkc: utbl.nth(0),
          rungs: List(rg0, rg1),
        },
      })
    } else if (ladder_case == 6) {
      val nw_only = List(honestWorkLadderRungs(UROWS).nth(0))
      assignWitness({
        ...base_w,
        target: TargetRootL,
        root: { ...base_w.root, src: SrcWork, rungs: nw_only },
      })
    } else {
      val ww_fv_row: WorkWitness = {
        ...base_w.work,
        sel_idx: UROWS,
        pv_vkc: utbl.nth(UROWS),
      }
      assignWitness({
        ...base_w,
        target: TargetRootL,
        work: ww_fv_row,
      })
    }
  }

  action step: bool = any {
    proverStitchMove,
    proverLimbMove,
    proverStepBusMove,
    proverKeyAndLadderMove,
  }

  // ------------------------------------------------------------------
  // Safety invariants and non-vacuity reachability witnesses
  // ------------------------------------------------------------------

  // Main refinement invariant (`PROMPT.md:21`):
  // Must hold (`[ok]`, exit 0) in `merge_work_ladder`, and fail (`[violation]`, exit 1) in every mutant.
  val invRefine: bool =
    acc implies refinesWitness(UROWS, wit)

  // Question 4 invariant over two programs and both routes (`PROMPT.md:40`):
  // One universal `UKeys` table (`keys_d`) across `ProgA` and `ProgB` and across plain and MERGE-WORK routes.
  val invUniversalKeys: bool = and {
    ukeysForProgramAndRoute(UROWS, ProgA, RoutePlain) == ukeysForProgramAndRoute(UROWS, ProgB, RouteWork),
    ukeysForProgramAndRoute(UROWS, ProgA, RouteWork) == ukeysForProgramAndRoute(UROWS, ProgB, RoutePlain),
  }

  // Negated reachability witnesses that MUST be violated (`[violation]`, exit 1) in the sound instance:
  val reachAcceptedHonest: bool =
    not(acc and wit == honestWitness(UROWS))

  val reachAcceptedWorkTarget: bool =
    not(acc and wit.target == TargetWork)

  val reachAcceptedNodeTarget: bool =
    not(acc and wit.target == TargetNode)

  val reachAcceptedPlainLadderToL: bool =
    not(acc and wit.target == TargetRootL and wit.root.src == SrcPlainN1)

  val reachAcceptedWorkLadderProgB: bool =
    not(acc and wit.target == TargetRootL and wit.root.src == SrcWork and wit.work.pv_seg.prog_d == ProgB)

  val reachRejectedTamper: bool =
    not(not(acc) and wit != honestWitness(UROWS))

  // Safety witness that MUST NOT be violated (`[ok]`, exit 0) in the sound instance:
  val reachAcceptedBadTamper: bool =
    not(acc and wit != honestWitness(UROWS) and not(refinesWitness(UROWS, wit)))
}

// ====================================================================
// Sound instance (UROWS = 7, all constraints including limb range check)
// ====================================================================
module merge_work_ladder {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = SOUND_CFG).*
}

// ====================================================================
// Mutant instances (`merge_work_ladder_m_<mutant>`)
// ====================================================================

// Q1 / Q5: Inline W linked at the left end of child 0 instead of the right end of child 1
module merge_work_ladder_m_w_left {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_W_LEFT).*
}

// Q1 / Q5: Drop `W.pre_d == post_d(1)` (`stmt.rs:1013`)
module merge_work_ladder_m_no_pre_link {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_NO_PRE_LINK).*
}

// Q1 / Q5: Drop `W.h_pre == h_hi(1)` (`stmt.rs:1018`)
module merge_work_ladder_m_no_h_link {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_NO_H_LINK).*
}

// Q1 / Q5: Drop `W.prog_d == prog_d(0)` (`stmt.rs:1014`)
module merge_work_ladder_m_no_prog_link {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_NO_PROG_LINK).*
}

// Q1 / Q5: Carries not boolean / wrap at 2^64 allowed (`stmt.rs:1052, 1055`)
module merge_work_ladder_m_h_wrap {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_H_WRAP).*
}

// REAL DEFECT IN CODE AS BUILT (`stmt.rs:928-935` and `stmt.rs:1049-1055`):
// Carries are boolean and top carry is 0, but `whi` (and `hhi`) limbs have NO 16-bit range check!
module merge_work_ladder_m_whi_no_range {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_WHI_NO_RANGE).*
}

// Q1 / Q5: Drop binary merge contiguity `post_d(0) == pre_d(1)` (`stmt.rs:999`)
module merge_work_ladder_m_merge_no_contig {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_MERGE_NO_CONTIG).*
}

// Q2 / Q5: `StmtAir` receives `STEP_PV_BUS` with multiplicity 1 instead of `STEP_PV_ROWS = 8` (`stmt.rs:1048`)
module merge_work_ladder_m_stepv_once {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_STEPV_ONCE).*
}

// Q2 / Q5: `wf` boolean but not pinned to row 0 (`stmt.rs:1043-1044`)
module merge_work_ladder_m_stepv_wf_free {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_STEPV_WF_FREE).*
}

// Q2: Split `CoreAir` sends `STEP_PV_ROWS` times on row 0 only instead of 1 per row (`core.rs:376`)
module merge_work_ladder_m_core_send_row0_only {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_CORE_SEND_ROW0_ONLY).*
}

// Q3 / Q5: Row 4 (`NW` over `MERGE-WORK`, one of rows 4..7) omitted from `DOM_UKEYS` (`stmt.rs:940`)
module merge_work_ladder_m_row_unbound {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_ROW_UNBOUND).*
}

// Q3 / Q5: Allowed node rows `i <= UROWS` instead of `i < UROWS` (`stmt.rs:950`)
module merge_work_ladder_m_half_split_off {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_HALF_SPLIT_OFF).*
}

// Q3 / Q5: Parent does not check child's key commitment (`stmt.rs:845-848`)
module merge_work_ladder_m_key_check_none {
  import merge_work_ladder_defs.*
  import merge_work_ladder_base(UROWS = 7, CFG = CFG_KEY_CHECK_NONE).*
}
