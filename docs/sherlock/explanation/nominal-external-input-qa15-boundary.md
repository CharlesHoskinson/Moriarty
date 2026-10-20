# Why this QA result uses two runtime cuts

The first compiled consumer ran 15 tests. Fourteen passed. Test 12 stopped at an incorrectly null StepCap locator, so its genesis and layout assertions had not executed. The designated owner corrected only that expectation after tracing the public guard and wire functions frozen before execution. Fresh nonauthor source/API review preceded the new compile, list and exact test 12 run.

That successful new run covers corrected StepCap and the downstream genesis/layout checks. The other 14 test bodies stayed byte-identical, so their actual QA4 passes were retained without replay. Source correction, observed result and current disposition remain distinct. The earlier compile failure and missed source/API assumptions remain visible.

These tests consume real production APIs and source-known fixtures. Shared-producer equivalence is not an independent semantic oracle. A passing error-projection test does not prove universal guard correctness or whole-process resource bounds. Nothing here establishes the full backend CLI, native proofs, target settlement or original EARS closure. See the [scoped result](../../results/sherlock-nominal-external-input-qa15-20261020/README.md).
