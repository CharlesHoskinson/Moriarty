# Independent website publication review

Date: 2026-10-01. Requested reviewer seat: GPT-6 Astra, medium effort. This fresh delegated review has no independently returned provider/model attestation available inside its context. The requested identity and effort are not a model attestation; retain host metadata separately. No counterpart publication audit was read.

## Candidate and votes

Candidate: `/home/charl/research/moriarty-website-2026-10-01/PUBLICATION-SOURCE-FREEZE.json`, SHA-256 `9eeea5e387e3db8ede006231a05892323b445412952622eb12109b1b40cbfbf1`. I recomputed the manifest digest and all 46 file digests: no mismatch. Recorded base is `cac114a44825f62fe36c1389ac64585d95a96e2e`; this review did not use Git to independently inspect that ancestry.

- Full current website source vote: **REQUEST CHANGES**, for B1 below.
- Actual-result faithfulness vote: **ACCEPT the narrow recorded passing predicates; REQUEST CHANGES to final navigation/pipeline result coverage**. The 18 interaction results and nine smoke results do not prove every fresh lesson link aligns or that the pipeline screenshot shows its subject.
- Scoped merge/Pages publication vote: **HOLD pending B1 repair and fresh source/result review of the changed candidate**. User publication authorization is understood; no additional user permission is required by this review. There is no public-live approval or financial/native acceptance vote.

## Blocking finding

### B1 — Fresh architecture URL settles at the wrong reading position and current lesson

Affected source: `site/src/tutorial/GuideApp.tsx`, mount `useLayoutEffect` initial `target.scrollIntoView()` and subsequent measured layout/current-place effects (approximately lines 640–777).

Read-only reproduction against the current local production build at `http://127.0.0.1:8896/Moriarty/tutorial.html#architecture`, Chromium headless shell 1234, fresh browser context, viewport 1440×1000. All browser requests were restricted to the existing local server. No build, source edit, CLI example, native operation or external network request was run.

1. Navigate directly to the architecture URL and wait for networkidle, then wait 1000 ms and another 3000 ms.
2. At both observations: scrollY=60644; architecture section top=747.1875; maximum scroll=62018; current lesson=`Support and trust`. A separate fresh run after 2500 ms measured architecture h2 top=796.1875, image top=956.828125, image complete=true, masthead bottom=63.75.
3. Click the sidebar Architecture link and wait 2000 ms: scrollY=61315, architecture section top=76.1875, current lesson=`Architecture`.

This is a persistent fresh-load defect, not merely a screenshot taken during an animation. The destination is reachable via a subsequent click, and the heading is near the bottom of the initial viewport, but direct navigation leaves the wrong lesson current and does not land the architecture content under the measured sticky boundary. An initial scroll occurs before the later layout has settled; the exact intervening layout cause needs implementation diagnosis.

The supplied `final-pipeline.png` independently exposes insufficient capture settling: it has a large blank upper region, the masthead halfway down, bridge/staking content and an architecture contents rail, with no pipeline in view. Keep this original evidence intact. The check script captures it immediately after `load(#architecture)`, whose settle wait is two frames plus 150 ms, without asserting that the pipeline is in view. The fresh-link test covers install and transfer only; architecture in the viewport loop is checked for containment only.

Repair the fresh hash destination after the relevant layout has stabilized, while preserving manual scrolling and reduced-motion behavior. Recheck a fresh and reloaded architecture URL, correct lesson/contents state, sticky clearance and an actually visible, settled pipeline. Obtain a replacement supplemental capture; do not relabel the old screenshot as a successful pipeline render. Since source changes are needed, freeze and audit the resulting candidate again.

## Source assessment

Read all six tutorial TSX modules, main entry, tutorial CSS, six lesson files, modified home App, Vite configuration, tutorial HTML, Pages workflow and tutorial smoke test/README. Read website plan, design integration and polish decision, image metadata/prompts, all six assembled-baseline design reports, and author model receipts. The historical design reviews remain baseline findings, not approval of the present result.

All eight families are discoverable under AMM, Lending, Stablecoins, Options/derivatives, Oracles, Governance, Bridges and Staking. Their imported package examples were inspected. Structural authoring is consistently separated from unsupported execution and open financial relations; repayment is separately LocalS0. Family prose distinguishes stored strings from enforced timelocks, prices, duty preservation, finality and share arithmetic. Worked atom amounts agree with the example declarations. Transfer 1000+10=1010 and repayment 3000=1000 accrued+2000 principal agree with complete lesson expectations.

Alias, economic identity, symbol, domain, scale and representation are distinguished. Signer/key references are not authenticated account ownership. The unsigned starter, separate signature verification, unverified key authority, local stipulated state, mandatory proof and ledger acceptance are not conflated. The optional federation is not a public compiler prerequisite or a substitute for ledger acceptance. Horizon syntax is visibly labeled proposed and unsupported by the beta parser. I found no financial/native/Preview acceptance claim introduced by these pages.

The checkout installation function preserves an absolute CLI path. Archive installation is correctly labeled a local build and unperformed for the page. Negative-test downloads have concrete filenames and explain the Downloads directory assumption. Captured outputs and derived illustrative formatter output are distinguished. Full local files retain exact text, including terminal-newline distinctions; signed examples are imported without rewriting.

The shell now has one h1, named lesson groups, local heading search without hidden sections, measured sticky offsets, onscreen mobile navigation, destination focus, Escape behavior, copy error status, bounded code/table overflow, and theme support. Source and actual observations support the major repairs to the six baseline reviews. B1 remains a direct-navigation defect despite those improvements.

The Vite third entry and relative base support project-subpath publication. The Pages workflow runs the tutorial smoke alongside existing gates before upload/deploy. This is source inspection of the workflow, not evidence of a completed remote job.

## Actual-result audit

I read `FINAL-INTERACTION-RESULTS.json`, `DEPLOYMENT-SMOKE-RESULTS.json`, `ROOT-CHECKS.json`, and the external `check-tutorial.py` implementation. The archived final interaction JSON is byte-identical to the external browser-run `results.json`. I independently compared all six retained downloaded source/scenario/test files against versioned lessons: all equal. The stored check/test stdout reports both local examples passing with `PreparedUnqualified` and `local-stipulation-only`; these support local evaluation only.

I directly viewed all seven final PNGs: desktop, mobile, tablet, dark, code, open drawer and pipeline. The first six show the new hierarchy, coherent themes, compact mobile header, accessible lesson route and one source toolbar. The pipeline image has the defect described above. Screenshots cannot establish clipboard, keyboard or search execution; those are supported by the inspected harness and its records, not inferred from the pictures.

The 18 recorded checks cover section/navigation structure, desktop containment, duplicate copy detection, install/transfer fresh links, search, actual native clipboard bytes and downloads, actual downloaded beta check/test, changing disclosure height, multiple widths/themes/motion, mobile focus/Escape/skip links, homepage tutorial entry, blocked clipboard failure and no page errors. The nine smoke checks cover project-subpath load, six download byte comparisons, mobile entry/navigation/direct transfer and containment/errors. Their scope matches the test code, except the natural-language name “one copy control per code block” is broader than its assertion: it rejects duplicates and does not require a control on every block. Intentionally uncopyable derived output makes that distinction reasonable.

ROOT-CHECKS records zero exits for strict TypeScript/build, 51 data tests, 49 home checks, static reference checks and 197 kernel checks. I did not rerun those suites. These records do not establish public deployment, exhaustive browser/accessibility coverage, a screen-reader pass or any financial acceptance. The source and records correctly leave public deployment unverified.

## Nonblocking observations

- `IMAGE-ASSETS.json` still describes the hero as having a closed gate. The current public alt/caption and the baseline image reviews correctly describe an open gate and explicitly deny acceptance meaning. This is stale internal image-inspection prose, not a public financial claim. Preserve historical provenance and add a correction when updating the notes.
- Several proposed LanguagePatterns blocks retain their own Copy buttons, but prominently say proposed horizon syntax and not accepted by the beta parser. The shell does not inject extra controls into these blocks. This is consistent with the polish decision allowing explicitly scoped proposed concepts; it is not a runnable-beta promise.
- The raw imported package examples are dependencies outside this 46-file website-change manifest. Their reviewed current bytes must remain the same in the publication tree. This audit did not independently use Git to establish the base dependency tree.

## Operational boundary

Loaded the installed Moriarty develop skill and repository AGENTS, and ran guarded `status --json`. No pending transactions. Status retains stale/missing financial campaign evidence and operational-history gaps; this read-only website review neither dispatches nor closes them. Only this external review file was written. All financial/native proof, authenticated-state, recursive-history and public-ledger acceptance obligations retain their existing scope and status.
