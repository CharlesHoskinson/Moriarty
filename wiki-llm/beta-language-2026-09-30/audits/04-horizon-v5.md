# Independent PL audit 04 — whole horizon v5

## Identity and exact inputs

Requested routing: GPT-6.1 Sol, medium effort, independent seat 4. No independent returned-model/effort host receipt is exposed in this session. Requested identity is recorded; actual routing is unverified, not silently attested.

| Reviewed artifact | Independently recomputed SHA-256 |
| --- | --- |
| `PROGRAMMER-MOCKUP.md` | `66a0afd0bc127eaf81ec58de16a86c484852bccdc7b849313302d684a036acc7` |
| `FULL-LANGUAGE-HORIZON.md` | `97f7f4b122f3ea59460914a1e577c0da69619006a73b7dc8377b3815b9907a29` |

Scope: the whole displayed proposed typed interface, all eight financial families and composition, sections 1–16 including the v5 recovery clarification, and the mockup's source/scenario trace, field mapping and status claims. This is a fresh evaluation of these bytes, not reuse of the v4 verdict. I read no peer review or standalone repair report. Author references to other audits inside the candidate are not independent evidence for this review.

Startup restored AGENTS.md and the checked-in Moriarty development skill and refreshed guarded status. SP01.6 retains stale binding/candidate inputs, missing accounting, unavailable live resources and unresolved operational history; pending transactions are empty. This document review neither clears those blocks nor establishes execution admission. Only this audit file was written; candidate documents and production code were not edited.

## Verdict

**Concur at proposed-interface scope. No remaining high or medium document finding identified.** The v4 Recovery-kind finding is resolved by the v5 specification. The prior reconciliation, failure-order and Deposit/Reward repairs remain coherent with this expansion. This verdict attaches only to the exact hashes above.

This is not proof that the horizon compiles, executes, authenticates native calls, settles on a ledger or is usable by developers. Those results require separate evidence.

## Recovery repair — agree

Section 3 no longer forwards Planning directly to Kernel.recover. The appended v5 clarification specifies the missing producer: Core.qualifyRecovery accepts the exact candidate, Planning scope, signed branch and qualified evidence, reruns Stage→Intent→Effect→Authority→History→Failure, and produces opaque RecoveryPrepared with Recovery qualification. Its branch/evidence/identity bindings cannot be created by caller flags or a grant name.

Kernel.recover now qualifies a proof service. It cannot write financial/control cells, consume economic head/replay, release custody/reservations or discharge duties. Qualified evidence is not a terminal financial outcome. The native payload/signature/finality/commit premises remain pending. RecoverOneDomain then binds returned evidence to the original parent/branch/stage/candidate/attempt and invokes OneDomain again with fresh scheduled facts. Planning, Signing and NativeComplete remain separate qualifications, followed by actual ledger evidence and Core.accept.

All displayed economic users receive that expansion:

| Caller | Proof-service premises and subsequent financial relation |
| --- | --- |
| §11 BridgeTransfer.Recover | Exact Pair, finalized complete nonreceipt and late-delivery exclusion; then one USD100 custody release/credit, escrow update and exact bridge-duty discharge under fresh authority/head. |
| §13 OriginRefund | Same nonreceipt/exclusion, signed USD100 branch and Escrow successor p1; then source refund and journal update, retaining original source gross100.20/fee0.20. |
| §13 DestinationReturn | Exact original Claim, reserved AliceF100 WUSD, authenticated atomic swap rejection/no unresolved spend; then receipt/journal updates under fresh f1 continuation, preserving source backing and creating no USD refund or second mint. |

The latter is still an economic ledger transition despite unchanged wallet balance. Its financial/control writes therefore correctly use OneDomain rather than a proof-service receipt.

### Source discriminators checked

These are predictions of the written protocol, not executed horizon tests:

- A valid p1 refund candidate gets Planning, then Recovery for the proof service, then fresh Planning/Signing/NativeComplete for financial execution. Passing the initial Planning directly to recover is a kind error.
- Recovery with no native payload signature can return a proof fact; it cannot dispatch. Actual refund/return execution must obtain the exact payload's required signatures through fresh native qualification.
- Qualified nonreceipt with a currently expired grant fails Authority once earlier predicates pass. With a bad signed cap too, Intent fails first. A parent window that also expired fails Intent Validity first. Invalid selected Stage proof wins over all later defects.
- Qualified proof service followed by unrelated head change fails the fresh OneDomain History check; proof success is not reused as authority. No refund, receipt/journal write, head consumption or duty discharge is published by that rejection.
- Unresolved preserves the existing claim reservation and signed duties. Unknown/conflicting receipt does not become nonreceipt or terminal return; a possible late swap debit still blocks destination custody qualification.
- Neither recovery kind nor a returned proof substitutes the other branch's asset, recipient, receipt floor or evidence. Accepted complete financial effects alone release the appropriate duty/reservation.

These rules repair the precise v4 mismatch without weakening native authorization or first-failure order. Core.qualifyRecovery and RecoverOneDomain remain specified interfaces awaiting implementation and correspondence evidence.

## Whole-interface and lifecycle decisions

**Nominal types and pure computation — agree.** Asset domain/representation/scale, Account, Shares, Qty, Delta, Position, price direction and clocks remain distinct. Reversed units or cross-domain identities cannot be supplied through nominal name equality. UInt128 storage and S0 Nominal127 narrowing have explicit ranges. Advanced arithmetic requires certified widths, denominator, rounding and beneficiary. The bounded pure function and exhaustive matches produce only a diagnostic amount, with no state read, effects or authority mutation. General horizon syntax is explicitly unsupported by beta grammar.

**Intent and kernel split — agree.** The source fixes recipients, program/policy, finite hole schema, resource capabilities, cumulative limits, receipt scopes, authority and outcome alternatives. Fill cannot widen them. Core owns ordered financial acceptance and complete effect/post/footprint equality. The optional kernel carries bounded facts and concrete service phases without making acknowledgements into accepted economic history. Parent/grant/resource/native statements have distinct signature types; epoch, revocation, expiry and payload correspondence are conjunctive. The new recovery expansion respects those same limits.

**Reconciliation and head rules — agree.** Both bridge reconciliation actions use signed Query identity and observational checkpoint scope, separate request/response qualification and bounded service journal. They consume no ledger head, nonce or economic controls and cannot supply Swap's predecessor. Lost response recovery qualifies the original Claim f0→f1 once under its original-time authority; observed checkpoint fx cannot replace f1. Initial and exact PreviousAcceptedHead rules remain signed, with fresh preserving authorization required after unrelated progress. Query work and durable attempt deduplication remain cumulative without economic counter reset.

**Failure order — agree as specified.** ScopeRequest collects deferred facts without an early expiry/grant failure. Stage→Intent→Effect→Authority→History→Failure applies to direct preparation, governance, observation request/response and recovery qualification. Signing and native qualification reschedule their exact payload and fresh facts. Missing facts are not true. The scheduler's execution/cost/correspondence proof remains open.

**Receipts and cumulative spending — agree.** Gross/fees/work are cumulative on every accepted prefix and survive refunds, retries and branch changes. Origination and mint receipts are milestones; ordinary/emergency redemption and composed delivery/refund/return have distinct exact terminal gates. Fresh StageDelivery and ReservedCustody cannot count unrelated old wallet balances. Exhaustive charges and explicit conversions prevent a net floor from silently ignoring fees or another asset. A refunded100 USD does not restore100.20 of spent source authority or satisfy GOLD delivery.

**Staking liabilities — agree within the restricted profile.** Signed SameTierProportional allocates holder-tier loss separately from external lock priority. Pending/mature-uncommitted holders stay exposed, Paid records stay excluded, senior protected backing is outside the denominator, and cursor/claim/book/duty updates remain complete. Deposit and Reward require complete same-head finite claim/duty registries and NoActiveWithdrawals before recomputation. Pending, mature, unburned zero-owed claims and remaining duties reject atomically; Paid-only history remains framed unchanged. Support for active withdrawals during deposit/reward is explicitly left to a future bounded complete update relation.

**Other financial lifecycles — agree at displayed profile scope.** AMM reserves/custody/invariant/LP supply are typed relations with separate route/clearing extensions; lending originates, rolls once, repays the bound creditor and liquidates pledged custody once while preserving other locks and default debt; stablecoin mint/burn/redemption/shutdown/new emergency claim expose backing/supply/duty obligations; the option funds premium/reserve, fixes qualified evidence, consumes the exercise right and pays exact payoff/residual; oracles retain missing/stale/disputed distinctions and aggregation assumptions; governance timelock/veto preserves existing duty terms. The text does not claim this finite selection supplies calendar-driven ACTUS, perpetual funding/netting, general clearing or existing burned-claim emergency payments. Those extensions stay open.

**Status and programmer horizon — agree.** Tables and checked boxes state visible proposed coverage only. The mockup preserves the smaller beta registry and existing local Source/6→Core/5 comparison as separate surfaces with four unverified wrapper bindings and four required external premises. Non-S0 recognition cannot establish financial support. The full horizon remains visible beyond S0 without pretending it is implemented. I do not infer empirical usability; the separately requested developer trials remain necessary for that question.

## Reproduction evidence

Independently recomputed both candidate hashes and executed bounded Python integer arithmetic for the literal examples. Assertions passed:

| Literal case | Reproduced value |
| --- | --- |
| B1210/S1100/L121, weights100/1000 | owed99/990; losses11/110 |
| Weights100/200/800 under the same loss | owed99/198/792; sum1089 |
| B101/S3/L1 | owed33.33/66.66; dust0.01 |
| Distinct B1110/S1100/L121 | owed89.90/899.09; dust0.01 |
| Deposit100, B1001/S100 | floor mint9; unsupported claim100.10→101.00 gives90-cent divergence |
| Supported all-free post B1101/S109 | old shares1010.09/new shares90.90; dust0.01 |
| Loan500, roll1%, repay30, liquidate400 | debt505→475→75 |
| Credit520 less charge20 / credit500 less20 | scoped net500 /480 |
| Coin0.200 deposited,0.190 redeemed | whole-life GOLD loss0.010 distinct from receipt0.190 |
| Swap input99.70+fee0.30 / source100+fee0.20 | gross100 /100.20 |
| Pledged custody2 sold1 | custody1; free wallet remains separate |
| Option spot140/strike100/notional1/reserve500/premium5 | payoff40, residual460, profit35 |
| Oracle observed150, now151/156, max age5 | ages1/6, fresh/stale |
| Parent[250,400], round301, grant expiry300 | parent valid, grant expired |
| UInt128.max+1 / floor0.900 less0.899 | overflow at2^128 / missing0.001 |
| Mockup transfer | balances8990/1200/60; allowance3990/spent1110 |
| Mockup funded repayment | balances7000/3200; principal8000/accrued0/outstanding8000 |

These arithmetic assertions do not execute the proposed first-failure scheduler, typechecker, verifier, kernel, native adapter or ledger. Semantic discriminator outcomes above are specified-only; signature, completeness, provenance and atomicity predicates were not experimentally discharged.

## Remaining obligations and abstention

Remaining work is explicit rather than a contradictory high/medium document claim: horizon grammar/elaboration/totality and bounds; financial certificate soundness; exact complete footprints and framing; scheduler and K/Quint correspondence; canonical signing bytes, opaque qualification soundness and fresh native signatures; authenticated complete registries and observational journals; reservation, replay/head and original accepted-fact linkage; persisted duties, native effect correspondence/finality and atomic ledger compare-and-consume. Beta executable results and developer trials must be reviewed independently on their own frozen inputs.

I abstain from implementation, formal proof, native-authentication, ledger settlement, empirical usability and release approval. Current proposed-interface concurrence does not rewrite previous findings or attach their historical approvals to new bytes.
