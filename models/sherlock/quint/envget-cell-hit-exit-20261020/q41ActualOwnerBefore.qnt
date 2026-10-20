module q41ActualOwnerBefore {
  import q41EnvGetOwnerTypes.* from "./q41EnvGetOwnerTypes"
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"

  pure val actualBefore:C6WholeHost = {
    admission: {
      identity: {
        profile_id: 2,
        grammar_domain: C6MDedicatedSorts,
        finance: C6MFinanceAbsent,
        layout_version: {
          first: List(101,110,118,103,101,116,45,111,119,110,101,114,45,112,111,115,105,116,105,118,101),
          second: List(49)
        },
        layout_digest: C6MUnselected,
        bounds_digest: C6MUnselected,
        field_id: C6MNoPair,
        program_membership_digest: C6MUnselected
      },
      bounds: {
        code_rows: 64,
        function_rows: 16,
        code_child_slots: 64,
        parameter_slots: 64,
        pattern_rows: 32,
        match_plan_rows: 32,
        match_arm_rows: 32,
        type_rows: 16,
        constructor_rows: 32,
        constructor_field_slots: 64,
        max_arity: 2,
        constant_nodes: 64,
        constant_field_slots: 64,
        data_nodes: 64,
        data_field_slots: 128,
        environment_cells: 64,
        frames: 16,
        helper_tasks: 64,
        serialization_bytes: 65536,
        max_reference_work: 256,
        max_helper_work: 1024,
        step_cap: 256
      },
      view: {
        entry: 0,
        expressions: List(C6PELet({
          binder: 0,
          value: 1,
          body: 2
        }),C6PELiteral({
          constant: 0
        }),C6PELet({
          binder: 1,
          value: 3,
          body: 4
        }),C6PEVar({
          binder: 0
        }),C6PEVar({
          binder: 1
        })),
        occurrences: List({
          owner_is_function: false,
          function: 0,
          path: {
            start: 0,
            len: 0
          }
        },{
          owner_is_function: false,
          function: 0,
          path: {
            start: 0,
            len: 1
          }
        },{
          owner_is_function: false,
          function: 0,
          path: {
            start: 1,
            len: 1
          }
        },{
          owner_is_function: false,
          function: 0,
          path: {
            start: 2,
            len: 2
          }
        },{
          owner_is_function: false,
          function: 0,
          path: {
            start: 4,
            len: 2
          }
        }),
        occurrence_paths: List(0,1,1,0,1,1),
        children: List(),
        functions: List(),
        parameters: List(),
        arms: List(),
        patterns: List(),
        pattern_children: List(),
        types: List(),
        type_args: List(),
        constructors: List(),
        ctor_fields: List(),
        decl_param_counts: List(),
        decl_ctor_counts: List(),
        decl_templates: List(),
        plan_nodes: List(),
        plan_cases: List(),
        field_paths: List(),
        constants: List(C6PKUInt({
          marker_free: true,
          n: 7
        })),
        constant_fields: List(),
        names: List(List(120),List(121))
      },
      layout: {
        layout_version: {
          first: List(101,110,118,103,101,116,45,111,119,110,101,114,45,112,111,115,105,116,105,118,101),
          second: List(49)
        },
        field_id: C6MNoPair,
        widths: C6MNoWidths,
        scalar_layout_id: C6MNoPair,
        bounds: {
          code_rows: 64,
          function_rows: 16,
          code_child_slots: 64,
          parameter_slots: 64,
          pattern_rows: 32,
          match_plan_rows: 32,
          match_arm_rows: 32,
          type_rows: 16,
          constructor_rows: 32,
          constructor_field_slots: 64,
          max_arity: 2,
          constant_nodes: 64,
          constant_field_slots: 64,
          data_nodes: 64,
          data_field_slots: 128,
          environment_cells: 64,
          frames: 16,
          helper_tasks: 64,
          serialization_bytes: 65536,
          max_reference_work: 256,
          max_helper_work: 1024,
          step_cap: 256
        },
        capacities: {
          name_rows: 64,
          name_bytes: 256,
          occurrence_path_slots: 256,
          value_slots: 256
        },
        boundary_capacities_present: false
      }
    },
    image: {
      run_id: {
        identity: {
          profile_id: 2,
          grammar_domain: C6MDedicatedSorts,
          finance: C6MFinanceAbsent,
          layout_version: {
            first: List(101,110,118,103,101,116,45,111,119,110,101,114,45,112,111,115,105,116,105,118,101),
            second: List(49)
          },
          layout_digest: C6MUnselected,
          bounds_digest: C6MUnselected,
          field_id: C6MNoPair,
          program_membership_digest: C6MUnselected
        },
        fuel_input: C6MNat(128),
        stack_limit_input: C6MNat(8)
      },
      control: C6Eval({
        expr: 3
      }),
      control_origin: 3,
      env_root: C6EnvSome(0),
      stack_limit: 8,
      outcome: C6Running,
      initial_fuel: 128,
      remaining_work: 114,
      consumed_work: 14,
      helper_work: 8,
      helper_visits: 2,
      reference_cursor: 6,
      alloc: {
        data_nodes: 0,
        data_fields: 0,
        environment_cells: 1
      },
      active: {
        data: 0,
        data_fields: 0,
        env: 1,
        value_slots: 0
      },
      helper: C6HSomeHelper(C6HEnvGet({
        common: {
          site: C6HSiteTagEvalVar,
          origin_reference_cursor: 6,
          parent_resume: C6HNoIndex
        },
        binder: 0,
        cursor: C6HSomeIndex(0),
        found: C6HNoRef,
        status: C6HTriStatusPending
      })),
      result: C6HNoResult,
      seg_base: {
        active: {
          data: 0,
          data_fields: 0,
          env: 0,
          value_slots: 0
        },
        alloc: {
          data_nodes: 0,
          data_fields: 0,
          environment_cells: 0
        },
        consumed_work: 0
      },
      data: List(),
      data_fields: List(),
      env: List({
        binder: 0,
        value: C6Const(0),
        previous: C6EnvNone
      }),
      frames: List(C6ScopeF({
        saved_env: C6EnvNone,
        origin: 0
      }),C6LetF({
        binder: 1,
        body: 4,
        saved_env: C6EnvSome(0),
        origin: 2
      })),
      tasks: List(),
      value_slots: List()
    }
  }
}
