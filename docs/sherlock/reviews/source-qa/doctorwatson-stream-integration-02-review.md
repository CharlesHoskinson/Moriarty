# DW2 integration-02 additive source review

**Verdict: APPROVE_SOURCE_WITH_SCOPE. Material corrections: none. Ready for Root's exact source integration of the reviewed DW2/DW3 composition, subject to the separate DW3 review.** The previously reported missing-helper integration gap is resolved in these exact additive bytes. The original DW2 review and verdict remain unchanged.

This review covers `07-doctorwatson-local-qa/integration-02/{README.md,SOURCE-MANIFEST.json,integration.patch}`, both complete changed afterimages, the complete new regression, the actual composed checker/helper dependency, and retained red/green results/logs. No candidate/compiler/test/model/native/provider operation was run. This is an ordinary source review, not a security vote or execution authority.

Exact final identities:

| Path | SHA256 |
|---|---|
| `models/sherlock/qa/run_host.py` | `8c537d5470f385ad88668732db96fb1a14d51344d18e99a0a818ffd493aa5109` |
| `models/sherlock/qa/test_run_stream.py` | `f08cdec6f0e15cb7f4f382758a98c4b063ac49c99d1ded4b23a4644427cc1484` |
| `SOURCE-MANIFEST.json` | `d9afde6862231555fb415c3df808e555ae333d03e9b90a01922bd074172c008c` |
| `integration.patch` | `bc18c05b8ee46708fabe32b92bec94cb9665f710728853a41bce49863e0801d7` |

I independently rehashed both actual original DW2 preimages and final afterimages and reconstructed the exact patch in memory. All 35 composed input pins and all three recorded DW3 pins matched actual staged bytes. The stream producer remains `724dd9b40b4f51ae70a1441a5beb693346fc795355a30cb6d6983012da70c921`. Oracle files were hashed without inspecting expected values.

The only wrapper change is CHECK_HELPERS membership: `host_observations.py` plus `source_reference_facts.py`. The actual DW3 checker has exactly these two literal `with_name` loads. COMMON_PATHS includes CHECK_HELPERS, and each host/image/stream source set includes COMMON_PATHS; both helpers therefore enter the copied snapshot for all three realizations. This matches the unchanged `comparison_helpers` equality check instead of bypassing it. Producer/test Rust compile closures remain eight/nine for stream; the stream attempted source set increases from 25 to 26. No compiler, guard, timeout, failure ordering, receipt, resource-counter, source-clock or evaluator body changes.

The added regression is substantive. It creates an executable-shaped plain-text compiler with the required directory shape, constructs the actual stream `Attempt`, and invokes real `Attempt.preflight` while mocking `Attempt.child` to fail if called. It then asserts helper receipt membership and byte equality between source and snapshot. It loads `check.py` from the snapshot, calls the actual `checker.run(snapshot)`, checks the source-facts result remains explicit source evidence, and loads the real host adapter from that snapshot. Thus it tests the real consumer that previously failed, rather than merely checking a constant list. The existing four wiring tests remain.

Retained red output records five tests with one preflight error: undeclared `models/sherlock/qa/source_reference_facts.py`; exit 1. Retained green output records all 40 selected narrow Python tests passing, exit 0, 0.675 seconds. I read these captured bytes; I did not reproduce them. The green selection `test_run*.py` does not execute DW3's separate `test_source_reference_facts.py` suite, and this review does not claim that it does. Separate DW3 review/evidence remains its own scope.

The full Rust streaming compile, eight actual Rust tests, producer execution and independent eight-program/216-run comparison remain UNRUN. Shell stream selection remains separately deferred to Root. This additive approval changes no native admission, resource limits, security judgments or U/M/MC/SP/G acceptance requirement.
