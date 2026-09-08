Composition candidate-04: **BLOCKED**.

The four original corruption controls now reject. The frozen suite passes84/84. Two fresh generations match the frozen JSON byte for byte.

The checker accepts50 of56 independently corrupted candidates. These counterexamples block the finite mechanism. Test success does not establish complete state or claim validation.

Candidate: `4879ae1af1c617b92e4494a69766de584e5c914d2dc33eabab5f458cd86ecc4e`. Five owned files total944686 bytes. The dispatch count944647 was incorrect. All22 distinct source files and five frozen files match their pins.

Requested reviewer: GPT-6 Astra high. The native interface supplies no independently verified serving identity or effort receipt.

Scope: finite checker only. No map, semantic freeze, BNF, K, runtime, proof, ledger, Preview or sprint acceptance.

**CM01: Complete states and mandatory claims do not govern acceptance**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1953).

- check_one_trace compares only supplied cash keys and the NAM principal. It does not compare the complete decoded poststate with replay.
- A missing postState or empty post accounts skips even the cash comparison. Declared work, duties, authority and unrelated frame records can disagree with replay.
- evaluate_hard reads canonicalIntent.hard into hards but never evaluates those supplied predicates. Several failures depend only on mutationId substrings.
- The reported 100 prefixes are the sum of internal replay list lengths. Only 14 abbreviated derivedPrefixes records exist. The checker never compares them.
- Removing mandatoryClaims or replacing an intent with a false hard claim still returns exit0 and ok:true.

Reproduced controls: `post_work_erased`, `post_duties_erased`, `post_unknown_authority_field`, `post_unrelated_frame_tamper`, `post_accounts_empty`, `post_absent`, `prefixes_erased`, `hard_claim_impossible`, `mandatory_claims_erased`.

Compare complete states and every declared prefix against replay. Evaluate the ordered mandatory predicates from typed operands.

**CM02: The decoder normalizes permissive dictionaries instead of enforcing a closed typed schema**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:196).

- closed_schemas returns documentation. decode_candidate checks trace IDs and effect ctor presence. It does not apply the declared product or sum schemas.
- decode_state accepts untyped nested material records. Negative and fractional runtime-nat balances are accepted. Unknown underscore-prefixed material fields disappear.
- Transfer unit WETH with asset USDC and scale999 is accepted. Missing Transfer IDs and unknown material effect fields are accepted.
- bound_frac calls limit_denominator(10**30) before checking bounds. This can change a supplied rational instead of rejecting it.
- Constructor signatures can be erased. Transfer relationId and equations can be changed while the checker accepts. The inventory checks presence and selected cashMovement fields.

Reproduced controls: `nested_bad_material_types`, `pre_negative_nat_balance`, `pre_fractional_nat_balance`, `runtime_fraction_transfer`, `transfer_wrong_unit_scale`, `transfer_unknown_material`, `remove_transfer_ids`, `pre_material_underscore_erased`, `schemas_erased`, `constructor_signature_erased`, `primitive_relation_text_tamper`, `wrong_relation_identity`.

Implement closed recursive field validation. Reject unit, scale, variant, cardinality and rational-bound violations. Bind complete constructor signatures and relation identities.

**CM03: Authority and system admission remain incomplete**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:344).

- Asset-indexed gross transfer accounting works for the six retained cash traces. Alice spends101 in the atomic trace and receives no refund refresh.
- normalize_authority_record replaces the supplied system allowedEffects with all SYSTEM_CTORS. An explicit empty system authority is silently expanded.
- Mint does not check mint authority, a nonnegative amount, or a pool quote. ConsumeJointEnvelope always consumes a synthesized envelope without checking an admission condition.
- Registry, required claim root, head freshness and permitted system changes do not enter a complete admission relation.

Reproduced controls: `system_authority_denied`, `missing_registry_and_head`.

Preserve supplied authority as data. Check every concrete effect against its principal, asset, payee and exact system or joint capability.

**CM04: Primitive execution still permits unfunded discharge and incomplete state transport**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:504).

- Transfer alone moves linked fee and lock cash. This fixes the original lock double debit in the retained positive cash replay.
- find_linked_transfer returns an ID match without checking sender, escrow, asset or amount. A Lock for999 or a Lock from Attacker still passes.
- SettleDuty never checks that its fundingTransferId exists, reaches the duty payee, has the correct asset, or funds the amount.
- ReduceDebt checks a found transfer asset and payee but never checks its amount or aggregate allocated amount. Funding is optional.
- UpdateWork rejects ordinary refresh but accepts negative closureDelta. Appending closureDelta=-8 decreases spent and increases remainingClosure.
- encode_state omits locks and accountsMeta from the encoded state. Complete internal lock records cannot survive that round trip.

Reproduced controls: `partial_no_funding`, `partial_funding_one_for_twenty`, `partial_payee_changed`, `async_lock_wrong_link`, `lock_wrong_sender`, `negative_closure_work_refresh`.

Validate every primitive precondition and complete effect. Require funded discharge and exact links. Preserve complete state fields and prohibit work refresh.

**CM05: NAM exact values are supplied inputs and source reporting is not derived**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:220).

- Independent Fraction arithmetic confirms cap1=7200/73, cap2=100616790123456789/730000000000000 and principal=3822616790123456789/730000000000000. Principal rounds to5236.461356333502.
- AccrueNominal reads accruedExact from the effect. It does not compute principal*rate*days/year or resolve observationRef. SetNominalRate likewise does not resolve its observation.
- Capitalize computes exactPrincipal but sets reported principal to the constant nam_exact principalReported, without projecting the actual computed exact value.
- Setting both accruedExact values to0 or both reset rates to0 still passes. Nonexistent observation references also pass.
- The pinned fourth result rate is0.11167901234567901. The candidate final debt and contract rate is0.111679012345679000. Difference is-1/100000000000000000. No declared projection explains this source mismatch.
- The retained CreateDebt omits rate and therefore creates rate0. The named IED step declares rate0.08. The event steps charge8/4/8/4 work, but the primitive plan charges all24 only at its end.
- Contract notional stays0 until the UpdateContract after capitalization. The IED and first RR source prefixes require notional5000.

Reproduced controls: `nam_accrual_exact_zero`, `nam_rate_second_zero`, `nam_contract_principal_zero`, `nam_observation_refs_nonexistent`.

Compute exact accrual from state and authenticated observations. Project each actual exact prefix into the pinned reported event state. Reconcile all event and primitive prefixes.

**CM06: Negative materializations and first-failure evidence remain disconnected**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1427).

- materialize_mutation expands the mutation ID using hardcoded branches. It ignores completeCandidate, mutatedInput, mutatedContext and supplied expanded effects.
- evaluate_hard manufactures most negative failures from the mutation ID. The work-refresh branch adds an ignored resetRemainingOrdinary field, then rejects by name.
- Replacing every completeCandidate with empty effects and states still passes all11 reported negative checks.
- Declared failure stage is computed into declared_stage but never compared. Removing failure labels passes. Reject.WrongAsset is accepted for the NAM erasure mutation.
- Earlier stages are filled with holds:true up to the chosen stage without evaluating them. A failed replay does not establish those earlier predicates.
- Negative prefixState, rollback snapshots and declared prefix effects are not checked against the actual admitted prefix.

Reproduced controls: `negative_expansion_empty`, `negative_failure_stage_wrong`, `negative_all_missing_labels`, `negative_unrelated_fail_label`, `negative_failure_label_substring`.

Use one typed override representation. Replay its actual effects and evaluate the first failing mandatory stage. Compare all negative expansions and rollback states.

**CM07: Async receive/refund eligibility does not govern the primitive phase transition**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1097).

- async_phase is a useful exclusive predicate, but UpdateMessageStatus Pending->Refundable does not call it or check time and observations.
- The frozen main plan opens Refundable while observedTick is still10. It sets tick100 only afterward.
- Changing every SetObservedTick to0 still passes. Adding authenticated finality99 to the prestate also allows the main refund.
- Both alternate branches can be erased and the report remains ok:true with branches0. Supplied alternate poststates are never compared with replay.
- Branch receive checks inject an observation for the literal branch name. They check selected cash balances and omit complete branch state equality, footprints and mandatory claims.

Reproduced controls: `async_early_refund`, `authenticated_receive_evidence_still_refunds`, `branches_erased`, `branch_receive_post_tamper`.

Make one phase relation mandatory for each status change. Bind branch observation, committed base, complete successor, nonce, entitlement and work checks.

**CM08: Footprints omit actual reads and do not enforce a complete frame**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:586).

- Reported footprint counts are23,11,21,32,31,26. These count manually added paths rather than a verified total resource projection.
- UpdateWork reads remainingClosure but never records that read or its write. AccrueNominal and SetNominalRate do not record observation reads.
- ReceiveMessage reads observations, nonce state and escrow cash without recording those reads. Registry and head admission reads are absent.
- compare_frame only compares unrelatedRecord and is never called. material_state_diff is also never called. Its slash paths do not match the dot write paths.
- project_resource accepts unrecognized paths and is not used by check_one_trace. Full authority, metadata and lock projection is not established.
- Erasing declared footprints and changing a declared unrelated frame record both pass. Branch and negative footprints are not validated.
- The actual complete footprint requirement remains unestablished. The reported values do not prove conformance to64.

Reproduced controls: `footprints_erased`, `post_unrelated_frame_tamper`.

Derive complete accesses through one total projection. Compare all material pre/post fields outside writes. Validate every branch and negative footprint.

**CM09: Finite composition relations and payment allocation laws are not implemented**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:866).

- finiteRelations contains string references and operatorAdmissibility text. No executable restriction, entailment, interference or renaming judgment exists.
- ord_serialize serializes the existing list and adds indexes. It does not validate an operator order, contract restriction, or sort-preserving rename.
- The funded partial fixture works at50-20=30. Removing its Transfer still produces30 and returns success.
- One allocation ID can discharge two different duties. Two IDs can discharge40 against one Transfer20. Both bypasses return success.
- Removing partialDuty50to30 or finiteRelations still passes. Proposed theorem names remain preserved in the frozen artifact, but their finite statement domains remain incomplete.

Reproduced controls: `partial_no_funding`, `allocation_reuse_across_duties`, `two_allocations_exceed_transfer`, `partial_omitted`, `finite_relations_erased`, `all_theorems_erased`.

Implement typed finite contract, restriction, observation and renaming relations. Enforce allocation use-once and aggregate-per-transfer limits across all duties and debts.

**CM10: Old Simulation transport preserves selected state but loses effects and settlement identity**

Source: [composition-model.py](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/candidate-04/experiments/moriarty-language/spec/successor/financial-fragments/composition-model.py:1575).

- The fixture is a genuine retained two-step Simulation witness. All nine value names, revisions0/1/2, final lifecycle0, residual4500000000 and two Settled duties are retained in the state summary.
- old_transport D0Effect stores only index, Simulation kind and the string Simulation-input. It omits action arguments and all DueCreated, Transfer and DueSettled effects.
- Each mapped state claims settlementAsset=USD_micro and USD_TEST_ASSET_not_used=true. The fixture settle input names settlement_asset USD_TEST_ASSET.
- The fixture transfer has asset USD_TEST_ASSET, nominal unit USD_micro and ledgerAmount533972602. Collapsing this distinction contradicts the required transport.
- check_claims recomputes a selected fixture summary but never compares candidate oldDomain.transport. Erasing the complete oldDomain still passes.

Reproduced controls: `old_transport_erased`.

Map the exact old input, state, effect and observation data. Preserve USD_micro denomination separately from USD_TEST_ASSET settlement. Bind the candidate mapping to the fixture.

The baseline reports6 positives,100 prefixes,2 branches,11 negatives and2 mappings. Only14 abbreviated declared prefix records exist.

Independent cash and history-ID subreplays agree for all6 traces. Alice gross debit remains101. These limited successes do not settle complete-state correctness.

[JSON evidence](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/gpt6-mechanism-result-review.json) contains exact source references, probe hashes, commands, source checks and timing limits.

[Probe script1](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/gpt6-mechanism-probes/probe.py) and [probe script2](/home/charl/.local/state/moriarty/sp01-map-fragments-20260908/composition/gpt6-mechanism-probes/probe2.py) reproduce all57 CLI cases. Disposable mutated files were removed.

One exploratory prefix print raised IndexError. The corrected inspection confirmed the limited prefix count. No CLI probe construction failed.

Do not accept candidate-04. Preserve the frozen candidate and this report for the next authorized correction.
