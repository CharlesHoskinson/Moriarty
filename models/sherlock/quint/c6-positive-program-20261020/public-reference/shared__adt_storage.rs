// PUBLIC REFERENCE ONLY shared/adt_storage.rs:1-16
use super::adt::TypeIdC;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CtorRowExt {
    pub owning_type: TypeIdC,
    pub name_id: usize,
    pub source_order: usize,
    pub field_start: usize,
    pub field_len: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CtorFieldRowExt {
    pub name_id: usize,
    pub type_id: TypeIdC,
}

