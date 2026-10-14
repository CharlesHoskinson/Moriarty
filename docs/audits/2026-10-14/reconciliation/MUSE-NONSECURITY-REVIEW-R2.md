# Muse nonsecurity review R2 — OpenSpec/EARS, wiki-llm and plugin reconciliation

## Verdict: APPROVE (nonsecurity scope only)

This verdict approves the nonsecurity documentation, requirements-currentness,
retention-validator and plugin-guidance content of the exact candidate below.
It is not a security approval, not a product/native/target acceptance, and not
a full runtime audit. Security coverage continuation remains pending under
Opus 5.5 and is expressly not counted as approved here.

## Candidate identity (exact)

- Audited commit: `f14df735dbcb56eb21e3b9584e92f3744748f36d`
  ("Preserve strict register compatibility and retain reconciliation evidence"),
  confirmed as worktree `HEAD` in
  `/home/charl/Moriarty/.worktrees/spec-wiki-plugin-reconcile-20261014`.
- Baseline: `fc6353bf19b87a76410319c378efe529d52d23d9` (PR50).
- Candidate parents: `b585967f` (initial reconciliation) plus the `f14df735`
  repair commit, which adds the committed dossier under
  `docs/audits/2026-10-14/reconciliation/`, restores the three machine
  registers, and adjusts `reconciliation-2026-10-14.md` (8 lines).
- Diff scope: 86 files, all under `AGENTS.md`, `docs/`, `openspec/`,
  `plugins/moriarty-dev/`, and `wiki-llm/`. No product source, runtime, SDK,
  formal-model, native or ledger code changed; source/runtime behavior is
  unchanged. Any later security-owned change to these bytes requires a scoped
  fresh review as applicable.

## Reviewer identity

- Requested: `muse-wonder-1.0`, high effort, fresh non-author session
  (`muse-review-r2-config.json`: provider meta, `--reasoning-effort high`,
  candidate `f14df735…`, no automatic retry).
- Returned identity observable in this session: Muse Code powered by
  `muse-wonder-1.0`. No separate provider returned-model receipt was
  observable inside the session, so no stronger returned-identity claim is
  made. Author (nonsecurity) requested identity: GPT-6.1 Sol, high effort.
- The first Muse run on `b585967f` was interrupted (exit 124, SIGTERM,
  no report) and is recorded as an interruption, not an approval, in
  `muse-initial-interruption.json` and the dossier. This R2 review is fresh.

## Scope and limits

In scope (reviewed in full where stated): `AGENTS.md`; the OpenSpec/EARS
reconciliation (`reconciliation-2026-10-14.md`, `requirements.md`,
`spec.md` UNI-018–024, `traceability.md`, `tasks.md`,
`current-requirement-crosswalk.json`, `openspec/sprints/verify.py` and
`test_verify.py`, sprint/entrypoint banners); the wiki-llm reconciliation
(`README.md`, `RECONCILIATION-2026-10-14.md`, maintained edits, audit
manifests); plugin guidance (`SKILL.md`, `execution-focus.md`, plugin
`README.md`, `.codex-plugin/plugin.json` v0.1.1, `tests/test_guidance.py`);
the committed reconciliation dossier; and the parent/author JSON/Markdown
reports in the runtime directory.

Excluded by instruction and not audited: all security analysis, judgments,
requirements merit and hostile test design (Opus 5.5 only, including the
OSR-1014 addendum content and `SECURITY-CURRENTNESS-2026-10-14.md` beyond
authorship/boundary labelling); native authority (zero throughout); any SDK
relation check, k/model/degree query, VM/keygen/prove/verify, ledger action;
private-key/PK/SRS/cache/witness/provider transcript reads; broad test
suites; product acceptance of any U/M/MC/SP/G exit. Implementation source
was read only to check documentation facts (profile constants, consumers,
stubs), not as a product or security audit. No candidate file was modified.

## Actual results reviewed and independently re-observed

Parent record `root-verification.json` (identical copy committed in the
dossier) and author reports `openspec-audit.json/.md`, `wiki-audit.json/.md`,
`plugin-audit.json/.md`, `ears-preservation-result.json` were read. The
following were independently re-run or re-derived in this session on the
frozen candidate, with matching results:

- `openspec validate --all --strict --no-interactive`: 14 passed, 0 failed.
- `python3 openspec/sprints/verify.py`: pass — 160 requirements, 12 sprints,
  21 stages, 239 original tasks, 45 sprint tasks, 18 lessons, 18 regressions,
  24 comparative cases, 24 legacy gates; scope labelled planning-structure
  only.
- `python3 openspec/sprints/test_verify.py`: 11 tests, OK.
- Plugin guidance tests (`test_guidance.py`): 9 tests, OK, including the
  isolated installed-package link test.
- Guarded CLI `status --json`: runs, reports no
  `unknown-program-field:currentGuidance`, preserves the historical blockers
  (stale SP01 binding/candidate inputs, missing current accounting,
  unavailable resource state, operational history) and no pending
  transactions. No blocker was waived and no acceptance inferred.
- Register byte identity: SHA-256 of `openspec/moriarty-completion-program.json`,
  `openspec/parallel-language-workflow.json` and `openspec/sprints/sprints.json`
  at baseline and candidate are pairwise identical; `currentGuidance` is
  absent from the candidate registers. The initial strict-loader rejection
  and the documentation-only repair are honestly recorded in
  `root-verification-initial.json`, `openspec-audit.json`
  (`compatibility_correction`) and the dossier README.
- `git diff --check` over the full baseline→candidate diff: clean.

## Substantive checks

1. Profiles and consumers are stated correctly. Rust source constants are
   `LANGUAGE = "moriarty/1"` and `SEMANTICS = "moriarty-core/1"`
   (`crates/mori-core/src/ir.rs:4-5`); the Node beta bridge is
   Source/6 → experimental TypeScript Core/5 with `PreparedUnqualified`
   local preparation (`packages/moriarty-beta/src/bridge.ts:1`). Docs keep
   the profiles, the two different lowerings (`mori-backend` transition
   compile vs `mori-ledger` key-free source-static export/reopen), consumer
   admission boundaries, and the Rust `mori lsp`/`mori mcp` `todo!` stubs
   distinct; a beta check is never presented as Rust compilation.
2. Preservation holds. The candidate `spec.md` begins with the complete
   baseline bytes, so UNI-001–017 and their original scenarios/SHALLs are
   unchanged; author mechanical checks retain 153 original canonical
   clauses, 659 checkbox lines, 239 mapped MC tasks, 24 release gates,
   ACTUS 277 fixtures / 18 executable types / 32 dispositions and DeFi
   72 rows, all still enforced by `verify.py` (target-family denominators
   and frozen-manifest membership are hard requirements, not prose).
3. UNI-018–024 (`spec.md:190` onward) map onto existing U/MC/SP/M owners in
   `traceability.md` and close through existing U0–U7 artifacts. They add
   profile/admission, scoped-lowering, independent-oracle, evidence,
   target-separation, retained-membership and developer-journey obligations
   without shrinking any parent exit; every crosswalk row keeps
   `acceptance: "open"`, which the validator enforces.
4. Denominators are honest. The crosswalk indexes 160 canonical `spec.md`
   headings and separately inventories the 15 Opus OSR-1014 headings in
   `separateAddenda`; both the crosswalk `qualification` field and the
   clause scope strings state this is not an atomic-predicate denominator,
   and no completion percentage is derived anywhere.
5. The strict validator is sound and usefully tested. It now pins the
   historical coverage file by SHA-256 (`verify.py:93`), requires the
   current crosswalk to equal the actual heading set with per-clause
   SHA-256, U-owner and open-acceptance checks, resolves the historical
   `docs/FOOTGUNS.md` report source only through the explicit snapshot
   whose SHA-256 must equal the original digest (`verify.py:167`), and
   retains all original task/family/report/gate checks. The Git-recovered
   snapshot at
   `openspec/source-snapshots/74027e6d…/FOOTGUNS.md` is byte-identical to
   commit `214141c1…:docs/FOOTGUNS.md` (SHA-256 `74027e6d…` re-derived in
   this session); maintained `docs/FOOTGUNS.md` was not relabelled or
   repinned. Three new mutation tests (`test_verify.py:89,96,101`:
   dropped current requirement, changed current clause, changed historical
   snapshot) cover the new failure modes.
6. Historical and report sources are treated correctly. Dated contracts,
   RESULT/REVIEW/receipt narratives, hash-bound mockup/horizon designs and
   frozen rosters keep original bytes, trees and reviewer identities;
   maintained entrypoints add dated currentness banners instead of editing
   history. `AGENTS.md` historical sections retain the October 10 and
   September 9 rosters under an explicit "current scoped routing controls"
   header, so no old Astra/Grok/I2 queue automatically displaces current
   work.
7. Current guidance lives in documentation only. Routing is GPT-6.1 Sol
   high (nonsecurity authoring), fresh Muse Wonder 1.0 high (nonsecurity
   audit), Opus 5.5 (all security), with two-vote consequential decisions
   kept separate; `SKILL.md`, `execution-focus.md` and the plugin README
   state this consistently, forbid new schedulers/campaigns/permissions,
   and keep native authority at zero. No machine register, runtime schema
   or admission code was changed to carry guidance.
8. Plugin package correctness. Version is `0.1.1`
   (`.codex-plugin/plugin.json:3`); skill/reference Markdown uses only
   package-internal relative links plus explicit checkout code paths, and
   the isolated-package test copies the package outside a checkout and
   proves every such link resolves inside it (13 broken links reproduced
   before repair, per `plugin-audit.md`). Context paths named in guidance
   (roadmap, audit plan, mid-roadmap review, EARS/traceability, wiki
   indexes, crosswalk, reconciliation docs) all exist in the checkout.
   Installation/refresh remains a parent action; the installed 0.1.0 cache
   snapshot is untouched and hook interception remains unverified, as
   stated.
9. Wiki scope is honestly bounded. Baseline inventories re-derived:
   1,871 wiki-llm paths (260 Markdown), 128 OpenSpec files, 28 plugin
   files. The wiki lane content-reviewed 44 nonsecurity authored records,
   delegated 41 security/mixed authored records to Opus, and left 3 frozen
   publication reports inventory-only; the ~87 KB of fenced horizon
   contract material and all native/private/raw/provider artifacts were
   not semantically revalidated, and the dossier/README state this rather
   than claiming every artifact was read. The committed `wiki-audit.json`
   `integration_refresh` digests for the four Opus-touched files match the
   candidate Git blobs exactly (re-derived in this session).
10. Open exits stay open. All U0–U7, M1–M17, MC/SP/G exits, full
    ACTUS/DeFi conformance, native/recursive/private-history and release
    acceptance remain open; Fenrir network/Anchor/committee/registry and
    target-tuple facts are left unknown with no invented configuration,
    and Preview evidence is never transferred to Fenrir.

## Findings

1. Minor (non-blocking) — stale navigation receipt commit:
   `docs/audits/2026-10-14/reconciliation/source-navigation.json:3` records
   `candidate_commit: b585967f…`, the pre-repair candidate. The receipt is
   honestly labelled with the tree it actually checked, and final-tree link
   coverage is independently supplied by `root-verification.json`, the
   guidance tests and this session's re-runs, so no claim about `f14df735`
   rests on it. Recommend the parent either refresh it on the frozen
   candidate or annotate it as a `b585967f` receipt when publishing.
2. Minor (non-blocking) — author-report copies differ across locations:
   the runtime `wiki-audit.json` is the pre-integration author copy, while
   the committed dossier copy refreshes the four Opus-touched file digests
   under `integration_refresh` (verified against candidate blobs above).
   Likewise `plugin-audit.json:7` retains `current_head` at the baseline
   (author-time HEAD). Neither misstates coverage, but readers should cite
   the committed dossier copies plus `root-verification.json` for exact
   candidate digests.
3. Scope note (not a defect) — Opus-authored security material in the
   candidate (`security-reconciliation-2026-10-14.md`,
   `SECURITY-CURRENTNESS-2026-10-14.md`, three family-README paragraphs,
   `OPUS-SECURITY-REQUIREMENTS.*`) was checked only for authorship
   labelling, separation from the 160-heading count and boundary wording.
   Its security merit, and the pending Opus security coverage continuation
   over the public handoffs, are outside this verdict and remain
   unapproved. The dossier correctly makes no full end-to-end or security
   audit claim, and the earlier runtime audit plan remains owed.

No blocking nonsecurity findings were found. Within the stated scope, the
reconciliation preserves every original requirement, task, gate and
membership obligation, keeps current guidance out of the consumed machine
registers, and reports its own coverage limits accurately.

— Fresh independent Muse Wonder 1.0 (high) nonsecurity audit of
`f14df735dbcb56eb21e3b9584e92f3744748f36d`; security judgments reserved to
Opus 5.5.
