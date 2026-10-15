I couldn't write the plan file or call ExitPlanMode because those tools weren't available here, so the full review is below. I made no edits and ran nothing.

# Opus 5.5 independent security, currentness and plugin review: **REQUEST_CHANGES**

The plugin runtime, the registration pin and the upgrade path have no blocking defects. The only blocking finding is F1: OSR-1014-11 weakens your direct M5 requirement. Fixing it changes none of the 37 plugin files.

## Candidate identity
- **Commit:** `f2923e22afe4ae702697e77f47200b90ceffc80a`. The worktree HEAD points to `refs/heads/docs/spec-wiki-plugin-reconcile-20261014`, and that ref holds this commit.
- **Not proven:** I couldn't run `git status`, so I can't prove the working tree matches the commit.
- **Baseline:** `fc6353bf…`.
- **Source projection:** `1725b33f6c5d15a8f8f9c18c9d8c77d083771e47f1c0bad7b1ba9b922483ec8e`, from `integrated-source-manifest.json` and `final-verification.json`.
- **Plugin manifest:** the 37-line `OPUS-PLUGIN-RECONCILIATION.sha256` copy in the dossier matches the runtime-directory copy line for line.
  - The worktree's `plugins/moriarty-dev/` holds exactly those 37 files, plus `__pycache__` files that `.gitignore:2` ignores.
  - The mutable source holds 35 files and no bytecode.
- **Method:** Read, Grep and Glob only. I computed no hashes. Native authority stayed at 0. I read nothing private and no native or proving caches.

## Findings

### F1 — Medium (blocking): OSR-11 weakens your direct M5 requirement
- **Before:** the earlier text said "WHEN an encoded transaction exceeds the 64 KB ceiling…" (`OPUS-SECURITY-REQUIREMENTS.md:202`).
- **Now:** `security-reconciliation-2026-10-14.md:205` says "…exceeds the encoded-transaction ceiling measured for the claimed target". Line `:196` only asks that the transaction ceiling and fraud k "be measured". Neither line states 64 KB, k ≤ 19 or the per-contract replay budget.
- **Why this weakens M5:** a target whose measured ceiling is above 64 KB, or whose fraud k is above 19, would satisfy OSR-11 while breaking M5.
- **What M5 actually requires:**
  - `traceability.md:174` gives M5 as "64 KB transaction; fraud k ≤ 19; replay budget".
  - `SECURITY-REVIEW.md:153` says "measurement of the encoded transaction against the 64 KB ceiling".
  - SEC-14 (`SECURITY-REVIEW.md:74`) only says no code enforces that ceiling yet; it doesn't say the requirement is wrong.
- **What went wrong:**
  - The author's stated reason ("wrongly called 64 KB a transaction ceiling", `OPUS-PLUGIN-RECONCILIATION.md:102`, `.json:83`, `OPUS-PLUGIN-IMPLEMENTATION-PUBLIC.md:19`) confuses the missing code with the requirement.
  - The original finding 6 (`OPUS-INDEPENDENT-REVIEW.md:61`) only asked for the evidence line to be split by scenario.
- **Partial protection only:** line `:12` keeps M1–M17 unchanged, so M5 itself still stands. But a row that says "Maps to: M5" now reads differently from M5.
- **Correction:** replace `:196` with:
  > When M5 is claimed, the encoded transaction SHALL be at most 64 KB, every fraud circuit SHALL have k ≤ 19, and each contract SHALL declare and stay within its replay budget, each measured on the claimed target. Measurement supplies evidence; it SHALL NOT relax these M5 thresholds.
- Replace `:205` with:
  > **WHEN** an encoded transaction exceeds 64 KB (M5), or a lower ceiling measured for the claimed target
- Add to `:207`:
  > The existing 64 KiB JSON, envelope and source bounds do not implement this requirement, and their existence does not loosen it.
- **Unit:** keep your own "64 KB" wording. Don't equate it with the code's 64 KiB bounds.
- **Frozen reports:** leave the author reports frozen. Root should add a supersession note to the dossier README.

### F2 — Low: the post-install test guidance would write bytecode into the installed cache
- `COMPATIBILITY.md:78`, `README.md:101` and `OPUS-PLUGIN-RECONCILIATION.md:192` give no bytecode guard. Without one, `__pycache__` files appear inside the installed cache.
- The pin and the installer's file comparison both ignore bytecode, so nothing breaks functionally.
- These lines also recommend a temporary `HOME`, which root won't use. Use the command in "Post-install verification" below instead; no source change is needed for this candidate.

### F3 — Low (advisory): the installer adds roots from the environment
- `deployment.py:237` adds the invoking environment's `PLUGIN_ROOT` and `CLAUDE_PLUGIN_ROOT` as roots to retain.
- If root runs the installer from an agent session where these point at an unrelated or missing path, installation either aborts or snapshots an unrelated directory.
- Before `install`, record both values. Unset them unless they point at a `moriarty-dev` root.

### F4 — Info: the 211-test receipt doesn't name its candidate
- `opus-selected-tests-exit.json` doesn't record the commit or a manifest check.
- Its selection matches V3 exactly with the `tail` pipe removed: true exit 0, "Ran 211 tests… OK".
- Root's next receipt should record HEAD and the 37/37 manifest result alongside the test result.

### F5 — Info: wording copied verbatim from the deployed docs
- `COMPATIBILITY.md:46` says to bump the manifest. That's harmless here because 0.1.1 is already bumped.
- `COMPATIBILITY.md:21` and `README.md:88` cite different hook-reference URLs.
- `COMPATIBILITY.md:23` says "responses above 8 MiB" where it probably means inputs.
- None of these is blocking.

## What I verified
- **Runtime pin:**
  - `hooks.json` contains the pin `0b40d8c3…` once per event, four times in all.
  - The mutable source and the installed `0.1.0+codex.20260912173932` cache carry the same pin.
  - The retained runtime `~/.local/share/moriarty-dev/runtimes/0b40…` contains exactly the 12 required modules.
  - `test_registration_pin.py` requires `hooks.json` to match the generated registration byte for byte, and it passed in the 211-test run.
  - Because the pin is a SHA-256 over every `.py` path and its bytes, the repository's runtime Python is byte-identical to the 0b40 runtime. That's an inference; I didn't hash anything.
- **No downgrade:**
  - Version 0.1.1 is higher than 0.1.0+build, and the runtime is the same.
  - `policy.py`, `records.py`, `runner.py` and `notifications.py` are unchanged; they're absent from the changed-file manifest.
  - So native counters, authorization, notification and public-receipt gates are unchanged.
- **Execution gate:**
  - `cli.py:387-396` and `:417-446` call `assess()` on every `run`, before and after reservation.
  - The hook never grants permission (`hook_protocol.py:21-24`), and Stop returns `{}`.
  - `store.py:127-130` and `:711-713` only tighten behaviour.
- **Narrower early-warning recognizer:**
  - The old hook denied any command where a token equalled an action ID (`plugin-refresh-stage/.../hook.py:148`).
  - The new hook recognizes only simple CLI `run --action` forms (`hook.py:140-187`).
  - Exact registered-argv matching is unchanged (old `:150`, new `:211`).
  - `README.md:44`, `COMPATIBILITY.md:25`, `cli.py:653` (`hooksRegistered = False`) and the doctor's `activation`/`hookTrust` values of `unknown` all avoid claiming interception.
  - The claim "no registered dispatch is newly admitted" holds only for dispatches that go through the CLI. Wrapped raw commands that bypass the CLI were never gated, before or after this change. Live host interception remains unknown.
- **Upgrade keeps old caches:**
  - `deployment.py:157-212` snapshots every cache root, runs the installer, then restores in a `finally` block without overwriting anything.
  - It also covers every marketplace, the legacy root and explicitly retained roots (`:97-107`, `:236-240`).
  - The upgrade tests for these cases passed.
- **Earlier findings:**
  - Finding 2: fixed in the dossier README (`:69-87`).
  - Finding 3: `:150-153`.
  - Finding 4: the wiki anchors `:39` (pointing to `AGENTS.md:107`) and `:41-42` (pointing to `ALIGNMENT-REVIEW` `:53` and `:71`) match the target lines.
  - Finding 5: the missing "Maps to" predicates are added.
  - Finding 7: `:115` and `:230`.
  - Finding 8: `:44` and `:67`.
  - Finding 9: `prepare-signed-call` is now listed.
  - Finding 10: the link at `SKILL.md:35` resolves to `execution-focus.md:54`.
  - Finding 11: `AGENTS.md:10` and `:142`.
  - All 15 OSR clauses are specified-only with acceptance open. Apart from F1, none overstates its source.
- **Root's other checks:** `final-verification.json` records exit 0 and `all_pass: true` for strict OpenSpec validation, the retention tests, the 9 guidance tests, `git diff --check` and 388-line preservation.

## Post-install verification (run only if the receipt shows `installedVerified: true`)
`test_compatibility` is safe to run against the installed package without changing `HOME`.
- **Why:** all 5 `InstallationCompatibilityTests` set their own temporary `CODEX_HOME`. So do the other two excluded tests (`test_host_adapter:64` and `test_notifications:29`).
- **Real-`HOME` reads:**
  - The only intended one is hashing `~/.local/share/moriarty-dev/runtimes/0b40…/scripts/moriarty_dev/*.py` (`compatibility.py:55-58`).
  - The only other path to real `HOME` comes from inherited `PLUGIN_ROOT` or `CLAUDE_PLUGIN_ROOT` (`compatibility.py:90-92`); the command below unsets them.
- **Writes:** none under `HOME`. Stores, repositories and caches are all temporary.

```bash
INST="$HOME/.codex/plugins/cache/personal/moriarty-dev/0.1.1"
MANIFEST=/home/charl/Moriarty/.worktrees/spec-wiki-plugin-reconcile-20261014/docs/audits/2026-10-14/reconciliation/OPUS-PLUGIN-RECONCILIATION.sha256
(cd "$INST" && sha256sum --strict -c "$MANIFEST")
WORK=$(mktemp -d); cd "$WORK"
env -u PLUGIN_ROOT -u CLAUDE_PLUGIN_ROOT -u PYTHONPATH PYTHONDONTWRITEBYTECODE=1 \
  python3 -I -B -m unittest discover -s "$INST/tests" -t "$INST/tests" -p test_compatibility.py -v; echo "exit=$?"
find "$INST" \( -name __pycache__ -o -name '*.pyc' \) -print   # expect no output
(cd "$INST" && sha256sum --strict -c "$MANIFEST")
```

- **Expected:** "Ran 25 tests… OK", exit 0, no bytecode, and 37/37 before and after.
- **Other two tests:** if wanted, run `test_doctor_cannot_infer_interception_from_file_existence` and `test_public_cli_observation_reports_and_updates` from the repository checkout, with the same environment prefix and `-k` filters. Don't run them from the cache.
- **Excluded:** `test_upgrade`. I didn't read its body.

## Not covered by this review
- I computed no hashes.
- I didn't hash the installed cache. The mutable `cli.py`/`store.py` and the four unchanged modules rest on the pin inference above.
- I didn't fully read the bodies of `test_upgrade`, `test_execution` or `test_records`.
- The 20 excluded tests were not run.
- Live host interception is unknown.
- Native authority is 0.
- All U/M/MC/SP/G exits remain open.

Approval of a corrected candidate would cover only currentness, specification and plugin-upgrade source. It wouldn't be product or security certification, or a full roadmap or runtime audit. The fresh review can be limited to the OSR-11 diff, plus confirming the 37 plugin hashes are unchanged.
