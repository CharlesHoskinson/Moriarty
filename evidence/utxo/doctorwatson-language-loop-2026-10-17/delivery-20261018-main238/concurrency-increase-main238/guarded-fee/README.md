The unchanged guarded-fee positive regression is prepared for immutable main `71c22f8a477231be869786eb42f04e3b50d1b3a7`. `guarded-quantity-fee-positive.patch` adds exactly three currently absent files: the full source/case under candidate/crates/fixtures/guarded-quantity-fee-positive and one candidate/crates/mori-core/tests/guarded_quantity_fee_positive.rs. No production behavior change is proposed; root alone integrates.

Source SHA256 `f49897116563631543bed4c38fa832c0214370072418f232fda238477e21240b`; selected case/expectations SHA256 `2dd5be3610affa0e3aea0ef35022df777e8c8a052b83f956edd7e54286e7a7ce`; test SHA256 `521d8d229dcba8af74d956d4fb0834a54836752c2cbd9c536e04f8359c1ed46c`; patch SHA256 `4fa2b5c0d2f4981a5eb08ef91bec700ff15729795fd05c005526cd3b56bff89b`. All are byte-identical to guarded-fee-positive-main79-20261018. REFRESH.json supplies exact prior/current hashes and pinned main71 Git blobs.

Actual main71 parser/evaluator source was inspected. The parser change since main79 only adjusts language-profile error messages; existing valid /1 expressions/statements remain unchanged. The runtime, schema, exported APIs, cost implementation, Core Cargo and CLI route are unchanged. The fixture uses supported moriarty/1 → moriarty-core/1 guarded UInt arithmetic: first21>=second6, quantity15, payment15*3=45, fee2+6=8, buyer debit53, closing duty14+45−11=48. Cash264 and Reserve26 totals are conserved; Reserve duty17, present-zero Cash duty and marker91 survive. This local discharge input does not authenticate external delivery/payment. Maximum/minimum are guarded local names, with no primitive, beta Quantity or compiler/lowerer coverage claim.

Original freeze SHA256 remains `797f0aac6e33914f4a99f62776de1198cfa3990fa94db735113ea2da79ef3c6a`. The selected case is exactly its literal varied-quantity-price-fee case; other preparatory cases remain unused. Same-author prior/current parser/runtime/test-source exposure is explicitly retained in REFRESH.json. The arithmetic was not derived from subject execution, but is not a fresh independent oracle. No evaluator or captured outcome was read here.

After integrating the patch, root can run from the checkout:

```sh
cargo test -p mori-core --locked --no-default-features --test guarded_quantity_fee_positive -- --exact guarded_quantity_arithmetic_keeps_fee_effects_and_residual_duties --test-threads=1
```

The single test calls real exported parse/simulate/simulate_metered, comparing frozen full pre/post and nine ordered effects,18 typed step values, preserved fields at each step and existing logical-cost/1 used50 within64. Metered/unmetered agreement is internal only. Full prefix snapshots, Statement/ExprId/whole-wire/CLI bytes, opposite-order cases and general max/min behavior remain unasserted.

Offline hash/JSON/arithmetic/patch reconstruction checks passed. No parser/evaluator/Cargo/CLI/provider/native/SDK command ran; fresh applicable review and actual main71 results remain pending. Startup skill/AGENTS/product contract/status were refreshed; historical blocked SP01 was not dispatched or changed. All original financial/roadmap/family/held-out/intent/regression/native/proof/target obligations remain intact. No security judgment is supplied.
