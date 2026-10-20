//! Nonproving host expectations for the linked producer/consumer seam.
//! The source fixture runs the real draft compiler and the real two-step
//! Machine witness (host work only). Frozen directories are synthetic: zero
//! binding lanes, placeholder keys and proofs, and caller-consistent claims.
//! Every consumer case except one refuses before `verify_linked_children`
//! is entered. The exception is the fully consistent synthetic directory:
//! it enters the sealed verifier, whose A102 prepare emits the layout
//! relation through the SDK IR and selects the real circuit degree with
//! `ir.k()`, then refuses because the synthetic claim differs from the
//! reconstructed claim. That refusal precedes key reconstruction and any
//! SRS access, so no SRS load, keygen, prove or stdlib verify runs in this
//! file; the suite is therefore not a no-k suite, and no case here is
//! cryptographic evidence.
use super::*;
use crate::sherlock_child::{
    CHILD_BINDING_VERSION, CHILD_FORMAT, CHILD_OPEN_OBLIGATIONS, CHILD_QUALIFICATION,
    CHILD_TRANSCRIPT,
};
use crate::sherlock_link::{
    GENESIS_ROOT_AUTHORITY, GenesisAnchor, LINK_EQUALITIES, LINK_FIELD_ENCODING,
    LINK_FORMAT, LINK_MANIFEST_DISCLOSURES, LINK_OPEN_OBLIGATIONS, LINK_PIN_AUTHORITY,
    LINK_PROVENANCE, LinkChild, RETURN_CHILD_BINDING_VERSION, RETURN_CHILD_FORMAT,
    RETURN_CHILD_QUALIFICATION, RETURN_OPEN_OBLIGATIONS, RETURN_SCOPE,
    RETURN_STATEMENT_FORMAT,
};
use crate::sherlock_machine::literal_bridge::{
    OPEN_OBLIGATIONS, PROFILE, RULE_CHARGE, RULE_START_LINE, RULE_TAG, SCOPE,
    STATEMENT_FORMAT,
};
use crate::sherlock_machine::return_publication_step::{
    RETURN_PUBLICATION_RULE_CHARGE, RETURN_PUBLICATION_RULE_START_LINE,
    RETURN_PUBLICATION_RULE_TAG,
};
use std::path::PathBuf;

const LINK: &str = "mori::backend::sherlock-link";

fn options() -> ProofOptions {
    // Never read: every case refuses before SRS access.
    ProofOptions { params_dir: PathBuf::new(), max_k: MAX_K }
}
fn refusal<T>(r: Result<T, Error>) -> Error {
    r.err().expect("expected a refusal")
}
fn fresh_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("moriarty-link-producer-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}

/// Zero binding lane, then each statement from its own layout.
fn instances(r: &LinkedRequests) -> (Vec<Fr>, Vec<Fr>) {
    let (a102, a80, _) = check_linked_requests(r).unwrap();
    let lanes = |s: Vec<u128>| -> Vec<Fr> {
        std::iter::once(Fr::from(0u64)).chain(s.into_iter().map(Fr::from)).collect()
    };
    (
        lanes(a102.statement(&r.a102.values).unwrap()),
        lanes(a80.statement(&r.a80.values).unwrap()),
    )
}

fn child_claim(r: &LinkedRequests) -> ChildClaim {
    let (layout, _, _) = check_linked_requests(r).unwrap();
    ChildClaim {
        format: CHILD_FORMAT.into(),
        binding: CHILD_BINDING_VERSION.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        statement_format: STATEMENT_FORMAT.into(),
        profile: PROFILE.into(),
        scope: SCOPE.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_start_line: RULE_START_LINE as u32,
        rule_tag: RULE_TAG as u32,
        charge: RULE_CHARGE as u32,
        capacities: layout.record(),
        identity: r.a102.identity.clone(),
        layout_hash: hex(&layout.layout_hash),
        relation_hash: hex(&layout.relation_hash),
        context_hash: layout.context_hex(),
        zkir_hash: sha(b"synthetic-a102-zkir"),
        adapter_source_sha256: sha(b"synthetic-a102-adapter"),
        circuit_k: 1,
        public_values: layout
            .statement(&r.a102.values)
            .unwrap()
            .iter()
            .map(u128::to_string)
            .collect(),
    }
}

fn return_claim(r: &LinkedRequests) -> ReturnChildClaim {
    let (_, layout, _) = check_linked_requests(r).unwrap();
    ReturnChildClaim {
        format: RETURN_CHILD_FORMAT.into(),
        binding: RETURN_CHILD_BINDING_VERSION.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        statement_format: RETURN_STATEMENT_FORMAT.into(),
        profile: PROFILE.into(),
        scope: RETURN_SCOPE.into(),
        ledger_revision: LEDGER_REVISION.into(),
        zkir_version: ZKIR_VERSION.into(),
        rule_start_line: RETURN_PUBLICATION_RULE_START_LINE as u32,
        rule_tag: RETURN_PUBLICATION_RULE_TAG as u32,
        charge: RETURN_PUBLICATION_RULE_CHARGE as u32,
        capacities: layout.record(),
        identity: r.a80.identity.clone(),
        // The A80 layout hashes are private to sherlock_link and are not
        // cross-checked by this consumer; synthetic digests suffice here.
        layout_hash: sha(b"synthetic-a80-layout"),
        relation_hash: sha(b"synthetic-a80-relation"),
        context_hash: layout.context_hex(),
        zkir_hash: sha(b"synthetic-a80-zkir"),
        adapter_source_sha256: sha(b"synthetic-a80-adapter"),
        carrier_source_sha256: sha(b"synthetic-carrier"),
        circuit_k: 1,
        public_values: layout
            .statement(&r.a80.values)
            .unwrap()
            .iter()
            .map(u128::to_string)
            .collect(),
    }
}

struct Synthetic {
    dir: PathBuf,
    requests: LinkedRequests,
    manifest: LinkManifest,
}

/// A frozen directory that is consistent at every host layer except that its
/// keys and proofs are placeholders.
fn build_synthetic(tag: &str) -> Synthetic {
    let (requests, shape, _record) = literal_seven_linked_requests().unwrap();
    let (a102_instance, a80_instance) = instances(&requests);
    let hexes = |i: &[Fr]| i.iter().map(field_hex).collect::<Vec<_>>();
    let files: Vec<(&str, Vec<u8>)> = vec![
        ("a102.claim.json", json_bytes(&child_claim(&requests)).unwrap()),
        ("a102.vk", vec![9u8; 8]),
        ("a102.proof", vec![7u8; 16]),
        ("a102.instance.json", json_bytes(&hexes(&a102_instance)).unwrap()),
        ("a80.claim.json", json_bytes(&return_claim(&requests)).unwrap()),
        ("a80.vk", vec![11u8; 8]),
        ("a80.proof", vec![13u8; 16]),
        ("a80.instance.json", json_bytes(&hexes(&a80_instance)).unwrap()),
    ];
    let entry = |name: &str, claim: &[u8], vk: &[u8], proof: &[u8], instance: &[u8]| {
        let a102 = name == "a102";
        let (context, zkir, adapter, carrier) = if a102 {
            let c = child_claim(&requests);
            (c.context_hash, c.zkir_hash, c.adapter_source_sha256, None)
        } else {
            let c = return_claim(&requests);
            (
                c.context_hash,
                c.zkir_hash,
                c.adapter_source_sha256,
                Some(c.carrier_source_sha256),
            )
        };
        LinkChild {
            name: name.into(),
            claim_file: format!("{name}.claim.json"),
            claim_sha256: sha(claim),
            vk_file: format!("{name}.vk"),
            vk_sha256: sha(vk),
            proof_file: format!("{name}.proof"),
            proof_sha256: sha(proof),
            instance_file: format!("{name}.instance.json"),
            instance_sha256: sha(instance),
            qualification: (if a102 { CHILD_QUALIFICATION } else { RETURN_CHILD_QUALIFICATION })
                .into(),
            claim_format: (if a102 { CHILD_FORMAT } else { RETURN_CHILD_FORMAT }).into(),
            binding: (if a102 { CHILD_BINDING_VERSION } else { RETURN_CHILD_BINDING_VERSION })
                .into(),
            instance_lanes: shape.instance_lanes,
            proof_bytes: proof.len() as u32,
            context_hash: context,
            zkir_hash: zkir,
            adapter_source_sha256: adapter,
            carrier_source_sha256: carrier,
            circuit_k: 1,
            rule_tag: if a102 { RULE_TAG as u32 } else { RETURN_PUBLICATION_RULE_TAG as u32 },
            charge: if a102 { RULE_CHARGE as u32 } else { RETURN_PUBLICATION_RULE_CHARGE as u32 },
            open_obligations: if a102 {
                OPEN_OBLIGATIONS
                    .iter()
                    .chain(&CHILD_OPEN_OBLIGATIONS)
                    .map(|s| s.to_string())
                    .collect()
            } else {
                RETURN_OPEN_OBLIGATIONS
                    .iter()
                    .chain(&LINK_OPEN_OBLIGATIONS)
                    .map(|s| s.to_string())
                    .collect()
            },
        }
    };
    let manifest = LinkManifest {
        format: LINK_FORMAT.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        field_encoding: LINK_FIELD_ENCODING.into(),
        provenance: LINK_PROVENANCE.into(),
        program_lanes: shape.program_lanes,
        state_lanes: shape.state_lanes,
        instance_lanes: shape.instance_lanes,
        offsets: shape.offsets().unwrap(),
        links: LINK_EQUALITIES.iter().map(|s| s.to_string()).collect(),
        children: vec![
            entry("a102", &files[0].1, &files[1].1, &files[2].1, &files[3].1),
            entry("a80", &files[4].1, &files[5].1, &files[6].1, &files[7].1),
        ],
        genesis: GenesisAnchor {
            height: 0,
            child: "a102".into(),
            state_offset: shape.offsets().unwrap().pre,
            state_lanes: shape.state_lanes,
            root_authority: GENESIS_ROOT_AUTHORITY.into(),
        },
        pin_authority: LINK_PIN_AUTHORITY.into(),
        open_obligations: LINK_OPEN_OBLIGATIONS.iter().map(|s| s.to_string()).collect(),
        disclosures: LINK_MANIFEST_DISCLOSURES.iter().map(|s| s.to_string()).collect(),
    };
    manifest.validate().unwrap();
    let dir = fresh_dir(tag);
    fs::create_dir(&dir).unwrap();
    for (name, bytes) in &files {
        fs::write(dir.join(name), bytes).unwrap();
    }
    fs::write(dir.join("link.json"), json_bytes(&manifest).unwrap()).unwrap();
    Synthetic { dir, requests, manifest }
}

fn rewrite(s: &Synthetic, name: &str, bytes: &[u8]) {
    fs::write(s.dir.join(name), bytes).unwrap();
}
fn rewrite_manifest(s: &Synthetic, manifest: &LinkManifest) {
    rewrite(s, "link.json", &json_bytes(manifest).unwrap());
}
/// Re-identify one child's four files in the manifest after a tamper, so the
/// case reaches the deeper layer under test instead of the digest layer.
fn refresh(s: &mut Synthetic, child: usize) {
    let name = s.manifest.children[child].name.clone();
    let read = |kind: &str| fs::read(s.dir.join(format!("{name}.{kind}"))).unwrap();
    let (claim, vk, proof, instance) =
        (read("claim.json"), read("vk"), read("proof"), read("instance.json"));
    let e = &mut s.manifest.children[child];
    e.claim_sha256 = sha(&claim);
    e.vk_sha256 = sha(&vk);
    e.proof_sha256 = sha(&proof);
    e.instance_sha256 = sha(&instance);
    e.proof_bytes = proof.len() as u32;
    let manifest = s.manifest.clone();
    rewrite_manifest(s, &manifest);
}
fn read_hexes(s: &Synthetic, name: &str) -> Vec<String> {
    serde_json::from_slice(&fs::read(s.dir.join(format!("{name}.instance.json"))).unwrap())
        .unwrap()
}
fn write_hexes(s: &Synthetic, name: &str, hexes: &[String]) {
    rewrite(s, &format!("{name}.instance.json"), &json_bytes(&hexes).unwrap());
}
/// Bump one instance lane and the matching claim public value together.
fn bump_lane(s: &mut Synthetic, child: usize, name: &str, lane: usize) {
    let mut hexes = read_hexes(s, name);
    let bumped = fr_from_canonical_hex(&hexes[lane]).unwrap() + Fr::from(1u64);
    hexes[lane] = field_hex(&bumped);
    write_hexes(s, name, &hexes);
    let raw = fs::read(s.dir.join(format!("{name}.claim.json"))).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    let cell = &mut value["public_values"][lane - 1];
    let n: u128 = cell.as_str().unwrap().parse().unwrap();
    *cell = serde_json::Value::String(
        n.checked_add(1).expect("fixture lane below u128 max").to_string(),
    );
    rewrite(s, &format!("{name}.claim.json"), &serde_json::to_vec(&value).unwrap());
    refresh(s, child);
}

#[test]
fn canonical_hex_roundtrips_and_refuses_noncanonical_forms() {
    for v in [0u128, 1, 7, u128::MAX] {
        let x = Fr::from(v);
        assert_eq!(fr_from_canonical_hex(&field_hex(&x)).unwrap(), x);
    }
    let mut wide = [0u8; 32];
    wide[16] = 1;
    let x = Fr::from_le_bytes(&wide).expect("2^128 is a valid scalar");
    assert_eq!(fr_from_canonical_hex(&field_hex(&x)).unwrap(), x);
    // Scalar 10 encodes as "0a" followed by zeros, so its uppercase form
    // genuinely differs; scalar 7 ("07" plus zeros) would not mutate.
    let good = field_hex(&Fr::from(10u64));
    let upper = good.to_uppercase();
    assert_ne!(upper, good, "uppercase mutation must change the encoding");
    for bad in [
        upper,
        format!("0x{good}"),
        good[..62].to_string(),
        format!("{good}00"),
        "zz".repeat(32),
        "ff".repeat(32), // above the scalar modulus
        String::new(),
    ] {
        assert_eq!(refusal(fr_from_canonical_hex(&bad)).code, LINK, "{bad}");
    }
}

#[test]
fn literal_seven_requests_are_derived_from_the_compiled_source() {
    let (r, shape, record) = literal_seven_linked_requests().unwrap();
    // The existing public-source capture pins this exact source digest.
    assert_eq!(
        record.source_sha256,
        "9dfa037ef19e4b0d5f0d803c96d43e27d02b6493edfc6030947ff1437257f62a"
    );
    assert_eq!(record.program_source_sha256, record.source_sha256);
    assert_eq!(record.k_source_sha256, K_SOURCE_SHA256);
    assert_eq!(r.a102.identity.program_source_sha256, record.source_sha256);
    assert_eq!(r.a102.identity.k_source_sha256, K_SOURCE_SHA256);
    assert_eq!(r.a80.identity, r.a102.identity);
    assert_eq!(check_linked_requests(&r).unwrap().2, shape);
    assert_eq!(
        shape.instance_lanes,
        shape.program_lanes + 2 * shape.state_lanes + 5
    );
    assert_eq!(
        (r.a102.values.rule_tag, r.a102.values.charge, r.a80.values.rule_tag, r.a80.values.charge),
        (3, 1, 0, 0)
    );
    assert_eq!(r.a102.values.post, r.a80.values.pre);
    assert_eq!(r.a102.values.program, r.a80.values.program);
    let (a102, a80) = instances(&r);
    check_link_lanes(&a102, &a80, &shape).unwrap();
}

#[test]
fn consumer_refuses_missing_extra_and_nonregular_entries() {
    let missing = build_synthetic("missing");
    fs::remove_file(missing.dir.join("a80.proof")).unwrap();
    let e = refusal(read_link_dir(&missing.dir, &missing.requests, &options()));
    assert_eq!(e.message, "Frozen link directory does not contain exactly the nine link files.");
    let extra = build_synthetic("extra");
    fs::write(extra.dir.join("notes.txt"), b"x").unwrap();
    let e = refusal(read_link_dir(&extra.dir, &extra.requests, &options()));
    assert_eq!(e.message, "Frozen link directory does not contain exactly the nine link files.");
    let other = build_synthetic("other-shape");
    let mut wrong_shape = other.manifest.clone();
    wrong_shape.program_lanes += 1;
    wrong_shape.instance_lanes += 1;
    wrong_shape.offsets = LinkShape {
        program_lanes: wrong_shape.program_lanes,
        state_lanes: wrong_shape.state_lanes,
        instance_lanes: wrong_shape.instance_lanes,
        a102_context: [0; 2],
        a80_context: [0; 2],
    }
    .offsets()
    .unwrap();
    wrong_shape.genesis.state_offset = wrong_shape.offsets.pre;
    for child in &mut wrong_shape.children {
        child.instance_lanes = wrong_shape.instance_lanes;
    }
    wrong_shape.validate().unwrap();
    rewrite_manifest(&other, &wrong_shape);
    let e = refusal(read_link_dir(&other.dir, &other.requests, &options()));
    assert_eq!(e.message, "link.json shape differs from the source-derived requests.");
    for s in [&missing, &extra, &other] {
        let _ = fs::remove_dir_all(&s.dir);
    }
}

#[cfg(unix)]
#[test]
fn consumer_refuses_a_symlinked_frozen_file() {
    let s = build_synthetic("symlink");
    let target = s.dir.join("a102.proof");
    let bytes = fs::read(&target).unwrap();
    let outside = fresh_dir("symlink-outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("proof"), &bytes).unwrap();
    fs::remove_file(&target).unwrap();
    std::os::unix::fs::symlink(outside.join("proof"), &target).unwrap();
    let e = refusal(read_link_dir(&s.dir, &s.requests, &options()));
    assert_eq!(e.code, LINK);
    assert!(e.message.contains("not a regular file"), "{}", e.message);
    let _ = fs::remove_dir_all(&s.dir);
    let _ = fs::remove_dir_all(&outside);
}

#[test]
fn consumer_refuses_digest_mismatch_swapped_children_and_empty_proof() {
    let flipped = build_synthetic("flipped-proof");
    rewrite(&flipped, "a102.proof", &[8u8; 16]);
    let e = refusal(read_link_dir(&flipped.dir, &flipped.requests, &options()));
    assert_eq!(e.message, "Frozen child bytes differ from link.json digests.");

    let swapped = build_synthetic("swapped");
    for kind in ["claim.json", "vk", "proof", "instance.json"] {
        let a = fs::read(swapped.dir.join(format!("a102.{kind}"))).unwrap();
        let b = fs::read(swapped.dir.join(format!("a80.{kind}"))).unwrap();
        rewrite(&swapped, &format!("a102.{kind}"), &b);
        rewrite(&swapped, &format!("a80.{kind}"), &a);
    }
    let e = refusal(read_link_dir(&swapped.dir, &swapped.requests, &options()));
    assert_eq!(e.code, LINK);

    let empty = build_synthetic("empty-proof");
    rewrite(&empty, "a102.proof", &[]);
    let e = refusal(read_link_dir(&empty.dir, &empty.requests, &options()));
    assert_eq!(e.code, "mori::backend::proof");
    assert_eq!(e.message, "Expected a nonempty bounded child proof.");
    for s in [&flipped, &swapped, &empty] {
        let _ = fs::remove_dir_all(&s.dir);
    }
}

#[test]
fn consumer_refuses_noncanonical_lane_claim_edit_and_public_value_edit() {
    let mut upper = build_synthetic("upper-lane");
    let mut hexes = read_hexes(&upper, "a102");
    // Deterministic noncanonical lane: an explicit uppercase letter, rather
    // than uppercasing an unobserved context lane that might be all digits.
    let mut mutated = hexes[1].clone();
    mutated.replace_range(0..1, "A");
    assert_ne!(mutated, hexes[1], "uppercase mutation must change the lane");
    hexes[1] = mutated;
    write_hexes(&upper, "a102", &hexes);
    refresh(&mut upper, 0);
    let e = refusal(read_link_dir(&upper.dir, &upper.requests, &options()));
    assert_eq!(e.code, LINK);
    assert_eq!(
        e.message,
        "Instance lane is not 64 lowercase hex of a canonical scalar."
    );

    let mut claim_edit = build_synthetic("claim-edit");
    let raw = fs::read(claim_edit.dir.join("a80.claim.json")).unwrap();
    let mut value: serde_json::Value = serde_json::from_slice(&raw).unwrap();
    value["circuit_k"] = serde_json::json!(2);
    rewrite(&claim_edit, "a80.claim.json", &serde_json::to_vec(&value).unwrap());
    refresh(&mut claim_edit, 1);
    let e = refusal(read_link_dir(&claim_edit.dir, &claim_edit.requests, &options()));
    assert_eq!(e.code, "mori::backend::claim");
    assert_eq!(e.message, "Frozen child claim differs from its link.json entry.");

    // A post lane changed in the instance only: link continuity still holds,
    // so the refusal must come from the claim/instance lane agreement.
    let mut lane_edit = build_synthetic("lane-edit");
    let post = lane_edit.manifest.offsets.post;
    let mut hexes = read_hexes(&lane_edit, "a80");
    hexes[post] = field_hex(&(fr_from_canonical_hex(&hexes[post]).unwrap() + Fr::from(1u64)));
    write_hexes(&lane_edit, "a80", &hexes);
    refresh(&mut lane_edit, 1);
    let e = refusal(read_link_dir(&lane_edit.dir, &lane_edit.requests, &options()));
    assert_eq!(e.message, "Claim public values differ from the child instance lanes.");
    for s in [&upper, &claim_edit, &lane_edit] {
        let _ = fs::remove_dir_all(&s.dir);
    }
}

#[test]
fn consumer_refuses_program_and_predecessor_tamper_at_lane_continuity() {
    let mut program = build_synthetic("program-tamper");
    let at = program.manifest.offsets.program;
    bump_lane(&mut program, 1, "a80", at);
    let e = refusal(read_link_dir(&program.dir, &program.requests, &options()));
    assert_eq!(e.message, "A102 and A80 ProgramBlock lanes differ.");

    let mut predecessor = build_synthetic("predecessor-tamper");
    let pre = predecessor.manifest.offsets.pre;
    bump_lane(&mut predecessor, 1, "a80", pre);
    let e = refusal(read_link_dir(&predecessor.dir, &predecessor.requests, &options()));
    assert_eq!(e.message, "A102 post lanes differ from A80 pre lanes.");

    // Wrong order: instances exchanged between roles (digests re-identified)
    // refuse no later than the claim/instance and lane-continuity layers.
    let mut order = build_synthetic("wrong-order");
    let (a102_hexes, a80_hexes) = (read_hexes(&order, "a102"), read_hexes(&order, "a80"));
    write_hexes(&order, "a102", &a80_hexes);
    write_hexes(&order, "a80", &a102_hexes);
    refresh(&mut order, 0);
    refresh(&mut order, 1);
    let e = refusal(read_link_dir(&order.dir, &order.requests, &options()));
    assert_eq!(e.code, LINK);
    for s in [&program, &predecessor, &order] {
        let _ = fs::remove_dir_all(&s.dir);
    }
}

#[test]
fn consistent_synthetic_directory_refuses_at_reconstructed_claim() {
    // Host-consistent at every layer of this consumer, so the case reaches
    // the sealed verifier. A102 prepare emits the layout relation and
    // selects ir.k() (SDK host cost selection), then the synthetic claim
    // (synthetic zkir/adapter digests, circuit_k 1) differs from the claim
    // reconstructed from the requests. No frozen key bytes are parsed, and
    // the refusal precedes key reconstruction and any SRS access.
    let s = build_synthetic("consistent");
    let e = refusal(read_link_dir(&s.dir, &s.requests, &options()));
    assert_eq!(e.code, "mori::backend::claim");
    assert_eq!(
        e.message,
        "Claim differs from the independently reconstructed A102 child statement."
    );
    let _ = fs::remove_dir_all(&s.dir);
}

#[test]
fn producer_refuses_bad_cap_and_existing_attempt_dir_before_native_work() {
    let (r, _shape, record) = literal_seven_linked_requests().unwrap();
    let dir = fresh_dir("producer-refusals");
    let e = refusal(prove_linked_to_attempt_dir(
        &r,
        &record,
        &dir,
        &ProofOptions { max_k: 0, ..options() },
    ));
    assert_eq!(e.code, "mori::backend::limit");
    assert!(!dir.exists(), "cap refusal must precede any output");
    fs::create_dir(&dir).unwrap();
    let e = refusal(prove_linked_to_attempt_dir(&r, &record, &dir, &options()));
    assert_eq!(e.code, "mori::backend::link-io");
    let _ = fs::remove_dir_all(&dir);
    let e = refusal(read_link_dir(
        &dir,
        &r,
        &ProofOptions { max_k: MAX_K + 1, ..options() },
    ));
    assert_eq!(e.code, "mori::backend::limit");
}

#[test]
fn producer_refuses_source_record_that_differs_from_request_identity() {
    let (r, _shape, record) = literal_seven_linked_requests().unwrap();
    for (tag, mut bad) in [
        ("identity-k", record.clone()),
        ("identity-reference", record.clone()),
        ("identity-program", record.clone()),
    ] {
        match tag {
            "identity-k" => bad.k_source_sha256 = sha(b"other-k-source"),
            "identity-reference" => {
                bad.reference_source_sha256 = sha(b"other-reference-source")
            }
            _ => bad.program_source_sha256 = sha(b"other-program-source"),
        }
        let dir = fresh_dir(tag);
        let e = refusal(prove_linked_to_attempt_dir(&r, &bad, &dir, &options()));
        assert_eq!(e.code, LINK, "{tag}");
        assert_eq!(
            e.message,
            "Link source record digests differ from the linked requests identity.",
            "{tag}"
        );
        assert!(!dir.exists(), "identity refusal must precede any output: {tag}");
    }
}

#[test]
fn consumer_refuses_oversized_frozen_file_at_the_read_bound() {
    let s = build_synthetic("oversized-proof");
    rewrite(&s, "a102.proof", &vec![0u8; MAX_PROOF_BYTES + 1]);
    let e = refusal(read_link_dir(&s.dir, &s.requests, &options()));
    assert_eq!(e.code, "mori::backend::limit");
    assert!(e.message.contains("exceeds its read bound"), "{}", e.message);
    let _ = fs::remove_dir_all(&s.dir);
}
