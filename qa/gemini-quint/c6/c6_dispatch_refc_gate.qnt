// -*- mode: Bluespec; -*-
//
// Quint formal model for Task 2.16 Commit 1 (4a8ba7acf):
// "Gate the RefC space bit the dispatch passes to the checked value reader"
// File: crates/mori-backend/src/sherlock_machine/c6_dispatch_step.rs (lines 718-811, 1153-1220, 1238-1331)
// Paired reader: crates/mori-backend/src/sherlock_machine/c6_checked_value_read.rs (lines 352-488)
// Reference semantics: crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs (lines 925-1051)

module c6_dispatch_refc_gate {

  // =========================================================================
  // Constants: Helper tags, task tags, status codes, and Rule IDs
  // =========================================================================

  // Helper record tags (c6_dispatch_step.rs:42-50)
  val H_NONE: int = 0
  val H_VALIDATE: int = 7
  val H_EQUAL: int = 8
  val H_BIND: int = 9

  // Task record tags (c6_phys_helper.rs:988-1051)
  val T_VALIDATE: int = 0
  val T_EQUAL: int = 1
  val T_PAIR: int = 2

  // Helper status codes (c6_dispatch_step.rs:721, 754, 804)
  val STATUS_PENDING: int = 0
  val STATUS_DONE_OK: int = 1
  val STATUS_DONE_FAIL: int = 2

  // Rule IDs modeled in the helper dispatch lattice (c6_dispatch_step.rs:93-112)
  val R_NONE: int = 0
  val R_VALIDATE_EMPTY: int = 1
  val R_VALIDATE_INVALID: int = 2
  val R_VALIDATE_OCCURRENCE: int = 3
  val R_VALIDATE_FIELD: int = 4
  val R_VALIDATE_PROPAGATE: int = 5
  val R_EQUAL_EMPTY: int = 6
  val R_EQUAL_SAME_REF: int = 7
  val R_EQUAL_DIFFERENT: int = 8
  val R_EQUAL_PAIR: int = 9
  val R_EQUAL_FIELD: int = 10
  val R_EQUAL_PROPAGATE: int = 11
  val R_BIND_EMPTY: int = 12
  val R_BIND_PAIR: int = 13

  // =========================================================================
  // Low-level ZKIRv3 / C6 step-kit gadget primitives
  // =========================================================================

  // Instruction::ConstrainToBoolean (c6_step_kit.rs:212-217)
  pure def cbool(x: int): bool = {
    or { x == 0, x == 1 }
  }

  // Instruction::CondSelect { cond, a, b } (c6_step_kit.rs:230-241)
  pure def cond_select(cond: int, a: int, b: int): int = {
    if (cond == 1) a else b
  }

  // em.gassert(gate, cond): Instruction::CondSelect(gate, cond, 1) == 1 (c6_step_kit.rs:265-271)
  pure def gassert(gate: int, cond: int): bool = {
    cond_select(gate, cond, 1) == 1
  }

  // =========================================================================
  // Checked Value Reader Gadget (c6_checked_value_read.rs:352-488)
  // =========================================================================

  type TableSpace = {
    found0: int,
    marker0: int,
    tag0: int,
    found1: int,
    marker1: int,
    tag1: int,
  }

  type CheckedReadOut = {
    constraints_ok: bool,
    selected_found: int,
    is_marker: int,
    tag: int,
  }

  // Look up index in a 2-row space (index 0 or 1; out-of-range -> found = 0)
  pure def space_lookup_found(sp: TableSpace, idx: int): int = {
    if (idx == 0) sp.found0 else if (idx == 1) sp.found1 else 0
  }

  pure def space_lookup_marker(sp: TableSpace, idx: int): int = {
    if (idx == 0) sp.marker0 else if (idx == 1) sp.marker1 else 0
  }

  pure def space_lookup_tag(sp: TableSpace, idx: int): int = {
    if (idx == 0) sp.tag0 else if (idx == 1) sp.tag1 else 0
  }

  // C6CheckedValueReadEmitter::emit (c6_checked_value_read.rs:352-488)
  // Note lines 355-356:
  //   em.cbool(en.clone());
  //   em.cbool(is_data.clone());
  // Both ConstrainToBoolean instructions execute UNCONDITIONALLY regardless of `en`!
  pure def emit_checked_value_read(
    en: int,
    ref_kind_is_data: int,
    ref_index: int,
    const_sp: TableSpace,
    data_sp: TableSpace
  ): CheckedReadOut = {
    val bool_ok = and { cbool(en), cbool(ref_kind_is_data) }
    val c_found = space_lookup_found(const_sp, ref_index)
    val c_marker = space_lookup_marker(const_sp, ref_index)
    val c_tag = space_lookup_tag(const_sp, ref_index)
    val d_found = space_lookup_found(data_sp, ref_index)
    val d_marker = space_lookup_marker(data_sp, ref_index)
    val d_tag = space_lookup_tag(data_sp, ref_index)

    val raw_found = cond_select(ref_kind_is_data, d_found, c_found)
    val raw_marker = cond_select(ref_kind_is_data, d_marker, c_marker)
    val raw_tag = cond_select(ref_kind_is_data, d_tag, c_tag)

    // Line 373: em.gassert(en.clone(), raw_found.clone());
    val assert_found_ok = gassert(en, raw_found)

    // Lines 443-446: all outputs are gated by `en` via `em.sel(en, ..., 0)`
    {
      constraints_ok: and { bool_ok, assert_found_ok },
      selected_found: cond_select(en, raw_found, 0),
      is_marker: cond_select(en, raw_marker, 0),
      tag: cond_select(en, raw_tag, 0),
    }
  }

  // =========================================================================
  // Dispatch Witness & State (c6_dispatch_step.rs:714-811)
  // =========================================================================

  type DispatchWitness = {
    enable: int,
    p0_running: int,
    p1_not_pub: int,
    p2_work_pos: int,
    site_defined: int,
    htag: int,
    h_status: int,
    live: int,
    task0: int,
    task1: int,
    task2: int,
    task3: int,
    task4: int,
    task5: int,
    const_sp: TableSpace,
    data_sp: TableSpace,
    claimed_rule: int,
  }

  // vgate computation (c6_dispatch_step.rs:723-730):
  //   inputs.enable && validate (htag == 7) && pending (h[7] == 0)
  //   && live_pos (live != 0) && validate_task (task[0] == 0)
  //   && validate_unvisited (task[3] == 0)
  pure def compute_vgate(w: DispatchWitness): int = {
    if (and {
      w.enable == 1,
      w.htag == H_VALIDATE,
      w.h_status == STATUS_PENDING,
      w.live > 0,
      w.task0 == T_VALIDATE,
      w.task3 == 0,
    }) 1 else 0
  }

  // egate computation (c6_dispatch_step.rs:757-764):
  //   inputs.enable && equal (htag == 8) && epending (h[9] == 0)
  //   && live_pos (live != 0) && equal_task (task[0] == 1)
  //   && equal_unvisited (task[5] == 0)
  pure def compute_egate(w: DispatchWitness): int = {
    if (and {
      w.enable == 1,
      w.htag == H_EQUAL,
      w.h_status == STATUS_PENDING,
      w.live > 0,
      w.task0 == T_EQUAL,
      w.task5 == 0,
    }) 1 else 0
  }

  // Rule guard and selection path in c6_dispatch_step.rs (lines 1153-1220, 1238-1331)
  pure def circuit_selected_rule(
    w: DispatchWitness,
    validate_marker: int,
    equal_differ: int
  ): int = {
    val base_ok = and {
      w.p0_running == 1,
      w.p1_not_pub == 1,
      w.p2_work_pos == 1,
      w.site_defined == 1,
    }
    if (not(base_ok)) R_NONE
    else if (w.htag == H_VALIDATE) {
      val pending = w.h_status == STATUS_PENDING
      val done = or { w.h_status == STATUS_DONE_OK, w.h_status == STATUS_DONE_FAIL }
      val live_zero = w.live == 0
      val live_pos = w.live > 0
      val vtask = w.task0 == T_VALIDATE
      val unvisited = w.task3 == 0
      val field_visit = w.task3 == 1

      if (and { pending, live_zero }) R_VALIDATE_EMPTY
      else if (and { pending, live_pos, vtask, unvisited, validate_marker == 1 }) R_VALIDATE_INVALID
      else if (and { pending, live_pos, vtask, unvisited, validate_marker == 0 }) R_VALIDATE_OCCURRENCE
      else if (and { pending, live_pos, vtask, field_visit }) R_VALIDATE_FIELD
      else if (done) R_VALIDATE_PROPAGATE
      else R_NONE
    }
    else if (w.htag == H_EQUAL) {
      val pending = w.h_status == STATUS_PENDING
      val done = or { w.h_status == STATUS_DONE_OK, w.h_status == STATUS_DONE_FAIL }
      val live_zero = w.live == 0
      val live_pos = w.live > 0
      val etask = w.task0 == T_EQUAL
      val unvisited = w.task5 == 0
      val field_visit = w.task5 == 1
      val same_ref = and { w.task1 == w.task3, w.task2 == w.task4 }

      if (and { pending, live_zero }) R_EQUAL_EMPTY
      else if (and { pending, live_pos, etask, unvisited, same_ref }) R_EQUAL_SAME_REF
      else if (and { pending, live_pos, etask, unvisited, not(same_ref), equal_differ == 1 }) R_EQUAL_DIFFERENT
      else if (and { pending, live_pos, etask, unvisited, not(same_ref), equal_differ == 0 }) R_EQUAL_PAIR
      else if (and { pending, live_pos, etask, field_visit }) R_EQUAL_FIELD
      else if (done) R_EQUAL_PROPAGATE
      else R_NONE
    }
    else if (w.htag == H_BIND) {
      val pending = w.h_status == STATUS_PENDING
      val live_zero = w.live == 0
      val live_pos = w.live > 0
      val ptask = w.task0 == T_PAIR

      if (and { pending, live_zero }) R_BIND_EMPTY
      else if (and { pending, live_pos, ptask }) R_BIND_PAIR
      else R_NONE
    }
    else R_NONE
  }

  // =========================================================================
  // Pre-4a8ba7acf (UNGATED) vs Post-4a8ba7acf (GATED) Circuit Evaluation
  // =========================================================================

  type CircuitEval = {
    satisfied: bool,
    vgate: int,
    egate: int,
    va_kind: int,
    left_kind: int,
    right_kind: int,
    validate_marker: int,
    equal_differ: int,
    selected_rule: int,
  }

  // Pre-4a8ba7acf circuit: passes task[1] and task[3] directly without gating by vgate/egate
  pure def eval_circuit_ungated(w: DispatchWitness): CircuitEval = {
    val vgate = compute_vgate(w)
    val egate = compute_egate(w)
    val va_kind = w.task1
    val left_kind = w.task1
    val right_kind = w.task3

    val va = emit_checked_value_read(vgate, va_kind, w.task2, w.const_sp, w.data_sp)
    val left = emit_checked_value_read(egate, left_kind, w.task2, w.const_sp, w.data_sp)
    val right = emit_checked_value_read(egate, right_kind, w.task4, w.const_sp, w.data_sp)

    // c6_dispatch_step.rs:748: validate_marker = em.and(va.selected_found, va.is_marker)
    val validate_marker = if (and { va.selected_found == 1, va.is_marker == 1 }) 1 else 0
    // c6_dispatch_step.rs:786-792: equal_differ compares gated projections of left and right
    val equal_differ = if (left.tag != right.tag) 1 else 0

    val sel = circuit_selected_rule(w, validate_marker, equal_differ)
    val rule_match_ok = gassert(w.enable, if (sel == w.claimed_rule) 1 else 0)

    {
      satisfied: and {
        cbool(w.enable),
        va.constraints_ok,
        left.constraints_ok,
        right.constraints_ok,
        rule_match_ok,
      },
      vgate: vgate,
      egate: egate,
      va_kind: va_kind,
      left_kind: left_kind,
      right_kind: right_kind,
      validate_marker: validate_marker,
      equal_differ: equal_differ,
      selected_rule: sel,
    }
  }

  // Post-4a8ba7acf circuit (c6_dispatch_step.rs:737, 765, 766):
  //   let va_kind = em.sel(vgate.clone(), task[1].clone(), Em::imm(0));
  //   let left_kind = em.sel(egate.clone(), task[1].clone(), Em::imm(0));
  //   let right_kind = em.sel(egate.clone(), task[3].clone(), Em::imm(0));
  pure def eval_circuit_gated(w: DispatchWitness): CircuitEval = {
    val vgate = compute_vgate(w)
    val egate = compute_egate(w)
    val va_kind = cond_select(vgate, w.task1, 0)
    val left_kind = cond_select(egate, w.task1, 0)
    val right_kind = cond_select(egate, w.task3, 0)

    val va = emit_checked_value_read(vgate, va_kind, w.task2, w.const_sp, w.data_sp)
    val left = emit_checked_value_read(egate, left_kind, w.task2, w.const_sp, w.data_sp)
    val right = emit_checked_value_read(egate, right_kind, w.task4, w.const_sp, w.data_sp)

    val validate_marker = if (and { va.selected_found == 1, va.is_marker == 1 }) 1 else 0
    val equal_differ = if (left.tag != right.tag) 1 else 0

    val sel = circuit_selected_rule(w, validate_marker, equal_differ)
    val rule_match_ok = gassert(w.enable, if (sel == w.claimed_rule) 1 else 0)

    {
      satisfied: and {
        cbool(w.enable),
        va.constraints_ok,
        left.constraints_ok,
        right.constraints_ok,
        rule_match_ok,
      },
      vgate: vgate,
      egate: egate,
      va_kind: va_kind,
      left_kind: left_kind,
      right_kind: right_kind,
      validate_marker: validate_marker,
      equal_differ: equal_differ,
      selected_rule: sel,
    }
  }

  // =========================================================================
  // Core Reference Semantics (owner_runtime.rs:925-1051, c6_phys_helper.rs:988-1051)
  // =========================================================================

  // Well-formedness of the pre-state under paired canonical checks when enable == 1
  pure def canonical_pre_state(w: DispatchWitness): bool = {
    and {
      cbool(w.p0_running),
      cbool(w.p1_not_pub),
      cbool(w.p2_work_pos),
      cbool(w.site_defined),
      Set(H_NONE, H_VALIDATE, H_EQUAL, H_BIND).contains(w.htag),
      Set(STATUS_PENDING, STATUS_DONE_OK, STATUS_DONE_FAIL).contains(w.h_status),
      and { w.live >= 0, w.live <= 2 },
      // When live > 0, top task must be canonical for its task tag (c6_helper_canonical.rs:766-795):
      // - ValidateTask (0): task1 is r.kind in {0,1}, task2 in {0,1}, task3 is visit in {0,1}
      // - EqualTask (1): task1 is left.kind in {0,1}, task2 in {0,1}, task3 is right.kind in {0,1}, task4 in {0,1}, task5 is visit in {0,1}
      // - PairTask (2): task1 is binder (NameId, can be >= 2!), task2 is sort.0 (can be >= 2!), task3 is arg.kind in {0,1}, task4 in {0,1}
      (w.live > 0) implies and {
        (w.htag == H_VALIDATE and w.h_status == STATUS_PENDING) implies and {
          w.task0 == T_VALIDATE,
          cbool(w.task1),
          or { w.task2 == 0, w.task2 == 1 },
          cbool(w.task3),
        },
        (w.htag == H_EQUAL and w.h_status == STATUS_PENDING) implies and {
          w.task0 == T_EQUAL,
          cbool(w.task1),
          or { w.task2 == 0, w.task2 == 1 },
          cbool(w.task3),
          or { w.task4 == 0, w.task4 == 1 },
          cbool(w.task5),
        },
        (w.htag == H_BIND and w.h_status == STATUS_PENDING) implies and {
          w.task0 == T_PAIR,
          and { w.task1 >= 0, w.task1 <= 3 },
          and { w.task2 >= 0, w.task2 <= 3 },
          cbool(w.task3),
          or { w.task4 == 0, w.task4 == 1 },
        },
      },
    }
  }

  // Direct reference semantics for rule selection (owner_runtime.rs:956-1039)
  pure def ref_select_rule(w: DispatchWitness): int = {
    val base_ok = and {
      w.p0_running == 1,
      w.p1_not_pub == 1,
      w.p2_work_pos == 1,
      w.site_defined == 1,
    }
    if (not(base_ok)) R_NONE
    else if (w.htag == H_VALIDATE) {
      if (w.h_status == STATUS_PENDING) {
        if (w.live == 0) R_VALIDATE_EMPTY
        else if (and { w.task0 == T_VALIDATE, w.task3 == 0 }) {
          val sp = if (w.task1 == 1) w.data_sp else w.const_sp
          val found = space_lookup_found(sp, w.task2)
          val marker = space_lookup_marker(sp, w.task2)
          if (found != 1) R_NONE
          else if (marker == 1) R_VALIDATE_INVALID
          else R_VALIDATE_OCCURRENCE
        }
        else if (and { w.task0 == T_VALIDATE, w.task3 == 1 }) R_VALIDATE_FIELD
        else R_NONE
      }
      else if (or { w.h_status == STATUS_DONE_OK, w.h_status == STATUS_DONE_FAIL }) R_VALIDATE_PROPAGATE
      else R_NONE
    }
    else if (w.htag == H_EQUAL) {
      if (w.h_status == STATUS_PENDING) {
        if (w.live == 0) R_EQUAL_EMPTY
        else if (and { w.task0 == T_EQUAL, w.task5 == 0 }) {
          val same_ref = and { w.task1 == w.task3, w.task2 == w.task4 }
          val lsp = if (w.task1 == 1) w.data_sp else w.const_sp
          val rsp = if (w.task3 == 1) w.data_sp else w.const_sp
          val lfound = space_lookup_found(lsp, w.task2)
          val rfound = space_lookup_found(rsp, w.task4)
          val ltag = space_lookup_tag(lsp, w.task2)
          val rtag = space_lookup_tag(rsp, w.task4)
          if (or { lfound != 1, rfound != 1 }) R_NONE
          else if (same_ref) R_EQUAL_SAME_REF
          else if (ltag != rtag) R_EQUAL_DIFFERENT
          else R_EQUAL_PAIR
        }
        else if (and { w.task0 == T_EQUAL, w.task5 == 1 }) R_EQUAL_FIELD
        else R_NONE
      }
      else if (or { w.h_status == STATUS_DONE_OK, w.h_status == STATUS_DONE_FAIL }) R_EQUAL_PROPAGATE
      else R_NONE
    }
    else if (w.htag == H_BIND) {
      if (w.h_status == STATUS_PENDING) {
        if (w.live == 0) R_BIND_EMPTY
        else if (w.task0 == T_PAIR) R_BIND_PAIR
        else R_NONE
      }
      else R_NONE
    }
    else R_NONE
  }

  // Honest witness predicate: either disabled (enable == 0) with 128-bit lanes,
  // or enabled (enable == 1) on a canonical pre-state where ref_select_rule(w) == claimed_rule != R_NONE
  pure def is_honest_witness(w: DispatchWitness): bool = {
    or {
      w.enable == 0,
      and {
        w.enable == 1,
        canonical_pre_state(w),
        w.claimed_rule != R_NONE,
        ref_select_rule(w) == w.claimed_rule,
      },
    }
  }

  // =========================================================================
  // State Machine for Model Checking (`quint verify` / `quint run`)
  // =========================================================================

  var witness: DispatchWitness

  val default_space: TableSpace = {
    found0: 1,
    marker0: 0,
    tag0: 1,
    found1: 1,
    marker1: 1,
    tag1: 2,
  }

  action init = {
    // Initialize with a canonical PairTask witness where binder (task1) == 2
    witness' = {
      enable: 1,
      p0_running: 1,
      p1_not_pub: 1,
      p2_work_pos: 1,
      site_defined: 1,
      htag: H_BIND,
      h_status: STATUS_PENDING,
      live: 1,
      task0: T_PAIR,
      task1: 2, // binder NameId(2) >= 2!
      task2: 3, // sort TypeId(3) >= 2!
      task3: 0, // arg.kind == Const (0)
      task4: 0,
      task5: 0,
      const_sp: default_space,
      data_sp: default_space,
      claimed_rule: R_BIND_PAIR,
    }
  }

  action step = {
    nondet enable = oneOf(Set(0, 1, 2))
    nondet htag = oneOf(Set(H_NONE, H_VALIDATE, H_EQUAL, H_BIND))
    nondet h_status = oneOf(Set(STATUS_PENDING, STATUS_DONE_OK, STATUS_DONE_FAIL))
    nondet live = oneOf(Set(0, 1))
    nondet task0 = oneOf(Set(T_VALIDATE, T_EQUAL, T_PAIR))
    nondet task1 = oneOf(Set(0, 1, 2, 3))
    nondet task2 = oneOf(Set(0, 1))
    nondet task3 = oneOf(Set(0, 1, 2))
    nondet task4 = oneOf(Set(0, 1))
    nondet task5 = oneOf(Set(0, 1))
    nondet c_found0 = oneOf(Set(0, 1))
    nondet c_marker0 = oneOf(Set(0, 1))
    nondet c_tag0 = oneOf(Set(1, 2))
    nondet d_found0 = oneOf(Set(0, 1))
    nondet d_marker0 = oneOf(Set(0, 1))
    nondet d_tag0 = oneOf(Set(1, 2))
    nondet claimed_rule = oneOf(Set(
      R_VALIDATE_EMPTY, R_VALIDATE_INVALID, R_VALIDATE_OCCURRENCE,
      R_VALIDATE_FIELD, R_VALIDATE_PROPAGATE,
      R_EQUAL_EMPTY, R_EQUAL_SAME_REF, R_EQUAL_DIFFERENT,
      R_EQUAL_PAIR, R_EQUAL_FIELD, R_EQUAL_PROPAGATE,
      R_BIND_EMPTY, R_BIND_PAIR
    ))

    val csp: TableSpace = {
      found0: c_found0, marker0: c_marker0, tag0: c_tag0,
      found1: 1, marker1: 1, tag1: 2,
    }
    val dsp: TableSpace = {
      found0: d_found0, marker0: d_marker0, tag0: d_tag0,
      found1: 1, marker1: 0, tag1: 1,
    }

    witness' = {
      enable: enable,
      p0_running: 1,
      p1_not_pub: 1,
      p2_work_pos: 1,
      site_defined: 1,
      htag: htag,
      h_status: h_status,
      live: live,
      task0: task0,
      task1: task1,
      task2: task2,
      task3: task3,
      task4: task4,
      task5: task5,
      const_sp: csp,
      data_sp: dsp,
      claimed_rule: claimed_rule,
    }
  }

  // =========================================================================
  // Verification Invariants
  // =========================================================================

  // 1. COMPLETENESS of Post-4a8ba7acf (Gated) Circuit:
  // Every honest witness (including PairTask with binder >= 2 and disabled enable == 0
  // with non-boolean task1/task3) satisfies the gated circuit constraints.
  val inv_gated_complete: bool = {
    is_honest_witness(witness) implies eval_circuit_gated(witness).satisfied
  }

  // 2. SOUNDNESS of Post-4a8ba7acf (Gated) Circuit:
  // Every enabled witness (enable == 1) on a canonical pre-state that satisfies the
  // gated circuit for a valid rule ID is accepted by Core reference semantics for that exact rule.
  val inv_gated_sound: bool = {
    val ev = eval_circuit_gated(witness)
    and {
      witness.enable == 1,
      canonical_pre_state(witness),
      witness.claimed_rule != R_NONE,
      ev.satisfied,
    } implies (ref_select_rule(witness) == witness.claimed_rule)
  }

  // 3. KEY QUESTION (1a): After gating, can a rule that does NOT use the RefC field
  // still be fed an arbitrary space bit that changes the checked read outcome?
  // When vgate == 0, va_kind is forced to 0 and validate_marker is forced to 0 regardless of task1.
  // When egate == 0, left_kind and right_kind are forced to 0 and equal_differ is forced to 0
  // regardless of task1 and task3.
  val inv_non_refc_rule_immune_to_space_bit: bool = {
    val ev = eval_circuit_gated(witness)
    // Construct a mutated witness with arbitrary space bits task1 = 3, task3 = 2
    val w_mut = { ...witness, task1: 3, task3: if (ev.vgate == 0) 2 else witness.task3 }
    val ev_mut = eval_circuit_gated(w_mut)
    and {
      (ev.vgate == 0) implies and {
        ev.va_kind == 0,
        ev.validate_marker == 0,
        ev_mut.va_kind == 0,
        ev_mut.validate_marker == 0,
      },
      (ev.egate == 0) implies and {
        ev.left_kind == 0,
        ev.right_kind == 0,
        ev.equal_differ == 0,
      },
    }
  }

  // 4. KEY QUESTION (1b): Can a rule that DOES use RefC still get an ungated or wrong bit?
  // Whenever a rule that depends on validate_marker (R_VALIDATE_INVALID, R_VALIDATE_OCCURRENCE)
  // is selected and satisfied with enable == 1, vgate == 1 is forced, va_kind == task1 is forced,
  // and task1 is boolean-constrained in {0, 1}.
  // Whenever a rule that depends on equal_differ (R_EQUAL_DIFFERENT, R_EQUAL_PAIR, or R_EQUAL_SAME_REF)
  // is selected and satisfied with enable == 1, egate == 1 is forced, left_kind == task1 and
  // right_kind == task3 are forced, and both task1 and task3 are boolean-constrained in {0, 1}.
  val inv_refc_rule_gets_exact_boolean_bit: bool = {
    val ev = eval_circuit_gated(witness)
    val uses_validate_refc = Set(R_VALIDATE_INVALID, R_VALIDATE_OCCURRENCE).contains(witness.claimed_rule)
    val uses_equal_refc = Set(R_EQUAL_SAME_REF, R_EQUAL_DIFFERENT, R_EQUAL_PAIR).contains(witness.claimed_rule)
    and {
      (and { witness.enable == 1, ev.satisfied, uses_validate_refc }) implies and {
        ev.vgate == 1,
        ev.va_kind == witness.task1,
        cbool(witness.task1),
      },
      (and { witness.enable == 1, ev.satisfied, uses_equal_refc }) implies and {
        ev.egate == 1,
        ev.left_kind == witness.task1,
        ev.right_kind == witness.task3,
        cbool(witness.task1),
        cbool(witness.task3),
      },
    }
  }

  // 5. Combined invariant for `quint verify`
  val inv_all_post_4a8ba7acf_properties: bool = and {
    inv_gated_complete,
    inv_gated_sound,
    inv_non_refc_rule_immune_to_space_bit,
    inv_refc_rule_gets_exact_boolean_bit,
  }

  // 6. Pre-4a8ba7acf Completeness Invariant (FALSE before commit 4a8ba7acf!):
  // Demonstrates why pre-4a8ba7acf was INCOMPLETE on honest PairTask witnesses with binder >= 2.
  val inv_ungated_complete_bug: bool = {
    is_honest_witness(witness) implies eval_circuit_ungated(witness).satisfied
  }

  // =========================================================================
  // Unit Tests (`quint test`)
  // =========================================================================

  // Test 1: Honest PairTask witness with binder = 2 (NameId(2)) fails pre-4a8ba7acf
  // (`cbool(task[1])` fails in `read_a` and `read_b`), but passes post-4a8ba7acf!
  run pairTaskBinder2CompletenessTest = {
    init.then(all {
      assert(is_honest_witness(witness)),
      assert(not(eval_circuit_ungated(witness).satisfied)),
      assert(eval_circuit_gated(witness).satisfied),
      witness' = witness,
    })
  }

  // Test 2: Disabled circuit (enable == 0) with arbitrary 128-bit lanes task1 = 3, task3 = 2
  // fails pre-4a8ba7acf, but passes post-4a8ba7acf!
  run disabledCircuitArbitraryTaskLanesTest = {
    init.then(all {
      val w_dis = { ...witness, enable: 0, task1: 3, task3: 2 }
      all {
        assert(is_honest_witness(w_dis)),
        assert(not(eval_circuit_ungated(w_dis).satisfied)),
        assert(eval_circuit_gated(w_dis).satisfied),
        witness' = w_dis,
      }
    })
  }

  // Test 3: Validate unvisited task with corrupted non-boolean task1 = 2 is rejected
  // by post-4a8ba7acf (`vgate == 1` -> `va_kind == 2` -> `cbool(va_kind)` fails).
  run validateTaskNonBooleanRefKindRejectedTest = {
    init.then(all {
      val w_bad = {
        ...witness,
        enable: 1,
        htag: H_VALIDATE,
        h_status: STATUS_PENDING,
        live: 1,
        task0: T_VALIDATE,
        task1: 2, // forged non-boolean RefC kind!
        task2: 0,
        task3: 0, // unvisited -> vgate == 1
        claimed_rule: R_VALIDATE_OCCURRENCE,
      }
      all {
        assert(compute_vgate(w_bad) == 1),
        assert(not(eval_circuit_gated(w_bad).satisfied)),
        witness' = w_bad,
      }
    })
  }
}
