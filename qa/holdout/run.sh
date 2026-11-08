#!/usr/bin/env bash
# Holdout entrypoint (evaluator-owned). Usage: qa/holdout/run.sh <suite> [--network NET]
#   suites: rejections-local | rejections-<NET> | uireq-NNN (001..035) | list
#   NET (config/networks.json): undeployed | preview | mainnet. stagenet is retired (owner decision
#   2026-11-08) and is refused. `--network NET` is accepted only for uireq-023 (scenario 2 target);
#   without it U023 uses DW_HOLDOUT_NETWORK, else the contract's default_network.
# Exit: 0 pass, 1 fail, 2 unimplemented (specified, system under test lacks the entrypoint), 3 host unfit.
set -u
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-6}"
export PYTHONDONTWRITEBYTECODE=1

if ! command -v python3 >/dev/null 2>&1; then
  echo "HOSTUNFIT: python3 not found"; exit 3
fi

usage() { echo "usage: run.sh rejections-local | rejections-{undeployed|preview|mainnet} | uireq-NNN | uireq-023 --network NET | list" >&2; exit 1; }
[ $# -eq 1 ] || [ $# -eq 3 ] || usage
if [ $# -eq 3 ]; then
  [ "$1" = "uireq-023" ] && [ "$2" = "--network" ] || usage
  export DW_HOLDOUT_NETWORK="$3"
fi

case "$1" in
  list)
    exec python3 - "$HERE" <<'PY'
import json, sys
from pathlib import Path
h = Path(sys.argv[1])
nets = json.loads((h / "config" / "networks.json").read_text())["selectable"]
print("suite\tcase\tstatus")
for d in sorted((h / "cases" / "rejections").iterdir()):
    c = json.loads((d / "case.json").read_text())
    if c.get("retired"):
        print(f"retired-{c['scope']}\t{c['id']}\tretired")
    elif c["scope"] == "local":
        print(f"rejections-local\t{c['id']}\trunnable-with-parent-verifier+fixtures")
    else:
        for n in nets:
            if n in c.get("networks", []):
                print(f"rejections-{n}\t{c['id']}\trunnable-with-{n}-reject-receipt")
for d in sorted((h / "cases" / "uireq").iterdir()):
    c = json.loads((d / "case.json").read_text())
    sts = [s["status"] for s in c["scenarios"]]
    st = "runnable" if all(s == "runnable" for s in sts) else ("partial" if "runnable" in sts else "specified-only")
    print(f"uireq-{d.name}\t{c['uireq']}\t{st}")
PY
    ;;
  rejections-*)        exec python3 "$HERE/lib/rejections.py" "${1#rejections-}" ;;
  uireq-[0-9][0-9][0-9])
    n="${1#uireq-}"
    if [ "$((10#$n))" -lt 1 ] || [ "$((10#$n))" -gt 35 ]; then usage; fi
    exec python3 "$HERE/lib/uireq.py" "$n" ;;
  *) usage ;;
esac
