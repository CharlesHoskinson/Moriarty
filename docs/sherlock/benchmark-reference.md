# Sherlock benchmark stage reference

Status: implementation plus ordinary synthetic transport tests. No actual native,
cryptographic proof or Midnight stagenet benchmark is established by this suite.
See the [how-to](benchmark-howto.md) and [EARS](../../openspec/changes/sherlock-moriarty-benchmarks/specs/benchmarks/spec.md).

## Consumer and result

`acceptance.assess(samples, policy, current, receipt_root, baseline=None)` checks
supplied evidence consistency. `assess_files(...)` rereads bounded inputs and
receipts. Both use `metrics.required_metric_ids`, exact decimal
`receipt_metric_value`, and the shared `compare_baseline`; there is one numeric
and regression interpretation. Hashes do not authenticate proof issuers, native
verification, independent oracles or owner adoption. Existing actual native/owner
verification and every retained target requirement remain mandatory.

The existing Python checker accepts `--benchmark-policy`, `--benchmark-samples`,
`--benchmark-current`, `--benchmark-receipt-root` together and optional
`--benchmark-baseline`. Relative arguments resolve against `--repo`; benchmark
modules and source traceability are loaded from that selected checkout. A report
records input and source/schema/catalog hashes and adds actual reads to source
reference facts. The unchanged shell wrapper does not forward these options.

An evidence mismatch/incomplete stage returns checker exit 1. Missing or malformed
files, invalid schemas/units and source-trace integrity errors return input-error
exit 2. Neither closes the stage. `complete` concerns the selected stage only;
`full_goal_complete` and `performance_target_satisfied` remain false. An explicit
limit has its narrowly named `declared_limits_satisfied` outcome. Comparison
reports its shared declared-tolerance outcome without inventing a network SLA.

## Fixed stage and metric minimums

The on-disk /2 workload catalog is validated with its generator/source/oracle
provenance closure. `workloads.generate.stage_contract` supplies immutable known
case/kind/evidence/layer/root-role and phase-metric minimums. Caller policy can
strengthen coverage/repetitions; it cannot remove cases, spoof a kind or replace
native metrics with a single timer. Fully qualified `layer:<id>:<metric>` IDs
are normalized exactly once.

| Stage | Required scope |
| --- | --- |
| `host-compiler-coverage` | Actual host/compiler MapQueue and MirrorArbor frontend/lowering/handler resources and artifact bytes; terminal value oracles; no fictitious ledger receipts |
| `initial-deployed-baseline` | At least one actual deployed linked two-transition child-0/child-1/parent/decider chain, complete native/ledger effects and required phase/layer metrics |
| `full-rollup-series` | Initial deployed baseline plus actual Fenrir-derived 1/2/4/6-batch cases |
| `full-suite` | All three scopes conjunctively; every broader Sherlock requirement remains retained |

Native minimum metrics retain frontend/lowering/native compile wall/CPU/RSS,
serialized deployment/call bytes, DUST and five distinct network durations. Each
required proof layer retains witness/keygen/prove/local-verify wall/CPU/RSS,
actual k/domain capacity and proof/PK/VK/instance/artifact bytes. Used/lookup rows
and columns remain optional diagnostics when unexposed, with typed unavailable
values. No used rows are inferred from 2^k. When both k/capacity are observed their
consistency is checked without an arbitrary k cap. Cold/warm cache state and
actual measurement scope remain explicit.

## Policy and current context /2

Policy schema is `sherlock-benchmark-acceptance-policy/2`. It has `stage`,
`catalog: {path, sha256}`, exact stage `cases`, `limits`, `regression`,
`observation_fields`, `metric_context_fields`, `status_values`,
`receipt_fields`, `terminal_values`, `proof_receipt_fields` and
`statement_fields`, `command_fields` and `command_role_values`. Cases copy the real catalog's identities/layers/root roles
and required metric IDs and add positive `min_successful_samples` and explicit
nonnegative `max_failures`/`max_censored`. Native receipt schemas are adapted by
reviewed RFC6901 pointers; benchmark code does not invent a native producer format.
Terminal mappings explicitly name observed semantic values. Numeric exit zero
alone cannot mean success; local cryptographic verifier predicates must be true.

Current schema is `sherlock-benchmark-current/2`, with `cases[workload_id]` holding
its own actual `identity`, `identity_pins`, workload/oracle pins and scope.
Source/compiler/environment/toolchain digests and their artifact pins are known.
Host ledger/circuit/network may be null. Deployed cases additionally require
actual ledger/circuit/network/target/anchor. Optional parameter identity is
accepted and pinned when present. A pin is `{path, algorithm}` with `sha256` or
`canonical-json-sha256`; canonical pins may select a `pointer`. Paths stay beneath
the selected retained root. A workload pin can select the exact catalog
`/cases/N/inputs` array. Source manifests need their complete reviewed source
closure; arbitrary digest labels do not establish provenance.

Expected effect pins retain canonical complete JSON, independence and scope.
Host scope `host-terminal-value` selects the catalog's independently frozen value;
optional unavailable work metadata does not enter that value predicate. Deployed
scope `complete-deployed-effects` needs a separate independent full oracle, not
an echo of the catalog's scalar/host descriptor. Nested unavailable/unknown
sentinels cannot qualify. Legitimate null option values remain valid; profile
required roots must be present and known. `expected_effect_root_pointers` maps the
catalog's semantic root roles to real oracle fields, including all four Fenrir
pre/post tree-root roles. `expected_state_pointers: {pre, post}` selects complete
opening/closing states for terminal public-state comparison. Root roles select
separate semantic fields (values can legitimately coincide). Deployed contexts
also provide `catalog_scalar_pointers`, mapping every known frozen semantic leaf
to its complete-oracle pointer. Linked semantics include literal/program, both
ordered transitions and known work; Fenrir semantics include balances, batch
count, ordered transaction IDs, known state counters and totals. Typed unavailable
root descriptors are excluded from this scalar projection and still cannot supply
complete roots. `transition_state_pointers` selects ordered full pre/post states;
`state_scalar_fields` maps frozen state scalar paths to actual state-view paths.
Linked control/fuel transitions and each Fenrir corpus batch's known scalar
pre/post state must agree with the validated catalog. Each leaf statement binds
those actual full states, including additional preserved context.

## Sample acceptance extension /2

`sample.extensions.acceptance.schema` is
`sherlock-benchmark-sample-acceptance/2`. `observation_id` and `observation` bind
actual producer identity/workload/cache/terminal state. A role binding is exactly
`{receipt_sha256, pointer}`. All retained successful, failed and censored samples
receive context, file hashes, exact decimal value and receipt-context checks.
Required measured metrics additionally come from qualified role/observation
receipts. Local verification, proving, different layer timers and used/capacity
rows cannot share one unrelated scalar pointer. Measured network durations must
match their receipt-bound event timestamps; RPC latency supplies no validator CPU.

Host samples use `receipt_roles.host`, binding source/compiler/profile/workload,
Halted outcome and exact independently frozen terminal value. Deployed samples
retain target/anchor/deployment/call and final proof/statement/VK identities plus
prove/local_verify/deployment/inclusion/readback role bindings.

Each deployed `proof_nodes` entry has `node_id`, `layer`, `operation_id`, actual
`circuit_sha256`, `circuit_k`, proof/statement/VK artifact digests, pre/post complete
state digests, ordered `inputs` and prove/local_verify receipt bindings. Those
native receipts must bind the complete node/subject fields and an actual positive
cryptographic verification verdict. `command_fields` maps `execution_id` and
`mode` in actual command results. Disjoint `command_role_values.prove` and
`.local_verify` values and distinct execution IDs separate prover and verifier
observations. A single JSON bundle can hold both at distinct command-result
pointers; the same result cannot satisfy both roles. These consistency checks do
not authenticate commands or replace actual native verification.
Nonempty artifact bytes are rehashed. Statement
JSON selects complete states through `statement_fields`; binary statements need
an existing native decoded `statement_view` receipt binding its artifact digest.
State views are compared as complete JSON, including fields and list order.

Sherlock requires exactly both children, their parent and terminal decider. Fenrir
requires the declared ordered batch leaves and an actual parent/decider graph.
Every predecessor proof/statement/VK/state reference is matched, all leaves are
continuous, parent/decider boundaries match and the terminal graph consumes every
transition/batch exactly once in order. Final artifacts and independent complete
opening/closing states must match the deployed terminal proof.

Layer metric sources include witness/keygen/prove/local-verify, circuit metadata
and every required artifact size. A multi-node layer requires
`layer_observations[layer]`, binding a real aggregate receipt and the exact ordered
node references through `layer_receipt_fields`. Its `aggregation` adapter selects
a recipe for every measured layer metric. Every recipe names all members in
order and each original member scalar source, unit/scale and command attribution.
All sources are re-read with the shared exact decimal normalizer.

Wall recipes use `mode: observed_parallel_span`, `scope: all-member-command-span`,
aggregate `start`/`end` pointers, and contributor `start`/`end` pointers relative
to each actual command result. Every member duration equals its observed interval;
aggregate endpoints equal the earliest member start and latest member end. This
allows parallel spans and never sums elapsed wall time. CPU uses `sum` with
`cumulative-member-subprocess-cpu`; RSS uses `maximum` with
`maximum-member-subprocess-peak-rss`, which claims only the largest member peak,
not simultaneous process-tree memory. Circuit metadata uses `common`.
Serialized member byte metrics use `sum`/`serialized-member-artifacts`; VK bytes
use `unique-verifying-keys`. Proof/instance/VK counts also agree with rehashed
artifact byte lengths. Qualified aggregation mode/scope signatures must match
across historical comparisons. Aggregate records cannot replace individual
positive native verifications. Setup/key artifacts may remain retained and
reused; the consumer never dispatches setup.

Raw duplicate records remain visible but cannot supply additional distinct
successful observations or failure attempts. Reused prove/verify/call/readback
observations cannot satisfy extra repetitions. Declared failures/censoring remain
counted and bounded. Only qualified unique successes enter regression statistics;
a renamed wrapper that reuses prove/verify/call operations stays descriptive.
Available failed/censored scalar sources must belong to retained actual observer,
role, node or layer records with matching producer context. Successful proof or
ledger artifacts are not required for a partial failure. Censored observations
bind an actual nonempty censoring reason through `observation_fields.censoring_reason`.
The raw and deduplicated counts are separate.

JSON and receipt reads use the shared 16MiB ingestion bound. Binary artifact reads
have `ingestion_limits.max_artifact_bytes`, default 64MiB. These are supplied-data
reader bounds, separate from native execution limits and protocol/performance
claims; a different retained-artifact size needs an explicit reader configuration,
never a native budget reset.

## Historical comparison and traceability

An initial baseline declares `regression.mode: initial_baseline` and explicit
`future_tolerances`. Comparison uses `mode: compare`, explicit per-case shared
statistic/relative/absolute/min-sample rules and allowed subject-field changes.
The baseline input is `{schema: sherlock-benchmark-baseline/2, root_prefix,
samples, policy, current}`. Its safe relative root contains its own frozen
catalog/identity/oracle/source artifacts and receipts. It is validated against
that historical cut before the shared comparison checks unchanged environment,
toolchain, ledger/network/workload/cache/scope/resolution and allowed compiler/
source/circuit changes. Retained failures/censoring do not masquerade as successful
statistics; duplicates cannot provide replicas, and current observations cannot
be their own baseline. The selected case repetition minimum applies inside each
compared cache/environment profile; a tolerance cannot weaken it. No unselected
cold/warm profile is invented. `comparison_scope` lists all mandatory measurements,
the explicitly selected tolerance metrics, and measurements without a declared
regression tolerance. Full measurement coverage remains required; passing a subset
of tolerances asserts only that subset, never a universal performance target.
Zero baselines have explicit absolute tolerance semantics.

`traceability.json` /2 pins the cross-lane implementation/schema/catalog/generator,
tests, documentation and workload closure, maps all SBENCH clauses and is checked
by the selected Python consumer. Source/schema/catalog digests accompany the
actual report. Pins establish currentness only; actual independent review and
existing owner adoption remain separate. The original 35 clauses, 25 refinements
and 68 scenarios retain their fixed catalogs and every larger full-goal exit.

Documentation separation follows [Diataxis reference](https://diataxis.fr/reference/)
and [how-to guidance](https://diataxis.fr/how-to-guides/), actually accessed
2026-10-21; no external protocol or performance threshold is borrowed.
