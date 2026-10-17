# Moriarty Webs and Strands

Open [index.html](index.html) directly in a modern browser. This second iteration explains the complete product design for Cardano dapp developers through five financial cases, a shared invoice worksheet, the seven-layer framework from Proving Nothing and a dedicated Groth16/setup section. A single technical appendix records current implementation evidence and assumptions.

The standalone HTML includes React, CSS, editable original SVG, data and full third-party MIT notices. It uses system fonts and has no external asset requests, wallet connection, signatures, proof execution, transactions, telemetry or persistent storage.

## Source and build

The app integrates the original frozen visual contribution with app-owned components in `app/src/`. `App.jsx` owns the invoice amount and funding state; `invoice.mjs` calculates the values used by both the worksheet and invoice inspector. `WebDiagram.jsx` renders one visible native HTML button per role over original SVG connections, with no hidden duplicate controls. `SevenLayers.jsx` supplies seven native selectors, dependency navigation, an artifact mapping, pinned book credit and direct links to relevant guide sections. `Groth16.jsx` explains statements, witnesses, keys and setup stages, with a hypothetical compatible-circuit counter. Exact security and cryptographic meaning comes from the designated author's V2 and two new section copies; original detailed security cards remain in the technical appendix.

Dependencies are already installed locally. The exact versions are React 19.3.1, React DOM 19.3.1 and esbuild 0.28.2, pinned in `app/package-lock.json` and checked against `DEPENDENCY-REGISTRY-PINS.json`. From `app/`:

```bash
npm run build
npm run check:artifact
```

The build reads only exact named package-local source and reference inputs. It writes generated app data, notices, `index.html` and `PAGE-REPORT.json`; the artifact check writes `app/ARTIFACT-CHECK.json`. Frozen v1 and contributions are not rewritten. Public source document basenames and their exact pointers are retained in source notes without absolute host paths. `app/OWNED-FILES.json` supplies the exact allowlist.

## Inspect the interactions

Switch among all five cases, select each role, then omit and restore it. The diagram and inspector share selection. On narrow screens, a deliberate role selection reveals the inspector. Keyboard focus stays on the native control; reduced-motion preferences disable smooth scrolling and transitions.

Change Seller payment from 35 to 36 and revisit the invoice role statement: both describe the same amount, fee, debit and remaining duties. The default fee-funded example starts with 112 CASH. At payment 35, cash remaining is 75, invoice duty is 65, reserve duty is 10, and total duties are 75. Selecting the 110 CASH source comparison leaves cash 73 and a 2 CASH gap. Partial values are restricted to 1–99. Program fees and network fees are separate. Source details retain the absence of an extra-funding action and reserve-duty-return action.

The visible product story describes the design. It does not establish a deployed Web integration or available Web SDK. The technical appendix preserves actual recorded evidence, source-reading boundaries, setup limitations, complete ACTUS/DeFi scope and current documented command interfaces. The repository entry point is the [Moriarty README](https://github.com/CharlesHoskinson/Moriarty/blob/utxo/README.md).

## Seven layers and setup counts

The seven layer names and order follow the pinned manuscript: Setup, Language, Witness, Arithmetization, Proof System, Cryptographic Bedrock and Verification. The navigator links each concern to its relevant guide section. Dependency links describe design relationships; the page does not turn the seven layer numbers into a runtime pipeline. The source credit links to commit `2789e5a63972276e20bc6144733ef9824ced9e49` of the book repository.

The circuit counter starts at K = 5 and accepts integers 1–20. Its output is one shared phase 1 plus K circuit-specific phase 2 stages, under the conditions printed beside it. It does not sum these into K + 1 complete ceremonies or assign a Cardano-wide total. Reuse examples distinguish changing inputs from changing the compiled relation. Current test-setup status stays in the implementation appendix.

The ten translation agents delivered 46 proposals and 110 questions. Their synthesis informed the section navigation and content. Separate Opus authors supplied the adopted section copy and bounded source checks; the page does not certify the whole book. The original V2 copy remains unchanged. Its source-read correction is an explicit display transform approved in `contributions/13-sevenlayer/SECTION-NOTES.md`: “The executed source differs.” becomes “The S12 source differs from this design.”

## Validation record

`PAGE-REPORT.json` pins inputs and the HTML output. `app/ARTIFACT-CHECK.json` pins the artifact checked for packaging, dependencies, source configuration, copy completeness, React static rendering and invoice arithmetic against frozen independent fixtures. `V2-REPORT.json` records implementation changes and exact file digests. Root owns fresh browser QA and final-byte independent editorial/security review; frontend checks do not establish those outcomes or proof/ledger/product acceptance.

Original contributions, `versions/v1/` and `persona-round1/` remain historical inputs. The ten model persona records are simulated feedback, not human user research. Full license notices are preserved in `app/THIRD-PARTY-NOTICES.txt` and inside the HTML.
