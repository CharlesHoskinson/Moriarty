#!/usr/bin/env bash
# T3 (per milestone): stagenet promotion checklist plus the T3 predicates.
# Usage: qa/tiers/t3.sh [--review] [--base REF]
#   --review    launch a non-blocking cold review in the background (never gates)
#   --base REF  diff base for the review (default: last tag, else origin/main)
set -euo pipefail

review=0
base=""
while [[ $# -gt 0 ]]; do
  case "$1" in
    --review) review=1 ;;
    --base)
      [[ $# -ge 2 ]] || { echo "t3.sh: --base needs a ref" >&2; exit 2; }
      base="$2"
      shift
      ;;
    -h | --help)
      sed -n '2,6p' "${BASH_SOURCE[0]}"
      exit 0
      ;;
    *)
      echo "t3.sh: unknown flag: $1" >&2
      exit 2
      ;;
  esac
  shift
done

DW_TIER=T3
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

cat <<'CHECKLIST'
Stagenet promotion checklist
  [ ] T0, T1 and T2 pass on this head (scripts/doctorwatson next shows no earlier failure)
  [ ] holdout verify passes and the evaluator has run the sealed scenarios
  [ ] host preflight is green (disk, load, no other heavy run)
  [ ] stagenet wallet funded and DUST registered; balances recorded
  [ ] proof server reachable; compiler and SDK pins match the repo pins
  [ ] contract deploy and every call transaction succeed on stagenet; tx hashes captured
  [ ] on-chain result matches the local witness and public-input digest
  [ ] evidence written for each run (trace path, head sha, endpoints, versions)
  [ ] stop-rule clear (scripts/doctorwatson stop-rule exits 0)
  [ ] cold review read; blocking findings filed as predicates
CHECKLIST

if [[ $review -eq 1 ]]; then
  if [[ -z "$base" ]]; then
    base="$(git describe --tags --abbrev=0 2>/dev/null || true)"
    [[ -n "$base" ]] || base="origin/main"
  fi
  stamp="$(date -u +%Y%m%dT%H%M%SZ)"
  out="qa/sherlock/traces/review-${stamp}.txt"
  mkdir -p "$DW_ROOT/qa/sherlock/traces"
  # Routed through the bus: publishes review.request, runs the cold reviewer (codex exec -s read-only,
  # subscription login only: the API-key variables are stripped), waits in the background for at
  # most 3600 s in chunks of <= 240 s, then proceeds. With the bus off it just runs the reviewer.
  scripts/doctorwatson bus review --t3 --range "${base}..HEAD" --out "$out" \
    < /dev/null > "qa/sherlock/traces/review-${stamp}.log" 2>&1 &
  echo "T3 review launched in background (pid $!, base $base) -> $out"
fi

dw_step "check --tier T3" scripts/doctorwatson check --tier T3

dw_finish
