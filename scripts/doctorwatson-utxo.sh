#!/usr/bin/env bash
# Public local metadata and read-only Cardano next-action selection.
# Explicit selectors run the reader without the default reporting regressions.
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"
export PYTHONDONTWRITEBYTECODE=1
if [[ ${1:-} == cardano-next ]]; then
  shift
  set -- --cardano-next "$@"
fi
python3 models/utxo/qa/check.py "$@"
if [[ $# -eq 0 ]]; then
  python3 -m unittest discover -s models/utxo/qa -p 'test_*.py' -v
fi
