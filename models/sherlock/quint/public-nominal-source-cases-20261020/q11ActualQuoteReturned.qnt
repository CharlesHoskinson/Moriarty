module q11ActualQuoteReturned {
  import c6PositiveHostTypes.* from "./c6PositiveHostTypes"
  import c6PositiveMetaTypes.* from "./c6PositiveMetaTypes"
  import c6PositiveProgramTypes.* from "./c6PositiveProgramTypes"
  import c6PositiveContextTypes.* from "./c6PositiveContextTypes"
  import c6PositiveStateTypes.* from "./c6PositiveStateTypes"

  pure val actualReturned:C6WholeHost = {
    admission: {
      identity: {
        bounds_digest: C6MUnselected,
        field_id: C6MNoPair,
        finance: C6MFinanceAbsent,
        grammar_domain: C6MDedicatedSorts,
        layout_digest: C6MUnselected,
        layout_version: {
          first: List(109,111,114,105,97,114,116,121),
          second: List(115,104,101,114,108,111,99,107,45,104,101,108,112,101,114,45,109,105,99,114,111,115,116,101,112,45,104,111,115,116,111,110,108,121,47,49)
        },
        profile_id: 2,
        program_membership_digest: C6MUnselected
      },
      bounds: {
        code_child_slots: 16,
        code_rows: 64,
        constant_field_slots: 16,
        constant_nodes: 16,
        constructor_field_slots: 16,
        constructor_rows: 8,
        data_field_slots: 64,
        data_nodes: 64,
        environment_cells: 64,
        frames: 64,
        function_rows: 4,
        helper_tasks: 64,
        match_arm_rows: 8,
        match_plan_rows: 8,
        max_arity: 8,
        max_helper_work: 4096,
        max_reference_work: 4096,
        parameter_slots: 8,
        pattern_rows: 8,
        serialization_bytes: 65536,
        step_cap: 4096,
        type_rows: 8
      },
      layout: {
        boundary_capacities_present: false,
        bounds: {
          code_child_slots: 16,
          code_rows: 64,
          constant_field_slots: 16,
          constant_nodes: 16,
          constructor_field_slots: 16,
          constructor_rows: 8,
          data_field_slots: 64,
          data_nodes: 64,
          environment_cells: 64,
          frames: 64,
          function_rows: 4,
          helper_tasks: 64,
          match_arm_rows: 8,
          match_plan_rows: 8,
          max_arity: 8,
          max_helper_work: 4096,
          max_reference_work: 4096,
          parameter_slots: 8,
          pattern_rows: 8,
          serialization_bytes: 65536,
          step_cap: 4096,
          type_rows: 8
        },
        capacities: {
          name_bytes: 256,
          name_rows: 32,
          occurrence_path_slots: 256,
          value_slots: 1024
        },
        field_id: C6MNoPair,
        layout_version: {
          first: List(109,111,114,105,97,114,116,121),
          second: List(115,104,101,114,108,111,99,107,45,104,101,108,112,101,114,45,109,105,99,114,111,115,116,101,112,45,104,111,115,116,111,110,108,121,47,49)
        },
        scalar_layout_id: C6MNoPair,
        widths: C6MNoWidths
      },
      view: {
        arms: List(),
        children: List(),
        constant_fields: List(),
        constants: List(C6PKUInt({
          marker_free: true,
          n: 19
        }),C6PKUInt({
          marker_free: true,
          n: 3
        }),C6PKUInt({
          marker_free: true,
          n: 2
        })),
        constructors: List(),
        ctor_fields: List(),
        decl_ctor_counts: List(),
        decl_param_counts: List(),
        decl_templates: List(),
        entry: 0,
        expressions: List(C6PECall({
          function_name: 1,
          args: {
            len: 0,
            start: 0
          }
        }),C6PELet({
          binder: 0,
          body: 3,
          value: 2
        }),C6PELiteral({
          constant: 0
        }),C6PELet({
          binder: 1,
          body: 9,
          value: 4
        }),C6PEBinary({
          op: C6PAdd,
          left: 5,
          right: 8
        }),C6PEBinary({
          op: C6PMul,
          left: 6,
          right: 7
        }),C6PEVar({
          binder: 0
        }),C6PELiteral({
          constant: 1
        }),C6PELiteral({
          constant: 2
        }),C6PEVar({
          binder: 1
        })),
        field_paths: List(),
        functions: List({
          body: 1,
          function_name: 1,
          params: {
            len: 0,
            start: 0
          },
          return_type: 0
        }),
        names: List(List(112,114,105,110,99,105,112,97,108),List(113,117,111,116,101)),
        occurrence_paths: List(0,1,1,0,1,0,0,1,0,0,0,1,0,0,1,1,0,1,1,1),
        occurrences: List({
          function: 0,
          owner_is_function: false,
          path: {
            len: 0,
            start: 0
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 0,
            start: 0
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 1,
            start: 0
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 1,
            start: 1
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 2,
            start: 2
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 3,
            start: 4
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 4,
            start: 7
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 4,
            start: 11
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 3,
            start: 15
          }
        },{
          function: 0,
          owner_is_function: true,
          path: {
            len: 2,
            start: 18
          }
        }),
        parameters: List(),
        pattern_children: List(),
        patterns: List(),
        plan_cases: List(),
        plan_nodes: List(),
        type_args: List(),
        types: List()
      }
    },
    image: {
      active: {
        data: 2,
        data_fields: 0,
        env: 2,
        value_slots: 0
      },
      alloc: {
        data_fields: 0,
        data_nodes: 2,
        environment_cells: 2
      },
      consumed_work: 70,
      control: C6Return({
        value: C6Data(1)
      }),
      control_origin: 0,
      data: List(C6HU128({
        n: 57
      }),C6HU128({
        n: 59
      })),
      data_fields: List(),
      env: List({
        binder: 0,
        value: C6Const(0),
        previous: C6EnvNone
      },{
        binder: 1,
        value: C6Data(1),
        previous: C6EnvSome(0)
      }),
      env_root: C6EnvNone,
      frames: List(),
      helper: C6HNoHelper,
      helper_visits: 13,
      helper_work: 41,
      initial_fuel: 1000,
      outcome: C6Halted(C6Data(1)),
      reference_cursor: 29,
      remaining_work: 930,
      result: C6HNoResult,
      run_id: {
        identity: {
          bounds_digest: C6MUnselected,
          field_id: C6MNoPair,
          finance: C6MFinanceAbsent,
          grammar_domain: C6MDedicatedSorts,
          layout_digest: C6MUnselected,
          layout_version: {
            first: List(109,111,114,105,97,114,116,121),
            second: List(115,104,101,114,108,111,99,107,45,104,101,108,112,101,114,45,109,105,99,114,111,115,116,101,112,45,104,111,115,116,111,110,108,121,47,49)
          },
          profile_id: 2,
          program_membership_digest: C6MUnselected
        },
        fuel_input: C6MNat(1000),
        stack_limit_input: C6MNat(32)
      },
      seg_base: {
        active: {
          data: 0,
          data_fields: 0,
          env: 0,
          value_slots: 0
        },
        alloc: {
          data_fields: 0,
          data_nodes: 0,
          environment_cells: 0
        },
        consumed_work: 0
      },
      stack_limit: 32,
      tasks: List(),
      value_slots: List()
    }
  }
  pure val actualRoot:C6Ref = C6Data(1)
}
