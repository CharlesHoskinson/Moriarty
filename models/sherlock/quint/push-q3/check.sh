#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for model in nativeInventory witnessStorage treeInterning planExports residualWorklist; do
  quint typecheck "$model.qnt"
  quint typecheck "${model}_test.qnt"
  quint test "${model}_test.qnt"
  case "$model" in
    nativeInventory) witnesses=(sawFragment sawGap sawUnknown) ;;
    witnessStorage) witnesses=(published cacheUsed storageReclaimed) ;;
    treeInterning) witnesses=(finished shared fourNodes) ;;
    planExports) witnesses=(unsupportedWitness constraintsOnly midnightSupported) ;;
    residualWorklist) witnesses=(prepared hitFallback compiledBranch retainedRecursiveCall fullSpecialization) ;;
  esac
  quint run "$model.qnt" --invariant modelInv --max-steps 30 --max-samples 10000 \
    --seed 0x303 --witnesses "${witnesses[@]}" --verbosity 1
done
quint typecheck s5ResidualProjection.qnt
quint typecheck s5ResidualProjection_test.qnt
quint test s5ResidualProjection_test.qnt
quint run s5ResidualProjection.qnt --invariant sourceInv --max-steps 30 --max-samples 10000 \
  --seed 0x303 --witnesses finished lostCallSuccessor --verbosity 1
# The pinned source's contract violation is expected, not a passing invariant.
if quint run s5ResidualProjection.qnt --invariant preservesDynamicTargets --max-steps 30 \
  --seed 0x303 --verbosity 2 > s5-residual-counterexample.log 2>&1; then
  echo 'Expected the pinned arith-family source counterexample' >&2
  exit 1
else
  result=$?
  if [ "$result" -ne 1 ] || ! rg -q 'Invariant violated' s5-residual-counterexample.log; then
    cat s5-residual-counterexample.log >&2
    exit 1
  fi
fi
python3 audit_s5.py > s5-source-observations.json
