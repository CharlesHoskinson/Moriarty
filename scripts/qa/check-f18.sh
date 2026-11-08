#!/usr/bin/env bash
# F18: memory guard (task 2.0, E11/E16/E17; SRC-016, SRC-015).
# Runs the factory unit tests for the 48 GiB default cap and clamp, the systemd-run scope
# (MemoryMax, MemorySwapMax=0), OOM as capExceeded rss, preflight memory and swap refusals,
# aggregate admission (<= 100 GiB), the concurrent slice, fixed caps and RUST_TEST_THREADS.
# Then checks that the tests above k=13 are #[ignore] in plain cargo test and run from T1.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
(cd models/doctorwatson && python3 -B -m unittest -q \
  factory.tests.test_heavy.MemoryGuardTests \
  factory.tests.test_heavy.ConcurrentHeavyTests \
  factory.tests.test_heavy.FixedBudgetTests \
  factory.tests.test_queue.MemoryAdmissionTests)
b=crates/mori-backend
fail=0
need() { # file, test fn, count of #[ignore] wanted in file
  local n; n=$(grep -c '#\[ignore = "' "$1" || true)
  [ "$n" -ge "$3" ] || { echo "F18: $1 has $n #[ignore], want >= $3 ($2)"; fail=1; }
}
need $b/src/sherlock_machine/c6_checked_value_read_tests.rs mockprover_synthesis_positive_and_wrong_kind 1
need $b/src/sherlock_machine/c6_data_canonical.rs in_circuit_mockprover_synthesis_outcomes 1
need $b/src/sherlock_machine/c6_scalar_failure_step.rs synthesis_distinguishes_constraint_failure 1
need $b/tests/soundness_mutation.rs "three k>13 mutation tests" 3
need $b/src/sherlock_machine/c6_dispatch_tests.rs "two dispatch tests" 2
for t in data_canonical_mutations_are_rejected mockprover_synthesis_positive_and_wrong_kind \
         in_circuit_mockprover_synthesis_outcomes synthesis_distinguishes_constraint_failure; do
  grep -q "$t" qa/tiers/t1.sh || { echo "F18: qa/tiers/t1.sh does not run $t"; fail=1; }
done
[ $fail -eq 0 ] || exit 1
echo "F18 PASS: memory guard unit tests pass; k>13 tests ignored in plain cargo test and run by T1"
