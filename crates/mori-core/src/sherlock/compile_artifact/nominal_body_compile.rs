//! Selected public GENERIC_ABSENT nominal body-typed HostOnly compiler.
//!
//! Bounded parse -> first owner T3 -> all bodies/entry -> actual raw compiler
//! (complete owner admission including second T3, then actual genesis). The
//! first T3 and checker intentionally precede AD-0a/1/2/3. No owner guard is
//! reproduced here. Parser extents do not bound the duplicate T3/checker work.
use super::ext_compile::{
    compile_ext_host, ExtCompileArtifact, ExtCompileConfig, ExtCompileFailure, ExtCompileReport,
};
use crate::sherlock::helper_machine::shared::{
    prepare_ext_program_types, HostViolation, LocatorC, Tooling,
};
use crate::sherlock::helper_machine::shared::owner_ext_source_program::ExtProgram;
use crate::sherlock::helper_machine::shared::owner_ext_t3_uses::{ExtExpr, ExtValue};
use crate::sherlock::helper_machine::shared::owner_ext_type_builder::ExtTypeBuildError;
use crate::sherlock_nominal_body_types::{
    check_nominal_program_bodies, NominalBodyTypeError, NominalBodyTypeErrorKind,
};
use crate::sherlock_nominal_source::{
    parse_nominal_source_bounded, DraftError, DraftParseBudget, NominalProvenance,
    SourceOwner, SourceSpan,
};

/// Source profile selected only through this completed body-checking route.
/// This does not change the raw compiler's execution profile or financial scope.
pub const NOMINAL_BODY_TYPED_PROFILE: &str = "moriarty-sherlock-nominal-body-typed/1";
pub const ADOPTED_NOMINAL_JUDGMENTS: &[&str] = &[
    "eq-any", "unique-parameters-every-declaration", "unique-binders-every-arm",
];

/// New exhaustive stage enum; existing raw/source error enums stay unchanged.
/// Every payload is the actual original error, including second-T3 owner errors.
pub enum NominalBodyCompileError {
    Parse(DraftError),
    Preparation(ExtTypeBuildError),
    BodyType(NominalBodyTypeError),
    OwnerAdmission(ExtTypeBuildError),
    Genesis(Tooling),
}
impl std::fmt::Debug for NominalBodyCompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(e) => f.debug_tuple("Parse").field(e).finish(),
            Self::Preparation(_) => f.write_str("Preparation(owner ExtTypeBuildError)"),
            Self::BodyType(e) => f.debug_tuple("BodyType").field(e).finish(),
            Self::OwnerAdmission(_) => f.write_str("OwnerAdmission(owner ExtTypeBuildError)"),
            Self::Genesis(_) => f.write_str("Genesis(owner Tooling)"),
        }
    }
}

#[derive(Debug)]
pub struct NominalBodyCompileFailure {
    pub error: NominalBodyCompileError,
    /// Exact parser-provenance site for a body diagnostic, when valid. The
    /// intact typed BodyType payload retains its original owner/path and kind.
    pub span: Option<SourceSpan>,
}
impl NominalBodyCompileFailure {
    fn unlocated(error: NominalBodyCompileError) -> Self { Self { error, span: None } }
    /// Body checking completed only on reaching actual owner admission/genesis.
    /// A BodyType failure means that checking ran and failed, not completed.
    pub fn body_check_completed(&self) -> bool {
        matches!(self.error, NominalBodyCompileError::OwnerAdmission(_) | NominalBodyCompileError::Genesis(_))
    }
}

/// No public constructor can attach completed typing to an unchecked artifact.
pub struct NominalBodyCompileArtifact {
    name: String,
    provenance: NominalProvenance,
    owner: ExtCompileArtifact,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CompletedNominalBodyCheck {
    pub status: &'static str,
    pub judgments: &'static [&'static str],
}
/// Wraps the actual unchanged raw report; no first-preparation TypeId escapes.
pub struct NominalBodyCompileReport<'a> {
    pub source_profile: &'static str,
    pub body_check: CompletedNominalBodyCheck,
    pub owner: ExtCompileReport<'a>,
}

/// Compile the selected nominal text source through the actual owner APIs.
///
/// Exactly config.layout/domain and locator.clone() feed first preparation.
/// All-body checking borrows that preparation. Its summary is discarded because
/// its type IDs refer only to the first table. The SAME prepared.program moves
/// into compile_ext_host, whose admission checks and second T3 remain intact.
///
/// This composite wrapper performs first T3 and body work BEFORE owner guards.
/// Bad body plus step_cap=0 returns BodyType; welltyped body reaches the actual
/// owner refusal. Widths:Some can incur full first T3/body work before refusing.
/// Only GENERIC_ABSENT is supported: first owner preparation refuses financial
/// and DedicatedSorts domains. This adapter adds no guard/domain/default policy,
/// evaluator steps, SDK operation, native artifact or correspondence claim.
pub fn compile_nominal_source_host_bounded(
    source: &str,
    budget: DraftParseBudget,
    config: ExtCompileConfig,
    locator: Option<LocatorC>,
) -> Result<NominalBodyCompileArtifact, NominalBodyCompileFailure> {
    let parsed = parse_nominal_source_bounded(source, budget)
        .map_err(|e| NominalBodyCompileFailure::unlocated(NominalBodyCompileError::Parse(e)))?;
    let program = {
        let prepared = prepare_ext_program_types(
            parsed.program, &config.layout, config.domain, locator.clone(),
        ).map_err(|e| NominalBodyCompileFailure::unlocated(NominalBodyCompileError::Preparation(e)))?;
        if let Err(error) = check_nominal_program_bodies(&prepared) {
            let span = nominal_body_error_span(source, &prepared.program, &parsed.provenance, &error);
            return Err(NominalBodyCompileFailure { error: NominalBodyCompileError::BodyType(error), span });
        }
        // REPORT_ONLY_COMPLETED: discard the summary, including first-T3 IDs.
        // Move the same program; the other first-preparation fields drop on
        // leaving this scope before actual admission builds its second tables.
        prepared.program
    };
    let owner = compile_ext_host(program, config, locator).map_err(|error| {
        NominalBodyCompileFailure::unlocated(match error {
            ExtCompileFailure::Preparation(e) => NominalBodyCompileError::OwnerAdmission(e),
            ExtCompileFailure::Genesis(e) => NominalBodyCompileError::Genesis(e),
        })
    })?;
    Ok(NominalBodyCompileArtifact { name: parsed.name, provenance: parsed.provenance, owner })
}
impl NominalBodyCompileArtifact {
    pub fn name(&self) -> &str { &self.name }
    pub fn provenance(&self) -> &NominalProvenance { &self.provenance }
    pub fn owner(&self) -> &ExtCompileArtifact { &self.owner }
    pub fn report(&self) -> Result<NominalBodyCompileReport<'_>, HostViolation> {
        Ok(NominalBodyCompileReport {
            source_profile: NOMINAL_BODY_TYPED_PROFILE,
            body_check: CompletedNominalBodyCheck { status: "completed", judgments: ADOPTED_NOMINAL_JUDGMENTS },
            owner: self.owner.report()?,
        })
    }
    /// Retain the actual raw admitted/genesis pair and complete source metadata.
    pub fn into_parts(self) -> (String, NominalProvenance, ExtCompileArtifact) {
        (self.name, self.provenance, self.owner)
    }
}

/// Map an intact checker error using exact owner/path parser provenance.
/// Invalid/missing/ambiguous records have no span; no nearest site is invented.
/// Pattern errors map their arm BODY, not a guessed pattern field. ExtValue
/// field suffixes are validated within the literal and truncate at Literal.
/// This helper also supports errors from the public owned-AST checker API.
pub fn nominal_body_error_span(
    source: &str,
    program: &ExtProgram,
    provenance: &NominalProvenance,
    error: &NominalBodyTypeError,
) -> Option<SourceSpan> {
    let valid = |span: SourceSpan| {
        source.get(span.start as usize..span.end as usize).map(|_| span)
    };
    match &error.kind {
        NominalBodyTypeErrorKind::UnsupportedDomain
        | NominalBodyTypeErrorKind::IncompletePreparation(_)
        | NominalBodyTypeErrorKind::Preparation(_) => return None,
        NominalBodyTypeErrorKind::DuplicateParameter { name, index } => {
            if !error.source.child_path.is_empty() { return None; }
            let function = error.source.function_index?;
            let parameter = program.functions.get(function)?.parameters.get(*index)?;
            let site = provenance.functions.get(function)?.parameters.get(*index)?;
            if &parameter.name != name || &site.binder.name != name { return None; }
            return valid(site.binder.span);
        }
        _ => {}
    }
    let (owner, mut expr) = match error.source.function_index {
        None => (SourceOwner::Entry, &program.entry),
        Some(i) => (SourceOwner::Function(i), &program.functions.get(i)?.body),
    };
    let path = &error.source.child_path;
    let mut depth = 0;
    while depth < path.len() {
        if let ExtExpr::Literal { value } = expr {
            let mut value = value;
            for index in &path[depth..] {
                value = match value {
                    ExtValue::Constructor { fields, .. } => fields.get(*index)?,
                    ExtValue::Core { .. } => return None,
                };
            }
            break;
        }
        let index = path[depth];
        expr = match expr {
            ExtExpr::Let { value, body, .. } => match index { 0 => value, 1 => body, _ => return None },
            ExtExpr::Binary { left, right, .. } => match index { 0 => left, 1 => right, _ => return None },
            ExtExpr::Unary { operand, .. } if index == 0 => operand,
            ExtExpr::If { cond, then_expr, else_expr } => match index {
                0 => cond, 1 => then_expr, 2 => else_expr, _ => return None,
            },
            ExtExpr::Call { args, .. } => args.get(index)?,
            ExtExpr::Construct { fields, .. } => fields.get(index)?,
            ExtExpr::Match { scrutinee, arms } => if index == 0 { scrutinee } else { &arms.get(index - 1)?.body },
            _ => return None,
        };
        depth += 1;
    }
    let mut sites = provenance.expressions.iter()
        .filter(|site| site.owner == owner && site.child_path == path[..depth]);
    let site = sites.next()?;
    if sites.next().is_some() { return None; }
    valid(site.span)
}
