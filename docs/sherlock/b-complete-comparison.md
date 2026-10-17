# Compare supplied complete B traces

Use `sherlock::helper_machine::complete_comparison::first_difference` when both
`sherlock-helper-microstep/1` observations are present as an initial
`HostSnapshot` and a slice of `CompleteStepRecord`.

```rust,ignore
let difference = first_difference(
    &expected_initial, &expected_records,
    &observed_initial, &observed_records,
);
```

`None` means exact typed equality of initial state, record count and every
record's before state, raw `StepEventC` and after state. A difference identifies
initial, record-before, record-event, record-after or length. Record indices are
zero based. The traversal compares initial first, then common records in order
(before, event, after), then lengths. A strict prefix differs at the common
record count. Snapshot/event differences borrow the original supplied values.
No serialization, raw-id normalization, erasure or A/B work conversion occurs.

An empty slice means a present zero-event trace. The caller must retain a
separate missing-layer status. This function has no returned-state argument;
compare returned values directly, and use `complete_observation::first_discontinuity`
for initial/before/after/returned continuity. Equality and continuity remain
separate from transition validity, source provenance and terminality.

The helper executes no engine, admission, capture, proof or ledger operation.
It compares whatever complete owned values the caller supplies, including
noncontiguous records, and does not certify their production. Existing raw
profile, program, layout, image and event fields remain in the comparison.
This leaf alone does not close SCON-007 or full B interpreter correspondence.
