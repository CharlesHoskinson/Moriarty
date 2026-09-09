# Proposed principal-payment data fixture

This is a complete illustrative **data-level teaching case**, not an admitted JSON schema, new grammar, implemented operation, signature, proof or ledger result. Names ending in `fixture` are symbolic bindings; an executable admission fixture must replace them with exact validated profile, encoding, custody, observation-authentication, claim and verifier references. All outcomes below are specified-only.

One principal-only allocation of 30 to identified debt 100 determines a funded transfer 30 and surviving principal 70. Authors express that one financial operation. The checker derives residual duties, authority consumption and work; redundant postconditions check the derived relation rather than constitute a second financial intent.

| Binding | Proposed fixture value / rule |
|---|---|
| Target and contract | Midnight Preview; `principal-payment-v0`; instance `loan-1`; predecessor `state-0`, exclusive consumption |
| Program and semantic references | `program-fixture`, `core-fixture`, `profile-fixture`, `encoding-fixture`; unresolved symbolic references, not production hashes |
| Asset | Nominal asset `usd-fixture`, unit USD, quantum 1; all cash/principal/gross/net quantities below are integer quanta of that asset |
| Parties and custody | borrower and lender; `custody-fixture` binds their two cash accounts to authenticated settlement positions for this asset |
| Pre-state | borrower cash 100; lender cash 0; duty map has only `debt-1`, principal 100; cumulative gross 0, fee 0, net delivery 0 |
| Duty | `debt-1`: obligor borrower; obligee lender; asset `usd-fixture`; principal 100; due time 10; priority `principal-only`; status outstanding |
| Financial policy | Principal-only funded allocation; interest 0; fee 0; no rounding because all quantities are integer quanta; no forgiveness or creditor change in this action |
| Observation | `time-1`, provider `fixture-clock`, effective time 1, expires 10, authenticated under `observation-auth-fixture`; acceptable time is 1 ≤ t < 10; strict sequence; duplicate ID rejects |
| Signed authority input | Outcome mode; principal borrower; nonce `n1`; domain Midnight Preview /`deployment-fixture`/`loan-1`; permitted program/profile above; recipient lender; gross cap 40; prior gross 0; fee cap 0; minimum cumulative net 30; new-liability cap 0; expires 10; partial fills allowed |
| Claim context | All four required: ContractInvariant, IntentRefinement, TransitionValidity, HistoryCompliance; symbolic `claim-root-fixture` and `verifier-lineage-fixture`; no waiver or fallback |
| Requested action | Allocate principal 30 to `debt-1`, funded by borrower cash; no other action/effect requested |
| Resource profile | Lifetime allowance 100; prior accepted consumption 0; proposed step charge 4 under `work-model-fixture`; recovery reserve 10 included in the allowance; horizon 10; observations per step 1; queue 1; one duty, one transfer, one allocation, one successor |

The hypothetical observation authentication, custody binding and signature are assumptions for explaining preparation, not provided evidence. Their exact validation requirements remain gates before a real execution fixture is admitted. No source file can manufacture signed authority by declaring these fields.

## Derived prepared result

| Component | Complete expected proposal |
|---|---|
| Status | Prepared; not proved, submitted or accepted |
| Post-state | borrower cash 70; lender cash 30; duty map contains `debt-1` with principal 70 and status outstanding |
| Preserved duty fields | Same identity, borrower, lender, nominal asset, due time 10 and priority; no duty removed or duplicated |
| Ordered financial effects | (1) Transfer 30 `usd-fixture` from borrower cash account to lender cash account, under the custody binding. (2) Record funded principal allocation 30 to `debt-1`, linked to that transfer. No fee, interest, mint, burn, forgiveness, liability increase or other transfer effect. These two records are outputs of one allocation rule, not two independent author requests. |
| Authority successor | Same domain/principal/nonce; cumulative gross 30, fee 0, cumulative net 30; remaining gross 10, remaining fee 0, minimum-net shortfall 0, liability increase 0; expiry 10 and allowed recipient/program/claim context unchanged; residual rights may be used only under the original constraints |
| Work successor | Consumed accepted work would become 4 and remaining allowance 96 on acceptance; recovery reserve 10 is inside 96, leaving 86 ordinary-work units; no additional successor; 4 + 96 = 100 |
| Proposed consumption/history | Exclusive use of `state-0` and current `n1` residual-authority state; one successor `state-1`; accepted consumption/history advances only through real acceptance |
| Required evidence | Native proof of exact statement binding program/profile, authority digest, observations, predecessor, complete effects/state/duties and residual authority/work, plus current ledger checks; none supplied here |

The domain relation is 100 = 30 + 70 with the correct creditor receiving the funded 30. Equal numeric totals with a different creditor, unfunded transfer, different nominal asset or omitted duty fail the relation. Unchanged duties in larger cases carry forward automatically; only identified duties affected by a specified rule change. Fees, accrual, priority changes, forgiveness and restructuring require their own named relations; principal-only equality is not their general conservation theorem.

## Distinguishing cases

| Changed input or attempted output | Proposed result / reason |
|---|---|
| Transfer recipient becomes stranger | Reject at authority/domain relation; valid effect kind alone grants no recipient permission |
| Borrower cash becomes 20 while payment remains 30 | Reject unfunded allocation; do not reduce debt or stage an accepted transfer |
| Output omits `debt-1` or sets principal 0 after payment 30 | Reject duty conservation;100 ≠ 30 + 0; static detection where structurally decidable, otherwise preparation/proof check |
| Observation is duplicate, wrong provider, or time 10 when validity is t<10 | Reject under explicit observation/expiry policy; no implicit fresh clock read |
| Prior cumulative gross 15 with otherwise identical payment 30 | Reject15 + 30 > 40; a refund cannot restore gross allowance |
| A guard follows `ensures`, `post` is read early, `next` is read, or same field is written twice | Reject the specified update discipline; immutable pre-state and suffix postconditions remain required |
| Final postcondition fails after staging effects | Reject the preparation atomically; no tentative financial state/effects become accepted |
| Expiry or cancellation attempts to drop residual 70 | Reject unsupported duty deletion; timeout itself is not forgiveness |
| A distinct forgiveness action is explicitly authorized by the required creditor/controller policy | Separate specified case may record forgiveness and updated liability/history; the current payment authority does not permit it |
| An accrual or fee-bearing instrument is substituted | Current principal-only policy is insufficient; require its own allocation, authority and conservation relation rather than reusing the 100 = 30 + 70 expectation |
| Work is exhausted or a split/join tries to reset it | Reject renewal; aggregate live residual allowance plus consumed accepted work stays within original 100; recovery reserve is not extra funding |
| Second proposal uses the same predecessor | Both may prepare; real exclusive ledger acceptance must admit at most one |
| All fixture arithmetic matches but signature/proof/verifier binding is missing | Preparation illustration supplies no acceptance evidence; actual acceptance fails closed |

A rejected preparation preserves the original accepted state, duties and cumulative authority/work. This is not a promise about fees from an attempted external network submission; this research submits none. The existing [source fragment](REPORT.md#surface-syntax-and-embedding) is nonstandalone and syntax-illustrative. This fixture does not invent a currently accepted financial operation or replace source/Core/K/Compact/ledger correspondence checks.
