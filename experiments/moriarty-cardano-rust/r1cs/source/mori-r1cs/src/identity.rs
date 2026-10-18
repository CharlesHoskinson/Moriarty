//! Identities bound by a compiled relation.
//!
//! These identities stay separate and none is inferred from another:
//! - raw source bytes (`source_sha256`): provenance only, never bound in-circuit;
//! - the canonical typed Program (`program_hash`): SHA-256 of
//!   `intent::canonical(program)`, the digest Core's owner intent and stage digests use
//!   (`intent.rs:72-88`, `stage/digest.rs:20-22`); bound in-circuit;
//! - the semantic profile (`profile_digest`): Core language, semantics and
//!   logical-cost profile plus this relation format and version; bound in-circuit;
//! - the statement shape (`shape_digest`): the action plan and the slot layout; bound
//!   in-circuit;
//! - the compiler build ([`CompilerBuild`]): byte identity of this crate's source and of
//!   the exact Core source closure it links, recomputed from embedded bytes (P3S1-R1,
//!   [`crate::build_identity`]); bound in-circuit as **byte identity only**
//!   (OSR-1014-02). It names which bytes produced the relation, not what the relation
//!   means. A Core closure change (for example the incoming `pub mod sherlock;` line in
//!   `mori-core/src/lib.rs`) changes this digest even when every financial semantics file
//!   is unchanged, so no key or artefact is inherited across it.
//!
//! The R1CS matrix digest ([`crate::ProbeReport::r1cs_sha256`]) is computed by an actual
//! synthesis. It identifies the matrices, but it does not bind a verifying key by
//! itself: without a setup transcript nothing links a VK to them (X7 review §7, F2).
use crate::adapter::StatementAdapter;
use crate::error::CompileError;
use mori_core::{Program, UInt};
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digest32(pub [u8; 32]);

impl Digest32 {
    /// Parses exactly 64 lowercase hexadecimal characters.
    pub fn from_hex(text: &str) -> Result<Self, CompileError> {
        let bytes = text.as_bytes();
        if bytes.len() != 64 {
            return Err(CompileError::Identity(format!("digest must be 64 hex characters, got {}", bytes.len())));
        }
        let mut out = [0u8; 32];
        for (i, pair) in bytes.chunks(2).enumerate() {
            let hi = nibble(pair[0])?;
            let lo = nibble(pair[1])?;
            out[i] = (hi << 4) | lo;
        }
        Ok(Self(out))
    }

    pub fn hex(&self) -> String {
        mori_core::intent::hex(&self.0)
    }

    /// Big-endian halves `(hi, lo)`: `hi` is bytes 0..16 and `lo` bytes 16..32.
    /// Both are below 2^128, hence below any accepted field modulus: no alias.
    pub fn halves(&self) -> (u128, u128) {
        let mut hi = [0u8; 16];
        let mut lo = [0u8; 16];
        hi.copy_from_slice(&self.0[..16]);
        lo.copy_from_slice(&self.0[16..]);
        (u128::from_be_bytes(hi), u128::from_be_bytes(lo))
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|b| *b == 0)
    }
}

impl Serialize for Digest32 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.hex())
    }
}

fn nibble(c: u8) -> Result<u8, CompileError> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        _ => Err(CompileError::Identity("digest must be lowercase hexadecimal".into())),
    }
}

/// SHA-256 of `intent::canonical(value)`, reusing Core's canonical encoder and hash.
pub fn digest_of<T: Serialize>(value: &T) -> Result<Digest32, CompileError> {
    let bytes = mori_core::intent::canonical(value).map_err(CompileError::Core)?;
    Ok(digest_bytes(&bytes))
}

/// SHA-256 of raw bytes through Core's `hash_bytes`.
pub fn digest_bytes(bytes: &[u8]) -> Digest32 {
    Digest32::from_hex(&mori_core::intent::hash_bytes(bytes)).expect("Core hash_bytes returns 64 lowercase hex characters")
}

/// Canonical typed-Program identity (equals `CoreSourceExportV1.program_hash` and the
/// owner-intent `program_hash` for the same Program).
pub fn program_hash(program: &Program) -> Result<Digest32, CompileError> {
    digest_of(program)
}

/// Raw source identity; never bound in-circuit by this compiler.
pub fn source_hash(source: &str) -> Digest32 {
    digest_bytes(source.as_bytes())
}

#[derive(Serialize)]
struct ProfileDescriptor<'a> {
    language: &'a str,
    semantics: &'a str,
    logical_cost_profile: &'a str,
    relation_format: &'a str,
    relation_version: u64,
}

/// Digest of the semantic profile a relation claims: Core language and semantics,
/// logical-cost profile and this compiler's relation format/version.
pub fn profile_digest() -> Result<Digest32, CompileError> {
    digest_of(&ProfileDescriptor {
        language: mori_core::LANGUAGE,
        semantics: mori_core::SEMANTICS,
        logical_cost_profile: mori_core::LOGICAL_COST_PROFILE,
        relation_format: crate::RELATION_FORMAT,
        relation_version: crate::RELATION_VERSION,
    })
}

/// Byte identity of the compiler build (P3S1-R1: byte-derived, see
/// [`crate::build_identity`]). The compiler recomputes it at run time from source bytes
/// embedded at compile time; a caller may only restate that value (review F-08).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CompilerBuild {
    /// SHA-256 of the canonical `mori-r1cs-build-manifest/2` document of this crate's
    /// Cargo.toml and src/ (no annotation fields; review R-04).
    pub compiler_source_manifest_sha256: Digest32,
    /// Core closure digest under the unchanged closure rule (`CORE-CLOSURE-PINS.json`
    /// `closure_sha256`): the exact mori-core/lexer/diagnostics/span bytes embedded.
    pub core_closure_sha256: Digest32,
}

impl CompilerBuild {
    /// The embedded byte-derived identity (the only one `compile` accepts).
    pub fn embedded() -> Result<Self, CompileError> {
        crate::build_identity::embedded_build()
    }

    /// Refuses a zero digest, then any value other than the embedded identity: an
    /// arbitrary caller-supplied build digest is never taken as provenance.
    pub fn checked(self) -> Result<Self, CompileError> {
        if self.compiler_source_manifest_sha256.is_zero() || self.core_closure_sha256.is_zero() {
            return Err(CompileError::Identity(
                "compiler build digests must be nonzero; use CompilerBuild::embedded()".into(),
            ));
        }
        if self != Self::embedded()? {
            return Err(CompileError::Identity(
                "caller-supplied compiler build identity differs from the embedded byte-derived identity".into(),
            ));
        }
        Ok(self)
    }

    /// `mori-r1cs-build-identity/2`: SHA-256 of the canonical
    /// `{format, compiler_source_manifest_sha256, core_closure_sha256}`.
    pub fn digest(&self) -> Result<Digest32, CompileError> {
        digest_of(&crate::build_identity::BuildDigestInputV2 {
            format: crate::build_identity::BUILD_IDENTITY_FORMAT,
            compiler_source_manifest_sha256: self.compiler_source_manifest_sha256,
            core_closure_sha256: self.core_closure_sha256,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProgramIdentity {
    pub language: String,
    pub semantics: String,
    pub program_hash: Digest32,
    /// Present only when the program was admitted from source text.
    pub source_sha256: Option<Digest32>,
}

/// Every identity a compiled relation names. Header components are bound in-circuit;
/// `source_sha256` is provenance only and is excluded from [`RelationIdentity::digest`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RelationIdentity {
    pub relation_format: String,
    pub relation_version: u64,
    pub language: String,
    pub semantics: String,
    pub logical_cost_profile: String,
    pub profile_digest: Digest32,
    pub program_hash: Digest32,
    pub source_sha256: Option<Digest32>,
    pub action: String,
    pub action_index: u16,
    pub shape_digest: Digest32,
    pub compiler_build: CompilerBuild,
    pub compiler_build_digest: Digest32,
    pub adapter: StatementAdapter,
    pub logical_work: UInt,
}

#[derive(Serialize)]
struct BoundIdentity<'a> {
    format: &'static str,
    relation_format: &'a str,
    relation_version: u64,
    profile_digest: Digest32,
    program_hash: Digest32,
    action: &'a str,
    action_index: u16,
    shape_digest: Digest32,
    compiler_build_digest: Digest32,
    adapter: StatementAdapter,
    logical_work: UInt,
}

impl RelationIdentity {
    /// Digest of the bound components only (never the raw source digest).
    pub fn digest(&self) -> Result<Digest32, CompileError> {
        digest_of(&BoundIdentity {
            format: "mori-r1cs-relation-identity/1",
            relation_format: &self.relation_format,
            relation_version: self.relation_version,
            profile_digest: self.profile_digest,
            program_hash: self.program_hash,
            action: &self.action,
            action_index: self.action_index,
            shape_digest: self.shape_digest,
            compiler_build_digest: self.compiler_build_digest,
            adapter: self.adapter,
            logical_work: self.logical_work,
        })
    }
}
