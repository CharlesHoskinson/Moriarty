# Implementer <-> evaluator loop conventions

Source: workflow retro 2026-10-25 (sherlock-u / sherlock-eval on factory-u).

1. **Contract first.** Implementer commits `qa/holdout/contracts/<ID>.json`, runs
   `scripts/doctorwatson contract-check <ID>` (exit 0), then posts STATUS. Output changes edit the
   contract in the same commit. Schema: qa/holdout/contracts/README.md.
2. **Not-implemented, exit 1.** The evaluator authors holdouts from the spec and contract before the
   entrypoint exists. A scenario whose entrypoint is missing or a stub FAILS with exit 1
   ("not-implemented"), never exit 2. Exit 2 is reserved for a broken harness/case. A case therefore
   starts runnable and every implementer commit can move it. Do not park scenarios as `status=unimplemented`.
3. **Auto-watch.** `scripts/evaluator-watch --worktree <factory-u> --interval 120` prints a TRIGGER line
   for each new commit touching crates/ or the contracts dir, and for bus STATUS/CONTRACT events.
   Run it under Monitor and resume the evaluator on each line. Target latency under 5 minutes.
4. **Factory state is not dirt.** `scripts/doctorwatson clean` ignores qa/sherlock/{status.json,runs.jsonl};
   queue scripts use it instead of `git status --porcelain`. `scripts/doctorwatson state-commit`
   commits only those two files as `chore(factory-state)` (no attribution lines, no pushes).
   Heavy.py has no clean-tree check of its own, so no heavy.py edit was needed.
5. **Holdout scans** key off the table's own `*_fragments` lists, so adding a list cannot break U023.
6. **Decision fast path.** Panel disagreement: apply the conservative option at once, post a dated
   owner question in the digest, and keep working elsewhere. Owner has granted Sherlock design
   authority (change Sherlock design and the Moriarty language freely; ledger 9 stays fixed; sync docs
   and chip animation), so an open opcode/interface question inside Sherlock design (e.g. interfaces.md:308
   Await/Commit) is decided by the coordinator on the Opus recommendation, recorded in the wiki,
   without waiting for the owner. Ledger-9, key custody and funds questions still go to the owner.
