# O1 initial design review: information hierarchy, composition and navigation

Date: 2026-10-01. Scope: initial, independent, source-only design review of the existing
website and the tutorial plan, before the integrated build. Not an implementation, not a
render review and not product acceptance.

## Identity and attestation

- Requested seat: Claude Opus 5.5, medium effort (seat O1).
- Actual returned provider attestation: **unavailable** in this session. No provider receipt
  was given to me. The model's own statement of its identity is not attestation.
- Continuation note: the prior O1 CLI invocation ended with exit 143 and no provider receipt.
  This is the same-scope continuation. No earlier O1 report existed in the report directory.

## What I inspected and what I did not

Startup: read `AGENTS.md` and the installed `moriarty-dev` develop skill, then ran the guarded
`status --json` from `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. It reported
capability SP01.6 loan-swap-subset, the stale/missing evidence gaps listed there, and no
pending transactions. I took no other CLI action.

Read (source only): `site/src/App.tsx`, `sections.tsx`, `mode.ts`, `styles.css`,
`components/Opening.tsx`, `Language.tsx`, `Categories.tsx`, `site/tutorial.html`,
`site/index.html`, `site/vite.config.ts`, `site/package.json`;
`packages/moriarty-beta/GETTING-STARTED.md` (first ~120 lines), `examples/README.md`,
`examples/amm.mori`, the example line lengths; `docs/MORIARTY-PRODUCT-CONTRACT.md`;
`wiki-llm/beta-language-2026-09-30/TUTORIAL-SITE-PLAN.md`; the root `PLAN.md`.

I also read the **in-progress, uncommitted** tutorial sources that are already on disk:
`site/src/tutorial/GuideApp.tsx`, `tutorial/styles.css`, and the heading/code-block outline
of `GettingStarted.tsx`, `LanguagePatterns.tsx`, `Markets.tsx`, `CrossChain.tsx`. These
files are still being written by other workers. Findings that cite them are **provisional
source observations about a draft**, not claims about the finished tutorial or its render.
`RiskGovernance.tsx` is imported by `GuideApp.tsx:5` but was not on disk when I looked.
That is expected during authoring and is noted only so integration checks for it.

Not done: no browser, screenshots, build, typecheck, tests, imports, network, native, proof,
wallet, credential, environment or git operations. I did not read any other reviewer's
output (D1–D3, O2, or any receipts). All layout effects below are inferred from CSS and JSX.
The follow-up review will need the assembled build and screenshots to confirm them.

Labels: **[obs]** repository observation, **[inf]** inference from source, **[rec]**
recommendation.

---

## Priority fixes (in order)

### P1. The homepage has no visible route to the tutorial below 1181px

- [obs] `App.tsx:37` puts the only Tutorials link in the `.jump` strip.
  `styles.css:637` sets `@media (max-width: 1180px) { .jump { display: none; } }`.
  `Opening.tsx` has no tutorial link in its body.
- [inf] On most laptops, tablets and phones the homepage has no visible path to the
  tutorial. The tutorial is the main thing a developer arriving today can actually use.
- [rec] Add a persistent primary action in `Opening` right after the lede. Two links:
  "Write your first agreement →" (`tutorial.html#getting-started`) and "What runs today"
  (`tutorial.html#overview-legend`, or the support matrix anchor once it exists). Separately,
  keep Tutorials/Kernel/Docs visible in the masthead at every width. Only the section
  anchors should collapse; the cross-page links should not. Split `.jump` into
  `.jump-sections` (collapsible) and `.jump-pages` (always shown).

### P2. The tutorial page has no h1, and its title/lead sit below a call to action

- [obs] React replaces the static `<h1>Moriarty tutorials</h1>` in `tutorial.html`. The
  rendered page then begins with `.guide-actions` buttons (`GuideApp.tsx:448–451`), followed
  by the Overview eyebrow and an **h2** "What Moriarty is for" (`GuideApp.tsx:89–90`). The
  sidebar group labels are also `h2` (`GuideApp.tsx:423`), so the outline interleaves "Start
  building" and "Language" with lesson titles.
- [inf] Screen-reader and visual readers get no page title. The first visible element is
  "Start with a transfer" before the page has said what Moriarty is. The heading outline
  mixes navigation labels with content headings.
- [rec] Render a page header inside `<main>` before the lessons:
  - h1: **"Write bounded financial agreements in Moriarty"** (or keep "Moriarty tutorials"
    as the eyebrow).
  - Lead, two sentences at most: *"A Moriarty program states exactly which asset, accounts,
    caps and rounds an owner authorizes. The beta toolchain checks it and prepares a local,
    unqualified candidate; signing, proof and settlement on Midnight are not done here."*
  - The support-label row (LocalS0 · PreparedUnqualified · SpecifiedOnly · Open) in one line,
    linking to the legend.
  - Then the two actions.
  Change the sidebar group labels from `h2` to a non-heading element (e.g. `<p
  class="guide-navhead" id=…>` with `aria-labelledby` on the `<ul>`). Content h2s are then
  the only level-2 headings.

### P3. Purpose explanation is split: the pipeline sits last, the hero first

- [obs] `Overview` holds the hero image, the full transfer program, two callouts, the
  legend and the release note (`GuideApp.tsx:86–177`). `Architecture`, which explains *how an
  intention becomes a candidate* and where tooling stops (Author → Authorize → Propose →
  Prove → Settle), is the very last module under "Design and limits"
  (`GuideApp.tsx:465–467`).
- [inf] The one diagram that explains the product's purpose and honest boundary comes after
  every DeFi lesson. The hero illustration ("workbench") is decorative and takes the full
  46rem column before the first line of code.
- [rec] Reorder the Start building group to: **Overview → How it fits together →
  Getting started → Transfer → Repayment → Testing**. In Overview, either replace the hero
  with the pipeline figure or shrink the hero to a short banner (max ~220px tall,
  `object-fit: cover`) placed after the lead. Keep the five-step `<ol>` text equivalent next
  to the pipeline image. Move "Release and sources" to the end of the page or the Limits
  group. A reader's first screen should be: title, lead, label row, pipeline, first program.

### P4. The eight DeFi areas cannot be found by name

- [obs] The sidebar shows each section's `h2` text (`GuideApp.tsx:256`). The draft area
  headings are task sentences: "Swap a fixed input for at least a named output", "Repay
  accrued interest, then principal", "Mint against backing, and keep the redemption duty",
  "Bridge escrow, claim and recovery", "Staking deposit, reward, slash and exit".
  The trust/support matrix "Support, identity and federation" sits inside the DeFi examples
  group (`CrossChain.tsx:416`). No list on the page names all eight areas.
- [inf] A reader looking for "AMM", "lending" or "oracles" has to recognise them from
  sentences. Search helps only if they guess a term in the body. The support matrix belongs
  with limits, not with examples.
- [rec]
  1. Add an **eight-area index** to Overview, directly after the first program: a 2×4 grid
     (1 column below 640px) of linked cards. Each card shows the area name, its status label
     in words, and one line on what the example covers. Order: AMM, Lending, Stablecoins,
     Options/derivatives, Oracles, Governance, Bridges, Staking. Every card must show
     **SpecifiedOnly** where that is the truth. The lending card may additionally point to
     the runnable **LocalS0** repayment, clearly as a separate operation.
  2. Give every section a short nav label through a `data-nav` attribute, read by the shell
     in place of the h2 text: e.g. `AMM — fixed-input swap`, `Lending — repay accrued first`,
     `Stablecoins — mint and redeem`. The area name always comes first.
  3. Move "Support, identity and federation" into the "Design and limits" group as
     **Support matrix**, next to Architecture. Wrap it in a separate `<Module group="Design
     and limits">` or set `data-group` on the section.
  4. Keep the area H2 as the editorial sentence, preceded by an eyebrow with the area name
     (`<p class="guide-eyebrow">AMM · SpecifiedOnly</p>`).

### P5. Code blocks have conflicting wrappers and duplicate copy buttons

- [obs] There are four different `Code` components. `GettingStarted.tsx:79`,
  `CrossChain.tsx:57` and `LanguagePatterns.tsx:52` put `className="guide-code"` on a
  `<figure>` holding a `figcaption` and a `<pre>`. `Markets.tsx:32` puts it on the `<pre>`.
  `.guide-code` sets `white-space: pre; overflow-x: auto; padding; background; border`
  (`tutorial/styles.css:54–58`). The global `pre` rule (`styles.css:165–169`) gives the inner
  `<pre>` its own padding, background and border.
- [obs] GettingStarted and CrossChain put a copy button inside the figcaption. The shell's
  auto-copy pass (`GuideApp.tsx:316–345`) only skips a `.guide-code` whose *previous sibling*
  is a `.guide-copybar`, so it also inserts a "Copy code" bar above those figures.
- [inf] Figure-style blocks render as a box within a box. The figcaption inherits
  `white-space: pre`, so long captions (status, note, Source link, Copy, Download) will not
  wrap and scroll sideways with the code on phones. The two modules that already have copy
  buttons get a duplicate.
- [rec] Standardise on one shared component in a small `tutorial/Code.tsx` (or a shell-owned
  class contract). Structure: `<figure class="guide-codeblock">` → header row (`title`, status
  label, scope, Source/Download/Copy aligned right, wrapping) → `<pre class="guide-code">`.
  Apply `white-space: pre` and the frame only to the `pre`. Have the shell skip any
  `.guide-code` whose enclosing figure already contains a `button`. Use one `Status` component
  (there are currently three prop shapes: children, `value`, `name`) so labels look the same
  in every lesson.

### P6. Mobile "Browse lessons" opens off-screen

- [obs] Below 1280px the sidebar is `position: static` in the single-column grid
  (`tutorial/styles.css:90–91`). The sticky `.guide-bar` toggles `is-open`. Focus is not moved
  into the menu (`GuideApp.tsx:385–394`). The `.guide-toc` gets `order: -1`, so it renders
  above the sidebar and main.
- [inf] A reader who scrolls into a lesson and taps "Browse lessons" opens a panel at the top
  of the document, out of view, and nothing appears to happen. "On this page" sits at the top
  of a very long page and only helps before scrolling.
- [rec] Make the narrow-screen sidebar a drawer: `position: fixed` beneath the guide bar,
  `max-height: calc(100dvh - bar)`, its own scroll, focus the search field on open, return
  focus on Escape/close (already handled). Below 1280px, move "On this page" into that drawer
  as a second list under the current lesson, rather than a separate block at the top of the
  page. Check the guide bar's `top: 3.4rem` against the real masthead height at 640px. The
  tutorial masthead wraps four links there (`tutorial/styles.css:8`), so the bar may slide
  under it. That needs a screenshot to confirm.

---

## Full code versus short excerpts

Grounding: [obs] the eight family examples are 12–20 lines, but each `intent` is a single
line, up to **233 characters** (`examples/*.mori`). The `mori init` transfer is 18 lines with
shorter lines (`GuideApp.tsx:21–38`). `Markets.tsx` already uses an excerpt plus `<details>`
full source.

[rec] Use one consistent rule across all lessons:

1. **Runnable lessons (transfer, repayment, testing):** show the complete file inline,
   because it is short, runnable and the subject of the lesson. Provide Copy and Download.
   Put the hand-derived amounts next to it (GettingStarted already does this).
2. **SpecifiedOnly area lessons:** lead with one excerpt of 3–8 lines centred on the
   `operation:` call and its bounds (`net_floor`, `fee_cap`, `gross_cap`). Then put the full
   verbatim file in a `<details>` titled "Full `amm.mori` (verbatim, SpecifiedOnly)" with
   Copy/Download/Source. Do not open full sources by default.
3. **Long one-line intents:** the downloadable and full blocks stay verbatim with horizontal
   scroll and a visible right-edge fade so the overflow is noticeable. If an excerpt is
   re-broken across lines for reading, label it "re-wrapped for reading; the download is
   verbatim". The beta grammar accepts the whitespace, but readers must not take a
   reformatted block for the shipped file.
4. **Typed-horizon/proposed syntax** (`LanguagePatterns.tsx:47–49` already labels it): give
   it a visibly different frame, e.g. a dashed border plus a header reading "Proposed — not
   accepted by the beta parser", so it cannot be mistaken for pasteable beta source. Never
   offer Download on these blocks. Copy is acceptable only with that header.
5. Every code header carries its status label in words. A block without a label is a defect.

## Status labels as composition, not decoration

- [obs] `.guide-status` (`tutorial/styles.css:65–68`) renders every label as the same
  neutral chip. `Markets.tsx` also chips `Unsupported` and `AuthoringOnly`, which the legend
  (`GuideApp.tsx:154–163`) does not define.
- [rec] Keep the words as the carrier and add a secondary non-colour cue per class: solid
  border for **LocalS0/PreparedUnqualified** (runs locally), dashed border for
  **SpecifiedOnly** (checked, not executed), outline-only with a leading "○" for **Open**.
  Either add `Unsupported` and `AuthoringOnly` to the legend with one-line meanings or write
  them as prose rather than chips. The product rule is that SpecifiedOnly must never look
  like execution, and a shared chip style currently weakens that.

## Homepage hierarchy relative to the tutorial

- [obs] The homepage Language section presents `moriarty-bounded-atomic/1` and
  `moriarty-successor-syntax/0` samples (`data/language.ts:62,68`; `Language.tsx:127–161`).
  The tutorial teaches `moriarty-beta/1` and warns against pasting successor samples
  (`GuideApp.tsx:171–173`). The homepage gives no matching warning.
- [rec] Add one line above the homepage source samples: "These samples illustrate earlier
  profiles. To write code you can check today, use the `moriarty-beta/1` tutorial." Link it.
  Also align the cross-page nav labels. The homepage says Kernel/Docs; the tutorial says
  "Kernel explanation"/"Language reference" and uses "Overview" for the homepage, which
  clashes with the tutorial's own Overview section. Use **Home · Tutorials · Kernel ·
  Language reference** on every page.
- [rec] The homepage hero (`Opening.tsx:12–17`) is an editorial claim ("Financial meaning
  that survives compilation, proof and settlement") with no status nearby. Under the P1
  action links, add a one-line honest scope: "Today: checking and local unqualified
  preparation. Native proof and ledger settlement: open." This matches the product contract
  ("This end-to-end capability is unfinished").

## Smaller layout notes

- [obs] `.guide-main p { max-width: 72ch }` over a 46rem column at 1.05rem is about right.
  Keep it. [rec] Callouts (`guide-callout`) use `--accent-soft` with the accent rule. The
  global stylesheet reserves the accent for "where the divergence is" (`styles.css:22`).
  Reserve accent callouts for hostile/rejection cases and use `--paper-sunk` plus a
  `--rule-strong` edge for neutral notes such as "Names and identities".
- [rec] Add "Next: <lesson>" and "Previous" links at the end of each section. On a
  single long page this gives a linear path without depending on the sidebar.
- [obs] Search hides whole sections (`GuideApp.tsx:265–281`). [rec] When a search is active,
  show the match count above the content too (not only in the sidebar). On mobile the
  sidebar is closed, so a reader could see only part of the page with no explanation.
- [obs] Section discovery runs once after mount (`GuideApp.tsx:253–263`). [inf] This is fine
  for static modules, but integration should confirm all six modules render synchronously.
- [rec] The reading-position note in the sidebar (`GuideApp.tsx:441–444`) is good and
  honest. Keep it.

## Checks for the follow-up build review

1. Homepage at 1024px and 390px: tutorial link visible above the fold (P1).
2. Tutorial accessibility tree: one h1, content h2s only, nav labels not headings (P2).
3. First tutorial screen at 1440px and 390px: title, lead, labels, pipeline, then code (P3).
4. All eight areas reachable from the Overview index and the sidebar by area name (P4).
5. One copy control per code block; captions wrap at 390px; no box-in-box (P5).
6. Browse lessons opened mid-page at 390px is visible and focused (P6).
7. Every code block header has a word label; proposed syntax is visually distinct and has
   no Download.
8. `RiskGovernance` exists and the options/oracles/governance sections appear in the
   DeFi group.

No finding here establishes financial execution, proof or settlement. The tutorial must not
imply any of them.
