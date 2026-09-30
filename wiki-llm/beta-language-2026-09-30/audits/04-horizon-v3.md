# Independent audit 04: full typed horizon v3

**Verdict:** concur with the corrected proportional slash and signed receipt-scope decisions, and with repair of my original H1/H2 presentation findings. One **medium** interface-completeness defect remains in reconciliation head binding. No high defect was found in the independently checked literal arithmetic. Do not count this review as unconditional approval of the whole interface while M1 below remains unresolved.

## Inputs, identity and scope

Requested seat: fourth PL reviewer, GPT-6.1 Sol medium. Returned model/effort receipt is not exposed to this context; actual routing conformance remains unverified. No peer reviews or repair reports were opened. Prior findings were recovered from my own context; references to peers inside the candidate were treated only as candidate text, not external review evidence.

Exact input hashes were verified at review start:

| Artifact | SHA-256 |
| --- | --- |
| PROGRAMMER-MOCKUP.md | `16d419cb4e8f26571d9aab79406e5e8864e7db623028b98929274c727e5d601e` |
| FULL-LANGUAGE-HORIZON.md | `f5c947822879f55d25a06efc48eec77eb82149c9cadb262b31e6274f683db488` |

Reread AGENTS.md and the checked-in develop skill, and refreshed guarded status in the assigned worktree. SP01.6 dependent dispatch remains blocked by unresolved operational history, stale binding/candidate inputs, missing accounting and unavailable live resource state; pending transactions are empty. This review performed no dependent dispatch.

Read the whole horizon §§1–16 and checklist, and the whole mockup, against the previously inspected requirements and Source6/Core5 baseline. Only this assigned audit file was written. No production code, proofs, credentials, ledger state, original audits or candidate documents were changed. No horizon compiler, certificate verifier, K/Quint model, authentication path or ledger transaction was run. The independent reproduction below is literal integer/rational arithmetic, not execution of the proposed language.

## Substantive concurrence

| Design choice / earlier finding | Decision and exact boundary |
| --- | --- |
| Original H1: financial interfaces hidden in prose strings | **Repaired as a proposed interface.** Nominal resources, prices/clocks, typed cells/read-write footprints, finite holes, bounded pure functions, explicit evidence and derived effects are visible. Typed profile relation signatures plus the adjacent financial equations supply inspectable requirements; implementation and soundness are explicitly open. |
| Original H2: composed episode lacks one signed promise | **Repaired for its economic stages.** §13 binds one parent signature, exact program/policy/recipient/asset terms, paired claim, grants, cumulative budgets, finite holes and three terminal receipts. It does not reuse independent bridge authority or let a solver create a new claim. The remaining reconciliation scope gap is M1. |
| v2 slash contradiction | **Repaired.** SameTierProportional explicitly places pending, mature-uncommitted and free holders in VaultEquity. External lock priority decides eligible restaking authority/capacity, not holder first-loss allocation. Rank changes cannot silently change the holder loss rule. Paid claims are excluded, and a zero owed claim still needs accepted burn/book/duty discharge. |
| v2 terminal net-floor ambiguity | **Repaired.** Bounds contains cumulative gross/fees only. ReceiptRequirement binds an exact signed gate, recipient, asset, scope, floor and complete delivery-charge selectors. Loan500 and mint100 are historical milestones. Redeemed/EmergencySettled and Delivered/OriginRefund/DestinationReturn are distinct terminal gates. No floor is inferred from a constructor name or old wallet balance. |
| Composition continuation heads | **Agree for the five declared economic stages.** Escrow and Claim use signed initial anchors; Swap and DestinationReturn name Claim's exact accepted successor; OriginRefund names Escrow's. Unrelated progress requires fresh preserving authority, not completion-selected latest head. Qualification of a delayed Claim links its original edge without reminting. |
| Signature/capability/expiry semantics | **Agree as a specification.** Parent, grant, resource and native signatures are conjunctive distinct statements. Explicit domain/epoch keys and revocation/expiry checks cannot be substituted by successful receipt status. Expired grants leave duties and reservation intact; renewal requires new signatures and preserves original terms, receipts, claim and spent counters. Replacement-key qualification and canonical renewal encoding remain open obligations. |
| Pledged collateral versus free wallet funds | **Agree.** DebitCustody targets LoanPledge under its capability and frames free_gold unchanged. Liquidation must remove only the relevant lock and fund the bound creditor, retaining75 USD residual debt in the worked case. No second wallet debit is inferred. |
| Pure function and match scope | **Agree as a proposal.** coverage/missing uses a same-asset comparison, guarded unsigned subtraction and finite exhaustive matches; the result is diagnostic information, not permission to weaken the floor. Declared work6/12 is a proposed cost contract; an operational counting semantics and its proof are still required. |
| Eight families and full horizon | **Visible proposal coverage achieved.** The eight families have named financial transitions, evidence, footprints, authority, failures and duties. General exchanges/routes, ACTUS calendars, perpetuals, oracle aggregation, protected additional risk tiers, preexisting burned-claim payments and wider restaking profiles retain explicit additional obligations. S0 does not replace those goals. |

The separation between accepted beta grammar and proposed horizon grammar is clear: this document establishes only specified horizon interfaces. Existing Source6/Core5 local preparation remains PreparedUnqualified with four external premises and four unverified bindings. Independent beta command/code evidence is outside this audit. A successful parser, arithmetic calculation or interface review cannot qualify signatures, observations, proofs or ledger commitment.

## M1 — Reconciliation has no resolved-head rule under its declared scope

**Repository observation:** §2 says every accepted stage resolves exactly one signed HeadRule (line328); AuthorizationConjunction requires ResolvedHeadBinding (lines314–318). Common mutating-stage rules consume work/nonce/head (line168 onward). The explicit composed Promise.heads list (lines1664–1673) covers Escrow, Claim, Swap, OriginRefund and DestinationReturn, but not Reconcile.

Nevertheless ObserveGrant authorizes Reconcile and the on_timeout branch constructs `scopeFor(stage: Reconcile, intent: Promise, identity: Economic(claim: Pair,stage: Reconcile), ... authorization)` (lines1724 and1831). The explanation says this reuses §11's reconciliation footprint. That stage contains a journal SetCell/write (lines1340–1348). Reconciliation is described as economically read-only, but that alone does not exclude its accepted ledger/journal write from the common mutating-stage or resolved-head rules. Kernel/service observation records are explicitly forbidden from becoming economic predecessors (line333), which is the right constraint but needs a compatible scope contract.

**Exact discriminator:** accept Escrow p0→p1, then send Claim f0→f1 with response Unknown. Trigger Reconcile while the receipt outcome is unresolved. Resolve the required AuthorizationConjunction.heads from the signed Promise: there is no Reconcile StageHead and no signed rule selecting its read head. If reconciliation writes a ledger journal under common mutation rules, it consumes a successor edge, yet subsequent Swap still requires the exact Claim successor f1. If reconciliation is only service observation recording, explain why it needs the universal accepted-stage ResolvedHeadBinding and why a ledger SetCell stage is reused. Caller selection of a convenient current head is prohibited by the proposal itself.

**Repair:** explicitly distinguish the two possibilities in typed source. Prefer an observational reconciliation action with authenticated query/snapshot binding, durable attempt/work authority and a service fact journal, without economic head/nonce consumption; qualification must then recover the original Claim edge, never use the observation journal as its predecessor. Alternatively declare a signed Reconcile StageHead, its exact ledger read/write/control effects, and an explicit signed continuation rule explaining how its accepted edge affects subsequent economic stages. In either design, provide a source-fixed rule for both “Claim never accepted” and “Claim accepted but response lost,” retain duties/budgets and reject caller-chosen unrelated heads. Do not make a service acknowledgement stand in for accepted economic history.

**Severity:** medium. This is a missing typed authority/history contract on a critical continuation path; the proposal does not assert that an existing implementation bypasses it. Repair is required before calling the whole proposed interface complete. It does not overturn the positive economic-stage head or qualified-recovery decisions above.

## Independent reproduction of §16

Ran a fresh Python3 integer/Fraction calculation in the worktree using equations, not a document-provided script. Command exited0 with all assertions passing. The core calculation was:

```python
def allocation(backing_cents, loss_cents, weights):
    remainder = backing_cents - loss_cents
    supply = sum(weights)
    owed = [remainder * weight // supply for weight in weights]
    return owed, remainder - sum(owed)
```

| Discriminator independently recalculated | Result |
| --- | --- |
| B1210/L121/S1100, weights100/1000 | Owed99/990, dust0; claim-first would instead pay0/1089 and therefore violates the signed rule. |
| Same tier, weights100/200/800 | Owed99/198/792, loss11/22/88, total loss121, dust0; changing maturity/lock rank changes no weight. |
| Paid record, active shares0 | Active denominator remains1100; paid55 is outside allocation. Cursor7→8 is a specified once-consumption transition, not something this arithmetic script authenticates. |
| B101/L1/S3, weights1/2 | Owed33.33/66.66, dust0.01, exact total100. |
| Protected senior100, active B1210/L121 | Senior100 unchanged, equity1089, owed99/990; dividing by total custody1310 would produce a different ratio. |
| Total1210 with protected100, active B1110/L121 | Active989, owed89.90/899.09, dust0.01, protected100 unchanged. |
| Deposit100 into1000/1000; funded reward110; slash121; withdrawal99/burn100 | Mint100 shares,1210/1100 before slash,1089/1100 after,990/1000 after withdrawal. No-reward slash110 produces a90 claim. |
| Loan500,1% coupon, repayment30, liquidation400 |505→475→75 residual debt; milestone500 and later cash delta470 are distinct facts. |
| Mint100, burn100; GOLD deposit0.200/redemption0.190 | Terminal MUSD0, GOLD cash delta−0.010; scoped mint100 and redemption0.190 still hold. Emergency gate selects the same0.190 floor without selecting ordinary Redeemed. |
| Composed100 USD escrow plus0.20 fee; destination input99.70 plus0.30 fee | Source gross100.20/fee0.20, destination gross100/fee0.30; refund100 leaves source cash delta−0.20 while preserving spent counters. |
| Fresh output0.900 versus0.899 | The former meets0.900; the latter fails regardless of old wallet balance. |
| Fresh credit520 less delivery charge20; fresh500 less20 |500 passes floor500;480 fails. These isolated charge probes do not authorize a20 fee under the actual fee0 LoanPromise. |
| Reserved claim90 plus old wallet1000 | Scoped90 still fails100; old money contributes zero. |
| Pledged custody2 minus transfer1; another lock0.5 | Custody1 and other lock0.5 remain; free5 unchanged; buyer receives1; creditor400 and default75 match the source equations. |
| Pure missing0.899 versus floor0.900 |0.001 GOLD; no financial effect or floor change. |

Also recomputed the mockup's transfer/repayment balances, allowance spent/remaining, accrued-first principal8000, option payoff40/residual460/profit35 and oracle age1/age6 examples. Their literal arithmetic agrees with the text.

The §16 head, signature, replay, expiry, query and resource mutations were checked against the written predicates, not represented as executed rejection tests. In particular, stale fx, a forged same-text predecessor, missing Reserve signature, expired301/401 authority, substituted free-balance debit and swapped query identity are forbidden by the specification. Their actual rejection and first-failure correspondence remain implementation/formal obligations. Reconciliation's own head case is not covered consistently, as M1 records.

## Remaining obligations and abstentions

Required follow-up includes the M1 scope/head decision; bounded grammar/type/totality and exhaustive-match semantics; exact profile financial equations and certificate verification; complete footprint/framing and observation equivalence; signed canonical envelopes/grants/renewals; authenticated same-head cells, keys, capabilities, observations and foreign exclusions; atomic allowance/replay/head/duty consumption; and K/Quint/native/ledger correspondence. Holder-tier dust, book/supply/custody coherence and concurrency must be proved for every accepted transition, not merely the displayed amounts. Safety claims do not establish progress after expiry without renewed authority.

Abstain on usability and the requested developer trials, actual beta/horizon execution correctness, proof soundness, privacy, authenticated providers/kernel receipts and financial/native/ledger acceptance. No mandatory qualification gate is closed by the arithmetic reproductions or this design vote. Actual returned reviewer identity/effort remains unverified without a host receipt.
