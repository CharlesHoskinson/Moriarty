use super::SliceC;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TypeIdC(pub usize);

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
