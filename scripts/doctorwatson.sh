#!/usr/bin/env bash
# Local QA; --design and --host select explicit additional local checks.
# See docs/DOCTORWATSON-CI.md for evidence limits.
set -euo pipefail

design=false
host=false
if [[ $# -eq 1 && $1 == --design ]]; then
  design=true
elif [[ $# -eq 1 && $1 == --host ]]; then
  host=true
elif [[ $# -ne 0 ]]; then
  printf 'Usage: %s [--design|--host]\n' "$0" >&2
  exit 2
fi

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"
export PYTHONDONTWRITEBYTECODE=1

if "$design"; then
  node_command="${DOCTORWATSON_NODE:-node}"
  quint_command="${DOCTORWATSON_QUINT:-quint}"
  for tool in "$node_command" "$quint_command" timeout env; do
    if ! command -v -- "$tool" >/dev/null 2>&1; then
      printf 'DoctorWatson --design requires an already installed tool: %s\n' "$tool" >&2
      exit 127
    fi
  done
  node_path="$(command -v -- "$node_command")"
  quint_path="$(command -v -- "$quint_command")"
  [[ "$node_path" == /* ]] || node_path="$PWD/$node_path"
  [[ "$quint_path" == /* ]] || quint_path="$PWD/$quint_path"
  if [[ ! -x "$node_path" || ! -f "$quint_path" ]]; then
    printf 'DoctorWatson --design requires a Node executable and a Quint JavaScript CLI file.\n' >&2
    exit 127
  fi
fi

python3 models/sherlock/qa/check.py
python3 -m unittest discover -s models/sherlock/qa -p 'test_*.py' -v

if "$host"; then
  python3 -I -B models/sherlock/qa/run_host.py \
    --rustc "${DOCTORWATSON_RUSTC:-}" \
    --output-dir "${DOCTORWATSON_OUTPUT_DIR:-}"
fi

if "$design"; then
  # Each local model command has its own 30-second wall limit and 256 MiB heap.
  # No install, verify, Rust backend or fallback command is selected.
  timeout --signal=KILL 30s env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR="${TMPDIR:-/tmp}" \
    "$node_path" --max-old-space-size=256 "$quint_path" \
    typecheck models/doctorwatson/doctorwatson.qnt
  timeout --signal=KILL 30s env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR="${TMPDIR:-/tmp}" \
    "$node_path" --max-old-space-size=256 "$quint_path" \
    test models/doctorwatson/doctorwatson_test.qnt --main=doctorwatson_test \
    --backend=typescript --max-samples=1 --seed=1
  timeout --signal=KILL 30s env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR="${TMPDIR:-/tmp}" \
    "$node_path" --max-old-space-size=256 "$quint_path" \
    run models/doctorwatson/doctorwatson.qnt --main=doctorwatson_small \
    --backend=typescript --max-samples=10000 --max-steps=60 --seed=1 --verbosity=1 \
    --invariant=workflowInvariant \
    --witnesses claimed frozen edited submitted started checkerAdvanced checkerFailed \
    testPassed timedOut reviewed rebased integrated dependenciesChanged availabilityChanged \
    layerRecorded completed processStoppedWitness reproduced approachChanged cancelled \
    blockerNamed finalized parallel sourceOnly sharedChanged
  # Directed reachability stays separate from the default random witness counts.
  # The deterministic test must finish all 47 steps; witness counts alone can be zero.
  timeout --signal=KILL 30s env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR="${TMPDIR:-/tmp}" \
    "$node_path" --max-old-space-size=256 "$quint_path" \
    test models/doctorwatson/doctorwatson_coverage_test.qnt --main=doctorwatson_coverage_test \
    --backend=typescript --max-samples=1 --seed=1
  timeout --signal=KILL 30s env -i PATH=/usr/bin:/bin LANG=C.UTF-8 TMPDIR="${TMPDIR:-/tmp}" \
    "$node_path" --max-old-space-size=256 "$quint_path" \
    run models/doctorwatson/doctorwatson_coverage.qnt --main=doctorwatson_coverage \
    --backend=typescript --max-samples=1 --max-steps=50 --seed=1 --verbosity=1 \
    --invariant=directedInvariant \
    --witnesses rebased integrated dependenciesChanged completed sourceOnly directedFinished
fi
