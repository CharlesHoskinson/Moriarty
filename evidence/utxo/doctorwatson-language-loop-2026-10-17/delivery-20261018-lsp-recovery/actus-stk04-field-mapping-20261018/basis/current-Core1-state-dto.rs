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
