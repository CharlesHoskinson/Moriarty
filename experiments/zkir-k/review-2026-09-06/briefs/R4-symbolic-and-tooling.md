# R4: the symbolic backend, the claims and the tooling

Focus: experiments/zkir-k/semantics/zkir-symbolic.k, the `[concrete]` attributes added in zkir-hash.k and zkir-curves.k, experiments/zkir-k/claims/*.k and README.md, tools/run_claims.py, the receipt zkir-k-claims-2026-09-06c.txt; and the tooling of this iteration: tools/upstream_drift.py, moriarty_preimages.mjs, preimage-json, moriarty_contexts.py, the Makefile, and the receipts zkir-k-upstream-drift, zkir-k-moriarty-contexts.

Questions:
1. Are the twelve claims stating what the README says they state? Check each claim's pre and post conditions against the rules: is any claim trivially true (a postcondition implied by the precondition, or a cell left unconstrained so that the claim proves nothing), and is any `[simplification]` lemma unsound (state a counterexample) or missing `preserves-definedness` where needed?
2. Does `[concrete]` on the hash rules make the hash symbol uninterpreted in the sense the README claims, and is functionality the only property available? Could the Haskell backend still unfold through a helper the attribute does not cover?
3. Is the compiler-obligation template (`spec_compiled_observable`) the right shape for the Moriarty specification to instantiate: what does a compiler proof need that it does not give?
4. Moriarty preimages: does moriarty_preimages.mjs produce the preimage the ledger would produce (scenario order, private state, witnesses, binding input and commitment randomness residuals), and is preimage-json's decoding of ProofPreimage faithful to the crate's serialisation?
5. Anything in the tooling that would make a receipt misleading (a silently skipped program, a default that hides a failure, an exit code that does not reflect the summary).
