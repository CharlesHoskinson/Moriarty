#!/usr/bin/env bash
# T2: host preflight, then the T2 predicates (heavy proofs, twin rehearsal).
set -euo pipefail

DW_TIER=T2
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

dw_step "preflight" scripts/doctorwatson preflight
dw_step "check --tier T2" scripts/doctorwatson check --tier T2

dw_finish
