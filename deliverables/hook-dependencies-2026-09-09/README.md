# Installed dependency reconciliation, 2026-09-09

The active Codex task had Lean plugin runtime references to cache version 4.8.6 after that directory had been removed. The installed package was 4.8.7. Executing the old UserPromptSubmit command reproduced exit 127 with “No such file or directory”; the same command against 4.8.7 returned 0.

The dependency registration was refreshed with `codex plugin add lean4@lean4-skills --json`, which selected 4.8.7. A compatibility symlink from the missing 4.8.6 cache root to 4.8.7 keeps this already-running task's cached absolute references operational. It does not install or claim to restore 4.8.6. Package contents and hook trust hashes were not rewritten. New tasks should use the registered 4.8.7 root; a later upgrade may require retiring this compatibility link after affected tasks finish.

All three Lean hooks returned 0 through both roots. Independent GPT-6 review also exercised the blocking guard and confirmed it remains active. These subprocess checks establish the repaired command path; they do not alone establish a newly observed host hook event.

The installed Codex registry contained 10 plugins. All 10 current roots and 8 command hooks resolved with their executable/path dependencies present. Other package source versions already matched their installed records, so no arbitrary upgrades or historical proof-pin changes were made. See `dependency-check.json`.

A second stale reference was the checkpoint skill's removed `scripts/fm-session.py`. Its command examples now use the installed compiled TypeScript runtime `runtime/dist/fm-session.js` through Node. Resolving the skill symlink with `realpath` is required by that CLI's direct-entry check. The new `recover --json` command returned valid recovery JSON. Existing historical revision warnings remain; no checkpoint store migration or repair was performed.

Local changes live in Codex's package registration/cache link and the installed checkpoint skill. These are environment repairs; this document is the repository record. Moriarty product source is unaffected. Fable authentication was repaired separately through the standard interactive sign-in flow. No authentication check was bypassed.
