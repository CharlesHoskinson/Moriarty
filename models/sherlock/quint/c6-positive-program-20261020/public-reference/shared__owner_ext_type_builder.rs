// PUBLIC REFERENCE ONLY shared/owner_ext_type_builder.rs:14-25
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct DeclIdxC(pub usize);

pub enum TypeExprSrc {
    UInt,
    Bool,
    List,
    Tree,
    Named { name: String },
    Apply { name: String, args: Vec<TypeExprSrc> },
    Param { name: String },
}

// PUBLIC REFERENCE ONLY shared/owner_ext_type_builder.rs:39-51
pub struct CtorFieldSrc {
    pub name: String,
    pub ty: TypeExprSrc,
}
pub struct CtorDeclSrc {
    pub name: String,
    pub fields: Vec<CtorFieldSrc>,
}
pub struct TypeDeclSrc {
    pub name: String,
    pub parameters: Vec<String>,
    pub ctors: Vec<CtorDeclSrc>,
}

