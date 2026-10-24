#!/usr/bin/env bash
# B1: deterministic Fenrir-derived dataset (seed, manifest, byte-identical regeneration) via the rollup-synth suite; needs the pinned external design sources.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
# FRTF_V3_ROOT: pinned Fenrir public corpus (v3). Overridable.
export FRTF_V3_ROOT="${FRTF_V3_ROOT:-$HOME/repos/projectfenrir/recursion/rollup-test-format/v3}"
cd models/sherlock/rollup-synth
out=$(python3 -m unittest discover -s tests 2>&1) && rc=0 || rc=$?
echo "$out" | tail -25
[ "$rc" -eq 0 ] || exit 1
echo "$out" | grep -Eq '^Ran [1-9][0-9]* tests' || exit 1
