# R73 external A grouping test proposal — frozen before visibility edits

Phase: external positive test source only. Production afterimage is NOT written.
All test/compiler/model/provider/native execution is UNRUN. W and Git are unchanged
by this lane. Root alone owns production registration, execution and integration.

The proposed external client imports
`mori_core::sherlock::reference_slice::{advance_m, ReferenceSlice, SliceBoundary,
SliceBoundaryKind, SliceStop}` and existing public A types. All Machine instances
come from `Machine::new` with the fixed valid N01 literal UInt7, fuel0/1 and stack8.
Only `advance_m` and public `snapshot()` inspect/change those machines. Tests do
not use private fields, arbitrary state imports, B owners or SDK paths.

## Independent expected data

K-A main.k SHA256 b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41
and the already frozen independent N01 normal-prefix/complete fuel1 records
supply the semantics. The handwritten Rust expectations explicitly construct
all8Snapshot fields and all7ReferenceSlice fields. Inputs/record/snapshot/schema
constructors are shared public typed API facts; that schema dependency is
unavoidable and is not claimed to be independent implementation evidence.

The existing grouping source, its private tests and Machine body were read for
API/currentness facts and to avoid duplicating their broad corpus. No observed
Machine/grouping output was read, run or used as an oracle. The new test's values,
rules and charges come from the frozen K/independent literals and hand arithmetic.
Tests never derive expected states from actual Machine snapshots or records.

N01 source has no functions, entryLiteral UInt7. Its post-startup initial state
has Eval7, empty environment/frames, stack8 and Pending. At fuel1, line102 costs1,
yields Return7/Pending and remaining0. With empty frames, line80 publishes Done7,
clears control and costs0, even at remaining0. At fuel0 while still Eval7, line84
publishes Exhausted, clears control and costs0. Initial fuel remains1 or0 as
supplied. Program/environment/frames/stack remain unchanged. No startup uiRun
record is invented in the post-startup public Machine API.

## Small positive source selection

| Case | Width | Exact records | Prefix work | Boundary/stop |
|---|---|---|---|---|
| Fresh N01 fuel1 |0|empty|[0]|TechnicalCut0, still Eval7/Pending|
| Fresh N01 fuel1 |1|102/charge1|[0,1]|TechnicalCut1, Return7/Pending|
| Fresh N01 fuel1 |2/4/8|102/1 then80/0|[0,1,1]|Terminal2, Done7|
| Resume after width1 |0|empty|[0]|TechnicalCut0, Return7/Pending|
| Resume same pending return |1|80/0|[0,0]|Terminal1, Done7|
| Already Done7 |0/1/2/4/8|empty|[0]|Terminal0, unchanged Done7|
| Fresh N01 fuel0 |0|empty|[0]|TechnicalCut0, still Eval7/Pending|
| Fresh N01 fuel0 |1/2/4/8|84/0|[0,0]|Terminal1, Exhausted|

Three Rust test functions cover this table using exact complete slice equality
and returned public Machine snapshots. Wider budgets stop at the first actual
publication instead of padding/replaying to the width. The resume test uses the
same Machine and verifies m0 neither advances nor publishes. No generic generated
corpus, new interpreter or duplicate engine is introduced.

## Root's next step and limits

Root should first typecheck this exact external source against the current
production metadata and retain the missing `reference_slice` module/API diagnostic.
The current production mod.rs places reference_slice behind cfg(test), and the
leaf itself has file cfg(test) plus pub(super) declarations. This lane has not
written production visibility changes. It stops at this frozen test proposal
until Root supplies the red record and authorizes the already scoped visibility
promotion. Root alone changes module registration/export.

This is the existing ordinary K-A host baseline. Passing later tests would only
establish this small public API's fixed literal behavior; full SCON015/B08 domain,
constraints, optimized/native grouping, persistent boundaries and canonical
composition remain open, together with all U/M/MC/SP/G obligations. Technical cuts
are observation grouping boundaries, not financial commits, source suspension or
new outcomes. Existing host-diagnostic/resource scope remains unchanged.
