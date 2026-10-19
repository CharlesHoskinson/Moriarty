# DESIGN-014 — Let Scheduling, Frame Installation, and Complete Routing

**Date:** 2026-10-19
**Log:** Sherlock circuit website concept log. This is a standalone iteration; DESIGN-013 is unchanged.
**Scope:** Internal visual and interaction concept only. This note does not implement, mock up, publish or deploy anything. The circuit website publishes only when Sherlock is fully finished.
**Inputs:** Public metadata (`DESIGN014-public-concept-input/1`) and DESIGN-013, which serves as historical visual context only. No private, raw, value, witness or proof body was supplied or used.

---

## 1. What this note stands on

**Observed (from metadata):**
- A114 public semantics:
  - Rule `k.machine.eval.let`, starting at line 114, ordered tag 6 of 39.
  - Premises: Pending outcome, `Eval(Let(name,value,body))` control, positive remaining fuel.
  - Event: it schedules `Push(LetFrame(name,body,savedEnv), Eval(value), savedEnv)`.
  - Charge 1. It writes control and decrements remaining fuel by 1. Everything else is preserved.
- A114 source is still in authorship (handle 23742). No A114 physical emitter has been returned, adopted or compiled.
- A main.k sixteen-operation comparison is recorded as `ROOT_CONFIRMED_PASS_EXACT16_SCOPED`. Its scope and limits are covered in §5.

**Proposed (this note):**
- Every layout, material and interaction below.

**Open owner choices:**
- A114 physical write, append and reference policy.
- A114 rule-specific statement and context.
- The independent physical expectation freeze.
- Final geometry for any interface.

Carried forward from DESIGN-013 as visual language only:
- Moriarty wordmark as silkscreen.
- Near-black layered board with via-only cross-layer links.
- Selection drawn as a steady glow; observed results drawn as a static fill.
- No idle motion.

None of DESIGN-013's dated status findings carry forward.

---

## 2. Default board view

Fixed camera, slight three-quarter tilt, static exploded layers. Nothing rotates or pulses.

The centre of the board is the **machine-state bus**. It is eight parallel labelled lanes that run left to right across every stage footprint:

| Lane | Label |
|---|---|
| C | Control |
| RF | Remaining fuel |
| IF | Initial fuel |
| P | Program (complete, including unused functions) |
| E | Live environment |
| F | Frames and depth |
| SL | Stack limit |
| O | Outcome |

Two stage footprints sit on the bus with a visible gap between them:
- **A114 — schedule**
- **A135 — install**

A third pad, **A140 — capacity refusal**, branches off near A135. It does not touch A114.

A persistent legend on L1 reads: *"39 semantic rule IDs — not 39 physical circuits."*

Visual grammar, with each state carrying a non-hue cue:

| Meaning | Treatment |
|---|---|
| Semantic route (normative) | Solid hairline, unlit |
| Selected object | Steady glow and a thicker stroke |
| Independently observed result | Static fill with a hatched border |
| Owner proposal | Dashed outline with the label `PROPOSAL · unreviewed` |
| Not established | Empty socket outline |

---

## 3. A114 schedules; A135 installs

The design goal is that nobody can mistake *building* pending control for *mutating* the stack.

**Inside the A114 footprint:**
- **Control (C).**
  - The incoming `Eval(Let(name,value,body))` terminates at a premise gate.
  - The outgoing lane carries a sealed **envelope component** labelled `Push(LetFrame(name, body, savedEnv), Eval(value), savedEnv)`.
  - The envelope is a *routed intention*. It is drawn as a closed package sitting on the control layer, not on the frames layer.
  - `Eval(value)` appears inside the envelope as the next child control, still sealed.
- **Live environment (E).**
  - E passes through unchanged.
  - A tap via copies a reference up into the envelope's `savedEnv` slot. A tap is drawn as a branch, so E is not cut.
- **Frames and depth (F).**
  - F is a ruler-straight lane with a fixed depth readout and an `unchanged` tag.
  - No via connects the envelope to F inside A114.
- **Fuel.**
  - RF shows a single `−1` tick at the A114 boundary.
  - IF runs beside it untouched, so remaining and initial fuel can always be compared.
- **Preserved lanes.**
  - P, SL and O (Pending) pass straight through.
  - P is labelled complete. Unused functions remain visible as unlit stubs and are not elided.
- **Charge.** A chip marked `charge 1` sits on the footprint corner.

**Inside the A135 footprint:**
- A135 has its own premise gate and its own charge chip.
- Only here does a via drop from the envelope into F. The frame becomes live and the depth readout steps.
- The envelope opens. Its child control continues on C, and its environment route continues on E.
- A135's internal routing is drawn from normative semantics only. Physical detail is dashed proposal or empty.

**A140** is a dead-end pad with its own label. It is never drawn as a failure state of A114.

Every lane is an **inspectable route**. Hovering or focusing a lane shows its name, its before/after relation and its source semantic line. It never shows a value. No light travels along the lanes.

---

## 4. Assembly view: how the interpreter circuit gets put together

A second camera preset separates the board into five strata. The user reads them top to bottom:

1. **S1 — Rule selector.**
   - The full set of 39 hex pads.
   - A114 is pad 6, labelled with `k.machine.eval.let`.
2. **S2 — Source, operand and field routes.**
   - Normative routes for `name`, `value`, `body`, `savedEnv`, and each state lane.
   - Drawn as solid hairlines.
3. **S3 — Proposed physical fragments.**
   - Dashed footprints only.
   - For A114 today this stratum shows labelled empty slots:
     - "write/append/reference policy — open"
     - "statement/context — open"
     - "source in authorship"
   - No fragment is drawn as if it existed.
   - A102 context is never placed in an A114 slot.
4. **S4 — Complete per-step state and statement.**
   - One full row per step, with all eight lanes and no diffs. Preserved lanes appear explicitly.
   - Values are never shown; cells are structural only.
5. **S5 — Evidence sockets.**
   - Three separate sockets: **execution**, **proof** and **target**.
   - Each fills only on its own independent evidence and never by inference from another socket.
   - For A114, all three are empty.

Vias between strata exist only where a relation is normative (S1→S2→S4) or explicitly proposed (S3, dashed). Each socket in S5 has its own via; none is ganged.

---

## 5. Observed main.k sixteen-operation plate

This is a separate plate on its own layer, offset from the A114 footprint. It has **no via to A114**.

**Shown:**
- Status `ROOT_CONFIRMED_PASS_EXACT16_SCOPED`, 16 accepted operations.
- Three rule-span chips: startup74, literal102, publication80.
- An explicit terminal no-rule marker.
- A nine-cell post-startup comparison grid, as outlined cells without values.
- 164 pre-comparison frozen members, with the pin match recorded.
- The retained 335 carrier relation.
- No source, model or resource changes.

**Footer (always visible):**
- Finite scalar case under declared mapping assumptions.
- `context_binds_counterpart_source: false`.
- Helper evaluation not traced.
- Printer and Root receipts trusted.
- No physical, native, refinement, recursive-proof, target or global claim.

The 35.396 s summed child wall time appears only in the inspect panel as a recorded field. It is not a speed, cost or benchmark figure. The plate never feeds an S5 socket for A114 or any physical fragment.

---

## 6. Interactions and reduced-motion presentation

| Action | Result |
|---|---|
| Select pad (S1) | Pad glows. Its S2 routes thicken. The side panel shows the rule ID, line, premises, writes and preserved lanes. |
| Focus a lane | The full route highlights end to end, with its before/after relation and a `charge` marker if one applies. |
| Toggle **Schedule ↔ Install** | Switches emphasis between A114 and A135 as a hard cut. The two are never blended or animated. |
| Toggle **Assembly** | Moves to the five-strata preset in one step. |
| Inspect plate (§5) | Opens metadata pins, scope and limits. |

**Reduced-motion and accessible mode:**
- A flat 2D schematic replaces the scene. Strata become stacked rows and lanes become labelled horizontal rules.
- No camera moves and no transitions.
- Every state uses text plus dash or fill patterns, so nothing relies on hue.
- Lanes are reachable by keyboard and announced as "Frames and depth — unchanged by A114."

The 3D view is conceptual. It is not a physical chip layout, a circuit placement, or a proof running on screen.

---

## 7. Alternative

**Timeline strip.** A horizontal sequence of full-state cards (A114 card → A135 card), with the envelope as a card attachment.
- It is simpler and very readable in 2D.
- It loses the stratum view of assembly, so it is better treated as the reduced-motion fallback than as the default.

## 8. Recommended next step

Once the A114 source author returns and the owner reviews it, draw only the S3 dashed fragments the owner actually proposes. Keep S5 empty until independent physical expectation freeze and execution evidence exist separately. Do not start A135 S3 work until A114's S3 is settled.

---

## Status boundary

- **Observed:** §1 facts and §5 contents, as recorded in metadata.
- **Proposed:** §2–§4 and §6–§8.
- **Not established:**
  - Any A114 physical source, emitter or observation.
  - Handwritten-circuit refinement.
  - Native compilation or proof.
  - Recursive proof.
  - Target settlement.
  - Sherlock completion.
- **Not done:** No code, assets, publication or provider calls. Hashes are carried from the input and were not re-verified.

## Source pins

All paths are under `/home/charl/Moriarty/.moriarty-dev/runtime/sherlock-review-20261017/`.

| Pin | Path | Bytes | SHA-256 |
|---|---|---|---|
| Concept input | `DESIGN014-public-concept-input/1`, frozen `2026-10-19T18:59:34.958302+00:00` (inline) | — | — |
| main.k result | `k-main-literal-complete-rule-correspondence-20261019/revision02-input-bindings-f03-gate/actual/ROOT-RESULT.json` | 8067 | `6435e94442b29853243ea23b817f8743a55270598946cf25f1be75e3d10901ff` |
| main.k readback | `…/actual/ROOT-ACTUAL-FINAL-READBACK.json` | 2397 | `01b214f759c9200158d26e62eed15d4130a899d700ff08e67bed8bbcd05d7329` |
| DESIGN-013 note | `post-PR297-formal-docs-integration-plan-20261019/design013-docs-component-02-new-K-tail/afterimages/wiki-llm/sherlock-circuit-atlas/DESIGN-013-LAYERED-BOARD-RULE-SELECTORS-AND-TRACE.md` | 12437 | `7dae428e526ca54ad6855b9d2cc5eeab661609388597c955aca23f1a41d7ce4b` |