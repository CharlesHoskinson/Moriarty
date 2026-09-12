# Moriarty plugin compatibility review result

Reviewed candidate: `0.1.0+codex.20260911184750`. Source is integrated in `/home/charl/Moriarty/plugins/moriarty-dev`, mirrored to the confirmed local marketplace source, and installed through `codex plugin add`. The isolated review checkout is `/home/charl/Moriarty-plugin-compatibility-20260911`.

## Changes

- Hook command recognition distinguishes supported CLI `run --action` and exact raw argv from harmless action-ID mentions and review ingestion.
- `hook_protocol.py` separates bounded UTF-8 decoding and supported event outputs from lifecycle/policy handling. Malformed input gets a diagnostic before repository work; Stop is `{}`; allowed calls omit the decision; denial survives output compaction.
- `compatibility.py` supplies consistent read-only doctor/report observations of source, cached, explicitly expected and environment roots. `--json` works before or after the subcommand. Installation observations never imply host trust.
- Sprint completion requires completed prerequisite sprints; missing/cyclic dependencies remain open.
- Shared database ownership preserves historical failures, admin intervals and pending notifications across main, linked and removed worktrees, without migrating rows. Unrelated repository reads/notifications stay isolated.
- Approvals clear only explicit validated stable finding IDs. Missing/empty resolution lists clear nothing; malformed new lists are rejected. Previously broad historical approvals may expose blockers again; see COMPATIBILITY.md before relying on old receipts.
- COMPATIBILITY.md specifies Python/OS/host boundaries, copied-package testing, retained-cache upgrades and rollback. Existing hook definitions and their native-root precedence remain unchanged.

## Verification

- Baseline: 176 tests passed. New regressions were observed failing before their fixes; logs retained.
- Integrated repository suite: 199 tests passed (`plugin-tests-integrated.txt`).
- Installed standalone compatibility suite: 23 tests passed (`installed-package-tests.txt`); copied package also passed. Full integration tests still require retained repository fixtures.
- Plugin manifest validation and scoped `git diff --check` passed. All 32 candidate files match in the main source and installed cache (`candidate-02.json`, `final-integrity.json`).
- Actual Codex 0.154.0 before/after events show the harmless `printf compatibility-denied-marker` changed from blocked to successful. The exact registered `python3 marker.py` remains blocked; the marker is absent; `pwd` and Stop complete. Current-version hook source paths are recorded. This proves only the exercised host paths (`host-comparison.json`, `host-*-events.json`, `host-*-result.json`).
- Both older cache roots `0.1.0+codex.20260911061236` and `0.1.0+codex.20260911181517` remain byte-identical. Codex hooks.json and config.toml hashes are unchanged. No cache retirement, trust change or campaign dispatch occurred. Routine mandated status inspection may append existing admin accounting; no history migration or production notification mutation was performed.

## Independent reviews

- Initial Astra/Grok reviews and all finding dispositions are retained. Candidate-01 Astra requested the removed-worktree outbox correction; candidate-02 includes it.
- Fresh Astra medium approved all 32 candidate hashes and independently reran 199 tests (`astra-final-02.md`).
- Grok 4.6 high source audit approved runtime/tests and completed omitted guidance/fixture coverage (`grok-final-03.json`, `grok-final-supplement.json`). Returned runtime identity is `grok-4.6-build`; an inconsistent prose self-description is preserved rather than substituted for runtime metadata. Grok reviewed supplied source; it did not independently execute tests or hash the filesystem.
- Two cancelled Grok attempts had no approval authority and are retained.
- Installed-result audits: Astra independently verified source/cache/config hashes and reviewed host/test records (`astra-installed-result-review.md`); Grok approved the supplied actual host/integrity observations with source-only limitations (`grok-installed-result-review.json`). Both approve the scoped installed result.

## Use and remaining limits

Start a fresh Codex thread to load the new plugin version. Existing sessions can continue using their preserved cache paths. The main source CLI contains the revised guards now. Doctor deliberately remains `installed-unverified`; separate host receipts supply scoped runtime evidence.

Native Windows shell/runner support, other-host transcript adapters, macOS host testing and portable Foreman launcher configuration remain outside this candidate. `nonblocking-followups.md` retains other reviewed cleanup opportunities.
