use super::{SliceC, SortTag};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TypeIdC(pub usize);

// The same canonical TypeId is used by type tables and live runtime signatures.
// These constants name the fixed builtin rows; every other TypeId keeps its ID.
#[allow(non_upper_case_globals)]
impl TypeIdC {
    pub const UInt: Self = Self(0);
    pub const Bool: Self = Self(1);
    pub const List: Self = Self(2);
    pub const Tree: Self = Self(3);

    pub const fn from_source_sort(sort: SortTag) -> Self {
        match sort {
            SortTag::UInt => Self::UInt,
            SortTag::Bool => Self::Bool,
            SortTag::List => Self::List,
            SortTag::Tree => Self::Tree,
        }
    }
}

impl From<SortTag> for TypeIdC {
    fn from(sort: SortTag) -> Self { Self::from_source_sort(sort) }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct CtorIdC(pub usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TypeRowC {
    pub name_id: usize,
    pub decl: Option<usize>,
    pub args: SliceC,
    pub ctor_start: CtorIdC,
    pub ctor_len: usize,
}
