# Financial pair v2: canonical balance order correction

Status: proposed source and independent expected-envelope preparation only. All Moriarty source parsing, interpretation, compilation, consumer tests, native/proof and target execution remain **UNRUN**. This revision supplies no successful runtime result or U2 walkthrough.

The immutable original packet is [financial-pair](../financial-pair/README.md). Its inventory `FREEZE.sha256` has SHA-256 `951ade3116836076a83e8658c9370bd364b4b78e62f1743fa3b7b14ae3807fdc`; its original payloads and inventory were checked unchanged during this revision. The v2 directory began as a copy of all ten original files. It adds this revision record, updates the three reserve wire representations and README ordering statement, and has a fresh inventory.

The ordinary review `doctorwatson-financial-pair-ordinary-review-20261017-a1-whole-review` returned changes-required. Its complete safe record is [AGREED-SCHEMA-FIELDS.json](../../../../../evidence/utxo/doctorwatson-language-loop-2026-10-17/financial-pair-consumer/reviews/ordinary-a1/AGREED-SCHEMA-FIELDS.json). Direct source inspection confirms `crates/mori-core/src/runtime.rs:631` sorts Snapshot balances by `(&a.asset, &a.account)`. `AssetId` derives ordering from `(network, issuer, reference, scale)` in `crates/mori-core/src/ir.rs:104`. In these fixtures both assets share network and issuer, and `BOND` sorts before `CASH`. The previously frozen reserve expectation instead grouped by account first. That is an expected-wire-order defect found statically; no evaluator output exists or was used.

The additional source inspection pins `crates/mori-core/src/runtime.rs` to SHA-256 `7c736b2f817c3dc095df27d96744369c3357c758267f9838564c8f9e583ac27d`. The public `ir.rs` hash remains `0f5c27000c7236fab11d2b9e6c8b38c48909433ee0dd41ae06114ddef6c2960d`. These schema facts determine row order only; they do not supply expected financial numbers.

The old-to-new positional permutation is `[0,1,3,2,4,5,6]`, applied simultaneously to balance columns and cells:

| New position | Exact account/asset | Old position |
| --- | --- | --- |
| 0 | auditor/BOND | 0 |
| 1 | observer/BOND | 1 |
| 2 | reserve/BOND | 3 |
| 3 | observer/CASH | 2 |
| 4 | reserve/CASH | 4 |
| 5 | supplier/CASH | 5 |
| 6 | treasury/CASH | 6 |

Changes are confined to `reserve-approved.scenario.json` balances, the reserve case's complete `expected.post.balances` in `mori-tests.json`, and reserve `balance_columns`, `pre.balances` and every `prefixes[*].balances` in `financial-and-work.json`. The README now states the exact `(asset, account)` wire order. Both `.mori` source files, both invoice Scenario files and `refusal-expectations.json` remain byte-identical. No source clause, numerical expected value, ordered effect, liability presence/value, state value, gross/fee/net calculation, reserve floor, dust, logical work or refusal expectation changed. Invoice cost is42 per call/84 total; reserve cost remains45.

A read-only Python standard-library JSON comparison reported `STATIC FIXTURE DATA CHECK: PASS`. It verified the original inventory digest and every original payload digest, the five unchanged payloads byte-for-byte, and exact full-object equality of each revised JSON file to its original after only the stated permutation. It checked equality of every account/asset/value mapping in reserve pre and all fifteen complete prefixes, and strict `(AssetId, account)` order in the literal reserve Scenario and expected post arrays. All other parsed fields were identical. This was static fixture-data validation; it invoked no Moriarty parser, evaluator, compiler or test consumer and generated no expected number from implementation output.

The actual consumer must retain exact array equality. Do not sort actual/result arrays, loosen comparison to a subset or silently normalize a differing execution to fit this oracle. After this packet is frozen and announced, the author may update its consumed path/digests and request fresh exact-candidate review. Review or source integration alone cannot become a runtime pass.

The original unresolved fields and product obligations remain: raw ExprId/serialized statement identities, `Step.value` Some/None convention, canonical program and artifact/ABI/relation/statement IDs, authenticated predecessor/history/time/budget, metered resource diagnostic bytes, Core/2 durable stages and full composition, complete U2 compiler/artifact/proof/target consumption, external/native work and all financial-family/release exits. The current pair still measures only its independently specified Core/1 source/reference envelope. The separate atomic guard does not fulfill durable waiting, and the numerical budget57 is not an authenticated balance binding.

The fresh `FREEZE.sha256` lists every v2 payload including this record and excludes itself. Root and the consumer author must receive its exact digest and file list before consuming v2. The original packet remains a preserved failed source-oracle candidate; it is not retroactively relabeled correct.
