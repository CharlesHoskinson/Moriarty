# Google Material Design 3 (`m3.material.io`) — Typography Scale, Shape Tokens & Component Specifications

## 1. M3 Design Tokens (`--md-sys-typescale-*`, `--md-sys-shape-corner-*`, `--md-sys-elevation-*`)
Scraped from `https://m3.material.io/styles/typography/type-scale-tokens`, `https://m3.material.io/styles/shape/shape-scale-tokens`, and `https://m3.material.io/components/`.

| M3 Token Category | Token Name | Value | Application in Sherlock |
| :--- | :--- | :--- | :--- |
| **Typography: Display** | `--md-sys-typescale-display-small` | `clamp(1.85rem, 2.6vw, 2.35rem)` / `1.18` / `-0.015em` | Top Masthead `h1` |
| **Typography: Headline** | `--md-sys-typescale-headline-small` | `1.35rem` (`21.6px`) / `1.3` / `600` | Deep-dive section headers |
| **Typography: Title** | `--md-sys-typescale-title-medium` | `1.0rem` (`16px`) / `1.4` / `600` | Card titles & Primer card headers |
| **Typography: Title Small** | `--md-sys-typescale-title-small` | `0.875rem` (`14px`) / `1.4` / `600` | Workbench pane headers |
| **Typography: Body Large** | `--md-sys-typescale-body-large` | `1.0rem` (`16px`) / `1.6` / `400` | Top lead prose |
| **Typography: Body Medium** | `--md-sys-typescale-body-medium` | `0.875rem` (`14px`) / `1.55` / `400` | Card body copy & table cells |
| **Typography: Label Large** | `--md-sys-typescale-label-large` | `0.875rem` (`14px`) / `1.25` / `600` | M3 Filled Buttons & Segmented Buttons |
| **Typography: Label Medium** | `--md-sys-typescale-label-medium` | `0.75rem` (`12px`) / `1.25` / `600` | M3 Filter Chips & section kickers |
| **Shape: Extra Small** | `--md-sys-shape-corner-extra-small` | `4px` | Code badges, inline pills |
| **Shape: Small** | `--md-sys-shape-corner-small` | `8px` | M3 Filter Chips (`.preset-btn`), Ring slots |
| **Shape: Medium** | `--md-sys-shape-corner-medium` | `12px` | Inner M3 cards, telemetry readouts, tables |
| **Shape: Large** | `--md-sys-shape-corner-large` | `16px` | Orientation cards, 3D stage frame, editorial cards |
| **Shape: Extra Large** | `--md-sys-shape-corner-extra-large` | `24px` | Outer workbench panes (`panel-left`, `panel-stage`, `panel-right`) |
| **Shape: Full** | `--md-sys-shape-corner-full` | `9999px` | M3 Segmented Button bar, Filled Buttons, Transport Pills |

---

## 2. M3 Component Mapping for Sherlock
1. **Camera View Selector (`#cameraLensBar` -> M3 Connected Segmented Button)**:
   - Outer capsule radius `9999px`, `1px solid var(--md-sys-color-outline)`, `36px–40px` height.
   - Active button (`aria-selected="true"` / `.active`) uses `--md-sys-color-secondary-container` fill (`#ccf2f7`) and `--md-sys-color-on-secondary-container` text (`#001f24`) with a subtle leading check/lens indicator.
2. **Program Presets (`.preset-strip` -> M3 Filter Chips)**:
   - `8px` corner radius (`--md-sys-shape-corner-small`), `1px solid var(--md-sys-color-outline-variant)`, `surface-container-lowest` fill.
   - Selected chip uses `--md-sys-color-primary-container` (`#ffddc1`) and `--md-sys-color-on-primary-container` (`#2e1500`).
3. **Compile & Transport Controls (`#compileRunBtn`, `#playPauseBtn`, `.btn-icon`)**:
   - Full pill shape (`9999px`), `40px` height (`36px` compact), M3 state layer hover/focus transitions.
4. **M3 Cards & Data Tables**:
   - Tonal surface containers (`surface-container-lowest` inside `surface-container-low` panes) with `12px–16px` rounded corners and soft M3 Level-1 elevation shadows (`0 1px 3px rgba(29,27,24,0.06), 0 1px 2px rgba(29,27,24,0.04)`).
