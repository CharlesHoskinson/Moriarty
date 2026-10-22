//! Named UTF-8 source with Unicode-scalar diagnostic columns.

use miette::{MietteError, NamedSource, SourceCode, SourceSpan, SpanContents};

/// Owned UTF-8 source for human diagnostics.
///
/// Span offsets and data remain UTF-8 bytes. The zero-based column returned
/// by [`SourceCode::read_span`] counts Unicode scalars, so graphical headers
/// display one-based scalar columns. It does not count UTF-16 or display cells.
/// The generic [`NamedSource`] API retains its existing behavior.
#[derive(Debug)]
pub struct NamedUtf8Source {
    inner: NamedSource<String>,
}

impl NamedUtf8Source {
    pub fn new(name: impl AsRef<str>, source: String) -> Self {
        Self {
            inner: NamedSource::new(name, source),
        }
    }

    /// Retains the source language metadata used by syntax highlighters.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.inner = self.inner.with_language(language);
        self
    }
}

impl SourceCode for NamedUtf8Source {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        context_lines_before: usize,
        context_lines_after: usize,
    ) -> Result<Box<dyn SpanContents<'a> + 'a>, MietteError> {
        let contents = self
            .inner
            .read_span(span, context_lines_before, context_lines_after)?;
        let column = scalar_column(
            self.inner.inner(),
            contents.span().offset(),
            contents.line(),
        )
        .unwrap_or_else(|| contents.column());
        Ok(Box::new(ScalarContents { contents, column }))
    }
}

/// Derives the column of the returned data start on the delegated logical line.
fn scalar_column(source: &str, start: usize, line: usize) -> Option<usize> {
    // Arbitrary Miette byte spans may start inside a UTF-8 character. Keep the
    // delegate's column for them rather than slicing at an invalid boundary.
    source.get(..start)?;
    let mut chars = source.char_indices().peekable();
    let mut current_line = 0;
    let mut line_start = 0;
    while current_line < line {
        let (index, character) = chars.next()?;
        if matches!(character, '\r' | '\n') {
            line_start = index + 1;
            if character == '\r' && chars.peek().is_some_and(|(_, next)| *next == '\n') {
                let (index, _) = chars.next()?;
                line_start = index + 1;
            }
            current_line += 1;
        }
    }
    // For a point on either CRLF byte, Miette can return the preceding line.
    // Count its content, excluding terminators, while retaining that line.
    let prefix = source.get(line_start..start)?;
    Some(
        prefix
            .chars()
            .take_while(|character| !matches!(*character, '\r' | '\n'))
            .count(),
    )
}

struct ScalarContents<'a> {
    contents: Box<dyn SpanContents<'a> + 'a>,
    column: usize,
}

impl<'a> SpanContents<'a> for ScalarContents<'a> {
    fn data(&self) -> &'a [u8] {
        self.contents.data()
    }

    fn span(&self) -> &SourceSpan {
        self.contents.span()
    }

    fn name(&self) -> Option<&str> {
        self.contents.name()
    }

    fn line(&self) -> usize {
        self.contents.line()
    }

    fn column(&self) -> usize {
        self.column
    }

    fn line_count(&self) -> usize {
        self.contents.line_count()
    }

    fn language(&self) -> Option<&str> {
        self.contents.language()
    }
}

#[cfg(test)]
mod tests {
    use super::NamedUtf8Source;
    use crate::{Diagnostic, LabeledSpan, MoriDiagnostic, NamedSource};
    use miette::{MietteError, SourceCode};

    fn assert_read(
        source: &str,
        point: usize,
        length: usize,
        before: usize,
        after: usize,
        expected: (usize, usize, usize),
    ) {
        let delegated =
            NamedSource::new("columns.mori", source.to_owned()).with_language("moriarty");
        let adapted =
            NamedUtf8Source::new("columns.mori", source.to_owned()).with_language("moriarty");
        let span = (point, length).into();
        let raw = delegated.read_span(&span, before, after).unwrap();
        let actual = adapted.read_span(&span, before, after).unwrap();
        assert_eq!(actual.data(), raw.data(), "data: {source:?}");
        assert_eq!(actual.span(), raw.span(), "byte span: {source:?}");
        assert_eq!(actual.name(), raw.name());
        assert_eq!(actual.name(), Some("columns.mori"));
        assert_eq!(actual.language(), raw.language());
        assert_eq!(actual.language(), Some("moriarty"));
        assert_eq!(actual.line(), raw.line());
        assert_eq!(actual.line_count(), raw.line_count());
        assert_eq!(
            (actual.span().offset(), actual.line(), actual.column()),
            expected,
            "source {source:?}; point {point}; context ({before}, {after})",
        );
    }

    #[test]
    fn normal_points_preserve_bytes_and_count_scalars() {
        // Independent literal byte offset, delegated line and scalar column.
        for (source, point, line, column) in [
            ("", 0, 0, 0),
            ("let x = @;\n", 8, 0, 8),
            ("/* café */ @\n", 12, 0, 11),
            ("/* 😀 */ @\n", 11, 0, 8),
            ("café", 0, 0, 0),
            ("café", 5, 0, 4),
            ("café\r\n", 7, 1, 0),
        ] {
            assert_read(source, point, 0, 0, 0, (point, line, column));
        }
    }

    #[test]
    fn crlf_halves_keep_the_delegated_line_in_every_context() {
        let source = "é\r\nx";
        for before in 0..=2 {
            for after in 0..=2 {
                for (point, line, column) in [(2, 0, 1), (3, 0, 1), (4, 1, 0)] {
                    let expected = if before == 0 {
                        (point, line, column)
                    } else {
                        (0, 0, 0)
                    };
                    assert_read(source, point, 0, before, after, expected);
                }
            }
        }
        let raw = NamedSource::new("columns.mori", source.to_owned());
        let lf = raw.read_span(&(3, 0).into(), 0, 0).unwrap();
        assert_eq!((lf.line(), lf.column()), (0, 2));
    }

    #[test]
    fn surrounding_context_uses_the_returned_span_start() {
        let source = "first\r\nmiddle\r\n/* café */ @\r\ntail\r\nlast";
        for (before, start, line, column) in [(0, 27, 2, 11), (1, 7, 1, 0), (2, 0, 0, 0)] {
            for after in 0..=2 {
                assert_read(source, 27, 0, before, after, (start, line, column));
            }
        }
    }

    #[test]
    fn non_boundary_returned_start_keeps_delegated_column() {
        // Byte 1 is inside é. Preserve raw byte data and the delegated fallback,
        // even when following context changes its returned column metadata.
        for (after, column) in [(0, 1), (1, 0), (2, 0)] {
            assert_read("é\r\nx", 1, 1, 0, after, (1, 0, column));
        }
    }

    #[test]
    fn delegated_out_of_bounds_errors_are_unchanged() {
        let source = "café";
        let raw = NamedSource::new("columns.mori", source.to_owned());
        let adapted = NamedUtf8Source::new("columns.mori", source.to_owned());
        let span = (100, 1).into();
        assert!(matches!(
            raw.read_span(&span, 0, 0),
            Err(MietteError::OutOfBounds)
        ));
        assert!(matches!(
            adapted.read_span(&span, 0, 0),
            Err(MietteError::OutOfBounds)
        ));
    }

    #[test]
    fn generic_named_source_and_report_api_remain_usable() {
        let source = "/* café */ @\n";
        let generic = NamedSource::new("generic.mori", source.to_owned());
        let contents = generic.read_span(&(12, 0).into(), 0, 0).unwrap();
        assert_eq!((contents.line(), contents.column()), (0, 12));
        let report = MoriDiagnostic::error("ordinary diagnostic")
            .with_source_code(NamedSource::new("generic.mori", source.to_owned()));
        assert_eq!(report.to_string(), "ordinary diagnostic");
    }

    #[test]
    fn colored_and_plain_attachments_share_scalar_headers() {
        use crate::render::{WIDTH, render_colored, render_plain};
        use miette::{GraphicalReportHandler, GraphicalTheme};

        let source = "/* 😀 */ @\n";
        let diagnostic = MoriDiagnostic::error("An ordinary source point.")
            .with_code("mori::test::point")
            .with_label(LabeledSpan::new_primary_with_span(
                Some("here".to_owned()),
                (11, 0),
            ));
        let report = diagnostic.clone().with_source_code(
            NamedUtf8Source::new("test.mori", source.to_owned()).with_language("moriarty"),
        );
        let plain = render_plain(&diagnostic, "test.mori", source);
        let mut from_report = String::new();
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
            .with_width(WIDTH)
            .with_links(false)
            .render_report(&mut from_report, report.as_ref())
            .unwrap();
        assert_eq!(plain, from_report);
        assert!(plain.contains("[test.mori:1:9]"), "{plain}");
        let colored = render_colored(&report, WIDTH);
        // Color codes wrap the header payload; they are not position units.
        assert!(colored.contains("test.mori:1:9"), "{colored}");
        let label = diagnostic.labels().unwrap().next().unwrap();
        assert_eq!((label.offset(), label.len()), (11, 0));
        insta::with_settings!({
            description => format!("Code:\n\n{source}"),
            omit_expression => true,
        }, {
            insta::assert_snapshot!("named_utf8_attachment_plain", plain);
        });
    }
}
