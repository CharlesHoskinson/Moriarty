//! Graphical rendering of diagnostics against their source.

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, NamedSource, Report};

use crate::MoriDiagnostic;

/// Widest rendering, in columns. Snapshots always use this width.
pub const WIDTH: usize = 80;

/// Renders a report in color for a terminal `width` columns wide, capped at
/// [`WIDTH`]. Write it through a stream that strips color when it is off.
pub fn render_colored(report: &Report, width: usize) -> String {
    render(GraphicalTheme::unicode(), width.min(WIDTH), report.as_ref())
}

/// Renders a diagnostic against its source with no color and a fixed width.
///
/// The output is exactly what a user sees, minus color, so it is stable
/// across terminals and suitable for snapshots.
pub fn render_plain(diagnostic: &MoriDiagnostic, name: &str, source: &str) -> String {
    let report = diagnostic
        .clone()
        .with_source_code(NamedSource::new(name, source.to_owned()));
    render(GraphicalTheme::unicode_nocolor(), WIDTH, report.as_ref())
}

fn render(theme: GraphicalTheme, width: usize, diagnostic: &dyn Diagnostic) -> String {
    let mut out = String::new();
    GraphicalReportHandler::new_themed(theme)
        .with_width(width)
        .with_links(false)
        .render_report(&mut out, diagnostic)
        .expect("writing to a String cannot fail");
    out
}

#[cfg(test)]
mod tests {
    use super::render_plain;
    use crate::{LabeledSpan, MoriDiagnostic};

    /// Renders the diagnostic built from `$source` and snapshots it, recording
    /// the source rather than the Rust expression.
    macro_rules! assert_render_snapshot {
        ($source:literal, $build:expr) => {{
            let source = indoc::indoc!($source);
            let build: fn(&str) -> MoriDiagnostic = $build;
            let diagnostic = build(source);
            insta::with_settings!({
                description => format!("Code:\n\n{source}"),
                omit_expression => true,
            }, {
                insta::assert_snapshot!(render_plain(&diagnostic, "test.mori", source));
            });
        }};
    }

    #[test]
    fn error_with_code_label_and_help() {
        assert_render_snapshot!(
            r#"
            const price = 10USD;
            "#,
            |source| {
                let asset = source.find("USD").unwrap();
                MoriDiagnostic::error("I need a space between a number and its asset.")
                    .with_code("BETA_QUANTITY_SEPARATOR")
                    .with_label(LabeledSpan::at(
                        asset..asset + 3,
                        "this asset touches the number",
                    ))
                    .with_help("Write the quantity as `10 USD`.")
            }
        );
    }

    #[test]
    fn warning_without_labels() {
        assert_render_snapshot!(
            r#"
            profile "moriarty-beta/1";
            "#,
            |_| MoriDiagnostic::warning("This agreement declares no actions.")
        );
    }
}
