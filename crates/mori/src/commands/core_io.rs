use crate::error::CliError;
use serde::de::DeserializeOwned;
use std::path::Path;

pub use super::source_io::{operation, print, read_source};

#[cfg(test)]
mod key_export_diagnostic_tests {
    use super::*;
    use mori_ledger::{Error, KeyExportDiagnostic, KeyExportFailureReason, KeyExportKind};

    fn refusal(action: &str, key_kind: KeyExportKind, reason: KeyExportFailureReason) -> Error {
        let cap_refusal = reason == KeyExportFailureReason::ByteCapExceeded;
        Error::new("native-proving", "generated-key-size-or-encode").with_key_export(
            KeyExportDiagnostic {
                action: action.into(),
                key_kind,
                discovered_degree: 0,
                byte_cap: 3,
                accepted_bytes: 2,
                failed_write_bytes: (reason != KeyExportFailureReason::Serialization).then_some(2),
                attempted_minimum: cap_refusal.then_some(4),
                reason,
            },
        )
    }
    #[test]
    fn key_export_cli_preserves_category_and_discriminates_reasons() {
        for kind in [KeyExportKind::Prover, KeyExportKind::Verifier] {
            for (reason, label) in [
                (KeyExportFailureReason::ByteCapExceeded, "byte-cap-exceeded"),
                (
                    KeyExportFailureReason::ByteCountOverflow,
                    "byte-count-overflow",
                ),
                (
                    KeyExportFailureReason::Serialization,
                    "serialization-failure",
                ),
            ] {
                let report = ledger_operation(refusal("settle", kind, reason)).to_string();
                let kind = if kind == KeyExportKind::Prover {
                    "prover"
                } else {
                    "verifier"
                };
                assert!(report.starts_with("mori::command: mori::ledger::native-proving: generated-key-size-or-encode; "), "{report}");
                assert!(report.contains(&format!("action=\"settle\" key-kind={kind} discovered-degree=0 byte-cap=3 accepted-bytes=2 reason={label}")), "{report}");
                assert_eq!(
                    report.contains("failed-write-bytes=2"),
                    reason != KeyExportFailureReason::Serialization
                );
                assert_eq!(
                    report.contains("attempted-minimum=4"),
                    reason == KeyExportFailureReason::ByteCapExceeded
                );
            }
        }
        let legacy = Error::new("native-proving", "sdk-keygen");
        assert_eq!(
            ledger_operation(legacy.clone()).to_string(),
            operation(legacy).to_string()
        );
    }
    #[test]
    fn key_export_cli_action_is_ascii_json_escaped_once_and_round_trips() {
        let action = "settle\"\\\nforged\r\u{1b}[31m\u{7f}\u{85}\u{202e}\u{1f9ea}";
        let error = refusal(
            action,
            KeyExportKind::Verifier,
            KeyExportFailureReason::ByteCapExceeded,
        );
        assert_eq!(error.key_export().unwrap().action, action);
        let report = ledger_operation(error).to_string();
        let encoded = report
            .split_once("; action=")
            .unwrap()
            .1
            .split_once(" key-kind=")
            .unwrap()
            .0;
        assert_eq!(serde_json::from_str::<String>(encoded).unwrap(), action);
        assert!(report.is_ascii(), "{report}");
        assert!(report.chars().all(|c| !c.is_control()), "{report}");
        assert!(encoded.contains(r"\u0085"));
        assert!(encoded.contains(r"\u202e"));
        assert!(encoded.contains(r"\ud83e\uddea"));
        assert_eq!(
            encoded,
            r#""settle\"\\\nforged\r\u001b[31m\u007f\u0085\u202e\ud83e\uddea""#
        );
    }
    #[test]
    fn key_export_quiet_shell_reports_safe_metadata_on_actual_stderr() {
        const CHILD: &str = "MORI_KEY_EXPORT_DIAGNOSTIC_REPORT_CHILD";
        if std::env::var_os(CHILD).is_some() {
            let report = ledger_operation(refusal(
                "settle\n\u{1b}\u{202e}",
                KeyExportKind::Prover,
                KeyExportFailureReason::ByteCapExceeded,
            ));
            let mut shell = crate::shell::Shell::new(
                crate::shell::Verbosity::Quiet,
                anstream::ColorChoice::Never,
            );
            shell.report(&report);
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "commands::core_io::key_export_diagnostic_tests::key_export_quiet_shell_reports_safe_metadata_on_actual_stderr", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        // The trusted renderer adds Unicode box drawing and may wrap a field
        // across lines. Inspect its joined content, plus raw hostile controls.
        let content: String = stderr
            .lines()
            .map(|line| {
                line.strip_prefix("  │ ")
                    .or_else(|| line.strip_prefix("  × "))
                    .unwrap_or(line)
            })
            .collect();
        for value in [
            "generated-key-size-or-encode",
            "key-kind=prover",
            "discovered-degree=0",
            "byte-cap=3",
            "accepted-bytes=2",
            "byte-cap-exceeded",
            "failed-write-bytes=2",
            "attempted-minimum=4",
            r#"action="settle\n\u001b\u202e""#,
        ] {
            assert!(content.contains(value), "{stderr}");
        }
        assert!(!stderr.contains(['\u{1b}', '\u{85}', '\u{202e}']));
        assert!(!stderr.contains("key-bytes"));
    }
}
pub fn ledger_operation(error: mori_ledger::Error) -> mori_diagnostics::Report {
    use mori_ledger::{DegreeRefusalReason, KeyExportFailureReason, KeyExportKind};
    let mut message = crate::display::text(&error.to_string());
    if let Some(diagnostic) = error.degree_refusal() {
        let reason = match diagnostic.reason {
            DegreeRefusalReason::CallerCeilingExceeded => "caller-ceiling-exceeded",
            DegreeRefusalReason::UnsupportedPinnedDegree => "unsupported-pinned-degree",
            DegreeRefusalReason::CachedDegreeMismatch { .. } => "cached-degree-mismatch",
        };
        // Quote the exact source identity once; operation() would escape this
        // already-rendered identity again. All remaining context is metadata.
        message.push_str(&format!(
            "; action={} discovered-degree={} caller-ceiling={} pinned-supported={} reason={reason}",
            review_identity(&diagnostic.action),
            diagnostic.discovered_degree,
            diagnostic.caller_ceiling,
            diagnostic.pinned_supported,
        ));
        if let DegreeRefusalReason::CachedDegreeMismatch { cached_degree } = diagnostic.reason {
            message.push_str(&format!(" cached-degree={cached_degree}"));
        }
    }
    if let Some(diagnostic) = error.key_export() {
        let kind = match diagnostic.key_kind {
            KeyExportKind::Prover => "prover",
            KeyExportKind::Verifier => "verifier",
        };
        let reason = match diagnostic.reason {
            KeyExportFailureReason::ByteCapExceeded => "byte-cap-exceeded",
            KeyExportFailureReason::ByteCountOverflow => "byte-count-overflow",
            KeyExportFailureReason::Serialization => "serialization-failure",
        };
        message.push_str(&format!(
            "; action={} key-kind={kind} discovered-degree={} byte-cap={} accepted-bytes={} reason={reason}",
            key_export_identity(&diagnostic.action),
            diagnostic.discovered_degree,
            diagnostic.byte_cap,
            diagnostic.accepted_bytes,
        ));
        if let Some(bytes) = diagnostic.failed_write_bytes {
            message.push_str(&format!(" failed-write-bytes={bytes}"));
        }
        if let Some(minimum) = diagnostic.attempted_minimum {
            message.push_str(&format!(" attempted-minimum={minimum}"));
        }
    }
    CliError::Operation {
        code: "mori::command".into(),
        message,
    }
    .into()
}
/// JSON escaping quotes/controls once, with UTF-16 escapes for non-ASCII
/// identities. This remains valid JSON even for supplementary Unicode scalars.
fn key_export_identity(value: &str) -> String {
    let json = serde_json::to_string(value).expect("serializing a string cannot fail");
    let mut escaped = String::with_capacity(json.len());
    for character in json.chars() {
        if character.is_ascii() && !character.is_control() {
            escaped.push(character);
        } else {
            let mut units = [0; 2];
            for unit in character.encode_utf16(&mut units) {
                use std::fmt::Write;
                write!(escaped, "\\u{unit:04x}").expect("writing to a String cannot fail");
            }
        }
    }
    escaped
}
pub fn read_json<T: DeserializeOwned>(path: &Path) -> super::Result<T> {
    let source = read_source(path)?;
    let value = mori_core::json::parse(&source).map_err(operation)?;
    serde_json::from_value(value).map_err(operation)
}
pub fn program(path: &Path) -> super::Result<(String, mori_core::Program)> {
    let source = read_source(path)?;
    let program = mori_core::parse(&source).map_err(operation)?;
    Ok((source, program))
}
/// Trusted signed Target, separately supplied expected authority and logical time.
pub fn signed_target(path: &Path) -> super::Result<mori_backend::Target> {
    let target: mori_backend::Target = read_json(path)?;
    if target.signed_policy.is_none() {
        return Err(operation(
            "the signed profile requires a trusted Target with signed_policy",
        ));
    }
    Ok(target)
}
pub fn authority(path: &Path) -> super::Result<mori_backend::authorization::ExpectedAuthority> {
    mori_backend::authorization::ExpectedAuthority::from_json(&read_source(path)?)
        .map_err(operation)
}
pub fn signed_intent(path: &Path) -> super::Result<mori_backend::authorization::SignedIntent> {
    mori_backend::authorization::SignedIntent::from_json(&read_source(path)?).map_err(operation)
}
/// Quote display identities without giving source text terminal control.
pub fn review_identity(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .chars()
            .flat_map(char::escape_default)
            .collect::<String>()
    )
}

#[cfg(test)]
mod degree_diagnostic_tests {
    use super::*;
    use mori_ledger::{DegreeRefusalDiagnostic, DegreeRefusalReason, Error};

    fn refusal(action: &str, reason: DegreeRefusalReason) -> Error {
        let (discovered_degree, caller_ceiling, pinned_supported) = match reason {
            DegreeRefusalReason::CallerCeilingExceeded => (14, 13, true),
            DegreeRefusalReason::UnsupportedPinnedDegree => (20, 25, false),
            DegreeRefusalReason::CachedDegreeMismatch { .. } => (13, 13, true),
        };
        Error::new("local-deployment", "exact-degree-unavailable").with_degree_refusal(
            DegreeRefusalDiagnostic {
                action: action.into(),
                discovered_degree,
                caller_ceiling,
                pinned_supported,
                reason,
            },
        )
    }

    #[test]
    fn ledger_conversion_preserves_category_and_complete_permitted_context() {
        for (reason, context) in [
            (
                DegreeRefusalReason::CallerCeilingExceeded,
                "discovered-degree=14 caller-ceiling=13 pinned-supported=true reason=caller-ceiling-exceeded",
            ),
            (
                DegreeRefusalReason::UnsupportedPinnedDegree,
                "discovered-degree=20 caller-ceiling=25 pinned-supported=false reason=unsupported-pinned-degree",
            ),
            (
                DegreeRefusalReason::CachedDegreeMismatch { cached_degree: 12 },
                "discovered-degree=13 caller-ceiling=13 pinned-supported=true reason=cached-degree-mismatch cached-degree=12",
            ),
        ] {
            let report = ledger_operation(refusal("freeze", reason));
            assert_eq!(
                report.to_string(),
                format!(
                    "mori::command: mori::ledger::local-deployment: exact-degree-unavailable; action=\"freeze\" {context}"
                )
            );
        }
        let legacy = Error::new("local-deployment", "selected-action");
        assert_eq!(
            ledger_operation(legacy.clone()).to_string(),
            operation(legacy).to_string()
        );
    }

    #[test]
    fn hostile_action_is_exact_in_api_and_escaped_once_in_command_report() {
        let action = "freeze\u{1b}[31m\nforged\r\"\\\u{85}\u{202e}";
        let error = refusal(action, DegreeRefusalReason::CallerCeilingExceeded);
        assert_eq!(error.degree_refusal().unwrap().action, action);
        let message = ledger_operation(error).to_string();
        assert!(
            message.contains(r#"action="freeze\u{1b}[31m\nforged\r\"\\\u{85}\u{202e}""#),
            "{message}"
        );
        assert!(message.chars().all(|c| !c.is_control()));
        assert_eq!(
            message,
            r#"mori::command: mori::ledger::local-deployment: exact-degree-unavailable; action="freeze\u{1b}[31m\nforged\r\"\\\u{85}\u{202e}" discovered-degree=14 caller-ceiling=13 pinned-supported=true reason=caller-ceiling-exceeded"#
        );
    }

    #[test]
    fn quiet_shell_reports_constructed_degree_refusal_on_stderr() {
        const CHILD: &str = "MORI_DEGREE_DIAGNOSTIC_REPORT_CHILD";
        if std::env::var_os(CHILD).is_some() {
            let report = ledger_operation(refusal(
                "freeze",
                DegreeRefusalReason::CallerCeilingExceeded,
            ));
            let mut shell = crate::shell::Shell::new(
                crate::shell::Verbosity::Quiet,
                anstream::ColorChoice::Never,
            );
            shell.report(&report);
            return;
        }
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "commands::core_io::degree_diagnostic_tests::quiet_shell_reports_constructed_degree_refusal_on_stderr", "--nocapture"])
            .env(CHILD, "1").output().unwrap();
        assert!(output.status.success());
        let stderr = String::from_utf8(output.stderr).unwrap();
        for value in [
            "exact-degree-unavailable",
            "action=\"freeze\"",
            "discovered-degree=14",
            "caller-ceiling=13",
            "caller-ceiling-exceeded",
        ] {
            assert!(stderr.contains(value), "{stderr}");
        }
        assert!(!stderr.contains('\u{1b}'));
    }
}
