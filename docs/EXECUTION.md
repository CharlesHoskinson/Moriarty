# Moriarty v1.3 execution

The user requested complete execution, a persistent loop, and repository commits
on 2026-09-04. The runtime goal loop owns continuation for this conversation.
Query the runtime goal for its current status. This document does not assert process liveness.

The controlling [XML v1.3](../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml)
defines 15 sprints, 24 release gates, and 22 deliverables. The
[program register](../evidence/execution/moriarty-v1.3-program.json) preserves
every required identifier and predicate. Empty evidence lists mean unassessed work.

## Execution procedure

1. Recover the typed checkpoint from `.foreman/session.ndjson` or its live database.
2. Verify the current repository state and applicable input digests.
3. Read the current sprint contract and its unresolved obligations.
4. Resolve specification ambiguities explicitly before closing a gate.
5. Implement the next dependency-ready task with focused negative tests.
6. Obtain independent review and resolve its material findings.
7. Recompute the sprint gate from the actual artifacts and preserved runs.
8. Record results, limitations, semantic-scope effects, and the next obligation.
9. Commit task artifacts and integrate reviewed increments into `main`.
10. Continue until the prompt's evidence-backed terminal decision is complete.

Use the runtime goal for continuation. Do not add a second concurrent scheduler
that writes to the same checkout. Runtime database and WAL files stay local.
Commit the typed checkpoint export, program register, code, specifications, tests,
and research evidence. Do not commit credentials or unredacted environments.

## Current execution boundary

The audited S01 starting increment, audit resolutions, and registries are
integrated into `main`. Tasks 1 through 5 have passed independent review.
The closed validator recomputes the ten local S01 package gates and validates
the published receipts. The Task 6 wiki handoff is committed in the implementation
branch. Whole-S01 review found a settlement-process/receipt terminology mismatch;
correct it and obtain re-review before the completion transition. These package
gates are not the 24 XML release gates.

S02 uses Quint models and the Quint CLI with the Apalache backend, as directed
by the user. The reviewed model-comparison design is preparation, not an
executed comparison or architecture selection.

All architecture candidates must be tested before selecting an architecture.
An absent proof, parameter file, pilot, or measurement is not evidence of infeasibility.
Do not use missing work to manufacture a terminal stop decision.

Independent human evaluations require real participants. Record that dependency
when it becomes critical. Do not invent pilot preferences or contact people
without explicit authorization. Local simulations cannot substitute for real
proof verification or measured ledger execution.
