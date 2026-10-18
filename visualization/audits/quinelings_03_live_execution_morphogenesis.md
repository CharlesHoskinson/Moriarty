# Quineling Audit 03 — Live Execution Morphogenesis & Step-by-Step State Choreography

**Target Subsystem**: `SherlockStage` Interior Core (`r < 1.00`, `y ∈ [-3.38, -0.24]`), coupling `designer2_well.js` (6 Split-Arc Rings, 10-Sector Control Stator, 39-Rule Silicon Floor), `designer3_frames.js` (12 Open-Bore Annular Continuation Wafers), `sherlockEngine.js` (`StepRecord` & `buildStepState`), and `app.js` (Stage Caption Bar & Right-Rail Inspector).  
**Contract Alignment**: QDL v1 (`docs/QDL.md`), Thought-to-Lifeform Assembly Grammar (`docs/THOUGHT-TO-LIFEFORM.md`), Program Contract (`docs/PROGRAM-CONTRACT.md`, `kernels.js`), and `sherlock-k-a/1` (`SHERLOCK_K_RULES` A080–A225).

## 1. Executive Summary & Architectural Invariants
Inside the Sherlock 3D interpreter die, the open-bore central light shaft (`r < 1.02` inside the 12 continuation wafers and `r < 0.135` through the 10-sector Control Optical Stator at `y = -0.24`) forms a 3D vertical atrium extending down to the 39-rule silicon decoder floor (`y = -3.38`). The **Quineling** inhabits this central column with its **nucleus anchored at `(0, -1.65, 0)`**, its **root tendrils reaching `y = -2.20`** above the 39-rule floor, and its **apical crown / split-arc coupling filaments reaching `y = -0.25`** just beneath the 6 concentric split-arc rings (`R1.A..R6.B`, `y = -0.21..+0.06`) and the 10-sector Control Stator (`y = -0.24`).

1. **Two Orthogonal Phase Drivers**:
   - **Presentation Phase (`t = stepState.ambient`)**: Drives the continuous QDL coupled-harmonic breathing and traveling tissue wave (`φ = r.rate · t`, `θ = φ + r.asymmetry · sin(φ)`) when paused or playing, never mutating or advancing interpreter state.
   - **Execution Morphogenesis Coordinate (`τ = stepIndex + stepFraction`)**: Drives deterministic structural growth, in-place torsion, floor-to-ring energy pulses, and terminal state transitions via the C²-continuous quintic Hermite step interpolant from `THOUGHT-TO-LIFEFORM.md`:
     $$H(z) = 6z^5 - 15z^4 + 10z^3, \quad z = \text{clamp}(\text{stepFraction}, 0, 1)$$
2. **Zero-Allocation Hot Path (`update(stepState)`)**:
   - All Quineling anatomical organs, vertebrae/petals (`MAX_WAFERS = 12`), call-chain branches (`MAX_CALL_BRANCHES = 16`), and root-to-crown pulse filaments (`12` root tendrils + `12` crown filaments) are pre-allocated in fixed `THREE.InstancedMesh` buffers (`DynamicDrawUsage`) using module-scope scratch matrices and colors.

## 2. Preset-to-QDL Species & Gesture Mapping
| Preset ID (`sherlockEngine.js`) | QDL `family` | Authored `gesture.kind` | `rhythm.mode` | Dominant Silhouette & Anatomical Growth Mode |
| :--- | :--- | :--- | :--- | :--- |
| `factorial` (`fact(4)`) | `nautilus` | `unfurl` | `periodic` | Logarithmic chambered spiral ($r = 0.1e^{g\theta}$); each recursive call and `Right(n * [?])` frame unfurls a golden-ratio chambered vertebra. |
| `list_fold` (`sum_list([5,8,3])`) | `filament` | `glide` | `periodic` | Segmented articulated axial spine; each `cons(h, t)` match arm and `Left/Right` frame extends a dorsal vertebral rib pair. |
| `tree_fold` (`sum_tree(branch(..))`) | `coral` | `unfurl` | `quasiperiodic` | Bifurcating dendritic polyp; `branch(l, r)` splits left (`West Slot A`) and right (`East Slot B`) dichotomous branches with contractive scale. |
| `mutual_even_odd` (`is_even(4)`) | `moth` | `hover` | `periodic` | Bilateral twin-wing membrane (`is_even` West wing $\leftrightarrow$ `is_odd` East wing) alternating phase stroke on each tail-like call handoff. |
| `lexical_let` (`cube_plus(12, 272)`) | `torus` | `gather` | `periodic` | Nested `(1,3)` toroidal weave; each `Let → Scope` binding wraps an inner cyan lexical flux sheath around the core. |
| `checked_underflow` (`clamp_sub(3, 9)`) | `jelly` | `gather` | `periodic` | Translucent bell with 16 trailing `aux/2` byte tentacles that contract into crimson fault spines on `bo[16] = 1` (`Rule A225`). |
| `stack_overflow` (`dive(6)`, limit 5) | `ribbon` | `unfurl` | `periodic` | Helical ascending ribbon that climbs toward the `stackLimit` horizon ring until `Rule A140 FrameErr` triggers a crimson horizon recoil. |
| `fuel_exhaustion` (`count_down(5)`, fuel 14) | `bloom` | `hover` | `quasiperiodic` | Golden-angle phyllotaxis rosette ($\Delta\alpha = \pi(3-\sqrt{5})$) whose outer petals dim centripetally as fuel drains to `0` (`Rule A084`). |
