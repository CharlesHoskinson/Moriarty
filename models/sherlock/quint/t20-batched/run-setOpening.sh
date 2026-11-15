#!/bin/bash
# T20 fix 5: setOpening.qnt checks. Expected: tests pass; Inv holds on the design; each mutant breaks
# Inv; each non-vacuity witness is reached (its negation is violated).
cd "$(dirname "$0")"
S=${SAMPLES:-3000}; N=${STEPS:-14}
q() { quint run setOpening_test.qnt --main="$1" --init=init --step=step --invariant="$2" --max-samples="$S" --max-steps="$N" --seed="${SEED:-7}" --verbosity=1 2>&1 | grep -E "^\[(ok|violation)\]" | head -1; }
for m in setOpening_test setOpening_mutants setOpening_mutants2 setOpening_mutants3 setOpening_mutants4; do
  echo "test $m: $(quint test setOpening_test.qnt --main=$m --match '^(pos|neg|mut)_' 2>&1 | grep -E 'passing|failing' | tr '\n' ' ')"
done
echo "design Inv: $(q setOpening_test Inv)"
for w in NotWitDeepOpen NotWitForeignOpen NotWitDiamond; do echo "design witness $w: $(q setOpening_test $w)"; done
for m in setOpening_mutants setOpening_mutants2 setOpening_mutants3 setOpening_mutants4; do echo "mutant $m Inv: $(q $m Inv)"; done
