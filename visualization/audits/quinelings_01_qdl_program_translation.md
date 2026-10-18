# Quinelings × Sherlock Bench Audit 01: QDL Framework & 8-Program Quineling Genome Translator

**Document Target:** `/home/charles/sherlock-bench/audits/quinelings_01_qdl_program_translation.md`  
**Author:** Gemini Quineling Agent 1 of 3 (QDL Framework & 8-Program Quineling Genome Translator)  
**Source Repositories Analyzed:**  
- Quinelings (`/home/charles/repos/quinelings`: `qdl.js`, `lifeform-families.js`, `morphology.js`, `anatomy.js`, `woven-body.js`, `chroma.js`, `core.js`, `docs/MAPPING.md`, `docs/TRANSLATION.md`, `docs/THOUGHT-TO-LIFEFORM.md`, `docs/QDL-V1.md`, `docs/QDL.md`, `docs/DESIGN-LANGUAGE.md`)
- Sherlock Bench (`/home/charles/sherlock-bench/src/engine/sherlockEngine.js`, `/home/charles/sherlock-bench/src/app.js`)

---

## 1. Executive Summary: Translating Sherlock Programs into Living Thoughts

In the **Quinelings** architecture (`qdl.js`, `lifeform-families.js`, `morphology.js`, `anatomy.js`, `woven-body.js`, `chroma.js`), an executable program is never rendered as an arbitrary particle cloud or generic diagram. Instead, it exists simultaneously in **three canonical representations** (`docs/MAPPING.md`):

1. **Executable Genotype (`ProgramImage` / Quoted AST):** The canonical expression table (`programImage.expressions` and `programImage.functions` in `sherlockEngine.js`) together with its fuel and continuation-frame bounds.
2. **Recoverable Mathematical & Color Genome (`quineling-harmonics-1` & `quineling-chroma-1`):**
   - **Harmonic Genome:** Canonical UTF-8 program bytes framed by the 4-byte magic `QLNG` (`[81, 76, 78, 71]`), a 32-bit big-endian byte length, and a 32-bit FNV-1a checksum (`h0 = 2166136261`, prime `16777619`), partitioned into 32-coefficient cosine bands (`a[j,k] = byte + 1`):
     $$F_j(\theta) = \sum_{k=1}^{32} a_{j,k} \cos(k\theta)$$
     Inversible via 65-point discrete cosine orthogonality at $\theta_m = 2\pi m / 65$.
   - **RGB Byte Strand (`quineling-chroma-1`):** Lossless per-byte triplet mapping with two redundant drift-rejection channels:
     $$R = b, \quad G = 255 - b, \quad B = (73b + 19) \bmod 256$$
3. **Animated Structural Phenotype (QDL Morphology & Chromamapped Membrane):** A deterministic, many-to-one 3D/2D mathematical body whose **QDL family**, **woven mechanism**, **skeletal attachment tree**, **organ rosettes**, **endpoint-pinned dependency filaments**, and **material color territories** are derived directly from the program's AST topology and modulated at runtime by the recorded `sherlock-k-a/1` execution trace without mutating source identity.

---

## 2. Summary Matrix of the 8 Sherlock Quineling Species

| # | Preset ID (Canonical / Alias) | Species Name (Common / Binomial) | QDL Family (`qdl.js`) | Woven Mechanism (`lifeform-families.js` / `woven-body.js`) | Symmetry Order | Gesture (`anatomy.js`) | AST Seed (`FNV-1a`) | AST Nodes / Max Frames / Steps |
| :-: | :--- | :--- | :--- | :--- | :--- | :--- | :--- | :-: |
| **1** | `factorial`<br>*(Factorial(4))* | **Nautilus Arithmeticus**<br>*(Spiralis multiplicata)* | `nautilus` | `logarithmic-mantle` | **$C_1$ Chiral Logarithmic Spiral** (2.12 whorls) | `glide`<br>`[180,170,450,200]` | `0x1c2e64fe`<br>`472802558` | 13 nodes<br>11 frames<br>145 steps |
| **2** | `list_fold`<br>*(alias: `list_sum`)* | **Taenia Cons-Catenata**<br>*(Vitta catenaria)* | `ribbon` | `logarithmic-mantle` | **$C_1$ Segmented Axial Pleat** (3 cons-vertebrae) | `gather`<br>`[180,180,420,220]` | `0xabe82779`<br>`2884118393` | 15 nodes<br>11 frames<br>112 steps |
| **3** | `tree_fold`<br>*(alias: `tree_sum`)* | **Corallium Arboris**<br>*(Dendron bifurcata)* | `coral` | `recursive-julia`<br>*(+ `phyllotaxis-fan`)* | **$D_2$ Bilateral Dichotomous Fan** (3 leaf-tips) | `unfurl`<br>`[220,170,430,180]` | `0x7ba706db`<br>`2074543835` | 17 nodes<br>8 frames<br>127 steps |
| **4** | `mutual_even_odd`<br>*(ParityCheck)* | **Lepidoptera Alternans**<br>*(Bimorpha paritatis)* | `moth` | `toroidal-weave` | **$C_2$ Bilateral Antipodal Wing-Pair** (`is_even` $\leftrightarrow$ `is_odd`) | `hover`<br>`[180,170,450,200]` | `0x2d6f9ecc`<br>`762289868` | 20 nodes<br>7 frames<br>121 steps |
| **5** | `lexical_let`<br>*(alias: `let_shadow`)* | **PhylloCube Umbra**<br>*(Granum cubicum)* | `seed` | `phyllotaxis-fan` | **$C_\varphi$ Golden-Angle Nested Capsule** ($137.51^\circ$ phyllotaxis, 3 shells) | `gather`<br>`[180,180,420,220]` | `0x0bcbca4b`<br>`197904971` | 16 nodes<br>5 frames<br>46 steps |
| **6** | `checked_underflow`<br>*(alias: `checked_sub_trap`)* | **Ignis Subtractor**<br>*(Filum fracturae)* | `filament` | `clifford-flow`<br>*(+ `phyllotaxis-fan`)* | **$C_1$ Asymmetric Snap-Tendril** (16-limb borrow whip) | `hover`<br>`[180,170,450,200]` | `0x7e767d50`<br>`2121694544` | 6 nodes<br>2 frames<br>18 steps |
| **7** | `stack_overflow`<br>*(StackTrap)* | **Cometa Abyssi**<br>*(Cauda quintus)* | `comet` | `logarithmic-mantle` | **$C_5$ Compressed 5-Frame Lance** (truncated at $L_{\text{stack}}=5$) | `glide`<br>`[180,170,450,200]` | `0x4f0ccbbd`<br>`1326238653` | 13 nodes<br>5 frames<br>55 steps |
| **8** | `fuel_exhaustion`<br>*(FuelBound)* | **Cyclopora Exhaurire**<br>*(Annulus quattuordecim)* | `torus` | `toroidal-weave` | **$C_{14}$ / $(1,3)$ Toroidal Seam-Loop** (14-step fuel ring) | `gather`<br>`[180,180,420,220]` | `0xe801cd61`<br>`3892432225` | 11 nodes<br>3 frames<br>15 steps |
