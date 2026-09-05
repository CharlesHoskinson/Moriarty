# S02 independent review ledger

Classification: repository observation. S02 is in progress. This ledger does
not establish a model result, architecture selection, or XML release gate.

## Contract plan

Independent review of the contract subproject plan found that requiring every
candidate to pass would make a justified rejection or stop impossible. The
corrected plan distinguishes completed, decisive evidence from eligibility
for selection. Zero sampled witness counts remain inconclusive. It also names
three separate bridge controls. Re-review approved the plan at `d838b0c`.

## Contract Task 1: acceptance vocabulary

Independent specification and quality review approved `af3abfe..e9b4f80` with
no findings. The registry closes all 17 top-level fields, exact identifiers,
ordering, pinned inputs, and specification-only status. Its evidence list is
empty and its selected candidate is null. The test loads the actual registry
and asserts the complete prescribed vocabulary.

The implementer recorded the expected missing-registry RED, focused GREEN,
unchanged passing S01 gate, and a full suite of 283 tests. The reviewer did not
rerun those commands and correctly noted that a final diff cannot prove TDD
chronology. The controller separately reran the focused test and read-only S01
validator successfully. The diff changes no S01 normative input or artifact.

## Contract Task 2 and whole-contract review

Independent review approved Task 2's specification compliance and quality and
the whole contract subproject through `02aba01`, with no material finding.
The review checked all ten normative scenarios, the closed registry, all four
representations, both workloads and signing profiles, candidate rejection and
stop dispositions, full E00 comparison, critical-control exceptions, and
manifest closure. All ten declared input SHA-256 pins independently matched.

The implementer recorded the missing-package RED, two focused passing tests,
284 full-suite tests, and the passing S01 validator. The reviewer did not rerun
tests and did not claim to prove test chronology from the final diff. No model,
execution result, selected architecture, S02 pass, or scope motion was claimed.

## Common observation and authorization design

The next design separates wallet/escrow effects from Core payments, requires
per-principal authorization and incoming consideration, and specifies parent
and residual consumption plus separately authorized cancellation recovery.
Independent review found two issues: signing must follow complete pre-sign
checks, and recovery witnesses need an explicit acceptance mapping. Commit
`d742a5b` corrected both. Re-review approved the bounded design with no remaining
material issue. Two recovery subscenarios are mandatory under S02-05/S02-09;
the original registry witness identifiers remain unchanged.

## Next work

The contract subproject is complete and remains specification-only. Plan and
execute the common model and four distinct
candidate representations before correspondence, negative controls, actual
Quint/Apalache checking, and the selection or stop decision.

## Effect foundation review

Task review of `aea316c..d069800` found no defect in transfer arithmetic,
multiplicity checks, guarded actions, or the OpenSpec supplement binding.
It verified source/evidence hashes and the three-state deposit/refund ITF.
The reviewer did not approve the task because command provenance was incomplete.
Quint structured compiler output does not preserve console witness counts.
The report also lacked original RED and incremental-run output.

The implementer must preserve exact commands, exit codes, and console output.
Recovered historical evidence must identify its source.
Any reconstructed check must be labeled retrospective, not original chronology.
The controller separately ran ten Quint tests, both witnesses in 100 sampled
traces, three Python contract tests, and the ten S01 gates successfully.
Those checks do not substitute for the missing original process record.

Commit `1bd4bff` adds fresh console receipts with exact commands and exit codes.
It also preserves reconstructed missing-import and deposit-only checks.
The original process output was unavailable, so the reconstruction is explicitly retrospective.
This correction remains unapproved at the current integration boundary.

The user now requires GPT-6 Astra, Grok 4.6, and Fable 5.1 council review for each gate.
The next acceptance review must follow `docs/COUNCIL_REVIEWS.md` from main.
Prior same-family task reviews cannot substitute for that three-provider council.
