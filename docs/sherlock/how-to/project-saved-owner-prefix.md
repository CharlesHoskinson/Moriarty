# Project a saved owner prefix

Keep the exact `Arc<Program>` associated with the admitted source, an initial
`HostSnapshot` and the ordered actual `CompleteStepRecord`s from the existing
owner observation API. Supply reference fuel independently of the raw B work
counters. The projector consumes those saved records; it does not run the owner
or the A machine.

```rust
use std::sync::Arc;
use mori_core::sherlock::Program;
use mori_core::sherlock::helper_machine::complete_observation::{
    CompleteStepRecord, HostSnapshot,
};
use mori_core::sherlock::helper_machine::reference_projection::{
    ProjectionGap, ReferencePrefixProjection,
};

fn project_saved(
    program: Arc<Program>,
    initial: &HostSnapshot,
    records: &[CompleteStepRecord],
    reference_fuel: u128,
) -> Result<ReferencePrefixProjection, ProjectionGap> {
    let mut prefix = ReferencePrefixProjection::new(program, initial, reference_fuel)?;
    for record in records {
        prefix.push(record)?;
    }
    prefix.finish()?;
    Ok(prefix)
}
```

Inspect `records()` for complete translated before/after states and logical
charges, and `returned()` for the current reference boundary. To consume spans
incrementally, match `ProjectionProgress` from each `push`: Pending retains raw
helper intermediates; Complete contains the exact B interval, source-event index
and translated reference record. Do not replace each B micro-record with an A
step or infer A fuel from B helper counters.

`finish()` rejects a trailing incomplete interval. It does not require a terminal
A outcome; a pending call at a complete semantic boundary is valid. Preserve raw
records when handling `ProjectionGap` and do not assume a failed `push` rolled
back the accumulator. The [API reference](../reference/saved-reference-projection.md)
lists available boundary data and typed gaps.

For comparison, retain complete programs, every state/frame/environment/control/
outcome, ordered records, logical work and pending-caller details from an
independent expectation. The [recorded three-case run](../../results/sherlock-witness-reference-prefix-20261018/README.md)
performed those comparisons for the frozen factorial/list/tree fixtures. It did
not establish all B clock/rule cases, a physical performance benefit, canonical
checkpoints, native proofs or whole UIREQ-035 closure. The accumulator and a
retaining sink can keep complete histories; their host costs remain separate.

## Portable fixture publication evidence — 2026-10-18

The [dated evidence addendum](../../results/sherlock-witness-reference-prefix-20261018/addenda/20261018-portable-currentness/README.md) records the fresh four-path source review and one successful portable consumer compile (0.571720103 seconds, verified five dep-info inputs). The three functional runtime passes remain evidence for the original absolute-include consumer. No runtime case was replayed on the portable binary. The four fixture copies are byte-identical; only the include paths changed.
