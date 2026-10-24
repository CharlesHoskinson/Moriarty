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
  dw_step "cargo test --workspace" cargo test --workspace --locked
else
  dw_skip_missing "soundness_mutation" "test target crates/mori-backend/tests/soundness_mutation.rs not found"
fi

dw_step "check --tier T1" scripts/doctorwatson check --tier T1

dw_finish
