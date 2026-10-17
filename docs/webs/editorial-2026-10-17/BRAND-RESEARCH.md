# Moriarty Webs branding research

Result: align Webs with the already published Moriarty product site; retain the README banner as a real project asset, with its Midnight-specific wording kept out of the Cardano page's product claim. Do not invent a new symbol or claim that the banner specifies a complete brand manual.

Falsifiable predicate: the recommended identity elements must appear in tracked Moriarty source or the public homepage registered on the official GitHub repository. This predicate passed for the plain Moriarty wordmark, financial-contract positioning, neutral ruled layout, system typography and the exact palettes below. It did not pass for the current Webs polygon logo, forest-green palette or “field guide” brand subtitle.

## Sources and pins

- Official repository: https://github.com/CharlesHoskinson/Moriarty ; GitHub structured repository metadata gives default branch `main` and homepage https://charleshoskinson.github.io/Moriarty/ . Stored `github-repository.json`.
- Public main pinned at `8174e52975d8db9bf59636fea0fa338c0f47c855`. Read exact `site/src/styles.css`, `site/src/App.tsx`, `site/src/components/Opening.tsx`, `Register.tsx`, `Fork.tsx`, `site/src/sections.tsx` and README through the GitHub structured contents API. Exact bytes and per-file receipts are stored beside this report. Main source: https://github.com/CharlesHoskinson/Moriarty/blob/8174e52975d8db9bf59636fea0fa338c0f47c855/site/src/styles.css .
- Public homepage returned 200 through Scrapling 0.4.15. Its linked stylesheet `/Moriarty/assets/styles-B67y5UUV.css` contains the same light/dark design tokens. Its linked application JS was also acquired, preserving the visible public asset pin. `WEB-RECEIPTS.json` records time, URL, status, SHA-256 and exact bytes. Root robots URL returned 404. No authenticated content, cookies, browsers or bypass were used.
- Active Webs worktree/remote utxo pinned at `dafcf596507753725f0b6971a419e81d4bd4b2ba`. `LOCAL-RECEIPTS.json` pins actual inspected file bytes. Main user checkout was not inspected or modified.
- Real banner: tracked `docs/assets/moriarty-banner.png`, used as README lead image. Introduced at `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`, “Add the Moriarty banner to the README”. A copied evidence asset is `moriarty-banner.png`. Public path: https://github.com/CharlesHoskinson/Moriarty/blob/dafcf596507753725f0b6971a419e81d4bd4b2ba/docs/assets/moriarty-banner.png .

## What the existing product site actually uses

Repository observation, confirmed in live linked CSS:

| Role | Light | Dark |
|---|---|---|
| Page | `#faf9f6` | `#14130f` |
| Recessed surface | `#f2f0ea` | `#0e0d0a` |
| Raised surface | `#ffffff` | `#1c1a16` |
| Rule | `#ddd9cf` | `#2f2c25` |
| Strong rule | `#b9b3a5` | `#4a463c` |
| Text | `#16150f` | `#f0ede4` |
| Secondary text | `#57534a` | `#a9a396` |
| Faint text | `#86806f` | `#7c7668` |
| Accent | `#9a3412` | `#e8724a` |
| Accent surface | `#f5e6df` | `#3a1f14` |

Source comment: “The subject is financial precision, so the page is built like a ledger”. Another source comment says “Figures are the brand.” This is an explicit design rationale, not an inferred brand slogan. The accent is reserved for divergence. Neutral marks indicate reader progress and must not imply proof status.

Typography: sans body and headings, `ui-sans-serif, -apple-system, "Segoe UI", Inter, system-ui, sans-serif`; mono financial values and code, `ui-monospace, "SF Mono", "JetBrains Mono", Menlo, Consolas, monospace`; serif is available but not the default display face. No actual bundled official font files were identified. Inter/JetBrains in the stack do not establish those fonts are shipped.

Masthead: plain sans “Moriarty”, weight600, about1.05rem; secondary mono “bounded financial contracts”. No inline icon is used in `site/src/App.tsx`. A compact nav and clear labels accompany it. Headings use weight600, line-height1.15 and moderate negative tracking. Main h1 max3.6rem, not the current Webs7rem italic serif composition.

The README banner is a separate authentic treatment: dark charcoal scene, eclipse/crescent and star symbol, ivory/gold Roman capitals, character illustration, “A language for Midnight” and “Private. Programmable. Provable.” The image provides visual evidence, not exact font names or approved hexadecimal tokens. There is no extracted independent vector logo or font in the inspected tracked assets. Recreating this emblem approximately would introduce a new mark. A full banner on the Cardano page would also import platform-specific copy, so use only with a clear project-level context.

## Exact Webs issues

1. `App.jsx` renders an arbitrary five-sided connected polygon and circle as `.brand-mark`. No inspected primary brand source uses it. Remove it from the masthead. It can survive only as an explicitly educational diagram, not a logo.
2. The header subtitle and document title claim “A field guide to required proofs”. The field-guide framing comes from this page's visual contribution and is not the published Moriarty positioning. Replace with a literal subject label such as “Webs & Strands” or “Financial proofs for Cardano”. Keep Moriarty primary and Webs secondary.
3. `design.css` provides the real baseline palette and styles. `app.css` is appended after it by `build.mjs`. Editing only the latter can correct current output while leaving frozen historical contributions intact. The current forest `#294b3e`, forest-soft `#e3e9df`, copper `#925e3c`, paper `#f5f1e8` and tall serif headline are a deliberate local page style, but they do not match published Moriarty.
4. The worksheet's forest background and many hard-coded green/pale-gold foregrounds will survive a root variable swap. Restyle the whole worksheet as a neutral ruled financial table or explicitly override every foreground, background and rule there. Existing pale `#d9dfd5`, `#d6c6a6`, `#7d9281` and beige chart values need attention.
5. `build.mjs` hardcodes `<meta name="color-scheme" content="light">` and the field-guide `<title>`. Update alongside the CSS. Keep the artifact offline; don't copy the live site's localStorage preference behavior merely to copy its appearance.

## Concrete implementation recommendation

Use neutral light mode immediately, matching the published main site exactly, with optional system dark mode if the complete diagram/worksheet foregrounds are checked. A safe backwards-compatible token mapping appended to `app.css` is:

```css
:root {
  --paper:#faf9f6; --paper-light:#fff; --paper-deep:#f2f0ea;
  --ink:#16150f; --muted:#57534a;
  --line:#ddd9cf; --line-soft:#e8e4da;
  --forest:#16150f; --forest-soft:#f2f0ea;
  --copper:#9a3412; --copper-soft:#f5e6df;
  --display:ui-sans-serif,-apple-system,"Segoe UI",Inter,system-ui,sans-serif;
  --body:var(--display);
  --mono:ui-monospace,"SF Mono","JetBrains Mono",Menlo,Consolas,monospace;
}
```

These alias names preserve compatibility; a future semantic rename is optional. The neutral ink token should handle selected controls and primary actions. Rust should mark omission, discrepancies, warnings and focus; don't replace every green surface with rust. Override large serif/italic heading rules, reduce h1 to roughly3.6–4.5rem depending on available measure, use weight600, and increase readable paragraph/diagram/control copy toward0.9–1rem. Use tabular numbers on body or at least every financial amount.

Header proposal: linked plain `Moriarty` wordmark, followed by a restrained divider and `Webs & Strands` in mono; navigation “Applications / Invoice / Seven layers / Proof system / Implementation”. Keep the accessible skip link and existing44px targets. Set document title `Moriarty Webs — Webs & Strands for Cardano`; description should name the subject directly. The exact proposed wording is a recommendation, not copied public brand text.

Optional authentic asset treatment: use the actual README banner as a project-level image with the whole image visible and accurate alt text. Do not fabricate a new vector crescent, claim an inferred gold hex as an official color, or crop the platform name into an apparent Cardano-specific promise. The published site's plain wordmark is the strongest existing evidence for the functional masthead.

Verbal direction: explain the financial action and the required artifact in concrete terms. The main site repeatedly leads with specific debts, creditors, assets, rounding and continuing duties. Keep `Web`, `Strand` and `claim specification` when they are defined product terms. Remove ornamental imperatives such as “Keep the distinctions useful”, generic coaching such as “Make the financial handoff explicit”, and repeated slogan fragments. Prefer headings that name the content: “Five applications”, “Invoice payment”, “From proof to claim”, “Developer inputs and outputs”, “Questions”. The main product page's limitations should not be copied wholesale: this task's current user steering controls where current implementation qualifications belong. Preserve existing technical and security meaning in its dedicated section.

## Limits

No brand manual or official independent logo/font package was found in this bounded inspection. No copyright/trademark license conclusion is made. The public app's live JavaScript DOM was not rendered; source plus linked live CSS were inspected, so no screenshot or accessibility-conformance claim follows. No repository/shared code edits, Git mutations, native commands, proofs, wallets, keys or private provider directories were accessed. This is visual/editorial research, not a security assessment. No new cookie or site-pattern knowledge needed to be saved.

## Integrator-selected adaptation after source review

Root selected a coherent hybrid after reading these sources: the actual banner's portrait/eclipse as a cropped illustration, the official site's charcoal/ivory dark tokens, an antique-gold accent adapted from the banner, restrained serif wordmark/banner heading, sans body and monospaced accounting. This is a recommendation/adaptation of two verified project assets, not a claimed formal brand guide. No research objection: it removes the unsupported mark and forest treatment, while keeping genuine Moriarty imagery. Crop through a presentation wrapper without altering the original asset or showing its Midnight-specific tagline as the Cardano page subtitle. Use an accurate decorative/semantic image treatment; retain the real product name as HTML text. Gold values need ordinary contrast checks because the image alone does not establish accessible UI token values. No additional web acquisition is needed.
