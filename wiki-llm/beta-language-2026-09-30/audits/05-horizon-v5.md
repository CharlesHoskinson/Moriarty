# Horizon v5 audit 5 — final typed correspondence review

**Scoped verdict: concur with these exact proposed interface bytes.** No remaining high or medium scope, type, authorization, first-failure or financial inconsistency was confirmed in this review. The v4 deferred scheduling and restricted Deposit/Reward repairs are coherent, and the v5 Recovery qualifier closes the proof-service versus financial-execution distinction. This is design concurrence only; it is not approval of executable horizon grammar, financial certificates, authenticated kernel behavior, native proofs or ledger settlement.

## Identity and inspected evidence

Requested host routing: `model: gpt-6.1-sol`, `reasoning_effort: medium`, `fork_turns: none`, independent seat5. The parent confirms these explicit dispatch arguments and host acceptance. No separate provider-returned identity or effective-effort attestation is available; this report records requested routing without inventing one.

Exact inspected SHA-256 values:

- `PROGRAMMER-MOCKUP.md`: `66a0afd0bc127eaf81ec58de16a86c484852bccdc7b849313302d684a036acc7`.
- `FULL-LANGUAGE-HORIZON.md`: `97f7f4b122f3ea59460914a1e577c0da69619006a73b7dc8377b3815b9907a29`.

Both matched the assignment before review. I read AGENTS.md, the checked-in `plugins/moriarty-dev/skills/develop/SKILL.md`, guarded CLI status, CONVERGENCE.md, the original programmer-facing mockup requirements, my-scope original `05-correspondence-validation.md`, `05-horizon-v2.md` and `05-horizon-v3.md`, and the entire two frozen candidate documents. Truncated tool segments were reread. No peer review was opened; peer-review references in the candidate were not followed.

Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Guarded status reports stale/missing operational admission evidence and no pending transactions. This read-only review is independent of the blocked financial campaign dispatch. Only this audit file was written; no code/source edits, commits, dispatches or transactions were performed.

## Findings and dispositions

Severity inventory: **high 0; medium 0**. Earlier findings are dispositioned below at the proposed-design level. Unperformed compiler, verifier and financial experiments remain open obligations rather than reproduced results.

| Earlier issue / current boundary | Disposition and substantive vote |
| --- | --- |
| Original UInt128 versus S127 lowering, economic IDs, total invalid repayment proposal and observation domain | Concur with CONVERGENCE and the supplement: field-specific narrowing, economic-ID restrictions, explicit invalid unsigned proposal placeholder, representable Source/6 comparison domain and complete wrapper observations are stated. These are specifications, not newly reproduced beta implementation results. |
| v2 slash allocation | Concur. SameTierProportional allocates to all active Free/Pending/MatureUncommitted holders; external lock priority and protected senior backing are separate. Paid records remain excluded. The old claim-first certificate cannot implement this policy. |
| v2 receipt semantics | Concur. Gross/fees/work are cumulative prefix bounds; signed receipt requirements independently bind milestone/terminal gate, recipient, asset, fresh delivery or reserved custody, and exhaustive deductions. The loan repayment, burned stablecoin and alternative bridge recovery branches do not silently waive a floor or net credits against gross. |
| v2 continuation heads | Concur. Composed Initial/PreviousAcceptedHead rules bind exact agreement, Pair, StageId and original accepted economic edge. Service observations cannot become an economic predecessor. Unrelated progress requires preserving fresh source authority. |
| v2 typed scopes, signatures/renewal, liquidation custody and pure functions | Concur. ScopeRequest carries the exact identity/attempt; parent/grant/resource/native signatures are conjunctive and distinct; renewal preserves original economics/history/counters. DebitCustody names LoanPledge and frames free GOLD unchanged. PureIntent supplies bounded reusable function bodies and exhaustive matches. |
| v3 early Authority failure | Resolved as a written contract. scopeFor is a total collector; missing/invalid raw facts remain deferred. The shared scheduler produces Stage→Intent→Effect→Authority→History→Failure observations. Planning, Signing and NativeComplete are distinct checked qualification kinds, with native signature explicitly pending until returned and scheduled. |
| v3 Deposit with active claim/duty | Resolved by the restricted profile. Complete same-head bounded claim/duty registries and coherent book membership are Stage premises; NoActiveWithdrawals is an explicit Effect predicate before recomputation. Pending, mature and zero-owed unburned claims reject atomically; paid-only history is framed unchanged. Reward uses the same guard. |
| Observational reconciliation | Concur with the stated Query rule and separate service journal. Request and response have separate scheduled premises; zero count/timeout alone is Unknown. Original accepted Claim can be qualified and linked once, without reminting or replacing its original edge. Query work/deduplication cannot reset parent counters or mutate ledger controls. |
| v5 recovery qualification | Concur. Core.qualifyRecovery schedules the full signed economic branch with fresh facts and returns opaque Recovery qualification. Planning cannot enter Kernel.recover. OriginRefund requires qualified complete nonreceipt/exclusion; DestinationReturn requires original receipt, surviving reserved custody and qualified no-debit rejection. Assets/evidence/branches cannot substitute for one another. |
| v5 recovery execution | Concur. Kernel.recover is proof-only and cannot write controls, advance an economic head, release custody/reservation or discharge duties. Qualified bound evidence reenters OneDomain; Planning→Signing→NativeComplete and actual ledger acceptance remain required for every recovery write, including destination receipt/journal writes. Fresh authorization/history failures cannot be overridden by a proof-service receipt. |

## Independent literal checks

I ran a separately written Python integer arithmetic probe, importing no profile, beta-expander or Core derivation helper. These are reproduced arithmetic observations only, not horizon execution or certificate verification.

| Literal case | Recomputed result |
| --- | --- |
| B1210/S1100/L121; weights100/1000 | Post backing1089; entitlements99/990; dust0. |
| Same backing/loss; weights100/200/800 | Entitlements99/198/792; dust0. Rank and uncommitted maturity do not enter this formula. Paid entries have zero active weight. |
| B101/S3/L1; weights1/2 | Entitlements33.33/66.66; reserve dust0.01. |
| Active B1110/S1100/L121, protected senior100 separately | Active backing989; entitlements89.90/899.09; dust0.01; protected100 is unchanged by the specified relation. |
| Old active Deposit discriminator B1001/S100, claim10; deposit100 | Mint9; old owed100.10; hypothetical recomputed owed101.00. The90-cent mismatch is real arithmetic, and the repaired active domain rejects rather than publishing it. |
| Supported no-active Deposit B1001/S100; deposit100 | B1101/S109; old free100 entitlement1010.09/new free9 entitlement90.90; reserve dust0.01. |
| YieldLife | Deposit mints100; reward yields1210/1100; slash leaves1089; withdraw99/burn100 leaves990/1000. No-reward110 loss leaves90 for100 reserved shares. |
| Loan and pledged liquidation | 500+5−30−400=75 USD residual. Custody2−1=1 GOLD; Other0.5 lock can remain; free5 GOLD is framed unchanged. |
| Option fixing140, strike100, notional1 | Payoff40 USD/residual reserve460; premium5 implies35 before other costs. |
| Supplement transfer | Owner8990/Recipient1200/Fee60 atoms; allowance3990/spent1110. |
| Supplement repayment | Owner7000/Recipient3200; principal8000/accrued0/outstanding8000 atoms. |
| Composed cap and receipt probes | 99.70+0.30=100.00 WUSD; source100+0.20=100.20 USD. Fresh520−20=500 passes isolated floor500;500−20=480 fails. Refund100 retains0.20 loss; coin0.190−0.200=−0.010 GOLD is not a promised profit. |
| Effect range / pure missing | UInt128.max+1 exceeds the declared range;900−899=1 GOLD atom, or0.001 GOLD. |

The qualitative §16 discriminators also agree with the explicit written rules: bad cap plus expired grant selects Intent; valid cap plus recipient overflow selects Effect; added invalid Stage selects Stage; expiry alone selects Authority; parent expiry selects Intent. None of these judgment results was executed by a horizon evaluator. Registry completeness, rejection publication and first-failure parity across entry points remain specified-only.

For recovery, the final source explicitly specifies Planning-to-recover kind rejection, expired-grant Authority failure, bad-cap-before-expiry Intent failure, no dispatch from Recovery without native signatures, and fresh History rejection after proof-service Qualified followed by a head change. These are adequate discriminators for the claimed interface repair; they have not run. A returned proof fact alone never selects an accepted terminal outcome.

## Full-interface scope and limits

The entire typed interface preserves the eight visible family lifecycles, one-domain atomic relations, exact footprints/common controls, bounded holes/work, source-fixed authority/evidence, residual liabilities, asynchronous Unknown reservation and distinct recovery outcomes. Base-per-Quote direction, nominal financial resources/clocks and the permissionless public compiler boundary remain explicit. Readable notation, matching arithmetic and this concurrence supply no verification constructor or authenticated fact.

The supplement distinguishes existing local Source/6–Core/5 S0 from specified beta records and specified/open horizon syntax. Horizon structs, stages, functions, ADTs and protocols are not executable beta grammar. This audit ran no beta or horizon compiler, native prover, K/Quint model, financial certificate verifier, kernel adapter or ledger path. Historical S0 experiments in the original audit retain their historical scope; they were not rerun or promoted to current horizon evidence.

Before executable/formal correspondence claims, require actual total bounded elaboration/type checking, independent financial oracles, complete effect/footprint/framing checks, common first-failure observations, arithmetic and qualification soundness, exact signing/renewal bytes, authenticated same-head snapshots/observations/edges, no duplicate effect across attempts, and atomic ledger consumption of effects/bounds/history/duties. W-D0–W-D4, M4-C1–C5 and native/U financial gates remain open.

**Recommendation:** retain these repaired proposed contracts and the literal expected results. This reviewer concurs with the frozen design, with no outstanding high/medium finding in this scope; implementation and financial acceptance require their separately named evidence.
