//! Proposed PR54 local reference bridge. This is not a UTXO body codec.
//!
//! The complete upstream outcome remains intact. Pending frames, effects,
//! liabilities and attempted/committed work are never flattened into /1.
use mori_core::{Error, Program, Scenario, UInt, intent, stage};
use serde::Serialize;

/// Public inspection only; it cannot reconstruct the private source context.
#[derive(Debug, Clone, Serialize)]
pub struct StagedSourceMetadataV1 {
    pub format: String,
    pub language: String,
    pub semantics: String,
    pub source_sha256: String,
    pub program_hash: String,
    pub plan_hash: String,
    pub logical_cost_profile: String,
    pub qualification: String,
    pub authentication: String,
    pub consumption: String,
    pub program: Program,
    pub plan: stage::StagePlan,
}

/// Constructed from source by actual stage::load, never from caller metadata.
/// This context supplies local reference semantics only, not target admission.
pub struct StagedReferenceContext {
    staged: stage::StagedProgram,
    source_sha256: String,
}

pub fn prepare_staged_reference(source: &str) -> Result<StagedReferenceContext, Error> {
    Ok(StagedReferenceContext {
        staged: stage::load(source)?,
        source_sha256: intent::hash_bytes(source.as_bytes()),
    })
}

impl StagedReferenceContext {
    pub fn metadata(&self) -> StagedSourceMetadataV1 {
        StagedSourceMetadataV1 {
            format: "moriarty-cardano-staged-source-inspection/1".into(),
            language: self.staged.program().language.clone(),
            semantics: self.staged.program().semantics.clone(),
            source_sha256: self.source_sha256.clone(),
            program_hash: intent::hex(&self.staged.program_hash()),
            plan_hash: intent::hex(&self.staged.plan_hash()),
            logical_cost_profile: stage::LOGICAL_COST_PROFILE_2.into(),
            qualification: stage::STAGE_LABELS[0].1.into(),
            authentication: stage::STAGE_LABELS[1].1.into(),
            consumption: stage::STAGE_LABELS[2].1.into(),
            program: self.staged.program().clone(),
            plan: self.staged.plan().clone(),
        }
    }

    /// Returns every upstream field without converting staged work to atomic W.
    pub fn start_reference(
        &self,
        action: &str,
        scenario: Scenario,
        cap: Option<UInt>,
    ) -> Result<stage::StageOutcome, stage::StageRejection> {
        stage::start(&self.staged, action, scenario, cap)
    }

    /// The upstream local interface accepts caller-supplied continuation/event.
    /// No authentication or consumption policy is added by this wrapper.
    pub fn resume_reference(
        &self,
        continuation: stage::Continuation,
        event: stage::StageEvent,
        action: Option<&str>,
        cap: Option<UInt>,
    ) -> Result<stage::StageOutcome, stage::StageRejection> {
        stage::resume(&self.staged, continuation, event, action, cap)
    }
}
