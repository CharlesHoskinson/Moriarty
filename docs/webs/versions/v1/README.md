# Moriarty Webs interactive editorial draft

Open [index.html](index.html) directly in a modern browser. The HTML includes React, CSS, SVG, scenario data and third-party license notices. It makes no network requests and uses no persistent storage. External hosting is not configured.

The page explains a proposed concept. Its five application cases and fifteen proof roles are illustrative. The invoice worksheet performs independent local arithmetic. It does not execute Moriarty, produce or verify a proof, sign anything, connect a wallet or submit a transaction. The evidence section retains the source-design and recorded-result limits.

## Editable source

The sole React integration lives in `app/src/App.jsx`. The visual contribution supplies `contributions/03-visual/design.css` and `HeroWeb.jsx`. The scenario contribution supplies `contributions/04-scenarios/scenarios.json` and `invoice-fixtures.json`. The final build requires the frozen role01 glossary, role02 positioning, role06 security copy, role08 learning data and role09 WebDiagram. The build projects role09 map data to readable proposed design targets. Frozen contributions remain untouched in their own directories. `app/build.mjs` reads exact named inputs and records their pins in `PAGE-REPORT.json`.

Five exact public concept/reference inputs are vendored under `reference-inputs/concept/`. Their relative structure is preserved. `reference-inputs/concept/MANIFEST.json` records origin paths, byte lengths and SHA-256 digests. Origins supply provenance only. The build reads the packaged copies and requires no sibling research directory.

The build removes absolute source paths from browser-facing scenario references. Readable source names and section/JSON pointers remain available in expandable details. Raw source provenance remains in the local contribution files and reports.

## Rebuild

Use Node.js and npm. This draft was built with Node v24.21.0 and npm 11.19.0. Its exact dependencies are React 19.3.1, React DOM 19.3.1 and esbuild 0.28.2. `app/package-lock.json` pins the complete installed dependency graph. Public-registry metadata is recorded in `DEPENDENCY-REGISTRY-PINS.json`.

From `app/`, create empty local configuration files, then install the locked packages:

```bash
touch .npmrc-user .npmrc-global
npm ci --ignore-scripts --no-audit --no-fund --registry=https://registry.npmjs.org --cache .npm-cache --userconfig .npmrc-user --globalconfig .npmrc-global
```

The two empty local configuration files keep this operation separate from user npm settings. The build uses esbuild's packaged platform executable. No lifecycle installation scripts are needed for the captured environment.

Build the standalone artifact:

```bash
npm run build
```

This overwrites only generated app data, the app notice file, `index.html` and `PAGE-REPORT.json`. It does not execute source contracts, native cryptography, a ledger SDK or any blockchain action. It does not alter Git.

## Interactions to inspect

Choose all five application cases. Select each illustrated proof role. Omit and restore the selected role. The status distinguishes incomplete illustration from illustrative assembly and never reports verification or settlement.

Move the invoice payment slider from 1 to 99. At payment 35, source funding 110 leaves cash 73 and duties 75. Proposed funding 112 leaves cash 75 and duties 75. Opening costs, exact asset identity and the missing reserve-duty-return action remain in the details.

Use keyboard navigation and expandable evidence/FAQ details. Security-related FAQ links open the complete original Opus card, retaining all assumptions, evidence stage and citations. Check the page at phone and desktop widths and with reduced motion. The CSS supplies a readable baseline and visible focus. Browser behavior and conformance require actual observations. The build report does not invent them.

The optional static packaging check is `npm run check:artifact`. It checks resource packaging, dependency integrity and frozen fixture values. Its receipt pins the exact checked HTML and establishes no browser or product behavior.

## Attribution and validation

Hero SVG and visual CSS are original local contribution work. The interactive diagram uses role09’s actual editable SVG/React. Its duplicate standalone HTML role controls are hidden because the page provides the same content and actions through its visible role list and selected-role inspector. Decorative perimeter connections are hidden to keep only the proposed required-set relationship. No source contribution was edited. Scenario, language, positioning, learning and security wording retain their contribution provenance. Security wording and judgments remain Opus-owned.

React, React DOM, Scheduler and esbuild use MIT licenses. Full notices are preserved in `app/THIRD-PARTY-NOTICES.txt` and the HTML artifact.

`PAGE-REPORT.json` records the actual build result, source/output pins, integrated contributions and remaining review boundaries. Root owns final browser QA and fresh editorial/security-wording review. A successful frontend build is not product, security, proof or ledger acceptance.
