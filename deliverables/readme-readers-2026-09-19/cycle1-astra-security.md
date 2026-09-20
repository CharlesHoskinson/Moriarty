# Cycle 1 — financial security and ZK reader

**Verdict: CLEAR.** No required changes identified.

Persona: adversarial financial security engineer and ZK auditor. This is a full-text reader assessment of the frozen `cycle1-README.md`, not an implementation audit or confirmation of its factual implementation claims. No other reviews, source briefs or style briefs informed the assessment. Required repository startup instructions and development skill were read, and the guarded status command was run; no campaign was dispatched.

## Assessment

The overview gives a coherent security boundary without promising that the current implementation already enforces the target relation. In particular:

- **Current versus target:** Line 9 says “the full proof and settlement path described here remains under development” and denies production SDK or audited-deployment status. Lines 60, 159 and 171 make the limitations concrete: restricted kernels do not move assets, the demos are local simulations, and retained Preview results do not establish arbitrary supported source-to-ledger correspondence. Lines 173–175 identify the missing enforcement work. I would not read the intervening normative “must” statements as claims of completed security.
- **Proof soundness and binding:** Lines 70–78 demand constraints on malicious witnesses, identify four distinct acceptance obligations, reject host checks as substitutes, and bind proofs to program, verifier, authenticated intention and complete effects. Line 52 expressly warns that honest testing does not exclude underconstrained circuits. These passages avoid the common mistaken inference that a successful proof alone establishes financial safety.
- **Financial failure modes:** Lines 42–44 distinguish surviving debt from authority and separate gross spending, fees and net results, including concurrent reservations. Lines 82–86 cover genesis, replay, consent, recovery scope, hidden liabilities and unavailable witnesses. Lines 94 and 128 preserve unknown external outcomes and reject assumed cross-chain atomicity. These are substantive boundaries, not merely a generic risk disclaimer.
- **External trust and custody:** Lines 90–92 distinguish inclusion, successful execution, finality and issuer assertions. Lines 112–114 distinguish ZK, threshold signing and TEEs, name correlated infrastructure risk, and explicitly warn that compromising a foreign account's signing threshold can bypass policy. “A Moriarty proof cannot force that account to verify a relation its native rules do not require” directly answers the most consequential federation trust question.
- **Limits:** Lines 98–102 separate privacy, safety and availability and deny guarantees of oracle honesty, profitability, optimal execution, ordering protection or eventual settlement. Line 20 also limits proofs to expressed preferences and named evidence assumptions.

## Optional clarification

**Low severity; optional.** Line 62: “Midnight's native PLONK/KZG stack is the proof foundation for this design.”

An explicit sentence that proof guarantees inherit the pinned backend's cryptographic, setup and verifier assumptions could help readers distinguish relation correctness from the soundness of the proof system implementing it. The overview currently explains application and federation trust more explicitly than cryptographic trust. This is not a required change: it neither claims unconditional cryptographic security nor needs to reproduce the backend's security specification. Any added wording should refer to the actual backend's documented assumptions rather than invent a project-specific setup model.

No other omissions or current/target ambiguities rise to a required correction for this overview.
