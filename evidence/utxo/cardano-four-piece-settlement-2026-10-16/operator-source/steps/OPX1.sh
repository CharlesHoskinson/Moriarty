# Four-piece VS harness-repair local measurement, step OPX1 (fresh task directory). PROPOSED SOURCE; never executed by its author.
# Run exactly once, only if OPX0 exited 0, after the review, positions and root authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# This script always exits 0. The step result is $T/logs/OPX1.exit (absent = failed: STOP; nothing else runs, T may not be ours).
# Unwrapped (accepted residual, as the parent OP1): one atomic mkdir chain; writes nothing at all if T already exists.
# Creates the empty aggregate-budget ledger journal/elapsed.txt; the ledger charges OPX1 a fixed 5 s.
umask 022
cd /
export BASE=/home/charl/moriarty-eutxo-validation-2026-10-12
export T="$BASE/current-cardano-four-way-vs-harness-repair-local-measurement-20261016"
S="$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)"
mkdir "$T" && mkdir "$T/logs" "$T/receipts" "$T/journal" "$T/bin" "$T/help" "$T/a2" "$T/tmp" "$T/empty-home" "$T/empty-xdg-cache" "$T/empty-xdg-config" "$T/empty-xdg-data" "$T/probe-cwd" && : > "$T/journal/elapsed.txt" && echo 0 > "$T/logs/OPX1.exit" && printf 'start %s\nend %s\n' "$S" "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" > "$T/journal/OPX1.txt" && times >> "$T/journal/OPX1.txt"
