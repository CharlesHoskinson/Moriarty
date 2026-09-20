# Content correction report

Date: 2026-09-19. Checkout: `/home/charl/Moriarty-pages-20260919` (branch `docs/two-reference-pages-20260919`). Scope: the bounded content corrections requested by `CONTENT-FINANCE.md`, `CONTENT-LANGUAGE.md` and `CONTENT-SECURITY.md`, plus the two optional clarifications and the result links. No commit, push or delegation. Reducer logic, fixture amounts, event order, mounting and error boundary are unchanged. The original review reports were not modified.

## Corrections applied

| Finding | Files | Change |
|---|---|---|
| F1 custody label | `site/src/kernel/KernelExplorer.tsx`, `site/kernel.html` (six-quantities bullet, shared-path timeout row, still-unknown branch row and article), `site/src/data/kernel-scenarios.mjs` (timeout and unknown-observation messages only) | Row relabeled `Last confirmed escrow balance`; `data-account-row="custody"` id kept. New hint under the account table states the row is the last confirmed accounted balance, a reservation is not custody, and while an attempt is in flight the current location of the covered amount is unknown. Static rows and article say the same. No fixture number changed; the 5.5/0.5/5.5/10 transcript is intact. |
| F2 discharge condition | `site/src/data/kernel-scenarios.mjs` duty fixture | `Authenticated confirmed receipt of every required fill, with each fill accepted against all signed predicates`. |
| CL-01 / CS-01 threshold | `site/src/kernel/evidence.ts`, `site/kernel.html` static row | Claim now says a signing threshold is not a threshold of honest signers: with t required shares and at most f corrupt, a valid signature needs at least t − f honest shares under the protocol assumptions, not t. Assumption reworded to the corruption bound. Retained: verifier checks the group key; no individual approval recorded. Static and enhanced text aligned. |
| CS-02 shared operators | `site/kernel.html` evidence lede | Common binding is necessary, not sufficient; checks must be enforced at the relevant acceptance point; shared operators are correlated dependencies; distinct proof, threshold and hardware assumptions still need separate analysis. "count once" removed. Inspector fine print unchanged and now consistent with the lede. |
| CS-03 privacy (optional) | `site/src/kernel/evidence.ts`, `site/kernel.html` static row | ZK claim qualified to the bound public statement; added assumption that zero knowledge is separate from soundness, hides the witness only relative to the public statement and intended disclosure, and does not hide metadata or authorized/public outputs; not an end-to-end confidentiality or witness-handoff guarantee. |
| CL-02 history (optional) | `site/kernel.html` continuation disclosure | One sentence: each stage and its predecessor fan-in remain bounded; a growing finite history can continue through authenticated stages within the signed episode and lifetime budgets; not a promise of unbounded execution or storage. |
| Result links (optional) | `site/kernel.html` status list | Direct links to the K comparison, Preview loan and Preview swap `RESULT.md` records beside the two current-result bullets. |

## Test changes

- `site/src/data/kernel-scenarios.test.mjs`: two `assert.match` lines on the delivery duty discharge text inside the existing consent test.
- `site/test/kernel.spec.py`: content checks for the custody row label, the in-flight custody note, the discharge text, the t − f threshold wording in inspector and static table, the evidence lede (no "count once", names correlated dependencies and "necessary, not sufficient"), and the ZK metadata qualification.

## Actual results

- `npm --prefix site run verify` (run once after edits): typecheck passed, 51/51 unit tests passed, build succeeded.
- `npm --prefix site run test:kernel` against the fresh build served on port 8891: 197/197 passed, no console errors or failed requests reported. The first invocation failed only because no server was running; it was then run with the server up. One further invocation was used to count passes; that is one repeat beyond the requested single run.
- Product text was grepped for model or vendor names in the kernel page and its modules; none present.

## Not done

- No other browser suites were run. No candidate manifest was regenerated; changed bytes need fresh independent audits per the develop skill.
- Guarded `status` reports the SP01.6 implementation action blocked on stale binding inputs and missing accounting; that does not affect these content edits.
