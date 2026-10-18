use crate::{Error, Result};
use midnight_serialize::{Deserializable, Serializable, Tagged};
use serde::de::DeserializeOwned;
use sha2::{Digest, Sha256};

pub const INPUT_BYTES: usize = 1_048_576;
pub const OUTPUT_BYTES: usize = 16_777_216;
pub fn encode<T: Serializable + Tagged>(value: &T) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();
    midnight_serialize::tagged_serialize(value, &mut bytes)
        .map_err(|_| Error::input("native-encode"))?;
    if bytes.len() > OUTPUT_BYTES {
        return Err(Error::input("artifact-size"));
    }
    Ok(bytes)
}
pub fn decode_native<T: Deserializable + Serializable + Tagged>(bytes: &[u8]) -> Result<T> {
    if bytes.len() > INPUT_BYTES {
        return Err(Error::input("input-size"));
    }
    decode_bounded(bytes)
}
pub fn decode_artifact<T: Deserializable + Serializable + Tagged>(bytes: &[u8]) -> Result<T> {
    if bytes.len() > OUTPUT_BYTES {
        return Err(Error::input("artifact-size"));
    }
    decode_bounded(bytes)
}
fn decode_bounded<T: Deserializable + Serializable + Tagged>(bytes: &[u8]) -> Result<T> {
    let value =
        midnight_serialize::tagged_deserialize(bytes).map_err(|_| Error::input("native-decode"))?;
    if encode(&value)? != bytes {
        return Err(Error::input("noncanonical-native"));
    }
    Ok(value)
}
pub fn json<T: DeserializeOwned>(text: &str) -> Result<T> {
    if text.len() > INPUT_BYTES {
        return Err(Error::input("input-size"));
    }
    let value = mori_core::json::parse(text).map_err(|_| Error::input("strict-json"))?;
    serde_json::from_value(value).map_err(|_| Error::input("public-schema"))
}
pub fn hex(bytes: &[u8]) -> String {
    mori_core::intent::hex(bytes)
}
pub fn unhex(text: &str) -> Result<Vec<u8>> {
    if text.len() > 2 * INPUT_BYTES
        || !text.len().is_multiple_of(2)
        || !text
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(Error::input("canonical-hex"));
    }
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).map_err(|_| Error::input("canonical-hex")))
        .collect()
}
pub fn native_hex<T: Serializable + Tagged>(value: &T) -> Result<String> {
    Ok(hex(&encode(value)?))
}
pub fn from_hex<T: Deserializable + Serializable + Tagged>(text: &str) -> Result<T> {
    decode_native(&unhex(text)?)
}
pub fn hash(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
pub fn object_hash<T: serde::Serialize>(value: &T) -> Result<String> {
    Ok(hash(
        &mori_core::intent::canonical(value).map_err(|_| Error::input("canonical-json"))?,
    ))
}
pub fn native_hash<T: Serializable + Tagged>(value: &T) -> Result<String> {
    Ok(hash(&encode(value)?))
}
