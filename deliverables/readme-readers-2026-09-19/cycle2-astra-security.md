# Cycle 2 security reader review

Verdict: CLEAR

Reviewer perspective: adversarial financial security engineer / ZK auditor. This is a full-text, independent reading of `cycle2-README.md` for material gaps, misleading guarantees and confusion between current behavior and targets. It is not an implementation audit or independent verification of the reported experiments. No earlier drafts, reviews or source/style briefs were inspected.

Required startup completed: read the checkout AGENTS.md and development skill, then ran guarded `status --json`. It reported unresolved execution admission/evidence gaps and no pending transactions. No campaign was run; those dispatch restrictions do not block this read-only review.

No material security communication flags.

- The opening explicitly says “the full proof and settlement path described here remains under development.” “Where the project stands” then distinguishes particular Preview executions from the missing general source-to-proof-to-complete-effects path. Its qualification that the September 17 payment left principal outstanding avoids presenting a partial payment as whole-loan closure.
- “The guarantee is limited to what the contract expresses and the evidence assumptions it names” correctly bounds provable intention. The external-evidence discussion separately distinguishes issuer authentication, actual truth, inclusion, application execution and finality.
- “Every accepted witness” and “complete financial relation” give the right adversarial boundary. The text explicitly rejects host checks, unused Booleans and labels as substitutes for native constraints or justified ledger enforcement, and includes effects retained after later failure.
- The authority/history passages cover replay, authenticated starting state, cumulative gross spending despite refunds, concurrent reservations, continuing liabilities, consent and private-position completeness. Recovery explicitly retains unknown outcomes and prevents refund/late-payment double discharge; compensation does not purport to reverse another ledger.
- “If a foreign account accepts a threshold signature alone” directly identifies the external custody bypass. The text does not suggest a Moriarty proof forces foreign accounts to enforce its relation, or that combining ZK, MPC and TEEs automatically supplies independent protection.
- Safety, privacy and availability are separated. Metadata leakage, missing private data, resource limits, storage growth and absent liveness are acknowledged without implying that bounded computation or recursion solves them.
- The local demos are clearly simulations that do not sign, prove, submit or move assets. The restricted Compact circuits are likewise identified as computing proposed effects rather than settling assets. The ZKIRv4 and recursion planning statements are explicitly proposals/assumptions rather than upstream delivery promises.

Optional editorial preference, not a finding: “Moriarty uses Midnight’s native proof system” could read “The native execution target uses Midnight’s proof system” to reinforce the implementation boundary immediately at that sentence. The surrounding native-path limitations already make the intended meaning clear; this does not warrant withholding CLEAR.

No exhaustive protocol specification is needed in this overview. The security obligations and present limitations are concrete enough for a reader to identify where further implementation and evidence review is necessary.
