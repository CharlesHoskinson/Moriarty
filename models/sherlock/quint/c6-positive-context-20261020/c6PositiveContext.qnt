module c6PositiveContext {
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"

  // Source-derived constants and exact22-lane positive representation only.
  // All usize fields fit the selected host; max_lifetime_work is full u128.
  // No admission, owner authentication, malformed decoder or resource policy.
  pure val C6_CTX_MAGIC:int = 18918541878711089
  pure val C6_CTX_VERSION:int = 1
  pure val C6_CTX_TAG:int = 1129601108

  pure val c6CtxZeroExt: C6CtxExtCaps = {
    plan_case_slots: 0,
    field_path_slots: 0,
    pattern_child_slots: 0,
    input_rows: 0,
    asset_rows: 0,
    state_rows: 0,
    account_rows: 0,
    liability_rows: 0,
    await_site_rows: 0,
    await_binder_slots: 0,
    balance_cells: 0,
    journal_rows: 0,
    pending_op_rows: 0,
    observations_per_invocation: 0,
    max_lifetime_work: 0,
    resume_rows: 0
  }

  pure def c6CtxEffectiveExt(optionArg:C6CtxExt): C6CtxExtCaps =
    match optionArg {
      | C6CtxAbsent => c6CtxZeroExt
      | C6CtxPresent(extArg) => extArg
    }

  // Preserve every field in source declaration order, with raw lifetime at14.
  pure def c6CtxExtValues(extArg:C6CtxExtCaps): List[int] = List(
      extArg.plan_case_slots, extArg.field_path_slots, extArg.pattern_child_slots, extArg.input_rows, extArg.asset_rows, extArg.state_rows, extArg.account_rows, extArg.liability_rows, extArg.await_site_rows, extArg.await_binder_slots, extArg.balance_cells, extArg.journal_rows, extArg.pending_op_rows, extArg.observations_per_invocation, extArg.max_lifetime_work, extArg.resume_rows
  )

  pure def c6CtxEncode(payloadArg:C6CtxPayload): List[int] = {
    val extLanes = match payloadArg.ext {
      | C6CtxAbsent => List(0).concat(c6CtxExtValues(c6CtxZeroExt))
      | C6CtxPresent(extArg) => List(1).concat(c6CtxExtValues(extArg))
    }
    List(C6_CTX_MAGIC, C6_CTX_VERSION, C6_CTX_TAG)
      .concat(extLanes)
      .concat(List(payloadArg.budgets.type_expr_nodes, payloadArg.budgets.template_bytes))
  }

  // Defined contract only for exact positive images. No malformed-input claim.
  // Header, bool-tag, zero-padding and host-fit premises come from the contract.
  pure def c6CtxDecodePositive(laneArg:List[int]): C6CtxPayload = {
    ext: if (laneArg.nth(3) == 1) C6CtxPresent({
        plan_case_slots: laneArg.nth(4),
        field_path_slots: laneArg.nth(5),
        pattern_child_slots: laneArg.nth(6),
        input_rows: laneArg.nth(7),
        asset_rows: laneArg.nth(8),
        state_rows: laneArg.nth(9),
        account_rows: laneArg.nth(10),
        liability_rows: laneArg.nth(11),
        await_site_rows: laneArg.nth(12),
        await_binder_slots: laneArg.nth(13),
        balance_cells: laneArg.nth(14),
        journal_rows: laneArg.nth(15),
        pending_op_rows: laneArg.nth(16),
        observations_per_invocation: laneArg.nth(17),
        max_lifetime_work: laneArg.nth(18),
        resume_rows: laneArg.nth(19)
      }) else C6CtxAbsent,
    budgets: {type_expr_nodes:laneArg.nth(20), template_bytes:laneArg.nth(21)},
  }

  pure def c6CtxDerivePhysCaps(inputArg:C6CtxInput): C6CtxPhysCaps = {
    val effectiveArg = c6CtxEffectiveExt(inputArg.context.ext)
    {
      max_frames: inputArg.bounds.frames,
      max_expressions: inputArg.bounds.code_rows,
      max_names: inputArg.capacities.name_rows,
      max_arms: inputArg.bounds.match_arm_rows,
      max_children: inputArg.bounds.code_child_slots,
      max_value_slots: inputArg.capacities.value_slots,
      max_constants: inputArg.bounds.constant_nodes,
      max_data_nodes: inputArg.bounds.data_nodes,
      max_env_cells: inputArg.bounds.environment_cells,
      max_functions: inputArg.bounds.function_rows,
      max_arity: inputArg.bounds.max_arity,
      max_types: inputArg.bounds.type_rows,
      max_occurrences: inputArg.bounds.code_rows,
      max_occurrence_paths: inputArg.capacities.occurrence_path_slots,
      max_parameters: inputArg.bounds.parameter_slots,
      max_patterns: inputArg.bounds.pattern_rows,
      max_pattern_children: effectiveArg.pattern_child_slots,
      max_constructors: inputArg.bounds.constructor_rows,
      max_ctor_fields: inputArg.bounds.constructor_field_slots,
      max_decl_templates: if (inputArg.bounds.type_rows >= 4) inputArg.bounds.type_rows - 4 else 0,
      max_plan_nodes: inputArg.bounds.match_plan_rows,
      max_plan_cases: effectiveArg.plan_case_slots,
      max_field_paths: effectiveArg.field_path_slots,
      max_constant_fields: inputArg.bounds.constant_field_slots,
      max_name_bytes: inputArg.capacities.name_bytes,
      max_type_expr_nodes: inputArg.context.budgets.type_expr_nodes,
      max_template_bytes: inputArg.context.budgets.template_bytes
    }
  }

  // Original presence is retained separately from effective allzero capacities.
  // Only max_state_lanes becomes the state-context total-lane limit.
  pure def c6CtxDeriveState(inputArg:C6CtxInput): C6CtxStateView = {
    caps: {
      phys:c6CtxDerivePhysCaps(inputArg),
      max_data_field_slots:inputArg.bounds.data_field_slots,
      max_helper_tasks:inputArg.bounds.helper_tasks,
    },
    ext_present: match inputArg.context.ext {
      | C6CtxAbsent => false
      | C6CtxPresent(_) => true
    },
    limits: {max_total_lanes:inputArg.limits.max_state_lanes},
  }

  pure def c6CtxStateDescriptor(stateArg:C6CtxStateView): List[int] = List(
      stateArg.caps.phys.max_frames, stateArg.caps.phys.max_expressions, stateArg.caps.phys.max_names, stateArg.caps.phys.max_arms, stateArg.caps.phys.max_children, stateArg.caps.phys.max_value_slots, stateArg.caps.phys.max_constants, stateArg.caps.phys.max_data_nodes, stateArg.caps.phys.max_env_cells, stateArg.caps.phys.max_functions, stateArg.caps.phys.max_arity, stateArg.caps.phys.max_types, stateArg.caps.phys.max_occurrences, stateArg.caps.phys.max_occurrence_paths, stateArg.caps.phys.max_parameters, stateArg.caps.phys.max_patterns, stateArg.caps.phys.max_pattern_children, stateArg.caps.phys.max_constructors, stateArg.caps.phys.max_ctor_fields, stateArg.caps.phys.max_decl_templates, stateArg.caps.phys.max_plan_nodes, stateArg.caps.phys.max_plan_cases, stateArg.caps.phys.max_field_paths, stateArg.caps.phys.max_constant_fields, stateArg.caps.phys.max_name_bytes, stateArg.caps.phys.max_type_expr_nodes, stateArg.caps.phys.max_template_bytes,
      stateArg.caps.max_data_field_slots, stateArg.caps.max_helper_tasks
  )

  pure def c6CtxObserve(inputArg:C6CtxInput): C6CtxObservation = {
    val encodedArg = c6CtxEncode(inputArg.context)
    val stateArg = c6CtxDeriveState(inputArg)
    {
      encoded_lanes:encodedArg,
      decoded_context:c6CtxDecodePositive(encodedArg),
      effective_ext:c6CtxEffectiveExt(inputArg.context.ext),
      state_context:stateArg,
      state_descriptor:c6CtxStateDescriptor(stateArg),
    }
  }

  // Independent data only enters this final comparison, never observation code.
  pure def c6CtxCompleteObservationsExact(
      casesArg:List[C6CtxInputCase], expectedArg:List[C6CtxObservation]): bool =
    if (casesArg.length() == expectedArg.length()) {
      range(0, casesArg.length()).foldl(true, (priorArg, indexArg) => {
        val inputArg = casesArg.nth(indexArg).input
        val actualArg = c6CtxObserve(inputArg)
        priorArg and actualArg == expectedArg.nth(indexArg)
          and actualArg.decoded_context == inputArg.context
          and actualArg.encoded_lanes.length() == 22
          and actualArg.state_descriptor.length() == 29
      })
    } else false

  // General pair property with declared same-bounds/budgets/zero-ext premises.
  pure def c6CtxPresenceSeparated(absentArg:C6CtxInput, presentArg:C6CtxInput): bool = {
    val leftArg = c6CtxObserve(absentArg)
    val rightArg = c6CtxObserve(presentArg)
    leftArg.encoded_lanes != rightArg.encoded_lanes
      and leftArg.decoded_context.ext != rightArg.decoded_context.ext
      and leftArg.effective_ext == rightArg.effective_ext
      and leftArg.state_context.caps == rightArg.state_context.caps
      and leftArg.state_descriptor == rightArg.state_descriptor
      and not(leftArg.state_context.ext_present)
      and rightArg.state_context.ext_present
  }

  // Raw integer transport, not a lifetime-work accounting or validation rule.
  pure def c6CtxLifetimeTransported(payloadArg:C6CtxPayload): bool = {
    val effectiveArg = c6CtxEffectiveExt(payloadArg.ext)
    val encodedArg = c6CtxEncode(payloadArg)
    encodedArg.nth(18) == effectiveArg.max_lifetime_work
      and c6CtxDecodePositive(encodedArg) == payloadArg
  }
}
