# Session roadmap: language to Midnight

This feature branch is a work-in-progress snapshot for a fresh context. It is not a reviewed release or a request to merge unfinished changes into main. The user requested preservation and a session restart on September 12, 2026.

## Start here

1. Load `moriarty-dev:develop` as required by AGENTS.md and run the repository plugin CLI `status`. Read this roadmap and the current `openspec/changes/language-to-ledger-lifecycle/tasks.md` before following older queues.
2. Recover the typed checkpoint with `/home/charl/foreman/skills/foreman/runtime/dist/fm-session.js recover --json`; redirect output to a file before parsing. Recheck perishable measurements. `.foreman/session.ndjson` is the tracked checkpoint export; the SQLite store remains local.
3. Inspect git status and actual process state. No implementation worker is intentionally left running for this handoff. The native goal was observed blocked after a controller capacity error; a saved goal or roadmap does not establish a running loop. Preserve that goal and its full scope; do not replace it with a smaller goal or mark it complete.
4. Preserve the unreviewed user/plugin/spec/wiki changes on this branch. Base main was `cd240b8e4fa1f5b34d5b334f406caf90ea561dc4`, including the user's latest DeFi decision correction. Existing worktrees remain on disk; see `deliverables/session-handoff-2026-09-12/retained-worktrees.json`.

## Completed and merged language work

- [x] PR1: computed funded repayment through the simulation CLI.
- [x] PR2: source-defined agreement schemas.
- [x] PR3: multiple named actions and explicit selection.
- [x] PR4: typed financial PRE reads and computed remaining repayment.
- [x] PR5: six financial POST reads in ensures; failed postconditions publish no changes.
- [x] PR6: funded origination and explicit rounded accrual, with duplicate-period protection.
- [x] PR7: complete source lifecycle, originate → accrue → partial repayment → settlement, with retained liabilities/history and cumulative work. README syntax, semantics and commands were updated.

PR5/6/7 merge commits are `1e8bf397546ae1a5f373cb50492f9d10b6e5f8f5`, `a3ada9d9f4c7b6bf41b5503a2eb1d308ba55cbdd`, and `81bed862c60b184e7f5fdb27577157c2bcf39530`. Exact audits and merge receipts are under `deliverables/language-to-ledger-2026-09-12/{postconditions,origination,lifecycle}/`. The PR7 candidate recorded 883 passing tests; that is historical candidate evidence, not a current run on this WIP branch.

## Next delivery: scoped K/evaluator agreement

- [ ] Finish the retained K crash diagnostic source repair and obtain a fresh exact-source Astra audit.
- [ ] Finalize and independently validate the prospective one-use admission records, then execute the sole guarded diagnostic if admitted.
- [ ] Resolve the native failure without narrowing accepted language bounds.
- [ ] Implement the reviewed lifecycle K contract and complete positive/negative differential corpus, with actual K results and independent full-result audit.

The latest source audit is `deliverables/language-to-ledger-2026-09-12/k-admission/audit-exec-shim-04.json` and `.md`: **BLOCKED_PENDING_SAME_AUTHOR_REPAIR**. Its six-file candidate is `7c0c4c7abe5b784f0c4c484db9979811e96bc4f767a8614e498c46241e4dcfae`; snapshot in `root-exec-shim-freeze-04.json`. Read all four findings together: HOME-based Java/nailgun loading, executable/PATH commitments, ownership/database checks, and stat/lstat symlink handling. Existing tests and harmless strong-containment preflight passed, but source/dispatch/result approval remains false. No actual native K diagnostic has run in this iteration.

The approved changed approach uses a small preflight plus `os.execve` shim and the existing plugin runner for output retention and containment. Do not rebuild the rejected duplicate process collector or add another orchestration framework. Root actual-runner/temporary-SQLite tests are in `k-admission/root-runner-consumer-probes-03.py/json`. The real store path is obtained through `store.get_db_path` and currently resolves to `.git/moriarty-dev/state.sqlite3`.

Same Grok author session: `27f1f3c5-957b-469a-b36b-8f5fe36dd1b2`. Use requested `grok-4.6` high, `--resume` and the verified Node Foreman launcher, not the legacy native launcher. The last author invocation timed out with124 and returned no valid provider result. No credit exhaustion was established. User permits Astra authoring if Grok actually exhausts credits; use a separate fresh Astra medium audit. Preserve prior failures and returned identities. Old Opus routes are superseded for this task.

The copied unfinished K files are listed with hashes in `deliverables/session-handoff-2026-09-12/active-k-files.json`; their original worktree is `.worktrees/lifecycle-k`. They include the reviewed K contract, reused expression inputs and partial codec/corpus/tests. They are not a completed K implementation. Independent contracts and expectations are under `design-review/`, `k/root-complete-oracle-01.json` and `k/independent-constructor-inventory-01.json`. Additional Source/Core boundary probes are local evaluator evidence only.

The retained failure uses macro05 trace106, parser and compiled artifacts in `.worktrees/sp03-expression-k-macro05`, at the fixed8MiB soft stack. Keep depth5947/Core65536 accepted and depth5948/Core65547 rejected. Existing wrapper113 and interpreter SIGSEGV are different observations. Do not skip the hard case, erase types or raise stack limits without the recorded review.

Prospective resource approval is C7/D1/G60: one60-second prepaid diagnostic, no compiles, retries, refund or reusable slack. Latest routing uses SP03.4 and `k-retained-trace106-diagnostic`. Use corrected routing02 plus adopted exec-shim design03, not earlier invalid drafts. Final runner timeout is20 including preflight, grace5, full charge60. All current record drafts still require exact source/digest/debit finalization; placeholders are not authority. No live diagnostic campaign/accounting adoption has occurred. Old exhausted budgets and unresolved liabilities remain preserved. Source review is not dispatch approval.

## Remaining Midnight delivery

- [ ] Review and implement authenticated source-to-Compact translation: exact source/profile/program/head, state, permissions, time and complete financial effects.
- [ ] Complete the already specified SDK executor/lifetime boundary and meaningful failure checks.
- [ ] Compile the generated lifecycle Compact contract and bind actual compiler artifacts to the reviewed source.
- [ ] Reconcile current accounting/funding/admission, then run the full lifecycle on Docker with complete native readback, rejection control, raw exit and containment.
- [ ] Obtain independent Docker result audit and current bounded Preview admission.
- [ ] Execute on Midnight Preview through the guarded plugin, report actual transaction IDs/status and audit complete finalized financial effects.
- [ ] Reconcile final roadmap/publication scope; retain remaining proof, correspondence, ACTUS/DeFi and full MC/SP gates.

Use `docs/superpowers/plans/2026-09-12-language-to-ledger.md`, the OpenSpec package, and `design-review/ledger-seam-observations.md` for exact interfaces. Existing fixed Compact loan code is not a lowering of the new Source5 lifecycle. Local tests, old Preview receipts and host-computed flags cannot satisfy the new execution path. Preview is the sole public target; preserve identities, keys and state.

## Overnight interruption and preservation limits

The controller ended at07:11:43UTC with `server_overloaded` (selected model at capacity), and the native goal's blocked timestamp matches. Grok later reached its1200-second timeout plus10-second grace at07:20:26UTC. The next recorded controller turn began14:32:17UTC after the user's message. See `deliverables/language-to-ledger-2026-09-12/overnight-stall-2026-09-12.json`. Do not claim unattended recovery has been verified.

Main-checkout uncommitted source/docs/evidence and the active K session files are preserved on this branch. Other historical worktrees are retained locally, not merged into this snapshot. Ignored build products, wallet state, private provider output and credentials are not GitHub artifacts. `deliverables/session-handoff-2026-09-12/publication-exclusions.json` records excluded originals, hashes and private backup paths. Credential-shaped graph captures have explicitly redacted compressed copies; those are not interchangeable with the original audited bytes. The235MB original graph remains local because it contains credential-shaped data and exceeds GitHub's file limit. No original capture was deleted.
