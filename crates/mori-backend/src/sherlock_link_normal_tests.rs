//! Owner-authored nonproving unit expectations for the fixed S1/S2 host surface; not an
//! independent oracle or cryptographic evidence. Binding lanes are zero, digests are
//! synthetic caller assertions, and child claims, keys and proofs are placeholders that
//! are never proved or verified. No k, SRS, keygen, prove, verify, SDK check or FS.
use super::*;
use mori_core::sherlock::{Expr, Value};
use std::path::PathBuf;

const LINK: &str = "mori::backend::sherlock-link";

fn caps() -> LiteralCapacities {
    LiteralCapacities {
        program: [2, 8, 2, 2, 2, 2, 1, 1, 1, 1, 4, 4, 5, 2, 2],
        state: [1; 6],
        stack_limit: 1,
    }
}
fn ident() -> LiteralSourceIdentity {
    LiteralSourceIdentity {
        k_source_sha256: "11".repeat(32),
        reference_source_sha256: "22".repeat(32),
        program_source_sha256: "33".repeat(32),
    }
}
fn program(entry: Expr) -> Arc<Program> {
    Arc::new(Program {
        functions: Vec::new(),
        entry: Arc::new(entry),
    })
}
fn produced() -> (LinkedRequests, LinkShape) {
    produce_linked_requests(
        program(Expr::Literal(Value::UInt(7))),
        &caps(),
        2,
        1,
        &ident(),
    )
    .expect("honest H1 A102 -> A80")
}
fn layouts() -> (LiteralLayout, ReturnPublicationLayout) {
    let n = local_max_statement_lanes();
    (
        LiteralLayout::new(&caps(), &ident(), n).unwrap(),
        ReturnPublicationLayout::new(&caps(), &ident(), n).unwrap(),
    )
}
fn options() -> ProofOptions {
    // Never read: every use refuses or returns before k selection or SRS access.
    ProofOptions {
        params_dir: PathBuf::new(),
        max_k: MAX_K,
    }
}
fn refusal<T>(r: Result<T, Error>) -> Error {
    r.err().expect("expected a refusal")
}
fn mutated(r: &LinkedRequests, m: impl FnOnce(&mut LinkedRequests)) -> Error {
    let mut x = r.clone();
    m(&mut x);
    refusal(check_linked_requests(&x))
}
/// Zero binding lane, then each statement from its own layout; lane 0 is never linked.
fn instances(r: &LinkedRequests) -> (Vec<Fr>, Vec<Fr>) {
    let (a102, a80, _) = check_linked_requests(r).unwrap();
    let lanes = |s: Vec<u128>| -> Vec<Fr> {
        std::iter::once(Fr::from(0u64))
            .chain(s.into_iter().map(Fr::from))
            .collect()
    };
    (
        lanes(a102.statement(&r.a102.values).unwrap()),
        lanes(a80.statement(&r.a80.values).unwrap()),
    )
}
/// Placeholder carriers for host refusal paths: empty keys, and the A102 claim slot is the
/// A80 template minus its carrier hash. Never a proved, verified or A102 claim.
fn children(r: &LinkedRequests, a102: Vec<Fr>, a80: Vec<Fr>, proof: Vec<u8>) -> LinkedChildren {
    let claim = prepare_return_non_native(&r.a80, &options())
        .unwrap()
        .claim_template;
    let mut slot = serde_json::to_value(&claim).unwrap();
    slot.as_object_mut()
        .unwrap()
        .remove("carrier_source_sha256");
    LinkedChildren {
        a102: SherlockChildProof {
            claim: serde_json::from_value(slot).unwrap(),
            instance: a102,
            vk_bytes: Vec::new(),
            proof: proof.clone(),
        },
        a80: ReturnChildProof {
            claim,
            instance: a80,
            vk_bytes: Vec::new(),
            proof,
        },
    }
}
fn bump(v: &[Fr], i: usize) -> Vec<Fr> {
    let mut v = v.to_vec();
    v[i] = v[i] + Fr::from(1u64);
    v
}

#[test]
fn offsets_follow_p_2s_5_and_malformed_shapes_refuse_without_panic() {
    let s = LinkShape {
        program_lanes: 10,
        state_lanes: 4,
        instance_lanes: 23,
        a102_context: [0; 2],
        a80_context: [0; 2],
    };
    assert_eq!(
        s.offsets().unwrap(),
        LinkOffsets {
            binding: 0,
            context: 1,
            program: 3,
            pre: 13,
            post: 17,
            rule_tag: 21,
            charge: 22
        }
    );
    let short = [Fr::from(0u64); 2];
    for bad in [
        LinkShape {
            instance_lanes: 22,
            ..s
        },
        LinkShape {
            instance_lanes: 24,
            ..s
        },
        LinkShape {
            instance_lanes: 2,
            ..s
        },
        LinkShape {
            program_lanes: usize::MAX,
            ..s
        },
        LinkShape {
            state_lanes: usize::MAX / 2 + 1,
            ..s
        },
        LinkShape {
            program_lanes: usize::MAX - 6,
            state_lanes: 1,
            instance_lanes: usize::MAX,
            ..s
        },
    ] {
        assert_eq!(refusal(bad.offsets()).code, LINK);
        assert_eq!(refusal(check_link_lanes(&[], &[], &bad)).code, LINK);
        assert_eq!(refusal(check_link_lanes(&short, &short, &bad)).code, LINK);
    }
    let (a102, a80) = layouts();
    let honest = link_shape(&a102, &a80).unwrap();
    assert_eq!(
        honest.instance_lanes,
        honest.program_lanes + 2 * honest.state_lanes + 5
    );
    assert_eq!(honest.instance_lanes, a80.statement_len() + 1);
    assert_eq!(
        (honest.a102_context, honest.a80_context),
        (a102.limbs(), a80.limbs())
    );
    assert_ne!(a102.context_hex(), a80.context_hex());
    assert_eq!(
        refusal(check_link_lanes(&[], &[], &honest)).message,
        "Child instance length differs from the link shape."
    );
}

#[test]
fn check_link_lanes_accepts_honest_source_lanes_and_refuses_each_swap() {
    let (r, shape) = produced();
    let (a102, a80) = instances(&r);
    let o = shape.offsets().unwrap();
    check_link_lanes(&a102, &a80, &shape).unwrap();
    let refuse = |x: &[Fr], y: &[Fr], s: &LinkShape| refusal(check_link_lanes(x, y, s)).message;
    let swapped = LinkShape {
        a102_context: shape.a80_context,
        a80_context: shape.a102_context,
        ..shape
    };
    let context = "A child context differs from its reconstructed layout.";
    assert_eq!(refuse(&a80, &a102, &shape), context);
    assert_eq!(refuse(&a102, &a80, &swapped), context);
    assert_eq!(refuse(&a102, &bump(&a80, o.context), &shape), context);
    assert_eq!(
        refuse(&a102, &bump(&a80, o.program), &shape),
        "A102 and A80 ProgramBlock lanes differ."
    );
    let midpoint = "A102 post lanes differ from A80 pre lanes.";
    assert_eq!(refuse(&a102, &bump(&a80, o.pre), &shape), midpoint);
    assert_eq!(refuse(&bump(&a102, o.post), &a80, &shape), midpoint);
    let envelope = "Child envelopes differ from A102 (3, 1) then A80 (0, 0).";
    assert_eq!(refuse(&bump(&a102, o.rule_tag), &a80, &shape), envelope);
    assert_eq!(refuse(&a102, &bump(&a80, o.charge), &shape), envelope);
    assert_eq!(
        refuse(&a102[..a102.len() - 1], &a80, &shape),
        "Child instance length differs from the link shape."
    );
}

#[test]
fn producer_links_requests_and_check_linked_requests_refuses_swaps() {
    let (r, shape) = produced();
    let (a102, a80) = layouts();
    assert_eq!(shape, link_shape(&a102, &a80).unwrap());
    assert_eq!(check_linked_requests(&r).unwrap().2, shape);
    assert_eq!(r.a102.values.post, r.a80.values.pre);
    assert_eq!(r.a102.values.program, r.a80.values.program);
    assert_eq!(
        (
            r.a102.values.rule_tag,
            r.a102.values.charge,
            r.a80.values.rule_tag,
            r.a80.values.charge
        ),
        (3, 1, 0, 0)
    );
    assert_eq!((&r.a102.capacities, &r.a102.identity), (&caps(), &ident()));
    assert_eq!(
        mutated(&r, |x| x.a80.identity.program_source_sha256 =
            "44".repeat(32))
        .message,
        "A102 and A80 requests differ in capacities or identity."
    );
    assert_eq!(
        mutated(&r, |x| x.a80.values.program.root[0] += 1).message,
        "A102 and A80 ProgramBlock images differ."
    );
    assert_eq!(
        mutated(&r, |x| x.a80.values.pre.root[0] += 1).message,
        "A102 post differs from A80 pre."
    );
    let envelope = "mori::backend::sherlock-envelope";
    assert_eq!(
        mutated(&r, |x| (x.a80.values.rule_tag, x.a80.values.charge) =
            (3, 1))
        .code,
        envelope
    );
    assert_eq!(
        mutated(&r, |x| (x.a102.values.rule_tag, x.a102.values.charge) =
            (0, 0))
        .code,
        envelope
    );
    assert_eq!(refusal(a102.statement(&r.a80.values)).code, envelope);
}

#[test]
fn a80_statement_emit_roundtrip_and_carrier_shape() {
    let (r, _) = produced();
    let (_, a80) = layouts();
    let statement = a80.statement(&r.a80.values).unwrap();
    assert_eq!(statement.len(), a80.statement_len());
    assert_eq!(statement[..2], a80.limbs());
    assert_eq!(statement[statement.len() - 2..], [0, 0]);
    let mut short = r.a80.values.clone();
    short.post.root.pop();
    assert_eq!(
        refusal(a80.statement(&short)).code,
        "mori::backend::sherlock-shape"
    );
    let mut wide = r.a80.values.clone();
    wide.rule_tag = 39;
    assert_eq!(
        refusal(a80.statement(&wide)).code,
        "mori::backend::sherlock-range"
    );
    let (ir, zkir) = a80.emit().unwrap();
    assert!(matches!(IrSource::load(zkir.as_bytes()), Ok(parsed) if parsed == ir));
    assert!(ir.inputs.is_empty() && !ir.do_communications_commitment);
    let n = ir
        .instructions
        .iter()
        .filter(|i| matches!(i, Instruction::PublicInput { .. }))
        .count();
    assert_eq!(n, statement.len());
    let (_, instance) = instances(&r);
    child_witness(&ir, &instance, vec![None; n]).unwrap();
    assert!(child_witness(&ir, &instance[..n], vec![None; n]).is_err());
}

#[test]
fn anchor_genesis_checks_shape_before_indexing_and_refuses_non_init() {
    let (r, shape) = produced();
    let o = shape.offsets().unwrap();
    let (a102, a80) = instances(&r);
    let c = children(&r, a102, a80.clone(), Vec::new());
    let g = anchor_genesis(&r, &c, &shape).unwrap();
    assert_eq!(
        (g.height, g.child.as_str(), g.state_offset, g.state_lanes),
        (0, "a102", o.pre, shape.state_lanes)
    );
    let short = children(&r, vec![Fr::from(0u64)], a80, Vec::new());
    for bad in [
        LinkShape {
            state_lanes: shape.state_lanes + 1,
            ..shape
        },
        LinkShape {
            a80_context: shape.a102_context,
            ..shape
        },
    ] {
        assert_eq!(
            refusal(anchor_genesis(&r, &short, &bad)).message,
            "Link shape differs from the reconstructed layouts."
        );
    }
    let genesis = "mori::backend::sherlock-genesis";
    assert_eq!(refusal(anchor_genesis(&r, &short, &shape)).code, genesis);
    let mut tampered = c.clone();
    tampered.a102.instance = bump(&tampered.a102.instance, o.pre);
    assert_eq!(refusal(anchor_genesis(&r, &tampered, &shape)).code, genesis);
    let mut later = r.clone();
    let depth = layouts()
        .0
        .schema
        .state_root
        .iter()
        .position(|l| l.name == "depth")
        .unwrap();
    later.a102.values.pre.root[depth] = 1;
    assert_eq!(
        refusal(anchor_genesis(&later, &c, &shape)).message,
        "A102 pre violates the Machine::new zero equations."
    );
}

#[test]
fn layout_admission_refuses_over_bound_before_h1_production() {
    let (a102, a80) = layouts();
    let n = a80.statement_len();
    assert_eq!(n, a102.statement_len());
    ReturnPublicationLayout::new(&caps(), &ident(), n).unwrap();
    let limit = "mori::backend::limit";
    assert_eq!(
        refusal(ReturnPublicationLayout::new(&caps(), &ident(), n - 1)).code,
        limit
    );
    assert_eq!(
        refusal(LiteralLayout::new(&caps(), &ident(), n - 1)).code,
        limit
    );
    let var = || program(Expr::Var("x".into()));
    assert_eq!(
        refusal(produce_linked_requests(var(), &caps(), 2, 1, &ident())).code,
        "mori::backend::sherlock-witness"
    );
    let mut big = caps();
    big.state[5] = local_max_statement_lanes();
    let max = local_max_statement_lanes();
    assert_eq!(refusal(LiteralLayout::new(&big, &ident(), max)).code, limit);
    assert_eq!(
        refusal(ReturnPublicationLayout::new(&big, &ident(), max)).code,
        limit
    );
    assert_eq!(
        refusal(produce_linked_requests(var(), &big, 2, 1, &ident())).code,
        limit
    );
    let mut huge = caps();
    huge.state[5] = usize::MAX;
    let overflow = "mori::backend::sherlock-construction";
    assert_eq!(
        refusal(ReturnPublicationLayout::new(&huge, &ident(), usize::MAX)).code,
        overflow
    );
    assert_eq!(
        refusal(produce_linked_requests(var(), &huge, 2, 1, &ident())).code,
        overflow
    );
}

#[test]
fn verify_linked_children_refuses_before_any_native_work() {
    let (r, shape) = produced();
    let o = shape.offsets().unwrap();
    let (a102, a80) = instances(&r);
    let lanes = children(&r, a102.clone(), bump(&a80, o.pre), Vec::new());
    assert_eq!(
        refusal(verify_linked_children(&r, &lanes, &options())).code,
        LINK
    );
    for proof in [Vec::new(), vec![0; MAX_PROOF_BYTES + 1]] {
        let c = children(&r, a102.clone(), a80.clone(), proof);
        assert_eq!(
            refusal(verify_linked_children(&r, &c, &options())).code,
            "mori::backend::proof"
        );
    }
}

// Appended with the batched R1-R4 edit: pure LinkManifest and pre-native gate expectations.
// The manifest is a literal synthetic /2 document from fixed texts, never from a seal,
// producer, writer or `link_child`; its digests are caller assertions, so acceptance is
// intrinsic-shape evidence only, not semantic or cryptographic evidence.
const TEXT: &str = "Manifest format, encoding, authority or disclosure text differs.";
const ENTRY: &str = "A child entry differs from its fixed role, encoding or bounds.";
const GENESIS: &str = "Genesis locator differs from the a102 pre lanes.";
const JSON: &str = "mori::backend::json";

fn hexd(b: u8) -> String {
    format!("{b:02x}").repeat(32)
}
/// a102 sits at the lower k and proof-size bounds, a80 at the upper ones.
fn entry(name: &str, n: usize) -> LinkChild {
    let a102 = name == "a102";
    let d: u8 = if a102 { 0x10 } else { 0x20 };
    let obligations: Vec<&str> = if a102 {
        OPEN_OBLIGATIONS
            .iter()
            .chain(&CHILD_OPEN_OBLIGATIONS)
            .copied()
            .collect()
    } else {
        RETURN_OPEN_OBLIGATIONS
            .iter()
            .chain(&LINK_OPEN_OBLIGATIONS)
            .copied()
            .collect()
    };
    LinkChild {
        name: name.into(),
        claim_file: format!("{name}.claim.json"),
        claim_sha256: hexd(d),
        vk_file: format!("{name}.vk"),
        vk_sha256: hexd(d + 1),
        proof_file: format!("{name}.proof"),
        proof_sha256: hexd(d + 2),
        instance_file: format!("{name}.instance.json"),
        instance_sha256: hexd(d + 3),
        qualification: (if a102 {
            CHILD_QUALIFICATION
        } else {
            RETURN_CHILD_QUALIFICATION
        })
        .into(),
        claim_format: (if a102 {
            CHILD_FORMAT
        } else {
            RETURN_CHILD_FORMAT
        })
        .into(),
        binding: (if a102 {
            CHILD_BINDING_VERSION
        } else {
            RETURN_CHILD_BINDING_VERSION
        })
        .into(),
        instance_lanes: n,
        proof_bytes: if a102 { 1 } else { MAX_PROOF_BYTES as u32 },
        context_hash: hexd(d + 4),
        zkir_hash: hexd(d + 5),
        adapter_source_sha256: hexd(d + 6),
        carrier_source_sha256: (!a102).then(|| hexd(d + 7)),
        circuit_k: if a102 { 1 } else { MAX_K },
        rule_tag: if a102 { 3 } else { 0 },
        charge: if a102 { 1 } else { 0 },
        open_obligations: obligations.iter().map(|t| t.to_string()).collect(),
    }
}
fn synthetic(p: usize, s: usize, offsets: LinkOffsets) -> LinkManifest {
    let n = p + 2 * s + 5;
    LinkManifest {
        format: LINK_FORMAT.into(),
        transcript: CHILD_TRANSCRIPT.into(),
        field_encoding: LINK_FIELD_ENCODING.into(),
        provenance: LINK_PROVENANCE.into(),
        program_lanes: p,
        state_lanes: s,
        instance_lanes: n,
        offsets,
        links: LINK_EQUALITIES.iter().map(|t| t.to_string()).collect(),
        children: vec![entry("a102", n), entry("a80", n)],
        genesis: GenesisAnchor {
            height: 0,
            child: "a102".into(),
            state_offset: offsets.pre,
            state_lanes: s,
            root_authority: GENESIS_ROOT_AUTHORITY.into(),
        },
        pin_authority: LINK_PIN_AUTHORITY.into(),
        open_obligations: LINK_OPEN_OBLIGATIONS
            .iter()
            .map(|t| t.to_string())
            .collect(),
        disclosures: LINK_MANIFEST_DISCLOSURES
            .iter()
            .map(|t| t.to_string())
            .collect(),
    }
}
fn manifest() -> LinkManifest {
    synthetic(
        10,
        4,
        LinkOffsets {
            binding: 0,
            context: 1,
            program: 3,
            pre: 13,
            post: 17,
            rule_tag: 21,
            charge: 22,
        },
    )
}
fn edited(f: impl FnOnce(&mut LinkManifest)) -> LinkManifest {
    let mut m = manifest();
    f(&mut m);
    m
}
/// The same refusal from `validate` and from the `from_json` reader.
fn refuses(message: &str, f: impl FnOnce(&mut LinkManifest)) {
    let m = edited(f);
    let v = refusal(m.validate());
    let j = refusal(LinkManifest::from_json(&serde_json::to_string(&m).unwrap()));
    assert_eq!(v.code, LINK);
    assert_eq!(v.message, message);
    assert_eq!(j.code, LINK);
    assert_eq!(j.message, message);
}
/// `from_json` over an edited JSON value of the honest synthetic manifest.
fn wire(f: impl FnOnce(&mut serde_json::Value)) -> Result<LinkManifest, Error> {
    let mut v = serde_json::to_value(manifest()).unwrap();
    f(&mut v);
    LinkManifest::from_json(&v.to_string())
}

#[test]
fn synthetic_manifest_roundtrips_and_refuses_each_fixed_text_edit() {
    assert_eq!(LINK_FORMAT, "moriarty-sherlock-link/2");
    let m = manifest();
    m.validate().unwrap();
    let text = serde_json::to_string(&m).unwrap();
    assert!(text.contains(r#""carrier_source_sha256":null"#));
    assert_eq!(LinkManifest::from_json(&text).unwrap(), m);
    let cases: &[fn(&mut LinkManifest)] = &[
        |m| m.format = "moriarty-sherlock-link/1".into(),
        |m| m.transcript.push('x'),
        |m| m.field_encoding.push('x'),
        |m| m.provenance.push('x'),
        |m| m.pin_authority.push('x'),
        |m| m.links.swap(0, 1),
        |m| m.links.push(String::new()),
        |m| m.links[4].push('x'),
        |m| m.open_obligations.truncate(4),
        |m| m.open_obligations[0].push('x'),
        |m| m.disclosures.truncate(4),
        |m| m.disclosures[2].push('x'),
        |m| {
            m.disclosures = LINK_OPEN_OBLIGATIONS
                .iter()
                .map(|t| t.to_string())
                .collect()
        },
    ];
    for f in cases {
        refuses(TEXT, *f);
    }
}

#[test]
fn manifest_offsets_and_overflowing_counts_refuse_without_panic() {
    let offsets = "Manifest offsets differ from its checked lane counts.";
    let length = "Link shape instance length differs from P + 2S + 5.";
    let overflow = "Link shape offsets overflow usize.";
    refuses(offsets, |m| m.offsets.pre += 1);
    refuses(offsets, |m| m.offsets.binding = 1);
    refuses(offsets, |m| {
        m.offsets = LinkOffsets {
            charge: 21,
            rule_tag: 22,
            ..m.offsets
        }
    });
    refuses(offsets, |m| (m.program_lanes, m.instance_lanes) = (11, 24));
    refuses(length, |m| m.instance_lanes += 1);
    refuses(length, |m| m.program_lanes += 1);
    refuses(length, |m| m.state_lanes -= 1);
    for (p, s, n, message) in [
        (usize::MAX, 4, 23, overflow),
        (10, usize::MAX / 2 + 1, 23, overflow),
        (usize::MAX - 6, 1, usize::MAX, length),
    ] {
        let m = edited(|m| (m.program_lanes, m.state_lanes, m.instance_lanes) = (p, s, n));
        let text = serde_json::to_string(&m).unwrap();
        let (v, j) = std::panic::catch_unwind(|| (m.validate(), LinkManifest::from_json(&text)))
            .expect("refusal, not panic");
        let v = refusal(v);
        assert_eq!(v.code, LINK);
        assert_eq!(v.message, message);
        assert!(j.is_err());
    }
}

#[test]
fn manifest_children_refuse_count_order_name_file_role_bounds_and_obligations() {
    let count = "Manifest needs exactly the a102 and a80 children.";
    refuses(count, |m| m.children.clear());
    refuses(count, |m| m.children.truncate(1));
    refuses(count, |m| m.children.push(m.children[1].clone()));
    let cases: &[fn(&mut LinkManifest)] = &[
        |m| m.children.swap(0, 1),
        |m| m.children[0].name = "a80".into(),
        |m| m.children[1].name = "A80".into(),
        |m| m.children[0].claim_file = "a102.claim".into(),
        |m| m.children[0].vk_file = "a80.vk".into(),
        |m| m.children[1].proof_file = "a102.proof".into(),
        |m| m.children[1].instance_file = "a80.instance".into(),
        |m| m.children[0].qualification = RETURN_CHILD_QUALIFICATION.into(),
        |m| m.children[1].qualification = CHILD_QUALIFICATION.into(),
        |m| m.children[0].claim_format = RETURN_CHILD_FORMAT.into(),
        |m| m.children[1].claim_format = CHILD_FORMAT.into(),
        |m| m.children[0].binding = RETURN_CHILD_BINDING_VERSION.into(),
        |m| m.children[1].binding = CHILD_BINDING_VERSION.into(),
        |m| m.children[0].rule_tag = 0,
        |m| m.children[0].charge = 0,
        |m| m.children[1].rule_tag = 3,
        |m| m.children[1].charge = 1,
        |m| m.children[0].instance_lanes = 22,
        |m| m.children[1].instance_lanes = 24,
        |m| m.children[0].proof_bytes = 0,
        |m| m.children[1].proof_bytes = MAX_PROOF_BYTES as u32 + 1,
        |m| m.children[0].circuit_k = 0,
        |m| m.children[1].circuit_k = MAX_K + 1,
        |m| m.children[0].carrier_source_sha256 = Some(hexd(0x17)),
        |m| m.children[1].carrier_source_sha256 = None,
        |m| {
            m.children[0].open_obligations.pop();
        },
        |m| m.children[0].open_obligations.push(String::new()),
        |m| m.children[0].open_obligations[0].push('x'),
        |m| m.children[1].open_obligations.swap(0, 1),
        |m| m.children[1].open_obligations.clear(),
        |m| m.children[1].open_obligations = m.children[0].open_obligations.clone(),
    ];
    for f in cases {
        refuses(ENTRY, *f);
    }
}

#[test]
fn manifest_digest_shape_and_per_role_carrier_presence() {
    fn field(c: &mut LinkChild, i: usize) -> &mut String {
        match i {
            0 => &mut c.claim_sha256,
            1 => &mut c.vk_sha256,
            2 => &mut c.proof_sha256,
            3 => &mut c.instance_sha256,
            4 => &mut c.context_hash,
            5 => &mut c.zkir_hash,
            6 => &mut c.adapter_source_sha256,
            _ => c.carrier_source_sha256.as_mut().expect("a80 carrier"),
        }
    }
    for (role, fields) in [(0usize, 7usize), (1, 8)] {
        for i in 0..fields {
            for bad in [
                "AB".repeat(32),
                "ab".repeat(31),
                "ab".repeat(33),
                "zz".repeat(32),
                String::new(),
            ] {
                refuses(ENTRY, |m| *field(&mut m.children[role], i) = bad);
            }
        }
    }
    // R1: the key is required; explicit null is admitted only in the a102 role.
    assert!(
        serde_json::to_value(manifest()).unwrap()["children"][0]["carrier_source_sha256"].is_null()
    );
    assert_eq!(
        wire(|v| v["children"][0]["carrier_source_sha256"] = serde_json::Value::Null).unwrap(),
        manifest()
    );
    for role in [0usize, 1] {
        let e = refusal(wire(|v| {
            v["children"][role]
                .as_object_mut()
                .unwrap()
                .remove("carrier_source_sha256");
        }));
        assert_eq!(e.code, JSON);
    }
    for (role, value) in [
        (0usize, serde_json::Value::from(hexd(0x17))),
        (1, serde_json::Value::Null),
    ] {
        let e = refusal(wire(|v| {
            v["children"][role]["carrier_source_sha256"] = value
        }));
        assert_eq!(e.code, LINK);
        assert_eq!(e.message, ENTRY);
    }
}

#[test]
fn from_json_refuses_duplicate_unknown_missing_trailing_and_mistyped_input() {
    let text = serde_json::to_string(&manifest()).unwrap();
    let duplicate = text.replacen('{', &format!("{{{:?}:{:?},", "format", LINK_FORMAT), 1);
    let unknown = text.replacen('{', r#"{"extra":0,"#, 1);
    for bad in [
        duplicate,
        unknown,
        format!("{text} {{}}"),
        format!("{text}x"),
        text[..text.len() - 1].to_string(),
        String::new(),
    ] {
        assert_eq!(refusal(LinkManifest::from_json(&bad)).code, JSON);
    }
    let edits: &[fn(&mut serde_json::Value)] = &[
        |v| v["offsets"]["extra"] = 0.into(),
        |v| v["genesis"]["extra"] = 0.into(),
        |v| v["children"][1]["extra"] = 0.into(),
        |v| {
            v.as_object_mut().unwrap().remove("genesis");
        },
        |v| {
            v["offsets"].as_object_mut().unwrap().remove("charge");
        },
        |v| {
            v["children"][0]
                .as_object_mut()
                .unwrap()
                .remove("circuit_k");
        },
        |v| v["children"][0]["circuit_k"] = 256.into(),
        |v| v["program_lanes"] = (-1).into(),
        |v| v["children"][1]["proof_bytes"] = "1".into(),
        |v| v["children"] = serde_json::Value::Null,
    ];
    for f in edits {
        assert_eq!(refusal(wire(*f)).code, JSON);
    }
}

#[test]
fn manifest_genesis_refuses_each_field() {
    let cases: &[fn(&mut LinkManifest)] = &[
        |m| m.genesis.height = 1,
        |m| m.genesis.child = "a80".into(),
        |m| m.genesis.state_offset = m.offsets.post,
        |m| m.genesis.state_offset -= 1,
        |m| m.genesis.state_lanes += 1,
        |m| m.genesis.root_authority.push('.'),
        |m| m.genesis.root_authority = String::new(),
    ];
    for f in cases {
        refuses(GENESIS, *f);
    }
}

#[test]
fn anchor_genesis_uses_root_authority_constant_and_fits_a_synthetic_manifest() {
    assert_eq!(
        GENESIS_ROOT_AUTHORITY,
        "consumer pinned Poseidon over a102 instance lanes [state_offset, state_offset + state_lanes); Init host subset only"
    );
    let (r, shape) = produced();
    let (a102, a80) = instances(&r);
    let g = anchor_genesis(&r, &children(&r, a102, a80, Vec::new()), &shape).unwrap();
    assert_eq!(g.root_authority, GENESIS_ROOT_AUTHORITY);
    let m = synthetic(
        shape.program_lanes,
        shape.state_lanes,
        shape.offsets().unwrap(),
    );
    assert_eq!(m.genesis, g);
    m.validate().unwrap();
}

#[test]
fn verify_return_child_refuses_empty_and_oversize_proofs_before_prepare() {
    let (r, _) = produced();
    let (a102, a80) = instances(&r);
    let mut unpreparable = r.a80.clone();
    unpreparable.capacities.state[5] = usize::MAX;
    for proof in [Vec::new(), vec![0; MAX_PROOF_BYTES + 1]] {
        let c = children(&r, a102.clone(), a80.clone(), proof);
        for (request, max_k) in [(&r.a80, MAX_K), (&unpreparable, MAX_K), (&r.a80, 0)] {
            let e = refusal(verify_return_child(
                request,
                &c.a80,
                &ProofOptions { max_k, ..options() },
            ));
            assert_eq!(e.code, "mori::backend::proof");
            assert_eq!(e.message, "Expected a nonempty bounded child proof.");
        }
    }
    // A bounded proof passes the gate; the next refusal is the k cap, still before k or SRS.
    let c = children(&r, a102, a80, vec![0; MAX_PROOF_BYTES]);
    let e = refusal(verify_return_child(
        &r.a80,
        &c.a80,
        &ProofOptions {
            max_k: 0,
            ..options()
        },
    ));
    assert_eq!(e.code, "mori::backend::limit");
    assert_eq!(e.message, "Native k cap must be between 1 and 19.");
}

#[test]
fn prepare_return_non_native_refuses_k_cap_outside_1_to_19_before_layout() {
    assert_eq!(MAX_K, 19);
    let (r, _) = produced();
    let mut huge = r.a80.clone();
    huge.capacities.state[5] = usize::MAX;
    assert_eq!(
        refusal(prepare_return_non_native(&huge, &options())).code,
        "mori::backend::sherlock-construction"
    );
    for max_k in [0, MAX_K + 1] {
        for request in [&r.a80, &huge] {
            let e = refusal(prepare_return_non_native(
                request,
                &ProofOptions { max_k, ..options() },
            ));
            assert_eq!(e.code, "mori::backend::limit");
            assert_eq!(e.message, "Native k cap must be between 1 and 19.");
        }
    }
    // The non-native template keeps the widest k; the cap applies only at native selection.
    for max_k in [1, MAX_K] {
        let base = prepare_return_non_native(&r.a80, &ProofOptions { max_k, ..options() }).unwrap();
        assert_eq!(base.claim_template.circuit_k, MAX_K);
    }
}
