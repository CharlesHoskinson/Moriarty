# Factory heavy-work CLI excerpt
> Source: file:///home/charl/code/Moriarty-perf/models/doctorwatson/factory/CONTRACT.md
> Collected: 2026-10-26
> Published: Unknown

90: 
91: | Command | Behavior | Exit |
92: |---|---|---|
93: | `next [--json]` | Print counts per set, then `FIRST FAILING: <id> — <title>` and `CHECK: <command>`. Reads `status.json` only | 0 |
94: | `check [ID…] [--set S] [--tier T] [--hypothesis TEXT]` | Run the selected checks, sending heavy ones through `heavy`. Update `status.json` and append to `runs.jsonl`. Print one line per predicate | 0 all pass, 1 any fail |
95: | `preflight [--json]` | Disk free on the repo filesystem ≥ 150 GiB (`DW_MIN_FREE_GIB`). 1-minute load < `os.cpu_count()`. No other process holding the heavy lock. Print the actual numbers | 0 ok, 3 unfit |
96: | `heavy --id ID [--budget KEY] [--hypothesis TEXT] -- CMD…` | Take an exclusive `fcntl.flock` on `~/.cache/doctorwatson/heavy.lock`, run preflight, then measure-then-cap (wall time and peak RSS of the child via `os.wait4`). Write the full trace. **Retry rule:** if the last run of this ID failed with the same `cmd` and the same `head`, `--hypothesis` is required (exit 5 without it). Every retry is logged. Nothing is ever "consumed" | the child's exit, 3 unfit, 4 budget, 5 hypothesis required |
97: | `stop-rule [--json]` | If `iterationsSinceFlip` ≥ 3, or more than 24 h have passed since `lastFlip`, print `STOP: escalate to owner` with the last failing run's trace path | 0 continue, 6 stop |
98: | `holdout verify` | Recompute sha256 for every file under `qa/holdout/` except `MANIFEST.sha256`, and compare with the manifest | 0 ok, 7 mismatch or unlisted file |
