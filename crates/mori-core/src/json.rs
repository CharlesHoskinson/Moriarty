//! Bounded JSON parsing that rejects duplicate keys before schema decoding.
use crate::Error;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::{cell::Cell, fmt};
const MAX_BYTES: usize = 1_048_576;
struct Seed<'a> {
    depth: usize,
    nodes: &'a Cell<usize>,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        use serde::de::Error as _;
        self.nodes.set(self.nodes.get() + 1);
        if self.depth > 32 || self.nodes.get() > 65_536 {
            return Err(D::Error::custom("JSON work/depth limit"));
        }
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite JSON number"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Seed {
            depth: self.depth + 1,
            nodes: self.nodes,
        })? {
            out.push(v)
        }
        Ok(Value::Array(out))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        use serde::de::Error as _;
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(A::Error::custom("duplicate decoded key"));
            }
            let v = a.next_value_seed(Seed {
                depth: self.depth + 1,
                nodes: self.nodes,
            })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
/// Strict recursive JSON value decoding for versioned DTO dynamic fields.
/// Duplicate decoded keys and the existing depth/node bounds apply even when
/// a consumer invokes serde directly instead of the bounded text entry point.
pub fn deserialize_strict<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Value, D::Error> {
    let nodes = Cell::new(0);
    Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(decoder)
}
pub fn parse(text: &str) -> Result<Value, Error> {
    parse_inner(text, MAX_BYTES, "JSON exceeds 1 MiB")
}
/// The same strict parser under a caller-selected byte ceiling. Duplicate-key,
/// depth (32) and node (65,536) limits are unchanged; `/2` stage files use it.
pub fn parse_limited(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    parse_inner(text, max, too_large)
}
/// Caller-budgeted strict JSON value decoding. Additive: legacy parse,
/// parse_limited, deserialize_strict and export are unchanged.
///
/// A budget is an inclusive upper bound, not a guaranteed acceptance
/// domain. `max_bytes` is the UTF-8 byte length, checked before decoding.
/// Nodes count one per value deserialization entry: the root, every array
/// element and every object member value; object keys are not nodes.
/// Depth is 0 at the root and increases by one per array/object level.
/// At one value entry the node limit is checked before the depth limit.
/// Text inside every budget can still be refused as
/// [`BudgetedParseError::Json`] for malformed JSON, duplicate decoded
/// keys, trailing data, out-of-range numbers, or a limit of the pinned
/// `serde_json` deserializer. In particular that deserializer refuses
/// the 128th nested array/object container (`recursion limit exceeded`)
/// regardless of a larger caller `max_depth`; see
/// `docs/core-json-budget-contract.md`. Legacy `parse`, `parse_limited`
/// and `deserialize_strict` keep their separate 32 / 65,536 / 1 MiB
/// bounds, which are not applied by `parse_budgeted`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JsonParseBudget { pub max_bytes: usize, pub max_nodes: usize, pub max_depth: usize }
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonBudgetKind { Bytes, Nodes, Depth }
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BudgetedParseError { Budget { kind: JsonBudgetKind, limit: usize }, Json(Error) }
impl std::fmt::Display for BudgetedParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self { Self::Budget { kind, limit } => write!(f, "JSON parse budget exceeded: {kind:?} limit {limit}"), Self::Json(e) => write!(f, "{e}") }
    }
}
impl std::error::Error for BudgetedParseError {}
#[derive(Clone, Copy)]
struct BudgetFlag(Option<JsonBudgetKind>);
struct BudgetSeed<'a> { depth: usize, nodes: &'a Cell<usize>, flag: &'a Cell<BudgetFlag>, budget: JsonParseBudget }
impl<'de> DeserializeSeed<'de> for BudgetSeed<'_> {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        use serde::de::Error as _;
        let count = self.nodes.get().saturating_add(1);
        self.nodes.set(count);
        if count > self.budget.max_nodes { self.flag.set(BudgetFlag(Some(JsonBudgetKind::Nodes))); return Err(D::Error::custom("JSON work/depth limit")); }
        if self.depth > self.budget.max_depth { self.flag.set(BudgetFlag(Some(JsonBudgetKind::Depth))); return Err(D::Error::custom("JSON work/depth limit")); }
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for BudgetSeed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result { f.write_str("bounded JSON") }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> { Ok(Value::Bool(v)) }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> { Ok(Value::Null) }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Value, E> { Ok(v.into()) }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Value, E> { Ok(v.into()) }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> { serde_json::Number::from_f64(v).map(Value::Number).ok_or_else(|| E::custom("nonfinite JSON number")) }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> { Ok(v.into()) }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> { Ok(v.into()) }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(BudgetSeed { depth: self.depth + 1, nodes: self.nodes, flag: self.flag, budget: self.budget })? { out.push(v) }
        Ok(Value::Array(out))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        use serde::de::Error as _;
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) { return Err(A::Error::custom("duplicate decoded key")); }
            let v = a.next_value_seed(BudgetSeed { depth: self.depth + 1, nodes: self.nodes, flag: self.flag, budget: self.budget })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
/// Parse `text` under the caller-supplied inclusive upper bounds in
/// `budget`. Budget refusals are returned as typed
/// [`BudgetedParseError::Budget`] data; every other refusal, including
/// the pinned `serde_json` container recursion guard described on
/// [`JsonParseBudget`], is [`BudgetedParseError::Json`]. Meeting all
/// three budgets therefore does not by itself guarantee acceptance.
pub fn parse_budgeted(text: &str, budget: JsonParseBudget) -> Result<Value, BudgetedParseError> {
    use serde::de::DeserializeSeed;
    if text.len() > budget.max_bytes { return Err(BudgetedParseError::Budget { kind: JsonBudgetKind::Bytes, limit: budget.max_bytes }); }
    let nodes = Cell::new(0usize);
    let flag = Cell::new(BudgetFlag(None));
    let mut decoder = serde_json::Deserializer::from_str(text);
    let result = BudgetSeed { depth: 0, nodes: &nodes, flag: &flag, budget }.deserialize(&mut decoder);
    let value = match result {
        Ok(v) => v,
        Err(e) => {
            if let BudgetFlag(Some(kind)) = flag.get() { let limit = match kind { JsonBudgetKind::Bytes => budget.max_bytes, JsonBudgetKind::Nodes => budget.max_nodes, JsonBudgetKind::Depth => budget.max_depth }; return Err(BudgetedParseError::Budget { kind, limit }); }
            return Err(BudgetedParseError::Json(Error::new("json", e.to_string())));
        }
    };
    decoder.end().map_err(|e| BudgetedParseError::Json(Error::new("json", e.to_string())))?;
    Ok(value)
}

fn parse_inner(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    use serde::de::DeserializeSeed;
    if text.len() > max {
        return Err(Error::new("json", too_large));
    }
    let nodes = Cell::new(0);
    let mut decoder = serde_json::Deserializer::from_str(text);
    let value = Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(&mut decoder)
    .map_err(|e| Error::new("json", e.to_string()))?;
    decoder
        .end()
        .map_err(|e| Error::new("json", e.to_string()))?;
    Ok(value)
}

/// ASCII-safe pretty JSON that fits the same public reader profile, including
/// the final newline added by CLI printing. Parsed values and signing bytes
/// are preserved; terminal-controlled Unicode remains visibly escaped.
pub fn export(value: &impl serde::Serialize) -> Result<String, Error> {
    let json =
        serde_json::to_string_pretty(value).map_err(|e| Error::new("json", e.to_string()))?;
    parse(&json)?;
    let mut display = String::with_capacity(json.len());
    for character in json.chars() {
        if character.is_ascii() && (!character.is_control() || character == '\n') {
            display.push(character);
        } else {
            for unit in character.encode_utf16(&mut [0; 2]) {
                use std::fmt::Write;
                write!(&mut display, "\\u{unit:04x}").expect("writing to String cannot fail");
            }
        }
        if display.len() >= MAX_BYTES {
            return Err(Error::new(
                "json",
                "Escaped JSON and final newline exceed 1 MiB",
            ));
        }
    }
    parse(&display)?;
    Ok(display)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_decoded_keys_and_trailing_data_reject() {
        assert!(parse(r#"{"x":1,"\u0078":2}"#).is_err());
        assert!(parse("{} []").is_err());
        assert!(parse(r#"{"x":"1"}"#).is_ok());
    }
}
