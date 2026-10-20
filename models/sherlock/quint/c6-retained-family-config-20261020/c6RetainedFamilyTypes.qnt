module c6RetainedFamilyTypes {
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  type C6RFBytePair = {first:List[int],second:List[int]}
  type C6RFPairOption = C6RFNoPair | C6RFPairPresent(C6RFBytePair)
  type C6RFWidthEntry = {name:List[int],width:int}
  type C6RFWidths = {
    index_widths:List[C6RFWidthEntry], counter_widths:List[C6RFWidthEntry],
    record_widths:List[C6RFWidthEntry],
  }
  type C6RFWidthsOption = C6RFNoWidths | C6RFWidthsPresent(C6RFWidths)
  type C6RFLayout = {
    layout_version:C6RFBytePair, field_id:C6RFPairOption,
    widths:C6RFWidthsOption, scalar_layout_id:C6RFPairOption,
    bounds:C6CtxBounds, capacities:C6CtxLayoutCaps,
    boundary_capacities_present:bool,
  }
  type C6RFConfig = {layout:C6RFLayout,ext:C6CtxExt,budgets:C6CtxBudgets}
  type C6RFRetained = {
    cfg:C6RFConfig,admission_bounds:C6CtxBounds,limits:C6CtxHostLimits,
  }
  type C6RFInput = {case_id:str,retained:C6RFRetained,program:C6PProgram}
  type C6RFBinding = {
    admission_bounds:C6CtxBounds,layout:C6RFLayout,limits:C6CtxHostLimits,
    admission_values:List[int],layout_values:List[int],capacity_values:List[int],
    layout_variable_expected:List[int],layout_total_lanes:int,
  }
  type C6RFGeometry = {
    caps:C6CtxStateCaps,image_total_lanes:int,data_nodes_lanes:int,
    data_fields_lanes:int,env_cells_lanes:int,value_slots_lanes:int,
    data_total_lanes:int,helper_tasks_lanes:int,helper_total_lanes:int,
    state_total_lanes:int,
  }
  type C6RFObservation = {
    retained:C6RFRetained,meta_binding:C6RFBinding,context:C6CtxObservation,
    caps:C6CtxStateCaps,geometry:C6RFGeometry,program:C6PObservation,
  }
}
