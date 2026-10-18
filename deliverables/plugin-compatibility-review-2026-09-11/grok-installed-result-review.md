# Installed-result verdict: APPROVE

Evidence-only review of unchanged **candidate-02** (`0.1.0+codex.20260911184750`). Runtime identity: **grok-4.6-build**. No independent execution, no file access, no promotion of untested paths.

## What the supplied observations show

**Install / integrity (author-stated, not rehashed here)**  
Marketplace add exited 0 for `moriarty-dev@personal` at  
`/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.0+codex.20260911184750`.  
Main and installed trees match the frozen manifest. Preexisting roots  
`…/0.1.0+codex.20260911061236` and `…/0.1.0+codex.20260911181517` remain byte-identical.  
Host `hooks.json` and `config.toml` hashes unchanged (plugin hook definitions still come from the versioned cache, not a rewritten host file). Suites 199 / 23 are author-reported.

**Current cache loaded**  
Fresh thread `01a091dd-9e35-7491-b7fd-b0d22a366525` runs  
`session-start` / `preToolUse` / `postToolUse` / `stop` from  
`…/0.1.0+codex.20260911184750/hooks/hooks.json`.  
The pre-upgrade thread kept `…/0.1.0+codex.20260911181517`, which is the intended live-session retention.

**Registered raw marker stays blocked; marker absent**  
After upgrade, `preToolUse` `exec-206ee63f-…` is `blocked` with  
`Registered dispatch requires guarded CLI run --action compatibility-denied-marker.`  
That id is not in `result.commands`. `markerExists` is `false`. Same deny class as the pre-upgrade raw-dispatch block (`exec-415a2a23-…`). This is the exact-argv path in approved `hook.py`, not an allow.

**Benign printf of the action id: blocked → successful**  
Pre-upgrade `exec-5897783a-…` is `blocked` with `AUTHORITY_UNAVAILABLE` (over-recognition of an ID mention).  
Post-upgrade `/bin/bash -lc 'printf compatibility-denied-marker'` (`exec-de895e9a-…`) is `preToolUse` `completed` with empty entries, tool `completed`, exit 0, stdout `compatibility-denied-marker`. Matches COMPATIBILITY.md: printing an action id is not a dispatch.

**pwd permitted**  
`/bin/bash -lc pwd` completed both sides (exit 0, cwd `/tmp/moriarty-plugin-compatibility-host-20260911`). Matching `preToolUse` status is `completed` with empty entries (no `permissionDecision`, no explicit allow).

**Stop completed**  
Both threads: `eventName` `stop`, `status` `completed`, empty entries, no continuation / decision / new action. `turnStatus` `completed`. Adapter times 24–46 ms, inside the 1 s host / 0.75 s adapter bound.

Permitted PreToolUse omits a decision; denials use host `blocked` plus the plugin reason text; SessionStart context is diagnostic (`History unresolved`), not a clean-zero claim.

## Scope of this APPROVE

This receipt supports only the exercised host paths on this Codex thread:

- SessionStart reminder  
- PreToolUse deny of the registered raw marker argv  
- PreToolUse omit-decision for `pwd`  
- PreToolUse omit-decision for `printf` of the action id (candidate-02 change)  
- PostToolUse completion with empty reminder (no pending outbox)  
- Stop `{}`-class completion  

It does **not** establish: `cli.py run --action` through the host, code-mode/delegation, source-edit mention, pending-outbox PostToolUse text, native-Windows, or generic plugin trust. `doctor` stays `installed-unverified`. Unit tests and cache scans still do not equal untested interception.

**APPROVE** for this scoped installed result. Combined source approval of candidate-02 is unchanged.
