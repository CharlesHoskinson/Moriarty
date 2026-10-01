# O1 rendered design review: assembled baseline before polish

Date: 2026-10-01. Seat O1. Scope: information hierarchy, discovery of the eight areas, editorial
composition, a useful first screen, and navigation. This is a design review only. It is not an
implementation and not product acceptance.

## Identity and attestation

- Requested: Claude Opus 5.5, medium effort, seat O1.
- Returned provider attestation: not available to me. The root captures it externally. A model's
  statement about itself is not attestation.

## Coverage

- Startup: read `AGENTS.md`, applied the checked-in `moriarty-dev` develop skill, and ran guarded
  `status --json` in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Result: SP01.6
  loan-swap-subset, stale or missing evidence gaps, no pending transactions. I took no other CLI action.
- **Screenshots: I opened and looked at all six PNGs** with the image-reading tool:
  `initial-desktop.png` (1440, top), `initial-dark.png` (1440, top, dark), `initial-tablet.png`
  (768, top), `initial-mobile.png` (390, top), `initial-transfer.png` (1440, scrolled into the
  transfer lesson), `initial-options-mobile.png` (390, scrolled into Options). Each shot shows one
  viewport only. I did not see any section below the first screen except the two scrolled shots,
  and I saw no hover, focus, open-drawer, search or dark-mode code states.
- Read `INITIAL-BROWSER-OBSERVATION.json`: 17 sections, `h1: 0`, no errors, no overflow at
  1440/390/768, 0 authored copy buttons, 55 injected copy buttons.
- Source read: `site/src/tutorial/GuideApp.tsx`, `tutorial/styles.css`, the relevant parts of
  `GettingStarted.tsx`, `RiskGovernance.tsx`, the section and h2 outline of `Markets.tsx`,
  `CrossChain.tsx` and `LanguagePatterns.tsx`, the `App.tsx` diff, global `styles.css` masthead
  rules, and `docs/MORIARTY-PRODUCT-CONTRACT.md:76`. I also re-read my own earlier
  `O1-initial-design-review.md`. I read no other designer's report.
- Not done: no source edits, browser, build, tests, imports, git, network, native, proof, wallet,
  credential or environment actions. I did not render the homepage, so homepage findings are
  based on source only.

Labels: **[shot]** seen in a screenshot. **[src]** repository source observation. **[inf]**
inference. **[rec]** recommendation.

---

## Prioritised fixes

### F1. The first screen is a hero picture, not a product. No h1. Buttons come before the title.

- [shot] Desktop and dark at 1440: the screen runs buttons → eyebrow → h2 "What Moriarty is for"
  → two long paragraphs → a decorative illustration that fills the rest of the viewport. There is no
  code, no status label, no pipeline and no list of areas. Tablet at 768 looks the same. Mobile at
  390 shows the stacked buttons and one paragraph.
- [src] `GuideApp.tsx:448–451` (actions placed before the content), `:89–90` (eyebrow plus **h2**),
  `:104–110` (hero figure at full column width). The browser observation records `h1: 0`.
- [rec] At the top of `<main>`, before `.guide-actions`, render one page header:
  1. eyebrow `Tutorials · moriarty-beta/1`;
  2. **h1** "Write bounded financial agreements in Moriarty";
  3. a lead of at most two sentences. Paragraph 1 at `:91–97` can be cut down for this;
  4. one row of labels: `LocalS0 · PreparedUnqualified · SpecifiedOnly · Open`, linked to `#overview-legend`;
  5. after that, the two buttons.
  Move paragraph 2 (the kernel/permissionless note, `:98–103`) below the first program. Either
  remove the hero image from the first screen, or limit it to `max-height: 14rem; object-fit:
  cover` after the first program (`tutorial/styles.css:49–50`). Target at 1440: header, labels,
  buttons and the first lines of `invoice.mori` visible without scrolling.
- [shot] Dark mode: the illustration stays a bright cream block on a near-black page and becomes
  the brightest object on screen. [rec] If it stays, add `filter: brightness(.85)` under the
  dark-mode token, or move it lower as described above.

### F2. The eight areas can't be found by name, and the support matrix sits in the wrong group

- [shot] Desktop sidebar under "DEFI EXAMPLES" lists: "Swap a fixed input for at least a named
  output", "Repay accrued interest, then principal", "Mint against backing, and keep the
  redemption duty", "Options", "Oracles", "Governance", "Bridge escrow, claim and recovery",
  "Staking deposit, reward, slash and exit", "Support, identity and federation". Five entries are
  sentences that wrap to two lines and three are bare nouns, so the list reads as uneven. The
  words AMM, Lending and Stablecoins appear nowhere in it. A ninth, non-area entry (the support
  matrix) sits in the same list.
- [src] Labels come from each section's h2 (`GuideApp.tsx:256`). The h2s are at `Markets.tsx:51,213,407`,
  `RiskGovernance.tsx:264,557,761`, `CrossChain.tsx:144,285`. The trust section is
  `CrossChain.tsx:415–416`, inside `<Module group="DeFi examples">` (`GuideApp.tsx:460–464`).
- [rec]
  1. Read a short label from `data-nav` on each `section` in place of the h2 text (change
     `GuideApp.tsx:256` to `el.dataset.nav ?? h2`). Lead each label with the area name: `AMM`,
     `Lending`, `Stablecoins`, `Options`, `Oracles`, `Governance`, `Bridges`, `Staking`. Keep the
     editorial sentence as the h2, with an eyebrow such as `AMM · SpecifiedOnly` above it.
  2. Add `data-group="Design and limits"` to the trust section, and change the group lookup to
     prefer `el.dataset.group` over the closest group (`:257`). Then rename it **Support matrix**
     and place it beside Architecture.
  3. Add an **eight-area index** to Overview immediately after the first program: a 2×4 grid of
     linked cards (one column below 640px). Each card shows the area name, the status in words
     (all eight `SpecifiedOnly`) and one line of scope. The Lending card may also link to the
     runnable `LocalS0` repayment lesson, labelled as a separate operation. This gives a
     first-screen route to every area at 390 and 768, where the sidebar is hidden.

### F3. On narrow screens the stacked chrome takes about 180px, and the lesson bar disappears

- [shot] At 390 the masthead wraps into three rows ("Overview Tutorials" / "Kernel explanation" /
  "Language reference"), with the wordmark centred vertically beside them. This band is about
  120px. "Browse lessons" adds about 55px and "On this page" another 40px. At 768 the masthead
  fits on one row.
- [shot] In `initial-options-mobile.png`, scrolled into Options, the sticky masthead is still
  about 120px tall. **The Browse lessons bar is not visible.** Body text shows faintly through the
  translucent masthead above the cut line. This confirms the concern I raised as P6 in my initial
  review.
- [src] `.guide-bar { position: sticky; top: 3.4rem }` (`tutorial/styles.css:85`) assumes a
  single-row masthead. `.guide-jump` wraps at ≤640px (`:8`). The masthead is translucent
  (`styles.css:115–116`), so the bar slides underneath it.
- [rec] At ≤640px, make the masthead one row: wordmark plus a "Menu" or compact link set
  (`Home · Tutorials · Kernel · Reference`, at 0.8rem, horizontal scroll if needed, no wrap). Set
  a `--masthead-h` custom property and use `top: var(--masthead-h)` for `.guide-bar`. Make the
  masthead opaque on tutorial pages. Then follow my initial P6: turn the opened sidebar into a
  fixed drawer under the bar, focus the search field, and fold "On this page" into the drawer so
  it is no longer a separate block above the content (`styles.css:82`, `order: -1`).

### F4. Scroll-spy reported the wrong lesson in the scrolled desktop shot

- [shot] In `initial-transfer.png` the viewport shows the transfer `mori.tests.json` block
  (`"literal fee payment"`), but the sidebar still highlights "What Moriarty is for". "On this
  page" lists the Overview h3s.
- [src] The active section comes from an IntersectionObserver (`GuideApp.tsx:284–298`). The
  initial hash scroll runs in the lessons effect (`:260–262`), before the observer is attached.
- [inf] The shot may have been taken before the observer fired, so the cause is not confirmed.
  Either way, the shell can show the wrong location. [rec] After attaching the observer, run
  `setActive` once by hand: pick the last section whose `getBoundingClientRect().top` is at or
  above 80px. Also re-run it on `hashchange`. Root should repeat the shot after a delay to tell
  which cause it is.

### F5. Code frames are nested, and copy controls appear twice

- [shot] `initial-transfer.png`: the JSON sits in a grey `figure.guide-code` box with a second
  bordered `pre` box inside it, so there are two frames and two inset margins. The 360–1095px band
  holds an inner box at about 377–1078px.
- [src] `GettingStarted.tsx:79`, `CrossChain.tsx:57` and `LanguagePatterns.tsx:52` put
  `guide-code` on a `<figure>`. `Markets.tsx:32` and `RiskGovernance.tsx:172,204` put it on the
  `<pre>`. The shell injects "Copy code" unless the previous sibling is a copybar
  (`GuideApp.tsx:318`). This produces the 55 injected buttons, including duplicates beside the
  authored Copy buttons in figcaptions.
- [rec] Use one shared `Code` component (a `figure.guide-codeblock` containing a header row with
  title, status label, Copy/Download/Source, followed by a single `pre.guide-code`). Until then,
  as a minimum: (a) the shell skips any `.guide-code` that is a `figure`, or that contains or
  follows a `button`; (b) add `figure.guide-code > pre { border: 0; background: none; padding: 0; margin: 0 }`
  and `figure.guide-code figcaption { white-space: normal }`.

### F6. Status labels are drawn in at least four ways. One of them makes a sentence into a chip.

- [src] There are four status components: `GuideApp.tsx:53` (children), `GettingStarted.tsx:54`
  (`status`), `Markets.tsx:26` (`value`), `CrossChain.tsx:19` (`name`). `LanguagePatterns.tsx:21`
  is a fifth. `RiskGovernance.tsx:90–95` applies `className="guide-status"` to a whole `<p>`, for
  example "SpecifiedOnly · Open · Unsupported" inside a callout (`:214`). The chip rule
  (`tutorial/styles.css:65–68`: inline-block, border, highlight fill) then applies to a paragraph.
  `RiskGovernance.tsx` also uses inline `style` objects (`:13–48`) instead of the shell classes.
  That gives prose at 1.0625rem/1.6 against 1.05rem/1.7 elsewhere.
- [inf] These cases were not in the screenshots I inspected, so the paragraph-chip render is
  inferred. If it renders as a chip, a SpecifiedOnly paragraph looks like a badge, which weakens
  the label system.
- [rec] Use one `Status` export with a `name` prop and per-class borders: solid for
  `LocalS0`/`PreparedUnqualified`, dashed for `SpecifiedOnly`, outline with a leading ○ for
  `Open`. Rename the RiskGovernance line class to `guide-statusline` (plain mono text, no chip),
  and replace its inline styles with shell classes. Either add `Unsupported` and `AuthoringOnly`
  to the legend (`GuideApp.tsx:154–163`) or write them as plain words.

### F7. Homepage: tutorial link still hidden below 1181px [src only]

- [src] The `App.tsx:37` diff adds Tutorials to `.jump`, but `.jump` is `display: none` at
  ≤1180px (`styles.css:637`). `Opening.tsx` has no link to the tutorial.
- [rec] Same as my initial P1. Add "Write your first agreement →" and "What runs today" links in
  `Opening` after the lede, with one honest scope line beneath: "Today: checking and local
  unqualified preparation. Native proof and ledger settlement: open." (This matches product
  contract `:76`, "This end-to-end capability is unfinished.") Keep the cross-page links visible
  at all widths. Use the same nav labels on every page: **Home · Tutorials · Kernel · Language
  reference**. The tutorial currently calls the homepage "Overview" (`GuideApp.tsx:377`), which
  clashes with its own Overview section.

### F8. Editorial details visible in the shots

- [shot] Options at 390: long prose with dense inline code reads well. The text "Scroll sideways
  to read every column." sits as a plain muted paragraph, separate from its table. [rec] Put it
  inside the table caption, or show it only when the table actually overflows. Add a right-edge
  fade to scroll regions.
- [shot] "ON THIS PAGE" on desktop lists only the current lesson's h3s. That works. On mobile and
  tablet it is a collapsed box above the buttons and comes before the page title. See F3.
- [rec] Neutral notes use the accent callout (`styles.css:70`). Keep the accent for hostile or
  rejection cases, and use `--paper-sunk` with a `--rule-strong` edge for neutral notes such as
  "Names and identities" (`GuideApp.tsx:132–146`).
- [rec] Add Previous/Next links at the end of each section, so a reader on the long single page
  has a straight path through it without opening the drawer.

## Order of work

F1 and F2 together fix the first screen and the discovery of the eight areas. F3 is a real
mobile navigation defect seen in a screenshot. F5 is a visible box-in-box defect. F4 needs a
delayed re-shot to identify its cause. F6–F8 are polish. None of these changes may alter the
factual labels. Keep `LocalS0`, `PreparedUnqualified`, `SpecifiedOnly` and `Open` exactly as
written. General native proof, Preview financial settlement and ledger acceptance remain open,
and nothing on the page may imply otherwise.

## Re-shot checks after repair

1. 1440 and 390, top: h1, lead, label row, buttons, start of the first program. Hero not above it.
2. Overview area index: eight named cards, each showing `SpecifiedOnly`. Sidebar labels start with the area name. Support matrix appears under Design and limits.
3. 390, scrolled mid-page: one-row masthead, Browse lessons bar visible, drawer opens in view.
4. Deep link `#transfer` after a 1s delay: sidebar highlights "Transfer with a fee".
5. Transfer code: a single frame, one copy control per block. The injected count drops.
6. A dark-mode shot of a status-heavy callout (Options `Boundary`).
7. Homepage at 1024 and 390: tutorial link visible above the fold.
