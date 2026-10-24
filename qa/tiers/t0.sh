#!/usr/bin/env bash
# T0 (under 3 min): holdout verify, predicate validation, factory unit tests,
# cargo test for touched crates, cargo fmt when .rs files are touched.
# Usage: qa/tiers/t0.sh [--staged | --all]
#   --staged  (default) changes in the index
#   --all     every uncommitted change: staged, unstaged and untracked
set -euo pipefail

mode=staged
for arg in "$@"; do
  case "$arg" in
    --staged) mode=staged ;;
    --all) mode=all ;;
    -h | --help)
      sed -n '2,6p' "${BASH_SOURCE[0]}"
      exit 0
      ;;
    *)
      echo "t0.sh: unknown flag: $arg" >&2
      exit 2
      ;;
  esac
done

DW_TIER=T0
# shellcheck source=qa/tiers/lib.sh
source "$(dirname "${BASH_SOURCE[0]}")/lib.sh"

changed=()
if [[ "$mode" == "staged" ]]; then
  mapfile -d '' -t changed < <(git diff --cached --name-only --no-renames -z)
else
  mapfile -d '' -t changed < <(
    git diff HEAD --name-only --no-renames -z
    git ls-files --others --exclude-standard -z
  )
fi

dw_step "holdout verify" scripts/doctorwatson holdout verify
dw_step "predicate validate" scripts/doctorwatson validate
dw_step "factory unittests" bash -c 'cd models/doctorwatson && python3 -m unittest discover -s factory/tests -t .'

# Map changed paths to workspace crates (crates/<dir>/... -> package name).
declare -A seen=()
crates=()
touched_rs=0
for p in "${changed[@]}"; do
  [[ "$p" == *.rs ]] && touched_rs=1
  if [[ "$p" == crates/*/* ]]; then
    dir="${p#crates/}"
    dir="${dir%%/*}"
    [[ -n "${seen[$dir]:-}" ]] && continue
    seen[$dir]=1
    pkg="$(dw_crate_package "$dir")"
    [[ -n "$pkg" ]] && crates+=("$pkg")
  fi
done

if [[ ${#crates[@]} -eq 0 ]]; then
  dw_skip "cargo check" "no crate touched"
else
  for pkg in "${crates[@]}"; do
    # T0 is a <3 min gate: compile every target of the touched crate. Full test
    # suites (some crates take tens of minutes in debug) run in T1.
    dw_step "cargo check -p $pkg" cargo check -p "$pkg" --all-targets --locked
  done
fi

if [[ $touched_rs -eq 1 ]]; then
  dw_step "cargo fmt --check" cargo fmt --all --check
else
  dw_skip "cargo fmt" "no .rs file touched"
fi

dw_finish
