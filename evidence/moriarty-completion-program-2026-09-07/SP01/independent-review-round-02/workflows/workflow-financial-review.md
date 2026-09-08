Verdict: **BLOCKED**. Independent GPT-6 Astra high financial-content review.

Frozen artifact: `/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/workflows/interrupted-workflows.json`
SHA256 before and after: `2f4f45e6d693eeec72fb621a5853c4f4c7a6bc9e1acfc9cee577f796f48e79c1`
Input packet SHA256 before and after: `f6f186ca7c3cb91b54cc18882baf4c88a83f203f18eb115a640a8149e9de5a84`

All 24 positive steps and 24 transfers reconcile with funded account balances. Ordinary/reserve work totals are 40/0, 46/0, 20/0, 22/0, 14/8, 24/0 and 28/0. Seven row identities, seven trace identities, nine original mutation identities and exact row source objects are preserved; all 19 mutation base references resolve. Local source hashes match.

**WF-F01: Custody negatives do not start from an admissible underfunded state**

Pointers: `/mutations/10/base`, `/mutations/10/mutatedInput/rewriteInsurerBalanceTo`, `/mutations/10/unchangedState`, `/mutations/18/base`, `/mutations/18/mutatedInput/rewriteInsurerBalanceTo`, `/mutations/18/unchangedState`

The selected contingent base holds insurer 20500; 10000 payout is funded and leaves 10500. The insurance base holds insurer 15200; 8000 payout is funded and leaves 7200. The proposed balance rewrites destroy 20400 and 15200 respectively without transfers. Conditional text if rewrite rejected explicitly leaves the targeted custody check unisolated. Contingent C1 also lacks payout authorization in this trace.

Required fix: Use a complete alternate admitted context with insufficient actual custody, conserved funding provenance, valid prior premium/adjudication and authorized payout, and all earlier admission predicates satisfied. Reject solely on balance<payout and bind the entire rejected result to that alternate base. For C1 either supply an explicit toy authorized payout context or retain forbidden-payout coverage separately; do not call the existing rewrite a custody test.

**WF-F02: Partial-fill receipt checks exclude fees from the net goal**

Pointers: `/traces/0/canonicalIntent/hardConstraints/minReceive`, `/traces/0/steps/1/minReceiveCheck`, `/traces/0/steps/2/minReceiveCheck`, `/traces/0/acceptance/IntentRefinement`, `/verification/7`, `/verification/8`

Checks use 3000/2=1500 and 2000/2=1000 while the financially charged debits are 3015 and 2015. Fee-aware thresholds are 3015/2 and 2015/2 milliWETH, equivalently integer cross multiplication; the positive 1510 and 1010 still pass.

Required fix: Specify minReceive over total economic debit including input-denominated fees. Use 2*received>=debit+fee, or ceilings 1508 and 1008 at the integer unit. Update acceptance and verification entries and add a fee-sensitive negative with a fully balanced proposed result.

**WF-F03: Cumulative authority and unsettled custody are inconsistent or conflated**

Pointers: `/traces/4/preState/allowances/lock/cumulativeLocked/amount`, `/traces/5/steps/0/postState/allowances/messageAmount`, `/traces/5/steps/1/postState/allowances/messageAmount`

Cross-domain pre-lock state reports cumulativeLocked=1000 even though no lock has occurred, the vault has 0, messages are empty and status is pre-lock. Async states report original=400,cumulative=400,remaining=400; as an allowance this sums to 800 against 400. If remaining means unpaid escrow rather than unspent authority, that distinction is not declared.

Required fix: Set cross-domain cumulativeLocked to 0 before locking and 1000 after locking; retain 1000 after refund. Define async cumulative as consumed send authority and remaining as unspent authority, giving 0/400 before send and 400/0 thereafter. Track unsettled escrow separately as 0,400,400,0 across pre/send/final/receive. Alternatively explicitly define and name a different coherent ledger equation. Do not restore send authority at settlement or refund.

**WF-F04: Expected financial effects contain an unfinished contradictory net value**

Pointers: `/traces/0/expected/effects/tokenTransfersNet/alice.USDC`, `/traces/0/expected/effects/aliceNetUSDC`

The first value is the string -5030+4970-wait while aliceNetUSDC correctly says -5030. This is a financial expectation, not whitespace packaging. Reapplying refund to net spend would give -60 rather than -5030.

Required fix: Replace the unfinished field with a typed -5030 net amount or remove the duplicate expectation and bind its reference to aliceNetUSDC. State the economic boundary for escrow gross debits explicitly; refunds must not reduce cumulative fill-plus-fee debit 5030.

**WF-F05: Four mandatory claims and rejection outcomes do not bind the complete financial record**

Pointers: `/traces/0/acceptance`, `/traces/1/acceptance`, `/traces/2/acceptance`, `/traces/3/acceptance`, `/traces/4/acceptance`, `/traces/5/acceptance`, `/traces/6/acceptance`, `/mutations/0/unchangedState`, `/mutations/18/unchangedState`

All four claim names are preserved but their prose summaries lack an explicit universal binding to complete pre/post states, effects, request/controllers, empty debt/share records, allowances, observations, duties and successor/history records. Genesis/admin are two history labels and a profile name, without complete admitted financial records or an explicit boundary treating the entire preState as their fixed result. All 19 unchangedState values are partial strings; financialStateUnchanged=true does not specify full result equality or admission-stage preconditions.

Required fix: For each trace define specified-only predicates over complete canonicalIntent, concretePlan, preState, each step transfer/nominal change, observations, entire postState, and complete genesis/admin admission context. Include frame conditions for untouched financial fields and controller/authority lineage. For each negative supply explicit expectedStateRef equality to the complete selected base (or a complete alternate fixture), rejected effects/fees, and satisfied earlier-stage predicates. Split disjunctive proposed inputs when claiming coverage of each alternative. These are design predicates, not proof claims.

**WF-F06: Concrete financial footprint bounds and state references remain incomplete**

Pointers: `/rows/0/boundedFootprint/writes/9`, `/rows/1/boundedFootprint/writes`, `/rows/2/boundedFootprint/writes`, `/rows/3/boundedFootprint/writes`, `/rows/4/boundedFootprint/writes/4`, `/rows/6/boundedFootprint/writes/3`, `/rows/2/representationPath/3/pointer`, `/rows/4/representationPath/3/pointer`, `/rows/5/representationPath/3/pointer`

RR-N7 changes on fill1, fill2 and cancel (3 writes, bound 2). D-cover-C9 changes at premium, adjudication and payout (3 writes, bound 2). DC-M1 is materialized then expired (2 writes, bound 1). Recurring duty/status/aggregate updates, delegated allowed-effect grant, and stored contingent observations are absent from write footprints. expected.duties does not exist in traces 2,4,5 although representation paths point there.

Required fix: Define whether bounds count accesses per trace or unique records, then enumerate every changed financial resource consistently. For the currently implied per-trace access convention use at least 3 writes for RR-N7 and D-cover-C9 and 2 for DC-M1; include allowance, duty, status, observation and authority writes. Replace missing expected.duties references with complete postState duties paths or explicit expected empty lists. Reconcile bounds with actual finite steps.

**WF-F07: Contingent premium declares creation of a cover absent from complete post-state**

Pointers: `/traces/3/steps/0/nominalChanges/0`, `/traces/3/steps/0/postState`, `/traces/3/steps/1/postState`, `/traces/3/steps/2/postState`

The premium step declares field cover, id C1-cover, premium-creates-cover-not-cash. No cover record exists in any complete postState. Later C1 admission has premiumPaid and observation but no persistent C1-cover identity or policy binding to consume or preserve.

Required fix: Represent C1-cover explicitly with principal, insurer, premium status, conditions, controller and lifecycle, carry it through observation/admission, and bind the claim to it; or define an explicit alternate cover representation and make the nominal effect point to that real record. Keep cover, nominal claim and actual payout distinct.

The JSON report records each negative, exact equations and counterexamples. The existing positives still pass fee-aware receipt bounds; the understated checks admit a 1501 milliWETH first fill although its required fee-aware minimum is 1507.5. The custody negatives have funded bases and cannot establish underfunding through unexplained balance destruction.

Author timeout and incomplete packaging are separate acceptance limits. This report does not accept a future artifact, full SP01, semantic freeze, runtime, proof, chain activity, deployment or twelve-sprint completion. Toy protocol assumptions remain disclosed and unproved. No network, build, proof, wallet, transaction or source edits were performed.
