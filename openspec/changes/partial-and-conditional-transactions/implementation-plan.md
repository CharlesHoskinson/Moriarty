# Bounded implementation briefs

**Consolidated schedule, 2026-09-19:** This file retains detailed requirements and historical planning. The [single U0–U7 roadmap](../../../ROADMAP.md) and [consolidated proposal](../consolidated-language-kernel/proposal.md) control current design and scheduling. Resolve former P/C/K owners through [traceability](../consolidated-language-kernel/traceability.md); do not dispatch this as a separate queue. Objective requirements and evidence remain in force.


All technical packages are specified-only. They extend the [existing P0–P7 plan](../permissionless-provable-intention/implementation-plan.md). This is one cross-cutting language enhancement, not a second independent acceptance implementation.

| Package | Immutable inputs | Owned outputs proposed for scoping | Required evidence | Stop condition |
|---|---|---|---|---|
| C0 | Product contract, MPLR register, NEAR pins and actual P0 target findings | Versioned stage/condition/recovery specifications under `experiments/moriarty-language/spec/` | Complete judgments, source/target observation relation, exact phase and trust assumptions | Any unsupported equivalence remains a named obligation |
| C1 | C0 and existing P2 source/Core/proof relation | Declared staged parser/checker/lowerer/proof files plus focused tests; exact list frozen before work | Novel program, predecessor/authority bindings, every retained effect, invalid witness and replay controls | No claim from local execution alone |
| C2 | C0/C1, canonical condition and evidence policy | Condition/evidence types, state machine and proof obligations in the owned staged profile | Funded pending request plus recipient signature/document predicate/proof combination; invalid/expired/substituted evidence rejects | Unsupported external fact or privacy guarantee remains conditional or rejected |
| C3 | C1/C2 and P4/P6 resource/accounting relations | Partial-fill, join and recovery semantics/proofs with complete trace fixtures | Duplicate/late/out-of-order results; branch obligations; refund-versus-compensation; separate typed equations | No implicit rollback, debt deletion or double recovery |
| C4 | Qualified C1–C3 and P6/P7 public flow | Conformance corpus, developer examples, evidence manifest and signing/reporting surfaces | Independent non-fixture authorship, actual target evidence, no project-metadata dependency, accurate pending states | Missing proof/target evidence prevents full completion claims |

Before each task, enumerate actual files, immutable artifact hashes, output schema and executable acceptance commands. Existing host checks are `npm --prefix experiments/moriarty-language run typecheck` and `npm --prefix experiments/moriarty-language test`; they cannot establish target or cryptographic correspondence. P0/C0 must determine and pin actual target proof/ledger commands. A failed verification skips paid review; one correction is allowed. No authority to spend funds, submit transactions or publish is created by this sample.
