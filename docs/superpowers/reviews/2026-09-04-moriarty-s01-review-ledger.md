# S01 independent review ledger

Classification: repository observation. Scope: XML v1.3 S01 specification and
local experiment. No entry is a mechanized proof or a release-gate verdict.

## Audit resolution supplement

The independent architecture reviewer requested global all-or-refund semantics:
all required domains fulfill, or all affected domains satisfy the jointly
authorized refund predicate. A mixed result is not sufficient. Commit `5e01ff7`
applied the correction. The re-review approved the additive supplement.
Original XML, design and Core scope remained unchanged.

## Task 2: registries and schema

The independent reviewer inspected the implementation at `21709e6` and requested
two important corrections:

1. Lifecycle producers and consumers must identify actual participants, not
   glossary authorship or a generic verifier/auditor set. Quote requests need a
   quote-provider consumer; proof and signing requests need their actual consumers.
2. Both report and manifest schemas must require exactly the ten named S01 gates
   and reject unknown gate names.

The controller also found that manifest digest records omitted the `role`
required by the implementation plan. Commit `3b485f0` corrected all three points
and added regression tests. The independent re-review of
`e7cffaa..3b485f0` approved specification compliance and quality, with no remaining
findings. The review was read-only and did not repeat the implementer's test run.

The reviewed metadata retains architecture neutrality and specification-only
status. Canonical runtime serialization remains profile-dependent; a glossary
definition does not establish a byte encoding.

## Task 3: candidate theorem

The independent review of `3b485f0..ba972dd` approved specification compliance
and quality, with no findings. It checked the exact quantifiers, premise
ID/expression pairs, conclusion, 29 bindings, 13 subsidiary claims and seven
exclusions against the governing design and supplement. Tests reject unknown
judgment, premise and conclusion fields. The theorem remains architecture
neutral and candidate-unmechanized. The reviewer used the recorded RED/GREEN
test evidence; a final diff alone cannot prove test chronology.

## Task 5: closed evidence validator

The independent reviewer found two important issues in `9b89753`:

1. The public `recompute_gate()` API omitted published-receipt checks even
   though the command-line path performed them.
2. Missing schema definitions and unresolved schema references could escape
   the documented `ValidationError` boundary.

Commit `7bec08c` restored the closed public API, kept a private semantic
bootstrap for explicit publication, and normalized schema failures. Regression
tests cover missing or stale receipts, missing definitions, and external or
unresolved references. The independent re-review approved specification
compliance and quality with no remaining findings. The controller separately
compared all 20 embedded source pins against the reviewed `ae5aa5c` boundary;
each matched. The reviewed changes were integrated into `main` at `3c87c3b`.

The validator is a local evidence-package check. The checker still denies
signing for both baseline and mutant. Neither self-hashing receipts nor source
pins constitute a signature, mechanized proof, or authenticated effect extraction.
The runner, pin table, interpreter, dependencies, operating system, and filesystem
remain trusted. Replacing the runner and its pin table together is outside the
stated guarantee.

## Open work

The evidence-only wiki transition and final whole-S01 review remain outstanding
at this ledger revision.
The earlier Tasks 1 and 4 review is recorded in the
[execution audit](2026-09-04-moriarty-v1.3-execution-audit.md).
