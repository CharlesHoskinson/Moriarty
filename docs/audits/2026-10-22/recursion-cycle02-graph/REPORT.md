# Recursion restart graph revision 02

This bounded delta graph records the current Source44 R2 source, formal K and Quint revisions, actual normal CLI build review, rollup-source review, and newer wallet/loop receipts. The original graph remains at `../graph.json` and is unchanged.

Graphify AST extraction covers exactly three Source44 R2 Rust files (212 AST nodes and 798 AST edges before receipt metadata). K and Quint files are hash-pinned navigation nodes because Graphify did not parse those syntaxes. The full selected evidence pins are in `coverage.json`; all graph paths are portable and repository/runtime-relative.

Source44 R2 Rust source and test changes were approved, while its R2 caller was rejected because its exclusive marker already existed. The actual successful normal CLI build used the corrected R3 caller; its independent review accepted that specific build. The retained ELF is recorded by SHA-256 `fe220ad89e4085ec359544697064ba77717cd3cb586bc676f270b06e6bac3a57`. This covers only the normal CLI: library test bodies remain uncompiled and unrun, and the recursive parent proof remains unproduced.

K R2 adds the required public equality import, but its source review is pending and all fourteen laws are unrun. Quint R2's two paired tests and separate 1,000-sample financial-refusal invariant run passed; this does not establish whole-loop completion. Rollup R2 has no blocking source-review findings; eleven native Rust test cases remain uncompiled and unrun. Two upstream ordinary checks passed, but the candidate native importer was not executed.

The read-only wallet observation returned `NOT_INDEXED_IN_THIS_OBSERVATION`; transaction status remains unknown and financial acceptance is false. The graph intentionally includes no wallet identifier or transaction hash. AgentBus merge remains distinct from live activation.

This graph is navigation context, not an execution prerequisite. It records source and receipt scope and does not imply parent proof, terminal proof, stagenet deployment, or benchmark completion.
