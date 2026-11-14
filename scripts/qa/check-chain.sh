#!/usr/bin/env bash
# V.9 and V.10 (sherlock-modular-leaf-20261111) on the T7 openvm tree (t7-bind): the slice leaf
# shards through leaf, N1, N2, N3, L and L_Fr on one universal key set. Usage: check-chain.sh omit|chain|cases
#   omit:  `t7_chain omit` (V.9 slice_air_omitted and the omit cases of N1, N3 and L)
#   chain: `t7_chain chain A`, `t7_chain chain B`, then `fp8_d3` on each (V.10: D3 >= 100 on every term)
#   cases: `t7_chain cases` (T7 binding cases: wrong prog_d, mixed chip sets, the FV unit under N3; T10: a changed
#          leaf post_d and an h_hi limb of 2^16 or more)
set -euo pipefail
mode="$1"
ovm="${SHERLOCK_SDK_ROOT:-$HOME/.cache/m7-wrap/src/openvm-t7}"
export RUSTUP_AUTO_INSTALL=0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cache/t7/target-ovm}"
export SHERLOCK_SDK_ROOT="$ovm"
export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-8}"
out="${T7_CHAIN_OUT:-$HOME/.cache/t7/check-$mode}"
mkdir -p "$out"
if [ -n "$(git -C "$ovm" status --porcelain -- crates Cargo.toml Cargo.lock)" ]; then
  echo "$mode FAIL: uncommitted changes in $ovm"; exit 1
fi
echo "openvm tree $ovm at $(git -C "$ovm" rev-parse HEAD); out $out"
run() { ( cd "$ovm" && "$HOME/.local/bin/heavy-slot" ${T7_EXCLUSIVE:+--exclusive} --mem 64G --name "t7-check-$1" -- \
  cargo run --release --offline -p sherlock-wrap-export --example "$@" ); }
nonempty() { for f in "$@"; do [ -s "$f" ] || { echo "$mode FAIL: $f is missing or empty"; exit 1; }; done; }
if [ "$mode" = omit ]; then
  rm -f "$out/omit.json"
  run t7_chain omit "$out" 2>&1 | grep -E "SLICEOMIT|T7CASE|panicked|error" | tail -80
  nonempty "$out/omit.json"
  python3 - "$out/omit.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
bad = [c["case"] for c in d["cases"] if not c["pass"]]
req = d["all_required"]
names = [c["case"] for c in d["cases"]]
need = ["slice_air_omitted_stock_verify_without_", "n1_parent_over_child_without_", "n3_parent_over_child_without_", "l_stock_verify_without_", "lfr_tape_honest_shape_without_"]
if bad or not all(req.values()) or not all(any(n.startswith(p) for n in names) for p in need):
    sys.exit(f"V.9 FAIL: failing {bad}, all_required {req}")
s = d["app_shape"]
print(f"V.9 PASS: {len(names)} cases; every AIR required in {sorted(req)}; app shape {s['airs']} AIRs, {s['columns']} columns, max interactions {s['max_interactions']}")
PY
elif [ "$mode" = chain ]; then
  for prog in A B; do
    o="$out/chain-$prog"; mkdir -p "$o"; rm -f "$o/chain.json" "$o/d3.json"
    echo "host load before chain $prog (DG-006): $(cat /proc/loadavg)"
    run t7_chain chain "$o" "$prog" 2>&1 | grep -E "T7CHAIN|T7CASE|T7KEY|panicked" | tail -40
    run fp8_d3 "$o" "$o/fv_vk.bitcode" 2>&1 | grep FP8D3
    nonempty "$o/chain.json" "$o/cases.json" "$o/d3.json"
  done
  python3 - "$out" <<'PY'
import json, sys, os
out = sys.argv[1]
seen = {}
for prog in "AB":
    d = os.path.join(out, f"chain-{prog}")
    c = json.load(open(os.path.join(d, "chain.json")))
    cs = json.load(open(os.path.join(d, "cases.json")))
    d3 = json.load(open(os.path.join(d, "d3.json")))
    fixed = all(f["fixed"] for f in c["fixed_heights"])
    layers = {"app", "leaf", "node1", "node2", "node", "L", "L_Fr"}
    g1 = all(c["heights"][k].get("g1") == "accepted" for k in layers)
    gates = all(r[k]["gate"] == "PASS" for r in d3 for k in ("measured", "vk", "bounds") if k in r and r[k] is not None)
    measured = sum(1 for r in d3 if r.get("measured")) >= 7
    cases = cs["all_pass"] and c["cases_all_pass"]
    # the internal-recursive key is at its fixed point: N3 over N3 has the N3 pre-hash
    fp = any(k.get("n3_over_n3_pre_equals_n3_pre") is True for k in c["keys"])
    if not (fixed and g1 and gates and measured and cases and fp):
        sys.exit(f"V.10 FAIL {prog}: fixed {fixed} g1 {g1} d3 {gates} measured {measured} cases {cases} n3 fixed point {fp}")
    seen[prog] = (c["prog_d"], c["key_root"], c["keys_d"])
    print(f"V.10 {prog} PASS: {c['shards']} shards, {c['proved_steps']} steps, fixed heights; G1 accepted; "
          f"D3 PASS on {len(d3)} keys; {len(cs['cases'])} cases; N3 fixed point; L_Fr words {c['lfr_public_words']}")
if seen["A"][0] == seen["B"][0] or seen["A"][1:] != seen["B"][1:]:
    sys.exit(f"V.10 FAIL: programs must differ in prog_d and share key_root and keys_d: {seen}")
print("V.10 PASS: two programs, one key_root and one keys_d")
PY
elif [ "$mode" = cases ]; then
  rm -f "$out/cases.json"
  run t7_chain cases "$out" 2>&1 | grep -E "T7CASE|T7KEY|panicked" | tail -60
  nonempty "$out/cases.json"
  python3 - "$out/cases.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
names = [c["case"] for c in d["cases"]]
need = ["mixed_chip_sets_N3_accepts", "wrong_prog_d_merge_refused_at_N1", "key_triple_outside_key_root_refused_at_N1",
        "fv_unit_under_N3_accepted", "fv_row_missing_refused_at_N3",
        "leaf_post_d_changed_refused_at_leaf", "leaf_post_d_changed_in_trace_refused_at_leaf",
        "h_hi_limb_over_16_bits_refused_at_N1"]
bad = [c["case"] for c in d["cases"] if not c["pass"]]
miss = [n for n in need if n not in names]
if bad or miss or not d["all_pass"]:
    sys.exit(f"T7 cases FAIL: failing {bad}, missing {miss}")
print(f"T7 cases PASS: {len(names)} cases")
PY
else
  echo "usage: check-chain.sh omit|chain|cases"; exit 2
fi
