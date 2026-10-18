# Independent review input — PR227 shared source checker

Review target: https://github.com/CharlesHoskinson/Moriarty/pull/227 (stored draft metadata; no fresh provider/GitHub request). Exact head `823ce081c00786a0c55b92af81009f9e7c5e6a16`, tree `aa593783a8717ac838ae556e24c187997ab3547e`, direct parent `e4d7a2b5709caa18a94f5256d03a0302364b8235` (E4D). Source bodies come from exact Git objects, never mutable refs. The runtime baseline is a separate `97c9b1565d71404413aa35e9f95fe7661b8a09b1`, tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; its entire delta from E4D is the exact15 test-only paths recorded below. This distinction must survive review.

Packet preparation and prior source author: GPT-6.1 Sol. This packet is NOT an independent review, approval or design vote. A fresh non-author reviewer must read and assess it. Requested current ordinary route is Muse Wonder1.0 high under root steering; record requested and actual returned identity/effort/status honestly. No provider invocation occurred in this preparation lane.

Falsifiable review predicate: each reported whole-byte equality for the eight normal public CLI check cases binds to the actual stored baseline and candidate head, correct source bytes/original filename, complete stdout including final newline, actual argv, exits and stderr. Source inspection must separately assess preservation of actual Beta/Core1/Core2 dispatch, diagnostics/spans, report order, lazy human reports and Core2 qualification labels. Passing eight cases does not prove general semantic equivalence.

## Review contract

You are a fresh independent non-author reviewer of PR227 at the exact pinned head. This is a substantive ordinary source/result audit, not a design vote, security review or approval by the packet author. Requested route follows current root steering and Moriarty routing; record actual provider/model/effort/terminal status honestly. Never substitute another identity or approve missing input.

Read PRIMARY.md in full, including all four complete Rust files, all 21 changed paths, eight sources, fixture bindings, complete eight baseline and eight after reports, scoped command and complete integration-test output, baseline-parent delta and pinned context. Report any unread/truncated portion explicitly. The eight-case suite is an exact target selection, not the full package/workspace suite; zero test names were filtered inside that target. Broader tests, clippy and general semantic/profile coverage are not implied.

Return one JSON object satisfying REVIEW-RESPONSE.schema.json. Cover every changed path with its exact SHA and full/partial/unread scope. Supply a substantive source assessment for every REVIEW-PREDICATES key and a separate result assessment for each of eight cases. Every assessment needs exact quotes with path, SHA256 and original line numbers, plus concrete evidence references and limits. For each byte claim cite the pair of stored START/RESULT/STDOUT records, actual baseline/head commits, source bytes and original filename. Compare entire stdout bytes including the final newline; JSON-value equality alone cannot establish Beta field order. Discuss compatibility-oracle provenance separately from independent semantics.

Findings require precise source/result quotes, an explicit violated predicate, expected versus actual behavior and a concrete repair or next check. If no defects are found, the nine source and eight result checks still require specific evidence and reasoning; empty findings alone is not a review. Assess source disposition separately from the eight-case result disposition. An unread mandatory file or unavailable reviewer prevents scoped source acceptance. Do not treat packet self-checks or root assertions as independent review.

Preserve human report laziness, real checked types, Parsed/check ordering, Beta DTO field order and spans, Core1/Core2 behavior/labels, exits, I/O behavior and permissionless public tool semantics. Do not invent allocation/verbose-timing measurement: inspect those properties from source and note the empirical limits. No new security/hostile-test judgments are assigned to this ordinary review; attribute existing source labels and route any such concern to the designated owner.

The Rust MCP command remains a TODO at this head. This increment is a useful prerequisite only. Preserve the separate full35-path official-SDK MCP delivery, two substantive design votes, designated dependency/build-script/resource judgments, exact lockfile resolution, result reviews and runtime/acceptance obligations as open. No narrower replacement or whole-language/native/ledger conclusion is authorized. No provider execution or source change was performed by this packet preparer.

Source predicates requiring individual substantive assessment:

- `public-consumer`: Actual public mori check path consumes the extracted dispatcher; no dead standalone helper or MCP runtime claim.
- `profiles-and-checkers`: Original lexer dispatch, Beta parse/check including parse errors and partial checked values, Core1 parse and Core2 stage load preserve existing acceptance paths.
- `beta-json-bytes`: Existing Beta CheckReport remains directly struct-serialized, with complete field order, hashes, partial results, diagnostics and byte spans.
- `human-laziness-and-order`: Human mode does not allocate owned JSON reports; Parsed runs before checker and AST/check data releases before final status/printing.
- `core-report-and-qualification`: Core accepted/rejected report fields and all three Core2 labels remain exact on both branches.
- `exit-and-diagnostics`: Existing exits and diagnostic rendering remain correct; quiet suppresses status but not errors; I/O/export errors still propagate.
- `public-regression-and-oracle`: Eight actual public binary cases cover JSON whole bytes/exits/stderr and quiet human checks; original baseline bytes are compatibility oracles, not independent semantic or financial oracles.
- `scope-and-guidelines`: No dependency, manifest, feature, editor, new profile or permissionless public policy change; Rust guideline and scoped-test deviations/limits explicitly assessed.
- `evidence-boundaries`: Baseline97c9-to-E4D exact15 test-only delta, candidate exact21 path pins and actual eight direct byte claims are qualified without full-language/MCP/native acceptance.

## Scope, limits and provenance

The real consumer is public `mori check FILE [--json]` → existing command routing → `check::run` → shared `check_source::dispatch` → unchanged actual Beta parser/checker or Core1 parse/Core2 stage loader → the CLI consumer → existing report/exporter. No new public profile, dependency/feature, Cargo file, editor, protocol, evaluator or SDK server is added. The Rust MCP command remains TODO and is included as context. Public use remains permissionless; internal reviews do not gate individual developers.

There are exactly21 changed paths: two modified production files, one new shared production module, one public CLI regression target, eight .mori fixtures, eight stdout goldens and one fixture manifest. All21 full bodies are represented losslessly below: the eight golden-file bodies alias the independently byte-equal baseline stdout blocks, with identical hashes. All four Rust files are inline in full. Context excerpts are explicit; parser/dependency internals, global guideline conformance and full runtime/semantic audit remain unestablished. Full preserved context bodies and hashes remain in `context/` and `CONTEXT-PINS.json`; excerpt selection is explicit, not a claim to full-module audit.

Execution results below are attributed stored root observations, not processes run by this packet author. Baseline build:41.284s/exit0. Candidate selected target:22.640s/eightPASS/zero filtered. Each test calls JSON and quiet human CLI modes. Eight direct candidate JSON stdout/exits equal baseline; all direct JSON stderr empty. Quiet human checks establish stdout silence/exits/diagnostic presence only, not exact renderer bytes, verbose timing, allocation counts or memory bounds. Lazy report construction and Parsed/check ordering are source predicates, not measured allocation/timing results.

The goldens are captured original-implementation compatibility oracles. The original frozen input also contains historical/authored specified DTO expectations and an unrelated proposed MCP envelope; only CLI expectations were run here. No MCP envelope, oracle provider, financial evaluator, native proof, wallet, network or ledger action was executed by these source checks. The full original21,209-byte freeze is preserved at `receipts/INPUT-CASES-FROZEN.json`; MCP-only fields are not hidden or relabeled as execution evidence. Each actual source text and complete baseline/after CLI JSON is inline below.

The only test selection was `--test check_source_dispatch -- --test-threads=1` with `--no-default-features`; it did not run the whole package or workspace. Clippy, broad suites, allocation/verbose measurements and full supported-grammar coverage are not established. Root formatted the candidate before commitment; this head's four Rust bodies, rather than the earlier unformatted proposal, are the review target. The initial broad precheck wrongly required unrelated UMP test paths; it failed before changed-source execution. The corrected exact15 test-only delta and final receipt are retained. `CURRENT-SOURCE.json` is a pre-execution snapshot and still says UNRUN_CHANGED_SOURCE; later FINAL-RESULT and START/RESULT logs establish the scoped observations. Do not treat that historical status string as a post-test verdict.

No full MCP first delivery is claimed. The separate35-path official-SDK candidate, two substantive design votes, designated dependency/build-script/resource judgments, lockfile resolution, independent result review and runtime delivery remain open. U0–U7, M1–M17 and MC/SP/G/native/target exits are not closed. Security authoring, hostile-test design and new security conclusions are outside this ordinary lane; existing source qualification labels are attributed facts only.

Startup restored the installed Moriarty develop skill and ran the required read-only status from K; its SP01 blockers are historical operational state, not this work queue. `STARTUP.json` preserves that snapshot. No Foreman, scheduler, provider, build, Git mutation or shared write was invoked by this preparation lane. No root-owned temporary build path was accessed; cleanup is a stored root receipt.

## Exact changed paths

| Path | Kind | Bytes | Head SHA256 |
|---|---|---:|---|
| `crates/fixtures/check-source/cases.json` | new | 3181 | `fc4306dd34f3bb76763b5d45afe5bb91496069e02b3102237bb1dff3a3edd2f8` |
| `crates/fixtures/check-source/cases/accepted.mori` | new | 273 | `2b08461661551b3d54f1db9e8d4c887ecfe31378585202c48ca7ef232cb7751b` |
| `crates/fixtures/check-source/cases/core1-accepted.mori` | new | 81 | `5014535ca4879d3a6e1580f6dc3f08dcf511309eb8757f2c9c34d7e226a80526` |
| `crates/fixtures/check-source/cases/core1-rejected.mori` | new | 42 | `eb7a9e96829f3ef9aae077f18adee18f334c4b6ab3b34db8463a9153af99d9e8` |
| `crates/fixtures/check-source/cases/core2-accepted.mori` | new | 81 | `29d20136db0fbe85bc161e134a497696ef5c7e994b0787b492af0246660359dc` |
| `crates/fixtures/check-source/cases/core2-rejected.mori` | new | 42 | `37d660cea5a09c6d85d9a8121a634320754cf833eee733efc208728ca5c24d19` |
| `crates/fixtures/check-source/cases/rejected.mori` | new | 92 | `4ad933a610329747a6cfb0258a5040e583880d28705eb355d4dd4374c57eba6b` |
| `crates/fixtures/check-source/cases/unicode-byte-offsets.mori` | new | 101 | `211dc3011819cc0e18300d745edc7b9510fc4b32a74146eb11566c69bc9bf34f` |
| `crates/fixtures/check-source/cases/unparsable.mori` | new | 66 | `b58eb56f4fb88fa8f93aea956f22020c9af595bb9223b83f772f1cdef7a92123` |
| `crates/fixtures/check-source/expected/accepted.stdout` | new | 1120 | `e0443b69cbbbcd432f4ed097d06e7a368b398331a7c579210ef2a8382c926d85` |
| `crates/fixtures/check-source/expected/core1-accepted.stdout` | new | 395 | `dc3bdde91774cdb2e92dcfdf962c91560044f5b3a19d99612b4cc7be648cc442` |
| `crates/fixtures/check-source/expected/core1-rejected.stdout` | new | 220 | `984922ce39818964f928d341ed8155a4438e6fce8a2fde0794c1f9bd9f8c988a` |
| `crates/fixtures/check-source/expected/core2-accepted.stdout` | new | 497 | `98fbc87d6c2515a856b571b43f4a89f53fd117160582495e327e1c4b2cf76484` |
| `crates/fixtures/check-source/expected/core2-rejected.stdout` | new | 322 | `6e603241ee81db4aa11b2b466b1745b76f2c534dfe6b3fa380e3033195bd9ff5` |
| `crates/fixtures/check-source/expected/rejected.stdout` | new | 1661 | `674b78dd4cb550144971bb16fbf08b476413956573481e0ae6cce47ed15da83e` |
| `crates/fixtures/check-source/expected/unicode-byte-offsets.stdout` | new | 1661 | `37475ec91dd4978b7c9ac5d5d9d426b1b0c19b8cf4a1a0f912a45dbecf248320` |
| `crates/fixtures/check-source/expected/unparsable.stdout` | new | 722 | `295ad8827179e74236cc6dd4392afc76b143062f370d9095bc49cbcada90faf6` |
| `crates/mori/src/commands/check.rs` | modified | 3744 | `2f08c04bdc1a07796c459385ec9271466a26572f2455ed7d63de906c1d5876bb` |
| `crates/mori/src/commands/check_source.rs` | new | 3014 | `b41456e7b975b1b68f499652765f8bc807f13cf1e811dd73ed77787a3a3320b6` |
| `crates/mori/src/commands/mod.rs` | modified | 2745 | `c47cca4f7f66e6f8ce0707f106598fb9b18a77fab36214096f0e09c2edfc0c06` |
| `crates/mori/tests/check_source_dispatch.rs` | new | 3431 | `cbc4529bd03751163e28bb8089ae6d451e120bd95f5023733845402b3185a91a` |

## Complete four Rust files at the exact review head

### crates/mori/src/commands/check.rs

`head/crates/mori/src/commands/check.rs`; complete body; SHA256 `2f08c04bdc1a07796c459385ec9271466a26572f2455ed7d63de906c1d5876bb`.

```rust
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

use super::check_source::{self, SourceCheckConsumer};

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    let outcome = check_source::dispatch(
        &source,
        CliConsumer {
            name: &name,
            source: &source,
            json: args.json,
            shell,
        },
    )?;
    if outcome.beta {
        if let Some(report) = outcome.report {
            super::core_io::print(&report)?;
        } else if outcome.accepted {
            shell.status(
                "Finished",
                format!("check in {:.2}s", started.elapsed().as_secs_f64()),
            );
        }
    }
    Ok(if outcome.accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

struct CliOutcome {
    accepted: bool,
    beta: bool,
    report: Option<CheckReport>,
}

struct CliConsumer<'a> {
    name: &'a str,
    source: &'a str,
    json: bool,
    shell: &'a mut Shell,
}

impl SourceCheckConsumer for CliConsumer<'_> {
    type Output = super::Result<CliOutcome>;

    fn parsed(&mut self, tokens: usize, nodes: usize) {
        if !self.json {
            self.shell
                .verbose_status("Parsed", format!("{tokens} tokens, {nodes} nodes"));
        }
    }

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output {
        let report = self.json.then(|| {
            check_source::beta_report(self.name.to_owned(), self.source, checked, &diagnostics)
        });
        let accepted = finish(self.shell, self.name, self.source, diagnostics, self.json);
        Ok(CliOutcome {
            accepted,
            beta: true,
            report,
        })
    }

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output {
        if self.json {
            let report = check_source::core_report(self.name, self.source, &result, staged);
            super::core_io::print(&report)?;
        } else {
            match &result {
                Ok(_) => self.shell.status("Finished", "bounded core check"),
                Err(error) => self.shell.error(error),
            }
        }
        Ok(CliOutcome {
            accepted: result.is_ok(),
            beta: false,
            report: None,
        })
    }
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}
```

### crates/mori/src/commands/check_source.rs

`head/crates/mori/src/commands/check_source.rs`; complete body; SHA256 `b41456e7b975b1b68f499652765f8bc807f13cf1e811dd73ed77787a3a3320b6`.

```rust
//! Shared source-profile dispatch and the existing check report formats.

use mori_diagnostics::MoriDiagnostic;

use crate::json::CheckReport;

/// Consume checked values while the Beta AST they borrow is still alive.
/// Reports remain optional: a human consumer can render diagnostics directly.
pub(super) trait SourceCheckConsumer {
    type Output;

    /// Called after Beta parsing, before its checker starts.
    fn parsed(&mut self, _tokens: usize, _nodes: usize) {}

    fn beta(
        self,
        checked: Option<&mori_checker::Checked<'_>>,
        diagnostics: Vec<MoriDiagnostic>,
    ) -> Self::Output;

    fn core(
        self,
        result: Result<mori_core::Program, mori_core::Error>,
        staged: bool,
    ) -> Self::Output;
}

/// Select exactly the same Beta, Core1 or staged Core2 path as `mori check`.
pub(super) fn dispatch<C: SourceCheckConsumer>(source: &str, mut consumer: C) -> C::Output {
    if mori_lexer::first_token_text(source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(source).is_ok_and(|language| language == "moriarty/2");
        let result = if staged {
            mori_core::stage::load(source).map(|program| program.program().clone())
        } else {
            mori_core::parse(source)
        };
        return consumer.core(result, staged);
    }
    match mori_parser::parse(source) {
        Err(error) => consumer.beta(None, vec![error]),
        Ok(ast) => {
            consumer.parsed(ast.tokens.len(), ast.nodes.len());
            let mori_checker::CheckResult {
                checked,
                diagnostics,
            } = mori_checker::check(&ast);
            consumer.beta(Some(&checked), diagnostics)
        }
    }
}

/// Preserve the Beta struct's serialization order and diagnostic/span mapping.
pub(super) fn beta_report(
    file: String,
    source: &str,
    checked: Option<&mori_checker::Checked<'_>>,
    diagnostics: &[MoriDiagnostic],
) -> CheckReport {
    CheckReport::new(file, source, checked, diagnostics)
}

/// Construct the existing Core JSON object only when a consumer requests it.
pub(super) fn core_report(
    file: &str,
    source: &str,
    result: &Result<mori_core::Program, mori_core::Error>,
    staged: bool,
) -> serde_json::Value {
    let mut report = match result {
        Ok(program) => {
            serde_json::json!({"status":"accepted", "file":file,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
        }
        Err(error) => {
            serde_json::json!({"status":"rejected","file":file,"diagnostics":[error]})
        }
    };
    if staged {
        let object = report
            .as_object_mut()
            .expect("stage reports are JSON objects");
        for (name, value) in mori_core::stage::STAGE_LABELS {
            object.insert(name.into(), serde_json::Value::String(value.into()));
        }
    }
    report
}
```

### crates/mori/src/commands/mod.rs

`head/crates/mori/src/commands/mod.rs`; complete body; SHA256 `c47cca4f7f66e6f8ce0707f106598fb9b18a77fab36214096f0e09c2edfc0c06`.

```rust
//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
pub mod check;
mod check_source;
pub mod compile;
pub mod compile_call;
mod core_io;
pub mod expand;
pub mod format;
pub mod init;
pub mod inspect;
pub mod intent;
pub mod keygen_call;
pub mod keygen_signed_call;
pub mod lsp;
pub mod mcp;
pub mod prepare_call;
pub mod prepare_signed_call;
mod proof_io;
pub mod prove;
pub mod prove_call;
pub mod prove_signed_call;
pub mod replay;
mod signed_call_io;
pub mod simulate;
pub mod test;
pub mod verify;
pub mod verify_call;
pub mod verify_intent;
pub mod verify_signed_call;

pub type Result<T> = std::result::Result<T, Report>;

pub fn run(command: Command, shell: &mut Shell) -> Result<ExitCode> {
    match command {
        Command::Init(args) => init::run(args, shell),
        Command::Check(args) => check::run(args, shell),
        Command::Compile(args) => compile::run(args, shell),
        Command::KeygenSignedCall(args) => keygen_signed_call::run(args, shell),
        Command::PrepareSignedCall(args) => prepare_signed_call::run(args, shell),
        Command::ProveSignedCall(args) => prove_signed_call::run(args, shell),
        Command::VerifySignedCall(args) => verify_signed_call::run(args, shell),
        Command::KeygenCall(args) => keygen_call::run(args, shell),
        Command::ProveCall(args) => prove_call::run(args, shell),
        Command::VerifyCall(args) => verify_call::run(args, shell),
        Command::CompileCall(args) => compile_call::run(args, shell),
        Command::PrepareCall(args) => prepare_call::run(args, shell),
        Command::Prove(args) => prove::run(args, shell),
        Command::Verify(args) => verify::run(args, shell),
        Command::Format(args) => format::run(args, shell),
        Command::Inspect(args) => inspect::run(args, shell),
        Command::Expand(args) => expand::run(args, shell),
        Command::Simulate(args) => simulate::run(args, shell),
        Command::Replay(args) => replay::run(args, shell),
        Command::Test(args) => test::run(args, shell),
        Command::Intent(args) => intent::run(args, shell),
        Command::VerifyIntent(args) => verify_intent::run(args, shell),
        Command::Lsp(args) => lsp::run(args, shell),
        Command::Mcp(args) => mcp::run(args, shell),
    }
}

#[cfg(not(feature = "native"))]
fn native_unavailable<T>() -> Result<T> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::native-unavailable".into(),
        message: "This mori build has no Midnight support. Rebuild with the native feature enabled."
            .into(),
    }
    .into())
}
```

### crates/mori/tests/check_source_dispatch.rs

`head/crates/mori/tests/check_source_dispatch.rs`; complete body; SHA256 `cbc4529bd03751163e28bb8089ae6d451e120bd95f5023733845402b3185a91a`.

```rust
//! Complete public `mori check` output and exits for the shared dispatch paths.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Case {
    id: String,
    source: String,
    file_label: String,
    argv: Vec<String>,
    expected_exit: i32,
    expected_stdout: String,
    human_argv: Vec<String>,
}

struct CaseDirectory(PathBuf);

impl CaseDirectory {
    fn new(id: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("current time is after the Unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "mori-check-{id}-{}-{nonce}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&path).expect("create isolated check directory");
        Self(path)
    }
}

impl Drop for CaseDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn check_case(id: &str) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures/check-source");
    let cases: Vec<Case> = serde_json::from_slice(
        &std::fs::read(fixtures.join("cases.json")).expect("read case manifest"),
    )
    .expect("decode case manifest");
    let case = cases.iter().find(|case| case.id == id).expect("named case");
    let source = std::fs::read(fixtures.join(&case.source)).expect("read source fixture");
    let directory = CaseDirectory::new(id);
    std::fs::write(directory.0.join(&case.file_label), &source).expect("copy source bytes");
    let expected = std::fs::read(fixtures.join(&case.expected_stdout)).expect("read byte oracle");
    let output = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(&case.argv)
        .output()
        .expect("run public JSON check");
    let description = format!("case {id} source:\n{}", String::from_utf8_lossy(&source));
    assert_eq!(
        output.status.code(),
        Some(case.expected_exit),
        "{description}"
    );
    assert_eq!(output.stdout, expected, "{description}");
    assert!(
        output.stderr.is_empty(),
        "{description}: {:?}",
        output.stderr
    );

    let human = Command::new(env!("CARGO_BIN_EXE_mori"))
        .current_dir(&directory.0)
        .args(&case.human_argv)
        .output()
        .expect("run public human check");
    assert_eq!(
        human.status.code(),
        Some(case.expected_exit),
        "{description}"
    );
    assert!(human.stdout.is_empty(), "{description}");
    assert_eq!(
        human.stderr.is_empty(),
        case.expected_exit == 0,
        "{description}"
    );
}

#[test]
fn beta_accepted() {
    check_case("accepted");
}

#[test]
fn beta_rejected() {
    check_case("rejected");
}

#[test]
fn beta_parse_error() {
    check_case("unparsable");
}

#[test]
fn core1_accepted() {
    check_case("core1-accepted");
}

#[test]
fn core1_rejected() {
    check_case("core1-rejected");
}

#[test]
fn core2_accepted() {
    check_case("core2-accepted");
}

#[test]
fn core2_rejected() {
    check_case("core2-rejected");
}

#[test]
fn beta_unicode_byte_offsets() {
    check_case("unicode-byte-offsets");
}
```

## Complete original production checker for comparison

### E4D commands/check.rs before refactor

`base/crates/mori/src/commands/check.rs`; complete body; SHA256 `2199d7b49173816c527436eef3cf30c2cb51d1b039603b1046e7bd508762faf7`.

```rust
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Instant;

use mori_diagnostics::{MoriDiagnostic, NamedSource};

use crate::json::CheckReport;
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Checks syntax, names, values, types, declarations, calls and intents.
pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    let name = args.file.display().to_string();
    let source = super::core_io::read_source(&args.file)?;

    if !args.json {
        shell.status("Checking", &name);
    }
    let started = Instant::now();
    if mori_lexer::first_token_text(&source).is_ok_and(|token| token == "language") {
        let staged = mori_core::language(&source).is_ok_and(|language| language == "moriarty/2");
        let result = if staged {
            mori_core::stage::load(&source).map(|program| program.program().clone())
        } else {
            mori_core::parse(&source)
        };
        if args.json {
            let report = match &result {
                Ok(program) => {
                    serde_json::json!({"status":"accepted", "file":name,"source_hash":mori_core::intent::hash_bytes(source.as_bytes()),"language":program.language,"agreement":program.name,"actions":program.actions,"diagnostics":[]})
                }
                Err(error) => {
                    serde_json::json!({"status":"rejected","file":name,"diagnostics":[error]})
                }
            };
            if staged {
                super::simulate::print_stage_report(report)?;
            } else {
                super::core_io::print(&report)?;
            }
        } else {
            match &result {
                Ok(_) => shell.status("Finished", "bounded core check"),
                Err(error) => shell.error(error),
            }
        }
        return Ok(if result.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        });
    }
    let (accepted, report) = match mori_parser::parse(&source) {
        Err(error) => {
            let diagnostics = vec![error];
            let report = args
                .json
                .then(|| CheckReport::new(name.clone(), &source, None, &diagnostics));
            (
                finish(shell, &name, &source, diagnostics, args.json),
                report,
            )
        }
        Ok(ast) => {
            if !args.json {
                shell.verbose_status(
                    "Parsed",
                    format!("{} tokens, {} nodes", ast.tokens.len(), ast.nodes.len()),
                );
            }
            let result = mori_checker::check(&ast);
            let report = args.json.then(|| {
                CheckReport::new(
                    name.clone(),
                    &source,
                    Some(&result.checked),
                    &result.diagnostics,
                )
            });
            (
                finish(shell, &name, &source, result.diagnostics, args.json),
                report,
            )
        }
    };

    if let Some(report) = report {
        super::core_io::print(&report)?;
    } else if accepted {
        shell.status(
            "Finished",
            format!("check in {:.2}s", started.elapsed().as_secs_f64()),
        );
    }
    Ok(if accepted {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

/// Renders the diagnostics, unless the report is JSON. True when there are none.
fn finish(
    shell: &mut Shell,
    name: &str,
    source: &str,
    diagnostics: Vec<MoriDiagnostic>,
    json: bool,
) -> bool {
    if diagnostics.is_empty() {
        return true;
    }
    if !json {
        let count = diagnostics.len();
        for diagnostic in diagnostics {
            shell.report(&diagnostic.with_source_code(NamedSource::new(
                crate::display::text(name),
                source.to_owned(),
            )));
        }
        let plural = if count == 1 { "" } else { "s" };
        shell.error(format!(
            "could not check `{name}` due to {count} previous error{plural}"
        ));
    }
    false
}
```

## All eight source inputs and exact fixture bindings

### Committed fixture bindings

`head/crates/fixtures/check-source/cases.json`; complete body; SHA256 `fc4306dd34f3bb76763b5d45afe5bb91496069e02b3102237bb1dff3a3edd2f8`.

```json
[
  {
    "id": "accepted",
    "source": "cases/accepted.mori",
    "fileLabel": "test.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "test.mori",
      "--json"
    ],
    "expectedExit": 0,
    "expectedStdout": "expected/accepted.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "test.mori"
    ]
  },
  {
    "id": "rejected",
    "source": "cases/rejected.mori",
    "fileLabel": "test.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "test.mori",
      "--json"
    ],
    "expectedExit": 1,
    "expectedStdout": "expected/rejected.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "test.mori"
    ]
  },
  {
    "id": "unparsable",
    "source": "cases/unparsable.mori",
    "fileLabel": "test.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "test.mori",
      "--json"
    ],
    "expectedExit": 1,
    "expectedStdout": "expected/unparsable.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "test.mori"
    ]
  },
  {
    "id": "core1-accepted",
    "source": "cases/core1-accepted.mori",
    "fileLabel": "minimal.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "minimal.mori",
      "--json"
    ],
    "expectedExit": 0,
    "expectedStdout": "expected/core1-accepted.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "minimal.mori"
    ]
  },
  {
    "id": "core1-rejected",
    "source": "cases/core1-rejected.mori",
    "fileLabel": "empty.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "empty.mori",
      "--json"
    ],
    "expectedExit": 1,
    "expectedStdout": "expected/core1-rejected.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "empty.mori"
    ]
  },
  {
    "id": "core2-accepted",
    "source": "cases/core2-accepted.mori",
    "fileLabel": "minimal.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "minimal.mori",
      "--json"
    ],
    "expectedExit": 0,
    "expectedStdout": "expected/core2-accepted.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "minimal.mori"
    ]
  },
  {
    "id": "core2-rejected",
    "source": "cases/core2-rejected.mori",
    "fileLabel": "empty.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "empty.mori",
      "--json"
    ],
    "expectedExit": 1,
    "expectedStdout": "expected/core2-rejected.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "empty.mori"
    ]
  },
  {
    "id": "unicode-byte-offsets",
    "source": "cases/unicode-byte-offsets.mori",
    "fileLabel": "test.mori",
    "argv": [
      "--color",
      "never",
      "check",
      "test.mori",
      "--json"
    ],
    "expectedExit": 1,
    "expectedStdout": "expected/unicode-byte-offsets.stdout",
    "humanArgv": [
      "--color",
      "never",
      "--quiet",
      "check",
      "test.mori"
    ]
  }
]
```

### accepted complete source

`head/crates/fixtures/check-source/cases/accepted.mori`; complete body; SHA256 `2b08461661551b3d54f1db9e8d4c887ecfe31378585202c48ca7ef232cb7751b`.

```text
profile "moriarty-beta/1";
agreement Oracle {
  domain Preview = { id: "preview", chain: "midnight", network: "preview" };
  observation price = { id: "price", domain: Preview };
  intent read = { operation: oracle.select(observation: price) };
  action fetch uses read;
}
```

### rejected complete source

`head/crates/fixtures/check-source/cases/rejected.mori`; complete body; SHA256 `4ad933a610329747a6cfb0258a5040e583880d28705eb355d4dd4374c57eba6b`.

```text
profile "moriarty-beta/1";
agreement Invoice {
  const price = 10;
  const price = pirce;
}
```

### unparsable complete source

`head/crates/fixtures/check-source/cases/unparsable.mori`; complete body; SHA256 `b58eb56f4fb88fa8f93aea956f22020c9af595bb9223b83f772f1cdef7a92123`.

```text
profile "moriarty-beta/1";
agreement Invoice {
  const price = 10
```

### core1-accepted complete source

`head/crates/fixtures/check-source/cases/core1-accepted.mori`; complete body; SHA256 `5014535ca4879d3a6e1580f6dc3f08dcf511309eb8757f2c9c34d7e226a80526`.

```text
language "moriarty/1"; agreement Minimal { action advance() { requires true; } }
```

### core1-rejected complete source

`head/crates/fixtures/check-source/cases/core1-rejected.mori`; complete body; SHA256 `eb7a9e96829f3ef9aae077f18adee18f334c4b6ab3b34db8463a9153af99d9e8`.

```text
language "moriarty/1"; agreement Empty {}
```

### core2-accepted complete source

`head/crates/fixtures/check-source/cases/core2-accepted.mori`; complete body; SHA256 `29d20136db0fbe85bc161e134a497696ef5c7e994b0787b492af0246660359dc`.

```text
language "moriarty/2"; agreement Minimal { action advance() { requires true; } }
```

### core2-rejected complete source

`head/crates/fixtures/check-source/cases/core2-rejected.mori`; complete body; SHA256 `37d660cea5a09c6d85d9a8121a634320754cf833eee733efc208728ca5c24d19`.

```text
language "moriarty/2"; agreement Empty {}
```

### unicode-byte-offsets complete source

`head/crates/fixtures/check-source/cases/unicode-byte-offsets.mori`; complete body; SHA256 `211dc3011819cc0e18300d745edc7b9510fc4b32a74146eb11566c69bc9bf34f`.

```text
// café
profile "moriarty-beta/1";
agreement Invoice {
  const price = 10;
  const price = pirce;
}
```

## Full baseline and after stdout, with actual invocation identities

Every JSON block below is verbatim stored stdout and includes its final newline. Both phase stderr files are zero bytes. Each baseline block also represents the byte-identical committed expected/*.stdout body named beside it. `BYTE-CLAIMS.json` and every raw START/RESULT/STDOUT/STDERR copy retain complete command/environment provenance.

### Case accepted invocation binding

Source SHA256 `2b08461661551b3d54f1db9e8d4c887ecfe31378585202c48ca7ef232cb7751b`; original filename `test.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/accepted`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `0`; stdout bytes `1120`, SHA256 `e0443b69cbbbcd432f4ed097d06e7a368b398331a7c579210ef2a8382c926d85`; stderr bytes `0`; START `receipts/baseline-cases/accepted/START.json`; RESULT `receipts/baseline-cases/accepted/RESULT.json`.

### accepted baseline-cases complete stdout

`receipts/baseline-cases/accepted/STDOUT.log`; complete body; SHA256 `e0443b69cbbbcd432f4ed097d06e7a368b398331a7c579210ef2a8382c926d85`.

```json
{
  "status": "accepted",
  "file": "test.mori",
  "source_hash": "2b08461661551b3d54f1db9e8d4c887ecfe31378585202c48ca7ef232cb7751b",
  "agreement": {
    "name": "Oracle",
    "span": {
      "start": 37,
      "end": 43
    }
  },
  "declarations": [
    {
      "kind": "domain",
      "name": "Preview",
      "span": {
        "start": 48,
        "end": 122
      },
      "name_span": {
        "start": 55,
        "end": 62
      }
    },
    {
      "kind": "observation",
      "name": "price",
      "span": {
        "start": 125,
        "end": 178
      },
      "name_span": {
        "start": 137,
        "end": 142
      }
    },
    {
      "kind": "intent",
      "name": "read",
      "span": {
        "start": 181,
        "end": 244
      },
      "name_span": {
        "start": 188,
        "end": 192
      }
    }
  ],
  "actions": [
    {
      "name": "fetch",
      "intent": "read",
      "support": "specified_only",
      "span": {
        "start": 247,
        "end": 270
      },
      "name_span": {
        "start": 254,
        "end": 259
      }
    }
  ],
  "diagnostics": []
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/accepted`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `0`; stdout bytes `1120`, SHA256 `e0443b69cbbbcd432f4ed097d06e7a368b398331a7c579210ef2a8382c926d85`; stderr bytes `0`; START `receipts/changed-cases/accepted/START.json`; RESULT `receipts/changed-cases/accepted/RESULT.json`.

### accepted changed-cases complete stdout

`receipts/changed-cases/accepted/STDOUT.log`; complete body; SHA256 `e0443b69cbbbcd432f4ed097d06e7a368b398331a7c579210ef2a8382c926d85`.

```json
{
  "status": "accepted",
  "file": "test.mori",
  "source_hash": "2b08461661551b3d54f1db9e8d4c887ecfe31378585202c48ca7ef232cb7751b",
  "agreement": {
    "name": "Oracle",
    "span": {
      "start": 37,
      "end": 43
    }
  },
  "declarations": [
    {
      "kind": "domain",
      "name": "Preview",
      "span": {
        "start": 48,
        "end": 122
      },
      "name_span": {
        "start": 55,
        "end": 62
      }
    },
    {
      "kind": "observation",
      "name": "price",
      "span": {
        "start": 125,
        "end": 178
      },
      "name_span": {
        "start": 137,
        "end": 142
      }
    },
    {
      "kind": "intent",
      "name": "read",
      "span": {
        "start": 181,
        "end": 244
      },
      "name_span": {
        "start": 188,
        "end": 192
      }
    }
  ],
  "actions": [
    {
      "name": "fetch",
      "intent": "read",
      "support": "specified_only",
      "span": {
        "start": 247,
        "end": 270
      },
      "name_span": {
        "start": 254,
        "end": 259
      }
    }
  ],
  "diagnostics": []
}
```

### Case rejected invocation binding

Source SHA256 `4ad933a610329747a6cfb0258a5040e583880d28705eb355d4dd4374c57eba6b`; original filename `test.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/rejected`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `1`; stdout bytes `1661`, SHA256 `674b78dd4cb550144971bb16fbf08b476413956573481e0ae6cce47ed15da83e`; stderr bytes `0`; START `receipts/baseline-cases/rejected/START.json`; RESULT `receipts/baseline-cases/rejected/RESULT.json`.

### rejected baseline-cases complete stdout

`receipts/baseline-cases/rejected/STDOUT.log`; complete body; SHA256 `674b78dd4cb550144971bb16fbf08b476413956573481e0ae6cce47ed15da83e`.

```json
{
  "status": "rejected",
  "file": "test.mori",
  "source_hash": "4ad933a610329747a6cfb0258a5040e583880d28705eb355d4dd4374c57eba6b",
  "agreement": {
    "name": "Invoice",
    "span": {
      "start": 37,
      "end": 44
    }
  },
  "declarations": [
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 49,
        "end": 66
      },
      "name_span": {
        "start": 55,
        "end": 60
      }
    },
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 69,
        "end": 89
      },
      "name_span": {
        "start": 75,
        "end": 80
      }
    }
  ],
  "actions": [],
  "diagnostics": [
    {
      "code": "mori::check::duplicate_name",
      "severity": "error",
      "message": "`price` is declared twice.",
      "help": "Each declaration and action in an agreement needs its own name.",
      "labels": [
        {
          "span": {
            "start": 55,
            "end": 60
          },
          "text": "first declared here",
          "primary": false
        },
        {
          "span": {
            "start": 75,
            "end": 80
          },
          "text": "declared again here",
          "primary": true
        }
      ]
    },
    {
      "code": "mori::check::unknown_name",
      "severity": "error",
      "message": "I cannot find a declaration named `pirce`.",
      "help": "Did you mean `price`?\n\n    const price = price;",
      "labels": [
        {
          "span": {
            "start": 83,
            "end": 88
          },
          "text": "not declared above",
          "primary": true
        }
      ]
    }
  ]
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/rejected`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `1`; stdout bytes `1661`, SHA256 `674b78dd4cb550144971bb16fbf08b476413956573481e0ae6cce47ed15da83e`; stderr bytes `0`; START `receipts/changed-cases/rejected/START.json`; RESULT `receipts/changed-cases/rejected/RESULT.json`.

### rejected changed-cases complete stdout

`receipts/changed-cases/rejected/STDOUT.log`; complete body; SHA256 `674b78dd4cb550144971bb16fbf08b476413956573481e0ae6cce47ed15da83e`.

```json
{
  "status": "rejected",
  "file": "test.mori",
  "source_hash": "4ad933a610329747a6cfb0258a5040e583880d28705eb355d4dd4374c57eba6b",
  "agreement": {
    "name": "Invoice",
    "span": {
      "start": 37,
      "end": 44
    }
  },
  "declarations": [
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 49,
        "end": 66
      },
      "name_span": {
        "start": 55,
        "end": 60
      }
    },
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 69,
        "end": 89
      },
      "name_span": {
        "start": 75,
        "end": 80
      }
    }
  ],
  "actions": [],
  "diagnostics": [
    {
      "code": "mori::check::duplicate_name",
      "severity": "error",
      "message": "`price` is declared twice.",
      "help": "Each declaration and action in an agreement needs its own name.",
      "labels": [
        {
          "span": {
            "start": 55,
            "end": 60
          },
          "text": "first declared here",
          "primary": false
        },
        {
          "span": {
            "start": 75,
            "end": 80
          },
          "text": "declared again here",
          "primary": true
        }
      ]
    },
    {
      "code": "mori::check::unknown_name",
      "severity": "error",
      "message": "I cannot find a declaration named `pirce`.",
      "help": "Did you mean `price`?\n\n    const price = price;",
      "labels": [
        {
          "span": {
            "start": 83,
            "end": 88
          },
          "text": "not declared above",
          "primary": true
        }
      ]
    }
  ]
}
```

### Case unparsable invocation binding

Source SHA256 `b58eb56f4fb88fa8f93aea956f22020c9af595bb9223b83f772f1cdef7a92123`; original filename `test.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/unparsable`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `1`; stdout bytes `722`, SHA256 `295ad8827179e74236cc6dd4392afc76b143062f370d9095bc49cbcada90faf6`; stderr bytes `0`; START `receipts/baseline-cases/unparsable/START.json`; RESULT `receipts/baseline-cases/unparsable/RESULT.json`.

### unparsable baseline-cases complete stdout

`receipts/baseline-cases/unparsable/STDOUT.log`; complete body; SHA256 `295ad8827179e74236cc6dd4392afc76b143062f370d9095bc49cbcada90faf6`.

```json
{
  "status": "rejected",
  "file": "test.mori",
  "source_hash": "b58eb56f4fb88fa8f93aea956f22020c9af595bb9223b83f772f1cdef7a92123",
  "agreement": null,
  "declarations": [],
  "actions": [],
  "diagnostics": [
    {
      "code": "mori::syntax::missing_semicolon",
      "severity": "error",
      "message": "I was partway through the declaration of `price` when I found the end of the file.",
      "help": "A declaration looks like this. The type after `:` is optional:\n\n    const fee: Qty<USD> = 0.10 USD;",
      "labels": [
        {
          "span": {
            "start": 65,
            "end": 65
          },
          "text": "I expected `;` here",
          "primary": true
        }
      ]
    }
  ]
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/unparsable`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `1`; stdout bytes `722`, SHA256 `295ad8827179e74236cc6dd4392afc76b143062f370d9095bc49cbcada90faf6`; stderr bytes `0`; START `receipts/changed-cases/unparsable/START.json`; RESULT `receipts/changed-cases/unparsable/RESULT.json`.

### unparsable changed-cases complete stdout

`receipts/changed-cases/unparsable/STDOUT.log`; complete body; SHA256 `295ad8827179e74236cc6dd4392afc76b143062f370d9095bc49cbcada90faf6`.

```json
{
  "status": "rejected",
  "file": "test.mori",
  "source_hash": "b58eb56f4fb88fa8f93aea956f22020c9af595bb9223b83f772f1cdef7a92123",
  "agreement": null,
  "declarations": [],
  "actions": [],
  "diagnostics": [
    {
      "code": "mori::syntax::missing_semicolon",
      "severity": "error",
      "message": "I was partway through the declaration of `price` when I found the end of the file.",
      "help": "A declaration looks like this. The type after `:` is optional:\n\n    const fee: Qty<USD> = 0.10 USD;",
      "labels": [
        {
          "span": {
            "start": 65,
            "end": 65
          },
          "text": "I expected `;` here",
          "primary": true
        }
      ]
    }
  ]
}
```

### Case core1-accepted invocation binding

Source SHA256 `5014535ca4879d3a6e1580f6dc3f08dcf511309eb8757f2c9c34d7e226a80526`; original filename `minimal.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core1-accepted`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "minimal.mori", "--json"]`; exit `0`; stdout bytes `395`, SHA256 `dc3bdde91774cdb2e92dcfdf962c91560044f5b3a19d99612b4cc7be648cc442`; stderr bytes `0`; START `receipts/baseline-cases/core1-accepted/START.json`; RESULT `receipts/baseline-cases/core1-accepted/RESULT.json`.

### core1-accepted baseline-cases complete stdout

`receipts/baseline-cases/core1-accepted/STDOUT.log`; complete body; SHA256 `dc3bdde91774cdb2e92dcfdf962c91560044f5b3a19d99612b4cc7be648cc442`.

```json
{
  "actions": [
    {
      "inputs": [],
      "name": "advance",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 0
        }
      ]
    }
  ],
  "agreement": "Minimal",
  "diagnostics": [],
  "file": "minimal.mori",
  "language": "moriarty/1",
  "source_hash": "5014535ca4879d3a6e1580f6dc3f08dcf511309eb8757f2c9c34d7e226a80526",
  "status": "accepted"
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core1-accepted`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "minimal.mori", "--json"]`; exit `0`; stdout bytes `395`, SHA256 `dc3bdde91774cdb2e92dcfdf962c91560044f5b3a19d99612b4cc7be648cc442`; stderr bytes `0`; START `receipts/changed-cases/core1-accepted/START.json`; RESULT `receipts/changed-cases/core1-accepted/RESULT.json`.

### core1-accepted changed-cases complete stdout

`receipts/changed-cases/core1-accepted/STDOUT.log`; complete body; SHA256 `dc3bdde91774cdb2e92dcfdf962c91560044f5b3a19d99612b4cc7be648cc442`.

```json
{
  "actions": [
    {
      "inputs": [],
      "name": "advance",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 0
        }
      ]
    }
  ],
  "agreement": "Minimal",
  "diagnostics": [],
  "file": "minimal.mori",
  "language": "moriarty/1",
  "source_hash": "5014535ca4879d3a6e1580f6dc3f08dcf511309eb8757f2c9c34d7e226a80526",
  "status": "accepted"
}
```

### Case core1-rejected invocation binding

Source SHA256 `eb7a9e96829f3ef9aae077f18adee18f334c4b6ab3b34db8463a9153af99d9e8`; original filename `empty.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core1-rejected`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "empty.mori", "--json"]`; exit `1`; stdout bytes `220`, SHA256 `984922ce39818964f928d341ed8155a4438e6fce8a2fde0794c1f9bd9f8c988a`; stderr bytes `0`; START `receipts/baseline-cases/core1-rejected/START.json`; RESULT `receipts/baseline-cases/core1-rejected/RESULT.json`.

### core1-rejected baseline-cases complete stdout

`receipts/baseline-cases/core1-rejected/STDOUT.log`; complete body; SHA256 `984922ce39818964f928d341ed8155a4438e6fce8a2fde0794c1f9bd9f8c988a`.

```json
{
  "diagnostics": [
    {
      "code": "mori::core::program",
      "message": "Program needs at least one action.",
      "offset": null,
      "step": null
    }
  ],
  "file": "empty.mori",
  "status": "rejected"
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core1-rejected`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "empty.mori", "--json"]`; exit `1`; stdout bytes `220`, SHA256 `984922ce39818964f928d341ed8155a4438e6fce8a2fde0794c1f9bd9f8c988a`; stderr bytes `0`; START `receipts/changed-cases/core1-rejected/START.json`; RESULT `receipts/changed-cases/core1-rejected/RESULT.json`.

### core1-rejected changed-cases complete stdout

`receipts/changed-cases/core1-rejected/STDOUT.log`; complete body; SHA256 `984922ce39818964f928d341ed8155a4438e6fce8a2fde0794c1f9bd9f8c988a`.

```json
{
  "diagnostics": [
    {
      "code": "mori::core::program",
      "message": "Program needs at least one action.",
      "offset": null,
      "step": null
    }
  ],
  "file": "empty.mori",
  "status": "rejected"
}
```

### Case core2-accepted invocation binding

Source SHA256 `29d20136db0fbe85bc161e134a497696ef5c7e994b0787b492af0246660359dc`; original filename `minimal.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core2-accepted`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "minimal.mori", "--json"]`; exit `0`; stdout bytes `497`, SHA256 `98fbc87d6c2515a856b571b43f4a89f53fd117160582495e327e1c4b2cf76484`; stderr bytes `0`; START `receipts/baseline-cases/core2-accepted/START.json`; RESULT `receipts/baseline-cases/core2-accepted/RESULT.json`.

### core2-accepted baseline-cases complete stdout

`receipts/baseline-cases/core2-accepted/STDOUT.log`; complete body; SHA256 `98fbc87d6c2515a856b571b43f4a89f53fd117160582495e327e1c4b2cf76484`.

```json
{
  "actions": [
    {
      "inputs": [],
      "name": "advance",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 0
        }
      ]
    }
  ],
  "agreement": "Minimal",
  "authentication": "none",
  "consumption": "untracked",
  "diagnostics": [],
  "file": "minimal.mori",
  "language": "moriarty/2",
  "qualification": "local-reference-stage",
  "source_hash": "29d20136db0fbe85bc161e134a497696ef5c7e994b0787b492af0246660359dc",
  "status": "accepted"
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core2-accepted`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "minimal.mori", "--json"]`; exit `0`; stdout bytes `497`, SHA256 `98fbc87d6c2515a856b571b43f4a89f53fd117160582495e327e1c4b2cf76484`; stderr bytes `0`; START `receipts/changed-cases/core2-accepted/START.json`; RESULT `receipts/changed-cases/core2-accepted/RESULT.json`.

### core2-accepted changed-cases complete stdout

`receipts/changed-cases/core2-accepted/STDOUT.log`; complete body; SHA256 `98fbc87d6c2515a856b571b43f4a89f53fd117160582495e327e1c4b2cf76484`.

```json
{
  "actions": [
    {
      "inputs": [],
      "name": "advance",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 0
        }
      ]
    }
  ],
  "agreement": "Minimal",
  "authentication": "none",
  "consumption": "untracked",
  "diagnostics": [],
  "file": "minimal.mori",
  "language": "moriarty/2",
  "qualification": "local-reference-stage",
  "source_hash": "29d20136db0fbe85bc161e134a497696ef5c7e994b0787b492af0246660359dc",
  "status": "accepted"
}
```

### Case core2-rejected invocation binding

Source SHA256 `37d660cea5a09c6d85d9a8121a634320754cf833eee733efc208728ca5c24d19`; original filename `empty.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core2-rejected`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "empty.mori", "--json"]`; exit `1`; stdout bytes `322`, SHA256 `6e603241ee81db4aa11b2b466b1745b76f2c534dfe6b3fa380e3033195bd9ff5`; stderr bytes `0`; START `receipts/baseline-cases/core2-rejected/START.json`; RESULT `receipts/baseline-cases/core2-rejected/RESULT.json`.

### core2-rejected baseline-cases complete stdout

`receipts/baseline-cases/core2-rejected/STDOUT.log`; complete body; SHA256 `6e603241ee81db4aa11b2b466b1745b76f2c534dfe6b3fa380e3033195bd9ff5`.

```json
{
  "authentication": "none",
  "consumption": "untracked",
  "diagnostics": [
    {
      "code": "mori::core::program",
      "message": "Program needs at least one action.",
      "offset": null,
      "step": null
    }
  ],
  "file": "empty.mori",
  "qualification": "local-reference-stage",
  "status": "rejected"
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/core2-rejected`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "empty.mori", "--json"]`; exit `1`; stdout bytes `322`, SHA256 `6e603241ee81db4aa11b2b466b1745b76f2c534dfe6b3fa380e3033195bd9ff5`; stderr bytes `0`; START `receipts/changed-cases/core2-rejected/START.json`; RESULT `receipts/changed-cases/core2-rejected/RESULT.json`.

### core2-rejected changed-cases complete stdout

`receipts/changed-cases/core2-rejected/STDOUT.log`; complete body; SHA256 `6e603241ee81db4aa11b2b466b1745b76f2c534dfe6b3fa380e3033195bd9ff5`.

```json
{
  "authentication": "none",
  "consumption": "untracked",
  "diagnostics": [
    {
      "code": "mori::core::program",
      "message": "Program needs at least one action.",
      "offset": null,
      "step": null
    }
  ],
  "file": "empty.mori",
  "qualification": "local-reference-stage",
  "status": "rejected"
}
```

### Case unicode-byte-offsets invocation binding

Source SHA256 `211dc3011819cc0e18300d745edc7b9510fc4b32a74146eb11566c69bc9bf34f`; original filename `test.mori`; baseline and after complete stdout bytes/exit equal; stored golden also equals both.

Phase `baseline-cases`; head `97c9b1565d71404413aa35e9f95fe7661b8a09b1`; tree `e20de64d1e0ada6872b80ddd941857145305dfe0`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/unicode-byte-offsets`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `1`; stdout bytes `1661`, SHA256 `37475ec91dd4978b7c9ac5d5d9d426b1b0c19b8cf4a1a0f912a45dbecf248320`; stderr bytes `0`; START `receipts/baseline-cases/unicode-byte-offsets/START.json`; RESULT `receipts/baseline-cases/unicode-byte-offsets/RESULT.json`.

### unicode-byte-offsets baseline-cases complete stdout

`receipts/baseline-cases/unicode-byte-offsets/STDOUT.log`; complete body; SHA256 `37475ec91dd4978b7c9ac5d5d9d426b1b0c19b8cf4a1a0f912a45dbecf248320`.

```json
{
  "status": "rejected",
  "file": "test.mori",
  "source_hash": "211dc3011819cc0e18300d745edc7b9510fc4b32a74146eb11566c69bc9bf34f",
  "agreement": {
    "name": "Invoice",
    "span": {
      "start": 46,
      "end": 53
    }
  },
  "declarations": [
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 58,
        "end": 75
      },
      "name_span": {
        "start": 64,
        "end": 69
      }
    },
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 78,
        "end": 98
      },
      "name_span": {
        "start": 84,
        "end": 89
      }
    }
  ],
  "actions": [],
  "diagnostics": [
    {
      "code": "mori::check::duplicate_name",
      "severity": "error",
      "message": "`price` is declared twice.",
      "help": "Each declaration and action in an agreement needs its own name.",
      "labels": [
        {
          "span": {
            "start": 64,
            "end": 69
          },
          "text": "first declared here",
          "primary": false
        },
        {
          "span": {
            "start": 84,
            "end": 89
          },
          "text": "declared again here",
          "primary": true
        }
      ]
    },
    {
      "code": "mori::check::unknown_name",
      "severity": "error",
      "message": "I cannot find a declaration named `pirce`.",
      "help": "Did you mean `price`?\n\n    const price = price;",
      "labels": [
        {
          "span": {
            "start": 92,
            "end": 97
          },
          "text": "not declared above",
          "primary": true
        }
      ]
    }
  ]
}
```

Phase `changed-cases`; head `823ce081c00786a0c55b92af81009f9e7c5e6a16`; tree `aa593783a8717ac838ae556e24c187997ab3547e`; cwd `/tmp/doctorwatson-shared-check-qn8ptfim/cwd/unicode-byte-offsets`; actual argv `["/usr/bin/timeout", "--signal=TERM", "--kill-after=5s", "10s", "/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/mori", "--color", "never", "check", "test.mori", "--json"]`; exit `1`; stdout bytes `1661`, SHA256 `37475ec91dd4978b7c9ac5d5d9d426b1b0c19b8cf4a1a0f912a45dbecf248320`; stderr bytes `0`; START `receipts/changed-cases/unicode-byte-offsets/START.json`; RESULT `receipts/changed-cases/unicode-byte-offsets/RESULT.json`.

### unicode-byte-offsets changed-cases complete stdout

`receipts/changed-cases/unicode-byte-offsets/STDOUT.log`; complete body; SHA256 `37475ec91dd4978b7c9ac5d5d9d426b1b0c19b8cf4a1a0f912a45dbecf248320`.

```json
{
  "status": "rejected",
  "file": "test.mori",
  "source_hash": "211dc3011819cc0e18300d745edc7b9510fc4b32a74146eb11566c69bc9bf34f",
  "agreement": {
    "name": "Invoice",
    "span": {
      "start": 46,
      "end": 53
    }
  },
  "declarations": [
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 58,
        "end": 75
      },
      "name_span": {
        "start": 64,
        "end": 69
      }
    },
    {
      "kind": "const",
      "name": "price",
      "span": {
        "start": 78,
        "end": 98
      },
      "name_span": {
        "start": 84,
        "end": 89
      }
    }
  ],
  "actions": [],
  "diagnostics": [
    {
      "code": "mori::check::duplicate_name",
      "severity": "error",
      "message": "`price` is declared twice.",
      "help": "Each declaration and action in an agreement needs its own name.",
      "labels": [
        {
          "span": {
            "start": 64,
            "end": 69
          },
          "text": "first declared here",
          "primary": false
        },
        {
          "span": {
            "start": 84,
            "end": 89
          },
          "text": "declared again here",
          "primary": true
        }
      ]
    },
    {
      "code": "mori::check::unknown_name",
      "severity": "error",
      "message": "I cannot find a declaration named `pirce`.",
      "help": "Did you mean `price`?\n\n    const price = price;",
      "labels": [
        {
          "span": {
            "start": 92,
            "end": 97
          },
          "text": "not declared above",
          "primary": true
        }
      ]
    }
  ]
}
```

## Exact scoped execution receipts, with no hidden test filter

### BASELINE-PARENT-DELTA.json

`receipts/BASELINE-PARENT-DELTA.json`; complete body; SHA256 `193b7367ddffc171ced3965145b815e29ae829f44864401365b3071819c16fc5`.

```json
{
  "recordedAt": "2026-10-18T12:57:08.005054+00:00",
  "baselineCommit": "97c9b1565d71404413aa35e9f95fe7661b8a09b1",
  "changedCandidateParent": "e4d7a2b5709caa18a94f5256d03a0302364b8235",
  "droppedTestOnlyPaths": [
    "crates/fixtures/financial-pair/actus-ump01/call-operands.json",
    "crates/fixtures/financial-pair/actus-ump01/independent/ACTUS-FINANCIAL-SCHEDULE.expected.json",
    "crates/fixtures/financial-pair/actus-ump01/independent/APPLICABILITY-AND-EXPOSURE.json",
    "crates/fixtures/financial-pair/actus-ump01/independent/CORE-FINANCIAL-WORK.expected.json",
    "crates/fixtures/financial-pair/actus-ump01/independent/FREEZE.json",
    "crates/fixtures/financial-pair/actus-ump01/independent/FREEZE.sha256",
    "crates/fixtures/financial-pair/actus-ump01/independent/INPUT-ONLY-MANIFEST.json",
    "crates/fixtures/financial-pair/actus-ump01/normative/LICENSE.md",
    "crates/fixtures/financial-pair/actus-ump01/normative/actus-tests-ump.json",
    "crates/fixtures/financial-pair/actus-ump01/opening.scenario.json",
    "crates/fixtures/financial-pair/actus-ump01/provenance.json",
    "crates/fixtures/financial-pair/actus-ump01/source.mori",
    "crates/fixtures/financial-pair/actus-ump01/subject-bindings.json",
    "crates/fixtures/financial-pair/actus-ump01/ump01.normative-operands.json",
    "crates/mori-core/tests/financial_pair.rs"
  ],
  "productionAndDependencyDelta": [],
  "precheckFailure": "Initial broad all-crates guard incorrectly expected unrelated UMP test-only baseline paths retained after branch reset; actual production closure is identical. No changed-source build/test executed.",
  "correction": "Exclude exact15 known test-only parentdelta paths, retain fullbaselinepins andverifyallothercurrentpaths. No sourcepatch/testexpectation changes, coldtarget retained."
}
```

### FINAL-RESULT.json

`receipts/FINAL-RESULT.json`; complete body; SHA256 `50bae4b1a879db1cea0446accb0a39e5789bda1cabdf7d7198b3b494a1f288e3`.

```json
{
  "finishedAt": "2026-10-18T12:57:30.900111+00:00",
  "head": "823ce081c00786a0c55b92af81009f9e7c5e6a16",
  "tree": "aa593783a8717ac838ae556e24c187997ab3547e",
  "tests": {
    "finishedAt": "2026-10-18T12:57:30.734539+00:00",
    "exitCode": 0,
    "wallSeconds": 22.639857841997582,
    "stdoutSha256": "be8a4c9b07459b55385648875af8bc7a716569cfd40f4a89d306e0c23646d9dd",
    "stderrSha256": "cb750164494ce2c0f27db1e3e98f171fb71a786f3873c64d6bdc7978ec123274",
    "ownedSessionRemaining": []
  },
  "directObservations": [
    {
      "name": "accepted",
      "finishedAt": "2026-10-18T12:57:30.743444+00:00",
      "exitCode": 0,
      "wallSeconds": 0.008079221996013075,
      "stdoutSha256": "e0443b69cbbbcd432f4ed097d06e7a368b398331a7c579210ef2a8382c926d85",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "rejected",
      "finishedAt": "2026-10-18T12:57:30.751109+00:00",
      "exitCode": 1,
      "wallSeconds": 0.007193192002887372,
      "stdoutSha256": "674b78dd4cb550144971bb16fbf08b476413956573481e0ae6cce47ed15da83e",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "unparsable",
      "finishedAt": "2026-10-18T12:57:30.758913+00:00",
      "exitCode": 1,
      "wallSeconds": 0.007285970998054836,
      "stdoutSha256": "295ad8827179e74236cc6dd4392afc76b143062f370d9095bc49cbcada90faf6",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "core1-accepted",
      "finishedAt": "2026-10-18T12:57:30.767028+00:00",
      "exitCode": 0,
      "wallSeconds": 0.0076542059978237376,
      "stdoutSha256": "dc3bdde91774cdb2e92dcfdf962c91560044f5b3a19d99612b4cc7be648cc442",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "core1-rejected",
      "finishedAt": "2026-10-18T12:57:30.774356+00:00",
      "exitCode": 1,
      "wallSeconds": 0.006822285999078304,
      "stdoutSha256": "984922ce39818964f928d341ed8155a4438e6fce8a2fde0794c1f9bd9f8c988a",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "core2-accepted",
      "finishedAt": "2026-10-18T12:57:30.782135+00:00",
      "exitCode": 0,
      "wallSeconds": 0.007336448004934937,
      "stdoutSha256": "98fbc87d6c2515a856b571b43f4a89f53fd117160582495e327e1c4b2cf76484",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "core2-rejected",
      "finishedAt": "2026-10-18T12:57:30.790092+00:00",
      "exitCode": 1,
      "wallSeconds": 0.0074867360017378815,
      "stdoutSha256": "6e603241ee81db4aa11b2b466b1745b76f2c534dfe6b3fa380e3033195bd9ff5",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    },
    {
      "name": "unicode-byte-offsets",
      "finishedAt": "2026-10-18T12:57:30.799514+00:00",
      "exitCode": 1,
      "wallSeconds": 0.008530517996405251,
      "stdoutSha256": "37475ec91dd4978b7c9ac5d5d9d426b1b0c19b8cf4a1a0f912a45dbecf248320",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "ownedSessionRemaining": [],
      "sameWholeStdoutBytesAsActualBaseline": true,
      "sameExitAsActualBaseline": true,
      "stderrEmpty": true
    }
  ],
  "allEightDirectCasesCompleted": true,
  "ownedTemporaryRoot": "/tmp/doctorwatson-shared-check-qn8ptfim",
  "ownedRootExistsAfter": false,
  "scope": "One8-testpublicCLI suite (eachJSON+quiethuman),8directwholebaseline comparisons. No MCPserver/SDK/native/proof/ledger/full-languageclosure."
}
```

### changed-public-tests/START.json

`receipts/changed-public-tests/START.json`; complete body; SHA256 `f2acd4357f52ee7579133a63bec55f1bac217940c2a422a35ae11f372e30c74b`.

```json
{
  "startedAt": "2026-10-18T12:57:08.094642+00:00",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "check_source_dispatch",
    "--",
    "--test-threads=1"
  ],
  "cwd": "/tmp/doctorwatson-shared-check-qn8ptfim/cwd",
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-shared-check-qn8ptfim/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-shared-check-qn8ptfim/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-shared-check-qn8ptfim/tmp"
  },
  "head": "823ce081c00786a0c55b92af81009f9e7c5e6a16",
  "tree": "aa593783a8717ac838ae556e24c187997ab3547e",
  "attempt": 1
}
```

### changed-public-tests/RESULT.json

`receipts/changed-public-tests/RESULT.json`; complete body; SHA256 `7266c83ebc90299d1dd1501dd992f6975f24bdab0ecce93ee73a570ac967830d`.

```json
{
  "finishedAt": "2026-10-18T12:57:30.734539+00:00",
  "exitCode": 0,
  "wallSeconds": 22.639857841997582,
  "stdoutSha256": "be8a4c9b07459b55385648875af8bc7a716569cfd40f4a89d306e0c23646d9dd",
  "stderrSha256": "cb750164494ce2c0f27db1e3e98f171fb71a786f3873c64d6bdc7978ec123274",
  "ownedSessionRemaining": []
}
```

### changed-public-tests/STDOUT.log

`receipts/changed-public-tests/STDOUT.log`; complete body; SHA256 `be8a4c9b07459b55385648875af8bc7a716569cfd40f4a89d306e0c23646d9dd`.

```text

running 8 tests
test beta_accepted ... ok
test beta_parse_error ... ok
test beta_rejected ... ok
test beta_unicode_byte_offsets ... ok
test core1_accepted ... ok
test core1_rejected ... ok
test core2_accepted ... ok
test core2_rejected ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

```

### changed-public-tests/STDERR.log

`receipts/changed-public-tests/STDERR.log`; complete body; SHA256 `cb750164494ce2c0f27db1e3e98f171fb71a786f3873c64d6bdc7978ec123274`.

```text
   Compiling libc v0.2.190
   Compiling rustix v1.1.5
   Compiling terminal_size v0.4.4
   Compiling getrandom v0.2.17
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-span)
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-diagnostics)
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-lexer)
   Compiling ff v0.13.1
   Compiling group v0.13.0
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling elliptic-curve v0.13.8
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-ast)
   Compiling signature v2.2.0
   Compiling getrandom v0.4.3
   Compiling ecdsa v0.16.9
   Compiling similar v2.7.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-parser)
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-core)
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-partial-delivery-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 22.58s
     Running tests/check_source_dispatch.rs (/tmp/doctorwatson-shared-check-qn8ptfim/target/debug/deps/check_source_dispatch-51bc4c5be4331471)
```

## Pinned unmodified context

All context below is at the exact review head and unchanged from E4D, verified against pinned Git bodies. Full file hashes identify excerpts; quoted line numbers use original source lines. This is selected dependency/interface context, not an independent audit of every implementation internals.

### crates/mori/src/json.rs

`context/crates/mori/src/json.rs`; original lines [(1, 199)]; SHA256 `24c66e87d335afd2ab4913c70868944a78420ed1410cecbd81fc3abc047b20bb`.

```rust
//! The `--json` report of `mori check`.
//!
//! Spans are UTF-8 byte ranges into the checked file. This format is our own;
//! it is not the TypeScript beta's.

use mori_ast::{Ast, NodeIdx};
use mori_checker::{Checked, DeclIdx, Support};
use mori_diagnostics::{Diagnostic, MoriDiagnostic, Severity};
use mori_lexer::TokenIdx;
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Serialize)]
pub struct CheckReport {
    /// `accepted` when there are no diagnostics, otherwise `rejected`.
    pub status: &'static str,
    pub file: String,
    /// SHA-256 of the file's bytes, in lowercase hex.
    pub source_hash: String,
    /// Absent when the file did not parse.
    pub agreement: Option<Named>,
    pub declarations: Vec<DeclarationReport>,
    pub actions: Vec<ActionReport>,
    pub diagnostics: Vec<DiagnosticReport>,
}

#[derive(Serialize)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl From<mori_span::Span> for Span {
    fn from(span: mori_span::Span) -> Self {
        Self {
            start: span.start,
            end: span.end,
        }
    }
}

#[derive(Serialize)]
pub struct Named {
    pub name: String,
    pub span: Span,
}

#[derive(Serialize)]
pub struct DeclarationReport {
    /// The declaring keyword, such as `asset` or `intent`.
    pub kind: String,
    pub name: String,
    pub span: Span,
    pub name_span: Span,
}

#[derive(Serialize)]
pub struct ActionReport {
    pub name: String,
    pub intent: String,
    /// `local_s0` when it runs locally, `specified_only` when it is only
    /// checked; absent when its intent did not check.
    pub support: Option<&'static str>,
    pub span: Span,
    pub name_span: Span,
}

#[derive(Serialize)]
pub struct DiagnosticReport {
    pub code: Option<String>,
    pub severity: &'static str,
    pub message: String,
    pub help: Option<String>,
    pub labels: Vec<LabelReport>,
}

#[derive(Serialize)]
pub struct LabelReport {
    pub span: Span,
    pub text: Option<String>,
    pub primary: bool,
}

impl CheckReport {
    pub fn new(
        file: String,
        source: &str,
        checked: Option<&Checked<'_>>,
        diagnostics: &[MoriDiagnostic],
    ) -> Self {
        let source_hash = Sha256::digest(source.as_bytes())
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        let mut report = Self {
            status: if diagnostics.is_empty() {
                "accepted"
            } else {
                "rejected"
            },
            file,
            source_hash,
            agreement: None,
            declarations: Vec::new(),
            actions: Vec::new(),
            diagnostics: diagnostics.iter().map(DiagnosticReport::new).collect(),
        };
        if let Some(checked) = checked {
            report.fill(checked);
        }
        report
    }

    fn fill(&mut self, checked: &Checked<'_>) {
        let ast = checked.ast;
        let name_token = ast.main_token(NodeIdx::ROOT);
        self.agreement = Some(Named {
            name: ast.token_text(name_token).to_owned(),
            span: token_span(ast, name_token).into(),
        });
        for index in 0..checked.declarations.len() {
            let decl = DeclIdx::new(index);
            let node = checked.declarations.node()[index];
            let keyword = ast.main_token(node);
            self.declarations.push(DeclarationReport {
                kind: ast.token_text(keyword).to_owned(),
                name: checked.name(decl).to_owned(),
                span: ast.span(node).into(),
                name_span: token_span(ast, ast.token_after(keyword, 1)).into(),
            });
        }
        for (&node, &support) in checked.actions.node().iter().zip(checked.actions.support()) {
            let keyword = ast.main_token(node);
            self.actions.push(ActionReport {
                name: ast.token_text(ast.token_after(keyword, 1)).to_owned(),
                intent: ast.token_text(ast.token_after(keyword, 3)).to_owned(),
                support: support.map(|support| match support {
                    Support::LocalS0 => "local_s0",
                    Support::SpecifiedOnly => "specified_only",
                }),
                span: ast.span(node).into(),
                name_span: token_span(ast, ast.token_after(keyword, 1)).into(),
            });
        }
    }
}

impl DiagnosticReport {
    fn new(diagnostic: &MoriDiagnostic) -> Self {
        let severity = match diagnostic.severity() {
            Some(Severity::Warning) => "warning",
            Some(Severity::Advice) => "advice",
            _ => "error",
        };
        let mut labels: Vec<LabelReport> = diagnostic
            .labels()
            .map(|labels| {
                labels
                    .map(|label| LabelReport {
                        span: Span {
                            start: to_u32(label.offset()),
                            end: to_u32(label.offset() + label.len()),
                        },
                        text: label.label().map(str::to_owned),
                        primary: label.primary(),
                    })
                    .collect()
            })
            .unwrap_or_default();
        // Every diagnostic reports one primary location: the marked label, or
        // else the first.
        if !labels.iter().any(|label| label.primary)
            && let Some(first) = labels.first_mut()
        {
            first.primary = true;
        }
        Self {
            code: diagnostic.code().map(|code| code.to_string()),
            severity,
            message: diagnostic.to_string(),
            help: diagnostic.help().map(|help| help.to_string()),
            labels,
        }
    }
}

fn token_span(ast: &Ast<'_>, token: TokenIdx) -> mori_span::Span {
    ast.tokens.span(token, ast.source)
}

/// Sources are at most 65,536 bytes, so offsets fit in `u32`.
fn to_u32(offset: usize) -> u32 {
    u32::try_from(offset).expect("source offsets fit in u32")
}

#[cfg(test)]
mod tests {
    use super::CheckReport;

```

### crates/mori/src/commands/core_io.rs

`context/crates/mori/src/commands/core_io.rs`; original lines [(1, 22), (247, 251)]; SHA256 `345be5e8b1b5ba4560b449e1a64cf0a990d54f0d4216261e35b816498ec948b4`.

```rust
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
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}
```

### crates/mori/src/commands/simulate.rs

`context/crates/mori/src/commands/simulate.rs`; original lines [(150, 159)]; SHA256 `970ea5a7b4a254a2771be8baeb9d0880ad6ac88e2aec141e520b0cca82ed0cc9`.

```rust
/// Render the same explicit qualification on every staged public object.
pub(super) fn print_stage_report(mut report: serde_json::Value) -> super::Result<()> {
    let object = report
        .as_object_mut()
        .expect("stage reports are JSON objects");
    for (name, value) in mori_core::stage::STAGE_LABELS {
        object.insert(name.into(), serde_json::Value::String(value.into()));
    }
    super::core_io::print(&report)
}
```

### crates/mori/src/shell.rs

`context/crates/mori/src/shell.rs`; original lines [(21, 52), (68, 79), (99, 108)]; SHA256 `91efa092ff90c7d7b4684e874ead792c8d461122ac7b3d451a27d8660cffaf8f`.

```rust
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
    fn aligned(&mut self, verb: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{verb:>VERB_WIDTH$}{style:#} {message}");
    }

    fn prefixed(&mut self, label: &str, style: Style, message: impl Display) {
        let message = crate::display::text(&message.to_string());
        let _ = writeln!(self.err, "{style}{label}:{style:#} {message}");
    }
}
```

### crates/mori/src/cli.rs

`context/crates/mori/src/cli.rs`; original lines [(1, 64), (122, 129)]; SHA256 `684db0ba78f79c470e0c20aeab62d18189283457ce36c30b488827ff68eebc4e`.

```rust
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
    keygen_signed_call, lsp, mcp, prepare_call, prepare_signed_call, prove, prove_call,
    prove_signed_call, replay, simulate, test, verify, verify_call, verify_intent,
    verify_signed_call,
};
use crate::shell::Verbosity;
use crate::styles;

/// Moriarty: a financial agreement language for Midnight.
#[derive(Debug, Parser)]
#[command(name = "mori", version, styles = styles::CLAP)]
#[cfg_attr(
    all(not(feature = "native"), not(feature = "source6-expansion")),
    command(
        after_help = "Built without native: local Core reference tooling only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
#[cfg_attr(
    all(not(feature = "native"), feature = "source6-expansion"),
    command(
        after_help = "Built without native: local Core reference tooling and explicit Source/6 transfer expansion only. Midnight compile, call, proof, targeted inspect, trace/replay and JubJub profiles require the native feature."
    )
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalArgs,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Args)]
#[command(next_help_heading = "Global Options")]
pub struct GlobalArgs {
    /// Print no status output
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    /// Print extra status output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// When to use color
    #[arg(long, global = true, value_name = "WHEN", default_value = "auto")]
    pub color: Color,
}

impl GlobalArgs {
    pub fn verbosity(&self) -> Verbosity {
        if self.quiet {
            Verbosity::Quiet
        } else if self.verbose {
            Verbosity::Verbose
        } else {
            Verbosity::Normal
        }
    }
}

    /// Compile a bounded Moriarty program into ledger-9 ZKIRv3 circuits
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Compile(compile::Args),
```

### crates/mori-checker/src/lib.rs

`context/crates/mori-checker/src/lib.rs`; original lines [(33, 48), (61, 92)]; SHA256 `86192cf5cafcb86a2b6f0a44366ead90c1f79abaa0a107e2835cf793ead1f0e2`.

```rust

/// Everything [`check`] could check, and every error it found. The program
/// is accepted only when there are no diagnostics.
pub struct CheckResult<'a> {
    pub checked: Checked<'a>,
    pub diagnostics: Vec<MoriDiagnostic>,
}

impl CheckResult<'_> {
    pub fn is_accepted(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

/// Index of a declaration in [`Checked`]. An agreement has at most 256 items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A declaration, in source order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Declaration {
    pub node: NodeIdx,
    /// It has an error, or depends on a declaration that does.
    pub failed: bool,
    /// Its value, unless it failed.
    pub value: Option<ValueIdx>,
}

/// An action, in source order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Action {
    pub node: NodeIdx,
    /// The intent it uses, when that resolved to an intent declaration.
    pub intent: Option<DeclIdx>,
    /// What that intent can do, when it was checked.
    pub support: Option<Support>,
}

/// The checked program, as side tables over the [`Ast`].
pub struct Checked<'a> {
    pub ast: &'a Ast<'a>,
    pub declarations: Soa<Declaration>,
    pub actions: Soa<Action>,
    pub values: Values,
    /// For each node, the declaration it names: a reference, a quantity's
    /// asset or an action's intent. [`UNRESOLVED`] otherwise.
    resolutions: Vec<u16>,
}
```

### crates/mori-checker/src/checker.rs

`context/crates/mori-checker/src/checker.rs`; original lines [(1, 44)]; SHA256 `91c07c8be439fa5eb2f8d7659cc41a0db6cc9712509cf6d726c6ee8c7659a209`.

```rust
//! The ordered walk over the agreement's items.

use std::collections::HashMap;

use mori_ast::{Ast, NodeIdx, NodeTag};
use mori_diagnostics::{Diagnostic, MoriDiagnostic, suggest};
use mori_lexer::{TokenIdx, TokenKind};
use mori_span::Span;
use soa_rs::Soa;

use crate::declarations::Identities;
use crate::names::{Names, name_token};
use crate::reserved::SOURCE6_RESERVED;
use crate::values::{ValueIdx, ValueTag, Values};
use crate::{Action, CheckResult, Checked, DeclIdx, Declaration, UNRESOLVED, diagnostics};

/// Checks `ast`, returning everything checked and every error found.
pub fn check<'a>(ast: &'a Ast<'a>) -> CheckResult<'a> {
    let items: Vec<NodeIdx> = ast.list(NodeIdx::ROOT).collect();
    let mut checker = Checker {
        ast,
        names: Names::default(),
        declarations: Soa::with_capacity(items.len()),
        actions: Soa::new(),
        values: Values::default(),
        resolutions: vec![UNRESOLVED; ast.nodes.len()],
        visible: HashMap::new(),
        identities: Identities::new(),
        diagnostics: Vec::new(),
    };
    checker.run(&items);
    // Name collection finds duplicates before the walk; report in source order.
    checker.diagnostics.sort_by_key(position);
    CheckResult {
        checked: Checked {
            ast,
            declarations: checker.declarations,
            actions: checker.actions,
            values: checker.values,
            resolutions: checker.resolutions,
        },
        diagnostics: checker.diagnostics,
    }
}
```

### crates/mori-parser/src/lib.rs

`context/crates/mori-parser/src/lib.rs`; complete body; SHA256 `3e03133719667b25827261a8d5ac1cf6995b0365057e1c9ef89c597acef67b99`.

```rust
//! Parser for `moriarty-beta/1` source.
//!
//! [`parse`] lexes and parses a file into an [`Ast`], stopping at the first
//! error. It checks syntax and syntactic bounds only; names, types and values
//! are checked by later phases.

mod diagnostics;
mod parser;

#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use mori_ast::Ast;
use mori_diagnostics::MoriDiagnostic;

/// The only profile this parser accepts.
pub const PROFILE: &str = "moriarty-beta/1";
/// Deepest nesting of expressions and types.
pub const MAX_DEPTH: usize = 64;
/// Most fields in one record or call, and most type arguments.
pub const MAX_FIELDS: usize = 64;
/// Most declarations and actions in one agreement.
pub const MAX_ITEMS: usize = 256;

/// Lexes and parses `source`, or reports the first error.
pub fn parse(source: &str) -> Result<Ast<'_>, MoriDiagnostic> {
    let tokens = mori_lexer::lex(source)?;
    let (nodes, extra) = parser::Parser::new(source, &tokens).run()?;
    Ok(Ast {
        source,
        tokens,
        nodes,
        extra,
    })
}
```

### crates/mori-core/src/json.rs

`context/crates/mori-core/src/json.rs`; original lines [(117, 140)]; SHA256 `c9b49504c5c05b1d7b63bb9aa30527516a0ae0920da3e10ea2cfa2a8b08d5a6c`.

```rust
pub fn export(value: &impl serde::Serialize) -> Result<String, Error> {
    let json =
        serde_json::to_string_pretty(value).map_err(|e| Error::new("json", e.to_string()))?;
    parse(&json)?;
    let mut display = String::with_capacity(json.len());
    for character in json.chars() {
        if character.is_ascii() && (!character.is_control() || character == '\n') {
            display.push(character);
        } else {
            for unit in character.encode_utf16(&mut [0; 2]) {
                use std::fmt::Write;
                write!(&mut display, "\\u{unit:04x}").expect("writing to String cannot fail");
            }
        }
        if display.len() >= MAX_BYTES {
            return Err(Error::new(
                "json",
                "Escaped JSON and final newline exceed 1 MiB",
            ));
        }
    }
    parse(&display)?;
    Ok(display)
}
```

### crates/mori-core/src/stage/mod.rs

`context/crates/mori-core/src/stage/mod.rs`; original lines [(27, 94)]; SHA256 `0d902d84047cb134d02bb0ecdd5c1c783b66e4570dcf44ce09cd16ad463169c8`.

```rust
pub const LANGUAGE_2: &str = "moriarty/2";
pub const SEMANTICS_2: &str = "moriarty-core/2";
pub const STAGE_PLAN_FORMAT: &str = "moriarty-stage-plan/1";
pub const CONTINUATION_FORMAT: &str = "moriarty-continuation/1";
pub const STAGE_EVENT_FORMAT: &str = "moriarty-stage-event/1";
pub const MAX_AWAITS_PER_ACTION: usize = 16;
pub const MAX_EVENT_BINDERS: usize = 16;
pub const MAX_CAPTURE_SLOTS: usize = 256;
pub const MAX_CONTINUATION_BYTES: usize = 4 * 1_048_576;
pub const MAX_EVENT_BYTES: usize = 65_536;
/// Inserted verbatim into every `/2` output object.
pub const STAGE_LABELS: [(&str, &str); 3] = [
    ("qualification", "local-reference-stage"),
    ("authentication", "none"),
    ("consumption", "untracked"),
];

/// A `/2` program that passed profile, validation, plan invariants and static
/// costing, with its canonical digests. Only `load` constructs one; there is
/// no public constructor, `Default` or `Deserialize`.
#[derive(Debug, Clone)]
pub struct StagedProgram {
    program: Program,
    plan: StagePlan,
    program_hash: [u8; 32],
    plan_hash: [u8; 32],
}
impl StagedProgram {
    pub fn program(&self) -> &Program {
        &self.program
    }
    pub fn plan(&self) -> &StagePlan {
        &self.plan
    }
    pub fn program_hash(&self) -> [u8; 32] {
        self.program_hash
    }
    pub fn plan_hash(&self) -> [u8; 32] {
        self.plan_hash
    }
    /// Declaration index of an action; plan and program actions share it.
    pub(crate) fn action_index(&self, name: &str) -> Option<usize> {
        self.program.actions.iter().position(|a| a.name == name)
    }
}

/// Parses, admits and plans a `moriarty/2` source. Legacy `parse`/`validate`
/// remain `/1`-only, so a `/2` source fails every legacy consumer.
pub fn load(src: &str) -> Result<StagedProgram, Error> {
    let program = crate::parser::parse_v2(src)?;
    profile::require_core2(&program)?;
    validate::validate_v2(&program)?;
    let mut plan = plan::build(&program)?;
    validate::plan_invariants(&program, &plan)?;
    cost::fill(&program, &mut plan)?;
    validate::cost_invariants(&program, &plan)?;
    let program_hash = digest::program_hash(&program)?;
    plan.program_hash = digest::hex(&program_hash);
    let plan_hash = digest::plan_hash(&plan)?;
    Ok(StagedProgram {
        program,
        plan,
        program_hash,
        plan_hash,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
```

### crates/mori-core/src/lib.rs

`context/crates/mori-core/src/lib.rs`; complete body; SHA256 `b33afe9a9f7a32d0c4b10211fa139bca5d1a8741ba61ee4df8acebbcd5f8304a`.

```rust
//! Bounded composable Moriarty language and deterministic local semantics.
pub mod intent;
mod ir;
pub mod json;
mod logical_cost;
mod parser;
pub mod policy;
pub mod sherlock;

// Unregistered draft source capability: Core-only ordinary host test proposal.
// These lexer-dependent modules stay outside Sherlock's standalone Std graph.
mod replay;
mod runtime;
#[cfg(test)]
#[path = "sherlock/draft_source.rs"]
mod sherlock_draft;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge.rs"]
mod sherlock_draft_image;
#[cfg(test)]
#[path = "sherlock/draft_image_bridge_tests.rs"]
mod sherlock_draft_image_tests;
#[cfg(test)]
#[path = "sherlock/draft_source_tests.rs"]
mod sherlock_draft_tests;
#[cfg(test)]
#[path = "sherlock/draft_typecheck.rs"]
mod sherlock_draft_types;
#[path = "sherlock/source_adapter.rs"]
pub mod sherlock_source;
// Caller-bounded scalar diagnostic proposal; H1 qualification is explicit in its API.
#[path = "sherlock/bounded_scalar_source.rs"]
pub mod sherlock_bounded_source;
mod source_sponsor;
pub mod stage;
pub use ir::*;
pub use logical_cost::*;
pub use parser::{language, parse};
pub use replay::{DetailedReplayError, ReplayDifference, compare_execution, replay_detailed};
pub use runtime::{replay, simulate, validate};
pub use source_sponsor::*;
```

### crates/mori/src/commands/mcp.rs

`context/crates/mori/src/commands/mcp.rs`; complete body; SHA256 `778c04c0168fd45efc8823a4f329f6b332f8808045b58d4fbccb58f279119756`.

```rust
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {}

pub fn run(_args: Args, _shell: &mut Shell) -> super::Result<ExitCode> {
    todo!("mori mcp")
}
```

### crates/mori/Cargo.toml

`context/crates/mori/Cargo.toml`; complete body; SHA256 `2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b`.

```toml
[package]
name = "mori"
version = "0.1.0"
description = "Command-line interface for the Moriarty toolchain"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true

[features]
core2-disjoint-draft = ["mori-core/core2-disjoint-draft"]
default = ["native"]
source6-expansion = ["dep:mori-source6", "mori-source6/source6-transfer-slice"]
native = [
    "dep:mori-backend",
    "dep:mori-ledger",
    "dep:futures",
    "dep:rand",
    "dep:midnight-ledger",
    "dep:midnight-base-crypto",
    "dep:midnight-coin-structure",
    "dep:midnight-onchain-state",
    "dep:midnight-onchain-runtime",
    "dep:midnight-storage",
    "dep:midnight-transient-crypto",
    "dep:midnight-serialize",
]

[dependencies]
anstream.workspace = true
anstyle.workspace = true
clap.workspace = true
ignore.workspace = true
mori-ast.workspace = true
mori-checker.workspace = true
mori-core.workspace = true
mori-backend = { workspace = true, optional = true }
mori-ledger = { workspace = true, optional = true }
mori-diagnostics = { workspace = true, features = ["fancy"] }
mori-fmt.workspace = true
mori-lexer.workspace = true
mori-parser.workspace = true
mori-source6 = { workspace = true, optional = true }
mori-span.workspace = true
rayon.workspace = true
futures = { workspace = true, optional = true }
rand = { workspace = true, optional = true }
serde.workspace = true
serde_json.workspace = true
sha2.workspace = true
terminal_size.workspace = true
thiserror.workspace = true

midnight-ledger = { workspace = true, optional = true }
midnight-base-crypto = { workspace = true, optional = true }
midnight-coin-structure = { workspace = true, optional = true }
midnight-onchain-state = { workspace = true, optional = true }
midnight-onchain-runtime = { workspace = true, optional = true }
midnight-storage = { workspace = true, optional = true }
midnight-transient-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
indoc.workspace = true
insta.workspace = true

[[test]]
name = "check_compile_io"
required-features = ["native"]

[[test]]
name = "check_static_compile_io"
required-features = ["native"]

[[test]]
name = "prepare_call"
required-features = ["native"]

[[test]]
name = "installed_call"
required-features = ["native"]

[[test]]
name = "signed_installed_call"
required-features = ["native"]

[[test]]
name = "native_proofs"
required-features = ["native"]

[[test]]
name = "signed_intents"
required-features = ["native"]

[[test]]
name = "replay"
required-features = ["native"]

[[test]]
name = "replay_challenge"
required-features = ["native"]

[[test]]
name = "enforcement_report"
required-features = ["native"]

[[test]]
name = "mapped_role_static_io"
required-features = ["native"]

[[test]]
name = "source6_expand"
required-features = ["source6-expansion"]
```

### Cargo.toml

`context/Cargo.toml`; original lines [(1, 24), (53, 61)]; SHA256 `8231895f5cb28130386a293fa08f823ff70d254f798b1b966d3be82e07db1afc`.

```toml
[workspace]
resolver = "3"
members = ["crates/*"]
# Standalone experiment and evidence crates keep their own builds, and
# crates/fixtures holds shared test programs, not a crate.
exclude = ["experiments", "raw", "wiki-llm", "deliverables", "crates/fixtures"]

[workspace.package]
edition = "2024"
rust-version = "1.99"
license = "Apache-2.0"
repository = "https://github.com/CharlesHoskinson/Moriarty"

[workspace.dependencies]
mori-ast = { path = "crates/mori-ast" }
mori-backend = { path = "crates/mori-backend" }
mori-checker = { path = "crates/mori-checker" }
mori-core = { path = "crates/mori-core" }
mori-diagnostics = { path = "crates/mori-diagnostics" }
mori-fmt = { path = "crates/mori-fmt" }
mori-ledger = { path = "crates/mori-ledger" }
mori-lexer = { path = "crates/mori-lexer" }
mori-parser = { path = "crates/mori-parser" }
mori-source6 = { path = "crates/mori-source6" }
rand = "=0.8.8"
rayon = "1.12.0"
serde = { version = "1.0.229", features = ["derive"] }
serde_json = "1.0.151"
sha2 = "0.11.0"
similar = "2.7.0"
soa-rs = "1.0.1"
terminal_size = "0.4.4"
thiserror = "2.0.21"
```

### RUST_GUIDELINES.md

`context/RUST_GUIDELINES.md`; original lines [(77, 114), (274, 292)]; SHA256 `662c71e3c10cea9f00b65c74751eeb9504dfddc2601082ad8ec9a534267d1894`.

```text
  reformat changes it. Hash the produced artifact or a declared version.
- No absolute paths. Locate files from `env!("CARGO_MANIFEST_DIR")`.

## Checks

- Document and run these local checks only within their inspected scope and
  existing execution authority:

  ```
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --locked -- -D warnings
  cargo test --workspace --locked
  ```
- `mori-core`'s Midnight-native compatibility test is gated behind the
  non-default `midnight-compat-tests` feature, so ordinary `mori-core` test
  selection is free of Midnight dependencies, for example
  `cargo test -p mori-core` or
  `cargo test --frozen -j1 -p mori-core --no-default-features --test continuations -- --test-threads=1`.
  The `cargo test --workspace --locked` above does not enable that feature, so
  run the compatibility test explicitly for full coverage (it builds the pinned
  Midnight SDK closure):

  ```
  cargo test --locked -p mori-core --features midnight-compat-tests --test intents
  ```
- The optional `rust` workflow is manual only. Ordinary work does not dispatch
  hosted CI. Source-only handoffs list unrun checks and blocked acceptance;
  broad Rust suites do not override the zero-native boundary.
- Fix a lint rather than silence it. An `#[allow(clippy::...)]` needs a comment
  saying why.
- Comments explain the code. Review status, authorship, model names, audit IDs
  and notes like "not yet run" go in commit messages, pull requests or
  `docs/audits/`, not in source.

## Front end

- **Lexer first, then parser.** A hand-written lexer tokenizes the whole source
  in one pass, then a hand-written recursive-descent parser walks the token
  panics, fed random strings, sequences of tricky fragments and, from the
  parser on, real programs with random damage.
- Formatter tests also assert that formatting the output again changes nothing.
- **Test semantics in the library crates; keep the CLI a thin layer over them.**
  The CLI feature-boundary work has a narrow exception for real binary tests
  of public dispatch, feature availability, frozen complete output, process
  exits and I/O. This exception does not make a shared evaluator an independent
  oracle. Never snapshot `--help`; use clap schema assertions for that contract.
- Existing external fixture paths and test-support mounts are retained during
  the feature-boundary cut; they are not precedents for new paths or mounts.
  `core_continuations` includes the three `examples/core/continuations/*.mori`
  sources and `normal-oracle.json`; `stage_test_public` reads
  `models/core-stage-tests`; `fee_pool_component` and one `core_test_report`
  case read `examples/core/fee-pool`. Native targets also retain their existing
  example/deliverable inputs. Their provenance and the rule against fixtures
  outside `crates/` need separate reconciliation before claiming full guideline
  conformance. Preserve originals and evidence identities.
- Existing `#[path = "../../mori-ledger/tests/support/mod.rs"]` mounts in
  `prepare_call`, `installed_call` and `signed_installed_call` remain unchanged
```

### docs/MORIARTY-PRODUCT-CONTRACT.md

`context/docs/MORIARTY-PRODUCT-CONTRACT.md`; original lines [(1, 13)]; SHA256 `bf89e240d04c1c3580767a9fc0f551acd8c7c6dad747314d160d93cec45135a6`.

```text
# Moriarty: permissionless language of provable intention

Status: target architecture, specified-only where not linked to implementation. Authority: user clarification, 2026-09-19.

Moriarty is a programming language for all DeFi developers on Midnight. Any developer may author, compile, prove, publish and deploy programs in the supported language without approval from Moriarty maintainers, an agent council, Foreman, a hosted solver or a program registry. Objective Midnight validity rules, resource costs and asset-owner authorization still apply.

This document supersedes conflicting product-scope and administrative-deployment requirements in earlier designs. Historical receipts remain evidence of their original scope. It does not declare missing proof or ledger capabilities implemented.

## Consolidated design and delivery

The [consolidated vision and design](MORIARTY-CONSOLIDATED-DESIGN.md), [single U0–U7 roadmap](../ROADMAP.md), and [next ZKIR/recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) integrate the research agenda. P/C/K identifiers remain requirement lineage; they are not independent delivery queues. The user supplied a planning assumption on 2026-09-19 that comprehensive Midnight recursion arrives in about six months. Plan full native recursive compliance and private multi-parent composition accordingly; current interface limitations do not remove that scope. The timing is not a verified release claim.

## Product boundary
```

### ROADMAP.md

`context/ROADMAP.md`; original lines [(239, 246)]; SHA256 `656a77595d1c7fb28f5237b5801b593174f8eaf63c6e1ca2243e15f4e9a36113`.

```text
The [M1–M17 crosswalk](openspec/changes/consolidated-language-kernel/traceability.md#fenrir-language-capability-crosswalk--2026-10-14)
keeps the user's language requirements explicit. Preview and Fenrir evidence
remain separately scoped; Fenrir network/Anchor, committee/registry identities
and the qualified target tuple are not invented. The complete U0–U7 exits and
all MC/SP/G, financial and composition obligations remain unchanged.

Progress will use evidence-backed capability and predicate records rather than
the earlier subjective completion percentages. The discovery package indexes
```

### openspec/changes/consolidated-language-kernel/requirements.md

`context/openspec/changes/consolidated-language-kernel/requirements.md`; complete body; SHA256 `aa7b2243bbb27e0420bce24977180eec253d0f8c201c94baa1d14333ca1e4a9a`.

```text
# Consolidated EARS requirements

[UNI-001–024](specs/consolidated-language-kernel/spec.md) are the normative consolidation clauses. UNI-001–017 retain their original positive and hostile scenarios unchanged. UNI-018–024 add the October 14 profile, admission, independent-oracle, evidence, target, membership and developer-journey reconciliation. Existing MOR-001–012, AEO-001–004 and MPLR-001–035 requirements remain incorporated as detailed behavioral obligations; DEV-001 applies only to internal delivery. [Traceability](traceability.md) names their single phase owners and retained MC/SP lineage. Grouping does not erase an individual scenario or theorem obligation.

The [PR50 reconciliation](reconciliation-2026-10-14.md) distinguishes delivered child capabilities from every open parent exit. Current implementation and review routing follows the October 14 assignment; all security work remains Opus-owned. No existing security clause changes here.

The [Opus-authored security addendum](security-reconciliation-2026-10-14.md) supplies the separate OSR-1014 clauses. Its authorship and evidence limits remain explicit. The current 160-heading index covers canonical `spec.md` clauses; the addendum is separately indexed by its original OSR-1014 IDs.

The [next ZKIR and recursion requirements](../../../docs/MORIARTY-BACKEND-REQUIREMENTS.md) add backend-specific capability and correctness requests. All are specified-only unless a row identifies scoped observed evidence. Their implementation belongs to the named component; no backend document substitutes for Moriarty financial semantics.

MNR01–MNR08 are the Mina-derived mandatory refinements under UNI-017 and ZR01–ZR16; their phase owners are recorded in the traceability register and their acceptance tests in the backend contract.
```

## Required typed response schema — unfilled, no self-approval

This schema describes the fresh reviewer response. It is not a returned review. Every exact quote must be checked against the cited pinned path/hash and original line range. No evidence field is a substitute for substantive independent reading.

### Typed independent-review response schema

`REVIEW-RESPONSE.schema.json`; complete body; SHA256 `c4e3cf019575e41f6d1410d0bc96ea66ab1c40f8c4d3c4d35110e019a9a8e0e6`.

```json
{"$schema":"https://json-schema.org/draft/2020-12/schema","$id":"mori-shared-check-independent-review/1","type":"object","additionalProperties":false,"required":["format","reviewer","head","tree","base","coverage","sourceChecks","resultChecks","findings","sourceDisposition","resultDisposition","unrunOrUnread","retainedOpenRequirements"],"properties":{"format":{"const":"mori-shared-check-independent-review/1"},"reviewer":{"type":"object","required":["requestedModel","returnedModel","effort","authorOfCandidate","independent","terminalStatus"],"properties":{"requestedModel":{"type":"string"},"returnedModel":{"type":"string"},"effort":{"type":"string"},"authorOfCandidate":{"const":false},"independent":{"type":"boolean"},"terminalStatus":{"enum":["completed","partial","unavailable","refused","failed"]}}},"head":{"const":"823ce081c00786a0c55b92af81009f9e7c5e6a16"},"tree":{"const":"aa593783a8717ac838ae556e24c187997ab3547e"},"base":{"const":"e4d7a2b5709caa18a94f5256d03a0302364b8235"},"coverage":{"type":"object","additionalProperties":false,"required":["crates/fixtures/check-source/cases.json","crates/fixtures/check-source/cases/accepted.mori","crates/fixtures/check-source/cases/core1-accepted.mori","crates/fixtures/check-source/cases/core1-rejected.mori","crates/fixtures/check-source/cases/core2-accepted.mori","crates/fixtures/check-source/cases/core2-rejected.mori","crates/fixtures/check-source/cases/rejected.mori","crates/fixtures/check-source/cases/unicode-byte-offsets.mori","crates/fixtures/check-source/cases/unparsable.mori","crates/fixtures/check-source/expected/accepted.stdout","crates/fixtures/check-source/expected/core1-accepted.stdout","crates/fixtures/check-source/expected/core1-rejected.stdout","crates/fixtures/check-source/expected/core2-accepted.stdout","crates/fixtures/check-source/expected/core2-rejected.stdout","crates/fixtures/check-source/expected/rejected.stdout","crates/fixtures/check-source/expected/unicode-byte-offsets.stdout","crates/fixtures/check-source/expected/unparsable.stdout","crates/mori/src/commands/check.rs","crates/mori/src/commands/check_source.rs","crates/mori/src/commands/mod.rs","crates/mori/tests/check_source_dispatch.rs"],"properties":{"crates/fixtures/check-source/cases.json":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/accepted.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/core1-accepted.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/core1-rejected.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/core2-accepted.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/core2-rejected.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/rejected.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/unicode-byte-offsets.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/cases/unparsable.mori":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/accepted.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/core1-accepted.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/core1-rejected.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/core2-accepted.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/core2-rejected.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/rejected.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/unicode-byte-offsets.stdout":{"$ref":"#/$defs/coverageItem"},"crates/fixtures/check-source/expected/unparsable.stdout":{"$ref":"#/$defs/coverageItem"},"crates/mori/src/commands/check.rs":{"$ref":"#/$defs/coverageItem"},"crates/mori/src/commands/check_source.rs":{"$ref":"#/$defs/coverageItem"},"crates/mori/src/commands/mod.rs":{"$ref":"#/$defs/coverageItem"},"crates/mori/tests/check_source_dispatch.rs":{"$ref":"#/$defs/coverageItem"}}},"sourceChecks":{"type":"object","additionalProperties":false,"required":["public-consumer","profiles-and-checkers","beta-json-bytes","human-laziness-and-order","core-report-and-qualification","exit-and-diagnostics","public-regression-and-oracle","scope-and-guidelines","evidence-boundaries"],"properties":{"public-consumer":{"$ref":"#/$defs/assessment"},"profiles-and-checkers":{"$ref":"#/$defs/assessment"},"beta-json-bytes":{"$ref":"#/$defs/assessment"},"human-laziness-and-order":{"$ref":"#/$defs/assessment"},"core-report-and-qualification":{"$ref":"#/$defs/assessment"},"exit-and-diagnostics":{"$ref":"#/$defs/assessment"},"public-regression-and-oracle":{"$ref":"#/$defs/assessment"},"scope-and-guidelines":{"$ref":"#/$defs/assessment"},"evidence-boundaries":{"$ref":"#/$defs/assessment"}}},"resultChecks":{"type":"object","additionalProperties":false,"required":["accepted","rejected","unparsable","core1-accepted","core1-rejected","core2-accepted","core2-rejected","unicode-byte-offsets"],"properties":{"accepted":{"$ref":"#/$defs/assessment"},"rejected":{"$ref":"#/$defs/assessment"},"unparsable":{"$ref":"#/$defs/assessment"},"core1-accepted":{"$ref":"#/$defs/assessment"},"core1-rejected":{"$ref":"#/$defs/assessment"},"core2-accepted":{"$ref":"#/$defs/assessment"},"core2-rejected":{"$ref":"#/$defs/assessment"},"unicode-byte-offsets":{"$ref":"#/$defs/assessment"}}},"findings":{"type":"array","items":{"type":"object","required":["id","axis","severity","predicate","quotes","expected","observed","fixOrNextCheck"],"properties":{"id":{"type":"string"},"axis":{"enum":["source","result","guideline","coverage"]},"severity":{"enum":["blocking","major","minor","note"]},"predicate":{"type":"string","minLength":20},"quotes":{"type":"array","minItems":1,"items":{"$ref":"#/$defs/quote"}},"expected":{"type":"string","minLength":20},"observed":{"type":"string","minLength":20},"fixOrNextCheck":{"type":"string","minLength":20}}}},"sourceDisposition":{"enum":["accept-scoped-source","changes-required","partial","abstain"]},"resultDisposition":{"enum":["stored-eight-case-results-consistent","inconsistent","partial","abstain"]},"unrunOrUnread":{"type":"array","items":{"type":"string"}},"retainedOpenRequirements":{"type":"array","minItems":1,"items":{"type":"string"}}},"$defs":{"quote":{"type":"object","additionalProperties":false,"required":["path","sha256","lineStart","lineEnd","exactQuote","supports"],"properties":{"path":{"type":"string","minLength":1},"sha256":{"type":"string","pattern":"^[0-9a-f]{64}$"},"lineStart":{"type":"integer","minimum":1},"lineEnd":{"type":"integer","minimum":1},"exactQuote":{"type":"string","minLength":8},"supports":{"type":"string","minLength":20}}},"assessment":{"type":"object","additionalProperties":false,"required":["disposition","reason","quotes","evidenceRefs","limitations"],"properties":{"disposition":{"enum":["supported","defect","partial","unassessed"]},"reason":{"type":"string","minLength":80},"quotes":{"type":"array","minItems":1,"items":{"$ref":"#/$defs/quote"}},"evidenceRefs":{"type":"array","minItems":1,"items":{"type":"string"}},"limitations":{"type":"array","items":{"type":"string"}}}},"coverageItem":{"type":"object","additionalProperties":false,"required":["sha256","readScope","assessment"],"properties":{"sha256":{"type":"string","pattern":"^[0-9a-f]{64}$"},"readScope":{"enum":["full","partial","unread"]},"assessment":{"type":"string","minLength":20}}}}}
```

## Packet integrity and remaining empirical boundary

`INLINE-BLOCKS.json` gives exact byte offsets/hashes for every inline body; `PACKET-VERIFICATION.json` records lossless recovery of all21 changed paths and all16 output bodies. `CHANGED-PATHS.json`, `CONTEXT-PINS.json`, `RECEIPT-PINS.json`, `BYTE-CLAIMS.json` and `BASELINE-DELTA-VERIFIED.json` preserve provenance. Packet generation only read exact Git blobs and stored receipts and wrote this exclusive review directory; it reran no product or compiler. Review source disposition and stored-results disposition separately. Nothing in this packet authorizes or approves integration, a new dependency/resource choice or full MCP delivery.
