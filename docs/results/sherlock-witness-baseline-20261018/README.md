# Witness baseline: observed ordinary tests, October 18, 2026

Root installed the exact eight source/test/fixture files after the [R339 result
and promotion review](reviews/R339/FINAL-RESPONSE.txt). The [installation record](integration/R339/ROOT-ACTUAL-EIGHT-FILE-INTEGRATION.json)
records eight verified installed hashes and seven live source-reference hash
replacements, with no new EARS closure. The separate M-01/M-02 ledger factual
update is outside this archive's adoption claim.

The [normal Core compile](evidence/R326/actual-attempt-01/RESULT.json) passed in
3.833853 seconds with 696 unchanged inputs. All 4,421 stderr bytes/nine warnings
are retained, without classifying their novelty. Its exact command is in
`evidence/R326/actual-attempt-01/ROOT-SELECTION.json`. That original record's
counter note was mistaken; the separate [counter correction](evidence/R326/actual-attempt-01/ROOT-COUNTER-NOTE-CORRECTION.json)
agrees with the frozen source and preserves the original. No source change or
compiler rerun followed that correction.

The [consumer result](evidence/R335/actual-attempt-01/RESULT.json) passed three
serial phases: compile, exact two-test listing and the two listed tests once.
The authoritative input count is **726**, unchanged before execution and after
each phase. Compilation emitted no diagnostics. Full six streams, exact argv,
list, dep-info membership and generated-artifact pins are preserved. The tests
are `actual_owner_records_materialize_all_four_normal_domains` and
`ordinary_shared_value_and_shadowed_environment_decode_exactly`.

Evidence covers the assertions inside those two reviewed tests: real owner
records through the uncached witness DAG/materializer/decoder, plus declared
ordinary shared-value and shadowed-environment decoder inputs. The latter are
explicit raw diagnostic inputs, not an authentic stepped owner image. Raw
libtest stdout contains pass labels, **not complete snapshot/prefix dumps**.
No independent exact B clock/full-prefix comparison, cold/warm cache, last-use
reclamation, streaming, physical measurement, SDK/native/proof/target result or
requirement acceptance closure is established. UIREQ-035, SCON-013 and its R02
remain open.

[Independent provenance](source/R321/revision02/EXPECTATION-FREEZE.json) and the
[additive reconciliation](source/R321/revision02/ADDITIVE-RECONCILIATION.json)
preserve the declaration-first freeze and withdrawal of the provisional raw
frame-order assumption. Historical SOURCE_ONLY/UNRUN/no-test-source fields retain
their dated meaning. The unconsumed historical A-prefix oracle is pin-referenced,
not copied or represented as a B comparison.

The [public consumer](../../../crates/mori-core/tests/sherlock_witness_baseline_independent.rs),
[configuration](../../../crates/mori-core/tests/fixtures/witness-baseline-independent/CONFIG.json)
and [decoder expectations](../../../crates/mori-core/tests/fixtures/witness-baseline-independent/NORMAL-DECODING-EXPECTATIONS.json)
already exist in source; their bodies are not duplicated here. All eight current
public source identities appear in `BOUND-SOURCE-AND-FIXTURES.json`.

Full original R327/R335/R339 responses and actual identity/result records remain
under `reviews/`, including qualifications. Actual returned model was
`gemini-4-argon-high` in the same independent context under the standing Opus
outage fallback; backend canonical attestation is not supplied. Absolute public
runtime paths in historical records remain verbatim. This archive omits binaries,
rlibs, dependency payloads, provider inputs/transcripts and private material.

The later [R340 factual update](reviews/R340/FINAL-RESPONSE.txt) records the
implemented baseline and observed tests in UIREQ-035/SCON-013 while preserving
OPEN acceptance. The reader initially found two historical citations pointing
to the changed live file; that [negative result](prior-negative/CURRENTNESS-RESULT.json)
is preserved. [R343](reviews/R343/FINAL-RESPONSE.txt) approved pointing those two
citations to the exact pre-witness source snapshot, keeping their old hash, span
and claim. The [result after that correction](CURRENTNESS-RESULT.json) is current
with eleven adopted originals and no findings. No interpreter test was rerun
to repair these metadata references.
