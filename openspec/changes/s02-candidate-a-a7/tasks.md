# A7 tasks

All implementation and acceptance tasks remain unchecked in this new contract.
Existing work must receive evidence intake, not duplicate implementation.

- [ ] Verify the exact dependencies and source pins.
- [ ] Read XML phase A7 and its named scenario inventory.
- [ ] Complete its concrete implementation plan before behavioral edits.
- [ ] Add the specified behavioral tests before implementation.
- [ ] Retain compiling RED sources and terminal receipts.
- [ ] Produce the exact outputs listed in README.md.
- [ ] Execute each positive and failure scenario.
- [ ] Preserve complete source closures and raw results.
- [ ] Write `evidence/s02-candidate-a-completion/a7/manifest.json`.
- [ ] Recompute `evidence/s02-candidate-a-completion/a7/validation.json` from evidence.
- [ ] Obtain the required independent review.
- [ ] Commit only the admitted package files.

## Specification structure check

```bash
openspec validate s02-candidate-a-a7 --strict --no-interactive
```

This command validates specification structure only.
Use the XML commands for existing boundary runtime checks.
Derive exact new runtime commands in the phase implementation plan.
Do not mark runtime tasks complete from this command.
