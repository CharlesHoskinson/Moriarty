# A1 tasks

Local planning acceptance is recorded at `82d2c0b` on `s02-model-comparison`.
The generic behavioral rows below belong to A2/A3 execution and remain open;
they are not prerequisites falsely claimed complete by static plan adoption.

- [x] Verify the exact dependencies and source pins.
- [x] Read XML phase A1 and its named scenario inventory.
- [x] Complete its concrete implementation plan before behavioral edits.
- [ ] Add the specified behavioral tests before implementation.
- [ ] Retain compiling RED sources and terminal receipts.
- [x] Produce the exact planning outputs listed in README.md.
- [ ] Execute each positive and failure scenario.
- [x] Preserve complete plan-assembly closures and raw static results.
- [x] Write `evidence/s02-candidate-a-completion/a1/manifest.json`.
- [x] Recompute `evidence/s02-candidate-a-completion/a1/validation.json` from evidence.
- [x] Obtain the required local independent plan review; Council remains A6.
- [x] Commit only the admitted package files.

## Specification structure check

```bash
openspec validate s02-candidate-a-a1 --strict --no-interactive
```

This command validates specification structure only.
Use the XML commands for existing boundary runtime checks.
Derive exact new runtime commands in the phase implementation plan.
Do not mark runtime tasks complete from this command.
