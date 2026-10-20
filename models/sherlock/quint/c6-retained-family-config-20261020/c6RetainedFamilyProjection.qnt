module c6RetainedFamilyProjection {
  import c6RetainedFamilyTypes.* from "./c6RetainedFamilyTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveContext.* from "./c6PositiveContext"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveProgramObservation.* from "./c6PositiveProgramObservation"

  // Positive normal construction only. Host fit, arithmetic fit, constructor
  // success and UTF8 projection are external premises, never refusal policies.
  pure def c6RFBoundsValues(boundsArg:C6CtxBounds):List[int] = List(
    boundsArg.code_rows, boundsArg.function_rows, boundsArg.code_child_slots,
    boundsArg.parameter_slots, boundsArg.pattern_rows, boundsArg.match_plan_rows,
    boundsArg.match_arm_rows, boundsArg.type_rows, boundsArg.constructor_rows,
    boundsArg.constructor_field_slots, boundsArg.max_arity, boundsArg.constant_nodes,
    boundsArg.constant_field_slots, boundsArg.data_nodes, boundsArg.data_field_slots,
    boundsArg.environment_cells, boundsArg.frames, boundsArg.helper_tasks,
    boundsArg.serialization_bytes, boundsArg.max_reference_work,
    boundsArg.max_helper_work, boundsArg.step_cap
  )

  pure def c6RFTextLanes(bytesArg:List[int]):List[int] =
    List(bytesArg.length()).concat(bytesArg)

  pure def c6RFPairLanes(pairArg:C6RFBytePair):List[int] =
    c6RFTextLanes(pairArg.first).concat(c6RFTextLanes(pairArg.second))

  pure def c6RFOptionalPairLanes(optionArg:C6RFPairOption):List[int] =
    match optionArg {
      | C6RFNoPair => List(0)
      | C6RFPairPresent(pairArg) => List(1).concat(c6RFPairLanes(pairArg))
    }

  pure def c6RFWidthListLanes(entriesArg:List[C6RFWidthEntry]):List[int] =
    entriesArg.foldl(List(entriesArg.length()), (priorArg,entryArg) =>
      priorArg.concat(c6RFTextLanes(entryArg.name)).append(entryArg.width))

  pure def c6RFOptionalWidthsLanes(optionArg:C6RFWidthsOption):List[int] =
    match optionArg {
      | C6RFNoWidths => List(0)
      | C6RFWidthsPresent(widthsArg) => List(1)
        .concat(c6RFWidthListLanes(widthsArg.index_widths))
        .concat(c6RFWidthListLanes(widthsArg.counter_widths))
        .concat(c6RFWidthListLanes(widthsArg.record_widths))
    }

  // Keep original layout options, repeated width entries and raw text bytes.
  // This observation exposes normal private constructor fields mathematically;
  // it is not a public Rust getter or authentication of an arbitrary raw record.
  pure def c6RFMetaBinding(retainedArg:C6RFRetained):C6RFBinding = {
    val layoutArg = retainedArg.cfg.layout
    val variableArg = c6RFPairLanes(layoutArg.layout_version)
      .concat(c6RFOptionalPairLanes(layoutArg.field_id))
      .concat(c6RFOptionalWidthsLanes(layoutArg.widths))
      .concat(c6RFOptionalPairLanes(layoutArg.scalar_layout_id))
    {
      admission_bounds:retainedArg.admission_bounds,
      layout:layoutArg, limits:retainedArg.limits,
      admission_values:c6RFBoundsValues(retainedArg.admission_bounds),
      layout_values:c6RFBoundsValues(layoutArg.bounds),
      capacity_values:List(layoutArg.capacities.name_rows,
        layoutArg.capacities.name_bytes,layoutArg.capacities.occurrence_path_slots,
        layoutArg.capacities.value_slots),
      layout_variable_expected:variableArg,
      layout_total_lanes:variableArg.length()+22+4+1,
    }
  }

  pure def c6RFContextInput(retainedArg:C6RFRetained):C6CtxInput = {
    context:{ext:retainedArg.cfg.ext,budgets:retainedArg.cfg.budgets},
    bounds:retainedArg.cfg.layout.bounds,
    capacities:retainedArg.cfg.layout.capacities,limits:retainedArg.limits,
  }

  // No allocation of padded state and no acceptance of an asserted geometry.
  // Arithmetic agrees with checked_new only on the declared positive fit domain.
  pure def c6RFGeometry(capsArg:C6CtxStateCaps):C6RFGeometry = {
    val imageArg = 48+13*capsArg.phys.max_frames
    val nodesArg = 5*capsArg.phys.max_data_nodes
    val fieldsArg = 2*capsArg.max_data_field_slots
    val envArg = 5*capsArg.phys.max_env_cells
    val valuesArg = 2*capsArg.phys.max_value_slots
    val dataArg = 7+nodesArg+fieldsArg+envArg+valuesArg
    val tasksArg = 7*capsArg.max_helper_tasks
    val helperArg = 31+tasksArg
    {
      caps:capsArg,image_total_lanes:imageArg,data_nodes_lanes:nodesArg,
      data_fields_lanes:fieldsArg,env_cells_lanes:envArg,
      value_slots_lanes:valuesArg,data_total_lanes:dataArg,
      helper_tasks_lanes:tasksArg,helper_total_lanes:helperArg,
      state_total_lanes:imageArg+dataArg+helperArg,
    }
  }

  // General input-to-observation composition. No fixture/expected import, case
  // selector, witness-supplied config, output literals or evaluator substitution.
  pure def c6RFObserve(inputArg:C6RFInput):C6RFObservation = {
    val contextArg = c6CtxObserve(c6RFContextInput(inputArg.retained))
    val capsArg = contextArg.state_context.caps
    {
      retained:inputArg.retained,
      meta_binding:c6RFMetaBinding(inputArg.retained),context:contextArg,
      caps:capsArg,geometry:c6RFGeometry(capsArg),
      program:c6PObserve(inputArg.program),
    }
  }

  // The independent data is accepted only by the final comparison predicate.
  pure def c6RFCompleteObservationsExact(
      inputsArg:List[C6RFInput],expectedArg:List[C6RFObservation]):bool =
    if(inputsArg.length()==expectedArg.length()) {
      range(0,inputsArg.length()).foldl(true,(priorArg,indexArg) => {
        val inputArg = inputsArg.nth(indexArg)
        val actualArg = c6RFObserve(inputArg)
        priorArg and actualArg==expectedArg.nth(indexArg)
          and actualArg.program.decoded_program==inputArg.program
          and actualArg.context.decoded_context==c6RFContextInput(inputArg.retained).context
          and c6PTableContinuity(actualArg.program)
          and c6PTextContinuity(actualArg.program)
      })
    } else false

  pure def c6RFLayoutCoupling(inputArg:C6RFInput):bool = {
    val actualArg = c6RFObserve(inputArg)
    val layoutArg = inputArg.retained.cfg.layout
    actualArg.caps==actualArg.context.state_context.caps
      and actualArg.geometry.caps==actualArg.caps
      and actualArg.caps.phys.max_frames==layoutArg.bounds.frames
      and actualArg.caps.phys.max_expressions==layoutArg.bounds.code_rows
      and actualArg.meta_binding.admission_values==c6RFBoundsValues(inputArg.retained.admission_bounds)
      and actualArg.meta_binding.layout_values==c6RFBoundsValues(layoutArg.bounds)
      and actualArg.context.state_context.limits.max_total_lanes==inputArg.retained.limits.max_state_lanes
      and actualArg.geometry.state_total_lanes<=inputArg.retained.limits.max_state_lanes
      and c6CtxLifetimeTransported(c6RFContextInput(inputArg.retained).context)
  }

  // Declared same-layout/budget/limits/program, absent-vs-present-zero pair.
  pure def c6RFOriginalPresenceContrast(leftArg:C6RFInput,rightArg:C6RFInput):bool = {
    val beforeArg = c6RFObserve(leftArg)
    val afterArg = c6RFObserve(rightArg)
    leftArg.retained.cfg.layout==rightArg.retained.cfg.layout
      and leftArg.retained.cfg.budgets==rightArg.retained.cfg.budgets
      and leftArg.retained.limits==rightArg.retained.limits
      and leftArg.retained.admission_bounds==rightArg.retained.admission_bounds
      and beforeArg.meta_binding==afterArg.meta_binding
      and beforeArg.context.encoded_lanes!=afterArg.context.encoded_lanes
      and beforeArg.context.decoded_context.ext!=afterArg.context.decoded_context.ext
      and beforeArg.context.effective_ext==afterArg.context.effective_ext
      and not(beforeArg.context.state_context.ext_present)
      and afterArg.context.state_context.ext_present
      and beforeArg.caps==afterArg.caps
      and beforeArg.geometry==afterArg.geometry
      and beforeArg.program==afterArg.program
  }

  // Changing only separate admission bounds cannot change layout-derived caps.
  pure def c6RFSeparateAdmissionContrast(leftArg:C6RFInput,rightArg:C6RFInput):bool = {
    val beforeArg = c6RFObserve(leftArg)
    val afterArg = c6RFObserve(rightArg)
    leftArg.retained.cfg==rightArg.retained.cfg
      and leftArg.retained.limits==rightArg.retained.limits
      and leftArg.retained.admission_bounds!=rightArg.retained.admission_bounds
      and beforeArg.meta_binding.admission_values!=afterArg.meta_binding.admission_values
      and beforeArg.meta_binding.layout_values==afterArg.meta_binding.layout_values
      and beforeArg.meta_binding.layout_variable_expected==afterArg.meta_binding.layout_variable_expected
      and beforeArg.context==afterArg.context and beforeArg.caps==afterArg.caps
      and beforeArg.geometry==afterArg.geometry and beforeArg.program==afterArg.program
  }
}
