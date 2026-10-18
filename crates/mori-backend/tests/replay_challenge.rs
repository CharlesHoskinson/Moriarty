//! Local reference challenge output. No proof, parameter or ledger APIs.
use mori_backend::{
    Target,
    authorization::{ExpectedAuthority, SignedIntent},
    replay::{self, ReplayAuthorization as Auth, ReplayError, ReplayRecord},
};
use mori_core::{Scenario, UInt};
use serde_json::{Value, json};
struct Roots {
    source: String,
    scenario: Scenario,
    target: Target,
}
impl Roots {
    fn new() -> Self {
        let o: Value = serde_json::from_str(include_str!(
            "../../mori-core/tests/fixtures/bound-replay-oracle.json"
        ))
        .unwrap();
        let c = &o["contrasting"];
        Self {
            source: c["source"].as_str().unwrap().into(),
            scenario: serde_json::from_value(c["scenario"].clone()).unwrap(),
            target: serde_json::from_value(c["unsigned_target"].clone()).unwrap(),
        }
    }
    fn record(&self) -> ReplayRecord {
        replay::generate(
            &self.source,
            "collect",
            &self.scenario,
            &self.target,
            Auth::Unsigned,
        )
        .unwrap()
    }
    fn challenge(&self, r: &ReplayRecord) -> Result<Option<replay::ReplayChallenge>, ReplayError> {
        replay::challenge(
            &self.source,
            "collect",
            &self.scenario,
            &self.target,
            Auth::Unsigned,
            r,
        )
    }
}
fn edit(r: &ReplayRecord, path: &str, value: Value) -> ReplayRecord {
    let mut v = serde_json::to_value(r).unwrap();
    *v.pointer_mut(path).unwrap() = value;
    ReplayRecord::from_json(&v.to_string()).unwrap()
}
#[test]
fn challenge_binds_trusted_context_and_candidate_bytes_with_early_semantics() {
    let roots = Roots::new();
    let actual = roots.record();
    assert!(roots.challenge(&actual).unwrap().is_none());
    let mut bad = edit(
        &actual,
        "/body/execution/steps/1/effects/0/amount",
        json!("4"),
    );
    bad.body.execution.steps[7].index = 99;
    bad.record_hash = "00".repeat(32);
    bad.body.claim.public_values[0] = "00".repeat(32);
    let c = roots.challenge(&bad).unwrap().unwrap();
    assert_eq!(c.divergence.step, Some(1));
    assert_eq!(
        c.divergence.field,
        "/body/execution/steps/1/effects/0/amount"
    );
    assert_eq!(c.trusted.record_hash, actual.record_hash);
    assert_eq!(c.trusted.input_hash, actual.body.input_hash);
    assert_eq!(c.trusted.target_hash, actual.body.claim.target_hash);
    assert_eq!(c.trusted.scenario_hash, actual.body.claim.scenario_hash);
    assert_eq!(
        c.candidate_digest,
        mori_core::intent::hash_bytes(&mori_core::intent::canonical(&bad).unwrap())
    );
    assert_ne!(c.candidate_digest, bad.record_hash);
    assert!(c.to_json().unwrap().len() < replay::MAX_CHALLENGE_BYTES);
    let mut late = bad.clone();
    late.body.execution.steps[7].index = 98;
    assert_ne!(
        roots.challenge(&late).unwrap().unwrap().candidate_digest,
        c.candidate_digest
    );
}
#[test]
fn missing_extra_and_last_steps_derive_ordinal_from_traversal() {
    let roots = Roots::new();
    let actual = roots.record();
    for (mut bad, ordinal) in [(actual.clone(), 7), (actual.clone(), 8)] {
        if ordinal == 7 {
            bad.body.execution.steps.pop();
        } else {
            bad.body
                .execution
                .steps
                .push(actual.body.execution.steps[7].clone());
        }
        let c = roots.challenge(&bad).unwrap().unwrap();
        assert_eq!(c.divergence.step, Some(ordinal));
        assert_eq!(
            c.divergence.field,
            format!("/body/execution/steps/{ordinal}")
        );
    }
    let mut bad = actual.clone();
    bad.body.execution.steps[7].index = u16::MAX;
    assert_eq!(
        roots.challenge(&bad).unwrap().unwrap().divergence.step,
        Some(7)
    );
}
#[test]
fn malformed_typed_record_and_invalid_trusted_inputs_stay_input_failures() {
    let mut roots = Roots::new();
    let r = roots.record();
    let mut bad = r.clone();
    bad.record_hash = "not-a-digest".into();
    assert!(matches!(roots.challenge(&bad), Err(ReplayError::Input(_))));
    roots.source = "bad source".into();
    assert!(matches!(roots.challenge(&r), Err(ReplayError::Input(_))));
    let mut roots = Roots::new();
    roots.target.network = "different-domain".into();
    assert!(matches!(roots.challenge(&r), Err(ReplayError::Input(_))));
    let roots = Roots::new();
    let bad = edit(&r, "/body/anchor", json!("other-domain"));
    let c = roots.challenge(&bad).unwrap().unwrap();
    assert_eq!(c.divergence.field, "/body/anchor");
    assert_eq!(c.trusted.anchor, roots.target.anchor);
}
#[test]
fn encoded_byte_bound_includes_ascii_escaping_and_newline() {
    let mut roots = Roots::new();
    roots.target.anchor = "🦀".repeat(256);
    let r = roots.record();
    let bad = edit(&r, "/body/execution/steps/0/index", json!(99));
    let c = roots.challenge(&bad).unwrap().unwrap();
    assert!(c.to_json().is_err());
    let roots = Roots::new();
    let bad = edit(&roots.record(), "/body/execution/steps/0/index", json!(99));
    let mut c = roots.challenge(&bad).unwrap().unwrap();
    let original_len = c.to_json().unwrap().len();
    c.trusted
        .anchor
        .push_str(&"x".repeat(replay::MAX_CHALLENGE_BYTES - original_len - 1));
    assert_eq!(c.to_json().unwrap().len() + 1, replay::MAX_CHALLENGE_BYTES);
    c.trusted.anchor.push('x');
    assert!(c.to_json().is_err());
    c.divergence.field = "x".repeat(replay::MAX_CHALLENGE_BYTES);
    assert!(c.to_json().is_err());
}
#[test]
fn signed_external_attachment_is_validated_and_never_exported() {
    let source = include_str!(
        "../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.mori"
    );
    let scenario: Scenario = serde_json::from_str(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.scenario.json")).unwrap();
    let target = Target::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.target.json")).unwrap();
    let authority = ExpectedAuthority::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.authority.json")).unwrap();
    let signature = SignedIntent::from_json(include_str!("../../../deliverables/native-owner-intent-2026-10-11/inputs/independent-payment.signed.json")).unwrap();
    let auth = Auth::Signed {
        authority: &authority,
        at: UInt(15),
        signature: &signature,
    };
    let r = replay::generate(source, "settle", &scenario, &target, auth).unwrap();
    let mut bad = r.clone();
    bad.body.execution.steps[0].index = 99;
    let c = replay::challenge(source, "settle", &scenario, &target, auth, &bad)
        .unwrap()
        .unwrap();
    assert_eq!(c.trusted.authority_hash, r.body.authority_hash);
    assert_eq!(c.trusted.at, Some(UInt(15)));
    let text = c.to_json().unwrap();
    assert!(!text.contains(&signature.signature_hex));
    assert!(!text.contains(&signature.signature_hex[..128]));
    assert!(!text.contains(&signature.signature_hex[128..]));
    for token in [
        "signature_hex",
        "message_hex",
        "proof_hex",
        "private_inputs",
        "private_transcript",
        "public_key_hex",
    ] {
        assert!(!text.contains(token), "{token}");
    }
    let mut changed = authority.clone();
    changed.predecessor = "other-predecessor".into();
    assert!(matches!(
        replay::challenge(
            source,
            "settle",
            &scenario,
            &target,
            Auth::Signed {
                authority: &changed,
                at: UInt(15),
                signature: &signature
            },
            &bad
        ),
        Err(ReplayError::Input(_))
    ));
    let mut invalid = signature.clone();
    invalid.signature_hex = "00".repeat(96);
    assert!(matches!(
        replay::challenge(
            source,
            "settle",
            &scenario,
            &target,
            Auth::Signed {
                authority: &authority,
                at: UInt(15),
                signature: &invalid
            },
            &bad
        ),
        Err(ReplayError::Input(_))
    ));
}
