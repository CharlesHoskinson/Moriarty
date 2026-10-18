//! Ordinary compiler result plus separate source-static projection metadata.
//! This module never invokes Sherlock execution or changes the target artifact.
use crate::{Artifact, Error, Target};
use mori_core::{
    Program,
    sherlock_source::{
        ActionProjection, ProjectionDiagnostic, SourceProjectionReport, source_projection_report,
    },
};
use serde::Serialize;

/// Report-wide ceiling on occurrence path elements, charged by this wrapper.
/// Core charges occurrences, but every path repeats one element per preceding
/// let, so path bytes grow quadratically in let count within that budget.
/// Allow each budgeted occurrence a path within the Core depth bound (64 plus
/// the let value slot) and the occurrence itself. Exhaustion withholds that
/// action as metadata; it never refuses or alters the compiled artifact.
pub const MAX_PROJECTION_PATH_ELEMENTS: usize =
    mori_core::sherlock_source::MAX_PROJECTION_OCCURRENCES * 66;

#[derive(Serialize)]
pub struct CompilationSourceProjectionReport {
    pub compiled_program_hash: String,
    pub artifact_format: String,
    pub abi: String,
    pub relation: String,
    pub projection: SourceProjectionReport,
}

pub struct ReportedCompilation {
    pub artifact: Artifact,
    pub source_projection: CompilationSourceProjectionReport,
}

pub fn compile_reported(program: &Program, target: &Target) -> Result<ReportedCompilation, Error> {
    // Preserve the same signed/unsigned selection as the existing CLI.
    let artifact = if target.signed_policy.is_some() {
        crate::compile_signed(program, target)
    } else {
        crate::compile(program, target)
    }?;
    // An unsupported projection remains metadata after successful compilation.
    let mut projection = source_projection_report(program);
    bound_paths(&mut projection);
    let source_projection = CompilationSourceProjectionReport {
        compiled_program_hash: artifact.program_hash.clone(),
        artifact_format: artifact.format.clone(),
        abi: artifact.abi.clone(),
        relation: artifact.relation.clone(),
        projection,
    };
    Ok(ReportedCompilation {
        artifact,
        source_projection,
    })
}

/// Charge each prepared action's occurrences and path elements, in action
/// order, against one report budget. An action that would exceed it is
/// withheld as Unsupported metadata; withheld actions are not charged.
fn bound_paths(projection: &mut SourceProjectionReport) {
    let mut remaining = MAX_PROJECTION_PATH_ELEMENTS;
    for entry in &mut projection.actions {
        let withheld = match entry {
            ActionProjection::Prepared { report } => {
                let cost = report
                    .occurrences
                    .iter()
                    .try_fold(0usize, |sum, occurrence| {
                        sum.checked_add(occurrence.body_path.len())?.checked_add(1)
                    });
                match cost.and_then(|cost| remaining.checked_sub(cost)) {
                    Some(left) => {
                        remaining = left;
                        None
                    }
                    None => Some((report.action_index, report.action.clone())),
                }
            }
            ActionProjection::Unsupported { .. } => None,
        };
        if let Some((action_index, action)) = withheld {
            *entry = ActionProjection::Unsupported {
                action_index,
                action,
                diagnostic: ProjectionDiagnostic {
                    action_index,
                    statement_index: None,
                    expression: None,
                    construct: "projection-path-budget".into(),
                    code: "projection-path-budget-exhausted".into(),
                },
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn reported_compile_preserves_target_artifact_and_contains_projection() {
        for specimen in [
            "language \"moriarty/1\"; agreement Reported { action quote(units: UInt, price: UInt) { let gross = units * price; let result = gross + 3; emit \"total\"(result); } }",
            "language \"moriarty/1\"; agreement Predicate { action inspect(observed: UInt, threshold: UInt) { let below = observed < threshold; let result = not below; emit \"atOrAbove\"(result); } }",
        ] {
            let program = mori_core::parse(specimen).unwrap();
            let target = Target::ledger9("local", "reported");
            let old = crate::compile(&program, &target).unwrap();
            let reported = compile_reported(&program, &target).unwrap();
            assert_eq!(
                serde_json::to_value(&old).unwrap(),
                serde_json::to_value(&reported.artifact).unwrap()
            );
            assert_eq!(
                reported.source_projection.compiled_program_hash,
                reported.artifact.program_hash
            );
            assert!(!reported.source_projection.projection.evaluation_performed);
            assert!(matches!(
                &reported.source_projection.projection.actions[0],
                mori_core::sherlock_source::ActionProjection::Prepared { .. }
            ));
        }
    }
    #[test]
    fn quadratic_let_paths_are_withheld_as_metadata_not_refused() {
        // 600 lets cost 361,801 path elements (prepared); 900 cost 812,701 (withheld).
        for (lets, prepared) in [(600, true), (900, false)] {
            let body: String = (0..lets).map(|i| format!("let v{i} = 0; ")).collect();
            let specimen = format!(
                "language \"moriarty/1\"; agreement Long {{ action chain() {{ {body}emit \"done\"(v{}); }} }}",
                lets - 1
            );
            let program = mori_core::parse(&specimen).unwrap();
            let target = Target::ledger9("local", "reported");
            let old = crate::compile(&program, &target).unwrap();
            let reported = compile_reported(&program, &target).unwrap();
            assert_eq!(
                serde_json::to_value(&old).unwrap(),
                serde_json::to_value(&reported.artifact).unwrap()
            );
            match &reported.source_projection.projection.actions[0] {
                ActionProjection::Prepared { .. } => assert!(prepared),
                ActionProjection::Unsupported {
                    action_index,
                    action,
                    diagnostic,
                } => {
                    assert!(!prepared);
                    assert_eq!((*action_index, action.as_str()), (0, "chain"));
                    assert_eq!(diagnostic.code, "projection-path-budget-exhausted");
                }
            }
        }
    }
}
