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
            let pos = v["circuits"][p]["pending_positions"]["post_site"].as_u64().unwrap();
            v["circuits"][p]["public_inputs"][pos as usize]["constant"] = "7".into();
        },
        // A pending position moved onto an ordinary binding.
        |v, p, _| v["circuits"][p]["pending_positions"]["pre_site"] = 0.into(),
        // A zero-await circuit claiming a pending frame.
        |v, p, z| {
            v["circuits"][z]["pending_positions"] =
                v["circuits"][p]["pending_positions"].clone()
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
