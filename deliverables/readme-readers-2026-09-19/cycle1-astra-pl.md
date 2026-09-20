# Independent README reader assessment — programming-language researcher

Verdict: **CLEAR**

Reader scope: the full frozen `cycle1-README.md`, assessed for semantic precision, intelligibility, and claim scope. Required repository startup instructions and guarded status were inspected first. No implementation, linked specifications, source briefs, style profiles, or other reader reviews were inspected. This is a reading assessment, not verification of implementation claims.

## Principal assessment

The README gives me a coherent account of the proposed language: bounded state transitions governed by authenticated authorization, complete financial effects, and obligations that persist across transitions. It distinguishes exact-plan authorization from outcome-intent authorization (lines 26–28), per-stage termination from agreement completion (lines 32–36), authority from liability (lines 42–44), and reference semantics from compilation and ledger enforcement (lines 58–78). These distinctions are material to evaluating the proposal and are understandable without opening the specifications.

The claims are appropriately scoped. Experimental status appears before the design discussion (line 9). The text expressly limits finite K comparisons (line 62), separates restricted Compact generation from asset movement and full acceptance (line 60), and states what the runnable demos do and do not demonstrate (lines 157–159). The status section distinguishes retained scoped results from the unestablished general source-to-ledger path (lines 171–175). The proposed backend label and recursion horizon are explicitly identified as project requirements and planning assumptions (line 66).

The treatment of proofs is especially clear about the boundary of the claim: expressed intentions and evidence assumptions (line 20), adversarial witnesses (lines 70–72), complete effects (line 78), external attestations (lines 90–94), and safety versus progress (lines 98–102). I did not find an unsupported guarantee in the README's own framing. The details needed to evaluate a specific formal judgment or compiler theorem appropriately belong in the linked syntax/semantics reference and scoped evidence.

## Optional refinements — low severity, preferences only

- **Lines 26 and 72: overloaded “contract.”** The text uses agreement, source-profile contract, required contract properties, and financial contract. Context mostly resolves these uses, but “source profiles specify distinct language interfaces/semantics” would avoid making a profile sound like another financial agreement. A brief example of a “required contract property” could also make the first of the four proof obligations more concrete. Neither point blocks comprehension.
- **Lines 146–167: no visible source fragment.** A short fragment from an existing supported program would let a PL reader see how the language expresses a transition or surviving obligation before running the demos. The current commands and specification links are adequate entry points, so this is an optional presentation improvement, not a demand for more reference documentation in the README.

Necessary changes: **none identified**.
