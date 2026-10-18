# Compatibility and upgrades

The supported deployment is a local Codex plugin running `python3` through a POSIX shell. The guarded CLI is the execution gate. Hook interception and package presence are separate observations.

## Support and evidence

| Surface | Contract | Verification scope |
| --- | --- | --- |
| Codex hooks | SessionStart, PreToolUse, PostToolUse, Stop | Linux/WSL host smoke with Codex 0.154.0; retain the event receipt for the exact installed candidate |
| Hook JSON | Snake-case host fields; legacy camel-case aliases | Package-local subprocess tests |
| Root variables | Use nonempty `PLUGIN_ROOT`; otherwise `CLAUDE_PLUGIN_ROOT`; require matching runtime pin | Registered POSIX command tests, including spaces in paths |
| Permitted calls | Omit `permissionDecision`; host permission handling continues | Subprocess and actual Codex event tests |
| Denied calls | PreToolUse `permissionDecision: "deny"` with reason | Protocol tests; verify actual host denial separately |
| Stop | Normal output `{}`; failures may add common `systemMessage` only | Regression tests; no continuation or event-specific envelope |
| Python | 3.11 or newer (`hashlib.file_digest` is used by the runner) | This cycle executed on Python 3.14.4; other versions are not claimed as tested |
| Operating system | POSIX shell and Python for installed hooks | Linux/WSL tested; macOS not yet host-tested; native Windows command syntax is not supported |
| Claude-compatible fields | Legacy aliases and `CLAUDE_PLUGIN_ROOT` | Protocol compatibility only; no claim of a tested Claude installation/session |
| Runner | Linux, existing admitted Foreman launcher, hash-bound inputs and strong containment | Existing runner tests; the launcher path is deployment-specific, not portable configuration |
| Notifications | Current inspected Codex transcript adapter | Existing notification tests; other host transcript formats unsupported |

The [official hook reference](https://learn.chatgpt.com/docs/hooks) describes the current host schema and both plugin root variables. Host features evolve. This plugin uses the conservative intersection observed in its supported hosts: it never rewrites tool inputs and never emits an explicit allow. Do not enable newer response fields solely because a newer reference documents them.

The adapter accepts at most 8,388,608 UTF-8 input bytes (8 MiB) and emits at most 2,048 bytes including its trailing newline. Malformed, non-object, oversized or invalid UTF-8 input receives a diagnostic before repository access. Normal large PreToolUse and PostToolUse envelopes are accepted within that input limit; responses above 8 MiB still receive a diagnostic, so inspect `status`/`report` for any omitted reminder. POSIX hooks have a 0.75-second adapter deadline inside the host's one-second command timeout. On systems without `setitimer`, only the host timeout bounds the whole command.

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

An active session retains its registered command. New registrations embed a small Python bootstrap and pin the complete Python runtime inventory by SHA-256. They can execute the same runtime under `~/.local/share/moriarty-dev/runtimes/DIGEST` after Codex removes the cache. They never select a mutable latest-version pointer. The command uses isolated Python (`-I`) and ignores user Python import settings. The runtime inventory binds each relative `.py` path and its content, rejects additions and symlinks, and excludes bytecode caches. Standard-library code and the interpreter are host dependencies, not part of the pin. Same-user hostile mutation between verification and execution is outside the workflow guard's threat model.

If neither exact runtime is available, or `python3` is absent, the registration emits bounded common diagnostic JSON with no permission decision or Stop continuation. Interception is degraded in that state. A diagnostic is not guard coverage. The existing malformed-input contract has the same absence of a decision. The guarded CLI independently validates admitted workflow execution; it cannot intercept raw commands launched outside it. A broken interpreter executable or missing POSIX shell still requires host repair.

1. Run tests against the source, review the entire candidate, and retain its file hashes. Confirm which local marketplace source actually supplies `moriarty-dev`; a repository copy and `~/plugins/moriarty-dev` may differ.
2. Bump the source manifest with the existing plugin-creator helper. Run `python3 plugins/moriarty-dev/scripts/moriarty_dev/deployment.py prepare --plugin plugins/moriarty-dev` to regenerate registrations and publish the pinned runtime. Freeze and review these exact bytes. Any later Python edit requires preparation and fresh review again.
3. Copy the reviewed package to the confirmed local marketplace source. Run the source entrypoint below. It verifies the source mapping, required Python modules, syntax, isolated protocol startup, and generated registration before invoking the supported Codex installer. It snapshots Moriarty cache roots from all marketplaces, the legacy installation, host-provided roots and repeatable `--retain-root PATH` references outside the cache. A missing explicitly retained root stops installation for recovery. Do not edit marketplace or trust configuration by hand.
4. The command restores every missing retained file in its `finally` block **before the same subprocess returns to PostToolUse**. A separate restoration tool call is unsafe: the deleted PreToolUse hook can prevent that call. Surviving modified files are preserved and reported as conflicts. Installer status, restoration status and installed-byte verification are separate fields in the retained receipt. The command serializes upgrades and runtime publication with a POSIX lock.
5. Run the package-only compatibility tests from the newly installed root. Start a fresh Codex thread to load the new version, then record actual hook events for a permitted command, a denied harmless fixture, and Stop. A direct Python invocation proves protocol behavior only. A changed hook definition may require host review; do not alter trust settings to bypass it.
6. Keep old roots for existing sessions. Roll back through the supported plugin installation flow using the preserved source; do not delete the newer cache while any session may reference it. Cache retirement is a separate explicit action after all dependent sessions have ended.

Both root variables usually name the same installed package in Codex. If both are set and disagree, a nonempty native `PLUGIN_ROOT` is authoritative, even if missing. Restoring that exact path preserves the version selected by the host; falling through to an unrelated `CLAUDE_PLUGIN_ROOT` would change the code being executed.

Example for the confirmed personal marketplace source:

```bash
python3 /home/charl/plugins/moriarty-dev/scripts/moriarty_dev/deployment.py install \
  --plugin /home/charl/plugins/moriarty-dev --marketplace personal
```

The installer runs from source, independently of cache/runtime availability. It does not delete backups or pinned runtimes. Receipt JSON and exact snapshots remain under `~/.local/share/moriarty-dev/backups/upgrade-*`. Interrupted legacy restoration can be recovered from those snapshots using the source module's `restore_snapshot`; do not overwrite conflicts. `try/finally` does not survive SIGKILL or power loss. New registrations can still use their retained runtime during an interrupted installation. Legacy registrations remain dependent on their old cache paths until their sessions end. Atomic directory publication is not a claim of filesystem crash durability.

Doctor additionally reports `runtimePin`, `sourceRuntimeMatches`, and the retained runtime path/digest observation. These fields never promote `hostCoverage`, activation or trust. Retained runtimes accumulate intentionally; retirement requires a separate decision after all dependent sessions end.

## Tests from source and from a cache

Full repository integration suite (requires retained OpenSpec/evidence fixtures):

```bash
python3 -m unittest discover -s plugins/moriarty-dev/tests -v
```

Self-contained compatibility suite, from any copied plugin root:

```bash
python3 -m unittest discover -s /absolute/plugin/root/tests -p test_compatibility.py -v
```

The package compatibility suite checks JSON contracts, diagnostics and package coexistence. The additional self-contained `test_upgrade.py` suite invokes actual generated registrations after cache removal and simulates installer deletion, failure, partial removal and conflicts. These controls do not establish real host interception. Running all repository integration tests inside a cache fails because retained repository fixtures are outside the plugin; do not copy production state into a cache to make that invocation pass.

The [host smoke procedure](tests/host-smoke.md) records runtime evidence. Each receipt must identify host version, installed source/hash, tested tool path, actual hook completion/denial and marker outcome. Unit tests, reviews, and cache scans cannot promote themselves to verified interception.

## Historical operational data

The shared Git common-directory database is the history boundary. Readers accept either checkout or common-directory identity and include historical rows written under main, linked and removed-worktree paths. The same ownership rule retains pending outbox notifications after a worktree is removed and permits idempotent re-observation from another linked checkout. Rows and schemas are not migrated or rewritten. A database from an unrelated repository is not accepted for that history request.

Approval ingestion still accepts receipts without resolution IDs, but they no longer clear blockers. To resolve findings, provide a list of nonempty exact IDs in `resolvedFindings` (legacy alias `resolved`). An explicit current-name field takes precedence over the legacy alias. Missing, empty or malformed historical lists clear nothing; malformed new lists are rejected before writes. A named resolution may refer to a stable finding from an earlier candidate; candidate changes alone never erase findings. Old broad approvals may therefore expose blockers again, requiring a fresh scoped resolution.
