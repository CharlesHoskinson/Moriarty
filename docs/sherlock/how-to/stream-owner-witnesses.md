# Stream complete owner witnesses

Use this API when an existing admitted host run needs complete materialized
before/after observations without retaining a record history in the streaming
function. Start with the matching `Admitted` and `MachineImage` produced by the
existing owner's `admit` and `admit_run` APIs.

The example stops after the first delivered boundary, then continues the same
image and caches until the owner reports Terminal. Its logging callback consumes
the complete after-state; replace that logging with your own sink.

```rust
use mori_core::sherlock::helper_machine::shared::{Admitted, MachineImage};
use mori_core::sherlock::helper_machine::witness_plan as wp;

fn observe_run(
    admitted: &Admitted,
    image: &mut MachineImage,
) -> Result<(), wp::StrategyStepError> {
    let mut cache = wp::WitnessRecordCache::default();
    let first = wp::stream_owner_steps(admitted, image, Some(&mut cache), |step| {
        println!("{:?}", step.after.evaluation.witness.decoded);
        wp::StreamAction::Stop
    })?;

    if first == wp::StreamStop::Sink {
        wp::stream_owner_steps(admitted, image, Some(&mut cache), |step| {
            println!("{:?}", step.after.evaluation.witness.decoded);
            wp::StreamAction::Continue
        })?;
    }
    Ok(())
}
```

The sink also has `step.observed`, `step.before.raw`, `step.after.raw`, both plans,
and separate executed-operation/scheduling diagnostics. It borrows one record;
clone explicitly if the application needs to retain it. Callback work and retained
clones are outside owner work counters. Omit caching with `None` when it is not
needed. A cache hit is optional: changed snapshot table or payload shapes may
change the graph and reset its scope.

Keep the same image after `StreamStop::Sink`; do not replay the delivered step.
`StreamStop::Terminal` takes precedence when the delivered record is terminal.
Inspect `StrategyStepError` before deciding how the application continues: an
owner step may already have happened, and materialization failures preserve its
record and any completed before boundary. The library performs no rollback or
retry.

For a single recorded step or a saved snapshot, use the functions in the
[API reference](../reference/witness-cache-stream.md). The existing uncached
materializer remains a useful differential comparison on those same saved
boundaries; agreement does not independently validate the owner transition.

The [recorded ordinary evidence](../../results/sherlock-witness-cache-stream-20261018/README.md)
contains one changed Core build, one consumer build and three exact normal tests.
Those observations cover full decoded variant equality, reads/cache counters,
last-use live-outs and same-image continuation for the selected cases. They do
not establish an independent complete B-prefix/work mapping, measured memory or
speed benefit, canonical checkpoints, native proofs or settlement.
