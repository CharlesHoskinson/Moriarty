# Application draft source notes

This contribution consists of `CASES.md`, `CASE-DISPLAY-DATA.json` and these source notes. It changes no active application, repository file, Git state, proof, SDK, wallet or ledger. The copy and diagrams are editorial proposals for root integration after the reader cohort finishes. No independent security or cryptographic judgment is made. Quotes assigning proof/consumer responsibility retain their existing source status; quotations from the Opus errata are attributed source-design statements.

## Sources and claim locations

Repository-relative paths below resolve against `/home/charl/Moriarty/.worktrees/utxo`. The cited inputs were read as public source material, without accessing provider-private logs, stderr or private records.

| ID | Public source | Locations used |
| --- | --- | --- |
| A01 | `docs/webs/editorial-2026-10-17/COPY.json` | `/main/caseCopy` entries `dex`, `issuance`, `loan`, `treasury`; exact claim and authentication wording. This is proposed editorial copy, not observed application behavior. |
| A02 | `docs/webs/reference-inputs/concept/integrated-r2/CONCEPT.md` | “How an application would use a Web,” “A first reference app that teaches the model,” “Five application cases.” All four application sections remain PROPOSED. |
| A03 | `docs/webs/reference-inputs/concept/integrated-r2/DEVELOPER-EXPERIENCE.md` | Journey A steps 3–7; Journey B; Journey C. Explains whole-execution proof coverage, missing mint/burn lifecycle, authorization/consumer boundaries and uncompiled-source status. |
| A04 | `docs/webs/contributions/04-scenarios/scenarios.json` | Entries with IDs `dex`, `issuance`, `loan`, `treasury`; `effects`, `dependencies`, `strands[*].limits`, `sourceRefs`. These are proposed scenarios and illustrative future proof roles, not concrete Strands. |
| A05 | `docs/webs/contributions/04-scenarios/NOTES.md` | CL-22–CL-25; exact attributed Opus ERR-08, ERR-09, ERR-21, ERR-23, ERR-24. Includes the original public errata paths and pointers. No new interpretation of those errata is supplied here. |
| A06 | `docs/webs/reference-inputs/concept/sources/S03-MORIARTY-PRODUCT-CONTRACT.md` | “What provable intention means”: complete per-asset effects and typed liability evolution; debt is not token supply. Frozen source says target architecture/specified-only where not linked to implementation. |
| A07 | `/home/charl/moriarty-eutxo-validation-2026-10-12/moriarty-webs-ten-study-20261016/sources/S13-conditional-reserve.mori` | Exact public source referenced by A04: `allocate` and `rebate`. This original was read directly because the docs/webs vendored concept source directory contains S03 but no S13 `.mori` file. No source was executed. |
| A08 | `/home/charl/webs-reader-round-20261017/jobs/H07/REVIEW.json` | Public model-reader feedback on loan depth: supplied accrual lacks its rate/period and principal-versus-interest allocation. Reader requests are feedback, not authoritative financial terms to invent. |
| A09 | Public `REVIEW.json` files for H01, H03 and H09 in `/home/charl/webs-reader-round-20261017/jobs/` | Issuance, treasury and transaction-cost comprehension feedback. H02/REVIEW.json was absent at read time. These are model-reader reports, not human studies, application evidence or security audits. |
| A10 | `docs/webs/app/src/data.generated.json` | Existing `scenarios` entries were read to inspect the active consumer data. It carries the same proposed scenario semantics as A04. No change made. |

The checkout-level `docs/MORIARTY-PRODUCT-CONTRACT.md` requested by the Moriarty skill is absent. A06 is the existing frozen public copy used for this bounded editorial task; no recovery or execution work was dispatched. Guarded status was read and preserved as historical operational state.

## Exact source details that add substantive depth

S13 directly states:

```moriarty
requires approved and not allocated;
requires remainder < 3;
requires units == quotient * 3 + remainder;
requires units >= 3 and quotient not = 0;
let toAlpha = quotient * 2;
let toBeta = quotient;
```

Its allocation body separately calls `fee(A, "vault", "fees", 2)`, transfers 4 B to alpha, creates `allocation_reserve` in A using `remainder`, and creates `rebate` in B with amount 6. Thus the fixed fee and remainder-based duty are distinct. The 4 B transfer is a literal, not another result of the quotient calculation. The state starts with allocated false, rounds zero and marker 77; allocation sets allocated true and increments rounds. `rebate(amount)` transfers B to owner and discharges the named rebate. These are source facts and arithmetic deductions, not execution observations or invented ledger datums.

The DEX source list supplies no pool reserves, pricing function or fee worksheet. The 92 B figures are illustrative owner results. Both CASES and display data leave pool balances unknown instead of reverse-engineering a pool from those results.

The loan source list supplies no program, accrual convention, term structure or payment waterfall. The draft therefore treats 100 and 70 as total duties and 5 as a supplied accrual premise. The 35 repayment/2 fee split is shown directly; no principal or accrued-interest fields are invented.

The issuance sources require application mint-policy enforcement and supply continuation but supply no concrete asset policy, authority matrix or supply datum. The 300/1,000/700 display is application accounting. The 700 capacity annotation is not a payment output, a minted asset or a redemption duty.

## Diagram integration contract

`CASE-DISPLAY-DATA.json` is an array of four objects with exactly `id`, `title`, `inputs`, `outputs`, `checks`, `note`. Each input/output has `id`, `label`, `kind`, `lines`, `checks`; each check has `id`, `label`, `explanation`. `checks` on nodes contains IDs from that case's check list. `outputs` includes financial continuations and annotations as well as payments; it is not a Cardano transaction-output schema.

Use the specific topology in each `note` rather than drawing all conditions into one Web oval:

1. DEX: two order-to-owner lanes share a pool-before/pool-after lane. Preserve the two independent 90 B floors and 92 B results; pool balances remain unspecified.
2. Issuance: a new-supply branch distributes 300 into 180 and 120, alongside a cap/supply continuation. Capacity is an annotation outside the payment branch.
3. Loan: an obligation line runs 100 → 105 → 70; a separate cash fork runs 37 → 35 + 2. Only the 35 repayment connects to duty discharge.
4. Treasury: separate A/B custody rows fan out to beneficiary and fee payments, then align the 2 A / 6 B retained custody with the 2 A / 6 B duty rows. Those duties are not extra transfers.

The figures do not establish a proof count. The three check IDs in each case describe required conditions. The sources permit one whole-execution instance to cover several specifications; actual relation/proof decomposition is unresolved. No concrete datum schema, signer matrix, UTxO reference, executed transaction, pool formula, mint authority or measured cost should be added during rendering without its own source.

## Input digests at drafting

| ID | SHA-256 |
| --- | --- |
| A01 | `dfbe53e6b3a8507e6f25101b97a05bba9cc3fa26eb3c29b39a206a1e95d78bcf` |
| A02 | `0ccbaf78dca6beec390fa86941f60e0b141b0ad941137d7af4e3440e5f85ebf1` |
| A03 | `a7fc3997b3ab4747cdaaa42fff141f01b5e74dad31575446b6d30d356a5a8eb5` |
| A04 | `b28ec317dca09b5ca47617af0aacb6ab0c07147a0d75749f6d1cdc36117cad57` |
| A05 | `2645de296c9c797540277cb9c11f42284ed3ce732d2419bd13a75545f39e64db` |
| A06 | `d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396` |
| A07 | `395e647534160fde45325232e5f1650a8bd48ce1455a68b28af203bf8bdbe357` |
| A10 | `78e8ad2320872de779901d06c9c2fd96bf64e48d03eff09d802ddf28504a7b4f` |

Validation is limited to JSON structure/reference integrity, independent integer reconciliation and source/copy inspection. No proof, native, SDK, wallet, ledger, Git or browser action was performed. Root's complete-candidate review and integration checks remain separate.
