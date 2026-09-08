# Credit partial-source review

**Source verdict: BLOCKED. Task/provenance verdict: BLOCKED (author timeout124).**

Candidate aggregate `13a2452a313be2c732af8c315338d4c75c0ce2de9140426f86ea320fc6f82c74`; source SHA256 `2733bf69fd4fc87342e03f6c99f5fe7766dd71c360975221d247d766d44b3200`. All three owned files and input packet match before and after review. Source bytes420187; total owned bytes434564.

Reviewer is system-described Codex based on GPT-6. Exact serving model/build and reasoning setting are unavailable. Author serving identity is also unavailable; requested model wording is not identity evidence.

The twelve cash transitions, authority/debt equations, work totals, immediate predecessor heads, all thirteen exact rollbacks and all35arithmetic values pass the independent bounded data replay. All12complete changed-path lists match (369leaf occurrences). Full financial pre/post records and all5rows/5traces/13mutation IDs remain unchanged from candidate02.

## CREDIT-HISTORY-PARTIAL-01: Primitive relation does not reconstruct complete states and leaves guard timing ambiguous

Independently applying the listed primitive equations and full-record replacements to each actual step preState matches the financial records, balances and head updates. All twelve COMPLETE post comparisons still fail. workCharge defines only ordinaryCharged and ordinaryRemaining, while snapshots append charges and change conservationOrdinary/perActionMaxObserved and introduce reserve/copy/bound metadata. No primitive creates/changes/removes committedPrefixIndex, dutiesConsumedThisPrefix or positionDeleted. The frame says every unnamed path remains equal. FullPostEquality has been qualified to 'core fields equal postState', which does not satisfy the requested complete equality.

Paths: `/library/primitiveSemantics/ops/workCharge`, `/library/primitiveSemantics/ops/historyProduce`, `/library/oracles/refinance/primitiveUpdates/8`, `/library/oracles/refinance/primitiveUpdates/17`, `/library/oracles/redemption/primitiveUpdates/1`, `/library/oracles/redemption/primitiveUpdates/12`, `/library/oracles/refinance/acceptance/TransitionValidity/checks`.

- refinance step 0: pre work has only lifetimeOriginal=1024, ordinaryRemaining=1024, recoveryReserveRemaining=16. Equations cannot create charges[0], reserve originals or perActionBound; committed state requires all of them.
- refinance step 2 debtWrite comes AFTER transfers 50000 and 50 debit the funded bridge from 50050 to 0, yet its guard says bridge.balance.USDC >= 50050.
- redemption step 0 shareNominalWrite follows the 100-share debit, so alice balance is 0 when its guard requires >=100; step 2 requestWrite follows the 3800 debit, so vaultLiquid is 0 when its guard requires >=3800.
- Some other instructions explicitly require post-transfer values, so applying all guards uniformly to the step-entry state is not a specified repair.

Required correction: Define exact step-entry versus sequential-current guard references; give every bookkeeping field a constructor/update/removal rule or explicitly exclude it from a separately typed annotation envelope. Specify defaults such as ordinaryCharged=0. Require full expanded ledger-state equality and frame for every committed step, with the final claim selecting its precise step-3 update slice.

## CREDIT-HISTORY-PARTIAL-02: Registry, budgets and four-claim acceptance still contain unbound assertions

Full preState references, registry entries and hard-constraint witness references are improvements. However the registry passIf checks only key presence/current/non-revoked; it does not compare the referenced entry's spec/oracle or bounded flag with the claimed pair. total-verification-work has max=1024 and a profile string but no used quantity or derivation. dependencies.used=12 is unbound while row dependencies are empty. Several financial checks are literal equations or IDs with no path predicate, for example usdcSum 300000=300000 and oldDebtZeroOnlyWithT-repay-old. Source/environment/authority/history/currentness are not an explicit shared acceptance conjunction for all four claims; most individual claim lists omit several. Merely including references or a boolean does not define those comparisons.

Paths: `/library/toyRegistry`, `/library/preVerificationBudgets/inputs`, `/library/oracles/refinance/acceptance/ContractInvariant/checks`, `/library/oracles/refinance/acceptance/HistoryCompliance/checks`, `/library/oracles/badDebt/acceptance/ContractInvariant/checks`.

- Change registry.entries[0].spec to a different spec, or bounded=false, leaving key/current/revoked unchanged: the written passIf remains true.
- No comparison can evaluate total verification work <=1024 because used work is not an input to the budget record.
- Changing an actual post-state USDC balance does not change the literal operands of usdcSum.

Required correction: Define a shared admission predicate over full current context, exact registry tuple including spec/bounded/current/revocation, actual dependency/claim/sidecar/work counts and their bounds. Conjoin it with each of the four claims or at a clearly binding outer acceptance rule. Express financial constraints and genesis/admin admission as comparisons to resolved state/effect paths, not constant equalities or check names.

## CREDIT-HISTORY-PARTIAL-03: Complete changed-path lists still omit semantic reads from footprints

All twelve declared changedPaths lists exactly match an independent flattened complete-state diff (369 changed leaf occurrences). Distinct-resource and access bounds now separate work writes=4 and bridge writes=2. But a diff does not enumerate reads of unchanged inputs. All frameReads arrays are empty. Redemption step 1 requestWrite requires vaultLiquid.balance.USDC==3800 and conversion=95, neither in that step's reads. BadDebt step 0 reads debt total, collateral quantity/value and price to derive 600, but reads list only history/work. BadDebt step 2 requires vault.balance.USDC>=400 but does not list it. Refinance Always witness reads price, collateral and debt on observable prefixes, absent in step-0 reads. Registry, source and budget inputs also lack an explicit separate admission footprint.

Paths: `/rows/0/boundedFootprint/perStep/0/reads`, `/rows/1/boundedFootprint/perStep/1/reads`, `/rows/4/boundedFootprint/perStep/0/reads`, `/rows/4/boundedFootprint/perStep/2/reads`, `/rows/0/boundedFootprint/frameReads`, `/library/completeStateDiffs`.


Required correction: Derive reads from primitive guards, invariant/refinement witnesses, frame equality and admission predicates, not changed paths. Add the concrete unchanged resources and all admission reads, resolve every resource to deterministic state paths, then recompute per-step/aggregate counts and the explicitly chosen finite profile. Existing abstract 60/50/56 charges may remain only with an explicit cost rule covering those reads.

## CREDIT-HISTORY-PARTIAL-04: Funded external boundary has a recursive state reference and no admitted capability source

Cash conservation, distinct successor head, replay-before-exactGoal order and preserved counters are corrected. But the exact ref convention recursively substitutes ref-only objects. Boundary.postState references fundedState, whose externalBoundary references the boundary; expansion never terminates. Its admitted preState is redemption.postState and contains no vault.deployedToLiquid.USDC grant. The new post state adds a capability already consumed 3800/3800/0 without a constructor, authenticated external grant input or authorized issuance predicate. Named sourceController/capability strings and a resource-name list alone do not define that admission. Mutation 5 still labels source/environment/authority/currentness prerequisites pass=true without the missing concrete predicates.

Paths: `/library/externalBoundaries/redemptionReplenish/postState`, `/library/committedStates/redemptionFundedLaterAfterFirstClaim/externalBoundary`, `/library/committedStates/redemptionFundedLaterAfterFirstClaim/boundaryCapability`, `/library/externalBoundaries/redemptionReplenish/preState`, `/mutations/5/mutatedContext/admissionOrder`.

- Expansion cycle: /library/committedStates/redemptionFundedLaterAfterFirstClaim -> /library/externalBoundaries/redemptionReplenish -> /library/committedStates/redemptionFundedLaterAfterFirstClaim.
- Input authorities contain only Alice request lock/fill grants; boundaryCapability first appears after it is spent.

Required correction: Use a non-expanding boundary identifier or separate annotation type to break the cycle. Supply a finite complete external admission context with the controller's existing capability or an explicit authenticated toy issuance assumption and consumption rule. Bind transfer, all changed fields, history, resource counts and costs to that context; derive replay prerequisites from it. Zero external ordinary cost is an explicit toy choice and is not by itself a blocker.

## CREDIT-HISTORY-PARTIAL-05: Missing-new-debt-authority control removes authority only from its attempted result

The selected actual prefix-2 preContext contains alice.newNominalDebt.USDC with original=50500, cumulativeGross=0, remaining=50500. The attempted result drops that authority record and creates debt 50050 naming that same grant. Thus a guard over actual pre-authority remaining >=50050 passes; the post-state instead violates authority consumption/frame rules. This can be an invalid candidate, but it does not isolate an absent pre-grant as the advertised UnauthorizedNewDebt predicate states.

Paths: `/mutations/3/preContext`, `/mutations/3/mutatedContext/removedAuthorization`, `/library/committedStates/refinance/afterStep2/authorities/3`, `/library/attemptedStates/mut3/authorities`.


Required correction: Either admit a distinct full pre-context in which the grant is absent and bind its valid lineage, or define an explicit supplied grant/use witness that is absent while the authenticated ledger state remains fixed. If this is intended to test authority-record erasure, name and bind that predicate instead. Preserve rollback to the selected admitted context.

## Prior findings and case limits

- CREDIT-RESULT-01: RESOLVED in stored heads/transcripts and exact rollback. All12current successor heads consume immediate previous head, final acceptance binds prefix2, all13rollback states exact; separate transcript maps genesis.
- CREDIT-RESULT-02: PARTIAL/BLOCKED. Full financial record constructors and witness refs improved; financial records replay. Complete primitive/frame/guard semantics and bound conjunctive admission remain incomplete.
- CREDIT-RESULT-03: PARTIAL/BLOCKED. All12state diffs exact and write/access counts corrected. Semantic reads still omitted; allframeReads empty.
- CREDIT-RESULT-04: PARTIAL/BLOCKED. Separate funded head and explicit early replay order fixed; residual debt/duty removal and authority-first abort fixed. Replenishment expansion and grant admission remain incomplete.

All five positive traces remain blocked for source acceptance despite passing cash/work/head data. All thirteen negative-case rollback comparisons pass. Case dispositions:

- 0 `heldouts:accepted-refinance:mut:discharge-without-settlement`: Settlement-omission predicate is numerically witnessed; exact rollback passes. General primitive/claim blockers remain.
- 1 `heldouts:accepted-refinance:mut:fee-untransferred`: Fee-omission predicate is numerically witnessed; exact rollback passes. General primitive/claim blockers remain.
- 2 `intent-cases:refinance:mut:temp-uncollateralized`: Observable zero collateral versus debt50000 violates 75000 requirement; proposed two moves also exceed gross20. This is not an isolated single-failure authority control.
- 3 `intent-cases:refinance:mut:new-debt-without-authority`: BLOCKED diagnostic isolation: actual pre-grant remains50500; see CREDIT-HISTORY-PARTIAL-05.
- 4 `heldouts:pending-redemption:mut:erase-claim-on-illiquidity`: Burn/claim erasure is inconsistent with residual obligation; exact rollback passes.
- 5 `heldouts:pending-redemption:mut:replay-claim`: BLOCKED external admission and recursive expansion; cash3800 is funded and zero-claimable failure is ordered before unreachable exact-goal13300.
- 6 `intent-cases:pending-redemption:mut:skip-to-settled`: Pending-to-Settled skip is explicitly forbidden; exact rollback passes.
- 7 `intent-cases:pending-redemption:mut:drop-exact-floor`: 3800 is not the9500 exact goal and5700 residual cannot disappear; exact rollback passes.
- 8 `DeFi-regressions:bad-debt:mut:erase-or-omit-socialized-loss`: Unrecovered debt erasure and omitted loss are invalid; this mutation intentionally changes multiple conditions.
- 9 `DeFi-regressions:bad-debt:mut:discharge-residual-without-auth`: SCOPED RESOLVED for authority-first rejection: debt0/0/0 and duty removal agree, authority is absent, later checks explicitly unreachable. AuthorizedShape is not independently accepted as a complete valid successor.
- 10 `heldouts:pending-redemption:mut:replay-claim-unfunded`: SCOPED RESOLVED unfunded control: have0 need3800; funded-source precedes replay.
- 11 `DeFi-regressions:bad-debt:mut:nominal-shareholder-changed-to-buyer`: SCOPED RESOLVED ownership mismatch: nominal buyer versus shareholders token account100.
- 12 `DeFi-regressions:bad-debt:mut:stale-current-head`: SCOPED RESOLVED stale-head mismatch and exact ledger rollback; attempted envelope is separate.

## Evidence and scope

The JSON report retains both independent Python scripts, outputs, hashes and per-step mismatches. Literal primitive replay reconstructs financial records but all12complete-state comparisons fail on unspecified metadata changes. The replay uses a generous ordinaryCharged=0 default and does not execute prose guards. Extra guard ordering failures are source counterexamples, not hidden by those replay results.

484ref-only pointers resolve individually; recursive expansion of the replenishment boundary fails with a demonstrated cycle. The finite expansion check is therefore not passed. No proof follows from names, booleans or example equalities.

Initial reviewer commands failed on unavailable python (127) and the envelope-only stale-state debt lookup (1); both were corrected locally without source edits. These attempts count in actual elapsed time.

Started 2026-09-08T08:45:14+00:00; finished 2026-09-08T08:54:50.280734+00:00. Native allowance600seconds; charged600seconds; actual elapsed 576.281seconds; overrun 0seconds. No cgroup enforcement claimed.

Only the two requested review reports were written. No project tests/compiler/build/runtime, network, wallet, git mutation, source edits, agents, provider private reasoning or proofs. No full-map, language, K, proof, runtime, ledger, Preview or sprint acceptance.
