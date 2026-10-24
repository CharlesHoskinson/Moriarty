# Selected CLI test provenance

Historical record from the October19 pre-execution source-review cut. Root later
observed all21 exact tests pass in the separate NW24 qualification; see the
[current observation](../../reference/nominal-body-typed-cli-observation-20261019.md).
The original UNRUN statements below retain their historical scope.

All21 new test functions are UNRUN. Initial20were frozen at d3aed04c;
public typed-profile recognition was additionally asserted in ba03f7ee before
new production edits. These are source-derived expectations, not observations.
The21st publication-failure metadata test was frozen in1f1f5c27 after initial
CLI wiring but before implementing that later-failure metadata behavior.

A passive read of the unchanged qualified `OwnerWire for TypeIdC` at
`crates/mori-core/src/sherlock/compile_artifact/wire.rs` exposed an expectation
error: builtin TypeIdC UInt/Bool serialize as the strings UInt/Bool, while the
initial20expected their internal IDs0/1. The test helper and unused-return
expectation were corrected to those source-defined external labels before any
execution. That correction happened after initial CLI wiring; the original
commits remain intact. No expected data was generated from candidate output.
The production adapter continues to call the unchanged owner wire projection.

The independently qualified BW27 tests and existing CWfive handler tests/two
positive artifact expectations are unchanged. Their observed results cover
only their exact source cuts; they do not qualify this new selected handler.
