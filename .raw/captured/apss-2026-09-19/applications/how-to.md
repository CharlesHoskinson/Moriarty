---
id: apss.applications.how-to
title: Audit a proposed Moriarty application boundary
status: draft
documentation_type: how-to
---

# Audit a proposed Moriarty application boundary

Use this guide when reviewing an application design, library or SDK that proposes to use Moriarty. The output is a specification and evidence checklist, not deployment approval. You need the program design, the user's authorization format, expected effects and the proposed acceptance path. No working implementation is assumed.

1. **Identify the program independently of the service.** Name its state, actions and supported language constructs. Remove any dependency on a maintainer identity, solver company, Foreman run or campaign ID from the product validity predicate. Keep service-specific policies in that service's contract. Verify a newly authored supported program is not excluded merely because it is outside the demonstration catalog. See [APP02](notes/APP02.md).

2. **Write separate statements for developer and user intention.** Record the application invariant and the user's exact assets, gross debit, fees, recipients, liabilities, expiry and outcome. Mark preferences as ranking criteria. For each field that the solver chooses, write the predicate limiting that choice. Do not let favorable price compensate for an unauthorized recipient. See [APP08](notes/APP08.md).

3. **List every observable effect.** Include transfers, fees, approvals, debt creation/discharge, locks, claims, authority consumption and state changes. Specify conservation by exact asset identity and rounding rules. Identify external observations and the evidence authenticating them. Check that imported assumptions have not been silently promoted into theorems. See [APP05](notes/APP05.md).

4. **Separate terminal completion from progress.** For asynchronous work, write each transition from pending to claimable to completed, including partial quantities and remaining duties. State which party can cancel, claim, recover or refuse continuation. Do not invent cancellation rights that an underlying interface does not provide. See [APP11](notes/APP11.md).

5. **Audit reuse and composition.** Try two independently valid candidates consuming the same authorization. Try a fresh order hash that represents the same spending intent. Check that cumulative limits and remaining work survive retries, restart and partial progress. State restrictions on multiple operations from the same application instance in one batch. See [APP09](notes/APP09.md), [APP10](notes/APP10.md).

6. **Separate proposed execution from acceptance.** Record what the optimizer, oracle, resolver and solver contribute. Then identify the independently checked predicate deciding validity. A trusted quote or resolver output must not replace the required program/refinement/history proof. See [APP12](notes/APP12.md).

7. **Make privacy claims explicit.** List public inputs, each witness holder and each communication recipient. Include metadata, timing and failure output. Verify that a claim of private settlement does not conceal plaintext disclosure during planning or delegated proving. See [APP06](notes/APP06.md).

8. **Create positive and negative fixtures before implementation.** Include one feasible intended execution, one specification-level economic failure, one authority violation, one conflicting reuse and one unavailable-remedy case. Label each evidence class: paper reasoning, executable local result, mechanized proof or finalized ledger result. Retain unsupported cases as open work.

## Proposed requirement implications

These IDs are research proposals for crosswalking into OpenSpec, not implemented requirements or new administrative gates.

| ID | Proposed EARS statement | Decisive control |
|---|---|---|
| APP-MOR-001 | When a new program uses supported constructs, the public toolchain shall apply objective semantic and proof checks without a project developer/program allowlist. | Clean developer environment with no workflow records; novel supported contract. |
| APP-MOR-002 | When an execution is accepted, its bound evidence shall establish both application invariants and the affected user's signed authority/outcome constraints. | Pool-invariant-valid trade with an unauthorized recipient rejects. |
| APP-MOR-003 | While an application is partially complete, it shall preserve outstanding liabilities, unconsumed authority and remaining work in successor state. | Partial redemption followed by restart cannot erase or duplicate the residual claim. |
| APP-MOR-004 | If a proposal changes its order identifier, acceptance shall not infer renewed economic authority from that change alone. | Two hashes cannot independently exhaust the same signed cumulative budget. |
| APP-MOR-005 | Where an oracle or solver ranks proposals, its output shall not substitute for the independently enforced validity predicate. | Malicious advice cannot make an invalid state transition pass. |
| APP-MOR-006 | When an application advertises privacy, it shall specify public disclosure and witness/communication assumptions separately from proof validity. | A delegated prover receiving plaintext is disclosed. |
| APP-MOR-007 | When an asynchronous remedy is described, the specification shall state its enabling conditions and required actor without treating an enabled transition as guaranteed submission. | Unavailable witness or absent submitter is not reported as finalized recovery. |

Finish the audit by listing unresolved predicates and a concrete evidence plan. Do not convert the checklist into permission that a third-party developer must obtain from the Moriarty project.
