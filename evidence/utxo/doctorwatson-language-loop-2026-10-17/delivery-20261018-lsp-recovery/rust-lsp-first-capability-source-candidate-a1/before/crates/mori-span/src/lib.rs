//! Source spans for the Moriarty toolchain.

use miette::{LabeledSpan, SourceSpan};

/// A half-open range `[start, end)` of UTF-8 byte offsets into a source file.
///
/// Sources are at most 65,536 bytes, so `u32` offsets always fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    pub const fn new(start: u32, end: u32) -> Self {
        debug_assert!(start <= end);
        Self { start, end }
    }

    /// An empty span at `offset`, such as the end-of-file position.
    pub const fn empty(offset: u32) -> Self {
        Self::new(offset, offset)
    }

    pub const fn len(self) -> u32 {
        self.end - self.start
    }

    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// The text this span covers in `source`.
    pub fn source_text(self, source: &str) -> &str {
        &source[self.start as usize..self.end as usize]
    }

    /// A diagnostic label with `text` pointing at this span.
    pub fn label(self, text: impl Into<String>) -> LabeledSpan {
        LabeledSpan::new_with_span(Some(text.into()), self)
    }

    /// Like [`Span::label`], but marks where the error is when a diagnostic
    /// has several labels. Its position is the one shown in the header.
    pub fn primary_label(self, text: impl Into<String>) -> LabeledSpan {
        LabeledSpan::new_primary_with_span(Some(text.into()), self)
    }
}

impl From<Span> for SourceSpan {
    fn from(span: Span) -> Self {
        SourceSpan::new((span.start as usize).into(), span.len() as usize)
    }
}

/// An unlabeled underline of the span.
impl From<Span> for LabeledSpan {
    fn from(span: Span) -> Self {
        LabeledSpan::underline(span)
    }
}
