# Majorana Study 05 — Majorana 1 PBR Material Shaders (OFHC Cryo-Gold, Epitaxial Al/InAs Sapphire & Crystalline Dielectrics)

**Agent**: Majorana Research Agent 5 of 7  
**Target Modules**: `src/designers/theme.js`, `src/stage/SherlockStage.js`, `src/designers/designer1_package.js`, `src/designers/designer2_well.js`, `src/designers/designer3_frames.js`, `src/designers/designer5_circuit.js`  
**Runtime**: `three@0.186.1` (`three/webgpu` NodeMaterial / TSL pipeline)  
**Primary Empirical Sources**:
- Microsoft Azure Quantum Press Kit & Macro Photography (*Majorana 1* `Majorana-1-001`..`010` and *Majorana 2* `Majorana-2-web-size_1`..`6`, photographed by John Brecher)
- Microsoft Azure Quantum & *Nature* (Feb 2025): *"Interferometric Single-Shot Parity Measurement in InAs-Al Hybrid Devices"* (`doi:10.1038/s41586-024-08445-2`)

---

## 1. Executive Summary & Optical Physics of the Majorana 1 QPU

In Microsoft’s official macro photography and studio 3D renders of the **Majorana 1** topological quantum processing unit (QPU), visual legibility is achieved through a disciplined **three-material optical contrast triad** engineered for millikelvin dilution refrigerator stages:

1. **Cryogenic OFHC Gold Interposer & Micro-Machined Carrier (`#d4af37` / `#e6c35c`)**:
   - Oxygen-Free High-Conductivity (OFHC) copper-core packaging electroplated in high-purity cryogenic gold.
   - Flat deck lands exhibit **anisotropic CNC micro-machined toolpaths** (`metalness: 0.92`, `roughness: 0.18..0.28`, `anisotropy: 0.76`), while 45° diamond-cut perimeter and bore chamfers are **mirror-polished** (`#e6c35c`, `metalness: 0.96`, `roughness: 0.08`, `anisotropy: 0.12`) to catch razor-sharp specular highlights.
2. **Epitaxial Aluminum & Indium Arsenide (`InAs/Al`) Wafer Substrate (`#0f172a`..`#1e293b`)**:
   - The central "topoconductor" die is grown atom-by-atom via molecular beam epitaxy (MBE): an **Indium Arsenide (`InAs`)** heterostructure paired with epitaxial **Aluminum (`Al`)** nanowires and insulated by atomic-layer-deposited (ALD) high-$\kappa$ dielectrics ($\text{Al}_2\text{O}_3$ / $\text{HfO}_2$, thickness $180\text{–}420\text{ nm}$).
   - Optically, this stack behaves as a **deep iridescent midnight-sapphire semiconductor mirror** (`#0f172a` base to `#1e293b` active terrace) topped by a **thin-film dielectric clearcoat** (`clearcoat: 0.65`, `clearcoatRoughness: 0.15`, `iridescence: 0.48`, `iridescenceIOR: 1.76`, `iridescenceThicknessRange: [180, 420]`) that shifts grazing reflections toward cool **cyan (`#38bdf8`)** and **royal indigo (`#818cf8`)**.
3. **Brushed Platinum / Rhodium Gate Electrodes & Horological Rings (`#dce2ec`)**:
   - Superconducting aluminum H-tetron nanowires, electrostatic top gates, microwave readout resonators, and the 12 concentric horological depth arcs are rendered as **crisp, high-albedo satin Platinum/Rhodium** (`#dce2ec`, `metalness: 0.85`, `roughness: 0.28`, `anisotropy: 0.68`).

---

## 2. Empirical Colorimetry & Material Breakdown

| Empirical Region in Microsoft Renders / Photos | Sampled sRGB Hex Cluster (Photo Crop) | Linear Albedo Target (`theme.js` Token) | Physical Surface Mechanism |
| :--- | :--- | :--- | :--- |
| **OFHC Cryo-Gold Deck (Machined Flat)** | `#c9aa6b`, `#d3c292`, `#a7895b` | `#d4af37` (`PALETTE.gold`) | CNC fly-cut / micro-milled gold plating (`roughness: 0.22`, `metalness: 0.92`, `anisotropy: 0.76`) |
| **OFHC Cryo-Gold 45° Diamond Chamfer** | `#e1c37e`, `#e2c57f`, `#f2e1b1` | `#e6c35c` (`PALETTE.goldBright`) | Single-crystal diamond-turned 45° bevel (`roughness: 0.08`, `metalness: 0.96`, `anisotropy: 0.12`) |
| **Epitaxial `InAs/Al` Wafer Cavity Base** | `#161414`, `#1e2d42`, `#201b18` | `#0f172a` (`PALETTE.wellSlate`) | Polished narrow-bandgap `InAs` wafer in shadow (`roughness: 0.20`, `metalness: 0.18`, `clearcoat: 0.65`) |
| **Epitaxial `InAs/Al` Active Lithography Terrace** | `#1e2d42`, `#353646`, `#505b63` | `#1e293b` (`PALETTE.graphite`) | ALD $\text{Al}_2\text{O}_3/\text{HfO}_2$ thin-film over `InAs` (`clearcoat: 0.65`, `clearcoatRoughness: 0.15`, cyan/indigo sheen) |
| **Brushed Pt/Rh Gate Electrodes & H-Tetrons** | `#b6b7b8`, `#939ead`, `#e0d3b0` | `#dce2ec` (`PALETTE.titanium`) | Electron-beam evaporated Pt/Rh/Al gate metal (`metalness: 0.85`, `roughness: 0.28`, `anisotropy: 0.68`) |
| **Crystalline Ceramic / Sapphire Carrier Base** | `#d9ccb1`, `#e0ceb2`, `#f7e8d6` | `#ebe5da` (`PALETTE.ceramicAlabaster`) | Sintered alumina ($\text{Al}_2\text{O}_3$) cryogenic carrier (`metalness: 0.0`, `roughness: 0.46`) |

---

## 3. Complete `THREE.MeshPhysicalMaterial` Parameter Table

| Material ID / Factory Key | Target Subsystem(s) | `color` (Hex) | `metalness` | `roughness` | `anisotropy` | `clearcoat` / `ccRoughness` | `iridescence` / `IOR` / `Range` |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **`cryoGoldMachined`** | D1 Package Interposer / Contacts | `0xd4af37` | `0.92` | `0.24` | `0.76` | `0.0` / `0.0` | `0.0` |
| **`cryoGoldChamfer`** | D1 45° Bore Chamfer, Seam Lip & D2 Stator Bezel | `0xe6c35c` | `0.96` | `0.08` | `0.12` | `0.15` / `0.06` | `0.0` |
| **`inasWaferSubstrate`** | D2 Hollow Well Cavity, Floor & D5 Flank Channels | `0x0f172a` | `0.18` | `0.22` | `0.0` | `0.65` / `0.15` | `0.48` / `1.76` / `[180, 420]` |
| **`platinumGateSatin`** | D1 Top Bezel, D2 12 Split-Ring Arcs, D3 Wafer Collars | `0xdce2ec` | `0.85` | `0.28` | `0.68` | `0.15` / `0.12` | `0.0` |
| **`cryoSapphireWafer`** | D3 Continuation Wafers (`[ ? ]` Glass Plates) | `0x162238` | `0.10` | `0.14` | `0.0` | `0.65` / `0.15` | `0.35` / `1.76` / `[200, 380]` |
| **`ceramicAlabasterBody`** | D1 Outer IC Carrier & D4 Call-Tile Plinths | `0xebe5da` | `0.00` | `0.46` | `0.0` | `0.0` / `0.0` | `0.0` |
