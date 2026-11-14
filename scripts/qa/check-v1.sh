#!/usr/bin/env bash
# V.1 (sherlock-modular-leaf-20261111): the slice program `entry x;` and its rule subset on the
# landed reference owner machine. Re-runs the probe test and checks the recorded rule list.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export RUSTUP_AUTO_INSTALL=0
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$HOME/.cache/t6/target-mor}"
out="${SLICE_OUT:-$HOME/.cache/t6/out}"
mkdir -p "$out"
export SLICE_OUT="$out"
"$HOME/.local/bin/heavy-slot" --mem 24G --name t6-v1 -- \
  cargo test --locked --offline -p mori-core --lib slice_v1_candidates -- --nocapture --test-threads=1
python3 - "$out/slice_v1.json" <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
want = ["BHelperEnvGetEntry", "BHelperEnvGetEmpty", "BHelperEnvGetFailure",
        "KMachineEvalVariableAbsent", "KAdminPublishError"]
run = next(r for r in d["runs"] if r["name"] == "unbound")
got = [s["rule"] for s in run["steps"]]
if got != want:
    sys.exit(f"V.1 FAIL: rule list {got} != {want}")
if run["outcome"] != "Rejected { code: 3 }":
    sys.exit(f"V.1 FAIL: outcome {run['outcome']}")
if any(s["new_data_rows"] for s in run["steps"]):
    sys.exit("V.1 FAIL: the slice run appends a record")
print("V.1 PASS: `entry x;` rules", got, "outcome", run["outcome"])
PY
