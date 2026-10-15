//! Source-only membership regression for `proving::source_closure` (H28).
//! `source_closure` is crate-private, so this reads its declaration statically
//! instead of calling it. No SDK, key, parameter or native operation is used.
//! The closure is declared source identity, not authentication of the build.
use std::{collections::BTreeSet, fs, path::PathBuf};

const PROVING: &str = include_str!("../src/proving.rs");

// Frozen independently from the pre-A2 `source_closure` at baseline fe01bc8b.
const BASELINE: [&str; 61] = [
    "proving.rs",
    "relation/mod.rs",
    "relation/compiler.rs",
    "relation/arithmetic.rs",
    "relation/event.rs",
    "relation/process.rs",
    "signed/mod.rs",
    "signed/nonce.rs",
    "signed/schnorr.rs",
    "signed/constraints.rs",
    "signed/context.rs",
    "signed/deployment.rs",
    "signed/preparation.rs",
    "../third-party/jubjub-fens/NOTICE.md",
    "../third-party/jubjub-fens/LICENSE-MIT",
    "../third-party/jubjub-fens/LICENSE-APACHE",
    "prepare.rs",
    "ops.rs",
    "layout.rs",
    "profile.rs",
    "custody.rs",
    "codec.rs",
    "lib.rs",
    "parameters.rs",
    "local_deployment.rs",
    "static_compile.rs",
    "call_artifacts.rs",
    "../../mori/Cargo.toml",
    "../../mori/src/cli.rs",
    "../../mori/src/commands/mod.rs",
    "../../mori/src/commands/call_io.rs",
    "../../mori/src/commands/compile_call.rs",
    "../../mori/src/commands/signed_call_io.rs",
    "../../mori/src/commands/keygen_signed_call.rs",
    "../../mori/src/commands/prepare_signed_call.rs",
    "../../mori/src/commands/prove_signed_call.rs",
    "../../mori/src/commands/verify_signed_call.rs",
    "../../mori/src/commands/keygen_call.rs",
    "../../mori/src/commands/prove_call.rs",
    "../../mori/src/commands/verify_call.rs",
    "../Cargo.toml",
    "../../../Cargo.toml",
    "../../../Cargo.lock",
    "../../mori-core/Cargo.toml",
    "../../mori-core/src/lib.rs",
    "../../mori-core/src/parser.rs",
    "../../mori-core/src/ir.rs",
    "../../mori-core/src/runtime.rs",
    "../../mori-core/src/logical_cost.rs",
    "../../mori-core/src/source_sponsor.rs",
    "../../mori-core/src/json.rs",
    "../../mori-core/src/intent.rs",
    "../../mori-core/src/policy.rs",
    "../../mori-core/src/replay.rs",
    "../../mori-lexer/Cargo.toml",
    "../../mori-lexer/src/lib.rs",
    "../../mori-lexer/src/scan.rs",
    "../../mori-lexer/src/token.rs",
    "../../mori-lexer/src/string.rs",
    "../../mori-lexer/src/number.rs",
    "../../mori-lexer/src/diagnostics.rs",
];

// Frozen A2 append: Core continuation stage files, alphabetical, after baseline.
const CORE_STAGE: [&str; 8] = [
    "../../mori-core/src/stage/admit.rs",
    "../../mori-core/src/stage/cost.rs",
    "../../mori-core/src/stage/decode.rs",
    "../../mori-core/src/stage/digest.rs",
    "../../mori-core/src/stage/mod.rs",
    "../../mori-core/src/stage/plan.rs",
    "../../mori-core/src/stage/profile.rs",
    "../../mori-core/src/stage/validate.rs",
];

/// Ordered `include_str!` paths of the single `source_closure` declaration.
/// Any other token inside `concat!` (literal, macro, comment) is refused.
fn members() -> Vec<&'static str> {
    let signature = "pub(crate) fn source_closure() -> String {";
    let mut defs = PROVING.match_indices("fn source_closure(");
    let (start, _) = defs.next().expect("source_closure declaration");
    assert!(defs.next().is_none(), "single source_closure declaration");
    let body = &PROVING[start..];
    let body = &body[..body.find("\n}\n").expect("source_closure end")];
    assert!(PROVING.contains(signature), "unchanged source_closure signature");
    let rest = body[body.find('{').unwrap() + 1..].trim_start();
    let rest = rest.strip_prefix("codec::hash(").expect("codec::hash wrapper");
    let rest = rest.trim_start().strip_prefix("concat!(").expect("concat!");
    let end = rest.find(".as_bytes()").expect("as_bytes");
    assert_eq!(rest[end..].matches("include_str!").count(), 0);
    let mut args = rest[..end]
        .trim_end()
        .strip_suffix(')')
        .expect("concat! close");
    let mut paths = Vec::new();
    loop {
        args = args.trim_start();
        if args.is_empty() {
            break;
        }
        let tail = args
            .strip_prefix("include_str!(\"")
            .unwrap_or_else(|| panic!("non-include_str closure input: {args:.40}"));
        let close = tail.find("\")").expect("include_str! close");
        let path = &tail[..close];
        assert!(!path.contains(['\\', '"']), "plain closure path: {path}");
        paths.push(path);
        args = tail[close + 2..].trim_start();
        args = args.strip_prefix(',').unwrap_or(args);
    }
    paths
}

fn resolve(path: &str) -> PathBuf {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    fs::canonicalize(src.join(path)).unwrap_or_else(|e| panic!("{path}: {e}"))
}

fn rust_files(dir: PathBuf, out: &mut BTreeSet<PathBuf>) {
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.insert(fs::canonicalize(path).unwrap());
        }
    }
}

#[test]
fn closure_is_frozen_baseline_then_core_stage_files() {
    let actual = members();
    assert!(actual.len() >= BASELINE.len(), "baseline members removed");
    assert_eq!(actual[..BASELINE.len()], BASELINE, "baseline prefix changed");
    assert_eq!(actual[BASELINE.len()..], CORE_STAGE, "Core stage suffix changed");
    assert_eq!(actual.len(), 69);
}

#[test]
fn closure_members_are_distinct_existing_files() {
    let actual = members();
    let resolved: BTreeSet<_> = actual.iter().map(|p| resolve(p)).collect();
    assert_eq!(resolved.len(), actual.len(), "duplicate closure member");
    assert!(resolved.iter().all(|p| p.is_file()));
}

#[test]
fn closure_covers_every_core_source_file() {
    let covered: BTreeSet<_> = members().iter().map(|p| resolve(p)).collect();
    let mut core = BTreeSet::new();
    rust_files(resolve("../../mori-core/src"), &mut core);
    let missing: Vec<_> = core.difference(&covered).collect();
    assert!(missing.is_empty(), "Core source outside closure: {missing:?}");
}
