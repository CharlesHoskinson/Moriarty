# O3 initial design review: responsive accessibility and imagery

Date: 2026-10-01. Scope: initial independent design review, not implementation.
Domain: code reading widths, type scale and spacing, contrast and focus, tables,
copy and download controls, the GPT-generated hero and pipeline images with their
text alternatives, mobile layouts, and image/text overload.

## Identity and limits

- Requested seat: Claude Opus 5.5, medium effort (O3).
- Returned attestation: none available to this worker. The prior invocation ended
  with exit 143 and no provider receipt. This continuation runs as `claude-opus-5-5`
  by self-report only; that is **not** an attestation. Root should record the
  provider receipt for this run if one exists, or record the seat as unattested.
- Startup: read `AGENTS.md` and the installed `moriarty-dev:develop` skill, then ran
  only `cli.py --repo . status --json` from the beta worktree. It returned
  capability `SP01.6 loan-swap-subset`, next action `sp01-loan-report`, no pending
  transactions, and evidence gaps (stale SP01 binding input, missing current
  accounting, unavailable resource live state). None of that bears on this review.
- Read: `site/src/App.tsx`, `site/src/styles.css`, `site/src/components/*`
  (by grep for structure), `site/index.html`, `site/tutorial.html`,
  `site/vite.config.ts`, `docs/MORIARTY-PRODUCT-CONTRACT.md`,
  `wiki-llm/beta-language-2026-09-30/TUTORIAL-SITE-PLAN.md`, the research `PLAN.md`,
  `packages/moriarty-beta/examples/README.md`, and line widths of the example files.
- Also read the **in-progress** draft tutorial source (`site/src/tutorial/GuideApp.tsx`,
  `styles.css`, the top of `GettingStarted.tsx`) and viewed the two PNGs in
  `site/public/tutorial-assets/`. These were mid-authoring files on disk at review
  time. I have not built, rendered, browsed or tested them, so nothing here says the
  tutorial renders correctly or incorrectly. Findings about draft files are source
  observations for the authors and root, to be confirmed in the actual-build review.
- I did not read any other reviewer's output. I changed no source and ran no tests,
  imports, browsers, network, native, proof, wallet, credential, environment or git
  operations. Contrast ratios below are WCAG 2.x relative-luminance arithmetic on
  the CSS hex tokens; they are not browser measurements.
- No image here is accepted as a final asset. Image findings describe the files as
  they currently exist.

## Priority summary

| # | Severity | Finding | Fix owner |
|---|---|---|---|
| 1 | Blocker | Pipeline alt text and caption describe an "optional kernel" that the image does not contain | Root (assets) / S1 |
| 2 | Blocker | Hero shows a checkmark seal passing through a gate, which reads as acceptance; its alt invents a scale and signet | Root (assets) / S1 |
| 3 | High | Three incompatible copy-control patterns; the shell will inject a duplicate "Copy code" button onto S2's code figures | S1 + S2 |
| 4 | High | `--ink-faint` fails AA text contrast (3.46–3.94:1 light, 3.3–4.3:1 dark) and is the default `th` colour tutorial tables inherit | S1 |
| 5 | High | Home-page nav, including the new Tutorials link, is `display:none` below 1180px | S1 |
| 6 | High | The tutorial page has no `h1` after React mounts; nav group headings are the first `h2`s | S1 |
| 7 | High | Two sticky bars with a hard-coded offset; anchors and focus can land underneath them on mobile | S1 |
| 8 | High | Long code lines (113–233 chars) in a ~80-column box; no wrap option on mobile | S1 + S2/S3/G* |
| 9 | Medium | 4.4 MB of PNG with no dimensions, modern formats or lazy loading | Root |
| 10 | Medium | `table{display:block}` and `overflow-wrap:anywhere` will hurt the trust and support matrices | S1 + G3 |
| 11 | Medium | Every status chip looks the same; screen readers hear run-together PascalCase tokens | S1 + S2 |
| 12 | Medium | The hero sits between prose and the first program, pushing code below the fold | S1 |
| 13 | Medium | Architecture prose contradicts itself on which steps run locally; the text alternative omits the image's reject loop | S1 |
| 14 | Medium | Download links use `data:` URLs with no filename/size/digest provenance | S2 |
| 15 | Low | Control borders (`--rule-strong`) are 1.8–2.1:1, below the 3:1 non-text minimum | S1 |
| 16 | Low | `.jump` has `overflow:hidden`, which clips focus outlines; nav links are about 20px tall | S1 |
| 17 | Low | The "Skip to tutorial navigation" target is `display:none` on narrow screens | S1 |
| 18 | Low | Dark mode puts two bright paper-coloured images on a near-black page | S1 |
| 19 | Low | The masthead "Language reference" link points to the successor-syntax page, which the overview warns readers not to paste from | S1 |

## Findings and exact fixes

### 1. Pipeline image: alt and caption describe content the image lacks (blocker)

Source: `site/src/tutorial/GuideApp.tsx:188-193`. Alt: "Five cards in order: Author,
Authorize, Propose, Prove, Settle, with an optional kernel beside them." Caption:
"The optional kernel sits beside the path, not on it."

Observed `intention-pipeline.png` (1536×1024): a baked-in title "From intent to
effects", five numbered cards (Author, Authorize, Propose, Prove, Settle), a
"Reject or revise" loop returning into Propose, a "Target pipeline" bracket under
Prove and Settle only, a ledger with figures `+100 −40 +25 −10` on the Settle card,
and a decorative landscape across the bottom quarter. **No kernel appears.**

Fix:
- Replace the alt with an accurate one, e.g. *"Five numbered steps: Author, Authorize,
  Propose, Prove, Settle. Prove and Settle are bracketed as the target pipeline. A
  'reject or revise' path returns to Propose. The numbered list below gives the
  same steps with their support status."*
- Replace the caption with *"Conceptual flow. Prove and Settle are targets; the
  ledger figures are decorative, not results."* Explain the kernel in the
  existing "Where the kernel fits" callout only, or regenerate the image with a
  kernel element. Do not describe one that isn't drawn.
- Add the reject/revise path to the `ol.guide-flow` text, so the text alternative
  carries everything the image does. A Core rejection leaves no effects and no
  post-state; the lessons already teach that rule.
- The figures on the Settle ledger could be read as an actual result. Either crop
  them out or state in the caption that they are decorative.

### 2. Hero image: acceptance imagery and invented alt (blocker)

Source: `GuideApp.tsx:104-110`. Alt: "...beside ledger cards, a scale and a signet...".

Observed `intention-workbench.png` (1536×1024): a person writing in a ledger, four
glass boxes of asset tokens (cubes, spheres, pyramids, coins), a black router block
fanning tokens to tiles, a document **with a checkmark seal**, a red arrow into an
**open arched gate** onto a landscape, a signed paper with a red wax seal, books and
a token tray. I see no scale. The red seal on the signed paper is the closest thing
to a signet.

Problems:
- The checkmark, the arrow and the open gate read as "approved and released". The
  page's first program is `PreparedUnqualified` and Settle is `Open`. The main
  site's design rule (`styles.css:200`) is "Never a checkmark, never green." This
  image breaks that rule.
- The alt describes objects that are not in the image.

Fix, in order of preference:
- Use it as a decorative banner (`alt=""`, `role` unnecessary) with no figcaption
  claim, and crop to the left ~1000 px (person, ledger, asset boxes, router). That
  removes the checked document and the gate. CSS-only crop:
  `object-fit: cover; object-position: 0 50%; aspect-ratio: 21/9` (see #12).
- Or regenerate it without a check or an opened gate, and write the alt from the
  actual output. The text alternative must be checked against the final pixels,
  not the prompt.
- Keep the caption factual: *"Illustration. Nothing on this page is signed,
  proved or settled."*

### 3. Copy controls: three patterns, duplicate buttons, silent state (high)

Sources:
- Shell `CopyButton` (`GuideApp.tsx:58-84`): a `div.guide-copybar` followed by a
  sibling `<pre class="guide-code">`, with a `role="status"` message. This one is good.
- Shell auto-injection (`GuideApp.tsx:316-345`): for every `.guide-code` whose
  previous sibling is not `.guide-copybar`, it inserts a raw-DOM "Copy code" button.
- S2 `Code` (`GettingStarted.tsx:78-88`): renders `<figure className="guide-code">`
  with its own `CopyButton` inside the `figcaption`.

Consequences, from reading the source:
- S2's figure has class `guide-code`, and its previous sibling is not a copybar, so
  the shell injects a **second** copy button above every S2 code figure.
- `.guide-code` styles (`tutorial/styles.css:54-58`: `white-space: pre`, sunk
  background, border, padding, `overflow-x`) apply to the **figure**. The inner
  `<pre>` then also gets base `pre` styles (`styles.css:136-140`). The result is a
  box inside a box, with `white-space: pre` applied to the figcaption.
- S2's button has a fixed `aria-label="Copy …"` while its visible text changes to
  "Copied"/"Copy failed". Screen readers will not announce the change, and the
  accessible name no longer contains the visible label (WCAG 2.5.3). The state
  never resets.
- Raw DOM inserted into a React-owned subtree can break when a module re-renders.
- Every injected button is named "Copy code", so a screen-reader button list shows
  many identical controls.

Fix, with a single contract:
- Export one `CodeBlock` from the shell (or a shared `tutorial/code.tsx`) with props
  `{ label, filename?, text, download?: boolean }`. Render it as
  `<figure class="guide-codefig"><figcaption>` containing the label, a Copy button
  and an optional Download link, followed by `<pre class="guide-code" tabindex="0"
  aria-labelledby=…><code>`. Workers import it instead of defining their own.
- The button has a static visible name that includes the file ("Copy
  transfer.mori"). Report outcomes through a sibling `<span role="status">`, and
  reset after about 4 s.
- Delete the DOM-injection effect, or limit it to `pre.guide-code` that has no
  `data-copy` owner. Never target a `figure`.
- Copy exactly the `text` prop, not `textContent`, so a visual wrap toggle (#8) or
  syntax highlighting cannot change the copied bytes.

### 4. Text contrast: `--ink-faint` fails AA (high)

Computed ratios for `--ink-faint` text:

| Theme | Token | paper | paper-sunk | paper-raised | accent-soft | mark-soft |
|---|---|---|---|---|---|---|
| Light | `#86806f` | 3.74 | 3.46 | 3.94 | 3.24 | 3.10 |
| Dark | `#7c7668` | 4.11 | 4.30 | 3.85 | 3.35 | 3.30 |

It is used for small text: `th` at 0.72 rem uppercase (`styles.css:132`), the
tagline, `.reg-chip` at 0.62 rem, `.fork-hint`, `.rung-state` at 0.7 rem,
`.target-shared` at 0.62 rem, and others. The tutorial stylesheet uses `--ink-muted`
(7.28:1) in its own rules, but tutorial **tables inherit the base `th` rule**, so
the G3 trust matrix and the support matrix would get failing headers.

Fix (the light value reuses the existing `--mark`; the dark value is a lighter
step of the same warm grey):
- Light `--ink-faint: #6b6559` gives 5.49 / 5.08 / 5.78 / 4.76 / 4.56.
- Dark `--ink-faint: #9a9486` gives 6.16 / 6.44 / 5.75 / 5.02 / 4.93.
- In tutorial tables, also set
  `.guide-main th { color: var(--ink-muted); font-size: 0.8rem; text-transform: none; letter-spacing: 0; }`.
  Uppercase 0.72 rem headers hurt legibility in dense matrices whatever the contrast.
- Raise the 0.62–0.66 rem labels to at least 0.75 rem (12 px).

### 5. The home-page Tutorials link disappears below 1180 px (high)

`styles.css:637` sets `@media (max-width:1180px){ .jump{display:none} }`. The
Tutorials, Kernel and Docs links added in `App.tsx:36-38` live inside `.jump`, so on
tablets and phones the home page has **no route to the tutorial** except typing the
URL.

Fix: move the three cross-page links out of `.jump` into their own
`<nav aria-label="Site">`, and keep them visible at every width (wrap below 640 px).
Only the in-page section anchors should collapse. Alternatively, add a "Tutorials"
call to action in `Opening`.

### 6. No `h1` on the tutorial page (high)

The fallback `<h1>` in `tutorial.html` sits inside `#root`, and `createRoot().render`
replaces it. `GuideApp` renders no `h1`; its first headings are the nav group
`h2.guide-navhead` elements ("Start building"…) in the sidebar, which comes before
`main` in DOM order. Fix: add `<h1 class="guide-title">Moriarty tutorials</h1>` at the
top of `main#guide-main`. Change the nav group headings to non-heading text with
`aria-labelledby` on each `ul`, or keep them as `h2` but put them after `main` in
the heading outline. The first is simpler.

### 7. Sticky stacking and anchor offsets on mobile (high)

- `.guide-bar` is `position:sticky; top:3.4rem` (`tutorial/styles.css:85`). Below
  640 px, `.guide-jump` gets `flex-wrap: wrap` (line 8). With four links plus the
  wordmark, the masthead becomes two rows (about 4.5–5 rem) and the Browse bar
  slides under it.
- `scroll-margin-top: 5rem` (lines 36, 39) is less than masthead plus bar (about
  6.5–8 rem on narrow screens). Hash navigation and in-page TOC links can put the
  heading under the bars, and the same goes for focused elements (WCAG 2.4.11).

Fix:
- Set the masthead height as a custom property (`--mast-h`) and use
  `top: var(--mast-h)` for `.guide-bar`.
- Use `scroll-margin-top: calc(var(--mast-h) + var(--bar-h) + 0.75rem)` and
  `scroll-padding-top` on `html` with the same value.
- Below 640 px, stop wrapping the site nav. Shorten the labels ("Overview ·
  Tutorials · Kernel · Reference") on one horizontally scrollable row with
  `overflow-x:auto`, or move the site links into the Browse panel.
- Alternatively, make only one bar sticky on narrow screens. Put Browse inside
  the masthead row and drop the second sticky bar.

### 8. Code reading width (high)

Measured longest lines: `transfer.mori` 113 chars, `repay.mori` 136 chars,
`packages/moriarty-beta/examples/*.mori` up to 233 chars. The shell's
`TRANSFER_SOURCE` has 96–113-char lines. The reading column is `minmax(0, 46rem)`
and `.guide-code` uses 0.9 rem mono with 16 px padding, which gives about 80 visible
columns on desktop and about 38 on a 360 px phone. Every first-screen program
therefore scrolls horizontally, and on a phone the signed fields
(`gross_cap`, `fee_cap`, `net_floor`) sit off-screen. The plan says signed terms must
remain visible.

These sources must stay byte-exact, because they are what `mori init` writes and
what users download. Fix the presentation:
- Add a per-block "Wrap lines" toggle (`aria-pressed`). It switches the `pre` to
  `white-space: pre-wrap; overflow-wrap: anywhere; padding-left: 2ch;
  text-indent: -2ch` for hanging continuation. Default it on below 640 px and off
  on desktop. Copy and download still use the exact `text` prop.
- Add a horizontal-scroll affordance when not wrapped: an edge shadow via
  `background-attachment: local` gradients. A focusable scroll region (already
  `tabIndex=0`, good) also needs a visible `:focus-visible` ring; one exists at
  `tutorial/styles.css:103`.
- On ≥1280 px, widen the main column to `minmax(0, 52rem)` and keep prose capped
  at 68ch. Code then gets about 90 columns without changing the prose measure.
- Lower code size on narrow screens only to 0.82 rem (about 13 px). Do not go below that.
- For SpecifiedOnly family files (up to 233 chars), show a short excerpt inline,
  with the full file behind download or `<details>`. Excerpts must be labelled as
  excerpts, and copy must say which it copies.

### 9. Image weight and loading (medium)

`intention-workbench.png` is 2,760,290 bytes and `intention-pipeline.png` is
1,684,811 bytes, both 1536×1024 8-bit RGB PNG. Neither `<img>` has `width`/`height`
(layout shift), `loading` or `decoding`. On mobile, about 4.4 MB of illustration
loads before the first program is useful.

Fix (root produces derived files from the same sources; no new imagery):
- Encode AVIF and WebP at 768 w and 1536 w (painterly sources typically land at
  about 80–250 KB each; verify actual sizes) and keep PNG only as fallback.
  Use `<picture>` with `srcset` and `sizes="(min-width:1280px) 46rem, 100vw"`.
- Add `width="1536" height="1024"`. Use `loading="lazy" decoding="async"` on the
  pipeline image; on the hero, use `decoding="async"` without lazy.
- Add a byte budget to the build check (e.g. ≤300 KB per illustration variant).
  Record source, tool and generation date for each image so the asset's provenance
  can be reviewed.

### 10. Tables and matrices (medium)

- `.guide-main table { display:block; overflow-x:auto }` (`tutorial/styles.css:104`).
  Changing a table's display can remove table semantics in some screen-reader and
  browser pairs (notably Safari/VoiceOver). It also needs `tabindex` to be scrolled
  by keyboard, and the `:focus-visible` rule at line 103 assumes focusability.
  Fix: wrap with `<div class="guide-table" role="region" aria-labelledby="{caption-id}"
  tabindex="0">` holding the overflow, and leave `table` as `display:table`. Give
  every table a `<caption>`.
- `overflow-wrap:anywhere` on `td` (line 101) shrinks min-content widths. Columns
  collapse and identifiers like `PreparedUnqualified` or `BETA_PROFILE_UNSUPPORTED`
  break mid-token with no hyphen. Fix: `td { overflow-wrap: break-word; }`, with
  `anywhere` only on `td code.hash`.
- Wide matrices (G3 trust matrix, eight-area support matrix): make the first column
  `position: sticky; left: 0; background: var(--paper)`, and use `th scope="row"`.
  Below 640 px, render each row as a definition list card ("Area → Status →
  What runs → What is open"). Do not shrink the font below 0.8 rem to fit.
- Status cells in tables must contain the status word, not a colour or icon alone.

### 11. Status labels (medium)

`.guide-status` (`tutorial/styles.css:65-68`) gives LocalS0, PreparedUnqualified,
SpecifiedOnly and Open identical styling. The words carry the meaning, which is
correct, but readers scanning matrices get no help. S2 already emits `data-status`.

Fix (status remains in words; shape and line style add redundancy):
- `[data-status="Open"] { border-style: dashed; background: transparent; }`
- `[data-status="SpecifiedOnly"] { border-style: dotted; background: transparent; }`
- `[data-status="PreparedUnqualified"] { background: var(--paper-raised); }`
- `[data-status="LocalS0"]` keeps the current `--mark-soft` fill.
- No green, no checkmarks. The accent is reserved for divergence or refusal.
- Make the shell `Status` component emit `data-status` too.
- For screen readers, prefix a group of labels with visually hidden text:
  `<span class="sr-only">Support status: </span>`. Optionally set
  `title`/`aria-describedby` to the legend entry. Where several statuses sit
  inline before a sentence (`GuideApp.tsx:114`), put them in a separate line
  (`<p class="guide-statusline">`) so the sentence does not begin with
  "LocalS0 PreparedUnqualified This is…".

### 12. Hero placement and image/text overload (medium)

The Overview section currently runs: eyebrow, h2, two paragraphs, the full-size
hero (at 46 rem that is about 736×490 px), the 18-line program, two callouts, a
legend and a sources paragraph. On a 1280×800 screen the first program starts
below the fold. On a phone the hero is about 240 px of decoration between the
introduction and the code. The plan says to show a small real program and a
readable result first.

Fix:
- Move the hero above the h2 as a banner. Constrain it with
  `aspect-ratio: 21/9; max-height: 16rem; object-fit: cover; object-position: 0 50%`,
  and on screens up to 640 px use `max-height: 9rem` or hide it
  (`display:none`, since it is decorative per #2).
- Allow at most one illustration per lesson page and none inside DeFi example
  sections. Use real code, tables and status lines there.
- Keep the pipeline image at most 46 rem wide. Below 640 px, hide it, because its
  baked-in labels shrink to about 5 px and are unreadable, and show only the
  `ol.guide-flow`, which is the real text. A CSS `display:none` is enough.
  Hiding is acceptable because the list carries all the content (after #1/#13 fixes).
- The baked title "From intent to effects" duplicates the h2 and cannot follow
  dark mode, zoom or translation. Prefer a crop that removes it (top ~22 %) and
  the bottom landscape (~25 %), leaving the five cards plus the bracket. Crop in
  the derived asset rather than with CSS so the alt matches what is shown.

### 13. Architecture text alternative accuracy (medium)

`GuideApp.tsx:184-201`: the intro says "The first three run locally in the beta
tooling". The list gives only Author and Propose status labels, says Authorize's
key authority is unverified, and the closing line says "Today's tooling covers
Author and, locally, Propose." These contradict each other. Fix: *"Author and
Propose run locally. Authorize can verify a signature over exact source bytes, but
key authority stays unverified. Prove and Settle are targets."* Give Authorize an
explicit status label (use the label the package's SIGNED-INTENT.md supports;
`Open` for key authority if nothing stronger is established). Map each step to the
real CLI verbs (`mori check`, `mori test`, `mori simulate`) so readers can see where
the tool stops. The image has no `check` or `test` step, so the text must supply them.

### 14. Downloads (medium)

`GettingStarted.tsx:59-61,84`: the download is
`data:text/plain;charset=utf-8,…` with `download={file}`. This works in desktop
browsers, but iOS Safari has historically opened data URLs in a tab instead of
saving them. The link also says nothing about which release the bytes come from.

Fix:
- Use `URL.createObjectURL(new Blob([text], {type:'text/plain'}))` created on
  click, or, better, have Vite emit the lesson files as real URLs (`?url`) alongside
  `?raw`, so download and display share one source file.
- Next to each download, show filename, byte size and the first 12 hex of SHA-256,
  computed at build time, plus the release/profile (`moriarty-beta 0.1.0-beta.1`,
  `moriarty-beta/1`). This follows the plan's rule that each displayed output states
  its producing release.
- For generated variants (`wrongExpectation`, `underfundedScenario`,
  `rejectionExpectation`), label the download "modified copy for this exercise",
  so the bytes are not mistaken for a shipped fixture.
- Give links the same visible-name pattern as copy buttons ("Download
  transfer.mori").

### 15. Non-text contrast of controls (low)

`--rule-strong` borders on the search input (`tutorial/styles.css:27`), copy buttons
(line 61) and mode switch measure 1.98:1 on paper (dark 1.85–1.98). WCAG 1.4.11
requires 3:1 for control boundaries. Fix: use `border-color: var(--ink-muted)` for
interactive controls only (7.28:1 light, 7.41:1 dark). Leave decorative rules as
they are.

### 16. Focus clipping and target size in the masthead (low)

`.jump` has `overflow:hidden` (`styles.css:633`), and its links have
`padding: 2px 0`. The global focus ring (`outline 2px, offset 2px`) extends 4 px
beyond each link, so the container clips it top and bottom. Links measure about
0.76 rem text plus 4 px, roughly 20 px tall. Fix: `.jump a { padding: 6px 2px; }`
and `.jump { overflow: visible; }` (or `overflow-x:auto; padding-block:4px` if
scrolling is wanted). The tutorial override sets `overflow: visible` only below
640 px.

### 17. Skip-to-navigation on narrow screens (low)

The "Skip to tutorial navigation" link (`GuideApp.tsx:370`) targets
`#guide-lessons`, which is `display:none` below 1280 px until opened (styles
lines 90-91), so focus goes nowhere. Fix: below 1280 px, point it at the Browse
button, or have its click open the menu and then focus the search input.

### 18. Dark mode imagery (low)

Both PNGs have warm-white backgrounds, and `color-scheme: light dark` turns the
page near-black. Fix:
`@media (prefers-color-scheme: dark){ :root:not([data-theme="light"]) .guide-figure img { filter: brightness(.88) contrast(.95); border-color: var(--rule-strong); } }`.
Mirror it for `[data-theme="dark"]`. Do not invert.

### 19. Navigation label for the successor-syntax reference (low)

`GuideApp.tsx:380` puts "Language reference" (→ `docs/language.html`) in the main
masthead. The overview (`GuideApp.tsx:172-173`) says that page is the older
successor-syntax profile and its samples should not be pasted into a beta project.
A top-level link labelled as *the* language reference invites exactly that mistake.
Fix: label it "Successor syntax (not beta)", or move it into the "Design and
limits" group with that qualifier. Point "Language reference" at the beta surface
reference that S3 is writing.

## Type scale and spacing notes (no defects, small adjustments)

- Tutorial body is 1.05 rem/1.7, h3 1.2 rem, h2 1.75 rem (1.45 rem ≤640 px). The
  h3-to-body step (1.14×) is weak in long lessons. Use h3 1.3 rem with
  `margin-top: var(--s7)` so each lesson subsection reads as a break.
- `.guide-main p { max-width: 72ch }` sits at the top of the comfortable range.
  Use 68ch; lists and callouts should follow the same cap.
- `.guide-grid` uses `minmax(15rem,1fr)` without `min(100%, …)`. At 320 px
  viewports with 16 px padding, 15 rem (240 px) fits, but zoomed text (200%)
  overflows. Use `minmax(min(100%,15rem),1fr)`, as the main stylesheet already does.
- `.guide-callout` used for both "Illustrative." notes (S2) and conceptual
  callouts gives the same accent-soft treatment to a warning about illustrative
  content and to neutral asides. Use a distinct `.guide-note-illustrative` with a
  dashed rule and no accent fill, keeping the accent for refusal/divergence, as the
  design system intends.
- 200% zoom and 320 CSS px reflow (WCAG 1.4.10) must be checked in the build
  review. Code and tables may scroll horizontally; prose, nav and controls may not.

## What the actual-build review should check

Supply these and I will verify each:
1. Screenshots at 360×800, 768×1024, 1280×800 and 1440×900, in light and dark,
   of: tutorial top, first program, a SpecifiedOnly example, the trust/support
   matrix, and the architecture figure.
2. Keyboard-only pass: skip links, Browse open/close/Escape, search, the TOC
   details, copy (with announced status), download, wrap toggle, table region
   scrolling, and focus never hidden under sticky bars.
3. Copy result equals source bytes for one lesson file (diff), and download bytes
   plus digest match the repository file.
4. Final image files (format, dimensions, bytes) and their alt text, so the alt
   can be checked against the pixels.
5. Home page at 768 px, where Tutorials must be reachable.
6. Axe or Lighthouse accessibility output, if the root runs it, as supporting
   evidence only. It does not replace the manual checks above.

No finding above concerns financial or proof acceptance. Nothing on this site
establishes signing, proving, settlement, or Preview financial acceptance. Every
fix keeps the beta's local, specified and open labels where they are.
