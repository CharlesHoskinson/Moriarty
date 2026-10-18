# Majorana Study 06 — Microsoft Azure Quantum Editorial Storytelling, Diagrams & Technical Rigor

**Target File:** `/home/charles/sherlock-bench/audits/majorana_study_06_editorial_storytelling.md`  
**Research Agent:** Majorana Research Agent 6 of 7  
**Primary Sources Analyzed:**
1. **Azure Quantum Blog (Chetan Nayak, Feb 19, 2025):** *"Microsoft unveils Majorana 1, the world’s first quantum processor powered by topological qubits"* (`Figure 1: Measurement-graphic.webp` & `Figure 2: Device-Roadmap-figure.webp`).
2. **Microsoft Source Feature / Press Kit (Feb 19, 2025):** *"Microsoft’s Majorana 1 chip carves new path for quantum computing"*.
3. **Peer-Reviewed *Nature* Companion (`Nature` 638, 651–655, Feb 2025):** Interferometric single-shot parity measurement in InAs–Al hybrid nanowires.

---

## 1. Visual-to-Prose Synchronization in Microsoft’s Majorana 1 Editorial Suite

Microsoft’s Majorana 1 launch materials solve a classic deep-tech communication problem: **how to explain an unfamiliar non-von-Neumann hardware architecture to technical newcomers without either dumbing down the physics or over-claiming what the current silicon actually executes.**

### 1.1 The 4-Stage Anatomical Callout Progression (Primitive $\rightarrow$ Unit Cell $\rightarrow$ Operation $\rightarrow$ Array)

| Stage | Anatomical Unit in Diagram | Synchronized Editorial Prose | Pedagogical Purpose |
| :--- | :--- | :--- | :--- |
| **1. Single Nanowire** | **Topoconductor Wire** ($\text{InAs} + \text{Al}$) hosting **Majorana Zero Modes (MZMs)** at both ends. | Quantum state is stored non-locally as **fermion parity** (even vs. odd electron count across the wire). | Establishes **where state lives** before explaining how computation moves. |
| **2. Tetron Qubit** | **`H`-Shaped Single-Qubit Tile**: Two parallel topological nanowires joined by a perpendicular superconducting bridge. | *"Aluminum nanowires joined together to form an H. Each H has four controllable Majoranas and makes one qubit."* | Gives the reader a **memorable geometric unit cell (`"H"`)** with paired endpoints. |
| **3. Digital Parity Readout** | **`Figure 1`**: Digital gate switches coupling both ends of a nanowire into a quantum dot loop with a reflected microwave pulse. | 1. **Couple** via digital switches $\rightarrow$ 2. **Capacitance shift** ($\Delta C_Q$) depends on parity $\rightarrow$ 3. **Microwave reflectometry** reads state in one shot. | Replaces abstract equations with a **3-beat causal mechanism** and contrasts digital pulses against analog dial rotations. |
| **4. Topological Core Array** | **`Figure 2`**: `"H"` tetrons tiled into a $4\times 2$ (8-qubit) array inside a palm-sized cryogenic carrier. | Explains how the unit cell composes into the macro chip while clearly separating the 8-qubit test die from future fault-tolerant arrays. | Shows **how the unit cell composes into the whole chip** with honest scientific caveats. |

---

## 2. Direct Application to Sherlock’s UI & Prose Deck (`index.html`)

Following Microsoft’s **Primitive $\rightarrow$ Unit Cell $\rightarrow$ Readout $\rightarrow$ Array** progression, every UI surface in Sherlock presents the exact same **3-Level Vertical & Radial Anatomy**:

1. **Level 1 — Central `Control` Stator (Center Bore, `y = -0.24`):**
   - A 10-sector dial at the center of the die representing the 10 `Control` states of `sherlock-k-a/1` (`Eval`, `Push`, `Return`, `Invoke`, `Enter`, `Bound`, `Cases`, `CaseResult`, `PrimitiveResult`, `Error`) with an open center bore (`r < 0.135`) and a hemisphere pointer indicating **West (`Slot A`)** or **East (`Slot B`)**.
2. **Level 2 — 12 Split-Arc Continuation Slots (`R1.A..R6.B`, Concentric Well, `y = +0.06..-0.21`):**
   - Six stepped concentric rings (`R1` outermost to `R6` innermost), each split across a North–South bridge into a **West half-arc (`Slot A`, older)** and an **East half-arc (`Slot B`, newer)**—a 12-slot optical window into `frames` ordered chronologically from `frames[len - 1]` to `frames[0]`.
3. **Level 3 — Hollow 3D Chamber (`y = -0.35..-2.15`) & 39-Rule Decoder Floor (`y = -2.18`):**
   - Up to **12 horizontal Continuation Wafers** suspended in the hollow square chamber, above a **6×7 micro-LED Decoder Floor** engraved with all **39 semantic rules (`A080..A225`)** of `sherlock-k-a/1`.
