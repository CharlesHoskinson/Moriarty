# Sherlock benchmark acceptance reference

Status: source implementation and ordinary parser tests. Actual native/stagenet
benchmark observations and target acceptance remain absent. See the
[how-to](benchmark-howto.md), [experiment](stagenet-recursive-rollup-experiment.md)
and [benchmark EARS](../../openspec/changes/sherlock-moriarty-benchmarks/specs/benchmarks/spec.md).

`models/sherlock/benchmarks/acceptance.py` exposes
`assess(samples, policy, current, receipt_root, baseline_samples=())` and
`assess_files(policy_path, samples_path, current_path, receipt_root, baseline_path=None)`.
The latter rereads selected inputs and receipts on every invocation. Neither API
executes programs or authenticates receipt issuers. A complete result means
supplied evidence passes the reviewed profile contract and is ready for existing
owner adoption; semantic review and retained native/target requirements remain.

The existing `models/sherlock/qa/check.py` accepts four options together:
`--benchmark-policy`, `--benchmark-samples`, `--benchmark-current`, and
`--benchmark-receipt-root`. `--benchmark-baseline` selects prior observations.
Relative selected paths are resolved against `--repo`. An incomplete recomputed
result sets the checker status to `mismatch` and exit code 1. Missing/malformed
inputs use the existing input-error exit 2. With no benchmark selection the
existing ordinary corpus and fixed requirement inventories keep their behavior.
The shell wrapper does not forward these additional options; select the existing
Python checker directly. No shell/native guard changes are part of this feature.

## Policy

`schema` is `sherlock-benchmark-acceptance-policy/1`. `profile` and `network` must
exactly match current context. `cases` is a nonempty unique array. Each case has:

| Field | Meaning |
| --- | --- |
| `workload_id`, `workload_digest_sha256` | Exact versioned catalog member |
| `kind` | `moriarty-nominal-source`, `sherlock-recursive-interpreter`, or `fenrir-derived-public-scalar` |
| `required_metrics` | Nonempty predeclared registry IDs for the whole observation |
| `required_layers` | Separate layer IDs; Sherlock requires `child-0`, `child-1`, `parent`, `decider`; Fenrir requires `leaf`, `parent`, `decider` |
| `required_layer_metrics` | Optional explicit per-layer registry IDs; defaults to required whole metrics |
| `min_successful_samples` | Explicit positive repetition count, selected within existing operation limits |
| `max_failures`, `max_censored` | Explicit nonnegative limits; all failed/censored/not-run observations remain visible |

`limits` maps metric IDs (including `layer:<id>:<metric>`) to `{max, unit}`.
An empty map asserts no unknown performance SLA. A Fenrir k19/64KB limit is not
applicable to Midnight merely because a workload was derived from Fenrir.

`regression.mode` is `initial_baseline` or `compare`. Initial baselines require
an explicit `future_tolerances` disposition and report performance without a target
satisfaction claim unless an applicable explicit limit was actually supplied.
Comparison requires `tolerances: {metric_id: {max_ratio: number}}` and prior
qualified samples in the exact `metrics.comparable_profile` group. Each current
observation is compared with that group's raw-observation arithmetic mean under
the selected ratio. A current observation cannot be its own prior baseline.
Changed source/compiler/ledger/circuit/network/cache/workload identity forms a
new exact group; the tool refuses to manufacture comparability. Counts and raw
observations must accompany any performance conclusion. For a single sample,
report a single observation, not a distribution/percentile claim.

## Current context and receipt semantics

`schema` is `sherlock-benchmark-current/1`. `identity` contains non-null
`source_sha256`, `compiler_sha256`, `ledger_sha256`, `network`,
`circuit_sha256`, and `profile`. `target_id` and `anchor_id` identify the actual
deployed target. `identity_pins` maps each SHA256 identity key to
`{path, algorithm}`. The path is beneath the selected receipt root; the algorithm
is `sha256` for exact bytes or `canonical-json-sha256` for complete parsed JSON.
These are real source/compiler/ledger/circuit artifacts or complete frozen source
manifests, not free-standing digest labels. Review must establish their complete
closure and that they describe the currently selected producer/target.

`current.cases[workload_id]` has `workload_digest_sha256`,
`expected_effects_sha256`, `workload_pin`, and `expected_effects_pin` using the
same path/algorithm structure. It also requires
`expected_effects_independence: independent` and
`expected_effects_scope: complete-deployed-effects`. The canonical digest retains
every field and list order. A nominal terminal-value oracle, correlated upstream
root, scalar sketch or source manifest alone is not a complete deployed-effect
oracle. The workload catalog distinguishes file-byte hashes from
`expected_effects_canonical_sha256`; copy the appropriate identity deliberately.

Each sample follows `sample.schema.json` and the metrics API. Its
`extensions.acceptance` object contains `target_id`, `anchor_id`, `deployment_id`,
`call_transaction_id`, `proof_sha256`, `statement_sha256`,
`proof_artifact_receipt`, and `receipt_roles`. The proof artifact receipt digest
must equal the actual nonempty proof bytes. Receipt roles are `prove`,
`local_verify`, `deployment`, `inclusion`, and `readback`, each mapped to a retained
receipt SHA256. Successful repeated observations require distinct actual
prove/verify/inclusion/readback receipts and call IDs. Shared setup/key artifacts
may remain shared; no repeated setup is requested.

`policy.receipt_fields[role]` maps semantic field names to RFC6901 pointers in the
existing actual receipt schema. `acceptance.ROLE_FIELDS` is the authoritative
required membership. Every role binds source/compiler/ledger/network/circuit,
profile and workload. Proof and verify roles bind proof/statement and an actual
`success` verdict. Deployment binds target/anchor/deployment. Inclusion additionally
binds the call/proof/statement. Readback binds the call and complete `effects`.
Success pointers must select the producer's terminal semantic result: for a
verifier this is the actual verification verdict, never shell exit zero. No
benchmark schema relabels a prospective command as an executed receipt.

Every sample receipt is read and rehashed; every measured metric is recomputed
from its retained receipt pointer and explicit scale (default 1). Required values
must be measured with exact registry units. Separate layer metrics cannot reuse
the whole-run or another layer's metric pointer as their own observation. Full
proof-ready/submission/ack/inclusion/readback timestamps are required. Local
cryptographic verification is distinct from RPC queue/inclusion/readback latency;
validator CPU is never inferred from those durations. Subprocess wall/CPU/RSS
measurements retain their scope and resolution and do not claim kernel timings.
Null/unavailable/censored data are retained and cannot fill required coverage.

## Requirement evidence

Benchmark EARS are additive `SBENCH-*` clauses. The existing declaration checker
keeps its 35/25/68 catalogs unchanged. Its `retainedInventory` can point to the
benchmark specification; affected existing rows can pin policy/current/report and
complete source/receipt references in their currentness/evidence fields. Pins
check currentness, not reviewer authenticity. Re-run the selected benchmark
completion check whenever dependent bytes change before an owner adopts closure.

The reference/how-to split follows the primary Diataxis guidance, accessed
2026-10-21: [reference](https://diataxis.fr/reference/) describes interfaces;
[how-to](https://diataxis.fr/how-to-guides/) serves the evidence-assessment task.
