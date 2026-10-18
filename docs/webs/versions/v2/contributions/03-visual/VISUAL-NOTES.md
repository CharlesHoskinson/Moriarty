# Visual contribution 03

Status: implemented local visual draft, ready for integration. This file records presentation choices and observed stylesheet checks. It is not product, security, publication or final-page acceptance.

## Design

The page is an editorial field guide: warm paper, dark ink, forest green and one restrained copper accent. Large serif display headings and generous margins set a reading rhythm. Fine rules group information without a wall of rounded cards. The dark green arithmetic worksheet marks a deliberate change of reading task. Body text uses the platform sans-serif stack; no fonts, images or scripts are fetched.

Two possible treatments were considered within the supplied approved direction: a dark technical dashboard and a paper field guide. The field guide was selected because the page explains vocabulary and financial distinctions through a sustained narrative. The explorer retains the precision of a technical workbench through tabular figures, quiet dotted plotting paper and explicit inspector labels.

Palette: paper `#f5f1e8`, ink `#232d29`, muted text `#5b655e`, forest `#294b3e`, copper `#925e3c`. Copper was darkened during checking from the initial `#98633e` to improve its contrast against paper. Contrast calculations are scoped color-pair measurements, not a conformance finding.

## Hero API and meaning

`import HeroWeb from '../contributions/03-visual/HeroWeb.jsx'` (adjust the relative path from the consumer).

`<HeroWeb className="optional-additional-class" />`

The component imports React and `useId`, returns a responsive inline SVG with `viewBox="0 0 540 540"`, and is decorative (`aria-hidden="true"`, `focusable="false"`). It has no state, event handlers or network behavior. Per-instance IDs prevent pattern and gradient collisions. The SVG paths and knot coordinates remain editable source. The engraving suggests interwoven connections; its knot count, geometry and colors do not specify a protocol, proof count, supported producer or verification state. Put explanatory wording in adjacent ordinary HTML.

The hero itself does not animate. `.hero-web__construction`, `__filaments`, `__weave`, `__knots` and `__marginalia` provide styling hooks. Host the component inside `.hero-art` beside `.hero-copy` within `.hero`. Optional `.hero-art-caption` can identify it as a visual study rather than a topology.

## CSS integration contract

Import `design.css` once before the integrator's small application-specific overrides. It includes baseline element styles and variables, so no CSS framework reset is required. Use one `.page-shell`, `.shell` or `.container` wrapper; nesting these aliases would apply the width rule more than once.

- Header: `.site-header`, `.brand`, `.brand-name` or `.brand-wordmark`, `.brand-mark`, `.site-nav`, `.draft-label`.
- Hero: `.hero`, `.hero-copy`, `.hero-art`, `.hero-lede` or `.hero-description`, `.hero-actions`, `.hero-note`, `.hero-art-caption`.
- Repeated narrative: `.section`, `.section-heading`, `.eyebrow`, `.section-number`, `.section-intro`, `.definition-strip`, `.scope-note`.
- Actions: `.button` or `.primary-button`, `.secondary-button`, `.text-link`. These classes work on an anchor or button; use the semantic element appropriate to the action.
- Scenario selectors: `.case-tabs` with buttons. Styling follows `aria-selected="true"` or `aria-pressed="true"`; the integrator chooses the correct interaction semantics. `.case-index` is optional.
- Explorer: `.explorer` > `.explorer-layout` > `.diagram-panel` and `.inspector`. Equivalent `.web-layout` and `.web-stage` aliases exist. `.diagram-header`, `.diagram-legend`, `.strand-list`, `.strand-button`, `.strand-number`, `.assembly-status` and `.source-note` support the interior. Selection follows `.strand-button[aria-pressed="true"]` or `.is-selected`; omission uses `.is-omitted`; incompleteness uses `.assembly-status.is-incomplete`. Status text must carry the distinction independently of color.
- Worksheet: `.worksheet` > `.worksheet-layout`, `.worksheet-controls`, `.range-heading`, `.range-limits`, `.funding-options`, `.ledger`, `.ledger-row`, `.ledger-row.total`, `.metric-grid`, `.metric`, `.metric-label`, `.metric-value`, `.worksheet-note`.
- Lifecycle: `.lifecycle` with `.lifecycle-step`, `.step-index`.
- Learning: `.learning-layout` or `.developer-layout`, `.learning-list`.
- Evidence: `.evidence`, `.evidence-header`, `.evidence-grid`, `.evidence-drawer`, `.source-list`, `.source-id`; native details/summary uses the open attribute. FAQ uses `.faq-item` on native details elements.
- End matter: `.closing`, `.site-footer`, `.footer-links`.

The baseline uses grid and flex layouts; an optional container query collapses the explorer at 720px of component space. Viewport fallbacks collapse major layouts at 760px. Tabs wrap onto multiple rows rather than forcing horizontal scrolling. The stylesheet supplies visible focus, a skip-link style, stable button boxes, at least 44px button targets in these classes, a 44px range-control box, selected borders, reduced motion, forced-colors adjustments and basic print rules. A checkbox's associated full label row supplies the larger interaction area; the checkbox glyph remains 1.2rem.

No viewport or scroll animation hides content. Only background/color/border control feedback transitions are enabled, at 160–180ms. There are no infinite animations, global will-change declarations, blur filters, remote assets or scroll hijacks. Actual diagram state and accessible labeling belong to the React integrator.

## Actual checks

1. `tinycss2.parse_stylesheet` plus recursive rule/declaration parsing: 190 top-level rules, zero parse errors. This parser check does not establish browser property support.
2. Color-pair calculation: ink/paper 12.60:1; muted/paper 5.37:1; forest/paper 8.58:1; light paper/forest 9.44:1; worksheet body/forest 7.13:1. Final copper/paper is reported in REPORT.json.
3. Initial Python Playwright default launch failed because its expected headless-shell executable was absent. No installation was attempted.
4. Root supplied the existing exact executable `/home/charl/.cache/ms-playwright/chromium-1223/chrome-linux64/chrome`. An isolated stylesheet harness was rendered with this Chromium, Python Playwright, `headless=True`, and `--no-sandbox`.
5. Harness viewport widths 1440, 768, 390 and 320: document scroll width equaled viewport width each time. Minimum button height was 44px. The hero used two columns at 1440/768 and one column at 390/320. Reduced-motion emulation returned `0s` transition duration. Keyboard Tab focused the skip link, whose top was 11.1875px and outline style was solid. Harness page errors: zero; network requests: zero.
6. The desktop screenshot of that stylesheet harness was inspected. It showed a legible editorial hierarchy, generous spacing, aligned rules and the deliberately visible keyboard skip link. Its SVG was a clearly labeled stand-in, not the actual HeroWeb component; the screenshot is not a claim that JSX rendered.

The harness was built in process memory and left no additional contribution file. Full React/JSX build, actual HeroWeb render, final scenario interactions, financial arithmetic, enlarged text, complete keyboard flow, SVG paint cost, final mobile screenshots and full-page accessibility remain integration/review work. No performance benchmark or WCAG conformance is asserted. No native, SDK, prover, ledger, provider, wallet, key or private capture work was performed.
