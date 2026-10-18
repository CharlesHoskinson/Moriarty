# Four-piece VS harness-repair local measurement, step OPX4 (parameter application and hash chain). PROPOSED SOURCE; never executed by its author.
# Run exactly once, in EXECUTION-PROPOSAL.json launch order, only after a fresh source/effect review of these exact bytes,
# a second substantive position on the operator clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX4.exit (absent = failed; 200 = not started, aggregate
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
J="$T/journal/OPX4.txt"
printf 'start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$J"
# OPX4 (gate OPX3 and OPX3p both 0; cap 240 s, kill-after 10 s, reserve 136 s). Acyclic; one parameter per apply, each read
# from the PRECEDING read-only blueprint; every hash is the pinned tool's `blueprint hash` output, never a fixture identifier:
#   U : C2 = hash(U, c2); VS1 = hash(U, vs1); VS2 = hash(U, vs2); VS4 = hash(U, vs4)       (no parameter)
#   B1 = apply(U,  vi,  581c||C2)                                            VI  = hash(B1, vi)
#   B2 = apply(B1, vs3, 581c||C2);  B3 = apply(B2, vs3, 581c||VI)            VS3 = hash(B3, vs3)
#   B4..B9 = apply(s2p) with 581c||C2, VI, VS1, VS2, VS3, VS4 in declaration order    S2' = hash(B9, s2p), derived last
#   final: C2, VI, VS1, VS2, VS3, VS4 re-derived from B9.
# 9 applications and 7 + 6 = 13 hash calls (22 aiken calls). CBOR bytes(28) = 0x58 0x1c || 28 hash bytes. Every output is a NEW file, read-only at
# once. tools/hash_chain4.py then applies the falsifiers and writes receipts/chain.json; the unapplied copy is re-verified.
if ok OPX3 OPX3p; then
run_step "$T/logs" OPX4 240 10 136 'ak() { unshare --user --map-root-user --net -- env -i PATH=/usr/bin:/bin HOME="$T/empty-home" XDG_CACHE_HOME="$T/empty-xdg-cache" XDG_CONFIG_HOME="$T/empty-xdg-config" XDG_DATA_HOME="$T/empty-xdg-data" TMPDIR="$T/tmp" NO_COLOR=1 "$AIKEN" "$@" < /dev/null; } &&
hx() { test "$(wc -l < "$1")" -eq 1 && grep -Exq "[0-9a-f]{56}" "$1" && cat "$1"; } &&
A="$T/a2" && R="$T/receipts" && cd "$T/run" &&
sha256sum -c "$R/plutus-unapplied-sha256.txt" && printf "%s  %s\n" "$(cut -d " " -f1 "$R/plutus-unapplied-sha256.txt")" "$A/plutus.unapplied.json" | sha256sum -c - &&
for f in b1-vi b2-vs3-c2 b3-vs3 b4-s2p-c2 b5-s2p-vi b6-s2p-vs1 b7-s2p-vs2 b8-s2p-vs3 b9-applied; do test ! -e "$A/$f.json" || { echo "exists: $f"; exit 1; }; done &&
ak blueprint hash --in "$A/plutus.unapplied.json" --module c2 --validator c2 > "$R/hash-c2.txt" && hx "$R/hash-c2.txt" &&
ak blueprint hash --in "$A/plutus.unapplied.json" --module vs1 --validator vs1 > "$R/hash-vs1.txt" && hx "$R/hash-vs1.txt" &&
ak blueprint hash --in "$A/plutus.unapplied.json" --module vs2 --validator vs2 > "$R/hash-vs2.txt" && hx "$R/hash-vs2.txt" &&
ak blueprint hash --in "$A/plutus.unapplied.json" --module vs4 --validator vs4 > "$R/hash-vs4.txt" && hx "$R/hash-vs4.txt" &&
printf "581c%s\n" "$(cat "$R/hash-c2.txt")" > "$R/param-vi-1-c2.hex" &&
ak blueprint apply --in "$A/plutus.unapplied.json" --out "$A/b1-vi.json" --module vi --validator vi "$(cat "$R/param-vi-1-c2.hex")" && chmod 0444 "$A/b1-vi.json" &&
ak blueprint hash --in "$A/b1-vi.json" --module vi --validator vi > "$R/hash-vi.txt" && hx "$R/hash-vi.txt" &&
printf "581c%s\n" "$(cat "$R/hash-c2.txt")" > "$R/param-vs3-1-c2.hex" &&
ak blueprint apply --in "$A/b1-vi.json" --out "$A/b2-vs3-c2.json" --module vs3 --validator vs3 "$(cat "$R/param-vs3-1-c2.hex")" && chmod 0444 "$A/b2-vs3-c2.json" &&
printf "581c%s\n" "$(cat "$R/hash-vi.txt")" > "$R/param-vs3-2-vi.hex" &&
ak blueprint apply --in "$A/b2-vs3-c2.json" --out "$A/b3-vs3.json" --module vs3 --validator vs3 "$(cat "$R/param-vs3-2-vi.hex")" && chmod 0444 "$A/b3-vs3.json" &&
ak blueprint hash --in "$A/b3-vs3.json" --module vs3 --validator vs3 > "$R/hash-vs3.txt" && hx "$R/hash-vs3.txt" &&
printf "581c%s\n" "$(cat "$R/hash-c2.txt")" > "$R/param-s2p-1-c2.hex" &&
ak blueprint apply --in "$A/b3-vs3.json" --out "$A/b4-s2p-c2.json" --module s2p --validator s2p "$(cat "$R/param-s2p-1-c2.hex")" && chmod 0444 "$A/b4-s2p-c2.json" &&
printf "581c%s\n" "$(cat "$R/hash-vi.txt")" > "$R/param-s2p-2-vi.hex" &&
ak blueprint apply --in "$A/b4-s2p-c2.json" --out "$A/b5-s2p-vi.json" --module s2p --validator s2p "$(cat "$R/param-s2p-2-vi.hex")" && chmod 0444 "$A/b5-s2p-vi.json" &&
printf "581c%s\n" "$(cat "$R/hash-vs1.txt")" > "$R/param-s2p-3-vs1.hex" &&
ak blueprint apply --in "$A/b5-s2p-vi.json" --out "$A/b6-s2p-vs1.json" --module s2p --validator s2p "$(cat "$R/param-s2p-3-vs1.hex")" && chmod 0444 "$A/b6-s2p-vs1.json" &&
printf "581c%s\n" "$(cat "$R/hash-vs2.txt")" > "$R/param-s2p-4-vs2.hex" &&
ak blueprint apply --in "$A/b6-s2p-vs1.json" --out "$A/b7-s2p-vs2.json" --module s2p --validator s2p "$(cat "$R/param-s2p-4-vs2.hex")" && chmod 0444 "$A/b7-s2p-vs2.json" &&
printf "581c%s\n" "$(cat "$R/hash-vs3.txt")" > "$R/param-s2p-5-vs3.hex" &&
ak blueprint apply --in "$A/b7-s2p-vs2.json" --out "$A/b8-s2p-vs3.json" --module s2p --validator s2p "$(cat "$R/param-s2p-5-vs3.hex")" && chmod 0444 "$A/b8-s2p-vs3.json" &&
printf "581c%s\n" "$(cat "$R/hash-vs4.txt")" > "$R/param-s2p-6-vs4.hex" &&
ak blueprint apply --in "$A/b8-s2p-vs3.json" --out "$A/b9-applied.json" --module s2p --validator s2p "$(cat "$R/param-s2p-6-vs4.hex")" && chmod 0444 "$A/b9-applied.json" &&
ak blueprint hash --in "$A/b9-applied.json" --module s2p --validator s2p > "$R/hash-s2p.txt" && hx "$R/hash-s2p.txt" &&
for g in c2 vi vs1 vs2 vs3 vs4; do ak blueprint hash --in "$A/b9-applied.json" --module "$g" --validator "$g" > "$R/hash-$g-final.txt" && hx "$R/hash-$g-final.txt" || exit 1; done &&
"$PY" -I "$OWN/tools/hash_chain4.py" "$A" "$R" "$R/chain.json" "$PT/receipts/chain.json" &&
sha256sum -c "$R/plutus-unapplied-sha256.txt" && printf "%s  %s\n" "$(cut -d " " -f1 "$R/plutus-unapplied-sha256.txt")" "$A/plutus.unapplied.json" | sha256sum -c - &&
( cd "$A" && sha256sum plutus.unapplied.json b1-vi.json b2-vs3-c2.json b3-vs3.json b4-s2p-c2.json b5-s2p-vi.json b6-s2p-vs1.json b7-s2p-vs2.json b8-s2p-vs3.json b9-applied.json ) | tee "$R/a2-sha256.txt"'
else skip_step "$T/logs" OPX4 "OPX3 or OPX3p not 0"; fi
printf 'end %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" >> "$J"
times >> "$J"
