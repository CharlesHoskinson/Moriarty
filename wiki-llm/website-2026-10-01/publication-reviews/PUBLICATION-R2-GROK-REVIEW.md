# Publication R2 Grok review — Moriarty tutorial website

Independent full-source and actual-result audit of the repaired tutorial publication candidate. This is a website/README/CI/wiki review. It does not accept native proof, Preview financial settlement, or a public live Pages URL.

## Votes

| Question | Vote |
|---|---|
| Current full source (freeze candidate) | **Approve** |
| Actual-result faithfulness of the recorded 18 interaction, 13 CI smoke, and 45 anchor/reload/hold-cancellation checks, plus the named final screenshots | **Approve** |
| Scoped merge to `origin/main` and GitHub Pages publication of this freeze | **Approve**, with the sequencing constraint below |

**Sequencing constraint.** The README already names `https://charleshoskinson.github.io/Moriarty/tutorial.html` as the tutorial entry. That URL becomes a true statement of this candidate only after the `site` workflow deploys from `main`. This vote does not claim that the public live site currently serves these bytes. Native financial acceptance, Preview settlement, and PCD correspondence remain Open regardless of this publication.

**Blocking findings:** none.

---

## Reviewer identity

| Field | Value |
|---|---|
| Requested reviewer | `grok-4.6` at high effort |
| Returned identity | Grok 4.6 (xAI), this session |
| Checkout | `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` |
| Branch | `feat/tutorial-website-20261001` |
| HEAD (committed) | `cac114a44825f62fe36c1389ac64585d95a96e2e` |
| Candidate form | Uncommitted working tree matching the R2 freeze |
| Host session | `01a0f79f-7898-7920-9719-0a8c73157233` |
| Written output | this file only |

This session has no separate host `modelUsage` JSON. The returned identity is the Grok 4.6 system identity of the running reviewer. R1 Grok produced no vote: `wiki-llm/website-2026-10-01/publication-reviews/R1-PUBLICATION-GROK-RECEIPT.json` records `process_exit_code: 143`, `parse_error: "No structured receipt"`, empty stdout/stderr. That receipt is preserved; it does not vote here.

Startup: installed develop skill applied from `plugins/moriarty-dev/skills/develop/SKILL.md`; `AGENTS.md` loaded; `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` returned capability `SP01.6 loan-swap-subset`, `nextAction` `sp01-loan-report`, `pendingTransactions: []`. This review is independent authorized work. No campaign, `next`, native, Cargo, CLI-example, git, or source mutation was performed.

---

## Freeze identity

File: `/home/charl/research/moriarty-website-2026-10-01/PUBLICATION-R2-SOURCE-FREEZE.json`

| Check | Result |
|---|---|
| SHA-256 of freeze JSON | `e9a8a3a44f418f3798ab2e70ff81f9b7813e294c17dacd27053f49b3e3c3f3ee` |
| Changed files | 57 / 57 live bytes match |
| Unchanged example dependencies | 27 / 27 live bytes match |
| `base_main` | `cac114a44825f62fe36c1389ac64585d95a96e2e` |
| `origin/main` | same commit |
| Local `main` ref | stale `870f998b…` (unused; merge target is `origin/main`) |
| Scope | Website, README tutorial entry, CI smoke, faithful wiki review/result notes. No compiler, native, or wallet changes. |

Recomputed hashes on disk. Missing files: none. Mismatches: none.

`site/public/docs/` exists as a generated, gitignored tree (`site/.gitignore`). It is outside the freeze and is not a commit candidate. Wiki files in the freeze are exactly the 34 listed paths.

---

## R1 disposition (historical)

`wiki-llm/website-2026-10-01/publication-reviews/R1-DISPOSITION.md`: the first Astra publication audit reproduced a fresh `#architecture` landing defect and held publication. Root accepted that finding. The original Grok audit was withdrawn because the candidate required source changes; process exit 143; no structured receipt; no vote.

Two later source repairs are in this freeze:

1. Architecture is the last of 17 sections (`GuideApp.tsx` module order: Overview, GettingStarted, LanguagePatterns, Markets, RiskGovernance, CrossChain, Architecture). A layout-following hash hold re-aligns until height is steady or a 900-frame budget expires.
2. Reader input (`wheel`, `touchstart`, `pointerdown`, `mousedown`, `keydown`) calls `takeover()`, which stops the hold and clears the 1.5s place pin.

Original failures remain archived:

- `verification/TRANSIENT-SMOKE-FAILURE-RESULTS.json` — desktop transfer displaced (`top` 920.42 vs ceiling 63.75); mobile transfer obscured (`top` 63.13 vs ceiling 98.05).
- `verification/DISCLOSURE-FAILURE-RESULTS.json` and `DISCLOSURE-REPRODUCTION.md` — after disclosure plus programmatic repayment scroll, current label stayed Transfer until a 1px wheel; S1 then cleared the pin on all reader inputs.
- `verification/PRE-DIRECT-LINK-SMOKE-RESULTS.json` — earlier smoke without architecture-all-URL coverage.

Those archives describe earlier trees. They do not describe this freeze.

---

## Full source

Inspected: `GuideApp.tsx`, `GettingStarted.tsx`, `LanguagePatterns.tsx`, `Markets.tsx`, `RiskGovernance.tsx`, `CrossChain.tsx`, `styles.css`, `main.tsx`, `tutorial.html`, `vite.config.ts`, `site/src/App.tsx`, `.github/workflows/site.yml`, `site/test/tutorial.spec.py`, `site/test/README.md`, README Authoring beta paragraph, raw lessons (`transfer.mori` / `repay.mori` and their scenario/test JSON), wiki PLAN / DESIGN-INTEGRATION / POLISH-DECISION / IMAGE-ASSETS, six initial designer reviews, six rendered designer reviews, R1 publication files, and the verification tree.

### Shell and navigation

Seventeen sections in document order: overview, getting-started, transfer, repayment, testing, syntax, patterns, amm, lending, stablecoins, options, oracles, governance, bridges, staking, trust, architecture. Architecture is last, in group “Design and limits”.

`PATH` beginner route: overview → getting-started → transfer → repayment → testing → syntax → patterns → amm.

`AREAS` catalog: AMM, Lending, Stablecoins, Options / derivatives, Oracles, Governance, Bridges, Staking. Every area is SpecifiedOnly. Lending also points at the separate LocalS0 repayment lesson.

Hash hold (`hold`) re-scrolls each animation frame while layout is still growing. Stop conditions: document complete, fonts loaded, every content image complete, and height steady for 30 frames, **or** `++frames > 900`. A distant lazy image can keep the hold alive to that budget. `takeover` on real reader input ends hold and pin immediately. Pin lifetime is 1.5s (`performance.now() + 1500`). `--guide-offset` is measured from masthead plus mobile bar; `scroll-margin-top` uses that variable with `!important`.

Visible page `h1`: “Write a bounded financial agreement”. Lead states that the first lessons do not sign, prove, send or deploy. Scope chips: LocalS0, PreparedUnqualified, Open.

### Labels and financial distinctions

Support legend in Overview:

| Label | Meaning on this page |
|---|---|
| LocalS0 | Runs locally against a stipulated scenario |
| PreparedUnqualified | A candidate; unsigned, unproved, unsettled |
| SignedPreparedUnqualified | Native verifier checked a signature over exact source bytes; key authority, proof, and ledger acceptance remain Open |
| SpecifiedOnly | Structure and names checked; execution refused as Unsupported |
| Open | Named and not established |

Owner authorization, objective acceptance, and optional federation are three separate questions. The Federated DeFi Kernel is optional coordination; authoring, check, and local preparation never require it. Prove and Settle are the target pipeline.

Eight DeFi families match the shipped `packages/moriarty-beta/examples/` catalog (amm, lending, stablecoin, derivatives, oracle, governance, bridge, staking). Transfer and repay are the LocalS0 operations. The product contract treats ACTUS/DeFi families as libraries and examples; this tutorial’s eight-area catalog is that authoring set.

Assets, accounts, and identity: source alias, economic `id`, and display `symbol` are distinct. Scale converts display quantities to atom strings. No implicit conversion across assets or domains.

### Lessons and bytes

`GettingStarted` loads `site/src/tutorial/lessons/*`. Download names are `invoice.mori` / `repayment.mori` plus `scenario.json` and `mori.tests.json`. Captured CLI stdout is labelled captured output from commit `df8b1551…` on 2026-10-01, qualification `local-stipulation-only`. Wrong-expectation and Core-rejection cases are derived from the lesson JSON so they cannot drift.

Lesson identity vs freeze unchanged examples:

| Artifact | SHA-256 | Relation |
|---|---|---|
| `lessons/transfer.mori` (960 bytes) | `fb507bfb…1d6c` | Same as signed-intent `transfer-schnorr-raw/program.mori` and `transfer-ecdsa-wallet/program.mori` |
| `lessons/transfer.scenario.json` | `12928a79…0430` | Same as those signed-intent scenarios |
| `lessons/transfer.test.json` (1903, `77053522…3a4b`) | Lesson/init tests | Signed-intent `mori.tests.json` is a different artifact (`85fe2024…493a`, 1914 bytes) |
| `lessons/repay.mori` (915, `cda0142e…1c66`) | Same as signed `repay-ecdsa-raw/program.mori` | `examples/local/repay/repayment.mori` is 916 bytes with a trailing newline (`1fc1749e…9158`); Markets discloses the newline/hash difference |
| `lessons/repay.scenario.json` / `repay.test.json` | `00bfbfaa…fe8e` / `53b0c1ae…5155` | Match local repay fixtures |

`LanguagePatterns` imports complete files as `?raw` (repayment, transfer, lending, bridge, amm). Horizon snippets are `data-scope="horizon"` / `data-kind="horizon"` and labelled proposed syntax the beta parser does not accept.

`Markets` (AMM, lending, stablecoins) keeps SpecifiedOnly on swap/mint/redeem, borrow/roll/liquidate, mint/redeem/emergency. LocalS0 repayment is a separate heading with atom tables derived from the shipped scenario.

`RiskGovernance` pins GitHub blobs to `df8b155143e90450168af04fbfd20f7620043620` and records SHA-256 of derivatives/oracle/governance files matching the freeze unchanged set. Check output is labelled derived from the formatter, not a capture.

`CrossChain` records AuthoringChecked SpecifiedOnly check text and Unsupported expand/simulate JSON with `publishedEffects: null`. Trust matrix lists all eight families SpecifiedOnly / Open, with LocalS0 repayment linked separately.

Pipeline and workbench images: freeze SHAs `a1238482…93bf` and `6b0b16ce…d9f1`. `IMAGE-ASSETS.json` records them as conceptual illustrations. Alt/captions describe an open decorative gate, Target pipeline on Prove/Settle, and illustrative ledger figures.

### Home, Vite, CI, README

`vite.config.ts`: three HTML entries (`index`, `kernel`, `tutorial`), `base: './'` for `/Moriarty/` Pages.

`App.tsx`: Tutorials link in `.jump`; compact duplicate at `max-width: 1180px`, matching `site/src/styles.css` hiding `.jump` at that width.

`site.yml`: tests gate deploy; kernel and tutorial driven under `/Moriarty/`; Pages artifact and deploy only on `main` and non-PR.

`tutorial.spec.py`: 13 smoke checks — 17 fresh URLs desktop and mobile, architecture and architecture-kernel reload, visible pipeline, six download SHA matches, mobile home entry. It does not run the beta CLI.

README Authoring beta paragraph links the Pages tutorial URL and states PreparedUnqualified local preparation, signed-intent as a separate consumer, and Open authenticated state / native financial acceptance / ledger settlement.

---

## Actual-result faithfulness

Source approval is not an actual-result review. The recorded results were read in full against their harnesses.

### ROOT-CHECKS.json

`wiki-llm/website-2026-10-01/verification/ROOT-CHECKS.json` (freeze SHA `967e2aee…03b0`):

| Command | Recorded result |
|---|---|
| `npm run verify` in `site` | exit 0; typecheck true; 51 data tests; production build true |
| `python3 check-tutorial.py` | exit 0; 18 checks; actual clipboard and downloads; downloaded beta check/test |
| `site.spec.py` / `docs.spec.py` / `kernel.spec.py` | 49 / static reference / 197, recorded as earlier with identical home/kernel/reference source |
| `tutorial.spec.py` | 13; all 17 fresh anchors desktop/mobile plus exact downloads |
| `check-all-anchors.py` at 8896 | 45; fresh anchors, reloads, visible pipeline, reader hold cancellation |

Scope line in that file: local production build. Public deployment is not yet verified. Financial/native/Preview acceptance is not established.

This reviewer did not re-run `npm run verify`, the 18-check interaction harness, or any beta CLI. The interaction harness itself runs `node dist/cli.js check/test` on downloaded folders; those recorded stdout values were read, not re-executed.

### 18 interaction checks

`FINAL-INTERACTION-RESULTS.json` artifacts directory: `/home/charl/research/moriarty-website-2026-10-01/browser-check-20261001T131409810234Z`. Role: bounded browser verification contributor. URL: `http://127.0.0.1:8896/Moriarty/tutorial.html`. `passed: true`. 18/18.

Harness: `/home/charl/research/moriarty-website-2026-10-01/check-tutorial.py`. Check names in the JSON match the harness `check(...)` calls: section contract (17 ids, eight domain labels), desktop containment, one copy control per block, fresh `#install` / `#transfer` plus reload, search “nonce” retaining 17 sections, real clipboard plus six downloads, downloaded `check`/`test`, disclosure height then repayment current, mobile/tablet/narrow/dark/reduced-motion, mobile drawer/skip, mobile home Tutorials entry, blocked-clipboard honest failure, no uncaught JS.

Recorded CLI stdout on downloads (historical, not re-run here):

- transfer check: `AuthoringChecked Invoice` / `pay: LocalS0` / `Local preparation remains PreparedUnqualified.`
- transfer test: `TestsPassed`, `qualification: local-stipulation-only`, case `literal fee payment` / `PreparedUnqualified`
- repayment check: `AuthoringChecked LoanAgreement` / `repay_loan: LocalS0`
- repayment test: `TestsPassed`, `local-stipulation-only`, case `interest first partial repayment` / `PreparedUnqualified`

Download bytes in that artifact directory match freeze lesson SHAs (recomputed this session).

Search destination `overview-first` is the h3 “Your first program: a transfer”. The first-program source contains `nonce: "n1"`. Sections retained: 17.

Disclosure check: height 2536.64 → 3411.14; `current_after_scroll: repayment`. That is the original assertion that failed in the pre-pin-repair archive and now passes.

### 13 CI smoke checks

`DEPLOYMENT-SMOKE-RESULTS.json` (freeze SHA `4e884f88…4eab`): 13 named checks, all `passed: true`, `errors: []`. Download SHAs in that record equal the freeze lesson SHAs. Harness: `site/test/tutorial.spec.py`. Check names match.

### 45 anchor / reload / cancellation checks

`ALL-ANCHOR-RESULTS.json` (freeze SHA `23b3ad99…3b11`) is byte-identical in substance to `/home/charl/research/moriarty-website-2026-10-01/anchor-check-20261001T131409819292Z/results.json`. `passed: true`. 45 named checks, all true. Harness: `check-all-anchors.py`.

Composition: 17 fresh URLs × {1440, 390} + architecture and architecture-kernel fresh+reload × 2 widths + visible pipeline × 2 + wheel and keyboard cancellation × 2 + no page/console errors = 45.

Material archived geometry (1440 `#architecture`): `top` 76.1875, `ceiling` 63.75, `scroll` 61315, `max` 62018, `geometry_current` architecture, `current` architecture, `at_document_end` false. Reload identical. `#architecture-kernel` at document end still `geometry_current` architecture. Pipeline capture `settled-pipeline-1440.png` / `settled-pipeline-390.png`.

Wheel cancellation 1440: scroll 6273 → 6923. Keyboard 1440: 6273 → 7148, `geometry_current_after_keyboard` repayment. 390 wheel 10005 → 10655; keyboard 10005 → 10880, repayment.

### Independent 8896 observation this session

Read-only Playwright against the already-running freeze preview `http://127.0.0.1:8896/Moriarty/tutorial.html`, Chromium headless-shell, viewport 1440×1000, routes bounded to that origin.

| Observation | Fresh `#architecture` | After reload |
|---|---|---|
| HTTP | 200 | 200 |
| top | 76.1875 | 76.1875 |
| ceiling | 63.75 | 63.75 |
| scroll / max | 61315 / 62018 | 61315 / 62018 |
| geometry_current | architecture | architecture |
| nav aria-current | `#architecture` | `#architecture` |
| last of 17 sections | architecture | architecture |
| pipeline | complete, naturalWidth 1536, top 285.83, bottom 777.16, visible true | same |

These numbers match the archived 1440 architecture rows. Preview `.../preview/Moriarty` is byte-identical to `site/dist` (22 files, 0 mismatches).

Wheel/keyboard cancellation was not re-driven in this session. Faithfulness of those four checks rests on the archived 45-check, the harness using real `wheel` / `PageDown`, and source `INPUTS` including `keydown` with `takeover` clearing both hold and pin.

### Screenshots inspected

`browser-check-20261001T131409810234Z/final-desktop.png`: h1, LocalS0 / PreparedUnqualified / Open, first program Copy + `invoice.mori` start, eight area labels in the rail (AMM … Staking), Architecture last under Design and limits, Overview current.

`final-mobile.png`: Browse + Overview, same h1/scope/CTAs, first-screen product copy above the hero.

`final-pipeline.png` and `anchor-check-20261001T131409819292Z/settled-pipeline-1440.png`: Architecture current, pipeline fully in the viewport (Author → Settle, Target pipeline, Reject or revise), caption states conceptual/illustrative, LocalS0 on Author.

Rendered designer reviews (D1–D3, O1–O3) inspected the pre-polish baseline (`h1: 0`, hero-first, sentence-length area labels). POLISH-DECISION records the chosen repairs. Current screenshots and source show those repairs integrated: visible h1, labels and first program on the first screen, short area names, Architecture last, measured sticky offset, Browse drawer, one Copy per artifact.

Author seats (S1–S3 claude-sonnet-5-5; G1–G3 grok-4.7-build xhigh; S1 direct-link and reader-pin repairs) are recorded in `AUTHOR-MODEL-RECEIPTS.json`. Designer rendered seats: O1–O3 claude-opus-5-5 medium; D1–D3 requested gpt-6.1-sol medium with host attestation limitation recorded.

---

## Non-blocking observations

These do not hold source approval, result faithfulness, or the scoped merge/Pages vote.

1. **README Pages URL is a post-deploy destination.** After merge, the `site` workflow must run on `main` and succeed before that URL is a live observation of this candidate.
2. **Mixed GitHub pins.** `GettingStarted.tsx` and `RiskGovernance.tsx` pin blobs to `df8b155143e90450168af04fbfd20f7620043620`. `LanguagePatterns.tsx` and `CrossChain.tsx` use `blob/main`. The cited example files are in the unchanged-dependency freeze and match current bytes.
3. **Markets status chips omit `data-status`.** They still use class `guide-status`. Dashed border for SpecifiedOnly/Open is driven by `[data-status=…]` in `styles.css`, so Markets chips can render a solid border while other modules dash Open/SpecifiedOnly.
4. **Transfer lesson tests differ from signed-intent `mori.tests.json`.** Expected: lesson tests are the init/starter record; signed-intent tests are a separate artifact.
5. **900-frame hold budget.** Explicit in `GuideApp.tsx` and in POLISH-DECISION. A lazy offscreen image can keep the programmed hold alive to that cap. Real reader input clears hold and pin. This is a bounded startup behavior.
6. **Local `main` ref is stale.** Merge target is `origin/main` = `cac114`.
7. **This review did not drive a screen reader, public HTTPS Pages, or any native/Preview ledger path.**

---

## What this vote does not establish

- Public live GitHub Pages currently serving this freeze
- Native recursive proof, PCD, or Midnight ledger acceptance
- Preview financial settlement
- Key authority over accounts
- Kernel federation as a required path
- Home 49 / docs / kernel 197 as re-run in this session (ROOT-CHECKS records them as earlier on identical home/kernel/reference source; remote CI re-runs them on the publication commit)

---

## Merge and Pages instruction (scoped)

Commit the freeze-identified paths (57 changed files; example dependencies already on `cac114`). Do not add `site/public/docs/` (gitignored generated output). Merge onto `origin/main` (`cac114`). The `site` workflow on `main` builds, typechecks, runs data tests, drives home/docs/kernel/tutorial, then deploys Pages.

After that job, a separate actual public-URL observation is required before anyone asserts the live tutorial is this candidate. Until then the README link is the intended destination.

Financial and Preview acceptance remain Open.

---

## Blocking findings

None.

End of R2 Grok 4.6 high publication review.
