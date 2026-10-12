use crate::error::CliError;
use serde::{Serialize, de::DeserializeOwned};
use std::path::Path;

pub fn read_source(path: &Path) -> super::Result<String> {
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut source = String::new();
    file.take(1_048_577)
        .read_to_string(&mut source)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if source.len() > 1_048_576 {
        return Err(operation("input file exceeds 1 MiB"));
    }
    Ok(source)
}
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}
pub fn ledger_operation(error: mori_ledger::Error) -> mori_diagnostics::Report {
    use mori_ledger::DegreeRefusalReason;
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
    CliError::Operation {
        code: "mori::command".into(),
        message,
    }
    .into()
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
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
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
