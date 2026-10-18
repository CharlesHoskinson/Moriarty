# Independently scoped complete current Boolean source and actual-result audit

This is a NEW audit of current candidate ce7d16557fc0605f929357628d907d2a66d6153c/tree77bf20aa4556293f1399f10c6a5deefa0697c819, parent829f4456dfda8d4fc5a4ece84c47f2539f846d9f. It is not a continuation or claimed completion of the old121body/94historical-meaning review. Original frozen packets/opinions are untouched optional archives. No earlier review or timeout supplies a vote.

Read every complete body below once. Mandatory scope: all nine current candidate bodies and full before-guideline, every current56source/requirement unit, full current production consumer chain, applicable M01/UNI/product and all27coverage obligations/exposure records, exact whole pre-execution derivation/spec/freeze, complete current actual receipts/driver/streams/four public process captures/two whole21node Programs and current886pin/commit binding. All46complete bodies from the earlier producer-truth companion are reused here without omission. Its intact237861Bfull duplicate and mechanically repeated JSON inventories are available as optional corroboration, not a second mandatory representation. Current CLI/expand source is retained; obsolete before-CLI/before-doc versions, nested failed opinions, prior review schemas and recursive historical mapping are no longer mandatory.

The result is one selected test PASS55.5382868700035s, four complete real public check processes and both captured complete API Program results against exact frozen expectations, on base829f plus nine working-tree paths followed by ce7d commit binding. There was no postcommit rerun. Full886pin matrix is separately pinned as custody/ALL886-SOURCE-PINS.json for read-only hash/record verification; it is custody metadata, not a requirement to read unrelated repository sources. Complete result/binding/driver records below remain mandatory.

Expectations were independently frozen from execution but the author inspected implementation/schema and used the finite M01 premise. They are hand-derived, source-exposed author corroboration, not blinded independent semantic/financial truth. Older base/status/productOutputsRead/reviewPending strings in original provenance describe their recorded authorship/capture time, not this audit's outcome. No production repair or RED defect is claimed; only seven named Boolean guideline-exception lines plus eight additions.

Whole source assessment includes every Boolean occurrence/precedence/type/repeated leaf, full21node arenas/two actions/six roots, state/asset/input/binder/statement metadata, public checking/human serialization and complete current parser/loader/validation/plan/cost/digest/lexer/diagnostic/serializer behavior used. Whole result assessment includes exact args/exit/raw streams/complete Program JSON and source/phase/binding provenance, not a summary count or boolean alone. Independently find defects or uncertainty.

No evaluator/action/Await advancement/arbitrary-input truth/work golden/finance/security/compilerArtifact/Target/native/SDK/proof/ledger/owner or27/148/292/20/U/M/MC/SP/Gparent acceptance is inferred. Ne remains unselected. Driver hashes/local receipts do not establish external backend or read attestation.

BEGIN FILE source/crates/fixtures/core-boolean-check/README.md | SHA256 3ee7348e4e7822e2544cb2efc457da3d04c3aba99424fc35fb8862d6ca433c05 | BYTES 2152
# Positive Boolean source-check specimens

These two programs check ordinary documented syntax in Core/1 and Core/2. They combine `<`, `or`, prefix `not`, `and`, `>`, Ensures, fresh repeated names/literals and a second action. Core/2 adds an Await event binder whose guard is retained in the arena before runtime. No scenario or action execution is used.

Expected complete Programs were manually enumerated from source in lexical statement/action order and child-first expression order, with author parser/DTO exposure declared. Core/2 uses the adopted finite M01 premise; Core/1 uses implementation-exposed representation and finite Program precedents, not a general normative proof. Both have 21 nodes: roots2/8/12/16/17 in choose and20 in inspect_limit. The false state initializer allocates no arena node; the guard binder allocates none. The expression table retains repeated false/name occurrences and the second action. No output was used to generate these values.

Expected check JSON includes complete actions/roots, exact source SHA256, labels and empty diagnostics. Its ASCII pretty form follows the existing checker/DTO and JSON exporter, with lexicographic object keys, two-space indentation and final LF. Sources include final LF. Human success uses the existing fixed Core status lines, with empty stdout. Expected results are frozen source-derived corroboration, not independent financial/work goldens.

A source-check pass asserts static admission only. No claim is made that all requires/ensures hold for arbitrary runtime inputs, that the await advances, or that a compiler/backend admits the same source. No undocumented Ne spelling is selected. All original 27 constructor obligations and complete language/Program/Target/Artifact, finance, execution, proof and ledger acceptance gates remain open.

The literal fixture test deliberately compares complete frozen JSON instead of generating snapshots, to keep pre-execution expectations immutable and show the exact source in assertion messages. It uses real public checker binary calls and public parse/stage::load APIs. The CLI test does not implement a second parser/evaluator.
END FILE source/crates/fixtures/core-boolean-check/README.md

BEGIN FILE source/crates/fixtures/core-boolean-check/core1.check.stdout.expected.json | SHA256 f4780f747afc62c7f118a5baa8b366499832d27e129fadd3c1b5c077aa97e1c0 | BYTES 1166
{
  "actions": [
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        },
        {
          "name": "enabled",
          "ty": "Bool"
        }
      ],
      "name": "choose",
      "statements": [
        {
          "kind": "Let",
          "name": "smaller",
          "value": 2
        },
        {
          "kind": "Requires",
          "predicate": 8
        },
        {
          "kind": "Next",
          "state": "result",
          "value": 12
        },
        {
          "kind": "Ensures",
          "predicate": 16
        },
        {
          "kind": "Emit",
          "name": "decision",
          "value": 17
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        }
      ],
      "name": "inspect_limit",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 20
        }
      ]
    }
  ],
  "agreement": "BooleanChoice",
  "diagnostics": [],
  "file": "boolean.mori",
  "language": "moriarty/1",
  "source_hash": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
  "status": "accepted"
}
END FILE source/crates/fixtures/core-boolean-check/core1.check.stdout.expected.json

BEGIN FILE source/crates/fixtures/core-boolean-check/core1.mori | SHA256 9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64 | BYTES 379
language "moriarty/1";
agreement BooleanChoice {
  state result: Bool = false;
  action choose(limit: UInt, enabled: Bool) {
    let smaller = 2 < limit;
    requires smaller or enabled and not false;
    next result = smaller or not enabled;
    ensures result or not enabled;
    emit "decision"(result);
  }
  action inspect_limit(limit: UInt) {
    requires limit > 0;
  }
}
END FILE source/crates/fixtures/core-boolean-check/core1.mori

BEGIN FILE source/crates/fixtures/core-boolean-check/core1.program.expected.json | SHA256 00ae8b9039ee09557181d5c06740a813be429f8fe6bb35d7566d7ab2b8b4dfca | BYTES 2755
{
  "language": "moriarty/1",
  "semantics": "moriarty-core/1",
  "name": "BooleanChoice",
  "states": [
    {
      "name": "result",
      "ty": "Bool",
      "initial": {
        "type": "Bool",
        "value": false
      }
    }
  ],
  "assets": [],
  "expressions": [
    {
      "kind": "Literal",
      "value": {
        "type": "UInt",
        "value": "2"
      }
    },
    {
      "kind": "Name",
      "name": "limit"
    },
    {
      "kind": "Binary",
      "op": "Lt",
      "lhs": 0,
      "rhs": 1
    },
    {
      "kind": "Name",
      "name": "smaller"
    },
    {
      "kind": "Name",
      "name": "enabled"
    },
    {
      "kind": "Literal",
      "value": {
        "type": "Bool",
        "value": false
      }
    },
    {
      "kind": "Not",
      "value": 5
    },
    {
      "kind": "Binary",
      "op": "And",
      "lhs": 4,
      "rhs": 6
    },
    {
      "kind": "Binary",
      "op": "Or",
      "lhs": 3,
      "rhs": 7
    },
    {
      "kind": "Name",
      "name": "smaller"
    },
    {
      "kind": "Name",
      "name": "enabled"
    },
    {
      "kind": "Not",
      "value": 10
    },
    {
      "kind": "Binary",
      "op": "Or",
      "lhs": 9,
      "rhs": 11
    },
    {
      "kind": "Name",
      "name": "result"
    },
    {
      "kind": "Name",
      "name": "enabled"
    },
    {
      "kind": "Not",
      "value": 14
    },
    {
      "kind": "Binary",
      "op": "Or",
      "lhs": 13,
      "rhs": 15
    },
    {
      "kind": "Name",
      "name": "result"
    },
    {
      "kind": "Name",
      "name": "limit"
    },
    {
      "kind": "Literal",
      "value": {
        "type": "UInt",
        "value": "0"
      }
    },
    {
      "kind": "Binary",
      "op": "Gt",
      "lhs": 18,
      "rhs": 19
    }
  ],
  "actions": [
    {
      "name": "choose",
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        },
        {
          "name": "enabled",
          "ty": "Bool"
        }
      ],
      "statements": [
        {
          "kind": "Let",
          "name": "smaller",
          "value": 2
        },
        {
          "kind": "Requires",
          "predicate": 8
        },
        {
          "kind": "Next",
          "state": "result",
          "value": 12
        },
        {
          "kind": "Ensures",
          "predicate": 16
        },
        {
          "kind": "Emit",
          "name": "decision",
          "value": 17
        }
      ]
    },
    {
      "name": "inspect_limit",
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        }
      ],
      "statements": [
        {
          "kind": "Requires",
          "predicate": 20
        }
      ]
    }
  ]
}
END FILE source/crates/fixtures/core-boolean-check/core1.program.expected.json

BEGIN FILE source/crates/fixtures/core-boolean-check/core2.check.stdout.expected.json | SHA256 df5b3adcfacc6b00ef80561cc5a962f60136450797e588352964a91da6e2d403 | BYTES 1422
{
  "actions": [
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        },
        {
          "name": "enabled",
          "ty": "Bool"
        }
      ],
      "name": "choose",
      "statements": [
        {
          "kind": "Let",
          "name": "smaller",
          "value": 2
        },
        {
          "binders": [
            {
              "name": "accepted",
              "ty": "Bool"
            }
          ],
          "guard": 8,
          "kind": "Await",
          "label": "approve"
        },
        {
          "kind": "Next",
          "state": "result",
          "value": 12
        },
        {
          "kind": "Ensures",
          "predicate": 16
        },
        {
          "kind": "Emit",
          "name": "decision",
          "value": 17
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        }
      ],
      "name": "inspect_limit",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 20
        }
      ]
    }
  ],
  "agreement": "StagedBooleanChoice",
  "authentication": "none",
  "consumption": "untracked",
  "diagnostics": [],
  "file": "boolean.mori",
  "language": "moriarty/2",
  "qualification": "local-reference-stage",
  "source_hash": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
  "status": "accepted"
}
END FILE source/crates/fixtures/core-boolean-check/core2.check.stdout.expected.json

BEGIN FILE source/crates/fixtures/core-boolean-check/core2.mori | SHA256 3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969 | BYTES 413
language "moriarty/2";
agreement StagedBooleanChoice {
  state result: Bool = false;
  action choose(limit: UInt, enabled: Bool) {
    let smaller = 2 < limit;
    await approve(accepted: Bool) when accepted or enabled and not false;
    next result = smaller or not enabled;
    ensures result or not accepted;
    emit "decision"(result);
  }
  action inspect_limit(limit: UInt) {
    requires limit > 0;
  }
}
END FILE source/crates/fixtures/core-boolean-check/core2.mori

BEGIN FILE source/crates/fixtures/core-boolean-check/core2.program.expected.json | SHA256 e4b93ee899aeebdcbf68be708d5bbf66653b3fc4c15659cefe2fdf2df5485e0b | BYTES 2911
{
  "language": "moriarty/2",
  "semantics": "moriarty-core/2",
  "name": "StagedBooleanChoice",
  "states": [
    {
      "name": "result",
      "ty": "Bool",
      "initial": {
        "type": "Bool",
        "value": false
      }
    }
  ],
  "assets": [],
  "expressions": [
    {
      "kind": "Literal",
      "value": {
        "type": "UInt",
        "value": "2"
      }
    },
    {
      "kind": "Name",
      "name": "limit"
    },
    {
      "kind": "Binary",
      "op": "Lt",
      "lhs": 0,
      "rhs": 1
    },
    {
      "kind": "Name",
      "name": "accepted"
    },
    {
      "kind": "Name",
      "name": "enabled"
    },
    {
      "kind": "Literal",
      "value": {
        "type": "Bool",
        "value": false
      }
    },
    {
      "kind": "Not",
      "value": 5
    },
    {
      "kind": "Binary",
      "op": "And",
      "lhs": 4,
      "rhs": 6
    },
    {
      "kind": "Binary",
      "op": "Or",
      "lhs": 3,
      "rhs": 7
    },
    {
      "kind": "Name",
      "name": "smaller"
    },
    {
      "kind": "Name",
      "name": "enabled"
    },
    {
      "kind": "Not",
      "value": 10
    },
    {
      "kind": "Binary",
      "op": "Or",
      "lhs": 9,
      "rhs": 11
    },
    {
      "kind": "Name",
      "name": "result"
    },
    {
      "kind": "Name",
      "name": "accepted"
    },
    {
      "kind": "Not",
      "value": 14
    },
    {
      "kind": "Binary",
      "op": "Or",
      "lhs": 13,
      "rhs": 15
    },
    {
      "kind": "Name",
      "name": "result"
    },
    {
      "kind": "Name",
      "name": "limit"
    },
    {
      "kind": "Literal",
      "value": {
        "type": "UInt",
        "value": "0"
      }
    },
    {
      "kind": "Binary",
      "op": "Gt",
      "lhs": 18,
      "rhs": 19
    }
  ],
  "actions": [
    {
      "name": "choose",
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        },
        {
          "name": "enabled",
          "ty": "Bool"
        }
      ],
      "statements": [
        {
          "kind": "Let",
          "name": "smaller",
          "value": 2
        },
        {
          "kind": "Await",
          "label": "approve",
          "binders": [
            {
              "name": "accepted",
              "ty": "Bool"
            }
          ],
          "guard": 8
        },
        {
          "kind": "Next",
          "state": "result",
          "value": 12
        },
        {
          "kind": "Ensures",
          "predicate": 16
        },
        {
          "kind": "Emit",
          "name": "decision",
          "value": 17
        }
      ]
    },
    {
      "name": "inspect_limit",
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        }
      ],
      "statements": [
        {
          "kind": "Requires",
          "predicate": 20
        }
      ]
    }
  ]
}
END FILE source/crates/fixtures/core-boolean-check/core2.program.expected.json

BEGIN FILE source/crates/mori/tests/core_boolean_check_public.rs | SHA256 167f754b5f7375b8f2db9b28b040f0c9f38d43ede47489a9a038b4c59f3485b2 | BYTES 5661
//! Positive documented Boolean source forms through real public Core checking.
//! Full Program and CLI expectations were manually frozen before execution;
//! the author's implementation-source exposure is explicit in the fixtures.
use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
};

const HUMAN_STDERR: &str = "    Checking boolean.mori\n    Finished bounded core check\n";

fn fixture(name: &str) -> Vec<u8> {
    fs::read(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../fixtures/core-boolean-check")
            .join(name),
    )
    .expect("read frozen Boolean fixture")
}

struct Project {
    root: PathBuf,
    temporary: bool,
}

impl Project {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let (root, temporary) = match std::env::var_os("MORI_BOOLEAN_CHECK_OBSERVATIONS") {
            Some(path) => {
                let path = PathBuf::from(path);
                assert!(path.is_absolute(), "use an absolute capture directory");
                (path, false)
            }
            None => (
                std::env::temp_dir().join(format!(
                    "mori-boolean-check-{}-{}",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                )),
                true,
            ),
        };
        fs::create_dir(&root).expect("use a fresh capture directory");
        Self { root, temporary }
    }

    fn run(&self, id: &str, mode: &str, source: &[u8]) -> Output {
        let dir = self.root.join(format!("{id}-{mode}"));
        fs::create_dir(&dir).unwrap();
        fs::write(dir.join("boolean.mori"), source).unwrap();
        let mut args = vec!["--color", "never", "check", "boolean.mori"];
        if mode == "json" {
            args.push("--json");
        }
        let captured = Command::new(env!("CARGO_BIN_EXE_mori"))
            .current_dir(&dir)
            .args(&args)
            .output();
        if let Err(error) = &captured {
            fs::write(
                dir.join("process.json"),
                serde_json::to_vec_pretty(&serde_json::json!({
                    "executable": env!("CARGO_BIN_EXE_mori"),
                    "args": args, "launchError": error.to_string(),
                }))
                .unwrap(),
            )
            .unwrap();
        }
        let output = captured.expect("run actual public check");
        // Retain complete actual process bytes and exit before assertions.
        fs::write(dir.join("stdout.bin"), &output.stdout).unwrap();
        fs::write(dir.join("stderr.bin"), &output.stderr).unwrap();
        fs::write(
            dir.join("process.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "executable": env!("CARGO_BIN_EXE_mori"),
                "args": args, "status": output.status.code(),
            }))
            .unwrap(),
        )
        .unwrap();
        output
    }
}

impl Drop for Project {
    fn drop(&mut self) {
        if self.temporary && !std::thread::panicking() {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
}

#[test]
fn documented_boolean_programs_reach_public_core_check() {
    let project = Project::new();
    // All four public CLI calls are captured before any product assertion.
    let outputs: Vec<_> = [("core1", false), ("core2", true)]
        .into_iter()
        .map(|(id, staged)| {
            let source = fixture(&format!("{id}.mori"));
            let json = project.run(id, "json", &source);
            let human = project.run(id, "human", &source);
            (id, staged, source, json, human)
        })
        .collect();
    let captured: Vec<_> = outputs
        .into_iter()
        .map(|(id, staged, source, json, human)| {
            let text = std::str::from_utf8(&source).expect("UTF-8 source fixture");
            // Actual exported consumers, with no evaluator, synthetic AST or lowering.
            let program = if staged {
                mori_core::stage::load(text).map(|program| program.program().clone())
            } else {
                mori_core::parse(text)
            };
            fs::write(
                project
                    .root
                    .join(format!("{id}.program-result.actual.json")),
                serde_json::to_vec_pretty(&program).unwrap(),
            )
            .unwrap();
            (id, source, json, human, program)
        })
        .collect();
    for (id, source, json, human, program) in captured {
        let description = format!(
            "{id} complete source:\n{}",
            String::from_utf8_lossy(&source)
        );
        let actual =
            serde_json::to_value(program.expect("admit frozen documented source")).unwrap();
        let expected: Value =
            serde_json::from_slice(&fixture(&format!("{id}.program.expected.json"))).unwrap();
        assert_eq!(actual, expected, "{description}: complete Program");
        assert_eq!(json.status.code(), Some(0), "{description}");
        assert_eq!(
            json.stdout,
            fixture(&format!("{id}.check.stdout.expected.json")),
            "{description}: complete check stdout"
        );
        assert!(json.stderr.is_empty(), "{description}: {:?}", json.stderr);
        assert_eq!(human.status.code(), Some(0), "{description}");
        assert!(human.stdout.is_empty(), "{description}");
        assert_eq!(
            human.stderr,
            HUMAN_STDERR.as_bytes(),
            "{description}: complete human stream"
        );
    }
}
END FILE source/crates/mori/tests/core_boolean_check_public.rs

BEGIN FILE source/crates/mori/src/main.rs | SHA256 35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232 | BYTES 495
use std::process::ExitCode;

use clap::Parser;

use crate::cli::Cli;
use crate::shell::Shell;

mod cli;
mod commands;
mod display;
mod error;
mod json;
mod shell;
mod styles;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let mut shell = Shell::new(cli.global.verbosity(), cli.global.color.into());

    match commands::run(cli.command, &mut shell) {
        Ok(code) => code,
        Err(report) => {
            shell.report(&report);
            ExitCode::FAILURE
        }
    }
}
END FILE source/crates/mori/src/main.rs

BEGIN FILE source/crates/mori/src/commands/mod.rs | SHA256 4e39c2dd984c1dcd0687da30eddd4e630f3d9480aef0b5df25e48828001abc47 | BYTES 2838
//! One module per subcommand. Each defines its `Args` and a `run` function.

use std::process::ExitCode;

use mori_diagnostics::Report;

use crate::cli::Command;
use crate::shell::Shell;

mod call_io;
pub mod capabilities;
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
        Command::Capabilities(args) => capabilities::run(args, shell),
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
END FILE source/crates/mori/src/commands/mod.rs

BEGIN FILE source/crates/mori/src/commands/check.rs | SHA256 2f08c04bdc1a07796c459385ec9271466a26572f2455ed7d63de906c1d5876bb | BYTES 3744
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
END FILE source/crates/mori/src/commands/check.rs

BEGIN FILE source/crates/mori/src/commands/check_source.rs | SHA256 a53d5c3689f3c32ddc8c595e577a0ac13cfc4fe187b42a4f83c9caf86dc9ace3 | BYTES 3139
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
        let language = match mori_core::language(source) {
            Ok(language) => language,
            Err(error) => return consumer.core(Err(error), false),
        };
        let staged = language == "moriarty/2";
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
END FILE source/crates/mori/src/commands/check_source.rs

BEGIN FILE source/crates/mori/src/shell.rs | SHA256 b2116fcd9bb80abfb175f6dc2da6327b4d9787cfa3d56481fde705e62d94fff1 | BYTES 3802
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
END FILE source/crates/mori/src/shell.rs

BEGIN FILE source/crates/mori/src/display.rs | SHA256 81d499d32561dc576651e0b1df5a349b68112030f296d93cca95a488dc3b0deb | BYTES 408
/// Escape data before a terminal renderer adds its trusted styles and layout.
pub fn text(value: &str) -> String {
    let mut result = String::with_capacity(value.len());
    for character in value.chars() {
        if character.is_ascii() && !character.is_control() {
            result.push(character);
        } else {
            result.extend(character.escape_default());
        }
    }
    result
}
END FILE source/crates/mori/src/display.rs

BEGIN FILE source/crates/mori-core/src/parser.rs | SHA256 66c839add400c62e70371a2cf1c412d391f3e0b626b860dc51a3e01039d46cc7 | BYTES 15611
use crate::*;
use mori_lexer::{TokenIdx, TokenKind};

/// Parses the explicitly versioned core profile; beta grammar is untouched.
pub fn parse(source: &str) -> Result<Program, Error> {
    parse_profile(source, false)
}

/// Syntax-only parser; only stage::load admits the resulting source.
pub(crate) fn parse_v2(source: &str) -> Result<Program, Error> {
    parse_profile(source, true)
}

fn parse_profile(source: &str, staged: bool) -> Result<Program, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let tokens = (0..lexed.len())
        .map(|i| {
            let idx = TokenIdx::new(i);
            let span = lexed.span(idx, source);
            let raw = &source[span.start as usize..span.end as usize];
            let text = if lexed.kind(idx) == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            };
            Tok {
                text,
                kind: lexed.kind(idx),
                start: span.start,
            }
        })
        .collect();
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: if staged {
                crate::stage::LANGUAGE_2
            } else {
                LANGUAGE
            }
            .into(),
            semantics: if staged {
                crate::stage::SEMANTICS_2
            } else {
                SEMANTICS
            }
            .into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    parser.expect("language")?;
    let profile = parser.string()?;
    let expected_language = if staged {
        crate::stage::LANGUAGE_2
    } else {
        LANGUAGE
    };
    if profile != expected_language {
        return Err(parser.error(
            "profile",
            format!("Expected language \"{expected_language}\"; found {profile:?}."),
        ));
    }
    parser.expect(";")?;
    parser.expect("agreement")?;
    parser.program.name = parser.word()?;
    parser.expect("{")?;
    while !parser.eat("}") {
        if parser.eat("state") {
            let name = parser.word()?;
            parser.expect(":")?;
            let ty = parser.ty()?;
            parser.expect("=")?;
            let initial = parser.literal()?;
            parser.expect(";")?;
            parser.program.states.push(StateDecl { name, ty, initial });
        } else if parser.eat("asset") {
            let name = parser.word()?;
            parser.expect("=")?;
            parser.expect("{")?;
            parser.expect("network")?;
            parser.expect(":")?;
            let network = parser.string()?;
            parser.expect(",")?;
            parser.expect("issuer")?;
            parser.expect(":")?;
            let issuer = parser.string()?;
            parser.expect(",")?;
            parser.expect("reference")?;
            parser.expect(":")?;
            let reference = parser.string()?;
            parser.expect(",")?;
            parser.expect("scale")?;
            parser.expect(":")?;
            let Value::UInt(UInt(scale)) = parser.literal()? else {
                return Err(parser.error("asset", "Asset scale must be an unsigned integer."));
            };
            let scale = u8::try_from(scale)
                .map_err(|_| parser.error("asset", "Asset scale must fit u8."))?;
            parser.eat(",");
            parser.expect("}")?;
            parser.expect(";")?;
            parser.program.assets.push(AssetDecl {
                name,
                identity: AssetId {
                    network,
                    issuer,
                    reference,
                    scale,
                },
            });
        } else if parser.eat("action") {
            let name = parser.word()?;
            parser.expect("(")?;
            let mut inputs = vec![];
            if !parser.eat(")") {
                loop {
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    inputs.push(InputDecl { name, ty });
                    if parser.eat(")") {
                        break;
                    }
                    parser.expect(",")?;
                }
            }
            parser.expect("{")?;
            let mut statements = vec![];
            while !parser.eat("}") {
                if parser.eat("input") {
                    if !statements.is_empty() {
                        return Err(
                            parser.error("input", "Declare action inputs before statements.")
                        );
                    }
                    let name = parser.word()?;
                    parser.expect(":")?;
                    let ty = parser.ty()?;
                    parser.expect(";")?;
                    inputs.push(InputDecl { name, ty });
                    continue;
                }
                statements.push(parser.statement()?);
                if statements.len() > MAX_STATEMENTS {
                    return Err(parser.error("bound", "Action exceeds 1,024 statements."));
                }
            }
            parser.program.actions.push(Action {
                name,
                inputs,
                statements,
            });
        } else {
            return Err(parser.error("unsupported", "Expected state, asset or action declaration; loops and native calls are unsupported."));
        }
        if parser.program.states.len() + parser.program.assets.len() + parser.program.actions.len()
            > MAX_DECLARATIONS
        {
            return Err(parser.error("bound", "Program exceeds 256 declarations."));
        }
    }
    if parser.peek().kind != TokenKind::Eof {
        return Err(parser.error("syntax", "Unexpected content after agreement."));
    }
    if !staged {
        validate(&parser.program)?;
    }
    Ok(parser.program)
}
struct Tok {
    text: String,
    kind: TokenKind,
    start: u32,
}
struct Parser {
    tokens: Vec<Tok>,
    pos: usize,
    program: Program,
}
impl Parser {
    fn peek(&self) -> &Tok {
        &self.tokens[self.pos]
    }
    fn error(&self, code: &str, message: impl Into<String>) -> Error {
        let mut e = Error::new(code, message);
        e.offset = Some(self.peek().start);
        e
    }
    fn eat(&mut self, text: &str) -> bool {
        if self.peek().text == text && self.peek().kind != TokenKind::String {
            self.pos += 1;
            true
        } else {
            false
        }
    }
    fn expect(&mut self, text: &str) -> Result<(), Error> {
        if self.eat(text) {
            Ok(())
        } else {
            Err(self.error(
                "syntax",
                format!("Expected {text:?}; found {:?}.", self.peek().text),
            ))
        }
    }
    fn word(&mut self) -> Result<String, Error> {
        if self.peek().kind.is_word() {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a name."))
        }
    }
    fn string(&mut self) -> Result<String, Error> {
        if self.peek().kind == TokenKind::String {
            let text = self.peek().text.clone();
            self.pos += 1;
            Ok(text)
        } else {
            Err(self.error("syntax", "Expected a quoted string."))
        }
    }
    fn ty(&mut self) -> Result<Type, Error> {
        match self.word()?.as_str() {
            "UInt" => Ok(Type::UInt),
            "Bool" => Ok(Type::Bool),
            _ => Err(self.error("type", "Supported types are UInt and Bool.")),
        }
    }
    fn literal(&mut self) -> Result<Value, Error> {
        if self.eat("true") {
            return Ok(Value::Bool(true));
        }
        if self.eat("false") {
            return Ok(Value::Bool(false));
        }
        if self.peek().kind != TokenKind::Number {
            return Err(self.error("syntax", "Expected an unsigned integer or boolean literal."));
        }
        let raw = self.peek().text.replace('_', "");
        let n = raw.parse::<u128>().map_err(|_| {
            self.error(
                "number",
                "UInt literal must fit u128; amounts use integral atomic units.",
            )
        })?;
        self.pos += 1;
        Ok(Value::UInt(UInt(n)))
    }
    fn push(&mut self, expr: Expr) -> Result<ExprId, Error> {
        if self.program.expressions.len() >= MAX_EXPRESSIONS {
            return Err(self.error("bound", "Program exceeds 8,192 expression nodes."));
        }
        let id = self.program.expressions.len() as u16;
        self.program.expressions.push(expr);
        Ok(id)
    }
    fn expr(&mut self, minimum: u8, depth: usize) -> Result<ExprId, Error> {
        if depth > 64 {
            return Err(self.error("bound", "Expression nesting exceeds 64."));
        }
        let mut lhs = if self.eat("(") {
            let e = self.expr(0, depth + 1)?;
            self.expect(")")?;
            e
        } else if self.eat("not") {
            let value = self.expr(6, depth + 1)?;
            self.push(Expr::Not { value })?
        } else if self.peek().kind == TokenKind::Number
            || self.peek().text == "true"
            || self.peek().text == "false"
        {
            let value = self.literal()?;
            self.push(Expr::Literal { value })?
        } else {
            let name = self.word()?;
            if matches!(name.as_str(), "liability_present" | "liability_amount") && self.eat("(") {
                let duty = self.string()?;
                self.expect(")")?;
                self.push(if name == "liability_present" {
                    Expr::LiabilityPresent { name: duty }
                } else {
                    Expr::LiabilityAmount { name: duty }
                })?
            } else {
                self.push(Expr::Name { name })?
            }
        };
        loop {
            // A decoded string spelling an operator is still a string token.
            if self.peek().kind == TokenKind::String {
                break;
            }
            let next = self
                .tokens
                .get(self.pos + 1)
                .filter(|t| t.kind != TokenKind::String)
                .map(|t| t.text.as_str());
            let (op, precedence, length) = match (self.peek().text.as_str(), next) {
                ("or", _) => (BinaryOp::Or, 1, 1),
                ("and", _) => (BinaryOp::And, 2, 1),
                ("=", Some("=")) => (BinaryOp::Eq, 3, 2),
                ("not", Some("=")) => (BinaryOp::Ne, 3, 2),
                ("<", Some("=")) => (BinaryOp::Le, 3, 2),
                (">", Some("=")) => (BinaryOp::Ge, 3, 2),
                ("<", _) => (BinaryOp::Lt, 3, 1),
                (">", _) => (BinaryOp::Gt, 3, 1),
                ("+", _) => (BinaryOp::Add, 4, 1),
                ("-", _) => (BinaryOp::Sub, 4, 1),
                ("*", _) => (BinaryOp::Mul, 5, 1),
                _ => break,
            };
            if precedence < minimum {
                break;
            }
            self.pos += length;
            let rhs = self.expr(precedence + 1, depth + 1)?;
            lhs = self.push(Expr::Binary { op, lhs, rhs })?;
        }
        Ok(lhs)
    }
    fn statement(&mut self) -> Result<Statement, Error> {
        let keyword = self.word()?;
        let s=match keyword.as_str() {
            "await" if self.program.language == crate::stage::LANGUAGE_2 => {
                let label = self.word()?;
                self.expect("(")?;
                let mut binders = Vec::new();
                if !self.eat(")") {
                    loop {
                        let name = self.word()?;
                        self.expect(":")?;
                        let ty = self.ty()?;
                        binders.push(InputDecl { name, ty });
                        if self.eat(")") { break; }
                        self.expect(",")?;
                    }
                }
                self.expect("when")?;
                Statement::Await { label, binders, guard: self.expr(0, 0)? }
            },
            "let"=>{let name=self.word()?;self.expect("=")?;Statement::Let{name,value:self.expr(0,0)?}},
            "next"=>{let state=self.word()?;self.expect("=")?;Statement::Next{state,value:self.expr(0,0)?}},
            "requires"=>Statement::Requires{predicate:self.expr(0,0)?},
            "ensures"=>Statement::Ensures{predicate:self.expr(0,0)?},
            "emit"=>{let name=self.string()?;self.expect("(")?;let value=self.expr(0,0)?;self.expect(")")?;Statement::Emit{name,value}},
            "transfer"|"fee"=>{self.expect("(")?;let asset=self.word()?;self.expect(",")?;let from=self.string()?;self.expect(",")?;let to=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;if keyword=="fee"{Statement::Fee{asset,from,to,amount}}else{Statement::Transfer{asset,from,to,amount}}},
            "liability"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let asset=self.word()?;self.expect(",")?;let debtor=self.string()?;self.expect(",")?;let creditor=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Liability{name,asset,debtor,creditor,amount}},
            "discharge"=>{self.expect("(")?;let name=self.string()?;self.expect(",")?;let amount=self.expr(0,0)?;self.expect(")")?;Statement::Discharge{name,amount}},
            _=>return Err(self.error("unsupported",format!("Unsupported statement {keyword:?}; use let, requires, ensures, next, emit, transfer, fee, liability or discharge."))),
        };
        self.expect(";")?;
        Ok(s)
    }
}

/// Read the explicit source profile for CLI routing; this does not admit a program.
pub fn language(source: &str) -> Result<&'static str, Error> {
    let lexed = mori_lexer::lex(source).map_err(|e| Error::new("lex", e.to_string()))?;
    let mut tokens = Vec::new();
    for i in 0..lexed.len() {
        let idx = TokenIdx::new(i);
        let span = lexed.span(idx, source);
        let raw = &source[span.start as usize..span.end as usize];
        let kind = lexed.kind(idx);
        tokens.push(Tok {
            text: if kind == TokenKind::String {
                mori_lexer::decode_string(raw)
            } else {
                raw.to_owned()
            },
            kind,
            start: span.start,
        });
    }
    let mut parser = Parser {
        tokens,
        pos: 0,
        program: Program {
            language: LANGUAGE.into(),
            semantics: SEMANTICS.into(),
            name: String::new(),
            states: vec![],
            assets: vec![],
            expressions: vec![],
            actions: vec![],
        },
    };
    let expected = format!(
        "language \"{LANGUAGE}\" or \"{}\"",
        crate::stage::LANGUAGE_2
    );
    parser.expect("language").map_err(|mut error| {
        error.message = format!("Expected {expected}. {}", error.message);
        error
    })?;
    let profile = parser.string().map_err(|mut error| {
        error.message = format!("Expected {expected}. {}", error.message);
        error
    })?;
    match profile.as_str() {
        LANGUAGE => Ok(LANGUAGE),
        crate::stage::LANGUAGE_2 => Ok(crate::stage::LANGUAGE_2),
        _ => Err(parser.error(
            "profile",
            format!("Expected {expected}; found {profile:?}."),
        )),
    }
}
END FILE source/crates/mori-core/src/parser.rs

BEGIN FILE source/crates/mori-core/src/ir.rs | SHA256 0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d | BYTES 12823
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const LANGUAGE: &str = "moriarty/1";
pub const SEMANTICS: &str = "moriarty-core/1";
pub const MAX_EXPRESSIONS: usize = 8192;
pub const MAX_STATEMENTS: usize = 1024;
pub const MAX_DECLARATIONS: usize = 256;
pub const MAX_TRACE_CELLS: usize = 65_536;

/// Unsigned integral atomic units; JSON uses canonical decimal strings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct UInt(pub u128);
impl Serialize for UInt {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}
impl<'de> Deserialize<'de> for UInt {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.is_empty()
            || !s.bytes().all(|b| b.is_ascii_digit())
            || (s.len() > 1 && s.starts_with('0'))
        {
            return Err(serde::de::Error::custom(
                "expected canonical unsigned decimal string",
            ));
        }
        s.parse().map(UInt).map_err(serde::de::Error::custom)
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Type {
    UInt,
    Bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "value", deny_unknown_fields)]
pub enum Value {
    UInt(UInt),
    Bool(bool),
}
impl Value {
    pub fn ty(&self) -> Type {
        match self {
            Self::UInt(_) => Type::UInt,
            Self::Bool(_) => Type::Bool,
        }
    }
}
pub type ExprId = u16;
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Expr {
    LiabilityPresent {
        name: String,
    },
    LiabilityAmount {
        name: String,
    },
    Literal {
        value: Value,
    },
    Name {
        name: String,
    },
    Binary {
        op: BinaryOp,
        lhs: ExprId,
        rhs: ExprId,
    },
    Not {
        value: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct StateDecl {
    pub name: String,
    pub ty: Type,
    pub initial: Value,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InputDecl {
    pub name: String,
    pub ty: Type,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(deny_unknown_fields)]
pub struct AssetId {
    pub network: String,
    pub issuer: String,
    pub reference: String,
    pub scale: u8,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct AssetDecl {
    pub name: String,
    pub identity: AssetId,
}
/// Serialize-only on the shared type: decoding always goes through the frozen
/// `v1` wire, so JSON can never yield `Await`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Statement {
    Await {
        label: String,
        binders: Vec<InputDecl>,
        guard: ExprId,
    },
    Let {
        name: String,
        value: ExprId,
    },
    Requires {
        predicate: ExprId,
    },
    Ensures {
        predicate: ExprId,
    },
    Next {
        state: String,
        value: ExprId,
    },
    Emit {
        name: String,
        value: ExprId,
    },
    Transfer {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Fee {
        asset: String,
        from: String,
        to: String,
        amount: ExprId,
    },
    Liability {
        name: String,
        asset: String,
        debtor: String,
        creditor: String,
        amount: ExprId,
    },
    Discharge {
        name: String,
        amount: ExprId,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Action {
    pub name: String,
    pub inputs: Vec<InputDecl>,
    pub statements: Vec<Statement>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Program {
    pub language: String,
    pub semantics: String,
    pub name: String,
    pub states: Vec<StateDecl>,
    pub assets: Vec<AssetDecl>,
    pub expressions: Vec<Expr>,
    pub actions: Vec<Action>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Balance {
    pub account: String,
    pub asset: AssetId,
    pub amount: UInt,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Liability {
    pub name: String,
    pub asset: AssetId,
    pub debtor: String,
    pub creditor: String,
    pub amount: UInt,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Scenario {
    pub state: BTreeMap<String, Value>,
    pub inputs: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub state: BTreeMap<String, Value>,
    pub balances: Vec<Balance>,
    pub liabilities: Vec<Liability>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum Effect {
    State {
        name: String,
        before: Value,
        after: Value,
    },
    Transfer {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Fee {
        asset: AssetId,
        from: String,
        to: String,
        amount: UInt,
    },
    Liability {
        name: String,
        asset: AssetId,
        debtor: String,
        creditor: String,
        before: UInt,
        after: UInt,
    },
    Emit {
        name: String,
        value: Value,
    },
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub index: u16,
    pub statement: Statement,
    pub value: Option<Value>,
    pub snapshot: Snapshot,
    pub effects: Vec<Effect>,
}
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Execution {
    pub semantics: String,
    pub action: String,
    pub pre: Snapshot,
    pub post: Snapshot,
    pub steps: Vec<Step>,
    pub effects: Vec<Effect>,
}
/// Frozen `moriarty-core/1` decode wire: the baseline (fe01bc8b) derives,
/// moved verbatim. Type names are kept so serde's `expecting` text, unknown
/// variant lists and error messages stay byte-identical. There is no Await.
pub mod v1 {
    use super::{AssetDecl, Effect, Expr, ExprId, InputDecl, Snapshot, StateDecl, Value};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(tag = "kind", deny_unknown_fields)]
    pub enum Statement {
        Let {
            name: String,
            value: ExprId,
        },
        Requires {
            predicate: ExprId,
        },
        Ensures {
            predicate: ExprId,
        },
        Next {
            state: String,
            value: ExprId,
        },
        Emit {
            name: String,
            value: ExprId,
        },
        Transfer {
            asset: String,
            from: String,
            to: String,
            amount: ExprId,
        },
        Fee {
            asset: String,
            from: String,
            to: String,
            amount: ExprId,
        },
        Liability {
            name: String,
            asset: String,
            debtor: String,
            creditor: String,
            amount: ExprId,
        },
        Discharge {
            name: String,
            amount: ExprId,
        },
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Action {
        pub name: String,
        pub inputs: Vec<InputDecl>,
        pub statements: Vec<Statement>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Program {
        pub language: String,
        pub semantics: String,
        pub name: String,
        pub states: Vec<StateDecl>,
        pub assets: Vec<AssetDecl>,
        pub expressions: Vec<Expr>,
        pub actions: Vec<Action>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Step {
        pub index: u16,
        pub statement: Statement,
        pub value: Option<Value>,
        pub snapshot: Snapshot,
        pub effects: Vec<Effect>,
    }
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(deny_unknown_fields)]
    pub struct Execution {
        pub semantics: String,
        pub action: String,
        pub pre: Snapshot,
        pub post: Snapshot,
        pub steps: Vec<Step>,
        pub effects: Vec<Effect>,
    }
}
impl From<v1::Statement> for Statement {
    fn from(s: v1::Statement) -> Self {
        use v1::Statement as W;
        match s {
            W::Let { name, value } => Self::Let { name, value },
            W::Requires { predicate } => Self::Requires { predicate },
            W::Ensures { predicate } => Self::Ensures { predicate },
            W::Next { state, value } => Self::Next { state, value },
            W::Emit { name, value } => Self::Emit { name, value },
            W::Transfer {
                asset,
                from,
                to,
                amount,
            } => Self::Transfer {
                asset,
                from,
                to,
                amount,
            },
            W::Fee {
                asset,
                from,
                to,
                amount,
            } => Self::Fee {
                asset,
                from,
                to,
                amount,
            },
            W::Liability {
                name,
                asset,
                debtor,
                creditor,
                amount,
            } => Self::Liability {
                name,
                asset,
                debtor,
                creditor,
                amount,
            },
            W::Discharge { name, amount } => Self::Discharge { name, amount },
        }
    }
}
impl From<v1::Action> for Action {
    fn from(a: v1::Action) -> Self {
        Self {
            name: a.name,
            inputs: a.inputs,
            statements: a.statements.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<v1::Program> for Program {
    fn from(p: v1::Program) -> Self {
        Self {
            language: p.language,
            semantics: p.semantics,
            name: p.name,
            states: p.states,
            assets: p.assets,
            expressions: p.expressions,
            actions: p.actions.into_iter().map(Into::into).collect(),
        }
    }
}
impl From<v1::Step> for Step {
    fn from(s: v1::Step) -> Self {
        Self {
            index: s.index,
            statement: s.statement.into(),
            value: s.value,
            snapshot: s.snapshot,
            effects: s.effects,
        }
    }
}
impl From<v1::Execution> for Execution {
    fn from(e: v1::Execution) -> Self {
        Self {
            semantics: e.semantics,
            action: e.action,
            pre: e.pre,
            post: e.post,
            steps: e.steps.into_iter().map(Into::into).collect(),
            effects: e.effects,
        }
    }
}
// Every direct, nested or generic decode of a shared type (CLI, replay,
// backend bundles) is routed through `v1`, so no decode site can be missed.
macro_rules! decode_via_v1 {
    ($($ty:ident),*) => {$(
        impl<'de> Deserialize<'de> for $ty {
            fn deserialize<D: serde::Deserializer<'de>>(decoder: D) -> Result<Self, D::Error> {
                v1::$ty::deserialize(decoder).map(Self::from)
            }
        }
    )*};
}
decode_via_v1!(Statement, Action, Program, Step, Execution);
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("{code}: {message}")]
pub struct Error {
    pub code: String,
    pub message: String,
    pub offset: Option<u32>,
    pub step: Option<u16>,
}
impl Error {
    pub(crate) fn new(code: &str, message: impl Into<String>) -> Self {
        Self {
            code: format!("mori::core::{code}"),
            message: message.into(),
            offset: None,
            step: None,
        }
    }
}
END FILE source/crates/mori-core/src/ir.rs

BEGIN FILE source/crates/mori-core/src/json.rs | SHA256 faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183 | BYTES 5391
//! Bounded JSON parsing that rejects duplicate keys before schema decoding.
use crate::Error;
use serde::de::{DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde_json::{Map, Value};
use std::{cell::Cell, fmt};
const MAX_BYTES: usize = 1_048_576;
struct Seed<'a> {
    depth: usize,
    nodes: &'a Cell<usize>,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    type Value = Value;
    fn deserialize<D: serde::Deserializer<'de>>(self, d: D) -> Result<Value, D::Error> {
        use serde::de::Error as _;
        self.nodes.set(self.nodes.get() + 1);
        if self.depth > 32 || self.nodes.get() > 65_536 {
            return Err(D::Error::custom("JSON work/depth limit"));
        }
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    type Value = Value;
    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("bounded JSON")
    }
    fn visit_bool<E: serde::de::Error>(self, v: bool) -> Result<Value, E> {
        Ok(Value::Bool(v))
    }
    fn visit_unit<E: serde::de::Error>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }
    fn visit_i64<E: serde::de::Error>(self, v: i64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_u64<E: serde::de::Error>(self, v: u64) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_f64<E: serde::de::Error>(self, v: f64) -> Result<Value, E> {
        serde_json::Number::from_f64(v)
            .map(Value::Number)
            .ok_or_else(|| E::custom("nonfinite JSON number"))
    }
    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Value, E> {
        Ok(v.into())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        let mut out = Vec::new();
        while let Some(v) = a.next_element_seed(Seed {
            depth: self.depth + 1,
            nodes: self.nodes,
        })? {
            out.push(v)
        }
        Ok(Value::Array(out))
    }
    fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Value, A::Error> {
        use serde::de::Error as _;
        let mut out = Map::new();
        while let Some(k) = a.next_key::<String>()? {
            if out.contains_key(&k) {
                return Err(A::Error::custom("duplicate decoded key"));
            }
            let v = a.next_value_seed(Seed {
                depth: self.depth + 1,
                nodes: self.nodes,
            })?;
            out.insert(k, v);
        }
        Ok(Value::Object(out))
    }
}
/// Strict recursive JSON value decoding for versioned DTO dynamic fields.
/// Duplicate decoded keys and the existing depth/node bounds apply even when
/// a consumer invokes serde directly instead of the bounded text entry point.
pub fn deserialize_strict<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Value, D::Error> {
    let nodes = Cell::new(0);
    Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(decoder)
}
pub fn parse(text: &str) -> Result<Value, Error> {
    parse_inner(text, MAX_BYTES, "JSON exceeds 1 MiB")
}
/// The same strict parser under a caller-selected byte ceiling. Duplicate-key,
/// depth (32) and node (65,536) limits are unchanged; `/2` stage files use it.
pub fn parse_limited(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    parse_inner(text, max, too_large)
}
fn parse_inner(text: &str, max: usize, too_large: &'static str) -> Result<Value, Error> {
    use serde::de::DeserializeSeed;
    if text.len() > max {
        return Err(Error::new("json", too_large));
    }
    let nodes = Cell::new(0);
    let mut decoder = serde_json::Deserializer::from_str(text);
    let value = Seed {
        depth: 0,
        nodes: &nodes,
    }
    .deserialize(&mut decoder)
    .map_err(|e| Error::new("json", e.to_string()))?;
    decoder
        .end()
        .map_err(|e| Error::new("json", e.to_string()))?;
    Ok(value)
}

/// ASCII-safe pretty JSON that fits the same public reader profile, including
/// the final newline added by CLI printing. Parsed values and signing bytes
/// are preserved; terminal-controlled Unicode remains visibly escaped.
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_decoded_keys_and_trailing_data_reject() {
        assert!(parse(r#"{"x":1,"\u0078":2}"#).is_err());
        assert!(parse("{} []").is_err());
        assert!(parse(r#"{"x":"1"}"#).is_ok());
    }
}
END FILE source/crates/mori-core/src/json.rs

BEGIN FILE source/crates/mori-core/src/lib.rs | SHA256 b33afe9a9f7a32d0c4b10211fa139bca5d1a8741ba61ee4df8acebbcd5f8304a | BYTES 1277
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
END FILE source/crates/mori-core/src/lib.rs

BEGIN FILE source/crates/mori-lexer/src/lib.rs | SHA256 5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7 | BYTES 8944
//! Lexer for `moriarty-beta/1` source.
//!
//! Turns source text into [`Tokens`] in a single pass.
//!
//! Lexing stops at the first error. Whitespace is skipped; comments are kept
//! separately for the formatter and do not count toward [`MAX_TOKENS`].

mod diagnostics;
mod number;
mod scan;
mod string;
mod token;

#[cfg(test)]
mod proptests;
#[cfg(test)]
mod tests;

use mori_diagnostics::MoriDiagnostic;
use mori_span::Span;
use soa_rs::Soa;

use crate::number::NumberError;
use crate::string::StringErrorKind;

pub use crate::token::{Comment, Token, TokenIdx, TokenKind, Tokens};

/// Largest accepted source file, in bytes.
pub const MAX_SOURCE_BYTES: usize = 65_536;
/// Most tokens in one file, including the end-of-file token.
pub const MAX_TOKENS: usize = 8_192;
/// Longest identifier or keyword, in bytes.
pub const MAX_IDENT_BYTES: usize = 64;
/// Longest decoded string, in UTF-8 bytes.
pub const MAX_STRING_BYTES: usize = 1_024;

/// The text of a string token, with escapes decoded.
///
/// `raw` must be the full text of a [`TokenKind::String`] token, quotes
/// included, as produced by [`lex`].
pub fn decode_string(raw: &str) -> String {
    let mut text = String::with_capacity(raw.len());
    string::decode_with(raw, |c| text.push(c)).expect("lexed strings are valid");
    text
}

/// Splits `source` into tokens and comments, or reports the first error.
pub fn lex(source: &str) -> Result<Tokens, MoriDiagnostic> {
    if source.len() > MAX_SOURCE_BYTES {
        return Err(diagnostics::source_too_large(source.len()));
    }
    Lexer::new(source).run(false)
}

/// Identify a leading profile token without requiring the remaining source to
/// lex successfully. Inspect only a bounded prefix; full parsing retains all
/// source and token limits.
pub fn first_token_text(source: &str) -> Result<&str, MoriDiagnostic> {
    let end = source.floor_char_boundary(source.len().min(MAX_SOURCE_BYTES));
    let prefix = &source[..end];
    let tokens = Lexer::new(prefix).run(true)?;
    Ok(tokens.span(TokenIdx::new(0), prefix).source_text(prefix))
}

struct Lexer<'src> {
    source: &'src str,
    bytes: &'src [u8],
    pos: usize,
    tokens: Soa<Token>,
    comments: Soa<Comment>,
}

impl<'src> Lexer<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            pos: 0,
            // Roughly one token per eight bytes, as in Zig.
            tokens: Soa::with_capacity(source.len() / 8 + 1),
            comments: Soa::new(),
        }
    }

    fn run(mut self, first_only: bool) -> Result<Tokens, MoriDiagnostic> {
        while let Some(&b) = self.bytes.get(self.pos) {
            let start = self.pos;
            match b {
                b' ' | b'\t' | b'\r' | b'\n' => self.pos += 1,
                b'/' if self.peek(1) == Some(b'/') => self.line_comment(start),
                b'/' if self.peek(1) == Some(b'*') => self.block_comment(start)?,
                b'"' => self.string(start)?,
                b'0'..=b'9' => self.number(start)?,
                b'A'..=b'Z' | b'a'..=b'z' => self.word(start)?,
                _ => match TokenKind::punctuation(b) {
                    Some(kind) => self.push(kind, start, start + 1)?,
                    None => return Err(self.unexpected_character(start)),
                },
            }
            if first_only && !self.tokens.is_empty() {
                break;
            }
        }
        // The end-of-file token is always pushed: the bound reserves room for it.
        self.tokens.push(Token {
            kind: TokenKind::Eof,
            start: offset(self.pos),
        });
        Ok(Tokens {
            tokens: self.tokens,
            comments: self.comments,
        })
    }

    fn peek(&self, ahead: usize) -> Option<u8> {
        self.bytes.get(self.pos + ahead).copied()
    }

    /// Records a token ending at `end` and moves past it.
    fn push(&mut self, kind: TokenKind, start: usize, end: usize) -> Result<(), MoriDiagnostic> {
        if self.tokens.len() >= MAX_TOKENS - 1 {
            return Err(diagnostics::too_many_tokens(span(start, end)));
        }
        self.tokens.push(Token {
            kind,
            start: offset(start),
        });
        self.pos = end;
        Ok(())
    }

    fn push_comment(&mut self, start: usize, end: usize) {
        self.comments.push(Comment {
            start: offset(start),
            end: offset(end),
        });
        self.pos = end;
    }

    /// `//` up to, but not including, the next line break.
    fn line_comment(&mut self, start: usize) {
        let end = self.bytes[start..]
            .iter()
            .position(|&b| b == b'\r' || b == b'\n')
            .map_or(self.bytes.len(), |len| start + len);
        self.push_comment(start, end);
    }

    /// `/*` through the first `*/`. Block comments do not nest.
    fn block_comment(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let body = start + 2;
        let Some(len) = self.source[body..].find("*/") else {
            return Err(diagnostics::unterminated_block_comment(span(start, body)));
        };
        self.push_comment(start, body + len + 2);
        Ok(())
    }

    fn string(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let Some(end) = scan::string_end(self.bytes, start) else {
            return Err(diagnostics::unterminated_string(span(start, start + 1)));
        };
        let raw = &self.source[start..end];
        let mut decoded_len = 0;
        if let Err(error) = string::decode_with(raw, |c| decoded_len += c.len_utf8()) {
            let at = span(start + error.start, start + error.end);
            return Err(match error.kind {
                StringErrorKind::ControlCharacter('\r' | '\n') => {
                    diagnostics::string_line_break(at)
                }
                StringErrorKind::ControlCharacter(c) => {
                    diagnostics::string_control_character(at, c)
                }
                StringErrorKind::InvalidEscape => {
                    diagnostics::invalid_escape(at, at.source_text(self.source))
                }
                StringErrorKind::InvalidUnicodeEscape => diagnostics::invalid_unicode_escape(at),
                StringErrorKind::LoneSurrogate => diagnostics::lone_surrogate(at),
            });
        }
        if decoded_len > MAX_STRING_BYTES {
            return Err(diagnostics::string_too_long(span(start, end), decoded_len));
        }
        self.push(TokenKind::String, start, end)
    }

    fn number(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::number_end(self.bytes, start);
        let raw = &self.source[start..end];
        if let Err(error) = number::validate(raw) {
            return Err(match error {
                NumberError::LeadingZero => diagnostics::number_leading_zero(
                    span(start, end),
                    &number::without_leading_zeros(raw),
                ),
                NumberError::MisplacedUnderscore { at } => {
                    diagnostics::number_misplaced_underscore(span(start + at, start + at + 1))
                }
                NumberError::MissingFraction => {
                    diagnostics::number_missing_fraction(span(end - 1, end))
                }
                NumberError::ExtraDot { at } => {
                    diagnostics::number_extra_dot(span(start + at, start + at + 1))
                }
            });
        }
        self.push(TokenKind::Number, start, end)
    }

    /// An identifier, or a keyword when the text matches one.
    fn word(&mut self, start: usize) -> Result<(), MoriDiagnostic> {
        let end = scan::word_end(self.bytes, start);
        if end - start > MAX_IDENT_BYTES {
            return Err(diagnostics::identifier_too_long(span(start, end)));
        }
        let kind = TokenKind::keyword(&self.source[start..end]).unwrap_or(TokenKind::Ident);
        self.push(kind, start, end)
    }

    fn unexpected_character(&self, start: usize) -> MoriDiagnostic {
        let c = self.source[start..]
            .chars()
            .next()
            .expect("a character remains at the current position");
        let at = span(start, start + c.len_utf8());
        match c {
            '/' => diagnostics::division(at),
            '“' | '”' | '‘' | '’' => diagnostics::curly_quote(at),
            '\'' => diagnostics::single_quote(at),
            '_' => diagnostics::leading_underscore(at),
            c if !c.is_ascii() => diagnostics::non_ascii(at, c),
            c => diagnostics::unexpected_character(at, c),
        }
    }
}

/// Converts a byte index to an offset. Sources fit in `u32` after the bound check.
fn offset(index: usize) -> u32 {
    u32::try_from(index).expect("source offsets fit in u32")
}

fn span(start: usize, end: usize) -> Span {
    Span::new(offset(start), offset(end))
}
END FILE source/crates/mori-lexer/src/lib.rs

BEGIN FILE source/crates/mori-lexer/src/scan.rs | SHA256 4119a98f6ade2d96cb1231b0531cdbb63d01663d24c7e2615ef5b74931db5637 | BYTES 1822
//! Token extent rules, shared by the lexer and by token-end recovery.

use crate::TokenKind;

pub fn is_word_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

/// End of the identifier or keyword starting at `start`.
pub fn word_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| is_word_byte(b))
            .count()
}

/// End of the number starting at `start`. Numbers extend over digits, `_`
/// and `.`; spelling is validated separately.
pub fn number_end(bytes: &[u8], start: usize) -> usize {
    start
        + 1
        + bytes[start + 1..]
            .iter()
            .take_while(|&&b| b.is_ascii_digit() || b == b'_' || b == b'.')
            .count()
}

/// End of the string starting with `"` at `start`, just past the closing
/// quote, or `None` if it is unterminated. A backslash escapes the next byte.
pub fn string_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut escaped = false;
    for (offset, &b) in bytes[start + 1..].iter().enumerate() {
        if b == b'"' && !escaped {
            return Some(start + 1 + offset + 1);
        }
        escaped = b == b'\\' && !escaped;
    }
    None
}

/// Recovers the end offset of a token from its kind and start.
pub fn token_end(kind: TokenKind, bytes: &[u8], start: u32) -> u32 {
    let start_index = start as usize;
    let end = match kind {
        TokenKind::Ident => word_end(bytes, start_index),
        _ if kind.is_keyword() => word_end(bytes, start_index),
        TokenKind::Number => number_end(bytes, start_index),
        TokenKind::String => string_end(bytes, start_index).expect("lexed strings are terminated"),
        TokenKind::Eof => start_index,
        _ => start_index + 1,
    };
    end as u32
}
END FILE source/crates/mori-lexer/src/scan.rs

BEGIN FILE source/crates/mori-lexer/src/token.rs | SHA256 e7860a893c0827bbf362165438c67d621f229f0eacecb9175d959788687bc3db | BYTES 5490
use mori_span::Span;
use soa_rs::{Soa, Soars};

use crate::scan;

/// The kind of a token. Keywords get their own kinds so the parser compares
/// tags instead of text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TokenKind {
    /// A name that is not a keyword: `[A-Za-z][A-Za-z0-9_]*`, at most 64 bytes.
    Ident,
    /// A canonical decimal number, such as `1_000` or `10.50`.
    Number,
    /// A JSON string literal, such as `"Midnight"`.
    String,

    // Keywords stay contiguous from `KwProfile` to `KwShareClass`; see `is_keyword`.
    KwProfile,
    KwAgreement,
    KwAction,
    KwUses,
    KwTrue,
    KwFalse,
    KwNone,
    KwSuccessOnly,

    // Declaration kinds stay contiguous from `KwDomain` to `KwShareClass`; see
    // `is_declaration_kind`.
    KwDomain,
    KwAccount,
    KwAsset,
    KwConst,
    KwIntent,
    KwObligation,
    KwPool,
    KwInstrument,
    KwObservation,
    KwPolicy,
    KwGrant,
    KwStage,
    KwEpisode,
    KwParty,
    KwShareClass,

    LBrace,
    RBrace,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Colon,
    Semicolon,
    Comma,
    Eq,
    Lt,
    Gt,
    Dot,
    Plus,
    Minus,
    Star,

    Eof,
}

impl TokenKind {
    /// The keyword spelled `text`, if any.
    pub fn keyword(text: &str) -> Option<Self> {
        let kind = match text {
            "profile" => Self::KwProfile,
            "agreement" => Self::KwAgreement,
            "action" => Self::KwAction,
            "uses" => Self::KwUses,
            "true" => Self::KwTrue,
            "false" => Self::KwFalse,
            "None" => Self::KwNone,
            "SuccessOnly" => Self::KwSuccessOnly,
            "domain" => Self::KwDomain,
            "account" => Self::KwAccount,
            "asset" => Self::KwAsset,
            "const" => Self::KwConst,
            "intent" => Self::KwIntent,
            "obligation" => Self::KwObligation,
            "pool" => Self::KwPool,
            "instrument" => Self::KwInstrument,
            "observation" => Self::KwObservation,
            "policy" => Self::KwPolicy,
            "grant" => Self::KwGrant,
            "stage" => Self::KwStage,
            "episode" => Self::KwEpisode,
            "party" => Self::KwParty,
            "share_class" => Self::KwShareClass,
            _ => return None,
        };
        Some(kind)
    }

    /// The punctuation token for byte `b`, if any.
    pub fn punctuation(b: u8) -> Option<Self> {
        let kind = match b {
            b'{' => Self::LBrace,
            b'}' => Self::RBrace,
            b'[' => Self::LBracket,
            b']' => Self::RBracket,
            b'(' => Self::LParen,
            b')' => Self::RParen,
            b':' => Self::Colon,
            b';' => Self::Semicolon,
            b',' => Self::Comma,
            b'=' => Self::Eq,
            b'<' => Self::Lt,
            b'>' => Self::Gt,
            b'.' => Self::Dot,
            b'+' => Self::Plus,
            b'-' => Self::Minus,
            b'*' => Self::Star,
            _ => return None,
        };
        Some(kind)
    }

    pub fn is_keyword(self) -> bool {
        (Self::KwProfile as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Keywords that start a declaration, such as `const` or `asset`.
    pub fn is_declaration_kind(self) -> bool {
        (Self::KwDomain as u8..=Self::KwShareClass as u8).contains(&(self as u8))
    }

    /// Identifiers and keywords. Record fields and call names accept either.
    pub fn is_word(self) -> bool {
        self == Self::Ident || self.is_keyword()
    }
}

/// A token's kind and start offset. Its end is recovered by re-scanning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub start: u32,
}

/// A `//` or `/* */` comment, kept for the formatter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Soars)]
#[soa_derive(Debug, PartialEq)]
pub struct Comment {
    pub start: u32,
    pub end: u32,
}

/// Index of a token in [`Tokens`]. Sources hold at most 8,192 tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenIdx(u16);

impl TokenIdx {
    pub fn new(index: usize) -> Self {
        Self(u16::try_from(index).expect("token index exceeds u16"))
    }

    pub fn index(self) -> usize {
        usize::from(self.0)
    }
}

/// The output of [`lex`](crate::lex): every token, ending with
/// [`TokenKind::Eof`], and every comment, both in source order.
#[derive(Debug, PartialEq)]
pub struct Tokens {
    pub tokens: Soa<Token>,
    pub comments: Soa<Comment>,
}

impl Tokens {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Always false: a token list ends with [`TokenKind::Eof`].
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn kind(&self, idx: TokenIdx) -> TokenKind {
        self.tokens.kind()[idx.index()]
    }

    pub fn start(&self, idx: TokenIdx) -> u32 {
        self.tokens.start()[idx.index()]
    }

    /// The token's span, recovered by re-scanning `source` from its start.
    pub fn span(&self, idx: TokenIdx, source: &str) -> Span {
        let start = self.start(idx);
        Span::new(
            start,
            scan::token_end(self.kind(idx), source.as_bytes(), start),
        )
    }

    pub fn comment_span(&self, index: usize) -> Span {
        Span::new(self.comments.start()[index], self.comments.end()[index])
    }
}
END FILE source/crates/mori-lexer/src/token.rs

BEGIN FILE source/crates/mori-lexer/src/string.rs | SHA256 b94878f41914f56dfd4945ebb0b44690db541e77848817bc6e265dc422f88aaa | BYTES 4862
//! JSON string literals.
//!
//! Strings use JSON syntax: no raw control characters, and only the escapes
//! `\" \\ \/ \b \f \n \r \t \uXXXX`. A `\u` escape that encodes half of a
//! surrogate pair without its other half is rejected, but only after the
//! whole literal is valid JSON, matching the reference implementation.

use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringErrorKind {
    /// A raw character below U+0020, such as a line break.
    ControlCharacter(char),
    /// A backslash followed by a character that is not a JSON escape.
    InvalidEscape,
    /// `\u` not followed by four hexadecimal digits.
    InvalidUnicodeEscape,
    /// A `\u` escape for half of a surrogate pair, without its other half.
    LoneSurrogate,
}

/// An error at byte range `start..end` of the literal, quotes included.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringError {
    pub kind: StringErrorKind,
    pub start: usize,
    pub end: usize,
}

/// Decodes the literal `raw`, quotes included, passing each character to
/// `emit`. `raw` must start and end with an unescaped `"`.
pub fn decode_with(raw: &str, mut emit: impl FnMut(char)) -> Result<(), StringError> {
    let body = &raw[1..raw.len() - 1];
    let mut chars = body.char_indices().peekable();
    let mut pending_high: Option<(u32, usize)> = None;
    let mut first_lone: Option<(usize, usize)> = None;

    let error = |kind, start: usize, end: usize| StringError {
        kind,
        start: start + 1,
        end: end + 1,
    };

    while let Some((i, c)) = chars.next() {
        if c == '\\' {
            let Some((_, escape)) = chars.next() else {
                return Err(error(StringErrorKind::InvalidEscape, i, i + 1));
            };
            if escape == 'u' {
                let unit = hex4(&mut chars).ok_or_else(|| {
                    let end = chars.peek().map_or(body.len(), |&(j, _)| j);
                    error(StringErrorKind::InvalidUnicodeEscape, i, end)
                })?;
                let end = i + 6;
                match (pending_high.take(), unit) {
                    (Some((high, _)), 0xDC00..=0xDFFF) => {
                        let scalar = 0x10000 + ((high - 0xD800) << 10) + (unit - 0xDC00);
                        emit(char::from_u32(scalar).expect("surrogate pair is a scalar"));
                    }
                    (pending, _) => {
                        if let Some((_, start)) = pending {
                            first_lone.get_or_insert((start, start + 6));
                            emit(char::REPLACEMENT_CHARACTER);
                        }
                        match unit {
                            0xD800..=0xDBFF => pending_high = Some((unit, i)),
                            0xDC00..=0xDFFF => {
                                first_lone.get_or_insert((i, end));
                                emit(char::REPLACEMENT_CHARACTER);
                            }
                            _ => emit(char::from_u32(unit).expect("non-surrogate is a scalar")),
                        }
                    }
                }
                continue;
            }

            let decoded = match escape {
                '"' => '"',
                '\\' => '\\',
                '/' => '/',
                'b' => '\u{8}',
                'f' => '\u{c}',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                _ => {
                    let end = i + 1 + escape.len_utf8();
                    return Err(error(StringErrorKind::InvalidEscape, i, end));
                }
            };
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(decoded);
        } else if c < ' ' {
            return Err(error(StringErrorKind::ControlCharacter(c), i, i + 1));
        } else {
            flush_high(&mut pending_high, &mut first_lone, &mut emit);
            emit(c);
        }
    }
    flush_high(&mut pending_high, &mut first_lone, &mut emit);

    match first_lone {
        Some((start, end)) => Err(error(StringErrorKind::LoneSurrogate, start, end)),
        None => Ok(()),
    }
}

/// A high surrogate not followed by a low surrogate escape is lone.
fn flush_high(
    pending_high: &mut Option<(u32, usize)>,
    first_lone: &mut Option<(usize, usize)>,
    emit: &mut impl FnMut(char),
) {
    if let Some((_, start)) = pending_high.take() {
        first_lone.get_or_insert((start, start + 6));
        emit(char::REPLACEMENT_CHARACTER);
    }
}

/// Reads exactly four hexadecimal digits.
fn hex4(chars: &mut Peekable<CharIndices<'_>>) -> Option<u32> {
    let mut value = 0;
    for _ in 0..4 {
        let digit = chars.peek()?.1.to_digit(16)?;
        chars.next();
        value = value * 16 + digit;
    }
    Some(value)
}
END FILE source/crates/mori-lexer/src/string.rs

BEGIN FILE source/crates/mori-lexer/src/number.rs | SHA256 5826e09117b22505a74ab3144f4862445b513039bac275c88ace4d85331fad9a | BYTES 2329
//! Canonical number spelling.
//!
//! A number is an integer part, optionally followed by `.` and a fraction
//! part. The integer part is `0` or starts with `1`–`9`. In both parts `_` may
//! only separate two digits. This matches the reference spelling
//! `(0|[1-9][0-9]*(_[0-9]+)*)(\.[0-9]+(_[0-9]+)*)?`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NumberError {
    /// The integer part has more than one digit and starts with `0`.
    LeadingZero,
    /// A `_` at this byte offset is not between two digits.
    MisplacedUnderscore { at: usize },
    /// The `.` at the end has no digits after it.
    MissingFraction,
    /// A second `.` at this byte offset.
    ExtraDot { at: usize },
}

/// Checks the spelling of a number scanned by `scan::number_end`.
pub fn validate(raw: &str) -> Result<(), NumberError> {
    let mut dots = raw.match_indices('.').map(|(at, _)| at);
    let dot = dots.next();
    if let Some(at) = dots.next() {
        return Err(NumberError::ExtraDot { at });
    }

    let integer = &raw[..dot.unwrap_or(raw.len())];
    if integer.len() > 1 && integer.starts_with('0') {
        return Err(NumberError::LeadingZero);
    }
    check_underscores(integer, 0)?;

    if let Some(dot) = dot {
        let fraction = &raw[dot + 1..];
        if fraction.is_empty() {
            return Err(NumberError::MissingFraction);
        }
        check_underscores(fraction, dot + 1)?;
    }
    Ok(())
}

/// `part` contains only digits and `_` and starts at `offset` in the number.
fn check_underscores(part: &str, offset: usize) -> Result<(), NumberError> {
    let bytes = part.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        let between_digits = i > 0
            && bytes[i - 1].is_ascii_digit()
            && bytes.get(i + 1).is_some_and(u8::is_ascii_digit);
        if b == b'_' && !between_digits {
            return Err(NumberError::MisplacedUnderscore { at: offset + i });
        }
    }
    Ok(())
}

/// The number with redundant leading zeros removed, for suggestions.
pub fn without_leading_zeros(raw: &str) -> String {
    let (integer, rest) = raw.split_at(raw.find('.').unwrap_or(raw.len()));
    let trimmed = integer.trim_start_matches(['0', '_']);
    let integer = if trimmed.is_empty() { "0" } else { trimmed };
    format!("{integer}{rest}")
}
END FILE source/crates/mori-lexer/src/number.rs

BEGIN FILE source/crates/mori/Cargo.toml | SHA256 2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b | BYTES 2883
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
END FILE source/crates/mori/Cargo.toml

BEGIN FILE source/crates/mori-core/Cargo.toml | SHA256 93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c | BYTES 881
[package]
name = "mori-core"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true

[features]
core2-disjoint-draft = []
# Opt-in and non-default: enables only the Midnight-native signing-frame
# compatibility test. Ordinary mori-core consumers and SDK-free test
# selection never pull these dependencies.
midnight-compat-tests = ["dep:midnight-base-crypto", "dep:midnight-serialize"]

[dependencies]
mori-lexer.workspace = true
serde.workspace = true
serde_json.workspace = true
thiserror.workspace = true
sha2.workspace = true
k256.workspace = true
# Optional and off by default; pulled in only by the midnight-compat-tests
# feature for the native compatibility test.
midnight-base-crypto = { workspace = true, optional = true }
midnight-serialize = { workspace = true, optional = true }

[dev-dependencies]
proptest.workspace = true
END FILE source/crates/mori-core/Cargo.toml

BEGIN FILE source/crates/mori-core/src/stage/plan.rs | SHA256 9d8e71b2d5089bf3221b3e06454038357559a9eddab8245464d55b19653d1e1c | BYTES 4177
//! Source-derived scalar layouts. Admission and work costs belong to sibling modules.
use crate::{Error, InputDecl, Program, Statement, Type, UInt};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Origin {
    Input,
    Let,
    Event,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CaptureSlot {
    pub name: String,
    pub ty: Type,
    pub origin: Origin,
    pub ordinal: u16,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageBinder {
    pub name: String,
    pub ty: Type,
}
impl From<&InputDecl> for StageBinder {
    fn from(value: &InputDecl) -> Self {
        Self {
            name: value.name.clone(),
            ty: value.ty,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AwaitSite {
    pub site: u16,
    pub ordinal: u16,
    pub label: String,
    pub binders: Vec<StageBinder>,
    pub captures: Vec<CaptureSlot>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionPlan {
    pub action: String,
    pub statement_count: u16,
    pub sites: Vec<AwaitSite>,
    pub segment_cost: Vec<UInt>,
    pub committed_before: Vec<UInt>,
    pub plan_total: UInt,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct StagePlan {
    pub format: String,
    pub program_hash: String,
    pub actions: Vec<ActionPlan>,
}

/// Builds layouts after validate_v2. Costs and hashes are filled by stage::load.
pub(crate) fn build(program: &Program) -> Result<StagePlan, Error> {
    let mut actions = Vec::new();
    for action in &program.actions {
        let mut types: BTreeMap<_, _> = program
            .states
            .iter()
            .map(|state| (state.name.clone(), state.ty))
            .collect();
        let mut captures = Vec::new();
        for (ordinal, input) in action.inputs.iter().enumerate() {
            types.insert(input.name.clone(), input.ty);
            captures.push(CaptureSlot {
                name: input.name.clone(),
                ty: input.ty,
                origin: Origin::Input,
                ordinal: ordinal as u16,
            });
        }
        let mut sites = Vec::new();
        for (ordinal, statement) in action.statements.iter().enumerate() {
            match statement {
                Statement::Let { name, value } => {
                    let ty = crate::runtime::expr_type(program, *value, &types, 0)?;
                    types.insert(name.clone(), ty);
                    captures.push(CaptureSlot {
                        name: name.clone(),
                        ty,
                        origin: Origin::Let,
                        ordinal: ordinal as u16,
                    });
                }
                Statement::Await { label, binders, .. } => {
                    sites.push(AwaitSite {
                        site: sites.len() as u16 + 1,
                        ordinal: ordinal as u16,
                        label: label.clone(),
                        binders: binders.iter().map(StageBinder::from).collect(),
                        captures: captures.clone(),
                    });
                    for binder in binders {
                        types.insert(binder.name.clone(), binder.ty);
                        captures.push(CaptureSlot {
                            name: binder.name.clone(),
                            ty: binder.ty,
                            origin: Origin::Event,
                            ordinal: ordinal as u16,
                        });
                    }
                }
                _ => {}
            }
        }
        actions.push(ActionPlan {
            action: action.name.clone(),
            statement_count: action.statements.len() as u16,
            sites,
            segment_cost: Vec::new(),
            committed_before: Vec::new(),
            plan_total: UInt(0),
        });
    }
    Ok(StagePlan {
        format: "moriarty-stage-plan/1".into(),
        program_hash: String::new(),
        actions,
    })
}
END FILE source/crates/mori-core/src/stage/plan.rs

BEGIN FILE source/crates/mori-core/src/stage/profile.rs | SHA256 933560466065d78b0ce3f3559c2cf2bb9cbdc8eea45903af564cf1ab469cd59d | BYTES 1237
//! Profile isolation between `moriarty-core/1` and `moriarty-core/2` consumers.
//! Both helpers keep the existing `mori::core::profile` error identity.
use super::{LANGUAGE_2, SEMANTICS_2};
use crate::{Error, LANGUAGE, Program, SEMANTICS, Statement};

const PROFILE_MESSAGE: &str = "Unsupported language or semantics identity.";

/// Required at every `/1` entry point before evaluation or layout. An
/// in-memory program labelled `/1` that still carries an Await is refused.
pub fn require_core1(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    if program
        .actions
        .iter()
        .flat_map(|action| &action.statements)
        .any(|statement| matches!(statement, Statement::Await { .. }))
    {
        return Err(Error::new("profile", "Await requires moriarty-core/2."));
    }
    Ok(())
}

/// Required at every staged entry point; `/1` programs are refused.
pub fn require_core2(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_2 || program.semantics != SEMANTICS_2 {
        return Err(Error::new("profile", PROFILE_MESSAGE));
    }
    Ok(())
}
END FILE source/crates/mori-core/src/stage/profile.rs

BEGIN FILE source/crates/mori-core/src/stage/validate.rs | SHA256 d301530692f2f02a51bb68cf8b30ee51db6d3f3c9019b9045ed2a50e3ad8debf | BYTES 18486
//! `/2` static validation and stage plan invariants.
//!
//! `validate_v2` applies every legacy `/1` program check with the legacy codes
//! and messages, plus the Await rules: top-level placement (structural in IR),
//! unique labels, non-shadowing binders, Bool guards typed over the prefix and
//! the Await's own binders, the per-action bounds, and `await`/`when` reserved.
//! Legacy `validate` is unchanged and remains `/1`-only.
use super::{
    AwaitSite, CaptureSlot, LANGUAGE_2, MAX_AWAITS_PER_ACTION, MAX_CAPTURE_SLOTS,
    MAX_EVENT_BINDERS, Origin, SEMANTICS_2, STAGE_PLAN_FORMAT, StageBinder, StagePlan,
};
use crate::runtime::{
    expr_type, identifier, require_uint, validate_asset, validate_liability_reads,
};
use crate::{
    Error, Expr, MAX_DECLARATIONS, MAX_EXPRESSIONS, MAX_STATEMENTS, Program, Statement, Type, UInt,
};
use std::collections::{BTreeMap, BTreeSet};

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}

fn invariant(what: &str) -> Error {
    failure("program", format!("Stage plan invariant violated: {what}."))
}

/// Legacy identifier rule with `await` and `when` also reserved (C6).
fn identifier_v2(name: &str) -> bool {
    identifier(name) && !matches!(name, "await" | "when")
}

fn names_v2<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for name in values {
        if !identifier_v2(name) || !seen.insert(name) {
            return Err(failure(
                "duplicate",
                format!("Invalid or duplicate {kind} name {name:?}."),
            ));
        }
    }
    Ok(())
}

/// Full finite validation of a `moriarty-core/2` program.
pub(crate) fn validate_v2(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE_2 || program.semantics != SEMANTICS_2 {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    if !identifier_v2(&program.name) {
        return Err(failure(
            "name",
            "Agreement name must be a non-reserved ASCII identifier of at most 64 bytes.",
        ));
    }
    if program.actions.is_empty() {
        return Err(failure("program", "Program needs at least one action."));
    }
    if program.states.len() + program.assets.len() + program.actions.len() > MAX_DECLARATIONS
        || program.expressions.len() > MAX_EXPRESSIONS
    {
        return Err(failure(
            "bound",
            "Program exceeds finite declaration/expression bounds.",
        ));
    }
    names_v2(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names_v2(program.actions.iter().map(|a| a.name.as_str()), "action")?;
    let mut identities = BTreeSet::new();
    for asset in &program.assets {
        let id = &asset.identity;
        if id.network.is_empty()
            || id.issuer.is_empty()
            || id.reference.is_empty()
            || id.network.len() > 1024
            || id.issuer.len() > 1024
            || id.reference.len() > 1024
            || id.scale > 38
            || !identities.insert(id)
        {
            return Err(failure(
                "asset",
                "Asset identities must be complete, unique and use scale 0..38.",
            ));
        }
        if program.assets.iter().any(|other| {
            other.identity.network == id.network
                && other.identity.issuer == id.issuer
                && other.identity.reference == id.reference
                && other.identity.scale != id.scale
        }) {
            return Err(failure(
                "asset",
                "Same asset identity cannot have conflicting scales.",
            ));
        }
    }
    let mut depths = Vec::with_capacity(program.expressions.len());
    for (i, e) in program.expressions.iter().enumerate() {
        let refs = match e {
            Expr::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
            Expr::Not { value } => vec![*value],
            _ => vec![],
        };
        let mut depth = 0;
        for r in refs {
            if usize::from(r) >= i {
                return Err(failure(
                    "expression",
                    "Expression references must point to earlier nodes; cycles are forbidden.",
                ));
            }
            depth = depth.max(depths[usize::from(r)] + 1);
        }
        if depth > 64 {
            return Err(failure("bound", "Expression depth exceeds 64."));
        }
        depths.push(depth);
    }
    let state_types: BTreeMap<_, _> = program
        .states
        .iter()
        .map(|s| (s.name.clone(), s.ty))
        .collect();
    for state in &program.states {
        if state.initial.ty() != state.ty {
            return Err(failure(
                "type",
                format!("Initial state {:?} has wrong type.", state.name),
            ));
        }
    }
    for action in &program.actions {
        if action.inputs.len() > MAX_DECLARATIONS || action.statements.len() > MAX_STATEMENTS {
            return Err(failure(
                "bound",
                "Action exceeds input or statement bounds.",
            ));
        }
        names_v2(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        let mut labels = BTreeSet::new();
        let mut awaits = 0_usize;
        // Inputs, lets and earlier binders: the captures of the next site.
        let mut captured = action.inputs.len();
        for statement in &action.statements {
            match statement {
                Statement::Await {
                    label,
                    binders,
                    guard,
                } => {
                    awaits += 1;
                    if awaits > MAX_AWAITS_PER_ACTION {
                        return Err(failure("bound", "Action exceeds 16 awaits."));
                    }
                    if binders.len() > MAX_EVENT_BINDERS {
                        return Err(failure("bound", "Await exceeds 16 event binders."));
                    }
                    if captured > MAX_CAPTURE_SLOTS {
                        return Err(failure("bound", "Await site exceeds 256 capture slots."));
                    }
                    if !identifier_v2(label) || !labels.insert(label.as_str()) {
                        return Err(failure(
                            "duplicate",
                            format!("Invalid or duplicate await label {label:?}."),
                        ));
                    }
                    for binder in binders {
                        if !identifier_v2(&binder.name)
                            || env.insert(binder.name.clone(), binder.ty).is_some()
                            || program.assets.iter().any(|a| a.name == binder.name)
                        {
                            return Err(failure(
                                "duplicate",
                                "Await binder cannot shadow an existing name.",
                            ));
                        }
                    }
                    if expr_type(program, *guard, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Await guard must have type Bool."));
                    }
                    captured += binders.len();
                }
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier_v2(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
                    captured += 1;
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    if expr_type(program, *predicate, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Predicate must have type Bool."));
                    }
                }
                Statement::Next { state, value } => {
                    let expected = state_types
                        .get(state)
                        .ok_or_else(|| failure("name", format!("Unknown state {state:?}.")))?;
                    if expr_type(program, *value, &env, 0)? != *expected {
                        return Err(failure("type", "State update has wrong type."));
                    }
                }
                Statement::Emit { name, value } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure("name", "Event name must contain 1..1024 bytes."));
                    }
                    expr_type(program, *value, &env, 0)?;
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if from.is_empty() || to.is_empty() || from.len() > 1024 || to.len() > 1024 {
                        return Err(failure("account", "Account identities cannot be empty."));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if name.is_empty()
                        || debtor.is_empty()
                        || creditor.is_empty()
                        || name.len() > 1024
                        || debtor.len() > 1024
                        || creditor.len() > 1024
                    {
                        return Err(failure(
                            "liability",
                            "Liability identities cannot be empty.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Discharge { name, amount } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure(
                            "liability",
                            "Liability name must contain 1..1024 bytes.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
            }
        }
    }
    validate_liability_reads(program)?;
    Ok(())
}

/// Independently re-derives each action's sites and capture layouts and
/// requires the Sol-built plan to equal them, then checks the A3 predicates
/// explicitly. Runs before costing, so cost vectors and hash must be empty.
pub(crate) fn plan_invariants(program: &Program, plan: &StagePlan) -> Result<(), Error> {
    if plan.format != STAGE_PLAN_FORMAT {
        return Err(invariant("format"));
    }
    if !plan.program_hash.is_empty() {
        return Err(invariant("program hash filled before costing"));
    }
    if plan.actions.len() != program.actions.len() {
        return Err(invariant("action count"));
    }
    let state_names: BTreeSet<_> = program.states.iter().map(|s| s.name.as_str()).collect();
    for (action, ap) in program.actions.iter().zip(&plan.actions) {
        if ap.action != action.name || usize::from(ap.statement_count) != action.statements.len() {
            return Err(invariant("action identity"));
        }
        if !ap.segment_cost.is_empty()
            || !ap.committed_before.is_empty()
            || ap.plan_total != UInt(0)
        {
            return Err(invariant("costs filled before costing"));
        }
        // Independent derivation in declaration and statement order.
        let mut types: BTreeMap<_, _> = program
            .states
            .iter()
            .map(|s| (s.name.clone(), s.ty))
            .collect();
        let mut slots = Vec::with_capacity(action.inputs.len());
        for (index, input) in action.inputs.iter().enumerate() {
            types.insert(input.name.clone(), input.ty);
            slots.push(CaptureSlot {
                name: input.name.clone(),
                ty: input.ty,
                origin: Origin::Input,
                ordinal: u16::try_from(index).map_err(|_| invariant("input ordinal"))?,
            });
        }
        let mut expected = Vec::new();
        for (index, statement) in action.statements.iter().enumerate() {
            let ordinal = u16::try_from(index).map_err(|_| invariant("statement ordinal"))?;
            match statement {
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &types, 0)?;
                    types.insert(name.clone(), ty);
                    slots.push(CaptureSlot {
                        name: name.clone(),
                        ty,
                        origin: Origin::Let,
                        ordinal,
                    });
                }
                Statement::Await { label, binders, .. } => {
                    expected.push(AwaitSite {
                        site: u16::try_from(expected.len() + 1).map_err(|_| invariant("site"))?,
                        ordinal,
                        label: label.clone(),
                        binders: binders.iter().map(StageBinder::from).collect(),
                        captures: slots.clone(),
                    });
                    for binder in binders {
                        types.insert(binder.name.clone(), binder.ty);
                        slots.push(CaptureSlot {
                            name: binder.name.clone(),
                            ty: binder.ty,
                            origin: Origin::Event,
                            ordinal,
                        });
                    }
                }
                _ => {}
            }
        }
        if ap.sites != expected {
            return Err(invariant("sites differ from source"));
        }
        // Explicit A3 predicates, stated separately from the derivation.
        if ap.sites.len() > MAX_AWAITS_PER_ACTION {
            return Err(failure("bound", "Action exceeds 16 awaits."));
        }
        let mut previous: Option<&AwaitSite> = None;
        for (index, site) in ap.sites.iter().enumerate() {
            if usize::from(site.site) != index + 1 {
                return Err(invariant("sites are not numbered 1..N"));
            }
            if previous.is_some_and(|p| p.ordinal >= site.ordinal) {
                return Err(invariant("ordinals are not strictly increasing"));
            }
            if site.binders.len() > MAX_EVENT_BINDERS || site.captures.len() > MAX_CAPTURE_SLOTS {
                return Err(failure("bound", "Await site exceeds stage layout bounds."));
            }
            let inputs = action.inputs.len();
            if site.captures.len() < inputs
                || site.captures[..inputs]
                    .iter()
                    .zip(&action.inputs)
                    .enumerate()
                    .any(|(i, (slot, input))| {
                        slot.origin != Origin::Input
                            || slot.name != input.name
                            || slot.ty != input.ty
                            || usize::from(slot.ordinal) != i
                    })
            {
                return Err(invariant("captures do not begin with all inputs"));
            }
            let mut seen = BTreeSet::new();
            for slot in &site.captures {
                if state_names.contains(slot.name.as_str())
                    || !seen.insert(slot.name.as_str())
                    || (slot.origin != Origin::Input && slot.ordinal >= site.ordinal)
                {
                    return Err(invariant(
                        "capture names a state, repeats or follows its site",
                    ));
                }
            }
            if let Some(p) = previous
                && !site.captures.starts_with(&p.captures)
            {
                return Err(invariant("captures are not prefix-monotone"));
            }
            previous = Some(site);
        }
    }
    Ok(())
}

/// Checks the filled cost vectors: N+1 entries, prefix sums, and a total that
/// also equals the per-statement sum plus one entry charge per await.
pub(crate) fn cost_invariants(program: &Program, plan: &StagePlan) -> Result<(), Error> {
    let overflow = || failure("bound", "Stage logical plan exceeds u128.");
    for (action, ap) in program.actions.iter().zip(&plan.actions) {
        let segments = ap.sites.len() + 1;
        if ap.segment_cost.len() != segments || ap.committed_before.len() != segments {
            return Err(invariant("cost vector lengths"));
        }
        let mut running = 0_u128;
        for (cost, before) in ap.segment_cost.iter().zip(&ap.committed_before) {
            if before.0 != running {
                return Err(invariant("committed_before is not a prefix sum"));
            }
            running = running.checked_add(cost.0).ok_or_else(overflow)?;
        }
        if ap.plan_total.0 != running {
            return Err(invariant("plan total"));
        }
        let mut independent = u128::try_from(ap.sites.len()).map_err(|_| overflow())?;
        for statement in &action.statements {
            let cost = super::cost::statement_cost(program, statement)?;
            independent = independent.checked_add(cost.0).ok_or_else(overflow)?;
        }
        if independent != running {
            return Err(invariant("plan total differs from statement costs"));
        }
    }
    Ok(())
}
END FILE source/crates/mori-core/src/stage/validate.rs

BEGIN FILE source/crates/mori-core/src/stage/digest.rs | SHA256 8193f18852499f0e3d276e2187f57fbb1fc6bf5ea8faa51a40510919655fdf72 | BYTES 1206
//! Canonical program and plan digests. Both hash `intent::canonical`, the
//! same encoding the backend uses for its `/3` program hash.
use super::StagePlan;
use crate::{Error, Program};
use sha2::{Digest, Sha256};

fn digest<T: serde::Serialize>(value: &T) -> Result<[u8; 32], Error> {
    Ok(Sha256::digest(crate::intent::canonical(value)?).into())
}

pub(crate) fn hex(hash: &[u8; 32]) -> String {
    crate::intent::hex(hash)
}

/// Exact comparison against the canonical 64-character lowercase encoding.
pub(crate) fn matches(text: &str, hash: &[u8; 32]) -> bool {
    text == hex(hash)
}

pub(crate) fn program_hash(program: &Program) -> Result<[u8; 32], Error> {
    digest(program)
}

/// Hashes the costed plan, which already names its program hash and format.
pub(crate) fn plan_hash(plan: &StagePlan) -> Result<[u8; 32], Error> {
    let filled = plan.program_hash.len() == 64
        && plan
            .program_hash
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if !filled {
        return Err(Error::new(
            "program",
            "Stage plan invariant violated: program hash is not filled.",
        ));
    }
    digest(plan)
}
END FILE source/crates/mori-core/src/stage/digest.rs

BEGIN FILE source/rust-toolchain.toml | SHA256 72f7dd8c9fdf9c1fcd8eb8bf9326d4bc66833e4ebf4bc2932636e2b85efcb7af | BYTES 86
[toolchain]
channel = "1.99.0"
components = ["rustfmt", "clippy"]
profile = "minimal"
END FILE source/rust-toolchain.toml

BEGIN FILE units/core_io-read-source.rs | SHA256 e8f1a0d5cd1c2eeee11708b9ceaab206b8f08435fd6e072259edc1932203ce33 | BYTES 530
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
END FILE units/core_io-read-source.rs

BEGIN FILE units/core_io-operation.rs | SHA256 51d63b55691c08ef387fc01056e995f95500a286660ef79ec6fea42edb8d9f6c | BYTES 221
pub fn operation(error: impl std::fmt::Display) -> mori_diagnostics::Report {
    CliError::Operation {
        code: "mori::command".into(),
        message: crate::display::text(&error.to_string()),
    }
    .into()
}
END FILE units/core_io-operation.rs

BEGIN FILE units/core_io-print.rs | SHA256 7f1351583d72965b909c84ede5884a48fba83dc868dde6f57ebcd4f165a868cd | BYTES 171
pub fn print<T: Serialize>(value: &T) -> super::Result<()> {
    let display = mori_core::json::export(value).map_err(operation)?;
    println!("{display}");
    Ok(())
}
END FILE units/core_io-print.rs

BEGIN FILE units/source-hash-bytes.rs | SHA256 035b8ce048dc09ecbcd5477c5d15b942bf9161e9b4ab65097b9e04451a0ff295 | BYTES 78
pub fn hash_bytes(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
END FILE units/source-hash-bytes.rs

BEGIN FILE units/cli-global-options.rs | SHA256 d39eb77f73e940188e0746b96231f679ee88c9bad30c4072a49a7e6761137fd2 | BYTES 684
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
END FILE units/cli-global-options.rs

BEGIN FILE units/cli-color-conversion.rs | SHA256 b3040b38b5cfa64baa23e2c50da404635b86adc6c7ca5b9d61bc049827df1fc5 | BYTES 338
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl From<Color> for anstream::ColorChoice {
    fn from(color: Color) -> Self {
        match color {
            Color::Auto => Self::Auto,
            Color::Always => Self::Always,
            Color::Never => Self::Never,
        }
    }
}
END FILE units/cli-color-conversion.rs

BEGIN FILE units/stage-load-complete-and-used-types.rs | SHA256 5318efe2b81d92855ef66f047b3a75f2e3d35ce9925a0f9a40035a181abfb14e | BYTES 3238
//! Staged `moriarty/2` programs and their local continuation interface.
//!
//! Local and unsigned only. Event binders and captures are caller-chosen values
//! (no authentication), a continuation can be resumed any number of times (no
//! consumption tracking), and no stage authority exists. Outputs carry
//! `STAGE_LABELS` so these limits stay visible.
mod admit;
mod cost;
pub mod decode;
mod digest;
#[cfg(feature = "core2-disjoint-draft")]
pub mod disjoint;
mod plan;
mod profile;
mod report;
mod validate;

pub use crate::LOGICAL_COST_PROFILE_2;
pub use admit::{resume, start};
pub use plan::{ActionPlan, AwaitSite, CaptureSlot, Origin, StageBinder, StagePlan};
pub use profile::{require_core1, require_core2};
pub use report::outcome_report;

use crate::{Error, Execution, Liability, Program, Snapshot, UInt, Value};
use serde::{Deserialize, Serialize};

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
END FILE units/stage-load-complete-and-used-types.rs

BEGIN FILE units/core-static-validation-and-type-helpers.rs | SHA256 fc4a4130fb948baf36fb7abea805be40d9cde2e06c19a244493ec721fca30d51 | BYTES 13850
use crate::policy::{CostPlane, Meter};
use crate::*;
use std::collections::{BTreeMap, BTreeSet};

fn failure(code: &str, message: impl Into<String>) -> Error {
    Error::new(code, message)
}
pub(crate) fn identifier(name: &str) -> bool {
    let mut bytes = name.bytes();
    matches!(bytes.next(), Some(b'A'..=b'Z' | b'a'..=b'z'))
        && name.len() <= 64
        && bytes.all(|b| b.is_ascii_alphanumeric() || b == b'_')
        && !matches!(
            name,
            "language"
                | "agreement"
                | "state"
                | "asset"
                | "action"
                | "input"
                | "let"
                | "next"
                | "requires"
                | "ensures"
                | "emit"
                | "transfer"
                | "fee"
                | "liability"
                | "discharge"
                | "true"
                | "false"
                | "and"
                | "or"
                | "not"
                | "UInt"
                | "Bool"
        )
}
fn names<'a>(values: impl Iterator<Item = &'a str>, kind: &str) -> Result<(), Error> {
    let mut seen = BTreeSet::new();
    for name in values {
        if !identifier(name) || !seen.insert(name) {
            return Err(failure(
                "duplicate",
                format!("Invalid or duplicate {kind} name {name:?}."),
            ));
        }
    }
    Ok(())
}
pub(crate) fn expr_type(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
) -> Result<Type, Error> {
    type_cached(
        program,
        id,
        env,
        depth,
        &mut vec![None; program.expressions.len()],
    )
}
fn type_cached(
    program: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
    depth: usize,
    cache: &mut [Option<Type>],
) -> Result<Type, Error> {
    if depth > 64 {
        return Err(failure("bound", "Expression depth exceeds 64."));
    }
    let index = usize::from(id);
    let node = program
        .expressions
        .get(index)
        .ok_or_else(|| failure("expression", "Expression index out of bounds."))?;
    if let Some(ty) = cache[index] {
        return Ok(ty);
    }
    let result = match node {
        Expr::LiabilityPresent { .. } => Ok(Type::Bool),
        Expr::LiabilityAmount { .. } => Ok(Type::UInt),
        Expr::Literal { value } => Ok(value.ty()),
        Expr::Name { name } => env
            .get(name)
            .copied()
            .ok_or_else(|| failure("name", format!("Unknown or forward name {name:?}."))),
        Expr::Not { value } => {
            if type_cached(program, *value, env, depth + 1, cache)? != Type::Bool {
                return Err(failure("type", "not requires Bool."));
            }
            Ok(Type::Bool)
        }
        Expr::Binary { op, lhs, rhs } => {
            let a = type_cached(program, *lhs, env, depth + 1, cache)?;
            let b = type_cached(program, *rhs, env, depth + 1, cache)?;
            let expected = match op {
                BinaryOp::And | BinaryOp::Or => Type::Bool,
                _ => Type::UInt,
            };
            if a != b || (!matches!(op, BinaryOp::Eq | BinaryOp::Ne) && a != expected) {
                return Err(failure("type", "Binary operands have incompatible types."));
            }
            Ok(
                if matches!(op, BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul) {
                    Type::UInt
                } else {
                    Type::Bool
                },
            )
        }
    }?;
    cache[index] = Some(result);
    Ok(result)
}
/// Validates even manually constructed/deserialized IR before interpretation.
pub fn validate(program: &Program) -> Result<(), Error> {
    if program.language != LANGUAGE || program.semantics != SEMANTICS {
        return Err(failure(
            "profile",
            "Unsupported language or semantics identity.",
        ));
    }
    if !identifier(&program.name) {
        return Err(failure(
            "name",
            "Agreement name must be a non-reserved ASCII identifier of at most 64 bytes.",
        ));
    }
    if program.actions.is_empty() {
        return Err(failure("program", "Program needs at least one action."));
    }
    if program.states.len() + program.assets.len() + program.actions.len() > MAX_DECLARATIONS
        || program.expressions.len() > MAX_EXPRESSIONS
    {
        return Err(failure(
            "bound",
            "Program exceeds finite declaration/expression bounds.",
        ));
    }
    names(
        program
            .states
            .iter()
            .map(|s| s.name.as_str())
            .chain(program.assets.iter().map(|a| a.name.as_str())),
        "declaration",
    )?;
    names(program.actions.iter().map(|a| a.name.as_str()), "action")?;
    let mut identities = BTreeSet::new();
    for asset in &program.assets {
        let id = &asset.identity;
        if id.network.is_empty()
            || id.issuer.is_empty()
            || id.reference.is_empty()
            || id.network.len() > 1024
            || id.issuer.len() > 1024
            || id.reference.len() > 1024
            || id.scale > 38
            || !identities.insert(id)
        {
            return Err(failure(
                "asset",
                "Asset identities must be complete, unique and use scale 0..38.",
            ));
        }
        if program.assets.iter().any(|other| {
            other.identity.network == id.network
                && other.identity.issuer == id.issuer
                && other.identity.reference == id.reference
                && other.identity.scale != id.scale
        }) {
            return Err(failure(
                "asset",
                "Same asset identity cannot have conflicting scales.",
            ));
        }
    }
    let mut depths = Vec::with_capacity(program.expressions.len());
    for (i, e) in program.expressions.iter().enumerate() {
        let refs = match e {
            Expr::Binary { lhs, rhs, .. } => vec![*lhs, *rhs],
            Expr::Not { value } => vec![*value],
            _ => vec![],
        };
        let mut depth = 0;
        for r in refs {
            if usize::from(r) >= i {
                return Err(failure(
                    "expression",
                    "Expression references must point to earlier nodes; cycles are forbidden.",
                ));
            }
            depth = depth.max(depths[usize::from(r)] + 1);
        }
        if depth > 64 {
            return Err(failure("bound", "Expression depth exceeds 64."));
        }
        depths.push(depth);
    }
    let state_types: BTreeMap<_, _> = program
        .states
        .iter()
        .map(|s| (s.name.clone(), s.ty))
        .collect();
    for state in &program.states {
        if state.initial.ty() != state.ty {
            return Err(failure(
                "type",
                format!("Initial state {:?} has wrong type.", state.name),
            ));
        }
    }
    for action in &program.actions {
        if action.inputs.len() > MAX_DECLARATIONS || action.statements.len() > MAX_STATEMENTS {
            return Err(failure(
                "bound",
                "Action exceeds input or statement bounds.",
            ));
        }
        names(action.inputs.iter().map(|i| i.name.as_str()), "input")?;
        let mut env = state_types.clone();
        for input in &action.inputs {
            if env.insert(input.name.clone(), input.ty).is_some()
                || program.assets.iter().any(|a| a.name == input.name)
            {
                return Err(failure("duplicate", "Input cannot shadow a declaration."));
            }
        }
        for statement in &action.statements {
            match statement {
                // Same identity as `stage::require_core1`; only an in-memory
                // program can carry Await here, since `/1` JSON cannot.
                Statement::Await { .. } => {
                    return Err(failure("profile", "Await requires moriarty-core/2."));
                }
                Statement::Let { name, value } => {
                    let ty = expr_type(program, *value, &env, 0)?;
                    if !identifier(name)
                        || env.insert(name.clone(), ty).is_some()
                        || program.assets.iter().any(|a| a.name == *name)
                    {
                        return Err(failure("duplicate", "let cannot shadow an existing name."));
                    }
                }
                Statement::Requires { predicate } | Statement::Ensures { predicate } => {
                    if expr_type(program, *predicate, &env, 0)? != Type::Bool {
                        return Err(failure("type", "Predicate must have type Bool."));
                    }
                }
                Statement::Next { state, value } => {
                    let expected = state_types
                        .get(state)
                        .ok_or_else(|| failure("name", format!("Unknown state {state:?}.")))?;
                    if expr_type(program, *value, &env, 0)? != *expected {
                        return Err(failure("type", "State update has wrong type."));
                    }
                }
                Statement::Emit { name, value } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure("name", "Event name must contain 1..1024 bytes."));
                    }
                    expr_type(program, *value, &env, 0)?;
                }
                Statement::Transfer {
                    asset,
                    from,
                    to,
                    amount,
                }
                | Statement::Fee {
                    asset,
                    from,
                    to,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if from.is_empty() || to.is_empty() || from.len() > 1024 || to.len() > 1024 {
                        return Err(failure("account", "Account identities cannot be empty."));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Liability {
                    name,
                    asset,
                    debtor,
                    creditor,
                    amount,
                } => {
                    validate_asset(program, asset)?;
                    if name.is_empty()
                        || debtor.is_empty()
                        || creditor.is_empty()
                        || name.len() > 1024
                        || debtor.len() > 1024
                        || creditor.len() > 1024
                    {
                        return Err(failure(
                            "liability",
                            "Liability identities cannot be empty.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
                Statement::Discharge { name, amount } => {
                    if name.is_empty() || name.len() > 1024 {
                        return Err(failure(
                            "liability",
                            "Liability name must contain 1..1024 bytes.",
                        ));
                    }
                    require_uint(program, *amount, &env)?;
                }
            }
        }
    }
    validate_liability_reads(program)?;
    Ok(())
}
// New reads require a source-fixed identity. Legacy Core external duties and
// discharge-only programs keep their existing validation rules.
pub(crate) fn validate_liability_reads(p: &Program) -> Result<(), Error> {
    let mut reads = BTreeMap::new();
    for expr in &p.expressions {
        if let Expr::LiabilityPresent { name } | Expr::LiabilityAmount { name } = expr {
            if name.is_empty() || name.len() > 1024 {
                return Err(failure(
                    "liability",
                    "Liability read name must contain 1..1024 bytes.",
                ));
            }
            reads.insert(name.as_str(), None);
        }
    }
    for stmt in p.actions.iter().flat_map(|a| &a.statements) {
        if let Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = stmt
            && let Some(found) = reads.get_mut(name.as_str())
        {
            let identity = (
                validate_asset(p, asset)?,
                debtor.as_str(),
                creditor.as_str(),
            );
            if found.is_some_and(|previous| previous != identity) {
                return Err(failure(
                    "liability",
                    format!("Conflicting source liability identity for {name:?}."),
                ));
            }
            *found = Some(identity);
        }
    }
    for (name, identity) in reads {
        if identity.is_none() {
            return Err(failure(
                "liability",
                format!("Liability read {name:?} has no source-derived identity."),
            ));
        }
    }
    Ok(())
}

pub(crate) fn require_uint(
    p: &Program,
    id: ExprId,
    env: &BTreeMap<String, Type>,
) -> Result<(), Error> {
    if expr_type(p, id, env, 0)? != Type::UInt {
        Err(failure(
            "type",
            "Financial amount requires UInt atomic units.",
        ))
    } else {
        Ok(())
    }
}
pub(crate) fn validate_asset<'a>(p: &'a Program, name: &str) -> Result<&'a AssetId, Error> {
    p.assets
        .iter()
        .find(|a| a.name == name)
        .map(|a| &a.identity)
        .ok_or_else(|| failure("asset", format!("Unknown asset {name:?}.")))
}

END FILE units/core-static-validation-and-type-helpers.rs

BEGIN FILE units/stage-static-cost-fill-and-helpers.rs | SHA256 67efcf8bec34ce1e7d115b7e0543b627112a5087ca11196705b5df01a631308a | BYTES 5657
//! Static logical work of successful segments (C3).
//!
//! The evaluator charges one unit per statement dispatch and one per distinct
//! expression node it visits. Its cache is per statement, and `And`/`Or`
//! evaluate both operands, so a successful statement costs exactly
//! `1 + |nodes reachable from its root expression|`, independent of values.
//! Segment j runs from entry site j (0 = start) and stops at the next await,
//! whose entry charge of 1 is paid but whose guard belongs to segment j+1.
use super::{ActionPlan, StageLogical, StagePlan};
use crate::policy::{Cost, Meter};
use crate::{Error, Expr, ExprId, Program, Statement, UInt};

fn overflow() -> Error {
    Error::new("bound", "Stage logical plan exceeds u128.")
}

fn add(a: UInt, b: UInt) -> Result<UInt, Error> {
    a.0.checked_add(b.0).map(UInt).ok_or_else(overflow)
}

/// Distinct expression nodes reachable from `root`: the per-statement cache
/// charges each shared DAG node once.
fn expression_nodes(program: &Program, root: ExprId) -> Result<u128, Error> {
    let mut seen = vec![false; program.expressions.len()];
    let mut pending = vec![root];
    let mut count = 0_u128;
    while let Some(id) = pending.pop() {
        let index = usize::from(id);
        let node = program
            .expressions
            .get(index)
            .ok_or_else(|| Error::new("expression", "Expression index out of bounds."))?;
        if std::mem::replace(&mut seen[index], true) {
            continue;
        }
        count += 1;
        match node {
            Expr::Binary { lhs, rhs, .. } => pending.extend([*lhs, *rhs]),
            Expr::Not { value } => pending.push(*value),
            Expr::LiabilityPresent { .. }
            | Expr::LiabilityAmount { .. }
            | Expr::Literal { .. }
            | Expr::Name { .. } => {}
        }
    }
    Ok(count)
}

fn root(statement: &Statement) -> ExprId {
    match statement {
        Statement::Await { guard, .. } => *guard,
        Statement::Let { value, .. }
        | Statement::Next { value, .. }
        | Statement::Emit { value, .. } => *value,
        Statement::Requires { predicate } | Statement::Ensures { predicate } => *predicate,
        Statement::Transfer { amount, .. }
        | Statement::Fee { amount, .. }
        | Statement::Liability { amount, .. }
        | Statement::Discharge { amount, .. } => *amount,
    }
}

/// Successful cost of one statement. For an Await this is its resume dispatch
/// plus guard; reaching an Await at segment exit costs 1 instead.
pub(crate) fn statement_cost(program: &Program, statement: &Statement) -> Result<UInt, Error> {
    let nodes = expression_nodes(program, root(statement))?;
    nodes.checked_add(1).map(UInt).ok_or_else(overflow)
}

fn segment_cost(
    program: &Program,
    action: usize,
    plan: &ActionPlan,
    segment: usize,
) -> Result<UInt, Error> {
    let statements = &program
        .actions
        .get(action)
        .ok_or_else(|| Error::new("program", "Stage plan invariant violated: action."))?
        .statements;
    let ordinal = |site: usize| -> Result<usize, Error> {
        plan.sites
            .get(site - 1)
            .map(|s| usize::from(s.ordinal))
            .filter(|ordinal| *ordinal < statements.len())
            .ok_or_else(|| Error::new("program", "Stage plan invariant violated: site."))
    };
    let n = plan.sites.len();
    let begin = if segment == 0 { 0 } else { ordinal(segment)? };
    let end = if segment == n {
        statements.len()
    } else {
        ordinal(segment + 1)?
    };
    if begin >= end && !(segment == 0 && begin == end) {
        return Err(Error::new(
            "program",
            "Stage plan invariant violated: ordinals.",
        ));
    }
    let mut total = UInt(0);
    for (offset, statement) in statements[begin..end].iter().enumerate() {
        // Only an advance's first statement is an Await (its dispatch).
        if matches!(statement, Statement::Await { .. }) && (segment == 0 || offset != 0) {
            return Err(Error::new(
                "program",
                "Stage plan invariant violated: unplanned await.",
            ));
        }
        total = add(total, statement_cost(program, statement)?)?;
    }
    if segment < n {
        total = add(total, UInt(1))?;
    }
    Ok(total)
}

/// Fills `segment_cost`, `committed_before` and `plan_total` for every action
/// of a Sol-built plan whose cost vectors are still empty.
pub(crate) fn fill(program: &Program, plan: &mut StagePlan) -> Result<(), Error> {
    if plan.actions.len() != program.actions.len() {
        return Err(Error::new(
            "program",
            "Stage plan invariant violated: action count.",
        ));
    }
    for (index, action) in plan.actions.iter_mut().enumerate() {
        if !action.segment_cost.is_empty()
            || !action.committed_before.is_empty()
            || action.plan_total != UInt(0)
        {
            return Err(Error::new(
                "program",
                "Stage plan invariant violated: costs are already filled.",
            ));
        }
        let mut costs = Vec::with_capacity(action.sites.len() + 1);
        let mut committed = Vec::with_capacity(action.sites.len() + 1);
        let mut total = UInt(0);
        for segment in 0..=action.sites.len() {
            let cost = segment_cost(program, index, action, segment)?;
            committed.push(total);
            total = add(total, cost)?;
            costs.push(cost);
        }
        action.segment_cost = costs;
        action.committed_before = committed;
        action.plan_total = total;
    }
    Ok(())
}
END FILE units/stage-static-cost-fill-and-helpers.rs

BEGIN FILE units/M01-arena-contract-complete.md | SHA256 edb4f032661ca96f5a98e92617c3e721c8e51a674c92ba0bb6d3dd89aaf5dc25 | BYTES 11964
## Adopted M01 arena supplement

This separate ordinary representation design was adopted on 2026-10-18 under the [recorded decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json). Its original frozen report-only proposal deliberately left M01 unselected; both original proposal packets remain unchanged. The SHALL clauses below are adopted and implementation-source-exposed. They describe the complete public Program representation of the finite documented positive source forms below; they grant no admission or execution authority. All fourteen individual gap rows remain OPEN. This supplement supplies an adopted finite representation premise, not a parser result, independent Program golden or completed GAP-05/M01 exit.

The two Core/1 full-Program fixtures from PR203 are finite precedent for source-order arrays, initializer placement and fresh expression occurrences. Their post-adoption derivation does not establish a general Core/1 grammar, general normalization or Core/1/Core/2 equivalence. Core/2 adopts nothing by inheritance here: each choice below was independently adopted by the two substantive design votes recorded on 2026-10-18; new independent expectations remain a separate freeze.

### Whole Program and declaration order

For already admitted source beginning language "moriarty/2", Program SHALL contain exactly language:"moriarty/2", semantics:"moriarty-core/2", name:the agreement identifier, states, assets, expressions and actions. No source hash, selected-action field, Scenario, capture, plan, registry, work counter or runtime result SHALL be inserted. Type SHALL serialize as "UInt" or "Bool". A typed UInt literal SHALL use {type:"UInt",value:its canonical unsigned decimal string}; a Bool literal SHALL use {type:"Bool",value:true or false}. Literal decoding follows the existing admitted numeric/string syntax; no decimal scaling, evaluation or new lexical normalization is selected.

StateDecl SHALL retain exactly name, ty and initial, with the literal initial stored directly as a typed Value. State declarations SHALL occur in Program.states in their own lexical declaration order. AssetDecl SHALL retain exactly name and identity; identity SHALL contain network, issuer and reference as decoded strings and scale as its numeric u8 value. Asset declarations SHALL occur in Program.assets in their own lexical declaration order. Action SHALL retain exactly name, inputs and statements; Program.actions SHALL retain every action, including unused actions, in lexical declaration order. Interleaving declaration kinds does not introduce a combined declaration-order field; each array is the source sequence filtered to its own kind. These representation rules do not authorize forward references or alter existing declaration admission.

A state's initializer, asset scale/identity, agreement/profile/action identifiers, input and binder declarations and statement metadata SHALL allocate no expression nodes. They SHALL NOT be converted into action expressions or synthetic statements. Action.inputs SHALL contain exactly {name,ty} entries: signature inputs in order, followed by any existing admitted leading input declarations in body order. Those leading declarations SHALL NOT count as statements. Await.binders SHALL retain exactly the written {name,ty} entries in binder declaration order, not action inputs. Runtime inputs, event values and captures SHALL NOT be substituted for declarations or expressions in Program.

### One fresh global expression arena

Program.expressions SHALL start empty and be shared across all actions in their lexical declaration order. Within each action, process statements in their written order and allocate only the statement's expression tree as below. Do not reset or rebase the arena at an action, statement, await, segment or selected-action boundary. Retain all nodes from unused actions. An otherwise empty selected action does not imply an empty whole Program arena.

For each source expression occurrence, allocate fresh nodes in left-to-right postorder. To append a node, set its ExprId to the current global array length, append exactly that node, then return that numeric zero-based index. Existing expression bounds and u16 representation remain unchanged. Do not intern repeated names, repeated literal values or structurally equal subexpressions, within or across statements/actions. No constant folding, algebraic reassociation, common-subexpression sharing, inserted coercion or desugared helper node is selected for this source-to-Program representation. Array position SHALL be the ExprId; no extra id member SHALL be inserted in a node.

| Source form | Exact appended node and allocation order |
| --- | --- |
| UInt or Bool literal in an action expression | One fresh Literal {value:typed decoded literal} |
| Ordinary name occurrence | One fresh Name {name:written identifier}; no substitution with a declaration, reference index or runtime value |
| Parenthesized expression | Its inner allocation/root only; no grouping node |
| Prefix not expression | Allocate its operand first, then one Not {value:operand ExprId} |
| Binary expression | Allocate the complete left subtree, then the complete right subtree, then one Binary {op:the selected operator, lhs:left ExprId, rhs:right ExprId} |
| liability_present("literal duty name") | One LiabilityPresent {name:decoded literal duty name}; quoted argument is metadata, not a Literal child |
| liability_amount("literal duty name") | One LiabilityAmount {name:decoded literal duty name}; quoted argument is metadata, not a Literal child |

Every brace form denotes exactly the listed fields plus kind carrying the variant name. Child fields are numeric ExprIds into this Program; they are not typed semantic values or source offsets. For this fresh-postorder source mapping, a child's index is lower than its parent's. Each repeated textual occurrence still allocates its own subtree. No rule here changes the separate admission of manually supplied DAGs or other profiles.

The finite documented expression grammar uses atoms above, parentheses, strongest prefix not, then decreasing binary precedence: * (Mul); + and - (Add/Sub); ==, <, <=, >, >= (Eq/Lt/Le/Gt/Ge); and (And); or (Or). Operators at one binary precedence are left-associative; explicit parentheses determine a different tree before postorder allocation. Minus is binary here; no unary-negative, division, generic function call, branching or implicit decimal node is introduced. This is an adopted finite representation grammar for already admitted forms, not a general frontend normalization claim.

The public DTO also contains BinaryOp::Ne. Current parser source recognizes it through token pair not followed by =, while the public README does not document that spelling and the shared lexer has no exclamation punctuation. This supplement SHALL NOT canonize that parser-specific spelling, invent != support or silently repair the language. Source inequality spelling/admission remains a separate ordinary grammar decision; if the intended spelling is !=, corresponding lexer/parser source repair and independent checks would be required. Such forms are outside this finite grammar's correspondence claim, without retiring any broader language obligation. Their potential future node shape remains Binary with op:"Ne" once source syntax is independently specified. No implementation mismatch is established for the chosen documented forms by this source-only review; conformance still needs real checks.

### Exact statement boundaries, metadata and roots

Each source statement SHALL contribute one Statement in Action.statements in written order, after its expression root is allocated. Statement boundaries SHALL not allocate nodes, reset numbering, append a unit/success result, or insert a statement. A semicolon ends the existing statement form; braces and leading input declarations are not Statement entries. Each expression-bearing field SHALL retain its numeric root ExprId, even where the expression's evaluated amount or boolean happens to have the same numeric spelling.

| Source statement form | Exact public Statement fields in addition to kind | Expression root |
| --- | --- | --- |
| let name = e; | Let: name, value | value is root(e) |
| requires e; | Requires: predicate | predicate is root(e) |
| ensures e; | Ensures: predicate | predicate is root(e) |
| next state = e; | Next: state, value | value is root(e) |
| emit "event name"(e); | Emit: name, value | name is decoded metadata; value is root(e) |
| transfer(asset,"from","to",e); | Transfer: asset, from, to, amount | asset is the alias, parties decoded metadata; amount is root(e) |
| fee(asset,"from","to",e); | Fee: asset, from, to, amount | same metadata roles; amount is root(e) |
| liability("name",asset,"debtor","creditor",e); | Liability: name, asset, debtor, creditor, amount | duty/parties decoded metadata and asset alias; amount is root(e) |
| discharge("name",e); | Discharge: name, amount | duty name decoded metadata; amount is root(e) |
| await label(name:Type,...) when e; | Await: label, binders, guard | label/binder declarations allocate nothing; guard is root(e) |

Ordinary expressions in an Await guard SHALL follow the same fresh allocation rule at that await's lexical statement position. A guard reference to one of its event binders SHALL allocate a Name occurrence; declaring that binder allocates none. Guard allocation SHALL not wait for runtime resume, and SHALL not depend on the event's value or whether a later invocation waits, advances or rejects. Await.guard's ExprId SHALL remain distinct in meaning from the action statement ordinal, plan site number and report Step.index. Name binding/type and financial/await execution semantics remain separate existing obligations.

No selected-action filtering, added declaration for admission, hidden arena node or runtime trace normalization SHALL alter this whole-source Program mapping. Retain unused declarations exactly when actually present in the source. No Mint, Burn or dedicated accrual construct is introduced, and broader missing financial obligations remain. The separately adopted site/capture, trace, envelope, rendering and local-work clauses above retain their own scope; original conditional-work records remain unchanged.

### M01 boundary and independent freeze

This supplement adopts an ordinary representation premise sufficient to derive a complete Program for the original finite escrow source from its actual declarations and expression occurrences. It supplies no such Program fixture, numeric arena specimen, parser output or expected execution. M01 source correspondence and GAP-05 remain OPEN until a distinct custodian derives/freezes the complete Program without using this source-exposed author or parser output as its oracle, and authorized real full-Program checks are observed. Recorded public-premise adoption enables that independent pre-run derivation while the source-correspondence exit remains OPEN awaiting real checks. The original partial escrow freeze, MISSING-PREMISES.json, derivation, all fourteen individual gap dispositions, R7 contradiction, original report candidate, freezes/failures and counters SHALL remain unchanged.

M03 local identity design is separately owner-adopted under its [decision](../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/M03-DESIGN-ADOPTION.json); only the verbatim designated-owner F4 text below supplies that normative identity clause. This ordinary author supplies no identity or security assessment. Source-file and proposal-byte digests identify inspected artifacts only, not canonical Program/Plan identity. This supplement supplies no admission, decoder/replay, proof, target or runtime result and closes no U0–U7, M1–M17, MC/SP/G, financial, composition or partial/conditional parent exit.

END FILE units/M01-arena-contract-complete.md

BEGIN FILE units/UNI-018.md | SHA256 97a9c37cc07986731b005ea281e07e4449e90617f8a77c2bc57a43acaf1cf1d5 | BYTES 814
### Requirement: UNI-018 Profile and consumer admission
When a tool reports checking, compilation or simulation, it SHALL identify the source/Core profile, resolved executable, consumer and admitted subset, and SHALL report unsupported consumer constructs explicitly.

#### Scenario: Supported Rust Core program
- **WHEN** A `moriarty/1` program is checked and compiled by its supported Rust consumers.
- **THEN** The record names `moriarty-core/1`, both consumers and the construct coverage actually exercised.

#### Scenario: Different authoring profile
- **WHEN** A beta or Source/6 program passes its local checker but is outside a Rust compiler's admitted input.
- **THEN** The record retains the local checking result and reports the compiler boundary as unsupported; compilation receives no inferred pass.

END FILE units/UNI-018.md

BEGIN FILE units/UNI-019.md | SHA256 efe795f62ee80cca24ba9d1fe7902a892e40081d0b23e139e9cc729b9c5901d2 | BYTES 779
### Requirement: UNI-019 Scoped lowering comparison
When two lowering consumers are compared, the comparison SHALL state their statements, admitted constructs and common domain, and SHALL compare independently specified complete observations only within that domain.

#### Scenario: Common construct domain
- **WHEN** Both lowerings admit a case with declared numeric and effect representations.
- **THEN** The comparison records each representation and the complete expected observations under the stated mapping.

#### Scenario: Different admitted statements
- **WHEN** One consumer intentionally refuses a construct admitted by another.
- **THEN** The admission matrix records that refusal and its reason; neither global equivalence nor identical artifact bytes is inferred.

END FILE units/UNI-019.md

BEGIN FILE units/UNI-020.md | SHA256 cee03cd3ef4953fa663c532740b81726e26adfe5c0d97e67c5c874d2161a0b19 | BYTES 4630
### Requirement: UNI-020 Independent financial and work expectations
When a semantic or conformance result is scored, its expected values SHALL come from independently derived arithmetic and observations frozen before the run, with shared-code dependencies and numeric-profile scope declared.

#### Scenario: Complete finite observation
- **WHEN** A finite financial example is evaluated under a versioned numeric and logical-work schedule.
- **THEN** The comparator checks ordered effects, changed and preserved state, gross debit, fees, net receipts, dust, liability changes, duty presence and logical work against the independent expectations.

#### Scenario: Shared evaluator oracle
- **WHEN** Expected results come only from the same evaluator, static reopen or a wrapper around the implementation under review.
- **THEN** The record describes internal agreement and leaves independent semantic-oracle coverage open.

**NR-04 — Statement deltas and ordered execution effects.** WHEN a scoped normal Core/1 report is serialized, each Step.effects SHALL contain only that statement's new ordered effects, and Execution.effects SHALL equal their ordered concatenation, retaining duplicates and array order. The derived prefix SHALL satisfy P[-1]=[] and P[i]=P[i-1] followed by Step[i].effects; it is not an extra wire member. The N=0 concatenation SHALL be []. Counter Step2/3/4 deltas SHALL respectively be [State(count0→count7)], [] and [Emit(count,UInt7)], with prefixes [State], [State] and [State,Emit]; the final execution effects SHALL be [State,Emit]. Final-state equality alone does not supply this observation.

**NR-05 — Positive metered/ordinary agreement.** WHERE a scoped normal invocation uses the existing moriarty-reference-logical-cost/1 profile with a sufficient cap, its complete Execution SHALL equal the same invocation without metering. The metered envelope SHALL preserve status, qualification and Execution, adding only profile and logical:{cap,used} with canonical decimal strings; no per-Step work member is introduced. For the contract's Counter at cap16, the independently specified increments4,4,2,4,2 SHALL total used16, with cap:"16" and used:"16". These units measure reference interpretation only. The complete ordinary/metered observation and literal work SHALL be checked against expectations frozen independently before the run under UNI-020.

**Adopted financial deltas.** For those normally completed financial statements, Step.effects SHALL contain exactly one new Transfer, Fee or Liability effect as specified by the adopted table; Discharge SHALL use Liability with before/after amounts. Zero/self cases already admitted by existing semantics SHALL retain their explicit effects. Execution.effects SHALL equal the ordered concatenation of every Step delta, including intervening nonfinancial effects, preserving repetitions and order; prefixes remain derived data. A before-zero effect SHALL NOT erase the full-snapshot distinction between absent creation and a present-zero opening duty. Complete statement/ExprId/value/snapshot/delta/prefix/work observations and exact consumer-specific bytes SHALL require independently derived expectations frozen before the run. No evaluator output SHALL generate those expectations.

**Adopted Core/2 work and observation authority.** The adopted schedule SHALL charge statement dispatch plus distinct per-statement expression-node visits, separate await entry and resumed guard dispatch, and SHALL report false-guard attempted work without adding it to committed successful-prefix work. Successful segment costs, committed_before and plan_total SHALL follow the contract's explicit sum equations; reports SHALL project the typed work fields without authenticating them. The original partial escrow freeze's5/2/6 attempted and5/5/11 committed predictions remain historically CONDITIONAL and unobserved; adopting the schedule does not rewrite that freeze. Complete source/IR/trace/status/byte/work expectations SHALL require separate independent derivation/freeze after adopted premises and before authorized comparison. No actual output, evaluator return or this implementation-exposed design SHALL generate independent goldens. The ordinary schedule was adopted on 2026-10-18 under the [recorded decision](../../../../../evidence/utxo/doctorwatson-language-loop-2026-10-17/core2-report-adoption-20261018/DESIGN-ADOPTION.json); M03 is separately owner-adopted and incorporated through the linked verbatim designated-owner clause. This ordinary author supplies no identity or security assessment; no cap/refusal/resource policy is changed.

END FILE units/UNI-020.md

BEGIN FILE units/UNI-021.md | SHA256 952a35788ae90d852073016dd8f220d72d6fce48331d3f64ef97eb3757c5af4c | BYTES 3613
### Requirement: UNI-021 Capability evidence and parent acceptance
When a capability report is updated, it SHALL record implementation, evidence and acceptance separately for the exact predicate, tree, profile, consumer and target, while retaining every unmet parent exit.

#### Scenario: Delivered finite library child
- **WHEN** The report cites the PR49 FeePool dossier's 23 actions and 162 captured finite financial cases.
- **THEN** It records captured local/reference/static evidence and leaves full M11, Capacity Exchange, U6 and target qualification open.

#### Scenario: Incomplete predicate denominator
- **WHEN** A discovery index contains 69 top-level IDs but mandatory atomic clauses are not fully reconciled.
- **THEN** The report supplies no whole-language completion percentage and identifies the unresolved coverage.

**Normal-report evidence boundary.** NR-01–06 are adopted design requirements, not observed runtime, source-to-IR or public-byte conformance. Their implementation, source observations, independently frozen expectations, actual results and acceptance SHALL remain separate for the exact candidate/profile/consumer. The proposal author's implementation exposure SHALL be disclosed rather than relabeled independent behavior proof. Original proposals, freezes and failures SHALL remain unchanged. A complete Counter, empty-action or Transfer observation can establish only its named child predicate; no parent exit or requirement denominator is closed by design adoption.

**Adopted financial evidence boundary.** The financial observation extension was adopted by the [recorded design decision](../../../../../evidence/utxo/doctorwatson-language-loop-2026-10-17/financial-report-adoption-20261018/DESIGN-ADOPTION.json) on 2026-10-18, and adoption SHALL remain separate from implementation and behavior. Its author inspected implementation source; source agreement supplies neither independent behavior proof nor goldens. Existing financial-pair and normal Transfer aggregate observations SHALL keep their exact scope without inferred complete raw Step.value/statement or ordinary simulate-byte coverage. Original proposals, freezes, failures, earlier five-kind adoption and pending normal-report cases SHALL remain unchanged. No parent acceptance, denominator or native/target result follows; all U0–U7, M1–M17, MC/SP/G and full financial/composition obligations remain. No conservation, soundness, authorization, authentication, proof or security condition is decided; security judgments and hostile/security tests remain with the designated security owner.

**Adopted Core/2 evidence boundary.** Source exposure SHALL be explicit. Every original staged GAP-01–14 SHALL retain its individual OPEN disposition; the contract's per-gap table identifies adopted ordinary design coverage and untouched designated-owner/unresolved fields without blanket closure. The partial escrow freeze and its M01–M07 record, original P-TRACE-PROJ/draft, C1–C6/N1–N6, conditional work records, R7 contradiction, invalidated R2 approvals, freezes, failures and counters SHALL remain unchanged. Two independent design votes, adoption, independent expectation freeze, actual consumer observation and acceptance SHALL be separate records. Public simulate, library pretty-byte and test-aggregate evidence SHALL remain distinct. This adopted design supplies no execution authority, independent behavior proof, native/target/security result, top-level requirement count or parent closure. All U0–U7, M1–M17, MC/SP/G, full finance/composition and partial/conditional obligations remain.

END FILE units/UNI-021.md

BEGIN FILE units/permissionless-product-boundary.md | SHA256 dc2de73287cd8cc172a409e10cfbe404badea933842d292452cceb146464b4f3 | BYTES 2230
# Moriarty: permissionless language of provable intention

Status: target architecture, specified-only where not linked to implementation. Authority: user clarification, 2026-09-19.

Moriarty is a programming language for all DeFi developers on Midnight. Any developer may author, compile, prove, publish and deploy programs in the supported language without approval from Moriarty maintainers, an agent council, Foreman, a hosted solver or a program registry. Objective Midnight validity rules, resource costs and asset-owner authorization still apply.

This document supersedes conflicting product-scope and administrative-deployment requirements in earlier designs. Historical receipts remain evidence of their original scope. It does not declare missing proof or ledger capabilities implemented.

## Consolidated design and delivery

The [consolidated vision and design](MORIARTY-CONSOLIDATED-DESIGN.md), [single U0–U7 roadmap](../ROADMAP.md), and [next ZKIR/recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) integrate the research agenda. P/C/K identifiers remain requirement lineage; they are not independent delivery queues. The user supplied a planning assumption on 2026-09-19 that comprehensive Midnight recursion arrives in about six months. Plan full native recursive compliance and private multi-parent composition accordingly; current interface limitations do not remove that scope. The timing is not a verified release claim.

## Product boundary

The product is the language, semantics, compiler, proof interfaces, verifier integration and developer tooling. ACTUS, loans, swaps, vaults and other DeFi applications test expressivity and conformance; they are libraries and examples, not an exhaustive list of programs developers may deploy. Supported constructs compose into new programs. New language primitives require sound semantics and implementation; they do not justify approval of individual programs built from existing constructs.

A managed multichain router, identity provider, adapter catalog or pricing service can be an optional application. None is required to use Moriarty. Application-specific access control remains expressible without becoming a language-wide deployment restriction.
END FILE units/permissionless-product-boundary.md

BEGIN FILE source/RUST_GUIDELINES.md | SHA256 e448f368eab4ff3634ebfc65edcc3ee62d6cf8bf3e16bb76d81f2facec370af9 | BYTES 19047
# Rust toolchain guidelines

Rules for the Rust implementation of the Moriarty toolchain in `crates/`.
Current checkout `AGENTS.md`, the product contract and explicit user steering
control profile scope, authorization and execution boundaries. Routine repairs
within existing authorization need no additional approval. Record intentional
differences; these guidelines do not grant compiler, SDK or native execution.

## Target language

Port `moriarty-beta/1`, the `.mori` language. The reference implementation is
[`packages/moriarty-beta/src/frontend.ts`](packages/moriarty-beta/src/frontend.ts);
the [reference page](https://charleshoskinson.github.io/Moriarty/reference.html#language)
documents it. Beta lowers to Source/6 and Core/5
([contract](experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md)).

The surface syntax is not frozen. Keep grammar-specific code separate from the
later phases so it can change. The Rust toolchain accepts and rejects the same
programs as the TypeScript frontend, except for the intentional differences
below. Error codes, messages and spans are our own.

### Intentional differences

The TypeScript frontend is the reference for what the language accepts and
what values mean, not for incidental implementation choices. Every deliberate
difference is listed here under the applicable user authorization.

- **Comments do not count toward the 8,192-token bound.** The bound limits
  analysis, and comments are not analyzed. The TypeScript lexer counts them, so
  a file whose comments push it past the bound is rejected there and accepted
  here. The 65,536-byte source bound still caps comments.
- **`mori format` formats any file that parses,** including files with name or
  type errors, as rustfmt and prettier do. The TypeScript formatter refuses any
  file with an error. Its layout also differs: ours fits lines to 80 columns.
  And `mori format` (alias `fmt`) writes in place by default, with `--check` instead of the
  TypeScript CLI's `--write`.
- **Syntax errors are reported before name and type errors.** The TypeScript
  frontend resolves names while parsing, so in a file with several errors it
  may report a different first one. Files with a single error behave the same.

### Adopted explicit Source/6 consumer disposition

The adopted `mori expand --emit-source6-transfer` interface, behind the non-default
`source6-expansion` feature, is a separate local transfer-only consumer. It
keeps default Rust Core expansion and the Node beta package unchanged. It
accepts the closed transfer subset of `moriarty-local-scenario/1`, refuses
repayment/family actions before scenario I/O, and explicitly refuses obligation
and candidate-effect override fields. It emits raw Source/6 text with its final
newline and reports local-stipulation-only on stderr; quiet suppresses that
status line. It does not claim full Node BetaExpansion JSON/provenance parity.
Existing Rust diagnostics may differ in code and presentation. The
[2026-10-18 root decision](evidence/utxo/doctorwatson-language-loop-2026-10-17/source6-public-consumer-adoption-20261018/DESIGN-ADOPTION.json)
adopts this ordinary interface only. Compilation, public CLI execution, actual
Source/6 lowerer consumption and native/security qualification remain unrun or
unestablished. The existing independent14 financial payload baseline predates
this consumer; complete wire/output/work expectations and the chosen WorkshopReceipt
fixture freeze remain missing. Interface adoption supplies no native embedding,
security approval or language-exit acceptance.

## Workspace

- Every crate lives in `crates/*`. The `mori` crate is the CLI entry point.
- The toolchain is pinned in `rust-toolchain.toml` (Rust 1.99.0, edition 2024).
- Declare every dependency version once in the root `[workspace.dependencies]`.
  Crates reference it with `dep = { workspace = true }` or `dep.workspace = true`.
- The pre-existing Rust crates under `experiments/`, `raw/` and `wiki-llm/` are
  not part of this toolchain. They are excluded from the workspace so they still
  build standalone; the beta CLI tells users to build `experiments/midnight-crypto`
  with `--manifest-path`, which fails if the workspace claims it.
- Every Rust file builds and tests through Cargo. No harness compiles a module
  with `rustc` directly, and no `#[path]` mounts a file from another directory
  or loads one file as several modules; share code through a crate or a module
  and `use` it.
- Code only tests use lives in test modules. Unadopted prototypes go in
  `experiments/` or behind a feature, not in `src/` gated by `#[cfg(test)]`.
- Never hash source text to identify an ABI, deployment, key or build. Any
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
  array, as Zig does.
- **Data-oriented layout.** Tokens, AST nodes and comments are struct-of-arrays
  containers built with [`soa-rs`](https://crates.io/crates/soa-rs)
  (`#[derive(Soars)]`). Nodes refer to tokens and other nodes by index handles,
  never by pointers or boxes. Variable-length node data goes in an `extra`
  `Vec`. Tag enums are `#[repr(u8)]`. Node payloads stay plain `lhs`/`rhs`
  fields interpreted by tag, not Rust enums.
- **Small handles.** Beta caps source at 65,536 bytes and tokens and nodes at
  8,192 each, so token and node handles are `u16` newtypes and byte offsets are
  `u32`. `soa-rs` indexes by `usize`; convert at the handle boundary.
- **Preallocate.** Size the token container with `source.len() / 8`, as Zig does.
- **Tokens store only `tag` and `start`.** The end is recovered by re-lexing.
- **Comments are kept for the formatter** in their own `start`/`end` table, not
  in the token stream. Only tokens, including the end-of-file token, count toward
  the 8,192-token bound; whitespace and comments do not.
- **Crates.** `mori-span` holds spans, `mori-lexer` the tokens and lexer,
  `mori-ast` the struct-of-arrays AST and its tree printer, `mori-parser` the
  parser that produces it, and `mori-checker` the checker that consumes it.
- **Separate phases.** Lex → parse → resolve and check → evaluate constants.
  Do not copy the TypeScript parser, which evaluates while parsing.
- **Numbers.** Values fit in `u128`; no bigint library. Decimal quantities are
  scaled by their asset during checking.
- **Node spans are computed, not stored.** Like Zig's `firstToken` and
  `lastToken`, `Ast` derives a node's first and last token from its tag and
  children when a diagnostic needs it.

## Checker

- **One `mori-checker` crate,** with modules for names, values, declaration
  rules, calls and intents. Checking is one ordered walk: each declaration's
  names are resolved, its value computed and its rules checked before the next,
  because later declarations depend on earlier values. Its codes are
  `mori::check::<kind>`.
- **Output is side tables and a value store,** in the same style as the AST: a
  `Soa` of declarations (kind, name token, value handle) and a `Soa` of values
  with `u16` handles, an `extra` array for records and lists, and amounts in a
  separate `u128` array. No boxed value trees.
- **`check` returns what was checked together with its diagnostics,** so valid
  declarations stay usable when others fail, as a language server needs. A
  program is accepted only when there are no diagnostics.
- **It reports every independent error.** A declaration that fails is marked
  failed, and anything depending on it is skipped without further errors, so
  one mistake never cascades. Which programs are accepted does not change.
- **The built-in call schemas are a static Rust table:** each call's name and
  its required and optional arguments with their expected types.

## Formatter

- **`mori-fmt` is a Wadler-style pretty printer.** It builds a document of
  text, line breaks, groups and indentation from the AST, then fits each group
  on one line within 80 columns or breaks it one item per line, with trailing
  commas when broken.
- **No `Format` trait.** The document is built by a `match` over `NodeTag`,
  like the other phases, into an arena of rows.
- **Output depends only on the program and its comments.** The profile line and
  every agreement item are followed by exactly one blank line, and comments
  above an item stay attached to it. Numbers and strings keep their spelling.
- **Comments come from the comment table** and stay next to the token they
  were written beside: on the same line, or on their own line above.
- **Correctness is proven by tests, not checked at run time.** Snapshot tests
  show each layout case with its `.mori` source, and every one also asserts
  that formatting the output again changes nothing. A property test checks
  that output reparses to the same tokens and comments.
- **`mori format [PATHS]...` (alias `fmt`) rewrites files in place.** A file formats that
  file, a directory every `.mori` file under it, and no arguments the current
  directory. Directories are walked with `ignore` (respecting `.gitignore`)
  and files are formatted in parallel with `rayon`; output is printed in
  sorted path order. A file that does not parse is reported and skipped.
- **`mori format --check [PATHS]...`** writes nothing and exits 1 when a file is
  not formatted, for CI. Each such file gets a `mori::fmt::not_formatted`
  diagnostic header (no snippet, no help), then the diff exactly as
  `cargo fmt --check` prints it (one `Diff in FILE:LINE:` per region, three
  context lines, red `-` and green `+`, via `similar`), indented to line up
  with the message. One summary line at the end says how to fix it.

## Diagnostics

- Follow [oxc](https://github.com/oxc-project/oxc): one open `MoriDiagnostic`
  struct (message, labels, help, severity, code) in a `mori-diagnostics` crate,
  implementing miette's `Diagnostic` trait. Do not create a struct or enum
  variant per error.
- Each compiler crate has a `diagnostics.rs` of builder functions:

  ```rust
  pub fn quantity_separator(span: Span, source: &str) -> MoriDiagnostic {
      MoriDiagnostic::error("A quantity needs a space between the number and its asset.")
          .with_code("mori::syntax::quantity_without_space")
          .with_label(span.label("no space here"))
          .with_help_code("Put a space between them:", corrected_line(source, span, ...))
  }
  ```

- Every phase builds a `MoriDiagnostic` where the error is found, through a
  builder in its `diagnostics.rs`. Errors are rare and never on a hot path.
- Messages must be Elm-quality: say what was being parsed, what was expected,
  show a correct example and give a specific hint. Errors should teach the
  language.
- To support that, the parser keeps a context stack (agreement body,
  declaration, record fields, call arguments, type annotation) and records the
  set of expected token tags at each failure.
- Common mistakes get their own codes and teaching messages, for example:
  `10USD` (missing space before the asset), a decimal without an asset, a
  forward reference, an unknown call (suggest the closest of the known calls),
  a reserved word as a name, `/` (no division), a dotted name that is not a
  call, and adding quantities of different assets.
- Codes follow miette's style, one per kind of error:
  `mori::<phase>::<kind>`, such as `mori::lex::leading_zero` or
  `mori::syntax::missing_comma`.
- No title banners. A help that suggests a fix shows the corrected code on its
  own indented lines, not inside a sentence:

  ```
    help: Fields in a record are separated by commas:

              { domain: Preview, id: "A" }
  ```
- Spans are UTF-8 byte ranges, matching miette's `SourceSpan` and beta. Editors
  and a future language server need UTF-16 line/character positions, converted
  from the same compact records.
- The lexer and parser stop at the first error, like the TypeScript frontend.
  Recovery at declaration boundaries may come later.
- `mori-span` converts `Span` into miette's span types, as `oxc_span` does.
- The `fancy` feature of `mori-diagnostics` enables miette's
  `fancy-no-backtrace`. The `mori` CLI turns it on; library crates turn it on
  only in `[dev-dependencies]`.
- Use `thiserror` for errors that never point at source, such as I/O. No
  `anyhow` in library crates.

## Testing

- Snapshot tests use [`insta`](https://insta.rs) and `cargo-insta`.
- The finite backend `financial_observation_relation` target uses frozen
  complete Program/Execution literals and positive SDK relation checks instead
  of insta snapshots. The exact sources and independent expectations remain in
  `crates/fixtures/financial-report-contract/`; expected values must not come
  from the evaluator, compiler output or generated baselines. This exception
  adds no authority to execute SDK checks or claim native/proof acceptance.
- The finite Core `financial_program_wire_contract` target uses independently
  frozen complete financial Program JSON and the public Program/v1 decoder
  instead of generated snapshots. It selects no Midnight compatibility feature
  and makes no compiler-Artifact, SDK, native-proof or ledger acceptance claim.
- The finite `core_boolean_check_public` target compares hand-derived complete
  Core Program JSON and public checker streams with frozen files in
  `crates/fixtures/core-boolean-check/` instead of generating insta snapshots.
  Its assertions include the complete source. Parser and checker source exposure
  is explicit; these static admission and serialization checks establish no
  runtime predicate truth, independent financial oracle, compiler Artifact,
  native-proof or ledger correspondence.
- Test programs shared by several crates live in `crates/fixtures/`, which is
  excluded from the workspace. Nothing under `crates/` reads files outside it;
  copy anything needed in.
- **Any test that consumes `.mori` source must put the source, not the Rust
  expression, in the snapshot.** Use a macro that takes an `indoc!` literal and
  sets `description` and `omit_expression`:

  ```rust
  insta::with_settings!({
      description => format!("Code:\n\n{}", input),
      omit_expression => true,
  }, {
      insta::assert_snapshot!(render(&output));
  });
  ```

- Use separate macros for success and error cases. Error snapshots render the
  diagnostic through miette's `GraphicalReportHandler` with
  `GraphicalTheme::unicode_nocolor()` and a fixed width, so the snapshot is
  exactly what a user sees.
- Snapshot the AST through a tree printer, not `Debug` of the struct-of-arrays.
- Every phase that consumes source has a `proptest` property that it never
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
  in the native-only targets. They are existing deviations from the no-remount
  rule above; this cut adds none and does not claim to resolve them.

## CLI

- [`clap`](https://crates.io/crates/clap) with derive, styled with cargo's colors
  through `clap::builder::styling::Styles`.
- Cargo-style status lines on stderr: a right-aligned, bold, colored verb, then
  details (`    Checking invoice.mori`, `    Finished check in 3.2ms`). Use
  `anstream` and `anstyle`.
- Results go to stdout. `--json` turns status lines off. JSON reports are our
  own format, not the TypeScript beta's; spans are UTF-8 byte ranges and every
  diagnostic has exactly one primary label.
- Global `-q`/`--quiet`, `-v`/`--verbose` and `--color` (`auto` respects
  `NO_COLOR` and non-terminal output). Verbose adds stage timings and
  token/node counts.
- Final verbs say exactly what happened, for example `Prepared (unqualified)`.
  Never imply proof, signature authority or ledger acceptance that did not occur.

## Later

- Port signature verification from `experiments/midnight-crypto`
  (`src/intent.rs`, `src/codec.rs`) so `verify-intent` runs natively. Its
  Midnight dependencies are Git-pinned; keep them behind a feature.
  `fixtures/intent-vectors.json` can serve as test vectors. A valid signature
  does not prove the signer controls the account; keep reporting possession as
  unchecked.
- The product target is ZKIRv3 on Midnight.
END FILE source/RUST_GUIDELINES.md

BEGIN FILE original-author/README.md | SHA256 029ba8766a5a2284175bfcecf4a1210c4f569103b3dc247db6b8b26a30c8e1cd | BYTES 3954
# Positive public Core Boolean source coverage

This unapplied candidate adds two admitted-language source fixtures and one public Rust integration test. It changes no production implementation. Exact base: 4c03df51f78337d1a9e17cfaaabbbea112fef0df, tree27b0211b04921485afae58603811549b7b93cb7d. The simulate diagnostic repair is already present and is not duplicated.

Predicate: both new documented positive Core/1 and Core/2 programs are admitted through their actual public Rust consumers; complete serialized Programs equal the frozen 21-node expectations; public `mori check` JSON stdout bytes and human stdout/stderr/exit match their complete frozen expectations. Static admission is the positive outcome. No evaluator, action runtime, compiler Artifact, Target, SDK, native or proof call is performed.

The retained bounded corpus lacks the named Lt/Or positive source forms in its full check/Program fixture pairs. BOUNDED-COVERAGE-DELTA.json enumerates that exact witness corpus and does not claim repository-wide absence. Production already implements these forms. The new composition fixes roots across prefix not, and/or precedence, repeated name occurrences, Ensures, Core/2 Await.guard/binder metadata, and a second action sharing the global arena. An implementation that changes a tree, forgets a statement/action or renumbers roots now differs from a complete frozen literal, rather than passing an operator-membership check.

The old Boolean proposal used undocumented `not =`. This candidate selects only the documented finite M01 grammar and leaves Ne untouched. The retained old27 obligations are copied byte-for-byte; no one of them is declared closed. The financial duty-read slice remains separate, requiring full independent financial/work expectations and an authorized runtime check; this source-only task does not use financial-budget availability as a reason to infer any result.

Expectations were manually enumerated before test authorship/execution. DERIVATION.md and EXPECTED-CASE-SPEC.json declare implementation exposure and every arena index. Core/2 has a normative finite M01 representation premise; Core/1 and public checker serialization use existing source-exposed contracts and finite precedent. This is ordinary author corroboration, not a blind independent semantic/financial oracle. EXPECTED-FREEZE.sha256 binds all exact input/expected bytes. No product outputs were used or observed by this author.

The integration target contains one aggregate Rust test and four actual CLI processes (two profiles × JSON/human). It captures every process stdout/stderr/exit and exact input before assertions, then records both actual exported API Program results before assertions. An optional absolute MORI_BOOLEAN_CHECK_OBSERVATIONS path must be fresh with an existing owned parent; it is retained on pass/fail. Without it, private temporary files are retained on failure and removed on success. Complete Program equality is a JSON-value comparison; complete public checker JSON/human output uses exact bytes.

current-source/ and SOURCE-PINS.json retain full actual source bodies for CLI dispatch, checker, parser, loader, validation/plan/cost/digest, serializer, lexer, declarations/DTO and bounded witness fixtures, all obtained from explicit Git objects. SOURCE-PINS-BEFORE/AFTER and candidate.patch contain eight new paths only: two sources, two complete Program fixtures, two complete check-output fixtures, fixture provenance and the public consumer regression. Full added source is under candidate/. No shared checkout or Git mutation occurred.

This is not a RED production-defect proposal: adding test coverage should pass unchanged implementation. Root owns the first real check and any result/audit integration. No parent acceptance, percentage, general normalization/equivalence, full grammar, financial/report/continuation, Program/Target/Artifact, U0–U7, M1–M17 or mandatory MC/SP/G exit closes here.
END FILE original-author/README.md

BEGIN FILE original-author/BOUNDED-COVERAGE-DELTA.json | SHA256 90680f9678af5c77e20efbc7c37bd9182328f28a56d5591fda45d4cdbefabacb | BYTES 4365
{
  "actualBase": "4c03df51f78337d1a9e17cfaaabbbea112fef0df",
  "existingWitnessScope": "All actual Core .mori sources under check-source, program-ir-contract and financial-report-contract fixture families, plus retained27 record; not whole repository absence",
  "witnesses": [
    {
      "path": "crates/fixtures/check-source/cases/core1-accepted.mori",
      "sha256": "5014535ca4879d3a6e1580f6dc3f08dcf511309eb8757f2c9c34d7e226a80526",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/core1-rejected.mori",
      "sha256": "eb7a9e96829f3ef9aae077f18adee18f334c4b6ab3b34db8463a9153af99d9e8",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/core2-accepted.mori",
      "sha256": "29d20136db0fbe85bc161e134a497696ef5c7e994b0787b492af0246660359dc",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/core2-rejected.mori",
      "sha256": "37d660cea5a09c6d85d9a8121a634320754cf833eee733efc208728ca5c24d19",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/financial-core1-loan.mori",
      "sha256": "764eb776e6175134e11ea2abcbf6e11b8025ca20d33863f98bfbca14082341ab",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/financial-core1-payment.mori",
      "sha256": "749636c36ea02807e1aa90258cb4285224a3be3c891d616861d5e4c090c38df5",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/financial-core2-escrow.mori",
      "sha256": "2b6acd88611b5866bb74f04d850771e6a512be898c66ea4f2aeb0cb86d5475ed",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/check-source/cases/financial-core2-rebate.mori",
      "sha256": "991a0317a16eae38c6454bfe640fc3da01e387706c39adcc6c47c45e8581bd6e",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/program-ir-contract/counter.mori",
      "sha256": "8e89db83018345a771224e3b20218eb06eb905562d4d3bc5f239b3aff41f07b0",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/program-ir-contract/empty-frame.mori",
      "sha256": "98e522971de7f008c019f662c7fc410c37eb80f954a54420957d004cbef6d9c2",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    },
    {
      "path": "crates/fixtures/financial-report-contract/conditional-reserve.mori",
      "sha256": "7cb7c96bae56f344467deb5ac03fe8157fce356527eae608f68dbeb10ac71fa1",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": true
    },
    {
      "path": "crates/fixtures/financial-report-contract/installment-invoice.mori",
      "sha256": "2f07d17b3aa9634819f65acbc64945af29588b1258d196fe30f55b8a377292c3",
      "LtTokenObserved": false,
      "OrWordObserved": false,
      "prefixNotWordObserved": false
    }
  ],
  "proposedSlice": [
    "Core1 and Core2 full Program + complete public check positive combinations of documented Lt/Or/Not/And/Gt",
    "Core2 Await.guard full arena allocation and event-binder metadata; Ensures exact root retained",
    "second action retained in whole source; global arena does not reset"
  ],
  "coverageClaim": "new source/check/Program observation only after root execution; no production defect or missing operator implementation alleged",
  "retained27UnchangedSha256": "4e250acb8b73e7176bdcc03f9598720423d672c52fa4bf47077d96484f347452",
  "all27Open": true,
  "Ne": "not selected; undocumented spelling remains unresolved",
  "financialN3": "duty reads/financial prefixes deferred to independent full financial/work expectations and separate execution scope; not exercised here"
}
END FILE original-author/BOUNDED-COVERAGE-DELTA.json

BEGIN FILE original-author/RETAINED-27-CONSTRUCTOR-SCOPE.json | SHA256 4e250acb8b73e7176bdcc03f9598720423d672c52fa4bf47077d96484f347452 | BYTES 2407
{
  "constructors": {
    "Expr": [
      "LiabilityPresent",
      "LiabilityAmount",
      "Literal",
      "Name",
      "Binary",
      "Not"
    ],
    "BinaryOp": [
      "Add",
      "Sub",
      "Mul",
      "Eq",
      "Ne",
      "Lt",
      "Le",
      "Gt",
      "Ge",
      "And",
      "Or"
    ],
    "Statement": [
      "Await",
      "Let",
      "Requires",
      "Ensures",
      "Next",
      "Emit",
      "Transfer",
      "Fee",
      "Liability",
      "Discharge"
    ]
  },
  "count": 27,
  "core1Statements": 9,
  "core2Statements": 10,
  "scope": "Seven complete Program oracle pairs plus eight selected normal test-source witnesses",
  "core1Gaps": [
    "BinaryOp.Ne",
    "BinaryOp.Lt",
    "BinaryOp.Or"
  ],
  "core2Gaps": [
    "Expr.LiabilityPresent",
    "Expr.LiabilityAmount",
    "Expr.Not",
    "BinaryOp.Ne",
    "BinaryOp.Lt",
    "BinaryOp.Gt",
    "BinaryOp.Or",
    "Statement.Ensures"
  ],
  "nextPositiveTargets": [
    {
      "id": "N1",
      "targets": [
        "core1.BinaryOp.Ne",
        "core1.BinaryOp.Lt",
        "core1.BinaryOp.Or"
      ],
      "sourceProposal": "proposal/crates/fixtures/positive-grammar-edges/core1.mori",
      "test": "core1_positive_less_inequality_and_disjunction"
    },
    {
      "id": "N2",
      "targets": [
        "core2.Expr.Not",
        "core2.BinaryOp.Ne",
        "core2.BinaryOp.Lt",
        "core2.BinaryOp.Or",
        "core2.Statement.Ensures"
      ],
      "sourceProposal": "proposal/crates/fixtures/positive-grammar-edges/core2.mori",
      "test": "core2_positive_boolean_values_survive_await_and_ensure"
    },
    {
      "id": "N3",
      "targets": [
        "core2.Expr.LiabilityPresent",
        "core2.Expr.LiabilityAmount",
        "core2.BinaryOp.Gt"
      ],
      "status": "specified-only",
      "nextStep": "Extend an independently frozen normal staged duty specimen with current-snapshot reads and positive > guard. Freeze complete ordinary financial/state/work expectation first."
    }
  ],
  "allGapsRemainOpen": true,
  "notWholeGrammarClosure": true,
  "currentness": "Exact retained corpus only. Other newly added tests were not exhaustively classified; no whole-repository absence claim.",
  "scope_status": "All 27 constructor coverage obligations remain OPEN; no row or profile inferred closed by this task.",
  "current_base": "5f7c2a28efb69d36e171d85ffd39bf59f8e68c68"
}
END FILE original-author/RETAINED-27-CONSTRUCTOR-SCOPE.json

BEGIN FILE original-author/SOURCE-TRACE.md | SHA256 1d56b9a13102302b0e92435cc58d389cb4634d23e13df362d7bb8b35ab776d32 | BYTES 2370
# Source path and premises

All references are exact actual Git objects retained complete in current-source/ with SOURCE-PINS.json blobs/SHA256.

- Original U2 grammar requirements and UNI-018/019/020/021 are retained in ROADMAP.md and consolidated-language-kernel spec. They require profile/consumer/subset honesty and scoped observations, not a whole-language pass inferred from fixed examples.
- CLI main.rs/cli.rs and commands/check.rs invoke the actual public check command. check_source.rs:dispatch chooses real Core source via first_token_text/language, Core/1 parse or Core/2 stage::load. core_report retains source hash, full action metadata/statement roots and exact stage labels. No target/backend is used.
- mori-core/src/parser.rs:parse_profile reads real declarations/actions/inputs; expr defines documented precedence and child-first fresh occurrences; statement retains roots. ir.rs gives complete Program/Action/Expr/Statement/Value DTOs. runtime.rs:validate supplies Core/1 static types/names, not an evaluator call. stage/mod.rs:load uses parse_v2, validate_v2, plan/invariant/static cost/digest. Complete loader dependencies are retained; no stage start/resume is invoked.
- docs/CORE-STAGED-TESTS.md adopted M01 gives explicit whole Program, fresh global arena, precedence, statement/binder boundaries and documented grammar. It does not canonize Ne spelling or close GAP-05 by source exposure. Core/1 finite Program precedents are retained under fixtures/program-ir-contract and financial-report-contract; general representation correspondence remains open.
- check_source.rs:core_report and stage/mod.rs:STAGE_LABELS define the whole checker DTO used. mori-core/src/intent.rs:hash_bytes is plain SHA256; expected hashes are computed independently from exact source bytes with Python hashlib. core_io.rs:print and mori-core/src/json.rs:export define pretty output; current complete public check fixtures supply finite serialization precedent. shell.rs:status/aligned gives fixed right-aligned verbs, normal color-never lines, without Core timing. Full stdout/stderr/exit are asserted.
- Fixture-family enumeration only establishes a bounded witness gap. It does not assert no test elsewhere uses an operator. Original27 inventory, unsupported grammar, independent financial/work/native/target/Artifact requirements and all staged gaps remain open.
END FILE original-author/SOURCE-TRACE.md

BEGIN FILE before/RUST_GUIDELINES.md | SHA256 7ac417984778c52387d49295364b1e2f85bde14d04d2130dc2c09b4f3bf9d404 | BYTES 18547
# Rust toolchain guidelines

Rules for the Rust implementation of the Moriarty toolchain in `crates/`.
Current checkout `AGENTS.md`, the product contract and explicit user steering
control profile scope, authorization and execution boundaries. Routine repairs
within existing authorization need no additional approval. Record intentional
differences; these guidelines do not grant compiler, SDK or native execution.

## Target language

Port `moriarty-beta/1`, the `.mori` language. The reference implementation is
[`packages/moriarty-beta/src/frontend.ts`](packages/moriarty-beta/src/frontend.ts);
the [reference page](https://charleshoskinson.github.io/Moriarty/reference.html#language)
documents it. Beta lowers to Source/6 and Core/5
([contract](experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md)).

The surface syntax is not frozen. Keep grammar-specific code separate from the
later phases so it can change. The Rust toolchain accepts and rejects the same
programs as the TypeScript frontend, except for the intentional differences
below. Error codes, messages and spans are our own.

### Intentional differences

The TypeScript frontend is the reference for what the language accepts and
what values mean, not for incidental implementation choices. Every deliberate
difference is listed here under the applicable user authorization.

- **Comments do not count toward the 8,192-token bound.** The bound limits
  analysis, and comments are not analyzed. The TypeScript lexer counts them, so
  a file whose comments push it past the bound is rejected there and accepted
  here. The 65,536-byte source bound still caps comments.
- **`mori format` formats any file that parses,** including files with name or
  type errors, as rustfmt and prettier do. The TypeScript formatter refuses any
  file with an error. Its layout also differs: ours fits lines to 80 columns.
  And `mori format` (alias `fmt`) writes in place by default, with `--check` instead of the
  TypeScript CLI's `--write`.
- **Syntax errors are reported before name and type errors.** The TypeScript
  frontend resolves names while parsing, so in a file with several errors it
  may report a different first one. Files with a single error behave the same.

### Adopted explicit Source/6 consumer disposition

The adopted `mori expand --emit-source6-transfer` interface, behind the non-default
`source6-expansion` feature, is a separate local transfer-only consumer. It
keeps default Rust Core expansion and the Node beta package unchanged. It
accepts the closed transfer subset of `moriarty-local-scenario/1`, refuses
repayment/family actions before scenario I/O, and explicitly refuses obligation
and candidate-effect override fields. It emits raw Source/6 text with its final
newline and reports local-stipulation-only on stderr; quiet suppresses that
status line. It does not claim full Node BetaExpansion JSON/provenance parity.
Existing Rust diagnostics may differ in code and presentation. The
[2026-10-18 root decision](evidence/utxo/doctorwatson-language-loop-2026-10-17/source6-public-consumer-adoption-20261018/DESIGN-ADOPTION.json)
adopts this ordinary interface only. Compilation, public CLI execution, actual
Source/6 lowerer consumption and native/security qualification remain unrun or
unestablished. The existing independent14 financial payload baseline predates
this consumer; complete wire/output/work expectations and the chosen WorkshopReceipt
fixture freeze remain missing. Interface adoption supplies no native embedding,
security approval or language-exit acceptance.

## Workspace

- Every crate lives in `crates/*`. The `mori` crate is the CLI entry point.
- The toolchain is pinned in `rust-toolchain.toml` (Rust 1.99.0, edition 2024).
- Declare every dependency version once in the root `[workspace.dependencies]`.
  Crates reference it with `dep = { workspace = true }` or `dep.workspace = true`.
- The pre-existing Rust crates under `experiments/`, `raw/` and `wiki-llm/` are
  not part of this toolchain. They are excluded from the workspace so they still
  build standalone; the beta CLI tells users to build `experiments/midnight-crypto`
  with `--manifest-path`, which fails if the workspace claims it.
- Every Rust file builds and tests through Cargo. No harness compiles a module
  with `rustc` directly, and no `#[path]` mounts a file from another directory
  or loads one file as several modules; share code through a crate or a module
  and `use` it.
- Code only tests use lives in test modules. Unadopted prototypes go in
  `experiments/` or behind a feature, not in `src/` gated by `#[cfg(test)]`.
- Never hash source text to identify an ABI, deployment, key or build. Any
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
  array, as Zig does.
- **Data-oriented layout.** Tokens, AST nodes and comments are struct-of-arrays
  containers built with [`soa-rs`](https://crates.io/crates/soa-rs)
  (`#[derive(Soars)]`). Nodes refer to tokens and other nodes by index handles,
  never by pointers or boxes. Variable-length node data goes in an `extra`
  `Vec`. Tag enums are `#[repr(u8)]`. Node payloads stay plain `lhs`/`rhs`
  fields interpreted by tag, not Rust enums.
- **Small handles.** Beta caps source at 65,536 bytes and tokens and nodes at
  8,192 each, so token and node handles are `u16` newtypes and byte offsets are
  `u32`. `soa-rs` indexes by `usize`; convert at the handle boundary.
- **Preallocate.** Size the token container with `source.len() / 8`, as Zig does.
- **Tokens store only `tag` and `start`.** The end is recovered by re-lexing.
- **Comments are kept for the formatter** in their own `start`/`end` table, not
  in the token stream. Only tokens, including the end-of-file token, count toward
  the 8,192-token bound; whitespace and comments do not.
- **Crates.** `mori-span` holds spans, `mori-lexer` the tokens and lexer,
  `mori-ast` the struct-of-arrays AST and its tree printer, `mori-parser` the
  parser that produces it, and `mori-checker` the checker that consumes it.
- **Separate phases.** Lex → parse → resolve and check → evaluate constants.
  Do not copy the TypeScript parser, which evaluates while parsing.
- **Numbers.** Values fit in `u128`; no bigint library. Decimal quantities are
  scaled by their asset during checking.
- **Node spans are computed, not stored.** Like Zig's `firstToken` and
  `lastToken`, `Ast` derives a node's first and last token from its tag and
  children when a diagnostic needs it.

## Checker

- **One `mori-checker` crate,** with modules for names, values, declaration
  rules, calls and intents. Checking is one ordered walk: each declaration's
  names are resolved, its value computed and its rules checked before the next,
  because later declarations depend on earlier values. Its codes are
  `mori::check::<kind>`.
- **Output is side tables and a value store,** in the same style as the AST: a
  `Soa` of declarations (kind, name token, value handle) and a `Soa` of values
  with `u16` handles, an `extra` array for records and lists, and amounts in a
  separate `u128` array. No boxed value trees.
- **`check` returns what was checked together with its diagnostics,** so valid
  declarations stay usable when others fail, as a language server needs. A
  program is accepted only when there are no diagnostics.
- **It reports every independent error.** A declaration that fails is marked
  failed, and anything depending on it is skipped without further errors, so
  one mistake never cascades. Which programs are accepted does not change.
- **The built-in call schemas are a static Rust table:** each call's name and
  its required and optional arguments with their expected types.

## Formatter

- **`mori-fmt` is a Wadler-style pretty printer.** It builds a document of
  text, line breaks, groups and indentation from the AST, then fits each group
  on one line within 80 columns or breaks it one item per line, with trailing
  commas when broken.
- **No `Format` trait.** The document is built by a `match` over `NodeTag`,
  like the other phases, into an arena of rows.
- **Output depends only on the program and its comments.** The profile line and
  every agreement item are followed by exactly one blank line, and comments
  above an item stay attached to it. Numbers and strings keep their spelling.
- **Comments come from the comment table** and stay next to the token they
  were written beside: on the same line, or on their own line above.
- **Correctness is proven by tests, not checked at run time.** Snapshot tests
  show each layout case with its `.mori` source, and every one also asserts
  that formatting the output again changes nothing. A property test checks
  that output reparses to the same tokens and comments.
- **`mori format [PATHS]...` (alias `fmt`) rewrites files in place.** A file formats that
  file, a directory every `.mori` file under it, and no arguments the current
  directory. Directories are walked with `ignore` (respecting `.gitignore`)
  and files are formatted in parallel with `rayon`; output is printed in
  sorted path order. A file that does not parse is reported and skipped.
- **`mori format --check [PATHS]...`** writes nothing and exits 1 when a file is
  not formatted, for CI. Each such file gets a `mori::fmt::not_formatted`
  diagnostic header (no snippet, no help), then the diff exactly as
  `cargo fmt --check` prints it (one `Diff in FILE:LINE:` per region, three
  context lines, red `-` and green `+`, via `similar`), indented to line up
  with the message. One summary line at the end says how to fix it.

## Diagnostics

- Follow [oxc](https://github.com/oxc-project/oxc): one open `MoriDiagnostic`
  struct (message, labels, help, severity, code) in a `mori-diagnostics` crate,
  implementing miette's `Diagnostic` trait. Do not create a struct or enum
  variant per error.
- Each compiler crate has a `diagnostics.rs` of builder functions:

  ```rust
  pub fn quantity_separator(span: Span, source: &str) -> MoriDiagnostic {
      MoriDiagnostic::error("A quantity needs a space between the number and its asset.")
          .with_code("mori::syntax::quantity_without_space")
          .with_label(span.label("no space here"))
          .with_help_code("Put a space between them:", corrected_line(source, span, ...))
  }
  ```

- Every phase builds a `MoriDiagnostic` where the error is found, through a
  builder in its `diagnostics.rs`. Errors are rare and never on a hot path.
- Messages must be Elm-quality: say what was being parsed, what was expected,
  show a correct example and give a specific hint. Errors should teach the
  language.
- To support that, the parser keeps a context stack (agreement body,
  declaration, record fields, call arguments, type annotation) and records the
  set of expected token tags at each failure.
- Common mistakes get their own codes and teaching messages, for example:
  `10USD` (missing space before the asset), a decimal without an asset, a
  forward reference, an unknown call (suggest the closest of the known calls),
  a reserved word as a name, `/` (no division), a dotted name that is not a
  call, and adding quantities of different assets.
- Codes follow miette's style, one per kind of error:
  `mori::<phase>::<kind>`, such as `mori::lex::leading_zero` or
  `mori::syntax::missing_comma`.
- No title banners. A help that suggests a fix shows the corrected code on its
  own indented lines, not inside a sentence:

  ```
    help: Fields in a record are separated by commas:

              { domain: Preview, id: "A" }
  ```
- Spans are UTF-8 byte ranges, matching miette's `SourceSpan` and beta. Editors
  and a future language server need UTF-16 line/character positions, converted
  from the same compact records.
- The lexer and parser stop at the first error, like the TypeScript frontend.
  Recovery at declaration boundaries may come later.
- `mori-span` converts `Span` into miette's span types, as `oxc_span` does.
- The `fancy` feature of `mori-diagnostics` enables miette's
  `fancy-no-backtrace`. The `mori` CLI turns it on; library crates turn it on
  only in `[dev-dependencies]`.
- Use `thiserror` for errors that never point at source, such as I/O. No
  `anyhow` in library crates.

## Testing

- Snapshot tests use [`insta`](https://insta.rs) and `cargo-insta`.
- The finite backend `financial_observation_relation` target uses frozen
  complete Program/Execution literals and positive SDK relation checks instead
  of insta snapshots. The exact sources and independent expectations remain in
  `crates/fixtures/financial-report-contract/`; expected values must not come
  from the evaluator, compiler output or generated baselines. This exception
  adds no authority to execute SDK checks or claim native/proof acceptance.
- The finite Core `financial_program_wire_contract` target uses independently
  frozen complete financial Program JSON and the public Program/v1 decoder
  instead of generated snapshots. It selects no Midnight compatibility feature
  and makes no compiler-Artifact, SDK, native-proof or ledger acceptance claim.
- Test programs shared by several crates live in `crates/fixtures/`, which is
  excluded from the workspace. Nothing under `crates/` reads files outside it;
  copy anything needed in.
- **Any test that consumes `.mori` source must put the source, not the Rust
  expression, in the snapshot.** Use a macro that takes an `indoc!` literal and
  sets `description` and `omit_expression`:

  ```rust
  insta::with_settings!({
      description => format!("Code:\n\n{}", input),
      omit_expression => true,
  }, {
      insta::assert_snapshot!(render(&output));
  });
  ```

- Use separate macros for success and error cases. Error snapshots render the
  diagnostic through miette's `GraphicalReportHandler` with
  `GraphicalTheme::unicode_nocolor()` and a fixed width, so the snapshot is
  exactly what a user sees.
- Snapshot the AST through a tree printer, not `Debug` of the struct-of-arrays.
- Every phase that consumes source has a `proptest` property that it never
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
  in the native-only targets. They are existing deviations from the no-remount
  rule above; this cut adds none and does not claim to resolve them.

## CLI

- [`clap`](https://crates.io/crates/clap) with derive, styled with cargo's colors
  through `clap::builder::styling::Styles`.
- Cargo-style status lines on stderr: a right-aligned, bold, colored verb, then
  details (`    Checking invoice.mori`, `    Finished check in 3.2ms`). Use
  `anstream` and `anstyle`.
- Results go to stdout. `--json` turns status lines off. JSON reports are our
  own format, not the TypeScript beta's; spans are UTF-8 byte ranges and every
  diagnostic has exactly one primary label.
- Global `-q`/`--quiet`, `-v`/`--verbose` and `--color` (`auto` respects
  `NO_COLOR` and non-terminal output). Verbose adds stage timings and
  token/node counts.
- Final verbs say exactly what happened, for example `Prepared (unqualified)`.
  Never imply proof, signature authority or ledger acceptance that did not occur.

## Later

- Port signature verification from `experiments/midnight-crypto`
  (`src/intent.rs`, `src/codec.rs`) so `verify-intent` runs natively. Its
  Midnight dependencies are Git-pinned; keep them behind a feature.
  `fixtures/intent-vectors.json` can serve as test vectors. A valid signature
  does not prove the signer controls the account; keep reporting possession as
  unchecked.
- The product target is ZKIRv3 on Midnight.
END FILE before/RUST_GUIDELINES.md

BEGIN FILE GUIDELINE-EXACT-DELTA.diff | SHA256 87ef5da6abf8003980d044418a0ff2d8a4f49fdb1a875f06e67b8edb353f91a9 | BYTES 1032
--- main829f/before/RUST_GUIDELINES.md
+++ current-candidate/RUST_GUIDELINES.md
@@ -253,6 +253,13 @@
   frozen complete financial Program JSON and the public Program/v1 decoder
   instead of generated snapshots. It selects no Midnight compatibility feature
   and makes no compiler-Artifact, SDK, native-proof or ledger acceptance claim.
+- The finite `core_boolean_check_public` target compares hand-derived complete
+  Core Program JSON and public checker streams with frozen files in
+  `crates/fixtures/core-boolean-check/` instead of generating insta snapshots.
+  Its assertions include the complete source. Parser and checker source exposure
+  is explicit; these static admission and serialization checks establish no
+  runtime predicate truth, independent financial oracle, compiler Artifact,
+  native-proof or ledger correspondence.
 - Test programs shared by several crates live in `crates/fixtures/`, which is
   excluded from the workspace. Nothing under `crates/` reads files outside it;
   copy anything needed in.
END FILE GUIDELINE-EXACT-DELTA.diff

BEGIN FILE source/crates/mori/src/cli.rs | SHA256 b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789 | BYTES 8998
use std::path::PathBuf;

use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::commands::{
    capabilities, check, compile, compile_call, expand, format, init, inspect, intent, keygen_call,
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

/// Selects one action of an agreement and the scenario to run it against.
#[derive(Debug, Args)]
pub struct ActionInput {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME")]
    pub action: String,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE")]
    pub scenario: PathBuf,
}

/// Output format for the signing commands.
#[derive(Debug, Args)]
pub struct SigningOutputArgs {
    /// Print a human review of what is being signed or verified
    #[arg(long, conflicts_with = "json")]
    pub review: bool,

    /// Print the result as JSON
    #[arg(long)]
    pub json: bool,
}

/// Signed native owner-intent profile selected explicitly by name.
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum SignedProfile {
    #[value(name = "moriarty-jubjub-owner-intent/1")]
    JubjubOwnerIntent,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Color {
    Auto,
    Always,
    Never,
}

impl From<Color> for anstream::ColorChoice {
    fn from(color: Color) -> Self {
        match color {
            Color::Auto => Self::Auto,
            Color::Always => Self::Always,
            Color::Never => Self::Never,
        }
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Describe compiled features and public source consumers without running them
    Capabilities(capabilities::Args),
    /// Create a new project from a starter template
    Init(init::Args),
    /// Check syntax, names, types and action support
    Check(check::Args),
    /// Compile a bounded Moriarty program into ledger-9 ZKIRv3 circuits
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Compile(compile::Args),
    /// Compile a source-static ordinary native contract-call relation
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    CompileCall(compile_call::Args),
    /// Derive all source action keys and an explicit local installed snapshot
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    KeygenCall(keygen_call::Args),
    /// Derive the complete local signed action map under a fixed JubJub policy
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    KeygenSignedCall(keygen_signed_call::Args),
    /// Reconstruct a signature-free canonical local call signing package
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    PrepareSignedCall(prepare_signed_call::Args),
    /// Prove the actual local typed SDK call with an external FENS attachment
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    ProveSignedCall(prove_signed_call::Args),
    /// Cold public-only reconstruction and Real verification of a signed call
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    VerifySignedCall(verify_signed_call::Args),
    /// Cold rederive keys and prove an actual local installed SDK call
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    ProveCall(prove_call::Args),
    /// Cold rederive public keys and Real-verify an installed call without PK files
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    VerifyCall(verify_call::Args),
    /// Prove a public local action using pinned offline parameters
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Prove(prove::Args),
    /// Prepare an unsigned, unproven local native call and public offer
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    PrepareCall(prepare_call::Args),
    /// Verify a native proof against the expected source, target and scenario
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Verify(verify::Args),
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, source-only inspection is available; --target requires the native feature."
        )
    )]
    Inspect(inspect::Args),
    /// Show the native core IR after checking an action against a scenario
    Expand(expand::Args),
    /// Run an action locally against a scenario
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, local Core simulation and staged start/resume are available; --trace requires the native feature."
        )
    )]
    Simulate(simulate::Args),
    /// Independently replay a public record against separately trusted inputs
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Unavailable in this build: this command requires the native feature."
        )
    )]
    Replay(replay::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, legacy raw Core intents are available; the JubJub --profile requires the native feature."
        )
    )]
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    #[cfg_attr(
        not(feature = "native"),
        command(
            after_help = "Without native, legacy Core signature/reference verification is available; the JubJub --profile requires the native feature."
        )
    )]
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}
END FILE source/crates/mori/src/cli.rs

BEGIN FILE source/crates/mori/src/commands/expand.rs | SHA256 f7ff48dcaece95b05392e25ad3a9dff66dd736f839ceaa2928bf9d768aedc320 | BYTES 1601
use std::process::ExitCode;

use crate::cli::ActionInput;
use crate::shell::Shell;

#[cfg(feature = "source6-expansion")]
mod source6;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Emit a local Source/6 transfer proposal; requires source6-expansion
    #[arg(
        long,
        long_help = "Emit raw local Source/6 transfer proposal text. Requires the source6-expansion build feature; default Core expansion is unchanged."
    )]
    pub emit_source6_transfer: bool,
}

pub fn run(args: Args, shell: &mut Shell) -> super::Result<ExitCode> {
    if args.emit_source6_transfer {
        return run_source6(args.input, shell);
    }
    let (_, program) = super::core_io::program(&args.input.file)?;
    // Validate the selected scenario/action without discarding the generic core.
    let scenario = super::core_io::read_json(&args.input.scenario)?;
    mori_core::simulate(&program, &args.input.action, &scenario)
        .map_err(super::core_io::operation)?;
    super::core_io::print(&program)?;
    Ok(ExitCode::SUCCESS)
}

#[cfg(feature = "source6-expansion")]
fn run_source6(input: ActionInput, shell: &mut Shell) -> super::Result<ExitCode> {
    source6::run(input, shell)
}

#[cfg(not(feature = "source6-expansion"))]
fn run_source6(_input: ActionInput, _shell: &mut Shell) -> super::Result<ExitCode> {
    Err(crate::error::CliError::Operation {
        code: "mori::command::source6-unavailable".into(),
        message: "Source/6 transfer expansion requires the source6-expansion build feature.".into(),
    }
    .into())
}
END FILE source/crates/mori/src/commands/expand.rs

BEGIN FILE source/crates/mori/src/error.rs | SHA256 9c1f5a1207cfaf2509e159cdeed344a60b172e196988d021acab6cc9a45b918c | BYTES 946
//! Command failures that do not point at source code.

use std::io;
use std::path::PathBuf;

use mori_diagnostics::Diagnostic;

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("{code}: {message}")]
    Operation { code: String, message: String },
    #[error("could not read `{}`", crate::display::text(&path.display().to_string()))]
    Read {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("could not write `{}`", crate::display::text(&path.display().to_string()))]
    Write {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("`{}` does not exist", crate::display::text(&path.display().to_string()))]
    NotFound { path: PathBuf },
    #[error("could not list `{}`", crate::display::text(&path.display().to_string()))]
    Walk {
        path: PathBuf,
        #[source]
        source: ignore::Error,
    },
}

impl Diagnostic for CliError {}
END FILE source/crates/mori/src/error.rs

BEGIN FILE source/crates/mori/src/json.rs | SHA256 6f68940fe32eced929b8af229336eb07e9af745a78b32f45966858c7c382a3aa | BYTES 7919
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

    /// Checks the source and snapshots its JSON report, recording the source.
    macro_rules! assert_report_snapshot {
        ($source:literal) => {{
            let source = indoc::indoc!($source);
            let report = match mori_parser::parse(source) {
                Err(error) => CheckReport::new("test.mori".to_owned(), source, None, &[error]),
                Ok(ast) => {
                    let result = mori_checker::check(&ast);
                    CheckReport::new("test.mori".to_owned(), source, Some(&result.checked), &result.diagnostics)
                }
            };
            insta::with_settings!({
                description => format!("Code:\n\n{source}"),
                omit_expression => true,
            }, {
                insta::assert_snapshot!(serde_json::to_string_pretty(&report).unwrap());
            });
        }};
    }

    #[test]
    fn accepted_report() {
        assert_report_snapshot!(
            r#"
            profile "moriarty-beta/1";
            agreement Oracle {
              domain Preview = { id: "preview", chain: "midnight", network: "preview" };
              observation price = { id: "price", domain: Preview };
              intent read = { operation: oracle.select(observation: price) };
              action fetch uses read;
            }
            "#
        );
    }

    #[test]
    fn rejected_report() {
        assert_report_snapshot!(
            r#"
            profile "moriarty-beta/1";
            agreement Invoice {
              const price = 10;
              const price = pirce;
            }
            "#
        );
    }

    #[test]
    fn unparsable_report() {
        assert_report_snapshot!(
            r#"
            profile "moriarty-beta/1";
            agreement Invoice {
              const price = 10
            "#
        );
    }
}
END FILE source/crates/mori/src/json.rs

BEGIN FILE source/crates/mori-diagnostics/src/lib.rs | SHA256 f0f50241dc901df6d66b3bc25c9c32c085b2de2b4dd7b8f04b89603b4ef5117c | BYTES 609
//! Diagnostics for the Moriarty toolchain.
//!
//! Every phase reports problems through one open [`MoriDiagnostic`] type,
//! following oxc's `OxcDiagnostic`. Phases define small constructor functions
//! in their own `diagnostics.rs` instead of a type per error.
//!
//! The `fancy` feature enables the [`render`] module and miette's graphical
//! handler.

mod diagnostic;
#[cfg(any(feature = "fancy", test))]
pub mod render;
mod suggest;

pub use diagnostic::MoriDiagnostic;
pub use miette::{Diagnostic, LabeledSpan, NamedSource, Report, Severity, SourceSpan};
pub use suggest::{corrected_line, suggest};
END FILE source/crates/mori-diagnostics/src/lib.rs

BEGIN FILE source/crates/mori-diagnostics/src/diagnostic.rs | SHA256 41bc1a257e594fc8da0c625c6856d61ac3036afc5b2ae3636289ff411eae78f1 | BYTES 3407
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

    /// Sets the stable diagnostic code, such as `mori::syntax::missing_comma`.
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

    /// Sets a help that ends with example code on its own indented lines.
    pub fn with_help_code(self, help: impl AsRef<str>, code: impl AsRef<str>) -> Self {
        let mut text = format!("{}\n", help.as_ref());
        for line in code.as_ref().lines() {
            text.push('\n');
            if !line.is_empty() {
                text.push_str("    ");
                text.push_str(line);
            }
        }
        self.with_help(text)
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
END FILE source/crates/mori-diagnostics/src/diagnostic.rs

BEGIN FILE source/crates/mori-diagnostics/src/render.rs | SHA256 b28d889ac235f07a7513995ef5bbffd42b1999ce95d8cc2056bd2774f5559bdc | BYTES 3348
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
END FILE source/crates/mori-diagnostics/src/render.rs

BEGIN FILE units/loader-canonical-json-complete.rs | SHA256 10af6779f70596a021b2cb193c7243611f2b8921dae2e7b3eef181915dfe54c5 | BYTES 770
pub fn canonical<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    fn sort(v: serde_json::Value) -> serde_json::Value {
        match v {
            serde_json::Value::Object(map) => {
                let sorted: std::collections::BTreeMap<_, _> =
                    map.into_iter().map(|(k, v)| (k, sort(v))).collect();
                serde_json::Value::Object(sorted.into_iter().collect())
            }
            serde_json::Value::Array(a) => {
                serde_json::Value::Array(a.into_iter().map(sort).collect())
            }
            v => v,
        }
    }
    let value = serde_json::to_value(value).map_err(|e| Error::new("intent", e.to_string()))?;
    serde_json::to_vec(&sort(value)).map_err(|e| Error::new("intent", e.to_string()))
}
END FILE units/loader-canonical-json-complete.rs

BEGIN FILE units/source-and-loader-hex-complete.rs | SHA256 1111d9a5818faa5bb64d5e466f3ba7527fbc8f1611f83ae2ced8a26bacc4ee15 | BYTES 94
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
END FILE units/source-and-loader-hex-complete.rs

BEGIN FILE actual/CHECK-PLAN.json | SHA256 93d3fd28696deaae15889b7a25108526018a346f260e5a0de8e0c3823968d95d | BYTES 3706
{
  "recordedAt": "2026-10-18T18:12:50.223300+00:00",
  "baseHead": "829f4456dfda8d4fc5a4ece84c47f2539f846d9f",
  "baseTree": "76dac56aac5e653729f3e4ad546b74ba07f1f780",
  "expectationFreezeSha256": "1287136fcb4e57450f28f55591fae8671466018ca8fdd1df93b1b706829b7598",
  "sourcePinsSha256": "e57986dba2ec596b4ad3fff8f5d7814aea527d8784dc2544f6bf16a85750a6dc",
  "argv": [
    "/usr/bin/timeout",
    "--signal=TERM",
    "--kill-after=5s",
    "90s",
    "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
    "test",
    "--manifest-path",
    "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/Cargo.toml",
    "--frozen",
    "-j1",
    "-p",
    "mori",
    "--no-default-features",
    "--test",
    "core_boolean_check_public",
    "documented_boolean_programs_reach_public_core_check",
    "--",
    "--exact",
    "--test-threads=1"
  ],
  "environment": {
    "PATH": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin:/usr/bin:/bin",
    "RUSTC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
    "RUSTDOC": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
    "CARGO_HOME": "/tmp/doctorwatson-boolean-main-wocvhhvg/cargo-home",
    "CARGO_TARGET_DIR": "/tmp/doctorwatson-boolean-main-wocvhhvg/target",
    "CARGO_NET_OFFLINE": "true",
    "CARGO_INCREMENTAL": "0",
    "CARGO_CACHE_AUTO_CLEAN_FREQUENCY": "never",
    "GIT_CONFIG_NOSYSTEM": "1",
    "GIT_CONFIG_GLOBAL": "/dev/null",
    "GIT_CONFIG_SYSTEM": "/dev/null",
    "LANG": "C.UTF-8",
    "TMPDIR": "/tmp/doctorwatson-boolean-main-wocvhhvg/tmp",
    "MORI_BOOLEAN_CHECK_OBSERVATIONS": "/home/charl/moriarty-c2-opus55-restored-20261017/boolean-main-integration-check-20261018/public-cli"
  },
  "ownedRoot": "/tmp/doctorwatson-boolean-main-wocvhhvg",
  "absentAncestorCargoConfigs": [
    "/.cargo/config",
    "/.cargo/config.toml",
    "/home/.cargo/config",
    "/home/.cargo/config.toml",
    "/home/charl/.cargo/config",
    "/home/charl/.cargo/config.toml",
    "/home/charl/Moriarty/.cargo/config",
    "/home/charl/Moriarty/.cargo/config.toml",
    "/home/charl/Moriarty/.worktrees/.cargo/config",
    "/home/charl/Moriarty/.worktrees/.cargo/config.toml",
    "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/.cargo/config",
    "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/.cargo/config.toml",
    "/tmp/.cargo/config",
    "/tmp/.cargo/config.toml",
    "/tmp/doctorwatson-boolean-main-wocvhhvg/.cargo/config",
    "/tmp/doctorwatson-boolean-main-wocvhhvg/.cargo/config.toml",
    "/tmp/doctorwatson-boolean-main-wocvhhvg/cwd/.cargo/config",
    "/tmp/doctorwatson-boolean-main-wocvhhvg/cwd/.cargo/config.toml"
  ],
  "toolPins": [
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/cargo",
      "sha256": "e951141cc55a6cd7b9876d187bd30a3720e6086b99413af95e3b8de1cdd72f14"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustc",
      "sha256": "f3834d26669b03f6855fa54bd2381443123e860167bc0c7a8dd137e5cf6e5e4f"
    },
    {
      "path": "/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin/rustdoc",
      "sha256": "a1321a60578a6b2e3e0ff428201b8618bbd9a9fbcd475c0d0f151bbf17cb0a9e"
    },
    {
      "path": "/usr/bin/timeout",
      "sha256": "f84e76ea4da1e0c4e92254bd0d7dccdabe2c8b3098e734f27997dae3ee5d8e02"
    }
  ],
  "scope": "Two positive Core1/Core2 Boolean programs, complete API Program values and four public checker streams on independent header-main base. Source-exposed hand expectations; no evaluator/financial/native/SDK/proof/wallet/ledger run; no RED defect claimed."
}
END FILE actual/CHECK-PLAN.json

BEGIN FILE actual/CHILD.json | SHA256 5c8d2b39350f4dd2cf7f9651ffb77cc4eb4664d556eba4fd3adcadae5f3f9c8d | BYTES 48
{
  "pid": 1026935,
  "ownedSession": 1026935
}
END FILE actual/CHILD.json

BEGIN FILE actual/DERIVATION.md | SHA256 6f21dd6e26d0c9a9b301408aaf29fe95f921025152a01f311cac8e5744a3c36d | BYTES 2511
# Pre-execution ordinary derivation

The author has source exposure; this is not a blind custodian oracle. No compiler/parser/evaluator was executed to obtain expected values.

Both programs have a Bool false initializer stored directly in StateDecl and no assets. Action choose has UInt limit then Bool enabled inputs. In Core/2 the Await declares Bool accepted; it remains a binder, not an action input or arena literal. The second action has its own UInt limit input and remains in the whole Program.

Literal UInt2=0; Name limit=1; Lt(0,1)=2. In the second statement, the left name is smaller in Core/1 and accepted in Core/2: node3. Name enabled=4; Literal false=5; Not(5)=6; And(4,6)=7; Or(3,7)=8. This explicitly distinguishes precedence `or < and < not`. Core/1 Requires.predicate and Core/2 Await.guard both use8; only Core/2 has label approve and binder accepted metadata.

Next: Name smaller=9, Name enabled=10, Not(10)=11, Or(9,11)=12. Ensures: Name result=13; the next name is enabled in Core/1 and accepted in Core/2 at14; Not(14)=15; Or(13,15)=16. Emit decision has fresh Name result=17. The second action continues the same arena: Name limit=18, Literal UInt0=19, Gt(18,19)=20. No nodes are interned or folded and no action root is rebased. The complete arrays and exact action metadata are frozen, rather than asserting only operator membership.

All values are statically well typed: literals2/0 and limit are UInt; comparisons Lt/Gt produce Bool; smaller, enabled, accepted and result are Bool, matching each Not/And/Or and the Requires/Ensures/Next target. There are no duties, assets, balances or financial calls. Await naming and later binder visibility use the existing admitted Core/2 path without selecting any new rule. No runtime truth/effect/work assertion is made.

Public checker output is independently assembled from the source identities, the manually enumerated complete action arrays and existing field/label/serializer contracts. Source hash is plain SHA256 of exact source bytes, computed using Python hashlib, not the product helper. Expected JSON bytes are ASCII pretty JSON with recursively sorted keys and LF, matching the current public report's Value representation; this is exposed output-contract corroboration. Human output is the fixed Checking/Finished Core branch, with right-aligned 12-column status verbs, `--color never`, normal verbosity and no timing line. These expected bytes are frozen before root execution and must not be updated from observed output.
END FILE actual/DERIVATION.md

BEGIN FILE actual/EXPECTED-CASE-SPEC.json | SHA256 6d95d40bb77759b0f70acfdb49b2c0da08d17b848df599d5ff4916cc8ce7ef2f | BYTES 2403
{
  "format": "ordinary-author-positive-boolean-source-check/1",
  "base": "4c03df51f78337d1a9e17cfaaabbbea112fef0df",
  "sourceExposure": "Author inspected current parser/DTO/check consumer and adopted M01 finite arena rule; manually enumerated every source occurrence before product execution. This is source-derived author corroboration, not blinded independent semantic or financial evidence.",
  "productOutputsRead": false,
  "cases": [
    {
      "id": "core1",
      "sourceSha256": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
      "programFile": "core1.program.expected.json",
      "checkStdoutFile": "core1.check.stdout.expected.json",
      "profile": "moriarty-core/1",
      "jsonArgv": [
        "--color",
        "never",
        "check",
        "boolean.mori",
        "--json"
      ],
      "humanArgv": [
        "--color",
        "never",
        "check",
        "boolean.mori"
      ],
      "expectedExit": 0,
      "expectedJsonStderr": "",
      "expectedHumanStdout": "",
      "expectedHumanStderr": "    Checking boolean.mori\n    Finished bounded core check\n"
    },
    {
      "id": "core2",
      "sourceSha256": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
      "programFile": "core2.program.expected.json",
      "checkStdoutFile": "core2.check.stdout.expected.json",
      "profile": "moriarty-core/2",
      "jsonArgv": [
        "--color",
        "never",
        "check",
        "boolean.mori",
        "--json"
      ],
      "humanArgv": [
        "--color",
        "never",
        "check",
        "boolean.mori"
      ],
      "expectedExit": 0,
      "expectedJsonStderr": "",
      "expectedHumanStdout": "",
      "expectedHumanStderr": "    Checking boolean.mori\n    Finished bounded core check\n"
    }
  ],
  "rootsBothPrograms": [
    2,
    8,
    12,
    16,
    17,
    20
  ],
  "expressionCountEach": 21,
  "claims": [
    "complete admitted Program value, including all expression nodes and both actions",
    "complete public check JSON stdout bytes and human stdout/stderr/exit"
  ],
  "excluded": [
    "evaluation, assertion truth for all inputs, await start/resume, financial/work observations",
    "stage plan byte/golden values",
    "compiler Artifact/Target/lowering, native SDK/proof/ledger results",
    "Ne spelling/admission decision",
    "all27 constructor closure"
  ]
}
END FILE actual/EXPECTED-CASE-SPEC.json

BEGIN FILE actual/EXPECTED-FREEZE.sha256 | SHA256 1287136fcb4e57450f28f55591fae8671466018ca8fdd1df93b1b706829b7598 | BYTES 1101
6f21dd6e26d0c9a9b301408aaf29fe95f921025152a01f311cac8e5744a3c36d  DERIVATION.md
6d95d40bb77759b0f70acfdb49b2c0da08d17b848df599d5ff4916cc8ce7ef2f  EXPECTED-CASE-SPEC.json
3ee7348e4e7822e2544cb2efc457da3d04c3aba99424fc35fb8862d6ca433c05  candidate/crates/fixtures/core-boolean-check/README.md
f4780f747afc62c7f118a5baa8b366499832d27e129fadd3c1b5c077aa97e1c0  candidate/crates/fixtures/core-boolean-check/core1.check.stdout.expected.json
9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64  candidate/crates/fixtures/core-boolean-check/core1.mori
00ae8b9039ee09557181d5c06740a813be429f8fe6bb35d7566d7ab2b8b4dfca  candidate/crates/fixtures/core-boolean-check/core1.program.expected.json
df5b3adcfacc6b00ef80561cc5a962f60136450797e588352964a91da6e2d403  candidate/crates/fixtures/core-boolean-check/core2.check.stdout.expected.json
3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969  candidate/crates/fixtures/core-boolean-check/core2.mori
e4b93ee899aeebdcbf68be708d5bbf66653b3fc4c15659cefe2fdf2df5485e0b  candidate/crates/fixtures/core-boolean-check/core2.program.expected.json
END FILE actual/EXPECTED-FREEZE.sha256

BEGIN FILE actual/FINAL-RESULT.json | SHA256 9ce3a5e5e36e8f5990e16a504ef8f395eac3fa7c2ec22f05cbf54d7caf28c272 | BYTES 768
{
  "finishedAt": "2026-10-18T18:13:45.889735+00:00",
  "result": {
    "startedAt": "2026-10-18T18:12:50.274674+00:00",
    "finishedAt": "2026-10-18T18:13:45.812955+00:00",
    "exitCode": 0,
    "wallSeconds": 55.5382868700035,
    "stdoutSha256": "01729163a7e1754398801667aa7f0971e6466e26af03e658bc8d4be4e6fedbd0",
    "stderrSha256": "e70e3daf2d0e95e4baece44b9f4b2fabbd25b7c477ea5c8665f14b2e9504c09b",
    "ownedSessionRemaining": []
  },
  "ownedRoot": "/tmp/doctorwatson-boolean-main-wocvhhvg",
  "existsAfter": false,
  "sourcePinsSha256": "e57986dba2ec596b4ad3fff8f5d7814aea527d8784dc2544f6bf16a85750a6dc",
  "scope": "Finite source checker coverage only; full grammar, independent financial semantics, compiler Artifact and target acceptance remain open."
}
END FILE actual/FINAL-RESULT.json

BEGIN FILE actual/POST-COMMIT-BINDING.json | SHA256 97311c1ac54bd5b89183919c61c9976bccaa8d100b4814e9ca974c76439f5946 | BYTES 1056
{
  "recordedAt": "2026-10-18T18:14:37.633392+00:00",
  "testedBaseHead": "829f4456dfda8d4fc5a4ece84c47f2539f846d9f",
  "afterCommit": "ce7d16557fc0605f929357628d907d2a66d6153c",
  "afterTree": "77bf20aa4556293f1399f10c6a5deefa0697c819",
  "all886AfterGitAndWorktreePinsMatch": true,
  "testPhase": "Base829f plus9workingtree paths executed once; postcommit binding only, no postcommit rerun.",
  "resultSha256": "cd364bec6031c76ecc20bb316c8decda7bb6e803dc6afc4aad833caf695c2218",
  "sourcePinsSha256": "e57986dba2ec596b4ad3fff8f5d7814aea527d8784dc2544f6bf16a85750a6dc",
  "driverSha256": "58a62dc61b9e546485c1fe047683f54b922181b7c3fd619902ee1cb4163d5f6e",
  "newMainGuidelineBeforeSha256": "7ac417984778c52387d49295364b1e2f85bde14d04d2130dc2c09b4f3bf9d404",
  "newMainGuidelineAfterSha256": "e448f368eab4ff3634ebfc65edcc3ee62d6cf8bf3e16bb76d81f2facec370af9",
  "exact8FixtureTestBytesUnchangedFrom1f3c": true,
  "pending33FinancialHelpDocsPathsNotImported": true,
  "old1f3cAdoptionNotTransferred": true,
  "freshWhole9SourceAndResultReview": "pending"
}
END FILE actual/POST-COMMIT-BINDING.json

BEGIN FILE actual/RESULT.json | SHA256 cd364bec6031c76ecc20bb316c8decda7bb6e803dc6afc4aad833caf695c2218 | BYTES 361
{
  "startedAt": "2026-10-18T18:12:50.274674+00:00",
  "finishedAt": "2026-10-18T18:13:45.812955+00:00",
  "exitCode": 0,
  "wallSeconds": 55.5382868700035,
  "stdoutSha256": "01729163a7e1754398801667aa7f0971e6466e26af03e658bc8d4be4e6fedbd0",
  "stderrSha256": "e70e3daf2d0e95e4baece44b9f4b2fabbd25b7c477ea5c8665f14b2e9504c09b",
  "ownedSessionRemaining": []
}
END FILE actual/RESULT.json

BEGIN FILE actual/ROOT-ACTUAL-CORRESPONDENCE.json | SHA256 76f0bad87181c44164d1c401cba185a65bdcc0ff670f13f4e55d9234b343dc37 | BYTES 1170
{
  "format": "root-boolean-main-actual-correspondence/1",
  "base": "829f4456dfda8d4fc5a4ece84c47f2539f846d9f",
  "selectedRustTestsPassed": 1,
  "publicProcesses": [
    {
      "case": "core1",
      "mode": "json",
      "exitCode": 0,
      "stdoutBytes": 1166,
      "stderrBytes": 0
    },
    {
      "case": "core1",
      "mode": "human",
      "exitCode": 0,
      "stdoutBytes": 0,
      "stderrBytes": 58
    },
    {
      "case": "core2",
      "mode": "json",
      "exitCode": 0,
      "stdoutBytes": 1422,
      "stderrBytes": 0
    },
    {
      "case": "core2",
      "mode": "human",
      "exitCode": 0,
      "stdoutBytes": 0,
      "stderrBytes": 58
    }
  ],
  "Programs": [
    {
      "case": "core1",
      "completeJsonEquality": true,
      "nodes": 21,
      "actions": 2,
      "OkJsonBytes": 3120
    },
    {
      "case": "core2",
      "completeJsonEquality": true,
      "nodes": 21,
      "actions": 2,
      "OkJsonBytes": 3290
    }
  ],
  "afterSourcePinsVerified": 886,
  "newBaseRun": true,
  "sourceExposedFrozenExpectationsUnchanged": true,
  "newSourceResultReviewRequired": true,
  "originalParentAcceptanceOpen": true
}
END FILE actual/ROOT-ACTUAL-CORRESPONDENCE.json

BEGIN FILE actual/STDERR.log | SHA256 e70e3daf2d0e95e4baece44b9f4b2fabbd25b7c477ea5c8665f14b2e9504c09b | BYTES 5034
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling syn v3.0.6
   Compiling cfg-if v1.0.5
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling typenum v1.20.1
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling zerovec-derive v0.11.6
   Compiling zerovec v0.11.8
   Compiling displaydoc v0.2.7
   Compiling zeroize v1.9.1
   Compiling writeable v0.6.4
   Compiling version_check v0.9.5
   Compiling libc v0.2.190
   Compiling generic-array v0.14.9
   Compiling tinystr v0.8.4
   Compiling rustix v1.1.5
   Compiling litemap v0.8.3
   Compiling icu_locale_fallback_data v2.3.0
   Compiling icu_locale_core v2.3.0
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling bitflags v2.13.2
   Compiling subtle v2.6.1
   Compiling linux-raw-sys v0.12.1
   Compiling const-oid v0.9.6
   Compiling icu_segmenter_data v2.3.0
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
   Compiling owo-colors v4.4.0
   Compiling supports-unicode v3.0.0
   Compiling unicode-width v0.1.14
   Compiling supports-hyperlinks v3.2.0
   Compiling miette v7.6.0
   Compiling rand_core v0.6.4
   Compiling digest v0.10.7
   Compiling syn v2.0.119
   Compiling soa-rs-derive v1.0.1
   Compiling mori-span v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-span)
   Compiling der v0.7.10
   Compiling crossbeam-utils v0.8.23
   Compiling memchr v2.8.3
   Compiling mori-diagnostics v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-diagnostics)
   Compiling soa-rs v1.0.1
   Compiling crossbeam-epoch v0.9.21
   Compiling mori-lexer v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-lexer)
   Compiling spki v0.7.3
   Compiling ff v0.13.1
   Compiling hybrid-array v0.4.15
   Compiling base16ct v0.2.0
   Compiling crossbeam-deque v0.8.8
   Compiling serde_core v1.0.229
   Compiling sec1 v0.7.3
   Compiling group v0.13.0
   Compiling pkcs8 v0.10.2
   Compiling hmac v0.12.1
   Compiling crypto-bigint v0.5.5
   Compiling once_cell v1.21.4
   Compiling zmij v1.0.23
   Compiling utf8parse v0.2.2
   Compiling anstyle-parse v1.0.0
   Compiling elliptic-curve v0.13.8
   Compiling rfc6979 v0.4.0
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling mori-ast v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-ast)
   Compiling aho-corasick v1.1.5
   Compiling signature v2.2.0
   Compiling rayon-core v1.13.0
   Compiling regex-syntax v0.8.11
   Compiling serde v1.0.229
   Compiling colorchoice v1.0.5
   Compiling anstyle v1.0.14
   Compiling const-oid v0.10.2
   Compiling cpufeatures v0.2.17
   Compiling getrandom v0.4.3
   Compiling thiserror v2.0.21
   Compiling serde_json v1.0.151
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle-query v1.1.5
   Compiling anstream v1.0.0
   Compiling sha2 v0.10.9
   Compiling digest v0.11.3
   Compiling regex-automata v0.4.18
   Compiling ecdsa v0.16.9
   Compiling bstr v1.13.1
   Compiling thiserror-impl v2.0.21
   Compiling serde_derive v1.0.229
   Compiling similar v2.7.0
   Compiling itoa v1.0.18
   Compiling strsim v0.11.1
   Compiling same-file v1.0.6
   Compiling log v0.4.34
   Compiling cpufeatures v0.3.1
   Compiling heck v0.5.0
   Compiling clap_lex v1.1.1
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling sha2 v0.11.0
   Compiling globset v0.4.20
   Compiling walkdir v2.5.0
   Compiling k256 v0.13.4
   Compiling mori-parser v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-parser)
   Compiling either v1.19.0
   Compiling fastrand v2.5.0
   Compiling tempfile v3.27.0
   Compiling rayon v1.12.0
   Compiling mori-fmt v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-fmt)
   Compiling mori-core v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core)
   Compiling ignore v0.4.33
   Compiling clap v4.6.7
   Compiling mori-checker v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-checker)
   Compiling console v0.16.6
   Compiling insta v1.49.0
   Compiling indoc v2.0.7
   Compiling mori v0.1.0 (/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 55.48s
     Running tests/core_boolean_check_public.rs (/tmp/doctorwatson-boolean-main-wocvhhvg/target/debug/deps/core_boolean_check_public-213e355f3b69144e)
END FILE actual/STDERR.log

BEGIN FILE actual/STDOUT.log | SHA256 01729163a7e1754398801667aa7f0971e6466e26af03e658bc8d4be4e6fedbd0 | BYTES 176

running 1 test
test documented_boolean_programs_reach_public_core_check ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

END FILE actual/STDOUT.log

BEGIN FILE actual/public-cli/core1-human/boolean.mori | SHA256 9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64 | BYTES 379
language "moriarty/1";
agreement BooleanChoice {
  state result: Bool = false;
  action choose(limit: UInt, enabled: Bool) {
    let smaller = 2 < limit;
    requires smaller or enabled and not false;
    next result = smaller or not enabled;
    ensures result or not enabled;
    emit "decision"(result);
  }
  action inspect_limit(limit: UInt) {
    requires limit > 0;
  }
}
END FILE actual/public-cli/core1-human/boolean.mori

BEGIN FILE actual/public-cli/core1-human/process.json | SHA256 2de503a90f94a01425d12752dd440fdf2ed18eada4e0c9ff3943ed1106bf59fb | BYTES 171
{
  "args": [
    "--color",
    "never",
    "check",
    "boolean.mori"
  ],
  "executable": "/tmp/doctorwatson-boolean-main-wocvhhvg/target/debug/mori",
  "status": 0
}
END FILE actual/public-cli/core1-human/process.json

BEGIN FILE actual/public-cli/core1-human/stderr.bin | SHA256 a410161251cd3b262ce175c3df51f80c1941d47a6cee56fc25729865afad959b | BYTES 58
    Checking boolean.mori
    Finished bounded core check
END FILE actual/public-cli/core1-human/stderr.bin

BEGIN FILE actual/public-cli/core1-human/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/public-cli/core1-human/stdout.bin

BEGIN FILE actual/public-cli/core1-json/boolean.mori | SHA256 9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64 | BYTES 379
language "moriarty/1";
agreement BooleanChoice {
  state result: Bool = false;
  action choose(limit: UInt, enabled: Bool) {
    let smaller = 2 < limit;
    requires smaller or enabled and not false;
    next result = smaller or not enabled;
    ensures result or not enabled;
    emit "decision"(result);
  }
  action inspect_limit(limit: UInt) {
    requires limit > 0;
  }
}
END FILE actual/public-cli/core1-json/boolean.mori

BEGIN FILE actual/public-cli/core1-json/process.json | SHA256 1697d9aef1ac074c8c0b1f4b90e867e8560d61940acd3a412584f1c30c4a0f96 | BYTES 185
{
  "args": [
    "--color",
    "never",
    "check",
    "boolean.mori",
    "--json"
  ],
  "executable": "/tmp/doctorwatson-boolean-main-wocvhhvg/target/debug/mori",
  "status": 0
}
END FILE actual/public-cli/core1-json/process.json

BEGIN FILE actual/public-cli/core1-json/stderr.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/public-cli/core1-json/stderr.bin

BEGIN FILE actual/public-cli/core1-json/stdout.bin | SHA256 f4780f747afc62c7f118a5baa8b366499832d27e129fadd3c1b5c077aa97e1c0 | BYTES 1166
{
  "actions": [
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        },
        {
          "name": "enabled",
          "ty": "Bool"
        }
      ],
      "name": "choose",
      "statements": [
        {
          "kind": "Let",
          "name": "smaller",
          "value": 2
        },
        {
          "kind": "Requires",
          "predicate": 8
        },
        {
          "kind": "Next",
          "state": "result",
          "value": 12
        },
        {
          "kind": "Ensures",
          "predicate": 16
        },
        {
          "kind": "Emit",
          "name": "decision",
          "value": 17
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        }
      ],
      "name": "inspect_limit",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 20
        }
      ]
    }
  ],
  "agreement": "BooleanChoice",
  "diagnostics": [],
  "file": "boolean.mori",
  "language": "moriarty/1",
  "source_hash": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
  "status": "accepted"
}
END FILE actual/public-cli/core1-json/stdout.bin

BEGIN FILE actual/public-cli/core1.program-result.actual.json | SHA256 850cfaaa0e643032107fc8526aa3dcf0c9099a44b91194c84e95b7b99a806b76 | BYTES 3120
{
  "Ok": {
    "language": "moriarty/1",
    "semantics": "moriarty-core/1",
    "name": "BooleanChoice",
    "states": [
      {
        "name": "result",
        "ty": "Bool",
        "initial": {
          "type": "Bool",
          "value": false
        }
      }
    ],
    "assets": [],
    "expressions": [
      {
        "kind": "Literal",
        "value": {
          "type": "UInt",
          "value": "2"
        }
      },
      {
        "kind": "Name",
        "name": "limit"
      },
      {
        "kind": "Binary",
        "op": "Lt",
        "lhs": 0,
        "rhs": 1
      },
      {
        "kind": "Name",
        "name": "smaller"
      },
      {
        "kind": "Name",
        "name": "enabled"
      },
      {
        "kind": "Literal",
        "value": {
          "type": "Bool",
          "value": false
        }
      },
      {
        "kind": "Not",
        "value": 5
      },
      {
        "kind": "Binary",
        "op": "And",
        "lhs": 4,
        "rhs": 6
      },
      {
        "kind": "Binary",
        "op": "Or",
        "lhs": 3,
        "rhs": 7
      },
      {
        "kind": "Name",
        "name": "smaller"
      },
      {
        "kind": "Name",
        "name": "enabled"
      },
      {
        "kind": "Not",
        "value": 10
      },
      {
        "kind": "Binary",
        "op": "Or",
        "lhs": 9,
        "rhs": 11
      },
      {
        "kind": "Name",
        "name": "result"
      },
      {
        "kind": "Name",
        "name": "enabled"
      },
      {
        "kind": "Not",
        "value": 14
      },
      {
        "kind": "Binary",
        "op": "Or",
        "lhs": 13,
        "rhs": 15
      },
      {
        "kind": "Name",
        "name": "result"
      },
      {
        "kind": "Name",
        "name": "limit"
      },
      {
        "kind": "Literal",
        "value": {
          "type": "UInt",
          "value": "0"
        }
      },
      {
        "kind": "Binary",
        "op": "Gt",
        "lhs": 18,
        "rhs": 19
      }
    ],
    "actions": [
      {
        "name": "choose",
        "inputs": [
          {
            "name": "limit",
            "ty": "UInt"
          },
          {
            "name": "enabled",
            "ty": "Bool"
          }
        ],
        "statements": [
          {
            "kind": "Let",
            "name": "smaller",
            "value": 2
          },
          {
            "kind": "Requires",
            "predicate": 8
          },
          {
            "kind": "Next",
            "state": "result",
            "value": 12
          },
          {
            "kind": "Ensures",
            "predicate": 16
          },
          {
            "kind": "Emit",
            "name": "decision",
            "value": 17
          }
        ]
      },
      {
        "name": "inspect_limit",
        "inputs": [
          {
            "name": "limit",
            "ty": "UInt"
          }
        ],
        "statements": [
          {
            "kind": "Requires",
            "predicate": 20
          }
        ]
      }
    ]
  }
}
END FILE actual/public-cli/core1.program-result.actual.json

BEGIN FILE actual/public-cli/core2-human/boolean.mori | SHA256 3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969 | BYTES 413
language "moriarty/2";
agreement StagedBooleanChoice {
  state result: Bool = false;
  action choose(limit: UInt, enabled: Bool) {
    let smaller = 2 < limit;
    await approve(accepted: Bool) when accepted or enabled and not false;
    next result = smaller or not enabled;
    ensures result or not accepted;
    emit "decision"(result);
  }
  action inspect_limit(limit: UInt) {
    requires limit > 0;
  }
}
END FILE actual/public-cli/core2-human/boolean.mori

BEGIN FILE actual/public-cli/core2-human/process.json | SHA256 2de503a90f94a01425d12752dd440fdf2ed18eada4e0c9ff3943ed1106bf59fb | BYTES 171
{
  "args": [
    "--color",
    "never",
    "check",
    "boolean.mori"
  ],
  "executable": "/tmp/doctorwatson-boolean-main-wocvhhvg/target/debug/mori",
  "status": 0
}
END FILE actual/public-cli/core2-human/process.json

BEGIN FILE actual/public-cli/core2-human/stderr.bin | SHA256 a410161251cd3b262ce175c3df51f80c1941d47a6cee56fc25729865afad959b | BYTES 58
    Checking boolean.mori
    Finished bounded core check
END FILE actual/public-cli/core2-human/stderr.bin

BEGIN FILE actual/public-cli/core2-human/stdout.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/public-cli/core2-human/stdout.bin

BEGIN FILE actual/public-cli/core2-json/boolean.mori | SHA256 3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969 | BYTES 413
language "moriarty/2";
agreement StagedBooleanChoice {
  state result: Bool = false;
  action choose(limit: UInt, enabled: Bool) {
    let smaller = 2 < limit;
    await approve(accepted: Bool) when accepted or enabled and not false;
    next result = smaller or not enabled;
    ensures result or not accepted;
    emit "decision"(result);
  }
  action inspect_limit(limit: UInt) {
    requires limit > 0;
  }
}
END FILE actual/public-cli/core2-json/boolean.mori

BEGIN FILE actual/public-cli/core2-json/process.json | SHA256 1697d9aef1ac074c8c0b1f4b90e867e8560d61940acd3a412584f1c30c4a0f96 | BYTES 185
{
  "args": [
    "--color",
    "never",
    "check",
    "boolean.mori",
    "--json"
  ],
  "executable": "/tmp/doctorwatson-boolean-main-wocvhhvg/target/debug/mori",
  "status": 0
}
END FILE actual/public-cli/core2-json/process.json

BEGIN FILE actual/public-cli/core2-json/stderr.bin | SHA256 e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 | BYTES 0

END FILE actual/public-cli/core2-json/stderr.bin

BEGIN FILE actual/public-cli/core2-json/stdout.bin | SHA256 df5b3adcfacc6b00ef80561cc5a962f60136450797e588352964a91da6e2d403 | BYTES 1422
{
  "actions": [
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        },
        {
          "name": "enabled",
          "ty": "Bool"
        }
      ],
      "name": "choose",
      "statements": [
        {
          "kind": "Let",
          "name": "smaller",
          "value": 2
        },
        {
          "binders": [
            {
              "name": "accepted",
              "ty": "Bool"
            }
          ],
          "guard": 8,
          "kind": "Await",
          "label": "approve"
        },
        {
          "kind": "Next",
          "state": "result",
          "value": 12
        },
        {
          "kind": "Ensures",
          "predicate": 16
        },
        {
          "kind": "Emit",
          "name": "decision",
          "value": 17
        }
      ]
    },
    {
      "inputs": [
        {
          "name": "limit",
          "ty": "UInt"
        }
      ],
      "name": "inspect_limit",
      "statements": [
        {
          "kind": "Requires",
          "predicate": 20
        }
      ]
    }
  ],
  "agreement": "StagedBooleanChoice",
  "authentication": "none",
  "consumption": "untracked",
  "diagnostics": [],
  "file": "boolean.mori",
  "language": "moriarty/2",
  "qualification": "local-reference-stage",
  "source_hash": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
  "status": "accepted"
}
END FILE actual/public-cli/core2-json/stdout.bin

BEGIN FILE actual/public-cli/core2.program-result.actual.json | SHA256 66c42549ec8d51d427505cd0809a25a8a427f8bc8840d3efbd7b3a43bb44f32a | BYTES 3290
{
  "Ok": {
    "language": "moriarty/2",
    "semantics": "moriarty-core/2",
    "name": "StagedBooleanChoice",
    "states": [
      {
        "name": "result",
        "ty": "Bool",
        "initial": {
          "type": "Bool",
          "value": false
        }
      }
    ],
    "assets": [],
    "expressions": [
      {
        "kind": "Literal",
        "value": {
          "type": "UInt",
          "value": "2"
        }
      },
      {
        "kind": "Name",
        "name": "limit"
      },
      {
        "kind": "Binary",
        "op": "Lt",
        "lhs": 0,
        "rhs": 1
      },
      {
        "kind": "Name",
        "name": "accepted"
      },
      {
        "kind": "Name",
        "name": "enabled"
      },
      {
        "kind": "Literal",
        "value": {
          "type": "Bool",
          "value": false
        }
      },
      {
        "kind": "Not",
        "value": 5
      },
      {
        "kind": "Binary",
        "op": "And",
        "lhs": 4,
        "rhs": 6
      },
      {
        "kind": "Binary",
        "op": "Or",
        "lhs": 3,
        "rhs": 7
      },
      {
        "kind": "Name",
        "name": "smaller"
      },
      {
        "kind": "Name",
        "name": "enabled"
      },
      {
        "kind": "Not",
        "value": 10
      },
      {
        "kind": "Binary",
        "op": "Or",
        "lhs": 9,
        "rhs": 11
      },
      {
        "kind": "Name",
        "name": "result"
      },
      {
        "kind": "Name",
        "name": "accepted"
      },
      {
        "kind": "Not",
        "value": 14
      },
      {
        "kind": "Binary",
        "op": "Or",
        "lhs": 13,
        "rhs": 15
      },
      {
        "kind": "Name",
        "name": "result"
      },
      {
        "kind": "Name",
        "name": "limit"
      },
      {
        "kind": "Literal",
        "value": {
          "type": "UInt",
          "value": "0"
        }
      },
      {
        "kind": "Binary",
        "op": "Gt",
        "lhs": 18,
        "rhs": 19
      }
    ],
    "actions": [
      {
        "name": "choose",
        "inputs": [
          {
            "name": "limit",
            "ty": "UInt"
          },
          {
            "name": "enabled",
            "ty": "Bool"
          }
        ],
        "statements": [
          {
            "kind": "Let",
            "name": "smaller",
            "value": 2
          },
          {
            "kind": "Await",
            "label": "approve",
            "binders": [
              {
                "name": "accepted",
                "ty": "Bool"
              }
            ],
            "guard": 8
          },
          {
            "kind": "Next",
            "state": "result",
            "value": 12
          },
          {
            "kind": "Ensures",
            "predicate": 16
          },
          {
            "kind": "Emit",
            "name": "decision",
            "value": 17
          }
        ]
      },
      {
        "name": "inspect_limit",
        "inputs": [
          {
            "name": "limit",
            "ty": "UInt"
          }
        ],
        "statements": [
          {
            "kind": "Requires",
            "predicate": 20
          }
        ]
      }
    ]
  }
}
END FILE actual/public-cli/core2.program-result.actual.json

BEGIN FILE actual/COMPLETE-ROOT-DRIVER.py | SHA256 58a62dc61b9e546485c1fe047683f54b922181b7c3fd619902ee1cb4163d5f6e | BYTES 7324
"""One finite source-only public checker observation against frozen literals."""
from pathlib import Path
import datetime
import hashlib
import json
import os
import shutil
import signal
import subprocess
import tempfile
import time

R = Path('/home/charl/moriarty-c2-opus55-restored-20261017')
W = Path('/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018')
P = R / 'concurrency-increase-main238/grammar-next4c03'
I = R / 'concurrency-increase-main238/boolean-main829f-integration-preparation'
C = R / 'boolean-main-integration-check-20261018'
TC = Path('/home/charl/.rustup/toolchains/1.99.0-x86_64-unknown-linux-gnu/bin')
HEAD = '829f4456dfda8d4fc5a4ece84c47f2539f846d9f'
TREE = '76dac56aac5e653729f3e4ad546b74ba07f1f780'


def sha(p):
    return hashlib.sha256(p.read_bytes()).hexdigest()


def now():
    return datetime.datetime.now(datetime.timezone.utc).isoformat()


def write(name, value):
    p = C / name
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(value, indent=2) + '\n')


def git(*args):
    return subprocess.check_output(['git', *args], cwd=W).decode().strip()


assert not C.exists()
assert git('rev-parse', 'HEAD') == HEAD
assert git('rev-parse', 'HEAD^{tree}') == TREE
assert sha(P / 'EXPECTED-FREEZE.sha256') == '1287136fcb4e57450f28f55591fae8671466018ca8fdd1df93b1b706829b7598'
for line in (P / 'EXPECTED-FREEZE.sha256').read_text().splitlines():
    digest, relative = line.split('  ', 1)
    assert not Path(relative).is_absolute() and '..' not in Path(relative).parts
    assert sha(P / relative) == digest, relative
for line in (I / 'FREEZE.sha256').read_text().splitlines():
    digest, relative = line.split('  ', 1)
    assert not Path(relative).is_absolute() and '..' not in Path(relative).parts
    assert sha(I / relative) == digest, relative
added = [{'path': row['path'], 'sha256': row['afterSha256']} for row in json.loads((I / 'CANDIDATE.json').read_text())['changedPaths']]
assert len(added) == 9
changed_map = {row["path"]: row for row in added}
for row in added:
    assert sha(W / row['path']) == row['sha256'] == sha(I / 'after' / row['path'])
assert git('diff', '--name-only') == 'RUST_GUIDELINES.md'
untracked = set(git('ls-files', '--others', '--exclude-standard').splitlines())
assert untracked == {row['path'] for row in added if row['path'] != 'RUST_GUIDELINES.md'}
names = subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', HEAD,
    'crates', 'examples/core/continuations', 'Cargo.toml', 'Cargo.lock',
    'rust-toolchain.toml', 'RUST_GUIDELINES.md', 'docs/NATIVE-CORE.md'], cwd=W).decode().splitlines()
pins = []
for name in names:
    raw = subprocess.check_output(['git', 'show', HEAD + ':' + name], cwd=W)
    if name in changed_map:
        assert name == 'RUST_GUIDELINES.md'
        assert sha(W / name) == changed_map[name]['sha256']
        pins.append({'path': name, 'sha256': sha(W / name), 'beforeGitSha256': hashlib.sha256(raw).hexdigest()})
    else:
        assert (W / name).read_bytes() == raw, name
        pins.append({'path': name, 'sha256': hashlib.sha256(raw).hexdigest()})
pins.extend({'path': row['path'], 'sha256': row['sha256']} for row in added if row['path'] not in names)
C.mkdir()
write('SOURCE-PINS.json', {'baseHead': HEAD, 'baseTree': TREE, 'files': pins})
for name in ['EXPECTED-FREEZE.sha256', 'EXPECTED-CASE-SPEC.json', 'DERIVATION.md']:
    (C / name).write_bytes((P / name).read_bytes())
root = Path(tempfile.mkdtemp(prefix='doctorwatson-boolean-main-'))
for name in ['cwd', 'tmp', 'cargo-home']:
    (root / name).mkdir()
for name in ['registry', 'git']:
    (root / 'cargo-home' / name).symlink_to(Path('/home/charl/.cargo') / name, target_is_directory=True)
env = {'PATH': str(TC) + ':/usr/bin:/bin', 'RUSTC': str(TC / 'rustc'),
    'RUSTDOC': str(TC / 'rustdoc'), 'CARGO_HOME': str(root / 'cargo-home'),
    'CARGO_TARGET_DIR': str(root / 'target'), 'CARGO_NET_OFFLINE': 'true',
    'CARGO_INCREMENTAL': '0', 'CARGO_CACHE_AUTO_CLEAN_FREQUENCY': 'never',
    'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
    'GIT_CONFIG_SYSTEM': '/dev/null', 'LANG': 'C.UTF-8', 'TMPDIR': str(root / 'tmp'),
    'MORI_BOOLEAN_CHECK_OBSERVATIONS': str(C / 'public-cli')}
configs = set()
for directory in [W, *W.parents, root / 'cwd', *(root / 'cwd').parents]:
    for name in ['config', 'config.toml']:
        p = directory / '.cargo' / name
        assert not p.exists(), str(p)
        configs.add(str(p))
argv = ['/usr/bin/timeout', '--signal=TERM', '--kill-after=5s', '90s',
    str(TC / 'cargo'), 'test', '--manifest-path', str(W / 'Cargo.toml'),
    '--frozen', '-j1', '-p', 'mori', '--no-default-features', '--test',
    'core_boolean_check_public', 'documented_boolean_programs_reach_public_core_check',
    '--', '--exact', '--test-threads=1']
write('CHECK-PLAN.json', {'recordedAt': now(), 'baseHead': HEAD, 'baseTree': TREE,
    'expectationFreezeSha256': sha(C / 'EXPECTED-FREEZE.sha256'),
    'sourcePinsSha256': sha(C / 'SOURCE-PINS.json'), 'argv': argv, 'environment': env,
    'ownedRoot': str(root), 'absentAncestorCargoConfigs': sorted(configs),
    'toolPins': [{'path': str(p), 'sha256': sha(p)} for p in
        [TC / 'cargo', TC / 'rustc', TC / 'rustdoc', Path('/usr/bin/timeout')]],
    'scope': 'Two positive Core1/Core2 Boolean programs, complete API Program values and four public checker streams on independent header-main base. Source-exposed hand expectations; no evaluator/financial/native/SDK/proof/wallet/ledger run; no RED defect claimed.'})
result = None
child = None
try:
    started = now()
    begin = time.monotonic()
    with (C / 'STDOUT.log').open('xb') as stdout, (C / 'STDERR.log').open('xb') as stderr:
        child = subprocess.Popen(argv, cwd=root / 'cwd', env=env, stdin=subprocess.DEVNULL,
            stdout=stdout, stderr=stderr, start_new_session=True)
        write('CHILD.json', {'pid': child.pid, 'ownedSession': child.pid})
        code = child.wait()
    remaining = [int(pid) for pid, sid in (line.split() for line in
        subprocess.check_output(['ps', '-eo', 'pid=,sid=']).decode().splitlines()) if int(sid) == child.pid]
    result = {'startedAt': started, 'finishedAt': now(), 'exitCode': code,
        'wallSeconds': time.monotonic() - begin, 'stdoutSha256': sha(C / 'STDOUT.log'),
        'stderrSha256': sha(C / 'STDERR.log'), 'ownedSessionRemaining': remaining}
    write('RESULT.json', result)
    print(json.dumps(result), flush=True)
    assert not remaining
    assert code == 0
    assert '1 passed; 0 failed' in (C / 'STDOUT.log').read_text()
    assert all(sha(W / row['path']) == row['sha256'] for row in pins)
    assert git('rev-parse', 'HEAD') == HEAD
    assert git('diff', '--name-only') == 'RUST_GUIDELINES.md'
finally:
    if child is not None:
        try:
            os.killpg(child.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        if child.poll() is None:
            child.wait(timeout=1)
    shutil.rmtree(root)
    write('FINAL-RESULT.json', {'finishedAt': now(), 'result': result,
        'ownedRoot': str(root), 'existsAfter': root.exists(),
        'sourcePinsSha256': sha(C / 'SOURCE-PINS.json'),
        'scope': 'Finite source checker coverage only; full grammar, independent financial semantics, compiler Artifact and target acceptance remain open.'})
END FILE actual/COMPLETE-ROOT-DRIVER.py

BEGIN FILE SOURCE-RESULT-CUSTODY.json | SHA256 8927532b9b6a08e1bd5739233d359639b5bbe3c4d3d481f6ced6a51d805866a2 | BYTES 6900
{
  "reviewToken": "mori-boolean-main-ce7d1655-7db2e695a76b08a29355",
  "candidate": "ce7d16557fc0605f929357628d907d2a66d6153c",
  "tree": "77bf20aa4556293f1399f10c6a5deefa0697c819",
  "parent": "829f4456dfda8d4fc5a4ece84c47f2539f846d9f",
  "all886RecordedCurrentWorktreePinsMatch": true,
  "sourcePinsSha256": "e57986dba2ec596b4ad3fff8f5d7814aea527d8784dc2544f6bf16a85750a6dc",
  "rootGitAttestation": "Original complete actual/POST-COMMIT-BINDING.json; packet builder performs file hashing only, no Git",
  "exactNineCandidateBodiesEqualPreparation": true,
  "exactEightFixturesEqualOriginal27bAnd1f3c": true,
  "guidelineBeforeSha256": "7ac417984778c52387d49295364b1e2f85bde14d04d2130dc2c09b4f3bf9d404",
  "guidelineAfterSha256": "e448f368eab4ff3634ebfc65edcc3ee62d6cf8bf3e16bb76d81f2facec370af9",
  "freshAggregate": {
    "startedAt": "2026-10-18T18:12:50.274674+00:00",
    "finishedAt": "2026-10-18T18:13:45.812955+00:00",
    "exitCode": 0,
    "wallSeconds": 55.5382868700035,
    "stdoutSha256": "01729163a7e1754398801667aa7f0971e6466e26af03e658bc8d4be4e6fedbd0",
    "stderrSha256": "e70e3daf2d0e95e4baece44b9f4b2fabbd25b7c477ea5c8665f14b2e9504c09b",
    "ownedSessionRemaining": []
  },
  "processes": [
    {
      "case": "core1",
      "mode": "json",
      "exitCode": 0,
      "args": [
        "--color",
        "never",
        "check",
        "boolean.mori",
        "--json"
      ],
      "stdoutBytes": 1166,
      "stderrBytes": 0,
      "stdoutSha256": "f4780f747afc62c7f118a5baa8b366499832d27e129fadd3c1b5c077aa97e1c0",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "sourceSha256": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
      "completeEqualityToOriginalFrozen": true
    },
    {
      "case": "core1",
      "mode": "human",
      "exitCode": 0,
      "args": [
        "--color",
        "never",
        "check",
        "boolean.mori"
      ],
      "stdoutBytes": 0,
      "stderrBytes": 58,
      "stdoutSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "stderrSha256": "a410161251cd3b262ce175c3df51f80c1941d47a6cee56fc25729865afad959b",
      "sourceSha256": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
      "completeEqualityToOriginalFrozen": true
    },
    {
      "case": "core2",
      "mode": "json",
      "exitCode": 0,
      "args": [
        "--color",
        "never",
        "check",
        "boolean.mori",
        "--json"
      ],
      "stdoutBytes": 1422,
      "stderrBytes": 0,
      "stdoutSha256": "df5b3adcfacc6b00ef80561cc5a962f60136450797e588352964a91da6e2d403",
      "stderrSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "sourceSha256": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
      "completeEqualityToOriginalFrozen": true
    },
    {
      "case": "core2",
      "mode": "human",
      "exitCode": 0,
      "args": [
        "--color",
        "never",
        "check",
        "boolean.mori"
      ],
      "stdoutBytes": 0,
      "stderrBytes": 58,
      "stdoutSha256": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
      "stderrSha256": "a410161251cd3b262ce175c3df51f80c1941d47a6cee56fc25729865afad959b",
      "sourceSha256": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
      "completeEqualityToOriginalFrozen": true
    }
  ],
  "completePrograms": [
    {
      "case": "core1",
      "nodes": 21,
      "actions": 2,
      "roots": [
        2,
        8,
        12,
        16,
        17,
        20
      ],
      "allFieldsEqualUnchangedFrozen": true,
      "actualBytes": 3120,
      "actualSha256": "850cfaaa0e643032107fc8526aa3dcf0c9099a44b91194c84e95b7b99a806b76",
      "expectedSha256": "00ae8b9039ee09557181d5c06740a813be429f8fe6bb35d7566d7ab2b8b4dfca"
    },
    {
      "case": "core2",
      "nodes": 21,
      "actions": 2,
      "roots": [
        2,
        8,
        12,
        16,
        17,
        20
      ],
      "allFieldsEqualUnchangedFrozen": true,
      "actualBytes": 3290,
      "actualSha256": "66c42549ec8d51d427505cd0809a25a8a427f8bc8840d3efbd7b3a43bb44f32a",
      "expectedSha256": "e4b93ee899aeebdcbf68be708d5bbf66653b3fc4c15659cefe2fdf2df5485e0b"
    }
  ],
  "originalFreezeSha256": "1287136fcb4e57450f28f55591fae8671466018ca8fdd1df93b1b706829b7598",
  "driverSha256": "58a62dc61b9e546485c1fe047683f54b922181b7c3fd619902ee1cb4163d5f6e",
  "executionVsCommit": "New actual run executed829f plus exactly nine working-tree paths, then exact same bytes bound toce7d/tree77bf. Postcommit metadata is not another execution.",
  "contextDifferenceDisposition": "Full selected parser/check/stage/static validation/IR/serializer/lexer and Cargo/toolchain contexts are body-equal to old1f3c. Complete cli.rs help/test metadata and expand.rs help text differences are supplied with both full bodies/diffs. Complete documentation difference hunks and all changed execution-pin rows are mandatory; full unused before/after docs physically preserved. This is inspected context comparison, not broad equivalence or old approval transfer.",
  "historicalPinQualification": "Old911 manifest binds original27b-equivalent runtime bytes, old1f3c docs differ, new886 pins bind this main run. Do not assert old911 files all match current main, or old guideline hash matches new7ac/e448.",
  "oldOpinions": "620e1 history and its corrected49-vs50 coordinate/guideline qualification remain unchanged. Qualified1f3c opinion196e6 and rootd2bc are not newhead approval; no old opinion/adoption is transferred.",
  "sourceExposure": "Original author hand-enumerated expectations after parser/DTO/consumer/M01 inspection before product execution. New run uses same literals, not output-derived goldens. Root/builder host comparisons corroborate complete bytes/values only, not independent financial/work truth.",
  "physicalVsModelRead": "Full886 current pins, old911 execution matrix and original packets/unused fixture families preserved in physical custody. This exact summary plus original binding is mandatory instead of reading unrelated hash rows. Every used candidate/expected/actual semantic body, every process field/stream and all original94 mandatory meanings remain required. No full-manifest model-read or full-repository audit claim.",
  "noOwnerAdmissionOrAcceptance": true,
  "staticOnly": "No evaluator/action invocation, Await advancement, truth for arbitrary inputs, incoming duties, financial outcome/work observation, security judgment, compiler Artifact/Target/native/SDK/proof/ledger qualification. Static stage::load internally plans/costs/digests; these values are not observed work goldens.",
  "allParentExitsOpen": "Original27/148/292/20 and U0-U7/M1-M17/MC/SP/G, financial/composition/compiler/target/proof/ledger exits remain open."
}
END FILE SOURCE-RESULT-CUSTODY.json

BEGIN FILE CURRENT-SOURCE-UNIT-MAP.json | SHA256 a4cb7e75bd7412ca18895fa46ea2a80d1a489b1f6f046f76bf7a337e7966d48d | BYTES 28704
{
  "scope": "All56CURRENT complete source/requirement units, retaining exact original provenance/coordinates. The old58map additionally counted two obsolete before-CLI/expand versions, now optional history only.",
  "units": [
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/README.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/README.md",
      "originSha256": "3ee7348e4e7822e2544cb2efc457da3d04c3aba99424fc35fb8862d6ca433c05",
      "startLine": 1,
      "endLine": 11,
      "unitSha256": "3ee7348e4e7822e2544cb2efc457da3d04c3aba99424fc35fb8862d6ca433c05",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/core1.check.stdout.expected.json",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/core1.check.stdout.expected.json",
      "originSha256": "f4780f747afc62c7f118a5baa8b366499832d27e129fadd3c1b5c077aa97e1c0",
      "startLine": 1,
      "endLine": 63,
      "unitSha256": "f4780f747afc62c7f118a5baa8b366499832d27e129fadd3c1b5c077aa97e1c0",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/core1.mori",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/core1.mori",
      "originSha256": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
      "startLine": 1,
      "endLine": 14,
      "unitSha256": "9a26f38e31342ff477f6d860ac899039a80ec80fc7c517c1ea1e7cdf864def64",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/core1.program.expected.json",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/core1.program.expected.json",
      "originSha256": "00ae8b9039ee09557181d5c06740a813be429f8fe6bb35d7566d7ab2b8b4dfca",
      "startLine": 1,
      "endLine": 178,
      "unitSha256": "00ae8b9039ee09557181d5c06740a813be429f8fe6bb35d7566d7ab2b8b4dfca",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/core2.check.stdout.expected.json",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/core2.check.stdout.expected.json",
      "originSha256": "df5b3adcfacc6b00ef80561cc5a962f60136450797e588352964a91da6e2d403",
      "startLine": 1,
      "endLine": 73,
      "unitSha256": "df5b3adcfacc6b00ef80561cc5a962f60136450797e588352964a91da6e2d403",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/core2.mori",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/core2.mori",
      "originSha256": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
      "startLine": 1,
      "endLine": 14,
      "unitSha256": "3522b4cf5960889943759dc604dbdf02001310b84b6f57cd161c244e2ed3c969",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/fixtures/core-boolean-check/core2.program.expected.json",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/fixtures/core-boolean-check/core2.program.expected.json",
      "originSha256": "e4b93ee899aeebdcbf68be708d5bbf66653b3fc4c15659cefe2fdf2df5485e0b",
      "startLine": 1,
      "endLine": 185,
      "unitSha256": "e4b93ee899aeebdcbf68be708d5bbf66653b3fc4c15659cefe2fdf2df5485e0b",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/tests/core_boolean_check_public.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/tests/core_boolean_check_public.rs",
      "originSha256": "167f754b5f7375b8f2db9b28b040f0c9f38d43ede47489a9a038b4c59f3485b2",
      "startLine": 1,
      "endLine": 156,
      "unitSha256": "167f754b5f7375b8f2db9b28b040f0c9f38d43ede47489a9a038b4c59f3485b2",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/main.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/main.rs",
      "originSha256": "35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232",
      "startLine": 1,
      "endLine": 27,
      "unitSha256": "35988592bec7038a7674403a0e897184d1099dd4164e89a3ca0aa22ab0b3a232",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/commands/mod.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/mod.rs",
      "originSha256": "4e39c2dd984c1dcd0687da30eddd4e630f3d9480aef0b5df25e48828001abc47",
      "startLine": 1,
      "endLine": 81,
      "unitSha256": "4e39c2dd984c1dcd0687da30eddd4e630f3d9480aef0b5df25e48828001abc47",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/commands/check.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/check.rs",
      "originSha256": "2f08c04bdc1a07796c459385ec9271466a26572f2455ed7d63de906c1d5876bb",
      "startLine": 1,
      "endLine": 143,
      "unitSha256": "2f08c04bdc1a07796c459385ec9271466a26572f2455ed7d63de906c1d5876bb",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/commands/check_source.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/check_source.rs",
      "originSha256": "a53d5c3689f3c32ddc8c595e577a0ac13cfc4fe187b42a4f83c9caf86dc9ace3",
      "startLine": 1,
      "endLine": 90,
      "unitSha256": "a53d5c3689f3c32ddc8c595e577a0ac13cfc4fe187b42a4f83c9caf86dc9ace3",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/shell.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/shell.rs",
      "originSha256": "b2116fcd9bb80abfb175f6dc2da6327b4d9787cfa3d56481fde705e62d94fff1",
      "startLine": 1,
      "endLine": 108,
      "unitSha256": "b2116fcd9bb80abfb175f6dc2da6327b4d9787cfa3d56481fde705e62d94fff1",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/display.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/display.rs",
      "originSha256": "81d499d32561dc576651e0b1df5a349b68112030f296d93cca95a488dc3b0deb",
      "startLine": 1,
      "endLine": 12,
      "unitSha256": "81d499d32561dc576651e0b1df5a349b68112030f296d93cca95a488dc3b0deb",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/parser.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/parser.rs",
      "originSha256": "66c839add400c62e70371a2cf1c412d391f3e0b626b860dc51a3e01039d46cc7",
      "startLine": 1,
      "endLine": 417,
      "unitSha256": "66c839add400c62e70371a2cf1c412d391f3e0b626b860dc51a3e01039d46cc7",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/ir.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/ir.rs",
      "originSha256": "0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d",
      "startLine": 1,
      "endLine": 483,
      "unitSha256": "0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/json.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/json.rs",
      "originSha256": "faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183",
      "startLine": 1,
      "endLine": 150,
      "unitSha256": "faef7db8115320cc483dce30d7b5d22bd01a780facae1c761a8c9203fbb66183",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/lib.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/lib.rs",
      "originSha256": "b33afe9a9f7a32d0c4b10211fa139bca5d1a8741ba61ee4df8acebbcd5f8304a",
      "startLine": 1,
      "endLine": 41,
      "unitSha256": "b33afe9a9f7a32d0c4b10211fa139bca5d1a8741ba61ee4df8acebbcd5f8304a",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-lexer/src/lib.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-lexer/src/lib.rs",
      "originSha256": "5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7",
      "startLine": 1,
      "endLine": 244,
      "unitSha256": "5cd5bd604e41fe57e754dd323a4fd86393d34c231a4dee16efe5fd13da5e28d7",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-lexer/src/scan.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-lexer/src/scan.rs",
      "originSha256": "4119a98f6ade2d96cb1231b0531cdbb63d01663d24c7e2615ef5b74931db5637",
      "startLine": 1,
      "endLine": 55,
      "unitSha256": "4119a98f6ade2d96cb1231b0531cdbb63d01663d24c7e2615ef5b74931db5637",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-lexer/src/token.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-lexer/src/token.rs",
      "originSha256": "e7860a893c0827bbf362165438c67d621f229f0eacecb9175d959788687bc3db",
      "startLine": 1,
      "endLine": 205,
      "unitSha256": "e7860a893c0827bbf362165438c67d621f229f0eacecb9175d959788687bc3db",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-lexer/src/string.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-lexer/src/string.rs",
      "originSha256": "b94878f41914f56dfd4945ebb0b44690db541e77848817bc6e265dc422f88aaa",
      "startLine": 1,
      "endLine": 131,
      "unitSha256": "b94878f41914f56dfd4945ebb0b44690db541e77848817bc6e265dc422f88aaa",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-lexer/src/number.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-lexer/src/number.rs",
      "originSha256": "5826e09117b22505a74ab3144f4862445b513039bac275c88ace4d85331fad9a",
      "startLine": 1,
      "endLine": 64,
      "unitSha256": "5826e09117b22505a74ab3144f4862445b513039bac275c88ace4d85331fad9a",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/Cargo.toml",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/Cargo.toml",
      "originSha256": "2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b",
      "startLine": 1,
      "endLine": 113,
      "unitSha256": "2202f89a5c40b7bbe503ebb66a406226c2e446ab46580e1c2ff5d4f870ecea1b",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/Cargo.toml",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/Cargo.toml",
      "originSha256": "93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c",
      "startLine": 1,
      "endLine": 28,
      "unitSha256": "93318da98ab974c2d30210bd868f98fa5c4af60390395dcbe4f65669e815ac7c",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/stage/plan.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/stage/plan.rs",
      "originSha256": "9d8e71b2d5089bf3221b3e06454038357559a9eddab8245464d55b19653d1e1c",
      "startLine": 1,
      "endLine": 125,
      "unitSha256": "9d8e71b2d5089bf3221b3e06454038357559a9eddab8245464d55b19653d1e1c",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/stage/profile.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/stage/profile.rs",
      "originSha256": "933560466065d78b0ce3f3559c2cf2bb9cbdc8eea45903af564cf1ab469cd59d",
      "startLine": 1,
      "endLine": 31,
      "unitSha256": "933560466065d78b0ce3f3559c2cf2bb9cbdc8eea45903af564cf1ab469cd59d",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/stage/validate.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/stage/validate.rs",
      "originSha256": "d301530692f2f02a51bb68cf8b30ee51db6d3f3c9019b9045ed2a50e3ad8debf",
      "startLine": 1,
      "endLine": 447,
      "unitSha256": "d301530692f2f02a51bb68cf8b30ee51db6d3f3c9019b9045ed2a50e3ad8debf",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-core/src/stage/digest.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/stage/digest.rs",
      "originSha256": "8193f18852499f0e3d276e2187f57fbb1fc6bf5ea8faa51a40510919655fdf72",
      "startLine": 1,
      "endLine": 38,
      "unitSha256": "8193f18852499f0e3d276e2187f57fbb1fc6bf5ea8faa51a40510919655fdf72",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/rust-toolchain.toml",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/rust-toolchain.toml",
      "originSha256": "72f7dd8c9fdf9c1fcd8eb8bf9326d4bc66833e4ebf4bc2932636e2b85efcb7af",
      "startLine": 1,
      "endLine": 4,
      "unitSha256": "72f7dd8c9fdf9c1fcd8eb8bf9326d4bc66833e4ebf4bc2932636e2b85efcb7af",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "units/core_io-read-source.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/core_io.rs",
      "originSha256": "6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594",
      "startLine": 5,
      "endLine": 22,
      "unitSha256": "e8f1a0d5cd1c2eeee11708b9ceaab206b8f08435fd6e072259edc1932203ce33",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/core_io-operation.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/core_io.rs",
      "originSha256": "6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594",
      "startLine": 158,
      "endLine": 164,
      "unitSha256": "51d63b55691c08ef387fc01056e995f95500a286660ef79ec6fea42edb8d9f6c",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/core_io-print.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/core_io.rs",
      "originSha256": "6ebccb1465788264448ac791dea248e68a1cea71942496ee303ead619672b594",
      "startLine": 247,
      "endLine": 251,
      "unitSha256": "7f1351583d72965b909c84ede5884a48fba83dc868dde6f57ebcd4f165a868cd",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/source-hash-bytes.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/intent.rs",
      "originSha256": "cb69505636fa017f473749e33b1772ef06813f061c68b15919e76b355cd815da",
      "startLine": 69,
      "endLine": 71,
      "unitSha256": "035b8ce048dc09ecbcd5477c5d15b942bf9161e9b4ab65097b9e04451a0ff295",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/cli-global-options.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/cli.rs",
      "originSha256": "b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789",
      "startLine": 37,
      "endLine": 63,
      "unitSha256": "d39eb77f73e940188e0746b96231f679ee88c9bad30c4072a49a7e6761137fd2",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/cli-color-conversion.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/cli.rs",
      "originSha256": "b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789",
      "startLine": 99,
      "endLine": 114,
      "unitSha256": "b3040b38b5cfa64baa23e2c50da404635b86adc6c7ca5b9d61bc049827df1fc5",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/stage-load-complete-and-used-types.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/stage/mod.rs",
      "originSha256": "e3c4a463319056d0a32ac192c57b3f587584976e08974f4dec04c9e881f80f23",
      "startLine": 1,
      "endLine": 92,
      "unitSha256": "5318efe2b81d92855ef66f047b3a75f2e3d35ce9925a0f9a40035a181abfb14e",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/core-static-validation-and-type-helpers.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/runtime.rs",
      "originSha256": "eceac176f673180e83533ce55a99afa5bdf3f97a548bbb1c463dac3f84c83930",
      "startLine": 1,
      "endLine": 394,
      "unitSha256": "fc4a4130fb948baf36fb7abea805be40d9cde2e06c19a244493ec721fca30d51",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/stage-static-cost-fill-and-helpers.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/stage/cost.rs",
      "originSha256": "eeb5c9fbd1c58c5b46f346a0e62484c1bb366e548c50824295d833648ba6bcc1",
      "startLine": 1,
      "endLine": 151,
      "unitSha256": "67efcf8bec34ce1e7d115b7e0543b627112a5087ca11196705b5df01a631308a",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/M01-arena-contract-complete.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/docs/CORE-STAGED-TESTS.md",
      "originSha256": "4ad9df1dc5111535800862cb6921751fb65d0db54c969a8a29bda9d7178315c3",
      "startLine": 237,
      "endLine": 299,
      "unitSha256": "edb4f032661ca96f5a98e92617c3e721c8e51a674c92ba0bb6d3dd89aaf5dc25",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/UNI-018.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
      "originSha256": "2da7f711ab50794cd7a94756c4e46ca450e65e2127610aa723805caaf7a8de76",
      "startLine": 190,
      "endLine": 200,
      "unitSha256": "97a9c37cc07986731b005ea281e07e4449e90617f8a77c2bc57a43acaf1cf1d5",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/UNI-019.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
      "originSha256": "2da7f711ab50794cd7a94756c4e46ca450e65e2127610aa723805caaf7a8de76",
      "startLine": 217,
      "endLine": 227,
      "unitSha256": "efe795f62ee80cca24ba9d1fe7902a892e40081d0b23e139e9cc729b9c5901d2",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/UNI-020.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
      "originSha256": "2da7f711ab50794cd7a94756c4e46ca450e65e2127610aa723805caaf7a8de76",
      "startLine": 228,
      "endLine": 246,
      "unitSha256": "cee03cd3ef4953fa663c532740b81726e26adfe5c0d97e67c5c874d2161a0b19",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/UNI-021.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md",
      "originSha256": "2da7f711ab50794cd7a94756c4e46ca450e65e2127610aa723805caaf7a8de76",
      "startLine": 247,
      "endLine": 263,
      "unitSha256": "952a35788ae90d852073016dd8f220d72d6fce48331d3f64ef97eb3757c5af4c",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "units/permissionless-product-boundary.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/docs/MORIARTY-PRODUCT-CONTRACT.md",
      "originSha256": "d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396",
      "startLine": 1,
      "endLine": 17,
      "unitSha256": "dc2de73287cd8cc172a409e10cfbe404badea933842d292452cceb146464b4f3",
      "scope": "Complete previously required semantic unit, refreshed against current main origin; full scope preserved"
    },
    {
      "packetPath": "source/RUST_GUIDELINES.md",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/RUST_GUIDELINES.md",
      "originSha256": "e448f368eab4ff3634ebfc65edcc3ee62d6cf8bf3e16bb76d81f2facec370af9",
      "startLine": 1,
      "endLine": 331,
      "unitSha256": "e448f368eab4ff3634ebfc65edcc3ee62d6cf8bf3e16bb76d81f2facec370af9",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "before/RUST_GUIDELINES.md",
      "origin": "/home/charl/moriarty-c2-opus55-restored-20261017/concurrency-increase-main238/boolean-main829f-integration-preparation/before/RUST_GUIDELINES.md",
      "originSha256": "7ac417984778c52387d49295364b1e2f85bde14d04d2130dc2c09b4f3bf9d404",
      "startLine": 1,
      "endLine": 324,
      "unitSha256": "7ac417984778c52387d49295364b1e2f85bde14d04d2130dc2c09b4f3bf9d404",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/cli.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/cli.rs",
      "originSha256": "b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789",
      "startLine": 1,
      "endLine": 271,
      "unitSha256": "b978fb649070a6ab2a3294d6b5d22ba088fc3d5faaeae3ffecef96958ec6e789",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/commands/expand.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/commands/expand.rs",
      "originSha256": "f7ff48dcaece95b05392e25ad3a9dff66dd736f839ceaa2928bf9d768aedc320",
      "startLine": 1,
      "endLine": 47,
      "unitSha256": "f7ff48dcaece95b05392e25ad3a9dff66dd736f839ceaa2928bf9d768aedc320",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/error.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/error.rs",
      "originSha256": "9c1f5a1207cfaf2509e159cdeed344a60b172e196988d021acab6cc9a45b918c",
      "startLine": 1,
      "endLine": 34,
      "unitSha256": "9c1f5a1207cfaf2509e159cdeed344a60b172e196988d021acab6cc9a45b918c",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori/src/json.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori/src/json.rs",
      "originSha256": "6f68940fe32eced929b8af229336eb07e9af745a78b32f45966858c7c382a3aa",
      "startLine": 1,
      "endLine": 258,
      "unitSha256": "6f68940fe32eced929b8af229336eb07e9af745a78b32f45966858c7c382a3aa",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-diagnostics/src/lib.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-diagnostics/src/lib.rs",
      "originSha256": "f0f50241dc901df6d66b3bc25c9c32c085b2de2b4dd7b8f04b89603b4ef5117c",
      "startLine": 1,
      "endLine": 17,
      "unitSha256": "f0f50241dc901df6d66b3bc25c9c32c085b2de2b4dd7b8f04b89603b4ef5117c",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-diagnostics/src/diagnostic.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-diagnostics/src/diagnostic.rs",
      "originSha256": "41bc1a257e594fc8da0c625c6856d61ac3036afc5b2ae3636289ff411eae78f1",
      "startLine": 1,
      "endLine": 116,
      "unitSha256": "41bc1a257e594fc8da0c625c6856d61ac3036afc5b2ae3636289ff411eae78f1",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "source/crates/mori-diagnostics/src/render.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-diagnostics/src/render.rs",
      "originSha256": "b28d889ac235f07a7513995ef5bbffd42b1999ce95d8cc2056bd2774f5559bdc",
      "startLine": 1,
      "endLine": 101,
      "unitSha256": "b28d889ac235f07a7513995ef5bbffd42b1999ce95d8cc2056bd2774f5559bdc",
      "scope": "Complete original body; no candidate or semantic fields truncated"
    },
    {
      "packetPath": "units/loader-canonical-json-complete.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/intent.rs",
      "originSha256": "cb69505636fa017f473749e33b1772ef06813f061c68b15919e76b355cd815da",
      "startLine": 72,
      "endLine": 88,
      "unitSha256": "10af6779f70596a021b2cb193c7243611f2b8921dae2e7b3eef181915dfe54c5",
      "scope": "Complete named helper used by source hashing/static loader serialization; no security judgment"
    },
    {
      "packetPath": "units/source-and-loader-hex-complete.rs",
      "origin": "/home/charl/Moriarty/.worktrees/utxo-boolean-main-20261018/crates/mori-core/src/intent.rs",
      "originSha256": "cb69505636fa017f473749e33b1772ef06813f061c68b15919e76b355cd815da",
      "startLine": 89,
      "endLine": 91,
      "unitSha256": "1111d9a5818faa5bb64d5e466f3ba7527fbc8f1611f83ae2ced8a26bacc4ee15",
      "scope": "Complete named helper used by source hashing/static loader serialization; no security judgment"
    }
  ]
}
END FILE CURRENT-SOURCE-UNIT-MAP.json
