#!/usr/bin/env bash
# V.9 and V.10 (sherlock-modular-leaf-20261111): the slice leaf shard through the recursion
# layers in the T6 openvm tree. Usage: check-chain.sh omit|chain
#   omit:  `slice_chain omit` (V.9 slice_air_omitted and the T3 omit cases on the slice keys)
#   chain: `slice_chain chain` then `fp8_d3` (V.10 one run through leaf, N1, N2, N3, L, L_Fr; D3)
set -euo pipefail
mode="$1"
ovm="${SHERLOCK_SDK_ROOT:-$HOME/.cache/m7-wrap/src/openvm-t6}"
export RUSTUP_AUTO_INSTALL=0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cache/t6/target-ovm}"
export SHERLOCK_SDK_ROOT="$ovm"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-8}"
out="${T6_CHAIN_OUT:-$HOME/.cache/t6/check-$mode}"
mkdir -p "$out"
if [ -n "$(git -C "$ovm" status --porcelain -- crates Cargo.toml Cargo.lock)" ]; then
  echo "$mode FAIL: uncommitted changes in $ovm"; exit 1
fi
echo "openvm tree $ovm at $(git -C "$ovm" rev-parse HEAD); out $out"
run() { ( cd "$ovm" && "$HOME/.local/bin/heavy-slot" ${T6_EXCLUSIVE:+--exclusive} --mem 32G --name "t6-check-$1" -- \
  cargo run --release --offline -p sherlock-wrap-export --example "$@" ); }
if [ "$mode" = omit ]; then
  rm -f "$out/omit.json"
  run slice_chain omit "$out" 2>&1 | grep -E "SLICEOMIT|panicked at examples|error" | tail -80
  python3 - "$out/omit.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
bad = [c["case"] for c in d["cases"] if not c["pass"]]
req = d["all_required"]
names = [c["case"] for c in d["cases"]]
need = ["slice_air_omitted_stock_verify_without_", "n1_parent_over_child_without_", "n3_parent_over_child_without_", "l_stock_verify_without_"]
if bad or not all(req.values()) or not all(any(n.startswith(p) for n in names) for p in need):
    sys.exit(f"V.9 FAIL: failing {bad}, all_required {req}")
s = d["app_shape"]
print(f"V.9 PASS: {len(names)} cases; every AIR required in {sorted(req)}; app shape {s['airs']} AIRs, {s['columns']} columns, max interactions {s['max_interactions']}")
PY
elif [ "$mode" = chain ]; then
  rm -f "$out/chain.json" "$out/d3.json"
  run slice_chain chain "$out" 2>&1 | grep -E "SLICECHAIN|SLICEKEY|panicked" | tail -40
  run fp8_d3 "$out" 2>&1 | grep FP8D3
  python3 - "$out" <<'PY'
import json, sys, os
d = sys.argv[1]
c = json.load(open(os.path.join(d, "chain.json")))
d3 = json.load(open(os.path.join(d, "d3.json")))
fixed = all(f["fixed"] for f in c["fixed_heights"])
layers = {"app", "leaf", "node1", "node2", "node", "L", "L_Fr"}
g1 = all(c["heights"][k].get("g1") == "accepted" for k in layers)
gates = all(r[k]["gate"] == "PASS" for r in d3 for k in ("measured", "vk", "bounds") if k in r)
measured = all("measured" in r for r in d3)
if not (fixed and g1 and gates and measured):
    sys.exit(f"V.10 FAIL: fixed {fixed} g1 {g1} d3 {gates} measured-every-layer {measured}")
print(f"V.10 PASS: {c['shards']} shards, {c['proved_steps']} steps, fixed heights; layers {sorted(layers)} G1 accepted; D3 PASS on {len(d3)} keys at measured shapes")
PY
else
  echo "usage: check-chain.sh omit|chain"; exit 2
fi
