# Six additional normal financial check cases

Base: 64c6dedff1d99b33c23832330d7911a5fad9ed59. Patch SHA256: `e355f2381c42e7ab3c4de5fb8ea7ce975bd084879b64d2d3e62b6a905174a92f`.

CHANGE.patch extends existing check_source_dispatch.rs and cases.json, adds six
source/stdout pairs and FINANCIAL-ORACLES.md. The original eight cases remain
unchanged. No separate helper or test target is added. Full target before/after
bodies and exact source Git-blob/SHA256 pins are retained. EXPECTATIONS.json
pins all six sources/outputs and discloses implementation exposure.

Actual captured baseline and K64c6 full outputs for the original eight cases were
read and byte-compared against the frozen Git expectations. They are historical
observations, not new executions. BASELINE-COMPARISON.json names their pins and
the concrete additional coverage. Existing financial `simulate` and `mori test`
consumers were inspected and remain separate.

Root command, unrun here:

```sh
cargo test --frozen --offline -j1 -p mori --no-default-features --test check_source_dispatch financial_ -- --test-threads=1
```

This selects only the six new `financial_` tests, each invoking JSON and quiet
human public check. Tests/compiler/runtime/provider/SDK/native/target work is
UNRUN. Metadata validation passed. New fixture validity and byte agreement await
actual review and authorized execution; no parent exit or financial qualification
is claimed.
