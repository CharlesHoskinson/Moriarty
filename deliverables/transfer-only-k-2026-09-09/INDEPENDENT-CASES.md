# Independent Transfer-only expectations

Authored on 2026-09-09 by the delegated independent fixture author. The fixture is `experiments/moriarty-language/formal/k/fixtures/transfer-only.json`: exactly ten new single-Transfer cases and six unchanged regression records.

**Provenance:** The new financial expectations were derived from `experiments/moriarty-language/spec/successor/repayment-kernel.md` (invariants, work, Transfer, atomic rejection, and stable errors) and `spec/successor/funded-source.md` under the same experiment (Transfer-only source behavior). The author read existing `formal/k/fixtures/cases.json` and `branches.json` for the record format and explicitly retained regression cases. The initial fixture is named `cases.json`; there is no `initial.json` in this checkout. No K semantics, codec, evaluator, observed K output, or runtime result was used to derive new expected values. No K run or result audit was performed by this author.

**Specified expectation:** Every new input has exactly two balances, one allowance, one obligation, empty input transfer/allocation histories, identity conversion (mantissa 1, scale 0, rounding none), and one Transfer. The obligation denomination is `Nominal`, its settlement asset is `Cash`, and its creditor is `Creditor`. This distinct denomination is preserved, not coerced. AccrualFirst and PrincipalFirst both appear. Expected Prepared records contain the complete post-state and one complete Transfer effect. Expected rejections contain exactly status, code, and actionIndex, with no tentative state or effects.

The ordinary starting quantities are sender cash 83, recipient cash 12, allowance remaining 67 / spent 5, debt principal 37 / accrued 8 / outstanding 45, and work remaining 1 / spent 6 / reserve 23. The Transfer is 19. These values are independently selected; they are not copied from a repayment result.

| New case ID | Hand-derived expected result |
| --- | --- |
| `transfer-only-one-work-debt-untouched` | Cash 83−19=64 and 12+19=31. Allowance 67−19=48, spent 5+19=24. Work 1−1=0, spent 6+1=7, reserve 23 unchanged. Debt remains 37/8/45; allocation IDs remain empty. Append OnlyTransfer and emit only the Transfer. |
| `transfer-only-missing-recipient-append` | Cash Debtor 64, Custodian 29 unchanged, then append Creditor 19. Allowance 48/24. Work 7−1=6, spent 7, reserve 23. Debt and allocation history unchanged. Existing balance order is preserved. |
| `transfer-only-swapped-third-party-non-creditor` | Merchant occupies row 0 and Sponsor row 1. Sponsor differs from Debtor; Merchant differs from Creditor. Row 0 becomes 12+19=31, row 1 becomes 83−19=64; Sponsor allowance becomes 48/24. Work becomes 0/7/23. PrincipalFirst debt remains 37/8/45. Transfer alone has no payer-to-debtor or recipient-to-creditor constraint. |
| `transfer-only-settled-debt-untouched` | Same cash, allowance and work arithmetic as the first case. PrincipalFirst obligation remains Settled with principal/accrued/outstanding all zero. An unrelated settled debt cannot invalidate cash movement or generate repayment effects. |
| `transfer-only-reserve-not-work` | Remaining work 0 is less than one action, despite reserve 23. INSUFFICIENT_WORK, null index; no execution result. |
| `transfer-only-zero-amount` | Work and invariants are valid, but Transfer amount is zero. ZERO_AMOUNT at index 0. |
| `transfer-only-receiver-overflow` | Recipient cash is UInt128 maximum M=340282366920938463463374607431768211455. M+19 exceeds M. OVERFLOW at index 0. Sender cash and allowance cover the amount. |
| `transfer-only-invariant-before-work` | Principal 37 plus accrued 8 is 45, but supplied outstanding is 44. Also remaining work is zero. Admission INVARIANT at null index takes precedence over insufficient work. |
| `transfer-only-missing-exact-allowance` | The sole allowance belongs to Sponsor, while the sender is Debtor. Sender cash covers 19. MISSING_ALLOWANCE at index 0. This preserves the one-allowance input shape while removing the required exact key. |
| `transfer-only-insufficient-balance` | Sender cash 18 is less than Transfer 19; allowance 67 is sufficient. INSUFFICIENT_BALANCE at index 0. |

All four successes append exactly `OnlyTransfer` to the empty transfer history. All obligation identity, denomination, settlement asset, allocation rule, conversion, and status fields remain as supplied. Ordinary work costs one action, and closure reserve is unchanged. The first success at exactly one unit distinguishes this action list from Transfer-plus-Repay, which costs two units.

The following regression entries are copied as exact raw JSON object substrings, including their input and expected bytes. Their financial expectations are retained provenance, not newly independent derivations.

| Source | Retained ID | Purpose |
| --- | --- | --- |
| `cases.json` | `principal-partial` | Transfer-plus-Repay partial success remains covered. |
| `cases.json` | `principal-first` | PrincipalFirst repayment success remains covered. |
| `cases.json` | `wrong-transfer-id` | Repay rejects TRANSFER_NOT_IN_STEP at index 1; no partial output. |
| `branches.json` | `missing-receiver-append` | Repayment success preserves an unrelated row and appends the recipient. |
| `branches.json` | `reject-recipient-mismatch` | Repay rejects TRANSFER_MISMATCH at index 1, unlike the new legal non-creditor Transfer-only success. |
| `branches.json` | `reject-duplicate-before-work` | Admission DUPLICATE at null index retains precedence over insufficient work. |

**Repository observations:** JSON parsing and structural checks confirmed 16 unique case IDs, ten single-action inputs with the required 2/1/1 collection shape, unchanged complete obligations and empty allocation IDs in all four new successes, and rejection-only shapes. Raw JSON object substring comparison confirmed all six regression entries were copied without byte changes. These are fixture-integrity checks, not semantic execution evidence.

SHA-256 at author completion:

- `formal/k/fixtures/cases.json`: `d46679961af39a10adec03cb86b4481a39fc9393d15ceb48b37b10b8ce28f832`
- `formal/k/fixtures/branches.json`: `2a9596d52e934533629354a7ecdbd60282f192efcacd3ef9ec7a95ff6d0845dd`
- `formal/k/fixtures/transfer-only.json`: `2d5120c5eb7ed451d1bd23fd29d4a793673139e2642c0b3777bfea2bd63dfa75`

These fixture paths are relative to `experiments/moriarty-language`.

**Limits:** These are specified-only finite expectations. This work does not establish K execution, source/Core/K correspondence, a general semantic freeze, authentication, signing, proof validity, ledger acceptance, or network settlement. It does not cover arbitrary collections, nonempty input histories, multiple Transfer-only actions, other conversion profiles, or ProRata. The six inherited repayment regressions remain within their original scope. A separate implementation runner and independent result reviewers must evaluate the candidate against these fixed records.
