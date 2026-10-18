# Four-piece VS harness-repair local measurement, step OPX7 (final integrity). PROPOSED SOURCE; never executed by its author.
# Run exactly once, ALWAYS after OPX1 succeeded (also after any failure of OPX1t..OPX2x), only after a fresh source/effect
# review of these exact bytes, a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX7.exit (absent = failed; 200 = not started, aggregate budget).
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
ok() { for s in "$@"; do test "$(cat "$T/logs/$s.exit" 2>/dev/null)" = 0 || return 1; done; }
J="$T/journal/OPX7.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX7 (cap 90 s, kill-after 10 s, reserve 36 s for OPX7f: its 35 s plus 1 s of elapsed rounding). Effects first (all a2
# blueprints read-only; tmp listing; final tree check and digest records of run and a2), then read-only integrity, whose
# status is the step result: binary (if staged) sha and 0555; empty dirs empty; unapplied blueprint (if built) equal to its
# recorded sha in a2 and run; tools/preflight.py post (pinstore, O'' manifest and verbatim originals, M manifest and its
# originals in C' and the author directory, candidate tree 32 / 3 / 0, EXPECTATIONS against the test declarations,
# parent-evidence and R1-control pins); coreutils re-check of M documents and package; stdlib pin; DATA-ONLY binary 0644;
# T's top-level entries exactly the expected set. No GREEN is possible unless this step exits 0.
FL=""; if ok OPX2a; then FL="--require-lock"; fi
if ok OPX3; then FL="$FL --plutus $T/receipts/plutus-unapplied-sha256.txt $T/a2/plutus.unapplied.json"; fi
export FL
run_step "$T/logs" OPX7 90 10 36 'EF=0;
{ find "$T/a2" -maxdepth 1 -type f -name "*.json" -exec chmod 0444 {} + &&
  find "$T/tmp" -mindepth 1 | tee "$T/receipts/tmp-final.txt" &&
  if test -d "$T/run"; then "$PY" -I "$OWN/tools/tree_check.py" "$T/run" "$OWN/data" "$T/receipts/tree-final.json" final $FL; "$PY" -I "$OWN/tools/freeze.py" "$T/receipts/run-final.json" "run tree at OPX7" "$T" run; else echo "no run tree"; fi &&
  "$PY" -I "$OWN/tools/freeze.py" "$T/receipts/a2-freeze.json" "unapplied, intermediate and applied blueprints" "$T" a2; } || EF=1;
if test -f "$T/receipts/tree-final.json" && ! grep -q "\"ok\": true" "$T/receipts/tree-final.json"; then EF=1; echo "final tree check failed"; fi;
cd / && if test -e "$AIKEN"; then printf "%s  %s\n" 4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677 "$AIKEN" | sha256sum -c - && test "$(stat -c %a "$AIKEN")" = 555; else echo "binary not staged"; fi &&
for d in empty-home empty-xdg-cache empty-xdg-config empty-xdg-data probe-cwd; do test -d "$T/$d" && test -z "$(find "$T/$d" -mindepth 1 -print -quit)" || { echo "not empty or missing: $d"; exit 1; }; done &&
if test -f "$T/receipts/plutus-unapplied-sha256.txt"; then printf "%s  %s\n" "$(cut -d " " -f1 "$T/receipts/plutus-unapplied-sha256.txt")" "$T/a2/plutus.unapplied.json" | sha256sum -c - && ( cd "$T/run" && sha256sum -c "$T/receipts/plutus-unapplied-sha256.txt" ); else echo "no unapplied blueprint recorded"; fi &&
"$PY" -I "$OWN/tools/preflight.py" post "$OWN" "$PINSTORE/PRE-LAUNCH.json" "$M" "$T" "$PT" "$R1T" &&
cd "$M" && sha256sum -c "$OWN/data/m-docs-10.sha256" &&
cd "$M/candidate/aiken" && sha256sum -c "$OWN/data/candidate-32.sha256" &&
test "$(find . -type f | wc -l)" -eq 32 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 3 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 &&
test ! -e validators/vs.ak && test ! -e validators/s2.ak && test ! -e build && test ! -e aiken.lock && test ! -e plutus.json &&
cd "$PT" && sha256sum -c "$OWN/data/parent-evidence-9.sha256" &&
cd "$PIN" && printf "%s  %s\n" a26daa087fae05f9b35a06ed382d20e92e4d1ea8b76b59947134b7e4b2197266 SOURCE-PIN.json e44ed44783d17f4cb0b58c59f98a75bc09fc64c793dd25f3ab56b4097cdf1b1c SOURCE-MANIFEST.json 90c07de94ebf858f0d78292a9482cfa3b2b45767ed85fd381e74c0463cdf9b51 stdlib-source.tar.gz | sha256sum -c - &&
cd "$PIN/source" && sha256sum -c "$OWN/data/stdlib-38.sha256" && test "$(find . -type f | wc -l)" -eq 38 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 13 &&
cd "$DATAONLY" && printf "%s  %s\n" 4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677 compiler-data/aiken-x86_64-unknown-linux-musl/aiken 8adde843b7552bc7810f4be9cfca73693b8cc5f11fbb9e07d586901fd656d487 DATA-ONLY-MANIFEST.json | sha256sum -c - &&
test "$(stat -c %a compiler-data/aiken-x86_64-unknown-linux-musl/aiken)" = 644 &&
TOP="$(ls -A "$T" | LC_ALL=C sort | tr "\n" " ")" && echo "T top level: $TOP" &&
{ test "$TOP" = "a2 bin empty-home empty-xdg-cache empty-xdg-config empty-xdg-data help journal logs probe-cwd receipts run tmp " || test "$TOP" = "a2 bin empty-home empty-xdg-cache empty-xdg-config empty-xdg-data help journal logs probe-cwd receipts tmp "; } &&
test "$EF" = 0'
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
