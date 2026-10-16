# K5 operator, step P1: create the empty K5 run directory, only after the root's K5 records check. PROPOSED SOURCE;
# never executed by its author. Run exactly once, first, after the K4H actual-result review closed and the root wrote
# every RR5 record of ROOT-PLACEMENT-INSTRUCTIONS.json (including K5-PRE-LAUNCH.json):
#   env -i PATH=/usr/bin:/bin HOME=/nonexistent LC_ALL=C TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is the content of $T/logs/P1.exit (absent = failed). On refusal it
# creates nothing and prints the reason on its own stdout, which the root keeps; there is no T5 to log into.
umask 022
cd /
export LC_ALL=C
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export M5="$BASE/current-cardano-k2-k5-setup-operator-materialized-20261016"
export OWN="$M5/operator/request"
export RR5="$BASE/current-cardano-k2-k5-root-records-20261016"
export M="$BASE/current-cardano-k2-k4h-hostile-materialized-20261016"
export TK="$BASE/current-cardano-k2-k4h-hostile-local-execution-20261016"
export T="$BASE/current-cardano-k2-k5-single-party-test-setup-20261016"
export W="$T/w"
S="$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)"
refuse() { echo "REFUSED(P1): $*; nothing was created"; exit 0; }
# P1 (clauses K5-1, K5-7; unwrapped like H1). Independent guards, in order: T5 absent; RR5 and its five root records
# are regular and not links; the operator bytes and the materialization manifest equal their root pins; every pinned
# public input verifies; k5_preconditions.py (now itself pinned) accepts every root record and K4H public receipt.
# Only then: one atomic mkdir of T5, its logs/receipts/journal and w/{bin,in,out}, and w/keys with mode 0700.
{ test ! -e "$T" && test ! -L "$T"; } || refuse "T5 already exists"
{ test ! -L "$RR5" && test -d "$RR5"; } || refuse "RR5 is absent or a link"
for f in K5-OPERATOR-PINS.sha256 K5-MATERIALIZATION-PIN.sha256 K5-PRECONDITIONS.json K5-AUTHORIZATION.json K5-PRE-LAUNCH.json; do
  { test ! -L "$RR5/$f" && test -f "$RR5/$f"; } || refuse "RR5/$f is absent, a link or not a regular file"
done
sha256sum -c --strict --quiet "$RR5/K5-OPERATOR-PINS.sha256" || refuse "operator files differ from RR5/K5-OPERATOR-PINS.sha256"
sha256sum -c --strict --quiet "$RR5/K5-MATERIALIZATION-PIN.sha256" || refuse "the materialization manifest differs from its root pin"
sha256sum -c --strict --quiet "$OWN/data/k5-abs.sha256" || refuse "a pinned public input differs from data/k5-abs.sha256"
PRE="$(/usr/bin/python3 -I "$OWN/data/k5_preconditions.py" p1 "$RR5" "$OWN" "$TK" "$T" "$M/operator/request/data/expected-KH3-10-names.txt" 2>&1)" || { printf '%s\n' "$PRE"; refuse "root preconditions"; }
mkdir "$T" && mkdir "$T/logs" "$T/receipts" "$T/journal" "$W" "$W/bin" "$W/in" "$W/out" && mkdir -m 0700 "$W/keys" &&
test "$(stat -c %a "$W/keys")" = 700 &&
printf 'record: P1 root preconditions PASS before mkdir; verified root and K4H public files:\n%s\n' "$PRE" > "$T/logs/P1.log" &&
echo 0 > "$T/logs/P1.exit" && printf 'start %s\nend %s\n' "$S" "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$T/journal/P1.txt" && times >> "$T/journal/P1.txt"
