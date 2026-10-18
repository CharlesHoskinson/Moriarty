use lsp_types::{
    Diagnostic, DiagnosticRelatedInformation, DiagnosticSeverity, Location, Position, Range, Uri,
};
use mori_diagnostics::{Diagnostic as _, MoriDiagnostic, Severity};

use super::super::check_source::{self, SourceCheckConsumer};

/// Consume the public check command's source-profile dispatch without running an action.
pub(super) fn check(uri: &Uri, source: &str, related: bool) -> Vec<Diagnostic> {
    check_source::dispatch(
        source,
        LspConsumer {
            uri,
            source,
            related,
        },
    )
}

struct LspConsumer<'a> {
    uri: &'a Uri,
    source: &'a str,
    related: bool,
}

impl SourceCheckConsumer for LspConsumer<'_> {
    type Output = Vec<Diagnostic>;

    fn beta(
        self,
        _checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        diagnostics
            .iter()
            .map(|diagnostic| legacy(self.uri, self.source, diagnostic, self.related))
            .collect()
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        let profile = if staged { "moriarty/2" } else { "moriarty/1" };
        result
            .err()
            .map(|error| core(self.source, profile, error))
            .into_iter()
            .collect()
    }
}

fn core(source: &str, profile: &str, error: mori_core::Error) -> Diagnostic {
    let at = position(source, error.offset.unwrap_or(0) as usize);
    Diagnostic::new(
        Range::new(at, at),
        Some(DiagnosticSeverity::Error),
        Some(error.code.into()),
        None,
        Some(format!("moriarty-rust ({profile})")),
        error.message.into(),
        None,
        None,
        None,
    )
}

fn legacy(uri: &Uri, source: &str, diagnostic: &MoriDiagnostic, related: bool) -> Diagnostic {
    let labels: Vec<_> = diagnostic.labels().into_iter().flatten().collect();
    let primary = labels.iter().position(|label| label.primary()).unwrap_or(0);
    let range = labels.get(primary).map_or_else(
        || Range::new(Position::new(0, 0), Position::new(0, 0)),
        |label| label_range(source, label.offset(), label.len()),
    );
    let information: Vec<_> = if related {
        labels
            .iter()
            .enumerate()
            .filter(|(index, _)| *index != primary)
            .filter_map(|(_, label)| {
                label.label().map(|text| {
                    DiagnosticRelatedInformation::new(
                        Location::new(
                            uri.clone(),
                            label_range(source, label.offset(), label.len()),
                        ),
                        text.to_owned(),
                    )
                })
            })
            .collect()
    } else {
        Vec::new()
    };
    let mut message = diagnostic.to_string();
    if let Some(help) = diagnostic.help() {
        message.push_str("\n\n");
        message.push_str(&help.to_string());
    }
    let severity = match diagnostic.severity() {
        Some(Severity::Warning) => DiagnosticSeverity::Warning,
        Some(Severity::Advice) => DiagnosticSeverity::Information,
        _ => DiagnosticSeverity::Error,
    };
    Diagnostic::new(
        range,
        Some(severity),
        diagnostic.code().map(|code| code.to_string().into()),
        None,
        Some("moriarty-rust (moriarty-beta/1)".into()),
        message.into(),
        None,
        (!information.is_empty()).then_some(information),
        None,
    )
}

fn label_range(source: &str, offset: usize, length: usize) -> Range {
    Range::new(
        position(source, offset),
        position(source, offset.saturating_add(length)),
    )
}

/// LSP positions count UTF-16 units and treat CRLF as one line break.
fn position(source: &str, byte: usize) -> Position {
    let mut byte = byte.min(source.len());
    while !source.is_char_boundary(byte) {
        byte -= 1;
    }
    let mut line = 0;
    let mut character = 0;
    let mut chars = source.char_indices().peekable();
    while let Some((offset, ch)) = chars.next() {
        if offset >= byte {
            break;
        }
        match ch {
            '\r' => {
                if let Some(&(next, '\n')) = chars.peek() {
                    // A byte position between CR and LF denotes the preceding line end.
                    if byte <= next {
                        break;
                    }
                    chars.next();
                }
                line += 1;
                character = 0;
            }
            '\n' => {
                line += 1;
                character = 0;
            }
            _ => character += ch.len_utf16() as u32,
        }
    }
    Position::new(line, character)
}
