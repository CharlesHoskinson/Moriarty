# Candidate A lifecycle verification-command addendum

Status: adopted local command consolidation, 2026-09-05.
Applies to Task3 of installment and swap plans adopted82d2c0b.
Original plan files/hashes remain unchanged. No behavioral, scenario, invariant,
witness, model-bound or semantic change is made.

## Equivalent static check

Run one recursive test-module typecheck per lifecycle instead of its four
standalone module typechecks. Independently verify and pin the complete import
closure, explicitly including fixtures, lifecycle, harness and test modules.
A missing module or failed terminal typecheck blocks this substitution.

Installed Quint0.32.0 source observations:
- cliCommands.js: parse resolves sources/imports then calls analyzeModules on all
  parsed modules at162–165.
- quintAnalyzer.js:27–31 analyzes every module's declarations.
- quintParserFrontend.js:130 source resolution recursively includes imported
  modules, including declarations not selected by an import.

Root's tiny control imports only dependency.ok, leaving unusedBroken:bool=1
unused in the dependency. Entry typechecking nevertheless fails QNT000 at that
declaration. Changing only1 totrue then passes. Exact sources, terminal outputs
and tool implementation hashes are in the companion control receipt. This is a
tool-behavior experiment, not a Moriarty semantic test or lifecycle RED.

## Shared final regressions

After BOTH final lifecycle source closures freeze, root runs boundary tests,
adapter tests and the full Python suite once. Archive the exact commands,
terminal outputs and complete relevant source/tool inventories. Bind this one
receipt to both units' exact final hashes. Neither unit is finally accepted
before that shared receipt passes. If an affected source changes, rerun the
affected checks; do not reuse a stale result as fresh.

Each lifecycle still runs its own complete17-test inventory and the original
100-sample command with every required action/profile witness and original
step/invariant settings. Missing witnesses follow the original diagnostic and
conditional1000-sample rule. Independent final source/runtime/evidence review,
A4 exports, A5 model checking and A6 Council remain mandatory.

## Independent disposition

Native nonauthor /root/a0_final_review approved this command addendum: recursive
typechecking covers the full verified import closure; shared regressions require
both closures frozen and bound; all own tests/witnesses stay intact. The reviewer
did not execute or independently authenticate the tool control. Root owns that
original receipt. This is not Council or lifecycle acceptance.
