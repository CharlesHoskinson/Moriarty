# Compatibility and upgrades

The supported deployment is a local Codex plugin running `python3` through a POSIX shell. The guarded CLI is the execution gate. Hook interception and package presence are separate observations.

## Support and evidence

| Surface | Contract | Verification scope |
| --- | --- | --- |
| Codex hooks | SessionStart, PreToolUse, PostToolUse, Stop | Linux/WSL host smoke with Codex 0.154.0; retain the event receipt for the exact installed candidate |
| Hook JSON | Snake-case host fields; legacy camel-case aliases | Package-local subprocess tests |
| Root variables | Use nonempty `PLUGIN_ROOT`; otherwise `CLAUDE_PLUGIN_ROOT` | Registered POSIX command tests, including spaces in paths |
| Permitted calls | Omit `permissionDecision`; host permission handling continues | Subprocess and actual Codex event tests |
| Denied calls | PreToolUse `permissionDecision: "deny"` with reason | Protocol tests; verify actual host denial separately |
| Stop | Normal output `{}`; failures may add common `systemMessage` only | Regression tests; no continuation or event-specific envelope |
| Python | 3.11 or newer (`hashlib.file_digest` is used by the runner) | This cycle executed on Python 3.14.4; other versions are not claimed as tested |
| Operating system | POSIX shell and Python for installed hooks | Linux/WSL tested; macOS not yet host-tested; native Windows command syntax is not supported |
| Claude-compatible fields | Legacy aliases and `CLAUDE_PLUGIN_ROOT` | Protocol compatibility only; no claim of a tested Claude installation/session |
| Runner | Linux, existing admitted Foreman launcher, hash-bound inputs and strong containment | Existing runner tests; the launcher path is deployment-specific, not portable configuration |
| Notifications | Current inspected Codex transcript adapter | Existing notification tests; other host transcript formats unsupported |

The [official hook reference](https://learn.chatgpt.com/docs/hooks) describes the current host schema and both plugin root variables. Host features evolve. This plugin uses the conservative intersection observed in its supported hosts: it never rewrites tool inputs and never emits an explicit allow. Do not enable newer response fields solely because a newer reference documents them.

The adapter accepts at most 8,192 UTF-8 input bytes and emits at most 2,048 bytes including its trailing newline. Malformed, non-object, oversized or invalid UTF-8 input receives a diagnostic before repository access. Large PostToolUse responses can exceed that input limit; inspect `status`/`report` for any omitted reminder. POSIX hooks have a 0.75-second adapter deadline inside the host's one-second command timeout. On systems without `setitimer`, only the host timeout bounds the whole command.

Simple Python `cli.py run --action ID` and `python -m moriarty_dev.cli ... run --action ID` forms are recognized, including `--repo`, `--json`, Python `-B`/`-u`, and `--action=ID`. Exact registered raw argv is denied in favor of the guarded CLI. Searches, printing action IDs, and `review --action` are not dispatches. Shell composition, interpreter code strings, wrapper commands, PowerShell syntax, code-mode composition and native delegation are outside recognition guarantees. Use CLI `run` for registered execution regardless of hook coverage.

## Inspect installation without changing it

```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor --json
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . doctor --json \
  --expected-plugin-root /absolute/cache/path/referenced/by/the/session
```

`--expected-plugin-root` is repeatable. Doctor never restores, deletes, enables, trusts or installs a package. `CODEX_HOME` selects the cache tree. Doctor and report use the same observation helper. JSON additions are additive; `--json` works before or after the subcommand.

`compatibility.executingPlugin` identifies the code running this command. `environmentRoots` checks the two host root variables without changing their precedence. `cachedRoots` lists observed package versions; `expectedRoots` checks explicitly supplied session references, including missing ones. `usable` means required entry files and manifest identity are present, not that imports, host trust or runtime execution have passed. `hostInstalled` retains its legacy name and means at least one structurally present package was observed in the cache. `installed-unverified` never means enabled or active. `activation` and `hookTrust` remain `unknown` until separately observed through the host.

## Upgrade without breaking an active session

An active session may retain the exact versioned path loaded at startup. Reinstalling a newer version does not rewrite that reference. The plugin cannot repair a missing interpreter or script before the host starts it. Keep missing-path errors visible and restore the reviewed package; do not hide them with an automatic permit or silently execute a different package version.

1. Run tests against the source, review the entire candidate, and retain its file hashes. Confirm which local marketplace source actually supplies `moriarty-dev`; a repository copy and `~/plugins/moriarty-dev` may differ.
2. Record all existing Moriarty cache roots and explicitly known active-session paths with doctor. Preserve complete copies and hashes outside the cache before upgrading. Never delete or overwrite a retained version merely because another version is installed.
3. Stage the reviewed package in the marketplace's existing local source. Check that the source has not changed since the review. Use the existing plugin-creator cachebuster helper and `codex plugin add moriarty-dev@<validated-marketplace-name>`; do not edit marketplace or trust configuration by hand. A new cachebuster identifies a new candidate; retain its manifest and verify installed bytes.
4. Immediately check every retained path. If the host updater removed a path, restore its exact preserved package to the same location before the original session resumes. Do not substitute the new implementation for a live session's preserved version.
5. Run the package-only compatibility tests from the newly installed root. Start a fresh Codex thread to load the new version, then record actual hook events for a permitted command, a denied harmless fixture, and Stop. A direct Python invocation proves protocol behavior only. A changed hook definition may require host review; do not alter trust settings to bypass it.
6. Keep old roots for existing sessions. Roll back through the supported plugin installation flow using the preserved source; do not delete the newer cache while any session may reference it. Cache retirement is a separate explicit action after all dependent sessions have ended.

Both root variables usually name the same installed package in Codex. If both are set and disagree, a nonempty native `PLUGIN_ROOT` is authoritative, even if missing. Restoring that exact path preserves the version selected by the host; falling through to an unrelated `CLAUDE_PLUGIN_ROOT` would change the code being executed.

## Tests from source and from a cache

Full repository integration suite (requires retained OpenSpec/evidence fixtures):

```bash
python3 -m unittest discover -s plugins/moriarty-dev/tests -v
```

Self-contained compatibility suite, from any copied plugin root:

```bash
python3 -m unittest discover -s /absolute/plugin/root/tests -p test_compatibility.py -v
```

The package suite checks JSON contracts, CLI diagnostics, dependency reporting, and coexistence of copied versioned packages. It does not simulate Codex's installer or prove uninterrupted upgrades. Running all repository integration tests inside a cache fails because those retained repository fixtures are outside the plugin; do not copy production state into a cache to make that invocation pass.

The [host smoke procedure](tests/host-smoke.md) records runtime evidence. Each receipt must identify host version, installed source/hash, tested tool path, actual hook completion/denial and marker outcome. Unit tests, reviews, and cache scans cannot promote themselves to verified interception.

## Historical operational data

The shared Git common-directory database is the history boundary. Readers accept either checkout or common-directory identity and include historical rows written under main, linked and removed-worktree paths. The same ownership rule retains pending outbox notifications after a worktree is removed and permits idempotent re-observation from another linked checkout. Rows and schemas are not migrated or rewritten. A database from an unrelated repository is not accepted for that history request.

Approval ingestion still accepts receipts without resolution IDs, but they no longer clear blockers. To resolve findings, provide a list of nonempty exact IDs in `resolvedFindings` (legacy alias `resolved`). An explicit current-name field takes precedence over the legacy alias. Missing, empty or malformed historical lists clear nothing; malformed new lists are rejected before writes. A named resolution may refer to a stable finding from an earlier candidate; candidate changes alone never erase findings. Old broad approvals may therefore expose blockers again, requiring a fresh scoped resolution.
