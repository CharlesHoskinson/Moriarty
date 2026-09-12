# Moriarty plugin compatibility review and repair

User scope: improve plugin code quality, design and compatibility after the September 11 cache/hook incident. Astra authors; fresh Grok 4.6 high and Astra medium independently review. Existing source fixes, operational denials, trust, active cache roots and unrelated work remain intact.

## Design decision

Use small, explicit compatibility boundaries in the existing plugin. Extract pure hook wire parsing/serialization and read-only installation observations; keep policy and admission as the launch gate. Recognize only documented simple CLI dispatch forms and exact registered argv. Resolve sprint completion conservatively through dependency requirements.

A minimal patch confined to hook.py would leave input contracts and inconsistent diagnostics hard to test. A new installer/host abstraction framework would create more upgrade state and scope than this repair needs. The selected bounded approach adds package-local tests and a documented staged upgrade/retention procedure; it does not promise native Windows dispatch, full Claude host support or arbitrary shell parsing.

## Acceptance and implementation sequence

- [x] Add standalone compatibility tests: malformed/non-object/oversized/invalid-UTF8 input, typed and unknown events, byte bounds and denials, cwd scoping, simple dispatch versus harmless mentions. Verify failures before changing runtime.
- [x] Extract wire contract into hook_protocol.py and refactor hook.py into readable orchestration and supported-command recognition. Known denied raw argv and CLI run remain denied; ordinary PreToolUse omits permissionDecision; normal Stop remains {}.
- [x] Add one read-only compatibility.py observation helper used by doctor and report. Distinguish executing source, valid/broken cached copies, explicitly expected roots, and unknown activation/trust. Keep old JSON keys; new diagnostics are additive. Respect CODEX_HOME.
- [x] Fix JSON option placement and dependency-aware sprint reporting; test missing/cyclic prerequisites and completion propagation.
- [x] Document support/verification matrix, package-only vs repository integration tests, upgrade/rollback/cache retention, and direct CLI fallback. Exercise copied old and new package roots and actual registered shell commands.
- [x] Run all plugin tests, manifest validation, source/cache preservation checks; freeze full plugin hashes and obtain both fresh independent audits. Repair findings and repeat fresh reviews when candidate changes.
- [x] Integrate only reviewed files against unchanged baseline hashes in the main checkout. Stage a versioned installation using the existing supported CLI workflow only if old roots can be preserved; test actual Codex events and report exact coverage.

## Validation limits

Runtime validation begins on Linux/WSL, Python 3.14.4 and Codex 0.154.0. Document Python 3.11+ minimum from hashlib.file_digest. Existing Bash root-variable tests are protocol compatibility evidence, not a Claude session audit. Cache existence never establishes registration, activation or trust. Existing runner platform/path restrictions remain explicit.

## Review-discovered store corrections

- [x] Preserve checkout-keyed historical events/admin intervals across main, linked and removed worktrees by checking shared database ownership before projection; do not migrate rows.
- [x] Require explicit validated resolution IDs to clear only named stable findings; generic approvals remain recorded but clear nothing.
- [x] Apply the same ownership rule to pending outbox reads and idempotent enqueue after worktree removal, preserving independent-repository isolation.

Candidate-02: 199 full tests and 23 copied-package tests passed. Fresh Astra final audit approves all 32 hashes; Grok final source plus supplement and installed-result reviews approve. Previous Grok attempts cancelled and do not count as approval.
