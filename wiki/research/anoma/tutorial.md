---
title: "Tutorial — complete an intention without changing it"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: tutorial
tags: [moriarty, anoma, pl-theory]
---

# Tutorial — complete an intention without changing it

This is a paper exercise and proposed experiment, not a working implementation.

1. The owner signs a request to exchange 10 units of A for at least 20 units of B at a specified destination. Fees may consume at most 1 additional A within an 11-A aggregate debit limit. Permitted domains, disclosures and documentary conditions are explicit.
2. An arbitrary solver supplies a compatible counterparty and route. Check that every introduced action lies within both parties' authenticated constraints. A balanced candidate with the wrong recipient must fail.
3. If a funded conditional stage commits before final delivery, represent the escrow, refund rights, evidence state and remaining delivery duties. Distinguish this accepted stage from an unbalanced proposal awaiting composition.
4. One foreign action succeeds while another remains unresolved. Preserve the success and remaining duties. Timeout cannot prove the first action never executed. Do not allow settlement and incompatible refund to both consume the same entitlement.
5. Check a valid alternative route and then mutate fee denomination, recipient, policy circuit, evidence domain, disclosure, or residual obligation. Record the specific rejection condition for each mutation.
6. Bind the complete accepted transition to pinned ZKIRv3 semantics. A valid RISC0 receipt or generated Lean file does not discharge that obligation.

[Solver completion requirement](../mplr/MPLR-035.md) · [How-to](how-to.md) · [Index](index.md).

