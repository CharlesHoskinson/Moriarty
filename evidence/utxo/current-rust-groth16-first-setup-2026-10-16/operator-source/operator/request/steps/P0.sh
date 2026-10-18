# K5 operator, step P0: root pins, the host copy of the K4H release mori-k2, the four inputs (host). PROPOSED SOURCE;
# never executed by its author. Run exactly once, after P1, under the root's K5 authorization:
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/logs/P0.exit (absent = failed).
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
ID=P0
{ test ! -L "$T" && test -d "$T" && test ! -L "$T/logs" && test -d "$T/logs"; } || { echo "REFUSED: $T/logs is absent or a link; P1 did not create T5"; exit 0; }
once() { if test -e "$T/logs/$ID.log" || test -e "$T/logs/$ID.exit" || test -e "$T/logs/$ID.skipped"; then echo "REFUSED: step $ID already ran or was skipped; one attempt only"; exit 0; fi; }
need() { for d in "$@"; do case "$d" in Z?2) f="$T/$d.exit" ;; *) f="$T/logs/$d.exit" ;; esac; test "$(cat "$f" 2>/dev/null)" = 0 || { echo "SKIPPED: prerequisite $d not 0" > "$T/logs/$ID.skipped"; exit 0; }; done; }
once
need P1
J="$T/journal/$ID.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# P0 (clauses K5-1, K5-7; outside any sandbox). (1) Operator and public-input pins. (2) k5_preconditions.py p0 re-checks
# every RR5 record and K4H public receipt and lists their hashes into receipts/P0-root-pins.sha256 (BLOCKED(prerequisite)
# on refusal). (3) bwrap not set-id; T5/w is exactly bin, in, out and keys (0700), all empty. (4) Behind the K-7 guards
# on <TK>/w/target/release only, the KH2 mori-k2 line of TK/receipts/KH-binaries-pre.sha256 verifies on the K4H file;
# the host copies that file to w/bin with mode 0555 and the copy verifies too. Nothing else under <TK>/w is read. (5) The
# two sources and two selectors of C go to w/in (0444) and verify against data/k5-in-4.sha256. (6) Disk and memory
# gates. No sandbox runs in P0; no key, VK, RNG or network is touched. Pin or copy failures are BLOCKED(identity).
run_step "$T/logs" "$ID" 600 'pins_ok &&
/usr/bin/python3 -I "$OWN/data/k5_preconditions.py" p0 "$RR5" "$OWN" "$TK" "$T" "$M/operator/request/data/expected-KH3-10-names.txt" > "$T/receipts/P0-root-pins.sha256" &&
echo "record: root preconditions PASS; root records, K4H receipts and review files verified by P0:" && cat "$T/receipts/P0-root-pins.sha256" &&
{ { test ! -u /usr/bin/bwrap && test ! -g /usr/bin/bwrap; } || { echo "BLOCKED(identity): /usr/bin/bwrap is set-id"; false; }; } &&
dirs_ok "$W" "$W/bin" "$W/in" "$W/out" "$W/keys" && test "$(ls -A "$W" | tr "\n" " ")" = "bin in keys out " &&
X="$(find "$W" -mindepth 2 -print -quit)" && { test -z "$X" || { echo "BLOCKED(hygiene): $X exists before placement"; false; }; } &&
test "$(stat -c %a "$W/keys")" = 700 &&
dirs_ok "$TK" "$TK/w" "$TK/w/target" "$TK/w/target/release" && tree_clean "$TK/w/target/release" &&
test ! -L "$TK/w/target/release/mori-k2" && test -f "$TK/w/target/release/mori-k2" &&
grep -E "^[0-9a-f]{64}  mori-k2\$" "$TK/receipts/KH-binaries-pre.sha256" > "$T/receipts/P0-bin.sha256" &&
test "$(wc -l < "$T/receipts/P0-bin.sha256")" -eq 1 &&
( cd "$TK/w/target/release" && sha256sum -c --strict "$T/receipts/P0-bin.sha256" ) &&
install -m 0555 -- "$TK/w/target/release/mori-k2" "$W/bin/mori-k2" &&
( cd "$W/bin" && sha256sum -c --strict "$T/receipts/P0-bin.sha256" ) && cmp "$W/bin/mori-k2" "$TK/w/target/release/mori-k2" &&
test "$(stat -c %a "$W/bin/mori-k2")" = 555 &&
install -m 0444 -- "$C/r1cs/fixtures/reused/installment-invoice.mori" "$W/in/installment-invoice.mori" &&
install -m 0444 -- "$C/r1cs/fixtures/reused/conditional-reserve.mori" "$W/in/conditional-reserve.mori" &&
install -m 0444 -- "$C/k2/requests/invoice-partial.selector.json" "$W/in/invoice-partial.selector.json" &&
install -m 0444 -- "$C/k2/requests/reserve-allocate.selector.json" "$W/in/reserve-allocate.selector.json" &&
( cd "$W/in" && sha256sum -c --strict "$OWN/data/k5-in-4.sha256" ) &&
tree_clean "$W/bin" "$W/in" && test "$(find "$W/in" -mindepth 1 | wc -l)" -eq 4 && bin_ok &&
X="$(find "$W/out" "$W/keys" -mindepth 1 -print -quit)" && { test -z "$X" || { echo "BLOCKED(hygiene): $X exists in out or keys"; false; }; } &&
stat -c "record: %n %s bytes mode %a" "$W/bin/mori-k2" "$W/in/installment-invoice.mori" "$W/in/conditional-reserve.mori" "$W/in/invoice-partial.selector.json" "$W/in/reserve-allocate.selector.json" &&
res_ok' 300
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
