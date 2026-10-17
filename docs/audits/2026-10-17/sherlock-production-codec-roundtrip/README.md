# Production codec: actual external roundtrip

The unchanged [external example](../../../../crates/mori-core/examples/sherlock_codec.rs)
compiled, linked against a fresh production Core library, and ran successfully
once on its fixed trusted input. Core compilation exited 0 in 2.775 seconds,
example linking exited 0 in 0.216 seconds, and execution exited 0 in 0.003 seconds.
Total recorded time was 2.995 seconds under the existing 30-second aggregate
limit. Compilers emitted 7 Core warnings and 1 example warning, with 0 errors.
Warning-summary lines are not extra warnings. Runtime stderr was empty.

Actual stdout was:

```text
canonical roundtrips=2 program_bytes=86 initial_bytes=103 after_bytes=103
```

The real example calls `prepare_a`, takes initial and first-step observations
from the existing A `Machine`, then uses `encode_observed_a`, canonical
`decode_observed_a` and `observed_state_bytes_a`. It compares the complete decoded
Snapshots with the supplied actual Snapshots and serializes the ProgramBlock.
The printed byte lengths are observations, not independent expected values.
This establishes actual external production-API roundtrip agreement for these
two observations, extending the earlier metadata-only result.

The [result summary](result.json) binds the unchanged example and actual records.
Normalized dep-info matched exactly 57 production Core sources and 1 example
source, with no extra or missing paths. Source, compiler and selected dependency
postpins were unchanged. This was a direct SDK-free build/run without Cargo or
`cfg(test)`; no test suite was run and no retry occurred.

The [trusted diagnostic contract](../sherlock-production-codec-api/README.md)
still applies: unmodified PreparedA, trusted encoder-produced StateBlocks, and
caller-trusted capacities. This internal agreement shares preparation and codec
logic; it is not independent semantic validation. It did not run the new literal
producer, construct a native Step witness, or demonstrate native row equality.
No hostile/untrusted-language, full interpreter, financial, formal/circuit,
SDK/native/proof/ledger/stagenet or settlement acceptance follows. Cached lexer,
transitive and procedural-macro source correspondence and compiler/sysroot/
linker/runtime trust remain qualified. This result supplies no source-security
recertification or parent gate closure.
