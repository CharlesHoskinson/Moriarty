# Compiler phase plan correction CP-R1

Status: source-only correction complete; narrow independent re-review and root adoption remain required. No executable diagnostic files or controls were created or run.

Before editing, I preserved the exact reviewed plan in `.superpowers/sdd/a5-phase-plan-original-20260906.md`, SHA-256 `6a861022ed8641e2674f36f5725636c79550fffe3550928012694331cc0d76e4`. The independent request-changes review remains unchanged at `.superpowers/sdd/a5-compiler-phase-implementation-plan-review-20260906.md`, SHA-256 `9091fb1864de49500d048e5e7926a26b466b20c49c4e9e8ba67820cf6d4c31cc`.

Corrected plan: `docs/superpowers/plans/2026-09-06-candidate-a-compiler-phase-diagnostic.md`, SHA-256 `b1df377b077cb7189f945ac493039e1f8d941f5076f61c1be6aaf8559de7bff9`.

CP-R1 was valid: raw/content checks previously followed all four native launches. The `record.py` block now validates successful direct JSON shape, selected declarations and empty stderr immediately after that child's terminal check. The error direct child immediately checks empty stdout and the genuine missing-alias stderr. Each observed child compares its raw stdout/stderr with its already validated direct baseline before the loop can launch another child. A failed predicate raises into the existing terminal/finalization path; no retry or continuation is added. CP003 prose states this exact sequencing.

The four-child order, inputs, budgets, existing trace/argv/resource predicates, final aggregate predicates and original failure preservation remain unchanged. The observer, launcher and mock-control blocks are byte-identical to the preserved original plan. No broader redesign was made.

Read-only extraction/static AST checks verified four complete source blocks, syntactically valid Python blocks, and all five moved content/comparison predicates inside the native child loop. The original plan/review hashes were rechecked and the executable diagnostic directory remains absent. No proposed code was imported or executed, and no model, runtime, helper, native/mock/test run or commit changed.
