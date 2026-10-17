# Moriarty Webs and Strands

Open [index.html](index.html) directly in a browser. The page explains how Webs carry proofs for Cardano financial transactions, starting with a partial invoice payment. It includes five applications, an interactive payment worksheet, settlement, developer integration, Groth16 setup and seven architectural concerns. Source credit for the seven-layer framework is in the architecture section's Sources disclosure.

The October 17 revision adds real source examples and quantitative transaction/application diagrams after ten Gemini 4 Argon and ten Claude Haiku 5.5 readers found the earlier explanation too shallow. [Their complete reports and limitations](depth-2026-10-17/reader-study/STUDY.json) are preserved. The deeper version has completed its source and final-delta reviews; [DELIVERY.md](DELIVERY.md) records their exact scopes.

The editorial revision also replaces the earlier marketing copy and visual treatment. It uses charcoal and ivory from the published product site, with an antique-gold accent and crescent emblem adapted from the repository banner. A new GPT-generated Web and Strands illustration replaces the portrait, following ten Luna visual studies. The [editing checklist](editorial-2026-10-17/SLOP-LIST.md) records 20 concrete problems and how to correct them. Three agents researched editorial practice, newcomer comprehension and actual brand assets using Scrapling for web retrieval. Their reports and source metadata are in [editorial-2026-10-17/](editorial-2026-10-17/).

The page is one HTML file containing React, CSS, the artwork and its data. It uses system fonts and makes no external asset requests. It has no wallet connection, proof generation, transaction submission, telemetry or persistent storage. Actual implementation evidence is kept in the technical notes.

## Build

The editable app is in [app/](app/). From that directory:

```bash
npm ci --ignore-scripts --no-audit --no-fund
npm run build
npm run check:artifact
```

React 19.3.1, React DOM 19.3.1 and esbuild 0.28.2 are pinned by the lockfile. The build embeds `assets/moriarty-web-strands.png`; it also pins the original repository banner as brand reference material, so retain the repository layout. It writes `index.html`, `PAGE-REPORT.json`, generated app data and license notices. The artifact checker writes `app/ARTIFACT-CHECK.json`.

`editorial-2026-10-17/COPY.json` supplies the revised display copy through `app/src/editorial.mjs`. Frozen research contributions remain unchanged. `app/src/brand.css` applies the current visual treatment after the historical base styles. The seven original technical layer explanations, setup trust conditions and detailed evidence records remain intact. `depth-2026-10-17/COPY.json` supplies the deeper worked cases, literal source/selector and Opus-authored cryptographic exposition. The new React modules and scoped CSS render those sources; `PAGE-REPORT.json` pins the exact inputs.

## Interactions

The default payment is 35 CASH from a proposed 112-CASH funding policy covering one partial-payment fee. The seller receives 35, the program fee recipient receives 2, and 75 remains to cover an unpaid invoice of 65 plus a reserve obligation of 10. The 110 CASH comparison leaves a 2 CASH shortfall. The literal source-opening calculation and the checked-in 109/90/14 request are explicitly separate scenarios. Payment values are restricted to 1–99; network fees are separate.

All five applications trace financial inputs, payments and continuing accounts. The invoice diagram shows one whole-execution Strand covering three checks. Its missing-evidence control removes the whole required evidence set and visibly qualifies the intended result. Other applications distinguish supplied accounting premises from missing transaction layouts.

The setup counter varies a hypothetical number of compatible compiled circuits. Its result counts one shared phase 1 and one phase 2 per distinct circuit. It does not count complete ceremonies or claim a fixed total for Cardano. The architecture diagram preserves all seven canonical names and draws the supplied relationships touching the selected concern; it does not turn reciprocal relationships into a directed execution order.

## Verification and history

[DELIVERY.json](DELIVERY.json) identifies the selected HTML and current review scope. [The candidate browser record](verification/depth-v4-published/RESULT.json) covers the revised artifact. Old check counts and reviews apply only to their original bytes. The first candidate's layout failures and the next passing candidate are retained in the verification history.

The original contributions, previous delivery and versions `v1`, `v2`, `v2-r2`, `v3-editorial`, `v3-web-art` and the v4 candidate snapshots remain historical evidence. The persona and book-translation rounds were model exercises, not human usability research. Frontend checks and editorial reviews do not establish compiler, cryptographic, production or Cardano settlement readiness.

The [art studies and synthesis](art-2026-10-17/SYNTHESIS.md) preserve all ten requested Luna proposals. [IMAGE-PROMPT.md](art-2026-10-17/IMAGE-PROMPT.md) records the exact built-in generation prompt. [ASSET.json](art-2026-10-17/ASSET.json) pins the selected PNG. The model identifier was not exposed by the image-generation tool.
