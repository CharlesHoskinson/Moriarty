#!/usr/bin/env bash
# T2: host preflight, then the T2 predicates (heavy proofs, twin rehearsal) of one milestone row set.
# Usage: qa/tiers/t2.sh [--milestone NAME]
#   NAME: sherlock-m6 (default; the C4 closure), synthetic-rollup-preview, synthetic-rollup-mainnet, or
#         all (the whole-tier sweep; a broader report, not a milestone gate)   (E15, SWF-012)
set -euo pipefail

milestone=sherlock-m6
while [[ $# -gt 0 ]]; do
  case "$1" in
    --milestone)
      [[ $# -ge 2 ]] || { echo "t2.sh: --milestone needs a name" >&2; exit 2; }
      milestone="$2"
      shift
      ;;
    -h | --help)
      sed -n '2,5p' "${BASH_SOURCE[0]}"
      exit 0
      ;;
    *)
      echo "t2.sh: unknown flag: $1" >&2
      exit 2
      ;;
  esac
  shift
done

DW_TIER=T2
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

scope=(--milestone "$milestone")
[[ "$milestone" == all ]] && scope=()

dw_step "preflight" scripts/doctorwatson preflight
dw_step "check --tier T2 (milestone $milestone)" scripts/doctorwatson check --tier T2 "${scope[@]}"

dw_finish
