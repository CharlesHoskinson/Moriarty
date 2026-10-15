# Oracle freeze record — 2026-10-15

The three artifacts in SHA256SUMS are the independent source-task candidate.
The oracle author has not read or run the subject, builder, evaluator, subject
tests or their outputs. No model/compiler execution was performed. The source
expectations are static-derived and unexecuted; this record is not a test pass.
The integrator must preserve the hashes before first builder execution and
record any compilation or runtime result separately.

Source digests (SHA-256):

- ORACLE-TYPES-AND-CONTRACT.rs.txt: `fc8f36ea9fe72bd2416b68bd67015b6040885e512dae3e18f88775daf066e504`
- INPUT-AST-TYPES.rs.txt: `82aed752b6fb49358b7d684df5eeb43863dd8dc4ce6a68e2bd5b5991e8880f5a`
- extra-normal-programs.json: `dd85e5d400f24e9be5e2ff017f4b8c141a0bf6e4270053917b735cbe5153f153`

The source files were read as declarations and frozen public literal inputs.
Read-only plugin status reported historical campaign blockers; none was
dispatched. No private Fenrir files were read. No provider/subagent calls,
commits or worktree edits were made. Writes are confined to this runtime folder.

Falsifiable predicate: for each `normal_cases()` input, the integrator's returned
ProgramImageDraft must equal the complete literal expected image, including
every empty-slice start and source path. N07 and N08 each have 13 occurrence
rows; the shared tree has 3 constants; the two unused functions retain names
8,3 and body IDs 1,2. The independent expectations do not establish product
acceptance or any native/target claim.
