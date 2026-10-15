# Opus 5.5 plugin reconciliation and security-document corrections — 2026-10-14

**Author:** Opus 5.5 (security owner). **Worktree:** `/home/charl/Moriarty/.worktrees/spec-wiki-plugin-reconcile-20261014`, HEAD `f14df735dbcb56eb21e3b9584e92f3744748f36d`, plus uncommitted edits. Nothing was staged, committed, pushed or installed. The candidate is the working tree described below; it needs a fresh independent review before root integrates or installs it.

**Native authority: 0.** No SDK relation/circuit check, k/model/degree query, VM/transcript, keygen, prove, verify, deploy or ledger action was run. No private key, PK, SRS, witness, proving cache or provider reasoning was read. The ledger-driver reproducer and the Foreman-launcher suite were not run. No installer, user config, mutable source, installed cache, retained runtime, counter or state DB was modified or read beyond the file hashing listed under provenance.

## 1. Result

The repository package `plugins/moriarty-dev/` now carries the hardened runtime that is already deployed as `0.1.0+codex.20260912173932`. Every runtime Python file is byte-identical to the mutable source and installed cache. The runtime digest is `0b40d8c3e184f246294188d89107f70bb2b38789de2276fa374792b955b4acb3`, which equals the pin in `hooks/hooks.json`. That file is byte-identical to the canonical registration that `deployment.definition()` generates for this runtime; a new package test enforces this.

The current guidance (manifest `0.1.1`, SKILL, execution reference, `test_guidance.py`) is kept, with findings 9 and 10 applied. The README merges the hardened-runtime text back in and replaces the bare `codex plugin add` procedure with the existing `deployment.py prepare`/`install` path. All eleven findings of the frozen independent review are dispositioned in §4. Five further defects found during reconciliation are fixed (§5).

Acceptance is not claimed: no product, security, milestone or host-interception verdict follows from this work.

## 2. Source provenance and file dispositions

Sources: mutable `/home/charl/plugins/moriarty-dev` and installed `/home/charl/.codex/plugins/cache/personal/moriarty-dev/0.1.0+codex.20260912173932`. Byte equality of the two comes from `plugin-pre-reconciliation-inventory.json`. I re-hashed the mutable source and the repository package myself; I did not re-hash the installed cache. The mutable source was re-hashed after the work and is unchanged.

"Before" is repository HEAD `f14df735`. "Deployed" is the mutable/installed bytes.

| Path (under `plugins/moriarty-dev/`) | Before | Deployed | After | Disposition |
|---|---|---|---|---|
| `.codex-plugin/plugin.json` | `c51dda89…` | `0ea9e424…` | `c51dda89…` | Repository kept (version 0.1.1, current metadata). Not part of the runtime pin |
| `COMPATIBILITY.md` | absent | `9e5fdf4a…` | `50f8ca60…` | Copied, plus one sentence (§3 A1) |
| `README.md` | `130f8a12…` | `9803595f…` | `121697fb…` | Merged (§3 A2) |
| `hooks/hooks.json` | `e379a1b6…` | `dd2515bd…` | `dd2515bd…` | Exact copy |
| `scripts/moriarty_dev/bootstrap.py` | absent | `957b5baa…` | `957b5baa…` | Exact copy |
| `scripts/moriarty_dev/cli.py` | `7588e0fe…` | `01305ac6…` | `01305ac6…` | Exact copy |
| `scripts/moriarty_dev/compatibility.py` | absent | `2e18c10f…` | `2e18c10f…` | Exact copy |
| `scripts/moriarty_dev/deployment.py` | absent | `ec0a9c2f…` | `ec0a9c2f…` | Exact copy |
| `scripts/moriarty_dev/hook.py` | `5c004808…` | `c185d2f0…` | `c185d2f0…` | Exact copy (mode 0755 kept) |
| `scripts/moriarty_dev/hook_protocol.py` | absent | `1cf8edcc…` | `1cf8edcc…` | Exact copy |
| `scripts/moriarty_dev/store.py` | `a4a7d058…` | `66417c0a…` | `66417c0a…` | Exact copy |
| `scripts/moriarty_dev/{__init__,notifications,policy,records,runner}.py` | = deployed | = before | unchanged | Already identical |
| `skills/develop/SKILL.md` | `4cfdd6aa…` | `319ce19b…` (older guidance) | `dfd24c56…` | Repository kept, plus finding 10 |
| `skills/develop/references/execution-focus.md` | `ec8ec4f0…` | `53a6eb5e…` (older guidance) | `27b12fb8…` | Repository kept, plus finding 9 |
| `tests/host-smoke.md` | `9974130c…` | `9c193dc2…` | `9c193dc2…` | Exact copy |
| `tests/test_compatibility.py` | absent | `186eb6cb…` | `186eb6cb…` | Exact copy |
| `tests/test_upgrade.py` | absent | `71d10552…` | `71d10552…` | Exact copy |
| `tests/test_execution.py` | `a92c0596…` | `812d4993…` | `812d4993…` | Exact copy (`resolvedFindings` in approval fixtures) |
| `tests/test_hooks.py` | `d4d8f205…` | `93c5402a…` | `93c5402a…` | Exact copy (no explicit allow; Stop `{}`) |
| `tests/test_host_adapter.py` | `f3627722…` | `736b261c…` | `736b261c…` | Exact copy |
| `tests/test_guidance.py` | `5b3ff16b…` | absent | `5b3ff16b…` | Repository kept (the nine guidance tests) |
| `tests/test_registration_pin.py` | absent | absent | `02e0e56f…` | New read-only test (§3 A3) |
| Other tests and fixtures | = deployed | = before | unchanged | Already identical |

Full 64-hex digests are in the JSON manifest.

Other edited files, with HEAD git blob → working-tree SHA-256 after:

| File | HEAD blob | After SHA-256 |
|---|---|---|
| `AGENTS.md` | `114c57d2` | `44ef60e5…` |
| `openspec/changes/consolidated-language-kernel/security-reconciliation-2026-10-14.md` | `35c65655` | `8398a78c…` |
| `wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md` | `02a0b9fb` | `5f688380…` |

The root's uncommitted A/B/E application already existed in the last two files and in `ALIGNMENT-REVIEW-2026-10-01.md` before my edits. I kept all of it, and I did not capture a SHA of that intermediate state.

## 3. Adaptations beyond exact deployed copies

- **A1. `COMPATIBILITY.md`.** Added one sentence after the full-suite command. While native authority is zero, the runner, ledger-driver and live-budget tests listed in the README must first be excluded. Reason: the deployed text recommends the unfiltered suite, which would import a Midnight ledger driver, start the external Foreman launcher and read live budget/ledger state. Everything else is exact.
- **A2. `README.md`.** Kept the current-guidance base. From the deployed README I restored:
  - the `resolvedFindings` approval rule (the repository text described the weaker pre-hardening behaviour);
  - the "Compatibility and upgrades" section;
  - the Host adapter response contract.
  I also changed the Coverage limitation to describe the hardened recognizer (simple CLI `run --action` forms and exact registered argv). "Validation and installation" now:
  - lists the registration test;
  - names the three tests that are not ordinary;
  - requires the `deployment.py prepare`/`install` upgrade path, and says why a bare `codex plugin add` is unsafe.
- **A3. `tests/test_registration_pin.py`.** This is a new read-only test, not a runtime mechanism. It asserts that:
  - `hooks/hooks.json` equals `deployment.definition(PLUGIN, runtime_digest(PLUGIN))` in both JSON and bytes;
  - every event embeds `-I`, the exact `bootstrap.py` source, one pin and its event;
  - `REQUIRED_MODULES` are present.
  It fails if any runtime Python byte changes without `prepare`, so a stale hard-coded pin cannot pass. Reason: the copied `test_upgrade.py` regenerates registrations on a copy, so it never checks the checked-in pin.
- No runtime Python byte, registration or mechanism was changed or redesigned. `deployment.py` stays the only upgrade path.

### Runtime difference review (repository before → deployed, now canonical)

| Area | Repository HEAD behaviour | Hardened behaviour now in the repository | Security effect |
|---|---|---|---|
| Hook launch | `python3 ".../hook.py"`, no isolation or pin | Embedded `python3 -I` bootstrap. Pins the complete `.py` inventory by SHA-256; rejects added files and symlinks; falls back only to `~/.local/share/moriarty-dev/runtimes/<pin>`; emits a bounded diagnostic otherwise | Stronger: no PYTHONPATH/user-site shadowing; survives cache eviction without running another version |
| Input bound | Reads 8,193 chars; oversize or malformed input becomes `{}` and **continues into repository handling** using `os.getcwd()` | 8 MiB UTF-8 byte limit; malformed, oversize, non-object or invalid `cwd` input gets a diagnostic before repository access | Stronger |
| Permit response | Explicit `permissionDecision: "allow"` | Decision omitted; the host owns permissions | Stronger (never grants) |
| Stop | `hookSpecificOutput` envelope | `{}` | Host-compatible; no continuation |
| Denial recognition | Denies when any token equals an action ID (catches wrapper forms such as `env python3 cli.py run --action X`, and false positives such as `echo X`) | Denies simple CLI `run --action` forms and exact registered argv only | Narrower early warning, by deliberate documented design (COMPATIBILITY.md:25). The CLI `run` gate still applies `assess()` to every invocation, including wrapper forms, so no registered dispatch is newly admitted. Recorded as a behaviour change, not a weakness of the gate |
| Review ingestion | `APPROVED` without a resolution list **clears every blocker** | Only named, nonempty `resolvedFindings`/`resolved` IDs clear; malformed lists are rejected before any write | Stronger |
| Store ownership | History keyed by IDs derivable from the checkout | Verifies the DB is this repository's common-dir DB, then reads every row in it, including removed-worktree rows; `enqueue_tx` checks ownership first | Stronger: blockers survive removed worktrees; foreign DBs are rejected |
| Sprint report | Complete when stages are done and `completionRequires` is empty | Least fixed point; unknown or cyclic dependencies never complete | Stronger (no synthetic progress) |
| Doctor/report | Cache-directory heuristic | Read-only `compatibility` observation; `activation` and `hookTrust` stay `unknown`; coverage never promoted | Equal or stronger |

Every native, admission, notification and public-receipt control from the repository version survives: `policy.py`, `records.py`, `runner.py` and `notifications.py` are unchanged, and the store and hook changes only tighten.

## 4. Disposition of the independent review findings (frozen review on `f14df735`)

| # | Finding | Disposition |
|---|---|---|
| 1 High | Refresh would downgrade the installed runtime | **Resolved.** The hardened runtime is reconciled into the repository with exact bytes. The README no longer prescribes a bare refresh. §7 gives the reviewed upgrade command. A pin-consistency test was added |
| 2 Medium | Dossier README overstates review status | **Handed to root** (dossier is root-owned; not edited). §8 has the replacement paragraph. The review's own suggested sentence on the coverage continuation is now out of date: that continuation (`OPUS-SECURITY-COVERAGE.md`) has completed. Root should state its actual status |
| 3 Low–Med | OSR-08 refuses all unsigned artifacts | **Resolved.** Refusal now applies to "any artifact with more than one public payer", and an explicit line says a legal single-payer unsigned artifact is not refused but still carries no authority (SEC-20) |
| 4 Low | Stale anchors in the wiki page | **Resolved.** README anchors were already applied by root (B5). The AGENTS anchor is now `AGENTS.md:107`, because the finding-11 insertion moved line 106 |
| 5 Low | Missing "Maps to" predicates | **Resolved and extended.** Added: OSR-02 U2, M2, U7 and the G gates (SEC-16 also lists G gates, which the review's table missed); OSR-03 U2; OSR-04 M17; OSR-05 M13; OSR-06 M9, U0. Also OSR-01 U0 (time), from SEC-01. OSR-12 now says SEC-04 supplies only the compiled-key premise |
| 6 Low | OSR-11 evidence overstated | **Resolved.** Evidence is split per scenario: B1 covers only the pinned loader; there is no current control for caller parameters, and key generation is blocked-SDK-native. "No current-local control exists" for the oversized case. The scenario also wrongly called 64 KB a transaction ceiling; per SEC-14 those 64 KiB bounds are JSON/envelope/source only, so the threshold is now the ceiling measured for the claimed target |
| 7 Low | Source readings stated as current facts | **Resolved.** `:113` and `:225` were already corrected by root (A3). OSR-13 now says double release is a hypothesis (SEC-18 HYP) |
| 8 Low | SEC-15 dual role | **Resolved** in the "no new requirement" list and in OSR-02's Source line |
| 9 Low | `prepare-call`/`prepare-signed-call` missing | **Resolved** in `execution-focus.md` (confirmed at SECURITY-REVIEW.md:124) |
| 10 Low | SKILL "linked" boundary has no link | **Resolved.** The link is in-package; the isolated-package link test passes |
| 11 Info | AGENTS frozen-gate and historical-routing wording | **Resolved.** Restored "Existing frozen signed-call native gates retain their Opus 5.5 and Muse audits." after line 9. Added "(superseded for security by the October 14 Opus assignment)" to the September 9 historical line |

## 5. Additional defects found and fixed

1. **Wiki B6 rows.** They were appended after a blank line, which produced a second Markdown table with no header that would not render. They are joined back into the "Statements to read as historical" table.
2. **ALIGNMENT-REVIEW refs.** `:54` and `:72` were stale after the root's E3 line join. They are now `:53` and `:71`, labelled "historical original-text section", with the `fc6353bf` originals `:17` and `:35`. These numbers depend on the root's uncommitted ALIGNMENT-REVIEW edit; re-check them if that edit changes.
3. **COMPATIBILITY full-suite instruction** was unsafe under zero native authority (§3 A1).
4. **README review-ingestion guarantee** described the weaker pre-hardening semantics (§3 A2).
5. **README Coverage limitation** described the old action-ID recognition (§3 A2).

The coverage handoff is recorded in both security documents: 64 OpenSpec entries, plus 34 authored wiki paths and 8 exact claims, inspected; not an exhaustive audit of all prose, fenced contracts, private material or raw captures.

## 6. Verification receipts

All commands ran from the worktree root using Python 3.14.4.

| # | Command | Result | Notes |
|---|---|---|---|
| V1 | `python3 -m unittest discover -s plugins/moriarty-dev/tests -p test_guidance.py -v` | Ran 9, OK | Before the guidance edits; re-run inside V3 after all edits |
| V2 | `python3 -m unittest discover -s plugins/moriarty-dev/tests -p test_registration_pin.py -v` | Ran 3, OK | Proves pin = runtime digest and canonical registration bytes |
| V3 | `python3 -m unittest discover -s plugins/moriarty-dev/tests -v -k 'test_campaign_history.*' -k 'test_compatibility.[!I]*' -k 'test_execution.*' -k 'test_guidance.*' -k 'test_hooks.*' -k 'test_host_adapter.*.test_[!d]*' -k 'test_notifications.*.test_[!p]*' -k 'test_policy.*' -k 'test_records.GenuineRegisters.*' -k 'test_records.UnsupportedShapes.*' -k 'test_records.AdmissionTraversal.test_[!a]*' -k 'test_records.AdmissionTraversal.test_acc*' -k 'test_records.AdmissionTraversal.test_acti*' -k 'test_registration_pin.*' -k 'test_regressions.*.test_[!r]*' -k 'test_store_connections.*' -k 'test_upgrade.*' 2>&1 \| tail -n 200` | **Ran 211 in 76.838 s, OK** | Output was piped to `tail`, so the pipeline exit status reflects `tail`; the unittest summary was `OK` with no failures or errors. 231 test methods minus 20 exclusions = 211 |
| V4 | `git diff --check HEAD` and `git diff --no-index --check /dev/null <each new file>` | No output | No whitespace errors |
| V5 | JSON/AST | Covered by V2 and V3 | `hooks.json` and `plugin.json` are parsed by tests. `validate_source` (in `test_upgrade`) compiles every runtime module. A standalone `json.tool` call and my pin script (`opus-plugin-pin-check.py`) were blocked by session permissions and **not executed**; V2 supersedes the script |
| V5b | `sha256sum --strict -c OPUS-PLUGIN-RECONCILIATION.sha256` from `plugins/moriarty-dev` | 37/37 OK | 29 tracked plus 8 new files; the manifest root uses in §7 |
| V6 | Re-hash of the mutable source after work | Unchanged | `hooks.json` `dd2515bd…`, `hook.py` `c185d2f0…`, `store.py` `66417c0a…`, README `9803595f…`, manifest `0ea9e424…` |

### Inspected and excluded, not executed (20 tests)

- **`test_runner.py` (11).** Starts the external Foreman launcher `/home/charl/foreman/.../foreman-launch.js` through `node`. That is outside the package and its side effects were not inspected.
- **`test_regressions…test_reproducer_demonstrates_defect` (1).** Imports and runs a Midnight local financial ledger driver (`run-local.mjs`). This is product/ledger.
- **`test_records…AdmissionTraversal.test_actual_master_family_admits_review` (1).** Reads live `~/.local/state/moriarty/...` budget, binding and ledger files.
- **`test_compatibility.InstallationCompatibilityTests` (5), `test_host_adapter…test_doctor_cannot_infer_interception_from_file_existence` (1), `test_notifications…test_public_cli_observation_reports_and_updates` (1).** CLI `doctor`/`report` hashes the real retained runtime under `~/.local/share/moriarty-dev/runtimes/<pin>`. Setting a temporary `HOME`, `cd` and env prefixes are not permitted in this session, so these were not run rather than reading that runtime. Root can run them with `HOME` set to a temporary directory.

### Side effects of V1–V3

- Temporary directories, git repositories, SQLite stores, fake caches, runtime homes and stub installers were created through Python `tempfile`, and the tests cleaned them up.
- Gitignored `__pycache__` directories were written under `plugins/moriarty-dev/{scripts/moriarty_dev,tests}`. Neither the runtime pin nor `deployment.files()` includes them; exclude them when syncing.
- Real `HOME` was inherited. The registered-hook tests resolved through `PLUGIN_ROOT`/`CLAUDE_PLUGIN_ROOT` to the repository package, whose digest matches the pin (V2), so the retained-runtime fallback was never selected.
- No test read the worktree's `.moriarty-dev/actions.json`: every hook invocation without a temporary `cwd` fails input validation before repository access.

### Not established

- Real host interception or permission behaviour.
- Installer behaviour against the real Codex host.
- `test_compatibility` run from a separately copied plugin root. A recursive copy needed approval; `test_upgrade` and `test_guidance` do exercise internal package copies.
- Byte equality of the installed cache, which I did not re-hash (the inventory supplies it).

## 7. Reviewed upgrade command for root (after an independent review of this exact candidate)

This copies the reviewed package into the configured source, then uses the existing source installer. That installer snapshots and restores every retained cache root, including `0.1.0+codex.20260912173932`, and keeps pinned runtimes. Do not bump the manifest, run a bare `codex plugin add`, edit caches by hand or change trust settings.

```bash
set -eu
REPO=/home/charl/Moriarty/.worktrees/spec-wiki-plugin-reconcile-20261014   # or the integrated checkout
SRC=/home/charl/plugins/moriarty-dev
STAMP=$(date -u +%Y%m%dT%H%M%SZ)
BACKUP=$HOME/.local/share/moriarty-dev/backups/mutable-source-0.1.0+codex.20260912173932-$STAMP

# 1. Preserve the current mutable source (equals the installed 0.1.0+codex.20260912173932 package).
mkdir -p "$(dirname "$BACKUP")"
cp -a "$SRC" "$BACKUP"

# 2. Synchronize the reviewed package (no bytecode).
rsync -a --delete --exclude '__pycache__/' "$REPO/plugins/moriarty-dev/" "$SRC/"

# 3. Verify every file against the reviewed manifest; stop on any mismatch, missing or extra file.
MANIFEST=/home/charl/Moriarty/.moriarty-dev/runtime/spec-wiki-plugin-reconcile-20261014/OPUS-PLUGIN-RECONCILIATION.sha256
(cd "$SRC" && sha256sum --strict -c "$MANIFEST")
test "$(find "$SRC" -type f -not -path '*/__pycache__/*' | wc -l)" -eq 37

# 4. Regenerate/confirm registrations and publish-or-reuse the pinned runtime.
python3 "$SRC/scripts/moriarty_dev/deployment.py" prepare --plugin "$SRC"
#    Expect {"runtimeDigest": "0b40d8c3e184f246294188d89107f70bb2b38789de2276fa374792b955b4acb3"}
sha256sum "$SRC/hooks/hooks.json"
#    Expect dd2515bd0be1f7b3f61bda9b59447fc1dac02761e2b7e6db07240deb6c4b7cea; otherwise stop.

# 5. Install through the source utility (snapshots/restores retained roots before returning).
python3 "$SRC/scripts/moriarty_dev/deployment.py" install --plugin "$SRC" --marketplace personal
#    Add --retain-root PATH for any session-referenced root outside the Codex cache.
```

Afterwards:

- Read the receipt under `~/.local/share/moriarty-dev/backups/upgrade-*/receipt.json`. Report `installerReturnCode`, `restorationComplete` and `installedVerified` exactly.
- `installedVerified` checks `plugins/cache/personal/moriarty-dev/0.1.1`. If the host installs under another directory name, it will read `false`; report that and do not claim success.
- Run `python3 -m unittest discover -s <installed root>/tests -p test_compatibility.py -v` with `HOME` set to a temporary directory.
- Host smoke (`tests/host-smoke.md`) is a separate, authorized step. Until it is observed, keep `doctor` coverage `installed-unverified`.
- Rollback: restore `$BACKUP` to `$SRC` and re-run step 5. Old cache roots and retained runtimes are never deleted by this procedure.

## 8. Ready-to-copy paragraph for the dossier README

> **Plugin runtime scope (corrected 2026-10-14).** The plugin change is no longer guidance-only. Opus reconciled the hardened runtime already deployed from the personal source as `0.1.0+codex.20260912173932` into the canonical `plugins/moriarty-dev/`. This adds the pinned `python3 -I` hook bootstrap (runtime digest `0b40d8c3…`), the `bootstrap`, `hook_protocol`, `compatibility` and `deployment` modules, the hardened `hook`, `cli` and `store` modules, `COMPATIBILITY.md`, and the compatibility and upgrade tests. Runtime Python files and `hooks/hooks.json` are byte-identical to the deployed source. Manifest 0.1.1 and the current guidance are kept, and a new read-only test requires registrations to match the generated pin. Opus's receipts, exclusions and finding dispositions are in [OPUS-PLUGIN-RECONCILIATION.md](OPUS-PLUGIN-RECONCILIATION.md). Installation is pending an independent review of this candidate and must use the source `deployment.py install` path, not a bare `codex plugin add`. Host hook interception remains unverified, native authority remains zero, and no product or milestone acceptance follows.
