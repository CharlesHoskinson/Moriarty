# Gemini Material Design 3 (M3) Audit 01 — Layout Grid, Spacing, Containment & Elevation

- **Auditor**: Gemini Material Design 3 Auditor 1 of 3 (`GMD Auditor 4/6`)
- **Scope**: `/home/charles/sherlock-bench/index.html` (lines 1–314), `/home/charles/sherlock-bench/src/styles.css` (lines 1–1396)
- **Standards Reference**: Material Design 3 (`m3.material.io/foundations/layout/applying-layout`, `m3.material.io/styles/color/roles`, `m3.material.io/styles/elevation`, `m3.material.io/foundations/interaction/states`)

## 1. Executive Summary
1. **Responsive Grid & Pane Proportions**: Align breakpoints to M3 Window Size Classes (`Compact <600px`, `Medium 600–839px`, `Expanded 840–1199px`, `Large 1200–1599px`, `Extra-large >=1600px`). In the `840px–1359px` Expanded/Large range, place `.stage-column` full-width across the top (`grid-column: 1 / -1`) and pair `.panel-left` + `.panel-right` side-by-side (`grid-template-columns: minmax(320px, 1fr) minmax(360px, 1.15fr)`).
2. **Surface Container Hierarchy vs. Heavy `1px` Wireframe Borders**: Replace `0px` corner radii and 37 `1px solid` wireframe boxes with M3’s 5-step tonal surface container hierarchy (`--md-sys-color-surface-container-lowest` through `--md-sys-color-surface-container-highest`), subtle `--md-sys-color-outline-variant` dividers, and M3 shape tokens (`4px`, `8px`, `12px`, `16px`, `24px`, `9999px`).
3. **M3 Elevation Levels (`Level 0`–`Level 3`) & State Layers (`8%` Hover, `12%` Focus/Pressed)**: Keep semantic container colors stable on hover and apply `color-mix()` / `::after` `8%` hover and `12%` focus/pressed state layers plus M3 Level 1–2 elevation shadows.
4. **M3 `4dp`/`8dp` Spatial Rhythm**: Normalize all padding, gap, and height declarations to `4px`/`8px` multiples (`4px`, `8px`, `12px`, `16px`, `24px`, `36px`, `40px`, `48px`).
