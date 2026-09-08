Markets financial result review: BLOCKED

Candidate `09d5616879f11b2212a5babd0afb076c091015a85df97144603e2c83028d1b9e`, 390919 bytes. Scope: corrected specified-only design fragment. This is not runtime, proof, full-map, semantic-freeze, Preview or sprint acceptance.

All six positive cash ledgers reconcile. All 16 step transfer lists match their plans and effects. Exact output19743 (remainder162290000), CL input477/output4882/fee1, AMM D1999/y850/output50, redemption burn150/gross300/fee3/net297, shared vault10/9/1, and margin75 reproduce independently. All eleven negative ordinary-work charges conserve original1024 and preserve reserve16.

MARKET-FR01: Materialized positive and rejection states contradict their complete-state definitions

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/expected/postState`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/3/expected/postState`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/5/expected/postState`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/4/expectedFinancialState/accounts`

- Independent deep-copy plus patch.replace replay gives seven CL differences. initializedTicks.100.initializedLiquidityBound and .110.initializedLiquidityBound are true in preState and not deleted by patch, but absent from materialized postState. The patch authority.nominalCaps uses original=0,cumulative=1,remaining=1; the materialized nominal cap instead uses pre=0,post=1 and notAGrossDebitCap=true. These are substantive different records.

- Other CL differences are pool.activeLiquidityNote, tick100.feeGrowthOutside0Note, positions.LP1.note and inventoryToyPost. Redemption principal.note and margin positions.P1.note/principalAccrualNonDoubleCount also differ from the deterministic patch result. Even notes count under the explicitly stated full field equality.

- CL ContractInvariant requires postState.duties == effects.duties. The effect duty contains id/amount/asset only, while the post duty adds to=lp, pending=true, survivesPartialProgress=true and note. Equality is false.

- Redemption mutation expectedFinancialState.accounts is an old full copy lacking the burnSink nonspendable policy fields retained in rollback.completePreState and accountsRef. The representation supplies conflicting complete account values, with no precedence rule.

Required correction: Regenerate materialized states from one authoritative patch and compare every field. Preserve initialized tick flags, reconcile the nominal LP claim representation, and make duties exactly equal or define a full-record projection relation. Use one rejection state source, retaining the burn sink policy, with an explicit work-charge override.

MARKET-FR02: Four claim predicates do not yet specify a complete bounded admission and successor relation

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/sharedRecords/genesis`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/sharedRecords/currentHead`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/0/expected/acceptance`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/0/expected/history`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/5/expected/acceptance`

- The registry contains currentAtAdmission and semanticId, but no finite registered/revoked semantics set or successor registry state. Post history still references sharedRecords.currentHead whose exact-output head remains hist:pool:exact-output:v1, while successorCurrentHead is hist:exact-output-swap:001 and the footprint declares a registry write. The relation saying the produced head becomes uniquely current lacks the corresponding updated registry record.

- Every row declares maxMessages=8, maxClaims=8, maxObservations=16 and collection/sidecar/fan-in bounds, but the four claim predicate arrays do not specify checks for these bounds against their bound complete records. A proposed extra messages array of length 9 with a matching patch preserves the listed cash/authority/work equalities; the design supplies no explicit message-count acceptance equation.

- Funded genesis contents provide policy and work limits, not a binding of initial account balances, positions, caps or predecessor contents. The altered Newton reserve, 90-cap margin and multi-position tick siblings reuse the same funded genesis/admin/current-head IDs without a complete admitted predecessor record binding those different values. The 'funded' flags alone do not establish that content binding.

Required correction: Provide finite admitted genesis/predecessor records or an explicit admission assumption that binds each complete sibling preState to its current head. Give complete pre/post registry records and a finite semantic membership/revocation rule. State concrete bounds and canonical record binding predicates for all required input/effect/state collections, not only work arithmetic. Keep these as toy design predicates.

MARKET-FR03: Footprints remain incomplete and contain nonexistent resource aliases

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/1/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/2/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/3/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/4/boundedFootprint`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/controls/boundedCross`

- CL patch writes pool.currentTickMarker and pool.boundaryEvent, but neither named resource appears in the write set. The materialized state also changes tick flags and inventoryToyPost, which must be covered after FR01 is resolved.

- Redemption tokenSupply.stable.outstandingNominalDebt changes 300 to 150, but the named write set only includes spendable and irrevocableSink supply fields. Shared-vault hook cap remaining changes 1 to 0, but only its cumulative field is listed.

- Iterative AMM reads/writes list pool.token0 and pool.token1 although its actual pool schema has xp, A, Ann, D and iterationCap. There is no deterministic alias mapping from these nonexistent fields to the account records or xp elements. Shared-vault vault.unsettledDelta similarly needs an explicit alias to top-level unsettledDelta.

- The complete-state:postState catchall cannot establish actual named resource writes under the declared distinct-record convention. No per-step complete state/footprint relation is given for temporary deltas, tick crossing or rejected verification work. The bounded crossing control has no own complete footprint.

- Declared list counts match list lengths. Iterative AMM has 30 entries but weighted bounds sum to 44 because two Newton entries each have bound 8; distinguish record count from access bound rather than calling these the same bound.

Required correction: Define one exact resource alias/granularity map; derive actual reads and writes from each primitive/state diff, including temporary and rejection work, nominal counters and successor registry. Remove nonexistent aliases, add missing fields, and state record versus weighted access totals.

MARKET-FR04: Bounded crossing control is still a partial state sketch

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/controls/boundedCross`

- The control preStateDeltaFromPositive introduces stop tick95, but postState.accounts is the string 'equal to positive post accounts (only segment0 moved tokens)' and the other post keys are dotted partial assignments. It has neither statePatchRule envelope nor complete materialized state, authority, work, duties, history or four acceptance predicates.

- Cross arithmetic independently gives input477/output4882/fee1, activeLiquidity 1000000-1000000=0 and outside0 at tick100=1. Continuing through zero liquidity to95 is explicitly a toy choice and is coherent as such. However inheriting the positive full post would retain currentTickMarker100, boundaryEvent touch, and touch-position fields unless explicitly updated; the control does not state those updates or a frame rule.

- Cross positions.LP1.tokensOwed0 is bare string1 while the live positive field is a typed quantity object. LP inventory at/below100 can coherently clamp to909/0, but a complete post record must state that convention and preserve ownership and fee snapshot fields.

Required correction: Make this control a fully bound sibling with complete admitted preState and full deterministic post patch/materialization, all counters/duties/history/work and exact footprints. Retain the explicit non-Uniswap zero-liquidity toy rule and clamp inventory definition.

MARKET-FR05: Some claimed isolated negatives still have independent authority or history failures

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/0`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/1`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/8/mutatedInput/completeSiblingPreState/authority`

`experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/10/mutatedInput`

- Exact-output overdelivery19744 exceeds the unchanged pool assetB outgoing cap19743. The checkOrder passes only Alice's 10000 cap. Thus all earlier gross-debit prerequisites do not pass; a pool-cap check rejects independently of exact-output equality.

- Reversed CL rounding output4883 exceeds the unchanged pool outgoing token1 cap4882; it also disagrees with the positive exact-output intent. It rejects for multiple reasons unless the sibling explicitly authorizes this output while preserving the rounding discriminator.

- Four-segment sibling geometry is valid: successive liquidity1000000/400000/300000/200000 yields token0 costs477+211+176+131=995 <=2000 and outputs4882+1953+1464+976=9275 <=200000 physical pool inventory. Its complete authority, however, contains only Alice's token0 cap; the pool token1 outgoing cap is absent. It cannot claim every required authority prerequisite passes.

- Stale-currentness attemptedConsumed has only genesis/admin/stale head, omitting the registry predecessor entirely. It therefore also fails the mandated exact four-predecessor shape. Preserve all other predecessors when isolating the stale head.

Required correction: Provide explicit admitted sibling intents and original outgoing caps that pass each earlier prerequisite; preserve semantic/currentness provenance. For stale testing replace only the targeted current-head value and retain the complete predecessor shape. State multiple failures honestly where isolation is not intended.

Prior findings: M01/M02/M03/M06/M07 are partly corrected and remain blocked as above. M04's90 margin and150 redemption arithmetic is corrected. M05's three-versus-four-segment discriminator is corrected, but complete outgoing authority is absent.

All303 checked structured references resolve, and all three local source digests match. The three owned files and bound input packet match their frozen hashes before and after review. The JSON report records every trace/mutation disposition, exact bounded replay commands, failed inspection commands, arithmetic evidence and limitations.

