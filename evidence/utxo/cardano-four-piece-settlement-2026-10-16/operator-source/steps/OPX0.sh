# Four-piece VS harness-repair local measurement, step OPX0 (read-only guarded preflight). PROPOSED SOURCE; never executed by its author.
# Run exactly once, FIRST, before T exists, only after the root has materialized O'' verbatim, written its
# ROOT-SOURCE-MANIFEST.json and PRE-LAUNCH.json per ROOT-PLACEMENT-INSTRUCTIONS.json, a fresh source/effect review of the
# exact candidate and O'' bytes, a second substantive position on the operator/resource clauses and the root's exact authorization:
#   env -i PATH=/usr/bin:/bin HOME=/home/charl/moriarty-eutxo-validation-2026-10-12/current-cardano-four-way-vs-harness-repair-local-measurement-20261016/empty-home TZ=UTC /usr/bin/bash --noprofile --norc <this file> < /dev/null
# EXCEPTION to the "always exit 0" rule: T does not exist yet, so OPX0 writes NOTHING anywhere. Its whole record is its
# stdout and its exit status, both captured by the root. Exit 0 = every check holds; anything else = STOP (no OPX1).
# The aggregate ledger charges OPX0 at its maximum, 70 s (60 s wall + 10 s kill-after).
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
export PY=/usr/bin/python3.14
printf 'OPX0 start %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)"
# (1) T absent; (2) tools/preflight.py: pinstore PRE-LAUNCH, operator manifest and verbatim originals, candidate manifest
# (41 rows and originals) against data/, candidate tree 32 files / 3 sub-directories (4 with the root) / 0 other,
# EXPECTATIONS names and fail flags against the two test modules' declarations, --match-tests module selection, parent
# evidence and R1 label control pins; (3) an independent coreutils check of the same package and document lists;
# (4) stdlib pin, 38 files / 13 dirs / 0 other; (5) DATA-ONLY compiler release data, binary still 0644; (6) host tools.
nice -n 10 timeout --kill-after=10 60 prlimit --as=34359738368 --core=0 -- bash -o pipefail -c '
test ! -e "$T" &&
"$PY" -I "$OWN/tools/preflight.py" pre "$OWN" "$PINSTORE/PRE-LAUNCH.json" "$M" "$T" "$PT" "$R1T" &&
cd "$M" && sha256sum -c "$OWN/data/m-docs-10.sha256" &&
cd "$M/candidate/aiken" && sha256sum -c "$OWN/data/candidate-32.sha256" &&
test "$(find . -type f | wc -l)" -eq 32 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 3 && test "$(find . -type d | wc -l)" -eq 4 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 &&
test "$(ls -A validators | LC_ALL=C sort | tr "\n" " ")" = "c2.ak s2p.ak vi.ak vs1.ak vs2.ak vs3.ak vs4.ak " &&
test ! -e validators/vs.ak && test ! -e validators/s2.ak && test ! -e build && test ! -e aiken.lock && test ! -e plutus.json &&
cd "$PT" && sha256sum -c "$OWN/data/parent-evidence-9.sha256" &&
cd "$PIN" && printf "%s  %s\n" a26daa087fae05f9b35a06ed382d20e92e4d1ea8b76b59947134b7e4b2197266 SOURCE-PIN.json e44ed44783d17f4cb0b58c59f98a75bc09fc64c793dd25f3ab56b4097cdf1b1c SOURCE-MANIFEST.json 90c07de94ebf858f0d78292a9482cfa3b2b45767ed85fd381e74c0463cdf9b51 stdlib-source.tar.gz | sha256sum -c - &&
cd "$PIN/source" && sha256sum -c "$OWN/data/stdlib-38.sha256" &&
test "$(find . -type f | wc -l)" -eq 38 && test "$(find . -mindepth 1 -type d | wc -l)" -eq 13 && test "$(find . -mindepth 1 ! -type f ! -type d | wc -l)" -eq 0 &&
cd "$DATAONLY" && printf "%s  %s\n" 8adde843b7552bc7810f4be9cfca73693b8cc5f11fbb9e07d586901fd656d487 DATA-ONLY-MANIFEST.json c378acccdaf9533f3375ea623c948d3dcb21f7e8c5ecbb741162c53653470b13 aiken-x86_64-unknown-linux-musl.tar.gz 4257cc8be37cbde5bf7b5dd111e43a1277f45352fb2433ad7a7e647209aa7677 compiler-data/aiken-x86_64-unknown-linux-musl/aiken | sha256sum -c - &&
sha256sum -c aiken-x86_64-unknown-linux-musl.tar.gz.sha256 && test "$(stat -c %a compiler-data/aiken-x86_64-unknown-linux-musl/aiken)" = 644 &&
test -x "$PY" && test -x /usr/bin/unshare && test -x /usr/bin/prlimit && test -x /usr/bin/timeout && test -x /usr/bin/nice && test -x /usr/bin/awk && test -x /usr/bin/bash &&
"$PY" -I -c "import sys; print(sys.version)" &&
if test -x /usr/bin/time; then echo "peak-rss-tool: /usr/bin/time present"; else echo "peak-rss-tool: absent (no peak RSS records)"; fi
' < /dev/null
RC=$?
printf 'OPX0 end %s exit %s\n' "$(date -u +%Y-%m-%dT%H:%M:%S.%NZ)" "$RC"
exit "$RC"
