#!/usr/bin/env bash
# T4 (mainnet only; owner-only): mainnet checklist, then the T4 predicates.
# Refuses unless the Preview rollback rehearsal receipt is committed (SWF-009, workflow.md §7 item 5).
# Usage: qa/tiers/t4.sh [--milestone NAME]   (NAME: fenrir-mainnet; default: every T4 row)
# Skeleton (E5): predicates observe and never spend. Each mainnet operation still runs separately,
# under explicit owner authorization at the time it runs and an owner-set spend ceiling, no retry.
set -euo pipefail

milestone=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --milestone)
      [[ $# -ge 2 ]] || { echo "t4.sh: --milestone needs a name" >&2; exit 2; }
      milestone=(--milestone "$2")
      shift
      ;;
    -h | --help)
      sed -n '2,6p' "${BASH_SOURCE[0]}"
      exit 0
      ;;
    *)
      echo "t4.sh: unknown flag: $1" >&2
      exit 2
      ;;
  esac
  shift
done

DW_TIER=T4
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

# OP-PV-ROLLBACK-REHEARSAL receipt (tasks.md M6 operations table, task 6.7).
receipt="qa/sherlock/receipt-traces/OP-PV-ROLLBACK/receipt.json"
refuse() {
  echo "t4.sh: REFUSED: $1" >&2
  echo "t4.sh: run OP-PV-ROLLBACK-REHEARSAL on Preview (task 6.7) and commit its receipt first (SWF-009)" >&2
  exit 9
}
[[ -f "$receipt" ]] || refuse "Preview rollback rehearsal receipt missing: $receipt"
git ls-files --error-unmatch -- "$receipt" >/dev/null 2>&1 ||
  refuse "Preview rollback rehearsal receipt not committed: $receipt"
git diff --quiet HEAD -- "$receipt" ||
  refuse "Preview rollback rehearsal receipt has changes that are not committed: $receipt"
echo "T4 rehearsal receipt committed: $receipt"

cat <<'CHECKLIST'
Mainnet pre-deploy checklist (workflow.md §6; SWF-009)
  [ ] local Docker at ledger 9.1.0.1 passed (undeployed rows)
  [ ] Preview qualification receipts committed (A8p, A9p, A10p; B4p, B5p, B8p, B9p for Fenrir)
  [ ] Preview rollback rehearsal receipt committed (checked above)
  [ ] validatePassword before any transaction; at least two DUST spends (#838)
  [ ] maintenance authority declared, owner-held, equal to the target-pins.json mainnet row (SMN-007)
  [ ] TooManyPublicInputs and the 256-op cap checked (SMN-006)
  [ ] wallet seed backed up encrypted after every deployment
  [ ] owner authorization and spend ceiling recorded for each mainnet operation; no automatic retry
CHECKLIST

dw_step "check --tier T4 ${milestone[*]}" scripts/doctorwatson check --tier T4 "${milestone[@]}"

dw_finish
