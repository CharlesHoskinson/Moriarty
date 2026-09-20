**Verdict: APPROVED** (target reader: **CLEAR**)

I read the full README once as a PL/DeFi developer evaluating the language, the optional kernel, the proposed requirements and the current formal semantics. No publication-blocking defect found. Structure, security-model framing and claim status are internally consistent.

**Fidelity checks that pass**

- **Claim status is preserved throughout.** The opening states the README describes target design; the final section separates repository evidence (parsers, evaluators, scoped K definitions, scoped Preview loan/swap results) from unestablished work (language-to-ledger correspondence, mandatory native proofs, recursion, private handoff, conditional settlement recovery). Use cases are explicitly labeled design illustrations.
- **ZKIRv4 and the March 2027 horizon** are correctly marked as a proposed requirements label and a planning assumption recorded on 2026-09-19, matching the link title "proposed ZKIRv4" and today's date.
- **Security model** is coherent: four obligations (contract properties, refinement of authenticated intention, valid transition and effects, compliant history), each requiring a native enforcement point; binding to program, profile, intention, domain, stage, state, effects, circuit and verifier identity; replay, unique consumption, authenticated genesis, inherited cumulative limits; explicit non-guarantees (profit, honest oracles, ordering, settlement).
- **Kernel section** correctly keeps the kernel optional, states direct Midnight programs need no federation or approval, and gives the CAKE layers (Applications, Permission, Solvers, Settlement) accurately with the correct reading of "permission."
- **No Lean dependency; K executions are finite-case evidence, not a compiler theorem.** Stated accurately.
- The requirement paragraphs are not duplicated here; the two references carry them.

**Non-blocking precision notes** (optional, not required for approval)

1. **"Midnight as the execution target," first sentence:** "target its native ZKIRv3 execution and proof infrastructure." ZKIR is Midnight's circuit intermediate representation for proof generation; on-ledger execution is the ledger's transcript/VM layer, not ZKIR. If the requirements page uses the same phrasing, leave it for consistency. Otherwise a minimal correction is "target its native ZKIRv3 proof infrastructure and ledger execution."
2. **Kernel section, final paragraph:** "The local protected financial evaluator" appears once without prior introduction. As a reader I inferred it means the local evaluator from the final section and recovered, so this is below flag threshold. If the term is defined in the requirements page, no change needed.
3. **"ZK" and "MPC"** are used as abbreviations in the kernel section after "Zero knowledge" and "multi-party computation" appear in prose. The audience will follow; no change required.

No grammar, equation or claim-status changes are needed. The document is ready to publish as-is.