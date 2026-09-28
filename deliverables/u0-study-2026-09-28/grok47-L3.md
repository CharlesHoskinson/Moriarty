I'll study the numeric profile, the U0 exit path, and the Aeon repay finding as a read-only review, then write the L3 proposal from those sources.## L3 proposal — numeric profile and the exit from U0

### 1. Goal restatement

U0 freezes the arithmetic contract that U1 certifies and U2 signs. It does not make `financial-lifecycle.ts` conform to D2, and it does not create a reserve.

The freeze has five parts: canonical price orientation (already D1); the dimension map the type checker already implements; rounding direction by economic role, with remainder classes that are not all "post a token to a reserve"; a named initial slice, **N0**, and the source laws a signed stage must carry; and a numeric reading of the U3 sentence in `ROADMAP.md:38`, so that sentence does not reopen D2 later.

`docs/MORIARTY-CONSOLIDATED-DESIGN.md:52` already assigns the next step: U0 freezes the profile, U1 certificates bind it. The U1 list is N0's exact operations, then one directed division before U3. It is not all 17 rows.

Sequence: U0 freezes N0, the dimensions, the remainder classes, and the U3 reading. U1 certifies N0 on the host, and natively only once a compatible pin tuple exists. U2 signs a program whose text states the N0 laws. U1 then certifies one role-directed division. U3 is the first program that spends at most 11 A, including at most 1 A fees, to receive at least 20 B.

### 2. Findings

**What holds.** D1–D4 are the policy source, and `numeric-profile.json` hash-binds them (`check_u0_exit_gate.py:189-191`). Re-ran `scripts/check_u0_numeric_profile.py`: `OK: 17 primitives, 6 open conformance gaps`. Re-ran `tests/test_u0_numeric_profile.py`: 76 passed. D1 matches `financial-expression-v1.ts:341`: an amount of the quote times `Price<base, quote, scale>` yields a scaled amount of the base. Price assets must differ (`financial-expression-types-v1.ts:92`). The four reciprocal vectors are evaluated by `base_per_quote_mantissa` (`check_u0_numeric_profile.py:1256-1281`). That is the one numeric fact U0 demonstrates. D4's field-element ban is recorded. The profile is explicit that the reserve is absent (`numeric-profile.json:55-58`), and the decision says U0 does not change the implementation (line 22). ProRata really does keep the truncated unit inside the payment: `dP = product / total`, `dA = n - dP` (`financial-lifecycle.ts:1423-1424`).

**The six gaps are not one missing reserve.** They are `accrual-interest`, `expression-obligation-division`, `expression-receipt-division`, `origination-settlement-conversion`, `prorata-principal-share`, and `repayment-settlement-conversion`. The schema forces every non-exact `remainderBeneficiary` to `protocol-reserve` (`numeric-profile.schema.json:295-296`). `conformance_reasons` (`check_u0_numeric_profile.py:1545-1546`) then opens a gap whenever the reserve is absent, including when the fixed direction already matches D2. `prorata-principal-share` is open only for that reason (`numeric-profile.json:273-283`). Posting that unit to a reserve makes `dP + dA < n` and drops the conservation result below. Ceil accrual is the other shape: `applyAccrue` divides toward zero and adds 1 when `rounding == 'ceil'` and the remainder is nonzero (`financial-lifecycle.ts:1779-1787`). The extra unit is the liability. Sub-unit price dust is a third shape. Amount 1, mantissa 3, scale 1, divisor 10: quotient 0, remainder 3, received base units 0. That remainder is not an atomic unit. D2's "dust accrues to the protocol reserve" (decision line 18) has no carrier for it, and the reciprocal formula forbids a second rounding (`numeric-profile.json:304`).

**One stored rounding mode cannot be both conversion rows.** `origination-settlement-conversion` (floor) and `repayment-settlement-conversion` (ceil) both cite `convertNominal` at `financial-lifecycle.ts:1376`. `applyOriginate` (`:1669`) and `applyRepay` (`:1585`) pass the same `conversion.rounding`. `ROLE_EVIDENCE` (`check_u0_numeric_profile.py:289-303`) requires both rows and never asks whether one field can be floor and ceil. Mantissa 1, scale 1: floor yields 0, then `DUST` at `:1674`; ceil yields 1. The canonical example avoids this with `rounding: "none"` (`loan-lifecycle.mori:76`). That exact branch (`financial-lifecycle.ts:1378-1382`) has no profile row.

**The slice U1 must certify is missing from the 17.** `loan-lifecycle.mori:76-77` is mantissa 1, scale 0, rounding `none`, allocation `AccrualFirst`. AccrualFirst is min and subtraction (`financial-lifecycle.ts:1405-1407`). D3 (decision lines 24-26) said the profile covers the primitives the successor actually implements. The checker implements that as division sites and `numericFits` sites, so the exact financial kernel is absent and expression constructors the loan does not use fill the inventory. The example's accrual selects `floor` (`loan-lifecycle.mori:81`). D2 requires ceil for a liability increase. That gap is real, and it is outside the smallest stage.

**Unit dimensions are implemented and not recorded.** The roadmap and design line 52 ask for them. The profile records four widths. Source rules:

- `numericFits` (`financial-expression-types-v1.ts:152-157`): Quantity, Rate, and SInt128 are signed 128. Amount and the default numeric tags are unsigned 128, except UInt256, AmountProduct, and ScaledAmount (256) and UInt64 (64).
- Price mantissa is UInt128. Quantity mantissa is SInt128 (`financial-expression-v1.ts:328-329`).
- Price, Quantity, and Rate scales are integers in 0..18 (`financial-expression-types-v1.ts:94-95`). Checker `MAX_SCALE = 77` (`check_u0_numeric_profile.py:101-104`) is a UInt256 width fact, wider than the language. The four vectors use scales inside 0..18.
- `unitDomain` (`financial-expression-types-v1.ts:67-75`): at most 8 units, nonzero powers in [-128, 127], names strictly increasing.
- A scale division must divide by the literal `10^scale` (`financial-expression-v1.ts:347`).

D2's ceil and floor are stated for amounts owed and amounts received. The expression reducer normalizes a negative remainder before the ceil step (`financial-expression-v1.ts:404`). `applyAccrue` and `convertNominal` do not. On a negative dividend their "ceil" is toward-zero plus one, which is a different function. N0 stays non-negative, where the two agree.

**The repay result is a missing source law.** Re-ran `evidence/verification-discharge/probe.smt2` with z3 4.13.3. It matches `counterexample-repay.txt`: conservation is unsat; `principal_1 = -1` from principal 2, accrued `2^128-4`, nominal `2^128-1`. That model does not print borrower balance `-1`. Its lender-credit goal has borrower balance 0 and nominal `2^128-1`, so the borrower balance under those equations is `1-2^128`. A smaller witness of the same equations: principal 2, accrued 0, nominal 3, borrower balance 2. Both results are `-1`, and nominal exceeds outstanding. Checked by arithmetic, not returned by that z3 model.

The encoding uses only `loan-lifecycle.mori:119-121` (`is_negative(nominal) == false`, `payment > 0`) and AccrualFirst as unbounded subtraction. It omits the kernel rejects at `financial-lifecycle.ts:1568` (`EXCEEDS_OUTSTANDING`), `:1426` (`ALLOCATION_COMPONENT`), and `:1459` (`INSUFFICIENT_BALANCE`). The same checks are in `repayment.ts:1110`, `:968`, and `:1002`. `requirements/refinement-types.md:805` already says `r1_repay_law.smt2` is not a proof about the TypeScript runtime. This review did not execute the evaluator. `dec` (`financial-lifecycle.ts:364-365`) would stringify a negative bigint if those checks were skipped. Quantity's domain would accept `"-1"` (`numericFits`, line 154). Amount's would not.

`r1_repay_law.smt2` re-ran unsat. `r2_totality.smt2` re-ran sat: a lender credit can leave UInt128 without a supply bound. `r3_supply_invariant.smt2` re-ran unsat. The ablation in `refinement-types.md:782-789` says the two written guards are not load-bearing, and that nominal ≤ outstanding, nominal ≤ borrower balance, principal ≥ 0, and the supply bound are. `r4_ablate.sh` was not re-run. That ablation is recorded.

`probe.smt2` types principal as UInt128. Source Quantity is SInt128. `principal = -1` fails the probe's unsigned goal and sits inside the language's quantity domain. The missing fact is a financial non-negativity law.

**The exit bit measures the wrong thing.** The numeric capability flag is `conformance_gap == 0 and reserve_status == "present"` (`check_u0_exit_gate.py:228`). The receipt prints `Evidence RECORDED` as the constant `yes` (`:262`). The profile note (`numeric-profile.json:58`) says a present reserve is a declaration citation, and the checker does not decide that a division posts there. A declared unused type would clear all six gaps, including ProRata, and the receipt would call the numeric capability closed.

The Aeon package is specified-only (`proposal.md:61-62`). Its `ROADMAP-DELTA.md` blocks `/6` on `rp01-full` and says the admissible piece is SP01 rows that name the laws. That scheduling is right. The laws belong in the U0 contract. The grammar, the solver loop, linear multiplicity, and refinement-directed lowering do not.

`target-pins.json` has `compatibleTupleEstablished: false`. A native N0 certificate waits on that tuple. A host certificate against `financial-lifecycle.ts` does not. Intent fields the U3 sentence needs are already judgment fields: `grossDebitCap`, `feeCap`, `minNetOutcome` (`judgments.json:37-40`). The enforcement map enforces 0 of 84. U0 binds their numeric reading. U3 demonstrates them.

### 3. Proposed work

**T1. Remainder classes, owner decision D5. S. Blocks T2.** Addendum to `docs/decisions/u0-numeric-profile-decision.md`. Leave D1–D4 values alone. Classes: `none` (exact, no remainder; N0 lives here); `conserved-split` (the unit stays in the other component; ProRata; `dP + dA = n`; a reserve posting fails conformance); `charged-increment` (ceil adds one atomic unit to a liability or charge; accrual ceil; no separate posting); `sub-unit-residual` (scale-division remainder that is not an integer unit of the received asset; not a reserve credit; not rounded again; carrier is the scaled residue, or `DUST` / `INEXACT_CONVERSION` when the stage requires a whole unit); `protocol-reserve` only for a whole atomic unit of an identified asset that would otherwise leave the conservation equation.

Checker: `scripts/check_u0_numeric_profile.py` must stop opening every floor/ceil row because `reserveMechanism.status` is absent. A `protocol-reserve` row stays open while the reserve is absent. `tests/test_u0_numeric_profile.py:48-50` currently locks the sentence "missing protocol-reserve posting"; update it. Exit: each of the six rows has one class, and ProRata is not `protocol-reserve`. G5 still matches the decision hash. Until the owner signs, record the conflict as an unresolved premise. The decision file is the only policy source (lines 3).

**T2. Name the exact mode and the shared rounding field. S. Depends on T1.** Add `exact-conversion-none`, citing `financial-lifecycle.ts:1378-1382`, role exact, direction none. Add `accrual-first-split`, citing `:1405-1407`, exact min and subtraction, separate from `prorata-principal-share`. Give the two existing `convertNominal` rows a shared-control field naming `Conversion.rounding`. Checker: two rows that share file, symbol, and line and require both floor and ceil must carry that field. The exact row must cite the `none` branch. The current line test accepts a `none` row only when the line has `numericFits` or `UINT*_MAX`; line 1378 has neither, so the checker needs a "reject nonzero remainder" pattern. Author-selectable floor or ceil on the other rows stays an open gap. Exit: the profile contains the mode `loan-lifecycle.mori:76` selects, and it states the mutual exclusion.

**T3. Dimension map. S. Independent.** A `dimensions` object in `numeric-profile.json`, each sort citing `numericFits`, `unitDomain`, or `arithmeticType`, covering the domains in section 2. Checker re-reads those predicates. Admitted price scale is 0..18. Keep 77 as a UInt256 numerator bound, not an admitted scale. Exit: every dimension row checks against a cited predicate. This is the roadmap's unit-dimension item. The widths list does not discharge it.

**T4. Freeze N0 and laws L1–L8. M. Depends on T2 and T3.** New `deliverables/u0-semantic-contract-2026-09-23/numeric-slice-n0.json`. N0 is one asset, one obligation, AccrualFirst, mantissa 1, scale 0, rounding none. No accrual, no ProRata, no price, no reserve.

- L1. `outstanding = principal + accrued`. `probe.smt2` asserts it. The source does not state it.
- L2. principal, accrued, and outstanding are ≥ 0. Stricter than Quantity. A consequence of L1, L3, and `ALLOCATION_COMPONENT`, not of `numericFits`.
- L3. nominal > 0 and nominal ≤ outstanding. The source has the first half (`loan-lifecycle.mori:121`). Kernel: `EXCEEDS_OUTSTANDING` at `:1568`.
- L4. settlement ≤ payer balance and ≤ remaining allowance. Kernel: `subU128` at `:1459` and `:1473`.
- L5. phase, paid, receiver credit, and allowance spent fit their unsigned width. Kernel: `addU128`. The probe shows these goals sat under the written guards. `r2` shows the lender credit needs a supply bound or a checked add.
- L6. settlement = nominal under N0's conversion. No dust.
- L7. AccrualFirst: `dA = min(nominal, accrued)`, `dP = nominal − dA`, parts sum to nominal, and each part fits the current component.
- L8. The signed source carries L1–L7. The fixture `ensures` at `loan-lifecycle.mori:140-146` are not these laws.

Pin the reproduced probe transcript, labelled source-gap. Kernel rejection was read from source and not executed. Quantity's domain accepts principal `-1`. Checker: `scripts/check_u0_numeric_slice.py` checks primitive ids, rounding none, remainder class `none`, and a source line plus a kernel line per law. Cache the z3 transcript so the checker does not require z3. Exit: U1 can name N0 as the initial slice. U0 does not edit the `.mori` file.

**T5. Freeze the U3 reading. S. Depends on T1 and T3.** A section `N1-reading` in the same file, not an implementation. Sentence (`ROADMAP.md:38`): spend at most 11 A, including at most 1 A fees, to receive at least 20 B. Reading: A and B are distinct assets; amounts are unsigned 128 smallest units. 11 is `grossDebitCap`, 1 is `feeCap`, 20 is `minNetOutcome` (`judgments.json:37-40`). A source price is canonical: `Price<B, A, S>` is units of B per unit of A. A quote-per-base observation uses the reciprocal once, role-directed, scale in 0..18. Received B floors. A computed fee ceils. A literal fee does not round. Successful completion requires the inequalities. A committed partial keeps a non-negative residual in whole units and does not re-round the price. The division's remainder class comes from T1. Partial fill, escrow, documents, recovery, and the hostile tests at `ROADMAP.md:40` stay U3. Exit: U3 has a numeric interpretation and U0 has not started the escrow.

**T6. Point U1 and U2 at N0. S. Depends on T4.** U1's predicate list: `addU128`, `subU128`, and L3–L5, one valid witness and one adversarial witness each. The small witness (principal 2, nominal 3) is the adversarial source witness; the kernel must reject it. Native bytes wait on a compatible pin tuple. A failed pin stays blocked, as the U1 cell already says. U2: one newly authored same-asset program whose source states L1–L8, plus one contrasting program still inside N0 (another party or a second exact obligation, not the two-asset sentence). Host evaluation can proceed while pins are open. Native verification cannot. Do not use the fixture `ensures` as the signed laws. Do not edit `spec/examples/loan.mori`; the Aeon proposal records those bytes as the Preview digest. T6's U0 exit is the named predicate list and the U2 source obligation. The certificate is U1's exit.

### 4. Exit redefinition

The numeric row should have two real bits.

**Frozen** when T1–T5 are in the deliverable and their checkers pass. The six gaps may remain. The reserve may remain absent.

**Demonstrated in U0:** the four reciprocal vectors, recomputed by the checker this review re-ran, and the classified probe transcript, re-ran here on z3 4.13.3 against the archived model. No other numeric fact was demonstrated. The lifecycle evaluator was not run. `bench.sh` was not run.

| Work | Owner |
| --- | --- |
| N0 host certificate, valid and adversarial witnesses | U1 |
| Native certificate of those checks | U1, after a compatible pin tuple |
| Signed source stating L1–L8, plus a contrasting N0 program | U2 |
| One certified role-directed division | U1 after N0, before U3 acceptance |
| The 11 A / 1 A / 20 B program, partial fill, recovery | U3 |
| Floor accrual, ProRata as a library rule, ACTUS overrides | U6 |
| `/6` grammar, SMT discharge, linear obligations, lowering tiers | SP02 after `rp01-full`, per the Aeon delta |

Replace `check_u0_exit_gate.py:228`. The numeric close bit becomes: N0 frozen, dimension map present, every rounding row classified, probe classified as a source gap. Keep the six gaps in the counts. Stop printing `Evidence RECORDED` as a constant, or drop that column.

### 5. Risks and things not to do

- A `ProtocolReserve` declaration would flip `reserveMechanism.status` and, with the current checker and gate, clear every floor/ceil gap without a posting.
- Posting ProRata's truncated unit to a reserve breaks `dP + dA = n`.
- Rounding sub-unit dust up into a token violates the reciprocal formula.
- Editing `financial-lifecycle.ts` inside U0 contradicts decision line 22.
- Rewriting `loan-lifecycle.mori` or `spec/examples/loan.mori` inside U0. Record the laws beside them.
- Adopting `openspec/changes/aeon-refinement-integration` as the U0 change. Take L1–L8 and the probe classification.
- Certifying all 17 primitives in U1. Two conversion rows contradict each other. ProRata and floor accrual are outside N0.
- Treating the Aeon bitvector timing table as a result of this review.
- Describing the archived probe model as one runtime trace that sets the borrower balance to `-1`. It sets principal to `-1` and, on another goal, overflows a lender credit. The joint `-1`/`-1` case is a smaller witness. The kernel source rejects it. That rejection was not executed.
- Reading a green `check_u0_numeric_profile.py` as semantic conformance. This review ran it. It checks citations, constructor windows, and policy-shaped gap reasons. It passes while one rounding field is required to be both floor and ceil.

### 6. Disagreements

The exit gate's numeric close condition (`check_u0_exit_gate.py:228`) is the wrong close. Design line 52 puts certificates in U1. The roadmap U0 cell asks for the policy in the profile. The capability column pulls implementation into U0, and the profile note says the reserve test does not show a posting.

D2's single beneficiary, as the schema encodes it, covers a whole atomic unit that would otherwise disappear. It does not cover a conserved split, a ceil increment, or a sub-unit residue. `numeric-profile.schema.json:295-296` makes any other beneficiary unrepresentable.

D3's inventory dropped AccrualFirst and rounding `none`, which are the example, and split one expression site into an obligation row and a receipt row. The checker comment at `check_u0_numeric_profile.py:270-271` already says the constructor names do not prove the role.

`proposal.md:11-16` is a real source-guard failure, and this review reproduced the principal model. The wording "principal and borrower balance to `-1`" compresses two goals and skips the kernel checks. Putting `/6` on the U0 path would block the freeze on `rp01-full`. The delta's SP01.3 bullet is the proportionate piece.

Waiting until all 17 rows are clean before naming the smallest stage inverts `ROADMAP.md:38`. That paragraph names the 11/1/20 sentence as the next discriminator. N0 is the stage that comes first.

### 7. Top three

1. Freeze remainder classes and the one-field conversion conflict (T1, T2) before anyone adds a reserve or edits `convertNominal`.
2. Name N0 and write L1–L8 as source obligations, with the probe result classified as a source gap (T4). That is where the Aeon finding belongs. The `/6` package stays specified-only.
3. Change the numeric exit bit so U0 closes on that freeze. Give the N0 certificate to U1, the signed laws to U2, and the 11 A / 1 A / 20 B program to U3 under the T5 reading.
