# A2 tasks

Partial progress: Task1 literal fixtures/tests accepted at `35959ae`, with
compiling RED/GREEN, nonauthor source approval and root receipt/archive audit.
Task2 deterministic authority routes are accepted at `955f56b`: seven tests and
recursive typecheck pass, with original compiling RED and independent source/
receipt admission. Task3 adversarial controls and action harness are dispatched;
shared final regressions and the complete A2 package remain open.

- [ ] Verify the exact dependencies and source pins.
- [ ] Read XML phase A2 and its named scenario inventory.
- [x] Complete its concrete implementation plan before behavioral edits (A1 `82d2c0b`).
- [ ] Add the specified behavioral tests before implementation.
- [ ] Retain compiling RED sources and terminal receipts.
- [ ] Produce the exact outputs listed in README.md.
- [ ] Execute each positive and failure scenario.
- [ ] Preserve complete source closures and raw results.
- [ ] Write `evidence/s02-candidate-a-completion/a2/manifest.json`.
- [ ] Recompute `evidence/s02-candidate-a-completion/a2/validation.json` from evidence.
- [ ] Obtain the required independent review.
- [ ] Commit only the admitted package files.

## Specification structure check

```bash
openspec validate s02-candidate-a-a2 --strict --no-interactive
```

This command validates specification structure only.
Use the XML commands for existing boundary runtime checks.
Derive exact new runtime commands in the phase implementation plan.
Do not mark runtime tasks complete from this command.
