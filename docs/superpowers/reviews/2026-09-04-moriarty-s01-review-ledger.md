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

## Task 6 and whole-package review

The whole-package reviewer inspected through Task 6 commit `89848ae`. The
evidence-only wiki handoff, source metadata, and corrected OpenSpec status had
no material findings. The review confirmed the local checker and receipt
closure, but identified one P2 terminology error: required XML alias
`settlement` referred to `SettlementReceipt`, an evidence object, rather than
the protocol-specific settlement process defined by XML v1.3.

The correction introduces a separate `SettlementProcess`, moves the alias,
retains the receipt's evidence meaning, adds a distinction regression, and updates
only the affected normative pin before recomputing receipts. Commit `5d38637`
implements this correction. This is a
specification correction within S01, not a Core semantic motion. The prior
manifest is preserved byte-for-byte under
`raw/repository-observations/2026-09-04-s01-pre-settlement-correction-manifest.json`.
Its SHA-256 is `e58efb60c4c8e85d9ff2de878bc0b9628c08b79430a5aa00a0ce397618d507bd`.
The original 20-pin comparison remains a historical review observation; the
revised terminology pin received new independent review.

The reviewer found no other material issue. Six whole-branch blank-line-at-EOF
warnings are a documented, nonblocking style exception; see the final
verification record.

The independent re-review approved the whole-S01 code and normative artifacts
at `5d38637`, with no remaining material finding. It checked all ten gates,
the four correction regression cases, and all 20 committed pins. The new
terminology pin matches the reviewed bytes; the other 19 pins are unchanged.
The provenance handoff archives SRC-0032 and registers the revised manifest as
SRC-0034. S01 completion does not pass any later release gate.

## Next obligation

Execute the reviewed S02 contract plan, build and model-check all four distinct
Quint architecture alternatives, and make an evidence-backed selection or stop
decision. The full XML program remains active.
The earlier Tasks 1 and 4 review is recorded in the
[execution audit](2026-09-04-moriarty-v1.3-execution-audit.md).
