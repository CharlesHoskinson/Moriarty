# Verdict: APPROVE

Independent source audit of **candidate-02** (`moriarty-dev` `0.1.0+codex.20260911184750`). Product contract is `COMPATIBILITY.md`. I did not execute tests, touch the host, or read files outside the supplied text.

## Reviewed scope

Runtime: `plugin.json`, `hooks/hooks.json`, `cli.py`, `compatibility.py`, `hook.py`, `hook_protocol.py`, `notifications.py`, `policy.py`, `records.py`, `runner.py`, `store.py`, `__init__.py`, `COMPATIBILITY.md`, `README.md`.

Changed tests (full files as supplied): `test_campaign_history.py`, `test_compatibility.py`, `test_execution.py`, `test_hooks.py`, `test_host_adapter.py`, `test_notifications.py`, `test_policy.py`, `test_records.py`, `test_regressions.py`, `test_runner.py`, `test_store_connections.py`, `reproduce_driver.py`, `tests/fixtures/README.md`.

Not in the supplied body (not reviewed as bytes): `skills/develop/SKILL.md`, `execution-focus.md`, `host-smoke.md`, campaign/SP01 fixture blobs. Those are not required to judge the wire/store/runner contracts above.

Model identity: **Grok 4.5 / grok-4-6** (xAI), independent of the Astra authoring noted in README.

Reported suites (199 / package-only 23) were treated as author claims, not re-run.

## Contract checks that hold

**Native `PLUGIN_ROOT` is authoritative even if missing.** `hooks/hooks.json` uses `${PLUGIN_ROOT:-${CLAUDE_PLUGIN_ROOT}}` (e.g. SessionStart at hooks.json:10, same form on Pre/Post/Stop). Nonempty `PLUGIN_ROOT` is not replaced by `CLAUDE_PLUGIN_ROOT`. A missing native path fails at process start instead of running another cache. `compatibility.inspect_compatibility` records both env roots without changing precedence (`compatibility.py:61-64`). Coexistence tests keep old hook bytes and pass a stale Claude root (`test_compatibility.py:153-168`; `test_hooks.py:35-69`).

**No silent allow / no input rewrite.** `hook_protocol.output` never emits `permissionDecision: allow` (`hook_protocol.py:21-24`). Permitted PreToolUse is `{"hookSpecificOutput":{"hookEventName":"PreToolUse"}}`. Denies are PreToolUse `permissionDecision: "deny"` with a reason (`hook.py:212-220`). Stop success is `{}`; Stop failure is common `systemMessage` only (`hook_protocol.py:22-28`; `test_host_adapter.py:97-109`). Oversized non-deny output collapses to a bounded diagnostic, not a continuation (`hook_protocol.py:65-70`).

**Dispatch recognition matches the documented intersection.** `_dispatch_request` accepts simple `python`/`cli.py`/`-m moriarty_dev.cli` `run --action` forms, including `--repo`, `--json`, `-B`/`-u`, `--action=ID` (`hook.py:140-187`). Exact registered argv is denied in favor of guarded CLI even without an action id (`hook.py:209-213`; `test_compatibility.py:95-98`). Searches, `review --action`, and file edits are not treated as dispatches (`test_compatibility.py:79-85`; `test_host_adapter.py:27-34`).

**Doctor/report do not mutate install state.** `cmd_doctor` never restores or trusts (`cli.py:642-683`). `usable` is presence+manifest identity (`compatibility.py:36-40`). `activation` / `hookTrust` stay `unknown` (`compatibility.py:65`). Coverage is `installed-unverified` or `unverified`, never `host-verified` (`compatibility.py:76-78`; `test_host_adapter.py:60-67`).

**One physical DB per Git common directory.** `get_db_path` follows worktree `commondir` (`store.py:84-108`). `_owns_store_db` accepts checkout identity or common-dir identity (`store.py:111-124`). After ownership, history/outbox project all rows, including removed-worktree keys (`store.py:194-217`, `836-858`). Duplicate submit does not roll back confirmation or wipe delivery (`store.py:748-753`; `test_compatibility.py:236-248`; `test_notifications.py:52-56,130-138`). Wrong DB is rejected before `init_db` creates a file (`store.py:711-713`; `test_compatibility.py:249-253`). That is the candidate-01 outbox correction, and it is present.

**Resolution IDs.** `resolvedFindings` wins over legacy `resolved` (`store.py:126-130`). Missing/empty lists clear nothing; malformed lists raise before the store is opened (`store.py:622`; `test_compatibility.py:215-234`). `defect_resolved` is ignored (`store.py:274`). Stable IDs survive candidate rename (`test_execution.py:429-455`). Historical malformed receipts clear nothing on read (`store.py:269-272`).

**Stop / adapter bounds.** 8192-byte UTF-8 input, 2048-byte output including newline, 0.75s `setitimer` inside 1s hook timeout (`hook_protocol.py:9-10,31-36`; `hook.py:50-62`; `hooks.json` `"timeout": 1`). Malformed input never reaches `handle_event` (`test_compatibility.py:50-55`).

**Runner / CLI gate.** Catalog argv is not launch authority (`runner.py:73-88`). Charge is consumed once, never refunded (`store.py:448-452`; `test_runner.py:75-85`). Child JSON is not completion authority (`runner.py:178-186`; `test_runner.py:180-185`). Supervisor/launcher ambiguity leaves the reservation active (`cli.py:474-480`; `runner.py:169-171`; `test_runner.py:139-173`). Dead parent PID is not treated as child death (`test_execution.py:188-217`). `_is_pid_alive` is unused, which is the correct product behavior.

**Policy order and report honesty.** Closed snapshot/action shapes; bool is not int (`policy.py:7-26,348-367`). Report remains allowed on every dispatch block (`policy.py:88-89,306-325`). Status names stale source beside unknown history (`cli.py:282-291`; `test_execution.py:480-499`).

## Non-blocking observations (not product-contract breaks)

1. `CORE_FILES` omits `hook_protocol.py` and `compatibility.py` (`compatibility.py:10-16`). A truncated cache can still be `usable`. That matches the documented meaning of `usable`, and a missing `hook_protocol` fails at import rather than falling through to another version.
2. `_is_pid_alive` (`store.py:400-406`) is dead code. Current readers correctly ignore PID liveness; a later change that starts using it would violate the recovery rule. Prefer deletion in a later candidate, not a behavior change now.
3. `bootstrap_store` swallows insert errors (`store.py:393-394`) and can still report CLI success. History stays `None` if no events landed, so it does not invent clean zeros.
4. Hook recognition is the documented conservative subset. Extra interpreter flags (`python3 -I ...`) are outside the recognizer; the CLI remains the launch gate, as COMPATIBILITY.md states.
5. `runner.py:18` hard-codes the Foreman launcher path. COMPATIBILITY.md already calls that deployment-specific.

## Limitations of this audit

- Source-only. No subprocess, SQLite, Codex session, or marketplace install.
- Did not re-hash files against the manifest.
- Did not review skill prose, host-smoke receipts, or binary fixtures.
- Linux/WSL host interception is still an operational claim (`COMPATIBILITY.md` table); this review cannot promote it to verified.
- macOS and native Windows remain unsupported per the contract.

No concrete runtime defect against the stated product contract. Old hook definitions, cache coexistence, PLUGIN_ROOT authority, Stop envelope, named resolution IDs, and shared-store/outbox ownership behave as specified.

**APPROVE**

Runtime identity metadata: grok-4.6-build; requested grok-4.6, effort high. The prose self-description differs and is retained as emitted. Terminal status: end_turn.
