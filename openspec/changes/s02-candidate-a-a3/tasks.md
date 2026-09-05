# A3 tasks

Local acceptance: aggregate `28d35d8` binds final source, all three task archives,
requirement mappings and shared final regressions. Task3 `926b350` passes17tests
and100samples with all24witnesses positive.
Both profiles are covered. These are finite observations, not model checking.
Cross-provider Council and integration remain open under A6/A7.

- [x] Verify the exact dependencies and source pins.
- [x] Read XML phase A3 and its named scenario inventory.
- [x] Complete its concrete implementation plan before behavioral edits (A1 `82d2c0b`).
- [x] Add the specified behavioral tests before implementation.
- [x] Retain compiling RED sources and terminal receipts.
- [x] Produce the exact outputs listed in README.md.
- [x] Execute each positive and failure scenario.
- [x] Preserve complete source closures and raw results.
- [x] Write `evidence/s02-candidate-a-completion/a3/manifest.json`.
- [x] Recompute `evidence/s02-candidate-a-completion/a3/validation.json` from evidence.
- [x] Obtain native non-author source review and independent root evidence intake.
- [ ] Obtain required cross-provider Council coverage (A6; native review is not a substitute).
- [x] Commit only the locally admitted package files.

## Specification structure check

```bash
openspec validate s02-candidate-a-a3 --strict --no-interactive
```

This command validates specification structure only. Runtime commands and
terminal receipts are retained in the phase and shared-final evidence manifests
on s02-model-comparison. No XML release gate is accepted by this local checklist.
