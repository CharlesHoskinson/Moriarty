#!/usr/bin/env bash
# F23: GPU phase G0 toolchain smoke on sm_121 (task 2.10a, gpu.md section 0 and section 6 G0, SRC-018).
# Builds tools/gpu-smoke (its own workspace; build.rs runs nvcc into a static lib) and runs it:
# nvcc release, driver and device recorded; 65,536 8 x 32-bit-limb products on plain Vec<u32>
# memory with 0 mismatches against a CPU u128 reference; cudaMalloc and cudaMallocManaged of
# 1 GiB each raise the process RSS by >= 0.9 GiB.
# Exit 0 = pass, 1 = fail, 2 = skip because nvcc or a CUDA device is absent (never a vacuous pass).
# Build and run each go in a systemd user scope (MemoryMax=16G, no swap) under gpu.lock.
# The receipt is written to target/gpu-smoke/f23-receipt.json.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
crate=tools/gpu-smoke
cuda=${CUDA_PATH:-/opt/cuda}
nvcc=$cuda/bin/nvcc
[ -f "$crate/Cargo.toml" ] || { echo "F23: $crate missing"; exit 1; }
[ -x "$nvcc" ] || { echo "F23 SKIP: $nvcc absent"; exit 2; }
command -v nvidia-smi >/dev/null || { echo "F23 SKIP: nvidia-smi absent"; exit 2; }
smi=$(nvidia-smi --query-gpu=name,driver_version --format=csv,noheader 2>/dev/null | head -1) || smi=
[ -n "$smi" ] || { echo "F23 SKIP: no CUDA device visible to nvidia-smi"; exit 2; }
echo "F23: $("$nvcc" --version | grep release)"
echo "F23: nvidia-smi: $smi"

target=$PWD/target/gpu-smoke
mkdir -p "$target"
receipt=$target/f23-receipt.json
rm -f "$receipt"

run_capped() {
  if command -v systemd-run >/dev/null && systemctl --user show-environment >/dev/null 2>&1; then
    systemd-run --user --scope --collect -q --expand-environment=no \
      -p MemoryMax=16G -p MemorySwapMax=0 -- "$@"
  else
    echo "F23: systemd-run unavailable, running without a cgroup cap" >&2
    "$@"
  fi
}

lockdir=$HOME/.cache/doctorwatson
mkdir -p "$lockdir"
exec 9>"$lockdir/gpu.lock"
flock -w "${F23_LOCK_WAIT:-900}" 9 || { echo "F23: gpu.lock busy"; exit 1; }

run_capped env CUDA_PATH="$cuda" CARGO_TARGET_DIR="$target" \
  cargo build --release --locked --quiet --manifest-path "$crate/Cargo.toml" \
  || { echo "F23 FAIL: build failed"; exit 1; }

set +e
run_capped "$target/release/mori-gpu-smoke" "$receipt"
rc=$?
set -e
case $rc in
  0) ;;
  2) echo "F23 SKIP: the smoke saw no CUDA device"; exit 2 ;;
  *) echo "F23 FAIL: smoke exit $rc"; exit 1 ;;
esac

SMI="$smi" python3 - "$receipt" <<'PY'
import json, os, sys
path = sys.argv[1]
r = json.load(open(path))
name, _, driver = os.environ["SMI"].rpartition(", ")
r["nvidiaSmi"] = {"name": name, "driver": driver}
gib = 1 << 30
ok = (r.get("pass") is True and r.get("mismatches") == 0 and r.get("products", 0) > 0
      and r.get("cudaMallocRssDelta", 0) >= 0.9 * gib
      and r.get("cudaMallocManagedRssDelta", 0) >= 0.9 * gib)
json.dump(r, open(path, "w"), indent=2)
open(path, "a").write("\n")
if not ok:
    sys.exit(f"F23 FAIL: receipt does not meet the predicate: {r}")
print(f"F23 PASS: {r['device']} cc {r['computeCapability']} ({r['arch']}), {r['nvcc']}, driver {driver}; "
      f"{r['products']} products, {r['mismatches']} mismatches; RSS +{r['cudaMallocRssDelta'] / gib:.2f} GiB "
      f"(cudaMalloc), +{r['cudaMallocManagedRssDelta'] / gib:.2f} GiB (cudaMallocManaged); receipt {path}")
PY
