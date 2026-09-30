# Independent horizon v2 audit 5 — correspondence and validation

**Scoped verdict: reject exact-byte concurrence pending H1–H3 disposition.** Concur with the repaired separation of local S0, specified beta records and specified/open horizon contracts; with coverage of all eight family lifecycles; and with the architectural choices listed below. The remaining findings concern contradictions or underspecified refinement rules inside the proposed contracts. This is a design review, not execution, financial safety or proof approval.

## Candidate and reviewer identity

- Requested routing: GPT-6.1 Sol, medium effort. No independent returned-identity/effective-effort provider receipt is available to this child; the requested identity is not provider attestation.
- `PROGRAMMER-MOCKUP.md` inspected SHA-256: `89ab5354e9ff0de7d0793318a8c9df93d617b71bfac13da120289402dc61dc65`.
- `FULL-LANGUAGE-HORIZON.md` inspected SHA-256: `7d2673703dfd271dd641b625c799dee3f536803827dacb7d66bc938cc82ec26f`.
- Both match the parent's exact-byte assignment. Entire candidate documents and original programmer-facing mockup requirements were read. No peer report was read; references to other audits inside the candidate were not followed.
- Checked-out Moriarty development skill remains applied; guarded status was refreshed and reported no pending transactions. No dispatch, code edit or commit was performed. This report is the only file written for this follow-up.

## Concurrences and scope

| Requirement / consequential choice | Vote and reason |
| --- | --- |
| Local/specification/open separation | Concur. The mockup expressly limits existing local execution to Source/6–Core/5 S0 and labels all new notation proposed. Horizon structs/transitions/functions/matches are unsupported beta syntax. |
| Typed authority/evidence instead of generic calls | Concur architecturally. Grant roles, expiry, revocation, threshold keys, evidence policies and the eight kernel methods are visible; receipts are never treated as proof. M1/M2 require local clarification. |
| Finite typed holes and signed refinement | Concur. Recipients/assets/programs/policy remain fixed; holes cannot rewrite authority or the outcome relation. General total elaboration/certified arithmetic remain open. |
| Typed complete footprints | Concur with equality of authored and derived read/write sets, common controls and unchanged framing. M3 identifies a concrete ambiguous cash/custody write. |
| Stage failure and persistent episode duty | Concur. First failure publishes no new financial effect/post-head; accepted earlier stages remain visible; Unknown retains reservations and duties. No global rollback or timeout-as-nonreceipt is introduced. |
| Eight-family horizon | Concur on source-level coverage: swap/LP mint/redeem; loan originate/roll/pay/liquidate/default residue; stable mint/burn/shutdown/emergency; option premium/fix/exercise/pay; oracle stale/disputed/aggregation assumptions; governance queue/veto/enact; bridge claim/reconcile/exclusion/refund; staking reward/slash/unbond/withdraw. Coverage is not evidence that any of these execute. |
| Composed signed promise | Concur with fixed Pair, recipient, programs, separate asset budgets, no reset on AttemptId, and three explicitly distinct terminal relations. H2/H3 prevent full contract concurrence. |
| Formal obligations | Concur. K/Quint observation/continuation, origin-preserving elaboration, exact bytes and program/policy/scale/agreement binding, authenticated same-head cells, arithmetic certificates and atomic ledger consumption remain separate open predicates. |

## High findings

### H1 — The staking witness and the signed slash priority describe different loss relations

**Repository/document observation:** §12 `Staking.Slash` says losses first reduce subordinate withdrawal claims, then junior free claims. YieldPromise fixes priority `[SeniorRestakingSlash,PendingWithdrawal,JuniorFreeShares]`. The numeric example and closing explanation instead apply proportional loss: a 110 USD pending quote loses 10%, becomes 99 USD, and the 100-share withdrawal pays 99.

**Exact arithmetic observation, not language execution:** With backing 1210 USD, supply1100 and a100-share claim, the quote is110 USD. A121 USD loss allocated to pending claims before junior free claims exhausts the sole110 USD pending claim and applies the remaining11 USD to free claims. A proportional10% allocation yields99 USD. These outputs cannot be the same signed relation.

**Repair:** Select one policy. If proportional, define the slash liability rank relative to other protocols separately from proportional allocation among the vault's holders, and remove the claim-first equation. If claim-first, repair the claim/supply/backing witness and explain what happens to the100 reserved shares with zero remaining redemption value. Preserve duties until the selected discharge relation holds.

**Discriminator:** Independent literal pre-state with one pending claim110 USD, free backing1100 USD and loss121 USD; then two pending claims of distinct ranks, a mature-but-uncommitted claim and an already paid claim. Specify exact loss shares, post owed, backing, supply, consumed cursor and duty. The candidate cannot mark the full slash/priority relation settled while these two formulas coexist.

### H2 — Terminal net semantics are not consistent across the displayed lifecycle bounds

**Document observation:** §2 defines cumulative gross without refund netting and actual terminal net receipt after every deduction; it also requires the selected terminal branch's net floor. LoanPromise fixes Alice USD net500, but its own funded30 USD repayment leaves470 USD net cash receipt. CoinPromise fixes MUSD net100, but Mint100 followed by Redeem/Burn100 leaves zero MUSD net receipt. Its GOLD deposit0.200 and minimum redemption0.190 also illustrate the difference between returned amount and episode profit. The composed Promise has a generic GOLD_F budget net0.900 while OriginRefund and DestinationReturn deliberately terminate without that GOLD_F receipt.

**Inference:** The author appears to intend branch-specific receipt constraints, potentially attached to milestones, rather than one lifetime net-cash floor on every asset. That interpretation is not expressed in the generic Bounds type or an explicit precedence rule. A verifier applying generic net fields can reject the intended lifecycle/recovery; one ignoring them can weaken a signed term. Successful claim arithmetic cannot decide this interpretation.

**Repair:** Separate cumulative `gross/fees/work` prefix invariants from explicitly keyed terminal-branch or milestone receipt relations. Define whether net is matched credited amount less branch-specific deductions, final spendable custody, or episode cash delta. Define recipients for each asset entry and forbid initial unrelated balances satisfying a receipt obligation. Fix LoanPromise/CoinPromise accordingly. For BridgeThenSwap, make Delivered's GOLD floor inapplicable to the other two signed branches by an explicit typed branch rule, while their own USD/WUSD floors apply and source fee remains a retained loss.

**Discriminator:** Evaluate the published loan, Mint→Redeem, OriginRefund and DestinationReturn literal traces under the selected definition. Deliver the required credited amount into an account with preexisting assets; then debit fees from that credited amount. Confirm that old custody cannot fulfill delivery, source fees do not vanish, and choosing a recovery branch changes only the signed branch's receipt obligations—not gross counters or previous effects.

### H3 — BridgeThenSwap needs a source-fixed continuation-head refinement rule

**Document observation:** The family `header` abbreviation in §4 explicitly expands to `PreviousAcceptedHead` for continuations. The composed §13 intent bypasses that header and manually fixes `pre_heads={Preview:p0,Foreign:f0}`. Claim and Swap share Foreign; Escrow and OriginRefund share Preview. No continuation-head term appears in the composed SignedIntent or episode links, although common controls require nonce/head consumption and the explanatory paragraph specifies stage-specific nonces.

**Inference:** After Claim commits f0→f1, Swap cannot literally satisfy f0 as its current signed pre-head. Likewise source refund follows an accepted escrow at a successor of p0. A solver-selected current head must not silently replace the source-fixed one. The journal/Composed.Refine may supply the intended history relation, but its stated equations only append facts/counters/branch receipts; they do not define that head derivation. The separate nonce explanation does not bind these heads.

**Repair:** Put a signed per-stage initial/continuation-head schema in Promise, or expressly instantiate the §4 PreviousAcceptedHead rule in the composed episode with exact domain, Pair, predecessor-stage/accepted fact and qualified head binding. Reconciled prior Claim must use the same original economic effect/head, not any current unrelated head. Prove each successful stage consumes its resolved current head atomically.

**Discriminator:** Escrow p0→p1, Claim f0→f1, Swap f1→f2; qualified OriginRefund from p1; a concurrent foreign head change; forged unrelated predecessor; Unknown Claim later finalized; and a retry with a fresh AttemptId but the same economic StageId. Specify whether unrelated ledger progress requires new source authorization or is admitted by an explicit signed relation. Do not accept whichever head is supplied by completion.

## Medium findings

### M1 — Kernel scope sugar drops the typed claim/attempt context at its documented expansion

§3 requires `Kernel.observe(scope: KernelScope<D,P>,selector,evidence)`; KernelScope carries intent, verified grant, claim and AttemptId. §9 says its exact expansion is `Kernel.observe(scope: ReadPromise.Read,...)`, but ReadPromise.Read is a grant, not that scope. Similar `kernel reconcile/recover(authority: grant,...)` clauses need explicit scope construction. This matters for the same-intent/claim/attempt binding, not merely syntax styling.

Define closed `scopeFor(stage,intent,grant,claim,attempt)` expansion and identify read-only query identity versus ledger economic claim. Test two queries/attempts sharing a grant: receipts and selected observations cannot be exchanged unless their exact binding relation permits it. No caller grant record may manufacture VerifiedGrant.

### M2 — The composed foreign signer set and recovery-expiry policy need a precise authority relation

Promise declares Owner-only signer sets, while ClaimGrant/RecoverGrant require Owner+Reserve. Additional resource authority can legitimately be conjunctive; it must be explicit whether Reserve signs the parent envelope, the delegated grant, a mint/recovery capability or the native payload. “Grant narrows parent” alone does not define that additional signature meaning. Foreign Reserve key binding is also implicit in Examples, which declares OwnerForeignKey but no concrete Reserve foreign key.

Spell out these conjunctive signatures and domain/key-epoch binding. Also state what happens when late nonreceipt/recovery proof arrives after RecoverGrant's Preview expiry300 or ReturnGrant's Foreign expiry400: duties must remain pending, and any renewed recovery authority must preserve original claims, terms and budgets. This is an open authorization/availability choice, not an excuse to extend expiry through completion.

### M3 — Liquidation's Debit account effect does not identify the custody cell it changes

Loan Liquidate reads/posts `custody(Alice,GOLD)` yet emits `Debit(Alice,1.000 GOLD)`. The common Effect signature takes Account; the special custody-resolution sugar applies to ResourceId. Alice is an Account, and `balance(Alice,GOLD)` is distinct from `custody(Alice,GOLD)` in the typed prelude. The exact footprint cannot decide which cell the ordinary Account debit targets without an additional rule.

Use an explicit typed custody object/capability debit or define its source-bound effect-selector mapping. Test distinct free balance and encumbered custody for Alice: liquidation must consume the pledged custody exactly once, preserve free balance, and remove only Loan's authorized lock. Do not count both a cash debit and a custody subtraction for the same transfer.

### M4 — Coverage tables should mark the remaining contracts as unresolved, and demonstrate pure function/library use

The three-surface status map and formal-obligation table are accurate. The requirements checklist's checked “complete” bounds/footprint/continuation items should mean visible proposal coverage only until H1–H3/M1–M3 are disposed. The syntax tour advertises bounded `fn` and reusable libraries but presents relation signatures and comments instead of an actual pure function body with a reusable call. The original surface request benefits from one small total typed function/example, an exhaustive ordinary ADT match and its explicit work bound; no extra financial implementation is required.

Keep parser acceptance, typechecking, certificates, kernel qualification, persisted duties and ledger results open. Proposed grammar still rejects through beta rather than acquiring support from this review.

## Validation performed and final scope

The two assigned hashes were verified before review. Entire proposed source was inspected against the original requirements; the loss/net arithmetic discriminator was evaluated with Python rational/integer arithmetic only. It is not a horizon compiler, simulator, model-checking, native or ledger experiment. No unperformed test is marked reproduced.

**Recommendation:** Preserve the repaired coverage and architecture, resolve H1–H3 with exact source rules and independent expected observations, and disposition M1–M4. Re-review changed bytes. This reviewer concurs with the listed design choices but does not concur that these exact candidate bytes yet satisfy every typed-contract requirement. Horizon execution, beta acceptance of horizon syntax and all formal/financial proof obligations remain unsupported/open.
