# Independent FeePool cases

`expected.json` freezes 162 unique cases: 87 accepted and 75 rejected. Amounts come from the frozen independent arithmetic review, including surplus 0, 1, 7, 99, 101 and U64max. Expected snapshots and ordered effects were prepared independently of execution output. The source author supplied statement order; this does not determine the expected money.

The root `mori test examples/core/fee-pool` manifest contains 76 cases (funded intake plus the 75 refusals). Each accepted case also has a bounded `cases/NAME/mori-tests.json` manifest. Run all cases through the public CLI with:

```bash
cargo +1.99.0 --offline --locked test -p mori --test fee_pool_component -j2 -- --test-threads=1
```

The integration test runs every accepted manifest, the root manifest, and a separate actual-poststate chain through all 23 primary actions. Overlapping intake checks are not additional unique cases. Every accepted case compares the entire poststate, exact asset and party identities, liabilities and ordered effects. An unrelated balance of 42 and unrelated duty of 11 survive the primary lifecycle. All nine primary duties have a positive creation and an actual transfer/discharge claim; the final pool balance is the held watcher reserve of 2.

The original aggregate 123-case CLI run evaluated all 123 cases successfully, then exited 1 because serializing all full traces exceeded the existing 1 MiB JSON output bound. These bounded manifests respect that bound. The aggregate report limitation remains unresolved; no trace was truncated and no generic bound was increased.

The independently specified 50 review obligations are covered or constrained as follows:

| Independent obligation | Executable finite profile disposition |
| --- | --- |
| valid_complete | Replaced by the permitted two-operator/one-DA roster; exact positive all-nine-duty lifecycle and independent 111 vector. The review's two-DA roster is outside this finite width. |
| design_vector | Exact gross111/cost10/zeroDA/target17 cases; backstop17 and dust9. |
| floor99, floor7, floor1, zero_gross | Exact accepted split and empty-roster finish cases. |
| zero_surplus | Gross100/cost100 retains claimable reimbursement100 after finish. |
| meter_exceeds_gross, zero_meter, meter_replay | Cost112 against111, zero amount and duplicate fixed receipt slot refuse. Global cross-epoch receipt hashing is outside this one-epoch source. |
| header_replay, epoch_replay | Repeated intake and extra header/epoch inputs refuse. Cryptographic global header identity is an external premise. |
| settle_replay, post_settle_meter, post_settle_seat | Duplicate settlement and late submissions refuse. |
| allocation_replay, finish_early | Duplicate allocation and premature finish refuse. |
| zero_weight_seats, no_reward_seats | Zero-seat allocations remain mandatory; zero duty exists but cannot claim. Empty roster finishes immediately. |
| watcher_not_reward | No watcher reward action exists; substituted role input refuses. Watcher reserve has its own kind7/8 spend and claim. |
| nonattested, operator_cap | Unattested86400 becomes zero; unattested86401 refuses. Forged effective weight refuses. |
| da_max_weight | Full U64max DA weight is accepted and allocated against an original U64max weight. One DA slot cannot overflow a second aggregate addition. |
| quota_wrong_q, quota_wrong_r, quota_noncanonical, quota_d0 | Exact quotient, remainder, remainder-range and zero-denominator refusals. |
| quota_full_u128_product, quota_extreme_nondiv | Full-U64 real FeePool weighted multiplication is exercised, with product above U64; arbitrary n=w=M and n=w=M-1 are generic arithmetic recipes, outside FeePool's category fraction range. No helper result is substituted for a real source consumer. |
| quota_high_operand, quota_high_q, quota_high_r | Above-U64 state and input operands refuse before products. |
| watcher_overflow | Held U64max plus a new positive reserve share refuses atomically. |
| target_lowered, target_zero | An injected local predecessor with backstop20 above target17 checks eager arithmetic safety; it is not claimed reachable from clean genesis. The actual reachable target15-to0 lowering retains funds and sends dust11 to L2. Target0 baseline sends L2floor15 and dust1. |
| wrong_cost_recipient, stipend_identity | Ordinary recipient and stipend watcher are source literals. Forged duty creditor/debtor and replacement claim input refuse. Audit/insurance receipt kinds and arbitrary watcher registration are outside these two fixed receipt slots. |
| stipend_duplicate_recipient, stipend_limit | Duplicate recipient stipend slot and amount above3 refuse. |
| wrong_stream_asset | Changing each exact identity component (network/issuer/reference/scale) refuses. Undeclared unrelated assets also refuse. |
| wrong_beneficiary, wrong_pool_epoch_role_seat | Fixed duties and accounts; forged creditor/debtor and added destination/pool/epoch/role inputs refuse. These shape checks do not authenticate a recipient secret. |
| claim_before_settle, reward_early_claim, reward_boundary_claim | Early cost settlement/close and early operator/DA day30 refuse. Boundary positive claims are exercised by full-range DA and supplementary claim scenarios; the main full lifecycle uses late claims. |
| late_claim, double_claim | All nine positive claims succeed after finish; every repeated claim refuses. |
| watcher_spend | Independent positive kind7 and kind8 spend3/claim3 leave held2; zero/kind3/kind9/overdraw/repeated slot refuse. Two sequential spend receipts are outside one fixed slot. |
| claim_adapter_credit_overflow | U128 recipient overflow and insufficient pool custody refuse with no execution/effects published. |
| native_u64_time_overflow | Above-U64 epoch close and reward hold overflow refuse in the actual source. |

Public clock, importer, governance and claimant observations remain premises. This is a finite source component, not proof of authenticated import, secret authorization, global registry equivalence, native custody, or full M11 conformance.
