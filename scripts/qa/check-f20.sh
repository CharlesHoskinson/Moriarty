#!/usr/bin/env bash
# F20: Quint model of the general aggregation protocol (task 2.11, SRC-017, design §3.6).
# Typechecks models/sherlock/quint/aggregation.qnt and its harness, runs the harness with
# TREE in {false, true} and S in {1, 2, 4, 8} (every seeded mutant must be exercised and caught),
# then bounded invariant sweeps of the runnable instances (chain, tree, tree at S=1, Q-C scheduler).
# Exit 2 when quint is absent. Bounded: --max-samples and --max-steps are explicit.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
command -v quint >/dev/null || { echo "SKIP F20: quint not installed (exit 2)"; exit 2; }
model=models/sherlock/quint/aggregation.qnt
tests=models/sherlock/quint/aggregation_test.qnt
samples="${F20_SAMPLES:-100}"
seed="${F20_SEED:-0x2a}"

quint typecheck "$model"
quint typecheck "$tests"

out="$(quint test "$tests" --main aggregation_test --max-samples "$samples" --seed "$seed" 2>&1)" \
  || { echo "$out" | grep -v '^    ok' | tail -40; echo "F20: quint test failed"; exit 1; }
echo "$out" | grep -E '^  [0-9]+ (passing|failed)'
if echo "$out" | grep -qE '^  [0-9]+ failed'; then echo "F20: tests failed"; exit 1; fi
# One seeded mutant per invariant (I1-I10, fixed keys, TREE guard, Q-C), each listed and passed.
for m in Linkage Range Reuse Padding Unary Schedule Genesis Replay Program Shape Liveness Keys \
         JoinInChain OverAdmit NodeEarly NoReserve; do
  grep -qE "ok .*Mutant${m}CaughtTest passed" <<<"$out" || { echo "F20: mutant $m not exercised"; exit 1; }
done
# Both shapes and every S must be present in the passing list.
for tag in chainS1 chainS2 chainS4 chainS8 treeS1 treeS2 treeS4 treeS8; do
  grep -qE "ok ${tag}EveryProgramTest passed" <<<"$out" || { echo "F20: $tag not covered"; exit 1; }
done

for inst in aggregation_chain aggregation_tree aggregation_tree_s1; do
  quint run "$model" --main "$inst" --invariant allInvariants --max-steps 30 --max-samples 200 --seed "$seed" \
    | grep -q 'No violation found' || { echo "F20: invariant violation in $inst"; exit 1; }
done
quint run "$model" --main leaf_scheduler_p3 --invariant QC_all --max-steps 40 --max-samples 300 --seed "$seed" \
  | grep -q 'No violation found' || { echo "F20: Q-C violation in leaf_scheduler_p3"; exit 1; }
echo "F20 ok: aggregation model typechecks, harness passes with every mutant caught, sweeps clean"
