I've read both receipts, the two earlier Opus reviews, the upgrade script and the installer code; here is the verdict.

# Opus 5.5 review of the plugin upgrade and post-install test result: **APPROVE**

There are no blocking findings. I only read files (Read, Grep, Glob): I ran nothing, computed no hashes, edited nothing and read nothing private. I couldn't write a plan file or call ExitPlanMode because those tools aren't available here, so the review is below.

## Earlier approvals
I read both earlier reviews in full:
- **`OPUS-APPROVED-CANDIDATE-REVIEW.md`** approves `9bd7ccc1` and the upgrade script, with notes A1–A4. It says outright that it ran nothing and computed no hashes.
- **`OPUS-SECURITY-FINAL-REVIEW-R2.md`** returned REQUEST_CHANGES because of F1, which was later fixed. It also defined the post-install command.

Neither review approves anything that was actually run. This review is the first to look at actual results.

## Source identity
- **Commit:** both receipts record `source_candidate_commit` as `9bd7ccc1b5522b22cf633b46cdad080d24448f39`. The upgrade script refuses to run unless the worktree HEAD is that commit (`refresh-reviewed-plugin.py:10`).
- **37 files:** the script takes the plugin from the commit with `git archive`. It then checks that the staged copy (`:29`), the editable source (`:33`, `:49`, `:56`) and the new cache (`:54`) each exactly match the 37-line `OPUS-PLUGIN-RECONCILIATION.sha256`. Exact dictionary equality means no file is missing and none is extra, apart from `__pycache__`.
- **Old 35 files:** `original_cache_before` (35 entries) matches `installer_result.retained[0].files` entry for entry.
  - Comparing them with the 37-file manifest: 30 hashes are unchanged.
  - Five changed, all documentation or version files: `plugin.json`, `COMPATIBILITY.md`, `README.md`, `SKILL.md` and `execution-focus.md`.
  - Two are new: `test_guidance.py` and `test_registration_pin.py`.
  - All runtime `.py` files and `hooks.json` (`dd2515…`) are unchanged. That fits with `runtimeDigest` staying at `0b40d8c3…`.
  - `test_compatibility.py` and `compatibility.py` are the same files as in the old version.

## Upgrade receipt (`plugin-installation.json`)
- **Commands:** each ran as `python3 -I -B`, and each exited 0 with empty stderr.
  - `prepare` printed `runtimeDigest` `0b40d8c3e184…acb3`.
  - `install --marketplace personal` printed the same digest.
  - `codex plugin list` exited 0.
- **Installer result** (raw stdout matches the parsed `installer_result`):
  - `installerReturnCode: 0`, `restorationComplete: true`, `installedVerified: true`, `restorationErrors: []`.
  - The old cache `0.1.0+codex.20260912173932` was snapshotted to `upgrade-4961uzl7/0-…`.
  - In `deployment.py:205`, `installedVerified` means the installed cache's files exactly match the source's.
  - `deployment.py:242` makes the exit code depend on `installerReturnCode` and `installedVerified`.
- **Checks by the upgrade script itself:**
  - `installed_hashes_match`, `original_cache_preserved` and `source_hashes_match` are all `true`.
  - `installed_manifest_version` is `0.1.1`.
  - The script asserts all of these (`:63-68`), and also asserts that `installedVerified` and `restorationComplete` are `true`.
- **Plugin listing:** `moriarty-dev@personal` shows version `0.1.1`, installed and enabled, with its source at `/home/charl/plugins/moriarty-dev`.
- **Environment:** `PLUGIN_ROOT` and `CLAUDE_PLUGIN_ROOT` were both null before removal. `PYTHONPATH` was removed (`:36`), `PYTHONDONTWRITEBYTECODE=1` was set, and `HOME` and `CODEX_HOME` weren't changed. That resolves F3 and A2.
- **Bytecode:** the before and after searches found none in either the source or the new cache, which resolves A1.
- **Backup:** the old editable source was copied to `mutable-source-…-20261015T000811Z` with symlinks kept. This happened only after the script confirmed that the source and the old cache both matched the earlier 35-file inventory (`:18-19`).
- `native_actions: 0` and `host_interception: not-established`.

## Post-install test receipt (`plugin-post-install-verification.json`)
- **Command:** `python3 -I -B -m unittest discover -s/-t <cache 0.1.1>/tests -p test_compatibility.py -v`. Exit code 0.
- **Output:** "Ran 25 tests in 10.149s / OK".
  - Every test is listed as `ok`.
  - That includes all 5 `InstallationCompatibilityTests` (broken cache, coexisting versions, old root diagnosed, JSON flag, source/cache/doctor agree).
- **Environment:** `PLUGIN_ROOT`, `CLAUDE_PLUGIN_ROOT` and `PYTHONPATH` were removed, and `HOME_unchanged` and `CODEX_HOME_unchanged` are both `true`.
- **Hashes and bytecode:** `before_manifest_matches` and `after_manifest_matches` are `true`, with `files_checked: 37`. No bytecode was found before or after in the source or the cache.

## Non-blocking notes
- **N1 (Info): the test receipt's script isn't kept.** I found no script that produced `plugin-post-install-verification.json`. So its working directory (claimed to be temporary), its hash method and its `*_unchanged` flags are self-reported. They aren't backed by code I could read. The test output itself is concrete.
- **N2 (Info): two counts are constants.** `old_cache_file_count` and `new_cache_file_count` are `len(expected_old)` and `len(expected)`, not counts of the caches on disk (`:62`). The actual counts follow from the exact-match flags, so this is a naming issue only.
- **N3 (Info): the receipt doesn't list the 37 new cache hashes.** It records only the result of comparing them with the manifest. That's adequate, but someone reviewing later has to trust the script's check.

## Limits
- This approval covers only the scoped upgrade, the receipts' flags, hashes and the 25-test result, and the fact that the 37 source files are unchanged.
- I computed no hashes myself.
- It does not show that hooks actually intercept anything on the host. It does not accept any native, target, product, roadmap or cryptography milestone; all of those remain open and unqualified.
- The 20 tests excluded from the earlier 211-test receipt are still unrun, except for these 5 `InstallationCompatibility` cases. This is not evidence for all 231 tests.
