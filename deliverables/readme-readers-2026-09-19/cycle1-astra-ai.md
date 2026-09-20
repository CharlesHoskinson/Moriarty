# Cycle 1 — AI solver / wallet integration engineer

Verdict: **FLAGS**

Scope: Read the complete frozen `cycle1-README.md`, lines 1–183, as an engineer deciding what can be built now, who holds authority, how OWS/x402 fit, and where to start. Assessment uses only that README. Required repository startup instructions and guarded status were inspected separately; they do not supply evidence for the findings below. Documentation review only; no implementation or linked-document verification was performed.

## Must fix

### Medium — The coordination and integration surfaces lack an explicit current availability statement

Passages: lines 104–116, especially “The optional Federated DeFi Kernel … can collect intentions, connect solvers, obtain evidence, arrange constrained signing, submit external transactions, observe their outcomes and manage authorized recovery” (106); lines 169–175, especially “Federation and solver integrations must preserve those obligations at their external boundaries” (173).

The README clearly identifies local authoring and simulation as available, and the complete proof/settlement path as unfinished. It does not give an equivalent availability statement for the kernel or solver/wallet integration surfaces. The kernel section reads as a description of an available optional coordination system. The status section lists obligations but never says whether an external solver can connect to an existing experimental interface, whether an adapter is only a local prototype, or whether that interface remains a design target. This affects the decision to build an integration now, independently of whether end-to-end proof settlement is complete.

Smallest useful repair: add one concrete status sentence identifying the current availability of kernel/solver integration and OWS/x402 adapters. If an experimental entry point exists, link its guide; otherwise say these are intended integrations and direct builders to the local evaluator API. No API reference or setup tutorial needs to be embedded in this README.

## Optional

### Low — Spell out what the wallet interface contributes

Passage: line 136, “Open Wallet Standard integration supplies a wallet interface; the program supplies the limits within which that interface may be used.”

The intended-use-case preface at line 120 adequately marks this as an illustration, so I do not treat the sentence alone as a false implementation claim. However, naming the standard without an acronym, reference, or a concrete role leaves a wallet engineer unsure which interoperability surface is intended. A short expansion such as “Open Wallet Standard (OWS)” with a normative reference, plus whether it is meant for account access, signing, transaction submission, or another function, would make the integration boundary easier to follow. This need not prescribe an implementation that has not been selected.

## Clear and useful as written

- Authority boundaries are unusually explicit. The solver selects within authenticated constraints (28), complete effects must be bound to acceptance (76–78), and external threshold signing can bypass policy if the destination does not enforce it (114). Neither solver intelligence nor a wallet interface grants unrestricted spending authority.
- The distinction between the local financial evaluator and the optional federation is stated directly (116). Owner consent and ledger validity are separated from project governance and repository contributor rules (116, 173, 183).
- x402 is assigned a narrow role in HTTP service payment, with payment, delivery, predicate satisfaction, and retry identity kept separate (140). This is enough conceptual detail for a README.
- The local starting path is actionable: Node version, clone and demo commands, expected rejection behavior, structured output, source-profile identity, source checking, and package/API documentation pointers (146–167). The absence of signatures, proofs, transactions, and asset movement in these demos is explicit.
- Restricted Compact output is correctly separated from asset movement and full acceptance (60), and scoped Preview evidence is separated from arbitrary-source end-to-end support (171). I would not ask this README to contain complete deployment or API documentation.
