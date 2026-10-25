#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for model in nativeInventory witnessStorage planExports residualWorklist; do
  quint typecheck "$model.qnt"
  quint typecheck "${model}_test.qnt"
  quint test "${model}_test.qnt"
  case "$model" in
    nativeInventory) witnesses=(sawFragment sawGap sawUnknown) ;;
    witnessStorage) witnesses=(published cacheUsed storageReclaimed) ;;
    planExports) witnesses=(unsupportedWitness constraintsOnly midnightSupported) ;;
    residualWorklist) witnesses=(prepared hitFallback compiledBranch retainedRecursiveCall fullSpecialization) ;;
  esac
  quint run "$model.qnt" --invariant modelInv --max-steps 30 --max-samples 10000 \
    --seed 0x303 --witnesses "${witnesses[@]}" --verbosity 1
done
