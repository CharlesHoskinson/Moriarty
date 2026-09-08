# Markets candidate-01 result review

**BLOCKED — specified-only financial design fragment.** No full-map, runtime, proof, semantic-freeze, network or sprint acceptance.

Frozen digest `148e4292f07bb646cc5d21b29218d236656ab46e82efffdbefd29c67656021c0`, 233839 bytes. All three owned-file hashes and the bound input-packet hash match. Author receipt records exit0 and actual model `grok-4.6-build`.

All six positive account balances, ordered token transfers, trader caps and ordinary-work arithmetic replay correctly. Exact-output is19743 (fee30); CL quotes are477/4882 and4883/432; amplified AMM converges toD1999 andy850; redemption sends297+3 collateral against150 stable; vault settlement10/9/1 derives zero; margin withdraws75 and retains collateral25, margin20 and debt5. The extreme AMM sequence is independently reproduced through all8 iterations, endingD39020571 with change19508019.

## M01: Poststates do not preserve the complete live financial state

- Redemption preState principal T1/T2/T3 is 100/100/100 and accrual is 0/0/0; postState drops both objects while debt becomes 0/50/100. Generic unchanged-field inheritance would leave principal inconsistent with debt; full-state replacement would erase it. Neither semantics is specified.
- Margin postState omits principal/accrual and removes controller=trader from the live funding duty. The position keeps spendableFormula collateral-heldMargin-fundingDebt=100-20-5=75 although collateral is now25 and spendable0.
- Shared-vault postState omits hookPermission while residualAuthority says hookAuthorityUnchanged. No local preserve/merge reference reconstructs that live field.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/3/expected/postState`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/5/expected/postState`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/4/expected/postState`

Required correction: Define complete poststates or an explicit deterministic inheritance/patch rule. Update redeemed principal to0/50/100 and retain zero accrual; retain margin principal/accrual with clear non-double-counting semantics, duty controller and post formula25-20-5=0; preserve hook permission explicitly. Audit every pre/post field for the same issue.

## M02: Mandatory claim predicates do not bind complete state or establish currentness

- The four claim names and concrete amount statements exist, but the listed predicates cover selected totals and transfers. There is no complete-state equality/frame predicate, canonical bundle binding, current-state registry check, or revoked/unknown-semantics check.
- Numeric counterexample: change margin post lifetimeRemaining from984 to1024 while retaining used40, balances85/25/0, funding5, IM20, pnl40 and one transfer75. All listed margin predicates still have the same truth values; work conservation fails by40.
- Genesis/admin inclusion is a flag plus history IDs. No funded genesis/admin transition or bound authoritative genesis/admin content is provided for most traces. Listing consumed IDs does not establish that a predecessor is current or prevent replay.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/0/expected/acceptance`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/expected/acceptance`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/2/expected/acceptance`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/3/expected/acceptance`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/4/expected/acceptance`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/5/expected/acceptance`

Required correction: Specify finite design predicates over the complete input, plan, poststate, effects and history. Bind genesis/admin records and current predecessor/semantic authority to the same candidate; specify one-time consumption and successor currentness. Add a numeric work-reset or stale-currentness discriminator. Runtime execution remains out of scope.

## M03: Resource footprints omit resources that the transitions read or write

- Declared aggregate read/write bounds are7/7,10/8,21/5,7/10,7/7,7/4. These are sums of declared records, not demonstrated complete footprints.
- Margin declares4 writes but also changes authority cumulative0->75 and remaining75->0, ordinary work used32->40 and remaining992->984, and position spendable75->0. These resources are absent.
- Concentrated liquidity omits authority/work, currentSqrtP105->100, feeGrowthInside0Last0->1, and the new pending duty from writes. Redemption omits authority/work, principal and duty/status resources. Shared vault omits authority/work. Genesis/admin/currentness resources are not consistently enumerated.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/0/boundedFootprint`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/1/boundedFootprint`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/2/boundedFootprint`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/3/boundedFootprint`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/4/boundedFootprint`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/rows/5/boundedFootprint`

Required correction: Enumerate all actual state, authority, work/reserve, duty and history reads/writes, with a stated record-granularity convention and computed total bounds. Include the resources required by corrected complete-state/currentness predicates.

## M04: The margin and redemption negatives are not isolated economic discriminators

- Margin withdrawal130 would produce collateral-30 from100, exceeds original debit authority75 by55, and contradicts intent withdraw75. It rejects even if contingent PnL40 is incorrectly considered cash; 75+40=115 is still below130. Therefore it does not distinguish cash from contingent PnL.
- Redemption mutation burns100 and pays198+2 collateral while unchanged canonicalIntent requires burn150. It fails the burn amount predicate independently of ordering.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/7/mutatedInput`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/7/rejectionPredicate`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/4/mutatedInput`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/4/rejectionPredicate`

Required correction: Preserve required mutation IDs. Supply coherent funded, explicitly authorized sibling intents/contexts for isolated tests, with provenance/currentness placeholders bound consistently. For margin choose a withdrawal at most physical collateral but above75, with an explicitly admitted allowance/intent rather than silently inheriting cap75; show a PnL-as-cash implementation would accept it. For redemption retain burn150 and valid fees/collateral while putting T3 before T1, e.g. T3-100 then T1-50. Specify the prerequisite check order and show earlier checks pass.

## M05: The tick-walk mutation does not exceed its own numeric cap

- The mutation has four visited entries105,100,95,90 and cap3. Its exact predicate is len(ticksVisited)-1 <= maxTicksTraversed AND every tick initialized. 4-1=3<=3 is true, so only unknown ticks95/90 reject. It is not an over-cap witness.
- The positive example has two visited entries and one segment; its predicate text says ticksVisited1<=3. Counting visited entries and traversed segments is inconsistent.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/2/mutatedInput`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/2/rejectionPredicate`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/expected/acceptance/ContractInvariant`

Required correction: Choose and use one counting convention. Provide a fully initialized, structurally valid path exceeding that bound, or explicitly separate unknown-tick and over-cap mutations. Include complete funded context and bounded geometry for the isolated cap test.

## M06: Concentrated-liquidity position and boundary behavior remain under-specified

- The explicit toy floor/ceil quotes and fee growth arithmetic are correct. However no position inventory formula or rule connecting position range/liquidity to token claims is defined.
- The plan declares ticksCrossed1 on105->100; tick100 has liquidityNet1000000, yet post activeLiquidity stays1000000 and feeGrowthOutside0 stays0 after global growth becomes1. No boundary convention explains whether the endpoint merely touches the range or actually crosses it, or how either field is updated.
- The observation says all three ticks100/105/110 are initialized, but tick105 is annotated not an initialized bound.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/input/formulas`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/input/preState/pool/initializedTicks`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/expected/postState/pool`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/input/concretePlan`

Required correction: Keep the explicit toy model if desired; define position token quantities, endpoint/crossing semantics, active liquidity and fee-growth-outside updates. If this only touches a boundary, label it accordingly and add an actual bounded crossing example. Do not import unsupported Uniswap protocol claims.

## M07: Full residual authority and negative work accounting are not reconstructible

- Each positive trader-side cap satisfies original=cumulative+remaining, and work charge8/12/8/10/8/8 reconciles. But full outgoing pool/vault authority and fee/nominal-liability authority are not enumerated numerically: redemption collateral gross debit300 and hook vault token1 gross debit10 are not represented by original/cumulative/remaining allowances. A boolean saying nominalDebtAuthoritySeparateFromTransferAuthority does not supply such authority.
- The nonconvergent Newton mutation performs8 iterations but specifies only unchanged accounts, feesCharged=false and DNotWritten. It supplies no full mutated work/authority/history state or rejection charge/reserve policy. Most other negatives likewise leave these out.
- Redemption calls a transfer of150 to a balance-bearing burnSink a burn, but does not define irrevocable sink/spendability or token supply rules; debt discharge depends on that assumption.

Locations: `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/1/expected/residualAuthority`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/3/expected/residualAuthority`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/traces/4/expected/residualAuthority`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/3/expectedFinancialState`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/1/expectedFinancialState`, `experiments/moriarty-language/spec/successor/financial-fragments/markets.json#/mutations/7/expectedFinancialState`

Required correction: Define numeric authority for each relevant gross debit and fee, plus explicit separate nominal-liability authority where applicable. Define whether rejection consumes ordinary verification work and give numeric pre/post work/reserve and complete rollback references, including the extreme-pool sibling. Specify the toy burn sink as irrevocably nonspendable or model an actual supply burn before treating transfer as debt discharge.

## Verification and limitations

All8 declared negative predicates evaluate to rejection and all explicitly provided rollback account balances match their applicable prestates. This does not establish isolation, complete rollback, or runtime enforcement. Required identities, row source pins, package/closure assignments and source-gap honesty are preserved. All representation paths resolve locally.

All attempted local commands exited0; command inventory, six complete replayed balance maps, eight mutation assessments and SHA256 maps are in `gpt6-result-review.json`. Inline Python command descriptions are summaries; exact invocation bodies remain in the tool transcript. Broad reads that truncated were followed by focused evidence reads. Candidate owned hashes and packet were verified early and at end; ancillary input hashes were first recorded near the end and compared again before output.

No network, build, wallet, git change, candidate write, subagent, raw worker stdout or private provider reasoning access. Source files were not re-fetched; source pins were checked against the bound packet. Toy formulas are allowed; no protocol-conformance objection is inferred merely from different toy math.
