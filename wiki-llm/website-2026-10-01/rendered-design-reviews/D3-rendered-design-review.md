# D3 rendered design review

Reviewer: requested GPT-6.1 Sol, medium effort. Date: 2026-10-01. Reviewed the initial assembled baseline, before polish, in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.

## Actual coverage

Viewed all six supplied PNGs with view_image: initial-desktop, initial-mobile, initial-tablet, initial-dark, initial-transfer and initial-options-mobile. Read INITIAL-BROWSER-OBSERVATION.json and inspected the newly integrated RiskGovernance source, including shared helpers and complete options section plus later section/helper locations. Other tutorial components/styles were inspected in the preceding integrated-source pass. RiskGovernance is now present; its absence limitation in that earlier report is superseded for this baseline.

Root reports `npm run verify` passed 51 data tests, strict TypeScript and Vite build. The supplied browser JSON records all 17 sections, no page errors, zero page overflow at 1440/390/768px, zero h1 elements and 55 injected copy controls. These are root-captured observations; this reviewer did not run browser, build or tests. Still screenshots do not demonstrate keyboard behavior, clipboard success/failure, search behavior, scrolling-region access or reduced-motion behavior. No publication, native or proof result is assessed.

## Overall visual result

Desktop spacing and reading width are good. The three-column structure separates lessons, prose and local contents without crowding the article. Body text is legible and the rust/ivory illustration fits the existing visual system. Dark mode keeps readable article text and clear primary actions; the opaque ivory illustration works as an intentional light art panel. Tablet is contained and readable. The supplied screenshot set and JSON do not show a whole-page overflow defect at the tested widths.

The main visible defects concern sticky navigation and reading position. The large introductory image and prose also delay the first useful program; the code presentation needs further contained-region inspection after its identified source fixes.

## Prioritized fixes

### P1 — Fix the mobile sticky stack

The 390px opening screenshot shows a roughly 122px masthead: wordmark on the left, site links wrapping into three lines on the right. The Browse lessons bar appears immediately below it at initial scroll. In the options-mobile screenshot, the masthead persists but Browse lessons is absent; content begins underneath the masthead mid-paragraph. This is consistent with the source's `.guide-bar {top:3.4rem;z-index:9}` sitting behind a taller `.masthead {z-index:10}`. The combined sticky stack exceeds the 5rem offsets used by tutorial sections/headings, including RiskGovernance's inline styles.

Use an explicit mobile header layout with bounded height and place Browse below that exact height, or make Browse nonsticky. Set one combined anchor-clearance value for section/h2/h3 navigation; replace RiskGovernance inline 5rem offsets as well, because inline values override a shared repair. Check actual scrolled narrow screens and zoomed layout after repair. Preserve access to Browse during reading.

### P1 — Correct the active section and local contents

In initial-transfer.png the central article displays the complete transfer expectation fixture, but the sidebar still marks “What Moriarty is for” and the right rail lists overview subsections. This is an observed mismatch in that captured state. The current IntersectionObserver stores intersection flags for large sections and updates active to the first intersecting section; very tall sections/deep-link scrolls can leave stale position and contents.

Track visible section headings or compute the nearest preceding heading on scroll, and update active explicitly on lesson/hash navigation. Recompute the right-rail h3 list for the destination. A deep link should identify the shown lesson immediately, independently of whether its huge containing section produces a fresh intersection transition. Verify transfer and options destinations after repaired navigation.

### P1 — Add a real page h1

The JSON reports h1 count zero, matching the source's overview h2. Add one visible tutorial page title, for example “Moriarty tutorials”, or promote the overview title to h1 while preserving a meaningful lesson navigation title. Keep subsequent lesson headings h2 and subordinate topics h3. This supplies the missing document-level hierarchy without creating a second oversized landing-page manifesto.

### P1 — Repair code captions and duplicate controls

The assembled JSON reports 55 injected controls. Source inspection identifies duplicate injection before authored code figures in GettingStarted/CrossChain and RiskGovernance SourceFrame: those figures already have a copy toolbar, but the inner `.guide-code` pre is not immediately preceded by `.guide-copybar`. RiskGovernance is otherwise a useful model: named focused pre, wrapping 44px controls, try/catch clipboard handling and `role="status"` feedback.

Use shared declarative code/copy components or explicit authored-copy markers rather than inferring from immediate siblings. Keep captions/status/download controls outside source overflow. Reset `white-space:normal` and sans type on figure wrappers, then apply preserved whitespace/monospace only to pre/code. The transfer screenshot confirms long JSON is visually dominant; keep complete fixtures available in details/reference while leading with readable output and independently derived amounts. Do not remove complete expectations.

### P1 — Fix residual inherited faint text contrast

Inactive header links are visibly faint in desktop/dark captures. Tutorial table th still inherit the shared faint color. Token calculations from the initial review are 3.74:1 light and 4.11:1 dark, below normal-text 4.5:1. Scope tutorial nav and table headers to ink-muted or ink. Do not lighten explanatory Open/SpecifiedOnly text; current body support explanations mostly use appropriate ink.

### P2 — Keep keyboard destination and local scrolling access consistent

The source-only navigation/copy issues remain untested by these images: mobile skip-to-navigation targets a hidden aside; closing a menu after selecting a lesson can hide the focused link; Markets/LanguagePatterns pre lack explicit keyboard focus. RiskGovernance's ScrollTable already supplies a labeled focusable region and sideways-reading hint, but its table also receives the global `display:block;overflow-x:auto` rule, creating nested overflow surfaces. Prefer a single named wrapper controlling scroll with a normal table inside. Avoid `white-space:nowrap` on long explanatory cells; preserve nowrap for identifiers/amounts and allow prose relationship columns to wrap to a sensible width. This reduces scrolling distance without concealing columns.

## Images and first-screen hierarchy

The desktop hero starts near 584px and fills the rest of a 1000px screenshot. Tablet similarly shows only the start of the hero after substantial prose; mobile has not reached it by the screenshot bottom. Put a concise purpose/support statement and first runnable action above the hero, or move the illustration after the small transfer example. The current opening mobile screen spends about 180px on sticky UI and another 200px on contents/buttons/labels before explanatory prose begins. Reducing the header to two deliberate rows and removing excess pre-title spacing would improve entry without shrinking body text.

The actual asset review remains applicable: workbench alt mentions a nonexistent scale; pipeline alt/caption mention a nonexistent optional kernel. Correct them to the viewed pixels. The pipeline contains Author/Authorize/Propose/Prove/Settle, a target bracket under Prove/Settle and a Reject or revise return path. Preserve those meanings in the nearby HTML list/caption. Add explicit image width/height attributes to reserve space. The screenshot set does not include rendered pipeline scale, captions or figure overflow, so those remain source/asset findings awaiting a later view.

## Disposition

The assembled baseline has coherent desktop/tablet/dark presentation and no reported page overflow at the three captured widths. Required repairs remain for the observed mobile sticky/navigation-position defects, missing page h1, source-demonstrated copy/caption behavior, inherited contrast and accurate image descriptions. Actual keyboard/clipboard/search/reduced-motion success is not established by this review.
