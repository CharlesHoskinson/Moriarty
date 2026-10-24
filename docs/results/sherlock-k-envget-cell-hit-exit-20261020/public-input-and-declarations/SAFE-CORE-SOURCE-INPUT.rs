// INPUT CONSTRUCTORS ONLY; no engine calls, injection, expected state or execution.
use mori_core::sherlock::helper_machine::shared::{CoreProgram, CoreExpr, CoreValue,
    BoundsC, LayoutSchemaC, LayoutCapacitiesC, RunIntC, FinanceInput};
use mori_core::sherlock::helper_machine::shared::i128_class::KIntC;

pub fn input_program() -> CoreProgram {
    CoreProgram { functions: vec![], entry: CoreExpr::Let {
        name: "x".to_string(),
        value: Box::new(CoreExpr::Literal { value: CoreValue::UInt { n: KIntC::InRange(7) } }),
        body: Box::new(CoreExpr::Let {
            name: "y".to_string(),
            value: Box::new(CoreExpr::Var { name: "x".to_string() }),
            body: Box::new(CoreExpr::Var { name: "y".to_string() }),
        }),
    } }
}
pub fn input_bounds() -> BoundsC {
    BoundsC {
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
    }
}
pub fn input_layout() -> LayoutSchemaC {
    LayoutSchemaC {
        layout_version: ("envget-owner-positive".to_string(), "1".to_string()),
        field_id: None, widths: None, scalar_layout_id: None,
        bounds: input_bounds(),
        capacities: LayoutCapacitiesC { name_rows: 64, name_bytes: 256,
            occurrence_path_slots: 256, value_slots: 256 },
        ext_capacities: None, boundary_capacities_present: false,
    }
}
pub fn run_inputs() -> (RunIntC, RunIntC, FinanceInput) {
    (RunIntC::Nat(128), RunIntC::Nat(8), FinanceInput::Absent)
}
