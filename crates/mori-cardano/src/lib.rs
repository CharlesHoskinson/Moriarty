//! Source-owned UNIT inspection metadata; no compiler or ledger admission.
mod export;
mod layout;
mod profile;
mod projection;
mod staged;
mod types;
pub use projection::{
    ActionProjectionMetadata, DutyCell, ProjectionBody, ProjectionContext, ProjectionData,
    ProjectionError, ProjectionInspectionV2, prepare_projection,
};
pub use types::*;
pub use export::{CoreSourceExportV1, export_source};
pub use staged::{StagedReferenceContext, StagedSourceMetadataV1, prepare_staged_reference};

/// Compose the actual Rust source export with explicit partial layout metadata.
/// This is Unit-only: real target identities, credentials and the layout-hash
/// preimage are unresolved, not established by this public metadata record.
pub fn inspect_cardano(
    source: &str,
    accounts: &[AccountBinding],
    target: InspectionTarget,
) -> Result<CardanoInspectionMetadataV1, InspectionError> {
    let source = export_source(source).map_err(InspectionError::Core)?;
    if target != InspectionTarget::Unit {
        return Err(InspectionError::Unresolved("pinned_target_identity"));
    }
    let layout = layout::describe(&source.program, accounts)?;
    Ok(CardanoInspectionMetadataV1 {
        format: "moriarty-cardano-inspection-metadata/1".into(),
        qualification: "UNIT".into(),
        profile: "moriarty-cardano-core1-adapter/1".into(),
        target,
        source,
        layout,
        layout_hash: None,
        unresolved: [
            "layout_hash_preimage",
            "physical_account_bindings",
            "target_identity",
            "validator_trace",
            "ledger_admission",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect(),
    })
}
