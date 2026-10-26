#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
for model in nativeInventory witnessStorage treeInterning planExports residualWorklist residualBlockBudget residualFlow kResidualDispatch s5ResidualRepaired exportMulBindings programPatternRows; do
  quint typecheck "$model.qnt"
  quint typecheck "${model}_test.qnt"
  backend=rust
  if [ "$model" = exportMulBindings ]; then backend=typescript; fi
  quint test "${model}_test.qnt" --backend="$backend"
  case "$model" in
    nativeInventory) witnesses=(sawFragment sawGap sawUnknown) ;;
    witnessStorage) witnesses=(published cacheUsed storageReclaimed) ;;
    treeInterning) witnesses=(finished shared fourNodes) ;;
    planExports) witnesses=(unsupportedWitness constraintsOnly midnightSupported) ;;
    residualWorklist) witnesses=(prepared hitFallback compiledBranch retainedRecursiveCall fullSpecialization) ;;
    residualBlockBudget) witnesses=(configured compiled genericUsed invalidRefused) ;;
    residualFlow) witnesses=(acceptedSome refusedSome usedSuccessor exitedGeneric) ;;
    kResidualDispatch) witnesses=(sampled genericUsed stopped nextArgumentBranches) ;;
    s5ResidualRepaired) witnesses=(finished delegatedCall sizeFallback fullImage) ;;
    exportMulBindings) witnesses=(success overflow unequalOperands) ;;
    programPatternRows) witnesses=(finished sourceLate sourceEarly repairedUnitRow) ;;
  esac
  quint run "$model.qnt" --backend="$backend" --invariant modelInv --max-steps 30 --max-samples 10000 \
    --seed 0x303 --witnesses "${witnesses[@]}" --verbosity 1
done
if quint run programPatternRows.qnt --invariant sourceMatchesCodec --max-steps 30 --max-samples 10000 \
  --seed 0x303 --verbosity 2 > program-pattern-counterexample.log 2>&1; then
  echo 'Expected the pinned PROG pattern-row boundary counterexample' >&2
  exit 1
else
  result=$?
  if [ "$result" -ne 1 ] || ! rg -q 'Invariant violated' program-pattern-counterexample.log; then
    cat program-pattern-counterexample.log >&2
    exit 1
  fi
fi
quint typecheck s5ResidualProjection.qnt
quint typecheck s5ResidualProjection_test.qnt
quint test s5ResidualProjection_test.qnt
quint run s5ResidualProjection.qnt --invariant sourceInv --max-steps 30 --max-samples 10000 \
  --seed 0x303 --witnesses finished lostCallSuccessor --verbosity 1
# The pinned source's contract violation is expected, not a passing invariant.
if quint run s5ResidualProjection.qnt --invariant preservesDynamicTargets --max-steps 30 --max-samples 10000 \
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
python3 audit_qualification.py > qualification-counterexample.json
quint typecheck rustExportClassification.qnt
quint typecheck rustExportClassification_test.qnt
quint test rustExportClassification_test.qnt
quint run rustExportClassification.qnt --invariant sourceInv --max-steps 30 --max-samples 10000 \
  --seed 0x303 --witnesses misclassifiedPendingFrame misclassifiedLeftFrame --verbosity 1
quint run rustExportClassification.qnt --step classifyRepaired --invariant repairedInv \
  --max-steps 30 --max-samples 10000 --seed 0x303 --witnesses repairedPendingFrame repairedLeftFrame --verbosity 1
if quint run rustExportClassification.qnt --invariant belongsToActiveStep --max-steps 30 --max-samples 10000 \
  --seed 0x303 --verbosity 2 > rust-export-classification-counterexample.log 2>&1; then
  echo 'Expected the pinned Rust pending-frame classification counterexample' >&2
  exit 1
else
  result=$?
  if [ "$result" -ne 1 ] || ! rg -q 'Invariant violated' rust-export-classification-counterexample.log; then
    cat rust-export-classification-counterexample.log >&2
    exit 1
  fi
fi
python3 audit_s5.py > s5-source-observations.json
quint typecheck rustResidualIdentity.qnt
quint typecheck rustResidualIdentity_test.qnt
quint test rustResidualIdentity_test.qnt
quint run rustResidualIdentity.qnt --invariant sourceInv --max-steps 30 --max-samples 10000 \
  --seed 0x303 --witnesses observedRuntimeDependency --verbosity 1
if quint run rustResidualIdentity.qnt --invariant stableAcrossFuel --max-steps 30 --max-samples 10000 \
  --seed 0x303 --verbosity 2 > rust-residual-identity-counterexample.log 2>&1; then
  echo 'Expected the pinned Rust runtime-specialization counterexample' >&2
  exit 1
else
  result=$?
  if [ "$result" -ne 1 ] || ! rg -q 'Invariant violated' rust-residual-identity-counterexample.log; then
    cat rust-residual-identity-counterexample.log >&2
    exit 1
  fi
fi
echo 'push-q3 checks passed'
