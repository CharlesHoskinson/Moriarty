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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub account: String,
    pub asset: AssetId,
    pub amount: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Liability {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub amount: UInt,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub state: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub state: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Effect {
    State {
        name: String,
        before: Value,
        after: Value,
    },
    Transfer {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Fee {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Liability {
        name: String,
        asset: AssetId,
        debtor: String,
        creditor: String,
        before: UInt,
        after: UInt,
    },
    Emit {
        name: String,
        value: Value,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub index: u16,
    pub statement: Statement,
    pub value: Option<Value>,
    pub snapshot: Snapshot,
    pub effects: Vec<Effect>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub semantics: String,
    pub action: String,
    pub pre: Snapshot,
    pub post: Snapshot,
    pub steps: Vec<Step>,
    pub effects: Vec<Effect>,
}
/// Frozen `moriarty-core/1` decode wire: the baseline (fe01bc8b) derives,
/// moved verbatim. Type names are kept so serde's `expecting` text, unknown
/// variant lists and error messages stay byte-identical. There is no Await.
pub mod v1 {
    use super::{AssetDecl, Effect, Expr, ExprId, InputDecl, Snapshot, StateDecl, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "kind", deny_unknown_fields)]
    pub enum Statement {
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
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Action {
        pub name: String,
        pub inputs: Vec<InputDecl>,
        pub statements: Vec<Statement>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Step {
        pub index: u16,
        pub statement: Statement,
        pub value: Option<Value>,
        pub snapshot: Snapshot,
        pub effects: Vec<Effect>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Execution {
        pub semantics: String,
        pub action: String,
        pub pre: Snapshot,
        pub post: Snapshot,
        pub steps: Vec<Step>,
        pub effects: Vec<Effect>,
    }
}
impl From<v1::Statement> for Statement {
    fn from(s: v1::Statement) -> Self {
        use v1::Statement as W;
        match s {
            W::Let { name, value } => Self::Let { name, value },
            W::Requires { predicate } => Self::Requires { predicate },
            W::Ensures { predicate } => Self::Ensures { predicate },
            W::Next { state, value } => Self::Next { state, value },
            W::Emit { name, value } => Self::Emit { name, value },
            W::Transfer {
                asset,
                from,
                to,
                amount,
            } => Self::Transfer {
                asset,
                from,
                to,
                amount,
            },
            W::Fee {
                asset,
                from,
                to,
                amount,
            } => Self::Fee {
                asset,
                from,
                to,
                amount,
            },
            W::Liability {
                name,
                asset,
                debtor,
                creditor,
                amount,
            } => Self::Liability {
                name,
                asset,
                debtor,
                creditor,
                amount,
            },
            W::Discharge { name, amount } => Self::Discharge { name, amount },
        }
    }
}
impl From<v1::Action> for Action {
    fn from(a: v1::Action) -> Self {
        Self {
            name: a.name,
            inputs: a.inputs,
            statements: a.statements.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<v1::Program> for Program {
    fn from(p: v1::Program) -> Self {
        Self {
            language: p.language,
            semantics: p.semantics,
            name: p.name,
            states: p.states,
            assets: p.assets,
            expressions: p.expressions,
            actions: p.actions.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<v1::Step> for Step {
    fn from(s: v1::Step) -> Self {
        Self {
            index: s.index,
            statement: s.statement.into(),
            value: s.value,
            snapshot: s.snapshot,
            effects: s.effects,
        }
    }
}
impl From<v1::Execution> for Execution {
    fn from(e: v1::Execution) -> Self {
        Self {
            semantics: e.semantics,
            action: e.action,
            pre: e.pre,
            post: e.post,
            steps: e.steps.into_iter().map(Into::into).collect(),
            effects: e.effects,
        }
    }
}
// Every direct, nested or generic decode of a shared type (CLI, replay,
// backend bundles) is routed through `v1`, so no decode site can be missed.
macro_rules! decode_via_v1 {
    ($($ty:ident),*) => {$(
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
                v1::$ty::deserialize(decoder).map(Self::from)
            }
        }
    )*};
}
decode_via_v1!(Statement, Action, Program, Step, Execution);
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub offset: Option<u32>,
    pub step: Option<u16>,
}
impl Error {
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: format!("mori::core::{code}"),
            message: message.into(),
            offset: None,
            step: None,
        }
    }
}
