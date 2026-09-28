# L3 proposal: numeric profile and exit path (Claude Opus 5.5)

## 1. Goal restatement

For U1 and U2 to start, U0 has to fix three things, and none of them needs a working implementation. First, a frozen, versioned numeric contract. For every primitive the initial slice uses, it states the exact value, the width, the unit dimension, the rounding direction and where any rounding difference goes. U1 certifies against that contract; it must not discover it. Second, a named initial slice: the smallest signed financial stage whose primitives U1 must certify, together with the gaps that the slice avoids. Third, an honest ledger of what is open, where each open item has an owner milestone and a checker that fails if someone later claims it closed without evidence. U0 is a contract milestone. It is not a milestone for native enforcement or evaluator conformance.

## 2. Findings

**Sound**

- `docs/decisions/u0-numeric-profile-decision.md` D1 to D4 are clear. `numeric-profile.json` binds the decision by hash (`decisionSha256`, line 4). Price orientation, the directed DeFiFormal reciprocal (with four test vectors, lines 301–334) and field-element coercion being forbidden (line 21) are all correct and checkable.
- `check_u0_numeric_profile.py` exits 0 with "OK: 17 primitives, 6 open conformance gaps". `tests/test_u0_numeric_profile.py` passes 76 of 76 (I re-ran both). The profile records open gaps plainly and claims no conformance it lacks.
- Row coverage matches D3. The only division sites on the source/5 path are `financial-expression-v1.ts:404`, `financial-lifecycle.ts:1376` (convertNominal), `:1423` (ProRata) and `:1779` (accrual). All four are rows.

**Weak or mis-scoped**

1. **D2's reserve posting is under-specified, not just unimplemented.** The rounding remainder of a division is a fraction of a quantum. `evidence/linear-obligations/cl5` shows it: the interest is 33972602 plus 54/73 of a quantum. A fraction cannot be posted to an integer reserve balance.
   - For `ceil` on an obligation (`accrual-interest`, `repayment-settlement-conversion`), no party pays anything at accrual time. The extra fraction becomes more debt owed to the creditor, so the creditor benefits, not a reserve.
   - D2 ("dust accrues to the protocol reserve … never silently dropped") therefore has no operational meaning yet. `reserveMechanism.status: "absent"` (line 56) makes all six gaps look like one implementation gap. The real gap is a missing decision: what gets posted (the whole-quantum spread between the ceil charged and the floor credited?), to which account, per asset and domain, under whose authority, and whether it counts against `signedIntent.feeCap`.
2. **`prorata-principal-share` (lines 272–284) is mis-classified.** The ProRata split of an exact nominal between principal and accrued conserves value: `dA = n - dP` (`financial-lifecycle.ts:1424`). No value is lost, so nothing should go to a reserve. The only economic effect is the classification: flooring `dP` keeps principal higher, so future interest is higher, which favors the creditor. That row should be `conforms`, with a D2 note that reclassification is not dust. On that reading there are 5 real gaps, not 6.
3. **"Unit dimensions" is only partly covered.** `units` (lines 19–48) lists four widths. The dimension algebra (Amount vs Quantity<Units<…>> vs ScaledAmount vs Price<A,B,S>, and scale propagation) appears only as prose in `priceOrientation.note`, citing `financial-expression-v1.ts:341`. No rule table exists, and the checker verifies none. ROADMAP.md:21 asks for unit dimensions explicitly.
4. **Signedness conflicts with the liability domain.** Obligation state is typed `Quantity<Units<Cash,1>,0>`, which is SInt128 (see the Aeon encoding `r1_repay_law.smt2`, which follows the source types). The profile never says that principal, accrued and outstanding are non-negative. The stage relation has only `liabilities.opening[]` and `liabilities.closing[]` (`judgments.json:66–75`). It has no accrual or discharge leaves, so the product-contract equation "opening plus creation/accrual minus explicit discharge equals closing" (`docs/MORIARTY-PRODUCT-CONTRACT.md:43`) cannot be stated on the leaves. The accrual rounding direction therefore has nowhere to attach.
5. **Each row's role is set at the call site, not by the primitive.** `expression-obligation-division` and `expression-receipt-division` share line 404. The reducer cannot know whether it is computing an obligation or a receipt. Closing these gaps needs the role in the type or effect judgment (the result's flow into a payer-side or payee-side effect). Removing the author's rounding choice at the reducer would not be enough.
6. **The checker is out of proportion to its assurance.** It is 1,973 lines of text and structure checks for a 336-line profile. It computes the DeFiFormal vectors in Python, but it never runs a TypeScript primitive. No conformance claim is exercised against the evaluator. That is correct for "recorded"; it should not grow further.

**The Aeon repay finding (uncommitted package)**

- I reproduced it: `z3 r1_repay_law.smt2` returns `unsat` with the four proposed `where` conjuncts. `RESULTS.txt` `cl3b` returns `sat` without the outstanding guard (`P=0, A=0, n=1, dP=1`). As a statement about **written source guards**, the finding is correct. `loan-lifecycle.mori:118–147` writes only `is_negative(nominal) == false` and `payment > 0`, and its `ensures` clauses are fixture constants (lines 140–146).
- It does **not** show that the evaluator can reach principal = -1. The source/5 lifecycle kernel rejects that case:
  - `financial-lifecycle.ts:1568–1569` rejects `EXCEEDS_OUTSTANDING`.
  - `:1459–1461` rejects `INSUFFICIENT_BALANCE`.
  - `:1433` rejects `ALLOCATION_COMPONENT`.
  - `repayment.ts:1111` has the same outstanding check.
  - `tests/successor-source-repayment.test.mjs:66` has overpayment and balance rejection rows.
  - The native loan kernel relies on `checkedSub` underflow and on fixture-literal asserts (`compact/generated/loan/kernel.compact:43,52`).
- I could not re-run the Node test: the environment's temp filesystem hit ENOSPC. So evaluator rejection is recorded from code and test source, not demonstrated by me.
- The Aeon model also encodes `repayment.ts` AccrualFirst, which the numeric profile says is not the source/5 path (row `prorata-principal-share`, line 273).
- The correct classification: **the load-bearing preconditions exist only in the host kernel and are absent from the source/Core contract and the stage relation.** A signed intention therefore does not bind them, and the enforcement map cannot cite them. That is squarely a U0 finding: it concerns the judgments, the embeddings and the enforcement map. It is not a request for refinement types in U0.
- The package itself is stale against the consolidated roadmap.
  - `ROADMAP-DELTA.md` edits SP01–SP12 sprint files and a "Design decisions before expanded execution" ROADMAP section that no longer exists. ROADMAP.md:5 says the sprint IDs are provenance aliases, not work queues.
  - It gates on `rp01-full`, which is not a U-milestone.
  - Its z3 results sit in `evidence/`, and its own §9 admits it is not reviewed repository evidence.
  - ROADMAP.md:46 and `MORIARTY-CONSOLIDATED-DESIGN.md:95` already adopt the useful part ("obligation/trust reports accompany U0–U2; exact advisory refinement checking follows a supported encoding and evaluator-replayed counterexamples"). The Aeon counterexample has not been replayed on the evaluator. Had it been, the replay would have been rejected, which is the point.

## 3. Proposed work

**T1. Amend the decision: D2a (reserve semantics) and D5 (liability domain).** Size S.

- Output: a new decision file `docs/decisions/u0-numeric-profile-decision-v2.md`, owner-signed. Do not edit v1, because its hash is bound.
- D2a must decide:
  - The sub-quantum remainder is not value and is not posted.
  - For a primitive that both charges and credits (conversion, fee), the reserve receives the whole-quantum spread `charged - credited`, which is 0 or 1 quantum.
  - Accrual `ceil` accrues to the creditor. That is a declared, rationale-bearing override, or else the reserve holds a matching receivable.
  - The reserve account is identified per asset and domain, with an authority to withdraw from it.
  - Whether the spread counts against `signedIntent.feeCap` and `grossDebitCap`. My recommendation: yes, it counts against both.
- D5 must state that principal, accrued and outstanding are non-negative as a domain invariant, and that liability evolution is opening + accrual − discharge = closing.
- Verified by: `check_u0_numeric_profile.py`, extended to hash-bind v2 and to reject a profile whose `remainderBeneficiary` contradicts D2a.
- Exit: v2 is merged and the checker binds it.
- Depends on: an owner decision.

**T2. Profile v2: role-typed primitives, a dimension table, reclassification.** Size M.

- Output: `numeric-profile.json` with `schemaVersion` `moriarty-u0-numeric-profile/2`, in a new deliverable directory, plus the checker update.
- Add a `dimensions[]` rule table: operator, operand types, result type and scale rule, citing `financial-expression-v1.ts:341` and each arithmeticType case.
- Add `domainInvariants[]`, taken from D5.
- Reclassify `prorata-principal-share` as conforms.
- Split each open gap into `directionGap` (author-selectable) and `reserveGap` (D2a posting).
- Add a `signedness` column so SInt128 liability fields are flagged against D5.
- Verified by: the checker, plus new tests that make it fail on a missing dimension row for any `arithmeticType` case.
- Exit: every `arithmeticType` case has a row; the gap counts are recomputed mechanically; the OK line prints direction gaps and reserve gaps separately.
- Depends on: T1.

**T3. Executable conformance vectors (recorded as observations, not a closure).** Size M.

- Output: `deliverables/u0-numeric-conformance-<date>/vectors.json` with inputs, the exact rational result, the expected directed result and the expected spread for each rounding primitive, including boundary cases (zero remainder, UINT128_MAX − 1 ceil overflow, negative dividend at :404). Add `observed.json` produced by running the TypeScript evaluator.
- Verified by: `scripts/check_u0_numeric_vectors.py`. It recomputes the expected values independently in Python, compares them with the observed values, and reports each gap row as "observed divergent under author choice X".
- Exit: every one of the 17 rows has at least 3 vectors; every expected value is independently recomputed; each open gap is shown by a vector in which an author selection violates D2.
- Why U0: this turns "recorded" gaps into **demonstrated** gaps, which U1 needs as regression oracles. It closes nothing.
- Depends on: T2. If the environment cannot run Node (see the ENOSPC above), record the blocker.

**T4. Kernel-precondition lift for repay (the correct home for the Aeon finding).** Size S to M.

- Output:
  - New rows in `judgments.json` and `enforcement-map.json` for `repay.nominal ≤ outstanding`, `debit ≤ balance`, `debit ≤ allowance` and liability non-negativity. Each row has status `host-only`, citing `financial-lifecycle.ts:1568`, `:1461`, `:1475` and `:1433`.
  - A matching `absent` status in `source-core-embeddings.json`.
  - A retained **evaluator replay** of the Aeon counterexample, `deliverables/u0-numeric-conformance-<date>/repay-replay.json`, showing the rejection code.
- Verified by: the existing `check_u0_enforcement_map.py` and `check_u0_stage_schema.py` (both need new leaf support), plus the replay compared against expected `EXCEEDS_OUTSTANDING`.
- Exit: the enforcement map shows these as host-only, not native. The replay is retained. The Aeon package README is corrected to say the evaluator rejects, and the gap is at source/Core and native level.
- Depends on: the stage-schema owner (L1/L2) adding `liabilities.accrual[]` and `liabilities.discharge[]` leaves.

**T5. Initial-slice declaration.** Size S.

- Output: `deliverables/u0-semantic-contract-.../initial-slice.json` naming the U1/U2 first stage and exactly the primitive rows it uses.
- Proposal: a **signed funded payment or bounded swap stage** using only `checked-add-u128`, `checked-sub-u128`, `expression-checked-construct-amount` and comparison.
- Price and limit constraints are written as **cross-multiplied inequalities in UInt256** (for example `spent·20 ≤ received·11`) instead of division. They are exact and need no rounding row.
- This slice avoids all rounding gaps and the reserve question, and it is the natural prefix of the U3 discriminator. U3 caps "at most 11 A including at most 1 A fees" and "at least 20 B" are adds, subtracts and compares; its partial fills can use cross-multiplication.
- Verified by: a check that every primitive in the slice has `conformance: conforms`, and that no slice primitive has `authorSelectable: true`.
- Exit: the file exists, the checker passes, and U1's "first exact arithmetic primitive" is named as `checked-add-u128`.
- Depends on: T2.

**T6. Aeon package disposition.** Size S.

- Output: none in U0 beyond T4. Record in the unified proposal that the package is re-scoped:
  - The advisory analyser (Aeon tasks §1) belongs in the U0–U2 "obligation and trust reports" line (ROADMAP.md:46).
  - Profile `/6`, SMT discharge and linear types are post-U2 authoring work, gated on a supported encoding plus evaluator replay.
  - Its sprint-file deltas (`ROADMAP-DELTA.md` §4–§5) are rejected as targeting superseded structure.
- Exit: the owner decides whether to commit it, and if so under re-scoped text.

## 4. Exit redefinition

Yes. Split the gate into two statuses and make the checker print both.

- **U0-FROZEN (contract).** Profile v2 is bound to decision v2; dimensions, domain invariants, reserve semantics and fee-cap treatment are decided; the initial slice is declared; every open item has an owner milestone field (U1, U2, U3 or U6) and a named blocker; host-only preconditions are recorded in the enforcement map; the target pins are re-verified or explicitly marked unresolved (L-other). This is what unblocks U1 and U2.
- **Demonstrated inside U0:** only the T3 vectors and the T4 evaluator replay. Both are observations of current behavior, not closures.
- **Deferred with named owners:**
  - The 5 direction gaps go to U2. The source frontend must derive direction from the effect role, or reject a contradicting selection.
  - Reserve posting goes to U2 for conversion and fee spreads, and to U3 when the first stage that settles a spread is exercised.
  - Accrual-rounding certification goes to U1, only when a slice uses accrual (the proposed initial slice does not).
  - The native certificate for each slice primitive goes to U1.
  - Native enforcement of the lifted repay preconditions goes to U2, for the loan program only.
  - Refinement discharge is post-U2 or U6.

"Capabilities open" should never block U0-FROZEN, provided each open capability has an owner and a failing-if-claimed check. U0 should not wait for 84 of 84 leaves to be enforced; that is U2's job.

## 5. Risks and things NOT to do

- Do not add refinement types, a `/6` grammar or an SMT solver on the U0 path. That would add a trusted computing base (TCB) member before any native path exists. The repay issue is solved at U0 by recording and lifting the preconditions, not by discharging proofs.
- Do not describe the Aeon result as "repay is unsound" in any gate text. The evaluator rejects the counterexample. The claim is "the source/Core contract omits the kernel-enforced preconditions".
- Do not implement a reserve account before D2a. An implementation would freeze an ill-defined policy (posting fractional remainders, or counting accrual dust twice).
- Do not choose a first slice that includes division. It would drag all open rounding and reserve gaps into U1.
- Do not extend `check_u0_numeric_profile.py` with more prose-shape checks. New assurance should come from executed vectors.
- Do not edit decision v1 in place. Its SHA is bound.
- Risk: the rounding spread counting against `feeCap` could make U3's "at most 1 A fee" tight. Decide it now, not during U3.

## 6. Disagreements

1. **With D2** (`u0-numeric-profile-decision.md`): "dust accrues to the protocol reserve" cannot be implemented as written for sub-quantum remainders, and it conflicts with ceil accrual, which benefits the creditor. It needs D2a.
2. **With `numeric-profile.json` `prorata-principal-share`**: it is not a gap. Nothing is lost, so there is nothing to post. The EXIT-GATE figure of 6 gaps overstates the direction and reserve problem by one row and understates the specification gap (items 1, 3 and 4 in section 2).
3. **With EXIT-GATE.md line 9**: marking the numeric-profile evidence as "recorded: yes" is fair. But the ROADMAP item "unit dimensions" is not recorded; only widths are.
4. **With the Aeon package**: its central claim is framed as an evaluator soundness bug but is a contract-omission finding. Its roadmap delta targets superseded sprint structure. Its evidence has not been replayed on the evaluator, contrary to ROADMAP.md:46.
5. **With the Immediate delivery brief (ROADMAP.md:38)**: "connect the smallest useful signed financial stage" is not named anywhere. U0 must name it (T5). Otherwise U1's "every primitive needed by the initial slice" has no denominator.

## 7. Top three recommendations (ranked)

1. **Declare a division-free initial slice now (T5)**, with limit ratios written as cross-multiplied UInt256 comparisons. This lets U1 start on `checked-add/sub-u128` with zero open numeric gaps, and it is a strict prefix of the U3 11A/1A/20B discriminator.
2. **Decide D2a and D5 before any implementation (T1, T2).** They cover: what reserve posting means for sub-quantum remainders, whether the spread counts against the fee cap, liability non-negativity, and accrual/discharge liability leaves. Then split the U0 gate into U0-FROZEN (the contract, which unblocks U1 and U2) and capabilities with named owner milestones.
3. **Take the Aeon repay finding into U0 as a precondition lift, not as refinement types (T4).** Record `nominal ≤ outstanding` and `debit ≤ balance/allowance` as host-only in the enforcement map, retain an evaluator replay of the counterexample (expected `EXCEEDS_OUTSTANDING`), and defer `/6`, SMT discharge and linear types to post-U2 authoring work.
