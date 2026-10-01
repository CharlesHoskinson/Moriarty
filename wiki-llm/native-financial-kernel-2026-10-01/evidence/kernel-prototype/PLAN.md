# Fixed beta fee transfer: bounded external plan

Consume existing public `packages/moriarty-beta/examples/signed-intent/transfer-ecdsa-wallet/{program.mori,scenario.json,signature.json}` through actual beta verify-intent and production Rust intent-build/frame. Freeze exact source/key/frame/signing-message projection; no new signing/key generation. Generate one fixed source instance, not a generic JSON parser.

Use inspected runtime0.20 little-endian scalar Bytes32 casting and Bytes slice syntax: compare signature.s high128/low128 with half-order; assert r/s nonzero and exact configured point coordinates derived from signed canonical SEC1 key. Pure checker requires exact production signing message bytes and hashes them in source. Original synthetic wrapper/high-S failure is retained separately.

One constructor-trusted ledger state holds key/address/color fixture mappings, round, head/revision/single nonce, work/allowance and all three balances. One impure entrypoint validates signature before updates, applies full fixed numerical delta/effect state and emits native escrow send claims if actual context supports them. Head successor is computed from predecessor/frame/result state; the Core opaque h1 label is an explicit local mapping, not a proven historical chain root. Round is constructor-trusted and expiry remains unverified.

Independent expected cases follow G2 vote: good low-S frame, high-S/zero/out-of-range/wrong key/frame/source/digest reject; full10000->8990/1000/10 state; consumed nonce rejects; native escrow/color/address effects inspected; no work/allowance/funding and overflow fault cases refuse without changing original context. Test is runtime evaluation, never native proof, durable ledger commit or Preview acceptance. No host accepted/owner-valid/proof-valid flag.

At most four new skip-zk compile attempts,60s each, <=2 CPU workers and total external artifact256MiB. Actual global/candidate deps unchanged. Preserve each failed source/receipt. Stop on concrete source/runtime limit. No SRS/keys/proof/network/wallet.
