# S0 independent expected outcomes

**Status:** Sprint 0 proposed fixtures. These are arithmetic and semantic expectations, not executed K, Quint, evaluator or ledger results. Stable wire codes and precedence remain W-D3 decisions. Each hostile case must keep all unrelated premises valid so it reaches the named judgment.

## Positive transfers and repayment

| Case | Pre-state and signed terms | Expected complete effect and state |
| --- | --- | --- |
| T-10-1 | Owner balance and allowance at least 11 A, recipient R and fee recipient F distinct, `v=10`, `f=1`, caps at least 11 gross and 1 fee, net floor at most 10. | Ordered debit O 11 A, credit R 10 A, credit F 1 A. Allowance remaining falls 11 and spent rises 11. Replay ID consumed once. One head successor. No duty. |
| R-30 | Existing obligation P1000/I10/outstanding1010, bound creditor C, payer balance and allowance at least 30, `n=30`, identity conversion. | Ordered debit payer 30 and credit C 30. Principal 980, accrued 0, outstanding 980, status Outstanding. Allowance remaining falls 30 and spent rises 30. Replay and head advance once. |
| R-near-bound | Let `U=2^128−1`, `S=2^127−1`. P=`S−1`, I=1, outstanding=S, `n=1`; creditor balance `U−1`, allowance remaining 1 and spent `U−1`. | Creditor and allowance spent become U, principal remains `S−1`, accrued becomes 0, outstanding becomes `S−1`. Other required cells stay within their bounds. |

## Hostile controls

| Case | One changed fact | Proposed first judgment | Required result |
| --- | --- | --- | --- |
| H-recipient | Submit R′ in place of the signed R with an otherwise valid envelope. | intent | No commit or effects. The signature must bind R. |
| H-fee-cap | Submit `f=feeCap+1` with enough balance and allowance. | intent | No commit or effects. |
| H-missing-fee | Omit the F credit while retaining a valid signed `f=1`. | effect | No commit or effects despite matching net owner delta. |
| H-missing-credit | Reduce obligation by 30 but omit the bound C credit. | effect | No debt discharge or published effects. |
| H-wrong-creditor | Credit X instead of obligation-bound C and keep conservation. | effect | No debt discharge or published effects. |
| H-allowance | Set allowance remaining to gross debit minus one, with enough balance. | authority | No commit or effects. Netting a self-transfer cannot satisfy the cap. |
| H-stale-head | Supply a signed old pre-head after another stage advances current head. | history | No commit or effects. |
| H-replay | Resubmit the same signed intent ID at the current head. | history | No second commit. A distinct later partial repayment ID remains possible. |
| H-overflow | Make one receiver balance `U`, then request a positive credit. | effect | Checked overflow rejection, no commit. |
| H-nominal | Supply a source nominal amount `S+1` while the existing debt and balances are otherwise UInt128-valid and sufficient. | source admission | Named nominal-range rejection before a K stage. Do not claim all lifecycle state fields have the S wire cap. |
| H-overpay | Submit `n>outstanding` with sufficient payer funds. | effect | No debt or balance change. |
| H-both | Omit the fee credit and provide a stale head. | effect | First rejection reports the omitted effect, not the later history failure, if W-D3 keeps the proposed order. |
| H-range-and-vector | Make the recipient balance `U` and omit the fee credit in one otherwise valid transfer. | effect/range | Numeric overflow precedes submitted-vector mismatch within Effect. No commit. |
| H-overpay-and-vector | Submit `n>outstanding` and omit the creditor credit, with sufficient payer funds. | effect/range | Overpayment precedes submitted-vector mismatch within Effect. No commit. |
| H-cell-binding | Omit or swap a required authenticated endpoint cell, or change its asset or allowance owner. | stage | Typed pre-state shape rejects before Effect. No commit. |
| H-debtor-binding | Bind the authenticated obligation to a debtor different from the signed payer. | stage | Typed obligation shape rejects before Authority. No commit. |
| H-conversion | Give a direct typed repayment a nonidentity conversion. | intent | The S0 signed action is outside the admitted identity-only scope. Source/6 cannot form this action. |
| H-scope-and-alias | Use a direct typed transfer alias and an invalid validity round or signed cap together. | intent/scope | Invalid signed scope precedes the direct-Core alias diagnosis. Source/6 rejects transfer aliases at formation. |

Zero-fee line representation, endpoint aliasing, absent receiver cells, within-judgment code order and the full S0 failure observation remain open normative W-D3 leaves. The prototype's choices and fixed witnesses do not close that decision. A Core semantic rejection reports judgment, code and one abstract diagnostic-work unit and publishes no post-state. A Source/6 formation rejection is outside the Core judgment sequence. An accepted signed phase failure is outside S0.
