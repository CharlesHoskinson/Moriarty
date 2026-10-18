#!/usr/bin/env bash
# Public local metadata only. Selectors run the reader without regressions.
set -euo pipefail
repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_root"
export PYTHONDONTWRITEBYTECODE=1
python3 models/utxo/qa/check.py "$@"
if [[ $# -eq 0 ]]; then
  python3 -m unittest discover -s models/utxo/qa -p 'test_*.py' -v
fi
