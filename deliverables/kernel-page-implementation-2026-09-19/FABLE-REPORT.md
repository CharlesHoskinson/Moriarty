# Implementation report: interactive Federated DeFi Kernel page

Date: 2026-09-19 (session ran into 2026-09-20 UTC). Implementer: Claude Fable 5.1 at medium effort, selected by the user for this page. A separate GPT-6 agent audits. This report records what was changed, what was actually run, and what was not established. Nothing was committed or pushed.

Worktree: `/home/charl/Moriarty-pages-20260919`, branch `docs/two-reference-pages-20260919`, base `0ce871a3`. Working tree is uncommitted.

Startup: read `AGENTS.md`, `plugins/moriarty-dev/skills/develop/SKILL.md`, and ran `cli.py --repo . status --json`. Status reports the unrelated `SP01.6 loan-swap-subset` block with no pending transactions. No campaign was dispatched, no chain, wallet or proving operation was performed.

Requirements source: `docs/superpowers/specs/2026-09-19-federated-kernel-interactive-page-design.md`. The GPT-6 expected-outcomes oracle in this directory (`GPT6-EXPECTED.md`) was read after the first working build and three of its independently derived expectations were adopted because they sharpen the design's own rules: a fee-capacity display that separates incurred from encumbered authority while an attempt is in flight, explicit provider-signature and recipient-acceptance toggles so the conjunction is exercisable rather than assumed, and an announced reset.

## Changed files

New:

- `site/kernel.html` — complete static article: agreement card, five-lane responsibility map with text equivalent and labeled arrows, static shared-path and three-branch transcripts (with `data-account` attributes checked by the Node tests), unknown-outcome comparison, static evidence table and signature-only bypass text, OWS/x402 and continuation disclosures as `<details>`, scoped status with source links, footer. Loads `/src/kernel/main.tsx` as its module entry.
- `site/src/data/kernel-scenarios.mjs` — pure deterministic model. Frozen policy (1100 gross, 100 fees, 2000 B goal, two fills of 500+50 for 1000, opening custody 1100 as a stated funding precondition), four candidate presets, five evidence fixtures, two release-condition toggles, three observation branches, typed event reducer `apply`, `derive`, `availableEvents`, `PRESETS`, `run`, `fmt`. Rejected events return an unchanged account with `lastRejected`. Accepted events append an `events` record carrying responsibility, meaning, delta and post-event account.
- `site/src/data/kernel-scenarios.d.mts` — type declarations for the model.
- `site/src/data/kernel-scenarios.test.mjs` — 32 Node tests (see below).
- `site/src/kernel/main.tsx` — mounts the explorer and inspector into separate roots; marks `data-enhanced` and hides each static block only after its `createRoot(...).render` succeeds; rethrows on failure with the fallback left visible.
- `site/src/kernel/KernelExplorer.tsx` — `useReducer(apply)` explorer: candidate selector, solver label, consent and commit, condition toggles, document-evidence selector, next-event and observation buttons, conflicting-event probes, reset; sticky account table, duties list, event record, `role="status"` announcement.
- `site/src/kernel/EvidenceInspector.tsx` and `site/src/kernel/evidence.ts` — mechanism, common-binding and destination selectors with claim, named assumptions, "does not establish", binding rationale, signature-only 3-of-5 bypass and local limit, hypothetical checking destination flagged as such.
- `site/src/kernel/styles.css` — scoped `k-` layer on the existing tokens; 320px single column, sticky ledger on desktop, dark via existing tokens, reduced motion inherited, print rules that reveal the static transcripts and hide the explorer and masthead.
- `site/test/kernel.spec.py` — 144 browser checks (see below).

Modified:

- `site/vite.config.ts` — two HTML inputs (`index.html`, `kernel.html`); `base: './'` retained.
- `site/src/App.tsx` — homepage jump strip gains a `Kernel` link to `kernel.html`.
- `README.md` — one sentence added to the kernel explanation linking `https://charleshoskinson.github.io/Moriarty/kernel.html` and naming it an educational model, not a third reference. Banner line and the two reference bullets are byte-identical (`docs/assets/moriarty-banner.png` digest unchanged; see protected baseline).
- `site/CONTENT-SPEC.md` — precedence note at the top: the product contract, consolidated design and published requirements control over older Compact/history statements; the kernel page takes its substance from the design spec and its fixture from the model module; sections 1–9 govern the home page only.
- `site/package.json` — `test:kernel` script.
- `.github/workflows/site.yml` — new step serving `dist` under `${RUNNER_TEMP}/pages/Moriarty` on port 8892 and running `test/kernel.spec.py` with `URL=http://localhost:8892/Moriarty/`.
- `site/test/README.md` — documents the two new suites and states that no screen reader was driven.

Not modified: language runtime, wiki, `site/docs/requirements.html`, `site/docs/language.html`, `site/scripts/build-docs.mjs` (the `kernel → requirements.html#architecture` alias is untouched), `site/test/docs.spec.py`, `site/test/site.spec.py`. `protected-baseline.json` digests all match.

Candidate digests (sha256) at the end of this session:

```
342fa39050b9ee9fd08a7eaf6e1a56c2ae81cc6ae28a8dbdb26c5a932e50cda3  site/kernel.html
9c4202c940236273f8462556572797611a3baeb6f71e2c08a77310f122bc3971  site/src/data/kernel-scenarios.mjs
cb98b58f4c582a0be6b87fe52a5b340b09391a197ed64e025451e280158f3409  site/src/kernel/EvidenceInspector.tsx
0f5a995bba0aefe2eebabb93a5db0d7042be190be31cdc966f62987a47a5d6e0  site/src/kernel/KernelExplorer.tsx
cefc8f91d8b64ee990af81ed5dd24d7753f383d884e77bbd50fac3db5a682297  site/src/kernel/main.tsx
950f3342f55a04882a5f30c86bf75b505bd577cbc23afbf917681494b1624ee0  site/src/kernel/evidence.ts
09614061aaa163b0f4e3e8ba7d264b40abd7add7630826f36b204d00659caa4a  site/src/kernel/styles.css
9e514b38d2a3f4c6315b6e0efc68561a10b4af35f8f3723c322d7366d3de3102  site/test/kernel.spec.py
```

## Accounting as implemented

Integer hundredths. Rows match the design table and the GPT-6 oracle table:

| State | Gross | Fees | Reserved | Confirmed B | Custody |
|---|---:|---:|---:|---:|---:|
| Fresh / candidate only / consent / commit / evidence | 0 | 0 | 0 | 0 | 1100 |
| First fill finalized | 550 | 50 | 0 | 1000 | 550 |
| Second fill reserved and submitted | 550 | 50 | 550 | 1000 | 550 |
| Timeout / still unknown / refused refund / refused second solver | 550 | 50 | 550 | 1000 | 550 |
| Late authenticated success | 1100 | 100 | 0 | 2000 | 0 |
| Authenticated failure, fee retained | 600 | 100 | 0 | 1000 | 500 |
| Duplicate terminal observation, replay | unchanged | | | | |

After failure: gross remaining 500, fee remaining 0, remedy duty `pending`; both `reserve-fill` and `finalize-fill` are refused with both cap failures named. While in flight the account shows fee capacity as `0 A free · 0.5 A encumbered by the pending attempt`. A reservation never moves custody. Late success requires provider signature, recipient acceptance and a met document predicate together; otherwise the exposure stands and the attempt remains in flight. Candidate rejection reads the policy only. Solver label is presentation only. Policy object is frozen; selecting another candidate after commitment is refused. Reset returns a fresh fixture and announces "New illustration … nothing modeled was rolled back."

## Tests actually run

All commands run from the worktree on 2026-09-20 UTC, against the build produced by `npm --prefix site run build`. Chromium: `/home/charl/.cache/ms-playwright/chromium_headless_shell-1234/chrome-headless-shell-linux64/chrome-headless-shell`.

| Command | Result |
|---|---|
| `npm --prefix site run verify` (typecheck, `node --test src/data/*.test.mjs`, docs build, `tsc -b`, `vite build`) | pass; 43 Node tests pass (11 existing completeness + 32 kernel); build emits `dist/index.html`, `dist/kernel.html`, assets |
| `python3 site/test/kernel.spec.py` with `URL=http://localhost:8892/Moriarty/` (dist symlinked as `/tmp/moriarty-pages/Moriarty`) | 144/144 |
| `python3 site/test/kernel.spec.py` with default `URL=http://localhost:8891/` (dist at origin root) | 144/144 |
| `python3 site/test/site.spec.py` at `http://localhost:8891/` | 49/49 |
| `python3 site/test/docs.spec.py` at `http://localhost:8891/` | PASS at 375 and 1280 for both reference pages |

Kernel browser suite coverage: direct 200 on `kernel.html`; no console errors or failed requests under `/Moriarty/`; both stylesheets resolve and apply; seven sections in order; five lanes; enhancement attributes and static hidden only after mount; rejected candidates (wrong recipient fails only the recipient check, excessive fee fails only the fee check with gross `11 A of 11 A`) leave the account at zero and commit disabled; evidence gate (`unknown`, `unmet`, `unsupported`, `met`) with refused release moving nothing; positive path with the exact rendered account at every step; second-solver refusal; timeout; late success completing to `11 A / 1 A / 0 A / 20 B of 20 B` with delivery duty `discharged`; duplicate terminal observation recorded with no account change; replay refused; failure branch `6 A / 1 A / 0 A / 10 B` with `0 A` fee capacity and remedy `pending`, further fill refused naming the fee cap; still-unknown branch stable, premature refund refused, late result afterwards still resolves; late success with wrong-issuer evidence refused with `every predicate`; withheld provider signature and withheld recipient acceptance each block release by name and release proceeds when restored; AI label changes neither account nor enabled actions; reset announced; inspector mechanism, binding and destination selectors change the shown claim, name `rolled back`, `flattering`, `3-of-5`, `cannot retroactively stop`, the hypothetical flag and `no generic proof-enforcing chain`; two disclosures, service trace with `same R1`; every button has a text label, no positive tabindex, every table has caption or header, status region `role=status aria-live=polite`, no ✓/✔ or native-proof claim, "illustration" present; keyboard: skip link first, Enter/Space operate candidate, consent, commit, evidence and finalize to a `5.5 A` debit, visible focus outline, controls precede account in document order; widths 320/400/768/1024/1280 with no page overflow, no container overflow, masthead fits; controls operate at 320px with no touch target under 24px; doubled root font size at 640px viewport keeps the account visible with no overflow; dark theme repaints; reduced motion sets `scroll-behavior: auto`; print shows static transcripts and hides the masthead; every local link and anchor resolves; `docs/kernel.html` redirect still resolves to `requirements.html#architecture`; homepage `Kernel` link present; JavaScript disabled shows h1, seven shared rows, three branch rows, static evidence table, empty root, styles applied, no overflow at 320px; enhancement bundle aborted leaves the static content visible and root unmarked; bundle replaced with a throwing script leaves both static blocks visible and root unmarked.

Node suite coverage: policy arithmetic; candidate-only proposal changes nothing; wrong recipient and over-fee fail by named check and cannot commit; both compliant presets pass all five checks; consent required before commit; default preset completes within caps; accepted prefix; reservation as exposure with custody unmoved; timeout unchanged; late success completes; late success without document predicate refused; failure retains fee and releases reservation with 500 gross and 0 fee capacity; no further fill after failure; still unknown stable and premature refund refused; second solver refused; duplicate terminal result no second discharge; replay changes nothing; solver label changes nothing; four evidence outcomes; release withheld for missing/stale/wrong-issuer/unsupported; frozen policy and candidate change refused after commit; reset announced and restores initial state; provider signature and recipient acceptance each required; late success without acceptance leaves exposure; fee encumbrance while in flight; out-of-order and conflicting terminal observations refused, duplicate failure retains no second fee, repeated timeout refused, repeated uncertainty stable; every event type reachable and every rejected event leaves the account unchanged with an accepted event always appending a record; `availableEvents` agrees with the reducer; `fmt` never rounds; no vendor attribution in the model; static `kernel.html` transcript numbers equal the reducer's.

Attribution checks: `git log --all --grep='Co-Authored-By' --grep='Claude-Session' --grep='Claude Code'` returns nothing; grep for vendor/model names over the new and edited files returns only the test's own forbidden-word regex.

## Limitations and what was not established

- **No screen reader was driven.** Automation checked the status region's role and politeness, text labels, focus visibility and tab order. That is a precondition for a screen-reader pass, not the pass. Recorded as a gap in `site/test/README.md`.
- **200% zoom was approximated** by setting the root font size to 32px in a 640px viewport, not by browser zoom. Text-only scaling is what CSS `rem` responds to, so this is a reasonable proxy, not the same thing.
- **The failed-script fallback was tested by intercepting the enhancement bundle** (abort, and fulfil with a throwing script). A failure inside React's render after a partial commit was not separately simulated; `mount` only marks enhancement after `render` returns without throwing, and React 19's `createRoot().render` is asynchronous, so a render-time exception would surface later as a page error with the root marked enhanced. The static block would then be hidden. This is a residual gap in the fallback design for that specific failure class.
- **Deployment is not claimed.** Nothing was committed or pushed. Live bytes and the live `/Moriarty/kernel.html` route are unverified until the Pages workflow runs. The README link points at a URL that does not resolve until then.
- **The model is an illustration.** Its tests establish internal consistency with the design's accounting table. They establish nothing about Moriarty, the kernel, Midnight or any adapter's failure policy, and the page says so.
- **CI step is unexecuted here.** The new workflow step was written to mirror the local procedure (symlink `dist` under `pages/Moriarty`, serve on 8892) but has not run on GitHub Actions.
- **`site/dist` and `site/public/docs` are build outputs** and are gitignored; the audit should rebuild rather than trust a checked-in artifact.
- **Back/forward snapshot navigation** ("if present" in the design) was not implemented. The event record is the explanatory history; there is no navigation that could be mistaken for undo.
- The research directory's `verify.py` expects a server at `http://127.0.0.1:8897/`; it was not run by this session. All four of its commands were run individually as listed above, with a different port.

## Local servers

Two `python3 -m http.server` processes (ports 8891 and 8892) were started for testing and stopped at the end of the session.
