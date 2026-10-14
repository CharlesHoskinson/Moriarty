# Public metered owner-intent reference vectors

`public-vectors.json` contains fixed public messages, keys and signatures for
Core owner-intent/2. Both integration test suites share these inputs. Runtime
tests verify signatures and do not generate keys or sign messages.

The independent pre-implementation expectation record supplied the exact source,
scenario, authority and hand-derived costs: Counter 16, Payment 4 and
TwoAssetDuty 18. Its SHA-256 was
`f4384a3968a7e09796717b5e1ce7bb591e71f779c629ef539a3540f96bcd0a59`.
The `oracle` fields copy its expected closing state, balances and duties;
variants alter the explicitly tested input or authority limit.

A temporary k256-only reference generator materialized deterministic BIP340 and
low-S ECDSA public outputs with the repository's already-public QA test constant.
The generator was removed after capture. High-S signatures replace public
signature component s with n-s. These vectors establish local reference checks,
not native proof validity, account ownership, nonce consumption or settlement.
