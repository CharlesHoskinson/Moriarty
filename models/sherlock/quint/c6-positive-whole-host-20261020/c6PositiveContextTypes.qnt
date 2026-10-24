module c6PositiveContextTypes {
  // Exact integer vocabulary on the declared host-fit positive code-image domain.
  // Width/malformed-input/authority checks are premises, not implemented here.
  type C6CtxExtCaps = {
    plan_case_slots: int,
    field_path_slots: int,
    pattern_child_slots: int,
    input_rows: int,
    asset_rows: int,
    state_rows: int,
    account_rows: int,
    liability_rows: int,
    await_site_rows: int,
    await_binder_slots: int,
    balance_cells: int,
    journal_rows: int,
    pending_op_rows: int,
    observations_per_invocation: int,
    max_lifetime_work: int,
    resume_rows: int,
  }
  type C6CtxExt = C6CtxAbsent | C6CtxPresent(C6CtxExtCaps)
  type C6CtxBudgets = {type_expr_nodes:int, template_bytes:int}
  type C6CtxPayload = {ext:C6CtxExt, budgets:C6CtxBudgets}
  type C6CtxBounds = {
    code_rows: int,
    function_rows: int,
    code_child_slots: int,
    parameter_slots: int,
    pattern_rows: int,
    match_plan_rows: int,
    match_arm_rows: int,
    type_rows: int,
    constructor_rows: int,
    constructor_field_slots: int,
    max_arity: int,
    constant_nodes: int,
    constant_field_slots: int,
    data_nodes: int,
    data_field_slots: int,
    environment_cells: int,
    frames: int,
    helper_tasks: int,
    serialization_bytes: int,
    max_reference_work: int,
    max_helper_work: int,
    step_cap: int,
  }
  type C6CtxLayoutCaps = {
    name_rows:int, name_bytes:int, occurrence_path_slots:int, value_slots:int,
  }
  type C6CtxHostLimits = {
    max_total_lanes:int, max_state_lanes:int, max_metadata_bytes:int,
    max_string_bytes:int, max_width_entries:int,
  }
  type C6CtxInput = {
    context:C6CtxPayload, bounds:C6CtxBounds,
    capacities:C6CtxLayoutCaps, limits:C6CtxHostLimits,
  }
  type C6CtxInputCase = {case_id:str, input:C6CtxInput}
  type C6CtxPhysCaps = {
    max_frames: int,
    max_expressions: int,
    max_names: int,
    max_arms: int,
    max_children: int,
    max_value_slots: int,
    max_constants: int,
    max_data_nodes: int,
    max_env_cells: int,
    max_functions: int,
    max_arity: int,
    max_types: int,
    max_occurrences: int,
    max_occurrence_paths: int,
    max_parameters: int,
    max_patterns: int,
    max_pattern_children: int,
    max_constructors: int,
    max_ctor_fields: int,
    max_decl_templates: int,
    max_plan_nodes: int,
    max_plan_cases: int,
    max_field_paths: int,
    max_constant_fields: int,
    max_name_bytes: int,
    max_type_expr_nodes: int,
    max_template_bytes: int,
  }
  type C6CtxStateCaps = {
    phys:C6CtxPhysCaps, max_data_field_slots:int, max_helper_tasks:int,
  }
  type C6CtxStateLimits = {max_total_lanes:int}
  type C6CtxStateView = {
    caps:C6CtxStateCaps, ext_present:bool, limits:C6CtxStateLimits,
  }
  type C6CtxObservation = {
    encoded_lanes:List[int], decoded_context:C6CtxPayload,
    effective_ext:C6CtxExtCaps, state_context:C6CtxStateView,
    state_descriptor:List[int],
  }
}
