#!/usr/bin/env bash
# Holdout entrypoint (evaluator-owned). Usage: qa/holdout/run.sh <suite>
#   suites: rejections-local | rejections-stagenet | uireq-NNN (001..035) | list
# Exit: 0 pass, 1 fail, 2 unimplemented (specified, system under test lacks the entrypoint), 3 host unfit.
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-6}"
export PYTHONDONTWRITEBYTECODE=1

if ! command -v python3 >/dev/null 2>&1; then
  echo "HOSTUNFIT: python3 not found"; exit 3
fi

usage() { echo "usage: run.sh rejections-local | rejections-stagenet | uireq-NNN | list" >&2; exit 1; }
[ $# -eq 1 ] || usage

case "$1" in
  list)
    exec python3 - "$HERE" <<'PY'
import json, sys
from pathlib import Path
h = Path(sys.argv[1])
print("suite\tcase\tstatus")
for d in sorted((h / "cases" / "rejections").iterdir()):
    c = json.loads((d / "case.json").read_text())
    st = "runnable-with-parent-verifier+fixtures" if c["scope"] == "local" else "specified-only"
    print(f"rejections-{c['scope']}\t{c['id']}\t{st}")
for d in sorted((h / "cases" / "uireq").iterdir()):
    c = json.loads((d / "case.json").read_text())
    sts = [s["status"] for s in c["scenarios"]]
    st = "runnable" if all(s == "runnable" for s in sts) else ("partial" if "runnable" in sts else "specified-only")
    print(f"uireq-{d.name}\t{c['uireq']}\t{st}")
PY
    ;;
  rejections-local)    exec python3 "$HERE/lib/rejections.py" local ;;
  rejections-stagenet) exec python3 "$HERE/lib/rejections.py" stagenet ;;
  uireq-[0-9][0-9][0-9])
    n="${1#uireq-}"
    if [ "$((10#$n))" -lt 1 ] || [ "$((10#$n))" -gt 35 ]; then usage; fi
    exec python3 "$HERE/lib/uireq.py" "$n" ;;
  *) usage ;;
esac
