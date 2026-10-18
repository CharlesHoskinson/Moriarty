//! Output-only preparation projection. No parser, type checker, AST copier,
//! evaluator, owner step, importable image or source-admission check is added.
//! Root must wire this immediately after its bounded preparation/admit_run seam.
use serde::Serialize;
use serde_json::Value;
use crate::sherlock::Type;
use crate::sherlock_draft as source;
use crate::sherlock::helper_machine::shared::HostViolation;
use super::draft_contract::{self, DraftConfigIdentity, DraftSourceIdentity, EntrySelection};
use super::recursive_source::AdmittedDraftSource;
use super::config::UsizeDecimal;
use super::wire;
use super::{CompileFailure, Evidence, InputPolicy, Observation, Plan, Plans, TargetDescriptor, EXECUTION_PROFILE};

#[derive(Serialize)]
pub struct DraftCompileArtifact {
    pub format: &'static str,
    pub qualification: &'static str,
    pub source: DraftSourceIdentity,
    pub execution_profile: &'static str,
    pub input_policy: InputPolicy,
    pub config: DraftConfigIdentity,
    pub target: TargetDescriptor,
    pub preparation: PreparationReport,
    // Existing complete OwnerWire views, not reconstructed stand-ins.
    pub admission: Value,
    pub genesis: Value,
    pub plans: Plans,
    pub observation: Observation,
    pub evidence: Evidence,
}

impl DraftCompileArtifact {
    /// This route never steps; publication/serialization failure keeps that fact.
    pub fn failure(&self, stage: &'static str, message: impl Into<String>) -> CompileFailure {
        CompileFailure::input(stage, message)
    }
}

#[derive(Clone, Copy, Serialize)]
pub enum SourceSort { UInt, Bool, List, Tree }
impl From<Type> for SourceSort {
    fn from(value: Type) -> Self {
        match value { Type::UInt => Self::UInt, Type::Bool => Self::Bool,
            Type::List => Self::List, Type::Tree => Self::Tree }
    }
}
#[derive(Clone, Copy, Serialize)]
pub struct SourceSpan { pub start: u32, pub end: u32 }
impl From<source::SourceSpan> for SourceSpan {
    fn from(value: source::SourceSpan) -> Self { Self { start: value.start, end: value.end } }
}
#[derive(Serialize)]
pub struct SourceLocation {
    pub function_index: Option<UsizeDecimal>,
    pub child_path: Vec<UsizeDecimal>,
}
impl From<&source::SourceLocation> for SourceLocation {
    fn from(value: &source::SourceLocation) -> Self {
        Self { function_index: value.function_index.map(UsizeDecimal),
            child_path: value.child_path.iter().copied().map(UsizeDecimal).collect() }
    }
}
#[derive(Serialize)]
pub struct BinderSite { pub name: String, pub span: SourceSpan }
impl From<&source::BinderSite> for BinderSite {
    fn from(value: &source::BinderSite) -> Self {
        Self { name: value.name.clone(), span: value.span.into() }
    }
}
#[derive(Serialize)]
pub struct ParameterSite { pub binder: BinderSite, pub type_span: SourceSpan }
#[derive(Serialize)]
pub struct FunctionSite {
    pub index: UsizeDecimal,
    pub span: SourceSpan,
    pub name_span: SourceSpan,
    pub parameters: Vec<ParameterSite>,
    pub return_type_span: SourceSpan,
}
#[derive(Serialize)]
pub struct ExpressionSite {
    pub source: SourceLocation,
    pub span: SourceSpan,
    pub kind: &'static str,
}
#[derive(Serialize)]
pub struct PatternSite {
    pub match_source: SourceLocation,
    pub arm_index: UsizeDecimal,
    pub span: SourceSpan,
    pub binders: Vec<BinderSite>,
}
#[derive(Serialize)]
pub struct SourceProvenance {
    pub language_span: SourceSpan,
    pub program_name_span: SourceSpan,
    pub functions: Vec<FunctionSite>,
    pub expressions: Vec<ExpressionSite>,
    pub patterns: Vec<PatternSite>,
}
impl From<&source::SourceProvenance> for SourceProvenance {
    fn from(value: &source::SourceProvenance) -> Self {
        Self {
            language_span: value.language_span.into(),
            program_name_span: value.program_name_span.into(),
            functions: value.functions.iter().map(|function| FunctionSite {
                index: UsizeDecimal(function.index), span: function.span.into(),
                name_span: function.name_span.into(),
                parameters: function.parameters.iter().map(|parameter| ParameterSite {
                    binder: (&parameter.binder).into(), type_span: parameter.type_span.into(),
                }).collect(), return_type_span: function.return_type_span.into(),
            }).collect(),
            expressions: value.expressions.iter().map(|expression| ExpressionSite {
                source: (&expression.source).into(), span: expression.span.into(),
                kind: expression.kind,
            }).collect(),
            patterns: value.patterns.iter().map(|pattern| PatternSite {
                match_source: (&pattern.match_source).into(), arm_index: UsizeDecimal(pattern.arm_index),
                span: pattern.span.into(), binders: pattern.binders.iter().map(BinderSite::from).collect(),
            }).collect(),
        }
    }
}
#[derive(Serialize)]
pub struct ParameterDeclaration { pub name: String, pub sort: SourceSort }
#[derive(Serialize)]
pub struct FunctionDeclaration {
    pub name: String,
    pub parameters: Vec<ParameterDeclaration>,
    pub return_sort: SourceSort,
}
#[derive(Serialize)]
pub struct TypeReport {
    pub function_returns: Vec<SourceSort>,
    pub entry_type: SourceSort,
}
#[derive(Serialize)]
pub struct PreparationReport {
    pub kind: &'static str,
    pub functions: Vec<FunctionDeclaration>,
    pub provenance: SourceProvenance,
    pub types: TypeReport,
    // This states the existing source/owner join, whose full cells the
    // independent oracle compares. It does not certify correspondence.
    pub owner_mapping: OwnerMapping,
}
#[derive(Serialize)]
pub struct OwnerMapping {
    pub program_view_pointer: &'static str,
    pub source_order: &'static str,
    pub join: &'static str,
    pub correspondence: &'static str,
}

/// Caller supplies identities/config from the same source preparation. This
/// projects that actual admitted pair; it does not validate digests or genesis.
/// The caller must pass the newly constructed, unstepped pair. No runtime
/// observation branch is exposed by this compile-only increment.
pub(crate) fn project_admitted_preparation(
    prepared: &AdmittedDraftSource,
    source_digest: String,
    config: DraftConfigIdentity,
    target: TargetDescriptor,
) -> Result<DraftCompileArtifact, HostViolation> {
    let observed = prepared.observe()?;
    let draft = &prepared.preparation.draft;
    let types = &prepared.preparation.types;
    Ok(DraftCompileArtifact {
        format: draft_contract::ARTIFACT_FORMAT,
        qualification: "HostOnly dedicated draft preparation; no constraints or native export",
        source: DraftSourceIdentity {
            language: draft_contract::SOURCE_PROFILE, semantics: draft_contract::SOURCE_SEMANTICS,
            program: draft.name.clone(), selection: EntrySelection::Entry, source_digest,
        },
        execution_profile: EXECUTION_PROFILE,
        input_policy: InputPolicy { kind: "closed-entry", declared: vec![], supplied: vec![] },
        config, target,
        preparation: PreparationReport {
            kind: "dedicated-source",
            functions: draft.program.functions.iter().map(|function| FunctionDeclaration {
                name: function.name.clone(),
                parameters: function.parameters.iter().map(|parameter| ParameterDeclaration {
                    name: parameter.name.clone(), sort: parameter.ty.into(),
                }).collect(), return_sort: function.return_type.into(),
            }).collect(),
            provenance: (&draft.provenance).into(),
            types: TypeReport {
                function_returns: types.function_returns.iter().copied().map(SourceSort::from).collect(),
                entry_type: types.entry_type.into(),
            },
            owner_mapping: OwnerMapping {
                program_view_pointer: "/admission/program_view",
                source_order: "entry-preorder-then-original-function-body-order",
                join: "source function_index/child_path to owner occurrence owner_is_function/function/path",
                correspondence: "unestablished-until-independent-comparison",
            },
        },
        admission: wire::admission(&observed), genesis: wire::image(&observed.image),
        plans: Plans {
            witness: Plan { status: "missing", artifact: None, reason: "dedicated draft witness plan not implemented" },
            constraint: Plan { status: "missing", artifact: None, reason: "dedicated draft constraint plan not implemented" },
            export: Plan { status: "unsupported", artifact: None, reason: "HostOnly; native/target export not implemented" },
        },
        observation: Observation::NotRequested { evaluation_performed: false },
        evidence: Evidence { source_preparation: "performed", host_observation: "genesis-only",
            native: "unrun", target: "unrun", acceptance: "unestablished" },
    })
}
