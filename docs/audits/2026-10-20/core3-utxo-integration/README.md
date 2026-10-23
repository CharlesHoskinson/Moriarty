# Core/3 UTXO source integration observation

The source commit `cd9caf15fbaffb71951fb8f343f3a4a5813756d1` adds the existing Core/3 implementation and public consumers to the SDK-conformance base `961a8dfca26f4eb830e0e3936ccbac7e731a3875`. Its 48 source/fixture/documentation paths are a candidate, without source, security or product approval.

The local offline Core library typecheck exited zero in 14.84 seconds. The complete command, replacement environment, pre-execution expectation and 692 source/tool/public-entry pins are in [typecheck-plan.json](typecheck-plan.json). [typecheck-result.json](typecheck-result.json), [Cargo JSON output](typecheck-stdout.jsonl) and [stderr](typecheck-stderr.txt) retain the actual result. All 526 selected source files and other pins held before and after. Cargo reported 68 artifact events and no Midnight SDK units. This invoked the compiler, ordinary registry build scripts and derive macros; it ran no interpreter or test function.

This evidence companion adds only captured public compiler data and this note. The tested source projection remains the source commit above. Local absolute paths identify that capture; the public record does not contain compiled binaries or private provider logs.

CLI linking and integrated runtime/wire/financial tests remain unrun. Earlier 118 Core passes describe the predecessor and do not transfer to this source. The original Core build timeout remains historical failure. Fresh source/result and applicable security reviews, the existing unadopted F3 condition, proof/compiler correspondence, authenticated composition and Cardano/Midnight target acceptance remain open. No EARS or parent milestone is closed.
