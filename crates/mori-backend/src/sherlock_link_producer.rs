//! Executable seam for the linked A102 -> A80 children: a source-authenticated
//! producer with per-child checkpoints, and a strict frozen-directory consumer.
//!
//! Producer: the first scoped fixture compiles the real LiteralSeven draft
//! source through `compile_draft_with_reference`, retains that exact
//! `Arc<Program>`, derives `LiteralSourceIdentity` from the SHA-256 of the
//! actual source, K and reference bytes compiled into this crate, and feeds the
//! Arc to `produce_linked_requests` (one real Machine: A102 Eval(Literal) ->
//! Return, then A80 Return -> StepDone). No literal is substituted for compiler
//! output and no digest is caller-asserted. Each child is key-reconstructed,
//! proved and checkpointed to its own attempt subdirectory before the next
//! child starts; only after both checkpoints does the sealed
//! `verify_linked_children` + `write_link_dir` export run. A checkpoint is
//! retained evidence of an expensive stage, never a verified link and never a
//! resume input: the export is always re-verified from the in-memory requests.
//! Failure preserves every completed file and writes a best-effort failure
//! marker with the phases actually reached.
//!
//! Consumer: `read_link_dir` reconstructs nothing from the frozen directory.
//! The caller supplies freshly source-derived `LinkedRequests`; the consumer
//! checks exact directory membership, capped regular-file reads, manifest
//! shape against the source-derived shape, file digests, canonical instance
//! lanes, claim/manifest agreement, claim/instance lane agreement and the
//! complete host lane continuity (contexts, one ProgramBlock, A102.post ==
//! A80.pre over every state lane, envelopes), in that order, before any SRS
//! access. Key authority is never the manifest: `verify_linked_children`
//! rebuilds both claims, instances and processed keys from the requests and
//! compares the frozen bytes before stdlib verification under the Poseidon
//! child transcript. Frozen key bytes are never parsed here: a truncated or
//! otherwise malformed key refuses by failing that bytewise comparison, with
//! no untrusted-byte key parser in this consumer.
//!
//! Scope and open premises, unchanged from `sherlock_link`: literal entry
//! only; the draft compiler confers no A admission and source-to-Program
//! correspondence is a premise of this fixture, not proved here; neither child
//! constrains Init, prepareA, projectA or full StepInv, and the other 38
//! dispatch rows remain open; every host link equality still needs the
//! reviewed parent to constrain the same lanes in circuit. Cross-crate byte
//! compatibility is empirical and unobserved: this backend pins
//! midnight-proofs 0.8.3 while the Fenrir recl9 consumer pins 0.8.2 with
//! circuit-params/committed-instances and a patched circuits graph.
//! Compile/keygen/prove/verify of this module: UNOBSERVED.
use crate::proving::{MAX_K, MAX_PROOF_BYTES, ProofOptions, hex};
use crate::sherlock_child::{ChildClaim, ChildKey, SherlockChildProof, field_hex};
use crate::sherlock_link::{
    LinkChild, LinkManifest, LinkShape, LinkVerification, LinkedChildren, LinkedRequests,
    ReturnChildClaim, ReturnChildKey, ReturnChildProof, check_link_lanes,
    check_linked_requests, produce_linked_requests, verify_linked_children, write_link_dir,
};
use crate::sherlock_machine::literal_host::{LiteralCapacities, LiteralSourceIdentity};
use crate::{Error, LEDGER_REVISION, ZKIR_VERSION, error};
use midnight_transient_crypto::curve::Fr;
use mori_core::sherlock::K_SOURCE_SHA256;
use mori_core::sherlock_compile_artifact::draft_reference::compile_draft_with_reference;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::{Read, Write};
use std::{fs, path::Path, sync::Arc, time::Instant};

pub const LINK_RUN_FORMAT: &str = "moriarty-sherlock-link-run/1";
pub const LINK_FAILURE_FORMAT: &str = "moriarty-sherlock-link-failure/1";
pub const LINK_CHECKPOINT_FORMAT: &str = "moriarty-sherlock-link-checkpoint/1";
pub const LINK_PRODUCER_QUALIFICATION: &str =
    "scoped A102 Eval(Literal)->Return + A80 Return->StepDone producer/consumer run; \
     not parent recursion, terminal validity, ledger acceptance or full-39 coverage";

const FIXTURE_SOURCE: &str =
    include_str!("sherlock_machine/fixtures/literal_complete_capture_literal-seven.mori");
const FIXTURE_CONFIG: &str =
    include_str!("sherlock_machine/fixtures/literal_complete_capture_draft-config.json");
const FIXTURE_TARGET: &str =
    include_str!("sherlock_machine/fixtures/literal_complete_capture_target.json");
const K_SOURCE: &[u8] = include_bytes!("../../../models/sherlock/k/main.k");
const REFERENCE_SOURCE: &[u8] =
    include_bytes!("../../mori-core/src/sherlock/reference.rs");
// The exact bounded profile under which the existing public-source capture
// drives this same two-step witness producer from the retained Arc.
const FIXTURE_FUEL: u128 = 1;
const FIXTURE_STACK_LIMIT: usize = 0;

fn fixture_capacities() -> LiteralCapacities {
    LiteralCapacities {
        program: [0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 4, 4, 5, 1, 0],
        state: [0, 0, 0, 0, 0, 1],
        stack_limit: FIXTURE_STACK_LIMIT as u128,
    }
}

fn sha(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}
fn json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>, Error> {
    serde_json::to_vec(value).map_err(|e| error("serialization", e.to_string()))
}
fn write_new(dir: &Path, name: &str, bytes: &[u8]) -> Result<String, Error> {
    let io = |e: std::io::Error| error("link-io", e.to_string());
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(name))
        .map_err(io)?;
    file.write_all(bytes).and_then(|_| file.sync_all()).map_err(io)?;
    Ok(sha(bytes))
}

/// One canonical instance lane: exactly 64 lowercase hex characters of the
/// 32-byte little-endian scalar, refused above the modulus, and the decoded
/// value must re-encode to the identical text. Mirrors the Fenrir consumer's
/// canonical-hex rule; uppercase and `0x` forms are refused here.
pub fn fr_from_canonical_hex(text: &str) -> Result<Fr, Error> {
    let refuse = || {
        error(
            "sherlock-link",
            "Instance lane is not 64 lowercase hex of a canonical scalar.",
        )
    };
    if text.len() != 64
        || !text.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err(refuse());
    }
    let mut bytes = [0u8; 32];
    for (i, byte) in bytes.iter_mut().enumerate() {
        *byte = u8::from_str_radix(&text[2 * i..2 * i + 2], 16).map_err(|_| refuse())?;
    }
    let value = Fr::from_le_bytes(&bytes).ok_or_else(refuse)?;
    if field_hex(&value) != text {
        return Err(refuse());
    }
    Ok(value)
}

/// Canonical decimal for a u128 statement lane: digits only, no sign, no
/// leading zero except the value zero itself, in range.
fn parse_canonical_u128(text: &str) -> Result<u128, Error> {
    let refuse = || {
        error("sherlock-link", "Claim public value is not a canonical u128 decimal.")
    };
    if text.is_empty()
        || !text.bytes().all(|b| b.is_ascii_digit())
        || (text.len() > 1 && text.starts_with('0'))
    {
        return Err(refuse());
    }
    text.parse::<u128>().map_err(|_| refuse())
}

/// The claim's decimal statement must reproduce instance lanes 1.. exactly.
/// Lane 0 (the binding) is not checked here: its authority is the claim and
/// instance reconstruction inside `verify_linked_children`.
fn check_public_values(public_values: &[String], instance: &[Fr]) -> Result<(), Error> {
    if instance.len() != public_values.len() + 1 {
        return Err(error(
            "sherlock-link",
            "Claim public values do not cover the child instance statement.",
        ));
    }
    for (text, lane) in public_values.iter().zip(&instance[1..]) {
        if Fr::from(parse_canonical_u128(text)?) != *lane {
            return Err(error(
                "sherlock-link",
                "Claim public values differ from the child instance lanes.",
            ));
        }
    }
    Ok(())
}

/// Exact source bytes and bounded profile behind one fixture run. Digests are
/// computed from the compiled-in bytes at call time, never supplied.
#[derive(Clone, Debug, Serialize)]
pub struct LinkSourceRecord {
    pub source_sha256: String,
    pub config_sha256: String,
    pub target_sha256: String,
    pub k_source_sha256: String,
    pub reference_source_sha256: String,
    pub program_source_sha256: String,
    pub fuel: String,
    pub stack_limit: usize,
    pub capacities_program: Vec<u64>,
    pub capacities_state: Vec<u64>,
    pub capacities_stack_limit: String,
}

/// Host only: compile the LiteralSeven draft source and run the real
/// two-step Machine witness on the retained `Arc<Program>`. Refuses if the
/// compiled-in K bytes differ from `K_SOURCE_SHA256` or if the compiler's own
/// source digest differs from the digest of the compiled-in source bytes.
pub fn literal_seven_linked_requests(
) -> Result<(LinkedRequests, LinkShape, LinkSourceRecord), Error> {
    let k_source_sha256 = sha(K_SOURCE);
    if k_source_sha256 != K_SOURCE_SHA256 {
        return Err(error(
            "sherlock-link",
            "Compiled-in K source differs from mori-core K_SOURCE_SHA256.",
        ));
    }
    let prepared = compile_draft_with_reference(FIXTURE_SOURCE, FIXTURE_CONFIG, FIXTURE_TARGET)
        .map_err(|e| {
            error(
                "sherlock-link",
                format!(
                    "Draft source preparation refused at stage {}: {}",
                    e.stage, e.actual_error
                ),
            )
        })?;
    let source_sha256 = sha(FIXTURE_SOURCE.as_bytes());
    if prepared.preparation().source.source_digest != source_sha256 {
        return Err(error(
            "sherlock-link",
            "Compiler source digest differs from the compiled-in source bytes.",
        ));
    }
    let identity = LiteralSourceIdentity {
        k_source_sha256: k_source_sha256.clone(),
        reference_source_sha256: sha(REFERENCE_SOURCE),
        program_source_sha256: source_sha256.clone(),
    };
    let capacities = fixture_capacities();
    let (requests, shape) = produce_linked_requests(
        Arc::clone(prepared.reference_program()),
        &capacities,
        FIXTURE_FUEL,
        FIXTURE_STACK_LIMIT,
        &identity,
    )?;
    let record = LinkSourceRecord {
        config_sha256: sha(FIXTURE_CONFIG.as_bytes()),
        target_sha256: sha(FIXTURE_TARGET.as_bytes()),
        reference_source_sha256: identity.reference_source_sha256.clone(),
        program_source_sha256: source_sha256.clone(),
        source_sha256,
        k_source_sha256,
        fuel: FIXTURE_FUEL.to_string(),
        stack_limit: FIXTURE_STACK_LIMIT,
        capacities_program: capacities
            .program
            .iter()
            .map(|n| u64::try_from(*n).unwrap_or(u64::MAX))
            .collect(),
        capacities_state: capacities
            .state
            .iter()
            .map(|n| u64::try_from(*n).unwrap_or(u64::MAX))
            .collect(),
        capacities_stack_limit: capacities.stack_limit.to_string(),
    };
    Ok((requests, shape, record))
}

#[derive(Clone, Debug, Serialize)]
pub struct LinkPhaseTiming {
    pub phase: &'static str,
    pub millis: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct LinkChildRecord {
    pub name: &'static str,
    pub circuit_k: u8,
    pub context_hash: String,
    pub vk_sha256: String,
    pub instance_lanes: usize,
    pub proof_bytes: u32,
}

/// Terminal record of one successful producer attempt. Written last as
/// `run-report.json`; its presence is not acceptance by any consumer.
#[derive(Clone, Debug, Serialize)]
pub struct LinkRunReport {
    pub format: &'static str,
    pub qualification: &'static str,
    pub ledger_revision: String,
    pub zkir_version: String,
    pub source: LinkSourceRecord,
    pub program_lanes: usize,
    pub state_lanes: usize,
    pub instance_lanes: usize,
    pub children: Vec<LinkChildRecord>,
    pub phases: Vec<LinkPhaseTiming>,
    /// Export subdirectory, relative to the attempt directory.
    pub link_dir: &'static str,
}

#[derive(Serialize)]
struct LinkFailure<'a> {
    format: &'static str,
    qualification: &'static str,
    error_code: &'a str,
    error_message: &'a str,
    phases_reached: &'a [LinkPhaseTiming],
}

#[derive(Serialize)]
struct LinkCheckpoint<'a> {
    format: &'static str,
    child: &'a str,
    claim_sha256: String,
    vk_sha256: String,
    proof_sha256: String,
    instance_sha256: String,
}

fn timed<T>(
    phases: &mut Vec<LinkPhaseTiming>,
    phase: &'static str,
    work: impl FnOnce() -> Result<T, Error>,
) -> Result<T, Error> {
    eprintln!("[sherlock-link] {phase}: start");
    let start = Instant::now();
    let out = work();
    let millis = u64::try_from(start.elapsed().as_millis()).unwrap_or(u64::MAX);
    eprintln!(
        "[sherlock-link] {phase}: {} in {millis} ms",
        if out.is_ok() { "done" } else { "FAILED" }
    );
    phases.push(LinkPhaseTiming { phase, millis });
    out
}

/// Persist one proved child with the same per-file bytes the sealed export
/// writes (claim JSON, processed VK, proof, instance hex JSON), marker last.
fn write_child_checkpoint(
    dir: &Path,
    name: &str,
    claim: &[u8],
    vk: &[u8],
    proof: &[u8],
    instance: &[u8],
) -> Result<(), Error> {
    fs::create_dir(dir).map_err(|e| error("link-io", e.to_string()))?;
    let marker = LinkCheckpoint {
        format: LINK_CHECKPOINT_FORMAT,
        child: name,
        claim_sha256: write_new(dir, "claim.json", claim)?,
        vk_sha256: write_new(dir, "vk", vk)?,
        proof_sha256: write_new(dir, "proof", proof)?,
        instance_sha256: write_new(dir, "instance.json", instance)?,
    };
    write_new(dir, "checkpoint.json", &json_bytes(&marker)?)?;
    Ok(())
}

fn write_failure_marker(attempt_dir: &Path, phases: &[LinkPhaseTiming], e: &Error) {
    let marker = LinkFailure {
        format: LINK_FAILURE_FORMAT,
        qualification: LINK_PRODUCER_QUALIFICATION,
        error_code: &e.code,
        error_message: &e.message,
        phases_reached: phases,
    };
    if let Ok(bytes) = json_bytes(&marker) {
        let _ = write_new(attempt_dir, "failure.json", &bytes);
    }
}

/// Native producer over caller-derived requests. `attempt_dir` must not
/// exist, and `source` must carry the same three identity digests as both
/// requests or the call refuses before any output or native work, so the
/// run report cannot mislabel the requests' provenance. Phases, in order:
/// a102 key reconstruction, a102 prove, a102
/// checkpoint, a80 key reconstruction, a80 prove, a80 checkpoint, sealed
/// independent verification, sealed export into `attempt_dir/link`, then
/// `run-report.json`. Each key is dropped before the next child so at most
/// one proving key is resident. On any failure the attempt directory and
/// every completed checkpoint are preserved and a failure marker is written
/// best-effort; nothing is retried or removed here.
pub fn prove_linked_to_attempt_dir(
    requests: &LinkedRequests,
    source: &LinkSourceRecord,
    attempt_dir: &Path,
    options: &ProofOptions,
) -> Result<LinkRunReport, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    for identity in [&requests.a102.identity, &requests.a80.identity] {
        if source.k_source_sha256 != identity.k_source_sha256
            || source.reference_source_sha256 != identity.reference_source_sha256
            || source.program_source_sha256 != identity.program_source_sha256
        {
            return Err(error(
                "sherlock-link",
                "Link source record digests differ from the linked requests identity.",
            ));
        }
    }
    let (_, _, shape) = check_linked_requests(requests)?;
    fs::create_dir(attempt_dir).map_err(|e| error("link-io", e.to_string()))?;
    let mut phases = Vec::new();
    match prove_attempt(requests, source, shape, attempt_dir, options, &mut phases) {
        Ok(report) => Ok(report),
        Err(e) => {
            write_failure_marker(attempt_dir, &phases, &e);
            Err(e)
        }
    }
}

fn prove_attempt(
    requests: &LinkedRequests,
    source: &LinkSourceRecord,
    shape: LinkShape,
    attempt_dir: &Path,
    options: &ProofOptions,
    phases: &mut Vec<LinkPhaseTiming>,
) -> Result<LinkRunReport, Error> {
    let a102_key = timed(phases, "a102-key-reconstruct", || {
        ChildKey::reconstruct(&requests.a102, options)
    })?;
    let a102 = timed(phases, "a102-prove", || a102_key.prove())?;
    drop(a102_key);
    timed(phases, "a102-checkpoint", || {
        write_child_checkpoint(
            &attempt_dir.join("a102.checkpoint"),
            "a102",
            &json_bytes(&a102.claim)?,
            &a102.vk_bytes,
            &a102.proof,
            &json_bytes(&a102.instance_hex())?,
        )
    })?;
    let a80_key = timed(phases, "a80-key-reconstruct", || {
        ReturnChildKey::reconstruct(&requests.a80, options)
    })?;
    let a80 = timed(phases, "a80-prove", || a80_key.prove())?;
    drop(a80_key);
    timed(phases, "a80-checkpoint", || {
        write_child_checkpoint(
            &attempt_dir.join("a80.checkpoint"),
            "a80",
            &json_bytes(&a80.claim)?,
            &a80.vk_bytes,
            &a80.proof,
            &json_bytes(&a80.instance_hex())?,
        )
    })?;
    let children = LinkedChildren { a102, a80 };
    let verification = timed(phases, "verify-linked-children", || {
        verify_linked_children(requests, &children, options)
    })?;
    timed(phases, "write-link-dir", || {
        write_link_dir(&attempt_dir.join("link"), requests, &children, &verification)
    })?;
    let report = LinkRunReport {
        format: LINK_RUN_FORMAT,
        qualification: LINK_PRODUCER_QUALIFICATION,
        ledger_revision: LEDGER_REVISION.to_string(),
        zkir_version: ZKIR_VERSION.to_string(),
        source: source.clone(),
        program_lanes: shape.program_lanes,
        state_lanes: shape.state_lanes,
        instance_lanes: shape.instance_lanes,
        children: vec![
            LinkChildRecord {
                name: "a102",
                circuit_k: verification.a102().circuit_k,
                context_hash: verification.a102().context_hash.clone(),
                vk_sha256: verification.a102().vk_sha256.clone(),
                instance_lanes: verification.a102().instance_lanes,
                proof_bytes: verification.a102().proof_bytes,
            },
            LinkChildRecord {
                name: "a80",
                circuit_k: verification.a80().circuit_k,
                context_hash: verification.a80().context_hash.clone(),
                vk_sha256: verification.a80().vk_sha256.clone(),
                instance_lanes: verification.a80().instance_lanes,
                proof_bytes: verification.a80().proof_bytes,
            },
        ],
        phases: phases.clone(),
        link_dir: "link",
    };
    crate::proving::validate_json_profile(&report)?;
    write_new(attempt_dir, "run-report.json", &json_bytes(&report)?)?;
    Ok(report)
}

/// Native: the scoped LiteralSeven fixture producer. Recompiles the source
/// in this process; see `literal_seven_linked_requests`.
pub fn prove_literal_seven_link(
    attempt_dir: &Path,
    options: &ProofOptions,
) -> Result<LinkRunReport, Error> {
    let (requests, _shape, record) = literal_seven_linked_requests()?;
    prove_linked_to_attempt_dir(&requests, &record, attempt_dir, options)
}

/// One frozen link directory after independent verification. The manifest is
/// retained for byte identification only; authority is `verification`.
pub struct VerifiedLinkDir {
    pub manifest: LinkManifest,
    pub verification: LinkVerification,
}

const LINK_FILES: [&str; 9] = [
    "a102.claim.json",
    "a102.vk",
    "a102.proof",
    "a102.instance.json",
    "a80.claim.json",
    "a80.vk",
    "a80.proof",
    "a80.instance.json",
    "link.json",
];

fn read_capped(path: &Path, cap: usize) -> Result<Vec<u8>, Error> {
    let io = |e: std::io::Error| error("link-io", e.to_string());
    let meta = fs::symlink_metadata(path).map_err(io)?;
    if !meta.file_type().is_file() {
        return Err(error(
            "sherlock-link",
            format!("Frozen artifact {} is not a regular file.", path.display()),
        ));
    }
    if meta.len() > cap as u64 {
        return Err(error(
            "limit",
            format!("Frozen artifact {} exceeds its read bound.", path.display()),
        ));
    }
    // Bound the actual read, not only the earlier stat: a file that grows
    // after the length check can contribute at most cap + 1 bytes here.
    let limit = cap
        .checked_add(1)
        .and_then(|n| u64::try_from(n).ok())
        .ok_or_else(|| error("limit", "Frozen read bound overflows u64."))?;
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(io)?
        .take(limit)
        .read_to_end(&mut bytes)
        .map_err(io)?;
    if bytes.len() > cap {
        return Err(error(
            "limit",
            format!("Frozen artifact {} exceeds its read bound.", path.display()),
        ));
    }
    Ok(bytes)
}

fn check_exact_membership(dir: &Path) -> Result<(), Error> {
    let refuse = || {
        error(
            "sherlock-link",
            "Frozen link directory does not contain exactly the nine link files.",
        )
    };
    let mut names = Vec::new();
    for entry in fs::read_dir(dir).map_err(|e| error("link-io", e.to_string()))? {
        let entry = entry.map_err(|e| error("link-io", e.to_string()))?;
        names.push(entry.file_name().to_str().map(str::to_string).ok_or_else(refuse)?);
        if names.len() > LINK_FILES.len() {
            return Err(refuse());
        }
    }
    names.sort();
    let mut expected: Vec<String> = LINK_FILES.iter().map(|s| s.to_string()).collect();
    expected.sort();
    if names != expected {
        return Err(refuse());
    }
    Ok(())
}

fn decode_instance(raw: &[u8], lanes: usize) -> Result<Vec<Fr>, Error> {
    let text = std::str::from_utf8(raw).map_err(|e| error("json", e.to_string()))?;
    let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
    let hexes: Vec<String> =
        serde_json::from_value(value).map_err(|e| error("json", e.to_string()))?;
    if hexes.len() != lanes {
        return Err(error(
            "sherlock-link",
            "Instance lane count differs from the source-derived shape.",
        ));
    }
    hexes.iter().map(|h| fr_from_canonical_hex(h)).collect()
}

fn parse_json<T: serde::de::DeserializeOwned>(raw: &[u8]) -> Result<T, Error> {
    let text = std::str::from_utf8(raw).map_err(|e| error("json", e.to_string()))?;
    let value = mori_core::json::parse(text).map_err(|e| error("json", e.to_string()))?;
    serde_json::from_value(value).map_err(|e| error("json", e.to_string()))
}

struct RawChild {
    claim: Vec<u8>,
    vk: Vec<u8>,
    proof: Vec<u8>,
    instance: Vec<Fr>,
}

/// Read one child by its manifest-fixed filenames (intrinsic validation has
/// already pinned them), refuse an empty proof before digest work, identify
/// every file by its manifest digest, then decode the complete instance.
fn read_child_files(
    dir: &Path,
    entry: &LinkChild,
    lanes: usize,
    claim_cap: usize,
    instance_cap: usize,
) -> Result<RawChild, Error> {
    let claim = read_capped(&dir.join(&entry.claim_file), claim_cap)?;
    let vk = read_capped(&dir.join(&entry.vk_file), 1 << 20)?;
    let proof = read_capped(&dir.join(&entry.proof_file), MAX_PROOF_BYTES)?;
    let instance_raw = read_capped(&dir.join(&entry.instance_file), instance_cap)?;
    if proof.is_empty() {
        return Err(error("proof", "Expected a nonempty bounded child proof."));
    }
    if sha(&claim) != entry.claim_sha256
        || sha(&vk) != entry.vk_sha256
        || sha(&proof) != entry.proof_sha256
        || sha(&instance_raw) != entry.instance_sha256
        || proof.len() != entry.proof_bytes as usize
    {
        return Err(error(
            "sherlock-link",
            "Frozen child bytes differ from link.json digests.",
        ));
    }
    let instance = decode_instance(&instance_raw, lanes)?;
    Ok(RawChild { claim, vk, proof, instance })
}

fn check_claim_against_entry(
    entry: &LinkChild,
    format: &str,
    binding: &str,
    circuit_k: u8,
    context_hash: &str,
    zkir_hash: &str,
    adapter_source_sha256: &str,
    carrier_source_sha256: Option<&str>,
    rule_tag: u32,
    charge: u32,
) -> Result<(), Error> {
    if entry.claim_format != format
        || entry.binding != binding
        || entry.circuit_k != circuit_k
        || entry.context_hash != context_hash
        || entry.zkir_hash != zkir_hash
        || entry.adapter_source_sha256 != adapter_source_sha256
        || entry.carrier_source_sha256.as_deref() != carrier_source_sha256
        || entry.rule_tag != rule_tag
        || entry.charge != charge
    {
        return Err(error(
            "claim",
            "Frozen child claim differs from its link.json entry.",
        ));
    }
    Ok(())
}

/// Strict frozen-directory consumer. Every structural check runs before any
/// SRS access, in directory-membership, manifest, per-file digest, instance
/// decode, claim agreement, claim/instance lane agreement, complete lane
/// continuity order; only then does
/// `verify_linked_children` rebuild both relations, claims, instances and
/// keys from `requests`, compare the frozen bytes and verify both Poseidon
/// child proofs. The directory must be quiescent while read. Manifest
/// digests identify bytes only and pin nothing.
pub fn read_link_dir(
    dir: &Path,
    requests: &LinkedRequests,
    options: &ProofOptions,
) -> Result<VerifiedLinkDir, Error> {
    if options.max_k == 0 || options.max_k > MAX_K {
        return Err(error("limit", "Native k cap must be between 1 and 19."));
    }
    let (_, _, shape) = check_linked_requests(requests)?;
    check_exact_membership(dir)?;
    let lanes = shape.instance_lanes;
    let bound = |per_lane: usize, fixed: usize| {
        lanes
            .checked_mul(per_lane)
            .and_then(|n| n.checked_add(fixed))
            .ok_or_else(|| error("limit", "Frozen read bound overflows usize."))
    };
    // Parsing bounds only, derived from the source shape: one lane is at
    // most 66 instance-JSON bytes or a 39-digit claim decimal plus framing.
    let (claim_cap, instance_cap) = (bound(48, 65_536)?, bound(80, 64)?);
    let raw_manifest = read_capped(&dir.join("link.json"), 1 << 20)?;
    let manifest_text =
        std::str::from_utf8(&raw_manifest).map_err(|e| error("json", e.to_string()))?;
    let manifest = LinkManifest::from_json(manifest_text)?;
    if manifest.program_lanes != shape.program_lanes
        || manifest.state_lanes != shape.state_lanes
        || manifest.instance_lanes != lanes
        || manifest.offsets != shape.offsets()?
    {
        return Err(error(
            "sherlock-link",
            "link.json shape differs from the source-derived requests.",
        ));
    }
    // Intrinsic validation fixed exactly two children in a102, a80 order.
    let raw_a102 = read_child_files(dir, &manifest.children[0], lanes, claim_cap, instance_cap)?;
    let raw_a80 = read_child_files(dir, &manifest.children[1], lanes, claim_cap, instance_cap)?;
    let claim_a102: ChildClaim = parse_json(&raw_a102.claim)?;
    let claim_a80: ReturnChildClaim = parse_json(&raw_a80.claim)?;
    check_claim_against_entry(
        &manifest.children[0],
        &claim_a102.format,
        &claim_a102.binding,
        claim_a102.circuit_k,
        &claim_a102.context_hash,
        &claim_a102.zkir_hash,
        &claim_a102.adapter_source_sha256,
        None,
        claim_a102.rule_tag,
        claim_a102.charge,
    )?;
    check_claim_against_entry(
        &manifest.children[1],
        &claim_a80.format,
        &claim_a80.binding,
        claim_a80.circuit_k,
        &claim_a80.context_hash,
        &claim_a80.zkir_hash,
        &claim_a80.adapter_source_sha256,
        Some(&claim_a80.carrier_source_sha256),
        claim_a80.rule_tag,
        claim_a80.charge,
    )?;
    check_public_values(&claim_a102.public_values, &raw_a102.instance)?;
    check_public_values(&claim_a80.public_values, &raw_a80.instance)?;
    check_link_lanes(&raw_a102.instance, &raw_a80.instance, &shape)?;
    let children = LinkedChildren {
        a102: SherlockChildProof {
            claim: claim_a102,
            instance: raw_a102.instance,
            vk_bytes: raw_a102.vk,
            proof: raw_a102.proof,
        },
        a80: ReturnChildProof {
            claim: claim_a80,
            instance: raw_a80.instance,
            vk_bytes: raw_a80.vk,
            proof: raw_a80.proof,
        },
    };
    let verification = verify_linked_children(requests, &children, options)?;
    Ok(VerifiedLinkDir { manifest, verification })
}

/// Native independent consumer for the scoped fixture: recompiles the
/// source in this process, so no request, key or shape is taken from the
/// frozen directory or from any producer process.
pub fn verify_literal_seven_link_dir(
    dir: &Path,
    options: &ProofOptions,
) -> Result<VerifiedLinkDir, Error> {
    let (requests, _shape, _record) = literal_seven_linked_requests()?;
    read_link_dir(dir, &requests, options)
}

#[cfg(test)]
#[path = "sherlock_link_producer_normal_tests.rs"]
mod normal_tests;

#[cfg(test)]
#[path = "sherlock_link_producer_native_tests.rs"]
mod native_tests;
