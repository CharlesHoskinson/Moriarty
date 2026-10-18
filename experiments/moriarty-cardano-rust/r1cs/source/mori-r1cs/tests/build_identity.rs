//! P3S1-R1 byte-derived build identity tests (review RR-11, FIX-PROPOSALS P-5). Source
//! only: never compiled or run.
//!
//! Expected values: `BUILD-MANIFEST-R2.json` (crate root, outside the membership) is the
//! R1 author's stdlib freeze of the same bytes (AUTH arithmetic), and the Core closure
//! digest is the pinned `CORE-CLOSURE-PINS.json` value f47318a2... of the compiler-repair
//! lineage. Equality shows byte identity of the build inputs only (OSR-1014-02).
mod common;

use common::SELF_MOVE;
use mori_r1cs::build_identity::{self, CORE_FILES, OWN_FILES};
use mori_r1cs::*;

const FROZEN_MANIFEST: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/BUILD-MANIFEST-R2.json"));
const PINNED_CORE_CLOSURE: &str = "f47318a26a4df0477e89962a9de0e9b48f12d01d0e9b4a8f5e74603beaa235ed";

#[test]
fn embedded_build_manifest_equals_frozen_bytes() {
    assert_eq!(build_identity::build_manifest_bytes().unwrap(), FROZEN_MANIFEST.to_vec());
    let expected = identity::digest_bytes(FROZEN_MANIFEST);
    assert_eq!(build_identity::build_manifest_digest().unwrap(), expected);
}

#[test]
fn embedded_core_closure_equals_pinned_closure() {
    assert_eq!(CORE_FILES.len(), 70);
    assert_eq!(build_identity::core_closure_digest().unwrap().hex(), PINNED_CORE_CLOSURE);
}

/// Membership rule: Cargo.toml and every `src/` module declared by `lib.rs`.
#[test]
fn every_declared_module_is_embedded() {
    let lib = OWN_FILES.iter().find(|f| f.path == "src/lib.rs").expect("lib.rs embedded");
    let text = std::str::from_utf8(lib.bytes).expect("utf8");
    let mut modules: Vec<String> = text
        .lines()
        .filter_map(|l| l.trim().strip_prefix("pub mod ").or_else(|| l.trim().strip_prefix("mod ")))
        .map(|rest| format!("src/{}.rs", rest.trim_end_matches(';')))
        .collect();
    modules.push("src/lib.rs".into());
    modules.push("Cargo.toml".into());
    modules.sort();
    let mut embedded: Vec<String> = OWN_FILES.iter().map(|f| f.path.to_string()).collect();
    embedded.sort();
    assert_eq!(embedded, modules);
}

#[test]
fn caller_supplied_build_identity_is_refused() {
    let mut opts = common::options("move", CapBinding::NotBound);
    opts.compiler = CompilerBuild { compiler_source_manifest_sha256: Digest32([0x5a; 32]), core_closure_sha256: Digest32([0xc0; 32]) };
    assert!(matches!(common::try_compile(SELF_MOVE, opts), Err(CompileError::Identity(_))));
}

#[test]
fn embedded_build_identity_is_bound_with_format_2() {
    let rel = common::compile_ok(SELF_MOVE, common::options("move", CapBinding::NotBound));
    let embedded = CompilerBuild::embedded().unwrap();
    assert_eq!(rel.identity().compiler_build, embedded);
    assert_eq!(rel.identity().compiler_build_digest, embedded.digest().unwrap());
    assert_eq!(build_identity::BUILD_IDENTITY_FORMAT, "mori-r1cs-build-identity/2");
}
