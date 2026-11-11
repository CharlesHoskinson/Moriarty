---
name: Sherlock pipeline page
description: A Material 3 dark explainer in which a 3D proof line and its narration read as one system.
colors:
  primary: "#e9c348"
  on-primary: "#412d00"
  primary-container: "#5a4400"
  on-primary-container: "#ffe086"
  secondary: "#d4c5a1"
  secondary-container: "#50462a"
  on-secondary-container: "#f1e1bb"
  tertiary: "#accfae"
  error: "#ffb3aa"
  surface: "#15130e"
  surface-container-lowest: "#100e09"
  surface-container-low: "#1e1b16"
  surface-container: "#221f1a"
  surface-container-high: "#2c2a24"
  surface-container-highest: "#37342e"
  on-surface: "#e8e2d9"
  on-surface-variant: "#cfc6b4"
  outline: "#989080"
  outline-variant: "#4c4639"
  program: "#cdbdff"
  input: "#6fd2ff"
  outcome: "#77d9b6"
  step: "#ffb782"
typography:
  title-large:
    fontFamily: "Roboto Flex, system-ui, sans-serif"
    fontSize: "22px"
    fontWeight: 400
    lineHeight: "28px"
  title-small:
    fontFamily: "Roboto Flex, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 500
    lineHeight: "20px"
  body-large:
    fontFamily: "Roboto Flex, system-ui, sans-serif"
    fontSize: "16px"
    fontWeight: 400
    lineHeight: "24px"
    letterSpacing: "0.01em"
  body-medium:
    fontFamily: "Roboto Flex, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 400
    lineHeight: "20px"
  label-large:
    fontFamily: "Roboto Flex, system-ui, sans-serif"
    fontSize: "14px"
    fontWeight: 500
    lineHeight: "20px"
  label-medium:
    fontFamily: "Roboto Flex, system-ui, sans-serif"
    fontSize: "12px"
    fontWeight: 500
    lineHeight: "16px"
    letterSpacing: "0.02em"
  icon:
    fontFamily: "Material Symbols Rounded"
    fontSize: "24px"
    fontWeight: 400
    lineHeight: "24px"
  icon-chip:
    fontFamily: "Material Symbols Rounded"
    fontSize: "18px"
    fontWeight: 400
    lineHeight: "18px"
  icon-fab:
    fontFamily: "Material Symbols Rounded"
    fontSize: "36px"
    fontWeight: 400
    lineHeight: "36px"
  code:
    fontFamily: "Roboto Mono, ui-monospace, monospace"
    fontSize: "12px"
    fontWeight: 500
    lineHeight: "16px"
rounded:
  hairline: "1px"
  track: "2px"
  extra-small: "4px"
  small: "8px"
  medium: "12px"
  large: "16px"
  extra-large: "28px"
  full: "9999px"
spacing:
  xs: "4px"
  sm: "8px"
  md: "16px"
  lg: "24px"
components:
  fab-large:
    backgroundColor: "{colors.primary-container}"
    textColor: "{colors.on-primary-container}"
    rounded: "{rounded.extra-large}"
    size: "96px"
  icon-button:
    textColor: "{colors.on-surface-variant}"
    rounded: "{rounded.full}"
    size: "48px"
  rail-destination-active:
    backgroundColor: "{colors.secondary-container}"
    textColor: "{colors.on-secondary-container}"
    rounded: "{rounded.full}"
    width: "56px"
    height: "32px"
  chip-scene:
    textColor: "{colors.on-surface-variant}"
    rounded: "{rounded.small}"
    height: "40px"
  chip-scene-active:
    backgroundColor: "{colors.secondary-container}"
    textColor: "{colors.on-secondary-container}"
  narration-card:
    backgroundColor: "{colors.surface-container-low}"
    textColor: "{colors.on-surface}"
    rounded: "{rounded.large}"
    padding: "24px"
  code-card:
    backgroundColor: "{colors.surface-container-high}"
    textColor: "{colors.on-surface}"
    typography: "{typography.code}"
    rounded: "{rounded.medium}"
    padding: "12px"
  stage-label:
    backgroundColor: "{colors.surface-container-highest}"
    textColor: "{colors.on-surface}"
    rounded: "{rounded.small}"
    padding: "4px 8px"
---

# Design System: Sherlock pipeline page

## Overview

**Creative North Star: "The Lit Proof Line"**

One dark room with one lit bench. The page is Material 3 in its dark scheme, and the 3D stage uses the same tonal palette as the chrome, so a viewer reads one object: a workbench that carries a program from the left edge to an accepted claim on the right. Nothing is decoration. The chrome stays quiet and tonal; the colors that carry meaning (program, input, outcome, running step, proof gold, refusal) are the only saturated things on screen.

The product is an Experience/Read page: viewers watch and read closely, step by step. The narration is real text in a calm card beside the stage. Controls follow Material 3 so the player needs no explanation.

**Key Characteristics:**
- Dark only. Tonal surfaces carry hierarchy; shadows appear only on floating things (FAB, labels, badge).
- One seed, `#d4af37` (the gold of proofs and hashes), generates every tonal palette in HCT.
- One hue, one meaning, identical on the canvas and in the DOM.
- Every moment is addressable and renders the same way each time.

## Colors

A warm-neutral dark scheme with a gold primary and four harmonized meaning colors.

### Primary
- **Proof Gold** (#e9c348): hashes, digests of state, proofs, keys, the accumulator, the slider and the active track. Container form (#5a4400 with #ffe086) holds the play FAB.

### Secondary
- **Parchment Dust** (#d4c5a1): focus rings. Its container (#50462a with #f1e1bb) marks the active scene.

### Tertiary
- **Lichen** (#accfae): reserved by the scheme; the page does not use it.

### Meaning colors (custom, harmonized toward the seed hue by at most 15 degrees)
- **Program Violet** (#cdbdff): source, program image, ROM, `prog_d`.
- **Input Sky** (#6fd2ff): the input value and `input_d`.
- **Outcome Mint** (#77d9b6): the outcome value and `outcome_d`.
- **Step Ember** (#ffb782): the step in progress and the moving request.
- **Refusal Rose** (#ffb3aa, the error role): a failed check or a refused transaction.

### Neutral
- **Deepest Slate** (#100e09, surface-container-lowest): chain-side ground (stations 7 and 8).
- **Page Ink** (#15130e, surface): page background.
- **Card Slate** (#1e1b16, surface-container-low): the narration card and chain-side bench slabs.
- **Prover Ground** (#2c2a24, surface-container-high): prover-side ground (stations 1 to 5) and the stage fallback.
- **Prover Bench** (#37342e, surface-container-highest): prover-side bench slabs, label and badge backgrounds, inactive slider track.
- **Parchment** (#e8e2d9, on-surface): text and ceramic objects. **Warm Stone** (#cfc6b4, on-surface-variant): secondary text. **Outline** (#989080) and **Outline Variant** (#4c4639): borders.

### Named Rules
**The One Meaning Rule.** A meaning color marks only its meaning. Gold never decorates; program violet never marks input.
**The Ground Change Rule.** The prover side is the lighter dark surface and the chain side the deepest one; the crossing in scene 6 interpolates between them.

## Typography

**Display and body font:** Roboto Flex (variable, with system-ui fallback)
**Mono font:** Roboto Mono, only for identifiers, tuples and numbers that are code
**Icon font:** Material Symbols Rounded at 24px (18px in scene chips, 36px in the FAB)

**Character:** neutral and exact. The weight steps are 400 for reading and 500 for labels.

### Hierarchy
- **Title large** (400, 22px, 28px): app bar title and the beat title in the narration card.
- **Title small** (500, 14px, 20px): the color key heading.
- **Body large** (400, 16px, 24px, 0.01em): narration paragraphs, capped at 68ch.
- **Body medium** (400, 14px, 20px): the scene and beat line under the beat title.
- **Label large** (500, 14px, 20px): scene chips, time text (tabular numerals), assist chips, stage badge.
- **Label medium** (500, 12px, 16px): rail labels and in-scene labels. Powers are written with a superscript.

## Layout

At 840px and wider: app bar (64px), then three columns: a 96px navigation rail, the stage column (stage with controls beneath) and a 400px narration column that scrolls on its own. Below 840px the order is stage, controls, scene chips (a horizontal scroller), narration card, color key. The stage keeps 4:3 on phones with 16px side gutters. Spacing runs on a 4px base: 8 between chips, 16 between groups, 24 inside the card.

## Elevation & Depth

Tonal layering first. Surfaces step up through the container tones; the narration card is a filled card with no shadow and no border. Shadows mark only floating items.

### Shadow Vocabulary
- **Level 1** (`0 1px 2px rgb(0 0 0 / 0.30), 0 1px 3px 1px rgb(0 0 0 / 0.15)`): stage badge and in-scene labels.
- **Level 3** (`0 1px 3px rgb(0 0 0 / 0.30), 0 4px 8px 3px rgb(0 0 0 / 0.15)`): the play FAB; level 4 on hover.

### Named Rules
**The Declared Elevation Rule.** An item uses a tone step, a border or a shadow for separation, never two.

## Shapes

The M3 shape scale: 4, 8, 12, 16, 28 and full. Stage and narration card use 16. Labels, badge and scene chips use 8. The FAB uses 28 (96px square). Icon buttons, the slider handle, the rail indicator pill and colour dots are fully round.

## Components

### Buttons
- **Play FAB (large):** 96px (80px below 840px), primary-container with on-primary-container icon, radius 28px, level-3 shadow. Icon swaps among play_arrow, pause and replay.
- **Icon buttons:** 48px touch target, 40px state layer inset 4px, on-surface-variant icon. Beat stepping uses skip_previous and skip_next; scene stepping uses first_page and last_page.
- **States:** hover 8%, focus 10%, pressed 10% layer in the current color, over 150ms with the standard easing. Focus adds a 3px secondary outline with 2px offset.

### Chips
- **Scene chip (below 840px):** 40px high, 8px radius, 1px outline, 8px gap between icon and label. Active: secondary-container fill, no outline, filled icon. Hit area 48px.
- **Assist chip (color key):** 32px high, 8px radius, 1px outline-variant, a 10px color dot.

### Navigation
- **Rail (840px and wider):** 96px wide, eight destinations with icon, a 56 by 32 indicator pill (secondary-container when active) and a label-medium label. Icons: description, directions_run, memory, account_tree, verified, send, gavel, hub.

### Slider
- 4px track, 20px round handle, primary active track on surface-container-highest. Beat boundaries show as 4px dots, scene boundaries as 2 by 16px bars. Hover and focus add a 20px halo at 8% and 10% primary.

### Cards
- **Narration card:** surface-container-low, 16px radius, 24px padding. Beat title in title-large, scene line in body-medium, paragraphs in body-large.

### Code card
- Surface-container-high, 12px radius, 12px padding, level-1 shadow, top right of the stage (56% wide, at most 30rem). Roboto Mono 12px on 18px lines, tabular line numbers in on-surface-variant, highlighted lines on secondary-container. It shows source and machine-language listings and never takes pointer events.

### In-scene labels
- DOM text anchored to 3D points, projected every frame. Surface-container-highest, 8px radius, label-medium, mono for identifiers, a round chip in the meaning color. At most six scene labels at once, plus the four carrier labels.

## Do's and Don'ts

### Do:
- **Do** take every canvas color from `theme.PALETTE` and `theme.FAMILY`, which map to the M3 dark roles (see `src/pipeline/SCENE-API.md`).
- **Do** keep text on `on-surface` or `on-surface-variant`; every pair on the container surfaces passes WCAG 2.2 AA (on-surface on surface-container-highest is 9.6:1, outline on it is 3.9:1 for borders).
- **Do** keep 48px touch targets and visible focus rings on every control.
- **Do** keep stepped mode (`prefers-reduced-motion`) as cuts to each beat's hold pose.

### Don't:
- **Don't** add a light scheme, a theme toggle or a `prefers-color-scheme` switch.
- **Don't** put a kicker or an eyebrow above a heading, or nest cards.
- **Don't** use a raw hex value in a scene; use the PALETTE key.
- **Don't** use shadows and borders together on one surface, or a colored side border above 1px.
