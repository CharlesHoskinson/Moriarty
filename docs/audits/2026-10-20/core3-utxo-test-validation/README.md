# Core/3 test compilation and static checks

This records local checks of source commit `cd9caf15fbaffb71951fb8f343f3a4a5813756d1`, carried unchanged by evidence head `7a4f0102af7b510fcfcc7a585c8c076dccf11a57`. This companion adds evidence only. The earlier [library typecheck](../core3-utxo-integration/README.md) remains a separate result.

All 18 Core test executables compiled in 33.64 seconds with the already warmed library-typecheck target. The [frozen command and source/tool pins](compile-plan.json), [result](compile-result.json), [Cargo output](compile-stdout.jsonl) and [stderr](compile-stderr.txt) retain the capture. Cargo emitted 115 artifact events, with no SDK compilation units. Six unused/dead-code warnings remain. This was compilation, without test execution; it is not a cold-build measurement.

Harness enumeration matched all 355 expected test names across the 18 executables; [the record](enumeration-result.json) retains every listed name. Enumeration executed no test functions.

The subsequent [static-test selection](static-test-plan.json) chose exactly 11 `validate_v3` functions and one frontend/profile/serialization function. [All 12 passed](static-test-result.json): the [validator output](static-validator-stdout.txt) reports 11 passed and 198 filtered; the [wire output](static-wire-stdout.txt) reports one passed and three filtered. Both invocations exited zero, with empty stderr, unchanged 694 source/tool/executable pins, EOF and no remaining owned processes. These checks exercised parsing, static validation, profile routing and legacy JSON handling. They did not invoke stage start/resume, interpreter execution, SDK, keys, proofs, a wallet or a ledger.

There are 343 listed functions still unrun on this candidate. The predecessor's 118 passes do not transfer. CLI linking, broader Core/1 and Core/2 regressions, Core/3 execution/public-consumer/positive-prefix checks and financial execution remain open. Passing these existing assertions supplies no independent source, result or security approval. The original F3 qualification, compiler/proof correspondence, authenticated composition, Cardano/Midnight target acceptance and parent EARS/milestones remain open.

Absolute paths identify the local capture. No compiled executable or private provider capture is included. Local CI only; no hosted checks were added.
