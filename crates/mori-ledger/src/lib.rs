//! Source-derived local native call preparation and explicit proving APIs.
//! Preparation stays unsigned/unproven; native APIs require separately admitted
//! resources. No funding, signatures, ledger apply or network I/O.
#[cfg(test)]
extern crate self as mori_ledger;

pub mod call_artifacts;
pub mod codec;
pub mod custody;
pub mod layout;
pub mod local_deployment;
pub mod ops;
pub(crate) mod parameters;
pub mod prepare;
pub mod profile;
pub mod proving;
pub mod relation;
pub mod signed;

/// Metadata for a refused already-discovered local deployment degree.
/// This carries no parameter bytes, paths, keys or private inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DegreeRefusalDiagnostic {
    pub action: String,
    pub discovered_degree: u8,
    pub caller_ceiling: u8,
    pub pinned_supported: bool,
    pub reason: DegreeRefusalReason,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DegreeRefusalReason {
    CallerCeilingExceeded,
    UnsupportedPinnedDegree,
    CachedDegreeMismatch { cached_degree: u8 },
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("mori::ledger::{layer}: {code}")]
pub struct Error {
    pub layer: &'static str,
    pub code: &'static str,
    degree_refusal: Option<DegreeRefusalDiagnostic>,
}
impl Error {
    pub fn new(layer: &'static str, code: &'static str) -> Self {
        Self {
            layer,
            code,
            degree_refusal: None,
        }
    }
    pub fn input(code: &'static str) -> Self {
        Self::new("input", code)
    }
    pub fn with_degree_refusal(mut self, diagnostic: DegreeRefusalDiagnostic) -> Self {
        self.degree_refusal = Some(diagnostic);
        self
    }
    /// None means no discovered-degree diagnostic was attached, even at degree 0.
    pub fn degree_refusal(&self) -> Option<&DegreeRefusalDiagnostic> {
        self.degree_refusal.as_ref()
    }
}
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod error_tests {
    use super::*;

    #[test]
    fn legacy_errors_have_no_discovered_degree_context() {
        let error = Error::new("local-deployment", "exact-degree-unavailable");
        assert_eq!(
            error.to_string(),
            "mori::ledger::local-deployment: exact-degree-unavailable"
        );
        assert_eq!(error.degree_refusal(), None);
        assert_eq!(
            Error::input("source-input").to_string(),
            "mori::ledger::input: source-input"
        );
        assert_eq!(Error::input("source-input").degree_refusal(), None);
    }

    #[test]
    fn typed_refusals_preserve_identity_clone_equality_and_legacy_display() {
        let action = "freeze\n\u{1b}[31m\r\"\\\u{202e}";
        let diagnostic = DegreeRefusalDiagnostic {
            action: action.into(),
            discovered_degree: 0,
            caller_ceiling: 0,
            pinned_supported: true,
            reason: DegreeRefusalReason::CachedDegreeMismatch { cached_degree: 1 },
        };
        let error = Error::new("local-deployment", "exact-degree-unavailable")
            .with_degree_refusal(diagnostic.clone());
        assert_eq!(error.degree_refusal(), Some(&diagnostic));
        assert_eq!(error.degree_refusal().unwrap().action, action);
        assert_eq!(error, error.clone());
        assert_eq!(
            error.to_string(),
            "mori::ledger::local-deployment: exact-degree-unavailable"
        );
        assert_ne!(error, Error::new(error.layer, error.code));
        let different =
            Error::new(error.layer, error.code).with_degree_refusal(DegreeRefusalDiagnostic {
                reason: DegreeRefusalReason::CallerCeilingExceeded,
                ..diagnostic
            });
        assert_ne!(error, different);
    }
}
