//! Source-static consumers only: no frames, native keys, parameters or VM.
use mori_ledger::{codec, profile::*, static_compile};
fn parser_bindings() -> StaticBindings {
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
#[test]
fn new_profile_rejects_explicit_null_without_changing_legacy_none() {
    let raw = serde_json::to_string(&parser_bindings()).unwrap();
    let legacy: StaticBindings = codec::json(&raw).unwrap();
    assert!(legacy.economic_bounds.is_none());
    assert!(
        static_compile::parse_static_bindings(&raw).is_err(),
        "new static profile must reject explicit economic_bounds:null"
    );
    let mut omitted = serde_json::to_value(legacy).unwrap();
    omitted.as_object_mut().unwrap().remove("economic_bounds");
    assert!(
        static_compile::parse_static_bindings(&omitted.to_string())
            .unwrap()
            .economic_bounds
            .is_none()
    );
}

const NUMERIC: &str = "language \"moriarty/1\";\nagreement StaticNumericDraft {\n  state zulu: UInt = 0;\n  state alpha: Bool = true;\n  action advance(amount: UInt, allow: Bool) {\n    requires allow;\n    let before = zulu;\n    next zulu = zulu + amount;\n    next alpha = not alpha;\n    ensures zulu == before + amount;\n    emit \"flag\"(alpha);\n  }\n  action reset() {\n    next zulu = 0;\n    next alpha = true;\n    ensures alpha;\n    emit \"flag\"(alpha);\n  }\n}\n";
const NUMERIC_ADVANCE_UNSIGNED: &[(&str, usize, &str)] = &[
    ("abi-root", 1, "Dup"),
    ("abi-root", 1, "Size"),
    ("abi-root", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("source-event-push", 14, "Push"),
    ("source-event-log", 1, "Log"),
];
const NUMERIC_ADVANCE_SIGNED_V2: &[(&str, usize, &str)] = &[
    ("signed-abi-root", 1, "Dup"),
    ("signed-abi-root", 1, "Size"),
    ("signed-abi-root", 4, "Popeq"),
    ("signed-authority-domain", 1, "Dup"),
    ("signed-authority-domain", 8, "Idx"),
    ("signed-authority-domain", 5, "Popeq"),
    ("signed-authority-pre-count", 1, "Dup"),
    ("signed-authority-pre-count", 4, "Idx"),
    ("signed-authority-pre-count", 1, "Size"),
    ("signed-authority-pre-count", 4, "Popeq"),
    ("signed-authority-pre-member", 1, "Dup"),
    ("signed-authority-pre-member", 4, "Idx"),
    ("signed-authority-pre-member", 5, "Push"),
    ("signed-authority-pre-member", 1, "Member"),
    ("signed-authority-pre-member", 4, "Popeq"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("source-event-push", 14, "Push"),
    ("source-event-log", 1, "Log"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("signed-authority-write", 4, "Idx"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 1, "Ins"),
    ("signed-authority-post-count", 1, "Dup"),
    ("signed-authority-post-count", 4, "Idx"),
    ("signed-authority-post-count", 1, "Size"),
    ("signed-authority-post-count", 4, "Popeq"),
    ("signed-authority-post-member", 1, "Dup"),
    ("signed-authority-post-member", 4, "Idx"),
    ("signed-authority-post-member", 5, "Push"),
    ("signed-authority-post-member", 1, "Member"),
    ("signed-authority-post-member", 4, "Popeq"),
];
const NUMERIC_RESET_UNSIGNED: &[(&str, usize, &str)] = &[
    ("abi-root", 1, "Dup"),
    ("abi-root", 1, "Size"),
    ("abi-root", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("source-event-push", 14, "Push"),
    ("source-event-log", 1, "Log"),
];
const NUMERIC_RESET_SIGNED_V2: &[(&str, usize, &str)] = &[
    ("signed-abi-root", 1, "Dup"),
    ("signed-abi-root", 1, "Size"),
    ("signed-abi-root", 4, "Popeq"),
    ("signed-authority-domain", 1, "Dup"),
    ("signed-authority-domain", 8, "Idx"),
    ("signed-authority-domain", 5, "Popeq"),
    ("signed-authority-pre-count", 1, "Dup"),
    ("signed-authority-pre-count", 4, "Idx"),
    ("signed-authority-pre-count", 1, "Size"),
    ("signed-authority-pre-count", 4, "Popeq"),
    ("signed-authority-pre-member", 1, "Dup"),
    ("signed-authority-pre-member", 4, "Idx"),
    ("signed-authority-pre-member", 5, "Push"),
    ("signed-authority-pre-member", 1, "Member"),
    ("signed-authority-pre-member", 4, "Popeq"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-write", 4, "Idx"),
    ("state-write", 6, "Push"),
    ("state-write", 5, "Push"),
    ("state-write", 1, "Ins"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("source-event-push", 14, "Push"),
    ("source-event-log", 1, "Log"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("state-read", 1, "Dup"),
    ("state-read", 8, "Idx"),
    ("state-opening", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("signed-authority-write", 4, "Idx"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 1, "Ins"),
    ("signed-authority-post-count", 1, "Dup"),
    ("signed-authority-post-count", 4, "Idx"),
    ("signed-authority-post-count", 1, "Size"),
    ("signed-authority-post-count", 4, "Popeq"),
    ("signed-authority-post-member", 1, "Dup"),
    ("signed-authority-post-member", 4, "Idx"),
    ("signed-authority-post-member", 5, "Push"),
    ("signed-authority-post-member", 1, "Member"),
    ("signed-authority-post-member", 4, "Popeq"),
];

const FINANCIAL: &str = "language \"moriarty/1\";\nagreement StaticFinancialDraft {\n  asset PAY = {network:\"local\", issuer:\"independent-static\", reference:\"PAY\", scale:0};\n  action accrue(amount: UInt) {\n    requires amount >= 3;\n    transfer(PAY,\"alice\",\"bob\",amount);\n    transfer(PAY,\"alice\",\"bob\",amount);\n    fee(PAY,\"alice\",\"treasury\",2);\n    liability(\"obligation\",PAY,\"alice\",\"bob\",amount);\n    emit \"accrued\"(true);\n  }\n  action repay(amount: UInt) {\n    requires liability_present(\"obligation\") and liability_amount(\"obligation\") >= amount;\n    discharge(\"obligation\",amount);\n    ensures liability_amount(\"obligation\") >= 0;\n  }\n}\n";
const FINANCIAL_ACCRUE_UNSIGNED: &[(&str, usize, &str)] = &[
    ("abi-root", 1, "Dup"),
    ("abi-root", 1, "Size"),
    ("abi-root", 4, "Popeq"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 4, "Idx"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 19, "Push"),
    ("source-claim", 1, "Member"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 18, "Idx"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 19, "Push"),
    ("source-claim", 5, "Push"),
    ("source-claim", 1, "Ins"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 4, "Idx"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 19, "Push"),
    ("source-claim", 1, "Member"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 18, "Idx"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 19, "Push"),
    ("source-claim", 5, "Push"),
    ("source-claim", 1, "Ins"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 4, "Idx"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 19, "Push"),
    ("source-claim", 1, "Member"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 18, "Idx"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 19, "Push"),
    ("source-claim", 5, "Push"),
    ("source-claim", 1, "Ins"),
    ("source-claim", 1, "Swap"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-write", 4, "Idx"),
    ("duty-write", 6, "Push"),
    ("duty-write", 14, "Push"),
    ("duty-write", 1, "Ins"),
    ("source-event-push", 14, "Push"),
    ("source-event-log", 1, "Log"),
];
const FINANCIAL_ACCRUE_SIGNED_V2: &[(&str, usize, &str)] = &[
    ("signed-abi-root", 1, "Dup"),
    ("signed-abi-root", 1, "Size"),
    ("signed-abi-root", 4, "Popeq"),
    ("signed-authority-domain", 1, "Dup"),
    ("signed-authority-domain", 8, "Idx"),
    ("signed-authority-domain", 5, "Popeq"),
    ("signed-authority-pre-count", 1, "Dup"),
    ("signed-authority-pre-count", 4, "Idx"),
    ("signed-authority-pre-count", 1, "Size"),
    ("signed-authority-pre-count", 4, "Popeq"),
    ("signed-authority-pre-member", 1, "Dup"),
    ("signed-authority-pre-member", 4, "Idx"),
    ("signed-authority-pre-member", 5, "Push"),
    ("signed-authority-pre-member", 1, "Member"),
    ("signed-authority-pre-member", 4, "Popeq"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 4, "Idx"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 19, "Push"),
    ("source-claim", 1, "Member"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 18, "Idx"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 19, "Push"),
    ("source-claim", 5, "Push"),
    ("source-claim", 1, "Ins"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 4, "Idx"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 19, "Push"),
    ("source-claim", 1, "Member"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 18, "Idx"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 19, "Push"),
    ("source-claim", 5, "Push"),
    ("source-claim", 1, "Ins"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 1, "Swap"),
    ("source-claim", 4, "Idx"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 19, "Push"),
    ("source-claim", 1, "Member"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 1, "Dup"),
    ("source-claim", 18, "Idx"),
    ("source-claim", 4, "Popeq"),
    ("source-claim", 19, "Push"),
    ("source-claim", 5, "Push"),
    ("source-claim", 1, "Ins"),
    ("source-claim", 1, "Swap"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-write", 4, "Idx"),
    ("duty-write", 6, "Push"),
    ("duty-write", 14, "Push"),
    ("duty-write", 1, "Ins"),
    ("source-event-push", 14, "Push"),
    ("source-event-log", 1, "Log"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("signed-authority-write", 4, "Idx"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 1, "Ins"),
    ("signed-authority-post-count", 1, "Dup"),
    ("signed-authority-post-count", 4, "Idx"),
    ("signed-authority-post-count", 1, "Size"),
    ("signed-authority-post-count", 4, "Popeq"),
    ("signed-authority-post-member", 1, "Dup"),
    ("signed-authority-post-member", 4, "Idx"),
    ("signed-authority-post-member", 5, "Push"),
    ("signed-authority-post-member", 1, "Member"),
    ("signed-authority-post-member", 4, "Popeq"),
];
const FINANCIAL_REPAY_UNSIGNED: &[(&str, usize, &str)] = &[
    ("abi-root", 1, "Dup"),
    ("abi-root", 1, "Size"),
    ("abi-root", 4, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-write", 4, "Idx"),
    ("duty-write", 6, "Push"),
    ("duty-write", 14, "Push"),
    ("duty-write", 1, "Ins"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
];
const FINANCIAL_REPAY_SIGNED_V2: &[(&str, usize, &str)] = &[
    ("signed-abi-root", 1, "Dup"),
    ("signed-abi-root", 1, "Size"),
    ("signed-abi-root", 4, "Popeq"),
    ("signed-authority-domain", 1, "Dup"),
    ("signed-authority-domain", 8, "Idx"),
    ("signed-authority-domain", 5, "Popeq"),
    ("signed-authority-pre-count", 1, "Dup"),
    ("signed-authority-pre-count", 4, "Idx"),
    ("signed-authority-pre-count", 1, "Size"),
    ("signed-authority-pre-count", 4, "Popeq"),
    ("signed-authority-pre-member", 1, "Dup"),
    ("signed-authority-pre-member", 4, "Idx"),
    ("signed-authority-pre-member", 5, "Push"),
    ("signed-authority-pre-member", 1, "Member"),
    ("signed-authority-pre-member", 4, "Popeq"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("duty-write", 4, "Idx"),
    ("duty-write", 6, "Push"),
    ("duty-write", 14, "Push"),
    ("duty-write", 1, "Ins"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("signed-state-cardinality", 1, "Dup"),
    ("signed-state-cardinality", 4, "Idx"),
    ("signed-state-cardinality", 1, "Size"),
    ("signed-state-cardinality", 4, "Popeq"),
    ("signed-duty-cardinality", 1, "Dup"),
    ("signed-duty-cardinality", 4, "Idx"),
    ("signed-duty-cardinality", 1, "Size"),
    ("signed-duty-cardinality", 4, "Popeq"),
    ("duty-membership", 1, "Dup"),
    ("duty-membership", 4, "Idx"),
    ("duty-membership", 6, "Push"),
    ("duty-membership", 1, "Member"),
    ("duty-membership", 4, "Popeq"),
    ("duty-old-read", 1, "Dup"),
    ("duty-old-read", 8, "Idx"),
    ("duty-old-read", 13, "Popeq"),
    ("signed-authority-write", 4, "Idx"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 5, "Push"),
    ("signed-authority-write", 1, "Ins"),
    ("signed-authority-post-count", 1, "Dup"),
    ("signed-authority-post-count", 4, "Idx"),
    ("signed-authority-post-count", 1, "Size"),
    ("signed-authority-post-count", 4, "Popeq"),
    ("signed-authority-post-member", 1, "Dup"),
    ("signed-authority-post-member", 4, "Idx"),
    ("signed-authority-post-member", 5, "Push"),
    ("signed-authority-post-member", 1, "Member"),
    ("signed-authority-post-member", 4, "Popeq"),
];

use midnight_coin_structure::coin::{UnshieldedTokenType, UserAddress};
use midnight_curves::{JubjubSubgroup, pairing::group::Group};
use midnight_ledger::structure::SignatureVerifyingKey;
use midnight_zkir_v3::ir::Operand;
use midnight_zkir_v3::{Instruction, IrSource};
use mori_core::Program;
use mori_ledger::{
    layout,
    static_compile::{StaticAuthorization, compile, reopen},
};
use std::{
    collections::BTreeMap,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};
fn bindings(source: &str) -> (Program, StaticBindings) {
    let p = mori_core::parse(source).unwrap();
    let names = ["alice", "bob", "treasury"];
    let points = [
        "0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        "02f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9",
    ];
    let roles=names.into_iter().zip(points).map(|(role,point)| {
        let bytes=codec::unhex(point).unwrap();
        let vk=<midnight_base_crypto::ecdsa::VerifyingKey as midnight_serialize::Deserializable>::deserialize(&mut bytes.as_slice(),0).unwrap();
        let key=SignatureVerifyingKey::ECDSA(vk);
        RoleBinding { role:role.into(),public_key_hex:codec::native_hex(&key).unwrap(),address_hex:codec::native_hex(&UserAddress::from(key)).unwrap() }
    }).collect::<Vec<_>>();
    let assets = p
        .assets
        .iter()
        .enumerate()
        .map(|(i, a)| AssetBinding {
            identity: a.identity.clone(),
            token_hex: codec::native_hex(&UnshieldedTokenType(
                midnight_base_crypto::hash::HashOutput([i as u8 + 1; 32]),
            ))
            .unwrap(),
        })
        .collect::<Vec<_>>();
    let mut duty_map = BTreeMap::new();
    for st in p.actions.iter().flat_map(|a| &a.statements) {
        if let mori_core::Statement::Liability {
            name,
            asset,
            debtor,
            creditor,
            ..
        } = st
        {
            duty_map.insert(
                name.clone(),
                DutyBinding {
                    name: name.clone(),
                    asset: p
                        .assets
                        .iter()
                        .find(|a| &a.name == asset)
                        .unwrap()
                        .identity
                        .clone(),
                    debtor: debtor.clone(),
                    creditor: creditor.clone(),
                },
            );
        }
    }
    let duties = duty_map.into_values().collect::<Vec<_>>();
    let abi = layout::LedgerAbi {
        profile: ABI.into(),
        language: p.language.clone(),
        semantics: p.semantics.clone(),
        program_hash: codec::object_hash(&p).unwrap(),
        states: p
            .states
            .iter()
            .map(|st| layout::StateField {
                name: st.name.clone(),
                ty: st.ty,
                key_hex: codec::native_hex(&layout::name_key("state", &st.name)).unwrap(),
            })
            .collect(),
        actions: p.actions.clone(),
        assets: assets.clone(),
        roles: roles.clone(),
        duties: duties.clone(),
        templates_hash: layout::templates_hash(),
    };
    let b = StaticBindings {
        abi: ABI.into(),
        source_hash: codec::hash(source.as_bytes()),
        program_hash: abi.program_hash.clone(),
        abi_hash: codec::object_hash(&abi).unwrap(),
        templates_hash: abi.templates_hash,
        toolchain: Toolchain::pinned(),
        limits: Limits::supported(),
        network: "local-static-example".into(),
        anchor: "local-unqualified".into(),
        address_hex: "public-contract-domain".into(),
        payer: "alice".into(),
        assets,
        roles,
        duties,
        economic_bounds: None,
    };
    (p, b)
}
fn limits() -> NativeWorkLimits {
    NativeWorkLimits {
        profile: "moriarty-native-call-work-limits/2".into(),
        max_degree: 1,
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
fn policy() -> mori_ledger::signed::FixedSignerPolicy {
    let (x, y) = mori_ledger::signed::schnorr::coordinates(JubjubSubgroup::generator());
    mori_ledger::signed::FixedSignerPolicy {
        profile: mori_ledger::signed::POLICY.into(),
        owner: "alice".into(),
        beneficiary: "bob".into(),
        public_key_hex: format!(
            "{}{}",
            mori_ledger::signed::schnorr::native_hex(x),
            mori_ledger::signed::schnorr::native_hex(y)
        ),
    }
}
fn signed() -> StaticAuthorization {
    StaticAuthorization::SignedV2 {
        signer_policy: policy(),
    }
}
static NEXT: AtomicUsize = AtomicUsize::new(0);
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        let p = std::env::temp_dir().join(format!(
            "mori-static-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
fn materialize(bundle: &static_compile::StaticBundle, dir: &Dir) {
    for (n, b) in &bundle.leaves().leaves {
        std::fs::write(dir.0.join(n), b).unwrap();
    }
    std::fs::write(
        dir.0.join(bundle.leaves().marker_name),
        &bundle.leaves().marker,
    )
    .unwrap();
}
fn manifest(bundle: &static_compile::StaticBundle, i: usize) -> mori_ledger::relation::Manifest {
    codec::json(
        std::str::from_utf8(&bundle.leaves().leaves[&format!("action-{i:06}.manifest.json")])
            .unwrap(),
    )
    .unwrap()
}
fn ir(bundle: &static_compile::StaticBundle, i: usize) -> IrSource {
    codec::decode_artifact(&bundle.leaves().leaves[&format!("action-{i:06}.ir.bin")]).unwrap()
}
fn oracle(
    source: &str,
    auth: &StaticAuthorization,
    arrays: &[&[(&str, usize, &str)]],
    fields: &[usize],
    args: &[usize],
) {
    let (p, b) = bindings(source);
    let bundle = compile(source, &p, &b, auth, &limits())
        .expect("all declared actions must compile from public inputs");
    if let Some(root) = std::env::var_os("MORI_STATIC_CAPTURE_DIR") {
        let id = if source == NUMERIC {
            "numeric"
        } else {
            "financial"
        };
        let mode = if matches!(auth, StaticAuthorization::Unsigned) {
            "unsigned"
        } else {
            "signed-v2"
        };
        let path = std::path::PathBuf::from(root).join(format!("{id}-{mode}"));
        std::fs::create_dir_all(&path).unwrap();
        std::fs::write(path.join("source.mori"), source).unwrap();
        let mut input = serde_json::to_value(&b).unwrap();
        input.as_object_mut().unwrap().remove("economic_bounds");
        std::fs::write(
            path.join("bindings.json"),
            serde_json::to_vec_pretty(&input).unwrap(),
        )
        .unwrap();
        std::fs::write(
            path.join("signer-policy.json"),
            serde_json::to_vec_pretty(&policy()).unwrap(),
        )
        .unwrap();
        std::fs::write(
            path.join("work-limits.json"),
            serde_json::to_vec_pretty(&limits()).unwrap(),
        )
        .unwrap();
        let dir = path.join("bundle");
        std::fs::create_dir_all(&dir).unwrap();
        for (n, v) in &bundle.leaves().leaves {
            std::fs::write(dir.join(n), v).unwrap();
        }
        std::fs::write(dir.join("static-compile.json"), &bundle.leaves().marker).unwrap();
    }
    assert_eq!(
        bundle.action_names(),
        p.actions
            .iter()
            .map(|a| a.name.as_str())
            .collect::<Vec<_>>()
    );
    for (i, a) in p.actions.iter().enumerate() {
        let m = manifest(&bundle, i);
        let ir = ir(&bundle, i);
        assert_eq!(m.action, a.name);
        assert_eq!(
            m.abi
                .states
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            p.states.iter().map(|s| s.name.as_str()).collect::<Vec<_>>()
        );
        assert_eq!(
            m.groups
                .iter()
                .map(|g| (g.origin.as_str(), g.field_count))
                .collect::<Vec<_>>(),
            arrays[i]
                .iter()
                .map(|(o, w, _)| (*o, *w))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            2 + m.groups.iter().map(|g| g.field_count).sum::<usize>(),
            fields[i]
        );
        assert!(ir.do_communications_commitment);
        assert!(ir.outputs.is_empty());
        assert_eq!(ir.inputs.len(), args[i]);
        let impacts = ir
            .instructions
            .iter()
            .filter_map(|ins| {
                if let Instruction::Impact { inputs, .. } = ins {
                    Some(inputs)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(impacts.len(), arrays[i].len());
        for (inputs, (_, width, class)) in impacts.iter().zip(arrays[i]) {
            assert_eq!(inputs.len(), *width);
            let Operand::Immediate(header) = inputs[0] else {
                panic!("static opcode header");
            };
            let raw = header.as_le_bytes();
            assert!(raw[1..].iter().all(|b| *b == 0));
            let opcode = raw[0];
            let ok = match *class {
                "Swap" => opcode & 0xf0 == 0x40,
                "Dup" => opcode & 0xf0 == 0x30,
                "Size" => opcode == 4,
                "Popeq" => (12..=13).contains(&opcode),
                "Idx" => (0x50..=0x8f).contains(&opcode),
                "Push" => (0x10..=0x11).contains(&opcode),
                "Ins" => (0x90..=0xaf).contains(&opcode),
                "Member" => opcode == 0x18,
                "Log" => opcode == 9,
                "Pop" => opcode == 11,
                _ => false,
            };
            assert!(ok, "{class}: {opcode}");
        }
        assert_eq!(
            ir.instructions
                .iter()
                .filter(|ins| matches!(ins, Instruction::Output { .. }))
                .count(),
            1
        );
        assert!(
            !ir.instructions
                .iter()
                .any(|ins| matches!(ins, Instruction::PrivateInput { .. }))
        );
        if let Some(sm) = &m.signed {
            assert_eq!(m.format, mori_ledger::signed::CONSUMING_RELATION);
            assert_eq!(sm.signer_policy, policy());
            assert_eq!(sm.typed_argument_count, args[i]);
            assert_eq!(sm.flattened_field_count, args[i] + 2);
            let ds = &sm.typed_argument_descriptors[a.inputs.len()..];
            assert_eq!(
                (ds[0].name.as_str(), ds[0].ty.as_str(), ds[0].encoded_fields),
                ("PK", "Point<Jubjub>", 2)
            );
            assert_eq!(
                (ds[1].name.as_str(), ds[1].ty.as_str(), ds[1].encoded_fields),
                ("R", "Point<Jubjub>", 2)
            );
            assert_eq!(ds[2].name, "s");
            let dbg = format!("{:?}", ir.instructions);
            for class in [
                "IntoCoordinates",
                "TransientHash",
                "JubjubScalarFromNative",
                "Encode",
                "EcMulGenerator",
                "EcMul",
            ] {
                assert!(dbg.contains(class), "{class}");
            }
        }
    }
    assert_ne!(
        bundle.leaves().leaves["action-000000.ir.bin"],
        bundle.leaves().leaves["action-000001.ir.bin"]
    );
}
#[test]
fn numeric_unsigned_independent_groups_and_widths() {
    oracle(
        NUMERIC,
        &StaticAuthorization::Unsigned,
        &[NUMERIC_ADVANCE_UNSIGNED, NUMERIC_RESET_UNSIGNED],
        &[120, 107],
        &[2, 0],
    );
}
#[test]
fn financial_unsigned_independent_groups_and_widths() {
    oracle(
        FINANCIAL,
        &StaticAuthorization::Unsigned,
        &[FINANCIAL_ACCRUE_UNSIGNED, FINANCIAL_REPAY_UNSIGNED],
        &[323, 185],
        &[1, 1],
    );
}
#[test]
fn consuming_v2_numeric_independent_groups_descriptors_signature() {
    oracle(
        NUMERIC,
        &signed(),
        &[NUMERIC_ADVANCE_SIGNED_V2, NUMERIC_RESET_SIGNED_V2],
        &[291, 278],
        &[13, 11],
    );
}
#[test]
fn consuming_v2_financial_independent_groups_descriptors_signature() {
    oracle(
        FINANCIAL,
        &signed(),
        &[FINANCIAL_ACCRUE_SIGNED_V2, FINANCIAL_REPAY_SIGNED_V2],
        &[518, 380],
        &[16, 16],
    );
}
#[test]
fn source_order_three_actions_and_arithmetic_boolean_contrasts() {
    let extra =
        NUMERIC.rsplit_once('}').unwrap().0.to_owned() + " action aardvark() { next zulu = 4; } }";
    let (p, b) = bindings(&extra);
    let out = compile(&extra, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    assert_eq!(out.action_names(), vec!["advance", "reset", "aardvark"]);
    let (p, b) = bindings(NUMERIC);
    let base = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    let mul = NUMERIC.replace("next zulu = zulu + amount", "next zulu = zulu * amount");
    let (p, b) = bindings(&mul);
    let changed = compile(&mul, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    assert_eq!(manifest(&base, 0).groups, manifest(&changed, 0).groups);
    assert_ne!(ir(&base, 0).instructions, ir(&changed, 0).instructions);
    assert!(format!("{:?}", ir(&changed, 0).instructions).contains("DivModPowerOfTwo"));
    let reads = NUMERIC.replace("requires allow;", "requires allow and alpha and zulu >= 0;");
    let (p, b) = bindings(&reads);
    let changed = compile(&reads, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    assert_eq!(manifest(&changed, 0).groups.len(), 34);
    assert_eq!(
        2 + manifest(&changed, 0)
            .groups
            .iter()
            .map(|g| g.field_count)
            .sum::<usize>(),
        146
    );
}
#[test]
fn typed_none_and_strict_marker_omission_identity_roundtrip() {
    let (p, b) = bindings(NUMERIC);
    for auth in [StaticAuthorization::Unsigned, signed()] {
        let out = compile(NUMERIC, &p, &b, &auth, &limits()).unwrap();
        let d = Dir::new();
        materialize(&out, &d);
        let opened = reopen(NUMERIC, &p, &b, &auth, &d.0, &limits()).unwrap();
        assert_eq!(opened.identity(), out.identity());
        assert_eq!(opened.action_names(), vec!["advance", "reset"]);
        let mut marker: serde_json::Value = serde_json::from_slice(&out.leaves().marker).unwrap();
        assert!(marker["bindings"].get("economic_bounds").is_none());
        let identity = marker.as_object_mut().unwrap().remove("identity").unwrap();
        assert_eq!(
            identity.as_str().unwrap(),
            codec::object_hash(&marker).unwrap()
        );
        assert_eq!(
            marker["qualification"],
            "source-static-unproven-uninstalled"
        );
    }
}
#[test]
fn coherent_network_address_change_is_unqualified_new_identity() {
    let (p, b) = bindings(NUMERIC);
    let mut alt = b.clone();
    alt.network = "coherent-public-alternative".into();
    alt.address_hex = "supplied-public-alternative".into();
    let a = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    let z = compile(NUMERIC, &p, &alt, &StaticAuthorization::Unsigned, &limits()).unwrap();
    assert_eq!(a.leaves().leaves, z.leaves().leaves);
    assert_ne!(a.identity(), z.identity());
    let a = compile(NUMERIC, &p, &b, &signed(), &limits()).unwrap();
    let z = compile(NUMERIC, &p, &alt, &signed(), &limits()).unwrap();
    assert_ne!(
        manifest(&a, 0).signed.unwrap().context_sha256,
        manifest(&z, 0).signed.unwrap().context_sha256
    );
    let d = Dir::new();
    materialize(&z, &d);
    assert!(reopen(NUMERIC, &p, &b, &signed(), &d.0, &limits()).is_err());
}
#[test]
fn stale_source_core_hashes_toolchain_and_mappings_refused() {
    let (p, b) = bindings(FINANCIAL);
    assert!(
        compile(
            &(FINANCIAL.to_owned() + " "),
            &p,
            &b,
            &StaticAuthorization::Unsigned,
            &limits()
        )
        .is_err()
    );
    let mut other = p.clone();
    other.actions.reverse();
    assert!(
        compile(
            FINANCIAL,
            &other,
            &b,
            &StaticAuthorization::Unsigned,
            &limits()
        )
        .is_err()
    );
    for which in 0..15 {
        let mut x = b.clone();
        match which {
            0 => x.program_hash = "0".repeat(64),
            1 => x.abi_hash = "0".repeat(64),
            2 => x.templates_hash = "0".repeat(64),
            3 => x.abi = "invented".into(),
            4 => x.toolchain.sdk = "stale".into(),
            5 => x.toolchain.compatibility = "stale".into(),
            6 => x.toolchain.rust = "stale".into(),
            7 => {
                x.toolchain
                    .packages
                    .insert("zkir-v3".into(), "stale".into());
            }
            8 => x.limits.ops = 1,
            9 => x.payer = "missing".into(),
            10 => x.roles[0].address_hex = x.roles[1].address_hex.clone(),
            11 => x.roles.push(x.roles[0].clone()),
            12 => x.assets.push(x.assets[0].clone()),
            13 => x.duties.push(x.duties[0].clone()),
            _ => x.duties.clear(),
        };
        assert!(
            compile(FINANCIAL, &p, &x, &StaticAuthorization::Unsigned, &limits()).is_err(),
            "control {which}"
        );
    }
}
#[test]
fn strict_inputs_unknown_duplicate_null_depth_node_and_auth_refused() {
    let mut value = serde_json::to_value(parser_bindings()).unwrap();
    value.as_object_mut().unwrap().remove("economic_bounds");
    let raw = value.to_string();
    assert!(static_compile::parse_static_bindings(&(raw.clone() + " []")).is_err());
    for bad in [
        raw.replacen("{", "{\"abi\":\"x\",", 1),
        raw.replace("\"sdk\":", "\"unknown\":0,\"sdk\":"),
        raw.replace("\"roles\":[]", "\"roles\":[null]"),
    ] {
        assert!(static_compile::parse_static_bindings(&bad).is_err());
    }
    assert!(
        static_compile::parse_authorization(r#"{"mode":"unsigned","signer_policy":{}}"#).is_err()
    );
    assert!(static_compile::parse_authorization(r#"{"mode":"signed-v2"}"#).is_err());
    assert!(static_compile::parse_authorization(r#"{"mode":"signed-v1"}"#).is_err());
    let deep = "[".repeat(34) + "0" + &"]".repeat(34);
    assert!(static_compile::parse_authorization(&deep).is_err());
    let nodes = "[".to_owned() + &vec!["0"; 65537].join(",") + "]";
    assert!(static_compile::parse_authorization(&nodes).is_err());
    let (p, b) = bindings(FINANCIAL);
    let mut pol = policy();
    pol.public_key_hex = "00".repeat(64);
    assert!(
        compile(
            FINANCIAL,
            &p,
            &b,
            &StaticAuthorization::SignedV2 { signer_policy: pol },
            &limits()
        )
        .is_err()
    );
    let mut pol = policy();
    pol.owner = "bob".into();
    assert!(
        compile(
            FINANCIAL,
            &p,
            &b,
            &StaticAuthorization::SignedV2 { signer_policy: pol },
            &limits()
        )
        .is_err()
    );
}
#[test]
fn every_action_and_all_leaf_marker_total_limits() {
    let (p, b) = bindings(NUMERIC);
    let base = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    let mut lim = limits();
    lim.max_actions = 2;
    lim.max_operations = 28;
    lim.max_public_inputs = 120;
    compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).unwrap();
    for which in 0..3 {
        let mut l = lim.clone();
        match which {
            0 => l.max_actions = 1,
            1 => l.max_operations = 27,
            _ => l.max_public_inputs = 119,
        };
        assert!(compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &l).is_err());
    }
    let leaf = base
        .leaves()
        .leaves
        .values()
        .map(Vec::len)
        .chain([base.leaves().marker.len()])
        .max()
        .unwrap();
    let total =
        base.leaves().leaves.values().map(Vec::len).sum::<usize>() + base.leaves().marker.len();
    let mut lim = limits();
    lim.max_leaf_bytes = leaf as u64;
    lim.max_artifact_bytes = total as u64;
    compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).unwrap();
    lim.max_artifact_bytes -= 1;
    assert!(compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).is_err());
    lim.max_artifact_bytes = total as u64;
    lim.max_leaf_bytes -= 1;
    assert!(compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).is_err());
    let mut lim = limits();
    lim.max_input_bytes = (serde_json::to_vec(&b)
        .unwrap()
        .len()
        .max(base.leaves().marker.len())
        - 1) as u64;
    assert!(compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).is_err());
    let mut lim = limits();
    lim.profile = "moriarty-native-call-work-limits/1".into();
    compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).unwrap();
    lim.max_input_bytes = u64::MAX;
    assert!(compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &lim).is_err());
}
#[test]
fn marker_and_leaf_tamper_exact_entry_set_no_self_authority() {
    let (p, b) = bindings(NUMERIC);
    let base = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    for field in [
        "profile",
        "qualification",
        "build",
        "source_hash",
        "program_hash",
        "identity",
    ] {
        let d = Dir::new();
        materialize(&base, &d);
        let mut m: serde_json::Value = serde_json::from_slice(&base.leaves().marker).unwrap();
        m[field] = "0".repeat(64).into();
        std::fs::write(d.0.join("static-compile.json"), m.to_string()).unwrap();
        assert!(
            reopen(
                NUMERIC,
                &p,
                &b,
                &StaticAuthorization::Unsigned,
                &d.0,
                &limits()
            )
            .is_err(),
            "{field}"
        );
    }
    for which in 0..10 {
        let d = Dir::new();
        materialize(&base, &d);
        let leaf = d.0.join("action-000001.ir.bin");
        match which {
            0 => std::fs::write(d.0.join("extra.vk.bin"), "no").unwrap(),
            1 => std::fs::remove_file(&leaf).unwrap(),
            2 => {
                let mut v = std::fs::read(&leaf).unwrap();
                v.push(0);
                std::fs::write(&leaf, v).unwrap();
            }
            3 => std::fs::write(&leaf, &base.leaves().leaves["action-000000.ir.bin"]).unwrap(),
            4 => std::fs::write(d.0.join("source.mori"), "substitution").unwrap(),
            5 => std::fs::write(d.0.join("action-000000.manifest.json"), "{}").unwrap(),
            _ => {
                let mut m: serde_json::Value =
                    serde_json::from_slice(&base.leaves().marker).unwrap();
                match which {
                    6 => m["actions"].as_array_mut().unwrap().reverse(),
                    7 => m["authorization"]["mode"] = "signed-v2".into(),
                    8 => m["bindings"]["economic_bounds"] = serde_json::Value::Null,
                    _ => m["degree"] = 1.into(),
                };
                std::fs::write(d.0.join("static-compile.json"), m.to_string()).unwrap();
            }
        }
        assert!(
            reopen(
                NUMERIC,
                &p,
                &b,
                &StaticAuthorization::Unsigned,
                &d.0,
                &limits()
            )
            .is_err(),
            "control {which}"
        );
    }
}
#[cfg(unix)]
#[test]
fn root_marker_and_leaves_no_follow_fifo_directory_rejected() {
    use std::os::unix::fs::symlink;
    let (p, b) = bindings(NUMERIC);
    let base = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    for name in ["static-compile.json", "source.mori", "action-000001.ir.bin"] {
        for kind in 0..3 {
            let d = Dir::new();
            materialize(&base, &d);
            let path = d.0.join(name);
            std::fs::remove_file(&path).unwrap();
            match kind {
                0 => symlink("missing", &path).unwrap(),
                1 => std::fs::create_dir(&path).unwrap(),
                _ => {
                    use std::ffi::CString;
                    use std::os::unix::ffi::OsStrExt;
                    let c = CString::new(path.as_os_str().as_bytes()).unwrap();
                    assert_eq!(unsafe { libc::mkfifo(c.as_ptr(), 0o600) }, 0);
                }
            }
            assert!(
                reopen(
                    NUMERIC,
                    &p,
                    &b,
                    &StaticAuthorization::Unsigned,
                    &d.0,
                    &limits()
                )
                .is_err()
            );
        }
    }
    let d = Dir::new();
    materialize(&base, &d);
    let parent = Dir::new();
    let alias = parent.0.join("alias");
    symlink(&d.0, &alias).unwrap();
    assert!(
        reopen(
            NUMERIC,
            &p,
            &b,
            &StaticAuthorization::Unsigned,
            &alias,
            &limits()
        )
        .is_err()
    );
}

#[test]
fn signed_scope_policy_and_input_size_boundaries() {
    let (p, b) = bindings(NUMERIC);
    let auth = signed();
    let out = compile(NUMERIC, &p, &b, &auth, &limits()).unwrap();
    let maximum = [
        NUMERIC.len(),
        serde_json::to_vec_pretty(&b).unwrap().len(),
        serde_json::to_vec_pretty(&auth).unwrap().len(),
        serde_json::to_vec_pretty(&limits()).unwrap().len(),
        out.leaves().marker.len(),
    ]
    .into_iter()
    .max()
    .unwrap();
    let mut l = limits();
    l.max_input_bytes = maximum as u64;
    compile(NUMERIC, &p, &b, &auth, &l).unwrap();
    l.max_input_bytes -= 1;
    assert!(compile(NUMERIC, &p, &b, &auth, &l).is_err());
    let (p, b) = bindings(FINANCIAL);
    let mut x = b.clone();
    x.economic_bounds = Some(Bounds {
        owner: "alice".into(),
        limits: vec![],
    });
    assert!(compile(FINANCIAL, &p, &x, &signed(), &limits()).is_err());
    let mut pol = policy();
    pol.profile = "unknown".into();
    assert!(
        compile(
            FINANCIAL,
            &p,
            &b,
            &StaticAuthorization::SignedV2 { signer_policy: pol },
            &limits()
        )
        .is_err()
    );
    let mut src = String::from("language \"moriarty/1\"; agreement Scope {");
    for i in 0..17 {
        src.push_str(&format!(
            "asset A{i} = {{network: \"local\",issuer: \"i\",reference: \"{i}\",scale:0}};"
        ));
    }
    src.push_str("action run() {} }");
    let (p, b) = bindings(&src);
    compile(&src, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    assert!(compile(&src, &p, &b, &signed(), &limits()).is_err());
    let (p, mut b) = bindings(FINANCIAL);
    for i in 1..17 {
        let mut duty = b.duties[0].clone();
        duty.name = format!("extra-{i}");
        b.duties.push(duty);
    }
    let abi = layout::LedgerAbi {
        profile: ABI.into(),
        language: p.language.clone(),
        semantics: p.semantics.clone(),
        program_hash: b.program_hash.clone(),
        states: vec![],
        actions: p.actions.clone(),
        assets: b.assets.clone(),
        roles: b.roles.clone(),
        duties: b.duties.clone(),
        templates_hash: b.templates_hash.clone(),
    };
    b.abi_hash = codec::object_hash(&abi).unwrap();
    assert!(compile(FINANCIAL, &p, &b, &signed(), &limits()).is_err());
}
#[test]
fn input_aggregate_counts_conservative_typed_and_raw_buffers() {
    let (_, b) = bindings(NUMERIC);
    let auth = StaticAuthorization::Unsigned;
    let mut l = limits();
    let rawb = serde_json::to_vec_pretty(&b).unwrap().len() + 7;
    let rawa = serde_json::to_vec_pretty(&auth).unwrap().len() + 3;
    let rawl = serde_json::to_vec_pretty(&l).unwrap().len() + 11;
    let wanted = NUMERIC.len() + rawb + rawa + rawl;
    l.max_artifact_bytes = wanted as u64;
    assert_eq!(
        static_compile::check_input_budget(NUMERIC.len(), &b, &auth, &l, rawb, rawa, rawl).unwrap(),
        wanted
    );
    l.max_artifact_bytes -= 1;
    assert!(
        static_compile::check_input_budget(NUMERIC.len(), &b, &auth, &l, rawb, rawa, rawl).is_err()
    );
}
#[test]
fn signed_nonce_domain_formula_address_omission_and_key_read_write() {
    let (p, b) = bindings(NUMERIC);
    let out = compile(NUMERIC, &p, &b, &signed(), &limits()).unwrap();
    let m = manifest(&out, 0);
    let sm = m.signed.unwrap();
    assert_eq!(sm.storage_abi, mori_ledger::signed::CONSUMING_STORAGE_ABI);
    assert_eq!(sm.signed_abi, mori_ledger::signed::CONSUMING_SIGNED_ABI);
    let expected = codec::object_hash(&serde_json::json!([
        "moriarty-signed-authority-domain/1",
        sm.storage_abi,
        mori_ledger::signed::CONSUMING_RELATION,
        p.language,
        p.semantics,
        b.source_hash,
        b.program_hash,
        b.network,
        b.anchor,
        policy()
    ]))
    .unwrap();
    assert_eq!(sm.context.as_array().unwrap().last().unwrap()[1], expected);
    let mut alt = b.clone();
    alt.address_hex = "alternate-public-contract".into();
    let z = compile(NUMERIC, &p, &alt, &signed(), &limits()).unwrap();
    let zs = manifest(&z, 0).signed.unwrap();
    assert_ne!(sm.context_sha256, zs.context_sha256);
    assert_eq!(zs.context.as_array().unwrap().last().unwrap()[1], expected);
    let ir = ir(&out, 0);
    let groups = ir
        .instructions
        .iter()
        .filter_map(|i| match i {
            Instruction::Impact { guard, inputs } => Some((guard, inputs)),
            _ => None,
        })
        .zip(&m.groups)
        .collect::<Vec<_>>();
    let before = groups
        .iter()
        .rfind(|(_, g)| g.origin == "signed-authority-pre-member")
        .unwrap()
        .0
        .1
        .last()
        .unwrap();
    constrained_boolean(&ir.instructions, before, false);
    selected_opening(
        &ir.instructions,
        before,
        &Operand::Immediate(midnight_transient_crypto::curve::Fr::from(1u64)),
    );
    let after = groups
        .iter()
        .rfind(|(_, g)| g.origin == "signed-authority-post-member")
        .unwrap()
        .0
        .1
        .last()
        .unwrap();
    constrained_boolean(&ir.instructions, after, true);
    selected_opening(
        &ir.instructions,
        after,
        &Operand::Immediate(midnight_transient_crypto::curve::Fr::from(1u64)),
    );
    use midnight_transient_crypto::repr::FieldRepr;
    let domain = "moriarty-signed-authority-domain-key/1";
    let mut preimage = (domain.len() as u32).to_le_bytes().to_vec();
    preimage.extend(domain.as_bytes());
    preimage.extend(0u32.to_le_bytes());
    let key_hash: [u8; 32] = codec::unhex(&codec::hash(&preimage))
        .unwrap()
        .try_into()
        .unwrap();
    let key: midnight_base_crypto::fab::AlignedValue =
        midnight_base_crypto::hash::HashOutput(key_hash).into();
    let key_fields = key
        .field_vec()
        .into_iter()
        .map(Operand::Immediate)
        .collect::<Vec<_>>();
    let domain_read = groups
        .iter()
        .filter(|(_, g)| g.origin == "signed-authority-domain")
        .nth(1)
        .unwrap()
        .0
        .1;
    assert_eq!(
        &domain_read[domain_read.len() - key_fields.len()..],
        key_fields.as_slice()
    );
    let writes = groups
        .iter()
        .filter(|(_, g)| g.origin == "signed-authority-write")
        .map(|(op, _)| *op)
        .collect::<Vec<_>>();
    let nonce = Operand::Variable(midnight_zkir_v3::ir::Identifier(format!(
        "%input.{}",
        p.actions[0].inputs.len() + 5
    )));
    for origin in [
        "signed-authority-pre-member",
        "signed-authority-post-member",
    ] {
        let pushed = groups
            .iter()
            .filter(|(_, g)| g.origin == origin)
            .nth(2)
            .unwrap()
            .0
            .1
            .last()
            .unwrap();
        assert_eq!(pushed, &nonce);
    }
    assert_eq!(writes[1].1.last().unwrap(), &nonce);
    assert!(
        ir.instructions
            .iter()
            .any(|i| matches!(i,Instruction::ConstrainBits{val,bits:128} if val==&nonce))
    );
    let pre_count = groups
        .iter()
        .rfind(|(_, g)| g.origin == "signed-authority-pre-count")
        .unwrap()
        .0
        .1
        .last()
        .unwrap();
    let post_count = groups
        .iter()
        .rfind(|(_, g)| g.origin == "signed-authority-post-count")
        .unwrap()
        .0
        .1
        .last()
        .unwrap();
    let one = Operand::Immediate(midnight_transient_crypto::curve::Fr::from(1u64));
    for count in [pre_count, post_count] {
        selected_opening(&ir.instructions, count, &one);
        assert!(
            ir.instructions
                .iter()
                .any(|i| matches!(i,Instruction::ConstrainBits{val,bits:128} if val==count))
        );
    }
    let add = ir
        .instructions
        .iter()
        .find_map(|i| match i {
            Instruction::Add { a, b, output } if a == pre_count && b == &one => {
                Some(Operand::Variable(output.clone()))
            }
            _ => None,
        })
        .unwrap();
    assert!(
        ir.instructions
            .iter()
            .any(|i| matches!(i,Instruction::ConstrainEq{a,b} if a==post_count && b==&add))
    );

    assert_eq!(
        writes[2].1.last().unwrap(),
        &Operand::Immediate(midnight_transient_crypto::curve::Fr::from(1u64))
    );
    assert!(
        groups
            .iter()
            .position(|(_, g)| g.origin == "signed-authority-pre-member")
            .unwrap()
            < groups
                .iter()
                .position(|(_, g)| g.origin == "signed-authority-write")
                .unwrap()
    );
    assert!(
        groups
            .iter()
            .rposition(|(_, g)| g.origin == "signed-authority-write")
            .unwrap()
            < groups
                .iter()
                .position(|(_, g)| g.origin == "signed-authority-post-member")
                .unwrap()
    );

    assert_eq!(
        groups
            .iter()
            .filter(|(_, g)| g.origin == "signed-authority-write")
            .count(),
        4
    );
}

fn rewrite_marker_identity(value: &mut serde_json::Value) {
    value.as_object_mut().unwrap().remove("identity");
    let identity = codec::object_hash(value).unwrap();
    value["identity"] = identity.into();
}
#[test]
fn self_consistent_rewritten_leaf_commitments_fail_fresh_caller_comparison() {
    let (p, b) = bindings(NUMERIC);
    let base = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    for bytes in [
        base.leaves().leaves["action-000001.ir.bin"].clone(),
        vec![0xff; 32],
    ] {
        let d = Dir::new();
        materialize(&base, &d);
        let mut m: serde_json::Value = serde_json::from_slice(&base.leaves().marker).unwrap();
        let digest = codec::hash(&bytes);
        m["actions"][0]["ir_hash"] = digest.clone().into();
        m["leaves"]["action-000000.ir.bin"] =
            serde_json::json!({"sha256":digest,"bytes":bytes.len()});
        rewrite_marker_identity(&mut m);
        std::fs::write(
            d.0.join("static-compile.json"),
            serde_json::to_vec_pretty(&m).unwrap(),
        )
        .unwrap();
        std::fs::write(d.0.join("action-000000.ir.bin"), bytes).unwrap();
        let err = reopen(
            NUMERIC,
            &p,
            &b,
            &StaticAuthorization::Unsigned,
            &d.0,
            &limits(),
        )
        .err()
        .unwrap();
        assert_eq!(
            err.code, "fresh-source-mismatch",
            "no generic SDK tagged decoder on hostile bytes"
        );
    }
}
#[test]
fn marker_declared_lengths_paths_and_lowered_reader_caps_refused() {
    let (p, b) = bindings(NUMERIC);
    let base = compile(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    for which in 0..5 {
        let d = Dir::new();
        materialize(&base, &d);
        let mut m: serde_json::Value = serde_json::from_slice(&base.leaves().marker).unwrap();
        match which {
            0 => m["leaves"]["source.mori"]["bytes"] = serde_json::json!(16_777_217u64),
            1 => m["leaves"]["action-000001.ir.bin"]["bytes"] = serde_json::json!(1),
            2 => m["actions"][0]["ir_leaf"] = "../outside.ir.bin".into(),
            3 => m["actions"][0]["ordinal"] = 1.into(),
            _ => m["actions"][0]["name"] = "reset".into(),
        };
        rewrite_marker_identity(&mut m);
        std::fs::write(
            d.0.join("static-compile.json"),
            serde_json::to_vec_pretty(&m).unwrap(),
        )
        .unwrap();
        assert!(
            reopen(
                NUMERIC,
                &p,
                &b,
                &StaticAuthorization::Unsigned,
                &d.0,
                &limits()
            )
            .is_err()
        );
    }
    let d = Dir::new();
    materialize(&base, &d);
    let mut l = limits();
    l.max_leaf_bytes = (base.leaves().marker.len() - 1) as u64;
    assert!(reopen(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &d.0, &l).is_err());
    let mut l = limits();
    l.max_artifact_bytes = (base.total_bytes() - 1) as u64;
    assert!(reopen(NUMERIC, &p, &b, &StaticAuthorization::Unsigned, &d.0, &l).is_err());
}

fn constrained_boolean(instructions: &[Instruction], operand: &Operand, expected: bool) {
    assert!(matches!(operand, Operand::Variable(_)));
    assert!(
        instructions
            .iter()
            .any(|i| matches!(i,Instruction::ConstrainToBoolean{val} if val==operand))
    );
    let value = Operand::Immediate(midnight_transient_crypto::curve::Fr::from(u64::from(
        expected,
    )));
    assert!(
        instructions
            .iter()
            .any(|i| matches!(i,Instruction::ConstrainEq{a,b} if a==operand && b==&value))
    );
}
#[test]
fn repeated_claim_prior_guards_keys_and_partial_discharge_structure() {
    let (p, b) = bindings(FINANCIAL);
    let out = compile(FINANCIAL, &p, &b, &StaticAuthorization::Unsigned, &limits()).unwrap();
    let native = ir(&out, 0);
    let m = manifest(&out, 0);
    let claims = native
        .instructions
        .iter()
        .filter_map(|i| match i {
            Instruction::Impact { guard, inputs } => Some((guard, inputs)),
            _ => None,
        })
        .zip(&m.groups)
        .filter(|(_, g)| g.origin == "source-claim")
        .map(|(op, _)| op)
        .collect::<Vec<_>>();
    assert_eq!(claims.len(), 39);
    assert_eq!(claims[3].1, claims[16].1, "repeated recipient claim key");
    assert_ne!(claims[3].1, claims[29].1, "fee treasury claim key");
    let zero = Operand::Immediate(midnight_transient_crypto::curve::Fr::from(0u64));
    for offset in [0, 13] {
        let oldguard = claims[offset + 6].0;
        assert_eq!(claims[offset + 7].0, oldguard);
        assert_eq!(claims[offset + 8].0, oldguard);
        let Operand::Variable(id) = oldguard else {
            panic!("prior guard operand");
        };
        let prior = native
            .instructions
            .iter()
            .find_map(|i| match i {
                Instruction::Mul { b, output, .. } if output == id => Some(b),
                _ => None,
            })
            .unwrap();
        let Operand::Variable(id) = prior else {
            panic!("prior nonzero operand");
        };
        let prior_zero = native
            .instructions
            .iter()
            .find_map(|i| match i {
                Instruction::Not { a, output } if output == id => Some(a),
                _ => None,
            })
            .unwrap();
        let Operand::Variable(id) = prior_zero else {
            panic!("prior equality operand");
        };
        let prior_total = native
            .instructions
            .iter()
            .find_map(|i| match i {
                Instruction::TestEq { a, b, output } if output == id && b == &zero => Some(a),
                _ => None,
            })
            .unwrap();
        assert_eq!(
            prior_total,
            if offset == 0 {
                &zero
            } else {
                claims[10].1.last().unwrap()
            },
            "first prior absent, second prior linked to first positive aggregate"
        );
    }
    let repay = ir(&out, 1);
    let manifest = manifest(&out, 1);
    let impacts = repay
        .instructions
        .iter()
        .filter_map(|i| match i {
            Instruction::Impact { guard, inputs } => Some((guard, inputs)),
            _ => None,
        })
        .zip(&manifest.groups)
        .collect::<Vec<_>>();
    assert_eq!(
        impacts
            .iter()
            .filter(|(_, g)| g.origin == "duty-membership")
            .count(),
        20
    );
    assert_eq!(
        impacts
            .iter()
            .filter(|(_, g)| g.origin == "duty-old-read")
            .count(),
        12
    );
    assert_eq!(
        impacts
            .iter()
            .filter(|(_, g)| g.origin == "duty-write")
            .count(),
        4
    );
    assert!(
        repay
            .instructions
            .iter()
            .any(|i| matches!(i, Instruction::Assert { .. }))
    );
    let writes = impacts
        .iter()
        .filter(|(_, g)| g.origin == "duty-write")
        .map(|(op, _)| *op)
        .collect::<Vec<_>>();
    let next = writes[2].1.last().unwrap();
    let Operand::Variable(next_id) = next else {
        panic!("discharge result variable");
    };
    let (old, neg) = repay
        .instructions
        .iter()
        .find_map(|i| match i {
            Instruction::Add { a, b, output } if output == next_id => Some((a, b)),
            _ => None,
        })
        .unwrap();
    let Operand::Variable(neg_id) = neg else {
        panic!("subtraction negation variable");
    };
    assert!(repay.instructions.iter().any(|i|matches!(i,Instruction::Neg{a:Operand::Variable(input),output} if output==neg_id && input.0=="%input.0")));
    let prior_reads = impacts
        .iter()
        .filter(|(_, g)| g.origin == "duty-old-read")
        .map(|(op, _)| *op)
        .collect::<Vec<_>>();
    assert_eq!(
        old,
        prior_reads[8].1.last().unwrap(),
        "discharge old amount from the third eager duty observation"
    );
    assert!(
        repay
            .instructions
            .iter()
            .any(|i| matches!(i,Instruction::ConstrainBits{val,bits:128} if val==next))
    );
    selected_opening(&repay.instructions, old, prior_reads[8].0);
    let membership = impacts
        .iter()
        .filter(|(_, g)| g.origin == "duty-membership")
        .nth(14)
        .unwrap()
        .0
        .1
        .last()
        .unwrap();
    assert!(
        repay
            .instructions
            .iter()
            .any(|i| matches!(i,Instruction::Assert{cond} if cond==membership))
    );
}

fn selected_opening(instructions: &[Instruction], selected: &Operand, guard: &Operand) {
    let Operand::Variable(id) = selected else {
        panic!("selected opening variable");
    };
    let zero = Operand::Immediate(midnight_transient_crypto::curve::Fr::from(0u64));
    let raw = instructions
        .iter()
        .find_map(|i| match i {
            Instruction::CondSelect { bit, a, b, output }
                if output == id && bit == guard && b == &zero =>
            {
                Some(a)
            }
            _ => None,
        })
        .unwrap();
    let Operand::Variable(raw_id) = raw else {
        panic!("raw public input variable");
    };
    assert!(instructions.iter().any(|i|matches!(i,Instruction::PublicInput{guard:Some(g),output,..} if g==guard && output==raw_id)));
}
