SECURITY TEST REPAIR AUTHOR (Gemini4ArgonHIGH designatedOpusoutagefallback, lastsubstantiveOpus4.8ratefailure06:54:51UTC). No review/admission/execution requested. Return ONEcomplete6fieldobject token doctorwatson-native-availability-gutter-source-author-20261018-a1-whole-source-patch with minimal valid unifiedsourcePatch --- a/native_feature_boundary.rs +++ b/native_feature_boundary.rs, completecatalogUNCHANGEDunless genuinelynecessary (preserves27exactcases),findings,limitations. No commands/tools/build/Mori/evaluator/models/SDK/keys/SRS/wallet/private/config/cache/providerstreams/network. Primarytestsourceandactualsource renderer+failurelogs INLINE.

Rootcause now OBSERVED: cold no-default source c24bfc... tree7a896 compiledsuccess52.22s, actualtarget2testfunctionsFAILED because normalize_ws onlysplit_whitespace; miette GraphicalTheme::unicode inserts a `│` continuation gutter at line start, so normalization leaves decorativetokensinsideexpected message. Observed firstnativecompile diag has "... no Midnight │ support..." aftercurrentnormalize; semanticintentmissingat has "requires --target │ and --at together". Allactualexpectedsemantictext mustremain EXACT independentliteral fixedmessages, code, exits, emptyout, caseargv, sentineltreechecks, cfg gate and all27cases/44plannedcalls unchanged. Product rendering implementation remainsunchanged. Keepwholefirstfailure andconsumedattempt no rerun/reuse. TestbodyRustcompiled; don't invent typeerrors.

Repairtestcomparison of wrappeddiagnostics with source-grounded line-oriented removal of ONLY documenteddecoration: oncontinuationlines removeleadingindent and ONE leadingUnicode│ gutter, thenwhitespacejoin; doNOTgloballydeleteevery│anywhere, strip arbitrarysemanticcontent, broadmatch/fragments, weakenexpectedphrases, ignoreexits or filechecks, or changeproductionmessages/rendering. Preserveliteralcontentandinternalguttercharacters (ifpresentinsidealine semanticpayload) ratherthan blanketreplace. Ensure robustforleadingnewline/unwrappedstderr, continuationlinemargins. A small literalnormalization regression insideexistingtestfunction or onepuretestonlyifnecessary may verify gutter removal whilekeepinginternal│ distinct; avoid extraCLIcalls/wholehelpsnapshots. Catalogueoriginalcaseidentitiesremain27andplannedchildcalls44; anynewpurehelpertestmustexplicitcountnotinventruntimepass. Sourcechangesonlytesthelper/comment/assertionslogicalfix, standardRust compilesplausible no expectedoutputgeneration fromMori. Source-generatednormalizationruleis basedactualrenderer andindependentfixedsemanticmessage, not acceptedfinancialoraclemutation. Finishwithsourceonly, noclaimsall44callsran orpassed (actualtestsabortedearly); freshnonauthorreviews/newconcreteattempt needed.

### native_feature_boundary.rs
#![cfg(not(feature = "native"))]

use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const NATIVE_CODE: &str = "mori::command::native-unavailable";
const NATIVE_MSG: &str =
    "This mori build has no Midnight support. Rebuild with the native feature enabled.";

static COUNTER: AtomicU64 = AtomicU64::new(0);

struct TempRoot(PathBuf);

impl TempRoot {
    fn new(label: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = env::temp_dir().join(format!(
            "mori_no_native_{}_{}_{}_{}",
            label,
            std::process::id(),
            nanos,
            seq
        ));
        fs::create_dir(&dir).expect("create temp root");
        Self(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn snapshot_tree(root: &Path) -> BTreeMap<PathBuf, Option<Vec<u8>>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<PathBuf, Option<Vec<u8>>>) {
        let mut entries: Vec<_> = fs::read_dir(dir)
            .expect("read_dir")
            .map(|e| e.expect("dir entry"))
            .collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let path = entry.path();
            let rel = path
                .strip_prefix(base)
                .expect("relative path")
                .to_path_buf();
            let ftype = entry.file_type().expect("file_type");
            if ftype.is_dir() {
                out.insert(rel, None);
                walk(base, &path, out);
            } else {
                out.insert(rel, Some(fs::read(&path).expect("read sentinel file")));
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn normalize_ws(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn run_mori(cwd: &Path, args: &[&str]) -> (Option<i32>, Vec<u8>, String) {
    let before = snapshot_tree(cwd);
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(cwd)
        .args(["--color", "never"])
        .args(args)
        .output()
        .expect("spawn mori subprocess");
    let after = snapshot_tree(cwd);
    assert_eq!(
        before, after,
        "filesystem tree mutated by invocation: {args:?}"
    );
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (output.status.code(), output.stdout, stderr)
}

const CALL_ARGS: &[&str] = &[
    "agreement.mori",
    "--deployment",
    "deploy.json",
    "--predecessor",
    "pred.bin",
    "--parameters",
    "params.bin",
    "--request",
    "req.json",
    "--action",
    "settle",
    "--params-dir",
    "srs_dir",
    "--work-limits",
    "limits.json",
];

const SIGNED_CALL_ARGS: &[&str] = &[
    "agreement.mori",
    "--signer-policy",
    "policy.json",
    "--deployment",
    "deploy.json",
    "--predecessor",
    "pred.bin",
    "--parameters",
    "params.bin",
    "--request",
    "req.json",
    "--action",
    "settle",
    "--params-dir",
    "srs_dir",
    "--work-limits",
    "limits.json",
];

const ACTION_ARGS: &[&str] = &[
    "agreement.mori",
    "--action",
    "settle",
    "--scenario",
    "scenario.json",
];

#[test]
fn native_routes_report_unavailable_before_io() {
    let routes: Vec<(&str, Vec<&str>)> = vec![
        (
            "compile",
            vec![
                "compile",
                "agreement.mori",
                "--target",
                "target.json",
                "--out",
                "out_dir",
            ],
        ),
        (
            "compile-call",
            vec![
                "compile-call",
                "agreement.mori",
                "--action",
                "settle",
                "--out",
                "out_dir",
            ],
        ),
        (
            "prepare-call",
            vec![
                "prepare-call",
                "agreement.mori",
                "--action",
                "settle",
                "--deployment",
                "deploy.json",
                "--predecessor",
                "pred.bin",
                "--parameters",
                "params.bin",
                "--request",
                "req.json",
                "--out",
                "out_dir",
            ],
        ),
        (
            "keygen-call",
            vec![
                "keygen-call",
                "agreement.mori",
                "--seed-deployment",
                "deploy.json",
                "--seed-state",
                "state.bin",
                "--parameters",
                "params.bin",
                "--action",
                "settle",
                "--params-dir",
                "srs_dir",
                "--work-limits",
                "limits.json",
                "--out",
                "out_dir",
            ],
        ),
        ("prove-call", {
            let mut v = vec!["prove-call"];
            v.extend_from_slice(CALL_ARGS);
            v.extend_from_slice(&["--keyset", "keyset.bin", "--out", "out_dir"]);
            v
        }),
        ("verify-call", {
            let mut v = vec!["verify-call"];
            v.extend_from_slice(CALL_ARGS);
            v.extend_from_slice(&["--public-keyset", "pub_keyset.bin", "--proof", "proof.bin"]);
            v
        }),
        (
            "keygen-signed-call",
            vec![
                "keygen-signed-call",
                "agreement.mori",
                "--signer-policy",
                "policy.json",
                "--seed-deployment",
                "deploy.json",
                "--seed-state",
                "state.bin",
                "--parameters",
                "params.bin",
                "--action",
                "settle",
                "--params-dir",
                "srs_dir",
                "--work-limits",
                "limits.json",
                "--out",
                "out_dir",
            ],
        ),
        ("prepare-signed-call", {
            let mut v = vec!["prepare-signed-call"];
            v.extend_from_slice(SIGNED_CALL_ARGS);
            v.extend_from_slice(&["--public-keyset", "pub_keyset.bin", "--out", "out_dir"]);
            v
        }),
        ("prove-signed-call", {
            let mut v = vec!["prove-signed-call"];
            v.extend_from_slice(SIGNED_CALL_ARGS);
            v.extend_from_slice(&[
                "--keyset",
                "keyset.bin",
                "--signed-attachment",
                "attach.json",
                "--out",
                "out_dir",
            ]);
            v
        }),
        ("verify-signed-call", {
            let mut v = vec!["verify-signed-call"];
            v.extend_from_slice(SIGNED_CALL_ARGS);
            v.extend_from_slice(&[
                "--public-keyset",
                "pub_keyset.bin",
                "--proof",
                "proof.bin",
                "--signed-attachment",
                "attach.json",
            ]);
            v
        }),
        ("prove", {
            let mut v = vec!["prove"];
            v.extend_from_slice(ACTION_ARGS);
            v.extend_from_slice(&[
                "--target",
                "target.json",
                "--params-dir",
                "srs_dir",
                "--out",
                "out.proof",
            ]);
            v
        }),
        ("verify", {
            let mut v = vec!["verify"];
            v.extend_from_slice(ACTION_ARGS);
            v.extend_from_slice(&[
                "--target",
                "target.json",
                "--params-dir",
                "srs_dir",
                "--proof",
                "out.proof",
            ]);
            v
        }),
        ("replay", {
            let mut v = vec!["replay"];
            v.extend_from_slice(ACTION_ARGS);
            v.extend_from_slice(&["--target", "target.json", "--record", "record.json"]);
            v
        }),
        (
            "inspect-target",
            vec!["inspect", "agreement.mori", "--target", "target.json"],
        ),
        ("simulate-trace-target", {
            let mut v = vec!["simulate"];
            v.extend_from_slice(ACTION_ARGS);
            v.extend_from_slice(&["--trace", "--target", "target.json"]);
            v
        }),
        ("intent-signed-profile", {
            let mut v = vec!["intent"];
            v.extend_from_slice(ACTION_ARGS);
            v.extend_from_slice(&[
                "--authority",
                "authority.json",
                "--profile",
                "moriarty-jubjub-owner-intent/1",
                "--target",
                "target.json",
                "--at",
                "0",
            ]);
            v
        }),
        ("verify-intent-signed-profile", {
            let mut v = vec!["verify-intent"];
            v.extend_from_slice(ACTION_ARGS);
            v.extend_from_slice(&[
                "--signature",
                "sig.json",
                "--authority",
                "authority.json",
                "--at",
                "0",
                "--profile",
                "moriarty-jubjub-owner-intent/1",
                "--target",
                "target.json",
            ]);
            v
        }),
    ];

    let temp = TempRoot::new("routes");

    for (phase, populate) in [("missing-inputs", false), ("populated-sentinels", true)] {
        if populate {
            fs::write(
                temp.path().join("agreement.mori"),
                r#"language "moriarty/2";
agreement TwoDeliveries {
  state phase: UInt = 0;
  asset UNIT = { network: "local", issuer: "independent-staged-test-oracle", reference: "UNIT", scale: 0 };
  action deliver() {
    transfer(UNIT, "alice", "bob", 7);
    next phase = 1;
    await approval(ok: Bool) when ok;
    transfer(UNIT, "alice", "bob", 3);
    await receipt(seen: Bool) when seen;
    next phase = 2;
    emit "done"(phase);
  }
  action declare() { liability("duty", UNIT, "alice", "bob", 0); }
}
"#,
            )
            .expect("write source sentinel");
            fs::create_dir_all(temp.path().join("srs_dir")).expect("create fake srs_dir");
            for name in [
                "target.json",
                "deploy.json",
                "pred.bin",
                "params.bin",
                "req.json",
                "state.bin",
                "limits.json",
                "keyset.bin",
                "pub_keyset.bin",
                "proof.bin",
                "policy.json",
                "attach.json",
                "scenario.json",
                "out.proof",
                "record.json",
                "authority.json",
                "sig.json",
            ] {
                fs::write(temp.path().join(name), b"not-valid-sentinel-payload")
                    .expect("write sentinel");
            }
        }

        for (label, argv) in &routes {
            let (code, stdout, stderr) = run_mori(temp.path(), argv);
            let norm = normalize_ws(&stderr);
            assert_eq!(
                code,
                Some(1),
                "{phase}/{label}: expected exit 1, stderr: {stderr}"
            );
            assert!(stdout.is_empty(), "{phase}/{label}: expected empty stdout");
            assert!(
                stderr.contains(NATIVE_CODE),
                "{phase}/{label}: missing {NATIVE_CODE} in {stderr}"
            );
            assert!(
                norm.contains(NATIVE_MSG),
                "{phase}/{label}: missing message in {norm}"
            );
        }
    }
}

#[test]
fn arg_and_clap_precedence_without_native_diagnostic() {
    let temp = TempRoot::new("precedence");

    let clap_cases: &[(&str, &[&str])] = &[
        (
            "compile-missing-out",
            &["compile", "agreement.mori", "--target", "target.json"],
        ),
        (
            "prove-missing-required",
            &["prove", "agreement.mori", "--action", "settle"],
        ),
        (
            "simulate-trace-conflicts-logical-cap",
            &[
                "simulate",
                "agreement.mori",
                "--action",
                "settle",
                "--scenario",
                "scenario.json",
                "--trace",
                "--logical-cap",
                "10",
            ],
        ),
        (
            "simulate-resume-conflicts-target",
            &[
                "simulate",
                "agreement.mori",
                "--resume",
                "cont.json",
                "--event",
                "ev.json",
                "--target",
                "target.json",
            ],
        ),
        (
            "verify-intent-cap-conflicts-profile",
            &[
                "verify-intent",
                "agreement.mori",
                "--action",
                "settle",
                "--scenario",
                "scenario.json",
                "--signature",
                "sig.json",
                "--authority",
                "authority.json",
                "--at",
                "0",
                "--profile",
                "moriarty-jubjub-owner-intent/1",
                "--target",
                "target.json",
                "--logical-cap",
                "10",
            ],
        ),
        (
            "global-quiet-verbose-conflict",
            &[
                "--quiet",
                "--verbose",
                "compile",
                "agreement.mori",
                "--target",
                "target.json",
                "--out",
                "out_dir",
            ],
        ),
    ];

    for (label, argv) in clap_cases {
        let (code, stdout, stderr) = run_mori(temp.path(), argv);
        assert_eq!(code, Some(2), "{label}: expected clap exit 2, got {stderr}");
        assert!(stdout.is_empty(), "{label}: expected empty stdout");
        assert!(!stderr.is_empty(), "{label}: expected clap diagnostic");
        assert!(
            !stderr.contains(NATIVE_CODE),
            "{label}: unexpected {NATIVE_CODE} in {stderr}"
        );
    }

    let semantic_cases: &[(&str, &[&str], &str)] = &[
        (
            "simulate-trace-missing-target",
            &[
                "simulate",
                "agreement.mori",
                "--action",
                "settle",
                "--scenario",
                "scenario.json",
                "--trace",
            ],
            "--trace requires --target",
        ),
        (
            "intent-signed-profile-missing-at",
            &[
                "intent",
                "agreement.mori",
                "--action",
                "settle",
                "--scenario",
                "scenario.json",
                "--authority",
                "authority.json",
                "--profile",
                "moriarty-jubjub-owner-intent/1",
                "--target",
                "target.json",
            ],
            "requires --target and --at together",
        ),
        (
            "intent-signed-profile-missing-target",
            &[
                "intent",
                "agreement.mori",
                "--action",
                "settle",
                "--scenario",
                "scenario.json",
                "--authority",
                "authority.json",
                "--profile",
                "moriarty-jubjub-owner-intent/1",
                "--at",
                "0",
            ],
            "requires --target and --at together",
        ),
        (
            "verify-intent-signed-profile-missing-target",
            &[
                "verify-intent",
                "agreement.mori",
                "--action",
                "settle",
                "--scenario",
                "scenario.json",
                "--signature",
                "sig.json",
                "--authority",
                "authority.json",
                "--at",
                "0",
                "--profile",
                "moriarty-jubjub-owner-intent/1",
            ],
            "requires --target",
        ),
    ];

    for &(label, argv, expected_msg) in semantic_cases {
        let (code, stdout, stderr) = run_mori(temp.path(), argv);
        let norm = normalize_ws(&stderr);
        assert_eq!(code, Some(1), "{label}: expected exit 1, got {stderr}");
        assert!(stdout.is_empty(), "{label}: expected empty stdout");
        assert!(
            !stderr.contains(NATIVE_CODE),
            "{label}: unexpected {NATIVE_CODE} in {stderr}"
        );
        assert!(
            norm.contains(expected_msg),
            "{label}: expected '{expected_msg}' in '{norm}'"
        );
    }
}

### render.rs
//! Graphical rendering of diagnostics against their source.

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, NamedSource, Report};

use crate::MoriDiagnostic;

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
        .with_source_code(NamedSource::new(name, source.to_owned()));
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

### shell.rs
//! Cargo-style status output on stderr.

use std::fmt::Display;
use std::io::{self, Stderr, Write};

use anstream::{AutoStream, ColorChoice};
use anstyle::Style;
use mori_diagnostics::Report;
use mori_diagnostics::render::{self, WIDTH};

use crate::styles;

/// Width of the right-aligned verb column, matching cargo.
const VERB_WIDTH: usize = 12;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verbosity {
    Quiet,
    Normal,
    Verbose,
}

/// Writes status lines such as `    Checking invoice.mori` to stderr.
///
/// Command results go to stdout and never pass through the shell. Output is
/// best effort: if stderr cannot be written, there is nowhere to report it.
pub struct Shell {
    err: AutoStream<Stderr>,
    verbosity: Verbosity,
}

impl Shell {
    pub fn new(verbosity: Verbosity, color: ColorChoice) -> Self {
        Self {
            err: AutoStream::new(io::stderr(), color),
            verbosity,
        }
    }

    /// Prints a right-aligned green verb followed by a message.
    pub fn status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    /// Like [`Shell::status`], but only under `--verbose`.
    pub fn verbose_status(&mut self, verb: &str, message: impl Display) {
        if self.verbosity == Verbosity::Verbose {
            self.aligned(verb, styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command prints notes")]
    pub fn note(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("note", styles::HEADER, message);
        }
    }

    #[expect(dead_code, reason = "used once a command emits warnings")]
    pub fn warn(&mut self, message: impl Display) {
        if self.verbosity != Verbosity::Quiet {
            self.prefixed("warning", styles::WARN, message);
        }
    }

    /// Prints an error line. Errors are shown even under `--quiet`.
    pub fn error(&mut self, message: impl Display) {
        self.prefixed("error", styles::ERROR, message);
    }

    /// Renders an error report, wrapped to the terminal width up to 80 columns.
    /// Errors are shown even under `--quiet`.
    pub fn report(&mut self, report: &Report) {
        let width = terminal_size::terminal_size_of(io::stderr())
            .map_or(WIDTH, |(width, _)| usize::from(width.0));
        let _ = writeln!(self.err, "{}", render::render_colored(report, width));
    }

    /// Prints `hunks` as `cargo fmt --check` does, indented to line up with a
    /// diagnostic's message.
    pub fn diff(&mut self, file: &str, hunks: &[mori_fmt::Hunk]) {
        const INDENT: &str = "    ";
        for hunk in hunks {
            let _ = writeln!(self.err, "{INDENT}Diff in {file}:{}:", hunk.line);
            for (change, line) in &hunk.lines {
                let (sign, style) = match change {
                    mori_fmt::Change::Context => (' ', Style::new()),
                    mori_fmt::Change::Removed => ('-', styles::REMOVED),
                    mori_fmt::Change::Added => ('+', styles::ADDED),
                };
                let _ = writeln!(self.err, "{INDENT}{style}{sign}{line}{style:#}");
            }
        }
        let _ = writeln!(self.err);
    }

    fn aligned(&mut self, verb: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{verb:>VERB_WIDTH$}{style:#} {message}");
    }

    fn prefixed(&mut self, label: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{label}:{style:#} {message}");
    }
}

### CATALOG.json
[
  {
    "argv": [
      "compile",
      "agreement.mori",
      "--target",
      "target.json",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out_dir uncreated; tree bytes unchanged",
    "route": "compile"
  },
  {
    "argv": [
      "compile-call",
      "agreement.mori",
      "--action",
      "settle",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out_dir uncreated; tree bytes unchanged",
    "route": "compile-call"
  },
  {
    "argv": [
      "prepare-call",
      "agreement.mori",
      "--action",
      "settle",
      "--deployment",
      "deploy.json",
      "--predecessor",
      "pred.bin",
      "--parameters",
      "params.bin",
      "--request",
      "req.json",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out_dir uncreated; tree bytes unchanged",
    "route": "prepare-call"
  },
  {
    "argv": [
      "keygen-call",
      "agreement.mori",
      "--seed-deployment",
      "deploy.json",
      "--seed-state",
      "state.bin",
      "--parameters",
      "params.bin",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; fake srs_dir untouched; out_dir uncreated",
    "route": "keygen-call"
  },
  {
    "argv": [
      "prove-call",
      "agreement.mori",
      "--deployment",
      "deploy.json",
      "--predecessor",
      "pred.bin",
      "--parameters",
      "params.bin",
      "--request",
      "req.json",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--keyset",
      "keyset.bin",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; fake srs_dir/keyset untouched; out_dir uncreated",
    "route": "prove-call"
  },
  {
    "argv": [
      "verify-call",
      "agreement.mori",
      "--deployment",
      "deploy.json",
      "--predecessor",
      "pred.bin",
      "--parameters",
      "params.bin",
      "--request",
      "req.json",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--public-keyset",
      "pub_keyset.bin",
      "--proof",
      "proof.bin"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "verify-call"
  },
  {
    "argv": [
      "keygen-signed-call",
      "agreement.mori",
      "--signer-policy",
      "policy.json",
      "--seed-deployment",
      "deploy.json",
      "--seed-state",
      "state.bin",
      "--parameters",
      "params.bin",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out_dir uncreated; tree bytes unchanged",
    "route": "keygen-signed-call"
  },
  {
    "argv": [
      "prepare-signed-call",
      "agreement.mori",
      "--signer-policy",
      "policy.json",
      "--deployment",
      "deploy.json",
      "--predecessor",
      "pred.bin",
      "--parameters",
      "params.bin",
      "--request",
      "req.json",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--public-keyset",
      "pub_keyset.bin",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out_dir uncreated; tree bytes unchanged",
    "route": "prepare-signed-call"
  },
  {
    "argv": [
      "prove-signed-call",
      "agreement.mori",
      "--signer-policy",
      "policy.json",
      "--deployment",
      "deploy.json",
      "--predecessor",
      "pred.bin",
      "--parameters",
      "params.bin",
      "--request",
      "req.json",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--keyset",
      "keyset.bin",
      "--signed-attachment",
      "attach.json",
      "--out",
      "out_dir"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out_dir uncreated; tree bytes unchanged",
    "route": "prove-signed-call"
  },
  {
    "argv": [
      "verify-signed-call",
      "agreement.mori",
      "--signer-policy",
      "policy.json",
      "--deployment",
      "deploy.json",
      "--predecessor",
      "pred.bin",
      "--parameters",
      "params.bin",
      "--request",
      "req.json",
      "--action",
      "settle",
      "--params-dir",
      "srs_dir",
      "--work-limits",
      "limits.json",
      "--public-keyset",
      "pub_keyset.bin",
      "--proof",
      "proof.bin",
      "--signed-attachment",
      "attach.json"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "verify-signed-call"
  },
  {
    "argv": [
      "prove",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--target",
      "target.json",
      "--params-dir",
      "srs_dir",
      "--out",
      "out.proof"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; out.proof untouched/uncreated; tree bytes unchanged",
    "route": "prove"
  },
  {
    "argv": [
      "verify",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--target",
      "target.json",
      "--params-dir",
      "srs_dir",
      "--proof",
      "out.proof"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "verify"
  },
  {
    "argv": [
      "replay",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--target",
      "target.json",
      "--record",
      "record.json"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "replay"
  },
  {
    "argv": [
      "inspect",
      "agreement.mori",
      "--target",
      "target.json"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & valid moriarty/2 TwoDeliveries source sentinel + invalid target sentinel; tree bytes unchanged",
    "route": "inspect-target"
  },
  {
    "argv": [
      "simulate",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--trace",
      "--target",
      "target.json"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "simulate-trace-target"
  },
  {
    "argv": [
      "intent",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--authority",
      "authority.json",
      "--profile",
      "moriarty-jubjub-owner-intent/1",
      "--target",
      "target.json",
      "--at",
      "0"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "intent-signed-profile"
  },
  {
    "argv": [
      "verify-intent",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--signature",
      "sig.json",
      "--authority",
      "authority.json",
      "--at",
      "0",
      "--profile",
      "moriarty-jubjub-owner-intent/1",
      "--target",
      "target.json"
    ],
    "diagnostic": "mori::command::native-unavailable + fixed message",
    "exit": 1,
    "ioBasis": "missing-inputs & populated-sentinels; tree bytes unchanged",
    "route": "verify-intent-signed-profile"
  },
  {
    "argv": [
      "compile",
      "agreement.mori",
      "--target",
      "target.json"
    ],
    "diagnostic": "clap missing --out; no mori::command::native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created; tree bytes unchanged",
    "route": "compile-missing-out"
  },
  {
    "argv": [
      "prove",
      "agreement.mori",
      "--action",
      "settle"
    ],
    "diagnostic": "clap missing required arguments; no mori::command::native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created; tree bytes unchanged",
    "route": "prove-missing-required"
  },
  {
    "argv": [
      "simulate",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--trace",
      "--logical-cap",
      "10"
    ],
    "diagnostic": "clap conflict --trace vs --logical-cap; no mori::command::native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created; tree bytes unchanged",
    "route": "simulate-trace-conflicts-logical-cap"
  },
  {
    "argv": [
      "simulate",
      "agreement.mori",
      "--resume",
      "cont.json",
      "--event",
      "ev.json",
      "--target",
      "target.json"
    ],
    "diagnostic": "clap conflict --resume vs --target; no mori::command::native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created; tree bytes unchanged",
    "route": "simulate-resume-conflicts-target"
  },
  {
    "argv": [
      "verify-intent",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--signature",
      "sig.json",
      "--authority",
      "authority.json",
      "--at",
      "0",
      "--profile",
      "moriarty-jubjub-owner-intent/1",
      "--target",
      "target.json",
      "--logical-cap",
      "10"
    ],
    "diagnostic": "clap conflict --logical-cap vs --profile/--target; no mori::command::native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created; tree bytes unchanged",
    "route": "verify-intent-cap-conflicts-profile"
  },
  {
    "argv": [
      "--quiet",
      "--verbose",
      "compile",
      "agreement.mori",
      "--target",
      "target.json",
      "--out",
      "out_dir"
    ],
    "diagnostic": "clap conflict --quiet vs --verbose; no mori::command::native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created; tree bytes unchanged",
    "route": "global-quiet-verbose-conflict"
  },
  {
    "argv": [
      "simulate",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--trace"
    ],
    "diagnostic": "--trace requires --target; no mori::command::native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate; tree bytes unchanged",
    "route": "simulate-trace-missing-target"
  },
  {
    "argv": [
      "intent",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--authority",
      "authority.json",
      "--profile",
      "moriarty-jubjub-owner-intent/1",
      "--target",
      "target.json"
    ],
    "diagnostic": "requires --target and --at together; no mori::command::native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate; tree bytes unchanged",
    "route": "intent-signed-profile-missing-at"
  },
  {
    "argv": [
      "intent",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--authority",
      "authority.json",
      "--profile",
      "moriarty-jubjub-owner-intent/1",
      "--at",
      "0"
    ],
    "diagnostic": "requires --target and --at together; no mori::command::native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate; tree bytes unchanged",
    "route": "intent-signed-profile-missing-target"
  },
  {
    "argv": [
      "verify-intent",
      "agreement.mori",
      "--action",
      "settle",
      "--scenario",
      "scenario.json",
      "--signature",
      "sig.json",
      "--authority",
      "authority.json",
      "--at",
      "0",
      "--profile",
      "moriarty-jubjub-owner-intent/1"
    ],
    "diagnostic": "requires --target; no mori::command::native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate; tree bytes unchanged",
    "route": "verify-intent-signed-profile-missing-target"
  }
]

### RESULT.json
{
  "endedAt": "2026-10-18T07:16:33.191587+00:00",
  "exitCode": 101,
  "wallSeconds": 52.26219328799925,
  "sourceCommitAfter": "c24bfc005910972663954eecfaf6d99698064438",
  "sourceTreeAfter": "7a89671b88b8e9e879c24657a299adcf6b7fac1e",
  "sourceStatusAfter": "",
  "stdoutSha256": "41132627da0793ef067e7cf10a468805f668fa4d057c8b2a2822f37e9d4ab89c",
  "stderrSha256": "0236490c2326f768e9e37799c86df70865a4e5bbbf20ef84b0f57b028b5bb198",
  "nativeAuthority": 0,
  "resultReview": "PENDING; no broadpassinferred"
}

### STDOUT.log

running 2 tests
test arg_and_clap_precedence_without_native_diagnostic ... FAILED
test native_routes_report_unavailable_before_io ... FAILED

failures:

---- arg_and_clap_precedence_without_native_diagnostic stdout ----

thread 'arg_and_clap_precedence_without_native_diagnostic' (449585) panicked at crates/mori/tests/native_feature_boundary.rs:591:9:
intent-signed-profile-missing-at: expected 'requires --target and --at together' in '× mori::command: --profile moriarty-jubjub-owner-intent/1 requires --target │ and --at together'
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- native_routes_report_unavailable_before_io stdout ----

thread 'native_routes_report_unavailable_before_io' (449594) panicked at crates/mori/tests/native_feature_boundary.rs:413:13:
missing-inputs/compile: missing message in × mori::command::native-unavailable: This mori build has no Midnight │ support. Rebuild with the native feature enabled.


failures:
    arg_and_clap_precedence_without_native_diagnostic
    native_routes_report_unavailable_before_io

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


### STDERR.log
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.26
   Compiling quote v1.0.47
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling stable_deref_trait v1.2.1
   Compiling typenum v1.20.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling writeable v0.6.4
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling icu_locale_fallback_data v2.3.0
   Compiling rustix v1.1.5
   Compiling litemap v0.8.3
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling linux-raw-sys v0.12.1
   Compiling const-oid v0.9.6
   Compiling icu_segmenter_data v2.3.0
   Compiling bitflags v2.13.2
   Compiling subtle v2.6.1
   Compiling icu_provider v2.3.1
   Compiling utf8_iter v1.0.4
   Compiling icu_collections v2.3.0
   Compiling icu_locale_fallback v2.3.0
   Compiling smallvec v1.16.2
   Compiling icu_segmenter v2.3.0
   Compiling unicode-width v0.2.2
   Compiling is_ci v1.2.0
   Compiling supports-color v3.0.2
   Compiling textwrap v0.16.4
   Compiling terminal_size v0.4.4
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling getrandom v0.2.17
   Compiling unicode-width v0.1.14
   Compiling supports-hyperlinks v3.2.0
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-span)
   Compiling der v0.7.10
   Compiling crossbeam-utils v0.8.23
   Compiling memchr v2.8.3
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling crossbeam-deque v0.8.8
   Compiling base16ct v0.2.0
   Compiling serde_core v1.0.229
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling zmij v1.0.23
   Compiling once_cell v1.21.4
   Compiling utf8parse v0.2.2
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling const-oid v0.10.2
   Compiling getrandom v0.4.3
   Compiling cpufeatures v0.2.17
   Compiling thiserror v2.0.21
   Compiling is_terminal_polyfill v1.70.2
   Compiling serde_json v1.0.151
   Compiling regex-syntax v0.8.11
   Compiling anstyle v1.0.14
   Compiling rayon-core v1.13.0
   Compiling colorchoice v1.0.5
   Compiling serde v1.0.229
   Compiling anstyle-query v1.1.5
   Compiling anstream v1.0.0
   Compiling regex-automata v0.4.18
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling serde_derive v1.0.229
   Compiling thiserror-impl v2.0.21
   Compiling strsim v0.11.1
   Compiling log v0.4.34
   Compiling cpufeatures v0.3.1
   Compiling similar v2.7.0
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling itoa v1.0.18
   Compiling same-file v1.0.6
   Compiling walkdir v2.5.0
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling sha2 v0.11.0
   Compiling globset v0.4.20
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-parser)
   Compiling fastrand v2.5.0
   Compiling either v1.19.0
   Compiling rayon v1.12.0
   Compiling tempfile v3.27.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-main-merge-20261017/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 52.22s
     Running tests/native_feature_boundary.rs (/tmp/doctorwatson-cli-availability-check-zpj1mxlq/target/debug/deps/native_feature_boundary-a12ad28045c649a9)
error: test failed, to rerun pass `-p mori --test native_feature_boundary`
