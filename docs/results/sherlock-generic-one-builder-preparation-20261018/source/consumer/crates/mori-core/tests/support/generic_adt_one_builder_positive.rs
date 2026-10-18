//! SOURCE_ONLY / UNRUN. Two new public ONE-builder generic preparation checks.
//! Frozen independent full source/names/tables/records are the oracle.
#[path = "generic_adt_one_builder_checks.rs"]
mod checks;
#[path = "one_builder_independent_inputs.rs"]
mod independent_inputs;

#[test]
fn generic_fifo_after_signature_full_preparation() {
    checks::prepare_case(independent_inputs::generic_fifo_after_signature(),
        include_str!("../fixtures/generic-adt-one-builder/expectations/generic_fifo_after_signature.json"));
}
#[test]
fn signature_only_reuse_full_preparation() {
    checks::prepare_case(independent_inputs::signature_only_reuse(),
        include_str!("../fixtures/generic-adt-one-builder/expectations/signature_only_reuse.json"));
}
