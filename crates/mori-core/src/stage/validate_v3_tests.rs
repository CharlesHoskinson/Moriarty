//! UNRUN owner-authored `validate_v3` source tests. Expectations are
//! hand-derived from the `/3` validation contract in `validate_v3`, not
//! from any evaluator run. They exercise `parser::parse_v3` followed
//! directly by `validate_v3`, without stage execution.
use super::{validate_v2, validate_v3};
use crate::parser::parse_v3;

const ASSET: &str = "asset A = { network: \"local\", issuer: \"i\", reference: \"A\", scale: 0 };";
const DUTY: &str = "duty \"loan\" = { asset: A, debtor: \"alice\", creditor: \"bob\" };";

fn v3(body: &str) -> crate::Program {
    parse_v3(&format!(
        "language \"moriarty/3\"; agreement P {{ {body} }}"
    ))
    .expect("syntax-only /3 parse")
}

fn code(body: &str) -> String {
    validate_v3(&v3(body)).expect_err("refused by validate_v3").code
}

#[test]
fn duty_discharge_only_program_is_admissible() {
    let program = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ discharge(\"loan\", 3); }}"
    ));
    assert!(validate_v3(&program).is_ok());
}

#[test]
fn duty_backed_liability_read_is_admissible() {
    let program = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ requires liability_present(\"loan\"); discharge(\"loan\", 3); }}"
    ));
    assert!(validate_v3(&program).is_ok());
}

#[test]
fn statement_only_liability_read_remains_admissible_without_duty() {
    let program = v3(&format!(
        "{ASSET} action pay() {{ liability(\"loan\", A, \"alice\", \"bob\", 1); requires liability_amount(\"loan\") > 0; }}"
    ));
    assert!(validate_v3(&program).is_ok());
}

#[test]
fn awaiting_duty_program_is_admissible() {
    let program = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ await ready(go: Bool) when go; discharge(\"loan\", 3); }}"
    ));
    assert!(validate_v3(&program).is_ok());
}

#[test]
fn duplicate_duty_even_if_identical_is_refused() {
    assert_eq!(
        code(&format!(
            "{ASSET} {DUTY} {DUTY} action pay() {{ discharge(\"loan\", 1); }}"
        )),
        "mori::core::sequence_duty_declaration"
    );
}

#[test]
fn duty_conflicting_with_statement_identity_is_refused() {
    assert_eq!(
        code(&format!(
            "{ASSET} {DUTY} action pay() {{ liability(\"loan\", A, \"alice\", \"carol\", 1); }}"
        )),
        "mori::core::sequence_duty_declaration"
    );
}

#[test]
fn duty_with_unknown_asset_is_refused() {
    assert_eq!(
        code(
            "duty \"loan\" = { asset: Missing, debtor: \"alice\", creditor: \"bob\" }; action pay() { discharge(\"loan\", 1); }"
        ),
        "mori::core::sequence_duty_declaration"
    );
}

#[test]
fn duty_with_empty_name_is_refused() {
    assert_eq!(
        code(&format!(
            "{ASSET} duty \"\" = {{ asset: A, debtor: \"alice\", creditor: \"bob\" }}; action pay() {{ discharge(\"loan\", 1); }}"
        )),
        "mori::core::sequence_duty_declaration"
    );
}

#[test]
fn undeclared_liability_read_keeps_liability_code() {
    assert_eq!(
        code(&format!(
            "{ASSET} action pay() {{ requires liability_present(\"ghost\"); }}"
        )),
        "mori::core::liability"
    );
}

#[test]
fn conflicting_statement_identities_keep_liability_code() {
    assert_eq!(
        code(&format!(
            "{ASSET} action pay() {{ liability(\"loan\", A, \"alice\", \"bob\", 1); }} action other() {{ liability(\"loan\", A, \"alice\", \"carol\", 1); }}"
        )),
        "mori::core::liability"
    );
}

#[test]
fn profiles_are_mutually_refused() {
    let program3 = v3(&format!(
        "{ASSET} {DUTY} action pay() {{ discharge(\"loan\", 3); }}"
    ));
    assert_eq!(
        validate_v2(&program3).expect_err("/3 refused by /2").code,
        "mori::core::profile"
    );
    let program2 = crate::parser::parse_v2(
        "language \"moriarty/2\"; agreement P { action a() { emit \"x\"(1); } }",
    )
    .expect("syntax-only /2 parse");
    assert_eq!(
        validate_v3(&program2).expect_err("/2 refused by /3").code,
        "mori::core::profile"
    );
}
