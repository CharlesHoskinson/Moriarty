#!/usr/bin/env bash
# B1: deterministic FSYN dataset (seed, golden pins, manifest, byte-identical
# regeneration) from the in-repository S0 golden only (SSRD-001, SSRD-003, SSRD-030).
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
unset FRTF_V3_ROOT FSYN_SOURCE_MAP FSYN_DESIGN_CUT FSYN_ARCHIVE_ROOT
cd models/sherlock/rollup-synth
python3 -B fsyn.py golden check
out=$(python3 -B -m unittest discover -s tests 2>&1) && rc=0 || rc=$?
echo "$out" | tail -25
[ "$rc" -eq 0 ] || exit 1
echo "$out" | grep -Eq '^Ran [1-9][0-9]* tests' || exit 1
