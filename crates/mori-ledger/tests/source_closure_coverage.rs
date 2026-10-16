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

// Sherlock append: every file under Core `src/sherlock`, alphabetical.
const SHERLOCK: [&str; 35] = [
    "../../mori-core/src/sherlock/checkpoint_normal_tests.rs",
    "../../mori-core/src/sherlock/compare.rs",
    "../../mori-core/src/sherlock/compare_tests.rs",
    "../../mori-core/src/sherlock/draft_image_bridge.rs",
    "../../mori-core/src/sherlock/draft_image_bridge_tests.rs",
    "../../mori-core/src/sherlock/draft_source.rs",
    "../../mori-core/src/sherlock/draft_source_tests.rs",
    "../../mori-core/src/sherlock/draft_typecheck.rs",
    "../../mori-core/src/sherlock/draft_typecheck_tests.rs",
    "../../mori-core/src/sherlock/extra_normal_oracles.rs",
    "../../mori-core/src/sherlock/extra_oracle_programs.rs",
    "../../mori-core/src/sherlock/helper_call_adapter.rs",
    "../../mori-core/src/sherlock/helper_call_adapter_tests.rs",
    "../../mori-core/src/sherlock/helper_schedule.rs",
    "../../mori-core/src/sherlock/helper_schedule_tests.rs",
    "../../mori-core/src/sherlock/image_reference.rs",
    "../../mori-core/src/sherlock/image_reference_tests.rs",
    "../../mori-core/src/sherlock/machine_image.rs",
    "../../mori-core/src/sherlock/machine_image_tests.rs",
    "../../mori-core/src/sherlock/mod.rs",
    "../../mori-core/src/sherlock/normal_oracles.rs",
    "../../mori-core/src/sherlock/program_image.rs",
    "../../mori-core/src/sherlock/program_image_oracles.rs",
    "../../mori-core/src/sherlock/program_image_tests.rs",
    "../../mori-core/src/sherlock/reference.rs",
    "../../mori-core/src/sherlock/reference_display.rs",
    "../../mori-core/src/sherlock/reference_oracles_tests.rs",
    "../../mori-core/src/sherlock/reference_slice.rs",
    "../../mori-core/src/sherlock/reference_slice_tests.rs",
    "../../mori-core/src/sherlock/reference_tests.rs",
    "../../mori-core/src/sherlock/reference_types.rs",
    "../../mori-core/src/sherlock/resource_tests.rs",
    "../../mori-core/src/sherlock/source_adapter.rs",
    "../../mori-core/src/sherlock/source_adapter_tests.rs",
    "../../mori-core/src/sherlock/source_origin_normal_tests.rs",
];

// Backend append: the FENS verifier and SRS pins the ledger calls, with the
// verifier's third-party notice and licenses.
const BACKEND: [&str; 6] = [
    "../../mori-backend/Cargo.toml",
    "../../mori-backend/src/schnorr.rs",
    "../../mori-backend/src/parameters.rs",
    "../../mori-backend/third-party/jubjub-fens/NOTICE.md",
    "../../mori-backend/third-party/jubjub-fens/LICENSE-MIT",
    "../../mori-backend/third-party/jubjub-fens/LICENSE-APACHE",
];

// Frozen A2 coverage roots: Core, lexer and ledger `src`, walked recursively.
const ROOTS: [&str; 3] = ["../../mori-core/src", "../../mori-lexer/src", "."];

/// A walked `.rs` file outside the closure. It must be the sibling module that
/// `parent` (a closure member, `lib.rs` or `mod.rs`) declares only under
/// `#[cfg(test)]`, so the hashed parent pins its test-only status.
struct Exclusion {
    path: &'static str,
    parent: &'static str,
    module: &'static str,
    reason: &'static str,
}

// Committed exclusions (Clarification §3.4). Each must name an unlisted walked
// file; unknown files fail closed and Core stage files can never appear here.
const EXCLUDED: [Exclusion; 3] = [
    Exclusion {
        path: "../../mori-lexer/src/proptests.rs",
        parent: "../../mori-lexer/src/lib.rs",
        module: "proptests",
        reason: "lexer property tests, compiled only under cfg(test)",
    },
    Exclusion {
        path: "../../mori-lexer/src/tests.rs",
        parent: "../../mori-lexer/src/lib.rs",
        module: "tests",
        reason: "lexer snapshot unit tests, compiled only under cfg(test)",
    },
    Exclusion {
        path: "signed/tests.rs",
        parent: "signed/mod.rs",
        module: "tests",
        reason: "signed-call ordinary QA unit tests, compiled only under cfg(test)",
    },
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
    assert!(
        PROVING.contains(signature),
        "unchanged source_closure signature"
    );
    let rest = body[body.find('{').unwrap() + 1..].trim_start();
    let rest = rest
        .strip_prefix("codec::hash(")
        .expect("codec::hash wrapper");
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
        let entry = entry.unwrap();
        let path = entry.path();
        let kind = entry.file_type().unwrap();
        assert!(!kind.is_symlink(), "symlink in source root: {path:?}");
        if kind.is_dir() {
            rust_files(path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.insert(fs::canonicalize(path).unwrap());
        }
    }
}

fn walked() -> BTreeSet<PathBuf> {
    let mut files = BTreeSet::new();
    for root in ROOTS {
        rust_files(resolve(root), &mut files);
    }
    files
}

/// Walked files neither listed nor excluded, then exclusions that name no
/// unlisted walked file (stale, broad, already listed or outside the roots).
fn gaps(
    walked: &BTreeSet<PathBuf>,
    listed: &BTreeSet<PathBuf>,
    excluded: &BTreeSet<PathBuf>,
) -> (Vec<PathBuf>, Vec<PathBuf>) {
    let unlisted: BTreeSet<_> = walked.difference(listed).cloned().collect();
    let missing = unlisted.difference(excluded).cloned().collect();
    let unused = excluded.difference(&unlisted).cloned().collect();
    (missing, unused)
}

#[test]
fn closure_is_frozen_baseline_then_appends() {
    let actual = members();
    let appends = [&CORE_STAGE[..], &SHERLOCK[..], &BACKEND[..]].concat();
    assert!(actual.len() >= BASELINE.len(), "baseline members removed");
    assert_eq!(
        actual[..BASELINE.len()],
        BASELINE,
        "baseline prefix changed"
    );
    assert_eq!(
        actual[BASELINE.len()..],
        appends,
        "appended members changed"
    );
    assert_eq!(actual.len(), BASELINE.len() + appends.len());
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
    assert!(
        missing.is_empty(),
        "Core source outside closure: {missing:?}"
    );
}

#[test]
fn closure_covers_every_core_lexer_and_ledger_source_file() {
    let listed: BTreeSet<_> = members().iter().map(|p| resolve(p)).collect();
    let excluded: BTreeSet<_> = EXCLUDED.iter().map(|x| resolve(x.path)).collect();
    assert_eq!(excluded.len(), EXCLUDED.len(), "duplicate exclusion");
    let walked = walked();
    for probe in ["../../mori-lexer/src/lib.rs", "lib.rs", CORE_STAGE[7]] {
        assert!(walked.contains(&resolve(probe)), "walk missed {probe}");
    }
    let (missing, unused) = gaps(&walked, &listed, &excluded);
    assert!(
        missing.is_empty(),
        "source outside closure and exclusions: {missing:?}"
    );
    assert!(
        unused.is_empty(),
        "exclusion is not an unlisted source file: {unused:?}"
    );
}

#[test]
fn exclusions_are_reasoned_cfg_test_modules_outside_core_stage() {
    let listed: BTreeSet<_> = members().into_iter().collect();
    let stage = resolve("../../mori-core/src/stage");
    assert!(CORE_STAGE.iter().all(|p| resolve(p).starts_with(&stage)));
    let sources: Vec<_> = walked()
        .into_iter()
        .map(|f| (fs::read_to_string(&f).unwrap(), f))
        .collect();
    for x in &EXCLUDED {
        let path = resolve(x.path);
        assert!(
            !x.reason.is_empty() && !x.reason.contains('\n'),
            "{}: reason",
            x.path
        );
        assert!(
            !path.starts_with(&stage),
            "{}: Core stage file excluded",
            x.path
        );
        assert!(
            listed.contains(x.parent),
            "{}: parent outside closure",
            x.path
        );
        let parent = resolve(x.parent);
        assert!(["lib.rs", "mod.rs"].iter().any(|n| parent.ends_with(n)));
        let file = format!("{}.rs", x.module);
        assert_eq!(
            path,
            parent.parent().unwrap().join(&file),
            "{}: module",
            x.path
        );
        let text = fs::read_to_string(&parent).unwrap();
        let decl = format!("#[cfg(test)]\nmod {};\n", x.module);
        assert_eq!(
            text.matches(&decl).count(),
            1,
            "{}: cfg(test) declaration",
            x.path
        );
        let any_decl = format!("mod {}", x.module);
        assert_eq!(
            text.matches(&any_decl).count(),
            1,
            "{}: other declaration",
            x.path
        );
        // A whole path component, so `foo_tests.rs` does not name `tests.rs`.
        let named = [format!("\"{file}\""), format!("/{file}\"")];
        for (source, f) in &sources {
            let names = named.iter().any(|n| source.contains(n.as_str()));
            assert!(!names, "{f:?} names excluded {file}");
        }
    }
}

#[test]
fn coverage_gap_controls_reject_unknown_files_and_unused_exclusions() {
    let set = |s: &[&str]| s.iter().map(PathBuf::from).collect::<BTreeSet<_>>();
    let paths = |s: &[&str]| s.iter().map(PathBuf::from).collect::<Vec<_>>();
    let walked = set(&["/r/a.rs", "/r/b/c.rs", "/r/t.rs"]);
    let listed = set(&["/r/a.rs", "/r/b/c.rs", "/r/Cargo.toml"]);
    let excluded = set(&["/r/t.rs"]);
    assert_eq!(gaps(&walked, &listed, &excluded), (paths(&[]), paths(&[])));
    let mut future = walked.clone();
    future.insert("/r/b/new.rs".into());
    let unknown = gaps(&future, &listed, &excluded);
    assert_eq!(unknown, (paths(&["/r/b/new.rs"]), paths(&[])));
    let unexcluded = gaps(&walked, &listed, &set(&[]));
    assert_eq!(unexcluded, (paths(&["/r/t.rs"]), paths(&[])));
    let broad = set(&["/r", "/r/a.rs", "/r/gone.rs", "/r/t.rs"]);
    let unused = gaps(&walked, &listed, &broad);
    assert_eq!(
        unused,
        (paths(&[]), paths(&["/r", "/r/a.rs", "/r/gone.rs"]))
    );
}
