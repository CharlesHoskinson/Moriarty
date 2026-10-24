//! Internal host-only L4 StateBlock rows; no public importer or native interface.
//! Logical lanes follow L4 column order. Byte serialization is a separate layer.
use super::limits::{Locator, PrepareFailure};
use super::rows::{ConstantNode, ProgramBlock, Slice, TableId, ValueRef};
use crate::sherlock::Program;

/// Every dimension is caller supplied. D must equal PreparedA.capacities.data.
/// No resource/default capacity is chosen by these structures.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StateCapacities {
    pub data: u128,
    pub data_fields: u128,
    pub env_cells: u128,
    pub value_slots: u128,
    pub frame_slots: u128,
    pub control_rows: u128,
    pub stack_limit: u128,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct EnvRoot {
    pub present: bool,
    pub index: usize,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct EnvCell {
    pub name_id: usize,
    pub value: ValueRef,
    pub parent: EnvRoot,
}

/// The logical Data node has exactly the same columns as the Constant node.
pub type DataNode = ConstantNode;

/// L4 tags: Let=0, Left=1, Right=2, Unary=3, If=4, Match=5,
/// Args=6, Return=7, Scope=8. Every unused column must be zero.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FrameRow {
    pub tag: u8,
    pub name_id: usize,
    pub binary_op: u8,
    pub unary_op: u8,
    pub type_id: u8,
    pub expr0: usize,
    pub expr1: usize,
    pub saved_env: EnvRoot,
    pub value: ValueRef,
    pub completed: Slice,
    pub remaining_exprs: Slice,
    pub arms: Slice,
}

/// L4 tags: Eval=0, Return=1, Push=2, Invoke=3, Enter=4, Bound=5,
/// Cases=6, CaseResult=7, PrimitiveResult=8, Error=9.
/// Push.next is the next active row, never a separately supplied pointer.
/// bindings_present distinguishes None from Some(empty EnvRoot).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ControlRow {
    pub tag: u8,
    pub expr_id: usize,
    pub value: ValueRef,
    pub pending_frame: FrameRow,
    pub environment: EnvRoot,
    pub name_id: usize,
    pub values: Slice,
    pub function_present: bool,
    pub function_id: usize,
    pub bindings_present: bool,
    pub bindings: EnvRoot,
    pub type_id: u8,
    pub arms: Slice,
    pub primitive_is_ok: bool,
    pub error_code: u8,
}

/// Outcome tags: Pending=0, Done=1, Rejected=2, Exhausted=3.
/// UInt128 fuel lanes use their complete 16 little-endian bytes. Stack limit
/// remains u128 until decoding checks host usize representability.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StateRoot {
    pub active_data: usize,
    pub active_data_fields: usize,
    pub active_env_cells: usize,
    pub active_value_slots: usize,
    pub depth: usize,
    pub control_length: usize,
    pub environment: EnvRoot,
    pub stack_limit: u128,
    pub remaining_fuel: [u8; 16],
    pub initial_fuel: [u8; 16],
    pub outcome_tag: u8,
    pub outcome_value: ValueRef,
    pub outcome_error: u8,
}

/// Fixed-capacity vectors include all inactive zero rows. Active lengths live
/// only in Root, not in a second divergent set of table headers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StateBlock {
    pub root: StateRoot,
    pub data: Vec<DataNode>,
    pub data_fields: Vec<ValueRef>,
    pub env_cells: Vec<EnvCell>,
    pub value_slots: Vec<ValueRef>,
    pub frame_slots: Vec<FrameRow>,
    pub control_rows: Vec<ControlRow>,
}

/// Internal typed image proposal. Full syntax and the complete ProgramBlock
/// are supplied independently and checked structurally, never by Arc identity
/// or digest. No byte parser, arbitrary-state machine import or network exists.
pub struct AImage<'a> {
    pub profile: &'a str,
    pub full_program: &'a Program,
    pub program: &'a ProgramBlock,
    pub state: &'a StateBlock,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeMode {
    Canonical,
    Step,
}

/// Exactly the seven located L4 tooling refusal classes. None is a semantic
/// Outcome or StepRecord. Preparation's two classes preserve their identity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StateFailure {
    Capacity {
        table: &'static str,
        needed: u128,
        limit: u128,
        locator: Locator,
    },
    ProgramMismatch {
        locator: Locator,
    },
    OutsideProgramClosure {
        field: &'static str,
        locator: Locator,
    },
    WrongRepresentation {
        block: &'static str,
        table: &'static str,
        row: usize,
        column: &'static str,
        definition: &'static str,
    },
    OutsideObservationDomain {
        condition: &'static str,
        locator: Locator,
    },
    WrongProfile {
        locator: Locator,
    },
    HostUnrepresentable {
        field: &'static str,
        locator: Locator,
    },
}
pub type StateResult<T> = std::result::Result<T, StateFailure>;

impl From<PrepareFailure> for StateFailure {
    fn from(value: PrepareFailure) -> Self {
        match value {
            PrepareFailure::Capacity {
                table,
                needed,
                limit,
                locator,
            } => {
                let table = match table {
                    TableId::Names => "Names",
                    TableId::NameBytes => "NameBytes",
                    TableId::Occurrences => "Occurrences",
                    TableId::OccurrencePaths => "OccurrencePaths",
                    TableId::Expressions => "Expressions",
                    TableId::ExprChildren => "ExprChildren",
                    TableId::Functions => "Functions",
                    TableId::Parameters => "Parameters",
                    TableId::Patterns => "Patterns",
                    TableId::Arms => "Arms",
                    TableId::Types => "Types",
                    TableId::Constructors => "Constructors",
                    TableId::ConstructorFieldTypes => "ConstructorFieldTypes",
                    TableId::Constants => "Constants",
                    TableId::ConstantFields => "ConstantFields",
                };
                Self::Capacity {
                    table,
                    needed,
                    limit,
                    locator,
                }
            }
            PrepareFailure::HostUnrepresentable { locator } => Self::HostUnrepresentable {
                field: locator.field,
                locator,
            },
        }
    }
}
