# Tasks: WP01 backend stop test

## 1. Freeze inputs

- [x] Pin the Core source and toolchain tuple in the artifact manifest.
- [x] Preserve the negative disclosure fixture and its compiler result.

## 2. Verify behavior

- [x] Run `uv run pytest tests/test_core_semantics.py tests/test_swap_bounds.py`.
- [x] Run `uv run pytest tests/test_compact_lowering.py tests/test_translation_certificate.py`.
- [x] Generate and compare at least 1,000 deterministic traces.
- [x] Compile the positive Compact artifact and four ZKIR circuits.
- [x] Confirm the undeclared-disclosure compilation exits nonzero.

## 3. Reconcile evidence

- [x] Update `wiki/benchmarks.md` with bounded claims.
- [x] Update `evidence/moriarty-goal-completion-matrix-2026-09-03.csv`.
- [ ] Reproduce the complete package from a fresh pinned environment.

## 4. Apply the gate

- [ ] Compare fresh outputs with the preserved artifact digests.
- [ ] Record library-only or stop if any acceptance predicate fails.
