# Actual SP01 design check through the plugin

The public CLI executed the retained SP01 RP01-MC02 design verifier successfully. A separate GPT-6 review passed this scoped result. The runner used a unique precharged 120-second bound; the later review used 180 seconds. No network transaction was submitted.

The check compares four positive trace identities, complete state/effect references, thirty negative trace identities and exact integer arithmetic. It is a static design check. It does not execute negative runtime cases, an evaluator, a native proof or a financial transaction, and it does not satisfy the real repair pilot.

The current binding is `.moriarty-dev/loan-design-verification-binding.json`. Original accepted design bindings, freeze and acceptance bytes remain unchanged. Current accounting was copied under the existing master lock for this single execution and removed afterward. The debit remains consumed; this evidence does not authorize a replay.

`execution.json` and `reservations.json` preserve the parent-observed result and finished reservation. `review.json` records independent scope and limitations. Budget hashes in these records describe their historical observation, not the current balance.
