# Symbolic MC02 projection semantics proposal

Design input only,2026-09-11. No grant, authority vote, implementation or live mutation. Companion JSON uses symbols rather than fabricated live grant values.

## Preserve the master owner

Keep `package:MC01`, every existing `charges` row, planning/overhead, MC01 package limit117710 and MC01 worker52/52 untouched. Existing external charge rows retain their original `package` attribution. Append MC02 new-grant debits only to `externalPackageCharges`, each with packageMC02, exact grant/charge/action/candidate/runner identity. Do not move or relabel old MC01 entries into MC02. Top-level counters retain their established meaning: MC01 worker count52, external worker count31+n, aggregate count83+n. The amendment extends aggregate limit83+N; MC01 worker limit remains52.

Select a new immutable-grant-bound `successorEnvelopes` entry whose owner is MC02 and whose counters describe only the new grant. Its `remainingReservedSeconds=G-S`, `workerDispatches=n`, `workerDispatchLimit=N`. G is the exact new seconds allowance; S is its committed usage. This is the entire active MC02 package allowance, not a claim about lifetime MC02 consumption or its previous allocations. A reviewed disposition closes older MC02 allocations and prohibits transfers into this grant. Unknown older costs remain unknown in immutable referenced dispositions.

## Existing fields and precise arithmetic

Let U be represented prior master used/reserved seconds, M its prior ceiling. Add historical-credit-closure row C=ceil(max(0,M-U)), with the exact closure-only basis required by the specification. Retain all other old reserves; no old grant can spend them. C closes credit, not paid costs. For the inspected values C=7 and U+C exceeds M by about0.04733seconds: adding G to M alone would leave the fully reserved new grant marginally overdrawn.

The concrete amendment must explicitly disposition that rounding deficit. One exact integer construction is D=max(0,ceil(U+C)-M), masterLimit=M+D+G. Here D=1 would be a *reviewed closure reconciliation increment*, not spendable historical credit. Record why the extra integer arises; do not silently forgive a deficit. If reviewers decline D, reduce the new effective reserved grant accordingly or reject. A genuine larger deficit requires explicit separate disposition, not the assertion that this is mere fractional rounding.

Reserve G once in a new `reserved` row. On an admitted debit costc, atomically transfer c from that reserve to its new MC02 external charge; S increases byc, reserve becomesG-S. Master used remains U+C+G, independent of that transfer. Selected grant capacity is G-S. The master may have less than1second numerical headroom after ceiling conversion; the independently enforced envelope blocks spending it. No active action can consume old reserves or the closure margin.

The current consumer totals master reserves before dispatch, so subtracting c again from master slack during a pre-debit admission would wrongly deny a funded reserved grant. Existing spec order already solves this: producer validates ownership of its reserve under lock, commits transfer/debit, then final reader sees a genuinely prepaid action and subtracts no cost. Do not demand final prepaid validation before creating the debit or skip it afterward.

## Minimal consumer change

Existing `_remaining_from_current` compares top-level package to campaign owner and uses `planning+overhead+charges` plus top-level worker counts for package checks. That describes MC01 in this master and cannot truthfully govern an MC02 action unchanged. It also checks worker exhaustion only for implementation kinds, while the live action is verify.

For only the exact `finalized-financial-settlement` profile, call the new accounting module’s validated current-projection routine. It must bind the immutable reviewed projection/grant to this action/campaign and identify MC02 as the selected envelope owner while MC01 remains ledger owner. It returns the exact active package/grant limit and committed usage, actual aggregate limits and matching prepaid ownership. The reader then uses these returned values rather than top-level MC01 package/worker values for the MC02 action. All ordinary master totals still include every preserved row. Legacy profiles keep their existing semantics; missing concrete disposition rejects `resource-package-projection-unresolved`.

No new top-level field family is needed: grant/projection provenance can be in the existing envelope/charge entries and exact immutable resource admission references, validated by accounting.py. Optional old `passBinding` is not an arbitrary free-form authority container; do not smuggle new unsupported fields through it. The active grant reference must be authenticated by the resource evidence, not chosen from attacker-supplied envelope names.

## Prepaid final boundary

Before a new debit: S+c<=G; n+1<=N; aggregate count plusone<=old83+N; exact action costs and all other attempt/financial bounds pass. After the final debit, S may equalG and n may equalN. That one exact not-yet-claimed prepaid invocation must still launch once: require n<=N and aggregate<=limit, matching debit fields, no conflicting/currently claimed owner, and retained original charge bound. The zero remaining grant does not mean this already-paid invocation lacks authority. Any second debit or replay fails. Store reservation and master one-shot claim must still agree.

Unknown historical costs are not assignedzero. The reviewed closure disposition must explicitly establish old allocations as nonspendable closed history and that unknown old costs cannot create credit or race against this active grant. If their applicability to the active envelope remains unresolved, fail closed. This proposal expresses an exact new allowance alongside preserved history; it must not be presented as lifetime-global accounting reconciliation.
