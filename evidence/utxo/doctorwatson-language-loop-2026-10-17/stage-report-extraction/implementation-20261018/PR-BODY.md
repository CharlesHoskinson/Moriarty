Staged `simulate` and staged `test` built their complete JSON reports separately. Both now call one typed Core renderer, preserving fields, qualification labels, exit mapping and I/O behavior. The inspect-report writer remains unchanged.

Two new SDK-free Core integration tests chain actual continuations and compare six complete independently frozen R3 reports, including exact serialization bytes. Original model fixtures and history remain unchanged.

Validation: `cargo test --frozen -j1 -p mori-core --no-default-features --test stage_reports -- --test-threads=1` passed 2 tests in a single bounded 26.65-second run. The exact tested tree is `61bc8233eb3c0ce421ac5a80e4112eed952925a6`; no Midnight SDK crates compiled. Source whitespace checks passed.

Public CLI execution, rejected-report tests, SDK/default/workspace checks, rustfmt and Clippy remain unrun. This finite local result closes no original language, financial, proof, ledger or full issue #80/#81/#84/#85/#86 exit.

Fresh independent source, ordinary result and designated Gemini security fallback reviews cover the exact candidate and prospective merge. The explicit binding clarification preserves the existing base tree and requires the merged tree to equal the tested tree. The one test-run admission is consumed.
