# RP01 funded full recovery

This fixed JSON design oracle demonstrates a further funded 450 Cash repayment, a retained zero-balance discharged Loan1 row, and the resulting loss reversal. It is a proposed generic case awaiting fresh independent GPT-6 and Grok 4.6 reviews. It is not an accepted language profile, protocol implementation, legal rule, proof relation, backend result, or financial ledger settlement.

The original partial case and all its files are preserved. Their hashes at task entry are in `preserved-partial-hashes.json`. Its approval remains scoped to partial recovery. That original checker exports only `checkCase`; it returns summary values, not a full successor. This successor imports that validator unchanged, then separately replays the pinned original initial state and events to compute the predecessor. The replay does not use any expected `before` or `after` snapshot as computational input. The original validator still reads those snapshots to check its results. Only the validated, event-derived state enters the extension.

## Supplied facts and proposed policy

Repository observation: the original partial case ends at time 120 with Pool Cash 640, Borrower Cash 45, FeeCollector Cash 15, Buyer and Custodian Cash zero. Buyer holds all four Collateral units. Loan1 principal/outstanding is 450, status Defaulted, and no collateral remains encumbered. Carrying receivable is zero because the 450 gross receivable has a 450 allowance. All eight old grants have zero remaining. Ordinary work has remaining 2/spent 10 under its original allocation 12; closure reserve 2 is separate.

Chosen hypothetical input: a distinct Sponsor owns 450 preexisting Cash, with zero Collateral. Composition includes that external account exactly once. The original five-account Cash subtotal is 700; the newly explicit six-account universe totals 1150 before and after the successor. This models a finite source, not minting, an oracle quote, or another payment by the underfunded Borrower. The source's actual ownership and custody remain assumed external facts; no chain observation establishes them here.

Chosen hypothetical policy: Sponsor pays Pool directly on Borrower's behalf. This contribution is nonrefundable, has no new fee, and creates no subrogation right, reimbursement duty, or successor liability. It is not loan forgiveness: Pool receives the full 450 Cash and grants funded repayment credit. The policy deliberately avoids assuming that the borrower can spend another 450. Selecting this contribution policy rather than a new borrowing arrangement needs SP01 consequential review. No protocol or legal fidelity is asserted.

Chosen hypothetical authority: three new source records, each issued at 121, valid through 130 inclusive, bound to Loan1, GenericLossV1, and the exact partial-case SHA-256. The financial transition runs at 125. Their fixed template is checked, so changing issuer, recipient, asset, quantity, dates, version, or predecessor does not silently change the example's admitted authority. Names describe supplied records, not verified signatures or institutional legitimacy.

| Fresh grant | Issuer | Holder | Scope | Asset/payee | Initial → remaining / spent |
| --- | --- | --- | --- | --- | --- |
| Sponsor450 | Sponsor | Sponsor | Transfer | Cash / Pool | 450 → 0 / 450 |
| Recovery450 | Pool | Servicer | Repay | Cash / Pool | 450 → 0 / 450 |
| Reverse450 | Governor | Servicer | ReverseImpairment | accounting / Loan1 | 450 → 0 / 450 |

The new grants occupy distinct IDs. Every exhausted old grant remains byte-for-byte identical in successor state. No old grant is renewed, and the BorrowerGrant remains spent 155/remaining zero. The new grants are supplied as a separate admission boundary; issuance signatures and revocation checks are not implemented by this oracle.

## Explicit finite work extension

The three events require three ordinary units, but the partial predecessor carries only two. A distinct RecoveryWorkSponsor resource account has initial 1. It contributes that one unit at composition, leaving source remaining zero/spent one. The successor has ordinary remaining 3/spent 10 and records `additionalAllocated: 1`. Its lifetime ceiling is explicitly amended from 12 to 13 ordinary units, while closure reserve stays 2. The total resource universe is old 12 + old reserve 2 + external source 1 = 15. At the end spent 13 + remaining 0 + reserve 2 = 15.

This is a proposed bounded one-unit resource extension requiring consequential review. It is not an automatic continuation refresh and is not authority for any registered campaign, compiler, proof, wallet, or network run. The original case's declared bounds and work history remain unchanged. Controls reject a source with no resource, a missing contribution, a ceiling of 12, and a changed reserve. Actual host test costs are outside these design units.

## Independent arithmetic and state

The extension is one atomic three-event step:

1. Sponsor transfers 450 Cash to Pool under Sponsor450. The executed transfer enters a same-step funding map keyed by FullCash. Sponsor falls from 450 to zero; Pool rises from 640 to 1090.
2. Servicer repays 450 on Loan1 under Recovery450, consuming exactly that Cash funding. Loan1 principal and outstanding fall from 450 to zero. Its row, identity, debtor, creditor, denomination and recourse policy remain. The status and continuation status become Discharged. Funded nominal recovery becomes 550 + 450 = 1000; created and forgiven nominal amounts stay zero.
3. Servicer reverses 450 impairment under Reverse450 against the same new repayment allocation. Allowance becomes 450 − 450 = 0. Cumulative recovery gain becomes 150 + 450 = 600. Reversal is impossible before the funded repayment allocation exists.

All events apply to a tentative private state. No successor is returned unless the entire step, complete expected state, and effects pass. A reversal failure after tentative transfer/repayment leaves the caller's full input unchanged, as verified by the common test wrapper. These are JSON call semantics, not a ledger atomicity demonstration.

| Quantity | Partial predecessor | Composed before | Full successor |
| --- | ---: | ---: | ---: |
| Pool Cash | 640 | 640 | 1090 |
| Borrower Cash | 45 | 45 | 45 |
| Sponsor Cash | outside old universe | 450 | 0 |
| FeeCollector Cash | 15 | 15 | 15 |
| Buyer / Custodian Cash | 0 / 0 | 0 / 0 | 0 / 0 |
| Buyer Collateral; all others zero | 4 | 4 | 4 |
| Loan1 principal/outstanding | 450 | 450 | 0 |
| Loan1 status | Defaulted | Defaulted | Discharged |
| Gross receivable | 450 | 450 | 0 |
| Impairment allowance | 450 | 450 | 0 |
| Carrying receivable | 0 | 0 | 0 |
| Pool NAV | 640 | 640 | 1090 |
| Cumulative impairment expense | 600 | 600 | 600 |
| Cumulative Pool fee expense | 10 | 10 | 10 |
| Cumulative recovery gain | 150 | 150 | 600 |
| Current net loss | 460 | 460 | 10 |
| HolderA book value / current loss | 384 / 276 | 384 / 276 | 654 / 6 |
| HolderB book value / current loss | 256 / 184 | 256 / 184 | 436 / 4 |
| Funded nominal recovery | 550 | 550 | 1000 |
| Ordinary remaining / spent | 2 / 10 | 3 / 10 | 0 / 13 |
| Separate closure reserve | 2 | 2 | 2 |

Final accounting follows the original policy: gross receivable equals nominal outstanding; carrying receivable equals gross minus allowance; NAV equals Pool Cash plus carrying receivable. Net loss = impairment expense 600 + Pool fee expense 10 − recovery gain 600 = 10, also initial NAV 1100 − final NAV 1090. Shares stay 600/400 with total 1000. Exact pro-rata values are 1090 × 600/1000 = 654 and 1090 × 400/1000 = 436; current loss allocations are 6 and 4. Every arithmetic result and multiplication is checked against UInt128; there is no rounding residue.

The retained `legalRecourse: Retained` field records the historical recourse policy under which this loan was recovered. At principal/outstanding zero and status Discharged it confers no remaining monetary claim in this case. It does not create a Sponsor claim or preserve a second liability. Canonical naming and representation of this historical metadata remain SP02 review work.

Historical loss is preserved as four ordered allocation records, computed from events: 600 (360/240), then 610 (366/244), then 460 (276/184), then 10 (6/4). `allocatedNetLoss` is current loss net of recovery, not a monotonically increasing counter. Cumulative impairment expense 600 is never erased. Pool's original fee 10 remains its loss; Borrower's earlier fee 5 remains a cash expense borne by Borrower, with FeeCollector holding 15 total. No new fee or refund occurs.

Twelve complete actor/asset effect rows cover all six actors and both assets. Sponsor gross-debits 450 Cash/net-change −450; Pool gross-credits 450/net-change +450. All other extension cash and collateral effects are zero. There is no incoming transfer treated as a refund or reduction in gross debit. Total Cash remains 1150 and Collateral four.

## Bounds and callable checks

Run from the worktree root:

```sh
node deliverables/rp01-loss-full-recovery-2026-09-09/check-case.mjs
node --test deliverables/rp01-loss-full-recovery-2026-09-09/check-case.test.mjs
```

The extension has one step, at most three events, six actors, twelve asset rows, one retained duty, two share claims, and eleven grants. Histories end with six transfers, three allocations, two original observations, and four loss snapshots. Source transfer and allocation identities have a 64-character limit; replay with any used prefix identity is rejected. UInt128 integers use canonical unsigned decimal strings. The compact extension sidecar is limited to 65536 bytes. The old prefix has its own unchanged bounded checker and pinned source dependencies. These are local design bounds, not cryptographic admission cost estimates.

The result emits the full final state, actor effects, conserved totals, and concrete read/write paths derived from state leaves. Read paths cover the entire composed state because boundary invariants inspect it; writes are exact changed leaves, including retained history additions. There is no parallel-execution or signed-footprint implementation claim. Inputs have finite size under the fixed schema and sidecar ceiling; parsing malformed/oversized JSON at the host boundary is not a native admission implementation.

The first retained test command failed against `FULL_RECOVERY_NOT_IMPLEMENTED`. The subsequent implementation passed all 47 tests. There are two successful cases (the complete trace and inclusive authority expiry) and 45 rejection cases, including the independent expected-state substitution test. Rejection controls exercise unfunded, underfunded and overdrawn repayment; exhausted old grants; wrong payee/asset/issuer; missing, rebound and stale fresh authority; replay; reversal without allocated funding; unknown discharge effects; deleted terminal duty; erased historical loss/expense; changed grants and effects; unauthorized work extension; integer boundaries; hidden asset accounts; source pins; and false language scope. Every success and rejection checks input immutability. This is a bounded suite, not exhaustive branch coverage or an independent audit.

`logs/red-01.log` preserves the failing command output. `logs/green-01.log` preserves the initial passing run. The final command receipts and result retain actual execution output with command, exit status, source hashes, and runtime version. The author-authored case expectations are separate from the checker; changing expected Pool Cash cannot select a different computed successor. Fixed source/policy equality checks intentionally reject inputs outside this illustrative case; they do not establish a parameterized financial library.

## Remaining ownership and evidence gates

SP01 must review the chosen contribution, no-subrogation, authority and work-extension policy and reconcile the full RP01 challenge map. SP02 owns canonical richer source/type/authority/display representation. SP03 owns implementation of this full transition in language semantics and correspondence tests. SP07 owns primary-source/version-specific policy binding, including alternative loss allocation and recourse policies. SP09 owns mandatory ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance with authenticated grant lineage, observations, custody and actual acceptance evidence.

The original repayment kernel has reusable nominal Cash Transfer/Repay arithmetic, but this oracle does not drop default classification, impairment, shareholder claims, history or authority to claim kernel success. No `.mori`, Core, K, Compact, compiler, native recursive proof, wallet, or ledger integration was performed. No transaction was submitted. Full RP01 design acceptance and broader product acceptance remain open until their separate requirements and independent reviews are satisfied. Historical Opus approvals remain unchanged and do not approve this new candidate; pending reviews use the current user-selected GPT-6 and Grok 4.6 route.
