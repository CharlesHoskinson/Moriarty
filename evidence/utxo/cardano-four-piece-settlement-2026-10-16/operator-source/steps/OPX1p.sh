# Four-piece VS harness-repair local measurement, step OPX1p (bounded placement). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX1p.exit (absent = failed; 200 = not started, aggregate
# budget; 201 = not started, gate).
umask 022
cd /
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export OWN="$BASE/current-cardano-four-way-vs-harness-repair-operator-materialized-20261016"
export PINSTORE="$BASE/current-cardano-four-way-vs-harness-repair-root-records-20261016"
export M="$BASE/current-cardano-four-way-vs-harness-repair-materialized-20261016"
export PT="$BASE/current-cardano-sharded-settlement-parser-repair-local-measurement-20261016"
export R1T="$BASE/current-cardano-p3-r1-aiken-local-execution-20261015"
export PIN="$BASE/aiken-stdlib-v220-source-20261015"
export DATAONLY="$BASE/aiken-fixed-compiler-release-discovery-20261015/v1.1.24-data-only"
export T="$BASE/current-cardano-four-way-vs-harness-repair-local-measurement-20261016"
export AIKEN="$T/bin/aiken"
export PY=/usr/bin/python3.14
# Aggregate cap: 1,800 s over the summed wall time of all step processes; OPX0 is charged 70 s and OPX1 5 s.
B_TOTAL=1800
B_FIXED=75
run_step() { # LOGDIR ID CAP KILL RESERVE CMD
  if test -f "$T/journal/elapsed.txt"; then E=$(awk '{s+=$2} END{print s+0}' "$T/journal/elapsed.txt"); else E=100000; fi
  case "$E" in ''|*[!0-9]*) E=100000;; esac
  U=$(( B_FIXED + E ))
  W=$(( B_TOTAL - U - $5 - $4 ))
  if test "$W" -gt "$3"; then W=$3; fi
  S0=$(date +%s)
  if test "$W" -lt 5; then
    echo "BUDGET: aggregate $B_TOTAL s, used $U s, reserve $5 s; $2 not started" | tee "$1/$2.log"; echo 200 > "$1/$2.exit"
  else
    printf 'used %s wall %s kill-after %s reserve %s\n' "$U" "$W" "$4" "$5" > "$T/journal/$2.budget"
    nice -n 10 timeout --kill-after="$4" "$W" prlimit --as=34359738368 --core=0 -- \
      bash -o pipefail -c "{ $6
} 2>&1 | tee \"$1/$2.log\""; echo $? > "$1/$2.exit"
  fi
  echo "$2 $(( $(date +%s) - S0 + 1 ))" >> "$T/journal/elapsed.txt"
}
skip_step() { # LOGDIR ID REASON
  echo "GATE: $3; $2 not started" | tee "$1/$2.log"; echo 201 > "$1/$2.exit"
  echo "$2 1" >> "$T/journal/elapsed.txt"
}
ok() { for s in "$@"; do test "$(cat "$T/logs/$s.exit" 2>/dev/null)" = 0 || return 1; done; }
J="$T/journal/OPX1p.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX1p (cap 120 s, kill-after 10 s, reserve 136 s). Placement from the ONE materialized candidate root M2 (its 32 package
# files are the full pin of ROOT-SOURCE-MANIFEST 54e0fc56...: parent 40 rows byte-identical plus the single corrected
# shard4_harness.ak constant vs2_t 0xd2 repeated 28):
#  (1) re-check the source package: 32 hashes; 32 files, sub-directories exactly lib, lib/pvc2, validators (3; 4 counting the
#      root); 0 other entries; validators exactly c2 s2p vi vs1 vs2 vs3 vs4; vs.ak, s2.ak, build, aiken.lock, plutus.json absent;
#  (2) 32 literal per-file copies into a fresh run/ with pre-created lib, lib/pvc2 and validators; re-check in run/;
#  (3) stdlib v2.2.0 offline: plain recursive byte copy of the 38 pinned files plus the exact 77-byte packages.toml;
#  (4) tree arithmetic 32 + 38 + 1 = 71 files, 3 + 3 + 13 = 19 directories below run/, 0 other entries, by coreutils and
#      again by tools/tree_check.py (no generated file allowed);
#  (5) digest record of the run tree before any aiken call.
if ok OPX1; then
run_step "$T/logs" OPX1p 120 10 136 'cd "$M/candidate/aiken" && sha256sum -c "$OWN/data/candidate-32.sha256" &&
test "$(find . -type f | wc -l)" -eq 32 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 3 && test "$(find . -type d | wc -l)" -eq 4 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 &&
test "$(find . -mindepth 1 -type d | LC_ALL=C sort | tr "\n" " ")" = "./lib ./lib/pvc2 ./validators " &&
test "$(ls -A validators | LC_ALL=C sort | tr "\n" " ")" = "c2.ak s2p.ak vi.ak vs1.ak vs2.ak vs3.ak vs4.ak " &&
test ! -e validators/vs.ak && test ! -e validators/s2.ak && test ! -e build && test ! -e aiken.lock && test ! -e plutus.json &&
test ! -e "$T/run" && mkdir "$T/run" "$T/run/lib" "$T/run/lib/pvc2" "$T/run/validators" &&
cp -- aiken.toml "$T/run/aiken.toml" &&
cp -- validators/c2.ak "$T/run/validators/c2.ak" &&
cp -- validators/s2p.ak "$T/run/validators/s2p.ak" &&
cp -- validators/vi.ak "$T/run/validators/vi.ak" &&
cp -- validators/vs1.ak "$T/run/validators/vs1.ak" &&
cp -- validators/vs2.ak "$T/run/validators/vs2.ak" &&
cp -- validators/vs3.ak "$T/run/validators/vs3.ak" &&
cp -- validators/vs4.ak "$T/run/validators/vs4.ak" &&
cp -- lib/pvc2/bytes.ak "$T/run/lib/pvc2/bytes.ak" &&
cp -- lib/pvc2/catalogue.ak "$T/run/lib/pvc2/catalogue.ak" &&
cp -- lib/pvc2/claim.ak "$T/run/lib/pvc2/claim.ak" &&
cp -- lib/pvc2/core.ak "$T/run/lib/pvc2/core.ak" &&
cp -- lib/pvc2/diag.ak "$T/run/lib/pvc2/diag.ak" &&
cp -- lib/pvc2/fixtures_r1.ak "$T/run/lib/pvc2/fixtures_r1.ak" &&
cp -- lib/pvc2/groth16.ak "$T/run/lib/pvc2/groth16.ak" &&
cp -- lib/pvc2/intent.ak "$T/run/lib/pvc2/intent.ak" &&
cp -- lib/pvc2/settle_r1_test.ak "$T/run/lib/pvc2/settle_r1_test.ak" &&
cp -- lib/pvc2/shard.ak "$T/run/lib/pvc2/shard.ak" &&
cp -- lib/pvc2/shard4.ak "$T/run/lib/pvc2/shard4.ak" &&
cp -- lib/pvc2/shard4_cost_test.ak "$T/run/lib/pvc2/shard4_cost_test.ak" &&
cp -- lib/pvc2/shard4_harness.ak "$T/run/lib/pvc2/shard4_harness.ak" &&
cp -- lib/pvc2/shard4_test.ak "$T/run/lib/pvc2/shard4_test.ak" &&
cp -- lib/pvc2/shard_cost_test.ak "$T/run/lib/pvc2/shard_cost_test.ak" &&
cp -- lib/pvc2/shard_harness.ak "$T/run/lib/pvc2/shard_harness.ak" &&
cp -- lib/pvc2/shard_test.ak "$T/run/lib/pvc2/shard_test.ak" &&
cp -- lib/pvc2/test_ctx.ak "$T/run/lib/pvc2/test_ctx.ak" &&
cp -- lib/pvc2/thread.ak "$T/run/lib/pvc2/thread.ak" &&
cp -- lib/pvc2/types.ak "$T/run/lib/pvc2/types.ak" &&
cp -- lib/pvc2/unit_r1_test.ak "$T/run/lib/pvc2/unit_r1_test.ak" &&
cp -- lib/pvc2/unit_test.ak "$T/run/lib/pvc2/unit_test.ak" &&
cp -- lib/pvc2/w0_cost_test.ak "$T/run/lib/pvc2/w0_cost_test.ak" &&
cp -- lib/pvc2/w1_gate_test.ak "$T/run/lib/pvc2/w1_gate_test.ak" &&
cd "$T/run" && sha256sum -c "$OWN/data/candidate-32.sha256" &&
test "$(find . -type f | wc -l)" -eq 32 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 3 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 &&
test "$(ls -A validators | LC_ALL=C sort | tr "\n" " ")" = "c2.ak s2p.ak vi.ak vs1.ak vs2.ak vs3.ak vs4.ak " &&
mkdir build build/packages && cp -R "$PIN/source" build/packages/aiken-lang-stdlib &&
printf "%s\n" "[[packages]]" "name = \"aiken-lang/stdlib\"" "version = \"v2.2.0\"" "source = \"github\"" > build/packages/packages.toml &&
printf "%s  %s\n" 42ca23893e5c0f43598c15a1c699fa2bcbb451999c054351f67dae174d49ffb6 build/packages/packages.toml | sha256sum -c - &&
test "$(ls -A build/packages | tr "\n" " ")" = "aiken-lang-stdlib packages.toml " &&
( cd build/packages/aiken-lang-stdlib && sha256sum -c "$OWN/data/stdlib-38.sha256" && test "$(find . -type f | wc -l)" -eq 38 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 13 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 ) &&
test "$(find . -type f | wc -l)" -eq 71 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 19 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 &&
test ! -e aiken.lock && test ! -e plutus.json && ls -A &&
cd / && "$PY" -I "$OWN/tools/tree_check.py" "$T/run" "$OWN/data" "$T/receipts/tree-placed.json" placed --forbid-generated &&
"$PY" -I "$OWN/tools/freeze.py" "$T/receipts/run-before-check.json" "run tree before any aiken call (71 files)" "$T" run'
else skip_step "$T/logs" OPX1p "OPX1 not 0"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
