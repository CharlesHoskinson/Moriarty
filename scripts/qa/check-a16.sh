#!/usr/bin/env bash
# A16: node step at the recorded k (task 2.6; SRC-015(a), SRC-011).
# Checks the `A16` record in qa/sherlock/decisions/speed-levers.json (gate G-A) against
# qa/sherlock/runs.jsonl:
#   - the node-step run: profile-gp node-only, one stand-in step, RAYON_NUM_THREADS equal to the
#     G-A thread count, exit 0, within the fixed node-k19-step cap (300 s, 24 GiB);
#   - the fold run: profile-gp with 3 stand-in steps and the run decider, exit 0, within
#     a4-terminal-d;
#   - the verdict: "pass" only if the node step prove wall <= 60 s, the process peak RSS
#     <= 16 GB (16e9 bytes) and k == 19; otherwise "shortfall" with a C0 phase cause.
# The predicate passes on "pass" or on a recorded shortfall (SRC-015(a)); the output says which.
# A16_RUN=1 reruns both measurements under scripts/doctorwatson heavy (needs A16_BIN, the
# off profile-gp binary, and A16_OUT, a new sidecar directory) and prints the run ids; the
# record is then updated by hand from the sidecars.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
if [ "${A16_RUN:-0}" = 1 ]; then
  bin="${A16_BIN:?A16_BIN: the off profile-gp binary}"
  out="${A16_OUT:?A16_OUT: a new sidecar directory}"
  mkdir -p "$out"
  scripts/doctorwatson heavy --id A16-node-step --budget node-k19-step \
    --hypothesis 'A16 rerun: one SHKIVC06 node step, node-only, 20 threads' -- \
    env RAYON_NUM_THREADS=20 MORIARTY_GP_STANDIN_STEPS=1 MORIARTY_GP_NODE_ONLY=1 \
    "$bin" --workload GP --cache-mode resident_pk --sidecar "$out/node-step.json" --threads 20
  scripts/doctorwatson heavy --id A16-fold3-decider --budget a4-terminal-d \
    --hypothesis 'A16 rerun: 3-step fold and run decider, 20 threads' -- \
    env RAYON_NUM_THREADS=20 MORIARTY_GP_STANDIN_STEPS=3 \
    "$bin" --workload GP --cache-mode resident_pk --sidecar "$out/fold3-decider.json" --threads 20
fi
python3 - <<'PY'
import json, sys
levers = json.load(open("qa/sherlock/decisions/speed-levers.json"))
budgets = json.load(open("qa/sherlock/budgets.json"))["budgets"]
ga = levers["gates"]["G-A"]
rec = ga.get("A16")
if not isinstance(rec, dict): sys.exit("A16: speed-levers.json gates.G-A has no A16 record")
threads = ga["decisions"]["rayon_threads"]
runs = {}
for line in open("qa/sherlock/runs.jsonl"):
    line = line.strip()
    if line:
        r = json.loads(line)
        runs[r.get("run")] = r
def run_of(part, budget, env):
    rid = part.get("run")
    r = runs.get(rid)
    if r is None: sys.exit(f"A16: run {rid!r} not in runs.jsonl")
    if r.get("exit") != 0: sys.exit(f"A16: run {rid} exit {r.get('exit')}")
    for token in env + [f"RAYON_NUM_THREADS={threads}", "profile-gp"]:
        if token not in r.get("cmd", ""): sys.exit(f"A16: run {rid} command lacks {token}")
    cap = budgets[budget]["cap"]
    peak = max(r.get("maxRssBytes") or 0, r.get("cgroupPeakBytes") or 0)
    if r["wallSeconds"] > cap["wallSeconds"] or peak > cap["maxRssBytes"]:
        sys.exit(f"A16: run {rid} exceeds {budget}")
    if part.get("process_wall_seconds") != r["wallSeconds"] or part.get("process_peak_rss_bytes") != peak:
        sys.exit(f"A16: record for {rid} does not match runs.jsonl ({r['wallSeconds']} s, {peak} B)")
    return r
step = rec["node_step"]
run_of(step, "node-k19-step", ["MORIARTY_GP_NODE_ONLY=1", "MORIARTY_GP_STANDIN_STEPS=1"])
if not budgets["node-k19-step"].get("fixed"): sys.exit("A16: node-k19-step cap is not fixed")
fold = rec["fold3_decider"]
run_of(fold, "a4-terminal-d", ["MORIARTY_GP_STANDIN_STEPS=3"])
if "MORIARTY_GP_NODE_ONLY=1" in runs[fold["run"]]["cmd"]: sys.exit("A16: fold run is node-only")
if len(fold.get("node_prove_wall_seconds") or []) != 3 or not fold.get("decider_prove_wall_seconds"):
    sys.exit("A16: fold record needs 3 node prove walls and the decider prove wall")
bar = rec["bar"]
if bar != {"prove_wall_seconds": 60, "peak_rss_bytes": 16_000_000_000, "k": 19}:
    sys.exit(f"A16: bar {bar} is not SRC-015(a)")
wall, peak, k = step["prove_wall_seconds"], step["process_peak_rss_bytes"], step["k"]
misses = [m for m, bad in (("wall", wall > bar["prove_wall_seconds"]),
                           ("rss", peak > bar["peak_rss_bytes"]), ("k", k != bar["k"])) if bad]
want = "shortfall" if misses else "pass"
if rec.get("verdict") != want: sys.exit(f"A16: verdict {rec.get('verdict')!r}, measurements give {want!r}")
print(f"A16: node step {wall} s prove, peak {peak / 1e9:.2f} GB, k={k} at {threads} threads ({step['run']})")
print(f"A16: 3-step fold {fold['node_prove_wall_seconds']} s, decider {fold['decider_prove_wall_seconds']} s, "
      f"peak {fold['process_peak_rss_bytes'] / 1e9:.2f} GB ({fold['run']})")
if misses:
    cause = rec.get("cause") or ""
    if "C0" not in cause: sys.exit("A16: shortfall without a C0 phase cause")
    print(f"A16 PASS (shortfall recorded for G-A: {', '.join(misses)} over SRC-015(a)); S carries the target")
else:
    print("A16 PASS: SRC-015(a) bar met")
PY
