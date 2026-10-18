//! Tier A: pure codec and strict-input tests. No compilation, no constraint synthesis, no
//! key, no proof, no pairing. Source only: never compiled or run; every assertion is a
//! prediction. Grades are in `tests/common/mod.rs` and `EXPECTATIONS.json`.
mod common;

use ark_bls12_381::{Bls12_381, Fr, G1Affine, G2Affine};
use ark_ec::{AffineRepr, CurveGroup};
use ark_groth16::{Proof, VerifyingKey};
use common::*;
use mori_k2_driver::claim::{self, PreparedClaim};
use mori_k2_driver::encoding::{self, PROOF_LEN, VK_LEN};
use mori_k2_driver::error::DriverError;
use mori_k2_driver::export::{self, ParsedSetup, ProofExport, SetupRecord};
use mori_k2_driver::hexutil;
use mori_k2_driver::input::{self, ContextInput};
use mori_k2_driver::relation::{self, RelationDescriptor};
use mori_r1cs::{BodyValues, TransitionValues};

// ## Hex, decimal and strict JSON

#[test]
fn a01_hex_is_strict_lowercase_and_exact() {
    assert_eq!(hexutil::decode_exact::<4>("deadbeef", "t").unwrap(), [0xde, 0xad, 0xbe, 0xef]);
    for bad in ["DEADBEEF", "deadbee", "0xdeadbe", "deadbe", "deadbeef00", "dead beef"] {
        assert!(hexutil::decode_exact::<4>(bad, "t").is_err(), "{bad:?} must be refused");
    }
    assert!(hexutil::decode_bounded("00".repeat(5).as_str(), 4, "t").is_err(), "over the byte bound");
}

#[test]
fn a02_decimal_is_canonical_u128() {
    assert_eq!(hexutil::parse_u128_decimal("0", "t").unwrap(), 0);
    assert_eq!(hexutil::parse_u128_decimal("340282366920938463463374607431768211455", "t").unwrap(), u128::MAX);
    for bad in ["", "007", "-1", "+1", "1.0", " 1", "340282366920938463463374607431768211456"] {
        assert!(hexutil::parse_u128_decimal(bad, "t").is_err(), "{bad:?} must be refused");
    }
}

#[test]
fn a03_selector_duplicate_key_unknown_field_trailing_data_refused() {
    let good = SEL_INVOICE.trim_end();
    assert!(input::parse_selector(good.as_bytes()).is_ok());
    let duplicate = good.replacen("\"action\": \"partial\",", "\"action\": \"partial\", \"action\": \"open\",", 1);
    assert_ne!(duplicate, good);
    assert!(matches!(input::parse_selector(duplicate.as_bytes()), Err(DriverError::Input(_))));
    let unknown = good.replacen("\"action\": \"partial\",", "\"action\": \"partial\", \"hint\": 1,", 1);
    assert!(matches!(input::parse_selector(unknown.as_bytes()), Err(DriverError::Input(_))));
    let trailing = format!("{good} {{}}");
    assert!(matches!(input::parse_selector(trailing.as_bytes()), Err(DriverError::Input(_))));
}

#[test]
fn a04_oversize_and_non_utf8_refused() {
    let big = vec![b' '; input::MAX_JSON_BYTES + 1];
    assert!(matches!(input::parse_selector(&big), Err(DriverError::Input(_))));
    assert!(matches!(input::parse_selector(&[0xff, 0xfe, b'{', b'}']), Err(DriverError::Input(_))));
}

#[test]
fn a05_request_format_and_transition_kind_are_required() {
    let wrong = REQ_INVOICE.replacen("moriarty-k2-prove-request/1", "moriarty-k2-prove-request/2", 1);
    assert!(matches!(input::parse_request(wrong.as_bytes()), Err(DriverError::Unsupported(_))));
    let no_kind = REQ_INVOICE.replacen("\"kind\": \"CoreReference\",", "", 1);
    assert!(matches!(input::parse_request(no_kind.as_bytes()), Err(DriverError::Input(_))));
    let other_kind = REQ_INVOICE.replacen("\"kind\": \"CoreReference\"", "\"kind\": \"Hint\"", 1);
    assert!(matches!(input::parse_request(other_kind.as_bytes()), Err(DriverError::Input(_))));
}

#[test]
fn a06_context_integer_ranges_are_typed() {
    for (from, to) in [
        ("\"networkId\": 0,", "\"networkId\": 256,"),
        ("\"consumedIx\": 0,", "\"consumedIx\": 65536,"),
        ("\"seq\": 1,", "\"seq\": -1,"),
        ("\"seq\": 1,", "\"seq\": 1.5,"),
        ("\"seq\": 1,", "\"seq\": \"1\","),
    ] {
        let text = REQ_INVOICE.replacen(from, to, 1);
        assert_ne!(text, REQ_INVOICE, "edit {to} applied");
        assert!(matches!(input::parse_request(text.as_bytes()), Err(DriverError::Input(_))), "{to}");
    }
}

#[test]
fn a07_context_hex_lengths_and_vk_restatement() {
    let base = request(REQ_INVOICE).context;
    let vk = [0x11u8; 32];
    let ctx = claim::context_from_input(&base, vk).expect("fixture context");
    assert_eq!(ctx.vk_digest, vk);
    assert_eq!(ctx.claimant_vkh, [0x55; 28]);
    assert_eq!(ctx.seq, 1);
    let mut long = base.clone();
    long.claimant_vkh = "55".repeat(32);
    assert!(claim::context_from_input(&long, vk).is_err(), "claimant must be 28 bytes");
    let mut short = base.clone();
    short.settle_hash = "a2".repeat(27);
    assert!(claim::context_from_input(&short, vk).is_err(), "settle hash must be 28 bytes");
    let mut upper = base.clone();
    upper.thread_id = upper.thread_id.to_uppercase();
    assert!(claim::context_from_input(&upper, vk).is_err(), "uppercase hex refused");
    let mut same = base.clone();
    same.vk_digest = Some("11".repeat(32));
    assert!(claim::context_from_input(&same, vk).is_ok(), "equal restatement accepted");
    let mut other = base;
    other.vk_digest = Some("12".repeat(32));
    assert!(matches!(claim::context_from_input(&other, vk), Err(DriverError::IdentityMismatch { .. })));
}

#[test]
fn a08_example_requests_transcribe_the_r1_fixtures() {
    for (text, id) in [(REQ_INVOICE, "P02"), (REQ_RESERVE, "P04")] {
        let req = request(text);
        let case = r1_case(id);
        assert_eq!(req.selector.action, case["action"].as_str().unwrap());
        let mori_k2_driver::TransitionInput::CoreReference { scenario } = &req.transition else {
            panic!("{id}: CoreReference")
        };
        assert_eq!(serde_json::to_value(scenario).unwrap(), case["bridgeScenario"], "{id}: scenario transcription");
        let c = &case["context"];
        let x = &req.context;
        assert_eq!(x.claimant_vkh, c["claimantVkh"].as_str().unwrap());
        assert_eq!(u64::from(x.network_id), c["networkId"].as_u64().unwrap());
        assert_eq!(x.settle_hash, c["settleHash"].as_str().unwrap());
        assert_eq!(x.thread_id, c["threadId"].as_str().unwrap());
        assert_eq!(x.consumed_txid, c["consumedTxid"].as_str().unwrap());
        assert_eq!(u64::from(x.consumed_ix), c["consumedIx"].as_u64().unwrap());
        assert_eq!(x.seq, c["seq"].as_u64().unwrap());
        assert_eq!(x.catalogue_digest, c["catalogueDigest"].as_str().unwrap());
        assert_eq!(x.intents_digest, c["intentsDigest"].as_str().unwrap());
        assert_eq!(x.vk_digest, None, "the VK digest is derived, never pre-stated in examples");
    }
}

#[test]
fn a09_selector_profile_refusals_before_compilation() {
    let ok = selector("partial");
    assert!(relation::check_selector(&ok).is_ok());
    let edits: [(&str, fn(&mut mori_k2_driver::RelationSelector)); 9] = [
        ("cap PublicCap", |s| s.cap = "PublicCap".into()),
        ("accounting NotBound", |s| s.accounting = "NotBound".into()),
        ("frame Include", |s| s.frame = "Include".into()),
        ("adapter DirectFieldV0", |s| s.adapter = "DirectFieldV0".into()),
        ("adapter ClaimantBoundAbiV1Pending", |s| s.adapter = "ClaimantBoundAbiV1Pending".into()),
        ("language moriarty/2", |s| s.language = "moriarty/2".into()),
        ("semantics moriarty-core/2", |s| s.semantics = "moriarty-core/2".into()),
        ("format /2", |s| s.format = "moriarty-k2-relation-selector/2".into()),
        ("empty action", |s| s.action = String::new()),
    ];
    for (what, edit) in edits {
        let mut s = ok.clone();
        edit(&mut s);
        let r = relation::check_selector(&s);
        assert!(matches!(r, Err(DriverError::Unsupported(_)) | Err(DriverError::Input(_))), "{what} must be refused");
    }
}

// ## BLS12-381 points (consumer constants and canonical round trip)

fn g1_times(k: u64) -> G1Affine {
    (G1Affine::generator() * Fr::from(k)).into_affine()
}

fn g2_times(k: u64) -> G2Affine {
    (G2Affine::generator() * Fr::from(k)).into_affine()
}

#[test]
fn a10_identity_encodings_equal_the_c2_consumer_constants() {
    // [CONSUMER] groth16.ak:22-26: c0 followed by zeros (48 and 96 bytes).
    let g1 = format!("c0{}", "00".repeat(47));
    let g2 = format!("c0{}", "00".repeat(95));
    assert_eq!(hex(&encoding::encode_g1(&G1Affine::zero()).unwrap()), g1);
    assert_eq!(hex(&encoding::encode_g2(&G2Affine::zero()).unwrap()), g2);
    assert_eq!(hex(&encoding::g1_identity_compressed()), g1);
    assert_eq!(hex(&encoding::g2_identity_compressed()), g2);
    // The canonical identity decodes (it is a valid point); profiles refuse it where needed.
    assert!(encoding::decode_g1(&unhex(&g1), "t").unwrap().xy().is_none());
}

#[test]
fn a11_generator_round_trip_and_flags() {
    let g1 = encoding::encode_g1(&G1Affine::generator()).unwrap();
    assert_eq!(g1[0] & 0x80, 0x80, "compression flag set");
    assert_eq!(g1[0] & 0x40, 0, "infinity flag clear");
    assert_eq!(encoding::decode_g1(&g1, "t").unwrap(), G1Affine::generator());
    let g2 = encoding::encode_g2(&G2Affine::generator()).unwrap();
    assert_eq!(g2[0] & 0xc0, 0x80);
    assert_eq!(encoding::decode_g2(&g2, "t").unwrap(), G2Affine::generator());
    let p = g1_times(5);
    assert_eq!(encoding::decode_g1(&encoding::encode_g1(&p).unwrap(), "t").unwrap(), p);
}

#[test]
fn a12_non_canonical_and_malformed_points_refused() {
    // Identity with a stray bit: arkworks alone would accept it (util.rs:107-109).
    let mut stray = encoding::g1_identity_compressed();
    stray[47] = 1;
    assert!(encoding::decode_g1(&stray, "t").is_err());
    // Identity with the sort flag set.
    let mut sorted = encoding::g1_identity_compressed();
    sorted[0] = 0xe0;
    assert!(encoding::decode_g1(&sorted, "t").is_err());
    let mut stray2 = encoding::g2_identity_compressed();
    stray2[95] = 1;
    assert!(encoding::decode_g2(&stray2, "t").is_err());
    // Compression flag cleared.
    let mut uncompressed = encoding::encode_g1(&G1Affine::generator()).unwrap();
    uncompressed[0] &= 0x7f;
    assert!(encoding::decode_g1(&uncompressed, "t").is_err());
    // x = 2^381 - 1 > p with the compression flag.
    let mut big = [0xffu8; 48];
    big[0] = 0x9f;
    assert!(encoding::decode_g1(&big, "t").is_err());
    // Wrong lengths.
    assert!(encoding::decode_g1(&[0u8; 47], "t").is_err());
    assert!(encoding::decode_g1(&[0u8; 49], "t").is_err());
    assert!(encoding::decode_g2(&[0u8; 95], "t").is_err());
}

// ## VK and proof layout (SPECIMEN points; no setup)

fn specimen_vk() -> VerifyingKey<Bls12_381> {
    VerifyingKey { alpha_g1: g1_times(2), beta_g2: g2_times(3), gamma_g2: g2_times(5), delta_g2: g2_times(7), gamma_abc_g1: vec![g1_times(11), g1_times(13), g1_times(17)] }
}

fn specimen_proof() -> Proof<Bls12_381> {
    Proof { a: g1_times(19), b: g2_times(23), c: g1_times(29) }
}

#[test]
fn a13_vk_layout_matches_the_c2_consumer() {
    let bytes = encoding::encode_vk(&specimen_vk()).unwrap();
    assert_eq!(bytes.len(), CONSUMER_VK_LEN);
    assert_eq!(VK_LEN, CONSUMER_VK_LEN);
    assert_eq!(hex(&bytes[336..344]), CONSUMER_VK_IC_COUNT_HEX);
    assert_eq!(hex(&bytes[0..48]), hex(&encoding::encode_g1(&g1_times(2)).unwrap()), "alpha at 0");
    assert_eq!(hex(&bytes[144..240]), hex(&encoding::encode_g2(&g2_times(5)).unwrap()), "gamma at 144");
    assert_eq!(hex(&bytes[440..488]), hex(&encoding::encode_g1(&g1_times(17)).unwrap()), "ic2 at 440");
    assert_eq!(encoding::decode_vk(&bytes).unwrap(), specimen_vk());
}

#[test]
fn a14_vk_malformed_refused() {
    let bytes = encoding::encode_vk(&specimen_vk()).unwrap();
    assert!(encoding::decode_vk(&bytes[..487]).is_err());
    let mut longer = bytes.clone();
    longer.push(0);
    assert!(encoding::decode_vk(&longer).is_err());
    for count in [2u8, 4] {
        let mut b = bytes.clone();
        b[336] = count;
        assert!(encoding::decode_vk(&b).is_err(), "IC prefix {count}");
    }
    // Identity gamma (C2 F-I also refuses) and identity alpha (driver is stricter than C2).
    let mut gamma = bytes.clone();
    gamma[144..240].copy_from_slice(&encoding::g2_identity_compressed());
    assert!(encoding::decode_vk(&gamma).is_err());
    let mut alpha = bytes.clone();
    alpha[0..48].copy_from_slice(&encoding::g1_identity_compressed());
    assert!(encoding::decode_vk(&alpha).is_err());
    let mut four_ic = specimen_vk();
    four_ic.gamma_abc_g1.push(g1_times(31));
    assert!(encoding::encode_vk(&four_ic).is_err(), "four IC points");
}

#[test]
fn a15_proof_layout_and_refusals() {
    let bytes = encoding::encode_proof(&specimen_proof()).unwrap();
    assert_eq!(bytes.len(), CONSUMER_PROOF_LEN);
    assert_eq!(PROOF_LEN, CONSUMER_PROOF_LEN);
    assert_eq!(encoding::decode_proof(&bytes).unwrap(), specimen_proof());
    assert!(encoding::decode_proof(&bytes[..191]).is_err());
    let mut identity_a = bytes.clone();
    identity_a[0..48].copy_from_slice(&encoding::g1_identity_compressed());
    assert!(encoding::decode_proof(&identity_a).is_err());
}

// ## Statements (R1-AUTH bytes; the compiler's host encoder only, no synthesis)

#[test]
fn a16_statement_parse_recovers_the_fixture_context() {
    for id in ["P02", "P04"] {
        let case = r1_case(id);
        let c = &case["context"];
        let stmt = unhex(case["statementHex"].as_str().unwrap());
        let view = claim::parse_statement(&stmt).expect("fixture statement parses");
        assert_eq!(view.predicate_digest, arr32(c["predicateDigestSynthetic"].as_str().unwrap()));
        assert_eq!(view.shape_digest, arr32(case["shapeDigest"].as_str().unwrap()));
        assert_eq!(view.context.vk_digest, arr32(c["vkDigestSynthetic"].as_str().unwrap()));
        assert_eq!(hex(&view.context.claimant_vkh), c["claimantVkh"].as_str().unwrap());
        assert_eq!(hex(&view.context.settle_hash), c["settleHash"].as_str().unwrap());
        assert_eq!(hex(&view.context.thread_id), c["threadId"].as_str().unwrap());
        assert_eq!(hex(&view.context.consumed_txid), c["consumedTxid"].as_str().unwrap());
        assert_eq!(u64::from(view.context.consumed_ix), c["consumedIx"].as_u64().unwrap());
        assert_eq!(view.context.seq, c["seq"].as_u64().unwrap());
        assert_eq!(hex(&view.context.catalogue_digest), c["catalogueDigest"].as_str().unwrap());
        assert_eq!(hex(&view.context.intents_digest), c["intentsDigest"].as_str().unwrap());
        assert_eq!(view.body_len as usize, unhex(case["bodyHex"].as_str().unwrap()).len());
    }
}

#[test]
fn a17_statement_tampering_refused_by_the_parser() {
    let stmt = unhex(r1_case("P02")["statementHex"].as_str().unwrap());
    let mut tag = stmt.clone();
    tag[0] ^= 1;
    assert!(claim::parse_statement(&tag).is_err(), "tag");
    let mut family = stmt.clone();
    family[16] ^= 1;
    assert!(claim::parse_statement(&family).is_err(), "family");
    let mut len = stmt.clone();
    len[342] = len[342].wrapping_add(1);
    assert!(claim::parse_statement(&len).is_err(), "body_len");
    assert!(claim::parse_statement(&stmt[..342]).is_err(), "short header");
    let mut longer = stmt.clone();
    longer.push(0);
    assert!(claim::parse_statement(&longer).is_err(), "trailing byte");
    assert!(claim::parse_statement(&vec![0u8; claim::MAX_STATEMENT_BYTES + 1]).is_err(), "oversize");
}

#[test]
fn a18_statement_digest_halves_match_the_fixture() {
    for id in ["P02", "P04"] {
        let case = r1_case(id);
        let stmt = unhex(case["statementHex"].as_str().unwrap());
        let digest = mori_r1cs::blake2b::blake2b256(&stmt);
        assert_eq!(hex(&digest), case["statementDigest"].as_str().unwrap());
        let [x1, x2] = claim::digest_halves(&digest);
        assert_eq!(x1.to_string(), case["publicInputs"]["x1"].as_str().unwrap());
        assert_eq!(x2.to_string(), case["publicInputs"]["x2"].as_str().unwrap());
    }
}

// ## Setup record and proof export structure (SPECIMEN record; no pairing)

fn specimen_descriptor() -> RelationDescriptor {
    let case = r1_case("P02");
    let stmt = case["statementHex"].as_str().unwrap();
    RelationDescriptor {
        relation_format: "mori-r1cs-transition/1-unadopted".into(),
        relation_version: 1,
        language: "moriarty/1".into(),
        semantics: "moriarty-core/1".into(),
        logical_cost_profile: "SPECIMEN".into(),
        profile_digest: "01".repeat(32),
        program_hash: "02".repeat(32),
        source_sha256: Some("03".repeat(32)),
        action: "partial".into(),
        action_index: 1,
        relation_shape_digest: "04".repeat(32),
        compiler_source_manifest_sha256: "05".repeat(32),
        core_closure_sha256: "06".repeat(32),
        compiler_build_digest: "07".repeat(32),
        adapter: "SettlementHashV2".into(),
        logical_work: "28".into(),
        predicate_digest: case["context"]["predicateDigestSynthetic"].as_str().unwrap().into(),
        abi2_shape_digest: case["shapeDigest"].as_str().unwrap().into(),
        abi2_shape_bytes_hex: case["shapeBytesHex"].as_str().unwrap().into(),
        body_len: (case["bodyHex"].as_str().unwrap().len() / 2) as u32,
        statement_tag_hex: stmt[0..32].into(),
        family_id_hex: stmt[32..96].into(),
        header_len: 343,
        field_modulus_le_hex: "SPECIMEN".into(),
        r1cs_sha256: "08".repeat(32),
        constraints: 1,
        instance_variables: 3,
        witness_variables: 1,
        public_input_count: 2,
    }
}

fn specimen_record(desc: RelationDescriptor) -> SetupRecord {
    let vk = encoding::encode_vk(&specimen_vk()).unwrap();
    SetupRecord {
        format: export::SETUP_FORMAT.into(),
        trust_profile: export::TRUST_PROFILE.into(),
        notice: export::SETUP_NOTICE.into(),
        relation: desc,
        curve: export::CURVE.into(),
        snark: export::SNARK.into(),
        encoding: export::ENCODING.into(),
        verifying_key_hex: hex(&vk),
        vk_digest_blake2b256: hex(&mori_r1cs::blake2b::blake2b256(&vk)),
        proving_key_sha256: "09".repeat(32),
        proving_key_bytes: 1,
        driver: mori_k2_driver::DRIVER_ID.into(),
    }
}

fn parsed(record: &SetupRecord) -> Result<ParsedSetup, DriverError> {
    export::parse_setup_record(&export::document_bytes(record).unwrap())
}

/// The P02 fixture statement with its VK digest replaced by the specimen VK's digest.
fn specimen_claim(setup: &ParsedSetup) -> PreparedClaim {
    let mut statement = unhex(r1_case("P02")["statementHex"].as_str().unwrap());
    statement[112..144].copy_from_slice(&setup.vk_digest);
    let view = claim::parse_statement(&statement).unwrap();
    let statement_digest = mori_r1cs::blake2b::blake2b256(&statement);
    let empty = BodyValues { states: vec![], cells: vec![], duties: vec![] };
    PreparedClaim {
        values: TransitionValues { inputs: vec![], pre: empty.clone(), post: empty, effects: vec![], cap: None },
        context: view.context,
        statement,
        statement_digest,
        public_inputs: claim::digest_halves(&statement_digest),
    }
}

fn specimen_export(setup: &ParsedSetup) -> ProofExport {
    export::build_export(setup, &specimen_claim(setup), &specimen_proof()).unwrap()
}

#[test]
fn a19_setup_record_strict_parse_and_profile() {
    let record = specimen_record(specimen_descriptor());
    let ok = parsed(&record).expect("specimen record parses");
    assert_eq!(ok.vk, specimen_vk());
    let edits: [(&str, fn(&mut SetupRecord)); 7] = [
        ("trust profile", |r| r.trust_profile = "ceremony/1".into()),
        ("driver", |r| r.driver = "other-driver/1".into()),
        ("vk digest", |r| r.vk_digest_blake2b256 = "00".repeat(32)),
        ("adapter", |r| r.relation.adapter = "DirectFieldV0".into()),
        ("public inputs", |r| r.relation.public_input_count = 3),
        ("shape digest vs bytes", |r| r.relation.abi2_shape_digest = "00".repeat(32)),
        ("vk hex length", |r| r.verifying_key_hex.push_str("00")),
    ];
    for (what, edit) in edits {
        let mut r = record.clone();
        edit(&mut r);
        assert!(parsed(&r).is_err(), "{what} must be refused");
    }
    let text = String::from_utf8(export::document_bytes(&record).unwrap()).unwrap();
    let unknown = text.replacen("\"format\":", "\"extra\": 1,\n  \"format\":", 1);
    assert!(export::parse_setup_record(unknown.as_bytes()).is_err(), "unknown field");
}

#[test]
fn a20_proof_export_structure_round_trips_without_pairing() {
    let setup = parsed(&specimen_record(specimen_descriptor())).unwrap();
    let e = specimen_export(&setup);
    let bytes = export::document_bytes(&e).unwrap();
    let back = export::parse_proof_export(&bytes).unwrap();
    assert_eq!(back, e);
    let checked = export::check_export(&setup, &back).expect("structurally consistent specimen");
    assert_eq!(checked.public_inputs, claim::digest_halves(&checked.statement_digest));
    assert_eq!(e.cardano.intent_target_hash_v2.len(), 194);
    assert_eq!(e.cardano.env_v2.family_id, specimen_descriptor().family_id_hex);
    assert_eq!(e.cardano.env_v2.settle_outref.transaction_id, "22".repeat(32));
    assert_eq!(e.cardano.intent_anchor, format!("cardano-plutus-v3:{}", "a2".repeat(28)));
}

#[test]
fn a21_proof_export_tampering_refused_without_pairing() {
    let setup = parsed(&specimen_record(specimen_descriptor())).unwrap();
    let base = specimen_export(&setup);
    let edits: [(&str, fn(&mut ProofExport)); 12] = [
        ("statement digest", |e| e.statement_digest_blake2b256 = "00".repeat(32)),
        ("x1", |e| e.public_inputs.x1 = "1".into()),
        ("x2 leading zero", |e| e.public_inputs.x2 = format!("0{}", e.public_inputs.x2)),
        ("context field only", |e| e.context.claimant_vkh = "66".repeat(28)),
        ("predicate copy", |e| e.predicate_digest = "00".repeat(32)),
        ("vk digest copy", |e| e.vk_digest_blake2b256 = "00".repeat(32)),
        ("setup record link", |e| e.setup_record_sha256 = "00".repeat(32)),
        ("r1cs copy", |e| e.r1cs_sha256 = "00".repeat(32)),
        ("proof length", |e| e.proof_hex.truncate(2 * 191)),
        ("premises", |e| e.premises.pop().map(|_| ()).unwrap_or(())),
        ("cardano statement digest", |e| e.cardano.env_v2.statement_digest = "00".repeat(32)),
        ("statement body byte", |e| {
            let last = e.statement_hex.len() - 1;
            let flipped = if e.statement_hex.ends_with('0') { '1' } else { '0' };
            e.statement_hex.replace_range(last.., &flipped.to_string());
        }),
    ];
    for (what, edit) in edits {
        let mut e = base.clone();
        edit(&mut e);
        assert!(export::check_export(&setup, &e).is_err(), "{what} must be refused");
    }
}

#[test]
fn a22_statement_constants_must_equal_the_record() {
    // Record predicate differs from the statement's (the export copies the record value).
    let mut desc = specimen_descriptor();
    desc.predicate_digest = "0a".repeat(32);
    let setup = parsed(&specimen_record(desc)).unwrap();
    let e = specimen_export(&setup);
    assert!(matches!(export::check_export(&setup, &e), Err(DriverError::IdentityMismatch { .. })));
}

#[test]
fn a23_descriptor_difference_is_named() {
    let a = specimen_descriptor();
    let mut b = a.clone();
    b.r1cs_sha256 = "0b".repeat(32);
    let (field, _, _) = relation::first_difference(&a, &b).expect("differ");
    assert_eq!(field, "r1csSha256");
    assert!(relation::require_same(&a, &b).is_err());
    assert!(relation::require_same(&a, &a.clone()).is_ok());
}

#[test]
fn a24_context_input_round_trips_through_the_request_parser() {
    let ctx = ContextInput { vk_digest: Some("11".repeat(32)), ..synthetic_context() };
    let text = serde_json::to_string(&ctx).unwrap();
    let back: ContextInput = input::parse_json(text.as_bytes(), "context").unwrap();
    assert_eq!(back, ctx);
}
