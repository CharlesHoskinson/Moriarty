# Delivery packages

**Consolidated schedule, 2026-09-19:** This file retains detailed requirements and historical planning. The [single U0–U7 roadmap](../../../ROADMAP.md) and [consolidated proposal](../consolidated-language-kernel/proposal.md) control current design and scheduling. Resolve former P/C/K owners through [traceability](../consolidated-language-kernel/traceability.md); do not dispatch this as a separate queue. Objective requirements and evidence remain in force.


| Package | Depends on | Deliverable and acceptance evidence |
|---|---|---|
| K0 | P0, C0 | Model stage authority, budget, payment, delivery and recovery. Test forged genesis, concurrency and late-result traces. |
| K1 | K0, P1/P2 | Prove reference primitive semantics, total resource bounds, acceptance predicates and PCD base/preservation. Bind the actual ZKIRv3 relation. |
| K2 | K0, C1/C2 | Implement obligation consent, gross-to-net refinement, private-state completeness and consent-preserving continuation evolution. |
| K3 | K1/K2 | Implement OWS-compatible solver delegation and exact-effect adapters. Demonstrate a valid independent solver and malicious-signer boundary cases. |
| K4 | K2/K3, C3 | Implement selected x402 schemes with explicit support profiles, durable reservations, reconciliation and service-delivery duties. |
| K5 | K1–K4, P7/C4 | Demonstrate actual Midnight effects and declared external evidence. Run positive, adversarial and recovery cases with independent review. |

Choose one bounded package per implementation candidate. State the supported fragment and assumptions before implementation. Independent modeling and source research can proceed in parallel.

A simulated effect, signature, audit vote or successful local proof is not evidence of the full source-to-ledger theorem. Keep undisclosed assumptions and unresolved external outcomes visible. No implementation package is complete in this documentation change.
