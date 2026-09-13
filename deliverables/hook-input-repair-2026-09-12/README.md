# Moriarty hook repair

Installed version `0.1.0+codex.20260912173932` fixes the valid-large-input warning and removes the new registration's dependency on disposable cache files. The registered bootstrap pins verified Python runtime contents and can load the same retained runtime outside the cache. The upgrade helper restores legacy cache roots before returning from its installer invocation.

The final source candidate was independently approved by fresh Grok 4.6 high and GPT-6 Astra medium audits. All 219 repository plugin tests pass. The installed package also passes 25 compatibility tests and 18 upgrade tests. [Final status](FINAL-STATUS.json), [source approval](durable-source-approval.json), [installation receipt](durable-install-result.json), and [design](durable-upgrade-design.md) retain the exact scope and hashes. Initial failed reviews and their repairs remain preserved.

## Host verification

After the user trusted the definitions in Codex, `hooks/list` reported all four as `trusted`. A fresh normal Codex session received Moriarty session context, completed the exact 16 KiB output command, and blocked the registered marker command through PreToolUse. The marker file is absent. The host recorded normal turn and task completion with no repeated Stop or unavailable-input warning. No trust configuration was edited and no bypass flag was used. See [trusted hook metadata](host-hook-status-after-trust.json), [host receipt](host-smoke-after-trust.json), and [independent verification](host-smoke-verification.json).

The earlier test before trust failed interception: the harmless marker ran while all four definitions were `modified`. Its [metadata](host-hook-status.json) and [receipt](host-smoke-session.json) remain preserved, as does the original fixture. The successful repeat used a separate copy. [Codex documents the exact-definition trust requirement](https://learn.chatgpt.com/docs/hooks#review-and-trust-hooks).

This evidence covers the stated fresh-host smoke. The doctor retains its broader `installed-unverified` coverage status; this test does not establish universal command interception. Guarded CLI execution remains mandatory for registered workflow dispatch.

## Recovery and limits

All retained cache roots match the snapshots taken immediately before this migration. The earlier user-restored cache contents were preserved. The migration completed installer invocation and restoration in one foreground process. New registrations retain their exact runtime across cache eviction; older sessions still require their retained cache paths until those sessions end. No automatic runtime retirement is implemented.

Digest pinning is not protection against a hostile same-user filesystem writer. Missing runtime/interpreter diagnostics do not establish interception. A broken interpreter executable, missing shell, SIGKILL, or power loss can require host recovery. These limits and source-dependent future upgrade preparation are documented in the plugin's compatibility contract.
