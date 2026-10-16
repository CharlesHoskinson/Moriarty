# Four-piece VS harness-repair local measurement, step OPX1t (tool staging, isolation probe, version/help equality). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX1t.exit (absent = failed; 200 = not started, aggregate
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
J="$T/journal/OPX1t.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX1t (cap 120 s, kill-after 10 s, reserve 136 s for OPX7 100 + OPX7f 36):
#  (1) release data re-checked; a NEW data copy of the pinned binary (literal cp, no helper), mode 0600, sha gate, then
#      chmod 0555 on the copy only; the DATA-ONLY binary stays 0644 and is never executed in place;
#  (2) user+net namespace shows only lo;
#  (3) --version and the four help texts, captured in isolation from the empty probe-cwd, must be BYTE-EQUAL to attempt 2's
#      frozen help (data/parent-evidence-9.sha256 rows, equal to R1's). That fixes every literal flag of OPX2a/OPX3/OPX4/OPX2;
#  (4) every empty directory still empty; tmp listing; help digest record; help read-only.
if ok OPX1; then
run_step "$T/logs" OPX1t 120 10 136 'ak() { unshare --user --map-root-user --net -- env -i PATH=/usr/bin:/bin HOME="$T/empty-home" XDG_CACHE_HOME="$T/empty-xdg-cache" XDG_CONFIG_HOME="$T/empty-xdg-config" XDG_DATA_HOME="$T/empty-xdg-data" TMPDIR="$T/tmp" NO_COLOR=1 "$AIKEN" "$@" < /dev/null; } &&
cd "$DATAONLY" && printf "%s  %s\n" 8adde843b7552bc7810f4be9cfca73693b8cc5f11fbb9e07d586901fd656d487 DATA-ONLY-MANIFEST.json c378acccdaf9533f3375ea623c948d3dcb21f7e8c5ecbb741162c53653470b13 aiken-x86_64-unknown-linux-musl.tar.gz 4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677 compiler-data/aiken-x86_64-unknown-linux-musl/aiken | sha256sum -c - &&
test "$(stat -c %a compiler-data/aiken-x86_64-unknown-linux-musl/aiken)" = 644 &&
test ! -e "$AIKEN" && cp -- "$DATAONLY/compiler-data/aiken-x86_64-unknown-linux-musl/aiken" "$AIKEN" && chmod 0600 "$AIKEN" &&
printf "%s  %s\n" 4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677 "$AIKEN" | sha256sum -c - && test "$(stat -c %a "$AIKEN")" = 600 &&
stat -c "%a %s %F %i" "$AIKEN" > "$T/receipts/bin-stat-staged.txt" &&
chmod 0555 "$AIKEN" && stat -c "%a %s %F %i" "$AIKEN" > "$T/receipts/bin-stat.txt" && cat "$T/receipts/bin-stat.txt" && test "$(stat -c %a "$AIKEN")" = 555 &&
test "$(stat -c %a "$DATAONLY/compiler-data/aiken-x86_64-unknown-linux-musl/aiken")" = 644 &&
unshare --user --map-root-user --net -- cat /proc/net/dev && test "$(unshare --user --map-root-user --net -- awk "NR>2{print \$1}" /proc/net/dev)" = "lo:" &&
cd "$T/probe-cwd" &&
ak --version > "$T/help/version.txt" && ak check --help > "$T/help/check.txt" && ak build --help > "$T/help/build.txt" &&
ak blueprint apply --help > "$T/help/blueprint-apply.txt" && ak blueprint hash --help > "$T/help/blueprint-hash.txt" &&
cat "$T/help/version.txt" && test "$(cat "$T/help/version.txt")" = "aiken v1.1.24+bacbeb3" &&
cd "$T" && grep " help/" "$OWN/data/parent-evidence-9.sha256" | sha256sum -c - && test "$(grep -c " help/" "$OWN/data/parent-evidence-9.sha256")" -eq 5 &&
for d in empty-home empty-xdg-cache empty-xdg-config empty-xdg-data probe-cwd; do test -d "$T/$d" && test -z "$(find "$T/$d" -mindepth 1 -print -quit)" || { echo "not empty or missing: $d"; exit 1; }; done &&
find "$T/tmp" -mindepth 1 | tee "$T/receipts/tmp-after-OPX1t.txt" &&
"$PY" -I "$OWN/tools/freeze.py" "$T/receipts/help-freeze.json" "raw pinned-tool help, byte-equal to the attempt-2 frozen help" "$T" help &&
chmod 0444 "$T"/help/*.txt && ls -l "$T/help"'
else skip_step "$T/logs" OPX1t "OPX1 not 0"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
