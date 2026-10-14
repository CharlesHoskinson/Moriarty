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
pub mod static_compile;

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

/// Metadata from one bounded tagged key export. Counts include its header;
/// an attempted minimum is only the accepted prefix plus the first refused
/// chunk, never a measurement of the complete encoded key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyExportDiagnostic {
    pub action: String,
    pub key_kind: KeyExportKind,
    pub discovered_degree: u8,
    pub byte_cap: usize,
    pub accepted_bytes: usize,
    pub failed_write_bytes: Option<usize>,
    pub attempted_minimum: Option<usize>,
    pub reason: KeyExportFailureReason,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyExportKind {
    Prover,
    Verifier,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyExportFailureReason {
    ByteCapExceeded,
    ByteCountOverflow,
    Serialization,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("mori::ledger::{layer}: {code}")]
pub struct Error {
    pub layer: &'static str,
    pub code: &'static str,
    degree_refusal: Option<DegreeRefusalDiagnostic>,
    key_export: Option<KeyExportDiagnostic>,
}
impl Error {
    pub fn new(layer: &'static str, code: &'static str) -> Self {
        Self {
            layer,
            code,
            degree_refusal: None,
            key_export: None,
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
    pub fn with_key_export(mut self, diagnostic: KeyExportDiagnostic) -> Self {
        self.key_export = Some(diagnostic);
        self
    }
    /// None means no export diagnostic was attached; degree/count zero are values.
    pub fn key_export(&self) -> Option<&KeyExportDiagnostic> {
        self.key_export.as_ref()
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
        assert_eq!(error.key_export(), None);
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

    #[test]
    fn key_export_metadata_preserves_identity_zero_clone_and_equality() {
        for reason in [
            KeyExportFailureReason::ByteCapExceeded,
            KeyExportFailureReason::ByteCountOverflow,
            KeyExportFailureReason::Serialization,
        ] {
            let diagnostic = KeyExportDiagnostic {
                action: "settle\n\u{1b}\u{202e}".into(),
                key_kind: KeyExportKind::Prover,
                discovered_degree: 0,
                byte_cap: 0,
                accepted_bytes: 0,
                failed_write_bytes: None,
                attempted_minimum: None,
                reason,
            };
            let error = Error::new("native-proving", "generated-key-size-or-encode")
                .with_key_export(diagnostic.clone());
            assert_eq!(error.key_export(), Some(&diagnostic));
            assert_eq!(error, error.clone());
            assert_eq!(error.degree_refusal(), None);
            assert_ne!(error, Error::new(error.layer, error.code));
            assert_eq!(
                error.to_string(),
                "mori::ledger::native-proving: generated-key-size-or-encode"
            );
        }
    }
}
