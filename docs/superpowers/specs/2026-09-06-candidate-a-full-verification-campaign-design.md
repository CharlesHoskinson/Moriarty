# Full Candidate A bounded-verification campaign proposal

Status: proposed design and campaign manifest, not execution authority. Prepared from current HEAD fbea1cee5de497a3e58fe3081bb49b4059edd97b while the separate A5 pilot is active. Its 36 frozen inputs remain unchanged. A successful independently admitted ten-stage pilot is a hypothetical entry prerequisite, not a result asserted here. If it fails, this design remains reviewable but does not authorize another costly attempt.

## Contract and architecture decision

The authority is completion XML A5-R01–R05 and A2-I01–I11/A3-S01–S08, plus the original factored-verification plan's full obligation map at line1186. A5 requires complete bounded workloads after the funding pilot. Neither corpus tests nor a positive pilot substitutes for terminal checking of those workloads.

Recommended approach: preserve every existing original and all26 derived modules; add campaign wrappers and an explicit adversarial transition driver with corresponding original/factored instances. Partition the campaign by existing workload/profile/scenario for compilation, but retain the complete union in the acceptance manifest. Compare each pair's full reachable records and guards, not only balances. This keeps the established H1 delegates and complete existing history semantics.

Two alternatives were considered. A single union of every workload and all histories would make coverage simple but forces the compiler to process unrelated branches together; it is not the first proposal after the documented resource failures. Propagating the pilot's13-helper omission would delete routeS, staleRouteS and nonCommitS used by the full workloads, and is rejected. A different history recurrence, record summarization or internal reduction microsteps would be a new factoring hypothesis and requires a separate preservation design.

All files below are **proposed future files** under specs/quint/s02/full_verification/, in a later reviewed implementation epoch. This document creates none:

- campaign_domains.qnt: references existing finite domain values and supplies exact workload/variant inventories; no replacement semantic types.
- campaign_properties.qnt: pure full-record and transition predicates specified below.
- core_campaign.qnt, installment_campaign.qnt, swap_campaign.qnt: wrappers around existing original harness init/step, with observable endpoint and enabledness operators.
- authority_interleavings.qnt: the new driver, with thin guarded actions calling the actual A boundary and execution updates.
- campaign_equivalence.qnt: paired original/factored transition miter, including admissibility equality and complete state/history equality.
- campaign_controls_test.qnt: deterministic action-level negative and restoration controls.
- Literal per-row wrapper modules: bind profile/scenario/case/selected property to each command row without runtime evaluation of a union of all integrated histories.

Each factored wrapper routes through the existing26 derived modules and joint kernel. New wrapper and driver bodies must be generated once with a finite import/module substitution for the other side, checked by roundtrip. Original bodies, signed policies, plans, rejection records, map keys and financial programs remain intact. Any direct AuthorityKey import needed for alias visibility is confined to a new complete compilation copy with a declared byte delta on both sides. The pilot24-file pruned view is never an input to this campaign.

## Exact retained domains

Source identity and complete ordered244-test inventory are frozen in .superpowers/sdd/a5-full-campaign-design-source-index-20260906.json (SHA256 ef586a27b237a5562f029f2ec4b0c8a44d72f9f13381750a7123d037dd57f5d1). Its28 original derivation pins and26 names are normative inventory references, not selected examples.

| Domain | Proposed campaign value |
|---|---|
| Principals/assets | PRINCIPALS = Alice/Bob/Mallory; both TokenA/TokenB; all six CORE_ACCOUNTS and every LEDGER_KEYS entry retained |
| Authority | All12 AUTHORITY_KEYS: SwapDomain/InstallmentDomain × three principals × nonce0/1; all8 ATTEMPT_IDS; complete registry, parent, signing and attempt maps |
| Core | All16 node IDs, five choice IDs, all original complete program maps including unused nodes; A_TIMES = 0/1/2/100/101; constants -1/0/1/5/10/20; choices -1/0/1/2; deposits -1/0/1/5/10/20/21; original balance340 potential and reduction bounds unchanged |
| Live environment | physicalTime in1/2/100/101; anchor0/1/2; implementationVersion0/1; enforcementMechanism0/1, exactly validEnvironment. Time0 belongs to Core and invalid-live-context controls, not live environment reachability |
| Signatures/evidence | Both signing profiles; tokens0/1 in adverse calls, canonical token0 retained; EvidenceValid/Unavailable/Invalid independently for effect and signature proofs; symbolic external verifier premise, no cryptographic theorem |
| Financial workload | Installment budget10, slots1/2 at5 each, residual5, recovery10/5; swap Alice10 TokenA and Bob20 TokenB. No saturation or reduced account/choice maps |
| Installment scenarios | TwoFillsI plus RecoverI(residual=false/true, mode=Choice2I/Timeout100I/Timeout101I/Refuse100I/Refuse101I):11 ×2 profiles |
| Swap scenarios | SettleS and RefundS at funded2; TimeoutS at funded0/1/2 and time100/101; RefuseS at funded2, times100/101 and chosen0/1:12 ×2 profiles; plus staleRouteS ×2 profiles |
| Corpus expansion | Original24 CASE_REQUESTS, all244 ordered original tests and their factored copies, six full-record equivalence tests, all current lifecycle/common controls; future accepted A4 exact78 IDs and every associated mandatory negative are an additional union, never a replacement |

The anchor0/1/2 choice preserves the actual source domain rather than silently narrowing it to the minimum0/1 mentioned in the obligation map. The complete finite program corpus means the admitted literal program/case inventory with full maps, not every mathematically possible16-node program. The manifest must state that scope in each claim.

## New adverse-interleaving model

This is shared-state coordination, so plain Quint is appropriate. There is no invented message protocol or need for Choreo. The model retains AAuthorityExecution as one complete state value. Driver metadata is separate:

- workload: Installment(ScenarioI) or Swap(ScenarioS), and profile, chosen in init;
- execution: unsignedI or unsignedS, exactly as in the original fixtures;
- events: a list of tagged actual transitions; counters/seen flags are derived from that list;
- previous execution, last event and last supplied evidence/observation, retained only for state predicates about the last transition; init gives an explicit NoEvent case;
- active policy/request selections are values supplied to a boundary call, not mutation of an already registered policy or signed snapshot.

Init also assigns all metadata. It does not pre-sign, pre-register, pre-compute a successful attempt, or initialize a witness true. All updates assign every variable explicitly. There is no model clock/cursor cap that silently disables nonterminal behavior: checker depth bounds the trace. Completed/rejected attempts are never cleared or recycled.

The native ordinary route rows remain unchanged. The new driver's scheduler instead chooses any enabled command from the finite workload command alphabet, with no route cursor. For installment this is all existing CommandI constructors with relevant IDs plus general proposed/verified rejection calls; for swap it is all existing CommandS constructors with relevant IDs and principals. Actual canCommandI/S and applyCommandI/S remain the ordinary command gates and updates. In particular the installment raceReadyI guard stays: both contenders must be verified before either race commit. This exposes both proposal orders, both verification orders and both winners. It does not erase the existing semantics to make progress.

Add the following independent actions to that scheduler:

| Action | Guard and exact update |
|---|---|
| AdvanceEnvironment(t) | Valid execution, t in1/2/100/101 and t greater than current physicalTime; change only context.environment.physicalTime. Preserve anchor/version/mechanism and every other record |
| ChangeAnchor(a) | a in0/1/2 differs from current anchor; change only that field |
| ChangeImplementation(v) | v in0/1 differs; change only implementationVersion |
| ChangeEnforcement(v) | v in0/1 differs; change only enforcementMechanism |
| Offer(policy, signer, token) | Uses canPrepareSigningAuthorityA or canSignAuthorityA for the appropriate stage and actual applyPrepareSigning/applySign. Policy candidates are the canonical workload policies and exact mutation cases below. No edit of registered authority |
| ProposeVariant(id, observation, actor) | Actual canPropose then applyPropose; only a NoAttempt slot. Includes canonical adaptAuthorityA output and finite explicit malformed observation variants; never installs VerifiedOperation directly |
| VerifySupplied(id, evidence) | Actual canVerifyAuthorityA then applyVerify, with evidence selected for the currently retained ProposedAttempt |
| RejectSupplied(id, evidence) | Actual canRejectProposedAuthorityA then applyRejectProposedAuthorityA; preserve the exact offered evidence, attempt and observed context |
| RejectVerified(id) | Actual canRejectVerifiedAuthorityA then applyRejectVerifiedAuthorityA. Never rebind stale evidence to current state |
| DeniedBoundaryProbe(call) | The actual guard is false; execution unchanged and a once-per-distinct-call/event diagnostic is appended. This records an observed refusal for witness purposes and is not a business transition or an unconditional stutter |

Ordinary AdvanceI/AdvanceS retain their original updates, including their environmentI/S reset to zero anchor/version/mechanism. They are distinguished from AdvanceEnvironment in history. That reset is an explicit existing transition, not a silent merge with the new per-field actions. Every scheduler branch may interleave at every enabled boundary; changes can repeat or revert within the declared32-transition bound. Finite once-only denied probes prevent observational self-loops; environment loops remain honest bounded behavior. No fairness is assumed.

Artifact identity is **not an Environment field**. ACall contains the full request/program and policies contain either an after-resolution plan or a before-resolution artifact constraint. Model artifact substitution at Offer/ProposeVariant/VerifySupplied: choose the canonical complete program or a same-root full-map program with N15 changed from CloseA to PayA(aliceA,Alice,ConstantA(0),N0); choose a call-time/input or plan-identity substitution as specified by the existing mutation controls. The unused-node variant remains syntactically valid but differs as an artifact. Do not replace live candidate.program, ledger or a stored signed policy by fiat. That would model an additional state-installation power absent from the current architecture.

Use finite mutation families with explicit labels and field-local deltas: unchanged; unused-node artifact; second planned operation successor/input/call/time/projection/effects/predecessor-facts; observation predecessor/successor/input/call/time/full raw projection; reversed/duplicated/omitted/extra transfer; wrong actor/principal/nonce/token; missing/extra proof key; effect or signature EvidenceUnavailable/EvidenceInvalid; old attempt proof; plan-view mismatch; display PublicDisplay/AlternateDisplay; disclosure set changes copied from the common policy controls. These labels are not permission to synthesize expected results: use actual adapter output before a mutation and retain that original alongside the exact delta. The implementation plan must enumerate every existing boundary/adapter/common test delta, including malformed-map cases, into literal data. Keep malformed state-envelope controls separate from valid-environment reachable states.

A display-only change can be semantically harmless; do not demand universal rejection. Check equality of admissible ordered effects/financial result under both displays. A changed signed disclosure policy must not become registered merely because a supplied proof carries it. A faithful independently signed artifact under AnyArtifactUnderMechanism can be permitted if all A and policy guards hold; the invariant is exact bound authorization, not “all changed artifacts reject.”

## Properties with exact semantics

Existing properties retain their original definitions and get separate rows or an explicitly mapped conjunction:

- coreTraceSafety in candidate_a_harness, installmentTraceSafety in candidate_a_installment_harness, corpusComplete in candidate_a_cases.
- adapterBindingSafety and boundarySafetyA, retaining their narrow original harness domains.
- installmentSafetyI = safetyI(state) and lastAtomicI and cursor/history constraints.
- swapSafetyS = safetyS(state) and lastAtomicS and cursor/history constraints.
- All integrated sourceInvariantA4 and noDiagnosticA4 rows use the complete accepted case input and actual completeA4 witness.

New driver properties are state operators over the actual execution and the last transition, not labels for tests:

1. fullMapsAndMoney: validAuthorityExecutionA plus the workload's original totalAsset equalities, all account/ledger coupling and nonnegative balances; installment additionally preserves parent paid+allowance10 and actual original safetyI history restrictions where applicable. Each premise's domain is explicit; malformed-envelope controls do not weaken valid-state invariants.
2. executedEnvelopeExact: for every ExecutedOperation(v), require authorityAttemptMatchesA(v.attempt), authorityEvidencePoliciesMatchA(v.evidence), verificationValid(v.attempt.context,v.attempt,v.evidence), and exact needed proof keys. The historical attempt.context is intentional: subsequent legitimate commits/environment changes must not invalidate an earlier execution. This checks whole call/program/plan/result binding, multiset/order/destination clauses and consumption guard, not only sums.
3. transitionExactFull: commit before/after equals applyCommit for the selected verified attempt; signing equals the actual signing update; environment edit equals the stated single-field record update; proposal/verification equals its actual update. Other records, including consumed registry cells and parent history, are unchanged except fields changed by that operation. Keep transitionExactI/nonCommitS on their original row domains too.
4. rejectedRecordExact: new RejectedOperation has the prior attempt, exactly supplied or retained evidence, pre-transition current context, authorityRejectionReasonA and the correct boundary. Context, signing and every other attempt are unchanged. Rejected and Executed records are immutable under all future actions.
5. staleCannotAdvance: whenever a PreparedSigning snapshot differs from signingSnapshot(currentContext,body), that exact canSignAuthorityA call is false; for proposed/verified attempts whose retained context differs, actual verification/commit guard is false. Refusal does not erase the record.
6. noReplayAndResidualExpansion: actual consumed nonce/history and slot multiplicity rules from operationFinancialGuard/committedParents/committedRevision; no duplicate slot increases paid twice, cancellation moves no funds, recovery changes neither original parent nor nonce0, total paid never exceeds10. Compare transitions and retained maps, not a new summarized accounting record.
7. displayCannotGrant: canonical and display-only variants produce the same permission/effects under a matched context/policy; proof and signed-policy substitutions cannot bypass equality against registered authorities. This requires paired calls from the same retained state; no broad assumption that display changes must reject.
8. enabledOrClassified: for scheduled rows, cursor at exact route length OR the next canCommand holds. For the adversarial driver, expose businessEnabled and environmentEnabled separately and classify settled, retained-refusal, stale-signing-blocked and depth-cutoff observations. Do not classify noPending alone, a refusal, or a depth bound as financial termination. All pending proposed/verified records must have an available verify/commit or corresponding rejection branch in a structurally valid state.

The miter checks both guard booleans on the same chosen call and full output record equality when enabled; the observation/evidence mutation on each side is the same explicit field delta. It also compares diagnostic outcomes when an adaptation fails. It does not assume away failed guards. Original/factored state histories start equal and must stay equal on every checked transition. This supports the declared bounded domain only.

## Proposed finite campaign rows

All rows compile with default flattening to compound .qnt.json and select the actual wrapper init, step and one named property/conjunction. Inspection must verify q::init/q::step/q::inv and the exact wrapper selected. No no-flatten result discharges these rows. Proposed per-command ceilings: compile900s/4096MiB; offline checker600s/4096MiB; typecheck/test1200s/4096MiB, retaining existing ceilings without an automatic increase. These are proposals for root adoption, not permission to start. One heavy command at a time.

| Family / deterministic expansion | Transition bound | Required safety result and endpoint evidence |
|---|---:|---|
| core-corpus, all24 CASE_REQUESTS in their original order | 0 | corpusComplete at init, actual computed payloads plus independent complete Python comparison; this is explicitly a state0 check, not a transition witness |
| core-swap, original candidate_a_harness | 4 | coreTraceSafety; all eight original recorded action witnesses and terminal distinction |
| core-installment, original candidate_a_installment_harness | 3 | installmentTraceSafety; first/second fill, both refund amounts, both deadline amounts and both deadline supplied-input rejection witnesses |
| adapter-original-harness | 1 | adapterBindingSafety and reached adapterProducedFirstFill |
| boundary-funding ×2 profiles | 5 | boundarySafetyA and every prepare/sign/propose/verify/commit endpoint, full profile completion |
| installment-routes ×11 scenarios ×2 profiles | 20 | installmentSafetyI plus full executed/rejected/history conjunction; each exact scenario completion; all original named action witnesses on their applicable rows |
| swap-routes ×12 scenarios ×2 profiles; stale ×2 profiles | 22 | swapSafetyS plus full executed/rejected conjunction; each exact completion, both disposition signers, each time advancement and each rejection class |
| interleaving-installment ×11 scenarios ×2 profiles | 32 | properties1–8 and miter; both contender orderings and winners, fresh cancellation, legal recovery, denied replay/old nonce/missing cancel, environment changes at each exposed boundary |
| interleaving-swap ×12 scenarios ×2 profiles plus stale ×2 profiles | 32 | properties1–8 and miter; funding/disposition boundaries, stale prepare/verified refusal, environment/artifact/evidence/display/disclosure matrix |
| integrated-A4 ×all78 accepted literal case IDs | each accepted case's unchanged steps, no inferred27 for every case | sourceInvariantA4/noDiagnosticA4, completeA4 and exact terminal case records; all mandatory negatives and mutations retained separately |

32 is a new declared exploration depth, allowing the existing20/22-transition route plus additional adverse actions. It is not a proof that every arbitrary number of environment changes was explored, and is not a guard/terminal condition. For endpoint tests the manifest stores the exact expected earliest/reachable trace length from an executed reviewed control; no successful count is invented now.

Each ordinary route/profile completion needs its own negated endpoint query or a checked existential query producing a retained trace. Aggregating profile witnesses across runs cannot satisfy missing combinations. A safety conjunction needs actual NoError through the requested states0..N (or the checked terminating behavior described below), and a selected witness query needs actual reached-state evidence after init. For the nondeterministic interleaving rows require concrete traces for each environment field change between prepare/sign, sign/propose, propose/verify and verify/commit where that boundary is reachable, plus repeated/reverted-change traces. No claim that all actions occurred comes from an actionWitness meaning merely history nonempty.

Normal terminals retain disabled step. Use the established offline no-deadlock checker setting for safety and a separately checked enabledOrClassified predicate. Record exact explored depth; do not infer states0..N from exit0 if the tool actually stopped early. Endpoint queries distinguish business completion from deliberate refusal and bound truncation. No fairness/liveness or inevitable recovery assertion is made.

## Required negative and preservation campaign

For every future selected property, the implementation manifest names a specific, typechecking mutant and affected input row; “negative” cannot mean a syntax error. Minimum classes:

- supplied-result substitution in the joint kernel; full-record corpus/miter must detect it;
- authorityAttemptMatchesA bypass, at actual verification and commit: rebound successor/unused-node/call/result mutations must produce the expected reachable violation;
- stale signing snapshot check bypass and stale execution-context check bypass, separately; an intervening environment action must reach unauthorized registration/commit in the mutant;
- effect-order or multiplicity guard bypass: actual two-payment order, duplicate/extra effect controls must fail;
- consumption/parent-revision bypass: duplicate fill, cancel race loser, old nonce recovery controls must fail;
- rejection update corruption: alter one retained evidence/context/reason/stage or rollback field and require rejectedRecordExact/full-record comparison failure;
- display/disclosure substitution bypass of registered-policy equality must be detected on an actual affected attempt.

A mutant may remain blocked by an independent gate. Such a surviving test is not a genuine RED and does not validate the detector: review the concrete counterfactual and choose a targeted affected control before spending another command. Never weaken production guards to make a control “work.” Each temporary mutant is a fresh copied input, has a retained baseline/mutant/report triple, and restores to exact original hashes; no mutation of primary/frozen originals.

Re-run complete original/factored244 tests, six equivalence tests and every added full-campaign/mutation test after implementation. Preserve the existing441 Python cases and complete Core53 comparison at their admitted scope; current expanded corpus must receive its own complete comparison. Future A4 integration uses all78 cases including failures/negatives, not only accepted successful traces. The current failed literal014 is still failed; this proposal grants it no acceptance. A4's independent exporter/checker gate remains separate from A5's transformation preservation.

## Manifest and execution gate shape

The future materialized manifest is a finite expansion, not a wildcard command generator. Every row has these mandatory fields:

rowId; side(original/factored/miter/control); module path/hash; complete transitive source closure and byte-delta provenance; original26/28 inventory membership; accepted A4 case/control ID when applicable; selected init/step/invariant and full conjunction member names; exact domains and profile/scenario; depth; expected semantic result; endpoint target and minimum nonzero transition; fairness:none; disabled-terminal rule; enabledness operator; compile/checker argv; tool/runtime pins; finite time/memory bounds; fresh output directory; prerequisite row IDs; original transport endpoints; actual outcome classification.

Deterministic row IDs are family/profile/scenario/property, using existing enum/record values and A4's authoritative case_id strings serialized in a frozen order. Finite names and counts are validated before dispatch; duplicate/missing/extra rows block admission. The integrated family must reconcile all78 accepted inventory entries and controls to final source digests before it can be frozen. The implementation plan must supply exact argv and the expanded manifest; this design does not counterfeit those not-yet-created paths/hashes.

Fresh evidence root proposed: .superpowers/sdd/a5-full-campaign-v1/. Reuse existing record() and offline capture interfaces with complete extra input closure, no helper-global mutation or new generic capture framework. Record original source, compiled JSON, selected declarations, time/resource stderr/stdout, SMT/checker logs/counterexamples, actual child/recorder/outer distinctions and owned-group cleanup. Reference shared pinned runtimes rather than nesting repeated archives.

Sequence for review: source/model design admission; exact driver/property and manifest implementation plan; original/factored corpus and meaningful negative controls; per-family paired default flattened compilation and raw size/selected-binding intake; root review before each offline family; terminal result review for every property and endpoint. H1 size comparison remains a distinct claim: preserve paired bytes and report any non-smaller family honestly. A non-smaller input cannot be called an H1 size win; any further execution requires root's explicit disposition, not relabeling.

Timeout/OOM/unsupported translation leaves its row open. It supplies neither an architecture counterexample nor an acceptance result, and permits no automatic heap increase, pruning, retry or substitution of a shallower row. The independent result reviewer maps exact terminal artifacts back to all rows. A5 acceptance needs the complete required union; A6/Council/dossier remains a later separate obligation.

## Source-only handoff and unresolved implementation gates

This proposal resolves the missing driver architecture and proposes32-transition adverse exploration with explicit finite inputs. It deliberately leaves implementation, exact expanded command materialization and empirical resource feasibility to the next reviewed plan. Adoption must approve the new driver/domain/depth/property semantics and the complete A4 union; no current source-freeze release is implied.

Grounding reads: XML A5 lines365 onward and A2/A3 case clauses; original plan1186 onward; authorization signingSnapshot/canPrepareSigning/canSign; execution canPropose/verificationValid/canCommit/applyReject*/applyCommit; actual A boundary wrappers; original full lifecycle fixtures and harnesses. Static source-index tool812434/0 checked28 pins/26 modules/244 ordered run names and unchanged frozen36/current HEAD. Read0537ee exited2 because a requested nonexistent signing.qnt path was included; subsequent readf6ee2b/0 inspected the real authorization.qnt. No helper, model, compiler, solver or runtime archive executed or audited. All outputs here are proposal documents or static-source receipts.
