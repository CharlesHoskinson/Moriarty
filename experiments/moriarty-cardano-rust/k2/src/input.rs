//! Bounded input reading and strict request parsing.
//!
//! Every JSON document goes through `mori_core::json::parse_limited` first: duplicate decoded
//! keys, trailing data, depth above 32 and more than 65,536 nodes are refused there. The value
//! is then decoded into `deny_unknown_fields` DTOs. Hex and decimal fields are checked by
//! [`crate::hexutil`] where they are used, never coerced.
use crate::error::DriverError;
use mori_core::{Effect, Scenario, Snapshot, Value};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Source text ceiling (bytes).
pub const MAX_SOURCE_BYTES: u64 = 262_144;
/// JSON document ceiling; equal to Core's own `json::parse` ceiling.
pub const MAX_JSON_BYTES: usize = 1_048_576;
/// Proving-key file ceiling. The real size is unmeasured; a larger key is refused, not read.
pub const MAX_PROVING_KEY_BYTES: u64 = 2 * 1024 * 1024 * 1024;

pub const SELECTOR_FORMAT: &str = "moriarty-k2-relation-selector/1";
pub const REQUEST_FORMAT: &str = "moriarty-k2-prove-request/1";

/// Reads a regular file of at most `max` bytes. Devices, directories and larger files are
/// refused before their content is read.
pub fn read_bounded(path: &Path, max: u64, what: &str) -> Result<Vec<u8>, DriverError> {
    let file = File::open(path).map_err(|e| DriverError::Io(format!("{what}: cannot open {}: {e}", path.display())))?;
    let meta = file.metadata().map_err(|e| DriverError::Io(format!("{what}: cannot stat {}: {e}", path.display())))?;
    if !meta.is_file() {
        return Err(DriverError::Io(format!("{what}: {} is not a regular file", path.display())));
    }
    if meta.len() > max {
        return Err(DriverError::Input(format!("{what}: {} bytes exceed the {max}-byte bound", meta.len())));
    }
    let mut out = Vec::new();
    file.take(max.saturating_add(1))
        .read_to_end(&mut out)
        .map_err(|e| DriverError::Io(format!("{what}: cannot read {}: {e}", path.display())))?;
    if out.len() as u64 > max {
        return Err(DriverError::Input(format!("{what}: file grew beyond the {max}-byte bound while read")));
    }
    Ok(out)
}

/// Reads a source file as UTF-8 text.
pub fn read_source(path: &Path) -> Result<String, DriverError> {
    let bytes = read_bounded(path, MAX_SOURCE_BYTES, "source")?;
    String::from_utf8(bytes).map_err(|_| DriverError::Input("source: not UTF-8".into()))
}

/// Strict JSON decoding of `bytes` into `T`.
pub fn parse_json<T: DeserializeOwned>(bytes: &[u8], what: &str) -> Result<T, DriverError> {
    if bytes.len() > MAX_JSON_BYTES {
        return Err(DriverError::Input(format!("{what}: {} bytes exceed the JSON bound", bytes.len())));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| DriverError::Input(format!("{what}: not UTF-8")))?;
    let value = mori_core::json::parse_limited(text, MAX_JSON_BYTES, "JSON input exceeds the driver bound")
        .map_err(|e| DriverError::Input(format!("{what}: {e}")))?;
    serde_json::from_value(value).map_err(|e| DriverError::Input(format!("{what}: {e}")))
}

/// Which relation to compile. Every option is spelled out; the driver supports exactly one
/// value of each and refuses the rest (see [`crate::relation::check_selector`]).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RelationSelector {
    pub format: String,
    pub language: String,
    pub semantics: String,
    pub action: String,
    pub adapter: String,
    pub cap: String,
    pub accounting: String,
    pub frame: String,
    /// Optional pin: lowercase hex SHA-256 of the raw source bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_source_sha256: Option<String>,
    /// Optional pin: lowercase hex canonical typed-Program hash.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_program_hash: Option<String>,
}

/// The claimed transition, as a proposal. There is no default kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum TransitionInput {
    /// Core `simulate` proposes the transition for this scenario.
    CoreReference { scenario: Scenario },
    /// Explicit typed records: inputs, pre and post snapshots and ordered effects.
    Records { inputs: BTreeMap<String, Value>, pre: Snapshot, post: Snapshot, effects: Vec<Effect> },
}

/// Per-proof context bytes, as caller-declared premises. The VK digest is derived from the
/// VK actually used; if a request restates it, the two must be equal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContextInput {
    pub claimant_vkh: String,
    pub network_id: u8,
    pub settle_hash: String,
    pub thread_id: String,
    pub consumed_txid: String,
    pub consumed_ix: u16,
    pub seq: u64,
    pub catalogue_digest: String,
    pub intents_digest: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vk_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProveRequest {
    pub format: String,
    pub selector: RelationSelector,
    pub transition: TransitionInput,
    pub context: ContextInput,
}

/// Parses and format-checks a selector document.
pub fn parse_selector(bytes: &[u8]) -> Result<RelationSelector, DriverError> {
    let s: RelationSelector = parse_json(bytes, "selector")?;
    if s.format != SELECTOR_FORMAT {
        return Err(DriverError::Unsupported(format!("selector format {:?}", s.format)));
    }
    Ok(s)
}

/// Parses and format-checks a prove request.
pub fn parse_request(bytes: &[u8]) -> Result<ProveRequest, DriverError> {
    let r: ProveRequest = parse_json(bytes, "prove request")?;
    if r.format != REQUEST_FORMAT {
        return Err(DriverError::Unsupported(format!("request format {:?}", r.format)));
    }
    if r.selector.format != SELECTOR_FORMAT {
        return Err(DriverError::Unsupported(format!("selector format {:?}", r.selector.format)));
    }
    Ok(r)
}
