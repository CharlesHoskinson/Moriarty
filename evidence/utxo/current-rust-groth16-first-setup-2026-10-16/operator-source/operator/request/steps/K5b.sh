# K5 operator, step K5b: release `mori-k2 compile` of reserve/allocate inside sbx_run (descriptor re-observation). PROPOSED SOURCE; never executed by its author.
# Run exactly once, after K5a, under the root's K5 authorization:
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/logs/K5b.exit (absent = failed).
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
# The K5 run view `sbx_run` (clause K5-2; identical text in Q5, K5a, K5b and K5M). No cargo, toolchain, registry,
# target, home or host path is mounted; /task/bin and /task/in are read-only; /task/out and /task/keys are the
# only writable binds; stdin is /dev/null.
sbx_run() { /usr/bin/bwrap --unshare-user --unshare-pid --unshare-net --unshare-ipc --unshare-uts \
  --die-with-parent --new-session \
  --ro-bind /usr /usr --ro-bind /bin /bin --ro-bind /lib /lib --ro-bind /lib64 /lib64 \
  --ro-bind-try /etc/ld.so.cache /etc/ld.so.cache \
  --dev /dev --proc /proc --size 268435456 --tmpfs /tmp \
  --ro-bind "$W/bin" /task/bin --ro-bind "$W/in" /task/in \
  --bind "$W/out" /task/out --bind "$W/keys" /task/keys \
  --chdir /task --clearenv \
  --setenv PATH /usr/bin:/bin --setenv HOME /nonexistent \
  -- "$@" < /dev/null; }
export -f sbx_run
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
ID=K5b
{ test ! -L "$T" && test -d "$T" && test ! -L "$T/logs" && test -d "$T/logs"; } || { echo "REFUSED: $T/logs is absent or a link; P1 did not create T5"; exit 0; }
once() { if test -e "$T/logs/$ID.log" || test -e "$T/logs/$ID.exit" || test -e "$T/logs/$ID.skipped"; then echo "REFUSED: step $ID already ran or was skipped; one attempt only"; exit 0; fi; }
need() { for d in "$@"; do case "$d" in Z?2) f="$T/$d.exit" ;; *) f="$T/logs/$d.exit" ;; esac; test "$(cat "$f" 2>/dev/null)" = 0 || { echo "SKIPPED: prerequisite $d not 0" > "$T/logs/$ID.skipped"; exit 0; }; done; }
once
need P1 P0 Q5 K5a
J="$T/journal/$ID.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# K5b (clauses K5-1, K5-2, K5-4M, K5-8): as K5a, for reserve/allocate. Stdout must equal the stage-C receipt
# TOLD/receipts/K4c-descriptor.json byte for byte. This compile is a descriptor re-observation only: the
# reserve/allocate setup itself is K6 scope and is never invoked here. A difference is BLOCKED(identity).
run_step "$T/logs" "$ID" 900 'pins_ok && bin_ok && res_ok &&
dirs_ok "$W/out" "$W/keys" &&
X="$(find "$W/out" "$W/keys" -mindepth 1 -print -quit)" && { test -z "$X" || { echo "BLOCKED(hygiene): $X exists in out or keys before K5M"; false; }; } &&
{ sbx_run /usr/bin/time -v /task/bin/mori-k2 compile --source /task/in/conditional-reserve.mori --selector /task/in/reserve-allocate.selector.json > "$T/receipts/K5b-descriptor.json"; RC=$?;
echo "record: mori-k2 compile exit $RC; descriptor follows"; cat "$T/receipts/K5b-descriptor.json"; test "$RC" -eq 0; } &&
cmp "$T/receipts/K5b-descriptor.json" "$TOLD/receipts/K4c-descriptor.json" &&
echo "record: K5b descriptor byte-identical to stage-C K4c-descriptor.json (8cf25094...)" &&
dirs_ok "$W/out" "$W/keys" &&
X="$(find "$W/out" "$W/keys" -mindepth 1 -print -quit)" && { test -z "$X" || { echo "BLOCKED(hygiene): compile left $X in out or keys"; false; }; }' 600
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
