//! Byte-derived compiler build identity (P3S1-R1 successor; review RR-11 and result
//! review FIX-PROPOSALS P-5). PROPOSED, unreviewed, never compiled or run.
//!
//! P3S1 took `CompilerBuild` from the caller (review F-08) and named a BUILD-MANIFEST whose
//! digest also covered annotation bytes (review R-04). This module replaces both, for this
//! successor only:
//!
//! - **Membership.** `OWN_FILES`: this crate's `Cargo.toml` and every file under `src/`
//!   (tests, fixtures and documents excluded, as in `/1`). `CORE_FILES`: the 70 files of
//!   the pinned Core closure (`CORE-CLOSURE-PINS.json`, rule unchanged), read from the
//!   isolated layout's `<root>/moriarty` copy that Cargo also compiles `mori-core` from.
//! - **Construction boundary.** Bytes are embedded at compile time with `include_bytes!`
//!   (no `build.rs`, no environment variable, no file read at run time). Digests are
//!   recomputed at run time from the embedded bytes. No caller input enters them.
//! - **Format** `mori-r1cs-build-manifest/2`: `{"files":[{"bytes":N,"path":P,"sha256":H}..],
//!   "format":"mori-r1cs-build-manifest/2"}`, entries sorted by path bytewise, no
//!   annotation field; `compiler_source_manifest_sha256` = SHA-256 of its canonical JSON
//!   (Core `intent::canonical`). The Core closure digest keeps its `/1` rule (SHA-256 of the
//!   canonical array of `{path, sha256, bytes}` sorted by path), so the pinned `f47318a2...`
//!   value is reproducible.
//! - **Build digest** `mori-r1cs-build-identity/2`: SHA-256 of the canonical JSON of
//!   `{format, compiler_source_manifest_sha256, core_closure_sha256}`.
//!
//! Trust limits (unchanged by any hash): this names the source bytes the compiler saw at
//! build time, not the binary that runs. A modified compiler, toolchain, linker or Cargo
//! source override can report any value; registry dependencies (arkworks, serde) are
//! pinned by the lock gate, not here; equal digests establish byte identity only, never
//! semantic equivalence, provenance of authorship, soundness or security (OSR-1014-02).
//! A catalogue author who pins a predicate digest must rebuild from the published bytes.
use crate::error::CompileError;
use crate::identity::{self, CompilerBuild, Digest32};
use serde::Serialize;

pub const BUILD_MANIFEST_FORMAT: &str = "mori-r1cs-build-manifest/2";
pub const BUILD_IDENTITY_FORMAT: &str = "mori-r1cs-build-identity/2";

/// One embedded member: its path relative to its root and its exact bytes.
#[derive(Debug, Clone, Copy)]
pub struct Embedded {
    pub path: &'static str,
    pub bytes: &'static [u8],
}

macro_rules! own {
    ($p:literal) => {
        Embedded { path: $p, bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/", $p)) }
    };
}

macro_rules! core_file {
    ($p:literal) => {
        Embedded { path: $p, bytes: include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../moriarty/", $p)) }
    };
}

/// This crate's members (`Cargo.toml` and every `src/` file, this one included).
pub const OWN_FILES: &[Embedded] = &[
    own!("Cargo.toml"),
    own!("src/abi2.rs"),
    own!("src/adapter.rs"),
    own!("src/blake2b.rs"),
    own!("src/build_identity.rs"),
    own!("src/circuit.rs"),
    own!("src/coverage.rs"),
    own!("src/error.rs"),
    own!("src/frontend.rs"),
    own!("src/gadgets.rs"),
    own!("src/groth16.rs"),
    own!("src/identity.rs"),
    own!("src/lib.rs"),
    own!("src/lower.rs"),
    own!("src/plan.rs"),
    own!("src/r1cs.rs"),
    own!("src/staged.rs"),
    own!("src/statement.rs"),
    own!("src/witness.rs"),
];

/// The pinned Core closure members (paths relative to the Moriarty checkout root).
pub const CORE_FILES: &[Embedded] = &[
    core_file!("Cargo.toml"),
    core_file!("crates/mori-core/Cargo.toml"),
    core_file!("crates/mori-core/src/intent.rs"),
    core_file!("crates/mori-core/src/ir.rs"),
    core_file!("crates/mori-core/src/json.rs"),
    core_file!("crates/mori-core/src/lib.rs"),
    core_file!("crates/mori-core/src/logical_cost.rs"),
    core_file!("crates/mori-core/src/parser.rs"),
    core_file!("crates/mori-core/src/policy.rs"),
    core_file!("crates/mori-core/src/replay.rs"),
    core_file!("crates/mori-core/src/runtime.rs"),
    core_file!("crates/mori-core/src/source_sponsor.rs"),
    core_file!("crates/mori-core/src/stage/admit.rs"),
    core_file!("crates/mori-core/src/stage/cost.rs"),
    core_file!("crates/mori-core/src/stage/decode.rs"),
    core_file!("crates/mori-core/src/stage/digest.rs"),
    core_file!("crates/mori-core/src/stage/mod.rs"),
    core_file!("crates/mori-core/src/stage/plan.rs"),
    core_file!("crates/mori-core/src/stage/profile.rs"),
    core_file!("crates/mori-core/src/stage/validate.rs"),
    core_file!("crates/mori-diagnostics/Cargo.toml"),
    core_file!("crates/mori-diagnostics/src/diagnostic.rs"),
    core_file!("crates/mori-diagnostics/src/lib.rs"),
    core_file!("crates/mori-diagnostics/src/render.rs"),
    core_file!("crates/mori-diagnostics/src/snapshots/mori_diagnostics__render__tests__error_with_code_label_and_help.snap"),
    core_file!("crates/mori-diagnostics/src/snapshots/mori_diagnostics__render__tests__warning_without_labels.snap"),
    core_file!("crates/mori-diagnostics/src/suggest.rs"),
    core_file!("crates/mori-lexer/Cargo.toml"),
    core_file!("crates/mori-lexer/src/diagnostics.rs"),
    core_file!("crates/mori-lexer/src/lib.rs"),
    core_file!("crates/mori-lexer/src/number.rs"),
    core_file!("crates/mori-lexer/src/proptests.rs"),
    core_file!("crates/mori-lexer/src/scan.rs"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__agreement_skeleton.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__comments_are_kept_separately.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__curly_quotes.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__division.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__empty_source.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__every_punctuation_symbol.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__identifier_too_long.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__keywords_and_identifiers.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__line_comment_stops_at_carriage_return.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__name_starting_with_underscore.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__non_ascii_name.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_touching_asset_is_two_tokens.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_with_double_underscore.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_with_leading_zero.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_with_trailing_point.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_with_trailing_underscore.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_with_two_points.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__number_with_underscore_after_point.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__numbers.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__qualified_call_name.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__single_quotes.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__string_syntax_error_wins_over_lone_surrogate.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__string_with_invalid_escape.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__string_with_line_break.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__string_with_lone_surrogate.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__string_with_short_unicode_escape.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__string_with_tab_character.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__strings.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__unknown_symbol.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__unterminated_block_comment.snap"),
    core_file!("crates/mori-lexer/src/snapshots/mori_lexer__tests__unterminated_string.snap"),
    core_file!("crates/mori-lexer/src/string.rs"),
    core_file!("crates/mori-lexer/src/tests.rs"),
    core_file!("crates/mori-lexer/src/token.rs"),
    core_file!("crates/mori-span/Cargo.toml"),
    core_file!("crates/mori-span/src/lib.rs"),
    core_file!("rust-toolchain.toml"),
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Entry {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}

#[derive(Serialize)]
struct BuildManifestV2 {
    format: &'static str,
    files: Vec<Entry>,
}

#[derive(Serialize)]
pub(crate) struct BuildDigestInputV2 {
    pub(crate) format: &'static str,
    pub(crate) compiler_source_manifest_sha256: Digest32,
    pub(crate) core_closure_sha256: Digest32,
}

/// Entries sorted by path bytewise; a repeated path is refused.
pub fn entries(files: &[Embedded]) -> Result<Vec<Entry>, CompileError> {
    let mut out: Vec<Entry> = files
        .iter()
        .map(|f| Entry { path: f.path.to_string(), sha256: identity::digest_bytes(f.bytes).hex(), bytes: f.bytes.len() as u64 })
        .collect();
    out.sort_by(|a, b| a.path.as_bytes().cmp(b.path.as_bytes()));
    if out.windows(2).any(|w| w[0].path == w[1].path) {
        return Err(CompileError::Identity("embedded build membership repeats a path".into()));
    }
    Ok(out)
}

/// Canonical bytes of the `mori-r1cs-build-manifest/2` document for `OWN_FILES`.
pub fn build_manifest_bytes() -> Result<Vec<u8>, CompileError> {
    mori_core::intent::canonical(&BuildManifestV2 { format: BUILD_MANIFEST_FORMAT, files: entries(OWN_FILES)? }).map_err(CompileError::Core)
}

/// SHA-256 of the canonical build manifest.
pub fn build_manifest_digest() -> Result<Digest32, CompileError> {
    Ok(identity::digest_bytes(&build_manifest_bytes()?))
}

/// The Core closure digest under the unchanged `/1` closure rule.
pub fn core_closure_digest() -> Result<Digest32, CompileError> {
    identity::digest_of(&entries(CORE_FILES)?)
}

/// The only `CompilerBuild` that `compile` accepts.
pub fn embedded_build() -> Result<CompilerBuild, CompileError> {
    Ok(CompilerBuild { compiler_source_manifest_sha256: build_manifest_digest()?, core_closure_sha256: core_closure_digest()? })
}
