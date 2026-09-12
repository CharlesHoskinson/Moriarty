# Independent validation

Observed 2026-09-11 against the dirty canonical checkout `/home/charl/Moriarty`.

- `npm --prefix experiments/moriarty-language test`: 682 tests passed, zero failures or skips. Full output: language-tests.txt.
- `npm --prefix experiments/moriarty-language run typecheck`: exit zero. Full output: language-typecheck.txt.
- Plugin baseline: 174 tests passed. These tests missed the actual host JSON rejection.
- Plugin after both protocol fixes: 176 tests passed. Full output: plugin-tests-final.txt.
- Actual Codex 0.154.0 host events reproduce both PreToolUse and Stop parser errors in host-pretool-before.json. Post-fix host-hooks-after.json records successful hook events; see HOOK_FIX.md for scope and independent reviews.
- CLI status reports stale admission inputs and unavailable current accounting/live resource evidence. No new campaign or financial transaction was dispatched.
- A separate default read-only sandbox smoke could run pwd but status encountered SQLite `unable to open database file`; codex-hook-smoke.jsonl preserves that failure. The ordinary authorized local status invocation succeeded. This is separate from the reproduced hook protocol errors.

These checks do not establish full language conformance, native PCD, or new Preview settlement. Grok's repository audit has its own source coverage and limitations. Test counts describe the recorded command and source state; they are not release acceptance.

Root also checked the retained `preview-loan-exit-01/RESULT.md` and `preview-swap-exit-01/RESULT.md` under `deliverables/sp05-financial-integration-2026-09-09/`. Both report PASS_SCOPED financial results. Swap records exit zero and containment; loan explicitly leaves raw main-process exit unavailable and the strict exit gate unmet. Neither closes full SP05 or mandatory PCD/history acceptance. These are checks of retained evidence, not new network observations.

The full repository file map was independently checked: its declared file count matches actual file nodes, all files exist, and no duplicate IDs or missing edge endpoints were found (`file-map-validation.json`). Chromium verified search, exact current-source links, lazy directory expansion and cross-reference navigation with no JavaScript errors (`file-map-browser-validation.json`).

Checkpoint recovery also reported 74 historical measurements not fresh, with several unavailable historical Git revision ranges. Those old measurements were not reused for this audit. This does not invalidate the fresh commands recorded above.
