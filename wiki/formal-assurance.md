---
id: assurance.formal.matrix
type: formal
title: Formal assurance matrix
status: active
updated_at: 2026-09-02T18:20:00Z
sources:
  - SRC-0002
  - SRC-0009
---

# Formal assurance matrix

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
