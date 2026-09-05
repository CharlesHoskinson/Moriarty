# Current execution progress

## 2026-09-05 current override

This block supersedes obsolete Foreman/type-approval instructions below.
Foreman development is closed by the user. The three requested formal-design
proposals are preserved in main cba9e72; the adopted decision preserves dissent.
No further human type approval is required. Council gate reviews remain open.
Observation carrier: complete as a structural unit (941a6fb..fd40bf9), with
independent review corrections in 45883ba and clean narrow re-review. No S02 gate.
Recovery registry v2: committed 7cccc5a, independent transcription review clean.
Branch policies/plan views: committed 320dc53, independent Astra review clean;
72 deterministic Quint tests with receipts, not lifecycle/recovery execution.
Persistent signing: implemented at cbd1f1d, corrected at f2941d4; independent
Astra review and narrow re-review clean. Current receipt selects 41 tests and
1,000 sampled signing traces. This is symbolic signing, not execution verification.
Next: full per-operation evidence verification, atomic parent updates, and actual
recovery, under docs/superpowers/plans/2026-09-05-moriarty-s02-authority-lifecycle.md.
Do not rebuild the completed policy unit or re-run its design vote.
Do not re-dispatch the finished three-expert design or observation implementation.

## Earlier progress (historical)

S01 is COMPLETE and integrated into main at 96782de. Independent whole-package
review approved code, the settlement-process correction, provenance, and status
handoff. Main checkpoint af3abfe closes obligation 1 and opens S02 obligation 2.
Main verification: 282 tests and ten local S01 gates passed; typed checkpoint
measurements 7/8 carry freshness. No later XML release gate passed.

Current worktree: /home/charl/Moriarty/.worktrees/s01-audit-start (reused isolation).
Current branch: s02-model-comparison; S02 base af3abfe.
S02 contract plan: docs/superpowers/plans/2026-09-04-moriarty-s02-package-contract.md,
independently approved at d838b0c. Task 1 registry is complete (af3abfe..e9b4f80,
independent spec and quality review approved); Task 2 OpenSpec is complete
(9f9dd12..02aba01), independent spec/quality and whole-contract review approved.
The common observation/authorization design is independently approved after
pre-sign check and recovery-subscenario corrections (d742a5b; status9665059).
The effect foundation plan is committed at aea316c. Its implementation is at
d069800. Independent review found no source defect but required corrected
runtime/provenance and incremental-validation evidence. Correction 1bd4bff records
fresh receipts and explicitly retrospective process reconstruction. Original raw
process output was unavailable. Do not mark this task complete yet.
The controller independently ran ten Quint tests and a 100-sample safety run
with both deposit/refund witnesses at 100/100. This is a non-candidate harness.
The consumption foundation plan at 9450d15 was implemented at 8b90511.
Sixteen Quint tests, all three typechecks, sampled witnesses, and two complete
cancellation paths are recorded. The controller reran the tests and regression
checks. This remains an unintegrated bookkeeping candidate, not a gate result.
The full candidate whitespace check detects trailing spaces in raw REPL output;
those bytes are preserved and the scope limitation is documented in the report.
The user added mandatory GPT-6 Astra/Grok 4.6/Fable 5.1 council review for gates.
Main 6e83f56 records the policy and S01 backfill queue in docs/COUNCIL_REVIEWS.md.
Foreman runtime verification and soft setup passed. Exact council canaries and
review quorum remain pending. The parallel Foreman skill update was committed
at 244e7c0 and 45121c7. Main b0141f9 records actual model routing probes and
a missing post-preflight review-binding carrier in the Council runtime.
User direction is pending on that runtime change and CLI identity policy.
Do not confuse setup probes or the Foreman skill update with Council approval.
After these foundations, build the full authority interface and four distinct
candidate representations, then correspondence/mutations/Quint Apalache checking
and selection. No candidate model or S02 gate has passed.
Runtime goal stays active for the full XML S01-S15 request. User directed Quint
instead of TLC; follow the installed quint-co/quint-llm-kit skills.

## Historical S01 progress (superseded by completion above)

Base: 40f1e5a40ac9. Branch: s01-audit-start.
Requested outcome: execute the full v1.3 prompt, maintain the runtime goal loop,
and commit/integrate task work locally. Runtime goal is active; check get_goal.

- Audit complete: d87fd42 and 0988006. See docs/superpowers/reviews/2026-09-04-moriarty-v1.3-execution-audit.md.
- Task 1: complete (b624c08 and f8c3aa8, independent spec and quality review approved).
- Audit resolutions: complete (b8acd58, 5e01ff7; independent review approved).
- Task 2: complete (21709e6, 3b485f0; independent spec and quality re-review clean). Lifecycle metadata, exact gate maps, and manifest role fixes included.
- Task 3: complete (ba972dd; independent spec and quality review approved). Candidate frozen, still unmechanized.
- Task 4: complete (5baada8 and f41b7d3, independent spec and quality review approved). Strict input parsing and no signing authority.
- Task 5: complete (9b89753, 7bec08c; controller independent spec+quality re-review approved). Public API closes receipts, references fail as ValidationError, no external reference retrieval. Report task-5-report.md records97focused/278full. Normative20pins independently matched ae5aa5c. Final S01 review remains required.
- Tasks 6 and 7: open. S01 completion requires actual validator and full verification evidence; S02 follows that gate.

Protected inputs: prompt 1.3, approved S01 design, Core/swap/backend, E00 artifacts,
and semantic snapshot 0.0.0-e00.2 remain unchanged in branch commits.

Environment note: /home/charl/Moriarty/.venv/bin/python supplies dependencies.
An ignored graphs symlink provides pre-existing graph fixtures. Commit 7bacd77
isolates graph-test output in temporary files and tests preservation of the
historical receipt. Main infrastructure is merged into this worktree at e7cffaa.

Final initial-slice review: approved at ece7534. Full suite: 151 passed in 1.20s.
The generated graph timestamp was restored to the branch base after verification.
The original checkout's earlier baseline test refreshed its existing timestamp.
This disclosed timestamp was committed at 34cf5d9 with the execution record,
checkpoint export, and DB ignore rules after the user requested all task commits.
Previous test counts are historical; use the typed checkpoint freshness result.

Main preparation: reviewed S02 design committed at 40f02f1 in main only, followed by checkpoint79f79a4.
Merge main into this worktree after Task5 implementation/review to include it.
Do not select an architecture or pass S02 from this design alone.
Task6/7 briefs and combined corrections are ready in this directory.
Runtime goal remains active. The user was asked asynchronously for existing
human pilot teams/results; no response is required for current independent work.
New user direction: Quint via quint-co/quint-llm-kit, not TLC. S02 design in
main now specifies .qnt models and Quint CLI with Apalache for model checking.
