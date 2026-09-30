# MIL/4 S0 finite local comparison

**Status:** finite prototype evidence. The first repaired packet received independent GPT-6.1 Sol high and Grok 4.7 xhigh reviews: both accepted its three common positives as a narrow local prototype and rejected W-D0–W-D4/Sprint 1 closure. A later S1B head/failure/premise packet was rejected as an aligned local comparison; [post-audit S1B repairs](S1B-FINITE-RESULT.md) have new bytes and need fresh review. These runs do not prove universal refinement, signature validity, native qualification or ledger admission.

## Three common positive fixtures

K's `corpus/common_run.py`, Quint's `corpus/s0_common_witnesses.qnt`, and the Source/6 test file each execute T-10-1, R-30 and R-near-bound with the same numeric pre-state, work budget 1, round 0, signed caps and complete expected effects. Each fixes expected output as literals rather than deriving it from the model under test. K's strict parser compares the **complete** accepted pre-state, ordered effects, post-state, terminal phase, empty duty and remaining work. Quint asserts all model state maps, including absence of receiver/creditor allowance rows. TypeScript asserts the complete prepared effect vector and candidate post-state.

| Fixture | Exact common numeric pre-state | Expected complete financial post-state |
| --- | --- | --- |
| T-10-1 | Owner 100, recipient 0, fee recipient 0, owner allowance 11/0, work 1/0; value 10, fee 1, gross cap 11, fee cap 1, net floor 0 | Ordered debit 11, credits 10 and 1; balances 89/10/1; allowance 0/11; work 0/1; one replay and head advance; no obligation or receiver allowances |
| R-30 | Payer 100, creditor 0, allowance 100/0, work 1/0; obligation principal 1000, accrued 10, outstanding 1010; payment 30 | Ordered debit and bound-creditor credit 30, SetObligation 980/0/980 Outstanding; balances 70/30; allowance 70/30; work 0/1; one replay and head advance; no creditor allowance |
| R-near-bound | Payer 1, creditor U−1, allowance 1/(U−1), work 1/0; obligation (S−1)/1/S; payment 1, where U=2^128−1 and S=2^127−1 | Creditor and allowance spent reach U; payer 0; principal/outstanding S−1, accrued 0; work 0/1; one replay and head advance |

The local mapping is explicit: K domain `D` corresponds to Source/6 `Midnight` and Quint `D`; K `O/R/F` corresponds to Source/6 `Owner/Recipient/Fee` for transfer. K repayment `P/C/L` corresponds to Source/6 `Payer/Creditor/Loan` and Quint `O/C/L`. The older, non-common Quint repayment witness uses `loan`; it is not part of this map. K/Source heads `h0→h1` correspond to Quint `0→1`. Source/6 nonce `n1` corresponds to the K and Quint fixture nonce named by the case. The replay key is compared after this identifier map as `(domain,signer,nonce)`. K and Quint effect lines omit the selected asset on some constructors; this comparison supplies the single signed asset `A` as context. These are local fixture bijections, not a `/3` signed-byte mapping.

This is a field-by-field **manual** normalization of separately asserted literal outputs. There is no general executable `α_local` adapter or ledger-committed `α` proof. The common fixtures do establish the specified complete local financial pre/effect/post observations under the map above; they do not establish exact Source/6 metadata binding in K or Quint.

## Hostile observations and boundaries

The repaired K regression corpus checks 37 complete outputs, including transfer cell identity/asset/allowance Stage shape, obligation debtor/creditor Stage shape, validity/cap-plus-alias Intent order, numeric-range-plus-vector Effect order, stale/replay History order, and the original single-fault controls. The first frozen Quint corpus had 35 witnesses and the TypeScript corpus had 25 targeted checks. Several original Quint witnesses asserted judgment only. The current S1B Quint witnesses now assert exact codes, and the current TypeScript targeted suite has 28 tests. The named overlap has finite first-code evidence; no universal diagnostic theorem follows. Core rejections report one **abstract** diagnostic-work unit and no published post/effects; Source/6 formation is outside the Core judgment sequence.

H-nominal remains a Source/6 admission rejection, outside the K and Quint stage models. Source/6 rejects transfer endpoint aliases and self-creditor repayment during formation; direct typed Core/K/Quint malformed-stage experiments have a separately stated judgment. The current Source/6 wrapper returns only `PreparedUnqualified` on success. S1B K and Quint receive stipulated tuples binding intent, state, round, expected successor and requested outcome. The TypeScript direct-Core path can compare such a tuple but still returns `PreparedUnqualified`. None of these stipulations verifies the corresponding external fact.

Important correspondence leaves remain open: K and Quint do not carry all Source/6 agreement, selected-program, source-hash, asset-scale and predecessor bindings; the provisional `/3` codec has no consumer that binds its digest to signature, proof and complete effects; no model proves the stipulated successor extends an authenticated head. S1B aligns named local head/failure/premise codes, but no general executable projection exists. The Quint Rust evaluator cannot represent its UInt128 maximum literal, so the fixed witnesses run on the TypeScript backend. No `quint verify` model check or native/ledger execution has run.

## Reproduction and audit state

- TypeScript: targeted Source/6 suite **28/28**, `npm run typecheck` exit 0, package suite **938/938** after the post-audit S1B test alignment. The current retained log and hash binding are `audits/s1b-h9-assert-npm-test.log` and `audits/s1b-h9-assert-ts-verification.json`; earlier `s1b-postreview-*` files are historical.
- K 7.1.337 LLVM: clean compile, original strict corpus **37/37**, expanded S1B H/F/P **20/20**, and round-zero common positives **3/3**. Exact commands, requests, outputs and exits are under `formal/k/mil4/corpus/`.
- Quint 0.32.0: all six current `.qnt` files typecheck; **83/83** fixed witnesses pass with `--backend typescript --seed 0x5` after the Stage guard repair. The earlier **59/59** result is historical for its recorded hashes. Exact commands and hashes are under `formal/quint/mil4/corpus/`.
- The two earlier frozen packet reviews and their dissent remain in `audits/`. Both new K/Quint/TypeScript source and evidence bytes require a fresh full-candidate review before broadening the S1B local claim.

A local financial correspondence claim must stay confined to the three common positives and the stated finite S1B overlap. W-D1/W-D2 signature/wire binding, W-D3 external authentication and complete code policy, W-D4 narrowing, and the guarded SP01.6 delivery remain open.
