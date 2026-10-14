# Plugin guidance reconciliation audit

Baseline `fc6353bf19b87a76410319c378efe529d52d23d9`; candidate plugin version **0.1.1**. Scope: nonsecurity guidance and metadata. No product acceptance, security verdict or installation success is claimed.

Inventoried all **28 baseline files**, recording byte/line counts and before/after SHA-256 values in `plugin-audit.json`. Four guidance/metadata files changed, plus one read-only guidance test. Unchanged code, hooks, tests and fixtures were inventoried/parsed and compared; they were not fully semantically or security audited.

## Changes

- `plugins/moriarty-dev/.codex-plugin/plugin.json`
- `plugins/moriarty-dev/README.md`
- `plugins/moriarty-dev/skills/develop/SKILL.md`
- `plugins/moriarty-dev/skills/develop/references/execution-focus.md`
- `plugins/moriarty-dev/tests/test_guidance.py`

Current request takes precedence over recovered I2/SP01 work. Current nonsecurity author/auditor are GPT-6.1 Sol high / Muse Wonder high; Opus owns all security. The reference keeps current profile/consumer distinctions, complete independent financial/work expectations, target scope and full U/M/MC/SP/G exits. Historical contracts and runtime records remain unchanged.

## Actual verification

- RED: 8 tests, 16 failing subcases, no errors before guidance edits (`plugin-guidance-red.json`).
- GREEN: 8 read-only guidance tests pass; no native/product/runtime code invoked by the new tests.
- AST: 20 Python files parse; JSON: 3 files parse; `git diff --check -- plugins/moriarty-dev` passes.
- Existing security/admission/notification-auth suite was not executed; Opus owns it. No external agent compliance trial ran.
- CLI `status --json` remains historical SP01/loan-swap-subset with stale binding/candidate inputs and missing current accounting/live resources; no pending transactions. This was not rewritten as roadmap progress.

## Canonical refresh procedure

1. After root review/integration, select the canonical merged commit in /home/charl/Moriarty; preserve any differing mutable source before synchronization.
2. Read-only discovery: codex plugin marketplace list --json; codex plugin list --marketplace personal --json.
3. Catalog /home/charl/.agents/plugins/marketplace.json resolves moriarty-dev to /home/charl/plugins/moriarty-dev, a real mutable directory separate from repo/cache.
4. From canonical merged repository: git archive --format=tar HEAD plugins/moriarty-dev | tar -xf - --strip-components=2 -C /home/charl/plugins/moriarty-dev
5. Compare every canonical tracked plugin file SHA-256 with the mutable source; inspect any extra local files separately. The archive uses committed files only, excluding ignored __pycache__ output.
6. Run codex plugin add moriarty-dev@personal --json, then codex plugin list --marketplace personal --json.
7. Confirm installed 0.1.1 and compare installed manifest, skill, reference and package digests with reviewed commit; start a fresh agent context and load refreshed skill.
8. Run repository CLI status/doctor only for their scoped observations; actual host hook trust/interception needs independent observed host smoke evidence.
9. No plugin install subcommand exists in inspected CLI 0.162.1. marketplace upgrade refreshes Git snapshots and does not publish this local directory. Do not copy directly into immutable cache.
10. Re-add behavior on this existing installation remains unexecuted; if it fails to refresh, preserve exact output and choose host-supported recovery rather than claim installation succeeded.

Installed cache remains `0.1.0+codex.20260912173932`; candidate manifest is `0.1.1`. Neither mutable marketplace source nor immutable cache was changed by this worker.

## Handoff and limits

- Opus review requested for restated zero-native/security ownership boundary in execution-focus.md and SKILL.md routing sentence. No new security analysis or control edits supplied.
- Guarded CLI admission, native gates/counters, receipt authentication and transaction delivery controls unchanged. Security test design/execution/review remains Opus-owned.
- Fresh independent Muse audit of exact integrated candidate pending parent
- Opus disposition of security boundary restatement pending parent
- Installed snapshot not refreshed; re-add update behavior untested
- Actual host hook interception/trust remains unverified
- Atomic U/M/MC/SP/G coverage and full product/native/Fenrir acceptance remain open; no completion score derived
- No empirical agent-compliance evaluation; literal guidance contracts only

The JSON includes exact command outputs, manifest, source provenance, per-file coverage/digests and these limitations.


## Packaging follow-up — isolated installed package

The earlier checkout-only link test missed a real installation defect: SKILL/reference links traversed outside the cached plugin into assumed repository ancestors. A new test copies the package into a disposable `TemporaryDirectory` outside a checkout and checks every relative Markdown link in SKILL/references for package containment and file existence.

RED: the isolated-package test failed on **13 links** before repair (actual CLI exit 1; `plugin-package-links-red.json` preserves the command and output). External repository references now use explicit checkout code paths; package-internal links remain relative. EARS scenario guidance remains explicit. No runtime/security behavior changed.

Final GREEN: **9 tests pass** (the original 8 plus isolated-package installation links). `git diff --check -- plugins/moriarty-dev` passes; AST/JSON parsing remains clean. `plugin-audit.json` preserves prior receipts, appends this follow-up and final command outputs, and refreshes every candidate file digest. The packaging test creates only a disposable copy, without touching a mutable marketplace source or installed cache. Installation refresh and independent final audits remain parent-owned.


## Final integration follow-up — current reconciliation entrypoints

Added explicit checkout code paths for `wiki-llm/README.md`, `wiki-llm/RECONCILIATION-2026-10-14.md`, `openspec/current-requirement-crosswalk.json` and the consolidated-language-kernel `reconciliation-2026-10-14.md`. The two current Opus-authored security addenda are source references only; no security analysis was supplied.

The existing current-requirements contract first failed on the four missing entrypoints (`plugin-entrypoints-red.json`, actual CLI exit 1). Final verification: **9 guidance tests pass**, including the isolated-package link test; all six new checkout paths exist, AST/JSON parsing remains clean and `git diff --check` passes. `plugin-audit.json` now holds the final candidate hashes, source provenance and appended command receipts. No runtime, index, commit, mutable marketplace or installed-cache changes were made. Plugin edits are finalized for the parent's candidate freeze.
