# Checked liability registry metadata

The checked registry was private to mori-core. `StagedProgram::liability_registry` now returns owned identities from the existing `decode::registry_for` helper. A compiler consumer can read every declared or statement-defined identity without constructing another registry or gaining mutable access to a sealed program.

The current source compiled and passed three metadata tests. They compare the complete Core/3 identity set, check Core/2 statement identities, and mutate a separately returned result before checking the program and subsequent query again. Expected source metadata was frozen before implementation; its provenance and implementation exposure are disclosed in the fixture README. The fresh Gemini source and result review approved this delta with no findings.

These results cover metadata access. The real Core/3 compiler still needs its registry layout, context and strict artifact contract, and the original financial tests remain unrun. No SDK, proof, wallet or ledger operation was selected by these tests. This directory records no EARS closure.

The first offline Cargo attempt failed during dependency resolution. The repaired RED build reached its 90-second limit. A direct baseline compiler check then observed the missing public type and method; that baseline differs in parser implementation, so it is not an exact-checkout Cargo RED result. Earlier pre-format passes remain historical. `actual-result.json` records the later compile and three tests on the exact formatted source. The whole test stdout file has one additional terminal newline compared with the embedded JSON projection; both are retained.

The original complete plans, compiler event streams and source pins remain at the paths recorded in the source evidence. Public copies here preserve their bytes and review limits. The parent branch and PR retain their separate validation requirements.

Only the raw patch and raw test stdout bypass whitespace diagnostics through this directory's two explicit attributes. Their context spaces and terminal newlines are evidence bytes; Rust source and maintained documentation keep the normal checks.
