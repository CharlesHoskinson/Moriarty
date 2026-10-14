# Financial state and operation composition — proposed draft

This is an author proposal for SP01.3/RP01, not an accepted profile or replacement
for the archived candidate04. The accompanying operation table accounts for its
38 names. It does not establish complete financial semantics, source lowering,
K rules, signing compatibility, native execution or Midnight acceptance.
Expression candidate02 remains immutable; its later scoped source approvals do not approve this new financial proposal. No new
operation is added here. Where a required rule body or binding is absent, profile admission rejects
`RULE_MISSING` or `RULE_BINDING`; a caller-supplied success flag,
arbitrary amount or unchecked callback cannot fill that gap.

## Sources and disposition

Inherited obligations are the RP01 challenge requirements in
[report reconciliation](../../openspec/REPORT-RECONCILIATION-2026-09-07.md:37),
[SP01.3](../../openspec/sprints/sp01-financial-contract-and-execution-admission.md:49),
[asset-study integration](../../openspec/ASSET-STUDY-INTEGRATION-2026-09-09.md), and
[the transformation model](../security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md:23).
The archived 38 signatures are proposals with known defects; see
[C04-F1, F3–F6](../../evidence/moriarty-completion-program-2026-09-07/SP01/independent-review-round-08/successor/gpt6-semantic-partial-review.md:11).
The report's opaque external citations are not independently verified evidence
of legal rights, implementation conformance or deployment. The new choices below
are distinguished from those inherited requirements.

## F-D1: Proposed finite financial registry

A bound program contains immutable identity declarations and a bounded financial
snapshot F. Identity is exact domain/network/deployment/identifier/version, never
a ticker, argument position or brand. Parties identify accounts, beneficiaries,
controllers and obligors separately; controlling a key does not prove legal
ownership. Every reference resolves through a declaration or an existing object
in F before execution. A dangling reference rejects. The proposed record model:

| Record | Required information and invariant |
| --- | --- |
| Asset | Domain, network, identifier, version, integer quantum, issuer, transfer-policy version; explicit external-claim reference if any. AssetA and AssetB cannot be added or substituted. |
| Resource | Either asset quanta `Asset(A)` or share quanta `Share(V)` of a declared vault. Balances are keyed by exact resource and account. A `Shares<V,H>` expression fixes both this resource and the holder; moving shares requires a new holder-indexed value, not an implicit cast. Shares are not automatically ERC-20 assets. |
| Balance | Owned resource quantity, with available = owned − sum of disjoint pledge/settlement reservations. Freeze is a separate transfer-policy restriction, not another subtraction. Each reservation has its own quantity and backing location. No claim against an issuer is available cash. |
| Supply | Issued and retired quanta for an explicitly represented asset/share system; no token supply is inferred from an unrelated entitlement record. Supply delta equals authorized mint minus burn. |
| Encumbrance | Fresh ID, owner, resource, beneficiary, kind `Pledge` or `SettlementHold`, remaining quantity, unique priority within a collateral group, release condition, scoped seizure/recovery authority and linked debt/claim. A frozen resource can also be pledged. The same quantum cannot back two exclusive reservations. |
| Debt | ID, debtor, creditor, controller, nominal denomination D, principal P, accrued interest I, immutable allocation/conversion/accrual rule IDs, last accrual endpoint, status, liability authority. Outstanding O=P+I. D need not equal settlement asset A. Fees are separate payment effects; they do not become interest by renaming. |
| Share system | Vault, supply, per-holder quantities, underlying-resource references, conversion rule ID, liquidity/fee/rounding rules, and claims attached to each tranche or holder. Accounting value, actual redeemable output and market price are separate fields/observations. |
| Position | ID, holder, obligor/counterparty, tagged economic kind, bounded kind-specific fields, collateral references, fee/P&L claims, rule ID, active/closed status. A liquidity range is a subtype, not the definition of all financial positions. |
| Request | ID, controller, beneficiary, input resource Q, output resource A, pending-input escrow, original input owner and bound refundRecipient (that owner unless an explicitly signed assignment says otherwise), input settlement sink, output funding account, dedicated output escrow, inputCommitted C, inputFulfilled F, inputRefunded R, inputPending P, outputAccrued E, outputClaimed K, fillOpen flag, conversion/cancel/expiry rule IDs. C=P+F+R; E=K+claimable; input/output counters are never interchangeable. |
| Message | ID, local/remote domains, sender, bound recipient, bound refundRecipient (sender unless an explicitly signed recovery assignment says otherwise), payload commitment, resource Q, locked quantity q, dedicated local escrow/reservation, deadline, evidence-policy ID, state `Pending`, `Received` or `Refunded`. Payload-only messages explicitly have q=0. Receiving evidence is not automatically financial settlement. |
| Event claim | ID, event, claimant, obligor, stake resource/quantity and funding reference, finite outcome set, resolution-policy ID, fixed payout-rule ID, optional resolved outcome/evidence, remaining settlement entitlement and status. |
| Reward account | ID, holder, obligor/funding account, share resource, share units, entitlement-rate rule, pending entitlement in a declared output asset, cumulative earned/paid/lost quantities and record-date snapshot reference when applicable. Share units and entitlement value are separate. |
| Duty | ID, obligor, beneficiary, typed quantity or bounded conditional claim, due/exit conditions, priority, backing/reservation reference or explicit credit-risk model, creation lineage, discharged quantity and residual. Nominal debt, funded withdrawal entitlement and external conditional claim remain distinct kinds. |
| Instance | ID, exact program/spec/policy versions, mode `Dormant`, `Live`, `Paused` or `Revoked`, predecessor identity, conserved ordinary/recovery work, attached claims/duties, authority and external assumptions. Program revocation is a separate registry fact, not silently altered by instance Revoke. |

These mathematical signatures now have a proposed closed reference wire schema
in financial-schema.json and financial-schema.md. A concrete product profile
monomorphizes every role and supplies its finite typed rule bodies; none may
remain an abstract mathematical type at profile admission. Each finite collection is proposed at most128 entries; total F
must also fit the expression snapshot 65,536-byte/4,096-value-node/depth64 limits.
128 entries per collection never permits their unbounded product. Immutable
history is referenced by bounded commitments and admitted predecessors; it is
not recursively embedded into every state. Every stored amount/counter is
UInt128. Proposed NetAmount<A> represents exact eligible-incoming minus paid-fee
counters in the full symmetric range −(2^128−1)..+(2^128−1). Its new type tag and
canonical signed-decimal encoding are specified in the mixed-arithmetic supplement;
no SInt128 narrowing is permitted. All accumulation is checked before subtraction.

Each operation also receives the same explicit immutable context C: program/spec/
policy versions, source/action and originating Emit span/path, actor/delegation,
current predecessor IDs, signed authorization identity, permitted resource/party/
operation sets, per-action and lifetime caps, observation snapshot with evidence
references, financial-rule registry, and remaining work. A row omitting an actor
operand does not get ambient caller authority: C fixes the acting party and the
row resolves the relevant debtor/holder/controller from F. Policy/rule references
must resolve to a finite versioned relation with specified inputs and guards;
`verified=true`, dynamic JavaScript and undeclared host callbacks reject. The
exact closed schemas, cryptographic authority judgments and source-bound helper
costs remain required before this context becomes executable. Genesis additionally
needs the real external-input/issuance boundary, not a fake predecessor receipt.

## F-D2: Asset movement, gross authority and receipt allocation

Every actual movement has a fresh action-local ID, exact resource, source,
recipient and positive quantity, operation origin, policy version and authority
reference. Ordered effects retain this information. `move(x,y,Q,q)` checks both
policy legs, scoped debit authority, available or specifically releasable backing,
recipient constraints and checked balances, then performs exactly one debit and
one credit. All changes are tentative until the complete action passes.

For each protected account and resource, cumulativeGross increases by **every**
outgoing movement, including a fee; cumulativeFees increases only for fee effects.
A refund/incoming transfer cannot decrease either counter. Remaining authorization
is cap − cumulative consumption; checked consumption must be at most cap.
`netGoal = cumulativeEligibleIncoming − cumulativeFeePaid` is the proposed inherited
receive-goal convention, distinct from full balance delta (all incoming − all
outgoing). A profile must explicitly declare which incoming classes qualify for
a goal; refunds and self-routing cannot silently satisfy a purchase goal. Net is
signed. A received fee is still a concrete inflow for balance conservation, but
it counts toward a particular receive goal only if its bound goal permits it.
Physical token input/change accounting at Midnight is a separate observation
binding; do not equate a semantic10,000 debit with a100,000 consumed UTXO.

Each basic movement can expose a bounded receipt. `SettlementUse(receiptId,q)`
allocates q of its quantity to one financial cause. Per receipt, allocations sum
to at most its exact quantity; each use names its operation and field purpose.
Receipt source, recipient and resource must match that purpose. A second use of
the same allocation rejects; partial allocation creates an explicit remainder.
All referenced receipts must precede the consuming operation in the descriptor
sequence and share its action/predecessor. A debt/share operation consuming a
receipt changes the financial record **without debiting those assets again**.
High-level operations such as Exchange, RequestFulfill and MessageRefund emit
and allocate their own movements atomically; a caller may not additionally claim
an earlier transfer for the same named internal leg. `operations.json` states
which form each operation uses. Unallocated transfer remainder remains an
ordinary transfer and cannot stand in for discharged debt. No receipt survives
into another action as fresh funding; persistent escrow is represented in F.

For each asset over a closed declared financial footprint:
Σbalances_after − Σbalances_before = authorizedMint − authorizedBurn.
All external ledger inputs/outputs must extend that footprint explicitly. For
shares use the analogous share-supply equation. Fees are transfers, not vanished
value. Nominal interest accrual changes debt, not token supply. A portfolio mark,
exchange rate, token burn or external attestation does not alone settle a duty.

## F-D3: Proposed staged composition with expressions and Ensure

The expression proposal deliberately defines only descriptor preparation and
ordinary post-writes. It cannot currently execute this combined rule unchanged.
This draft proposes the following extension for later review/implementation;
**it does not claim candidate02 already gives financial post-state to Ensure**.

1. Admit complete finite Σ, source/Core, arguments, observations, ordinary state,
   F, authority, predecessor and input bounds; type-check the whole action. All
   financial fields are read-only to NextWrite. Unknown rule IDs reject before
   expensive work. Bind operation record types to their Emit descriptors.
2. Evaluate the lexical body before the Ensure suffix once, using immutable pre,
   arguments/observations, staged ordinary writes and descriptor accumulation.
   `ReadPre(pre)` always sees original F, even after Emit. No next read is added.
3. Interpret descriptors in emission order on a tentative F overlay, passing the
   explicit finite operation context. Earlier successful descriptors are visible
   to later financial descriptors, never retroactively to an earlier expression.
   Consume one additional ordinary work unit per entered financial operation;
   bounded helper work and rule-specific iteration costs require a complete
   source-bound cost table before freeze. A quoted charge is not a certificate.
4. Merge staged ordinary fields with tentative financial fields; footprints must
   be disjoint by registry classification. Evaluate the original lexical Ensure
   suffix once, now with `ReadPre(post)` referring to this merged state. Expressions
   before the suffix are not rerun. An Ensure false discards all tentative financial
   and ordinary state, receipts and ordered effects. It is not ledger acceptance.
5. Check complete post/descriptor/effect/duty aggregates and work conservation;
   produce a proposed FinancialPrepared containing full post, complete ordered
   effects, residual duties/authority, source/span associations and remaining work.
   Signature verification, currentness consumption, ContractInvariant,
   IntentRefinement, TransitionValidity and HistoryCompliance remain mandatory
   later acceptance judgments. No callback or supplied Boolean can establish them.

Example: pre debt P100/I10; body emits funded repayment7; Ensure reads post P100
and I3. It succeeds under the interest-first rule. Ensure I0 rejects and restores
P100/I10 plus the original cash state. An ordinary x10→11 write can coexist, but
a second x write rejects statically and a raw debt write rejects TYPE_FINANCIAL_WRITE.
Operation descriptors may refer to IDs created earlier in their sequence only if
their static ID sort and bounded declaration are known; runtime freshness and
lookup occur against the overlay. Reading a newly created object in a later
*expression* is not thereby authorized; that needs explicit staged elaboration.

Proposed operation error order: current record/identity and version; actor and
policy authority; status/currentness; rule/observation provenance; typed amount
and arithmetic domain; available funding/reservation and receipt allocation;
caps; post-record invariant; complete output bounds. Static input failures still
precede all operations. Earlier descriptor rejection precedes later descriptors
and all Ensure failures. Within a row follow its ordered guards; no supplied
`firstFailingPredicate` label is trusted. Exact error-code roster, origin-span
transport and disclosure rules are still to be instantiated, not inferred from
this prose. Invalid private input must not be copied into public diagnostics.

## F-D4: Claim continuity and honest failure

For each pre-existing claim, the result identifies one or more successors whose
typed residual quantities plus authorized discharge equal the consumed quantity,
or preserves the exact claim unchanged. Unlike units require an explicit reviewed
conversion; an input amount is not simply added to output cash. Priority and
beneficiary persist unless an authorized transformation explicitly changes them.
Every new duty has a named obligor, source/rule, authority and enforceability
assumption. Insolvency/default may change recoverability, but cannot silently
zero nominal debt. Funding cannot be reserved for two independent duties.

Request cancellation closes future fills and refunds only pending input; already
accrued output entitlement survives, including after a partial claim. The proposed
escrow form below has claimable output fully backed. A credit-bearing request is
not covered by that assumption: it needs a distinct explicit debt/funding profile.
Expiry removes future-fill authority when its rule says so; it is not an automatic
request deletion, refund or paid claim. Message refund requires accepted conflict/
absence evidence under its stated remote trust model and exact local escrow;
local `Pending` alone cannot prove no remote payout. Recovery produces a new
transition and consumes current history; it does not edit the past.

All unspecified fields frame unchanged. On preparation rejection the entire
pre-state survives, with no candidate effects or duty changes. A local attempted
work diagnostic does not refresh the stored work or permit ledger retry. External
observation failure, insufficient liquidity and unavailable witness remain
possible; these rules do not promise liveness or remote atomic settlement.
