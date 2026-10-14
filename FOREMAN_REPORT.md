# FOREMAN_REPORT

- run_id: moriarty-tutorial-20261001
- role: implement (polish continuation)
- slug: S2
- branch: foreman/moriarty-tutorial-20261001/implement/S2
- base_sha: df8b155143e90450168af04fbfd20f7620043620
- status: complete

## Summary
Polish pass: checkout-first install, captured-vs-observation provenance pinned to df8b1551, one announced Copy per artifact, concrete negative-case file locations, narrowed rejection wording. Lesson files unchanged.

## Changes
- Install: first block is a pasteable checkout route (git clone https://github.com/CharlesHoskinson/Moriarty.git, cd Moriarty/packages/moriarty-beta, npm ci, npm run build, MORI_PKG=$PWD, mori() function). All lesson commands now use `mori`. Archive route is separate: MORI_TGZ="$PWD/$(npm pack --silent)" captured before cd, then npm install "$MORI_TGZ" and mori() { npx mori; }. Not run by me (no install/pack); stated on the page. No /absolute/path placeholder remains.
- Copy: one toolbar per artifact (figcaption.guide-copybar with button.guide-copy, span[role=status].guide-copy-state with guide-copy-ok/fail, plus .guide-code-kind label and download link). Figures carry data-copy-ready="true" so the GuideApp injector skips them. Missing navigator.clipboard is handled as a failure message. GuideApp/CSS untouched; .guide-code-kind is a new class S1 may style.
- Subsection IDs set in source: install, install-archive, commands, units, open-premises, transfer-derive, repayment-derive, wrong-expectation, core-rejection, what-a-pass-means.
- Negative cases: prose and commands name ~/Downloads via a DL variable with explicit replace instruction, browser-suffix note, and the /tmp/invoice prerequisite; each download is copied into a new dir under the filename `mori test` reads.
- Provenance: every output panel is labelled Captured output with exact command, exit code, commit df8b1551 link and date 2026-10-01, run against the shown file bytes; the old 'Illustrative' wording and the compacted failing JSON were replaced by exact stdout. The repayment 1010.00/1010.01 simulate results are described as observation, not displayed output. Reference links pinned to full SHA df8b155143e90450168af04fbfd20f7620043620. No receipt exists; the page says so.
- Rejection wording narrowed: the case asserts status and code only, compares no effects or post state; the pass paragraph distinguishes full effects/post expectations from status/code-only cases.
- Four premises and four bindings paragraph and all hand arithmetic unchanged.
- Not applied: D1 items 1-3 (except h3 IDs), 4-8 and D2 shell/LanguagePatterns/CrossChain/GuideApp findings, which belong to other owners.

## Evidence
- Primary dist cli.js existed; used only in mktemp scratch dirs; no build, install, pack, native crypto.
- mori init output (transfer and repay) diffed identical to the six lesson files.
- mori test: transfer pass exit 0; wrong-expectation TestsFailed exit 1 mismatch /post/balances/0/amount 8991 vs 8990; underfunded rejection TestsPassed exit 0 status CoreRejected code S0_EFFECT_RANGE; repay TestsPassed exit 0. Stdout is the displayed text exactly.
- check on underfunded scenario: AuthoringChecked Invoice, pay LocalS0. Repay amount 1010.00 USD simulate: PreparedUnqualified, status Settled, zeros; 1010.01 USD: CoreRejected S0_EFFECT_RANGE.
- tsc strict + noUnusedLocals on GettingStarted.tsx in a temp dir with a ?raw shim against primary site node_modules: pass. No browser, vitest, or site build run.

## Remaining root actions
- Style .guide-code-kind and figcaption.guide-copybar; confirm no duplicate Copy in a browser; confirm clone URL and that commit df8b1551 is public; run npm run verify and downloads/clipboard browser tests; npm pack route is unexecuted.

## Limits
- Requested/returned provider identity: Sonnet 5.5 (claude-sonnet-5-5) is what ran; no independent confirmation. Other modules and deployment were not tested.

## Second bounded polish (D1 item 5, D2 P1 derivation order)
- Changed only site/src/tutorial/GettingStarted.tsx. All first-polish changes remain.
- Transfer: the "Derive the expected amounts first" list (1000+10=1010; Owner 8990, Recipient 1000, Fee 10; allowance, work, replay key, head) and the nonce/round note now follow the lesson intro and precede every file and command. New h3 `transfer-files` ("Files and commands") holds invoice.mori inline, then the commands.
- Repayment: obligation sentence, the three-case derivation table and the 30.00 USD arithmetic (1000 interest + 2000 principal = 3000) now precede the files. New h3 `repayment-files`. repayment.mori stays inline; the "change the amount constant" sentence moved with the source note.
- New FileDetails helper: a labelled `<details className="guide-details">` wrapping the existing Code figure, so its Copy and Download controls appear once opened. Collapsed: transfer scenario.json and mori.tests.json, repayment scenario.json and mori.tests.json, and the three negative-case files (wrong expectation, underfunded scenario, core rejection), with "Negative case:" summaries. Texts, filenames and raw bytes unchanged (same ?raw imports and derived functions).
- Prose adjusted to say the JSON is collapsed and to open the files to download them. No financial defaults hidden; the terms are in the visible derivation and the complete program. Captured-output panels, Capture provenance and command labels untouched.
- Not done: no tests, build, native, network, git or cleanup run; no CSS, source or grammar change (guide-details is unstyled, using browser details defaults). Root browser checks needed: details open/close, Copy/Download inside opened details, no duplicate shell Copy, anchors and active lesson for the new h3 ids.
