# factory

DoctorWatson factory core. Interfaces are frozen in [CONTRACT.md](CONTRACT.md).

```
scripts/sherlock-next                                  # counts + first failing predicate
scripts/doctorwatson check [ID…] [--set S] [--tier T] [--hypothesis TEXT]
scripts/doctorwatson heavy --id ID [--budget KEY] [--hypothesis TEXT] -- CMD…
scripts/doctorwatson preflight [--json]
scripts/doctorwatson stop-rule [--json]
scripts/doctorwatson holdout verify | seal             # seal needs DW_ROLE=evaluator
scripts/doctorwatson validate                          # schema-check predicates.yaml
```

Notes:
- `--tier T` selects predicates whose tier is exactly T.
- A predicate whose dependency fails prints `BLOCKED-BY <id>` and counts as failing,
  unless it is named on the command line.
- `heavy` records a budget measurement only for a run that exits 0.
- Exit 3 (unfit) and 5 (hypothesis required) mean nothing ran; they are not logged and
  leave `status.json` untouched.
- Test knobs: `DW_REPO`, `DW_LOCK`, `DW_SKIP_PREFLIGHT=1` (skips disk and load, not the
  lock), `DW_MIN_FREE_GIB`, `DW_MAX_LOAD`.

Tests: `cd models/doctorwatson && python3 -m unittest discover -s factory/tests -t . -v`
