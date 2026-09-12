# Supplemental verdict: APPROVE  
# Combined candidate-02 audit: APPROVE

Provider identity: **grok-4.6-build**. This pass covers only the five files omitted from the first verdict. Runtime, tests, README, and COMPATIBILITY.md are unchanged and are not re-litigated except where these files must agree with them.

## Supplemental scope

| File | Role |
| --- | --- |
| `skills/develop/SKILL.md` | Agent workflow, not a gate |
| `skills/develop/references/execution-focus.md` | I2/F0/K guidance |
| `tests/fixtures/campaign-admission.json` | Frozen historical campaign store |
| `tests/fixtures/sp01-financial-contract-and-execution-admission.md` | Hash-bound sprint input |
| `tests/host-smoke.md` | Actual-host interception procedure |

## Skill text vs runtime contract

`SKILL.md` states the split the runtime already implements: CLI `run` is the dispatch gate; routing, focus, and reconciliation are agent duties, not new mechanical gates (SKILL.md:9–10, 19–20). That matches README and `COMPATIBILITY.md`.

Aligned points:

- Direct `cli.py run` is the fallback when hook trust is unverified (SKILL.md:20). Doctor remains `unverified` / `installed-unverified` and cannot self-promote (SKILL.md:89–93). Same as `cli.py:647-655` and `compatibility.py:76-78`.
- Receipt ingestion checks schema/identity and does **not** enforce the two-auditor roster (SKILL.md:80). Same as README.
- Missing auditors must not approve or silently substitute (SKILL.md:17).
- September 11 Astra-author / Grok 4.6 high + Astra medium override is scoped to this plugin update and does not rewrite general product routing (SKILL.md:15–16; execution-focus.md:94).
- Notify/deliver: dedicated `Midnight Preview transaction …` lines, host-observed delivery, no caller JSON, test IDs are not chain evidence (SKILL.md:37–45). Same as `notifications.py` and `cli.py` deliver.
- Exit codes 0/2/3/4 match `cli.py`. The skill’s gloss for 2 (policy) and 4 (child failure) is narrower than every CLI cause (reservation conflict, runner unavailable, unresolved completion). That is workflow shorthand, not a wire-contract error.
- “No synthetic progress” and I2 uncertified until MC05 (SKILL.md:22, 56; execution-focus.md:9) match report/sprint least-fixed-point behavior and `records.py` evidence profiles.

`execution-focus.md` is explicit that it is not a scheduler, gate, or evidence schema (line 3). Paths are checkout-relative, not cache-relative (line 3), which matches “do not copy production state into a cache.” Pressure table row “Installed skill describes these rules; claim host enforcement or measured speedup” (execution-focus.md:98–99) forbids the main residual risk of shipping this prose: treating guidance as interception or speedup proof.

The skill does not authorize PLUGIN_ROOT fallback, generic approval clearing, raw-argv launch, or host-trust edits.

## Fixtures vs admission

`campaign-admission.json` is schema `moriarty.campaign-admission/1`. Extra keys would fail `_validate_campaigns`; inspected fields sit in `CAMPAIGN_RECORD_KEYS`. Historical K fields (`admission`/`result`/`consumed`) are absent, so the closed K observer is not triggered on this register.

`sp01-loan-swap-grok-01` is the live test admission target: `stage` `rp01-mc02`, `status` `complete`, scope `RP01-MC02 specified design consistency only`, candidate `982062069f2146b98598e5a17ef214c4a0745b7adef4d00c0470575bd2fd748f`. That matches `test_records.LOAN_CANDIDATE` / `DESIGN_SCOPE`. Complete status cannot admit implement/repair/admin (`records.py` `_status_allows`); tests already assert that. No secrets. Timing disclosures record post-launch index writes rather than pre-dispatch proof.

`sp01-atomic-preparation-closure-01` and `sp01-atomic-fixture-01` omit `stage`. That is allowed for store validation; they are not the selected loan `admissionRef`. Selecting them would fail gate/stage match. Fail-closed.

The sprint markdown is the frozen hash-bound input named in `tests/fixtures/README.md` (`fdb586d9…`). It is data for candidate-04 / loan binding checks, not a plugin skill. Embedded “REQUIRED SUB-SKILL” text is historical document content, not candidate-02 runtime instruction. Plugin report completion does not read this file.

## Host smoke

`tests/host-smoke.md` matches `COMPATIBILITY.md`: protocol tests ≠ interception; retain host version, installed hash, tool path, denial, marker; Codex 0.154.0; existing trust; no internals rewrite; disposable harmless action; Stop must not continue; test outbox IDs are not chain evidence; doctor cannot become `host-verified`. It does not tell an operator to hide a missing native `PLUGIN_ROOT` or to substitute another cache.

This file is a procedure, not a receipt that this candidate was host-smoked.

## Combined audit

Nothing in the omitted set retracts the first APPROVE: PLUGIN_ROOT authority, Stop `{}` / systemMessage diagnostics, named `resolvedFindings`, shared common-dir store, outbox ownership-before-create, and CLI-as-launch-gate still hold. Skills and smoke text disclaim the claims the runtime correctly refuses to make. Fixtures are historical and fail-closed for implementation dispatch.

**Supplemental: APPROVE**  
**Candidate-02 overall: APPROVE**

## Limits

- Source-only. Suites 199 / 23 and 32 hashes are author-verified, not re-run or re-hashed here.
- Did not check fixture `bindingSha256` values against evidence binding bytes (those files are outside this dump).
- Did not treat host-smoke.md as an executed Codex 0.154.0 receipt.
- Skill prose cannot bind a non-compliant agent; that remains the documented security boundary.
