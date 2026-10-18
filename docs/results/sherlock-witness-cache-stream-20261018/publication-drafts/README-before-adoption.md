# Witness cache and stream: ordinary results, 2026-10-18

Root executed five selected ordinary operations once. Both compiles exited0; each of three separate runs selected and passed exactly one test, with two filtered out. The original source/proposal UNRUN records remain historical. Actual-result review is being collected separately; this staging receipt does not invent its disposition.

| Operation | Exit | Elapsed seconds |
| --- | ---: | ---: |
| 01-compileNormalCoreRlib | 0 | 4.331410013997811 |
| 02-compileWitnessCacheStreamConsumer | 0 | 0.39660815900424495 |
| 03-factorial_three_cold_warm_last_use_and_stopped_stream | 0 | 1.1735098010030924 |
| 04-list_two_five_cold_warm_last_use_and_stream | 0 | 1.0670753800004604 |
| 05-tree_four_one_three_cold_warm_last_use_and_stream | 0 | 2.28482142300345 |

All481 selected static inputs and four generated outputs matched their pins through the run and current data readback. All ten streams were retained completely. The Core compiler emitted11 warnings (5520B stderr), preserved without edits; consumer compile stderr and all test stderr were empty. Core dep-info reports90 source members and consumer dep-info one; these are source-membership observations, not exclusive-read or SDK attestations.

The tests exercise real fixed-owner boundaries for factorial3, list[2,5] and tree4,(1,3), final literals6/7/8, complete decoded equality across uncached/cold/warm/last-use variants, logical cache/read/live-out diagnostics and factorial same-image stop/resume. The authored differential tests do not supply independently bound complete B semantic-prefix/work expectations. B-to-A boundary/helper-phase and clock mapping, UIREQ-035, SCON-013/024, H1 physical costs, native proofs and target settlement remain open. Elapsed command times are observations, not a performance comparison.

Use the [API reference](../../sherlock/reference/witness-cache-stream.md) and [streaming guide](../../sherlock/how-to/stream-owner-witnesses.md). Raw [Root result](actual/attempt-01/RESULT.json), per-phase streams/commands/checks, full source snapshot, source-review receipts and pins are immutable copies. Historical runtime absolute paths remain inside original receipts; COPY-MANIFEST.json maps them to this public archive. Tool/dependency binaries and generated rlib/test executable are not distributed here; their exact observed pins remain in the result.

Canonical integration base is the GitHub `sherlock` branch at Root-reported d39488f…, not `main`; backend-only6b26bd93 is separate. Product integration and final result-review adoption are Root actions. This archive grants no semantic acceptance from matching files/hashes/test counts.
