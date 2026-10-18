//! Named logical L4 rows. Byte expansion is separate from native embedding.
use std::sync::Arc;
use crate::sherlock::{ExprRef, Program};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum TableId { Names, NameBytes, Occurrences, OccurrencePaths, Expressions,
    ExprChildren, Functions, Parameters, Patterns, Arms, Types, Constructors,
    ConstructorFieldTypes, Constants, ConstantFields }

/// Caller dimensions, with no defaults. D participates in ValueRef width even
/// though preparation emits Constant-space references only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Capacities { pub tables: [u128; 15], pub data: u128 }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slice { pub start: usize, pub len: usize }
impl Slice { pub const ZERO: Self = Self { start: 0, len: 0 }; }

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ValueRef { pub space: bool, pub index: usize }
impl ValueRef { pub fn constant(index: usize) -> Self { Self { space: false, index } } }

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NameRow { pub bytes: Slice }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct OccurrenceRow { pub owner_tag: u8, pub function_index: usize, pub path: Slice }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExpressionRow { pub tag: u8, pub occurrence_id: usize, pub value: ValueRef,
    pub name_id: usize, pub binary_op: u8, pub unary_op: u8, pub children: Slice, pub arms: Slice }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct FunctionRow { pub source_table_index: usize, pub name_id: usize,
    pub parameters: Slice, pub return_type: u8, pub body_expr_id: usize }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ParameterRow { pub name_id: usize, pub type_id: u8 }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PatternRow { pub tag: u8, pub name0: usize, pub name1: usize }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ArmRow { pub pattern_id: usize, pub body_expr_id: usize }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct TypeRow { pub kind: u8, pub constructors: Slice }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConstructorRow { pub owning_type: u8, pub source_order: usize, pub fields: Slice }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ConstantNode { pub kind: u8, pub type_id: u8, pub ctor_id: usize,
    pub uint_bytes: [u8; 16], pub bool_value: bool, pub fields: Slice }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Table<T> { pub active: usize, pub rows: Vec<T> }
impl<T> Default for Table<T> { fn default() -> Self { Self { active: 0, rows: Vec::new() } } }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgramHeader { pub entry_expr_id: usize, pub active_lengths: [usize; 15] }
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ProgramBlock {
    pub header: ProgramHeader,
    pub names: Table<NameRow>, pub name_bytes: Table<u8>,
    pub occurrences: Table<OccurrenceRow>, pub occurrence_paths: Table<usize>,
    pub expressions: Table<ExpressionRow>, pub expr_children: Table<usize>,
    pub functions: Table<FunctionRow>, pub parameters: Table<ParameterRow>,
    pub patterns: Table<PatternRow>, pub arms: Table<ArmRow>,
    pub types: Table<TypeRow>, pub constructors: Table<ConstructorRow>,
    pub constructor_field_types: Table<u8>, pub constants: Table<ConstantNode>,
    pub constant_fields: Table<ValueRef>,
}

/// Source occurrence identity, not a guessed execution origin.
#[derive(Clone, Debug)]
pub struct OccurrenceMetadata { pub source: ExprRef, pub owner_tag: u8,
    pub function_index: usize, pub path: Vec<usize> }
#[derive(Clone, Debug)]
pub struct PreparedA { pub block: ProgramBlock, pub capacities: Capacities,
    pub full_program: Arc<Program>, pub occurrences: Vec<OccurrenceMetadata>, pub names: Vec<String> }
