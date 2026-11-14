#!/bin/bash
# Rerun the Quint commands behind every trace of tests/quint-regressions (models/traces.tsv):
# each `quint run` must exit 1 (the invariant violation the trace witnesses), each `quint test`
# must pass; the regenerated ITF states must equal the committed trace.
# usage: models/run.sh [substring]   (Quint 0.32.0; light memory, one run at a time)
set -u
here=$(cd "$(dirname "$0")" && pwd)
dir=$(dirname "$here")
tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
cd "$here"
fail=0
while IFS=$'\t' read -r trace kind cmd; do
  [[ $trace == \#* || -z $trace ]] && continue
  [[ -n ${1:-} && $trace != *"$1"* ]] && continue
  if [[ $kind == run ]]; then
    eval "$cmd --out-itf $tmp/out.itf.json" > "$tmp/log" 2>&1; rc=$?; want=1; out=$tmp/out.itf.json
  else
    eval "$cmd --out-itf '$tmp/{test}_{seq}.itf.json'" > "$tmp/log" 2>&1; rc=$?; want=0
    out=$(ls "$tmp"/*_*.itf.json 2>/dev/null | head -1)
  fi
  same=$(python3 - "$out" "$dir/$trace" <<'P'
import json, sys
def st(p):
    try:
        return [{k: v for k, v in s.items() if k != '#meta'} for s in json.load(open(p))['states']]
    except Exception:
        return None
a, b = st(sys.argv[1]), st(sys.argv[2])
print('same' if a is not None and a == b else 'DIFFERENT')
P
)
  if [[ $rc == "$want" && $same == same ]]; then echo "OK   $trace (exit $rc)"; else echo "FAIL $trace (exit $rc, want $want, trace $same)"; tail -3 "$tmp/log"; fail=1; fi
  rm -f "$tmp"/*.itf.json
done < "$here/traces.tsv"
exit $fail
