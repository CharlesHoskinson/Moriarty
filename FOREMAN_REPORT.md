# S3 report: surface reference and reusable patterns

## Files
- Created `site/src/tutorial/LanguagePatterns.tsx` (exports `LanguagePatterns`; section ids `syntax`, `patterns`).
- Wrote this report and `FOREMAN_REPORT.json`. Nothing committed.

## Sources used
Product contract and AGENTS.md read; `site/VOICE.md`; `packages/moriarty-beta/GETTING-STARTED.md`, `SIGNED-INTENT.md`, `examples/` (`local/repay/repayment.mori`, `signed-intent/transfer-schnorr-raw/program.mori`, `lending.mori`, `bridge.mori`, `amm.mori`); `wiki-llm/beta-language-2026-09-30/{TUTORIAL-SITE-PLAN,CONVERGENCE,FULL-LANGUAGE-HORIZON}.md`.
Beta blocks are `?raw` imports of those files, so they cannot drift. Horizon blocks are short verbatim excerpts, labelled non-executable. Links are absolute GitHub `blob/main` URLs; no `../../` links.
Status labels (LocalS0/SpecifiedOnly/Open/PreparedUnqualified) accompany every block. No CLI commands are displayed.

## Verification
- Performed: reading and manual comparison of claims against the docs above.
- Not performed: tsc/vite build, npm ci, browser tests, any CLI run (no node_modules present; installs/builds prohibited).
- Unchecked risks: the five `?raw` imports reach outside `site/` (`../../../packages/...`); Vite `server.fs.allow` may need to permit the repo root in dev. Build should be fine. Excerpts `TRANSFER_CORE` contains an elision comment and is labelled non-runnable. `overpay-scenario.json` in the test-shape block is illustrative, stated as such. GitHub links assume the files exist on `main`.
- Other modules and deployment were not tested.

## Remaining root actions
Run `npm ci && npm run verify`; confirm `?raw` paths resolve and CSS for `data-status` / `data-kind` attributes (optional); confirm the `guide-*` classes style the `figure.guide-code`/`span.guide-status` elements as intended.

## Provider identity
Requested: Sonnet 5.5 developer. The session reports model claude-sonnet-5-5; no separate returned-identity receipt or effort setting was available to me.

## Polish continuation
- `D2-integrated-source-review.md` was not present in this worktree or the main checkout (bounded search), so the polish follows the task text only.
- Code blocks now carry one of three scopes: Complete beta file (imported bytes rendered untrimmed), Beta excerpt (non-runnable), Proposed horizon syntax (not accepted by beta parser).
- SpecifiedOnly is used only for the checked beta files lending/bridge/amm. Horizon Duty, holes and recovery are Open and proposed. The grammar skeleton and the Invoice excerpt carry no LocalS0 badge and are labelled non-runnable.
- "Kernel evidence" is now "Candidate acceptance evidence", separated from optional federation.
- Each block has one toolbar: `guide-code-toolbar`, `guide-copy` button, and a `role="status"` `guide-copy-status` that reports success or an actual failure when the clipboard is unavailable. The root's real toolbar class names were not visible to me, so root should align these three class names or tell me the correct ones.
- Not performed: tsc/build/browser test (no node_modules, installs prohibited). No commands run besides reads and a text edit.
