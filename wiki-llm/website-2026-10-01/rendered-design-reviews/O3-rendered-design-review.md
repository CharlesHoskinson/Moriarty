# O3 rendered design review: assembled tutorial baseline (before polish)

Date: 2026-10-01. Scope: design review only, no implementation. Domain: responsive
layout, accessibility, caption and code overflow, contrast, imagery and captions,
and sticky header and drawer behaviour.

## Identity, startup and coverage

- Requested seat: Claude Opus 5.5, medium effort (O3). This worker self-reports as
  `claude-opus-5-5`. That is not an attestation. Root records the returned identity
  separately.
- Startup: read `AGENTS.md` and applied the checked-in `moriarty-dev:develop` skill
  (`plugins/moriarty-dev/skills/develop/SKILL.md`). Ran only `cli.py --repo . status --json`
  in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. It returned capability
  `SP01.6 loan-swap-subset`, next action `sp01-loan-report`, no pending transactions,
  and evidence gaps (stale SP01 inputs, missing current accounting, unavailable resource
  live state). None of these affect this review.
- **Screenshots: I viewed all six PNGs directly** with the image-reading tool:
  `initial-desktop.png` (1440×1000), `initial-dark.png` (1440×1000),
  `initial-transfer.png` (1440×1000), `initial-tablet.png` (768×1000),
  `initial-mobile.png` (390×1000) and `initial-options-mobile.png` (390×1000).
  Each one is a single 1000 px viewport. I saw only those viewports and did not see
  the rest of any page. Pixel heights below are estimates read from the images.
  They are not DOM measurements.
- I also read `INITIAL-BROWSER-OBSERVATION.json` (17 sections, `h1: 0`, no errors,
  overflow 0 at 1440/390/768, `copy_buttons: 0`, `injected_copy_buttons: 55`).
- Source read: `site/tutorial.html`, `site/src/tutorial/GuideApp.tsx`, `styles.css`,
  the code, table and status components in `GettingStarted.tsx`, `LanguagePatterns.tsx`,
  `Markets.tsx`, `RiskGovernance.tsx` and `CrossChain.tsx`, the base `site/src/styles.css`
  (tokens, masthead, `.jump`, `pre`, `th`), and my own earlier `O3-initial-design-review.md`.
  I did not read any other designer's report.
- I made no source edits and ran no build, browser, test, import, git, network, native,
  proof, wallet, credential or environment operation. Contrast ratios are WCAG
  arithmetic on the CSS hex tokens. They are not browser measurements.
- Factual labels must stay exactly as written: LocalS0, PreparedUnqualified, SpecifiedOnly,
  Open. General native proof and Preview acceptance remain **Open**. Nothing in this
  review accepts the site or any example.

## Priority list

| # | Sev | Finding | Evidence | Owner |
|---|---|---|---|---|
| 1 | Blocker | Mobile masthead is ~122 px tall, and the sticky Browse bar hides under it | mobile + options-mobile shots | Shell |
| 2 | Blocker | 55 injected duplicate copy buttons; `.guide-code` on `<figure>` makes nested boxes and unwrappable captions | JSON + transfer shot + source | Shell + all modules |
| 3 | High | About 48 missing word spaces in RiskGovernance (JSX newline collapse); 9 code chips in GettingStarted have a leading space | options-mobile shot + source | RiskGovernance, GettingStarted |
| 4 | High | No `h1`; the visible page title is an `h2` | JSON `h1:0` + all shots | Shell |
| 5 | High | Image alts still describe things the images don't show; hero uses gate/check acceptance imagery | desktop/tablet/dark shots | Shell / root assets |
| 6 | High | Hero fills the first screen and has no size attributes; transfer deep link lands in the wrong place and shows a stale active lesson | desktop + transfer shots | Shell / root |
| 7 | High | Translucent masthead lets text show through; site links are `--ink-faint` 0.76 rem (3.74:1) | transfer + options-mobile shots | Shell |
| 8 | Medium | Table patterns differ across modules; base `th` contrast fails | source; options-mobile hint | Modules + shell |
| 9 | Medium | Full JSON expectations are printed inline (~45+ lines) | transfer shot | GettingStarted |
| 10 | Medium | Dark mode shows a bright paper-white hero | dark shot | Shell |
| 11 | Low | Skip-to-navigation targets a hidden drawer below 1280 px; status chips look identical | source | Shell |

## Findings and fixes

### 1. Mobile sticky header and drawer (blocker)

Observed at 390 px: the wordmark sits beside a column of three link rows (Overview +
Tutorials / Kernel explanation / Language reference). The masthead's bottom rule is
at about y≈122, so the header takes about 12% of the viewport. In
`initial-options-mobile.png`, scrolled into Options, the **Browse lessons bar is
not visible at all**. `.guide-bar` is `sticky; top: 3.4rem` (≈54 px,
`tutorial/styles.css:85`), so it slides under the 122 px masthead (`z-index:10` above
the bar's 9). On a phone the drawer is therefore unreachable once you scroll. At
768 px the masthead is one row (~52 px) and the bar sits correctly under it
(tablet shot).

Fix (`tutorial/styles.css:5-8, 84-87`; `GuideApp.tsx:373-396`):
- Below 640 px, keep the site nav on a single row:
  `.guide-jump{flex-wrap:nowrap; overflow-x:auto; gap:var(--s3)}`. Shorten labels
  through a `<span class="long">` hidden at narrow widths (for example "Kernel",
  "Reference"). Or move the site links into the Browse drawer and show only the
  wordmark plus Browse in one sticky row.
- Drive the offsets from one variable: `--mast-h` (set to 3.4rem, or 3rem on mobile).
  Use `.guide-bar{top:var(--mast-h)}` and
  `html{scroll-padding-top:calc(var(--mast-h) + 3.25rem)}`. Remove the hard-coded
  `scroll-margin-top:5rem` at lines 36/39 and in `RiskGovernance.tsx:13-14`.
- With the drawer open, keep the open panel visible:
  `.guide-side.is-open{position:sticky; top:calc(var(--mast-h)+3.25rem); max-height:70vh; overflow:auto}`.
  It is currently `position:static` (line 90), so it opens at the top of the page,
  possibly off-screen above the reader. Clicking Browse while deep in the page should
  show it in place.

### 2. Code blocks and copy controls (blocker)

The observation counts 55 injected "Copy code" buttons. In source,
`GuideApp.tsx:316-345` injects a button before every `.guide-code` whose previous
sibling is not `.guide-copybar`. The modules' own controls never meet that test:
- `GettingStarted.tsx:79`, `LanguagePatterns.tsx:52` and `CrossChain.tsx:57` put
  `className="guide-code"` on the **figure**. GettingStarted and CrossChain already
  have a Copy button in the figcaption, so they get a second one.
- `RiskGovernance.tsx:172` `SourceFrame` has its own Copy/Download toolbar, but the
  `pre` sits after a `p[role=status]`, so the shell injects a third control. The
  derived-check `pre` at line 204 also gets one.
- `Markets.tsx:32` uses a bare `pre.guide-code` inside `<details>`. That is the only
  case where injection is the sole copy control.

Visible effect: in `initial-transfer.png` the JSON block is **a box inside a box**.
The outer sunk panel with its border spans about x 360–1095 and the inner `pre` about
x 378–1078. This happens because `.guide-code` (border, padding, sunk background)
styles the figure and base `pre` (`styles.css:165-168`) styles the inner element too.
`.guide-code` also sets `white-space: pre` on the figure, which the figcaption
inherits. Long captions such as LanguagePatterns' "Typed horizon concept (proposed
syntax, not executable, not accepted by any beta parser)", or CrossChain's
note + Source + Copy, **cannot wrap**. They scroll sideways inside the figure, and at
390 px the copy button can end up off-screen. Page overflow stays 0 only because the
figure scrolls.

Fix:
- Make one shared `CodeBlock` (shell-exported, for example `tutorial/CodeBlock.tsx`)
  that renders `figure.guide-codefig > figcaption (title, status, Copy {file},
  Download {file}, role=status)` + `pre.guide-code[tabindex=0][aria-labelledby]`.
  RiskGovernance's `SourceFrame` already has the right behaviour (static name, status
  region, Blob download) and should be the model. Replace the five local `Code`/
  `SourceFrame` variants with it.
- Until then, as a stopgap: restrict injection to `pre.guide-code` with no
  `[data-copy-owner]` ancestor, and never to a `figure`. Add
  `figure.guide-code{white-space:normal; padding:0; border:0; background:none}`.
- Copy must use the `text` prop, never `textContent` (`GuideApp.tsx:330`).
- GettingStarted's `CopyButton` (`:62-76`) keeps a fixed `aria-label="Copy …"` while
  the visible text changes to "Copied"/"Copy failed", and it never resets. Use the
  shell pattern instead.

### 3. Missing word spaces in prose (high)

In `initial-options-mobile.png`, "Optional intent fields gross_cap, fee_cap and
net_floor", "The instrument needs domain" and "id and domain" show the code chips
touching the preceding word. This is a source defect. When a JSX text line ends at a
word and the next line starts with `<code>`/`<DocLink>`, the newline collapses to
**no space**, so the text node reads "andnet_floor", "needsdomain", and
"derivatives.mori,examples README". Screen readers, copy-paste and search see the
words run together. A line-pattern scan found **about 48 sites in `RiskGovernance.tsx`**,
including `:411, :420, :422, :657, :669-672, :877-880` and the source-link lists at
`:525-530, :734-736, :972-975`. GettingStarted handles this by writing `<code> simulate</code>`
(9 sites, for example `:183-185, :199, :232, :248-249, :292, :309, :339`). That puts
the space **inside** the chip, so it renders as a padded blank at the start of the
code and gets copied into commands. Fix: end each such line with `{' '}` and remove
the leading spaces inside `<code>`. Check each scan hit by hand. A lint rule
(`react/jsx-child-element-spacing`) would catch new cases.

### 4. No `h1` (high)

All shots show "What Moriarty is for" as the visual title, but it is
`h2#overview-h` (`GuideApp.tsx:90`). The observation records `h1: 0`, because the
fallback `h1` in `tutorial.html:15` is replaced on mount. The sidebar's `h2.guide-navhead`
group labels (`:423`) come before `main` in the outline. Fix: render
`<h1 class="guide-title">Moriarty tutorials</h1>` (visible, or `sr-only` above the
CTAs) at the top of `main#guide-main`. Turn the nav group labels into `<p id>` +
`<ul aria-labelledby>`.

### 5. Imagery and text alternatives (high)

Visible in the desktop, tablet and dark shots: a woman writing in a ledger, four glass
asset boxes, a black router block, a document with a **checkmark seal**, a red arrow
into an **open arched door**, a landscape, and a signed paper with a red wax seal.
There is no scale. The alt at `GuideApp.tsx:107` still says "a scale and a signet".
The pipeline alt and caption (`:190-192`) still describe an "optional kernel" that is
not in the image (from my previous pixel inspection of that asset; the pipeline is not
in these six shots). The check and open gate read as approval, while the program
below it is PreparedUnqualified and Settle is Open.
Fix: make the hero decorative (`alt=""`, caption "Illustration. Nothing on this page
is signed, proved or settled.") and crop it to the left part
(`object-fit:cover; object-position:0 50%; aspect-ratio:21/9; max-height:16rem`).
Rewrite the pipeline alt from the pixels ("…Prove and Settle bracketed as target; a
reject-or-revise path returns to Propose"), and add that path to `ol.guide-flow`
(`:194-200`). Give Authorize an explicit label. Fix the contradiction in `:185`
("first three run locally") against `:201` ("Author and, locally, Propose").

### 6. Hero dominates the first screen; deep link lands wrong (high)

At 1440×1000 the hero starts at y≈585 and runs past the fold. Neither the transfer
program nor the "Your first program" heading is visible. At 768 px the hero starts at
y≈706, and at 390 px the first screen is entirely masthead, CTAs and prose.
`initial-transfer.png` shows a ~45-line JSON expectation in mid-scroll. The
`#transfer` heading is not visible, and the sidebar still marks **"What Moriarty is for"**
as current, with the On-this-page list showing Overview's h3s. A likely cause (an
inference, not measured): the `<img>` elements have no `width`/`height`
(`GuideApp.tsx:105, 188`), the hash scroll runs at mount (`:260-262`), and the
2.7 MB hero then loads and pushes everything down. Fix: add `width="1536"
height="1024"` (or a CSS `aspect-ratio`), `decoding="async"`, and `loading="lazy"`
on the pipeline. Serve AVIF/WebP at 768/1536 w with `srcset`. Re-run the hash scroll
after `document.fonts.ready` and the image load, or rely on native anchor scrolling
with `scroll-padding-top`. Root should confirm by re-shooting `#transfer` with the
image cached and uncached.

### 7. Masthead transparency and link contrast (high)

`.masthead` uses `color-mix(var(--paper) 88%, transparent)` plus blur
(`styles.css:114-115`). In the transfer shot, code text shows through behind the
masthead. In the options-mobile shot, body text shows through behind "Overview" and
"Kernel explanation". Site links use `.jump a{color:var(--ink-faint); font-size:.76rem}`
(`styles.css:631-633`). That is 3.74:1 on light paper, which fails AA for 12 px text.
Over bleed-through text the effective contrast drops further. Fix for the tutorial
page: `.guide-page .masthead{background:var(--paper)}` and
`.guide-jump a{color:var(--ink-muted); font-size:.85rem; padding:6px 2px}`
(7.28:1). Raise `--ink-faint` globally to `#6b6559` (light) / `#9a9486` (dark), as in
my initial review #4.

### 8. Tables (medium)

Three patterns ship. RiskGovernance's `ScrollTable` (`RiskGovernance.tsx:98-110`) is
the most accessible: a focusable `role=region` wrapper with a label, and a caption.
But it prints "Scroll sideways to read every column." unconditionally (visible in
the options-mobile shot) even when the table fits. GettingStarted (`:272`) and
LanguagePatterns (`:235`) tables have no caption. Every bare table uses
`.guide-main table{display:block}` (`tutorial/styles.css:104`), which can drop
table semantics. All tables inherit base `th` (0.72 rem, uppercase,
`--ink-faint` 3.74:1). `overflow-wrap:anywhere` on `td` (`:101`) breaks
identifiers mid-token. Fix: promote `ScrollTable` to the shell and use it everywhere,
with a caption required. Show the scroll hint only when
`scrollWidth > clientWidth`, or replace it with an edge shadow. Add
`.guide-main th{color:var(--ink-muted); font-size:.85rem; text-transform:none; letter-spacing:0}`.
Use `td{overflow-wrap:break-word}`.

### 9. Long JSON outputs (medium)

The transfer shot shows a full `mori.tests.json`-style expectation printed at
length: six effect objects, one property per line. Keep the bytes exact, but wrap
the full file in `<details><summary>Full transfer.test.json (N lines)</summary>`
and show the essential lines (status and effects) inline as a labelled excerpt.
Copy and download stay on the full file.

### 10. Dark mode (medium)

The dark shot shows the hero's warm-white field as the brightest area on a near-black
page. It pulls the eye away from the heading and CTAs. The light-filled primary CTA
also competes with it. Fix: in dark mode set
`.guide-figure img{filter:brightness(.85) contrast(.95)}`, using both
`prefers-color-scheme` and `[data-theme=dark]`, or hide the decorative hero in dark
mode. The eyebrow accent `#e8724a` reads well on dark.

### 11. Low

- "Skip to tutorial navigation" (`GuideApp.tsx:370`) targets `#guide-lessons`,
  which is `display:none` below 1280 px (`styles.css:90`). Below 1280 px, make it open
  the drawer and focus the search box.
- `.guide-status` (`tutorial/styles.css:65-68`) styles all four labels the same.
  Markets uses `<strong>` and RiskGovernance uses `<p class="guide-status">` (a
  block, so a chip border wraps a whole paragraph). Unify on one `Status` component
  with `data-status`. Add redundancy through line style (Open dashed, SpecifiedOnly
  dotted), never green or checks, and keep the exact words.
- The search input and copy buttons use `--rule-strong` borders, about 1.98:1, below
  the 3:1 non-text minimum. Use `--ink-muted` for interactive borders.

## What the shots show working

- No horizontal page overflow at 1440, 768 or 390. Prose measure is good (~72ch) and
  body text is comfortably large at all widths.
- The three-column desktop layout reads clearly. The active lesson has an accent bar
  and weight, not colour alone. Nav group labels use `--ink-muted` and pass.
- Tablet: a single column, with the On-this-page panel collapsed above the content
  and the Browse bar correctly below a one-row masthead.
- Mobile CTAs stack to full-width-ish targets of ≥44 px.
- Dark tokens apply consistently to chrome, text and borders.

## Limits

Each screenshot is a single viewport, so I saw no table, code block or status chip
at 390 px except what `initial-options-mobile.png` shows. I saw no focus state, no open
drawer and no pipeline image in these shots. Findings 2, 4, 8 and 11 rest on source
reading together with the JSON counts. The cause in finding 6 is an inference. Polish
work in isolated worktrees should be re-shot at the same viewports, plus 390 px views
of a code figure, an open drawer, a focused copy button and the pipeline.
