# Independent GPT-6 final code audit — candidate 02

Verdict: **PASS for the bounded R1–R5 behavioral repair scope.** No remaining material behavioral defect was found in that scope. This is not approval of the final article's content, accessibility acceptance, deployment, or a later changed candidate.

Audit date: 2026-09-20 UTC. Reviewer: independently delegated GPT-6 auditor, task `kernel_code_final`; GPT-6 Astra review routing requested by the parent. Implementation author: Claude Fable 5.1, as recorded in the author report. No candidate files were edited.

## Exact scope

Reviewed the immutable files under `frozen-02/` against the approved `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md`, the original `GPT6-AUDIT-01.md`, and `FABLE-REPAIR-REPORT.md`. Source inspection focused on the complete scenario reducer and its helpers, declarations, the explorer's event/control/rendering path, and the React mount/error-boundary implementation; inspected the relevant repair regression tests. This is a code repair audit, not a fresh full content audit of all 17 files. Other reviewers separately own content.

All 17 frozen file SHA-256 digests matched `candidate-02.json`. Manifest SHA-256: `e81ecc02859e59794e0a34bc0e8a45bf5d2bf4fce7937a4e3a075c515b352954`. Important reviewed sources:

- Model: `f4fafb4de4d327167720605e8811bab5848d57e89e2ef38d665cb5fe402da7a8`
- Explorer: `8c91f4d6c99fc6b963f7f284e22f5ba7f79b1b2127e79ed9cb3405cc8b9238c6`
- Mount/error boundary: `3283b83e810b917913b382f202a19e34f28936275987b54b08b8612525b6ff55`

Applied required Moriarty development skill/startup and refreshed guarded status. No pending transactions were reported. The unrelated financial campaign remains blocked; this review dispatched no campaign or chain operation.

## Findings and independent reproduction

| Original finding | Disposition | Evidence |
|---|---|---|
| R1: authenticated success forgotten when acceptance predicate missing | Closed | Independent Node sequences used stale, missing, wrong-issuer and unsupported evidence, plus withheld provider signature and recipient acceptance. Each success now records account 1100 gross / 100 fees / 0 reserved / 0 custody / 2000 receipt, consumes `attempt-2`, keeps `fillsDone=1`, and retains both duties in `observed`. Subsequent failure, unknown and repeated success are refused without changing account, duties or consumed identifiers. Restoring the predicate and accepting advances to completion without moving money; a second acceptance is refused. |
| R2: consumed attempt identifier reused | Closed | Reducer rejects first-fill reservation and second local finalization; UI availability follows that ordering. A failed second attempt cannot be retried. Independently used a policy with gross cap 2000 and fee cap 200 to remove the normal capacity refusal and confirmed the distinct `attempt-2 is already consumed` guard. Failure followed by success, unknown or repeat failure is refused. |
| R3: asynchronous render failure hides fallback | Closed | Source now marks enhancement after commit and restores fallback from the boundary/root error path. On the production page, independently injected `createElement('fieldset')` failure before initial rendering and `createElement('code')` failure after initial mount, triggered by first-fill finalization. Both returned visible static transcripts, removed enhancement, and emptied the failed root. The separately rooted inspector remained interactive in both cases. |
| R4: remedy duty remains open after success | Closed | Immediate and delayed successful acceptance discharge both delivery and the untriggered conditional remedy. Failure leaves the remedy pending. The helper only discharges remedy status `accepted`, preserving a triggered pending duty. |
| R5: false arithmetic in reachable narratives | Closed | The previously problematic first-external-attempt path is refused at the reducer boundary. Canonical failed-second-attempt prose derives 5 A ordinary and no fee capacity from actual numbers. Competing reservation reports both exclusive reservation authority and the real 16.5 A prospective exposure against 11 A. |

Additionally performed an independent semantic-state traversal from the default initial state across all declared event types and all enumerated candidate, evidence, condition, solver and observation values. The traversal deduplicated states by semantic fields, excluding event history, last note and last rejection. It reached **1,040 states and checked 30,160 transitions**. Assertions passed for:

- gross debit plus reservation within 1100 and fees within 100;
- custody plus cumulative gross debit equals opening 1100;
- nonnegative reservations and unique consumed identifiers;
- monotone gross debit, fees and confirmed receipt except explicit reset;
- rejected events leaving the account unchanged;
- complete states meeting the receipt goal and having no open duties.

This bounded traversal does not establish a formal proof, arbitrary-input safety, arbitrary policy correctness, or correctness of every narrative string. The deliberately expanded-cap retry probe is separate from the default-fixture traversal.

## Runtime scope and limitations

Runtime probes used `http://127.0.0.1:8897/kernel.html` with headless Chromium, selecting the installed `chromium-1234` executable. The first launch using Playwright's default executable failed because the expected browser revision was absent; selecting the already installed browser resolved this environment issue. Both actual fault probes then passed.

The production runtime was supplied by the parent. Immutable-source identity is independently verified above; a separately reconstructed build or source-map attestation of that served bundle was not performed. Later content-only edits to the live worktree are outside this frozen-source verdict and must retain their own content reviews. Any subsequent behavioral change to the reducer, explorer event wiring, or mount boundary needs a current code check.

Did not repeat the parent-reported full typecheck/build/site/browser suites. Actual screen-reader use and actual browser 200% zoom remain unperformed in this audit; the original acceptance gaps must not be silently closed. No live deployment or native proof/ledger behavior was tested. R6/R7 and article-wide financial/security wording belong to the separate current content review.
