#!/bin/bash
# T20 Quint runner. Usage: run-model.sh MODEL "TEST_MAIN MUTANT_MAINS..." "WITNESS_NEGATIONS..."
# Expected: every test passes; Inv holds on the design main (first); every mutant main violates Inv;
# every witness negation is violated on the design main (the witness is reachable).
cd "$(dirname "$0")"
M=$1; read -r -a MAINS <<< "$2"; read -r -a WITS <<< "$3"
S=${SAMPLES:-3000}; N=${STEPS:-14}
q() { quint run "${M}_test.qnt" --main="$1" --init=init --step=step --invariant="$2" --max-samples="$S" --max-steps="$N" --seed="${SEED:-7}" --verbosity=1 2>&1 | grep -E "^\[(ok|violation)\]" | head -1; }
echo "quint $(quint --version) model $M samples $S steps $N seed ${SEED:-7}"
for m in "${MAINS[@]}"; do echo "test $m: $(quint test ${M}_test.qnt --main=$m --match '^(pos|neg|mut)_' 2>&1 | grep -E 'passing|failing' | tr '\n' ' ')"; done
echo "design Inv: $(q ${MAINS[0]} Inv)"
for w in "${WITS[@]}"; do echo "design witness $w: $(q ${MAINS[0]} $w)"; done
for m in "${MAINS[@]:1}"; do echo "mutant $m Inv: $(q $m Inv)"; done
