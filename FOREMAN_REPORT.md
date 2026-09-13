# FOREMAN_REPORT

- run_id: moriarty-ll-loop-20260912
- role: plan
- slug: k-grounded-proposal
- branch: foreman/moriarty-ll-loop-20260912/plan/k-grounded-proposal
- worktree: /home/charl/Moriarty-wt-moriarty-ll-loop-20260912-plan-k-grounded-proposal
- base_sha: 056e662254c38936758af9c6f72c019705a1ff09
- status: complete

## Summary

Authored the grounded change proposal that obligation 127's first leg
requires, covering the retained macro05/parse04 trace106 SIGSEGV. Produced
exactly the two files at
`deliverables/language-to-ledger-2026-09-12/k-proposal/grounded-proposal-01.{md,json}`
via the Codex implement lane (model gpt-5.6-sol, reasoning medium) from a
five-part spec I authored after independently researching the repository's
evidence (pins, requisites, wiki claim CLM-0524, roadmap task 4.2, the prior
`k-failure-diagnosis.md`, and the live sp03-expression-k-macro05 worktree
artifacts). No K binary was run at any point, by me or by Codex. Obligation
127 is NOT claimed satisfied; no retry, audit, or admission was performed or
requested.

## Findings

- The retained evidence contains TWO distinct trace-106 SIGSEGV observations
  at different stages: **parse04** (default macro expansion, exit 139,
  SIGSEGV inside `kore-expand-macros` before configuration construction) and
  **macro05** (macro expansion disabled with `--no-expand-macros`, exit 113,
  SIGSEGV inside the LLVM `interpreter`, with a downstream `kore-print`
  "unexpected EOF" that is a consequence of the crashed interpreter's
  missing output, not independent evidence of a malformed input). The task
  brief's framing ("a segmentation fault in kore-expand-macros ... tracked as
  macro05 trace106") conflates the two; the proposal documents corrects this
  explicitly and respectfully, naming both stages precisely.
- CLM-0524 in `wiki/zkir/midnight-k-tooling.md` (sourced from
  `midnightntwrk/k-rust`'s `scripts/reference-differential.toml`, verified
  against the primary source at
  `/home/charl/Moriarty/repos/midnightntwrk/k-rust/scripts/reference-differential.toml`,
  HEAD `687ccd01e4708c495358c834c3685f297ecc8ac6`) documents that k-rust's own
  pinned canonical `runtimeverification/k` reference — release `v7.1.337`,
  commit `4a46d1231473b599c699160132fd6e76a5c46406` — excludes the
  `evm-execution` differential case because "the pinned reference execution
  path fails in kore-expand-macros before producing a configuration." Our
  own retained toolchain pins the IDENTICAL release and commit (not merely
  the same version). This means **parse04 is, at the failure-stage level, a
  member of an already-documented upstream exclusion class on the identical
  K build** — not a novel defect requiring its own from-scratch upstream bug
  report. This does NOT establish a shared root cause (KEVM's macro volume
  vs. our `ConstructNone` metadata-nesting depth are different plausible
  triggers), and it explicitly does NOT cover macro05's interpreter-stage
  crash, which CLM-0524 says nothing about.
- I agree with the prior diagnosis (`k-failure-diagnosis.md`) that this is
  not yet a proved stack-limit diagnosis, that `ConstructNone` is a
  supported constructor (ruling out a parser/grammar gap as the leading
  hypothesis), and that its own specified "smallest discriminator" diagnostic
  has NOT been run by anyone. My refinement is the CLM-0524 cross-project
  corroboration above and a sharper, explicit statement of the parse04-vs-
  macro05 stage distinction (which the prior document supports implicitly
  but states less crisply).
- Four options are documented in the proposal: (A) semantic-preserving
  iterative traversal / non-amplifying transport fix — recommended, does not
  narrow the language; (B) reviewed bounded stack increase for the exec-shim
  child only — explicitly secondary, not the primary recommendation, gated
  on the not-yet-run discriminator; (C) evaluate a newer pinned K release /
  upstream engagement; (D) explicit language narrowing as an owner-approval-
  only last resort, explicitly not recommended. The recommendation to the
  owner names Option A but is framed strictly as a recommendation, is
  contingent on the not-yet-run discriminator and on the fresh audits +
  root admission obligation 127 itself requires, and states the acceptance
  shape any repair must meet (retain depth-5947/Core-65536-byte boundary,
  reject depth-5948/65547 bytes, preserve all constructors and independent
  K typing, full correspondence + conformance).

## Evidence

- Spec given to Codex:
  `/tmp/claude-1000/-home-charl/b328e638-6326-4e27-a200-979fe93ee2ab/scratchpad/k-grounded-proposal-spec.md`
- Codex transcript: `/tmp/claude-1000/-home-charl/b328e638-6326-4e27-a200-979fe93ee2ab/scratchpad/codex-run.log`
- Deliverables (both untracked, uncommitted):
  - `deliverables/language-to-ledger-2026-09-12/k-proposal/grounded-proposal-01.md`
  - `deliverables/language-to-ledger-2026-09-12/k-proposal/grounded-proposal-01.json`
- Independent verification performed by me (foreman), not just Codex's
  self-report:
  - `python3 -m json.tool` on the JSON file: parses cleanly.
  - All six required JSON sections (`observedFailure`, `upstreamRelation`,
    `options`, `recommendation`, `openQuestions`, `notAuthorised`) present
    and non-empty (confirmed programmatically).
  - Extracted all 20 unique `{path, sha256}` evidence pairs cited in the
    JSON and recomputed every one with Python's `hashlib.sha256` against
    the named path (10 in-worktree paths, 8 paths in the still-registered
    `sp03-expression-k-macro05` git worktree, 1 in the pinned
    `midnightntwrk/k-rust` clone, 1 additional design-review doc). Result:
    **0 mismatches, 0 read errors** — all 20 verified OK.
  - `git status --porcelain --untracked-files=all` and `git diff --stat`
    confirm no tracked file was modified anywhere in the repository; the
    only new paths are the two proposal files (plus the pre-existing
    untracked `FOREMAN_REPORT.*` templates and the adapter's own
    `--output-last-message` side file `.foreman-last.txt`, neither of which
    is proposal content).
  - Confirmed no K binary (`kompile`, `krun`, `kore-expand-macros`, `kast`,
    `kore-print`, `interpreter`) and no invocation of
    `.moriarty-dev/k-macro05-trace106-diagnostic.py` appears anywhere in the
    Codex transcript; Codex's own commands were limited to `sha256sum`,
    `cat`, `mkdir -p` (for the new proposal directory), and file writes
    under `deliverables/language-to-ledger-2026-09-12/k-proposal/`.

## Evidence contract (git)

- head_before: 056e662254c38936758af9c6f72c019705a1ff09
- head_after: 056e662254c38936758af9c6f72c019705a1ff09
- status_digest_before: ca6f679ef50a955b50ee8ae851416319147513fbc34c9ac59b74bc77219513e5
- status_digest_after: 011b5f0229883b0999c6c03b983c528a3bb8253e1af7f1583cee3db33dcd3c48
- unauthorized_git_activity: false (HEAD unchanged; no commits, no branch/ref writes)

## Open questions

Carried into the proposal's own `openQuestions` section (owner/reviewer must
settle before any retry):

1. Do parse04's kore-expand-macros crash and macro05's interpreter crash
   share a root cause, or are they independent defects triggered by the same
   oversized/deeply nested input?
2. Is CLM-0524's stage-level corroboration sufficient to treat parse04 as an
   already-documented upstream exclusion class, or do reviewers require a
   separate upstream `runtimeverification/k` bug report?
3. Who authorizes the already-specified, not-yet-executed smallest-
   discriminator diagnostic, and under which obligation — running it is
   itself a bounded retry variant of the frozen case?
4. Can Option A be implemented and correspondence-tested entirely in
   transport/codec/native traversal without modifying
   `formal/k/expression-v1.k`, or does task 4.2 also constrain which K
   semantics files may change?
5. Should the pinned K release (`v7.1.337` / commit
   `4a46d1231473b599c699160132fd6e76a5c46406`) be reconsidered against newer
   upstream releases independent of any Moriarty-side fix?
6. What exact content/acceptance threshold must the fresh Astra-medium and
   Grok-high audits meet before root admission can even be requested?

This report itself does not resolve any of the above; it only confirms the
proposal document raises them for the owner.
