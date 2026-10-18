# Public CLI profile guidance on main79

This integration candidate combines exactly the two prior Rust help files and
one documentation file. All3 original bodies match main79 byte-for-byte; no
conflict or new-base repair was needed. Prior candidate bytes are reused intact.

CANDIDATE.patch changes only cli.rs, commands/expand.rs and docs/NATIVE-CORE.md.
source/ and candidate/ contain their complete bodies. MANIFEST.json records
original Git blob/SHA pins, exact input-patch hashes, combined hash and proposed
root checks. EXPECTED-CLAP-CONTRACT.json and CLAP-REGRESSION.rs preserve the
selected ordinary schema regression; DOC-EXPECTATIONS.json and
DOC-COMMAND-PREDICATES.json distinguish source-derived examples from actual
execution. doc-shell/ holds exactly the5 new walkthrough blocks for bash -n.

Main79 has the original direct check.rs dispatch and lacks the wave-only
check_source.rs helper. CURRENT-CONTEXT-PINS.json records that context difference;
no21file shared-checker branch is imported. The actual main CheckReport and
Core JSON branches retain the documented fields. No MCP/LSP changes are included.

Only offline source/hash/link/patch checks were performed. The selected Clap
test,5 shell syntax checks and fresh full-source/actual-result audit remain
root work. Do not execute doc-shell/*.sh as tests; bash -n parses syntax only.
No native, SDK, compiler, provider or product runtime was invoked; no shared
checkout/Git/GitHub changes were made. Parent acceptance obligations remain open.
