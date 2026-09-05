# Root adoption of sharded Task6 tests

Original handoff SHA256: `5f0d07c051bd0a21a7aec8d3ee0819e67659501a082418efc628ea9725f06201`.
Tracked plan SHA256: `f9b46e2321faa6eb55c1a817c51d6c09e7e9123bbbc46ec76081fa3aaf1517bf`.
The tracked copy removes one final empty line; all instruction/code bytes are otherwise unchanged.
This review adopts the implementation design; it does not claim tests ran.

Root read the complete plan and Python append. A read-only AST parse completed
with exit 0: 30 top-level nodes, 20 distinct helper/test functions, and all 27
distinct named semantic mutants. The source review checked the admitted checker
APIs, unchanged mutation rules, per-event transformations, effective-change
assertions, full raw binding before semantic rejection, corrected/unrelated
controls, subprocess lifecycle and original artifact retention.

The expected pre-export count is 114 passes and one expressly pending native
package test. The final count is 115 with no skips or deselections. These are
planned counts, not observations. Actual package admission is a hard prerequisite
for the seven package controls and eight full honest traversals.

The seven package controls change only manifest/admission copies, so they can
reference unchanged read-only native files without an overlay. No native file
mutation or broader semantic package attack is claimed. Any future native file
change requires the previously specified recoverable copy-on-write overlay.

Only the existing checker test file changes permanently. The genuine raw-field
omission RED requires a root-controlled source window and exact restoration of
the admitted checker hash. The recorder must pin itself and all actual imports,
retain each child's argv/streams/terminal and generated files, use fresh process
cache/basetemp paths, and distinguish synthetic artifacts from native evidence.
No producer recorded command may observe moving checker/test bytes.

T6S001–007 use EARS and OpenSpec-style acceptance scenarios. Full A4, A5, Council
and integration remain open. This adoption does not grant a source edit window;
root issues that release separately after active producer commands terminate.
