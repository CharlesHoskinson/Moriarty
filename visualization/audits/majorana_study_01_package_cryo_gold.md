# Majorana Study 01 — Majorana 1 QPU Package Architecture, Cryogenic Gold Carrier & Indium-Arsenide Die Aesthetics

**Research Agent**: Majorana Research Agent 1 of 7  
**Primary Sources Examined**:
- Microsoft Azure Quantum Launch Visuals & High-Resolution Hardware Photography by John Brecher (`Majorana-1-001-Hero.jpg`, `majorana1_1260x708_v2.jpg`, `Majorana-1-009.jpg`, `Measurement-graphic.webp`, `Device-Roadmap-figure.webp`, Feb 19, 2025).
- Morteza Aghaee *et al.* (Microsoft Azure Quantum), *"Interferometric single-shot parity measurement in InAs–Al hybrid devices"*, **Nature** 638, 651–655 (19 February 2025).
- Microsoft Quantum, *"Roadmap to fault tolerant quantum computation using topological qubit arrays"*, `arXiv:2502.12252` (February 2025).

---

## 1. Physical Architecture of the Microsoft Majorana 1 QPU Package

Macro inspection and colorimetric sampling of Microsoft's official Majorana 1 QPU photographs reveal a multi-tiered cryogenic hybrid package that bridges macro-scale dilution-refrigerator thermal mechanics down to atomic-scale Molecular Beam Epitaxy (MBE) nanowires:

1. **Tier 1 — Outer Cryogenic Gold-Plated OFHC Copper Enclosure**:
   - Raised rounded-square gold flange (`~52×52 mm`) with 16 counterbored thermal clamping bores and a recessed stepped gold pedestal with 10 horizontal micro-milled thermal ribs.
2. **Tier 2 — Cryogenic Daughterboard Interposer & Satin Silver-Rhodium Plaque**:
   - West flank features a flush satin silver-rhodium shield plaque laser-marked `Majorana 1` in humanist sans-serif (`weight 500`), while North & South ENIG gold bus bars and radial micro-traces converge toward the central basin.
3. **Tier 3 — Concentric Stadium Interposer & `InAs–Al` Topological Core**:
   - Two concentric stepped gold wirebond terraces frame the recessed dark-sapphire / obsidian Indium Arsenide (`InAs`) + epitaxial Aluminum (`Al`) die containing the H-bar tetron nanowires.

---

## 2. Material Contrast & Color Hierarchy: Why the Central Cavity Immediately Commands the Eye

| Architectural Tier | Sampled sRGB Hex Range | Recommended Three.js PBR Tokens | Optical & Perceptual Role |
| :--- | :--- | :--- | :--- |
| **1. Cryogenic Machined Gold Frame & Stepped Ribs** | Base `#aa7f2c`–`#d4af37`<br>Spec `#f9eab5` | `color: 0xd4af37`, `metalness: 0.92`, `roughness: 0.24`, `anisotropy: 0.55` | **Warm High-Mass Perimeter Anchor**: Frames the module and catches grazing rim light. |
| **2. Satin Silver-Rhodium / Platinum Deck** | Base `#d8dde6`–`#e5ceb5`<br>Spec `#fff9f3` | `color: 0xd8dde6`, `metalness: 0.82`, `roughness: 0.32`, `anisotropy: 0.55` | **Cool Neutral Palate Cleanser**: Sits between the outer carrier and the central aperture, giving laser-etched dark typography effortless legibility. |
| **3. Deep Sapphire-Obsidian `InAs` Cavity & Trench** | Trench `#141824`–`#263650`<br>Oxide glint `#4597bc` | `color: 0x161d29`, `metalness: 0.35`, `roughness: 0.24`, `clearcoat: 0.65` | **Maximum Local Contrast Sink**: Creates a deep optical well at the center of the chip so every internal gold/cyan/violet signal pops with maximum contrast. |
| **4. Epitaxial `Al` Nanowires & MZM Accent Signals** | Topo wire `#1d70b8`<br>Trivial wire `#7cc4fa`<br>MZM mode `#fd5d3a` | Cyan/Teal `0x38bdf8` / `0x0d9488`<br>Amber/Gold `0xf59e0b` / `0xfbbf24` | **Complementary Chromatic Pop**: Deep sapphire-cyan (`~475 nm`) and warm gold/amber (`~585–605 nm`) sit opposite on the perceptual color wheel. |

---

## 3. Direct Application to Sherlock (`designer1_package.js` & `designer5_circuit.js`)

1. **PBR Texture Multiplies Fixed (`theme.js` & `designer1_package.js`)**:
   - High-key neutral brush and ceramic grain textures ensure `color * map` and `roughness * roughnessMap` produce authentic satin brushed silver-platinum (`roughness ~0.30`) and bright alabaster porcelain ceramic (`0xebe5da`).
2. **Recessed Edge Contact Pads & Cached Rim Engraving (`designer1_package.js`)**:
   - The 32 instanced gold contact pads sit recessed in the lower ceramic flank (`y = -0.14`) below the champagne-gold interposer seam (`y = -0.06`), and the laser-etched program call (`FACT(4)`) is cached by program signature.
3. **Flush Dark-Sapphire `InAs` Lithography Channels (`designer5_circuit.js` & `designer6_loom.js`)**:
   - Recessed sapphire-slate (`0x18202c`) channels sit flush in the silver-platinum deck (`y = 0.1052`), with dirty-checked `instanceColor` updates.
