//! Normal public diagnostic attachment regressions; no compile/evaluation commands.
//! Expected headers and UTF-8 byte spans were frozen before this test source.
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use mori_diagnostics::{Diagnostic, MoriDiagnostic, render::render_plain};
use serde_json::Value;

const CORE: &str = "language \"moriarty/1\";\n/* café */ agreement ;\n";
const BETA: &str = "/* 😀 */ @\n";
const FORMAT: &str = "// note\r\n/* café */ @\r\n";
const FILE: &str = "diagnostic.mori";

struct SourceDirectory(PathBuf);

impl SourceDirectory {
    fn new(source: &str) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "mori-diagnostic-columns-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed),
        ));
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join(FILE), source).unwrap();
        Self(path)
    }

    fn invoke(&self, color: &str, command: &str, json: bool) -> Output {
        let mut cli = Command::new(env!("CARGO_BIN_EXE_mori"));
        cli.current_dir(&self.0)
            .args(["--quiet", "--color", color, command, FILE])
            .stdin(Stdio::null());
        if command == "format" {
            cli.arg("--check");
        }
        if json {
            cli.arg("--json");
        }
        cli.output().unwrap()
    }
}

impl Drop for SourceDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn assert_public_headers(
    directory: &SourceDirectory,
    source: &str,
    command: &str,
    plain: &str,
    header: &str,
    summary: &str,
) {
    assert!(plain.contains(&format!("[{header}]")), "{plain}");
    for color in ["never", "always"] {
        let output = directory.invoke(color, command, false);
        assert_eq!(output.status.code(), Some(1), "{output:?}");
        assert!(output.stdout.is_empty(), "{output:?}");
        let stderr = std::str::from_utf8(&output.stderr).unwrap();
        // Styles may surround the payload, but cannot change its position units.
        assert!(stderr.contains(header), "{stderr}");
        if color == "never" {
            assert_eq!(stderr, format!("{plain}\n{summary}"));
        } else {
            assert!(stderr.contains("\u{1b}["), "{stderr}");
        }
        assert_eq!(
            std::fs::read(directory.0.join(FILE)).unwrap(),
            source.as_bytes(),
        );
    }
}

fn assert_beta_label(diagnostic: &MoriDiagnostic, start: usize, end: usize) {
    assert_eq!(
        diagnostic.code().unwrap().to_string(),
        "mori::lex::unexpected_character",
    );
    assert_eq!(
        diagnostic.to_string(),
        "I do not recognize the character `@`.",
    );
    let labels: Vec<_> = diagnostic
        .labels()
        .unwrap()
        .map(|label| (label.offset(), label.len()))
        .collect();
    assert_eq!(labels, [(start, end - start)]);
}

#[test]
fn core_check_multibyte_header_matches_plain_and_keeps_byte_offset() {
    let directory = SourceDirectory::new(CORE);
    let error = mori_core::parse(CORE).unwrap_err();
    assert_eq!(error.code, "mori::core::syntax");
    assert_eq!(error.message, "Expected a name.");
    assert_eq!(error.offset, Some(45));
    let diagnostic = MoriDiagnostic::error(error.message.clone())
        .with_code(error.code.clone())
        .with_label(mori_span::Span::empty(45).primary_label("here"));
    let plain = render_plain(&diagnostic, FILE, CORE);
    assert_public_headers(
        &directory,
        CORE,
        "check",
        &plain,
        "diagnostic.mori:2:22",
        "",
    );
    insta::with_settings!({
        description => format!("Code:\n\n{CORE}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("core_syntax_multibyte_scalar", plain);
    });
    let output = directory.invoke("never", "check", true);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["diagnostics"][0]["offset"], 45);
    assert_eq!(report["diagnostics"][0]["code"], "mori::core::syntax");
    assert_eq!(std::fs::read(directory.0.join(FILE)).unwrap(), CORE.as_bytes());
}

#[test]
fn beta_check_astral_header_matches_plain_and_keeps_byte_label() {
    let directory = SourceDirectory::new(BETA);
    let diagnostic = match mori_parser::parse(BETA) {
        Err(diagnostic) => diagnostic,
        Ok(_) => panic!("the frozen normal source must have its lexical error"),
    };
    assert_beta_label(&diagnostic, 11, 12);
    let plain = render_plain(&diagnostic, FILE, BETA);
    assert_public_headers(
        &directory,
        BETA,
        "check",
        &plain,
        "diagnostic.mori:1:9",
        "error: could not check `diagnostic.mori` due to 1 previous error\n",
    );
    insta::with_settings!({
        description => format!("Code:\n\n{BETA}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("beta_lex_astral_scalar", plain);
    });
    let output = directory.invoke("never", "check", true);
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report["diagnostics"][0]["labels"][0]["span"],
        serde_json::json!({"start":11,"end":12}),
    );
    assert_eq!(
        report["diagnostics"][0]["code"],
        "mori::lex::unexpected_character",
    );
    assert_eq!(std::fs::read(directory.0.join(FILE)).unwrap(), BETA.as_bytes());
}

#[test]
fn format_crlf_multibyte_header_matches_plain_without_writing() {
    let directory = SourceDirectory::new(FORMAT);
    let diagnostic = mori_fmt::format(FORMAT).unwrap_err();
    assert_beta_label(&diagnostic, 21, 22);
    let plain = render_plain(&diagnostic, FILE, FORMAT);
    assert_public_headers(
        &directory,
        FORMAT,
        "format",
        &plain,
        "diagnostic.mori:2:12",
        "error: could not format 1 file due to errors\n",
    );
    insta::with_settings!({
        description => format!("Code:\n\n{FORMAT}"),
        omit_expression => true,
    }, {
        insta::assert_snapshot!("format_lex_crlf_scalar", plain);
    });
}
