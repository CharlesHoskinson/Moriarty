# A0 tasks

Local A0 intake accepted at source d14cfea and evidence 0190cb9/95899b3.
The existing boundary was reviewed and reproduced, not implemented twice.
Evidence lives on s02-model-comparison; this main-branch status is not integration.

- [x] Verify the exact dependencies and source pins.
- [x] Read XML phase A0 and its named scenario inventory.
- [x] Inspect the adopted boundary implementation plan and completed source.
- [x] Inspect existing behavioral RED/GREEN tests; distinguish supplemental tests.
- [x] Retain compiling RED sources and terminal receipts, with historical gaps disclosed.
- [x] Produce the exact outputs listed in README.md.
- [x] Execute all forty boundary tests and every action/profile witness.
- [x] Preserve complete Quint source closures and raw results; disclose the old Python closure gap.
- [x] Write `evidence/s02-candidate-a-completion/a0/manifest.json`.
- [x] Recompute `evidence/s02-candidate-a-completion/a0/validation.json` from evidence.
- [x] Obtain independent native source/spec and coordinator runtime/archive review.
- [x] Commit only the admitted package files.

Council remains open and is not represented by the native review checkbox.

## Specification structure check

```bash
openspec validate s02-candidate-a-a0 --strict --no-interactive
```

This command validates specification structure only.
Use the XML commands for existing boundary runtime checks.
Derive exact new runtime commands in the phase implementation plan.
Do not mark runtime tasks complete from this command.
