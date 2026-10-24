#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct BoundsC { pub code_rows: usize, pub function_rows: usize,
    pub code_child_slots: usize, pub parameter_slots: usize,
    pub pattern_rows: usize, pub match_plan_rows: usize,
    pub match_arm_rows: usize, pub type_rows: usize,
    pub constructor_rows: usize, pub constructor_field_slots: usize,
    pub max_arity: usize, pub constant_nodes: usize,
    pub constant_field_slots: usize, pub data_nodes: usize,
    pub data_field_slots: usize, pub environment_cells: usize,
    pub frames: usize, pub helper_tasks: usize,
    pub serialization_bytes: usize, pub max_reference_work: u128,
    pub max_helper_work: u128, pub step_cap: u128 } // all 22, schema #/bounds
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct LayoutCapacitiesC { pub name_rows: usize, pub name_bytes: usize,
    pub occurrence_path_slots: usize, pub value_slots: usize }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RefC { Const(usize), Data(usize) } // shared owned ref; owner maps to raw
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SliceC { pub start: usize, pub len: usize }
impl SliceC {
    pub fn normalized(start: usize, len: usize) -> Self {
        if len == 0 { Self { start: 0, len: 0 } } else { Self { start, len } }
    }
    pub fn empty() -> Self { Self { start: 0, len: 0 } }
}
pub type ValSliceC = SliceC; pub type FieldSliceC = SliceC;
pub type EnvRootC = Option<usize>;
