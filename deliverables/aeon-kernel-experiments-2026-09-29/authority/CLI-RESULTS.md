# Independent Aeon CLI reproduction

Date: 2026-09-29. Each row was run in a fresh `python -m aeon` process from the Moriarty worktree root. Exact command template:

```bash
/home/charl/.graphify/repos/alcides/aeon/.venv/bin/python -m aeon -n --strict-decidable deliverables/aeon-kernel-experiments-2026-09-29/authority/NAME.ae
```

| Source `NAME.ae` | Exit code | Captured stdout/stderr | Observation |
| --- | ---: | --- | --- |
| `minimal_false` | 0 | `minimal_false.cli.log` (empty) | An impossible `false` refinement was accepted. |
| `minimal_breach` | 0 | `minimal_breach.cli.log` (empty) | `101` was accepted against `v = 101 && v <= 100`. |
| `minimal_cap` | 13 | `minimal_cap.cli.log` | `101` was rejected against `v <= 100`. |
| `gross_cap_violation` | 0 | `gross_cap_violation.cli.log` (empty) | The combined gross equality/cap refinement accepted 101. |
| `gross_cap_violation_split` | 13 | `gross_cap_violation_split.cli.log` | A separate cap obligation rejected the same gross amount. |

The CLI printed no output on exit code 0. The first attempt to capture logs used the Aeon checkout as the working directory and failed at shell redirection before Aeon ran. The rows above are the subsequent successful invocations from the Moriarty worktree.
