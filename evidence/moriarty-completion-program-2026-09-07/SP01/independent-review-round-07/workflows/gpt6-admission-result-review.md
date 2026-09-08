# Workflow candidate-02 admission result review

**BLOCKED within the specified-only financial design scope.** Four remaining issues prevent full admission/state acceptance. This does not claim protocol, evaluator, K, proof, ledger, full-map, semantic-freeze, Preview or sprint acceptance.

Reviewer: independent Codex reviewer, system-described GPT-6; requested routing GPT-6 high. Exact backend model identifier is not exposed. Review allowance: 600 seconds; no author fixes, network, builds, installations, raw worker stdout/provider reasoning, or subagents.

Candidate: `4deabfb0da41413684838df39221dc14d918a847f1d725742a98ecc2af441840`, 453720 bytes, three owned files. Author receipt reports grok-4.6-build, exit 0, 24 tool calls.

The independent replay passed all 24 positive cash/work steps and 24 transfers, with fees applied once. All 786 typed monetary records match the asset/unit/scale registry. All 1190 checked structural references resolve; nine source pin/pointer records match three source files. Fee-aware floors 1508/1008 and Alice net USDC -5030 are preserved. All 25 full rollback references and empty rejected financial effects/fees remain.

WF-R01's original funded-history contradiction is locally resolved: the two underfunded states now have distinct whole-state enrollment, own genesis/admin/history, payout authority and initial work zero. C1 totals 30000 with custody100<10000; C9 totals16000 with custody0<8000. WF-R05 units are resolved. Global admission defects below still prevent claiming these controls pass every prerequisite.

## WF-A01 — Admission conjunction lacks candidate-bound currentness, complete assumption membership and typed current-head relation

Paths: `$.admittedToyContext.mandatoryClaimFiniteConjunction[0:5]`, `$.admittedToyContext.currentSemanticsKeySpecRegistry`, `$.admittedToyContext.environmentAssumptionRegistry`, `$.admittedToyContext.preVerificationBudgets`, `$.traces[5].steps[1].observations[0].signer`, `$.traces[*].acceptance.*.authorizationAndCurrentHead`.

- Trace 5 step 1 requires authenticated-assumption signer Midnight.FinalityOracle. This identifier is absent from environmentAssumptionRegistry. The declared every-named-assumption membership predicate therefore rejects a positive trace if applied to its named observation source.

- The three registry predicates test literal sem-toy-v1/key-toy-v1/spec-toy-v1. There are no candidate semanticsId/keyId/specId bindings in the positive admission record. Changing a proposed semanticsId to sem-unknown does not change those literal registry facts; specifiedControls provides a rejection label, not the missing lookup relation. Revoked-key control flags conflict with the still-current registry entry unless an explicit control overlay is supplied.

- preVerificationBudgets contains maxima, but no measured sidecar representation/byte count or per-candidate collection, observation, message, claim and fan-in counts. Its conjunction checks only sidecarBytes, ordinaryCharge, conserved remaining work and a boolean ordering label. Full bounded admission cannot be independently evaluated from these operands.

- The conjunction requires each consumed history ID to be current, while successor.current contains S-* identifiers. For example trace 0 step 1 consumes H-lock-N7, but the selected current head is S-lock. All 17 non-initial steps have this H/S distinction. No typed head-to-history relation is supplied. Reading history.produced as the intended registry would be an additional rule. Initial genesis/admin IDs are already listed in history.consumed and then consumed by step 0; the boundary convention also needs an explicit typed rule.

Required correction: Specify the complete finite candidate admission record and predicates over its actual selected registry IDs, registry/control overlays, named assumptions, measured bounded inputs and typed history/head mapping, including initial enrollment. Bind these predicates into all four claims and each negative prerequisite. A precise finite design relation is sufficient; no production implementation is requested.

## WF-A02 — Complete update relation does not enumerate the state changes it claims to determine

Paths: `$.traces[*].updateRelation`, `$.traces[0].steps[2].nominalChanges`, `$.traces[0].steps[2].postState.allowances`, `$.traces[3].steps[1].postState.covers[0].lifecycle`, `$.admittedToyContext.mandatoryClaimFiniteConjunction[5:7]`.

- All 24 updateRelation entries repeat applies=[transfers,fees,nominalChanges,observations,workCharge,consumedHistory,producedHistory], with complete post equality and a frame for fields not in applies. They contain no typed field equations or actual replacement records for the additional duty/status/allowance/successor changes named by the outer conjunction.

- Trace 0 fill2 has nominalChanges=[] but changes fee cumulative/remaining, grossDebit cumulative/remaining and routeRights.residualFillCap. Transfer replay alone does not specify which authority record must change or how the cumulative fee participates in gross debit.

- Trace 3 authenticated-observation has nominalChanges=[] but changes covers[0].lifecycle as well as storing the observation. No cover-lifecycle assignment appears in its applies records. A literal frame forbids this recorded change; allowing unspecified inferred lifecycle updates makes the claimed complete relation underdetermined.

- Nominal records such as add D-clearing and set allowedEffects='subset' do not supply a typed complete new record. Some records point to their own proposed post-state, which identifies desired output but is not an independently defined transition relation.

- Cash/work checks pass when fees are applied once via transfers; this does not settle the missing nominal, authority, lifecycle and frame equations.

Required correction: Provide deterministic, typed per-action state equations or exact field assignments with unambiguous input/output paths, fee-list alias semantics, and frame complement for all 24 steps. Derive allowance, duty, cover, message, status and head changes without consulting the desired post-state as the definition of success. Independently compare the complete derived post-state.

## WF-A03 — Negative materialization and rejection-stage prerequisites remain inconsistent

Paths: `$.refConvention.legalStepRef`, `$.mutations[19].mutatedInput.exactOverrides`, `$.mutations[19].mutatedInput.proposedPostAccounts`, `$.mutations[1].mutatedInput.exactOverrides`, `$.mutations[1].mutatedInput.proposedSuccessors`, `$.mutations[1].earlierStagePredicates`, `$.mutations[14].earlierStagePredicates`, `$.admittedToyContext.candidateStageOrder`.

- Apply mutation 19 exactOverrides to its complete legal step as refConvention directs: proposed transfer becomes 1501, but inherited postState accounts remain Alice WETH=1510 and maker1 WETH=3490. Independent replay requires 1501 and 3499. Separately supplied proposedPostAccounts has the correct numbers. There is no merge/precedence rule that replaces postState.accounts with proposedPostAccounts. Thus two candidate materializations disagree.

- Mutation 1 legalStepRef is a step object whose keys include postState, not successor. exactOverrides uses successor.count and successor.current, so literal replacement addresses absent fields rather than postState.successor. Its S-fill3 record supplies escrowBefore and completeRecord:true but no complete typed fill input or successor financial state. The earlier type-admission pass is not derivable from a defined minimal header type.

- The declared stage order puts unique-consumption-successor-currentness before message-or-effect-admission. Mutation 1 marks the former pass and the latter reject for two current successors; mutation 14 marks the former pass and the latter reject for consumed=true. The shared conjunction assigns these candidate properties to unique-consumption/successor-currentness, without distinguishing a base-state-only check. expectedRejectionStage also uses successor-admission or message-admission names outside the common ordered list, without a mapping.

- The intended M2 replay-before-custody ordering is valid as a design choice and 0<400 is correctly identified as unreachable after replay rejection. The report blocks the inconsistent shared stage definition, not the existence of that safe ordering.

- Other controls retain correct local rejection facts, but many earlierStagePredicates still list only previous successful trace steps rather than all candidate admission prerequisites. Cross-cutting missing measured budgets/currentness prevent an exact first-error claim for all 25.

Required correction: Define one negative input/materialization convention with a single complete candidate record or explicitly typed admission-only header. Correct override paths and specify proposed-field precedence. Use one ordered stage vocabulary and separate base-state admission from candidate uniqueness checks. Reproduce each first failure with all earlier predicates evaluated from concrete candidate operands.

## WF-A04 — Footprint reconciliation covers prior writes but omits reads required by the new admission predicates

Paths: `$.rows[*].boundedFootprint.reads`, `$.rows[*].boundedFootprint.writes`, `$.resourceAliasRegistry`, `$.traces[*].acceptance.*.authorizationAndCurrentHead`, `$.admittedToyContext.mandatoryClaimFiniteConjunction`.

- None of the seven read inventories includes successor:current, although every acceptance relation now checks membership and uniqueness of the selected current head and every row writes that resource. history:head is separately listed; no registry alias equates it to successor.current.

- Request/controller authorization and semantics/key/spec/environment registry reads are not assigned canonical resources/counts. The new conjunction therefore introduces reads outside the declared footprint without an explicit admission-footprint scope or separate bounded inventory.

- The fragment provides per-trace maxima and a boolean reconciledAgainstCompleteStateDiffs, but no exact per-step read/write mapping. Whole-state diffs can establish writes and do show the prior missing write entries are now present; they cannot establish reads or the resource cost of negative variants.

- Recurring remaining_aggregate aliases now explicitly exclude original/cumulative, which is an improvement. An alias formula alone does not supply the missing per-action dependency/accounting relation.

Required correction: Define the canonical resource projection and per-step read/write sets for the complete acceptance relation, including head/controller/registry admission dependencies, with an explicit accounting scope for admitted initial boundaries and negative variants. Recompute per-trace access maxima from those sets and the actual state diffs; retain the corrected financial write entries.

## Case dispositions

- Mutation 0: 5030 cumulative gross must survive refund; reset to 0/10000 rejects; rollback work 40.
- Mutation 1: Two proposed current successors violate max 1; exact override/header and stage definition blocked by WF-A03; rollback work 32.
- Mutation 2: Cancel extinguishes RR-N7, not pending D-clearing; rollback work 40.
- Mutation 3: After five payments state is 7 periods, 700 remaining, 978 ordinary remaining; restoring 12/1200/1024 rejects; rollback work 46.
- Mutation 4: T5 time 1714435200 equals due time; payer 1600 funds 5*100; ticket outstanding once makes five presentations illegal; shared stage/materialization caveat WF-A03; rollback work 38.
- Mutation 5: 10 USDC transfer funded, mallory outside venuePool/venueFee whitelist; local authority rejection correct; rollback work 6.
- Mutation 6: Unbacked claim cash credit adds 10000 without debit, violating 30000 conserved total; rollback work 22.
- Mutation 7: 1710007200-1710000000=7200>3600; authorized signer and anchor retained; rollback work 8.
- Mutation 8: Anchor O9-FORGED differs from bound O9; age1800 valid and signer retained; rollback work8.
- Mutation 9: Signer alice differs from OracleRegistry.rainfall with otherwise valid observation; rollback work8.
- Mutation 10: Distinct admitted C1 boundary now coherent:9500+100+20400=30000; enrolled10000 payout exceeds100; rollback whole alternate with work0.
- Mutation 11: Refunded M1 replay prohibited; proposed none-mint-illegal transfer is an admission-only payload without defined header schema; stage caveat WF-A03; rollback work14/reserve8.
- Mutation 12: Late observation age10<=600 is fresh but M1 already Refunded; destination claim payload supplied; stage caveat; rollback14/8.
- Mutation 13: Created M2 lacks required finality observation; escrow400 funds proposed receive, so local missing-finality control is isolated; global admission caveat WF-A01.
- Mutation 14: Received consumed M2 replay intended to reject before unreachable0<400 custody; global stage inconsistency WF-A03 and registry issue WF-A01; rollback24.
- Mutation 15: Assessor B differs from authorized-assessor-A, with truth flag separated; rollback8.
- Mutation 16: Adjudication age7200>3600; rollback8.
- Mutation 17: Payout15000 differs from bound8000; authorized signer and original anchor retained; rollback8.
- Mutation 18: Distinct C9 boundary now coherent:800+0+15200=16000; enrolled8000 payout exceeds0; rollback whole alternate work0.
- Mutation 19: Fee-aware threshold3002<3015 and funded maker5000 hold; proposed accounts1501/3499 correct but legal-step exact materialization1510/3490 conflicts, WF-A03; rollback lock work8.
- Mutation 20: Controller rewrite alice->agent42 rejected by controller lineage intent; named histories/cash held fixed; rollback20.
- Mutation 21: Typed CreateDebt amount1000, debtor alicePortfolio creditor agent42 supplied; op excluded from allowedEffects; effects may be unconstructed after admission reject; shared legalStepRef/stage caveat; rollback6.
- Mutation 22: Refunded M1 cannot credit DC-M1 dest995; asset/unit now consistent, complete message/claim refs supplied; shared stage caveat; rollback14/8.
- Mutation 23: Authorized assessor's provedExternalTruth=true violates explicit authenticated-assumption scope; rollback8.
- Mutation 24: ADJ-BLK-9-SUB differs from bound ADJ-BLK-9 while payout8000 retained; rollback8.

Every mutation retains the complete selected-state rollback reference and empty rejected financial effects/fees. Local rejection facts above are independently checked; full first-error admission is not established because the candidate-bound predicates and transition relation remain incomplete.

## Reproduction and evidence

The JSON companion retains exact commands, exits and outputs for arithmetic, units, negative materialization, registry/head membership, structural references, source pins and protected hashes. The initial quantity probe exited1 for a dictionary-valued asset wrapper; its corrected string-asset scan exited0. The in-memory 1501 materialization probe returns `(('alice','WETH'),1501,1510)` and `(('maker1','WETH'),3499,3490)` as expected-versus-inherited-post differences. No tampered candidate was submitted to root verify2; passing cash/work checks are not full semantic validation.

Hash checks confirm all three candidate files, two bound inputs and five review inputs are unchanged. All three unique source pins match at first source access and after review; no start-of-review source filesystem snapshot is claimed. Candidate digest and 453720-byte total match the freeze.

Completed: 2026-09-08 08:21:37 UTC. Approximate elapsed review seconds: 607.683.

All substantive tests and limits are recorded in `gpt6-admission-result-review.json`.
