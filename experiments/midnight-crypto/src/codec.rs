//! Experimental canonical scalar JSON; not the production intent/3 encoding.
use serde::de::{DeserializeSeed, Error as _, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::{cell::Cell, fmt};
const MAX_BYTES: usize = 65536;
struct Seed<'a> {
    depth: usize,
    nodes: &'a Cell<usize>,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        self.nodes.set(self.nodes.get() + 1);
        if self.depth > 32 || self.nodes.get() > 4096 {
            return Err(D::Error::custom("JSON work/depth limit"));
        }
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON with exact numbers as strings")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(Value::String(v.into()))
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> {
        Ok(Value::String(v))
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut v = Vec::new();
        while let Some(x) = a.next_element_seed(Seed {
            depth: self.depth + 1,
            nodes: self.nodes,
        })? {
            v.push(x);
        }
        Ok(Value::Array(v))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut v = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if v.contains_key(&k) {
                return Err(A::Error::custom("duplicate decoded key"));
            }
            let x = a.next_value_seed(Seed {
                depth: self.depth + 1,
                nodes: self.nodes,
            })?;
            v.insert(k, x);
        }
        Ok(Value::Object(v))
    }
}
/// Reject duplicates, numbers, trailing data and excessive work before canonicalization.
pub fn parse(text: &str) -> Result<Value, String> {
    if text.len() > MAX_BYTES {
        return Err("JSON byte limit".into());
    }
    let nodes = Cell::new(0);
    let mut d = serde_json::Deserializer::from_str(text);
    let v = Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(&mut d)
    .map_err(|e| e.to_string())?;
    d.end().map_err(|e| e.to_string())?;
    Ok(v)
}
fn sorted(v: Value) -> Value {
    match v {
        Value::Array(a) => Value::Array(a.into_iter().map(sorted).collect()),
        Value::Object(m) => {
            let mut entries: Vec<_> = m.into_iter().collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));
            Value::Object(entries.into_iter().map(|(k, v)| (k, sorted(v))).collect())
        }
        x => x,
    }
}
/// Canonical object bytes framed by an experimental domain and big-endian UTF8 byte count.
pub fn canonical_message(text: &str) -> Result<Vec<u8>, String> {
    let v = parse(text)?;
    if !v.is_object() {
        return Err("statement must be an object".into());
    }
    let b = serde_json::to_vec(&sorted(v)).map_err(|e| e.to_string())?;
    if b.len() > MAX_BYTES {
        return Err("canonical output limit".into());
    }
    let mut out = b"moriarty-midnight-auth-experiment/1\0".to_vec();
    out.extend_from_slice(&(b.len() as u32).to_be_bytes());
    out.extend_from_slice(&b);
    Ok(out)
}
/// Exact connector signData prefix for bytes; never UTF16 character length.
pub fn wallet_message(data: &[u8]) -> Vec<u8> {
    let mut b = format!("midnight_signed_message:{}:", data.len()).into_bytes();
    b.extend_from_slice(data);
    b
}
