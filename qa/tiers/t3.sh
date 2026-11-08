#!/usr/bin/env bash
# T3 (per milestone): Preview promotion checklist plus the T3 predicates of one milestone row set.
# Usage: qa/tiers/t3.sh [--review] [--base REF] [--milestone NAME]
#   --review    launch a non-blocking cold review in the background (never gates)
#   --base REF  diff base for the review (default: last tag, else origin/main)
#   --milestone sherlock-m6 (default; the C4 closure), synthetic-rollup-preview, synthetic-rollup-mainnet, or
#               all (the whole-tier sweep; a broader report, not a milestone gate)   (E15, SWF-012)
set -euo pipefail

review=0
base=""
milestone=sherlock-m6
while [[ $# -gt 0 ]]; do
  case "$1" in
    --review) review=1 ;;
    --base)
      [[ $# -ge 2 ]] || { echo "t3.sh: --base needs a ref" >&2; exit 2; }
      base="$2"
      shift
      ;;
    --milestone)
      [[ $# -ge 2 ]] || { echo "t3.sh: --milestone needs a name" >&2; exit 2; }
      milestone="$2"
      shift
      ;;
    -h | --help)
      sed -n '2,8p' "${BASH_SOURCE[0]}"
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
Preview promotion checklist (stagenet is retired)
  [ ] T0, T1 and T2 pass on this head (scripts/doctorwatson next shows no earlier failure)
  [ ] holdout verify passes and the evaluator has run the sealed scenarios
  [ ] host preflight is green (disk, load, no other heavy run)
  [ ] Preview wallet funded from the faucet and DUST registered; balances recorded
  [ ] proof server reachable; compiler and SDK pins match the repo pins
  [ ] contract deploy and every call transaction succeed on Preview; tx hashes captured
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

scope=(--milestone "$milestone")
[[ "$milestone" == all ]] && scope=()
dw_step "check --tier T3 (milestone $milestone)" scripts/doctorwatson check --tier T3 "${scope[@]}"

dw_finish
