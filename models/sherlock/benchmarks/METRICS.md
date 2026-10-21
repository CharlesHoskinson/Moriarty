# Receipt measurements and baseline reports

`metrics.py` collects existing, explicitly selected, SHA-256 pinned JSON receipts. It has no subprocess, SDK, native, proof, wallet, RPC or retry entrypoint. DoctorWatson's benchmark acceptance consumer checks current target evidence separately. A matching digest establishes local receipt lineage and does not authenticate a producer or verify a proof.

Use Python 3.10 or newer. Ordinary tests require only the standard library:

```sh
python3 -B -m unittest discover -s models/sherlock/benchmarks/tests -p 'test_metrics.py' -v
```

All unit fixtures are synthetic public data. A passing test is suite-code evidence. It is not a benchmark observation or target acceptance.

## Collect existing observations

Select a sample template conforming to `sample.schema.json` and the additional semantics in `validate_sample`. Select a nonempty JSON list of receipt bindings. Each binding has a relative `path`, an expected `sha256` and a list of `fields`. Each field names a registered `metric`, an RFC6901 `pointer`, its normalized `unit` and optional positive `scale`, `scope` and `timer_resolution_seconds`.

```json
[
  {
    "path": "selected-public-receipt.json",
    "sha256": "REPLACE_WITH_ACTUAL_RECEIPT_DIGEST",
    "fields": [
      {
        "metric": "host.handler.wall_seconds",
        "pointer": "/operations/6/elapsed_seconds",
        "unit": "seconds",
        "scope": "subprocess"
      }
    ]
  }
]
```

This example describes the published nominal-simulation actual-results shape. Its operation index must be checked against its `id` before use. It is an example selector, not an executed benchmark. The historical MapQueue handler and MirrorArbor handler walls are SDK-free subprocess measurements. The consumer link operations are `host.consumer_link`, not complete compiler builds. Neither proves frontend/lowering/native phase coverage. A K or Quint tool's frontend wall is not Moriarty compiler frontend time.

```sh
python3 -B models/sherlock/benchmarks/metrics.py collect \
  --template selected-sample-template.json --bindings selected-bindings.json \
  --base-dir selected-public-receipts --output sample.json
python3 -B models/sherlock/benchmarks/metrics.py report \
  --samples sample.json --output baseline.json --markdown baseline.md
```

A missing or null metric pointer becomes an unavailable metric with a null value and reason. An absent metric remains absent. Wrong units, malformed JSON, duplicate keys, wrong digests, unsafe relative paths and conflicting metric assignments fail input validation. The collector reads regular files within the selected base, each at most 16 MiB, without changing operation resource limits. A receipt that exceeds this ingestion bound needs an explicit public measurement receipt; it is never re-executed automatically.

A binding may also declare `terminal: {pointer, success_values, censored_values}`. Values match with exact scalar types: boolean false is distinct from integer zero. Every other terminal value is failure. Use the actual semantic verifier result: Fenrir can report `verifies: false` and exit zero. Exit status alone cannot establish successful local verification. All necessary correctness predicates remain the independent acceptance consumer's responsibility. Known `sherlock-normal-host-run/1`, `sherlock-image-host-run/1` and `sherlock-stream-host-run/1` receipts force host evidence and retain timeout/interruption/failure outcomes. Generic receipt formats need an explicit terminal mapping when their result is consumed; caller templates are claims, not authenticated verdicts.

## Metric meaning

Time units are seconds; CPU is separately measured user-plus-system CPU only if the producer supplies that quantity. Wall durations include subprocess startup, wrappers and IO for the declared scope. `scope` identifies subprocess, process tree, single process, RPC elapsed time, circuit metadata, serialized artifact, ledger receipt, or unknown. Missing scope or clock resolution remains unknown. Peak RSS is bytes only after an explicit producer-specific conversion, such as verified KiB times 1024; no universal OS unit is assumed. The collector does not add CPU components or derive process-tree memory from independent process peaks. Record combined totals in an eligible producer receipt first, or mark unavailable. Resource values and actual measured zeroes remain distinguishable from missing data.

`compile.frontend`, `compile.lowering`, `compile.native`, `parameters`, `witness`, `keygen`, `prove` and `local_verify` have separate wall/CPU/peak-RSS slots. Combined compile timing never fills all three compiler phases. `host.handler`, `host.consumer_link` and `host.reference_execution` are explicitly host phases. `local_verify` is local cryptographic verification; network proof-ready/submission/ack/inclusion/readback durations are separate. `event_latencies` returns diagnostic UTC timestamp differences without manufacturing receipt-bound measurements. Validator CPU has no registered metric because an RPC latency does not expose it.

Per-layer metrics live in `layers[layer_id]`. Required coverage refers to them as `layer:<layer_id>:<metric_id>`. `circuit.k`, domain capacity, used rows, lookup rows and advice/fixed/instance columns are separate observed values. Missing capacity or rows are never computed from k. Proof, proving key, verifying key, instance and artifact bytes remain distinct from serialized deployment/call transaction bytes. DUST is recorded in native DUST units, without currency conversion or invented Midnight limits. Workload-specific required-metric declarations decide completeness; the registry itself does not assert availability.

The metric `source` retains the exact receipt digest, pointer, normalization scale and any measurement scope/resolution. The acceptance consumer must independently rehash the receipt and rederive the selected scalar to validate that lineage. Effects digests and the `match` claim must agree structurally; digest equality alone does not establish complete effects or independent expectations.

## Statistics and comparison

Raw samples, failed attempts, censored measurements and unavailable metrics remain in the report. Counts distinguish sample terminal states from metric availability. Successful uncensored numeric observations supply min/max/mean/median, nearest-rank p90/p95 and sample standard deviation. Standard deviation is null below two observations. Percentiles at small n are descriptive and have no tail confidence claim. Reusing one receipt/pointer/scale under another sample ID does not add an observation: duplicates are counted separately, while contradictory values from that same source fail. Absent measurements never enter a distribution as zero. Different resource scopes cannot be pooled.

Pooling requires exactly the same workload ID/digest, full source/compiler/ledger/network/circuit/profile identity, evidence class, cache state and cache identity. Optional environment/toolchain/parameter hashes participate in this identity. Warm cache requires a cache-identity digest. Record hardware/OS/architecture/concurrency in the immutable environment manifest and artifact/cache preparation in the cache manifest; hashes bind these declared manifests but do not establish their authenticity. Unknown identities cannot establish comparability.

`compare_baseline` requires known environment and toolchain digests, one profile per report, explicit tolerances and sample minima, complete uncensored observations, no duplicates/failures and comparable scope/resolution. Its upper bound is `baseline * (1 + relative_increase) + absolute_increase`. Both tolerances are mandatory, including for a zero baseline. It reports `incomplete`, `regression` or `within_declared_tolerance`; none is target acceptance.

By default the full identities must match. To compare an optimized implementation, explicitly permit only named subject fields from `source_sha256`, `compiler_sha256` and `circuit_sha256`. All workload/network/ledger/environment/toolchain/parameter/cache conditions must still match. The comparison retains both identities/profile digests and the allowed and actual changes; this policy never changes raw-sample pooling.

```sh
python3 -B models/sherlock/benchmarks/metrics.py compare \
  --baseline baseline.json --current current.json --tolerances explicit-tolerances.json \
  --allow-changed-subject-field compiler_sha256 --output comparison.json
```

For example, the tolerances file can map a metric ID to `{"statistic":"median","relative_increase":0.05,"absolute_increase":0,"min_samples":5}`. Those values are an illustrative policy only, not supplied project SLAs. No default threshold or automatic repetitions exist. Repetitions must be separately selected and charged through existing operation limits; passed setup/keygen must not be rerun to satisfy a benchmark flag.

## Sources and method

Consulted October 21, 2026 using read-only browser access to primary documentation:

- [Python statistics](https://docs.python.org/3/library/statistics.html): mean, median and sample standard deviation definitions; explicit exclusion of nonfinite observations before statistics. Percentiles here use the documented suite's nearest-rank rule rather than silently adopting Python's interpolating quantile defaults.
- [Python resource](https://docs.python.org/3/library/resource.html): user/system CPU, maximum RSS and process-versus-children accounting are distinct resource quantities with platform details. This informed explicit units and measurement scopes; the suite never invokes resource limits.

The GNU time manual was requested twice but the browser fetch timed out; no design assertion depends on that unavailable page. No repository contents, private source or secrets were sent to any search query. No historical result is promoted to a current compiler, proof, native or stagenet baseline. At least one actual deployed end-to-end qualified sample and all declared required metrics remain necessary for the separate post-deployment acceptance predicate.
