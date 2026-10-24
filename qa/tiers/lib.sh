# shellcheck shell=bash
# Shared helpers for the DoctorWatson tier runners. Source this file; do not execute it.

DW_ROOT="$(git rev-parse --show-toplevel)"
export DW_ROOT
cd "$DW_ROOT"

DW_TIER="${DW_TIER:-T?}"
DW_PASS=0
DW_FAIL=0
DW_SKIP=0
DW_SKIP_MISSING=0
DW_T_START=$SECONDS
DW_TMP="$(mktemp -d)"
trap 'rm -rf "$DW_TMP"' EXIT

# Git exports these to hooks. Steps must not inherit them: a test that runs `git`
# in a scratch repo would otherwise write into this repository's index.
DW_SCRUB_ENV=(env -u GIT_DIR -u GIT_INDEX_FILE -u GIT_WORK_TREE -u GIT_OBJECT_DIRECTORY
  -u GIT_ALTERNATE_OBJECT_DIRECTORIES -u GIT_COMMON_DIR -u GIT_PREFIX -u GIT_QUARANTINE_PATH)

# dw_step NAME CMD... : run CMD, print one PASS/FAIL line, tail the output on failure.
# DW_VERBOSE=1 streams the output as it runs.
dw_step() {
  local name="$1"
  shift
  local log="$DW_TMP/step-$((DW_PASS + DW_FAIL + DW_SKIP)).log"
  local t0=$SECONDS rc=0
  if [[ "${DW_VERBOSE:-0}" == "1" ]]; then
    set +e
    "${DW_SCRUB_ENV[@]}" "$@" 2>&1 | tee "$log"
    rc=${PIPESTATUS[0]}
    set -e
  else
    "${DW_SCRUB_ENV[@]}" "$@" >"$log" 2>&1 || rc=$?
  fi
  local dt=$((SECONDS - t0))
  if [[ $rc -eq 0 ]]; then
    DW_PASS=$((DW_PASS + 1))
    printf '%s PASS  %s (%ss)\n' "$DW_TIER" "$name" "$dt"
  else
    DW_FAIL=$((DW_FAIL + 1))
    printf '%s FAIL  %s (exit %s, %ss)\n' "$DW_TIER" "$name" "$rc" "$dt"
    if [[ "${DW_VERBOSE:-0}" != "1" ]]; then
      tail -n 30 "$log" | sed 's/^/    | /'
    fi
  fi
}

# dw_skip NAME REASON : a step that did not apply (does not fail the tier).
dw_skip() {
  DW_SKIP=$((DW_SKIP + 1))
  printf '%s SKIP  %s (%s)\n' "$DW_TIER" "$1" "$2"
}

# dw_skip_missing NAME REASON : a required step whose target does not exist yet.
# Never counts as a pass. Fails the tier only when DW_STRICT_SKIP=1.
dw_skip_missing() {
  DW_SKIP=$((DW_SKIP + 1))
  DW_SKIP_MISSING=$((DW_SKIP_MISSING + 1))
  printf '%s SKIP-MISSING  %s (%s)\n' "$DW_TIER" "$1" "$2"
}

# dw_finish : print the total line and exit with the tier status.
dw_finish() {
  local verdict=PASS rc=0
  if [[ $DW_FAIL -gt 0 ]]; then
    verdict=FAIL
    rc=1
  elif [[ $DW_SKIP_MISSING -gt 0 ]]; then
    verdict="INCOMPLETE"
    if [[ "${DW_STRICT_SKIP:-0}" == "1" ]]; then
      verdict=FAIL
      rc=1
    fi
  fi
  printf '%s %s  passed=%d failed=%d skipped=%d total=%ss\n' \
    "$DW_TIER" "$verdict" "$DW_PASS" "$DW_FAIL" "$DW_SKIP" "$((SECONDS - DW_T_START))"
  exit "$rc"
}

# dw_crate_package DIR : print the cargo package name of crates/DIR, or nothing.
dw_crate_package() {
  local manifest="$DW_ROOT/crates/$1/Cargo.toml"
  [[ -f "$manifest" ]] || return 0
  awk '
    /^\[package\]/ { p = 1; next }
    /^\[/ { p = 0 }
    p && /^name[ \t]*=/ { sub(/^name[ \t]*=[ \t]*"/, ""); sub(/".*/, ""); print; exit }
  ' "$manifest"
}
