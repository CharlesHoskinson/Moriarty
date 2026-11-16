# Formal check of every EARS and spec: brief (2026-11-15)

Owner order: "Check all the EARS and specs with Quint and Lean". The owner is AFK ("finish everything with the recommended options"). Take the recommended picks and mark them LEAD PICK. Never change a hard rule.

## Scope

Every EARS requirement in:
- `openspec/changes/sherlock-e2e-prototype-20261115/specs/*/spec.md` (this worktree; slices A, B and C)
- `openspec/changes/sherlock-completion-20261115/` (branch `spec-audit`, worktree ~/Projects/Moriarty-wt/spec-audit; 62 EARS)
- the reused families the e2e specs adopt by reference (RSL, RKY, ANC, LW, ZK, SQG, LPC, DG, ...), where they live in `sherlock-processor-final-20261111` and the other changes.

## Method (software-factory formal pipeline, proportionate)

Use the `formal:formal` metaskill and its sub-skills (`formal:formal-oracle`, `formal:formal-quint-check`, `formal:formal-reference-suite`), plus `~/Projects/software-factory/plugins/formal` and its worked example `examples/token-bucket`. The unit is one **bounded context** from `architecture/DDD.md` (or `bus/ddd-B.md`, `bus/ddd-C.md` until DDD.md is final).

Per bounded context:
1. **Lean oracle** (Opus lane): executable Lean state and transition definitions from the spec text. Each EARS invariant is a theorem, with no `sorry`. The `oracle` executable uses the kink-implementation/1 protocol and a `--step` JSON Lines mode. No Mathlib.
2. **Quint model** (Gemini Carbon lane through the owner's herdr pane, written from the spec and the EARS only, never from the oracle). Each invariant carries its requirement ID. Run the witnesses and mutants: each mutant must break its invariant.
3. **Replay**: the ITF traces of the Quint model go through the oracle's `--step` with `scripts/quint_replay.py` until every transition agrees.
4. **Findings**: every disagreement, violated invariant or contradiction between EARS is a finding. Per the owner rules, every finding is validated by building its counterexample on the real code. The verdict is one of CONFIRMED, NOT REPRODUCED, GUARD HOLDS or OPEN DESIGN, written in `COUNTEREXAMPLES.md`. Every fixed finding gets a shrunk ITF trace plus a named replay test that fails before and passes after.

Every EARS ID gets exactly one row in `formal/sherlock-e2e/COVERAGE-FORMAL.md`, with one of these verdicts:
- **MODELED**: a Quint invariant and a Lean theorem both name the ID, and the replay agrees.
- **CONSTANT-CHECKED**: a numeric or structural bound (k <= 21, VK <= 50,000 B, column bounds, MERGE-WORK 5,400, K_MAX 2). A Lean `decide` or `native_decide` theorem over the recorded MEASURED value, with the source cited. A Quint invariant where the bound constrains state.
- **SPEC-CHECKED**: a process or operations requirement that has no state machine. The Lean or Quint model checks only its consistency with the other requirements (no contradiction), with the reason stated.
- **FINDING**: see point 4.

No blank rows. A requirement is never dropped because it is hard to model.

## Where it runs

- Files: `formal/sherlock-e2e/<context>/` in this worktree (`lean/`, `quint/`, `traces/`, `REPORT.md`). Do not edit `openspec/`. Fable-A owns it. Send any spec text fix as a finding to `research/sherlock-e2e-spec-20261115/bus/formal-findings.md`.
- **CPU:** muninn is starved. Run every `lake build`, `quint run`, `quint verify` (Apalache) and replay on **odin** (`ssh odin`, Lean 4.35.0, quint 0.33.0, java present; 20 cores). Sync with `rsync -a --exclude .lake` to odin `~/sherlock/formal-e2e/`. Odin rules: run under `systemd-run --user --scope -p MemoryMax=24G nice -n 10`; bind to 127.0.0.1; no /mnt/c, wsl.exe, cmd.exe or powershell.exe. The R_SL suite prove and a GUI lane share odin, so never kill another job. huginn (Lean 4.33.1, quint 0.33.0, 24 cores) is the fallback once its suite prove ends.
- **Carbon:** use the single owner agy pane on muninn: `herdr agent list | jq ...` gives pane `w1:p1C`. One lane at a time, only when the pane is idle. Never spawn new panes. Follow the agy skill section 6: absolute paths, `REPORT.md` to a named path. The lead (you) reruns every quint command the lane reports, on odin, before the result counts.

## Rules

- Never commit `qa/sherlock/status.json` or `runs.jsonl`. Stage explicit paths. Docs-only commits use `DW_DOCS_COMMIT=1`. Never use `--no-verify`. T0 is flaky under load, so retry it. NO Co-Authored-By or AI attribution. Never push.
- Commit only after Fable-A's e2e commit has landed on `sherlock-e2e-spec` (check `git log`). Then commit `formal/sherlock-e2e/` on the same branch.
- Wiki: add one log entry per context done in `~/Projects/Moriarty/llm-wiki/wiki/log.md` (ops: qa, lane-done, commit; format `## [YYYY-MM-DD HH:MM] op | title` with 1 to 4 bullets). Lint with `cd ~/Projects/Moriarty/llm-wiki && timeout 60 python3 tools/lint.py </dev/null` until PASS.
