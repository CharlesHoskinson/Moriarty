# Witness cache and owner-record streaming

Public module: `mori_core::sherlock::helper_machine::witness_plan`.
The retained uncached functions remain available. These optional APIs reconstruct
complete observations from saved owner boundaries; they do not evaluate source
expressions or replace the owner transition engine.

## Evaluate a saved snapshot

| API | Result and behavior |
| --- | --- |
| `evaluate_last_use(plan, input)` | `Result<StrategyEvaluation, WitnessFailure>`; release scratch values after their final scheduled consumer. |
| `evaluate_memoized(plan, input, cache, storage)` | Same result; use a mutable `WitnessCache` and `SlotStorage::{RetainAll, LastUse}`. |
| `evaluate_with_storage(plan, input, storage, cache)` | Same result; `cache: Option<&mut WitnessCache>` selects optional memoization. |
| `materialize_with_storage(input, lifetime, storage, cache)` | `Result<StrategyMaterializedSnapshot, WitnessFailure>` containing the raw snapshot, constructed plan and evaluation. |

Arguments `plan` and `input` are borrowed `&WitnessPlan` and `&HostSnapshot`.
`LifetimeUnit` distinguishes `Snapshot`, `RecordBefore` and `RecordAfter`.
`StrategyEvaluation.witness` retains the complete `DecodedSnapshot` and executed
`HostDiagnostics`; `scheduling` carries separate `StrategyDiagnostics`.

`WitnessCache::default()` starts empty. `clear()` discards its scope and entries;
`value_slots()` and `argument_slots()` count retained top-level memo cells.
A cache keeps one graph and one latest argument/result entry per non-Read node.
Its scope compares the format, lifetime, complete `AdmissionSnapshot`, exact nodes
and binding descriptors. Hits additionally compare every complete typed argument
value by equality. All Read nodes still execute the existing immutable-input,
type and location checks, including on warm evaluations. Validation runs on every
call. This is optional host storage, with no new admission or numeric budget.

`LastUse` frees non-live-out scratch slots after the existing definition's final
scheduled use. Declared boundary live-outs remain through publication. Cache
copies, raw snapshots, plans and anything retained by a sink are separate storage.
A caller may choose `RetainAll` or omit the cache without changing the decoded
boundary representation.

## Record and stream the actual owner

```rust
pub fn record_step_with_storage(
    admitted: &Admitted,
    image: &mut MachineImage,
    storage: SlotStorage,
    cache: Option<&mut WitnessRecordCache>,
) -> Result<StrategyObservedStep, StrategyStepError>;

pub fn stream_owner_steps<F>(
    admitted: &Admitted,
    image: &mut MachineImage,
    cache: Option<&mut WitnessRecordCache>,
    sink: F,
) -> Result<StreamStop, StrategyStepError>
where F: FnMut(&StrategyObservedStep) -> StreamAction;
```

Signatures omit parameter `mut`. `StrategyObservedStep` contains the original
`ObservedStep` plus complete materialized `before` and `after` boundaries.
`WitnessRecordCache` keeps separate before/after caches and supports `clear()`.

The recorder calls existing `complete_observation::record_step` exactly once.
Materialization then consumes that step's saved boundaries. `StrategyStepError::Owner`
preserves its `CaptureStepError`, which distinguishes before-observation, owner-step
and after-observation failures. `Materialization` preserves the actual `ObservedStep`,
the failed `WitnessSide` and any completed before materialization. A failure does
not roll back the image or retry the owner.

Streaming uses `LastUse`. The sink borrows each complete record; the function drops
its current record buffers before advancing and retains no history. Return
`StreamAction::Stop` to receive `StreamStop::Sink` after that delivered boundary.
Call again with the same admitted pair, image and cache to continue. An actual
terminal boundary is delivered to the sink and returns `StreamStop::Terminal`,
even if that callback requests Stop. A sink can retain its own clones.

## Diagnostics and evidence

Scheduling reports scheduled nodes, hits/misses, scope resets, selected cache
clone calls, reclaimed scratch slots, peak/retained scratch slots and retained memo
cells. `HostDiagnostics` counts operations actually executed. These are logical
counts, not total cloning, heap allocation, physical bytes, elapsed-time savings,
B work charges or prover memory; measured bytes/time remain `None`.

The [actual ordinary run](../../results/sherlock-witness-cache-stream-20261018/README.md)
compiled the changed normal Core and consumer, then passed three selected normal
cases. Each compared complete decoded variants on real saved owner boundaries,
with final literals 6/7/8; factorial also stopped and resumed the same image.
The tests are authored differential checks, not an independent complete B-prefix
or work oracle. UIREQ-035, SCON-013/024 and full B-to-A boundary/clock correspondence
remain open. See [the streaming guide](../how-to/stream-owner-witnesses.md).
