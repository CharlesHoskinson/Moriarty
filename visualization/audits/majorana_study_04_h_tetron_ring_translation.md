# Majorana Study 04 — Translating Majorana 1's H-Shaped Tetron & Interferometer Loops to Sherlock's Split-Arc Rings

**Research Agent:** Majorana Research Agent 4 of 7 (H-Shaped Tetron, Twin Topoconductor Nanowires, Superconducting Bridge & Coupler-Dot Loops $\rightarrow$ Sherlock Split-Arc Rings `R1`..`R6`)  
**Primary Hardware Sources:**
- Microsoft Quantum, *"Roadmap to fault tolerant quantum computation using topological qubit arrays"*, `arXiv:2502.12252v1` (Feb 2025).
- Microsoft Azure Quantum (M. Aghaee *et al.*), *"Interferometric single-shot parity measurement in InAs–Al hybrid devices"*, *Nature* **638**, 651–655 (2025) / `arXiv:2401.09549v4`.

---

## 1. The Paired-End Architecture of Microsoft's H-Shaped Topological Qubit ("Tetron")

In Microsoft's **Majorana 1** Topological Core QPU, the fundamental single-qubit unit cell is the **two-sided H-shaped tetron**: a floating superconducting island formed by **two parallel 1D topological superconductor (1DTS) nanowire arms** joined at their midpoints by a **perpendicular trivial superconducting crossbar ("backbone")**, surrounded by precision **gate-defined quantum dots (QDs)** and **cutter/plunger gates** that form closed interferometric measurement loops.

Inside the fixed-parity subspace $-\gamma_1\gamma_2\gamma_3\gamma_4 = +1$, the orthogonal single-qubit Pauli operators are pairs of Majorana bilinears:
$$Z = i\gamma_1\gamma_2 = i\gamma_3\gamma_4 \quad (\text{intra-wire parity along a single arm})$$
$$X = i\gamma_1\gamma_3 = -i\gamma_2\gamma_4 \quad (\text{inter-wire parity across the central H-bridge})$$

---

## 2. Direct Structural Analogy to Sherlock's Split-Arc Rings (`Slot A` West + `Slot B` East + Central `Control` Stator)

| Majorana 1 H-Tetron Hardware Element | Sherlock Split-Arc Ring Element (`designer2_well.js`) | Exact Structural & Operational Correspondence |
| :--- | :--- | :--- |
| **Twin Parallel Topoconductor Arms (`Arm 1` & `Arm 2`)** | **West Half-Arc (`Slot A = 2r`, $\theta \in [0.54\pi, 1.46\pi]$) & East Half-Arc (`Slot B = 2r+1`, $\theta \in [-0.46\pi, 0.46\pi]$)** | Just as a tetron pairs two parallel nanowire arms to host a single logical unit, each ring `R_r` pairs two $165.6^\circ$ ($0.92\pi$) half-arcs (`Slot A` West and `Slot B` East) to hold the two chronological continuation frames (`2r` and `2r+1`) of that recursion depth. |
| **Central Perpendicular Superconducting Crossbar (`H`-Backbone)** | **Central 10-Sector `Control` Stator Core (`y = -0.24`, $r \in [0.142, 0.216]$) + North/South Bridge Tabs** | Sits at the midpoint between West `Slot A` and East `Slot B`, routing execution state between the two half-arcs while keeping the central `r < 0.135` optical bore open. |
| **Intra-Arm $Z$-Loop Parity Measurement ($Z = i\gamma_1\gamma_2$)** | **In-Place Frame Rewrite (`Rules A145, A149, A177`) Along One Half-Arc** | Mutates the state of a **single** Half-Arc in-place at fixed stack depth ($\Delta d = 0$), shifting its inner channel color (`Amethyst → Sky-Cyan` on `A145`, `Amber → Gold` on `A149`, `Copper → Gold` on `A177`) and firing a torsional detent pulse. |
| **Inter-Arm $X$-Loop Parity Measurement ($X = i\gamma_1\gamma_3$)** | **Paired-Slot Call Activation Across West `Slot A` & East `Slot B`** | Pairs West `Slot A` (`Frame::Return`, `ENV` Deep Teal `#0d9488`) with East `Slot B` (`Frame::Left → Right`, `OP` Amber `#f59e0b` $\to$ Gold `#fbbf24`) across the central `Control` stator. |
