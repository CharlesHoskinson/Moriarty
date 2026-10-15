I've brought the hardened plugin runtime that is already installed into the repo's `plugins/moriarty-dev/` and fixed all eleven review findings except #2, which belongs to you. A filtered run of 211 plugin tests passed. Nothing is staged, committed, pushed or installed, and native authority stayed at zero. The candidate needs a fresh independent review before you install it.

**Plugin**
- **Runtime:** copied byte-for-byte from the installed `0.1.0+codex.20260912173932` source:
  - the four new modules: `bootstrap`, `hook_protocol`, `compatibility` and `deployment`;
  - the hardened `hook`, `cli` and `store`;
  - `hooks/hooks.json`, `COMPATIBILITY.md` and the compatibility and upgrade tests.

  The repo now has the same hardening as the installed copy: Python isolation, bounded hook input/output, no explicit "allow" decisions, and approvals that only clear the findings they name.
- **Pin:** the hook registration is pinned to the runtime digest `0b40d8c3…`, which matches the copied files. A new read-only test, `tests/test_registration_pin.py`, fails if the runtime changes without regenerating the registration, so a stale pin can't slip through.
- **Kept from the repo:** manifest 0.1.1, the current SKILL and execution reference (now with fixes 9 and 10), and the nine guidance tests.
- **README:** merged. It restores the hardened-runtime sections and replaces the bare `codex plugin add` with the existing `deployment.py prepare`/`install` upgrade path.
- **One change beyond exact copies:** I added a sentence to `COMPATIBILITY.md`. Its "run the full suite" advice would otherwise launch a ledger driver and read live budget state.

**Security documents**
- **OSR addendum:**
  - OSR-08 now refuses only artifacts with more than one public payer.
  - The "Maps to" lists are complete, including SEC-16's G gates, which the review's table also missed.
  - OSR-11's evidence is split per scenario, and its "64 KB transaction ceiling" is corrected: per SEC-14 those bounds cover only JSON, envelope and source.
  - The double release in OSR-13 is labelled a hypothesis, and SEC-15's role is clarified.
- **Wiki currentness page:** stale line references are updated (`AGENTS.md:107`; `ALIGNMENT-REVIEW:53`/`:71`, which were `:17`/`:35` at `fc6353bf`). The rows you appended after a blank line formed a second, header-less table that wouldn't render; I rejoined them. Both documents now state the coverage scope.
- **`AGENTS.md`:** the frozen signed-call gate sentence is restored, and the September 9 routing line is marked superseded for security.

**Tests**
- **Passed:** 211 of the 231 test methods. The selection was a single `-k`-filtered `unittest discover` run.
- **Not run (20 tests):**
  - `test_runner.py` (11): it starts the external Foreman launcher.
  - One `test_regressions` test: it runs a Midnight ledger driver.
  - One `test_records` test: it reads live budget and ledger files.
  - Seven `doctor`/`report` tests: they would hash the real retained runtime under your home directory, and this session couldn't redirect `HOME`. You can run these with `HOME` pointed at a temp directory.
- **Not done:** installation, host hook interception (still unverified), and re-running `test_compatibility` from a separate package copy, because a recursive copy needed approval.

**For you**
- The dossier README is yours. Section 8 of the report has a ready-to-copy paragraph correcting the runtime scope. Note that the Opus coverage continuation the review called pending has since completed.
- Section 7 has the upgrade command. It backs up the current source and syncs the reviewed package, then checks all 37 files against the `.sha256` manifest. It runs `prepare` and confirms that `hooks.json` and the digest are unchanged. Finally it runs `deployment.py install`, which snapshots and restores old cache roots and keeps pinned runtimes.

Files are in `/home/charl/Moriarty/.moriarty-dev/runtime/spec-wiki-plugin-reconcile-20261014/`:
- OPUS-PLUGIN-RECONCILIATION.md
- OPUS-PLUGIN-RECONCILIATION.json
- OPUS-PLUGIN-RECONCILIATION.sha256
