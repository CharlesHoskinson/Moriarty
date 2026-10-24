pub struct DeclIdxC(pub usize);

pub enum TypeExprSrc {
    UInt,
    Bool,
    List,
    Tree,
    Named { name: String },
    Apply { name: String, args: Vec<TypeExprSrc> },
    Param { name: String },
