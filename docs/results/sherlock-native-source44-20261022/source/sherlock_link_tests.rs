//! Owner tests of sherlock_link.rs. The ordinary tests are nonproving: no cost model, SRS,
//! keygen, prover, native verifier, pairing or degree probe. They use only Poseidon CPU hashing,
//! serde and files in a fresh temp directory. The ignored test reads two independently
//! pinned child keys and stays nonproving. MockProver, real-child, deferred-pairing,
//! decider and anchor negatives are not here: they need separate eligibility.

use super::*;
use ff::Field;
use std::path::PathBuf;

fn fx(v: u64) -> F {
    F::from(v)
}

/// A context hash with byte 0 = lo and byte 16 = hi.
fn hex_ctx(lo: u8, hi: u8) -> String {
    let mut b = [0u8; 32];
    b[0] = lo;
    b[16] = hi;
    hex::encode(b)
}

fn ctx(h: &str) -> [F; 2] {
    context_limbs(h).unwrap()
}

/// P = 2, S = 2 (n = 11): A102 pre (31, 32) -> mid (41, 42), A80 mid -> post (51, 52).
fn lanes(ca: &[F; 2], cb: &[F; 2]) -> (Vec<F>, Vec<F>) {
    let f = |v: &[u64]| v.iter().map(|x| fx(*x)).collect::<Vec<F>>();
    let a = [&f(&[11u64])[..], &ca[..], &f(&[21u64, 22, 31, 32, 41, 42, 3, 1])[..]].concat();
    let b = [&f(&[12u64])[..], &cb[..], &f(&[21u64, 22, 41, 42, 51, 52, 0, 0])[..]].concat();
    (a, b)
}

fn pins() -> LinkPins {
    LinkPins {
        a102_vk_sha256: "a".repeat(64),
        a80_vk_sha256: "b".repeat(64),
        a102_context_hash: hex_ctx(1, 0),
        a80_context_hash: hex_ctx(0, 2),
        program_lanes: 2,
        state_lanes: 2,
    }
}

/// A /2 manifest under the pins; every child's proof and instance digests cover `proof` and
/// `inst`. Roles come from `link_roles`, so this fixture is not an independent restatement.
fn manifest(p: &LinkPins, o: &LinkOffsets, proof: &[u8], inst: &[u8]) -> serde_json::Value {
    let pinned = [(&p.a102_vk_sha256, &p.a102_context_hash), (&p.a80_vk_sha256, &p.a80_context_hash)];
    let children: Vec<serde_json::Value> = link_roles().into_iter().zip(pinned).map(|((name, q, f, b, (t, c), carrier, open), (vk, cx))| {
        serde_json::json!({
            "name": name, "claim_file": format!("{name}.claim.json"), "claim_sha256": "c".repeat(64),
            "vk_file": format!("{name}.vk"), "vk_sha256": vk, "proof_file": format!("{name}.proof"),
            "proof_sha256": sha256_hex(proof), "instance_file": format!("{name}.instance.json"),
            "instance_sha256": sha256_hex(inst), "qualification": q, "claim_format": f, "binding": b,
            "instance_lanes": o.len, "proof_bytes": proof.len(), "context_hash": cx, "zkir_hash": "d".repeat(64),
            "adapter_source_sha256": "e".repeat(64), "carrier_source_sha256": carrier.then(|| "f".repeat(64)),
            "circuit_k": 14, "rule_tag": t, "charge": c, "open_obligations": open
        })
    }).collect();
    serde_json::json!({
        "format": LINK_FORMAT, "transcript": LINK_TRANSCRIPT, "field_encoding": LINK_FIELD_ENCODING,
        "provenance": LINK_PROVENANCE, "program_lanes": p.program_lanes,
        "state_lanes": p.state_lanes, "instance_lanes": o.len,
        "offsets": {
            "binding": o.binding, "context": o.context, "program": o.program, "pre": o.pre,
            "post": o.post, "rule_tag": o.rule_tag, "charge": o.charge
        },
        "links": LINK_EQUALITIES, "children": children,
        "genesis": {
            "height": 0, "child": "a102", "state_offset": o.pre, "state_lanes": p.state_lanes,
            "root_authority": GENESIS_ROOT_AUTHORITY
        },
        "pin_authority": LINK_PIN_AUTHORITY, "open_obligations": LINK_OPEN, "disclosures": LINK_DISCLOSURES
    })
}

fn tmp(tag: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("recl9-sherlock-link-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir(&d).unwrap();
    d
}

fn put(d: &Path, name: &str, b: &[u8]) {
    fs::write(d.join(name), b).unwrap();
}

#[test]
fn offsets_follow_the_pinned_shape_and_bounds() {
    let o = link_offsets(2, 2).unwrap();
    assert_eq!(o, LinkOffsets { binding: 0, context: 1, program: 3, pre: 5, post: 7, rule_tag: 9, charge: 10, len: 11 });
    assert_eq!(link_offsets(16376, 1).unwrap().len, 16383);
    for (p, s) in [(0, 1), (1, 0), (16377, 1), (u32::MAX, u32::MAX)] {
        assert!(link_offsets(p, s).is_err(), "P {p} S {s}");
    }
}

#[test]
fn context_and_lane_hex_are_canonical_lowercase() {
    assert_eq!(ctx(&hex_ctx(1, 0)), [F::ONE, F::ZERO]);
    assert_eq!(ctx(&hex_ctx(0, 2)), [F::ZERO, fx(2)]);
    let mut b = [0u8; 32];
    b[15] = 1;
    assert_eq!(ctx(&hex::encode(b))[0], F::from_u128(1u128 << 120));
    let h = hex_ctx(0xab, 0);
    for bad_hex in [h.to_uppercase(), format!("0x{}", &h[2..]), h[..63].to_string(), format!("g{}", &h[1..])] {
        assert!(context_limbs(&bad_hex).is_err(), "{bad_hex}");
    }
    let x = fx(0xab);
    assert_eq!(f_from_canonical_hex(&f_to_hex(&x)).unwrap(), x);
    // The lax util parser accepts these; the declared encoding refuses them.
    for lax in [f_to_hex(&x).to_uppercase(), format!("0x{}", f_to_hex(&x))] {
        assert_eq!(f_from_hex(&lax).unwrap(), x);
        assert!(f_from_canonical_hex(&lax).is_err(), "{lax}");
    }
    assert!(f_from_canonical_hex(&"f".repeat(64)).is_err(), "above the modulus");
}

#[test]
fn fixed_bases_share_only_the_checked_minus_g() {
    let m = |kv: &[(&str, u64)]| kv.iter().map(|(k, v)| (k.to_string(), *v)).collect::<BTreeMap<String, u64>>();
    let a = m(&[("-G", 7), ("a_f0", 1), ("a_p0", 2)]);
    let b = m(&[("-G", 7), ("b_f0", 3)]);
    let merged = merge_fixed_bases("a", a.clone(), "b", b.clone(), &7).unwrap();
    assert_eq!(merged, m(&[("-G", 7), ("a_f0", 1), ("a_p0", 2), ("b_f0", 3)]));
    let refuse = |an: &str, x: &BTreeMap<String, u64>, bn: &str, y: &BTreeMap<String, u64>| {
        merge_fixed_bases(an, x.clone(), bn, y.clone(), &7).is_err()
    };
    assert!(refuse("a", &a, "b", &m(&[("-G", 7), ("a_f0", 1)])), "same name and base still refused");
    assert!(refuse("a", &a, "b", &m(&[("-G", 7), ("a_p0", 9)])), "no silent replacement");
    assert!(refuse("a", &a, "b", &m(&[("-G", 8), ("b_f0", 3)])), "-G not the negated generator");
    assert!(refuse("a", &a, "b", &m(&[("b_f0", 3)])), "-G missing from A80");
    assert!(refuse("a", &m(&[("a_f0", 1)]), "b", &b), "-G missing from A102");
    assert!(refuse("a", &a, "a", &b), "one prefix");
}

#[test]
fn lanes_bind_contexts_program_continuity_and_envelopes() {
    let o = link_offsets(2, 2).unwrap();
    let (ca, cb) = (ctx(&hex_ctx(1, 0)), ctx(&hex_ctx(0, 2)));
    let (a, b) = lanes(&ca, &cb);
    let ok = |a: &[F], b: &[F]| check_link_lanes(&o, &ca, &cb, a, b).is_ok();
    let tweak = |v: &[F], i: usize, x: F| {
        let mut v = v.to_vec();
        v[i] = x;
        v
    };
    assert!(ok(&a, &b));
    assert!(!ok(&b, &a), "children swapped");
    assert!(!ok(&tweak(&a, 2, fx(5)), &b), "A102 context");
    assert!(!ok(&a, &tweak(&b, 1, fx(5))), "A80 context");
    assert!(!ok(&a, &tweak(&b, 4, fx(5))), "ProgramBlock");
    assert!(!ok(&tweak(&a, 8, fx(5)), &b), "A102.post vs A80.pre");
    assert!(!ok(&a, &tweak(&b, 6, fx(5))), "A80.pre vs A102.post");
    assert!(!ok(&tweak(&a, 9, fx(0)), &b), "A102 rule_tag");
    assert!(!ok(&a, &tweak(&b, 10, fx(1))), "A80 charge");
    assert!(!ok(&a[..10], &b[..10]), "shape");
    // Bindings and the A80 post are not lane facts; proof verification binds them.
    assert!(ok(&a, &tweak(&tweak(&b, 0, fx(5)), 8, fx(5))));
}

#[test]
fn heights_advance_one_per_step_below_two_to_the_64() {
    assert_eq!(link_heights(0).unwrap(), [0, 1, 2]);
    let top = u64::MAX - 2;
    assert_eq!(link_heights(top).unwrap(), [top, top + 1, u64::MAX]);
    assert!(link_heights(top + 1).is_err() && link_heights(u64::MAX).is_err());
}

#[test]
fn statement_layout_and_ordered_height_trace() {
    let o = link_offsets(2, 2).unwrap();
    let (ca, cb) = (ctx(&hex_ctx(1, 0)), ctx(&hex_ctx(0, 2)));
    let (a, b) = lanes(&ca, &cb);
    let st = |a: &[F], b: &[F], h: u64| link_statement(fx(99), &o, a, b, link_heights(h).unwrap());
    let s = st(&a, &b, 10);
    assert_eq!(s.len(), LINK_STATE_FIELDS);
    assert_eq!(s[..5], [fx(99), ca[0], ca[1], cb[0], cb[1]]);
    assert_eq!(s[5], tagged(DOM_PROG, &a[3..5]));
    assert_eq!((s[6], s[8], s[9]), (state_root(&a[5..7]), state_root(&b[5..7]), state_root(&b[7..9])));
    assert_eq!(s[8], state_root(&a[7..9]));
    assert_eq!((s[7], s[10], s[11], s[12], s[13], s[14]), (fx(10), fx(12), a[0], b[0], fx(2), fx(1)));
    // TRACE_ORDER is recomputable from statement fields and envelope constants alone.
    let trace = poseidon(&[
        dom(DOM_TRACE), s[0], s[1], s[2], s[3], s[4], s[5], s[6], s[7], s[11], fx(3), fx(1),
        s[8], s[7] + F::ONE, s[12], fx(0), fx(0), s[9], s[10], s[13], s[14],
    ]);
    assert_eq!(s[15], trace);
    assert_ne!(st(&a, &b, 11)[15], s[15], "trace commits the heights");
    let rebind = |v: &[F], x: F| {
        let mut v = v.to_vec();
        v[0] = x;
        v
    };
    let t = st(&rebind(&a, b[0]), &rebind(&b, a[0]), 10);
    assert_eq!((t[11], t[12]), (s[12], s[11]));
    assert_ne!(t[15], s[15], "trace commits the step order of the bindings");
    let (mut a2, mut b2) = (a.clone(), b.clone());
    a2[7] = fx(43);
    b2[5] = fx(43);
    let u = st(&a2, &b2, 10);
    assert!(u[8] != s[8] && u[15] != s[15] && u[6] == s[6] && u[9] == s[9]);
}

#[test]
fn genesis_root_takes_only_the_a102_pre() {
    let o = link_offsets(2, 2).unwrap();
    let (ca, cb) = (ctx(&hex_ctx(1, 0)), ctx(&hex_ctx(0, 2)));
    let (a, b) = lanes(&ca, &cb);
    assert_eq!(genesis_root_lanes(&o, &ca, &a).unwrap(), state_root(&a[5..7]));
    assert!(genesis_root_lanes(&o, &ca, &b).is_err(), "an A80 pre is the mid root, never genesis");
    assert!(genesis_root_lanes(&o, &cb, &b).is_err(), "A80 envelope");
    assert!(genesis_root_lanes(&o, &ca, &a[..10]).is_err());
}

#[test]
fn reads_are_bounded() {
    let d = tmp("cap");
    put(&d, "x", &[7u8; 10]);
    assert_eq!(read_capped(&d.join("x"), 10).unwrap(), vec![7u8; 10]);
    assert!(read_capped(&d.join("x"), 9).unwrap_err().to_string().contains("exceeds 9 bytes"));
    assert!(read_capped(&d.join("missing"), 10).is_err());
    fs::remove_dir_all(&d).unwrap();
}

#[test]
fn children_reader_refuses_before_any_proof_authority() {
    let p = pins();
    let o = link_offsets(2, 2).unwrap();
    let (a, _) = lanes(&ctx(&p.a102_context_hash), &ctx(&p.a80_context_hash));
    let hexes: Vec<String> = a.iter().map(f_to_hex).collect();
    let json = |v: &serde_json::Value| serde_json::to_vec(v).unwrap();
    let lanes_json = |h: &[String]| serde_json::to_vec(h).unwrap();
    let d = tmp("children");
    put(&d, "a102.vk", b"not a key");
    put(&d, "a102.proof", b"p");
    // a102 lanes plus a link.json whose digests cover them and the proof.
    let stage = |b: &[u8]| {
        put(&d, "a102.instance.json", b);
        let m = manifest(&p, &o, b"p", b);
        put(&d, "link.json", &json(&m));
        m
    };
    let err = |q: &LinkPins| read_link_children(&d, q).unwrap_err().to_string();
    stage(&lanes_json(&hexes));
    assert!(err(&p).contains("pinned digest"), "canonical lanes reach the key pin");
    let mut up = hexes.clone();
    up[0] = up[0].to_uppercase();
    stage(&lanes_json(&up));
    assert!(err(&p).contains("lowercase"));
    stage(&vec![b' '; 80 * o.len + 65]);
    assert!(err(&p).contains("exceeds"));
    let honest = stage(&lanes_json(&hexes));
    type Edit = fn(&mut serde_json::Value);
    let refuse = |f: Edit| {
        let mut m = honest.clone();
        f(&mut m);
        put(&d, "link.json", &json(&m));
        err(&p)
    };
    let (role102, role80) = ("child a102 disagrees with its fixed role", "child a80 disagrees with its fixed role");
    let cases: &[(Edit, &str)] = &[
        (|m| m["children"][0]["instance_sha256"] = "0".repeat(64).into(), "differ from link.json"),
        (|m| m["children"][0]["proof_sha256"] = "0".repeat(64).into(), "differ from link.json"),
        (|m| m["children"][0]["proof_bytes"] = 2.into(), "differ from link.json"),
        (|m| m["format"] = "moriarty-sherlock-link/1".into(), "text differs"),
        (|m| m["disclosures"][4] = "x".into(), "text differs"),
        (|m| m["extra"] = 1.into(), "unknown field"),
        (|m| { m.as_object_mut().unwrap().remove("transcript"); }, "missing field"),
        (|m| m["offsets"]["pre"] = 6.into(), "checked lane counts"),
        (|m| m["genesis"]["state_offset"] = 3.into(), "genesis locator"),
        (|m| { m["children"].as_array_mut().unwrap().pop(); }, "exactly the a102 and a80"),
        (|m| m["children"][0]["vk_sha256"] = "9".repeat(64).into(), "child a102 disagrees with the pins"),
        (|m| m["children"][0]["context_hash"] = hex_ctx(9, 0).into(), "child a102 disagrees with the pins"),
        (|m| m["children"][0]["proof_bytes"] = 262_144.into(), "differ from link.json"),
        (|m| m["genesis"]["height"] = 1.into(), "genesis locator"),
        (|m| m["genesis"]["child"] = "a80".into(), "genesis locator"),
        (|m| m["genesis"]["root_authority"] = "x".into(), "genesis locator"),
        (|m| m["links"][0] = "x".into(), "text differs"),
        (|m| { m["open_obligations"].as_array_mut().unwrap().pop(); }, "text differs"),
    ];
    let role_cases: &[(Edit, &str)] = &[
        (|m| m["children"].as_array_mut().unwrap().swap(0, 1), role102),
        (|m| m["children"][0]["claim_file"] = "a80.claim.json".into(), role102),
        (|m| { m["children"][0]["open_obligations"].as_array_mut().unwrap().pop(); }, role102),
        (|m| { m["children"][1]["open_obligations"].as_array_mut().unwrap().pop(); }, role80),
        (|m| m["children"][0]["carrier_source_sha256"] = "f".repeat(64).into(), role102),
        (|m| m["children"][1]["carrier_source_sha256"] = serde_json::Value::Null, role80),
        (|m| m["children"][1]["zkir_hash"] = "D".repeat(64).into(), role80),
        (|m| m["children"][0]["circuit_k"] = 0.into(), role102),
        (|m| m["children"][0]["circuit_k"] = 20.into(), role102),
        (|m| m["children"][1]["circuit_k"] = 20.into(), role80),
        (|m| m["children"][0]["proof_bytes"] = 0.into(), role102),
        (|m| m["children"][0]["proof_bytes"] = 262_145.into(), role102),
        (|m| m["children"][0]["instance_lanes"] = 12.into(), role102),
        (|m| m["children"][1]["instance_lanes"] = 12.into(), role80),
        (|m| m["children"][0]["rule_tag"] = 0.into(), role102),
        (|m| m["children"][0]["charge"] = 0.into(), role102),
        (|m| m["children"][1]["rule_tag"] = 3.into(), role80),
        (|m| m["children"][1]["charge"] = 1.into(), role80),
        (|m| m["children"][0]["qualification"] = A80_QUALIFICATION.into(), role102),
        (|m| m["children"][1]["qualification"] = A102_QUALIFICATION.into(), role80),
        (|m| m["children"][0]["claim_format"] = A80_CLAIM_FORMAT.into(), role102),
        (|m| m["children"][1]["claim_format"] = A102_CLAIM_FORMAT.into(), role80),
        (|m| m["children"][0]["binding"] = A80_BINDING.into(), role102),
        (|m| m["children"][1]["binding"] = A102_BINDING.into(), role80),
        (|m| m["children"][0]["name"] = "a80".into(), role102),
        (|m| m["children"][1]["name"] = "a102".into(), role80),
    ];
    // Accepted carrier and k values reach the C3 key pin, the fake key's intentional early
    // refusal; that boundary is never verification.
    let (reach, missing) = ("pinned digest", "missing field `carrier_source_sha256`");
    let carrier_cases: &[(Edit, &str)] = &[
        (|m| { m["children"][0]["circuit_k"] = 19.into(); m["children"][1]["circuit_k"] = 19.into(); }, reach),
        (|m| m["children"][0]["carrier_source_sha256"] = serde_json::Value::Null, reach),
        (|m| { m["children"][0].as_object_mut().unwrap().remove("carrier_source_sha256"); }, missing),
        (|m| { m["children"][1].as_object_mut().unwrap().remove("carrier_source_sha256"); }, missing),
        (|m| m["children"][0]["carrier_source_sha256"] = 1.into(), "invalid type"),
        (|m| m["children"][1]["carrier_source_sha256"] = 1.into(), "invalid type"),
    ];
    for (f, want) in cases.iter().chain(role_cases).chain(carrier_cases) {
        let e = refuse(*f);
        assert!(e.contains(*want), "{want}: {e}");
    }
    put(&d, "link.json", &json(&honest));
    let mut q = p.clone();
    q.state_lanes = 3;
    assert!(err(&q).contains("pinned shape"));
    for (raw, want) in [
        (br#"{"format":1,"\u0066ormat":2}"#.to_vec(), "duplicate decoded key"),
        ([json(&honest), b" []".to_vec()].concat(), "trailing"),
        ([vec![b'['; 40], vec![b']'; 40]].concat(), "depth limit"),
        (vec![b' '; (1 << 20) + 1], "exceeds"),
        ([b"[".to_vec(), vec!["0"; 65_536].join(",").into_bytes(), b"]".to_vec()].concat(), "work/depth limit"),
        (vec![b'"', 0xff, b'"'], "utf-8"),
    ] {
        put(&d, "link.json", &raw);
        assert!(err(&p).contains(want), "{want}");
    }
    // 65,536 nodes (the array and 65,535 elements) are within the bound; typed decode refuses.
    put(&d, "link.json", &[b"[".to_vec(), vec!["0"; 65_535].join(",").into_bytes(), b"]".to_vec()].concat());
    assert!(!err(&p).contains("limit"), "node bound refuses only above 65,536");
    // Proof length alone: the manifest's digest covers the actual two-byte proof.
    put(&d, "a102.proof", b"pq");
    let mut m = manifest(&p, &o, b"pq", &lanes_json(&hexes));
    put(&d, "link.json", &json(&m));
    assert!(err(&p).contains("pinned digest"), "matching length and digest reach the key pin");
    m["children"][0]["proof_bytes"] = 1.into();
    put(&d, "link.json", &json(&m));
    assert!(err(&p).contains("differ from link.json"), "length differs; proof digest matches");
    put(&d, "a102.proof", b"p");
    let mut q = p.clone();
    q.a80_context_hash = q.a102_context_hash.clone();
    assert!(err(&q).contains("coincide"));
    let mut q = p.clone();
    q.a102_vk_sha256 = q.a102_vk_sha256.to_uppercase();
    assert!(err(&q).contains("key digest pins"));
    fs::remove_dir_all(&d).unwrap();
}

#[test]
#[ignore = "needs two independently pinned Moriarty child keys; nonproving"]
fn real_child_keys_share_one_minus_g_and_entrypoints_refuse_bypass() {
    let env = |n: &str| std::env::var(n).unwrap_or_else(|_| panic!("{n} must be set explicitly"));
    let vk = |n: &str| vk_from_bytes(&fs::read(env(n)).unwrap()).unwrap();
    let p = LinkPins {
        a102_vk_sha256: env("SHERLOCK_A102_VK_SHA256"),
        a80_vk_sha256: env("SHERLOCK_A80_VK_SHA256"),
        a102_context_hash: env("SHERLOCK_A102_CONTEXT_HASH"),
        a80_context_hash: env("SHERLOCK_A80_CONTEXT_HASH"),
        program_lanes: env("SHERLOCK_PROGRAM_LANES").parse().unwrap(),
        state_lanes: env("SHERLOCK_STATE_LANES").parse().unwrap(),
    };
    let rel = SherlockLinkRelation::from_pins(vk("SHERLOCK_A102_VK"), vk("SHERLOCK_A80_VK"), &p).unwrap();
    relation_bytes(&rel).unwrap();
    let fa = fixed_bases::<S>(&rel.a102_name(), rel.a102_vk.vk());
    let fb = fixed_bases::<S>(&rel.a80_name(), rel.a80_vk.vk());
    let merged = rel.fixed_base_values().unwrap();
    assert_eq!(merged.len(), fa.len() + fb.len() - 1);
    assert_eq!(merged.get(SHARED_BASE), Some(&-<C as Group>::generator()));
    assert!(fa.iter().chain(&fb).all(|(k, v)| merged.get(k) == Some(v)));
    let o = rel.offsets().unwrap();
    let mut i = vec![F::ZERO; o.len];
    i[o.context..o.program].copy_from_slice(&rel.a102_context);
    i[o.rule_tag] = fx(3);
    i[o.charge] = F::ONE;
    let child = |k: &MidnightVK| SherlockChild { vk: k.clone(), name: "fixture".into(), instance: i.clone(), proof: vec![1] };
    assert_eq!(genesis_root(&rel, &child(&rel.a102_vk)).unwrap(), state_root(&i[o.pre..o.post]));
    assert!(genesis_root(&rel, &child(&rel.a80_vk)).is_err(), "the A80 key is never genesis");
    let w = SherlockLinkWitness { a102_instance: i.clone(), a102_proof: vec![1], a80_instance: i.clone(), a80_proof: vec![1], h_pre: 0 };
    let mut one_key = rel.clone();
    one_key.a80_vk = rel.a102_vk.clone();
    let mut one_ctx = rel.clone();
    one_ctx.a80_context = rel.a102_context;
    let mut wide = rel.clone();
    wide.a102_context[1] = F::from_u128(u128::MAX) + F::ONE;
    for r in [&one_key, &one_ctx, &wide] {
        assert!(r.keys_id().is_err() && r.fixed_base_values().is_err());
        assert!(r.check_witness(&w).is_err() && r.statement(&w).is_err() && r.link_acc_offcircuit(&w).is_err());
    }
}

#[test]
fn parent_proof_phase_records_have_only_fixed_labels_and_elapsed() {
    let phases = [
        (ParentProofPhase::Model, "model"), (ParentProofPhase::ChildA102Verify, "child_a102_verify"),
        (ParentProofPhase::ChildA80Verify, "child_a80_verify"), (ParentProofPhase::AccumulatorFold, "accumulator_fold"),
        (ParentProofPhase::FoldedPairing, "folded_pairing"), (ParentProofPhase::SetupPk, "setup_pk"),
        (ParentProofPhase::Prove, "prove"), (ParentProofPhase::StdlibVerify, "stdlib_verify"),
        (ParentProofPhase::PairingVerify, "pairing_verify"), (ParentProofPhase::Write, "write"),
    ];
    for (phase, label) in phases {
        for complete in [false, true] {
            let record = parent_proof_phase_record(phase, complete, Duration::from_millis(1234));
            assert_eq!(record, serde_json::json!({"kind": "sherlock-parent-proof-phase", "stage": label,
                "boundary": if complete { "complete" } else { "start" }, "elapsed_seconds": 1.234}));
            assert_eq!(record.as_object().unwrap().len(), 4, "no payload or error fields");
        }
    }
}

fn assert_same_own(a: &Accumulator<S>, b: &Accumulator<S>) {
    for (lhs, rhs) in [(a.lhs(), b.lhs()), (a.rhs(), b.rhs())] {
        assert_eq!(lhs.bases(), rhs.bases(), "ordered original bases");
        assert_eq!(lhs.scalars(), rhs.scalars(), "ordered original scalars");
        assert_eq!(lhs.fixed_base_scalars(), rhs.fixed_base_scalars(), "complete named fixed-base map");
    }
}

#[test]
#[ignore = "actual pinned linked children and pinned 2^14 SRS; transcript/verification work, separately selected"]
fn verified_child_owns_preserve_the_original_combined_instance() {
    use midnight_proofs::utils::SerdeFormat;
    use std::io::BufReader;
    let env = |n: &str| std::env::var(n).unwrap_or_else(|_| panic!("{n} must be set explicitly"));
    let p = LinkPins {
        a102_vk_sha256: env("SHERLOCK_A102_VK_SHA256"), a80_vk_sha256: env("SHERLOCK_A80_VK_SHA256"),
        a102_context_hash: env("SHERLOCK_A102_CONTEXT_HASH"), a80_context_hash: env("SHERLOCK_A80_CONTEXT_HASH"),
        program_lanes: env("SHERLOCK_PROGRAM_LANES").parse().unwrap(),
        state_lanes: env("SHERLOCK_STATE_LANES").parse().unwrap(),
    };
    let (mut a, mut b) = read_link_children(Path::new(&env("SHERLOCK_LINK_CHILDREN_DIR")), &p).unwrap();
    let rel = SherlockLinkRelation::from_pins(a.vk.clone(), b.vk.clone(), &p).unwrap();
    let w = SherlockLinkWitness::from_children(&a, &b, 0);
    let bytes = fs::read(Path::new(&env("MIDNIGHT_SRS_DIR")).join("bls_midnight_2p14")).unwrap();
    assert_eq!(sha256_hex(&bytes), "fc253016885ec830e97808c9ec920bb5cab5c21af590380a6cb5eb0538e2b244");
    let vp = ParamsKZG::<Bls12>::read_custom(&mut BufReader::new(&bytes[..]), SerdeFormat::RawBytesUnchecked)
        .unwrap().verifier_params();
    a.name = rel.a102_name();
    b.name = rel.a80_name();
    let own_a = verify_sherlock_child_own_offcircuit(&vp, &a).unwrap();
    let own_b = verify_sherlock_child_own_offcircuit(&vp, &b).unwrap();
    let fresh_a = prepare_own_offcircuit(&a.vk, &a.name, &a.instance, &a.proof).unwrap();
    let fresh_b = prepare_own_offcircuit(&b.vk, &b.name, &b.instance, &b.proof).unwrap();
    assert_same_own(&own_a, &fresh_a);
    assert_same_own(&own_b, &fresh_b);
    assert_eq!(acc_pis(&own_a), acc_pis(&fresh_a));
    assert_eq!(acc_pis(&own_b), acc_pis(&fresh_b));
    let reversed = rel.fold_verified_own([own_b.clone(), own_a.clone()]).unwrap();
    assert_same_own(&own_a, &fresh_a);
    assert_same_own(&own_b, &fresh_b);
    let folded = rel.fold_verified_own([own_a, own_b]).unwrap();
    assert!(folded.check(&vp, &BTreeMap::new()));
    assert_ne!(acc_pis(&folded), acc_pis(&reversed), "ordered fold transcript");
    let reused = [rel.statement(&w).unwrap(), acc_pis(&folded)].concat();
    assert_eq!(reused, rel.instance(&w).unwrap(), "original reference independently prepares both transcripts again");
}
