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

- [x] Bind `wiki/benchmarks.md` to commit `006c4d91ed09c0a89261861b6e7203b3efa3e2df`.
- [x] Bind the goal matrix to the four evidence digests in the scope snapshot.
- [x] Reproduce the complete package from a fresh pinned environment.
- [x] Write `evidence/wp01/reproduction-receipt.json`.
- [x] Write `evidence/wp01/current-checkout-validation.json`.
- [x] Write `evidence/wp01/evidence-manifest.json`.
- [x] Run Moriarty's manifest-versus-generated disclosure validator.
- [x] Compare compiler-reported versions, circuits, arguments, witnesses, and
  ledger fields with the artifact manifest.

## 4. Apply the gate

- [x] Compare fresh outputs with the preserved artifact digests.
- [ ] Re-run WP01 after WP04 freezes the Core scope.
- [x] Run `uv run python scripts/validate_sprint_evidence.py --package WP01 --manifest openspec/work-packages.json`.
- [ ] Record library-only or stop if any acceptance predicate fails.
