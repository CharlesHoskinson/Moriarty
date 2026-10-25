#!/usr/bin/env bash
# B1: deterministic Fenrir-derived dataset (seed, manifest, byte-identical regeneration) via the rollup-synth suite; needs the pinned external design sources.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
# Resolve moved source bytes against the approved 123-source baseline, including
# recovered revision-20 inputs. Never substitute newer design source bytes.
export FENRIR_ROOT="${FENRIR_ROOT:-$HOME/repos/projectfenrir-main}"
export FRTF_V3_ROOT="${FRTF_V3_ROOT:-$FENRIR_ROOT/recursion/rollup-test-format/v3}"
python3 -B -m unittest discover -s scripts/qa/tests -p 'test_fsyn_*.py'
if [[ -z "${FSYN_SOURCE_MAP:-}" ]]; then
  FSYN_SOURCE_MAP="$(python3 -B models/sherlock/rollup-synth/sources.py \
    --cut "${FSYN_DESIGN_CUT:-openspec/changes/sherlock-synthetic-rollup-data/design-cut.json}" \
    --fenrir-root "$FENRIR_ROOT" --cache "$PWD/target/fsyn-sources")"
fi
export FSYN_SOURCE_MAP
cd models/sherlock/rollup-synth
out=$(python3 -m unittest discover -s tests 2>&1) && rc=0 || rc=$?
echo "$out" | tail -25
[ "$rc" -eq 0 ] || exit 1
echo "$out" | grep -Eq '^Ran [1-9][0-9]* tests' || exit 1
