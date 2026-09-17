# Historical trace106 input recovered

The compressed input expands to exactly 3,278,570 bytes with SHA256 `6c29dc49a828956f5056605b672b519a515129383c7dd46b0dc34bba174c3c1b`, matching the historical [diagnosis](../../language-to-ledger-2026-09-12/design-review/k-failure-diagnosis.md). `record.json` identifies the archived fixture, codec and deterministic encoding. `astra-verification.json` records a separate reconstruction and byte comparison.

This recovers the input only. It does not execute K, recover the compiled interpreter/definition or pinned runtime, reproduce the native crash, or authorize a campaign. Preserve historical resource charges and obtain the missing artifact/runtime before the admitted reproduction.
