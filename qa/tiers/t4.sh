#!/usr/bin/env bash
# T4 (mainnet only; owner-only): gate one mainnet operation, print the checklist, run the T4 predicates.
# Usage: qa/tiers/t4.sh OP-ID [--milestone NAME]   (OP-ID: OP-MN-...; NAME e.g. synthetic-rollup-mainnet)
# Refuses (exit 9) unless both are committed (SWF-009, workflow.md §7 item 5):
#   qa/sherlock/receipt-traces/OP-PV-ROLLBACK-REHEARSAL/receipt.json   Preview rollback rehearsal
#   qa/sherlock/authorizations/OP-ID.json   owner authorization: op, authorizedBy, spendCeiling,
#                                           expires (ISO date, not past), singleUse: true
# A single-use authorization is spent once qa/sherlock/receipt-traces/OP-ID/receipt.json exists.
# Predicates observe and never spend; the operation itself runs separately, with no automatic retry.
set -euo pipefail

usage() {
  echo "t4.sh: usage: qa/tiers/t4.sh OP-MN-<NAME> [--milestone NAME]" >&2
  exit 2
}

op=""
milestone=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --milestone)
      [[ $# -ge 2 ]] || usage
      milestone=(--milestone "$2")
      shift
      ;;
    -h | --help)
      sed -n '2,9p' "${BASH_SOURCE[0]}"
      exit 0
      ;;
    -*) usage ;;
    *)
      [[ -z "$op" ]] || usage
      op="$1"
      ;;
  esac
  shift
done
[[ "$op" =~ ^OP-MN-[A-Z0-9][A-Z0-9-]*$ ]] || usage

DW_TIER=T4
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

refuse() {
  echo "t4.sh: REFUSED $op: $1" >&2
  exit 9
}

# committed PATH WHAT : PATH exists, is tracked and has no uncommitted change.
committed() {
  [[ -f "$1" ]] || refuse "$2 missing: $1"
  git ls-files --error-unmatch -- "$1" >/dev/null 2>&1 || refuse "$2 not committed: $1"
  git diff --quiet HEAD -- "$1" || refuse "$2 has changes that are not committed: $1"
}

receipt="qa/sherlock/receipt-traces/OP-PV-ROLLBACK-REHEARSAL/receipt.json"
committed "$receipt" "Preview rollback rehearsal receipt (run OP-PV-ROLLBACK-REHEARSAL, task 6.7)"

auth="qa/sherlock/authorizations/$op.json"
committed "$auth" "owner authorization record"
why="$(python3 -B - "$auth" "$op" <<'PY'
import datetime, json, sys
path, op = sys.argv[1], sys.argv[2]
try:
    with open(path, encoding="utf-8") as fh:
        rec = json.load(fh)
except (OSError, ValueError) as exc:
    print(f"authorization is not valid JSON: {exc}"); sys.exit(1)
if not isinstance(rec, dict):
    print("authorization must be a JSON object"); sys.exit(1)
if rec.get("op") != op:
    print(f"authorization op must be {op!r} (got {rec.get('op')!r})"); sys.exit(1)
if not (isinstance(rec.get("authorizedBy"), str) and rec["authorizedBy"].strip()):
    print("authorization authorizedBy must be a non-empty string"); sys.exit(1)
ceil = rec.get("spendCeiling")
if not ((isinstance(ceil, str) and ceil.strip()) or
        (isinstance(ceil, (int, float)) and not isinstance(ceil, bool) and ceil > 0)):
    print("authorization spendCeiling must be a positive number or a non-empty string"); sys.exit(1)
try:
    exp = datetime.date.fromisoformat(str(rec.get("expires"))[:10])
except ValueError:
    print(f"authorization expires must be an ISO date (got {rec.get('expires')!r})"); sys.exit(1)
if exp < datetime.datetime.now(datetime.timezone.utc).date():
    print(f"authorization expired on {exp.isoformat()}"); sys.exit(1)
if rec.get("singleUse") is not True:
    print("authorization singleUse must be true"); sys.exit(1)
print(f"authorized by {rec['authorizedBy']}, spend ceiling {ceil}, expires {exp.isoformat()}")
PY
)" || refuse "$why ($auth)"
[[ ! -e "qa/sherlock/receipt-traces/$op/receipt.json" ]] ||
  refuse "single-use authorization already used: qa/sherlock/receipt-traces/$op/receipt.json exists"

echo "T4 $op: rehearsal receipt committed: $receipt"
echo "T4 $op: $why"

cat <<'CHECKLIST'
Mainnet pre-deploy checklist (workflow.md §6; SWF-009)
  [ ] local Docker at ledger 9.1.0.1 passed (undeployed rows)
  [ ] Preview qualification receipts committed (A8p, A9p, A10p; B4p, B5p, B8p, B9p for the synthetic rollup)
  [x] Preview rollback rehearsal receipt committed (checked above)
  [x] owner authorization record committed, unexpired, single use (checked above)
  [ ] validatePassword before any transaction; at least two DUST spends (#838)
  [ ] maintenance authority declared, owner-held, equal to the target-pins.json mainnet row (SMN-007)
  [ ] TooManyPublicInputs and the 256-op cap checked (SMN-006)
  [ ] wallet seed backed up encrypted after every deployment
  [ ] spend stays within the authorization's spendCeiling; no automatic retry
CHECKLIST

dw_step "check --tier T4 ${milestone[*]}" scripts/doctorwatson check --tier T4 "${milestone[@]}"

dw_finish
