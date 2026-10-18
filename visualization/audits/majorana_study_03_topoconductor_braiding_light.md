# Majorana Study 03 — Visualizing Invisible Quantum & Abstract States via Waveguide Pulses & Parity Readouts

**Research Track:** Majorana Research Agent 3 of 7 (Topoconductor Braiding, Microwave Interferometry & Sub-Surface Waveguide Light)  
**Target Subsystem:** `src/designers/designer2_well.js` (`createWellSubsystem` — 10-Sector `Control` Optical Stator & 12 Split-Arc Half-Rings `R1.A .. R6.B`)  
**Primary Scientific & Visual References:**
- **Microsoft Azure Quantum (`arXiv:2401.09549v4` / *Nature* 638, 651–655, Feb 2025):** *"Interferometric Single-Shot Parity Measurement in an InAs-Al Hybrid Device"* (Figs. 1–5, Secs. 1–4, Supplement S1–S4).
- **Microsoft Quantum (`arXiv:2502.12252v3`, Feb/Jul 2025):** *"Roadmap to fault tolerant quantum computation using topological qubit arrays"* (Figs. 1–4, 6, 10; Secs. II.1–II.2, III.1–III.2).
- **Microsoft Majorana 1 Technical Animations & Launch Visuals (Feb 2025):** 3D hardware explainers of the InAs–Al topoconductor tetron, digital cutter-gate switching, quantum-dot charge-sensing loops, and dispersive microwave reflectometry.
- **Sherlock Ground Truth & Audits:** `audits/grok_audit_03_rings_control_core.md`, `audits/grok_audit_02_lighting_materials.md`, `src/designers/ring_study_02_frames.md`, `src/designers/ring_study_03_control_core.md`, `src/designers/ring_study_04_split_arc_geometry.md`, and `src/designers/designer2_well.js`.

---

## 1. Digital Parity Readout & Waveguide Interferometry Visuals in Microsoft's Majorana 1

A central visualization challenge in quantum hardware—and in abstract machines like the Sherlock CEK machine (`sherlock-k-a/1`)—is **how to make an invisible, non-local internal state legible to the human eye without fabricating fake floating holograms or mutating the underlying state**.

In Microsoft's **Majorana 1** architecture, quantum information is stored non-locally in the **shared fermion parity** ($Z = \pm 1$, corresponding to an even or odd number of electrons—the difference between $10^9$ and $10^9 + 1$ electrons) split across spatially separated **Majorana Zero Modes (MZMs, $\gamma_1, \gamma_2, \gamma_3, \gamma_4$)** at the ends of Indium Arsenide–Aluminum ($\text{InAs–Al}$) hybrid "topoconductor" nanowires. Because a topological wire in its one-dimensional topological superconductor (1DTS) phase hides its state from local electrostatic probes, Microsoft neither measures the wire directly nor physically swaps nanowires in 3D space. Instead, all state readout and measurement-based braiding are performed via **gate-defined quantum-dot interferometry and dispersive microwave reflectometry**.

### 1.1 Anatomy of the Tetron & Triple-Quantum-Dot Interferometer (`arXiv:2401.09549` Fig. 1 & `arXiv:2502.12252` Figs. 1–2)

1. **The "H"-Shaped Tetron Core**:
   - Two parallel horizontal $\text{InAs–Al}$ topological nanowires ($L \approx 3\text{ }\mu\text{m}$, rendered in cold sapphire-blue / brushed rhodium) host four Majorana zero modes ($\gamma_1, \gamma_2$ on the top wire; $\gamma_3, \gamma_4$ on the bottom wire, rendered as warm luminous amber-orange terminal nodes $\star$ embedded at the wire ends).
   - A perpendicular trivial superconducting Aluminum backbone (light blue/silver) bridges the two topological wires at their midpoint so the entire "H" forms a single floating superconducting island with conserved total charge.
2. **Digital Cutter Gates & Plunger Gates (Two-Layer Lithography)**:
   - **Layer 1 (Yellow in false-color SEM)**: Wire plunger gates ($V_{\text{WP1}}$) and depletion gates carve the 2D electron gas (2DEG) mesa and tune the nanowire chemical potential into the topological regime at in-plane magnetic field $B_\parallel \approx 1.8\text{–}2.0\text{ T}$.
   - **Layer 2 (Purple in false-color SEM)**: Overlying quantum dot plunger gates ($V_{\text{QD1}}, V_{\text{QD2}}, V_{\text{QD3}}$) define tiny semiconductor charge islands adjacent to the MZM wire ends, while narrow rectangular **cutter gates** sit across the junctions between the quantum dots and the MZM terminals.
3. **Closing the Aharonov–Bohm Interference Loop**:
   - In the **idle state**, all quantum dots are detuned into deep Coulomb valleys and all cutter gates are pinched off—the topological nanowire is isolated.
   - To perform a **digital parity readout**, a voltage pulse opens the cutter gates along the chosen measurement loop and tunes the quantum dots near their $N \leftrightarrow N+1$ charge degeneracy point ($E_D \approx 0$).
   - Because the two tunneling pathways ($t_L$ via $\gamma_1$ and $t_R$ via $\gamma_2$) enclose a magnetic flux $\Phi$, their quantum amplitudes interfere constructively or destructively depending on the **non-local fermion parity $Z = i\gamma_1\gamma_2 \in \{-1, +1\}$** of the nanowire:
     $$t_C(Z, \phi) = \sqrt{t_L^2 + t_R^2 + 2 Z\, t_L t_R \cos\phi}$$

### 1.2 Orthogonal Loop Topologies: Intra-Wire $Z$-Loops vs. Cross-Wire $X$-Loops
- **$Z$-Measurement Loop (Single-Wire Intra-Pair Parity, $i\gamma_1\gamma_2$)**: Connects $\text{QD}$s strictly along **one** topological nanowire, confirming or locking internal parity in place (analogous to `Rule 149` `Left -> Right` in Sherlock).
- **$X$-Measurement Loop (Cross-Wire Inter-Pair Parity, $i\gamma_1\gamma_3$)**: Bridges **across** the central superconducting backbone to project state across families at constant depth (analogous to `Rule 145` `Let -> Scope` in Sherlock).

---

## 2. Sub-Surface Emissive Conduits vs. Floating Clutter

In Microsoft's Majorana 1 3D animations, **100% of dynamic energy travels sub-surface** inside three micro-machined architectural features:
1. **Recessed Waveguide Grooves**: Coplanar microwave resonators etched *below* the ground plane, catching grazing reflections on 45° chamfered sidewalls (*anglage*).
2. **Quantum-Dot Charge Basins**: Sub-micron electrostatic wells recessed at discrete stations along the nanowire perimeter (dark slate when detuned, igniting into a crisp jewel node when charged).
3. **Cutter-Gate Micro-Bridges**: Ultra-thin second-layer metallic gate fingers crossing dielectric isolation gaps without obstructing the central optical sightline.

---

## 3. Direct Application to Sherlock (`designer2_well.js`)

| Transition in `reference.rs` | Majorana 1 Physical Analogue | Sherlock Sub-Surface Visual Signature |
| :--- | :--- | :--- |
| **`Control::Eval` (`Rules 102..131`)** | Plunger gate pre-bias ($\Delta d = 0$) | Stator Tooth `0` (`EVL`) elevates `+0.006 Y` in warm champagne (`0xd4af37`). |
| **`Rule 135 PushFrame` ($d \to d+1$)** | Cutter gate opening & charge injection | Frontier Half-Arc (`Rn.A` West or `Rn.B` East) elevates `+0.027 Y` and lights its recessed waveguide groove in its family hue. |
| **`Rule 149 RightOp` (`Left -> Right`, $\Delta d = 0$)** | Intra-wire $Z$-loop parity lock | Frontier `OP` Half-Arc shifts in-place from **Amber (`0xf59e0b`)** to **Luminous Gold (`0xfbbf24`)** with a $\pm 2.2^\circ$ detent click. |
| **`Rule 145 BindLet` (`Let -> Scope`, $\Delta d = 0$)** | Cross-wire $X$-loop basis projection | Frontier Half-Arc shifts in-place from **Violet (`0x818cf8`)** to **Sky-Cyan (`0x38bdf8`)** at constant depth. |
| **`Rule 177 NextArg` (`Args -> Args`, $\Delta d = 0$)** | Triple-QD sequential charge ratchet | Frontier `Args` Half-Arc shifts from **Copper-Orange (`0xf97316`)** to **Luminous Gold (`0xfbbf24`)** as evaluated argument count advances. |
