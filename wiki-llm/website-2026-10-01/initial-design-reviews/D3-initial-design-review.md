# D3 initial design review — responsiveness, accessibility, visual coherence

Reviewer: GPT-6.1 Sol, requested medium effort; independent D3 source review, 2026-10-01. This review covers inspected existing source and the tutorial plans. It does not approve the future tutorial build, generated images, browser appearance, financial execution, proof acceptance, or deployment.

## Scope and evidence

Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.

Read `AGENTS.md`, the installed Moriarty develop skill, guarded `status --json`, `site/src/styles.css`, `site/docs/styles.css`, `site/src/kernel/styles.css`, `site/src/App.tsx`, and components Opening, Language, Categories, Guarantees, Register and Roadmap. The larger first combined source read was output-truncated; focused subsequent reads covered App/Opening/Language, the complete docs stylesheet and the shared stylesheet's lower half. Read the repository tutorial site plan and the external implementation PLAN.md. No source edits, tests, browser, native operations, credentials or network acquisition were performed. Graphify guidance was read; no graph construction was run because this assignment authorizes only source inspection and this external review artifact.

Guarded status reports SP01.6 loan-swap-subset with unresolved operational history, stale binding/candidate inputs, missing accounting and unavailable resource state. There are no pending transactions. These conditions do not block this read-only design review and do not support settlement or proof claims.

## Assessment

The existing warm paper, dark ink, rust accent and measured rule system is a suitable foundation. Global 16px body type, 1.6 line height, 34rem prose measure, clear visible-focus styling, skip link and reduced-motion CSS are useful existing source features. Retain them in the tutorial. The separate legacy docs stylesheet uses 18px Georgia prose and green links; importing it into the new entry would create a second visual system. Use the shared React tokens with tutorial-specific classes instead.

The existing landing page is an explanatory spread, not an adequate documentation shell by itself. Its section navigation is `display:none` below 1180px, with no replacement in App. Dense tables and tiny labels are purposeful in the landing page's register but should not become the tutorial's default reading interface.

## Required fixes for the tutorial candidate

1. **Make all meaningful support and caveat text pass normal-text contrast.** Shared `--ink-faint` is `#86806f` on `#faf9f6` (3.74:1), or 3.94:1 on white. The dark equivalent `#7c7668` on `#14130f` is 4.11:1. These are deterministic calculations from the inspected tokens, not browser measurements, and fall below 4.5:1 for small normal text. Existing headers, chips, caveats and closing text often use these tokens at 0.62–0.82rem. Use `--ink-muted` for tutorial status labels, premises, release scopes, captions and navigation; reserve faint ink for nonessential decoration. The existing muted and rust colors on paper calculate to 7.28:1 and 6.94:1. Avoid reducing opacity for Open or SpecifiedOnly content.

2. **Provide navigation at every width.** Build the desktop shell as `14rem minmax(0, 1fr) 11rem`, with a bounded article around 65–72ch and optional wider code/table content. At approximately 1200px drop the right rail; below approximately 900px replace the left sticky rail with a native `details` menu or a button-controlled mobile menu that exposes `aria-expanded` and `aria-controls`. Keep search and repository/version links available. Do not inherit the existing `.jump` breakpoint that removes all section links. Tune thresholds after the actual content is assembled rather than treating these suggested numbers as verified.

3. **Contain long code and tables without horizontal page overflow.** Every grid/flex article child needs `min-width:0`; the central grid column needs `minmax(0,1fr)`. Put each `pre` and wide table in a local overflow container, capped at `max-inline-size:100%`. Preserve code whitespace and allow its own horizontal scrolling; use `overflow-wrap:anywhere` for prose URLs, inline hashes and inline identifiers. Do not break executable source tokens to manufacture a narrow screenshot. Existing `.src-body` already uses `min-width:0`; extend that discipline to the new shell and worker modules.

4. **Make scrolling code/table regions usable with the keyboard.** Add a programmatic label and keyboard focus to overflow regions where needed, preferably `tabIndex={0}` and an accessible name identifying the example/table. Ensure the visible focus ring is not clipped by `overflow:hidden` wrappers. Give tables captions and scoped row/column headers, plus a visible narrow-screen hint when the table exceeds its viewport. Preserve the support/residual column while scrolling; do not hide it on phones. For compact two-column comparison lists, use semantic stacked descriptions rather than shrinking text.

5. **Keep copy/download controls outside the scrolled source.** Use a wrapped toolbar containing a human example name, support label, Copy and Download. Aim for 40–44px controls with sufficient spacing; 24px is the minimum target-size reference, not a desirable tutorial toolbar. Announce copy success/failure through a short `role="status"` region, retain focus on the button, and never report Copied until clipboard success is observed. A failed clipboard write should leave selectable source and download available. Give repeated copy buttons unique accessible names such as “Copy transfer source”.

6. **Account for sticky UI and motion.** Apply `scroll-margin-top` to tutorial headings and sections, based on the actual sticky header height. The existing `main > section` 64px rule does not cover headings nested inside a tutorial article. Provide “Skip to tutorial content” and optionally “Skip to tutorial navigation”. Retain the shared reduced-motion override. If the new entry does not import shared CSS, reproduce the focus, skip and reduced-motion behavior explicitly. Avoid animated pipeline arrows or active-section movement that bypasses that preference.

7. **Keep support scope beside the action.** Every runnable command/example should show plain words such as LocalS0 or PreparedUnqualified, with a short explanatory sentence. Every specified DeFi pattern should show SpecifiedOnly before code and say unsupported expansion/simulation publishes null effects. Place the four external premises and four unverified bindings where readers encounter preparation results; a distant trust appendix is insufficient. Labels need both text and style, with no checkmark or green treatment suggesting accepted proof or settlement.

## Typography and visual hierarchy recommendations

Use sans prose at 16–18px and line height around 1.6, with clear paragraph spacing; keep monospace for syntax, identifiers and bounded status labels. Do not propagate the landing register's 0.62rem chips or 0.74rem table body into lessons. Code should generally remain at least 14px with a readable 1.5–1.65 line height. Each lesson should lead with its learning goal and support scope, then the shortest complete runnable example, independently derived expectation, actual readable output and interpretation. Collapse dense origin maps/JSON under an explicit `details` summary; keep signed terms and financial defaults discoverable through complete source/download links.

Keep the page title smaller than the landing manifesto and bound it to about 18–22ch. Use one strong hero image with nearby text; don't let a decorative image push the first useful example below an oversized introduction. An ivory/rust raster may be light-theme specific, so frame it with shared rules and test its edges against both themes rather than assuming it inherits dark tokens.

## Pending image and actual-build review

The five-card pipeline must read Author → Authorize → Propose → Prove → Settle, with a nearby target label and an HTML ordered-list text equivalent. A raster alone cannot supply usable mobile step descriptions or accessible reading order. Show it full-width on larger screens and preserve the complete image with a stacked HTML equivalent on phones; avoid a cropped strip that loses card labels. The nearby explanation must distinguish the architectural target from current local preparation and avoid visual ticks implying completed proof acceptance. Use concise useful alternative text or empty alt when the adjacent text completely conveys the diagram; never duplicate a long description twice.

No generated hero/pipeline bytes or assembled tutorial DOM/screenshots have been inspected. The follow-up review must inspect actual desktop/mobile views, 200% zoom/reflow, keyboard navigation and focus visibility, search empty/results behavior, copy success/failure, code/table containment, dark mode, reduced motion, image text equivalents and support-label placement. These are specified inspection requirements, not performed results.

Disposition: suitable visual foundation with the contrast and responsive-navigation issues above to resolve in the assembled tutorial. No approval of the uninspected candidate is given.
