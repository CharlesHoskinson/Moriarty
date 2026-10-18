# K5 operator, step K5g: host gate on the setup record, the K5M stdout summary and the proving-key file. PROPOSED SOURCE; never executed by its author.
# Run exactly once, after K5M:
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/logs/K5g.exit (absent = failed).
umask 022
cd /
export LC_ALL=C
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export M5="$BASE/current-cardano-k2-k5-setup-operator-materialized-20261016"
export OWN="$M5/operator/request"
export RR5="$BASE/current-cardano-k2-k5-root-records-20261016"
export M="$BASE/current-cardano-k2-k4h-hostile-materialized-20261016"
export C="$BASE/current-cardano-k2-g1-materialized-candidate-20261015"
export RR="$BASE/current-cardano-k2-ordinary-root-records-20261015"
export TOLD="$BASE/current-cardano-k2-ordinary-local-execution-20261015"
export TK="$BASE/current-cardano-k2-k4h-hostile-local-execution-20261016"
export FREEZE="$BASE/claim-utxo-abi-v1-aiken-placement-candidate-20261015/tools/freeze.py"
export T="$BASE/current-cardano-k2-k5-single-party-test-setup-20261016"
export W="$T/w"
run_step() { nice -n 10 timeout --kill-after=60 "$3" prlimit --as=8589934592 --cpu="$5" --core=0 -- \
  bash -o pipefail -c "{ $4
} 2>&1 | tee \"$1/$2.log\""; echo $? > "$1/$2.exit"; }
# Host-side link-safety guards (clause K-7; identical text wherever they appear). Only ever called after every sandbox
# of the step has exited. dirs_ok: each argument is a directory and not a symbolic link. tree_clean: find (which never
# follows links) lists every argument subtree completely (exit 0) and reports no entry other than regular files and
# directories.
dirs_ok() { for d in "$@"; do { test ! -L "$d" && test -d "$d"; } || { echo "BLOCKED(hygiene): $d is a link or not a directory"; return 1; }; done; }
tree_clean() { ODD="$(find "$@" ! -type f ! -type d -print -quit)" || { echo "BLOCKED(hygiene): find could not list $* completely"; return 1; }; test -z "$ODD" || { echo "BLOCKED(hygiene): non-regular entry $ODD"; return 1; }; }
# K5 pins and gates (R-K4H-01; identical text in every logged K5 step). pins_ok: the operator bytes against the
# root's RR5 list, every pinned public input, and after P0 the root records and K4H receipts that P0 verified.
# bin_ok: w/bin holds only the release mori-k2 with the KH2 hash. res_ok: the K5-4M disk and MemAvailable gates.
pins_ok() { { test ! -L "$RR5/K5-OPERATOR-PINS.sha256" && sha256sum -c --strict --quiet "$RR5/K5-OPERATOR-PINS.sha256"; } ||
  { echo "BLOCKED(identity): operator files differ from RR5/K5-OPERATOR-PINS.sha256"; return 1; }
  sha256sum -c --strict --quiet "$OWN/data/k5-abs.sha256" || { echo "BLOCKED(identity): a pinned public input differs from data/k5-abs.sha256"; return 1; }
  if test -e "$T/receipts/P0-root-pins.sha256"; then sha256sum -c --strict --quiet "$T/receipts/P0-root-pins.sha256" ||
  { echo "BLOCKED(identity): a root record or K4H receipt changed after P0"; return 1; }; fi; }
bin_ok() { dirs_ok "$W/bin" && test "$(ls -A "$W/bin")" = mori-k2 && test ! -L "$W/bin/mori-k2" && test -f "$W/bin/mori-k2" &&
  ( cd "$W/bin" && sha256sum -c --strict --quiet "$T/receipts/P0-bin.sha256" ) ||
  { echo "BLOCKED(identity): w/bin is not exactly the P0-verified release mori-k2"; return 1; }; }
res_ok() { FREE="$(df --output=avail -B1 "$T" | tail -n 1)" && echo "record: free bytes at T5 = $FREE" &&
  { test "$FREE" -ge 6442450944 || { echo "BLOCKED(resource): free disk below the 6 GiB gate"; return 1; }; } &&
  MA="$(awk "/^MemAvailable:/{print \$2}" /proc/meminfo)" && echo "record: MemAvailable kB = $MA" &&
  { test "$MA" -ge 10485760 || { echo "BLOCKED(resource): MemAvailable below the 10 GiB gate"; return 1; }; }; }
export -f dirs_ok tree_clean pins_ok bin_ok res_ok
ID=K5g
{ test ! -L "$T" && test -d "$T" && test ! -L "$T/logs" && test -d "$T/logs"; } || { echo "REFUSED: $T/logs is absent or a link; P1 did not create T5"; exit 0; }
once() { if test -e "$T/logs/$ID.log" || test -e "$T/logs/$ID.exit" || test -e "$T/logs/$ID.skipped"; then echo "REFUSED: step $ID already ran or was skipped; one attempt only"; exit 0; fi; }
need() { for d in "$@"; do case "$d" in Z?2) f="$T/$d.exit" ;; *) f="$T/logs/$d.exit" ;; esac; test "$(cat "$f" 2>/dev/null)" = 0 || { echo "SKIPPED: prerequisite $d not 0" > "$T/logs/$ID.skipped"; exit 0; }; done; }
once
need P1 P0 Q5 K5a K5b K5M
J="$T/journal/$ID.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# K5g (clauses K5-8, K5-8A; custody DV-K2-1-R): host /usr/bin/python3 -I with the strict gate data/k5g_gate.py over
# the host-written K5M summary and binary receipt, the root-pinned stage-C descriptor, lock pin and driver source
# texts, and, behind its lstat plus O_NOFOLLOW guards, the one record in w/out and the one key file in w/keys.
# The gate hashes the key in chunks and records only its path, mode, size, link count and SHA-256; key and VK
# bytes never enter a receipt. Verdict PASS, STOP-CUSTODY (BLOCKED(hygiene)), STOP-IDENTITY (BLOCKED(identity))
# or FAIL-PREDICTION (FAILED(prediction)); an interpreter fault with no verdict is BLOCKED(environment). The
# gate performs no pairing, subgroup, proof or verify work: that is K6 scope. Even on an abnormal K5M exit this
# step's need() chain means it is skipped, and the leftover, if any, is still recorded by H5 and Z51 metadata;
# it is never reused.
run_step "$T/logs" "$ID" 600 'pins_ok && bin_ok && dirs_ok "$W/out" "$W/keys" &&
/usr/bin/python3 -I "$OWN/data/k5g_gate.py" "$W/out" "$W/keys" "$T/receipts/K5M-summary.json" "$TOLD/receipts/K4b-descriptor.json" "$T/receipts/P0-bin.sha256" "$RR/K2-LOCK-PIN.sha256" "$M/patches/k2/src/lib.rs" "$C/k2/src/export.rs" > "$T/receipts/K5g.json"; RC=$?;
cat "$T/receipts/K5g.json"; echo "record: k5g_gate.py exit $RC";
test "$RC" -eq 0 && grep -qF "\"verdict\": \"PASS\"" "$T/receipts/K5g.json"' 300
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
