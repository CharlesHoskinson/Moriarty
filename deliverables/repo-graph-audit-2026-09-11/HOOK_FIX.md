# Hook protocol repair

Codex 0.154.0 rejected the previous Moriarty hook JSON even though Python exited zero:

- PreToolUse: `unsupported permissionDecision:allow`.
- Stop: `invalid stop hook JSON output`.

Permitted calls now omit a permission decision, preserving normal host handling. Explicit denial paths remain unchanged. Stop emits `{}`; error diagnostics use only a top-level systemMessage.

Both failure cases were reproduced through actual host events in `host-pretool-before.json`. After restoration, `host-hooks-after.json` records successful PreToolUse, PostToolUse and Stop events, with no hook errors. This proves the tested permitted shell call and lifecycle responses; it is not a claim of universal interception of every tool composition. Existing unit tests retain blocked dispatch coverage.

All 176 plugin tests pass (`plugin-tests-final.txt`). Fresh Astra and Grok 4.6 reviews approve the exact v2 source candidate (`hook-astra-review-v2.json`, `hook-grok-review-v2.json`, `hook-candidate-v2.json`). The Grok returned identity is `grok-4.6-build`.

The old active-session cache path disappeared during the update workflow, causing all tool calls and Stop to fail before Python could start. The user repaired that path from a separate terminal. Both old and new caches now contain the reviewed script; hashes are in `hook-installation-verification.json`. Preserve the old cache while an active session still refers to it. The cachebuster helper itself only rewrites the source version; the actor that removed the old cache was not established here.
