#!/usr/bin/env bash
# Slice gates V.2..V.8 (sherlock-modular-leaf-20261111): run one mode of the sherlock-leaf
# `slice` example in the T6 openvm tree and require every case PASS. Usage: check-slice.sh vN
set -euo pipefail
mode="$1"
ovm="${SHERLOCK_SDK_ROOT:-$HOME/.cache/m7-wrap/src/openvm-t6}"
export RUSTUP_AUTO_INSTALL=0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cache/t6/target-ovm}"
export SHERLOCK_SDK_ROOT="$ovm"
export T6_OUT="${T6_OUT:-$HOME/.cache/t6/check-out}"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-8}"
mkdir -p "$T6_OUT"
if [ -n "$(git -C "$ovm" status --porcelain -- crates Cargo.toml Cargo.lock)" ]; then
  echo "$mode FAIL: uncommitted changes in $ovm"; exit 1
fi
echo "openvm tree $ovm at $(git -C "$ovm" rev-parse HEAD)"
rm -f "$T6_OUT/$mode.json"
( cd "$ovm" && "$HOME/.local/bin/heavy-slot" --mem 24G --name "t6-check-$mode" -- \
    cargo run --release --offline -p sherlock-leaf --example slice -- "$mode" ) 2>&1 | grep -v "Air idx" | tail -40
python3 - "$T6_OUT/$mode.json" "$mode" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
bad = [c["case"] for c in d["cases"] if c["pass"] is not True]
if bad or not d["cases"]:
    sys.exit(f"{sys.argv[2]} FAIL: {bad}")
print(f"{sys.argv[2]} PASS: {len(d['cases'])} cases:", ", ".join(c["case"] for c in d["cases"]))
PY
