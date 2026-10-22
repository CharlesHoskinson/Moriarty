# DoctorWatson cycle finalization requirements

Status: proposed requirements over the existing DoctorWatson consumer and the bounded finalization helper. Implementation, tests, fresh independent review, merge and actual operation observations are separate evidence. The original native implementation loop remains paused. These requirements neither grant native resources nor change financial/proof/target acceptance.

## DWCF-001 — Truthful adapter capabilities

WHEN DoctorWatson assesses a task, it SHALL preserve the legacy qualified `selection_candidate` and `ready_for_root_selection` semantics and SHALL report `dependency_ready`, `collector_available`, `executor_available`, `ready_for_dispatch` and `dispatch_blockers` from the pure shared capability interface. It SHALL report `dispatch_readiness_scope: dependencies-and-adapter-capabilities` and `dispatch_preflight: not-observed`; this readiness view does not establish attempt reservation, selection validation or resource preflight.

A task SHALL enter `ready_for_dispatch` only when its qualified candidate predicate and both capability predicates hold. WHEN reconciliation changes final readiness, DoctorWatson SHALL recompute that dispatch view. Capability availability SHALL NOT promote unknown, debt, failed, consumed, source-integrated or accepted outcomes into a new dispatch.

### Scenario: dependency-ready unsupported adapter

Given an active Build9 task with current input pins, no consumed attempt and satisfied dependencies, when it is inspected without an implemented executor/collector, then it remains a legacy candidate, its dispatch readiness is false and its capability blocker is reported.

### Scenario: reconciled candidate and consumed success

Given a properly discharged historical not-started attempt and an eligible newer SDK task, when reconciliation finishes, then both inspection and executor see the refreshed dispatch candidate. Given an already accepted current success, then capability availability does not redispatch it.

## DWCF-002 — Local reuse with fresh checks

WHEN computing one assessment, DoctorWatson SHALL calculate each task fingerprint once in dependency order using the existing canonical byte semantics. It MAY reuse a FileRef check only for identical path, pinned size, pinned digest and parse mode within that assessment. The requirement checker MAY reuse a file's raw bytes and actual digest only within one report call.

WHEN an independent assessment or predispatch recheck starts, it SHALL read the selected evidence afresh. Matching mtimes or a persistent cache SHALL NOT substitute for content checking.

### Scenario: shared dependency edit

Given a diamond dependency graph and an independent task, when a shared dependency identity changes, then dependent fingerprints change, their old results remain stale, and the independent fingerprint remains unchanged.

### Scenario: intervening evidence edit

Given a passing assessment, when a source or artifact changes before the independent use-time assessment, then the later assessment detects the change. Equal length and restored mtime do not preserve currentness. Different pins or parse modes do not reuse a prior successful FileRef check.

## DWCF-003 — Required complete execution history

WHEN execution is requested, DoctorWatson SHALL require an explicitly selected authoritative complete history file before dispatch. Inspection MAY use an explicitly empty example history without claiming a replay barrier. Execution SHALL preserve original attempts and their unknown/cleanup obligations and SHALL reserve the selected attempt durably before launch.

### Scenario: omitted history

Given a valid selected task, when execution omits history, then it refuses before child dispatch. An explicit empty history remains distinct from an omitted input.

### Scenario: original attempt retained

Given a recorded failed, possible-launch or unknown original operation, when a later selection is assessed, then it cannot silently become a fresh retry. Missing or changed reserved original results remain blocking; an attempt-directory deletion alone does not remove the barrier.

## DWCF-004 — Fixed public verifier consumer

WHEN the `doctorwatson.handkey-selection/1` branch is selected, DoctorWatson SHALL bind the exact executable, key, tagged proof, public-input JSON, task, fingerprint and attempt and SHALL invoke only the code-fixed `verify-pis VERIFIER TAGGED_PROOF PIS_JSON` operation with the fixed environment and finite resource profile. Task JSON SHALL NOT supply arbitrary commands, shell, environment or caps. The SDK-build selection/2 path SHALL retain its existing contract.

The consumer SHALL retain durable PREEXEC and actual terminal/full-stream/cleanup observations. It SHALL accept verifier success only from the exact typed result and successful applicable process outcome. Unobserved termination or cleanup SHALL remain unknown.

### Scenario: exact invocation and changed tuple

Given harmless temporary executable fixtures and a matching selected tuple, when the real subprocess route runs, then its argv and typed output are recorded. When a tuple pin, source, attempt or selection changes before use, then launch is refused.

### Scenario: false or incomplete output

Given exit zero with false, malformed or absent verifier output, when collection runs, then no successful verification is recorded. A timeout or surviving descendant does not produce invented cleanup completion. Fixture success supplies no real proof or ledger evidence.

## DWCF-005 — Current source and evidence stages

WHEN DoctorWatson reports a cycle, it SHALL retain separate locators/statuses for source prepared, installed receiving tree, tested, independently reviewed, merged, operation observed and target accepted. It SHALL identify the exact source cut and applicability of each result. A source/hash/model/test/merge observation SHALL NOT imply downstream semantic, proof or target acceptance.

WHEN a provider terminates without the requested source or substantive review, the cycle SHALL record that missing artifact and a concrete next action rather than infer approval or duplicate a still-live request.

### Scenario: three-tree mismatch

Given old W `f935feb`, published `c26859a5` and a runtime Source44 afterimage, when reporting currentness, then absence in W is not reported as absence from published source, and runtime presence is not reported as a tested merge.

### Scenario: native prerequisites remain separate

Given observed Source43 release build and k20 VK generation but a timed-out parent attempt with no proof, then the report preserves build/VK success and proof incompleteness separately. It does not report completed PK setup or stagenet acceptance from those observations.

## DWCF-006 — Ordered executable final phase

WHEN finalizing an authorized bounded cycle, DoctorWatson SHALL first obtain and validate a completed graph refresh for the exact affected source pins, SHALL then append the cycle's outcome and lessons to the selected existing `wiki-llm` log, and SHALL only then attempt retirement of eligible selected local worktrees/branches.

The bounded cleanup entrypoint is `python3 models/doctorwatson/cleanup.py --repo ROOT --request REQUEST.json --report REPORT.json [--apply]`. Without `--apply`, it SHALL inspect only. Its `doctorwatson.cleanup-request/1` references an immutable `doctorwatson.graph-refresh/1` receipt binding cycle ID, exact affected-source coverage and output artifact digests. The existing Graphify worker supplies the affected-scope refresh; the cleanup helper verifies the receipt and bytes and SHALL NOT claim it regenerated the graph itself or constitute a new graph engine. Semantic graph completeness and the producer’s truthfulness remain external premises. No caller-supplied executable command is accepted.

### Scenario: successful ordering

Given fresh graph receipt/output bytes, a valid existing wiki destination and a bounded selection, when apply runs, then graph verification precedes wiki append and wiki success precedes any retirement. The report retains each phase's observed status.

### Scenario: missing or stale graph

Given missing affected-source coverage, changed source/output bytes or an invalid refresh receipt, when finalization runs, then no wiki append or Git deletion follows and the cycle remains incomplete with the failed phase recorded. Inventory coverage does not imply semantic reading.

## DWCF-007 — Idempotent wiki and resume

WHEN a cycle append is requested, the helper SHALL bind a stable cycle ID to the canonical outcome/lessons and graph-receipt identity, SHALL preserve all existing log bytes, and SHALL append that exact cycle entry at most once. A matching repeat SHALL report already-appended; a conflicting reuse of the cycle ID SHALL refuse without changing history. Historical evidence, prior entries and the circuit atlas SHALL remain unchanged. For unchanged source/output pins, a repeated finalization MAY reuse its completed refresh receipt only after checking those bytes again; changed identity requires a newly identified cycle.

### Scenario: same-cycle repeat

Given a successfully appended entry, when the same request is repeated, then no second entry is added. Cleanup repeats SHALL NOT delete a different or unknown ref. A retained actual retirement receipt may establish already-retired status only when its stable operation digest and branch/tip deletion evidence match and fresh checks confirm that branch, registered worktree and selected path remain absent. Fresh inventory remains mandatory and intervening inspection or incomplete reports SHALL preserve that receipt. Missing evidence or a recreated different tip SHALL block that disposition rather than invent prior deletion provenance.

### Scenario: changed same-cycle content

Given a prior cycle marker, when different lessons or receipt identity reuse its ID, then the helper records a conflict, preserves the original entry and performs no subsequent cleanup for that failed phase.

## DWCF-008 — Conservative merged-only retirement

WHEN evaluating cleanup candidates, the helper SHALL require fresh bounded inventory (at most 900 seconds old), canonical repository identity, exact selected branch/worktree tips, and exact retained-root identities. It SHALL retire a local branch only when its selected tip is an ancestor of retained history and fresh checks establish that no protected worktree still checks it out. Deletion SHALL use an atomic expected-tip/retained-root Git ref transaction. A tip or retained-root race SHALL fail closed. Retained roots may be exact local or remote-tracking refs; remote branch refs themselves remain undeleted. Apply SHALL acquire the shared common-directory cleanup lock and require a no-concurrent-writer window; the advisory lock does not constrain arbitrary Git or user-file writers.

The helper SHALL require a named owner and explicit `inactive: true` attestation for each retirement candidate; transport lease expiry, broker presence and DID identity SHALL NOT establish inactivity. It SHALL preserve current/default/main/master/sherlock and retained roots, active branches/worktrees, leased worktrees, open-PR heads and bases, locked/unknown worktrees, submodules and any worktree containing tracked changes, untracked files or ignored user data. It SHALL remove no worktree with force and SHALL perform no remote deletion, object pruning or garbage collection. Branch configuration retention SHALL be explicit.

The request's complete open-PR and known-lease inventory is a caller attestation, not authenticated GitHub or process evidence. Missing/stale/ambiguous inventory SHALL block affected retirement. Squash/rebase merge labels without retained ancestry SHALL NOT bypass the ancestry predicate.

### Scenario: eligible inactive merged work

Given exact inactive selected tips reachable from a retained root, complete fresh inventory with no protected PR/lease reference and a pristine worktree, when apply revalidates at mutation time, then it may remove that worktree without force and delete only the expected local branch ref. Other work and data remain.

### Scenario: race or user data

Given any changed tip/root, active/PR/lease reference, locked state, modified tracked content, untracked/ignored file or submodule, when retirement is evaluated, then the candidate is retained or blocked with owner, reason, next action and revisit event; no force deletion occurs. A stale snapshot cannot authorize mutation.

## DWCF-009 — Visible incomplete failure

IF graph verification, wiki append, required cleanup validation, Git removal/ref transaction or report persistence fails, THEN the helper SHALL retain the actual error and completed partial actions, SHALL mark the cycle incomplete and SHALL NOT publish a completed-cycle claim. It SHALL NOT silently retry deletion against a different tip. Safe intentional retention of a protected candidate is a recorded disposition, not proof it was deleted.

### Scenario: partial cleanup

Given a successful wiki append and one retired candidate, when a later candidate fails or a ref race is detected, then the report records the actual retired/retained/blocked dispositions and incomplete final status. The earlier evidence is not rewritten into all-or-nothing success.

### Scenario: append/report failure

Given a wiki write failure, then no candidate deletion follows. Given final report persistence failure, then the command reports failure and does not leave a durable complete receipt. A later attempt resumes only after fresh validation.

## DWCF-010 — Local checks and independent review

WHEN accepting the final candidate, the integrator SHALL run affected ordinary checks locally, retain exact commands/exits and bind returned independent review to the exact final source and actual results. Author self-checks SHALL NOT stand in for a returned non-author verdict. No automatic hosted checks SHALL be introduced.

### Scenario: disposable cleanup tests

Given disposable temporary repositories, tests SHALL cover default inspect, ordered phases, append idempotency/conflict, graph coverage drift, merged ancestry, PR/active/lease/user-data preservation, expected-tip/root races and partial failure. These tests establish cleanup behavior without claiming live repository retirement.

### Scenario: changed final candidate

Given an earlier passing cut or review, when final source changes, then the integrator retains earlier evidence at its original cut and obtains applicable checks/review for the changed candidate before reporting acceptance. No full39, recursion, wallet discharge or target result follows from this workflow acceptance.


### Retirement receipt clarification

`doctorwatson.local-retirement/1` records are minted only after actual successful
CAS deletion. Stable `operation_sha256` excludes live inventory but binds the
actual Git common directory and selected operation fields; fresh PR/lease/HEAD,
source/graph and ref checks remain mandatory. `request_sha256` retains the exact
raw request identity. Retirement receipts survive inspect and incomplete reports;
unknown absence is not a deletion receipt. Cooperating writers use the existing
apply lock, and the caller keeps a no-concurrent-writer window including any
inspector sharing its report path.
