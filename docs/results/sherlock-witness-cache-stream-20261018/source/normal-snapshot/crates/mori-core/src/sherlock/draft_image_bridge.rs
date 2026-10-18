//! SOURCE_ONLY / UNRUN: Core-only cfg(test) bridge to actual private image code.
//! Requires precisely the crate-only visibility changes in shared-interface.patch.
use crate::sherlock::program_image::{self, ExprId, ProgramImageDraft, SourceLocator, SourceOwner};
use crate::sherlock_draft::{parse_draft_source, DraftError, ExpressionSite, ParsedDraft};
use crate::sherlock_draft_types::{check_program_types, DraftTypeError, DraftTypeSummary};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ImageSourceSite { pub expression: ExprId, pub source: ExpressionSite }

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DraftImagePreparation {
    pub draft: ParsedDraft,
    pub types: DraftTypeSummary,
    pub image: Arc<ProgramImageDraft>,
    pub image_sources: Vec<ImageSourceSite>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum DraftImageError {
    Source(DraftError),
    Types(DraftTypeError),
    MissingProvenance(SourceLocator),
    ProvenanceCount { image: usize, provenance: usize },
}

/// Parse -> ordinary candidate typing -> actual unchanged image builder.
/// No machine execution, admission/resource policy or new public runtime path.
///
/// # Security Warning (H1 — Unmetered Host Evaluator)
/// This machine is a host diagnostic reference with zero native authority and no host time,
/// heap, or native stack admission bounds. Do not expose it to untrusted inputs.
pub(crate) fn prepare_draft_image(source: &str) -> Result<DraftImagePreparation, DraftImageError> {
    let draft = parse_draft_source(source).map_err(DraftImageError::Source)?;
    let types = check_program_types(&draft.program).map_err(DraftImageError::Types)?;
    let image = Arc::new(program_image::prepare_program_image(&draft.program));
    if image.source_map.len() != draft.provenance.expressions.len() {
        return Err(DraftImageError::ProvenanceCount {
            image: image.source_map.len(), provenance: draft.provenance.expressions.len() });
    }
    let image_sources = image.source_map.iter().zip(&draft.provenance.expressions).enumerate()
        .map(|(index, (locator, site))| {
            let function_index = match locator.owner {
                SourceOwner::Entry => None,
                SourceOwner::Function(function) => Some(function),
            };
            if site.source.function_index != function_index || site.source.child_path != locator.ast_path {
                return Err(DraftImageError::MissingProvenance(locator.clone()));
            }
            Ok(ImageSourceSite { expression: ExprId(index), source: site.clone() })
        }).collect::<Result<Vec<_>, DraftImageError>>()?;
    Ok(DraftImagePreparation { draft, types, image, image_sources })
}
