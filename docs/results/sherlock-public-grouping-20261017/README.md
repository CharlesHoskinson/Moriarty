# Public K-A grouping — October 17

Production callers can now import the existing
`mori_core::sherlock::reference_slice` API. Previously both the module and file
were configured only for internal tests. Root registered the public module and
applied the exact visibility-only leaf afterimage. The entire `advance_m` body,
type shapes and field order remain unchanged. See the [API reference](../../sherlock/reference/host-grouping.md).

The independent fixed external test source froze before the production change.
Its complete literal expectations come from frozen K/N01 semantics and public
typed declarations, rather than machine output. Three test functions cover
widths 0/1/2/4/8, charged/free-publication cuts, same-machine continuation and
already published outcomes. They are not a full rule-domain corpus.

| Check | Actual result |
|---|---|
| [Before public exposure](73-a-public-grouping/red-01/RESULT.json) | Metadata exit 1: exactly one missing `reference_slice` public-module error |
| [Production metadata](75-a-public-grouping-metadata/CORE-RESULT.json) | Exit 0 in 1.798 seconds; six existing warnings |
| [Three external functions](75-a-public-grouping-metadata/TESTS-RESULT.json) | Metadata exit 0 in 0.035 seconds; empty stderr |

The [green attempt](75-a-public-grouping-metadata/RESULT.json) binds base
`9dde3c3baa4dae26964f5287d588d58b731e7fdc` plus the leaf visibility change,
module registration and fixed external test. All 127 source/config/test files
and dependency/toolchain pins were unchanged. Dep-info contains 60 production
paths and one test path, all pinned after normalizing path spellings. Each child
exit was saved before later collector checks.

No linking or test execution occurred. Source/runtime review and the actual
three-test run remain pending. This is the ordinary A host grouping baseline,
not full SCON-015, B grouping, residualization, persistent boundaries,
constraints, cross-layer correspondence or native/stagenet acceptance. No
transition/model/circuit body was edited or model/native command run. Existing
private grouping tests remain configured as tests; they were not rerun here.

[Membership](membership.json) preserves 23 byte-identical original receipts.
Binary metadata outputs stay local with their recorded hashes. Original local
paths identify archival inputs rather than portable rerun instructions.
