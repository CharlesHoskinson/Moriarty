# Change: WP01 Compact DSL feasibility

## Why

Moriarty needs a falsification test before the Core grows. The current
atomic-swap experiment is reproduced at S4, but real proof generation remains
outside this package.

## What Changes

- Freeze the finite Core fragment and its generated Compact output.
- State the exact boundary of the resulting Compact DSL evidence.
- Preserve bound, disclosure, manifest, certificate, and differential evidence.
- Make every backend stop condition executable and reviewable.
- Reject hidden unbounded collections and unconstrained witness callbacks.

## Capabilities

### New Capabilities

- `backend-stop-test`: Reproduce the finite Core-to-Compact feasibility test with
  explicit acceptance and fallback behavior.

### Modified Capabilities

None.

## Impact

This package controls `moriarty/`, `experiments/moriarty-core-swap/`, focused
tests, `wiki/benchmarks.md`, and the goal completion matrix. It does not claim a
general compiler, real proof, ledger deployment, production cost, or audit.
