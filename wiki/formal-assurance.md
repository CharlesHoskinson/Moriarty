---
id: assurance.formal.matrix
type: formal
title: Formal assurance matrix
status: active
updated_at: 2026-09-06T22:00:49.403297+00:00
sources:
  - SRC-0050
  - SRC-0043
  - SRC-0044
  - SRC-0045
  - SRC-0046
  - SRC-0002
  - SRC-0009
  - SRC-0040
  - SRC-0042
  - SRC-0047
  - SRC-0048
  - SRC-0049
---

# Formal assurance matrix

## Intents and R2 boundary — 2026-09-06

CLM-0195–0197 add signed authority, plan refinement, complete/pending receipt
judgments and residual obligations to the proposed acceptance relation.
R2's independent effect checker enforces a restricted exact-plan profile;
gross edge caps and net credits have adversarial tests. Local Ed25519 checks
authenticate key possession and bindings. These are S3 checks, not a proved
compiler/refinement theorem or contract/history certificate. All required real
claims remain unavailable. See [evidence](../evidence/moriarty-r2-language-2026-09-06/README.md)
and the [intents amendment](../docs/research/2026-09-06-intents-report-integration.md).
R2b authority composition and R3 native/ledger proof gates remain open.

The theorem inventory was reproduced from pinned Isabelle sources at
`marlowe-lang/marlowe` commit
`7b5b1e90c171eae2674a6fc08aa7d8caa92b16af`. The table states the defensible
claim, not the marketing shorthand.

| Property | Existing evidence | Required assumptions or qualification | Moriarty work |
|---|---|---|---|
| Termination | Structural reduction proof and total executable path | V1 core only; no new recursive construct | Prove the Core step decreases a well-founded measure; surface loops elaborate to finite Core |
| Finite maximum lifetime | Timeout maximum over finite syntax | Every wait has a finite absolute bound | Reject unbounded subscriptions; certify the bound in the manifest |
| Conservation | Isabelle money-preservation theorems | Valid positive state and successful transaction | Prove abstract conservation and separately prove correspondence to Midnight kernel effects |
| Positive accounts | Preservation results | Valid initial state | Make validity a constructor and transaction precondition |
| Closure and no residual internal value | Closure theorem family | Semantic execution path exists and can be submitted | Separate semantic closure from proving, ledger, witness, and participant liveness |
| Quiescence and idempotence | Isabelle results | Pinned V1 evaluator | Preserve in Core; do not require one giant ZK circuit to reduce an entire lifetime |
| Grouped/split input equivalence | Conditional Isabelle theorem | Applies only under stated valid transaction conditions | Reprove around atomic action sets; atomicity intentionally changes some equivalences |
| Determinism | First-case transition structure | Canonical interval and serialization agreement | Define total deterministic step relation and canonical input ordering |
| Transaction count bound | `countWhens`-style upper bound | Abstract transactions, not ledger resource feasibility | Certificate includes max transitions and per-entry-point resource estimates |
| Authorization | Operational address/role checks | Does not validate arbitrary role-token policy | Model credentials/capabilities and nullifier/replay rules explicitly |
| Continuation integrity | Hash comparison | Does not imply continuation availability | Bind continuation roots in ZK; specify replicated availability separately |
| Cross-implementation correspondence | Semantics-oracle, golden, property, and on-chain tests exist | No single theorem covers Isabelle, Agda, Haskell, TypeScript, Plutus | Normative Core plus conformance vectors, differential tests, and translation validation |

Agda is experimental and contains explicit termination pragmas in important
paths. It is not ready to become normative merely because it is executable.
The sustainable strategy is a small mathematical Core specification plus a
compact executable reference interpreter, with proofs in one supported proof
assistant and independent differential implementations. Extraction may be
used, but it is not the only correspondence mechanism.

The assurance vocabulary is permanently layered:

```text
semantic validity
  -> Moriarty-to-Compact translation validity
  -> Compact-to-ZKIR compiler correctness
  -> circuit/proof correctness
  -> Midnight ledger feasibility
  -> transaction construction and wallet correctness
  -> private witness correctness and availability
  -> participant and oracle liveness
```

A proof at one arrow never silently discharges the next arrow.


## CLM-0191: Proposed Moriarty contract certificates and transaction PCD

The [semantic proposal](../docs/superpowers/specs/2026-09-06-moriarty-unified-semantics-design.md)
defines separate obligations for language termination, package/contract
invariants, authorized transaction execution, predecessor-history compliance
and compiler/ledger correspondence. Its first property set covers asset and due
accounting, event identity/order, LAM accrual, exact-input AMM arithmetic,
authority/effects and finite lifecycle/closure. None is proved by this proposal.

Contract certification must bind a checked proof to program, package, bounds,
assumptions and property identities. Transaction PCD binds the authorized state
change and predecessors to that certified relation. Cross-program composition
also requires a joint rule and each instance's program/certificate mapping.
Live consumption and oracle trust remain separate. A hash of an unchecked
certificate or a simulated proof is insufficient.

Metadata: SRC-0040, SRC-0042; authority primary research interpreted as a design
recommendation; date 2026-09-06; scope proposed Moriarty language, S2;
reproduction specified-only for these proof obligations; confidence medium for
the design and unknown for implementation/proof feasibility. Pinned Marlowe
results above are prior evidence with their original scopes, not these new proofs.


## CLM-0192–0193: Typed evidence and Midnight recursion correction

The [PCD report revision](../docs/research/2026-09-06-pcd-report-integration.md)
separates execution validity, one execution's effects, all-domain contract
properties, current-state applicability and external truth. The signed required
claim root and deployment-authorized verifier policy prevent stripping and
predicate substitution. These are S2 design obligations, not theorems.

The native Midnight backend has recursive verification/IVC source at the
[SRC-0045 pins](../raw/pcd-midnight-recursion-2026-09-06/repository-inspection.receipt.json).
Compact's source recursion restriction is a different boundary. Native IVC
success, accepted ledger proof format, multi-parent PCD semantics and private
witness handoff each need separate evidence. No Moriarty cryptographic result
was reproduced. Metadata and qualification are in journal CLM-0192–0193.

## R2b local outcome authority boundary

CLM-0198 adds an independent bounded trace checker and a runtime that recomputes
one registered pool swap or loan settlement. Aggregate gross spending, permitted
recipients, net goals, fees, expiry and one-shot consumption are tested locally.
The [evidence](../evidence/moriarty-r2b-outcomes-2026-09-06/README.md) identifies
source hashes and rejection/positive controls. These tests and Ed25519 signatures
are not contract certificates, refinement proofs, PCD or ledger uniqueness.
The four required real claims remain unavailable. CLM-0199 preserves held-out
financial extensions before further Core design. Metadata: SRC-0042/0047/0050;
2026-09-06; experiment observation, S3; tested predicates reproduced, confidence
high within their scope. General proof obligations remain specified-only.
