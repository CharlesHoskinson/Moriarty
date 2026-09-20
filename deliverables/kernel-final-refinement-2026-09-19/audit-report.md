# Federated DeFi Kernel article: final audit and refinement pass

Date: 2026-09-19 (workspace time). Worktree: `/home/charl/Moriarty-pages-20260919`, branch `docs/two-reference-pages-20260919`, HEAD `1816e123` (unchanged; nothing committed or pushed).

Scope followed the dispatch: `site/kernel.html`, explanatory copy in `site/src/kernel/KernelExplorer.tsx` and `site/src/kernel/EvidenceInspector.tsx`. The financial reducer, evidence model, generated reference pages, tests, `VOICE.md` and `styles.css` are untouched. The guarded CLI status was read at startup (capability SP01.6, no pending transactions); its unrelated blocked campaign action does not affect an authorized prose edit.

## Files changed

| File | Baseline SHA-256 | Final SHA-256 |
|---|---|---|
| `site/kernel.html` | `d09c7a72…ba404` | `516d780ca17d31719ca39516ac3f81b965bd7df0c03764d05b68af409338fb67` |
| `site/src/kernel/EvidenceInspector.tsx` | `6c1de425…0fc77` | `8db4d393470af9cf6616f54fb6fb89dbcc48bf047f0a2d3bb5356eaa8bdf0552` |

`KernelExplorer.tsx`, `evidence.ts`, `kernel-scenarios.mjs`, `styles.css`, `VOICE.md` and `kernel.spec.py` hash identically to `baseline.json` (see `final-sha256.txt`). The full diff is in `final-pass.diff`.

## Findings fixed in `site/kernel.html`

1. **Rendering defect.** The status lede broke `source-to-ledger` across a source line, so the page rendered "source-to- ledger". The word is now on one line; the built HTML was checked.
2. **Prose contradicted the browser model.** The late-success paragraph stated that "the release conditions held when the attempt was submitted". The explorer lets an attempt be reserved and submitted while the document predicate is unmet, and the audited acceptance-withheld scenario depends on that. The paragraph now says that the predicates releasing a fill can change between submission and acceptance, which is true of the design and of the model. No reducer change.
3. **Confusing term.** "Brings spending plus exposure to the 11 A cap" used exposure to mean reserved, while the account row defines exposure as spent plus reserved. Now "brings spent plus reserved to the 11 A cap".
4. **Reservation versus custody.** One sentence said the custody record "establishes where the assets are held", in tension with the next paragraph's point that location is unknown while an attempt is in flight. The reservation paragraph now says a reservation says nothing about location and that the question is answered by the custody record and authenticated observations.
5. **Orphan paragraph.** The adapter caveat ("not every adapter can supply a failure policy this clean") stood alone after the failure branch. It is merged into that argument, with its consequence stated: where an adapter cannot establish nonexecution, the reservation would have to stay.
6. **Remedy discharge rule.** The failure branch now says only an authenticated remedy transition discharges a pending remedy, and that failure-free completion discharges the same conditional duty on the successful branch because no remedy became due. No payout amount or refund is invented.
7. **Threshold paragraph.** A circular clause (whether honest participants checked the policy "depends on honest participants following the specified protocol") is replaced by the plain assumption: that honest participants checked the policy before signing is a further assumption about their behavior, because the signature records no such check. The 3-of-5 example is introduced as the inspector's illustrative configuration instead of appearing unannounced. The t − f statement is unchanged.
8. **Evidence combination paragraph.** The sentence that joined two ideas (enforcement at the acceptance point; correlated operator dependencies) is split; "talking about the same thing" becomes "speak to the same statement". The phrases the browser suite asserts ("common statement is not enough", "correlated dependencies") are preserved.
9. **Redundancy.** The witness-handoff point was made three times; it is now made in the zero-knowledge paragraph and once in the continuation disclosure. The trace disclosure's closing paragraph repeated the paragraph above it and is removed. The page lede repeated paragraph four's verb list and now summarizes instead.
10. **Weak openers and announcements.** "These responsibilities cross organizational boundaries" is replaced by the CAKE mapping stated directly. "Whose assumptions and enforcement points are explained below" is cut. The federation paragraph now ties ZK, MPC and TEE to the federation's own work instead of listing them.
11. **Terms and headings.** "The owner's signed intention" now introduces "intention" before it is used. "Open Wallet Standard (OWS)" is expanded on first use so the status list's "OWS" reads. "MPLRs" in the sources list is expanded. The kernel-eligibility sentence now reads "a federation's willingness to serve an agreement is never a condition of using Moriarty". The aside heading is "What the buyer fixes and what the solver chooses". Section 4's heading no longer duplicates its eyebrow ("Waiting on an external result"). The meta description no longer says a language "needs an optional" coordinator.
12. **Continuation disclosure.** "Signed episode and lifetime budgets" is rewritten for a reader while preserving bounded stages and fan-in, a growing finite history, no reset of a signed lifetime budget, shared ancestry without double consumption, and duties and consumed identifiers travelling with the history.
13. **Funding aside.** "Matters for the same reason" had no referent; the paragraph now says why the funding line exists (the accounting has to say what counts) and keeps the explicit exclusion from the gross cap.
14. **Duties paragraph.** Split from the custody paragraph; the "Finally," enumeration and the fragment "Duties survive partial progress." are folded into one connected sentence.
15. **No-script parity.** The static evidence block now carries the same "needs scripts" note as the static explorer block.
16. **Source indentation** normalized for ledes and one disclosure paragraph; no rendered change.

## `site/src/kernel/EvidenceInspector.tsx`

- Panel label "Under these named assumptions" is now "Under these assumptions", matching the static table header; "named" carried nothing.

## Reviewed and left unchanged

- `KernelExplorer.tsx` explanatory strings are consistent with the revised prose; no change was needed.
- `VOICE.md` already states the required editorial rules; no change was necessary.
- Tests: every asserted phrase survives, so `kernel.spec.py` needed no edit.

## Observations that are not page defects

- The reducer allows `reserve-fill` while the document predicate is unmet, which the audited acceptance-withheld scenario relies on. The prose is now aligned with that behavior rather than contradicting it. Whether a production kernel should submit an external attempt while a release predicate is unmet is a design question outside this pass; it is recorded here, not changed.
- The explorer's candidate-check hint and the "How these map onto the fixture" disclosure make the same mapping in two places, one beside the control and one in the explanation. Kept.

## Claim inventory check

Preserved as stated in `deliverables/kernel-prose-revision-2026-09-19/claim-inventory.md`: permissionless language, ZKIRv3 on Midnight, optional kernel; signed authority, complete effects, residual duties; the four distinct acceptance claims and source/Core/native correspondence; general failed phases retaining specified effects and duties; the 11 A gross cap inclusive of 1 A fees for 20 B with pre-funding excluded; fills of 5.5/0.5/10; reservation 5.5; failure 6/1/10; no invented refund, rollback or budget reset; provider consent before duty; late external result accounted even when acceptance predicates are unavailable, restored acceptance exactly once; remedy duty with discharge rule and no payout amount; t − f honest participation under protocol and corruption assumptions plus the honest policy-checking assumption; TEE input/output/report binding, freshness and rollback; common binding insufficient without enforcement at the destination; correlated operator dependencies; foreign signing compromise not reversible by local refusal; OWS scoped delegation and x402 payment/result/delivery separation; bounded stages, growing finite history, private handoff. Implementation status remains confined to the final section.

## Verification

| Step | Command | Result | Log |
|---|---|---|---|
| Type check, unit tests, build | `npm --prefix site run verify` | exit 0; tsc clean; 51 tests pass, 0 fail; Vite build succeeded | `logs/verify.log` (earlier run on the same edits minus the second-read fixes: `logs/verify-prelim.log`, exit 0) |
| Serve under project prefix | `python3 -m http.server 8899 --bind 127.0.0.1 --directory /tmp/pages-8899` with `/tmp/pages-8899/Moriarty -> site/dist` | `GET /Moriarty/kernel.html` 200, `GET /Moriarty/docs/requirements.html` 200 | `logs/http-server-8899.log` |
| Browser suite | `URL=http://127.0.0.1:8899/Moriarty/ CHROME=/home/charl/.cache/ms-playwright/chromium_headless_shell-1234/chrome-headless-shell-linux64/chrome-headless-shell python3 test/kernel.spec.py` | 197/197 passed, exit 0 | `logs/kernel-spec.log`, screenshots in `shots/` |
| Visual check | full-page and element captures (`scripts/kernel-fullpage.py`, `scripts/kernel-elements.py`) | edited passages render as intended; status lede shows `source-to-ledger` intact | `shots/fullpage-*.png`, `shots/el-*.png` |
| Built HTML text checks | grep of `site/dist/kernel.html` and sources | `source-to-ledger` present twice, no broken hyphen; no em or en dashes in body prose; none of the VOICE.md banned words | in this report |

The suite exercises the published behavior only; it establishes nothing about a native proof, a ledger settlement or a deployed kernel, exactly as the status section says.

## Remaining defects

None known within the scope of this pass. The server on port 8899 was stopped after the checks.
