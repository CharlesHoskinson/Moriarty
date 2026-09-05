# A3 tasks

Partial progress: Task1 literal fixtures/tests accepted at `3f440d2`, with
compiling RED/GREEN, nonauthor source approval and root receipt/archive audit.
Task2 ordinary authority routes are accepted at `c3c89fa`: five tests and
recursive typecheck pass, with original compiling RED and exact evidence audit.
Task3 stale/adversarial/action-witness work is dispatched; full A3 stays open.

- [ ] Verify the exact dependencies and source pins.
- [ ] Read XML phase A3 and its named scenario inventory.
- [x] Complete its concrete implementation plan before behavioral edits (A1 `82d2c0b`).
- [ ] Add the specified behavioral tests before implementation.
- [ ] Retain compiling RED sources and terminal receipts.
- [ ] Produce the exact outputs listed in README.md.
- [ ] Execute each positive and failure scenario.
- [ ] Preserve complete source closures and raw results.
- [ ] Write `evidence/s02-candidate-a-completion/a3/manifest.json`.
- [ ] Recompute `evidence/s02-candidate-a-completion/a3/validation.json` from evidence.
- [ ] Obtain the required independent review.
- [ ] Commit only the admitted package files.

## Specification structure check

```bash
openspec validate s02-candidate-a-a3 --strict --no-interactive
```

This command validates specification structure only.
Use the XML commands for existing boundary runtime checks.
Derive exact new runtime commands in the phase implementation plan.
Do not mark runtime tasks complete from this command.
