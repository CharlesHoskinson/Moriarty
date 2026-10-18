use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;
pub const MAX_TRACE_CELLS: usize = 65_536;

/// Unsigned integral atomic units; JSON uses canonical decimal strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UInt(pub u128);
impl Serialize for UInt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> Deserialize<'de> for UInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err(serde::de::Error::custom(
                "expected canonical unsigned decimal string",
            ));
        }
        s.parse().map(UInt).map_err(serde::de::Error::custom)
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    UInt,
    Bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", deny_unknown_fields)]
pub enum Value {
    UInt(UInt),
    Bool(bool),
}
impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::UInt(_) => Type::UInt,
            Self::Bool(_) => Type::Bool,
        }
    }
}
pub type ExprId = u16;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Expr {
    LiabilityPresent {
        name: String,
    },
    LiabilityAmount {
        name: String,
    },
    Literal {
        value: Value,
    },
    Name {
        name: String,
    },
    Binary {
        op: BinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    },
    Not {
        value: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StateDecl {
    pub name: String,
    pub ty: Type,
    pub initial: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputDecl {
    pub name: String,
    pub ty: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct AssetId {
    pub network: String,
    pub issuer: String,
    pub reference: String,
    pub scale: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetDecl {
    pub name: String,
    pub identity: AssetId,
}
/// Serialize-only on the shared type: decoding always goes through the frozen
/// `v1` wire, so JSON can never yield `Await`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Statement {
    Await {
        label: String,
        binders: Vec<InputDecl>,
        guard: ExprId,
    },
    Let {
        name: String,
        value: ExprId,
    },
    Requires {
        predicate: ExprId,
    },
    Ensures {
        predicate: ExprId,
    },
    Next {
        state: String,
        value: ExprId,
    },
    Emit {
        name: String,
        value: ExprId,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount: ExprId,
    },
    Discharge {
        name: String,
        amount: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub inputs: Vec<InputDecl>,
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub language: String,
    pub semantics: String,
    pub name: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub expressions: Vec<Expr>,
    pub actions: Vec<Action>,
}
