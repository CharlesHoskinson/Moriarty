//! Source-owned typed Core interchange, independent of any target backend.
use crate::{Error, Program, intent, parse};
use serde::Serialize;

/// Inspectable source metadata, not a target artifact or an admission record.
/// Construct exports with [`export_source`]; serialized fields are not authority.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CoreSourceExportV1 {
    pub format: String,
    pub language: String,
    pub semantics: String,
    pub source_sha256: String,
    pub program_hash: String,
    pub program: Program,
}

/// Parse and validate current source, preserving its exact typed Program.
/// Raw-source identity and canonical Program identity are deliberately separate.
pub fn export_source(source: &str) -> Result<CoreSourceExportV1, Error> {
    // The existing parser validates the Program before returning it.
    let program = parse(source)?;
    let program_hash = intent::hash_bytes(&intent::canonical(&program)?);
    Ok(CoreSourceExportV1 {
        format: "moriarty-core-source-export/1".into(),
        language: program.language.clone(),
        semantics: program.semantics.clone(),
        source_sha256: intent::hash_bytes(source.as_bytes()),
        program_hash,
        program,
    })
}
