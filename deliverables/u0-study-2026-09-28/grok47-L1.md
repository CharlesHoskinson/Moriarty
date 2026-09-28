I'll study U0 through the semantics lens only: the stage-relation schema, the six judgments, and K-reference coverage. Reading the contract, embeddings, and checkers first so the proposal cites what is actually recorded.## 1. Goal restatement

U0 has to freeze one stage relation that U1 and U2 can implement, and demonstrate that relation on the smallest signed financial stage. That stage is one single-asset transfer that funds one repay of an already admitted obligation: two-sided gross movement, principal/accrued/outstanding evolution, allowance consumption, replay, atomic rejection, and a canonical predecessor preimage. U1 then certifies arithmetic and native feasibility against those laws. U2 carries the same relation through a pinned ZKIRv3 path, with a second program, authenticated history, and signer checks.

A recorded JSON inventory is not that contract. `deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md` already says the contract is recorded and the capabilities are open. U1 cannot start from 66 absent leaves and a schema that cannot state the liability law the lifecycle code already checks.

## 2. Findings

**The schema is a closed bag of strings.** `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` (`$id` `…/stage-relation/1`) has 18 required objects and 84 leaves. Amounts, including `signedIntent.grossDebitCap` (lines 158–160) and `effects.gross[].amount` (lines 297–299), match `^-?[0-9]+$`, so negatives and leading zeros validate. `authority.consumed`, `authority.remaining`, and `authority.replayState` are single strings (lines 461–468). `effects.gross[]` has one `account` (lines 285–302). `liabilities.opening[]` and `closing[]` each have one `amount` (lines 391–414). Judgment status is `held | violated | unchecked` (lines 573–578) with no schema rule that an accepting stage requires `held` and a non-null `enforcementRef`. `unchecked` is a valid document.

**The lifecycle kernel already has a stricter model, and the embedding does not use it.** `experiments/moriarty-language/src/successor/financial-lifecycle.ts`:

- `TransferAction` has `from` and `to` (lines 85–92), not one account.
- `LifecycleObligation` has `principal`, `accrued`, and `outstanding` (lines 63–83).
- `parseObligation` rejects a state unless `principal + accrued = outstanding` (lines 854–859), and `requireSignedBound` admits only canonical `UInt128` text (lines 464–476).
- `applyRepay` rejects `nominal > outstanding` (lines 1568–1570). `allocateNominal` rejects `dP > principal` or `dA > accrued` (lines 1426–1428).

The partial rows cite `outstanding` as both `liabilities.opening[].amount` and `liabilities.closing[].amount`, and `TransferAction.from` as `effects.gross[].account` (`source-core-embeddings.json` classification table: `LifecycleObligation`, `TransferAction`). Promoting those rows to `present` would drop principal, accrued, and the counterparty. Opening and closing are the same record type seen twice. There is no evolution law in the schema.

**Realisation is one-sided name matching.** The checker (`scripts/check_u0_stage_schema.py` lines 1–6) proves declaration, context, and profile membership only. Profiles are source/5 (`financial-agreement-source-v5-grammar.ebnf`, `frontend.ts`) and lifecycle/1 (`financial-lifecycle.ts`, `formal/k/lifecycle-v1.k`). Of 84 rows: 1 `present`, 17 `partial`, 66 `absent`. All 17 partials cite Core only. The one present leaf, `profiles.semanticProfile`, cites `ProfileDecl.value` in `frontend.ts` and records Core as null. Its note says Core has no semantic-profile field. The boolean `present` is true for every non-absent row (`classificationRule` in the embeddings file), so a sum of that boolean is 18.

The 12 leaves `judgments.{stage,intent,effect,authority,history,failure}.{status,enforcementRef}` are absent and are not in any judgment’s `schemaFields`. They are results of evaluating laws. Searching source/5 for a declaration named `status` cannot realize them.

**The six judgments are prose partitions.** `judgments.json` maps `stage` to “contract properties”, `intent` to “intent refinement”, `history` to “compliant history”, and both `effect` and `authority` to the same design sentence, “valid state/effect transition”. `failure` lists only `failurePolicy.phasePolicy`, `retainedEffects`, and `retainedFees`. The design sentence it cites (`docs/MORIARTY-CONSOLIDATED-DESIGN.md` lines 48–49; `docs/MORIARTY-PRODUCT-CONTRACT.md` lines 45–47) also accounts for authority and residual duties. Definitions say when a judgment “holds”. They do not say when it is `violated`, when `unchecked` fails closed, or which conjunct may stay open until a later milestone.

**K reconciliation is a universal checklist with no covered row.** `k-reconciliation.json` has UNI-001–017 plus the six judgments: 0 covered, 5 partial, 18 not covered. Partials cite symbols such as `lcRepay` and `lcTransfer` in `formal/k/lifecycle-kernel.k` (repay rules at lines 50–54) and `<effects>` / `lcKernelRejected` in `lifecycle-v1.k`. `scripts/check_u0_k_reconciliation.py` proves citations and verbatim quotes. `executionEvidence` quotes the 2026-09-17 run (125 expression cases, 104 lifecycle cases, four-step loan). That run is regression evidence. It is not a match against a stage-relation record. History is `not-covered`: “K lacks predecessor commitments and obligation commitments.” `lifecycle-kernel.k` is not in the embedding `profileFiles`.

**The source/5 repay counterexample is a different artifact from the lifecycle kernel.** `openspec/changes/aeon-refinement-integration/proposal.md` lines 10–16 reports an SMT encoding of `repay` in `loan-lifecycle.mori`, using that file’s `requires` clauses and AccrualFirst in `src/successor/repayment.ts`, satisfying every written guard while `principal` and the borrower balance go to `-1`. `evidence/verification-discharge/counterexample-repay.txt` lines 6–12 shows `principal_1 = -1` once range assumptions are dropped. Source/5 has no production for `nominal <= outstanding(...)` (`financial-agreement-source-v5-grammar.ebnf` financial reads are terminals, lines 98–128). That shows fixture `ensures` clauses are not the effect judgment. It does not show that `applyRepay` accepts a negative principal.

**Mis-scope.** `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md` lines 29–61 says U0 must freeze source forms for continuations, evidence, phase, recovery, delegation, and private history, and also says this audit does not invent a source `/6`. Those later interfaces are U3–U5 capabilities. Stuffing them into source/5, or into the uncommitted Aeon `/6` refinement profile, does not embed the stage relation.

## 3. Proposed work

Slice **R0** is the critical path. One signer, one domain, one settlement asset, one admitted obligation, one transfer, one repay, atomic rejection, empty fees, empty supply changes, empty observations, empty disclosures, unit policies `consent = none`, `delegation = none`, `recovery = none`, `outcome = terminal`. Accrual is outside R0: interest creation waits on the numeric profile. Settlement conversion stays a cited function (`convertNominal`), bound by numeric-profile id, and does not close that profile’s six conformance gaps.

Critical leaves, in order: relation `schemaVersion`; `profiles.semanticProfile` and `profiles.numericProfile`; `programIdentity.{programId,entryPoint,sourceRef,coreRef}`; `lifecycleIds.{lifecycleId,stageId}` with `logicalRequestId = stageId`; `domain.domainId`; the signed-intent record (`intentId`, `signer`, one asset, creditor recipient, `grossDebitCap`, `replayPolicy`, unit fee and net laws); two-sided gross lines; liability components and the evolution law; allowance entries and replay ids; failure policy `atomic-reject` with empty retained effects and fees; predecessor and obligation preimages. Circuit pins, `chainId`, `stateFrameRef`, and authenticated history are required conjuncts that stay `unchecked` with owners U1 and U2. Acceptance is false while they are unchecked.

1. **Repair the relation.** Bump the schema to `moriarty-stage-relation/2`. Keep the 2026-09-23 packet as the recorded baseline. Amounts are canonical non-negative integers. Gross lines are `{asset, from, to, amount}`. Liabilities carry `principal`, `accrued`, `outstanding`. Authority `consumed`, `remaining`, and `replayState` are arrays of allowance and id-list entries. Add law ids: `outstanding = principal + accrued`; repay does not increase either component; `nominal <= outstanding`; settlement amount matches the funding transfer; per-asset gross lines balance; net is the function of gross and fees; atomic rejection leaves post-state, allowances, and replay ids unchanged. Output: `schemas/stage-relation.schema.json`, `schemas/stage-laws.json` (hashed beside `stageSchemaSha256`). Checker: extend `scripts/check_u0_stage_schema.py`. Test: `tests/test_u0_stage_schema.py`. Exit: a negative amount, a one-account gross line, and a document whose required R0 judgment is `unchecked` fail validation. Dependencies: none. Effort: **M**.

2. **Make the six judgments evaluable.** Replace prose-only `judgments.json` with `holdsWhen`, `violatedWhen`, `uncheckedWhen`, `schemaFields`, and `ownerIfUnchecked`. Split the old “valid state/effect transition” sentence so effect owns observations, gross/fees/net/supply, liabilities, and outcome, and authority owns allowance, replay, and the R0 unit resource law. Failure for R0 is non-consumption of authority, not a second copy of the effect. Acceptance is the conjunction of the R0-required judgments, each `held` with a non-null `enforcementRef`. Output: `deliverables/u0-semantic-contract-2026-09-23/` successor directory `judgments.json`. Checker: C9 in `check_u0_stage_schema.py`. Exit: every leaf is `required`, `unit`, `unchecked-owned`, or `deferred-owned`. Dependencies: task 1. Effort: **S**.

3. **Project one R0 fixture.** Add a pure function from `(signed-intent record, LifecycleInput, LifecycleResult)` to a schema-valid stage document, calling `prepareFinancialLifecycle` rather than reimplementing repay. Positive fixture: transfer plus repay. Negative fixtures: nominal above outstanding; replayed transfer id; missing signer; obligation whose components do not sum. Do not edit source/5, `loan.mori`, or Preview loan bytes. Output: `experiments/moriarty-language/src/successor/stage-projection.ts` and fixtures beside it. Test: `tests/test_u0_stage_projection.py`. Exit: the positive fixture yields `held` on effect, authority, and atomic failure; each negative fixture yields `violated` and is not accepted; circuit and authenticated-history conjuncts are `unchecked`, so native acceptance is false. Dependencies: tasks 1–2. Effort: **M**.

4. **Retarget embeddings at the projection.** `present` means the projection emits the field and the cited law id matches a declaration the projection reads. `partial` means a unit law or a one-sided citation. Demote `profiles.semanticProfile` until the stage record and Core both carry it. Do not mark `RepayAction.obligationId` present as a commitment; the commitment preimage is the canonical obligation components. Output: new `source-core-embeddings.json`. Checker: `check_u0_stage_schema.py` gains a projection-fixture clause. Exit: every R0 required leaf is `present` on the strength of the task-3 test; no leaf is `present` only because a symbol name matched. Dependencies: task 3. Effort: **M**.

5. **Cover K on that same fixture only.** Add a K trace whose post-state matches the projection on principal, accrued, outstanding, balances, allowance spent/remaining, and used transfer and allocation ids. `covered` requires that trace digest, not a symbol citation. Give every other row `deferredTo` and an owner (table below). Output: new `k-reconciliation.json` plus the trace under `deliverables/`. Checker: `scripts/check_u0_k_reconciliation.py`. Exit: effect-slice and authority-slice rows are `covered`; history-preimage is specified and matched on canonical bytes; authenticated history stays deferred to U2; zero rows are `not-covered` without an owner. Dependencies: task 3. Effort: **M**.

6. **Record the source/5 gap as a law test, not a new language profile.** One projection fixture documents that `loan-lifecycle.mori` `ensures` clauses are outside the effect judgment, and that a broken component sum is `violated`. Output: the negative fixture in task 3 and a row in `judgments.json`. Checker: the projection test. Exit: the `-1` model is refused by the R0 law without adopting `openspec/changes/aeon-refinement-integration/`. Dependencies: task 3. Effort: **S**.

| Deferred conjunct | Owner |
| --- | --- |
| `circuitIdentity.*` able to hold | U1 |
| Numeric-profile conformance, including accrual | U1 |
| `chainId`, `stateFrameRef`, signer at the native boundary, authenticated predecessor, second program, effect readback | U2 |
| Non-empty fees, observations, continuations, `phase-retain` failure | U3 |
| Recursive multi-parent history | U4 |
| Cumulative reservations, delegation other than `none` | U5 |

## 4. Exit redefinition

Yes. Split the gate.

**U0-F, contract frozen.** Schema `/2` and hashed laws, evaluable judgments, every leaf classified `required | unit | unchecked-owned | deferred-owned`, K rows `covered` or `deferredTo`. `check_u0_stage_schema.py` and `check_u0_k_reconciliation.py` fail closed on a required R0 leaf that is absent or a deferred row with no owner. This is the artifact U1 and U2 may depend on.

**U0-D, slice demonstrated.** The task-3 projection and the task-5 K post-state match. Effect, authority, and atomic failure are `held` for the honest fixture and `violated` for the negative ones. Stage identity is `held` only for profile, program, domain id, and lifecycle ids. Circuit binding and authenticated history remain `unchecked`, and the native-acceptance conjunction is false. That false result is the handoff. It is a demonstrated judgment, not an open design question.

U0 does not demonstrate a Midnight-accepted stage. U2 owns that, on this frozen relation.

## 5. Risks and things not to do

The schema bump changes `stageSchemaSha256`. Regenerating embeddings while leaving the old hash in place would make the checker lie. Keep the 2026-09-23 files intact.

K uses unbounded `Int` in `lcRepay` (`lifecycle-kernel.k` lines 52–54). The correspondence trace has to use values inside the lifecycle `UInt128` bounds and include one near-bound case, or state the bound as a premise of `covered`. A symbol-level match repeats the class of hole in `counterexample-repay.txt`.

Do not reclassify the 17 partials by renaming `from` to `account` or `outstanding` to `amount`. Do not search source/5 for the 66 absent names. Do not put stage embeddings in Aeon source `/6`. Do not edit Preview loan bytes. Do not mark UNI-001–017 covered from the 2026-09-17 suite. Do not treat checker exit 0, today or after a wording change, as a demonstrated capability (`EXIT-GATE.md` limitations; `docs/FOOTGUNS.md` items 1 and 4). Do not block the semantic freeze on circuit pins or on concrete syntax for recovery, delegation, and private history.

## 6. Disagreements

The roadmap exit row (`ROADMAP.md` line 21) lists versioned embeddings, six judgments, and “K-reference reconciliation status” as U0 evidence. The 2026-09-23 packet meets that wording with 1/17/66, six prose judgments, and 0/5/18, while `EXIT-GATE.md` correctly leaves capabilities open. U1’s dependency should be U0-F plus the R0 demonstration.

The embedding method equates a leaf with a same-named declaration. Net, judgment status, and commitments are computed. The checker’s one-sided `present` rule made `profiles.semanticProfile` the only present leaf while Core has no such field.

Freezing schema `/1` would freeze a relation the lifecycle kernel does not implement. Two-sided transfers and the component invariant are already in `financial-lifecycle.ts`. The canonical schema should follow those laws.

`docs/MORIARTY-CONSOLIDATED-DESIGN.md` line 48 states four judgments. Six is the right refinement for U0, provided effect and authority have distinct `holdsWhen` texts. Alignment line 61 asks U0 to freeze source forms for the whole later interface set and also forbids inventing source `/6`. Freeze R0 sorts and unit values. Leave general syntax to the milestone that demonstrates it.

K’s 0 covered rows are the honest score of an oversized matrix. `lcRepay` existing is not coverage. UNI-001 and UNI-016 are release conditions. UNI-007, UNI-009, UNI-010, and UNI-011 belong to U3–U5. Keeping them as U0 holes makes the semantic exit unfinishable before those milestones.

The Aeon `-1` result is about `loan-lifecycle.mori` and `repayment.ts`. Using it to open a refinement-type profile inside U0 replaces the missing effect law with an SMT workstream. Task 6 is the U0 obligation.

## 7. Top three recommendations

1. Rewrite the stage schema around the laws `financial-lifecycle.ts` already enforces: two-sided gross lines, principal/accrued/outstanding, allowance-shaped authority, and fail-closed `unchecked`. Stop the declaration search.
2. Demonstrate R0 by projecting one funded repay and its negative witnesses, and redefine `present` as “this projection emits the field under the cited law.”
3. Mark K `covered` only where that same post-state matches, and assign every other reconciliation row to a named milestone.
