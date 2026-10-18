# Google Material Design 3 (`m3.material.io`) — Foundations, Adaptive Layout Grid & Surface Container Architecture

## 1. Material Design 3 (M3) Adaptive Layout Specifications
Scraped from `https://m3.material.io/foundations/layout/applying-layout`, `https://m3.material.io/foundations/layout/understanding-layout`, `https://m3.material.io/styles/color/roles`, and `https://m3.material.io/styles/elevation/overview`.

### A. Window Size Classes & Breakpoints
| M3 Window Size Class | Width Breakpoint | Columns | Outer Margin | Pane / Spacer Gap | Layout Strategy for Sherlock Workbench |
| :--- | :--- | :--- | :--- | :--- | :--- |
| **Compact** | `< 600px` | 4 cols | `16dp` (`16px`) | `16dp` (`16px`) | Single-pane vertical stack; full-bleed 3D stage (`420px` min-height), horizontal scrolling Filter Chips. |
| **Medium** | `600px – 839px` | 8 cols | `24dp` (`24px`) | `16dp` (`16px`) | Single-pane stacked workbench with 2-column sub-grids for orientation cards and inspector metrics. |
| **Expanded** | `840px – 1199px` | 12 cols | `24dp` (`24px`) | `24dp` (`24px`) | Two-pane layout: Program + Stage paired above full-width Telemetry Inspector pane. |
| **Large / Extra-Large** | `1200px+` (`1600px+`) | 12 cols | `24dp` (`24px`) | `20dp`–`24dp` | Canonical 3-Zone M3 Workbench (`340px` Left Input Pane, `1fr` Central 3D Hardware Viewport Pane, `380px` Right Telemetry & Ring Inspector Pane) bounded at `1880px` max width. |

### B. Containment via Tonal Surface Hierarchy (Replacing Heavy `1px` Borders)
Material Design 3 replaces high-contrast `1px solid` wireframe boxes with **5 tonal surface container steps** (`surface-container-lowest` through `surface-container-highest`) plus subtle `--md-sys-color-outline-variant` (`1px` low-contrast divider only where structural separation is required).

---

## 2. M3 Tonal Surface Container Roles (Warm Alabaster, Precision Brass & Sapphire-Teal Palette)

```css
:root {
  /* Primary: Deep Horological Amber / Brass */
  --md-sys-color-primary: #8f4e00;
  --md-sys-color-on-primary: #ffffff;
  --md-sys-color-primary-container: #ffddc1;
  --md-sys-color-on-primary-container: #2e1500;

  /* Secondary: Sapphire-Teal Continuation / Verification Accent */
  --md-sys-color-secondary: #006874;
  --md-sys-color-on-secondary: #ffffff;
  --md-sys-color-secondary-container: #ccf2f7;
  --md-sys-color-on-secondary-container: #001f24;

  /* Tertiary: Indigo Pattern / Environment Accent */
  --md-sys-color-tertiary: #495395;
  --md-sys-color-on-tertiary: #ffffff;
  --md-sys-color-tertiary-container: #dde1ff;
  --md-sys-color-on-tertiary-container: #000d4b;

  /* Error: Rejected / Fuel Exhaustion Crimson */
  --md-sys-color-error: #ba1a1a;
  --md-sys-color-on-error: #ffffff;
  --md-sys-color-error-container: #ffdad6;
  --md-sys-color-on-error-container: #410002;

  /* M3 5-Step Tonal Surface Container Hierarchy */
  --md-sys-color-surface: #f7f4ee;
  --md-sys-color-surface-dim: #dedad2;
  --md-sys-color-surface-bright: #fdfbf7;
  --md-sys-color-surface-container-lowest: #ffffff;
  --md-sys-color-surface-container-low: #f2eee6;
  --md-sys-color-surface-container: #ece7de;
  --md-sys-color-surface-container-high: #e6e1d6;
  --md-sys-color-surface-container-highest: #e0dbd0;

  /* On-Surface & Outline Tokens */
  --md-sys-color-on-surface: #1d1b18;
  --md-sys-color-on-surface-variant: #4d463c;
  --md-sys-color-outline: #7e766a;
  --md-sys-color-outline-variant: #d0c5b6;
}
```

---

## 3. Four-Zone M3 Page Architecture for Sherlock
1. **Zone 1 — M3 Top App Bar & Hero Header Card (`surface-container-low`, `24dp` radius)**:
   - Displays the M3 Assist/Status Badge (`Moriarty Proof VM · sherlock-k-a/1`), `display-small` headline, plain-English lead paragraph, and live status badge (`#backendBadge`).
2. **Zone 2 — M3 4-Card Orientation Strip (`top-primer-grid`, `16dp` radius)**:
   - Four equal-height M3 Filled/Elevated cards explaining:
     1. **01 · No Hidden Scratchpad** (Explicit continuation rings vs. hidden call stack)
     2. **02 · One Rule & One Fuel per Step** (Bounded `StepRecord` chain)
     3. **03 · How to Read the Six Rings** (Outside-in recursion, inside-out return)
     4. **04 · The 39 Floor Modules (`A080..A225`)** (The `6 × 7` silicon instruction ROM on the chamber floor)
3. **Zone 3 — M3 3-Pane Interactive Workbench (`workbench-grid`)**:
   - **Left Pane (`panel-left`, `surface-container-low`, `24dp` radius)**: M3 Filter Chips for presets (`.preset-btn`), M3 Filled Card for source editor, M3 Filled Pill Button (`#compileRunBtn`), and M3 Sliders (`#fuelSlider`, `#stackSlider`).
   - **Center Pane (`panel-stage`, `surface-container-low`, `24dp` radius)**: M3 Top Header with **M3 Connected Segmented Button Bar (`#cameraLensBar`)**, `16dp`-rounded 3D WebGPU Viewport, and M3 Bottom Transport Bar (`#playPauseBtn` M3 Filled Pill Button + M3 Tonal Step Buttons + M3 Scrubber).
   - **Right Pane (`panel-right`, `surface-container-low`, `24dp` radius)**: Live `StepRecord` telemetry cards (`surface-container-lowest`, `12dp` radius), 12-Slot Split-Arc Ring Matrix, Environment Bindings, and Execution Tape.
4. **Zone 4 — M3 Deep-Dive Reference Cards & Data Tables**:
   - **6-Card Editorial Explanation Grid** (including the dedicated card on the **39 Silicon Floor Modules (`A080..A225`)**).
   - **5-Layer System Architecture Table** (`Moriarty -> Solver/Executor -> Sherlock -> Cryptographic Proof Machinery -> Midnight`).
   - **40-Unit Loan Installment Walkthrough & Authorized vs. Rejected Variations Table**.
   - **39-Rule Silicon Floor Decoder (`sherlock-k-a/1` ROM) Reference Table**.
   - **5-Row CEK-to-3D-Die Mapping Table** (including the 39-Rule Silicon Floor at `y = -3.38`).
