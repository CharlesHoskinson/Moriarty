Designated SECURITY SOURCE REPAIR AUTHOR Gemini4ArgonHIGH fallback: fresh substantiveOpus4.8taskfailed rate_limit_event rejected06:54:51UTC; no recovery. This task authors no productionbehavior/securityapproval; independentnonauthorreview/exactfutureexecution still required. No commands/tools/builds/Mori/models/SDK/key/SRS/wallet/private/config/cache/network. Exactsource andbaselineArgs INLINE. Return onecomplete structuredobject token doctorwatson-cli-native-boundary-gemini-source-repair-20261018-a4-whole-source-patch, sourcePatch as small valid unifieddiff with --- a/native_feature_boundary.rs and +++ b/native_feature_boundary.rs, full coherentcatalog. Keep concise; doNOTreprintwholeRustsource. Do not guessflags/APIs.

Concrete defects torepair: current source writes b"moriarty/2\n\naction settle() {}\n", invalid actualCore2source; replace literalwith EXACT suppliedvalidR3fixturebytes as rawRuststring insidewrite (no externalfixturepath/newdependency). Allrequiredsourcebytesbelow. Nativeunavailableprecedence means settle actionneednotexist inreadsource, but realvalid/2mustbediscriminatedbytest. TempRoot::new should fs::create_dir(&dir), not create_dir_all reusingexistingroot; inner srs_dircreationunchanged. Check allargvagainst baselineArg definitions, including complete requiredflatteninputs and allnativewrappers. Check compileplausibility of borrowedstringtuple args and contains; change onlyrealissues. Keepfilehard #![cfg(not(feature="native"))], stdonly, ownedtemp, exactexit1nativecode/fixedmessage normalizedwhitespace/emptystdout/unchangedsentinels, clapexit2/semanticcompanionerrors. No helpsnapshots or fakekeys/nativeinvocations. Catalogmustmatchactual22+ cases/membership; no computedimplementationoracle. Unchangedsentinels/staticaudit doNOTproveabsenceofread-onlysyscalls. Thisunappliedsource is EXCLUDED fromcurrent28pathcandidate and alreadyrunningexisting12tests. Do not change currentcheckout.

### Untrustedsource
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
        fs::create_dir_all(&dir).expect("create temp root");
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
            let rel = path.strip_prefix(base).expect("relative path").to_path_buf();
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
                b"moriarty/2\n\naction settle() {}\n",
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
            assert_eq!(code, Some(1), "{phase}/{label}: expected exit 1, stderr: {stderr}");
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

    for (label, argv, expected_msg) in semantic_cases {
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

### Exactvalid/2source
language "moriarty/2";
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

### Baseline actualargs
Author a COMPACT complete standard-Rust integration test for independently specified no-native availability behavior. Security ownership: Gemini4ArgonHIGH explicit Opus outage fallback, source-only author not reviewer. Prior90KB request timeout420s no object retained as failure; this smaller independent request does not inspect changedcandidate. Read primary source INLINE below only. Do not run commands/tools/Mori/compiler, browse, write, inspect realconfig/keys/params/caches or candidate. Return structured whole object, ENTIRE rustSource <=9000characters, catalog <=5000characters. Use TWO test functions with table loops to minimize duplication. No essays.

Contract: file path crates/mori/tests/native_feature_boundary.rs; crate-level #![cfg(not(feature = "native"))] BEFORE allimports/tests, so defaultnative selects ZERO and zero is not pass. stdonly, CARGO_BIN_EXE_mori subprocess. Owned unique temp root guard Drop, set child current_dir there; all fake relative paths/sentinels under root; no real keys/params/wallets/network. Invoke --color never. For valid syntactic native-only routes compile,compile-call,prepare-call,keygen-call,prove-call,verify-call,keygen-signed-call,prepare-signed-call,prove-signed-call,verify-signed-call,prove,verify,replay plus mixed inspect --target,simulate --trace+--target,intent --profile+target+at,verify-intent --profile+target, expect OS exit1 EMPTYstdout and stderr diagnostic code mori::command::native-unavailable with message This mori build has no Midnight support. Rebuild with the native feature enabled. Existing main renders diagnostic with wrapping, so normalize stderr.split_whitespace().join(" ") only for fixedmessage substring; do not guess fullwrapped stderrbytes. All required argv from baselineclap definitions below (not invented). Test validroutes once with missinginputs and once existing benignsentinels including valid /2 source on inspect to confirm availability precedence. Entire directory tree bytes unchanged after each invocation; no outputdirs/leaves created. Fake params-dir/keyset still just paths, never nativeexecution because no-native gate.

Second function checks representative missingrequiredargument/clapconflict OSexit2 beforeunavailability, plus argument-only semantic errors OSexit1/emptyout/no native-unavailable. simulate --trace missingtarget => --trace requires --target; intent signedprofile missingat => requires --target and --at together; verify-intent signedprofile missingtarget => requires --target. Include complete commonrequiredActionInput/source/scenario/authority/at/signature flags so these reach specified semantic error, not clap missingargument. Cases cannot loadfiles. Only realfilesystemwrites are owned benign testfixturecreation; no realnative/default invocations. Catalog exact argv/exit/diagnostic/IObasis independentbeforeimplementation execution. Limitations state unchangedsentinels/errorprecedence plus sourceaudit do NOT prove absence of read-only syscalls; no strace/seam; defaultnative tests unrun. Keep source <=9000characters, concisecatalog; deliver one complete consistent object.
### crates/mori/src/commands/compile.rs baseline argument definitions
use super::core_io;
use crate::{error::CliError, shell::Shell};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    /// Exact target profile JSON with network, anchor and compiler limits
    #[arg(long)]
    pub target: PathBuf,
    /// New output directory; refuses to overwrite existing artifacts
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/compile_call.rs baseline argument definitions
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};

#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: Option<String>,
    #[arg(long)]
    pub deployment: Option<PathBuf>,
    /// New output directory, containing tagged IR and static manifest
    #[arg(long)]
    pub out: Option<PathBuf>,
    #[arg(long)]
    pub all_actions: bool,
    #[arg(long)]
    pub static_bindings: Option<PathBuf>,
    #[arg(long)]
    pub authorization: Option<String>,
    #[arg(long)]
    pub signer_policy: Option<PathBuf>,
    #[arg(long)]
    pub work_limits: Option<PathBuf>,
    #[arg(long)]
    pub inspect_static: Option<PathBuf>,
}
pub(crate) fn artifact_bytes(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = vec![];
    file.take((mori_ledger::codec::OUTPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::OUTPUT_BYTES {
        return Err(core_io::operation("relation artifact exceeds output bound"));
    }
    Ok(bytes)
}

### crates/mori/src/commands/prepare_call.rs baseline argument definitions
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use std::{
    io::Read,
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    /// Source-static native relation directory produced by compile-call
    #[arg(long)]
    pub relation: Option<PathBuf>,
    #[arg(long)]
    pub out: PathBuf,
}
fn binary(path: &Path) -> super::Result<Vec<u8>> {
    let file = std::fs::File::open(path).map_err(|source| CliError::Read {
        path: path.into(),
        source,
    })?;
    let mut bytes = Vec::new();
    file.take((mori_ledger::codec::INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|source| CliError::Read {
            path: path.into(),
            source,
        })?;
    if bytes.len() > mori_ledger::codec::INPUT_BYTES {
        return Err(core_io::operation("native public input exceeds 1 MiB"));
    }
    Ok(bytes)
}

### crates/mori/src/commands/keygen_call.rs baseline argument definitions
use super::{call_io, core_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub seed_deployment: PathBuf,
    #[arg(long)]
    pub seed_state: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/prove_call.rs baseline argument definitions
use super::{call_io, core_io};
use crate::shell::Shell;
use rand::SeedableRng;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: call_io::Input,
    #[arg(long)]
    pub keyset: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/verify_call.rs baseline argument definitions
use super::{call_io, core_io};
use crate::shell::Shell;
use std::{io::Write, path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub proof: PathBuf,
}

### crates/mori/src/commands/keygen_signed_call.rs baseline argument definitions
use super::{call_io, core_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    pub file: PathBuf,
    #[arg(long)]
    pub signer_policy: PathBuf,
    #[arg(long)]
    pub seed_deployment: PathBuf,
    #[arg(long)]
    pub seed_state: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/prepare_signed_call.rs baseline argument definitions
use super::core_io;
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/prove_signed_call.rs baseline argument definitions
use super::{call_io, core_io};
use crate::shell::Shell;
use rand::SeedableRng;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub keyset: PathBuf,
    #[arg(long)]
    pub signed_attachment: PathBuf,
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/verify_signed_call.rs baseline argument definitions
use super::core_io;
use crate::shell::Shell;
use std::{io::Write, path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: super::signed_call_io::Input,
    #[arg(long)]
    pub public_keyset: PathBuf,
    #[arg(long)]
    pub proof: PathBuf,
    #[arg(long)]
    pub signed_attachment: PathBuf,
}

### crates/mori/src/commands/prove.rs baseline argument definitions
use super::{core_io, proof_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: proof_io::Input,
    /// New proof bundle file; refuses to overwrite existing files
    #[arg(long)]
    pub out: PathBuf,
}

### crates/mori/src/commands/verify.rs baseline argument definitions
use super::{core_io, proof_io};
use crate::shell::Shell;
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: proof_io::Input,
    /// Native proof bundle to verify without modifying it
    #[arg(long)]
    pub proof: PathBuf,
}

### crates/mori/src/commands/replay.rs baseline argument definitions
//! Actual CLI replay consumer; all roots and signature attachment are separate inputs.
use crate::{
    cli::{ActionInput, SignedProfile},
    shell::Shell,
};
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    replay::{ReplayAuthorization, ReplayError, ReplayRecord},
};
use mori_core::UInt;
use std::{path::PathBuf, process::ExitCode};

#[derive(Debug, clap::Args, Default)]
pub struct AuthorizationArgs {
    /// Strict signed owner-intent profile (signed Targets require the complete group)
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Separately trusted expected account/domain authority
    #[arg(long, value_name = "FILE")]
    pub authority: Option<PathBuf>,
    /// Separately trusted logical time (not authenticated ledger time)
    #[arg(long)]
    pub at: Option<u128>,
    /// Actual external signed wrapper; never loaded from the record
    #[arg(long, value_name = "FILE")]
    pub signature: Option<PathBuf>,
}
pub enum LoadedAuthorization {
    Unsigned,
    Signed {
        authority: Box<ExpectedAuthority>,
        at: UInt,
        signature: Box<SignedIntent>,
    },
}
impl AuthorizationArgs {
    pub fn present(&self) -> bool {
        self.profile.is_some()
            || self.authority.is_some()
            || self.at.is_some()
            || self.signature.is_some()
    }
    pub fn load(&self, target: &Target) -> super::Result<LoadedAuthorization> {
        if target.signed_policy.is_none() {
            if self.present() {
                return Err(super::core_io::operation(
                    "Signed replay arguments require a trusted Target with signed_policy",
                ));
            }
            return Ok(LoadedAuthorization::Unsigned);
        }
        let (Some(_), Some(authority), Some(at), Some(signature)) =
            (&self.profile, &self.authority, self.at, &self.signature)
        else {
            return Err(super::core_io::operation(
                "Signed Target requires --profile moriarty-jubjub-owner-intent/1, --authority, --at and --signature together",
            ));
        };
        Ok(LoadedAuthorization::Signed {
            authority: Box::new(super::core_io::authority(authority)?),
            at: UInt(at),
            signature: Box::new(super::core_io::signed_intent(signature)?),
        })
    }
}
impl LoadedAuthorization {
    pub fn as_input(&self) -> ReplayAuthorization<'_> {
        match self {
            Self::Unsigned => ReplayAuthorization::Unsigned,
            Self::Signed {
                authority,
                at,
                signature,
            } => ReplayAuthorization::Signed {
                authority,
                at: *at,
                signature,
            },
        }
    }
}
#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,
    /// Separately trusted Target JSON
    #[arg(long, value_name = "FILE")]
    pub target: PathBuf,
    /// Public sidecar to compare; never supplies trusted roots
    #[arg(long, value_name = "FILE")]
    pub record: PathBuf,
    /// Export a bounded local first-divergence diagnostic to a new file
    #[arg(long, value_name = "FILE")]
    pub challenge_out: Option<PathBuf>,
    #[command(flatten)]
    pub authorization: AuthorizationArgs,
}

### crates/mori/src/commands/intent.rs baseline argument definitions
use std::process::ExitCode;

use clap::ValueEnum;

use super::core_io::review_identity;
use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Legacy host profile: signature scheme of the owner key
    #[arg(long)]
    pub scheme: Option<Scheme>,

    /// Legacy host profile: owner public key, hex encoded
    #[arg(long, value_name = "HEX")]
    pub public_key: Option<String>,

    /// Legacy host profile: how the signing bytes are framed
    #[arg(long)]
    pub framing: Option<Framing>,
    /// Domain, predecessor, nonce, validity and per-asset authority JSON
    #[arg(long)]
    pub authority: std::path::PathBuf,

    /// Signed native owner-intent profile; requires --target and --at
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    /// Logical time checked against the validity interval; not signed
    #[arg(long)]
    pub at: Option<u128>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Scheme {
    #[value(name = "schnorr_bip340")]
    SchnorrBip340,
    #[value(name = "ecdsa_secp256k1_sha256")]
    EcdsaSecp256k1Sha256,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Framing {
    Raw,
    MidnightSignData,
}


### crates/mori/src/commands/verify_intent.rs baseline argument definitions
use std::path::PathBuf;
use std::process::ExitCode;

use crate::cli::{ActionInput, SignedProfile, SigningOutputArgs};
use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[command(flatten)]
    pub input: ActionInput,

    /// Signature JSON produced by the external signer (the exact signed
    /// wrapper under --profile moriarty-jubjub-owner-intent/1)
    #[arg(long, value_name = "FILE")]
    pub signature: PathBuf,
    /// Expected account authority and domain binding JSON
    #[arg(long)]
    pub authority: PathBuf,
    /// Current logical time used to check the signed validity interval
    #[arg(long)]
    pub at: u128,
    /// Signed native owner-intent profile; requires --target
    #[arg(long)]
    pub profile: Option<SignedProfile>,
    /// Trusted Target JSON with signed_policy (signed profile only)
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,

    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with_all = ["profile", "target"])]
    pub logical_cap: Option<mori_core::UInt>,

    #[command(flatten)]
    pub output: SigningOutputArgs,
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}


### crates/mori/src/commands/simulate.rs baseline argument definitions
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,

    /// Action to run
    #[arg(long, value_name = "NAME", required_unless_present = "resume")]
    pub action: Option<String>,

    /// Scenario JSON with the local state and inputs
    #[arg(long, value_name = "FILE", required_unless_present = "resume")]
    pub scenario: Option<PathBuf>,
    /// Resume a moriarty/2 continuation (local, unauthenticated, consumption untracked)
    #[arg(
        long,
        value_name = "FILE",
        requires = "event",
        conflicts_with_all = ["scenario", "trace", "target", "profile", "authority", "at", "signature"]
    )]
    pub resume: Option<PathBuf>,
    /// Stage event JSON for --resume (at most 64 KiB)
    #[arg(long, value_name = "FILE", requires = "resume")]
    pub event: Option<PathBuf>,
    /// Generate a complete public replay sidecar against a trusted Target
    #[arg(long)]
    pub trace: bool,
    /// Cap local statement and uncached expression attempts (other costs unmeasured)
    #[arg(long, value_name = "CANONICAL_U128", value_parser = logical_cap, conflicts_with = "trace")]
    pub logical_cap: Option<mori_core::UInt>,
    /// Trusted Target JSON (required with --trace)
    #[arg(long, value_name = "FILE")]
    pub target: Option<std::path::PathBuf>,
    #[command(flatten)]
    pub authorization: super::replay::AuthorizationArgs,
}

impl Args {
    /// Legacy start inputs. Clap requires both whenever --resume is absent.
    fn legacy(&self) -> super::Result<(&str, &Path)> {
        match (self.action.as_deref(), self.scenario.as_deref()) {
            (Some(action), Some(scenario)) => Ok((action, scenario)),
            _ => Err(super::core_io::operation(
                "--action and --scenario are required without --resume",
            )),
        }
    }
}

fn logical_cap(value: &str) -> Result<mori_core::UInt, String> {
    serde_json::from_value(serde_json::Value::String(value.into()))
        .map_err(|_| "expected canonical unsigned decimal string in u128 range".into())
}


### crates/mori/src/commands/inspect.rs baseline argument definitions
use std::{collections::BTreeSet, path::PathBuf, process::ExitCode};

use mori_backend::{Artifact, CircuitArtifact, PublicBinding};
use mori_core::{Action, BinaryOp, Expr, ExprId, Program, Statement, Type};
use serde_json::{Value, json};

use crate::shell::Shell;

#[derive(Debug, clap::Args)]
pub struct Args {
    /// Source file
    pub file: PathBuf,
    /// Compile a public enforcement map against this exact target descriptor
    #[arg(long, value_name = "FILE")]
    pub target: Option<PathBuf>,
}


### crates/mori/src/commands/proof_io.rs baseline argument definitions
use super::core_io;
use crate::{cli::ActionInput, error::CliError};
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

#[derive(Debug, clap::Args)]
pub struct Input {
    #[command(flatten)]
    pub action: ActionInput,
    /// Exact target profile JSON
    #[arg(long)]
    pub target: PathBuf,
    /// Explicit directory containing checksum-pinned offline SRS files
    #[arg(long)]
    pub params_dir: PathBuf,
    /// Maximum permitted native circuit degree (1 through 19)
    #[arg(long, default_value_t = 9)]
    pub max_k: u8,
    /// Exact signed owner-intent wrapper (required with a signed Target)
    #[arg(long, value_name = "FILE")]
    pub signature: Option<PathBuf>,
    /// Expected owner authority JSON (required with a signed Target)
    #[arg(long, value_name = "FILE")]
    pub authority: Option<PathBuf>,
    /// Logical time checked against the signed interval (required with a signed Target)
    #[arg(long)]
    pub at: Option<u128>,
}
/// The complete signed group accompanying a trusted signed Target.

### crates/mori/src/commands/call_io.rs baseline argument definitions
//! Explicit installed-call inputs and create-new publication. No proving
//! provider, parameter fallback or alternate TransitionClaim consumer exists.
use super::{core_io, proof_io};
use crate::{error::CliError, shell::Shell};
use mori_ledger::{
    call_artifacts::{self, ArtifactLeaves},
    profile::{CallRequest, InstalledDeployment, NativeWorkLimits},
};
use std::{
    path::{Path, PathBuf},
    process::ExitCode,
};
#[derive(Debug, clap::Args)]
pub struct Input {
    pub file: PathBuf,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
}
pub struct Inputs {
    pub source: String,
    pub program: mori_core::Program,
    pub deployment: InstalledDeployment,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
    pub request: CallRequest,
    pub limits: NativeWorkLimits,
}
pub fn limits(path: &Path) -> super::Result<NativeWorkLimits> {
    let text = text(path, mori_ledger::codec::INPUT_BYTES as u64)?;
    let value = NativeWorkLimits::from_json(&text).map_err(core_io::operation)?;
    if text.len() as u64 > value.max_input_bytes {
        return Err(core_io::operation(
            "work limits input exceeds explicit input bound",
        ));
    }
    Ok(value)
}
pub fn binary(path: &Path, cap: u64) -> super::Result<Vec<u8>> {
    call_artifacts::read_bounded_file(path, cap).map_err(core_io::operation)
}
pub fn text(path: &Path, cap: u64) -> super::Result<String> {
    String::from_utf8(binary(
        path,
        cap.min(mori_ledger::codec::INPUT_BYTES as u64),
    )?)
    .map_err(core_io::operation)
}
pub fn program(path: &Path, cap: u64) -> super::Result<(String, mori_core::Program)> {
    let source = text(path, cap)?;
    let program = mori_core::parse(&source).map_err(core_io::operation)?;
    Ok((source, program))
}

### crates/mori/src/commands/signed_call_io.rs baseline argument definitions
use super::{call_io, core_io};
use mori_ledger::{profile::NativeWorkLimits, signed::*};
use std::{path::PathBuf, process::ExitCode};
#[derive(Debug, clap::Args)]
pub struct Input {
    pub file: PathBuf,
    #[arg(long)]
    pub signer_policy: PathBuf,
    #[arg(long)]
    pub deployment: PathBuf,
    #[arg(long)]
    pub predecessor: PathBuf,
    #[arg(long)]
    pub parameters: PathBuf,
    #[arg(long)]
    pub request: PathBuf,
    #[arg(long)]
    pub action: String,
    #[arg(long)]
    pub params_dir: PathBuf,
    #[arg(long)]
    pub work_limits: PathBuf,
    #[arg(long)]
    pub derive_supplied_frame: bool,
}
pub struct Inputs {
    pub source: String,
    pub program: mori_core::Program,
    pub policy: FixedSignerPolicy,
    pub deployment: SignedInstalledDeployment,
    pub predecessor: Vec<u8>,
    pub parameters: Vec<u8>,
    pub request: SignedCallRequest,
    pub limits: NativeWorkLimits,
}

### cli.rs baseline
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
    /// Create a new project from a starter template
    Init(init::Args),
    /// Check syntax, names, types and action support
    Check(check::Args),
    /// Compile a bounded Moriarty program into ledger-9 ZKIRv3 circuits
    Compile(compile::Args),
    /// Compile a source-static ordinary native contract-call relation
    CompileCall(compile_call::Args),
    /// Derive all source action keys and an explicit local installed snapshot
    KeygenCall(keygen_call::Args),
    /// Derive the complete local signed action map under a fixed JubJub policy
    KeygenSignedCall(keygen_signed_call::Args),
    /// Reconstruct a signature-free canonical local call signing package
    PrepareSignedCall(prepare_signed_call::Args),
    /// Prove the actual local typed SDK call with an external FENS attachment
    ProveSignedCall(prove_signed_call::Args),
    /// Cold public-only reconstruction and Real verification of a signed call
    VerifySignedCall(verify_signed_call::Args),
    /// Cold rederive keys and prove an actual local installed SDK call
    ProveCall(prove_call::Args),
    /// Cold rederive public keys and Real-verify an installed call without PK files
    VerifyCall(verify_call::Args),
    /// Prove a public local action using pinned offline parameters
    Prove(prove::Args),
    /// Prepare an unsigned, unproven local native call and public offer
    PrepareCall(prepare_call::Args),
    /// Verify a native proof against the expected source, target and scenario
    Verify(verify::Args),
    /// Format source files in place
    #[command(visible_alias = "fmt")]
    Format(format::Args),
    /// Show identity claims, operative bounds and open premises
    Inspect(inspect::Args),
    /// Show the native core IR after checking an action against a scenario
    Expand(expand::Args),
    /// Run an action locally against a scenario
    Simulate(simulate::Args),
    /// Independently replay a public record against separately trusted inputs
    Replay(replay::Args),
    /// Run the test cases in a project directory
    Test(test::Args),
    /// Produce the owner signing bytes for an action
    Intent(intent::Args),
    /// Verify a signature, then prepare the action locally
    VerifyIntent(verify_intent::Args),
    /// Run the language server over stdio
    Lsp(lsp::Args),
    /// Run the read-only MCP server over stdio
    Mcp(mcp::Args),
}

### Priorcatalog
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
    "ioBasis": "missing-inputs & populated-sentinels; out.proof untouched/uncreated",
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
    "ioBasis": "missing-inputs & valid moriarty/2 source sentinel + invalid target sentinel; tree unchanged",
    "route": "inspect --target"
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
    "route": "simulate --trace --target"
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
    "route": "intent --profile --target --at"
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
    "route": "verify-intent --profile --target"
  },
  {
    "argv": [
      "compile",
      "agreement.mori",
      "--target",
      "target.json"
    ],
    "diagnostic": "clap missing --out; no native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created",
    "route": "clap-missing-arg-compile-out"
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
    "diagnostic": "clap conflict --logical-cap vs --trace; no native-unavailable",
    "exit": 2,
    "ioBasis": "empty temp root; no files loaded or created",
    "route": "clap-conflict-simulate-cap-trace"
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
    "diagnostic": "--trace requires --target; no native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate",
    "route": "semantic-simulate-trace-no-target"
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
    "diagnostic": "requires --target and --at together; no native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate",
    "route": "semantic-intent-signed-no-at"
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
    "diagnostic": "requires --target; no native-unavailable",
    "exit": 1,
    "ioBasis": "empty temp root; fails before file load or native gate",
    "route": "semantic-verify-intent-signed-no-target"
  }
]
