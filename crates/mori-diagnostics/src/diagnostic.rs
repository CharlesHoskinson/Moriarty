use std::borrow::Cow;
use std::fmt;

use miette::{Diagnostic, LabeledSpan, Report, Severity, SourceCode};

/// A problem found in Moriarty source, with optional code, labels and help.
///
/// The payload is boxed so `Result<T, MoriDiagnostic>` stays one pointer wide.
#[derive(Debug, Clone)]
pub struct MoriDiagnostic {
    inner: Box<Inner>,
}

#[derive(Debug, Clone)]
struct Inner {
    message: Cow<'static, str>,
    severity: Severity,
    code: Option<Cow<'static, str>>,
    labels: Vec<LabeledSpan>,
    help: Option<Cow<'static, str>>,
}

const _: () = assert!(size_of::<MoriDiagnostic>() == size_of::<usize>());

impl MoriDiagnostic {
    pub fn error(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Severity::Error, message.into())
    }

    pub fn warning(message: impl Into<Cow<'static, str>>) -> Self {
        Self::new(Severity::Warning, message.into())
    }

    fn new(severity: Severity, message: Cow<'static, str>) -> Self {
        Self {
            inner: Box::new(Inner {
                message,
                severity,
                code: None,
                labels: Vec::new(),
                help: None,
            }),
        }
    }

    /// Sets the stable diagnostic code, such as `BETA_SYNTAX`.
    pub fn with_code(mut self, code: impl Into<Cow<'static, str>>) -> Self {
        self.inner.code = Some(code.into());
        self
    }

    /// Adds a labeled source span. Labels render in the order they are added.
    pub fn with_label(mut self, label: impl Into<LabeledSpan>) -> Self {
        self.inner.labels.push(label.into());
        self
    }

    pub fn with_help(mut self, help: impl Into<Cow<'static, str>>) -> Self {
        self.inner.help = Some(help.into());
        self
    }

    /// Attaches the source the labels point into, producing a renderable report.
    pub fn with_source_code(self, source: impl SourceCode + 'static) -> Report {
        Report::new(self).with_source_code(source)
    }
}

impl fmt::Display for MoriDiagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.inner.message)
    }
}

impl std::error::Error for MoriDiagnostic {}

impl Diagnostic for MoriDiagnostic {
    fn code<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.inner
            .code
            .as_ref()
            .map(|code| Box::new(code) as Box<dyn fmt::Display>)
    }

    fn severity(&self) -> Option<Severity> {
        Some(self.inner.severity)
    }

    fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
        self.inner
            .help
            .as_ref()
            .map(|help| Box::new(help) as Box<dyn fmt::Display>)
    }

    fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan> + '_>> {
        if self.inner.labels.is_empty() {
            None
        } else {
            Some(Box::new(self.inner.labels.iter().cloned()))
        }
    }
}
