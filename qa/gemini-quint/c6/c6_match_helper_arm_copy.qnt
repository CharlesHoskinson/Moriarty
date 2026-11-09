// -*- mode: Bluespec; -*-
//
// Quint formal model for Task 2.16 Commit 2 (dba5f344d):
// "Copy the arm pattern index before the PROG/CTX pattern capture"
// File: crates/mori-backend/src/sherlock_machine/c6_match_helper_step.rs (lines 150-352, specifically 163 and 213-226)
// Paired PROG/CTX capture: crates/mori-backend/src/sherlock_machine/c6_program_context_canonical.rs (lines 305-313, 2294-2363)
// Reference semantics:
//   - crates/mori-core/src/sherlock/helper_machine/families/structural.rs (lines 791-831)
//   - crates/mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs (lines 925-1051, 1300-1324)

module c6_match_helper_arm_copy {

  // =========================================================================
  // Constants: Control tags, Helper tags, Expression tags, Pattern tags, Rules
  // =========================================================================

  // Control frame tags (c6_match_helper_step.rs:34, c6_dispatch_step.rs:42)
  val K_CASES: int = 6
  val K_OTHER: int = 1

  // Helper record tags (c6_match_helper_step.rs:35)
  val H_NONE: int = 0
  val H_MATCH: int = 6
  val H_VALIDATE: int = 7

  // Result record tags
  val RES_NONE: int = 0
  val RES_MATCH_MATCHED: int = 13

  // Expression row tags in PROG (c6_dispatch_step.rs:647)
  val EXPR_MATCH: int = 7
  val EXPR_OTHER: int = 1

  // Pattern row tags in PROG (c6_match_helper_step.rs:230-232)
  val PAT_WILDCARD: int = 0
  val PAT_BIND: int = 1
  val PAT_CONSTRUCTOR: int = 2

  // Match helper phases (c6_match_helper_step.rs:44-48)
  val PHASE_ARM: int = 0
  val PHASE_OCCURRENCE: int = 1
  val PHASE_FIELDS: int = 2
  val PHASE_BINDINGS: int = 3

  // Match helper rules (c6_match_helper_step.rs:155-161)
  val R_MATCH_ENTRY: int = 1
  val R_MATCH_ARM: int = 2
  val R_MATCH_OCCURRENCE: int = 3
  val R_MATCH_FIELD: int = 4
  val R_MATCH_BINDING: int = 5

  // =========================================================================
  // Part A: Formal Model of the Arm Pattern Index Copy Gadget
  // (c6_match_helper_step.rs:201-226 & c6_program_context_canonical.rs:305-313, 2294-2363)
  // =========================================================================

  type ArmTable = {
    arm0_present: int,
    arm0_pattern: int,
    arm0_body: int,
    arm1_present: int,
    arm1_pattern: int,
    arm1_body: int,
  }

  type PatternTable = {
    pat0_present: int,
    pat0_tag: int,
    pat0_binder_present: int,
    pat0_binder: int,
    pat1_present: int,
    pat1_tag: int,
    pat1_binder_present: int,
    pat1_binder: int,
  }

  type CopyGadgetWitness = {
    enable: int,
    arm_index: int,
    // Witness cell allocated for `let pattern_index = em.add(arm.arm_row[0].clone(), Em::imm(0));`
    pattern_index_cell: int,
    // Identifier namespace tag: true if operand is in `%{scope}.c6progctx.temp.*`
    pattern_index_is_progctx_temp: bool,
    arms: ArmTable,
    patterns: PatternTable,
  }

  pure def cbool(x: int): bool = {
    or { x == 0, x == 1 }
  }

  pure def cond_select(cond: int, a: int, b: int): int = {
    if (cond == 1) a else b
  }

  pure def gassert(gate: int, cond: int): bool = {
    cond_select(gate, cond, 1) == 1
  }

  // emit_prog_arm_capture (c6_program_context_canonical.rs:2294-2313)
  pure def capture_arm_found(arms: ArmTable, idx: int): int = {
    if (idx == 0) arms.arm0_present else if (idx == 1) arms.arm1_present else 0
  }

  pure def capture_arm_pattern(arms: ArmTable, idx: int): int = {
    if (idx == 0) arms.arm0_pattern else if (idx == 1) arms.arm1_pattern else 0
  }

  // emit_prog_pattern_capture (c6_program_context_canonical.rs:2316-2363)
  pure def capture_pat_found(pats: PatternTable, idx: int): int = {
    if (idx == 0) pats.pat0_present else if (idx == 1) pats.pat1_present else 0
  }

  pure def capture_pat_tag(pats: PatternTable, idx: int): int = {
    if (idx == 0) pats.pat0_tag else if (idx == 1) pats.pat1_tag else 0
  }

  // Evaluate BHelperMatchArm's arm-to-pattern capture WITH the ZKIRv3 Instruction::Add(arm_row[0], 0)
  // copy constraint (commit dba5f344d, c6_match_helper_step.rs:216)
  pure def eval_copy_gadget_constrained(w: CopyGadgetWitness): bool = {
    val arm_found = cond_select(w.enable, capture_arm_found(w.arms, w.arm_index), 0)
    val arm_row_0 = cond_select(w.enable, capture_arm_pattern(w.arms, w.arm_index), 0)

    // c6_match_helper_step.rs:216:
    //   let pattern_index = em.add(arm.arm_row[0].clone(), Em::imm(0));
    // In ZKIRv3, Instruction::Add { a: arm.arm_row[0], b: Imm(0), output: pattern_index }
    // allocates `pattern_index` in `%{scope}.c6kit.temp.*` (so `pattern_index_is_progctx_temp == false`)
    // and emits the field constraint `pattern_index == arm_row_0 + 0`.
    val add_zero_copy_constraint = (w.pattern_index_cell == arm_row_0 + 0)

    // c6_program_context_canonical.rs:2321:
    //   reject_reserved(&self.scope, &inputs.pattern_index)?;
    val builder_accepts_operand = not(w.pattern_index_is_progctx_temp)

    val pat_found = cond_select(w.enable, capture_pat_found(w.patterns, w.pattern_index_cell), 0)

    and {
      cbool(w.enable),
      builder_accepts_operand,
      gassert(w.enable, arm_found),
      add_zero_copy_constraint,
      gassert(w.enable, pat_found),
    }
  }

  // Hypothetical broken gadget if `pattern_index_cell` were only witness-assigned
  // WITHOUT the Instruction::Add equality constraint:
  pure def eval_copy_gadget_unconstrained_assignment(w: CopyGadgetWitness): bool = {
    val arm_found = cond_select(w.enable, capture_arm_found(w.arms, w.arm_index), 0)
    val builder_accepts_operand = not(w.pattern_index_is_progctx_temp)
    val pat_found = cond_select(w.enable, capture_pat_found(w.patterns, w.pattern_index_cell), 0)

    and {
      cbool(w.enable),
      builder_accepts_operand,
      gassert(w.enable, arm_found),
      gassert(w.enable, pat_found),
    }
  }

  // =========================================================================
  // Part B: Full C6MatchHelperEmitter Step Circuit vs Reference Semantics
  // (c6_match_helper_step.rs:150-352, specifically line 163 `// BISECT: select(...)`)
  // =========================================================================

  type MatchStepWitness = {
    enable: int,
    rule: int,
    // Pre-state image & dispatch fields
    outcome_running: int,       // pre_image[L_OUTCOME] == 0
    not_publication: int,       // P1 guard
    remaining_work_pos: int,    // pre_image[L_REMAINING_WORK] > 0
    site_defined: int,          // dispatch site_defined == 1
    control_tag: int,           // k[0]: K_CASES (6) or other
    control_arms_start: int,    // k[3]
    control_arms_len: int,      // k[4]
    origin_expr_tag: int,       // expr(control_origin) tag: EXPR_MATCH (7) or other
    origin_match_plan_some: int,// expr(control_origin) plan: 0 (None) or 1 (Some)
    // Pre-state helper & result slots
    pre_htag: int,              // pre_helper[L_HELPER_RECORD]
    pre_rtag: int,              // pre_helper[L_RESULT_RECORD]
    pre_m_phase: int,           // h[M_PHASE]
    pre_m_arm: int,             // h[M_ARM]
    pre_m_arms_end: int,        // h[M_ARMS_END]
    pre_m_occ_len: int,         // h[M_OCC_LEN]
    pre_m_field_cursor: int,    // h[M_FIELD_CURSOR]
    pre_m_bind_cursor: int,     // h[M_BIND_CURSOR]
    pre_m_bind_len: int,        // h[M_BIND_LEN]
    // Arm & pattern capture witness
    copy_w: CopyGadgetWitness,
  }

  // Dispatch selection constraint `select(&mut self.composed, em, rule, inputs)`
  // (c6_match_helper_step.rs:139-148, c6_dispatch_step.rs:868-899, 1082-1102, 1256)
  pure def eval_dispatch_select(w: MatchStepWitness): bool = {
    val base_ok = and {
      w.outcome_running == 1,
      w.not_publication == 1,
      w.remaining_work_pos == 1,
      w.site_defined == 1,
    }
    val is_cases_site = and {
      w.control_tag == K_CASES,
      w.origin_expr_tag == EXPR_MATCH,
      w.origin_match_plan_some == 0,
    }
    val rule_guard = if (w.rule == R_MATCH_ENTRY) {
      and {
        base_ok,
        w.pre_htag == H_NONE,
        w.pre_rtag == RES_NONE,
        is_cases_site,
        w.control_arms_len > 0,
      }
    } else if (w.rule == R_MATCH_ARM) {
      and {
        base_ok,
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_ARM,
      }
    } else if (w.rule == R_MATCH_OCCURRENCE) {
      and {
        base_ok,
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_OCCURRENCE,
      }
    } else if (w.rule == R_MATCH_FIELD) {
      and {
        base_ok,
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_FIELDS,
      }
    } else if (w.rule == R_MATCH_BINDING) {
      and {
        base_ok,
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_BINDINGS,
      }
    } else false

    gassert(w.enable, if (rule_guard) 1 else 0)
  }

  // Body of C6MatchHelperEmitter::body (c6_match_helper_step.rs:150-352)
  // Parameter `include_select` toggles line 163:
  //   // BISECT: select(&mut self.composed, em, rule, inputs)?;
  pure def eval_match_helper_body(w: MatchStepWitness, include_select: bool): bool = {
    val select_ok = if (include_select) eval_dispatch_select(w) else true
    val rule_body_ok = if (w.rule == R_MATCH_ENTRY) {
      // Lines 180-198: only checks k[0] == 6 (Cases) and canonical post_helper (arms_start <= arms_end)
      and {
        gassert(w.enable, if (w.control_tag == K_CASES) 1 else 0),
        gassert(w.enable, if (w.control_arms_start >= 0 and w.control_arms_len >= 0) 1 else 0),
      }
    } else if (w.rule == R_MATCH_ARM) {
      // Lines 199-249: checks k[0] == 6, h[0] == 6, h[M_PHASE] == 0, arm_lt_end == 1,
      // and runs arm capture + pattern_index copy + pattern capture
      and {
        gassert(w.enable, if (w.control_tag == K_CASES) 1 else 0),
        gassert(w.enable, if (w.pre_htag == H_MATCH) 1 else 0),
        gassert(w.enable, if (w.pre_m_phase == PHASE_ARM) 1 else 0),
        gassert(w.enable, if (w.pre_m_arm < w.pre_m_arms_end) 1 else 0),
        w.copy_w.enable == w.enable,
        w.copy_w.arm_index == cond_select(w.enable, w.pre_m_arm, 0),
        eval_copy_gadget_constrained(w.copy_w),
      }
    } else if (w.rule == R_MATCH_OCCURRENCE) {
      // Lines 251-285
      and {
        gassert(w.enable, if (w.control_tag == K_CASES) 1 else 0),
        gassert(w.enable, if (w.pre_htag == H_MATCH) 1 else 0),
        gassert(w.enable, if (w.pre_m_phase == PHASE_OCCURRENCE) 1 else 0),
        gassert(w.enable, if (w.pre_m_arm < w.pre_m_arms_end) 1 else 0),
        gassert(w.enable, if (w.pre_m_occ_len > 0) 1 else 0),
        w.copy_w.enable == w.enable,
        w.copy_w.arm_index == cond_select(w.enable, w.pre_m_arm, 0),
        eval_copy_gadget_constrained(w.copy_w),
      }
    } else if (w.rule == R_MATCH_FIELD) {
      // Lines 286-315
      and {
        gassert(w.enable, if (w.control_tag == K_CASES) 1 else 0),
        gassert(w.enable, if (w.pre_htag == H_MATCH) 1 else 0),
        gassert(w.enable, if (w.pre_m_phase == PHASE_FIELDS) 1 else 0),
        gassert(w.enable, if (w.pre_m_arm < w.pre_m_arms_end) 1 else 0),
        w.copy_w.enable == w.enable,
        w.copy_w.arm_index == cond_select(w.enable, w.pre_m_arm, 0),
        eval_copy_gadget_constrained(w.copy_w),
      }
    } else if (w.rule == R_MATCH_BINDING) {
      // Lines 316-348
      and {
        gassert(w.enable, if (w.control_tag == K_CASES) 1 else 0),
        gassert(w.enable, if (w.pre_htag == H_MATCH) 1 else 0),
        gassert(w.enable, if (w.pre_m_phase == PHASE_BINDINGS) 1 else 0),
        gassert(w.enable, if (w.pre_m_arm < w.pre_m_arms_end) 1 else 0),
        gassert(w.enable, if (w.pre_m_bind_cursor < w.pre_m_bind_len) 1 else 0),
        w.copy_w.enable == w.enable,
        w.copy_w.arm_index == cond_select(w.enable, w.pre_m_arm, 0),
        eval_copy_gadget_constrained(w.copy_w),
      }
    } else false

    and {
      cbool(w.enable),
      select_ok,
      rule_body_ok,
    }
  }

  // Core Reference Semantics for the 5 Match Helper rules
  // (owner_runtime.rs:925-1051, 1300-1324; families/structural.rs:791-831)
  pure def ref_accepts_match_step(w: MatchStepWitness): bool = {
    // All Core steps require P0 (Running), P1 (not publication), P2 (remaining_work > 0),
    // site_defined == 1, and in structural.rs:800-805 (`plan_match`):
    //   `matches!(pre.expr(&pre.control_origin())?, ExpressionRowC::Match { plan: None, .. })`
    val common_pre = and {
      w.outcome_running == 1,
      w.not_publication == 1,
      w.remaining_work_pos == 1,
      w.site_defined == 1,
      w.control_tag == K_CASES,
      w.origin_expr_tag == EXPR_MATCH,
      w.origin_match_plan_some == 0,
    }
    if (not(common_pre)) false
    else if (w.rule == R_MATCH_ENTRY) {
      and {
        w.pre_htag == H_NONE,
        w.pre_rtag == RES_NONE,
        w.control_arms_start >= 0,
        w.control_arms_len > 0,
      }
    } else if (w.rule == R_MATCH_ARM) {
      val arm_found = capture_arm_found(w.copy_w.arms, w.pre_m_arm) == 1
      val arm_pat = capture_arm_pattern(w.copy_w.arms, w.pre_m_arm)
      val pat_found = capture_pat_found(w.copy_w.patterns, arm_pat) == 1
      and {
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_ARM,
        w.pre_m_arm < w.pre_m_arms_end,
        arm_found,
        pat_found,
      }
    } else if (w.rule == R_MATCH_OCCURRENCE) {
      val arm_found = capture_arm_found(w.copy_w.arms, w.pre_m_arm) == 1
      val arm_pat = capture_arm_pattern(w.copy_w.arms, w.pre_m_arm)
      val pat_found = capture_pat_found(w.copy_w.patterns, arm_pat) == 1
      and {
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_OCCURRENCE,
        w.pre_m_arm < w.pre_m_arms_end,
        w.pre_m_occ_len > 0,
        arm_found,
        pat_found,
      }
    } else if (w.rule == R_MATCH_FIELD) {
      val arm_found = capture_arm_found(w.copy_w.arms, w.pre_m_arm) == 1
      val arm_pat = capture_arm_pattern(w.copy_w.arms, w.pre_m_arm)
      val pat_found = capture_pat_found(w.copy_w.patterns, arm_pat) == 1
      and {
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_FIELDS,
        w.pre_m_arm < w.pre_m_arms_end,
        arm_found,
        pat_found,
      }
    } else if (w.rule == R_MATCH_BINDING) {
      val arm_found = capture_arm_found(w.copy_w.arms, w.pre_m_arm) == 1
      val arm_pat = capture_arm_pattern(w.copy_w.arms, w.pre_m_arm)
      val pat_found = capture_pat_found(w.copy_w.patterns, arm_pat) == 1
      and {
        w.pre_htag == H_MATCH,
        w.pre_m_phase == PHASE_BINDINGS,
        w.pre_m_arm < w.pre_m_arms_end,
        w.pre_m_bind_cursor < w.pre_m_bind_len,
        arm_found,
        pat_found,
      }
    } else false
  }

  // =========================================================================
  // State Machine for Model Checking (`quint verify` / `quint run`)
  // =========================================================================

  var mw: MatchStepWitness

  val default_arms: ArmTable = {
    arm0_present: 1, arm0_pattern: 1, arm0_body: 0,
    arm1_present: 1, arm1_pattern: 0, arm1_body: 1,
  }

  val default_patterns: PatternTable = {
    pat0_present: 1, pat0_tag: PAT_WILDCARD, pat0_binder_present: 0, pat0_binder: 0,
    pat1_present: 1, pat1_tag: PAT_CONSTRUCTOR, pat1_binder_present: 1, pat1_binder: 2,
  }

  action init = {
    mw' = {
      enable: 1,
      rule: R_MATCH_ARM,
      outcome_running: 1,
      not_publication: 1,
      remaining_work_pos: 1,
      site_defined: 1,
      control_tag: K_CASES,
      control_arms_start: 0,
      control_arms_len: 1,
      origin_expr_tag: EXPR_MATCH,
      origin_match_plan_some: 0,
      pre_htag: H_MATCH,
      pre_rtag: RES_NONE,
      pre_m_phase: PHASE_ARM,
      pre_m_arm: 0,
      pre_m_arms_end: 1,
      pre_m_occ_len: 1,
      pre_m_field_cursor: 0,
      pre_m_bind_cursor: 0,
      pre_m_bind_len: 1,
      copy_w: {
        enable: 1,
        arm_index: 0,
        pattern_index_cell: 1, // arm0_pattern == 1
        pattern_index_is_progctx_temp: false,
        arms: default_arms,
        patterns: default_patterns,
      },
    }
  }

  action step = {
    nondet enable = oneOf(Set(0, 1))
    nondet rule = oneOf(Set(R_MATCH_ENTRY, R_MATCH_ARM, R_MATCH_OCCURRENCE, R_MATCH_FIELD, R_MATCH_BINDING))
    nondet outcome_running = oneOf(Set(0, 1))
    nondet remaining_work_pos = oneOf(Set(0, 1))
    nondet site_defined = oneOf(Set(0, 1))
    nondet origin_expr_tag = oneOf(Set(EXPR_MATCH, EXPR_OTHER))
    nondet origin_match_plan_some = oneOf(Set(0, 1))
    nondet pre_htag = oneOf(Set(H_NONE, H_MATCH, H_VALIDATE))
    nondet pre_rtag = oneOf(Set(RES_NONE, RES_MATCH_MATCHED))
    nondet pre_m_phase = oneOf(Set(PHASE_ARM, PHASE_OCCURRENCE, PHASE_FIELDS, PHASE_BINDINGS))
    nondet pre_m_arm = oneOf(Set(0, 1))
    nondet control_arms_len = oneOf(Set(0, 1))
    nondet pat_idx_cell = oneOf(Set(0, 1, 2))

    val eff_arm_idx = cond_select(enable, pre_m_arm, 0)

    mw' = {
      enable: enable,
      rule: rule,
      outcome_running: outcome_running,
      not_publication: 1,
      remaining_work_pos: remaining_work_pos,
      site_defined: site_defined,
      control_tag: K_CASES,
      control_arms_start: 0,
      control_arms_len: control_arms_len,
      origin_expr_tag: origin_expr_tag,
      origin_match_plan_some: origin_match_plan_some,
      pre_htag: pre_htag,
      pre_rtag: pre_rtag,
      pre_m_phase: pre_m_phase,
      pre_m_arm: pre_m_arm,
      pre_m_arms_end: 1,
      pre_m_occ_len: 1,
      pre_m_field_cursor: 0,
      pre_m_bind_cursor: 0,
      pre_m_bind_len: 1,
      copy_w: {
        enable: enable,
        arm_index: eff_arm_idx,
        pattern_index_cell: pat_idx_cell,
        pattern_index_is_progctx_temp: false,
        arms: default_arms,
        patterns: default_patterns,
      },
    }
  }

  // =========================================================================
  // Verification Invariants
  // =========================================================================

  // 1. KEY QUESTION (2a): Is the copied arm pattern index equality-constrained
  // to the original (a copy constraint, not just an assignment)?
  // And is the constrained value unchanged as the commit claims?
  val inv_copy_gadget_equality_enforced: bool = {
    val cw = mw.copy_w
    val orig_arm_row_0 = cond_select(cw.enable, capture_arm_pattern(cw.arms, cw.arm_index), 0)
    eval_copy_gadget_constrained(cw) implies and {
      // Strictly equality-constrained to arm.arm_row[0]:
      cw.pattern_index_cell == orig_arm_row_0,
      // And the captured pattern tag is identical to capturing at arm.arm_row[0] directly:
      cond_select(cw.enable, capture_pat_tag(cw.patterns, cw.pattern_index_cell), 0)
        == cond_select(cw.enable, capture_pat_tag(cw.patterns, orig_arm_row_0), 0),
    }
  }

  // 2. COMPLETENESS of the copy gadget & Match Helper Step:
  // Every honest witness (where ref_accepts_match_step(mw) holds and pattern_index_cell
  // is honestly assigned to arm_row[0]) satisfies the full circuit (both with and without select).
  val inv_copy_and_step_complete: bool = {
    val cw = mw.copy_w
    val orig_arm_row_0 = cond_select(cw.enable, capture_arm_pattern(cw.arms, cw.arm_index), 0)
    val honest_copy_cell = (cw.pattern_index_cell == orig_arm_row_0)
    and {
      mw.enable == 1,
      ref_accepts_match_step(mw),
      honest_copy_cell,
      cw.enable == mw.enable,
      cw.arm_index == cond_select(mw.enable, mw.pre_m_arm, 0),
      not(cw.pattern_index_is_progctx_temp),
    } implies and {
      eval_match_helper_body(mw, true),
      eval_match_helper_body(mw, false),
    }
  }

  // 3. SOUNDNESS of Match Helper Step WHEN `select(...)` at line 163 is ENABLED:
  val inv_step_sound_when_select_enabled: bool = {
    and {
      mw.enable == 1,
      eval_match_helper_body(mw, true),
    } implies ref_accepts_match_step(mw)
  }

  // 4. Combined invariant for `quint verify` on the Commit 2 copy gadget + intended step circuit:
  val inv_commit2_copy_and_intended_step: bool = and {
    inv_copy_gadget_equality_enforced,
    inv_copy_and_step_complete,
    inv_step_sound_when_select_enabled,
  }

  // 5. SOUNDNESS of Match Helper Step in the CURRENT WORKING TREE (`// BISECT: select(...)`
  // commented out at `c6_match_helper_step.rs:163`):
  // Expected to FAIL with concrete counterexamples!
  val inv_step_sound_current_working_tree: bool = {
    and {
      mw.enable == 1,
      eval_match_helper_body(mw, false),
    } implies ref_accepts_match_step(mw)
  }

  // =========================================================================
  // Unit Tests (`quint test`)
  // =========================================================================

  // Test 1: Passing `arm.arm_row[0]` directly without `em.add(..., 0)` has
  // `pattern_index_is_progctx_temp == true`, which `reject_reserved` rejects at build time.
  // With `em.add(arm.arm_row[0], 0)`, `pattern_index_is_progctx_temp == false` and succeeds.
  run reservedOperandFixedByAddZeroCopyTest = {
    init.then(all {
      val direct_w = { ...mw.copy_w, pattern_index_is_progctx_temp: true }
      all {
        assert(not(eval_copy_gadget_constrained(direct_w))),
        assert(eval_copy_gadget_constrained(mw.copy_w)),
        mw' = mw,
      }
    })
  }

  // Test 2: Forged `pattern_index_cell = 0` when `arm0_pattern = 1` (trying to substitute
  // PAT_WILDCARD at index 0 for PAT_CONSTRUCTOR at index 1) would pass an unconstrained
  // assignment, but is REJECTED by `Instruction::Add(arm.arm_row[0], 0)`!
  run forgedPatternIndexRejectedByCopyConstraintTest = {
    init.then(all {
      val forged_cw = { ...mw.copy_w, pattern_index_cell: 0 } // arm0_pattern is 1, forged to 0!
      all {
        assert(eval_copy_gadget_unconstrained_assignment(forged_cw)),
        assert(not(eval_copy_gadget_constrained(forged_cw))),
        mw' = mw,
      }
    })
  }

  // Test 3: Counterexample demonstrating UNSOUNDNESS of `c6_match_helper_step.rs:163`
  // (`// BISECT: select(&mut self.composed, em, rule, inputs)?;`) on `BHelperMatchEntry`:
  // When `pre_rtag == RES_MATCH_MATCHED` (or `pre_htag == H_VALIDATE`, or `control_arms_len == 0`,
  // or `outcome_running == 0`, or `origin_match_plan_some == 1`), Core rejects `BHelperMatchEntry`,
  // `eval_match_helper_body(bad_w, true)` rejects, but `eval_match_helper_body(bad_w, false)` ACCEPTS!
  run bisectCommentedSelectUnsoundnessTest = {
    init.then(all {
      val bad_entry_w = {
        ...mw,
        rule: R_MATCH_ENTRY,
        outcome_running: 0,          // Halted/Rejected machine!
        control_arms_len: 0,         // Empty arms (Core selects KMachineCasesEmpty)!
        origin_match_plan_some: 1,   // Match { plan: Some(_) } (Core selects XAdtHelperPlanEntry)!
        pre_htag: H_VALIDATE,        // Active Validate helper already present!
        pre_rtag: RES_MATCH_MATCHED, // Match result already present!
      }
      all {
        assert(not(ref_accepts_match_step(bad_entry_w))),
        assert(not(eval_match_helper_body(bad_entry_w, true))),
        assert(eval_match_helper_body(bad_entry_w, false)),
        mw' = bad_entry_w,
      }
    })
  }
}
