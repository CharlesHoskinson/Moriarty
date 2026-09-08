# Credit candidate04 result audit: BLOCKED

**Source: BLOCKED. Task: BLOCKED.** Current author process completed (exit0; grok-4.6-build;33calls). Task blockers are source correctness and retained verification, not the historical timeout.

Candidate `8e7fa58decd5b74139855a0718b3151da6ed2eba62e6efdbe16de9670a3c1f7b`;471091bytes;3ownedfiles. Scope: specified-only credit fragment. No full-SP01, runtime, proof, ledger, network or semantic-freeze acceptance.

Preserved independently:12full update/poststate equalities (conditional on nominal guards),13exact rollbacks and zero rejected costs,35arithmetic outputs,5rows/5traces/13mutation IDs,8transfers,40shareburn,work60/50/56,561resolved references and an acyclic161target graph. Seven of eight evaluated timed balance guards pass; one rejects the positive refinance. The23remaining prose guard texts are explicitly unchecked by that instrument.

## CREDIT-COMPLETE-01: A remaining sequential-current guard rejects the advertised refinance

Paths: `/library/oracles/refinance/primitiveUpdates/16`; `/library/oracles/refinance/primitiveUpdates/17`; `/library/primitiveSemantics/guardOperandState`.

- The twelve complete poststates now reconstruct from the listed updates, including work charges/defaults, prefix bookkeeping and final removals. All twelve comparisons pass when nominal-record guards are not silently assumed true.
- Refinance step3 tokenTransfer moves20WETH from vaultA to vaultB before positionUpdate. primitiveUpdates/17 explicitly selects sequential-current and requires vaultA.balance.WETH>=20. At that instruction the actual balance is0, so0>=20 is false. This positive cannot pass its specified relation.
- Independent timed-balance evaluation confirms the four highlighted retiming corrections and seven of eight concrete balance guards. The remaining failure was not included in the author's four stored counterexamples.

**Required correction:** Bind the collateral availability guard to the intended entry snapshot, or restate it as an exact post-transfer guard with the correct operands. Independently evaluate every instruction's actual guard at its selected time before claiming all twelve transitions valid; retain full-state equality and rollback.

## CREDIT-COMPLETE-02: Shared admission still leaves several predicates and currentness inputs detached

Paths: `/library/sharedAdmission/predicate`; `/library/sharedAdmission/completeContext`; `/library/oracles/refinance/acceptance/HistoryCompliance/checks/3`; `/library/preVerificationBudgets/inputs`; `/library/oracles/refinance/acceptance/IntentRefinement/checks/3`.

- Full registry tuple comparison, four-claim outer conjunction, populated budget records and USDC typed state sums are substantial corrections. Independently changing spec or bounded rejects the tuple; post USDC+1 changes300000 to300001; used1025>1024 and sidecar4097>4096 reject the declared scalar bounds.
- The shared predicate still contains bare source/environment/authority/history/genesis/admin/currentness names without exact Boolean comparisons for several components. Context references and conjoinsSharedAdmission=true do not define their verification relation. Some hard-constraint witnesses still use fixed textual amounts rather than typed path operands.
- HistoryCompliance.currentHeadMembership and shared currentness carry copied currentHeads/consumed arrays. Independent mutation of the selected afterStep2.history.currentHeads to hist:stale leaves the written subset comparison true: both check arrays remain hist:refinance-prefix-2. Full primitive replay could separately reject this mutation; the probe demonstrates that the claimed admission check itself is not bound to the changed ledger operand.
- Budget derivation descriptions now identify4claims,12prefix records and166work, but the retained executable recipe never checks equality between used/usedByOracle and actual records, nor the full source/environment/authority/genesis/admin predicates.

**Required correction:** Define each shared predicate with resolved input paths and exact comparison rules. Bind history/currentness to the selected admitted state and explicit external context, not duplicated literals. Check derived counts and bytes against both their source records and limits. Execute the complete conjunction and independently corrupt each input family.

## CREDIT-COMPLETE-03: Read footprints remain internally inconsistent and their projection is incomplete

Paths: `/library/canonicalResourceProjection/resourceToStatePath`; `/rows/0/boundedFootprint`; `/rows/1/boundedFootprint`; `/rows/4/boundedFootprint`; `/library/ordinaryCostRule`.

- Named guard/witness reads and nonempty frame/admission reads were added. The abstract60/50/56cost rule explicitly covers those read categories; an explicitly fixed toy price is allowed and is not itself a blocker.
- For redemption, vaultLiquid.balance.USDC appears in three per-step read sets and aggregateAccessBounds.reads=3, while top-level reads.bound remains1. observation.conversion appears three times and aggregate=3 but is absent from top-level reads. Refinance debt-old-alice.total similarly has per-step/aggregate3 versus top-level bound1; observation.WETH.priceUSDC has2 versus no top-level entry.
- Canonical projection lists32resource mappings but leaves46 refinance,44 redemption and46bad-debt resources from the top-level read/frame/admission union unmapped. Missing examples include authorities, request fields, duties, source/environment/genesis/admin inputs and five of the six registry fields. FrameSets name whole account/record metadata that the few amount-only projections cannot cover.
- A single unqualified toyRegistry.entries.spec mapping always resolves entries/0/spec, although redemption and badDebt require entries1/2. Merely setting aliasDedup=true does not define oracle-qualified identity lookup, complete expansion, alias equivalence or finite access counts.

**Required correction:** Provide total oracle-qualified resource-to-path/identity mappings for every read/write/frame/admission resource. Derive all top-level and per-step lists/counts from one normalized representation, include frame/admission work under the chosen fixed cost rule, and check missing/extra paths and aliases with direct mutations.

## CREDIT-COMPLETE-04: External grant and cycle are fixed, but complete boundary state updates remain unspecified

Paths: `/library/externalBoundaries/redemptionReplenish/preState`; `/library/externalBoundaries/redemptionReplenish/fullStateBinding`; `/library/committedStates/redemptionFundedLaterAfterFirstClaim`; `/mutations/5/mutatedContext/admissionOrder`.

- The new boundary identifier is non-expanding. Independent graph traversal covers161unique reference targets and finds no cycle;561ref-only pointers resolve. The issued toy pregrant is present unconsumed with original3800/remaining3800/cumulative0, and post authorities contain remaining0/cumulative3800. Explicit toy issuance is permitted in this source scope and does not imply real authentication.
- Independent boundary replay applies the declared grant patch, transfer3800, authority consumption and history update. Accounts, authorities, work and history all match. Full state still differs at six material paths: /boundaryCapability, /externalBoundary, /firstClaimCountersUnchanged, /observations/availableLiquidityUSDC, /observations/replenishment, /replenishedAfterFirstClaim.
- The boundary supplies no corresponding complete primitive slice/constructor/removal rules or closed nonmaterial projection for those six changes. fullStateBinding names cash/history/controller, not the full remaining updates. In mutation5, several prerequisite records still expose pass=true and have/need values rather than a complete executable derivation from this boundary.

**Required correction:** Retain an exact complete boundary update slice, including liquidity observation and all added material fields, or precisely separate a closed annotation projection. Replay full equality/frame from the issued precontext, then compute mutation5 prerequisites and first replay failure from that accepted result. Preserve finite expansion, pregrant consumption and the explicit zero ordinary-cost choice.

## CREDIT-COMPLETE-05: The retained independent replay recipe does not execute its advertised checks

Paths: `/library/independentCompleteReplay/recipe`; `/library/independentCompleteReplay/results`; `/library/independentCompleteReplay/notTmpOnly`; `FOREMAN_REPORT.md#independent-replay`; `FOREMAN_REPORT.json`.

- Parsing the retained recipe yields exactly Import, ImportFrom, Assign. Its only action loads credit.json. All subsequent replay/guard/admission instructions are comments; there are no assertions or validator calls.
- Independently executing that recipe against six synthetic records finishes without rejection after: changing registry spec, setting bounded=false, increasing actual post USDC by1, usedWork1025, sidecar4097, or invalidating the share-lock guard time. These are failures of the retained verification recipe, not a claim that a hidden /tmp script accepted the records.
- Reports cite python3 /tmp/fix_credit_history_partial.py and a second import of the same script. Their retained recipe cannot reproduce the stated all12FullEqual/negativeControls flags. Reimporting the author transform also does not establish independent implementation. notTmpOnly=true does not cure absent retained checking code.
- The current worker did complete: receipt exit0,1124.218seconds,33model calls under grok-4.6-build. That process identity is verified separately; previous124 author identity remains unavailable and is not retroactively repaired.

**Required correction:** Retain actual bounded executable stdlib validation code and exact command/capture provenance within the authorized evidence surface. It must independently compute full transition/guard/frame/admission/boundary checks and reject meaningful corruptions. Preserve the successful current process receipt and all prior failures; replace unsupported stored pass claims with reproducible scoped results.

## Prior findings and limits

- CREDIT-HISTORY-PARTIAL-01: Full material state reconstruction/defaults/removals corrected; one concrete guard-time failure remains CREDIT-COMPLETE-01.
- CREDIT-HISTORY-PARTIAL-02: Registry tuple, real path sums and populated budgets improved; complete bound admission remains CREDIT-COMPLETE-02.
- CREDIT-HISTORY-PARTIAL-03: Known guard reads and frame/admission lists improved; total projection/count consistency remains CREDIT-COMPLETE-03.
- CREDIT-HISTORY-PARTIAL-04: Cycle and absent pregrant corrected for explicit toy issuance; complete external updates remain CREDIT-COMPLETE-04.
- CREDIT-HISTORY-PARTIAL-05: SCOPED RESOLVED: authenticated afterStep2 grant remains50500, attempted debt50050, explicit supplied use-witness is absent. The stated two-conjunct rule rejects; exact rollback passes. This does not establish general witness authentication or a complete positive validator.

The JSON report contains exact probes, per-step outputs, timing, pin checks and command dispositions. Initial arithmetic inspection rejected max() in the reviewer allowlist; after explicit inspection, the corrected bounded int/max replay passed all35. No source edits, raw worker stdout, private reasoning, network, wallet, install, compiler, proof or Git action.

Substantive work ended521.524seconds after dispatch, before620-second cutoff. Final integrity/report timing is appended below. The720-second allocation is native, not cgroup enforced.

All7unique owned/input/source files unchanged after review. Final elapsed: 753.892 seconds;720-second allowance exceeded.
