//! Shared source analysis for the public check command and MCP diagnostics.

use mori_diagnostics::MoriDiagnostic;
use serde_json::Value;

use crate::json::CheckReport;

pub(super) enum CheckAnalysis {
    Beta {
        report: CheckReport,
        diagnostics: Vec<MoriDiagnostic>,
        parsed: Option<(usize, usize)>,
    },
    Core {
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    },
}

/// Uses the same profile dispatch for file checks and supplied source text.
pub(super) fn analyze_source(file: &str, source: &str) -> CheckAnalysis {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(source).is_ok_and(|language| language == "moriarty/2");
        let result = if staged {
            mori_core::stage::load(source).map(|program| program.program().clone())
        } else {
            mori_core::parse(source)
        };
        return CheckAnalysis::Core { result, staged };
    }

    match mori_parser::parse(source) {
        Err(error) => {
            let diagnostics = vec![error];
            CheckAnalysis::Beta {
                report: CheckReport::new(file.to_owned(), source, None, &diagnostics),
                diagnostics,
                parsed: None,
            }
        }
        Ok(ast) => {
            let parsed = Some((ast.tokens.len(), ast.nodes.len()));
            let result = mori_checker::check(&ast);
            // The checked AST is borrowed. Own its public report before it drops.
            let report = CheckReport::new(
                file.to_owned(),
                source,
                Some(&result.checked),
                &result.diagnostics,
            );
            CheckAnalysis::Beta {
                report,
                diagnostics: result.diagnostics,
                parsed,
            }
        }
    }
}

impl CheckAnalysis {
    pub(super) fn is_accepted(&self) -> bool {
        match self {
            Self::Beta { diagnostics, .. } => diagnostics.is_empty(),
            Self::Core { result, .. } => result.is_ok(),
        }
    }

    /// Preserves the existing full `mori check --json` objects.
    pub(super) fn cli_report(&self, file: &str, source: &str) -> Result<Value, serde_json::Error> {
        match self {
            Self::Beta { report, .. } => serde_json::to_value(report),
            Self::Core { result, staged } => {
                let mut report = match result {
                    Ok(program) => serde_json::json!({
                        "status": "accepted",
                        "file": file,
                        "source_hash": mori_core::intent::hash_bytes(source.as_bytes()),
                        "language": program.language,
                        "agreement": program.name,
                        "actions": program.actions,
                        "diagnostics": []
                    }),
                    Err(error) => serde_json::json!({
                        "status": "rejected", "file": file, "diagnostics": [error]
                    }),
                };
                if *staged {
                    for (name, value) in mori_core::stage::STAGE_LABELS {
                        report[name] = Value::String(value.to_owned());
                    }
                }
                Ok(report)
            }
        }
    }
}

#[cfg(test)]
mod tests;
