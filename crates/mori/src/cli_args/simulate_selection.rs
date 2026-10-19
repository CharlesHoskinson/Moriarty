//! Pure production draft selection, shared with the actual simulate handler.
//! No input reads, runtime stepping, backend or ledger imports.
use std::path::Path;
use mori_core::sherlock_compile_artifact::{self as host, CompileFailure};
use super::Args;

/// Pure command selection before input reads. Clap enforces the same conflicts
/// for public invocations; the explicit check also keeps direct callers honest.
pub fn selection(args: &Args) -> Result<(&Path, &Path), CompileFailure> {
    if args.action.is_some() || args.scenario.is_some() || args.resume.is_some()
        || args.event.is_some() || args.trace || args.logical_cap.is_some()
        || args.authorization.present()
    {
        return Err(CompileFailure::input("select",
            "selected Sherlock draft simulation does not accept action/scenario, resume/event, trace, logical-cap or signed replay arguments"));
    }
    let execution_profile = args.execution_profile.as_deref().ok_or_else(||
        CompileFailure::input("select", "missing --execution-profile"))?;
    if execution_profile != host::EXECUTION_PROFILE {
        return Err(CompileFailure::input("select",
            format!("unsupported execution profile: {execution_profile}")));
    }
    let source_profile = args.source_profile.as_deref().ok_or_else(||
        CompileFailure::input("select", "selected Sherlock simulation requires explicit --source-profile"))?;
    match host::draft_contract::select_source(Some(source_profile), None, args.entry) {
        Ok(host::draft_contract::SourceSelection::DraftEntry) => {}
        Ok(host::draft_contract::SourceSelection::CoreAction(_)
            | host::draft_contract::SourceSelection::NominalEntry) => {
            return Err(CompileFailure::input("select", "selected Sherlock simulation supports the explicit dedicated draft entry"));
        }
        Err(error) => {
            return Err(CompileFailure {
                stage: "select", actual_error: serde_json::json!({
                    "selection_error": format!("{error:?}"),
                    "source_profile": source_profile, "entry": args.entry,
                }), evaluation_performed: false, events_forwarded: "0".into(), completed: false,
            });
        }
    }
    let config = args.execution_config.as_deref().ok_or_else(||
        CompileFailure::input("config", "selected Sherlock simulation requires --execution-config"))?;
    let target = args.target.as_deref().ok_or_else(||
        CompileFailure::input("config", "selected Sherlock simulation requires --target"))?;
    Ok((config, target))
}

