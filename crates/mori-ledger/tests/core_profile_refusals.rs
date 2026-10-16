//! A9/H19: ledger `/1` entries refuse an admitted `moriarty/2` program with the
//! typed `mori::ledger::profile: core1-required` identity before input budgets,
//! policy checks or lowering. Public fixtures only: no files, frames, keys,
//! parameters or VM. The expected identities are written out, not computed.
use midnight_curves::{JubjubSubgroup, pairing::group::Group};
use mori_ledger::{
    profile::*,
    signed::{self, FixedSignerPolicy, SignedStaticBindings, SignedVersion},
    static_compile::{self, StaticAuthorization},
};

const DELIVERY: &str = include_str!("../../../examples/core/continuations/delivery.mori");
const LEGACY: &str = include_str!("../../../examples/core/continuations/atomic-compatibility.mori");

fn bindings() -> StaticBindings {
    StaticBindings {
        abi: ABI.into(),
        source_hash: "a".repeat(64),
        program_hash: "b".repeat(64),
        abi_hash: "c".repeat(64),
        templates_hash: "d".repeat(64),
        toolchain: Toolchain::pinned(),
        limits: Limits::supported(),
        network: "local".into(),
        anchor: "local-unqualified".into(),
        address_hex: "".into(),
        payer: "alice".into(),
        assets: vec![],
        roles: vec![],
        duties: vec![],
        economic_bounds: None,
    }
}
/// `max_degree` 0 is refused by `NativeWorkLimits::validate`.
fn limits(max_degree: u8) -> NativeWorkLimits {
    NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/2".into(),
        max_degree,
        max_actions: 64,
        max_srs_bytes: 1,
        max_prover_key_bytes: 1,
        max_verifier_key_bytes: 1,
        max_input_bytes: 1_048_576,
        max_leaf_bytes: 16_777_216,
        max_artifact_bytes: 16_777_216,
        max_operations: 100_000,
        max_public_inputs: 65_536,
    }
}
/// Public curve generator only; no signing key exists in this file.
fn policy(profile: &str) -> FixedSignerPolicy {
    let (x, y) = signed::schnorr::coordinates(JubjubSubgroup::generator());
    FixedSignerPolicy {
        profile: profile.into(),
        owner: "alice".into(),
        beneficiary: "bob".into(),
        public_key_hex: format!(
            "{}{}",
            signed::schnorr::native_hex(x),
            signed::schnorr::native_hex(y)
        ),
    }
}
fn refusal<T>(result: mori_ledger::Result<T>) -> mori_ledger::Error {
    match result {
        Ok(_) => panic!("ledger entry accepted the program"),
        Err(error) => error,
    }
}
fn core1_required(error: mori_ledger::Error) {
    assert_eq!((error.layer, error.code), ("profile", "core1-required"));
    assert_eq!(error.to_string(), "mori::ledger::profile: core1-required");
}

#[test]
fn static_compile_refuses_core_two_for_every_authorization_before_limits_or_lowering() {
    let staged = mori_core::stage::load(DELIVERY).unwrap();
    for authorization in [
        StaticAuthorization::Unsigned,
        StaticAuthorization::UnsignedMappedRoles,
        StaticAuthorization::SignedV2 {
            signer_policy: policy(signed::POLICY),
        },
    ] {
        // Invalid limits would refuse as input/native-work-limits if read first.
        for max_degree in [1, 0] {
            core1_required(refusal(static_compile::compile(
                DELIVERY,
                staged.program(),
                &bindings(),
                &authorization,
                &limits(max_degree),
            )));
        }
    }
}

#[test]
fn signed_static_admission_refuses_core_two_before_policy_checks() {
    let staged = mori_core::stage::load(DELIVERY).unwrap();
    for version in [SignedVersion::LegacyV1, SignedVersion::ConsumingV2] {
        // A wrong policy profile would refuse as signed policy-profile if read first.
        for profile in [signed::POLICY, "not-a-signer-policy"] {
            core1_required(refusal(SignedStaticBindings::from_source(
                DELIVERY,
                staged.program(),
                &bindings(),
                &policy(profile),
                version,
            )));
        }
    }
}

#[test]
fn core_one_program_passes_the_profile_guard_to_the_next_typed_refusal() {
    let program = mori_core::parse(LEGACY).unwrap();
    let error = refusal(SignedStaticBindings::from_source(
        LEGACY,
        &program,
        &bindings(),
        &policy(signed::POLICY),
        SignedVersion::ConsumingV2,
    ));
    assert_eq!(
        (error.layer, error.code),
        ("native-relation", "source-identity")
    );
    let error = refusal(static_compile::compile(
        LEGACY,
        &program,
        &bindings(),
        &StaticAuthorization::Unsigned,
        &limits(0),
    ));
    assert_eq!((error.layer, error.code), ("input", "native-work-limits"));
}
