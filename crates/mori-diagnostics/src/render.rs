//! Graphical rendering of diagnostics against their source.

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, Report};

use crate::{MoriDiagnostic, NamedUtf8Source};

/// Widest rendering, in columns. Snapshots always use this width.
pub const WIDTH: usize = 80;

/// Renders a report in color for a terminal `width` columns wide, capped at
/// [`WIDTH`]. Write it through a stream that strips color when it is off.
pub fn render_colored(report: &Report, width: usize) -> String {
    trim_lines(&render(
        GraphicalTheme::unicode(),
        width.min(WIDTH),
        report.as_ref(),
    ))
}

/// Renders a diagnostic against its source with no color and a fixed width.
///
/// The output is what a user sees, minus color and trailing spaces, so it is
/// stable across terminals and editors and suitable for snapshots.
pub fn render_plain(diagnostic: &MoriDiagnostic, name: &str, source: &str) -> String {
    let report = diagnostic
        .clone()
        .with_source_code(NamedUtf8Source::new(name, source.to_owned()));
    trim_lines(&render(
        GraphicalTheme::unicode_nocolor(),
        WIDTH,
        report.as_ref(),
    ))
}

/// Removes trailing spaces from every line.
fn trim_lines(text: &str) -> String {
    text.lines()
        .map(|line| line.trim_end().to_owned() + "\n")
        .collect()
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
                    .with_code("mori::syntax::quantity_without_space")
                    .with_label(LabeledSpan::at(
                        asset..asset + 3,
                        "this asset touches the number",
                    ))
                    .with_help_code("Put a space between them:", "const price = 10 USD;")
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

#[cfg(test)]
mod scalar_column_tests {
    use super::render_plain;
    use crate::{Diagnostic, LabeledSpan, MoriDiagnostic};

    // Headers count Unicode scalars, independently of UTF-8 byte spans,
    // UTF-16 editor positions and terminal display widths.
    macro_rules! assert_scalar_point_snapshot {
        ($name:literal, $source:literal, $byte:literal, $header:literal) => {{
            let source = $source;
            let byte: usize = $byte;
            assert!(source.is_char_boundary(byte));
            let diagnostic = MoriDiagnostic::error("An ordinary source point.")
                .with_code("mori::test::point")
                .with_label(LabeledSpan::new_primary_with_span(
                    Some("here".to_owned()),
                    (byte, 0),
                ));
            assert_eq!(
                diagnostic
                    .labels()
                    .unwrap()
                    .map(|label| (label.offset(), label.len()))
                    .collect::<Vec<_>>(),
                [(byte, 0)],
            );
            let rendered = render_plain(&diagnostic, "test.mori", source);
            assert!(rendered.contains($header), "{rendered}");
            assert_eq!(
                diagnostic
                    .labels()
                    .unwrap()
                    .map(|label| (label.offset(), label.len()))
                    .collect::<Vec<_>>(),
                [(byte, 0)],
            );
            insta::with_settings!({
                description => format!("Code:\n\n{source}"),
                omit_expression => true,
            }, {
                insta::assert_snapshot!($name, rendered);
            });
        }};
    }

    #[test]
    fn ascii_header_preserves_the_byte_position() {
        assert_scalar_point_snapshot!("human_scalar_ascii", "let x = @;\n", 8, "[test.mori:1:9]");
    }

    #[test]
    fn multibyte_prefix_header_counts_scalars() {
        assert_scalar_point_snapshot!(
            "human_scalar_multibyte_prefix",
            "/* café */ @\n",
            12,
            "[test.mori:1:12]"
        );
    }

    #[test]
    fn multibyte_eof_header_counts_scalars() {
        assert_scalar_point_snapshot!("human_scalar_eof", "café", 5, "[test.mori:1:5]");
    }

    #[test]
    fn crlf_context_preserves_lines_and_byte_positions() {
        assert_scalar_point_snapshot!(
            "human_scalar_crlf",
            "first\r\n/* café */ @\r\n",
            19,
            "[test.mori:2:12]"
        );
    }

    #[test]
    fn astral_prefix_header_is_not_utf16_or_display_width() {
        assert_scalar_point_snapshot!(
            "human_scalar_astral_prefix",
            "/* 😀 */ @\n",
            11,
            "[test.mori:1:9]"
        );
    }

    #[test]
    fn unspanned_diagnostic_does_not_invent_a_location() {
        let source = "/* café */\n";
        let diagnostic =
            MoriDiagnostic::error("An ordinary source point.").with_code("mori::test::point");
        assert!(diagnostic.labels().is_none());
        let rendered = render_plain(&diagnostic, "test.mori", source);
        assert!(!rendered.contains("test.mori:"), "{rendered}");
        assert!(diagnostic.labels().is_none());
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!("human_scalar_unspanned", rendered);
        });
    }
}
