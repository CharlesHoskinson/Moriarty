//! `/4` relation-shape security checks over compiled and decoded artifacts.
//! Source-only: these tests never invoke SDK check/model, VM or proving, and
//! they establish no event authentication, stage authority or consumption.
use mori_backend::{
    Target,
    stage::{STAGE_GATES, StagedArtifact, compile_staged},
};
use serde_json::Value;
use std::path::Path;

const RESERVE: &str = r#"language "moriarty/2";
agreement ReserveContinuationOracle {
  asset CREDIT = { network: "local", issuer: "continuation-reserve-oracle", reference: "UNIT", scale: 0 };
  state basis: UInt = 40;
  state allocated: UInt = 0;
  state marker: UInt = 77;
  action allocate(extra: UInt) {
    let held = basis + extra;
    next basis = basis - 5;
    liability("allocation", CREDIT, "reserve", "target", held);
    await approval(quote: UInt) when quote >= held;
    next allocated = held;
    transfer(CREDIT, "reserve", "target", held);
    discharge("allocation", held);
    next basis = basis + quote;
    await settlement(ready: Bool) when ready;
    requires quote <= 50;
    fee(CREDIT, "reserve", "treasury", extra);
    emit "settled_quote"(quote);
  }
}
"#;
const MIXED: &str = r#"language "moriarty/2";
agreement Mixed {
  state n: UInt = 0;
  action a(x: UInt) { await go(ready: Bool) when ready; next n = x; }
  action b() { next n = n + 1; }
}
"#;
const PENDING: [&str; 5] = [
    "pre_pending_site",
    "pre_pending_capture",
    "event",
    "post_pending_site",
    "post_pending_capture",
];

fn compile(source: &str) -> StagedArtifact {
    compile_staged(
        source,
        &Target::ledger9("local", "stage-oracle"),
        Path::new("unused-output"),
    )
    .unwrap()
}

fn decode(value: &Value) -> Result<StagedArtifact, serde_json::Error> {
    serde_json::from_value(value.clone())
}

/// Pending roles occur exactly at the recorded positions, with the site and
/// padding constants fixed by the layout, and never in a zero-await circuit.
#[test]
fn pending_roles_occur_only_at_recorded_positions() {
    for source in [RESERVE, MIXED] {
        for c in compile(source).circuits {
            let recorded: Vec<u32> = match &c.pending_positions {
                None => Vec::new(),
                Some(p) => [p.pre_site]
                    .into_iter()
                    .chain(p.pre_captures.iter().copied())
                    .chain(p.events.iter().copied())
                    .chain([p.post_site])
                    .chain(p.post_captures.iter().copied())
                    .collect(),
            };
            let found: Vec<u32> = c
                .public_inputs
                .iter()
                .enumerate()
                .filter(|(_, b)| PENDING.contains(&b.role.as_str()))
                .map(|(i, _)| i as u32)
                .collect();
            let mut sorted = recorded.clone();
            sorted.sort_unstable();
            assert_eq!(found, sorted, "{} segment {}", c.action, c.segment);
            let Some(p) = &c.pending_positions else {
                assert_eq!((c.entry_site, c.exit_site), (0, 0));
                continue;
            };
            let at = |i: u32| &c.public_inputs[i as usize];
            let entry = c.entry_site.to_string();
            let exit = c.exit_site.to_string();
            assert_eq!(at(p.pre_site).constant.as_deref(), Some(entry.as_str()));
            assert_eq!(at(p.post_site).constant.as_deref(), Some(exit.as_str()));
            assert_eq!(p.pre_captures.len(), usize::from(c.capture_width));
            assert_eq!(p.post_captures.len(), usize::from(c.capture_width));
            assert_eq!(p.events.len(), c.event.len());
            for (i, &pos) in p.pre_captures.iter().enumerate() {
                let padded = i >= c.capture_in.len();
                assert_eq!(at(pos).constant.as_deref(), padded.then_some("0"));
            }
            for (i, &pos) in p.post_captures.iter().enumerate() {
                let padded = i >= c.capture_out.len();
                assert_eq!(at(pos).constant.as_deref(), padded.then_some("0"));
            }
            for &pos in &p.events {
                assert_eq!(at(pos).constant, None);
            }
        }
    }
}

/// Each circuit has its own context; all ten gates stay unperformed.
#[test]
fn contexts_are_per_circuit_and_gates_unperformed() {
    let artifact = compile(RESERVE);
    let mut contexts: Vec<_> = artifact.circuits.iter().map(|c| &c.context_hash).collect();
    contexts.sort();
    contexts.dedup();
    assert_eq!(contexts.len(), artifact.circuits.len());
    assert_eq!(artifact.gates.len(), 10);
    assert!(artifact.gates.iter().all(|g| g.status == "unperformed"));
    for predicate in STAGE_GATES {
        assert!(artifact.gates.iter().any(|g| g.predicate == predicate));
    }
}

/// The strict decoder accepts the compiled artifact and refuses forged
/// pending frames, unknown fields, missing nullable keys and performed gates.
#[test]
fn decoder_refuses_forged_frames_and_fields() {
    let honest = serde_json::to_value(compile(MIXED)).unwrap();
    decode(&honest).unwrap();
    let pending = |v: &mut Value| -> usize {
        v["circuits"]
            .as_array()
            .unwrap()
            .iter()
            .position(|c| !c["pending_positions"].is_null())
            .unwrap()
    };
    let zero = |v: &Value| -> usize {
        v["circuits"]
            .as_array()
            .unwrap()
            .iter()
            .position(|c| c["pending_positions"].is_null())
            .unwrap()
    };
    let forgeries: [fn(&mut Value, usize, usize); 6] = [
        // A post site constant that is not the exit site.
        |v, p, _| {
            let pos = v["circuits"][p]["pending_positions"]["post_site"]
                .as_u64()
                .unwrap();
            v["circuits"][p]["public_inputs"][pos as usize]["constant"] = "7".into();
        },
        // A pending position moved onto an ordinary binding.
        |v, p, _| v["circuits"][p]["pending_positions"]["pre_site"] = 0.into(),
        // A zero-await circuit claiming a pending frame.
        |v, p, z| {
            v["circuits"][z]["pending_positions"] = v["circuits"][p]["pending_positions"].clone()
        },
        // The required nullable key is absent.
        |v, _, z| {
            v["circuits"][z]
                .as_object_mut()
                .unwrap()
                .remove("pending_positions");
        },
        |v, _, _| v["circuits"][0]["unknown"] = true.into(),
        |v, _, _| v["gates"][9]["status"] = "performed".into(),
    ];
    for (i, forge) in forgeries.into_iter().enumerate() {
        let mut forged = honest.clone();
        let (p, z) = (pending(&mut forged), zero(&forged));
        forge(&mut forged, p, z);
        assert!(decode(&forged).is_err(), "forgery {i} decoded");
    }
}

/// The `/3` compilers refuse a moriarty/2 program with a typed profile error
/// instead of lowering or skipping an Await.
#[test]
fn legacy_compilers_refuse_staged_programs() {
    let sp = mori_core::stage::load(MIXED).unwrap();
    let target = Target::ledger9("local", "stage-oracle");
    let error = mori_backend::compile(sp.program(), &target).unwrap_err();
    assert_eq!(error.code, "mori::backend::program");
}

// Hostile and positive cases (F1/T1/T2).
use mori_backend::{
    ARTIFACT_FORMAT, PUBLIC_ABI_VERSION, RELATION_VERSION,
    stage::{
        STAGE_CONTEXT_FORMAT, STAGED_ABI_VERSION, STAGED_ARTIFACT_FORMAT, STAGED_RELATION_VERSION,
    },
};

/// An await whose binder is read only by its guard, beside a zero-await action.
const EMPTY_CAPTURE: &str = r#"language "moriarty/2";
agreement EmptyCapture {
  state n: UInt = 0;
  action tick() { await go(ready: Bool) when ready; next n = n + 1; }
  action idle() { next n = n + 1; }
}
"#;

fn refusal(value: &Value) -> String {
    decode(value).expect_err("forgery decoded").to_string()
}

fn contexts(artifact: &StagedArtifact) -> Vec<String> {
    artifact
        .circuits
        .iter()
        .map(|c| c.context_hash.clone())
        .collect()
}

/// A signed Target from the static owner-intent vector (data only).
fn signed_target() -> Target {
    let golden: Value = serde_json::from_str(include_str!(
        "../../../examples/core/native-owner-intent/golden-vector.json"
    ))
    .unwrap();
    Target::from_json(golden["target_canonical_utf8"].as_str().unwrap()).unwrap()
}

/// Freshly validated output still admits zero-await, empty-capture and
/// multi-await programs, and round-trips byte-identically through the decoder.
#[test]
fn fresh_validation_admits_every_valid_frozen_shape() {
    for source in [RESERVE, MIXED, EMPTY_CAPTURE] {
        let artifact = compile(source);
        let bytes = serde_json::to_vec_pretty(&artifact).unwrap();
        assert_eq!(bytes.len(), artifact.resources.artifact_bytes as usize);
        let decoded: StagedArtifact = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(serde_json::to_vec_pretty(&decoded).unwrap(), bytes);
    }
    let empty = compile(EMPTY_CAPTURE);
    let tick: Vec<_> = empty
        .circuits
        .iter()
        .filter(|c| c.action == "tick")
        .collect();
    assert_eq!(tick.len(), 2);
    for c in tick {
        assert_eq!(c.capture_width, 0);
        assert!(
            c.pending_positions
                .as_ref()
                .is_some_and(|p| p.pre_captures.is_empty())
        );
    }
    let idle: Vec<_> = empty
        .circuits
        .iter()
        .filter(|c| c.action == "idle")
        .collect();
    assert_eq!(idle.len(), 1);
    assert!(idle[0].pending_positions.is_none());
}

/// `compile_staged` refuses a signed Target at entry, before loading the
/// source; the same Target without its policy compiles.
#[test]
fn compile_staged_refuses_signed_target() {
    let signed = signed_target();
    assert!(signed.signed_policy.is_some());
    for source in [MIXED, "not a moriarty program"] {
        let e = compile_staged(source, &signed, Path::new("unused-output")).unwrap_err();
        assert_eq!(e.code, "mori::backend::unsupported");
        assert_eq!(
            e.message,
            "Staged moriarty/2 relations are unsigned; a signed Target is refused."
        );
    }
    let mut unsigned = signed;
    unsigned.signed_policy = None;
    compile_staged(MIXED, &unsigned, Path::new("unused-output")).unwrap();
}

/// A single Target or program field change moves every circuit's context,
/// while identical inputs reproduce identical contexts.
#[test]
fn contexts_bind_single_target_and_program_fields() {
    let base = compile(RESERVE);
    assert_eq!(contexts(&compile(RESERVE)), contexts(&base));
    let target = Target::ledger9("local", "stage-oracle");
    let mut anchor = target.clone();
    anchor.anchor = "stage-oracle-2".into();
    let mut instructions = target.clone();
    instructions.max_instructions += 1;
    let mut bytes = target.clone();
    bytes.max_artifact_bytes -= 1;
    for perturbed in [anchor, instructions, bytes] {
        let a = compile_staged(RESERVE, &perturbed, Path::new("unused-output")).unwrap();
        assert_eq!(
            (&a.program_hash, &a.plan_hash),
            (&base.program_hash, &base.plan_hash)
        );
        assert_eq!(a.circuits.len(), base.circuits.len());
        for (x, y) in contexts(&a).iter().zip(contexts(&base).iter()) {
            assert_ne!(x, y);
        }
    }
    // An initializer no action reads still changes the committed program.
    let source = RESERVE.replace("state marker: UInt = 77;", "state marker: UInt = 78;");
    assert_ne!(source, RESERVE);
    let a = compile(&source);
    assert_ne!(a.program_hash, base.program_hash);
    assert_eq!(a.circuits.len(), base.circuits.len());
    for (x, y) in contexts(&a).iter().zip(contexts(&base).iter()) {
        assert_ne!(x, y);
    }
}

/// `/4` versions and the stage context format are distinct from `/3`, and a
/// `/4` envelope carrying a `/3` version string refuses.
#[test]
fn staged_versions_are_domain_separated_from_v3() {
    assert_eq!(STAGE_CONTEXT_FORMAT, "moriarty-stage-context/1");
    let v3 = [ARTIFACT_FORMAT, PUBLIC_ABI_VERSION, RELATION_VERSION];
    for v4 in [
        STAGE_CONTEXT_FORMAT,
        STAGED_ARTIFACT_FORMAT,
        STAGED_ABI_VERSION,
        STAGED_RELATION_VERSION,
    ] {
        assert!(!v3.contains(&v4), "{v4}");
    }
    let honest = serde_json::to_value(compile(MIXED)).unwrap();
    let expected = format!(
        "mori::backend::artifact: Expected {STAGED_ARTIFACT_FORMAT}, {STAGED_ABI_VERSION} and {STAGED_RELATION_VERSION}"
    );
    for (key, legacy) in [
        ("format", ARTIFACT_FORMAT),
        ("abi", PUBLIC_ABI_VERSION),
        ("relation", RELATION_VERSION),
    ] {
        let mut forged = honest.clone();
        forged[key] = legacy.into();
        let message = refusal(&forged);
        assert!(message.contains(&expected), "{key}: {message}");
    }
}

fn bump_uint(v: &mut Value, by: u128) {
    let n: u128 = v.as_str().unwrap().parse().unwrap();
    *v = (n + by).to_string().into();
}

/// Each forgery leaves every earlier decoder check intact, so the refusal
/// must name its own predicate. The byte count is checked last and every
/// envelope keeps its exact serde shape, so neither layer can mask it.
#[test]
fn decoder_refusals_name_the_targeted_predicate() {
    let artifact = compile(RESERVE);
    let honest = serde_json::to_value(&artifact).unwrap();
    decode(&honest).unwrap();
    // Preconditions the forgeries rely on, checked on the honest artifact.
    let c = &artifact.circuits;
    assert_eq!(c.len(), 3);
    assert!(
        c.iter()
            .all(|c| c.action == "allocate" && c.pending_positions.is_some())
    );
    assert_eq!(c[1].capture_in, c[0].capture_out);
    assert!(!c[1].capture_in.is_empty() && !c[1].event.is_empty());
    assert_eq!(c[1].public_inputs[2].role, "pre_state");
    assert!(
        c[1].effect_frame
            .iter()
            .any(|e| !e.public_positions.is_empty())
    );
    assert!(!c[1].balance_frame.is_empty() && !c[1].liability_frame.is_empty());
    let pending =
        |v: &Value, key: &str| -> Value { v["circuits"][1]["pending_positions"][key].clone() };
    let a = r#""allocate""#;
    let artifact_error = |m: &str| format!("mori::backend::artifact: {m}");
    type Forge = Box<dyn Fn(&mut Value)>;
    let forgeries: Vec<(&str, Forge, String)> = vec![
        (
            "capture_in is not the predecessor's capture_out",
            Box::new(|v: &mut Value| {
                v["circuits"][1]["capture_in"].as_array_mut().unwrap().pop();
            }),
            artifact_error(&format!(
                "Capture layout of segment 1 of {a} is not the width-L prefix."
            )),
        ),
        (
            "input role in an advance segment",
            Box::new(|v: &mut Value| v["circuits"][1]["public_inputs"][2]["role"] = "input".into()),
            artifact_error("Advance segments have no input positions."),
        ),
        (
            "events in segment 0",
            Box::new(|v: &mut Value| v["circuits"][0]["event"] = v["circuits"][1]["event"].clone()),
            artifact_error("Event layout differs from the entry site."),
        ),
        (
            "pending position overlaps an effect position",
            Box::new(move |v: &mut Value| {
                let site = pending(v, "pre_site");
                let effects = v["circuits"][1]["effect_frame"].as_array_mut().unwrap();
                let e = effects
                    .iter_mut()
                    .find(|e| !e["public_positions"].as_array().unwrap().is_empty())
                    .unwrap();
                e["public_positions"][0] = site;
            }),
            artifact_error("Pending positions overlap other declared positions."),
        ),
        (
            "pending position overlaps a balance position",
            Box::new(move |v: &mut Value| {
                v["circuits"][1]["balance_frame"][0]["pre_position"] = pending(v, "post_site")
            }),
            artifact_error("Pending positions overlap other declared positions."),
        ),
        (
            "pending position overlaps a liability position",
            Box::new(move |v: &mut Value| {
                v["circuits"][1]["liability_frame"][0]["post_amount_position"] =
                    pending(v, "pre_captures")[0].clone()
            }),
            artifact_error("Pending positions overlap other declared positions."),
        ),
        (
            "pending block moved off its insertion point",
            Box::new(move |v: &mut Value| {
                let site = pending(v, "pre_site").as_u64().unwrap() as usize;
                let first = pending(v, "pre_captures")[0].as_u64().unwrap() as usize;
                v["circuits"][1]["public_inputs"]
                    .as_array_mut()
                    .unwrap()
                    .swap(site, first);
                let p = &mut v["circuits"][1]["pending_positions"];
                p["pre_site"] = first.into();
                p["pre_captures"][0] = site.into();
            }),
            artifact_error("Pending blocks are not at their frame insertion points."),
        ),
        (
            "pending role outside the declared positions",
            Box::new(|v: &mut Value| v["circuits"][1]["public_inputs"][2]["role"] = "event".into()),
            artifact_error("A pending role appears outside the declared positions."),
        ),
        (
            "committed_before tampered",
            Box::new(|v: &mut Value| {
                bump_uint(
                    &mut v["resources"]["logical"][0]["segments"][1]["committed_before"],
                    1,
                )
            }),
            artifact_error(&format!(
                "Logical work of segment 1 of {a} is inconsistent."
            )),
        ),
        (
            "plan_total tampered",
            Box::new(|v: &mut Value| bump_uint(&mut v["resources"]["logical"][0]["plan_total"], 1)),
            artifact_error(&format!("Plan total of {a} differs from its segments.")),
        ),
        (
            "qualification label tampered",
            Box::new(|v: &mut Value| v["qualification"] = "stage-qualified".into()),
            artifact_error("Staged qualification labels differ from the profile."),
        ),
        (
            "authentication label tampered",
            Box::new(|v: &mut Value| v["authentication"] = "authenticated".into()),
            artifact_error("Staged qualification labels differ from the profile."),
        ),
        (
            "consumption label tampered",
            Box::new(|v: &mut Value| v["consumption"] = "tracked".into()),
            artifact_error("Staged qualification labels differ from the profile."),
        ),
        (
            // +2 is never a fixed point, even across a decimal digit boundary.
            "artifact_bytes differs from the envelope size",
            Box::new(|v: &mut Value| {
                let n = v["resources"]["artifact_bytes"].as_u64().unwrap();
                v["resources"]["artifact_bytes"] = (n + 2).into();
            }),
            artifact_error("Staged artifact byte count is not its envelope size."),
        ),
    ];
    for (name, forge, expected) in &forgeries {
        let mut forged = honest.clone();
        forge(&mut forged);
        assert_ne!(forged, honest, "{name}: forgery changed nothing");
        let message = refusal(&forged);
        assert!(message.contains(expected.as_str()), "{name}: {message}");
    }
}

/// H20 in the other direction: the `/3` decoder refuses a serialized `/4`
/// artifact at its first `/3` version key. Artifact bytes keep serialization
/// order, so `format` refuses first. A `serde_json::Value` map is key-sorted
/// here (the lockfile has no `preserve_order`), so `abi` refuses first.
#[test]
fn legacy_decoder_refuses_staged_artifacts() {
    assert_eq!(ARTIFACT_FORMAT, "moriarty-zkir-artifact/3");
    assert_eq!(PUBLIC_ABI_VERSION, "moriarty-public-abi/3");
    let artifact = compile(MIXED);
    let honest = serde_json::to_value(&artifact).unwrap();
    decode(&honest).unwrap();
    assert_eq!(honest["format"], "moriarty-zkir-artifact/4");
    assert_eq!(honest["abi"], "moriarty-public-abi/4");
    let bytes = serde_json::to_vec_pretty(&artifact).unwrap();
    assert!(bytes.starts_with(b"{\n  \"format\": "));
    let e = serde_json::from_slice::<mori_backend::Artifact>(&bytes).unwrap_err();
    assert!(e.is_data(), "{e}");
    let at = format!(" at line {} column {}", e.line(), e.column());
    assert_eq!(
        e.to_string().strip_suffix(at.as_str()),
        Some("Expected moriarty-zkir-artifact/3; recompile for this backend revision.")
    );
    let e = serde_json::from_value::<mori_backend::Artifact>(honest).unwrap_err();
    assert!(e.is_data(), "{e}");
    assert_eq!(
        e.to_string(),
        "Expected moriarty-public-abi/3; recompile for this backend revision."
    );
}
