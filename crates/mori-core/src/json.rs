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
pub fn parse(text: &str) -> Result<Value, Error> {
    use serde::de::DeserializeSeed;
    if text.len() > MAX_BYTES {
        return Err(Error::new("json", "JSON exceeds 1 MiB"));
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
