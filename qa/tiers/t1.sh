#!/usr/bin/env bash
# T1 (up to 20 min): witness-mutation soundness test, then the T1 predicates.
# A missing soundness_mutation test target reports SKIP-MISSING (never PASS).
# DW_STRICT_SKIP=1 makes SKIP-MISSING fail the tier.
set -euo pipefail

DW_TIER=T1
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

test_src=crates/mori-backend/tests/soundness_mutation.rs
if [[ -f "$test_src" ]] || grep -qs 'name *= *"soundness_mutation"' crates/mori-backend/Cargo.toml; then
  dw_step "soundness_mutation" cargo test -p mori-backend --test soundness_mutation --locked
  # MockProver tests above k=13 are #[ignore] in plain `cargo test` (task 2.0, SRC-016); run them one at a time.
  dw_step "soundness_mutation k>13" cargo test -p mori-backend --test soundness_mutation --locked -- \
    --ignored --exact --test-threads=1 \
    data_canonical_mutations_are_rejected checked_value_read_mutations_are_rejected \
    scalar_failure_step_mutations_are_rejected
  dw_step "mori-backend lib k>13" cargo test -p mori-backend --lib --locked -- \
    --ignored --exact --test-threads=1 \
    sherlock_machine::c6_checked_value_read_tests::mockprover_synthesis_positive_and_wrong_kind \
    sherlock_machine::c6_data_canonical::tests::in_circuit_mockprover_synthesis_outcomes \
    sherlock_machine::c6_scalar_failure_step::tests::synthesis_distinguishes_constraint_failure
  dw_step "cargo test --workspace" cargo test --workspace --locked
else
  dw_skip_missing "soundness_mutation" "test target crates/mori-backend/tests/soundness_mutation.rs not found"
fi

dw_step "check --tier T1" scripts/doctorwatson check --tier T1

dw_finish
