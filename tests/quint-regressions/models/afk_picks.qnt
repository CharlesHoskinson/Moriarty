// -*- mode: Bluespec; -*-
// Soundness & design-consistency model for QA lane `afk-picks`:
//   - Pick 1 (N-7, dissent D-6): `shape_d` binds execution intervals `(h_lo, h_hi)` of a SEG output
//     (`lane-src/sherlock-lfr/gp.rs:441-465`, `:711-744`).
//   - Pick 2 (N-8): append-only `reg_root` history with revocation (`docs/ARCHITECTURE.md:3600-3603`),
//     NODE INTEGRATION `reg_root` equal across enabled child slots (`gp.rs:313-315`, `stmt.rs:993-996`),
//     and LEDGER CONTRACT `StaleOrForeignRegRootRefused` (`docs/ARCHITECTURE.md:3085-3101`).
//   - Pick 3 (MERGE-WORK bound 5,400): `frozen.rs:157-166` (`MERGE_WORK_COLUMNS = 5400`) vs owner N-6
//     estimate 4,400, against measured T8 (5,128 cols) and T8b (5,284 cols, 999/1,000 split CORE
//     constraints, chip set 2 leaf CORE at 1,298 constraints, +62 cols; STACKS & VALUE DIGEST pending).

module afk_picks_configs {
  // =========================================================================
  // 1. Parameterised design & code-gate configuration record
  // =========================================================================

  type PicksConfig = {
    // N-7 (`gp.rs:444-445, 718-719` vs rejected option "claim graph only"):
    shape_binds_intervals: bool,
    // AD-23(a) leaf shape vs built `stmt.rs:854, 980` (`Kind::First` leaving `shape_d = 0`):
    leaf_binds_shape: bool,
    // `ARCHITECTURE.md:2298` witnessed `reg_root` vs built `stmt.rs:854, 1036` (`reg_root = 0`):
    n1_witness_reg_root: bool,
    // N-8 append-only history vs rejected `latest_root_only`:
    append_only_history: bool,
    // N-8 key revocation invalidating tainted historical roots vs `no_revocation`:
    enforce_revocation: bool,
    // NODE INTEGRATION equal `reg_root` across enabled child slots (`gp.rs:313-315`, `stmt.rs:993-996`):
    node_equal_reg_root: bool,
    // Conditional N-8 (`ARCHITECTURE.md:3602`): `FOREIGN(reg_root, fclaim_d)` binding lifting root:
    foreign_binds_reg_root: bool,
    // Open Design Point 1 (Question 2): false = Option A (`acceptCollection` finality, `Set<Field>`);
    // true = Option B (`verified_roots: Map<set_pub, reg_pub>` re-checked at `openClaim`):
    recheck_revocation_on_open: bool,
    // MERGE-WORK frozen column cap (`frozen.rs:166`: 5,400 vs N-6 estimate 4,400):
    col_bound: int,
    // Not-yet-built inline kernel column ranges for STACKS and VALUE DIGEST:
    stacks_cols_range: Set[int],
    vd_cols_range: Set[int],
    // Whether CORE is split a second time so per-AIR constraints stay <= 1,000:
    second_core_split: bool
  }

  // =========================================================================
  // 2. Typed enums & constants from `gp.rs`, `stmt.rs`, `frozen.rs`, T8 & T8b
  // =========================================================================

  type PhaseTag =
    | PhaseInitHonest
    | PhaseCheckGP4Shape
    | PhaseLongRunAfterAdmission
    | PhaseAttackRevokedForeign
    | PhaseAttackMixedRootJoin
    | PhaseAttackN2IntervalCollision
    | PhaseAttackN1InternalSplitCollision
    | PhaseOpenClaimLifecycle
    | PhaseAttackStepPvBus
    | PhaseAttackSealBus
    | PhaseAttackHeightGap

  type SubMode =
    | SubAcceptCollection
    | SubShapePinnedSeg
    | SubOpenClaim
    | SubCheckGP4Dag

  pure val STMT_ERR: int = 0
  pure val STMT_SEG: int = 1
  pure val STMT_SET: int = 2
  pure val STMT_SUB: int = 3
  pure val STMT_DL: int = 4

  pure val OUT_RUNNING: int = 0
  pure val OUT_HALTED: int = 1
  pure val OUT_REJECTED: int = 2
  pure val OUT_EXHAUSTED: int = 3

  pure val MODE_MERGE: int = 0
  pure val MODE_SEAL: int = 2
  pure val MODE_SEALD: int = 3
  pure val MODE_DL: int = 4
  pure val MODE_DESC: int = 5
  pure val MODE_JOIN: int = 6

  pure val KSEL_LEAF: int = 1
  pure val KSEL_NODE: int = 2
  pure val KSEL_FOREIGN: int = 3
  pure val WK_NONE: int = 0
  pure val WK_SLICE0: int = 1

  // `stmt.rs:89` and `:1048`
  pure val UROWS: int = 7
  pure val STEP_PV_ROWS: int = 8
  pure val UKEYS_D_PINNED: int = 700
  pure val KEY_ROOT_PINNED: int = 600
  // Abstracted height-wrap bound (Rust checks 2^64 via four 16-bit limbs + carries in `stmt.rs:928-934`,
  // `:1050-1055`; Quint 0.32.0 Rust evaluator uses signed `i64`, so we model wrap at 2^16 = 65536):
  pure val H_WRAP_BOUND: int = 65536

  // Registry roots and foreign keys
  pure val R_ZERO: int = 0
  pure val R0: int = 10       // Initial root: admits {FKEY_1}
  pure val R1: int = 11       // After key admission: admits {FKEY_1, FKEY_2}
  pure val R2: int = 12       // After revoking FKEY_1: admits {FKEY_2}
  pure val R_UNREG: int = 99  // Foreign / never-published root: {FKEY_3}

  pure val FKEY_1: int = 1
  pure val FKEY_2: int = 2
  pure val FKEY_3: int = 3

  // Measured MERGE-WORK kernel counts (`T8-RESULTS.md:92-110`, `T8B-RESULTS.md:41, 58-60, 71-72`, `frozen.rs:142-166`)
  pure val T8_MERGE_WORK_COLS: int = 5128
  pure val T8B_UROWS_DELTA_COLS: int = 102      // UROWS 4 -> 7 in StmtAir (`2 * 3 * 16 + 2 * 3`)
  pure val T8B_CORE_STEP_PV_COLS: int = 54      // C_PVK main columns replacing CORE public values
  pure val T8B_MERGE_WORK_COLS: int = 5284      // 5128 + 102 + 54 = 5284 (116 cols below 5400)
  pure val T8B_MERGE_WORK_AIRS: int = 49
  pure val INTERNAL_MAX_COLS: int = 3000
  pure val MAX_CONSTRAINTS_PER_AIR: int = 1000
  pure val MAX_INTERACTIONS_PER_AIR: int = 200
  pure val MAX_AIRS: int = 64
  pure val SPLIT_CORE_CONSTRAINTS: int = 999
  pure val UNSPLIT_CHIP2_CORE_CONSTRAINTS: int = 1298
  pure val MEASURED_CHIP2_EXTRA_COLS: int = 62  // 738 - 676 cols in leaf CORE for 2 value rules
  pure val SPEC_STACKS_TOP_WORDS: int = 71      // 52-word frame top + 19-word task top (`ARCHITECTURE.md:1280-1282`)

  // =========================================================================
  // 3. Collision-free symbolic Poseidon2 (`P2`) digests via Polish token lists
  // =========================================================================

  type Digest = List[int]

  pure val D_ZERO: Digest = List(0)             // `[F::ZERO; 8]`
  pure val D_SHAPE_OFF: Digest = List(1)        // `P2(SHK_SHAPE, T_NONE)` (`gp.rs:64`)
  pure val D_EMPTY_DEPS: Digest = List(2)       // `P2(SHK_DEPS, [])` (`gp.rs:68`)
  pure val D_NO_LINK: Digest = List(3)          // `P2(SHK_RUN, [])` (`gp.rs:72`)
  pure val D_NO_PREM: Digest = List(4)          // `P2(SHK_ADM, [])` (`gp.rs:76`)

  pure def dScalar(x: int): Digest = List(5, x)

  pure def hashLeafShape(cfg: PicksConfig, h_pre: int, steps: int, rules_d: int): Digest =
    if (not(cfg.leaf_binds_shape)) D_ZERO
    else if (cfg.shape_binds_intervals) List(10, h_pre, steps, rules_d)
    else List(10, 0, 0, rules_d)

  pure def hashNodeShape(
    mode_code: int,
    slot_bits: int,
    ksel0: int,
    ksel1: int,
    sh0: Digest,
    sh1: Digest,
    wk: int,
    ws: int,
    oa: Digest,
    ob: Digest
  ): Digest =
    List(11, mode_code, slot_bits, ksel0, ksel1)
      .concat(sh0)
      .concat(sh1)
      .concat(List(wk, ws))
      .concat(oa)
      .concat(ob)

  pure def hashFirstShape(cfg: PicksConfig, sh0: Digest, en1: bool, sh1: Digest, h_lo: int, h_hi: int): Digest =
    if (not(cfg.leaf_binds_shape)) D_ZERO
    else
      val slot_bits = if (en1) 3 else 1
      val s1 = if (en1) sh1 else D_SHAPE_OFF
      val oa = if (cfg.shape_binds_intervals) dScalar(h_lo) else D_ZERO
      val ob = if (cfg.shape_binds_intervals) dScalar(h_hi) else D_ZERO
      hashNodeShape(MODE_MERGE, slot_bits, KSEL_LEAF, if (en1) KSEL_LEAF else 0, sh0, s1, WK_NONE, 0, oa, ob)

  pure def hashRunClaim(prog_d: int, input_d: int, out_tag: int, out_pay: int): Digest =
    List(20, prog_d, input_d, out_tag, out_pay)

  pure def hashForeignClaim(cfg: PicksConfig, reg_r: int, fclaim_id: int): Digest =
    if (cfg.foreign_binds_reg_root) List(22, reg_r, fclaim_id)
    else List(21, fclaim_id)

  pure def hashSetLeaf(claim_d: Digest, n_steps: int): Digest =
    List(30, n_steps).concat(claim_d)

  pure def hashSetDep(claim_d: Digest, n_steps: int, cov_root: Digest, in_lk: Digest): Digest =
    List(31, n_steps).concat(claim_d).concat(cov_root).concat(in_lk)

  pure def hashSetJoin(left_r: Digest, right_r: Digest): Digest =
    List(32).concat(left_r).concat(right_r)

  pure def hashDepsCons(pch: Digest, head_claim: Digest): Digest =
    List(40).concat(pch).concat(head_claim)

  // =========================================================================
  // 4. Data types: DAG envelopes, registry state, ledger state, machine state
  // =========================================================================

  type LeafWitness = {
    prog_d: int,
    pre_d: int,
    post_d: int,
    h_pre: int,
    steps: int,
    rules_d: int,
    out_tag: int,
    out_pay: int,
    input_d: int,
    deps_d: Digest,
    key_root: int
  }

  // `EnvNode` carries the 74-word + vkc statement words inspected by circuit constraints,
  // plus ghost provenance fields (`used_fkeys`, `lifted_roots`, `exec_splits`) read ONLY
  // by invariants and NEVER read by circuit/ledger acceptance `evalAccepted`.
  type EnvNode = {
    ok: bool,
    tag: int,
    prog_d: int,
    pre_d: int,
    post_d: int,
    h_lo: int,
    h_hi: int,
    out_tag: int,
    out_pay: int,
    input_d: int,
    deps_d: Digest,
    cset_root: Digest,
    sub_root: Digest,
    head_claim: Digest,
    chain_d: Digest,
    cover_root: Digest,
    in_link: Digest,
    prem_root: Digest,
    key_root: int,
    reg_root: int,
    shape_d: Digest,
    ukeys_d: int,
    ukey_row: int,
    // Ghost provenance (ground-truth reference semantics, never read by `evalAccepted`):
    used_fkeys: Set[int],
    lifted_roots: Set[int],
    exec_splits: List[(int, int)]
  }

  type RegState = {
    published_roots: Set[int],
    latest_root: int,
    root_fkeys: int -> Set[int],
    revoked_fkeys: Set[int]
  }

  type LedgerState = {
    verified_csets: Set[Digest],
    verified_cset_reg: Digest -> int,
    settled_runs: Set[Digest],
    settled_foreign: Set[Digest]
  }

  type MachineState = {
    phase_tag: PhaseTag,
    sub_mode: SubMode,
    is_tampered: bool,
    reg_st: RegState,
    ledger_st: LedgerState,
    sub_node: EnvNode,
    expected_h_lo: int,
    expected_h_hi: int,
    expected_shape_d: Digest,
    expected_splits: List[(int, int)],
    open_claim_d: Digest,
    open_is_foreign: bool,
    open_claim_bound_root: int,
    open_claim_fkeys: Set[int],
    revoked_before_accept: Set[int],
    gp4_shape_id: int,
    step_pv_send_mult: int,
    step_pv_recv_mult: int,
    seal_bus_balanced: bool,
    stacks_cols: int,
    vd_cols: int
  }

  pure val INVALID_NODE: EnvNode = {
    ok: false,
    tag: STMT_ERR,
    prog_d: 0,
    pre_d: 0,
    post_d: 0,
    h_lo: 0,
    h_hi: 0,
    out_tag: 0,
    out_pay: 0,
    input_d: 0,
    deps_d: D_EMPTY_DEPS,
    cset_root: D_ZERO,
    sub_root: D_ZERO,
    head_claim: D_ZERO,
    chain_d: D_EMPTY_DEPS,
    cover_root: D_ZERO,
    in_link: D_NO_LINK,
    prem_root: D_NO_PREM,
    key_root: 0,
    reg_root: 0,
    shape_d: D_ZERO,
    ukeys_d: 0,
    ukey_row: 0,
    used_fkeys: Set(),
    lifted_roots: Set(),
    exec_splits: List()
  }

  // =========================================================================
  // 5. Claim DAG builder (`gp.rs` & `stmt.rs` constraints AS BUILT + picks)
  // =========================================================================

  pure def leafValid(lf: LeafWitness): bool =
    lf.steps >= 1
      and lf.h_pre >= 0
      and (lf.h_pre + lf.steps < H_WRAP_BOUND)
      and (lf.key_root == KEY_ROOT_PINNED)

  // `Kind::First` (N1, `stmt.rs:852-935, 986-1038`): lifts 1 or 2 Sherlock leaves into `STMT_SEG`.
  pure def dagFirstN1(cfg: PicksConfig, lf0: LeafWitness, en1: bool, lf1: LeafWitness, wit_reg_root: int): EnvNode =
    val l0_ok = leafValid(lf0)
    val l1_ok = if (en1) leafValid(lf1) else true
    val h0_hi = lf0.h_pre + lf0.steps
    val h1_hi = lf1.h_pre + lf1.steps
    val stitch_ok =
      if (en1)
        (lf0.prog_d == lf1.prog_d)
          and (lf0.key_root == lf1.key_root)
          and (lf0.post_d == lf1.pre_d)
          and (h0_hi == lf1.h_pre)
      else true
    // `stmt.rs:854, 1036`: AS BUILT sets `reg_root = 0`; `ARCHITECTURE.md:2298` witnesses `reg_root`.
    val out_reg_root = if (cfg.n1_witness_reg_root) wit_reg_root else R_ZERO
    val sh0 = hashLeafShape(cfg, lf0.h_pre, lf0.steps, lf0.rules_d)
    val sh1 = if (en1) hashLeafShape(cfg, lf1.h_pre, lf1.steps, lf1.rules_d) else D_SHAPE_OFF
    val out_h_hi = if (en1) h1_hi else h0_hi
    val out_shape = hashFirstShape(cfg, sh0, en1, sh1, lf0.h_pre, out_h_hi)
    val splits =
      if (en1) List((lf0.h_pre, h0_hi), (lf1.h_pre, h1_hi))
      else List((lf0.h_pre, h0_hi))
    if (not(l0_ok and l1_ok and stitch_ok)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_SEG,
      prog_d: lf0.prog_d,
      pre_d: lf0.pre_d,
      post_d: if (en1) lf1.post_d else lf0.post_d,
      h_lo: lf0.h_pre,
      h_hi: out_h_hi,
      out_tag: if (en1) lf1.out_tag else lf0.out_tag,
      out_pay: if (en1) lf1.out_pay else lf0.out_pay,
      input_d: lf0.input_d,
      deps_d: if (en1) lf1.deps_d else lf0.deps_d,
      key_root: lf0.key_root,
      reg_root: out_reg_root,
      shape_d: out_shape,
      ukeys_d: UKEYS_D_PINNED,
      ukey_row: 0,
      used_fkeys: Set(),
      lifted_roots: Set(out_reg_root),
      exec_splits: splits
    }

  // `Kind::Node` in `m::MERGE` (`stmt.rs:938-1037`, `gp.rs:313-315, 435-465`):
  // Covers binary N2/N3 merges AND unary parent-ladder steps `NW` (row 4), `NW2` (row 5), `N3` (row 6).
  pure def dagMerge(cfg: PicksConfig, e0: EnvNode, en1: bool, e1: EnvNode, ukey_row: int): EnvNode =
    val row_ok = (0 <= ukey_row) and (ukey_row < UROWS)
    val c0_ok = e0.ok and (e0.tag == STMT_SEG) and (e0.ukeys_d == UKEYS_D_PINNED)
    val c1_ok =
      if (en1) e1.ok and (e1.tag == STMT_SEG) and (e1.ukeys_d == UKEYS_D_PINNED)
      else true
    val reg_ok = if (en1 and cfg.node_equal_reg_root) (e0.reg_root == e1.reg_root) else true
    val bin_ok =
      if (en1)
        (e0.prog_d == e1.prog_d)
          and (e0.key_root == e1.key_root)
          and (e0.post_d == e1.pre_d)
          and (e0.h_hi == e1.h_lo)
      else true
    // Verifier ladder shape check (`T8B-RESULTS.md:58-60`):
    val child0_ladder_ok =
      if (e0.ukey_row >= UROWS) (ukey_row == 4)
      else if (e0.ukey_row == 4) (ukey_row == 5)
      else true
    val child1_ladder_ok =
      if (not(en1)) true
      else if (e1.ukey_row >= UROWS) (ukey_row == 4)
      else if (e1.ukey_row == 4) (ukey_row == 5)
      else true
    val out_h_lo = e0.h_lo
    val out_h_hi = if (en1) e1.h_hi else e0.h_hi
    val oa = if (cfg.shape_binds_intervals) dScalar(out_h_lo) else D_ZERO
    val ob = if (cfg.shape_binds_intervals) dScalar(out_h_hi) else D_ZERO
    val sh1 = if (en1) e1.shape_d else D_SHAPE_OFF
    val slot_bits = if (en1) 3 else 1
    val out_shape = hashNodeShape(
      MODE_MERGE, slot_bits, KSEL_NODE, if (en1) KSEL_NODE else 0,
      e0.shape_d, sh1, WK_NONE, 0, oa, ob
    )
    if (not(row_ok and c0_ok and c1_ok and reg_ok and bin_ok and child0_ladder_ok and child1_ladder_ok))
      INVALID_NODE
    else {
      ...e0,
      ok: true,
      tag: STMT_SEG,
      post_d: if (en1) e1.post_d else e0.post_d,
      h_lo: out_h_lo,
      h_hi: out_h_hi,
      out_tag: if (en1) e1.out_tag else e0.out_tag,
      out_pay: if (en1) e1.out_pay else e0.out_pay,
      deps_d: if (en1) e1.deps_d else e0.deps_d,
      reg_root: e0.reg_root,
      shape_d: out_shape,
      ukey_row: ukey_row,
      used_fkeys: if (en1) e0.used_fkeys.union(e1.used_fkeys) else e0.used_fkeys,
      lifted_roots: if (en1) e0.lifted_roots.union(e1.lifted_roots) else e0.lifted_roots,
      exec_splits: if (en1) e0.exec_splits.concat(e1.exec_splits) else e0.exec_splits
    }

  // `Kind::Work` (`MERGE-WORK`, `stmt.rs:949-954, 1004-1056`):
  pure def dagMergeWork(cfg: PicksConfig, e0: EnvNode, e1: EnvNode, wk: LeafWitness, ukey_row: int): EnvNode =
    val row_ok = (UROWS <= ukey_row) and (ukey_row < 2 * UROWS)
    val children_internal = (e0.ukey_row < UROWS) and (e1.ukey_row < UROWS) and (e0.ukey_row != 4) and (e1.ukey_row != 4)
    val c_ok =
      e0.ok and e1.ok
        and (e0.tag == STMT_SEG) and (e1.tag == STMT_SEG)
        and (e0.ukeys_d == UKEYS_D_PINNED) and (e1.ukeys_d == UKEYS_D_PINNED)
        and leafValid(wk)
    val reg_ok = if (cfg.node_equal_reg_root) (e0.reg_root == e1.reg_root) else true
    val stitch_ok =
      (e0.prog_d == e1.prog_d)
        and (e0.key_root == e1.key_root)
        and (e0.post_d == e1.pre_d)
        and (e0.h_hi == e1.h_lo)
        and (wk.prog_d == e0.prog_d)
        and (wk.pre_d == e1.post_d)
        and (wk.h_pre == e1.h_hi)
    val out_h_lo = e0.h_lo
    val out_h_hi = wk.h_pre + wk.steps
    val no_wrap = out_h_hi < H_WRAP_BOUND
    val oa = if (cfg.shape_binds_intervals) dScalar(out_h_lo) else D_ZERO
    val ob = if (cfg.shape_binds_intervals) dScalar(out_h_hi) else D_ZERO
    val ws_in_shape = if (cfg.shape_binds_intervals) wk.steps else 0
    val out_shape = hashNodeShape(
      MODE_MERGE, 3, KSEL_NODE, KSEL_NODE,
      e0.shape_d, e1.shape_d, WK_SLICE0, ws_in_shape, oa, ob
    )
    if (not(row_ok and children_internal and c_ok and reg_ok and stitch_ok and no_wrap))
      INVALID_NODE
    else {
      ...e0,
      ok: true,
      tag: STMT_SEG,
      post_d: wk.post_d,
      h_lo: out_h_lo,
      h_hi: out_h_hi,
      out_tag: wk.out_tag,
      out_pay: wk.out_pay,
      deps_d: wk.deps_d,
      reg_root: e0.reg_root,
      shape_d: out_shape,
      ukey_row: ukey_row,
      used_fkeys: e0.used_fkeys.union(e1.used_fkeys),
      lifted_roots: e0.lifted_roots.union(e1.lifted_roots),
      exec_splits: e0.exec_splits.concat(e1.exec_splits).append((wk.h_pre, out_h_hi))
    }

  // `FOREIGN` lift (`ARCHITECTURE.md:2298, 2503, 3600-3603`):
  pure def dagForeign(cfg: PicksConfig, reg_r: int, fkey_id: int, fclaim_id: int, reg_st: RegState): EnvNode =
    val key_in_root =
      reg_st.root_fkeys.keys().contains(reg_r)
        and reg_st.root_fkeys.get(reg_r).contains(fkey_id)
    val fclaim = hashForeignClaim(cfg, reg_r, fclaim_id)
    val sroot = hashSetLeaf(fclaim, 0)
    val out_shape = hashNodeShape(
      MODE_SEAL, 1, KSEL_FOREIGN, 0,
      D_ZERO, D_SHAPE_OFF, WK_NONE, 0, sroot, D_ZERO
    )
    if (not(key_in_root)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_SET,
      cset_root: sroot,
      head_claim: fclaim,
      key_root: KEY_ROOT_PINNED,
      reg_root: reg_r,
      shape_d: out_shape,
      ukeys_d: UKEYS_D_PINNED,
      ukey_row: 2,
      used_fkeys: Set(fkey_id),
      lifted_roots: Set(reg_r),
      exec_splits: List()
    }

  // `m::SEAL` (`gp.rs:317-343, 387-389, 407`):
  pure def dagSeal(e0: EnvNode, ukey_row: int): EnvNode =
    val row_ok = (0 <= ukey_row) and (ukey_row < UROWS) and (e0.ukey_row < UROWS) and (e0.ukey_row != 4)
    val term_ok = Set(OUT_HALTED, OUT_REJECTED, OUT_EXHAUSTED).contains(e0.out_tag)
    val seal_ok =
      e0.ok and row_ok
        and (e0.tag == STMT_SEG)
        and (e0.h_lo == 0)
        and term_ok
        and (e0.deps_d == D_EMPTY_DEPS)
    val cl = hashRunClaim(e0.prog_d, e0.input_d, e0.out_tag, e0.out_pay)
    val sroot = hashSetLeaf(cl, e0.h_hi)
    val out_shape = hashNodeShape(
      MODE_SEAL, 1, KSEL_NODE, 0,
      e0.shape_d, D_SHAPE_OFF, WK_NONE, 0, sroot, D_ZERO
    )
    if (not(seal_ok)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_SET,
      cset_root: sroot,
      head_claim: cl,
      key_root: e0.key_root,
      reg_root: e0.reg_root,
      shape_d: out_shape,
      ukeys_d: e0.ukeys_d,
      ukey_row: ukey_row,
      used_fkeys: e0.used_fkeys,
      lifted_roots: e0.lifted_roots,
      exec_splits: e0.exec_splits
    }

  // `m::DL_CHAIN_LEAF` / `m::DL_CHAIN_DEP` (`gp.rs:288-411`):
  pure def dagDlChain(cfg: PicksConfig, e0: EnvNode, en1: bool, prev_dl: EnvNode, ukey_row: int): EnvNode =
    val row_ok = (0 <= ukey_row) and (ukey_row < UROWS)
    val e0_ok = e0.ok and Set(STMT_SET, STMT_SUB).contains(e0.tag) and (e0.head_claim != D_ZERO)
    val e1_ok = if (en1) (prev_dl.ok and (prev_dl.tag == STMT_DL)) else true
    val reg_ok = if (en1 and cfg.node_equal_reg_root) (e0.reg_root == prev_dl.reg_root) else true
    val pch = if (en1) prev_dl.chain_d else D_EMPTY_DEPS
    val pcov = if (en1) prev_dl.cover_root else D_ZERO
    val chout = hashDepsCons(pch, e0.head_claim)
    val cov_out = if (pcov == D_ZERO) e0.cset_root else hashSetJoin(pcov, e0.cset_root)
    val sh1 = if (en1) prev_dl.shape_d else D_SHAPE_OFF
    val out_shape = hashNodeShape(
      MODE_DL, if (en1) 3 else 1, KSEL_NODE, if (en1) KSEL_NODE else 0,
      e0.shape_d, sh1, WK_NONE, 0, chout, cov_out
    )
    if (not(row_ok and e0_ok and e1_ok and reg_ok)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_DL,
      chain_d: chout,
      cover_root: cov_out,
      in_link: if (en1) prev_dl.in_link else D_NO_LINK,
      prem_root: if (en1) prev_dl.prem_root else D_NO_PREM,
      key_root: e0.key_root,
      reg_root: e0.reg_root,
      shape_d: out_shape,
      ukeys_d: e0.ukeys_d,
      ukey_row: ukey_row,
      used_fkeys: if (en1) e0.used_fkeys.union(prev_dl.used_fkeys) else e0.used_fkeys,
      lifted_roots: if (en1) e0.lifted_roots.union(prev_dl.lifted_roots) else e0.lifted_roots,
      exec_splits: if (en1) e0.exec_splits.concat(prev_dl.exec_splits) else e0.exec_splits
    }

  // `m::SEALD` (`SEAL-WITH-DEPS`, `gp.rs:307-308, 319-332, 362-365, 390-394, 408`):
  pure def dagSealWithDeps(cfg: PicksConfig, seg: EnvNode, dl_node: EnvNode, ukey_row: int): EnvNode =
    val row_ok = (0 <= ukey_row) and (ukey_row < UROWS) and (seg.ukey_row < UROWS) and (seg.ukey_row != 4)
    val term_ok = Set(OUT_HALTED, OUT_REJECTED, OUT_EXHAUSTED).contains(seg.out_tag)
    val reg_ok = if (cfg.node_equal_reg_root) (seg.reg_root == dl_node.reg_root) else true
    val seald_ok =
      seg.ok and dl_node.ok and row_ok and reg_ok
        and (seg.tag == STMT_SEG)
        and (dl_node.tag == STMT_DL)
        and (seg.h_lo == 0)
        and term_ok
        and (seg.deps_d == dl_node.chain_d)
        and (dl_node.cover_root != D_ZERO)
    val cl = hashRunClaim(seg.prog_d, seg.input_d, seg.out_tag, seg.out_pay)
    val sroot = hashSetDep(cl, seg.h_hi, dl_node.cover_root, dl_node.in_link)
    val out_shape = hashNodeShape(
      MODE_SEALD, 3, KSEL_NODE, KSEL_NODE,
      seg.shape_d, dl_node.shape_d, WK_NONE, 0, sroot, D_ZERO
    )
    if (not(seald_ok)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_SET,
      cset_root: sroot,
      head_claim: cl,
      cover_root: dl_node.cover_root,
      key_root: seg.key_root,
      reg_root: seg.reg_root,
      shape_d: out_shape,
      ukeys_d: seg.ukeys_d,
      ukey_row: ukey_row,
      used_fkeys: seg.used_fkeys.union(dl_node.used_fkeys),
      lifted_roots: seg.lifted_roots.union(dl_node.lifted_roots),
      exec_splits: seg.exec_splits.concat(dl_node.exec_splits)
    }

  // `m::JOIN` (`gp.rs:310-315, 370-371, 395-397, 415`):
  pure def dagJoin(cfg: PicksConfig, e0: EnvNode, e1: EnvNode, ukey_row: int): EnvNode =
    val row_ok = (0 <= ukey_row) and (ukey_row < UROWS)
    val tags_ok = e0.ok and e1.ok and (e0.tag == STMT_SET) and (e1.tag == STMT_SET)
    val reg_ok = if (cfg.node_equal_reg_root) (e0.reg_root == e1.reg_root) else true
    val jroot = hashSetJoin(e0.cset_root, e1.cset_root)
    val out_shape = hashNodeShape(
      MODE_JOIN, 3, KSEL_NODE, KSEL_NODE,
      e0.shape_d, e1.shape_d, WK_NONE, 0, jroot, D_ZERO
    )
    if (not(row_ok and tags_ok and reg_ok)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_SET,
      cset_root: jroot,
      head_claim: D_ZERO,
      key_root: e0.key_root,
      reg_root: e0.reg_root,
      shape_d: out_shape,
      ukeys_d: e0.ukeys_d,
      ukey_row: ukey_row,
      used_fkeys: e0.used_fkeys.union(e1.used_fkeys),
      lifted_roots: e0.lifted_roots.union(e1.lifted_roots),
      exec_splits: e0.exec_splits.concat(e1.exec_splits)
    }

  // `m::DESC_JL` (`gp.rs:372, 394-397, 412`):
  pure def dagDescendJoinLeft(joined_parent: EnvNode, left_child: EnvNode, right_child: EnvNode, ukey_row: int): EnvNode =
    val row_ok = (0 <= ukey_row) and (ukey_row < UROWS)
    val cur_x = if (joined_parent.tag == STMT_SUB) joined_parent.sub_root else joined_parent.cset_root
    val open_ok =
      joined_parent.ok and left_child.ok and right_child.ok and row_ok
        and Set(STMT_SET, STMT_SUB).contains(joined_parent.tag)
        and (cur_x == hashSetJoin(left_child.cset_root, right_child.cset_root))
    val out_shape = hashNodeShape(
      MODE_DESC, 1, KSEL_NODE, 0,
      joined_parent.shape_d, D_SHAPE_OFF, WK_NONE, 0, joined_parent.cset_root, left_child.cset_root
    )
    if (not(open_ok)) INVALID_NODE
    else {
      ...INVALID_NODE,
      ok: true,
      tag: STMT_SUB,
      cset_root: joined_parent.cset_root,
      sub_root: left_child.cset_root,
      head_claim: left_child.head_claim,
      key_root: joined_parent.key_root,
      reg_root: joined_parent.reg_root,
      shape_d: out_shape,
      ukeys_d: joined_parent.ukeys_d,
      ukey_row: ukey_row,
      used_fkeys: joined_parent.used_fkeys,
      lifted_roots: joined_parent.lifted_roots,
      exec_splits: joined_parent.exec_splits
    }

  // =========================================================================
  // 6. GP-4 Reference Shapes (1..4)
  // =========================================================================

  pure def mkLeaf(prog_d: int, pre_d: int, post_d: int, h_pre: int, steps: int, out_tag: int, deps_d: Digest): LeafWitness = {
    prog_d: prog_d,
    pre_d: pre_d,
    post_d: post_d,
    h_pre: h_pre,
    steps: steps,
    rules_d: 1,
    out_tag: out_tag,
    out_pay: 42,
    input_d: 7,
    deps_d: deps_d,
    key_root: KEY_ROOT_PINNED
  }

  // GP-4 Shape 1: Unbalanced chain (binary N1 `[0,2)+[2,5)` merged at N2 with unary N1 `[5,8)`, sealed at N3).
  pure def buildGP4Chain(cfg: PicksConfig, reg_r: int): EnvNode =
    val lf0 = mkLeaf(100, 1, 2, 0, 2, OUT_RUNNING, D_EMPTY_DEPS)
    val lf1 = mkLeaf(100, 2, 3, 2, 3, OUT_RUNNING, D_EMPTY_DEPS)
    val lf2 = mkLeaf(100, 3, 4, 5, 3, OUT_HALTED, D_EMPTY_DEPS)
    val n1_left = dagFirstN1(cfg, lf0, true, lf1, reg_r)
    val n1_right = dagFirstN1(cfg, lf2, false, lf2, reg_r)
    val n2_seg = dagMerge(cfg, n1_left, true, n1_right, 1)
    dagSeal(n2_seg, 6)

  // GP-4 Shape 2: Balanced 4-leaf binary tree (`[0,2)+[2,4)` and `[4,7)+[7,10)` -> N2 -> N3 SEAL).
  pure def buildGP4BalancedTree(cfg: PicksConfig, reg_r: int): EnvNode =
    val lf0 = mkLeaf(101, 10, 11, 0, 2, OUT_RUNNING, D_EMPTY_DEPS)
    val lf1 = mkLeaf(101, 11, 12, 2, 2, OUT_RUNNING, D_EMPTY_DEPS)
    val lf2 = mkLeaf(101, 12, 13, 4, 3, OUT_RUNNING, D_EMPTY_DEPS)
    val lf3 = mkLeaf(101, 13, 14, 7, 3, OUT_HALTED, D_EMPTY_DEPS)
    val left_n1 = dagFirstN1(cfg, lf0, true, lf1, reg_r)
    val right_n1 = dagFirstN1(cfg, lf2, true, lf3, reg_r)
    val merged_n2 = dagMerge(cfg, left_n1, true, right_n1, 1)
    dagSeal(merged_n2, 6)

  // GP-4 Shape 3: `MERGE-WORK` (arity 2 + inline owner work) lifted via `NW (4) -> NW2 (5) -> N3 (6)`.
  pure def buildGP4MergeWorkLadder(cfg: PicksConfig, reg_r: int): EnvNode =
    val lf0 = mkLeaf(102, 20, 21, 0, 3, OUT_RUNNING, D_EMPTY_DEPS)
    val lf1 = mkLeaf(102, 21, 22, 3, 4, OUT_RUNNING, D_EMPTY_DEPS)
    val wk_inline = mkLeaf(102, 22, 23, 7, 5, OUT_HALTED, D_EMPTY_DEPS)
    val s0 = dagFirstN1(cfg, lf0, false, lf0, reg_r)
    val s1 = dagFirstN1(cfg, lf1, false, lf1, reg_r)
    val mw = dagMergeWork(cfg, s0, s1, wk_inline, 7)
    val nw = dagMerge(cfg, mw, false, mw, 4)
    val nw2 = dagMerge(cfg, nw, false, nw, 5)
    dagSeal(nw2, 6)

  // GP-4 Shape 4: Diamond claim DAG + claim consumed TWICE + GP-3 FOREIGN composition + DESCEND + JOIN.
  pure def buildGP4DiamondDoubleConsume(cfg: PicksConfig, reg_r: int, reg_st: RegState): EnvNode =
    val foreign_f = dagForeign(cfg, reg_r, FKEY_1, 501, reg_st)
    val prod_lf = mkLeaf(103, 30, 31, 0, 2, OUT_HALTED, D_EMPTY_DEPS)
    val prod_seg = dagFirstN1(cfg, prod_lf, false, prod_lf, reg_r)
    val prod_set = dagSeal(prod_seg, 1)
    val superset = dagJoin(cfg, foreign_f, prod_set, 2)
    val sub_f = dagDescendJoinLeft(superset, foreign_f, prod_set, 2)
    val dl1 = dagDlChain(cfg, sub_f, false, INVALID_NODE, 2)
    val dl2 = dagDlChain(cfg, sub_f, true, dl1, 2)
    val deps_two_f = hashDepsCons(hashDepsCons(D_EMPTY_DEPS, foreign_f.head_claim), foreign_f.head_claim)
    val c1_lf = mkLeaf(104, 40, 41, 0, 3, OUT_HALTED, deps_two_f)
    val c1_seg = dagFirstN1(cfg, c1_lf, false, c1_lf, reg_r)
    val c1_set = dagSealWithDeps(cfg, c1_seg, dl2, 3)
    val deps_one_f = hashDepsCons(D_EMPTY_DEPS, foreign_f.head_claim)
    val c2_lf = mkLeaf(105, 50, 51, 0, 2, OUT_HALTED, deps_one_f)
    val c2_seg = dagFirstN1(cfg, c2_lf, false, c2_lf, reg_r)
    val c2_set = dagSealWithDeps(cfg, c2_seg, dl1, 3)
    dagJoin(cfg, c1_set, c2_set, 6)

  pure def buildGP4ById(cfg: PicksConfig, shape_id: int, reg_r: int, reg_st: RegState): EnvNode =
    if (shape_id == 1) buildGP4Chain(cfg, reg_r)
    else if (shape_id == 2) buildGP4BalancedTree(cfg, reg_r)
    else if (shape_id == 3) buildGP4MergeWorkLadder(cfg, reg_r)
    else buildGP4DiamondDoubleConsume(cfg, reg_r, reg_st)

  // =========================================================================
  // 7. Registry, Ledger Contract, Budget Arithmetic & Pure Transition Steps
  // =========================================================================

  pure val BASE_ROOT_FKEYS: int -> Set[int] = Map(
    R_ZERO -> Set(),
    R0 -> Set(FKEY_1),
    R1 -> Set(FKEY_1, FKEY_2),
    R2 -> Set(FKEY_2),
    R_UNREG -> Set(FKEY_3)
  )

  pure val INIT_REG_STATE: RegState = {
    published_roots: Set(R_ZERO, R0),
    latest_root: R0,
    root_fkeys: BASE_ROOT_FKEYS,
    revoked_fkeys: Set()
  }

  pure def admitKeyR1(rs: RegState): RegState = {
    ...rs,
    published_roots: rs.published_roots.union(Set(R1)),
    latest_root: R1
  }

  pure def revokeKey1ToR2(rs: RegState): RegState = {
    ...rs,
    published_roots: rs.published_roots.union(Set(R2)),
    latest_root: R2,
    revoked_fkeys: rs.revoked_fkeys.union(Set(FKEY_1))
  }

  // `StaleOrForeignRegRootRefused` (`ARCHITECTURE.md:3098, 3600-3603`):
  pure def ledgerRegRootAccepted(cfg: PicksConfig, reg_r: int, rs: RegState): bool =
    val in_published = rs.published_roots.contains(reg_r)
    val history_ok = if (cfg.append_only_history) in_published else (reg_r == rs.latest_root)
    val rev_ok =
      if (cfg.enforce_revocation)
        rs.root_fkeys.keys().contains(reg_r)
          and (rs.root_fkeys.get(reg_r).intersect(rs.revoked_fkeys) == Set())
      else true
    in_published and history_ok and rev_ok

  pure def allOnChainAdmittedKeys(rs: RegState): Set[int] =
    rs.published_roots.fold(Set(), (acc_k, r) =>
      if (rs.root_fkeys.keys().contains(r)) acc_k.union(rs.root_fkeys.get(r)) else acc_k
    )

  pure def evalAccepted(cfg: PicksConfig, s: MachineState): bool =
    val buses_ok = (s.step_pv_send_mult == s.step_pv_recv_mult) and s.seal_bus_balanced
    val node_ok = s.sub_node.ok and buses_ok
    if (s.sub_mode == SubAcceptCollection)
      node_ok
        and (s.sub_node.tag == STMT_SET)
        and ledgerRegRootAccepted(cfg, s.sub_node.reg_root, s.reg_st)
    else if (s.sub_mode == SubShapePinnedSeg)
      node_ok
        and (s.sub_node.tag == STMT_SEG)
        and (s.sub_node.h_lo == s.expected_h_lo)
        and (s.sub_node.h_hi == s.expected_h_hi)
        and (s.sub_node.shape_d == s.expected_shape_d)
    else if (s.sub_mode == SubOpenClaim)
      val cset_verified = s.ledger_st.verified_csets.contains(s.sub_node.cset_root)
      val no_replay =
        if (s.open_is_foreign) not(s.ledger_st.settled_foreign.contains(s.open_claim_d))
        else not(s.ledger_st.settled_runs.contains(s.open_claim_d))
      val bound_root_ok =
        if (s.open_is_foreign and cfg.foreign_binds_reg_root)
          ledgerRegRootAccepted(cfg, s.open_claim_bound_root, s.reg_st)
        else true
      val recheck_open_ok =
        if (cfg.recheck_revocation_on_open and s.ledger_st.verified_cset_reg.keys().contains(s.sub_node.cset_root))
          ledgerRegRootAccepted(cfg, s.ledger_st.verified_cset_reg.get(s.sub_node.cset_root), s.reg_st)
        else true
      node_ok and cset_verified and no_replay and bound_root_ok and recheck_open_ok
    else
      node_ok
        and (s.sub_node.tag == STMT_SET)
        and ledgerRegRootAccepted(cfg, s.sub_node.reg_root, s.reg_st)

  pure def totalMergeWorkCols(sc: int, vc: int): int =
    T8B_MERGE_WORK_COLS + sc + vc

  pure def effectiveMaxAirConstraints(cfg: PicksConfig): int =
    if (cfg.second_core_split) SPLIT_CORE_CONSTRAINTS else UNSPLIT_CHIP2_CORE_CONSTRAINTS

  pure def effectiveAirCount(cfg: PicksConfig): int =
    T8B_MERGE_WORK_AIRS + (if (cfg.second_core_split) 2 else 1)

  pure def budgetFitsAt(cfg: PicksConfig, sc: int, vc: int): bool =
    (totalMergeWorkCols(sc, vc) <= cfg.col_bound)
      and (effectiveMaxAirConstraints(cfg) <= MAX_CONSTRAINTS_PER_AIR)
      and (effectiveAirCount(cfg) <= MAX_AIRS)

  // Pure next-state constructors (keeping actions thin per `quint-modeling/guidelines/review.md`):
  pure def mkBaseState(cfg: PicksConfig, sc: int, vc: int): MachineState =
    val honest_dag = buildGP4Chain(cfg, R0)
    {
      phase_tag: PhaseInitHonest,
      sub_mode: SubAcceptCollection,
      is_tampered: false,
      reg_st: INIT_REG_STATE,
      ledger_st: {
        verified_csets: Set(honest_dag.cset_root),
        verified_cset_reg: Map(honest_dag.cset_root -> honest_dag.reg_root),
        settled_runs: Set(),
        settled_foreign: Set()
      },
      sub_node: honest_dag,
      expected_h_lo: 0,
      expected_h_hi: 0,
      expected_shape_d: D_ZERO,
      expected_splits: List(),
      open_claim_d: hashForeignClaim(cfg, R0, 501),
      open_is_foreign: true,
      open_claim_bound_root: R0,
      open_claim_fkeys: Set(FKEY_1),
      revoked_before_accept: Set(),
      gp4_shape_id: 4,
      step_pv_send_mult: STEP_PV_ROWS,
      step_pv_recv_mult: STEP_PV_ROWS,
      seal_bus_balanced: true,
      stacks_cols: sc,
      vd_cols: vc
    }

  pure def nextCheckGP4Shape(cfg: PicksConfig, gid: int, sc: int, vc: int): MachineState = {
    ...mkBaseState(cfg, sc, vc),
    phase_tag: PhaseCheckGP4Shape,
    sub_mode: SubCheckGP4Dag,
    is_tampered: false,
    sub_node: buildGP4ById(cfg, gid, R0, INIT_REG_STATE),
    gp4_shape_id: gid
  }

  pure def nextLongRunAfterAdmission(cfg: PicksConfig, sc: int, vc: int): MachineState = {
    ...mkBaseState(cfg, sc, vc),
    phase_tag: PhaseLongRunAfterAdmission,
    sub_mode: SubAcceptCollection,
    is_tampered: false,
    reg_st: admitKeyR1(INIT_REG_STATE),
    sub_node: buildGP4DiamondDoubleConsume(cfg, R0, INIT_REG_STATE)
  }

  pure def nextAttackRevokedForeign(cfg: PicksConfig, sc: int, vc: int): MachineState =
    val reg_revoked = revokeKey1ToR2(admitKeyR1(INIT_REG_STATE))
    {
      ...mkBaseState(cfg, sc, vc),
      phase_tag: PhaseAttackRevokedForeign,
      sub_mode: SubAcceptCollection,
      is_tampered: true,
      reg_st: reg_revoked,
      sub_node: buildGP4DiamondDoubleConsume(cfg, R0, reg_revoked),
      revoked_before_accept: reg_revoked.revoked_fkeys
    }

  pure def nextAttackMixedRootJoin(cfg: PicksConfig, bad_r: int, sc: int, vc: int): MachineState =
    val reg_revoked = revokeKey1ToR2(admitKeyR1(INIT_REG_STATE))
    val valid_left = buildGP4Chain(cfg, R2)
    val bad_fk = if (bad_r == R0) FKEY_1 else FKEY_3
    val bad_right = dagForeign(cfg, bad_r, bad_fk, 909, reg_revoked)
    {
      ...mkBaseState(cfg, sc, vc),
      phase_tag: PhaseAttackMixedRootJoin,
      sub_mode: SubAcceptCollection,
      is_tampered: true,
      reg_st: reg_revoked,
      sub_node: dagJoin(cfg, valid_left, bad_right, 6),
      revoked_before_accept: reg_revoked.revoked_fkeys
    }

  pure def nextAttackN2IntervalCollision(cfg: PicksConfig, alt_split: int, sc: int, vc: int): MachineState =
    val ref_l0 = mkLeaf(200, 1, 2, 0, 2, OUT_RUNNING, D_EMPTY_DEPS)
    val ref_l1 = mkLeaf(200, 2, 3, 2, 6, OUT_HALTED, D_EMPTY_DEPS)
    val ref_seg = dagMerge(
      cfg,
      dagFirstN1(cfg, ref_l0, false, ref_l0, R0),
      true,
      dagFirstN1(cfg, ref_l1, false, ref_l1, R0),
      1
    )
    val wit_l0 = mkLeaf(200, 1, 2, 0, alt_split, OUT_RUNNING, D_EMPTY_DEPS)
    val wit_l1 = mkLeaf(200, 2, 3, alt_split, 8 - alt_split, OUT_HALTED, D_EMPTY_DEPS)
    val wit_seg = dagMerge(
      cfg,
      dagFirstN1(cfg, wit_l0, false, wit_l0, R0),
      true,
      dagFirstN1(cfg, wit_l1, false, wit_l1, R0),
      1
    )
    {
      ...mkBaseState(cfg, sc, vc),
      phase_tag: PhaseAttackN2IntervalCollision,
      sub_mode: SubShapePinnedSeg,
      is_tampered: (alt_split != 2),
      sub_node: wit_seg,
      expected_h_lo: 0,
      expected_h_hi: 8,
      expected_shape_d: ref_seg.shape_d,
      expected_splits: ref_seg.exec_splits
    }

  pure def nextAttackN1InternalSplitCollision(cfg: PicksConfig, alt_split: int, sc: int, vc: int): MachineState =
    val ref_l0 = mkLeaf(201, 1, 2, 0, 1, OUT_RUNNING, D_EMPTY_DEPS)
    val ref_l1 = mkLeaf(201, 2, 3, 1, 3, OUT_HALTED, D_EMPTY_DEPS)
    val ref_n2 = dagMerge(cfg, dagFirstN1(cfg, ref_l0, true, ref_l1, R0), false, INVALID_NODE, 1)
    val wit_l0 = mkLeaf(201, 1, 2, 0, alt_split, OUT_RUNNING, D_EMPTY_DEPS)
    val wit_l1 = mkLeaf(201, 2, 3, alt_split, 4 - alt_split, OUT_HALTED, D_EMPTY_DEPS)
    val wit_n2 = dagMerge(cfg, dagFirstN1(cfg, wit_l0, true, wit_l1, R0), false, INVALID_NODE, 1)
    {
      ...mkBaseState(cfg, sc, vc),
      phase_tag: PhaseAttackN1InternalSplitCollision,
      sub_mode: SubShapePinnedSeg,
      is_tampered: (alt_split != 1),
      sub_node: wit_n2,
      expected_h_lo: 0,
      expected_h_hi: 4,
      expected_shape_d: ref_n2.shape_d,
      expected_splits: ref_n2.exec_splits
    }

  pure def nextOpenClaimLifecycle(cfg: PicksConfig, revoke_after_accept: bool, already_settled: bool, sc: int, vc: int): MachineState =
    val f_node = dagForeign(cfg, R0, FKEY_1, 501, INIT_REG_STATE)
    val reg_now = if (revoke_after_accept) revokeKey1ToR2(admitKeyR1(INIT_REG_STATE)) else INIT_REG_STATE
    val settled_set = if (already_settled) Set(f_node.head_claim) else Set()
    {
      ...mkBaseState(cfg, sc, vc),
      phase_tag: PhaseOpenClaimLifecycle,
      sub_mode: SubOpenClaim,
      is_tampered: already_settled,
      reg_st: reg_now,
      ledger_st: {
        verified_csets: Set(f_node.cset_root),
        verified_cset_reg: Map(f_node.cset_root -> R0),
        settled_runs: Set(),
        settled_foreign: settled_set
      },
      sub_node: f_node,
      open_claim_d: f_node.head_claim,
      open_is_foreign: true,
      open_claim_bound_root: R0,
      open_claim_fkeys: Set(FKEY_1),
      revoked_before_accept: Set()
    }

  pure def nextAttackBusOrStitchTamper(cfg: PicksConfig, tamper_kind: int, sc: int, vc: int): MachineState =
    val base = mkBaseState(cfg, sc, vc)
    val gap_l0 = mkLeaf(202, 1, 2, 0, 2, OUT_RUNNING, D_EMPTY_DEPS)
    val gap_l1 = mkLeaf(202, 2, 3, 3, 2, OUT_HALTED, D_EMPTY_DEPS)
    val gap_dag = dagSeal(dagFirstN1(cfg, gap_l0, true, gap_l1, R0), 6)
    if (tamper_kind == 1) {
      ...base,
      phase_tag: PhaseAttackStepPvBus,
      is_tampered: true,
      step_pv_send_mult: STEP_PV_ROWS - 1
    } else if (tamper_kind == 2) {
      ...base,
      phase_tag: PhaseAttackSealBus,
      is_tampered: true,
      seal_bus_balanced: false
    } else {
      ...base,
      phase_tag: PhaseAttackHeightGap,
      is_tampered: true,
      sub_node: gap_dag
    }

  // =========================================================================
  // 8. Pure invariant predicates (shared by state machine and exhaustive tests)
  // =========================================================================

  pure def pureWitnessSoundnessRefines(s: MachineState, a: bool): bool =
    a implies (
      ((s.sub_mode == SubShapePinnedSeg) implies (s.sub_node.exec_splits == s.expected_splits))
      and ((s.sub_mode == SubOpenClaim and s.open_is_foreign)
            implies not(s.ledger_st.settled_foreign.contains(s.open_claim_d)))
      and (s.step_pv_send_mult == s.step_pv_recv_mult)
      and s.seal_bus_balanced
    )

  pure def pureGP4NotNarrowed(s: MachineState, a: bool): bool =
    (s.sub_mode == SubCheckGP4Dag) implies a

  pure def pureNoRevokedForeign(cfg: PicksConfig, s: MachineState, a: bool): bool =
    a implies (
      ((s.sub_mode == SubAcceptCollection or s.sub_mode == SubCheckGP4Dag) implies (
        (s.sub_node.used_fkeys.intersect(s.reg_st.revoked_fkeys) == Set())
        and s.sub_node.used_fkeys.subseteq(allOnChainAdmittedKeys(s.reg_st))
      ))
      and ((s.sub_mode == SubOpenClaim) implies (
        (s.open_claim_fkeys.intersect(s.revoked_before_accept) == Set())
        and (cfg.foreign_binds_reg_root implies (s.open_claim_fkeys.intersect(s.reg_st.revoked_fkeys) == Set()))
      ))
    )

  pure def pureLongRunSurvives(s: MachineState, a: bool): bool =
    (s.phase_tag == PhaseLongRunAfterAdmission) implies a

  pure def pureHistoryJoinSafe(cfg: PicksConfig, s: MachineState, a: bool): bool =
    (a and (s.sub_mode == SubAcceptCollection or s.sub_mode == SubCheckGP4Dag)) implies (
      s.sub_node.lifted_roots.forall(r1 =>
        s.sub_node.lifted_roots.forall(r2 => r1 == r2)
      )
      or cfg.foreign_binds_reg_root
    )

  pure def pureRefines(cfg: PicksConfig, s: MachineState, a: bool): bool =
    pureWitnessSoundnessRefines(s, a)
      and pureGP4NotNarrowed(s, a)
      and pureNoRevokedForeign(cfg, s, a)
      and pureLongRunSurvives(s, a)
      and pureHistoryJoinSafe(cfg, s, a)
      and budgetFitsAt(cfg, s.stacks_cols, s.vd_cols)

  // =========================================================================
  // 9. Canonical configuration values (sound, open options, and 9 mutants)
  // =========================================================================

  pure val CFG_SOUND: PicksConfig = {
    shape_binds_intervals: true,
    leaf_binds_shape: true,
    n1_witness_reg_root: true,
    append_only_history: true,
    enforce_revocation: true,
    node_equal_reg_root: true,
    foreign_binds_reg_root: false,
    recheck_revocation_on_open: false,
    col_bound: 5400,
    stacks_cols_range: 0.to(58),
    vd_cols_range: 0.to(58),
    second_core_split: true
  }

  pure val CFG_OPT_RECHECK_OPEN: PicksConfig = {
    ...CFG_SOUND,
    recheck_revocation_on_open: true
  }

  pure val CFG_OPT_FOREIGN_BINDS_ROOT: PicksConfig = {
    ...CFG_SOUND,
    node_equal_reg_root: false,
    foreign_binds_reg_root: true
  }

  pure val CFG_M_LATEST_ROOT_ONLY: PicksConfig = {
    ...CFG_SOUND,
    append_only_history: false
  }

  pure val CFG_M_NO_REVOCATION: PicksConfig = {
    ...CFG_SOUND,
    enforce_revocation: false
  }

  pure val CFG_M_JOIN_MIXED_ROOTS: PicksConfig = {
    ...CFG_SOUND,
    node_equal_reg_root: false,
    foreign_binds_reg_root: false
  }

  pure val CFG_M_SHAPE_NO_INTERVALS: PicksConfig = {
    ...CFG_SOUND,
    shape_binds_intervals: false
  }

  pure val CFG_M_BOUND_4400: PicksConfig = {
    ...CFG_SOUND,
    col_bound: 4400,
    stacks_cols_range: 0.to(0),
    vd_cols_range: 0.to(0)
  }

  pure val CFG_M_REALISTIC_STACKS_VD_5400: PicksConfig = {
    ...CFG_SOUND,
    stacks_cols_range: 71.to(110),
    vd_cols_range: 62.to(110)
  }

  pure val CFG_M_UNSPLIT_CORE2_5400: PicksConfig = {
    ...CFG_SOUND,
    stacks_cols_range: 0.to(20),
    vd_cols_range: 62.to(80),
    second_core_split: false
  }

  pure val CFG_M_AS_BUILT_N1_ZERO_REG_ROOT: PicksConfig = {
    ...CFG_SOUND,
    n1_witness_reg_root: false
  }

  pure val CFG_M_AS_BUILT_N1_ZERO_SHAPE: PicksConfig = {
    ...CFG_SOUND,
    leaf_binds_shape: false
  }
}

// ===========================================================================
// Parameterised Base State-Machine Module (`PROMPT.md` line 21)
// ===========================================================================
module afk_picks_base {
  import afk_picks_configs.*

  const CFG: PicksConfig

  var st: MachineState
  var acc: bool

  // Soundness, GP-consistency, budget & non-vacuity invariants:
  val witnessSoundnessRefines: bool = pureWitnessSoundnessRefines(st, acc)
  val invGP4NotNarrowed: bool = pureGP4NotNarrowed(st, acc)
  val invNoRevokedForeign: bool = pureNoRevokedForeign(CFG, st, acc)
  val invLongRunSurvives: bool = pureLongRunSurvives(st, acc)
  val invHistoryJoinSafe: bool = pureHistoryJoinSafe(CFG, st, acc)
  val invBudgetFits: bool = budgetFitsAt(CFG, st.stacks_cols, st.vd_cols)
  val invRefine: bool = pureRefines(CFG, st, acc)

  // Open Design Point 1 diagnostic invariant (Option A vs Option B):
  val invStrictOpenRevocation: bool =
    not(acc and (st.sub_mode == SubOpenClaim) and (st.open_claim_fkeys.intersect(st.reg_st.revoked_fkeys) != Set()))

  // Non-vacuity `reach*` invariants (MUST exit 1 `[violation]` in `afk_picks_sound`):
  val reachAcceptedHonestInit: bool =
    not(acc and (st.phase_tag == PhaseInitHonest))

  val reachAcceptedMergeWorkLadder: bool =
    not(acc and (st.sub_mode == SubCheckGP4Dag) and (st.gp4_shape_id == 3))

  val reachAcceptedDiamondDoubleConsume: bool =
    not(acc and (st.sub_mode == SubCheckGP4Dag) and (st.gp4_shape_id == 4))

  val reachAcceptedLongRunAfterAdmission: bool =
    not(acc and (st.phase_tag == PhaseLongRunAfterAdmission))

  val reachAcceptedOpenClaim: bool =
    not(acc and (st.sub_mode == SubOpenClaim))

  // Anti-tamper safety invariant (must NOT be violated — exit 0 `[ok]` — in `afk_picks_sound`):
  val reachAcceptedBadTamper: bool =
    not(acc and st.is_tampered and not(witnessSoundnessRefines and invNoRevokedForeign and invHistoryJoinSafe))

  // Thin actions delegating all logic to pure functions:
  action init = {
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val s0 = mkBaseState(CFG, sc, vc)
    all {
      st' = s0,
      acc' = evalAccepted(CFG, s0)
    }
  }

  action stepCheckGP4Shape = {
    nondet gid = Set(1, 2, 3, 4).oneOf()
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextCheckGP4Shape(CFG, gid, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepLongRunAfterAdmission = {
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextLongRunAfterAdmission(CFG, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepAttackRevokedForeign = {
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextAttackRevokedForeign(CFG, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepAttackMixedRootJoin = {
    nondet bad_r = Set(R0, R_UNREG).oneOf()
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextAttackMixedRootJoin(CFG, bad_r, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepAttackN2IntervalCollision = {
    nondet alt_split = Set(2, 5).oneOf()
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextAttackN2IntervalCollision(CFG, alt_split, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepAttackN1InternalSplitCollision = {
    nondet alt_split = Set(1, 2).oneOf()
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextAttackN1InternalSplitCollision(CFG, alt_split, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepOpenClaimLifecycle = {
    nondet revoke_after_accept = Set(false, true).oneOf()
    nondet already_settled = Set(false, true).oneOf()
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextOpenClaimLifecycle(CFG, revoke_after_accept, already_settled, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action stepAttackBusOrStitchTamper = {
    nondet tamper_kind = Set(1, 2, 3).oneOf()
    nondet sc = CFG.stacks_cols_range.oneOf()
    nondet vc = CFG.vd_cols_range.oneOf()
    val ns = nextAttackBusOrStitchTamper(CFG, tamper_kind, sc, vc)
    all {
      st' = ns,
      acc' = evalAccepted(CFG, ns)
    }
  }

  action step = any {
    stepCheckGP4Shape,
    stepLongRunAfterAdmission,
    stepAttackRevokedForeign,
    stepAttackMixedRootJoin,
    stepAttackN2IntervalCollision,
    stepAttackN1InternalSplitCollision,
    stepOpenClaimLifecycle,
    stepAttackBusOrStitchTamper
  }
}

// ===========================================================================
// Concrete Runnable Instances (`quint run afk_picks.qnt --main <module>`)
// ===========================================================================

module afk_picks_sound {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_SOUND).*
}

module afk_picks_opt_recheck_open {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_OPT_RECHECK_OPEN).*
}

module afk_picks_opt_foreign_binds_root {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_OPT_FOREIGN_BINDS_ROOT).*
}

module afk_picks_m_latest_root_only {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_LATEST_ROOT_ONLY).*
}

module afk_picks_m_no_revocation {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_NO_REVOCATION).*
}

module afk_picks_m_join_mixed_roots {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_JOIN_MIXED_ROOTS).*
}

module afk_picks_m_shape_no_intervals {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_SHAPE_NO_INTERVALS).*
}

module afk_picks_m_bound_4400 {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_BOUND_4400).*
}

module afk_picks_m_realistic_stacks_vd_5400 {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_REALISTIC_STACKS_VD_5400).*
}

module afk_picks_m_unsplit_core2_5400 {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_UNSPLIT_CORE2_5400).*
}

module afk_picks_m_as_built_n1_zero_reg_root {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_AS_BUILT_N1_ZERO_REG_ROOT).*
}

module afk_picks_m_as_built_n1_zero_shape {
  import afk_picks_configs.*
  import afk_picks_base(CFG = CFG_M_AS_BUILT_N1_ZERO_SHAPE).*
}
