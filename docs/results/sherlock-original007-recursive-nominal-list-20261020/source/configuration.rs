fn parser_budget() -> DraftParseBudget {
    DraftParseBudget { max_source_bytes: 4096, max_tokens: 1024, max_functions: 8,
        max_parameters_per_function: 8, max_total_parameters: 32, max_ast_occurrences: 256,
        max_parser_recursion_depth: 64, max_structural_ast_depth: 32 }
}

// Explicit positive inputs, using the existing generic-C6 SAFE configuration.
// These values are test inputs, not defaults or an admission policy.
fn configuration() -> ExtCompileConfig {
    let bounds = BoundsC {
        code_rows:256,function_rows:8,code_child_slots:256,parameter_slots:16,
        pattern_rows:128,match_plan_rows:64,match_arm_rows:32,type_rows:64,
        constructor_rows:64,constructor_field_slots:128,max_arity:4,
        constant_nodes:128,constant_field_slots:128,data_nodes:64,data_field_slots:128,
        environment_cells:64,frames:64,helper_tasks:64,serialization_bytes:1048576,
        max_reference_work:100000,max_helper_work:100000,step_cap:1,
    };
    ExtCompileConfig { bounds, layout: LayoutSchemaC {
        layout_version:("independent-public-generic-runtime".into(),"1".into()),
        field_id:None,widths:None,scalar_layout_id:None,bounds,
        capacities:LayoutCapacitiesC { name_rows:64,name_bytes:1024,
            occurrence_path_slots:1024,value_slots:4096 },
        ext_capacities:Some(ExtCapacitiesC {
            plan_case_slots:256,field_path_slots:512,pattern_child_slots:256,
            input_rows:0,asset_rows:0,state_rows:0,account_rows:0,liability_rows:0,
            await_site_rows:0,await_binder_slots:0,balance_cells:0,journal_rows:0,
            pending_op_rows:0,observations_per_invocation:0,max_lifetime_work:0,resume_rows:0,
        }), boundary_capacities_present:false,
    }, domain:ExtDomain::GENERIC_ABSENT,fuel:RunIntC::Nat(4096),stack:RunIntC::Nat(64) }
}

