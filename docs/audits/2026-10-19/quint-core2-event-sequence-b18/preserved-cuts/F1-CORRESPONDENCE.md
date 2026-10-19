# F1 action-form correspondence

This candidate copies the exact fallback supplied by the prior full fresh Opus review after the actual frontend settled F1 as a failure. The old source and actual failure remain immutable; no five-step run occurred. Only init and advanceInput changed. Imported actions and their assignments are byte unchanged.

For init, the old guard returned false exactly when the queue was empty; the new first all member is false in the same case, before head/initInvocation evaluation. For advanceInput, the old disabled condition was phase not Published OR record length zero OR length at least queue length. The new first three guards are the exact conjunction of its negation. The same last-record/next-queue continuation and context checks precede the same beginInvocation assignment. All nth/head accesses occur only after their existing bounds guards pass.

This equivalence uses installed TypeScript evaluator source: builder.js builds argument evaluator closures, passes those closures to lazyBuiltinLambda for lazyOps; lazyOps includes actionAll; actionAll loops in argument order and immediately returns false/restores its snapshot before later arguments. User-defined arguments are eager only when their own evaluator closure is invoked. Thus no head/nth user-function argument is evaluated before an earlier false all guard. Raw source excerpts are supplied; no model execution supports this statement yet.

No semantic/cost/profile/seed/prefrozen expected/state-schedule change is intended. Five transitions/six complete states, init/sequenceStep/sequenceInv/allInputsPublishedWitness, one TypeScript sample seed1 remain selected proposal. Fresh source/op review and actual frontend/run are still required; no passing result is asserted.
