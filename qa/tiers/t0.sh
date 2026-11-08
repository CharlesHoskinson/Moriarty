#!/usr/bin/env bash
# T0 (under 3 min): holdout verify, predicate validation, factory unit tests, F14
# traceability, F17 no-Fenrir guard, cargo check/fmt for touched crates, F15 when a pin file is touched.
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
dw_step "profile isolation" python3 -B scripts/qa/check-profile-isolation.py
dw_step "F14 traceability" python3 -B scripts/qa/check-traceability.py
dw_step "F17 no Fenrir dependency" scripts/qa/check-no-fenrir.sh

# Map changed paths to workspace crates (crates/<dir>/... -> package name).
declare -A seen=()
crates=()
touched_rs=0
touched_pins=0
for p in "${changed[@]}"; do
  [[ "$p" == *.rs ]] && touched_rs=1
  case "$p" in
    Cargo.toml | Cargo.lock | crates/*/Cargo.toml | qa/sherlock/target-pins.json) touched_pins=1 ;;
    tools/sherlock-sdk/*.test.mjs) ;;
    tools/sherlock-sdk/*.mjs | tools/sherlock-sdk/package.json | tools/sherlock-sdk/package-lock.json) touched_pins=1 ;;
  esac
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

# F15 static target pins (SWF-007: "when a commit touches pins"). It runs when a
# pin file changes (Cargo manifests and lockfile, target-pins.json, the SDK
# manifest and the SDK version asserts); other commits skip it.
# `scripts/doctorwatson check F15` always runs it.
if [[ $touched_pins -eq 1 ]]; then
  dw_step "F15 target pins" python3 -B scripts/qa/check-target-pins.py
else
  dw_skip "F15 target pins" "no pin file touched"
fi

dw_finish
