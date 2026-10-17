# Complete B pairwise supplied-data comparison

Approved scope: Root task for SCON-007, including the later fixed UInt7 source-only fixture instruction. This is a new leaf over the existing concrete `HostSnapshot` and `CompleteStepRecord`. No shared interface redesign is needed.

The callable predicate is: `first_difference` returns None exactly when initial snapshots, record counts, and every corresponding before/event/after value are equal under their existing typed Eq implementations. Its deterministic order is initial; then ascending common record index, before then raw event then after; then record length. A strict prefix differs at length after its equal common records. Empty slices are present zero-event observations; missing-layer status belongs to the caller.

API: four borrowed arguments (expected initial/records, observed initial/records) and an optional borrowed `TraceDifference` enum. Snapshot/event differences carry references to the original supplied values; length carries both lengths and the common count. The implementation allocates nothing, retains raw typed values, and uses no normalization, serialization, clock conversion or owner operation.

The alternative of comparing whole records first obscures the first location. A generic Eq kernel would weaken the concrete test domain. Neither is used. Existing `first_discontinuity` remains a separate continuity predicate; equal supplied traces need not be contiguous or executable to compare equal.

Source-only tests obtain a real initial snapshot using the exact R46 fixed UInt7, fuel64, stack8 admission/capture fixture. SegBase stays opaque and is copied through Clone. Public typed event literals and cloned observation variants are supplied comparison data, not claims of actual owner events. No step, run, record_step or owner mutation occurs. All Rust compilation/execution is UNRUN pending fresh designated-owner classification; R44/R46 decisions do not transfer.

Root alone may register `pub mod complete_comparison;` in the existing helper_machine module after review. Production consumer: callers already holding complete_observation HostSnapshot/CompleteStepRecord (including the observer records) invoke this leaf directly. The new helper adds same-profile pairwise correspondence to existing continuity checks; it does not authenticate source, validate transitions, establish terminality, satisfy proof/financial obligations or close SCON-007 in full.
