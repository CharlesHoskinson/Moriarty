use std::fmt::{self, Write};

use owo_colors::{OwoColorize, Style, StyledList};
use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

use crate::diagnostic_chain::{DiagnosticChain, ErrorKind};
use crate::handlers::theme::*;
use crate::highlighters::{Highlighter, MietteHighlighter};
use crate::protocol::{Diagnostic, Severity};
use crate::{LabeledSpan, ReportHandler, SourceCode, SourceSpan, SpanContents};

/**
A [`ReportHandler`] that displays a given [`Report`](crate::Report) in a
quasi-graphical way, using terminal colors, unicode drawing characters, and
other such things.

This is the default reporter bundled with `miette`.

This printer can be customized by using [`new_themed()`](GraphicalReportHandler::new_themed) and handing it a
[`GraphicalTheme`] of your own creation (or using one of its own defaults!)

See [`set_hook()`](crate::set_hook) for more details on customizing your global
printer.
*/
#[derive(Debug, Clone)]
pub struct GraphicalReportHandler {
    pub(crate) links: LinkStyle,
    pub(crate) termwidth: usize,
    pub(crate) theme: GraphicalTheme,
    pub(crate) footer: Option<String>,
    pub(crate) context_lines: usize,
    pub(crate) tab_width: usize,
    pub(crate) with_cause_chain: bool,
    pub(crate) wrap_lines: bool,
    pub(crate) break_words: bool,
    pub(crate) with_primary_span_start: bool,
    pub(crate) word_separator: Option<textwrap::WordSeparator>,
    pub(crate) word_splitter: Option<textwrap::WordSplitter>,
    pub(crate) highlighter: MietteHighlighter,
    pub(crate) link_display_text: Option<String>,
    pub(crate) show_related_as_nested: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LinkStyle {
    None,
    Link,
    Text,
}
