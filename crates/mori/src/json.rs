//! The `--json` report of `mori check`.
//!
//! Spans are UTF-8 byte ranges into the checked file. This format is our own;
//! it is not the TypeScript beta's.

use mori_ast::{Ast, NodeIdx};
use mori_checker::{Checked, DeclIdx, Support};
use mori_diagnostics::{Diagnostic, MoriDiagnostic, Severity};
use mori_lexer::TokenIdx;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub struct CheckReport {
    /// `accepted` when there are no diagnostics, otherwise `rejected`.
    pub status: &'static str,
    pub file: String,
    /// SHA-256 of the file's bytes, in lowercase hex.
    pub source_hash: String,
    /// Absent when the file did not parse.
    pub agreement: Option<Named>,
    pub declarations: Vec<DeclarationReport>,
    pub actions: Vec<ActionReport>,
    pub diagnostics: Vec<DiagnosticReport>,
}

#[derive(Serialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl From<mori_span::Span> for Span {
    fn from(span: mori_span::Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Serialize)]
pub struct Named {
    pub name: String,
    pub span: Span,
}

#[derive(Serialize)]
pub struct DeclarationReport {
    /// The declaring keyword, such as `asset` or `intent`.
    pub kind: String,
    pub name: String,
    pub span: Span,
    pub name_span: Span,
}

#[derive(Serialize)]
pub struct ActionReport {
    pub name: String,
    pub intent: String,
    /// `local_s0` when it runs locally, `specified_only` when it is only
    /// checked; absent when its intent did not check.
    pub support: Option<&'static str>,
    pub span: Span,
    pub name_span: Span,
}

#[derive(Serialize)]
pub struct DiagnosticReport {
    pub code: Option<String>,
    pub severity: &'static str,
    pub message: String,
    pub help: Option<String>,
    pub labels: Vec<LabelReport>,
}

#[derive(Serialize)]
pub struct LabelReport {
    pub span: Span,
    pub text: Option<String>,
    pub primary: bool,
}

impl CheckReport {
    pub fn new(
        file: String,
        source: &str,
        checked: Option<&Checked<'_>>,
        diagnostics: &[MoriDiagnostic],
    ) -> Self {
        let source_hash = Sha256::digest(source.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let mut report = Self {
            status: if diagnostics.is_empty() {
                "accepted"
            } else {
                "rejected"
            },
            file,
            source_hash,
            agreement: None,
            declarations: Vec::new(),
            actions: Vec::new(),
            diagnostics: diagnostics.iter().map(DiagnosticReport::new).collect(),
        };
        if let Some(checked) = checked {
            report.fill(checked);
        }
        report
    }

    fn fill(&mut self, checked: &Checked<'_>) {
        let ast = checked.ast;
        let name_token = ast.main_token(NodeIdx::ROOT);
        self.agreement = Some(Named {
            name: ast.token_text(name_token).to_owned(),
            span: token_span(ast, name_token).into(),
        });
        for index in 0..checked.declarations.len() {
            let decl = DeclIdx::new(index);
            let node = checked.declarations.node()[index];
            let keyword = ast.main_token(node);
            self.declarations.push(DeclarationReport {
                kind: ast.token_text(keyword).to_owned(),
                name: checked.name(decl).to_owned(),
                span: ast.span(node).into(),
                name_span: token_span(ast, ast.token_after(keyword, 1)).into(),
            });
        }
        for (&node, &support) in checked.actions.node().iter().zip(checked.actions.support()) {
            let keyword = ast.main_token(node);
            self.actions.push(ActionReport {
                name: ast.token_text(ast.token_after(keyword, 1)).to_owned(),
                intent: ast.token_text(ast.token_after(keyword, 3)).to_owned(),
                support: support.map(|support| match support {
                    Support::LocalS0 => "local_s0",
                    Support::SpecifiedOnly => "specified_only",
                }),
                span: ast.span(node).into(),
                name_span: token_span(ast, ast.token_after(keyword, 1)).into(),
            });
        }
    }
}

impl DiagnosticReport {
    fn new(diagnostic: &MoriDiagnostic) -> Self {
        let severity = match diagnostic.severity() {
            Some(Severity::Warning) => "warning",
            Some(Severity::Advice) => "advice",
            _ => "error",
        };
        let mut labels: Vec<LabelReport> = diagnostic
            .labels()
            .map(|labels| {
                labels
                    .map(|label| LabelReport {
                        span: Span {
                            start: to_u32(label.offset()),
                            end: to_u32(label.offset() + label.len()),
                        },
                        text: label.label().map(str::to_owned),
                        primary: label.primary(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        // Every diagnostic reports one primary location: the marked label, or
        // else the first.
        if !labels.iter().any(|label| label.primary)
            && let Some(first) = labels.first_mut()
        {
            first.primary = true;
        }
        Self {
            code: diagnostic.code().map(|code| code.to_string()),
            severity,
            message: diagnostic.to_string(),
            help: diagnostic.help().map(|help| help.to_string()),
            labels,
        }
    }
}

fn token_span(ast: &Ast<'_>, token: TokenIdx) -> mori_span::Span {
    ast.tokens.span(token, ast.source)
}

/// Sources are at most 65,536 bytes, so offsets fit in `u32`.
fn to_u32(offset: usize) -> u32 {
    u32::try_from(offset).expect("source offsets fit in u32")
}

#[cfg(test)]
mod tests {
    use super::CheckReport;

    /// Checks the source and snapshots its JSON report, recording the source.
    macro_rules! assert_report_snapshot {
        ($source:literal) => {{
            let source = indoc::indoc!($source);
            let report = match mori_parser::parse(source) {
                Err(error) => CheckReport::new("test.mori".to_owned(), source, None, &[error]),
                Ok(ast) => {
                    let result = mori_checker::check(&ast);
                    CheckReport::new("test.mori".to_owned(), source, Some(&result.checked), &result.diagnostics)
                }
            };
            insta::with_settings!({
                description => format!("Code:\n\n{source}"),
                omit_expression => true,
            }, {
                insta::assert_snapshot!(serde_json::to_string_pretty(&report).unwrap());
            });
        }};
    }

    #[test]
    fn accepted_report() {
        assert_report_snapshot!(
            r#"
            profile "moriarty-beta/1";
            agreement Oracle {
              domain Preview = { id: "preview", chain: "midnight", network: "preview" };
              observation price = { id: "price", domain: Preview };
              intent read = { operation: oracle.select(observation: price) };
              action fetch uses read;
            }
            "#
        );
    }

    #[test]
    fn rejected_report() {
        assert_report_snapshot!(
            r#"
            profile "moriarty-beta/1";
            agreement Invoice {
              const price = 10;
              const price = pirce;
            }
            "#
        );
    }

    #[test]
    fn unparsable_report() {
        assert_report_snapshot!(
            r#"
            profile "moriarty-beta/1";
            agreement Invoice {
              const price = 10
            "#
        );
    }
}
