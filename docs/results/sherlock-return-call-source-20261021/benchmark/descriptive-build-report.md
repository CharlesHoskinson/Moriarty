# Sherlock benchmark baseline

Baseline only; performance targets and target acceptance are not assessed.

Distributions use successful uncensored measurements. Raw observations, failures and unavailable metrics remain in the JSON report.

Percentiles use nearest rank; a single observation is not evidence of a stable tail.

## ledger9-backend-libtest-build — native, unknown

Comparable profile: `d1e3507731d60ebc69b9f40569661135e956e8cce8899ad89dd8f5843cae16e0`. Counts: {'total': 1, 'success': 1, 'failure': 0, 'censored': 0, 'not_run': 0}.

| Metric | Unit | Scope | n | Duplicates | Missing | Unavailable | Censored | Excluded terminal | Median | p95 |
| --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| artifact.bytes | bytes | serialized_artifact | 1 | 0 | 0 | 0 | 0 | 0 | 642171320 | 642171320 |
| circuit.advice_columns | columns | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| circuit.domain_capacity_rows | rows | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| circuit.fixed_columns | columns | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| circuit.instance_columns | columns | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| circuit.k | exponent | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| circuit.lookup_rows | rows | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| circuit.used_rows | rows | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.frontend.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.frontend.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.frontend.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.lowering.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.lowering.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.lowering.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.native.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.native.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| compile.native.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| fee.dust | DUST | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.consumer_link.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.consumer_link.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.consumer_link.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.handler.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.handler.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.handler.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.reference_execution.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.reference_execution.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| host.reference_execution.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| instance.bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| keygen.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| keygen.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| keygen.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| local_verify.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| local_verify.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| local_verify.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| network.ack_to_inclusion_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| network.inclusion_to_readback_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| network.proof_ready_to_submission_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| network.submission_to_ack_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| network.submission_to_readback_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| parameters.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| parameters.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| parameters.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| proof.bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| prove.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| prove.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| prove.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| proving_key.bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| transaction.call.bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| transaction.deployment.bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| verifying_key.bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| witness.cpu_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| witness.peak_rss_bytes | bytes | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |
| witness.wall_seconds | seconds | unknown | 0 | 0 | 0 | 1 | 0 | 0 | unavailable | unavailable |

