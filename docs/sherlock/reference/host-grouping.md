# K-A host event grouping

`mori_core::sherlock::reference_slice::advance_m(&mut machine, max_events)`
groups steps from the existing `sherlock-k-a/1` host machine. The public
`ReferenceSlice` retains the complete starting snapshot, ordered `StepRecord`s,
cumulative `prefix_work`, boundary, ending snapshot and stop reason.

```rust
use mori_core::sherlock::reference_slice::{advance_m, SliceStop};

let slice = advance_m(&mut machine, 4);
assert_eq!(slice.prefix_work.len(), slice.records.len() + 1);
match slice.stop {
    SliceStop::TechnicalCut => { /* Continue with the same machine. */ }
    SliceStop::Outcome(outcome) => { /* First published outcome. */ }
}
```

The example assumes an existing `mori_core::sherlock::Machine`. The API advances
at most the requested number of events, retaining every internal rule boundary.
Free publication occupies an event position and adds zero work. `prefix_work`
starts at zero and adds each retained record's actual charge. The single
`SliceBoundary.position` counts records.

For a pending machine, a zero-event request returns unchanged start/end snapshots,
empty records, `[0]` work and a `TechnicalCut` at position zero. A cut after a
charged return leaves its free publication for the next group. The first
published outcome stops the group immediately. An already published machine
returns an empty terminal group, including for a zero-event request.

The types `ReferenceSlice`, `SliceBoundary`, `SliceBoundaryKind` and `SliceStop`
and their fields are public. [Exact declarations](../../../crates/mori-core/src/sherlock/reference_slice.rs)
retain the existing grouping implementation. Snapshot cloning and host
allocation remain unmetered; this exposes the host baseline for callers.
Technical cuts create no financial commit or suspension. Native grouping,
B-profile helpers, persistent import and constrained/residual execution retain
their separate requirements.

The [recorded checks](../../results/sherlock-public-grouping-20261017/README.md)
show a missing-public-module compile failure before the visibility change, then
successful production and external-test metadata checks. The three fixed test
functions typecheck but have not run. Fresh review remains pending.
