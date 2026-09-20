**APPROVED (reader: FLAGS)**

The README is publishable as written. No factual, semantic or claim-status defect blocks release. The design/evidence boundary is stated consistently: target design in the body, implementation status confined to the final section, ZKIRv4 and the March 2027 horizon explicitly marked as a proposed label and a planning assumption dated today. The security model's four obligations, the binding requirements and the external-trust caveats read as accurate and mutually consistent. Use cases are correctly framed as illustrations, not delivered applications.

**Structure and fidelity checks passed**

- Opening promise delivered: the intro says the final section separates evidence from remaining work, and "Where the project stands" does exactly that.
- ZKIRv3 as current target and ZKIRv4 as "next-version" workstream do not conflict.
- K definitions are described as finite-case evidence, not a compiler correctness theorem. Jets, underconstrained circuits and recursion composition are stated with correct limits.
- The kernel section correctly denies the kernel any right to weaken agreements and correctly limits what a Moriarty proof can force on a foreign account.
- The CAKE reading (Permission as user/application authority, not deployment licensing) matches the framework's published layering.

**Target-reader flags (medium; not blocking)**

1. "In CAKE's Applications, Permission, Solvers and Settlement model" — the acronym arrives with no expansion or attribution, so I had to infer from the four nouns that this is an external chain-abstraction framework being mapped onto, rather than a Moriarty component. Minimal fix: name it once as an external framework (for example, "the CAKE chain-abstraction framework's ... model"). — severity: medium

2. "The local protected financial evaluator is part of the language implementation." — this term appears for the first time here, in a paragraph contrasting it with the kernel. Nothing earlier established what the evaluator is, so I paused to work out whether it was the stage evaluator described under "Bounded computation" or something new. Minimal fix: a brief appositive tying it to the local evaluator already implied by the bounded-stage semantics, or drop "protected" if it carries no distinct meaning here. — severity: medium

**Optional, not requested for action**

- "Open Wallet Standard" and "x402" are unexplained on first use, but each sentence supplies enough surrounding context (wallet interface; HTTP service payments) that comprehension did not break for the stated audience.
- "The target does not impose one fixed depth" uses "the target" for the target design; it is recoverable from context.

**Verdict**

Publish as is, or apply the two one-phrase clarifications above. Neither changes any claim, equation or status statement, and neither warrants reopening the editorial cycle.