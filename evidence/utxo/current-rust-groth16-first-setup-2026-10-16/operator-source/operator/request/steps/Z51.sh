# K5 operator, step Z51: hygiene postcheck (ALWAYS once K5 started). PROPOSED SOURCE; never executed by its author.
# Run exactly once, after H5:
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/logs/Z51.exit (absent = failed).
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
ID=Z51
once() { if test -e "$T/logs/$ID.log" || test -e "$T/logs/$ID.exit" || test -e "$T/logs/$ID.skipped"; then echo "REFUSED: step $ID already ran or was skipped; one attempt only"; exit 0; fi; }
test -e "$T/logs/P1.log" || test -e "$T/logs/P1.skipped" || exit 0
once
J="$T/journal/$ID.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# Z51 hygiene (clauses K5-7, K5-8; R-K4H-01 final pin check): after every sandbox has exited, the operator bytes are
# verified against RR5 one last time, the placed binary and the four inputs against their pins, and out/keys are
# examined behind the K-7 guards. The setup record is hashed as a public artefact. The proving key is hashed
# ONLY when its listed entry is a regular non-link file of mode 0600 with one hard link; otherwise the custody
# guard records the refusal and the key bytes are never opened. Only path, mode, size, link count and SHA-256
# enter the record, exactly as DV-K2-1-R requires for leftovers as well as for a completed key. Nothing is
# deleted, moved or reused; T5/w is retained unchanged for the actual-result review. A failure is
# BLOCKED(hygiene): no K5 result then qualifies.
run_step "$T/logs" "$ID" 600 'pins_ok && bin_ok &&
( cd "$W/in" && sha256sum -c --strict "$OWN/data/k5-in-4.sha256" ) &&
dirs_ok "$W" "$W/bin" "$W/in" "$W/out" "$W/keys" && tree_clean "$W/bin" "$W/in" "$W/out" &&
test "$(ls -A "$W" | tr "\n" " ")" = "bin in keys out " &&
test "$(stat -c %a "$W/keys")" = 700 &&
echo "--- record: out and keys listings (names and metadata only) ---" && ls -la "$W/out" "$W/keys" &&
{ if test -e "$W/out/invoice-partial.setup.json" || test -L "$W/out/invoice-partial.setup.json"; then
  { test ! -L "$W/out/invoice-partial.setup.json" && test -f "$W/out/invoice-partial.setup.json"; } &&
  stat -c "record: setup record %n %s bytes mode %a links %h" "$W/out/invoice-partial.setup.json" &&
  sha256sum "$W/out/invoice-partial.setup.json" || { echo "BLOCKED(hygiene): setup record is a link or not a regular file; not read"; false; }; else echo "record: no setup record present"; fi; } &&
{ if test -e "$W/keys/invoice-partial.pk" || test -L "$W/keys/invoice-partial.pk"; then
  if test ! -L "$W/keys/invoice-partial.pk" && test -f "$W/keys/invoice-partial.pk" && test "$(stat -c %a "$W/keys/invoice-partial.pk")" = 600 && test "$(stat -c %h "$W/keys/invoice-partial.pk")" -eq 1; then
    stat -c "record: proving key %n %s bytes mode %a links %h" "$W/keys/invoice-partial.pk" && sha256sum "$W/keys/invoice-partial.pk";
  else echo "BLOCKED(hygiene): proving key fails the custody guard (link, type, mode or link count); its bytes were not read"; false; fi; else echo "record: no proving key present"; fi; } &&
echo "--- record: task-owned listings ---" && ls -la "$T" "$W" &&
echo "record: free bytes at T5 = $(df --output=avail -B1 "$T" | tail -n 1)" &&
echo "record: keys retained unread apart from the guarded hash above; no deletion in this run (clause K5-7)"' 300
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
