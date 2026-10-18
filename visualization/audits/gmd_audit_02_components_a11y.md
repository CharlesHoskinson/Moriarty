# Gemini Material Design 3 (M3) Audit 02 — Components, Interactive Controls, Accessibility & State Layers

- **Auditor**: Gemini Material Design 3 Auditor 2 of 3 (`GMD Auditor 5/6`)
- **Scope**: `/home/charles/sherlock-bench/index.html`, `/home/charles/sherlock-bench/src/styles.css`, `/home/charles/sherlock-bench/src/app.js`
- **Standards Reference**: Material Design 3 (`m3.material.io/components/segmented-buttons/specs`, `m3.material.io/components/chips/specs`, `m3.material.io/components/buttons/specs`, `m3.material.io/components/sliders/specs`)

## Key Component Upgrades
1. **`.preset-strip` & `.preset-btn` -> M3 Filter Chips / Interactive Cards (`8dp`–`12dp` radius, `48dp` target height)**:
   - Uses `--md-sys-color-surface-container-lowest` resting fill, `--md-sys-color-outline` accessible border, `8%`/`12%` state layer, and `--md-sys-color-secondary-container` (`#f5dfc3` / `#2e1500`) active selection state.
2. **`#cameraLensBar` -> M3 Connected Single-Select Segmented Button (`40dp` height, `9999px` stadium pill)**:
   - Shared `1px solid var(--md-sys-color-outline)` collapsed borders (`margin-left: -1px`), `--md-sys-color-secondary-container` active segment fill with leading `✓` check indicator, and `8%`/`12%` state layer.
3. **`.transport-bar` -> M3 Filled Primary (`#playPauseBtn`), Tonal Step (`#stepPrevBtn`, `#stepNextBtn`), Outlined (`#resetStepBtn`), and M3 Sliders (`#stepScrubber`, `#fuelSlider`, `#stackSlider`)**:
   - `40dp` pill buttons (`border-radius: 9999px`) and custom `6dp` track + `20dp` thumb sliders with `40dp` (`10px` spread) M3 hover/focus state halos.
4. **WCAG 2.1 AA/AAA Contrast & Tabular Numerals**:
   - Fixes `.control-sector-chip.active.is-done` contrast (`#0f766e` background with `#ffffff` text = `5.44:1`), uses `#787166` (`4.35:1`) for interactive outlines, and applies `font-variant-numeric: tabular-nums` to all live step/fuel counters.
