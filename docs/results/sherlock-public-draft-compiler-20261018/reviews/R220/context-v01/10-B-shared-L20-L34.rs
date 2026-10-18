
use std::sync::Arc;
use super::raw::{ConstRow as RawConstRow, DataRow as RawDataRow, RuleId as RawRuleId};

// ---------- plain shared enums: Copy, no raw-104 types inside ----------
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GrammarDomain { DedicatedSorts, GenericNominalExtensionReserved }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FinanceInput { Absent, TypedExtensionReserved }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SortTag { UInt, Bool, List, Tree }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryOp { Add, Sub, Mul, Eq, Lt, Cons, Branch }
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnaryOp { Leaf, Not }
