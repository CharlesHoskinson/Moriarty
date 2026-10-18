# Session-completion sprint design

**Goal:** Complete the preserved branch's remaining K and authenticated ledger outcomes with actual, scoped acceptance evidence.

**Architecture:** Use the existing plugin runner, canonical accounting store, K runner, financial consumer, and SDK executor boundary. Admit each live action separately. Prepare independent contracts and tests in parallel. Use one implementation writer at a time.

**Tech Stack:** Python plugin and diagnostic shim, K, TypeScript Source5/Core4, JavaScript ledger SDK, Compact, a static C prover supervisor, systemd, Docker, and Midnight Preview.

## Inputs and authority

The frozen input manifest identifies branch `feat/session-handoff-2026-09-12`, HEAD `056e662254c38936758af9c6f72c019705a1ff09`, and aggregate input digest `c14d06fb296c070bafde03009538cd35de9b6f588397bf5a503536a3b361ae95`. All manifest files and the additional frozen plugin status were read in full. The four actual proposal JSON files were also read in full. The reconciliation records their hashes and every original task, requirement, and scenario identity.

The original lifecycle OpenSpec package, `lifecycle-k-contract.md`, implementation plan, and `afk-loan-executor/spec.md` retain their complete normative requirements. This draft adds sequencing and acceptance gates. It does not replace their detailed interfaces or fault matrices. If a source contract conflicts with a proposal, retain the source contract and document the proposal's rejected alternative.

Apply the seven existing semantic dispositions in deliverables/language-to-ledger-2026-09-12/sprints/roadmap-crosswalk-draft.md, section Mapping defects versus semantic conflicts. Historical source identities remain intact. The expired September 11 block and old four-submission loan plan confer no authority for the full Source5 lifecycle. LL04/LL05 must independently reconcile the actual consumer with the unchanged closed financial schema; if incompatible, freeze the exact versioned change within these existing tasks before dependent implementation. LL07/LL08 require newly reviewed stage grouping, allocation and deadline evidence with all prior charges and relative executor margins preserved. No mapping row grants execution.

## Global constraints

Preserve main and all unrelated preimages, retained worktrees, wallet identities, private exclusions, historical failures, spent attempts, and unresolved liabilities. The working tree need not be clean. Use explicit source paths for staging and reviewed publication. Do not reset or broadly stage user work.

Keep implementation, source audit, live admission, process result, and financial acceptance separate. A blocker records an incomplete sprint. It is not an alternative completion condition. Preparation without live authority can continue when it does not consume the blocked resource.

Resume the retained Grok author for K repair using requested `grok-4.6` high and session `27f1f3c5-957b-469a-b36b-8f5fe36dd1b2`. Use the verified Node Foreman launcher and `--resume`. Under raw/assignments/moriarty-new-completion-loop-2026-09-12.md, obtain separate fresh Claude Opus and GPT-6 Astra medium audits of the same full exact candidate and applicable actual results. Source review does not substitute for result review. Preserve historical reviewer identities. Only actual credit exhaustion permits the recorded Astra-author fallback. Timeout124, provider capacity failure, and a failed max-turns planning attempt do not establish credit exhaustion. Preserve returned model identities. Retain additional independent auditors where the executor contract requires both.

This current routing applies to each future source and actual-result review in LL01–LL09 and the wider roadmap. Older singular-auditor wording is historical; unavailable reviewers do not approve or permit silent substitution.

One implementation writer owns behavioral edits. Independent oracle design, source review, dependency inspection, fault-matrix design, and evidence preparation may proceed concurrently. No preparation worker can approve its own implementation or mint live authority.

## Sprint dependency graph

| Sprint | Outcome | Required predecessor for completion | Independent preparation |
|---|---|---|---|
| LL01 | Preserved baseline and repaired, audited K diagnostic source | Frozen scope and retained author context | Ledger contract and executor inventories |
| LL02 | One admitted diagnostic and evidence-grounded backend correction | LL01 source approval and current one-use admission | LL04/LL05 source analysis |
| LL03 | Actual complete scoped K agreement | LL02 corrected backend and separately reviewed execution resources | LL04/LL05 contract and negative tests |
| LL04 | Authenticated lowerer, custody state, and real SDK caller | LL01 baseline, independently reviewed ledger contract | Begin after LL01 without waiting for K runtime |
| LL05 | Callable executor, durable raw exit, and bounded prover lifetime | LL01 baseline and inherited executor contract | Prepare alongside LL02–LL04 |
| LL06 | Actual lifecycle Compact compilation and source/artifact binding | LL03 and LL04 | Compiler path inspection may start earlier |
| LL07 | Complete Docker lifecycle and independent financial audit | LL05, LL06, current accounting and Docker admission | Funding and evidence checks before dispatch |
| LL08 | Guarded Preview lifecycle and independent finalized-effects audit | LL07 and fresh Preview admission | Non-dispatch planning can start earlier |
| LL09 | Reviewed publication and complete scope reconciliation | LL03 and LL08, all inherited acceptance gates | Maintain identity mapping throughout |

The implementation writer serializes LL04 and LL05 edits when their source paths or authority contracts overlap. This graph permits independent preparation, not simultaneous unreviewed integration.

## LL01–LL03: retained diagnostic and scoped K execution

LL01 repairs the retained diagnostic within the exact current source-candidate manifest: diagnostic, tests, pins, requisites, K package map, ownership, snapshot manifest, host-evidence pointer and bound factual host records. Preserve unchanged snapshot and retained-artifact bytes. Record each authorized correction and its dependent digest refresh in the candidate manifest; audit the complete resulting candidate. Expand the four findings into failing offline tests before repair. Preserve the small preflight plus `os.execve` design and the existing runner's retention and containment.

E04-1 must bind the actual HOME Java classpath and both HOME `.kserver/socket` and default nailgun selection. Preserve HOME. Do not rely on variables ignored by the loader. E04-2 must reproduce actual reversed wrapper PATH prepends and bin-unwrapped prepends, then pin selected helper, Java, LLVM, and kore identities including links and targets. Already mapped LLVM/K loader bytes remain credited. E04-3 must reject missing host evidence and check actual selected executables, Nix query binary, database, ancestors, ownership, and effective nonwrite guarantees. A matching UID or a nonwritable file alone is insufficient. E04-4 separates `stat` target data from `lstat` link data. The audit's 28 comparison mismatches do not prove host drift.

LL02 finalizes exact current source, pointer, pin, ownership, runner, candidate, and debit records before dispatch. The prospective route is SP03.4 action `k-retained-trace106-diagnostic`. It is not callable authority merely because the design names it. C7/D1/G60 permits one prepaid 60-second diagnostic only. The runner timeout is 20 seconds including preflight, grace is 5 seconds, and charge remains 60 seconds. There is no compile, retry, refund, or reusable slack. Bind `-I -B` before shim loading. Refresh host ownership immediately before the dispatch digest is accepted. Resolve the real store through `store.get_db_path`, currently `.git/moriarty-dev/state.sqlite3`.

Before dispatch, disposition the native conditions retained by the latest exact-candidate audits, including Java HOME/direct-loader behavior, nested containment/ptrace compatibility and setup-time headroom. Unobserved conditions remain open; source acceptance is insufficient. C7/D1/G60 authorizes no additional canary, compile or retry.

Run the exact retained macro05 trace106 input, compiled interpreter, and parser from `.worktrees/sp03-expression-k-macro05` with the fixed 8 MiB soft stack and core limit zero. Retain separate wrapper113 and interpreter SIGSEGV observations. Depth5947/Core65536 remains accepted. Depth5948/Core65547 remains INPUT_BOUND. No cap1500, hard-case omission, type erasure, or silent stack increase can close the task. Use the sole diagnostic evidence to repair the backend. Further validation, compilation, and the full K suite require separately reviewed resource authority. If the diagnostic is not admitted, LL02 remains blocked and LL04/LL05 preparation can continue.

LL03 extends the existing `experiments/moriarty-language/formal/k/run.py` financial-lifecycle-v1 path, lifecycle codec/corpus, and K rules. Proposed new rule and fixture paths, subject to exact source review, are `formal/k/financial-lifecycle.k` and `formal/k/fixtures/financial-lifecycle-v1.json` under that package. Do not add another runner. Follow the full reviewed lifecycle K contract, including lexical/semantic ownership and error precedence. Normal input contains schema, request, and financialPreState. Missing-context input remains a distinct schema/request shape. Retain original JSON bytes, duplicate-last-key behavior, and numeric distinctions. Host decoding cannot silently canonicalize these distinctions or perform K's semantic rejection work.

K owns complete state, work, effects, history, and every rejection observation. Preserve U128, signed128, and U64 checks and all static branches. Terminal results are lcPrepared, lcExpressionRejected, lcKernelRejected, or lcAdapterRejected. lcPending is not terminal. Require empty continuation, one terminal result, and immutable input cells. Do not fill absent fields from host expectations, sort away differences, or classify a crash, timeout, stuck state, or codec failure as semantic rejection.

Run each independently identified corpus case, the four lifecycle transitions, and the constructor inventory. Preserve the contract's 38-case set by identities, not a target count. Rerun the relevant expression corpus on the corrected backend. Check exact debt0→100→110→80→0, state identities and unrelated rows, allowance remaining/spent, ordered history, and all financial effects. Reference work totals are 99/65/88/86, prefix 36/10/35/33, kernel 2/1/2/2, suffix 61/54/51/51. From remaining512/spent17/reserve16, cumulative remaining is 413/348/260/174 and spent is 116/181/269/355. Reject independent mutations of principal/interest split with equal total debt, allowance spent, work, cursor, effect metadata, and history order. Full differential agreement establishes this scope only. It does not prove language metatheorems or compiler/ledger correspondence.

## LL04 and LL06: authenticated source and actual compiler

Inspect the existing Source5 frontend/compiler and financial-expression-v4 interfaces under `experiments/moriarty-language/src/successor/`. Implement the reviewed lowerer at proposed `financial-lifecycle-compact-lower.ts` in that directory, or record a reviewed exact existing interface before edits. Proposed companion test path is `experiments/moriarty-language/test/financial-lifecycle-compact-lower.test.ts`. These are planned paths, not claims that the files already exist.

Modify the actual custody generation and consumer path under `experiments/moriarty-midnight-financial/`: `custody/generate.mjs`, `custody/loan.compact`, `custody/bindings.json`, and the reviewed ledger caller selected from `ledger/continue-initialized-loan.mjs`, `ledger/continue-loan-plan.mjs`, and `ledger/integrate-local.mjs`. Record exact functions and source preimages before implementation. The existing atomic/1 fixed loan, with initialized borrower funds and fixed cursor, is not a Source5 lowering.

Freeze source/Core/profile/program/deployment/network/head/revision binding, authenticated PRE/POST state, consent and funding roles, time, liabilities, work, history, and fee constraints before source changes. Bind lender funding authority and debtor consent to exact terms, caps, program, head, and roles. Do not guess blanket lender settlement authority. Persist initialPrincipal, incurred/cap, complete allowances, remaining/spent/reserve, period cursor, terms, identity sets, and ordered history. Reject stale or substituted source, profile, state, head, role, amount, asset, and deployment.

Reuse the existing block-time constraint where valid: the disclosed time has UInt64/horizon bounds and a block interval. Bind authenticated observed time to eligibility and to the same blockTimeGte observation. Mutate the whole block context in tests. Constrain actual transitions in the generated contract. An old contract with a new hash, host-computed boolean, or callback-only validation cannot satisfy this gate. Include native payer UTXO ownership, receive/send effects, independent fees, gross debits, net goals, recipient assets, and change.

LL06 first freezes the actual generated lifecycle build path and compiler argv from reviewed code. The package's existing `run build` uses `ledger/build-proven.mjs`; its presence does not prove that it compiles the new lifecycle path. Inspect `custody/build.mjs` and the bounded build runner as part of this decision. Run the actual compiler under current resource authority. Retain generated Compact bytes, compiler/toolchain identity, argv, stdout/stderr, raw exit, ZKIR/runtime artifacts, and exact source/Core/profile/program/artifact correspondence. Inject a source/artifact substitution that the consumer must reject. A mocked compiler, generic package build, or pre-existing artifact cannot establish this acceptance.

## LL05: inherited executor and prover lifetime

Implement the full frozen `afk-loan-executor/spec.md`, including every classification and fault row. Existing action identity remains `sp05-loan-execute-once`. Proposed module `plugins/moriarty-dev/scripts/moriarty_dev/loan_executor.py` exposes `execute_once(plan, admission, dependencies)` and inert `main(argv=None, dependencies=None)`. Its specified argv grammar uses `--plan`, `--sha256`, and `--admission`. Placeholders in the inherited grammar are not runnable commands. Install the reviewed loan_exit_retention modules through package-relative imports. Do not import historical execute-once.py or start stopped executor06 containers.

Implement the closed `moriarty.loan-executor-plan/1` schema and current authority checks through the real producer and shared validators. Preserve immutable source commitments. Permit only the specified derived runtime financial deadline change through `{runtimeFinancialPlan}`. Use canonical store, master, history, observation, correspondence, debit/reservation, and wallet-ownership evidence. No alternate store, stub authority, executor debit/adoption/reset/funding refresh, or source/runtime hash cycle is allowed. Hold the parent wallet lock for the specified lifetime and retain unresolved ownership. Current context must remain at most 120 seconds old at financial startup.

The actual runner-to-script AF_UNIX channel uses a private0700 directory, socket0600, a 256-bit nonce, parent environment overwrite, and the actual strong launcher. Persist invocation evidence before handshake. Verify authority, nonce, UID, host PID ancestry/startTicks, and one-shot identity. Retain the channel through cleanup. Environment values alone are not authority.

Retain outer+setup120, operation1600, bootstrap1606, collect1618, timer1620, cleanup1740, outer1800, grace5, minimum debit1835, and timerAccuracy at most one second. Use one monotonic/boot identity. Entry at +50 retains 70 seconds of setup. Entry at +120 refuses. Do not compress margins or restore the old 2490/2500/2550 schedule. Preserve the exact bounded show/stop budgets in the inherited specification.

Observe actual systemd Type=exec, RemainAfterExit=yes, Transient=yes, valid InvocationID, MainPID, and cgroup. Preserve the complete loaded-main classification table and rawMainExit exit/signal/unknown with code int/null. Write exclusive nofollow evidence, fsync file and directory, and persist the terminal observation before evidence-driven stop. Wrapper zero cannot erase main failure. Missing main evidence remains unknown.

Containment uses exact container ID/StartedAt and unit InvocationID. Durable observed containment precedes timer cancellation. Success also requires cancellation. Preserve every identity in the inherited twelve-row fault matrix, including failure combinations. Implement the static import-free `prover_lifetime.c` boundary and its seven startup-race rows. It launches only the pinned child, within min(entry+1500, outer+1620), with latest start outer+120. Validate the bounded six-line, readonly, nofollow control file with boot/time-namespace/monotonic/invocation identity. Use a newly created invocation container from the pinned image, private PID namespace, fixed mounts/limits, no restart, and no privileged/host-PID mode. Keep historical stopped containers unchanged. Parent death and post-start receipt loss cannot remove the lifetime bound. Signal/reap the owned group and tear down remaining descendants.

Test the actual static helper with controlled children in isolation. If required namespace support is unavailable, record a blocker instead of claiming a fake pass. Source tests do not start live Docker. Persist `moriarty.loan-process-result/1` with PROCESS_SUCCESS, PROCESS_FAILED, PROCESS_UNKNOWN, or REFUSED, retry false, and financialAcceptance pending. The real runner consumes the contained result and exact invocation digest, never stdout alone. Preserve child/runner mappings and unresolved reservations from the inherited specification. Obtain both required source audits. Source readiness does not admit unit creation, prover startup, Docker, or Preview.

## LL07–LL08: actual Docker, then actual Preview

Refresh existing accounting, bindings, observations, funding, reservations, and wallet ownership as prerequisites within the affected sprint. The observed status has stale SP01 binding/candidate input, missing current-accounting.json, and unavailable resource live state. These are present blockers, not a reason to build another accounting framework or reset spent capacity.

Docker must deploy and initialize, then execute originate, accrue, partial repayment, and settlement. The old deploy/initialize/accrue/settle path is insufficient. Derive transaction identities and grouping from the actual reviewed lifecycle, not a guessed transaction count. Retain canonical native state at every stage, full liabilities/allowances/work/history, fee-inclusive movements, and a rejected invalid transaction with unchanged canonical state. Retain actual raw main exit, containment, and independent financial audit as distinct artifacts. Process zero alone is insufficient.

Only after successful Docker audit obtain current bounded Preview admission through the guarded plugin. Preview is the sole public target. Preserve existing identities, state, spent attempts, and keys. Require current funding and capacity evidence, not an old receipt or source approval. Execute the actual lifecycle, retain transaction bytes and IDs/status, reconcile notifications against canonical finality, and audit complete finalized financial effects with fee debits, asset identity, recipient movement, change, debt, allowances, work, and history. Indexer success alone is insufficient. Missing or uncontained evidence remains blocked or unresolved. It does not trigger a retry or fresh identity.

## LL09: publication and retained program scope

Use the reconciliation identity map to assign current evidence or a precise open blocker to every original obligation. A useful blocker report can be published while the affected sprint remains incomplete. The September 12 full-branch execution instruction requires all six original branch outcomes and the entire retained roadmap before branch or active-loop completion. LL01–LL09 are the language-to-ledger subset. Continue through the original OpenSpec dependencies for every wider obligation; an open blocker cannot satisfy the terminal condition. Preserve historical goals in other threads.

Preserve wider ContractInvariant, IntentRefinement, TransitionValidity, HistoryCompliance, full semantics and correspondence, RP01–RP03, MC01–MC08, and SP01–SP12. Preserve ACTUS field/type/taxonomy dispositions and all DeFi rows and correction lessons. Retain private handoff and composition operators. Current ROADMAP's September11 ledger-anchored PCD route supersedes the older fixed-instance/F2-before-F3 route in the September7 report. Keep Stage0 and F3 E2 k≤17/E1 Preview distinct from ledger10 k18–19 certificate feasibility and E3/E5 native segment lengths. Native recursive proofs are not a new prerequisite for the direct ledger route. The ledger verifies the accumulator rather than adding final pairing inside the circuit.

Historical local tests, the prior loan's missing raw exit, a later swap exit zero, exhausted public attempts, and controller/launcher interruption remain unchanged facts. Publication excludes private originals and credentials. Archive or complete no sprint until its deterministic and result gates actually pass.

## Verification commands and limitations

The following commands are present in inspected source manifests or actual CLI discovery. Their presence is not a test result and does not confer resource admission:

```sh
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
python3 .moriarty-dev/test_k_macro05_trace106_diagnostic.py
npm --prefix experiments/moriarty-language test
npm --prefix experiments/moriarty-language run typecheck
npm --prefix experiments/moriarty-language run loan-lifecycle-demo
npm --prefix experiments/moriarty-midnight-financial run test:ledger
npm --prefix experiments/moriarty-midnight-financial run test:compiled
npm --prefix experiments/moriarty-midnight-financial run test:compiled-rejection
npm --prefix experiments/moriarty-midnight-financial run test:finalized-state
npm --prefix experiments/moriarty-midnight-financial run test:preview
openspec validate language-to-ledger-lifecycle --strict --no-interactive
```

The K, compiler, Docker, and Preview dispatch argv must come from the actual reviewed candidate and finalized admission record. This draft does not invent flags or action names. Before each behavioral implementation sprint, expand its exact functions and failing tests against the observed source preimages. Freeze any proposed filename change in the sprint record and preserve the original obligation mapping.

Only draft validation and planning consistency checks are run for this planning task. The reconciliation states their exact results. No behavioral test, K run, compiler run, Docker action, Preview action, financial acceptance, or publication is claimed here.

## Supporting research and tooling recovery

The [model documentation graph](../../../deliverables/model-docs-2026-09-12/graphify-out/GRAPH_REPORT.md) covers 20 official sources, including the later Grok card intake. Provider outputs retain the exact original frozen session digest; the later intake is supplemental evidence, not retroactively part of their shared prompt. The [plugin recovery design](../../../deliverables/hook-input-repair-2026-09-12/durable-upgrade-design.md) is a separate authorized reliability repair. Its source and host checks do not consume or confer financial execution authority.
